use std::result;

use serde::de::{Deserialize, MapAccess, SeqAccess};

pub trait IndirectDeserialize<'de, T> {
    type R;

    fn with_deserialize<D: Deserialize<'de> + Into<T>>(&mut self) -> Self::R;
}

// *********

pub(crate) struct SeqIndirect<'a, V>(pub &'a mut V);

impl<'a, 'de: 'a, T, V: SeqAccess<'de>> IndirectDeserialize<'de, T> for SeqIndirect<'a, V> {
    type R = result::Result<Option<T>, V::Error>;

    fn with_deserialize<D: Deserialize<'de> + Into<T>>(&mut self) -> Self::R {
        self.0.next_element::<D>().map(|a| a.map(|b| b.into()))
    }
}

// *********

pub(crate) struct MapIndirect<'a, V>(pub &'a mut V);

impl<'a, 'de: 'a, T, V: MapAccess<'de>> IndirectDeserialize<'de, T> for MapIndirect<'a, V> {
    type R = result::Result<T, V::Error>;

    fn with_deserialize<D: Deserialize<'de> + Into<T>>(&mut self) -> Self::R {
        self.0.next_value::<D>().map(|a| a.into())
    }
}
