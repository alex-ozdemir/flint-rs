extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=flint");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        /*
        .clang_args([
            "-fno-inline-functions",
            "-DFLINT_INLINES_C",
            "-DPERM_INLINES_C",
            "-DMPOLY_INLINES_C",
            "-DULONG_EXTRAS_INLINES_C",
            "-DFMPZ_INLINES_C",
            "-DFMPZ_VEC_INLINES_C",
            "-DFMPZ_FACTOR_INLINES_C",
            "-DFMPZ_MAT_INLINES_C",
            "-DFMPZ_LLL_INLINES_C",
            "-DFMPZ_POLY_INLINES_C",
            "-DFMPZ_POLY_MAT_INLINES_C",
            "-DFMPZ_POLY_FACTOR_INLINES_C",
            "-DFMPZ_MPOLY_INLINES_C",
            "-DFMPZ_MPOLY_FACTOR_INLINES_C",
            "-DLONG_EXTRAS_INLINES_C",
            "-DMPN_EXTRAS_INLINES_C",
            "-DAPRCL_INLINES_C",
            "-DARITH_INLINES_C",
            "-DFFT_INLINES_C",
            "-DQSIEVE_INLINES_C",
            "-DFMPQ_INLINES_C",
            "-DFMPQ_VEC_INLINES_C",
            "-DFMPQ_MAT_INLINES_C",
            "-DFMPQ_POLY_INLINES_C",
            "-DFMPQ_MPOLY_FACTOR_INLINES_C",
            "-DFMPQ_MPOLY_INLINES_C",
            "-DFMPZ_POLY_Q_INLINES_C",
            "-DNMOD_VEC_INLINES_C",
            "-DNMOD_MAT_INLINES_C",
            "-DNMOD_POLY_INLINES_C",
            "-DNMOD_POLY_MAT_INLINES_C",
            "-DNMOD_POLY_FACTOR_INLINES_C",
            "-DN_POLY_INLINES_C",
            "-DNMOD_MPOLY_INLINES_C",
            "-DNMOD_MPOLY_FACTOR_INLINES_C",
            "-DFMPZ_MOD_INLINES_C",
            "-DFMPZ_MOD_VEC_INLINES_C",
            "-DFMPZ_MOD_POLY_INLINES_C",
            "-DFMPZ_MOD_POLY_FACTOR_INLINES_C",
            "-DFMPZ_MOD_MPOLY_INLINES_C",
            "-DFMPZ_MOD_MPOLY_FACTOR_INLINES_C",
            "-DFMPZ_MOD_MAT_INLINES_C",
            "-DFQ_INLINES_C",
            "-DFQ_EMBED_INLINES_C",
            "-DFQ_VEC_INLINES_C",
            "-DFQ_MAT_INLINES_C",
            "-DFQ_NMOD_INLINES_C",
            "-DFQ_NMOD_MPOLY_INLINES_C",
            "-DFQ_NMOD_MPOLY_FACTOR_INLINES_C",
            "-DFQ_NMOD_EMBED_INLINES_C",
            "-DFQ_NMOD_VEC_INLINES_C",
            "-DFQ_NMOD_MAT_INLINES_C",
            "-DFQ_NMOD_POLY_INLINES_C",
            "-DFQ_NMOD_POLY_FACTOR_INLINES_C",
            "-DFQ_ZECH_INLINES_C",
            "-DFQ_ZECH_EMBED_INLINES_C",
            "-DFQ_ZECH_VEC_INLINES_C",
            "-DFQ_ZECH_MAT_INLINES_C",
            "-DFQ_ZECH_POLY_INLINES_C",
            "-DFQ_ZECH_MPOLY_INLINES_C",
            "-DFQ_ZECH_MPOLY_FACTOR_INLINES_C",
            "-DFQ_DEFAULT_INLINES_C",
            "-DFQ_DEFAULT_EMBED_INLINES_C",
            "-DFQ_DEFAULT_MAT_INLINES_C",
            "-DFQ_DEFAULT_POLY_INLINES_C",
            "-DFQ_DEFAULT_POLY_FACTOR_INLINES_C",
            "-DFQ_POLY_INLINES_C",
            "-DFQ_POLY_FACTOR_INLINES_C",
            "-DFQ_NMOD_MPOLY_INLINES_C",
            "-DFQ_NMOD_MPOLY_FACTOR_INLINES_C",
            "-DFQ_ZECH_POLY_FACTOR_INLINES_C",
            "-DFQ_ZECH_EMBED_INLINES_C",
            "-DPADIC_INLINES_C",
            "-DPADIC_POLY_INLINES_C",
            "-DPADIC_MAT_INLINES_C",
            "-DQADIC_INLINES_C"
                ])
                */
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
