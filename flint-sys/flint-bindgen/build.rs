extern crate bindgen;

use std::env;
use std::path::PathBuf;

include!("scripts/bindgen_headers.rs");


fn get_imports(header: &str) -> String {
    let headers = FLINT_HEADERS.into_iter().filter(|&h| h != &header);
    let mut out = String::from("use libc::*;\n");
    out += "use crate::deps::*;\n";
    out += "use crate::bindgen::*;\n";

    for h in headers {
        let temp = h.split(".")
            .next()
            .expect("Error making import statement")
            .replace("-", "_");
        out += &format!("use crate::{}::*;\n", temp);
    }
    return out
}

fn generate_bindings(header: &str, include_path: &PathBuf, out_path: &PathBuf) {
    let include_arg = format!("-I{}", include_path.display());
    let include_fp = include_path.join("flint").join(header);
    
    let mut out_fp = out_path.join(header);
    let mut extern_out_fp = out_path.join("extern").join(header);
    out_fp.set_extension("rs");
    extern_out_fp.set_extension("c");

    let imports = get_imports(header);
    
    let bindings = bindgen::Builder::default()
        .header(include_fp.to_string_lossy())
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .wrap_static_fns(true)
        .wrap_static_fns_path(extern_out_fp)
        .allowlist_file(include_fp.to_string_lossy())
        .allowlist_recursively(false)
        .clang_arg(include_arg)
        .raw_line(imports)
        .generate_cstr(true)
        .ctypes_prefix("libc")
        .c_naming(false)
        .derive_debug(true)
        .derive_copy(true)
        .derive_default(true)
        .merge_extern_blocks(true)
        .blocklist_type("slong")
        .blocklist_type("ulong")
        .blocklist_item("FP_NAN")
        .blocklist_item("FP_NAN")
        .blocklist_item("FP_INFINITE")
        .blocklist_item("FP_ZERO")
        .blocklist_item("FP_SUBNORMAL")
        .blocklist_item("FP_NORMAL")
        .blocklist_function("flint_vprintf")
        .blocklist_function("flint_vfprintf")
        .blocklist_function("flint_va_end")
        .blocklist_function("flint_set_throw")
        .generate()
        .expect(&format!("Unable to generate bindings for {}", header));

    bindings.write_to_file(out_fp).expect(&format!("Unable to write bindings for {}", header));
}

fn main() {
    println!("cargo:rustc-link-lib=flint");
        
    // Use INCLUDE_DIR env variable to pass flint include dir if needed
    let include_path = PathBuf::from(env::var("INCLUDE_DIR")
        .expect("Environment variable INCLUDE_DIR is not defined."));
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    for h in FLINT_HEADERS {
        generate_bindings(h, &include_path, &out_path);
    }
}
