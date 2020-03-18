use std::any::TypeId;
use std::collections::HashSet;

use lazy_static::lazy_static;

use snapshot::version_map::VersionMap;
use snapshot::{Result, Versionize};
use snapshot_derive::Versionize;

lazy_static! {
    static ref VERSION_MAP: VersionMap = {
        {
            let mut vm = VersionMap::new();
            vm.new_version()
                .set_type_version(TypeId::of::<A2>(), 1)
                .new_version()
                .set_type_version(TypeId::of::<A2>(), 2);
            vm
        }
    };
    static ref FOREIGN_TYPES: HashSet<TypeId> = { HashSet::new() };
}

#[derive(Versionize, Clone, Debug, Default, PartialEq)]
pub struct A2 {
    _s: String,
}
