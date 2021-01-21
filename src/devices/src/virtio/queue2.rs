use std::cmp::min;
use std::num::Wrapping;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::Ordering;

use vm_memory::{Address, Bytes, GuestAddress, GuestMemoryMmap};
use vm_virtio::{DescriptorChain as Chain, Queue as Q, QueueConfig};

pub(super) const VIRTQ_DESC_F_NEXT: u16 = 0x1;
pub(super) const VIRTQ_DESC_F_WRITE: u16 = 0x2;

pub struct Queue {
    cfg: QueueConfig,
}

impl Deref for Queue {
    type Target = QueueConfig;

    fn deref(&self) -> &Self::Target {
        &self.cfg
    }
}

impl DerefMut for Queue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cfg
    }
}

impl Queue {
    fn q<'a, 'b: 'a>(&'a self, mem: &'b GuestMemoryMmap) -> Q<&GuestMemoryMmap, &QueueConfig> {
        Q::with_cfg(mem, &self.cfg)
    }

    fn q_mut<'a, 'b: 'a>(
        &'a mut self,
        mem: &'b GuestMemoryMmap,
    ) -> Q<&GuestMemoryMmap, &mut QueueConfig> {
        Q::with_cfg(mem, &mut self.cfg)
    }

    pub fn new(max_size: u16) -> Self {
        Self {
            cfg: QueueConfig::with_max_size(max_size),
        }
    }

    pub fn get_max_size(&self) -> u16 {
        self.max_size
    }

    pub fn actual_size(&self) -> u16 {
        min(self.size, self.max_size)
    }

    pub fn is_valid(&self, mem: &GuestMemoryMmap) -> bool {
        Q::with_cfg(mem, &self.cfg).is_valid()
    }

    pub fn len(&self, mem: &GuestMemoryMmap) -> u16 {
        let avail_idx = self.q(mem).avail_idx(Ordering::Acquire).unwrap();
        (avail_idx - self.next_avail).0
    }

    /// Checks if the driver has made any descriptor chains available in the avail ring.
    pub fn is_empty(&self, mem: &GuestMemoryMmap) -> bool {
        self.len(mem) == 0
    }

    /// Pop the first available descriptor chain from the avail ring.
    pub fn pop<'a, 'b: 'a>(&'a mut self, mem: &'b GuestMemoryMmap) -> Option<DescriptorChain<'a>> {
        DescriptorChain::asdf(self.q_mut(mem).pop_descriptor_chain()?)
    }

    /// Undo the effects of the last `self.pop()` call.
    /// The caller can use this, if it was unable to consume the last popped descriptor chain.
    pub fn undo_pop(&mut self) {
        self.next_avail -= Wrapping(1);
    }

    /// Puts an available descriptor head into the used ring for use by the guest.
    pub fn add_used(&mut self, mem: &GuestMemoryMmap, desc_index: u16, len: u32) -> Result<(), ()> {
        self.q_mut(mem).add_used(desc_index, len).map_err(|_| ())
    }
}

pub struct DescriptorChain<'a> {
    chain: Chain<&'a GuestMemoryMmap>,

    /// Index into the descriptor table
    pub index: u16,

    /// Guest physical address of device specific data
    pub addr: GuestAddress,

    /// Length of device specific data
    pub len: u32,

    /// Includes next, write, and indirect bits
    pub flags: u16,

    /// Index into the descriptor table of the next descriptor if flags has
    /// the next bit set
    pub next: u16,
}

impl<'a> DescriptorChain<'a> {
    fn asdf(mut chain: Chain<&'a GuestMemoryMmap>) -> Option<Self> {
        let index = chain.head_index();
        let desc = chain.next()?;

        Some(Self {
            chain,
            index,
            addr: desc.addr(),
            len: desc.len(),
            flags: desc.flags(),
            next: desc.next(),
        })
    }

    fn checked_new(
        mem: &'a GuestMemoryMmap,
        desc_table: GuestAddress,
        queue_size: u16,
        index: u16,
    ) -> Option<DescriptorChain> {
        let mut chain = Chain::new(mem, desc_table, queue_size, index);
        Self::asdf(chain)
    }

    /// Gets if this descriptor chain has another descriptor chain linked after it.
    pub fn has_next(&self) -> bool {
        self.flags & VIRTQ_DESC_F_NEXT != 0 // && self.ttl > 1
    }

    /// If the driver designated this as a write only descriptor.
    ///
    /// If this is false, this descriptor is read only.
    /// Write only means the the emulated device can write and the driver can read.
    pub fn is_write_only(&self) -> bool {
        self.flags & VIRTQ_DESC_F_WRITE != 0
    }

    /// Gets the next descriptor in this descriptor chain, if there is one.
    ///
    /// Note that this is distinct from the next descriptor chain returned by `AvailIter`, which is
    /// the head of the next _available_ descriptor chain.
    pub fn next_descriptor(&self) -> Option<DescriptorChain<'a>> {
        let index = self.next;
        let mut chain = self.chain.clone();
        if let Some(desc) = chain.next() {
            Some(Self {
                chain,
                index,
                addr: desc.addr(),
                len: desc.len(),
                flags: desc.flags(),
                next: desc.next(),
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {

    pub use super::*;

    use crate::virtio::test_utils::VirtQueue;
    // use crate::virtio::QueueError::{DescIndexOutOfBounds, UsedRing};
    use vm_memory::{GuestAddress, GuestMemoryMmap};

    #[test]
    fn test_checked_new_descriptor_chain() {
        let m = &GuestMemoryMmap::from_ranges(&[
            (GuestAddress(0), 0x10000),
            (GuestAddress(0x20000), 0x2000),
        ])
        .unwrap();
        let vq = VirtQueue::new(GuestAddress(0), m, 16);

        assert!(vq.end().0 < 0x1000);

        // index >= queue_size
        assert!(DescriptorChain::checked_new(m, vq.dtable_start(), 16, 16).is_none());

        // desc_table address is way off
        assert!(DescriptorChain::checked_new(m, GuestAddress(0x00ff_ffff_ffff), 16, 0).is_none());

        // Let's create an invalid chain.
        {
            // The first desc has a normal len, and the next_descriptor flag is set.
            vq.dtable[0].addr.set(0x1000);
            vq.dtable[0].len.set(0x1000);
            vq.dtable[0].flags.set(VIRTQ_DESC_F_NEXT);
            // .. but the the index of the next descriptor is too large
            vq.dtable[0].next.set(16);

            // The value of the next index is lazily checked in the upstream impl
            // assert!(DescriptorChain::checked_new(m, vq.dtable_start(), 16, 0).is_none());
        }

        // Finally, let's test an ok chain.
        {
            vq.dtable[0].next.set(1);
            vq.dtable[1].set(0x2000, 0x1000, 0, 0);

            let c = DescriptorChain::checked_new(m, vq.dtable_start(), 16, 0).unwrap();

            // assert_eq!(c.mem as *const GuestMemoryMmap, m as *const GuestMemoryMmap);
            // assert_eq!(c.desc_table, vq.dtable_start());
            // assert_eq!(c.queue_size, 16);
            // assert_eq!(c.ttl, c.queue_size);

            assert_eq!(c.index, 0);
            assert_eq!(c.addr, GuestAddress(0x1000));
            assert_eq!(c.len, 0x1000);
            assert_eq!(c.flags, VIRTQ_DESC_F_NEXT);
            assert_eq!(c.next, 1);

            assert!(c.next_descriptor().unwrap().next_descriptor().is_none());
        }
    }

    #[test]
    fn test_queue_validation() {
        let m = &GuestMemoryMmap::from_ranges(&[(GuestAddress(0), 0x10000)]).unwrap();
        let vq = VirtQueue::new(GuestAddress(0), m, 16);

        let mut q = vq.create_queue();

        // q is currently valid
        assert!(q.is_valid(m));

        // shouldn't be valid when not marked as ready
        q.ready = false;
        assert!(!q.is_valid(m));
        q.ready = true;

        // or when size > max_size
        q.size = q.max_size << 1;
        assert!(!q.is_valid(m));
        q.size = q.max_size;

        // or when size is 0
        q.size = 0;
        assert!(!q.is_valid(m));
        q.size = q.max_size;

        // or when size is not a power of 2
        q.size = 11;
        assert!(!q.is_valid(m));
        q.size = q.max_size;

        // or when avail_idx - next_avail > max_size
        q.next_avail = Wrapping(5);
        assert!(!q.is_valid(m));
        // avail_ring + 2 is the address of avail_idx in guest mem
        m.write_obj::<u16>(64 as u16, q.avail_ring.unchecked_add(2))
            .unwrap();
        assert!(!q.is_valid(m));
        m.write_obj::<u16>(5 as u16, q.avail_ring.unchecked_add(2))
            .unwrap();
        q.max_size = 2;
        assert!(!q.is_valid(m));

        // reset dirtied values
        q.max_size = 16;
        q.next_avail = Wrapping(0);
        m.write_obj::<u16>(0, q.avail_ring.unchecked_add(2))
            .unwrap();

        // or if the various addresses are off

        q.desc_table = GuestAddress(0xffff_ffff);
        assert!(!q.is_valid(m));
        q.desc_table = GuestAddress(0x1001);
        assert!(!q.is_valid(m));
        q.desc_table = vq.dtable_start();

        q.avail_ring = GuestAddress(0xffff_ffff);
        assert!(!q.is_valid(m));
        q.avail_ring = GuestAddress(0x1001);
        assert!(!q.is_valid(m));
        q.avail_ring = vq.avail_start();

        q.used_ring = GuestAddress(0xffff_ffff);
        assert!(!q.is_valid(m));
        q.used_ring = GuestAddress(0x1001);
        assert!(!q.is_valid(m));
        q.used_ring = vq.used_start();
    }

    #[test]
    fn test_queue_processing() {
        let m = &GuestMemoryMmap::from_ranges(&[(GuestAddress(0), 0x10000)]).unwrap();
        let vq = VirtQueue::new(GuestAddress(0), m, 16);
        let mut q = vq.create_queue();

        q.ready = true;

        // Let's create two simple descriptor chains.

        for j in 0..5 {
            vq.dtable[j].set(
                0x1000 * (j + 1) as u64,
                0x1000,
                VIRTQ_DESC_F_NEXT,
                (j + 1) as u16,
            );
        }

        // the chains are (0, 1) and (2, 3, 4)
        vq.dtable[1].flags.set(0);
        vq.dtable[4].flags.set(0);
        vq.avail.ring[0].set(0);
        vq.avail.ring[1].set(2);
        vq.avail.idx.set(2);

        // We've just set up two chains.
        assert_eq!(q.len(m), 2);

        // The first chain should hold exactly two descriptors.
        let d = q.pop(m).unwrap().next_descriptor().unwrap();
        assert!(!d.has_next());
        assert!(d.next_descriptor().is_none());

        // We popped one chain, so there should be only one left.
        assert_eq!(q.len(m), 1);

        // The next chain holds three descriptors.
        let d = q
            .pop(m)
            .unwrap()
            .next_descriptor()
            .unwrap()
            .next_descriptor()
            .unwrap();
        assert!(!d.has_next());
        assert!(d.next_descriptor().is_none());

        // We've popped both chains, so the queue should be empty.
        assert!(q.is_empty(m));
        assert!(q.pop(m).is_none());

        // Undoing the last pop should let us walk the last chain again.
        q.undo_pop();
        assert_eq!(q.len(m), 1);

        // Walk the last chain again (three descriptors).
        let d = q
            .pop(m)
            .unwrap()
            .next_descriptor()
            .unwrap()
            .next_descriptor()
            .unwrap();
        assert!(!d.has_next());
        assert!(d.next_descriptor().is_none());
    }

    /*
    #[test]
    fn test_add_used() {
        let m = &GuestMemoryMmap::from_ranges(&[(GuestAddress(0), 0x10000)]).unwrap();
        let vq = VirtQueue::new(GuestAddress(0), m, 16);

        let mut q = vq.create_queue();
        assert_eq!(vq.used.idx.get(), 0);

        //Valid queue addresses configuration
        {
            //index too large
            match q.add_used(m, 16, 0x1000) {
                Err(DescIndexOutOfBounds(16)) => (),
                _ => unreachable!(),
            }

            //should be ok
            q.add_used(m, 1, 0x1000).unwrap();
            assert_eq!(vq.used.idx.get(), 1);
            let x = vq.used.ring[0].get();
            assert_eq!(x.id, 1);
            assert_eq!(x.len, 0x1000);
        }

        //Invalid queue addresses configuration
        {
            q.used_ring = GuestAddress(0xffff_ffff);
            //writing descriptor index to this ring address should fail
            match q.add_used(m, 1, 0x1000) {
                Err(UsedRing(GuestMemoryError::InvalidGuestAddress(GuestAddress(
                                                                       0x0001_0000_000B,
                                                                   )))) => {}
                _ => unreachable!(),
            }

            q.used_ring = GuestAddress(0xfff0);
            //writing len to this ring address should fail
            match q.add_used(m, 1, 0x1000) {
                Err(UsedRing(GuestMemoryError::InvalidGuestAddress(GuestAddress(0x1_0000)))) => {}
                _ => unreachable!(),
            };
        }
    }

    #[test]
    fn test_queue_error_display() {
        let err = UsedRing(GuestMemoryError::InvalidGuestAddress(GuestAddress(0)));
        let _ = format!("{}{:?}", err, err);

        let err = DescIndexOutOfBounds(1);
        let _ = format!("{}{:?}", err, err);
    }
    */
}
