use std::env;
use std::fs;
use std::path::Path;

use snapshot::version_map::{generate_foreign_type_fn, generate_type_version_fn, VersionMap};

fn main() {
    // Here goes version map and type configuration.

    let mut vm = VersionMap::new();

    vm.new_version()
        .set_type_version("A2", 1)
        .new_version()
        .set_type_version("A2", 2);

    let foreign_types = &[];
    let test_types = &[];

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
