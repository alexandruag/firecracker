// Copyright 2020 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Maps struct/enum/union versions to a sequence of root versions.
//! This is required to support the concept of a snapshot version
//! composed of individually versioned components.

use std::collections::hash_map::HashMap;

const BASE_VERSION: u16 = 1;

///
/// The VersionMap API provides functionality to define the version for each serialized
/// type and attach them to specific root versions.
///
/// !TODO: Find an O(1) solution for `get_type_version()`.
///
#[derive(Clone, Debug, Default)]
pub struct VersionMap {
    versions: Vec<HashMap<&'static str, u16>>,
}

impl VersionMap {
    /// Create a new version map and set root version to 1.
    pub fn new() -> Self {
        VersionMap {
            versions: vec![HashMap::new(); 1],
        }
    }

    /// Bumps root version by 1 to create a new root version and set it as latest version.
    pub fn new_version(&mut self) -> &mut Self {
        self.versions.push(HashMap::new());
        self
    }

    /// Define a mapping between a specific type version and the latest root version.
    pub fn set_type_version(&mut self, type_name: &'static str, type_version: u16) -> &mut Self {
        let current_version = self.versions.len();
        self.versions[current_version - 1].insert(type_name, type_version);
        self
    }

    /// Returns the version of `type_id` corresponding to the specified `root_version`.
    pub fn get_type_version(&self, root_version: u16, type_name: &'static str) -> u16 {
        let version_space = self.versions.split_at(root_version as usize).0;

        for i in (0..version_space.len()).rev() {
            if let Some(version) = version_space[i].get(type_name) {
                return *version;
            }
        }

        BASE_VERSION
    }

    /// Returns the latest top version.
    pub fn latest_version(&self) -> u16 {
        self.versions.len() as u16
    }

    /// TODO: add doc
    pub fn type_history(&self, type_name: &'static str) -> Vec<u16> {
        let mut current_version = BASE_VERSION;
        let mut v = Vec::new();

        for h in self.versions.iter() {
            current_version = *h.get(type_name).unwrap_or(&current_version);
            v.push(current_version);
        }

        assert_eq!(v.len(), self.versions.len());
        v
    }

    /// TODO: add doc
    pub fn type_names(&self) -> Vec<&'static str> {
        let mut names = Vec::new();

        for h in self.versions.iter() {
            for k in h.keys() {
                if !names.contains(k) {
                    names.push(*k);
                }
            }
        }
        names
    }
}

/// TODO: add doc
pub fn generate_foreign_type_fn(test_types: &[&str], names: &[&str]) -> String {
    let mut s = String::from(
        "
        pub(crate) fn is_foreign_type(_tid: std::any::TypeId) -> bool {
    ",
    );

    for name in names {
        if test_types.contains(&name) {
            s.push_str("#[cfg(test)]");
        }

        s.push_str(&format!(
            "{{
                if _tid == std::any::TypeId::of::<{}>() {{
                    return true;
                }}
            }}
        ",
            name
        ));
    }

    s.push_str(
        "
            false
        }",
    );
    s
}

/// TODO: add doc
pub fn generate_type_version_fn(test_types: &[&str], m: &VersionMap) -> String {
    let names = m.type_names();

    let mut s = String::from(
        "
        pub(crate) fn type_version(_tid: std::any::TypeId, _app_version: u16) -> u16 {
    ",
    );

    for name in names {
        if test_types.contains(&name) {
            s.push_str("#[cfg(test)]");
        }

        s.push_str(&format!(
            "{{
            if _tid == std::any::TypeId::of::<{}>() {{
                let mapping = &{:?};
                return mapping[(_app_version - 1) as usize];
            }}
        }}
        ",
            name,
            m.type_history(name)
        ));
    }

    s.push_str(&format!(" return {}; }}", BASE_VERSION));
    s
}

#[cfg(test)]
mod test {
    use super::{VersionMap, BASE_VERSION};

    pub struct MyType;
    pub struct MySecondType;
    pub struct MyThirdType;

    #[test]
    fn test_default_version() {
        let vm = VersionMap::new();
        assert_eq!(vm.latest_version(), 1);
    }

    #[test]
    fn test_new_versions() {
        let mut vm = VersionMap::new();
        vm.new_version().new_version();
        assert_eq!(vm.latest_version(), 3);
    }

    #[test]
    fn test_1_app_version() {
        let mut vm = VersionMap::new();
        vm.set_type_version("MyType", 1);
        vm.set_type_version("MySecondType", 2);
        vm.set_type_version("MyThirdType", 3);

        assert_eq!(vm.get_type_version(1, "MyType"), 1);
        assert_eq!(vm.get_type_version(1, "MySecondType"), 2);
        assert_eq!(vm.get_type_version(1, "MyThirdType"), 3);
    }

    #[test]
    fn test_100_app_version_full() {
        let mut vm = VersionMap::new();

        for i in 1..=100 {
            vm.set_type_version("MyType", i)
                .set_type_version("MySecondType", i + 1)
                .set_type_version("MyThirdType", i + 2)
                .new_version();
        }

        for i in 1..=100 {
            assert_eq!(vm.get_type_version(i, "MyType"), i);
            assert_eq!(vm.get_type_version(i, "MySecondType"), i + 1);
            assert_eq!(vm.get_type_version(i, "MyThirdType"), i + 2);
        }
    }

    #[test]
    fn test_app_versions_with_gap() {
        let my_type_id = "MyType";
        let my_second_type_id = "MySecondType";
        let my_third_type_id = "MyThirdType";

        let mut vm = VersionMap::new();
        vm.set_type_version(my_type_id, 1);
        vm.set_type_version(my_second_type_id, 1);
        vm.set_type_version(my_third_type_id, 1);
        vm.new_version();
        vm.set_type_version(my_type_id, 2);
        vm.new_version();
        vm.set_type_version(my_third_type_id, 2);
        vm.new_version();
        vm.set_type_version(my_second_type_id, 2);

        assert_eq!(vm.get_type_version(1, my_type_id), 1);
        assert_eq!(vm.get_type_version(1, my_second_type_id), 1);
        assert_eq!(vm.get_type_version(1, my_third_type_id), 1);

        assert_eq!(vm.get_type_version(2, my_type_id), 2);
        assert_eq!(vm.get_type_version(2, my_second_type_id), 1);
        assert_eq!(vm.get_type_version(2, my_third_type_id), 1);

        assert_eq!(vm.get_type_version(3, my_type_id), 2);
        assert_eq!(vm.get_type_version(3, my_second_type_id), 1);
        assert_eq!(vm.get_type_version(3, my_third_type_id), 2);

        assert_eq!(vm.get_type_version(4, my_type_id), 2);
        assert_eq!(vm.get_type_version(4, my_second_type_id), 2);
        assert_eq!(vm.get_type_version(4, my_third_type_id), 2);
    }

    #[test]
    fn test_unset_type() {
        let vm = VersionMap::new();
        assert_eq!(vm.get_type_version(1, "MyType"), BASE_VERSION);
    }
}
