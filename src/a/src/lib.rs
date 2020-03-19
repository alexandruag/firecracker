use a2::A2;
use snapshot::{Result, Versionize};
use snapshot_derive::Versionize;

include!(concat!(env!("OUT_DIR"), "/version_support.rs"));

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
