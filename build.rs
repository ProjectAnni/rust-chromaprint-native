extern crate cmake;

use std::env;

fn main() {
    let search = if let Some(lib_dir) = env::var("CHROMAPRINT_LIB_DIR").ok() {
        lib_dir
    } else {
        cmake::build("chromaprint").display().to_string() + "/lib"
    };

    println!("cargo:rerun-if-changed=CHROMAPRINT_LIB_DIR");
    println!("cargo:rustc-link-search=native={}", search);
    println!("cargo:rustc-link-lib=chromaprint");
}
