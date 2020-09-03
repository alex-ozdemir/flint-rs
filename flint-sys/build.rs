fn main() {
    // Tell cargo to tell rustc to link the system flint
    // shared library.
    println!("cargo:rustc-link-lib=flint");
}

