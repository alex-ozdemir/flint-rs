extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=flint");

    // Use INCLUDE_DIR env variable to pass flint include dir if clang cant find it
    let mut include_arg = String::new();
    if let Ok(include_dir) = env::var("INCLUDE_DIR") {
        include_arg = format!("-I{}", include_dir);
    }

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args([include_arg])
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .blocklist_item("FP_NAN")
        .blocklist_item("FP_INFINITE")
        .blocklist_item("FP_ZERO")
        .blocklist_item("FP_SUBNORMAL")
        .blocklist_item("FP_NORMAL")
        .layout_tests(false)
        //.generate_inline_functions(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
