use std::env;
use std::fs;
use std::path::Path;

use std::collections::hash_map::HashMap;

// !!!!!!!!!!!!!!!!!
// !!!!!!!!!!!!!!!!!
// !!!!!!!!!!!!!!!!!
// !!!! TEMPORARY HACK BECAUSE I CAN'T FIGURE OUT HOW TO USE STUFF FROM
// !!!! THE CURRENT CRATE IN THE BUILD.RS SCRIPT

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

fn main() {
    // Here goes version map and type configuration.

    let mut vm = VersionMap::new();
    vm.new_version()
        // .set_type_version("primitives::tests::Message", 1)
        .set_type_version("tests::A", 2)
        .set_type_version("tests::Test", 2)
        .new_version()
        .set_type_version("tests::A", 3)
        .set_type_version("tests::Test", 3)
        .new_version()
        .set_type_version("tests::Test", 4);

    let foreign_types = &[];
    let test_types = &["tests::A", "tests::Test"];

    // End of configuration. Generate support code.

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("version_support.rs");
    fs::write(
        &dest_path,
        format!(
            "{}\n{}",
            generate_type_version_fn(test_types, &vm),
            generate_foreign_type_fn(test_types, foreign_types)
        ),
    )
    .unwrap();
}
