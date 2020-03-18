use std::any::TypeId;
use std::collections::HashSet;

use lazy_static::lazy_static;

use a2::A2;
use snapshot::version_map::VersionMap;
use snapshot::{Result, Versionize};
use snapshot_derive::Versionize;

lazy_static! {
    static ref VERSION_MAP: VersionMap = {
        {
            let mut vm = VersionMap::new();
            vm.new_version()
                .set_type_version(TypeId::of::<OneLocalStruct>(), 2)
                .set_type_version(TypeId::of::<A2>(), 1)
                .new_version()
                .set_type_version(TypeId::of::<OneLocalStruct>(), 2)
                .set_type_version(TypeId::of::<A2>(), 2);
            vm
        }
    };
    static ref FOREIGN_TYPES: HashSet<TypeId> = {
        let mut h = HashSet::new();
        h.insert(TypeId::of::<A2>());
        h
    };
}

#[derive(Versionize, Clone, Debug, Default, PartialEq)]
struct OneLocalStruct {
    _x: u32,
}

#[derive(Versionize, Clone, Debug, Default, PartialEq)]
pub struct A {
    _local: OneLocalStruct,
    _a2: A2,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        let a = A::default();
        let mut v = Vec::new();

        a.serialize(&mut v, 1).unwrap();
        assert_eq!(a, A::deserialize(&mut v.as_slice(), 1).unwrap());
    }
}
