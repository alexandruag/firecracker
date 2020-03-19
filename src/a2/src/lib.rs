use snapshot::{Result, Versionize};
use snapshot_derive::Versionize;

include!(concat!(env!("OUT_DIR"), "/version_support.rs"));

#[derive(Versionize, Clone, Debug, Default, PartialEq)]
pub struct A2 {
    _s: String,
}
