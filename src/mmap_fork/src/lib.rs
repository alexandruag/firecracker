mod mmap;

// I may have missed some exports :-s
pub use mmap::{GuestMemoryMmap, GuestRegionMmap};

// TODO: re-export things from upstream so we don't need to import both this crate and the real
// vm-memory locally and consume the wrong `GuestMemoryMmap`.