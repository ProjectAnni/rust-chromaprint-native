extern crate cmake;

use std::env;

fn main() {
    let dst = cmake::build("chromaprint");
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=chromaprint");
}
