extern crate cc;
extern crate cmake;

fn main() {
    let dst = cmake::build("chromaprint");

    cc::Build::new().compile("chromaprint");

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=chromaprint");
}
