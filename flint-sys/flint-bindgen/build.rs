extern crate bindgen;

use std::env;
use std::path::PathBuf;

const FLINT_HEADERS: &[&str] = &[
    "acb.h",
    "acb_calc.h",
    "acb_dft.h",
    "acb_dirichlet.h",
    "acb_elliptic.h",
    "acb_hypgeom.h",
    "acb_mat.h",
    "acb_modular.h",
    "acb_poly.h",
    "acb_theta.h",
    "acb_types.h",
    "acf.h",
    "acf_types.h",
    "aprcl.h",
    "arb.h",
    "arb_calc.h",
    "arb_fmpz_poly.h",
    "arb_fpwrap.h",
    "arb_hypgeom.h",
    "arb_mat.h",
    "arb_poly.h",
    "arb_types.h",
    "arf.h",
    "arf_types.h",
    "arith.h",
    "bernoulli.h",
    "bool_mat.h",
    "ca.h",
    "ca_ext.h",
    "ca_field.h",
    "ca_mat.h",
    "ca_poly.h",
    "ca_vec.h",
    "calcium.h",
    "d_mat.h",
    "d_vec.h",
    "dirichlet.h",
    "dlog.h",
    "double_extras.h",
    "double_interval.h",
    "fexpr.h",
    "fexpr_builtin.h",
    "fft.h",
    "fft_tuning.h",
    "flint-config.h",
    "flint.h",
    "fmpq.h",
    "fmpq_mat.h",
    "fmpq_mpoly.h",
    "fmpq_mpoly_factor.h",
    "fmpq_poly.h",
    "fmpq_types.h",
    "fmpq_vec.h",
    "fmpz.h",
    "fmpz_extras.h",
    "fmpz_factor.h",
    "fmpz_lll.h",
    "fmpz_mat.h",
    "fmpz_mod.h",
    "fmpz_mod_mat.h",
    "fmpz_mod_mpoly.h",
    "fmpz_mod_mpoly_factor.h",
    "fmpz_mod_poly.h",
    "fmpz_mod_poly_factor.h",
    "fmpz_mod_types.h",
    "fmpz_mod_vec.h",
    "fmpz_mpoly.h",
    "fmpz_mpoly_factor.h",
    "fmpz_mpoly_q.h",
    "fmpz_poly.h",
    "fmpz_poly_factor.h",
    "fmpz_poly_mat.h",
    "fmpz_poly_q.h",
    "fmpz_types.h",
    "fmpz_vec.h",
    "fmpzi.h",
    "fq.h",
    "fq_default.h",
    "fq_default_mat.h",
    "fq_default_poly.h",
    "fq_default_poly_factor.h",
    "fq_embed.h",
    "fq_embed_templates.h",
    "fq_mat.h",
    "fq_mat_templates.h",
    "fq_nmod.h",
    "fq_nmod_embed.h",
    "fq_nmod_mat.h",
    "fq_nmod_mpoly.h",
    "fq_nmod_mpoly_factor.h",
    "fq_nmod_poly.h",
    "fq_nmod_poly_factor.h",
    "fq_nmod_types.h",
    "fq_nmod_vec.h",
    "fq_poly.h",
    "fq_poly_factor.h",
    "fq_poly_factor_templates.h",
    "fq_poly_templates.h",
    "fq_templates.h",
    "fq_types.h",
    "fq_vec.h",
    "fq_vec_templates.h",
    "fq_zech.h",
    "fq_zech_embed.h",
    "fq_zech_mat.h",
    "fq_zech_mpoly.h",
    "fq_zech_mpoly_factor.h",
    "fq_zech_poly.h",
    "fq_zech_poly_factor.h",
    "fq_zech_types.h",
    "fq_zech_vec.h",
    "gmpcompat.h",
    "gr.h",
    "gr_generic.h",
    "gr_mat.h",
    "gr_mpoly.h",
    "gr_poly.h",
    "gr_special.h",
    "gr_vec.h",
    "hypgeom.h",
    "limb_types.h",
    "long_extras.h",
    "longlong.h",
    "mag.h",
    "mpf-impl.h",
    "mpfr_mat.h",
    "mpfr_vec.h",
    "mpn_extras.h",
    "mpoly.h",
    "mpoly_types.h",
    "n_poly.h",
    "n_poly_types.h",
    "nf.h",
    "nf_elem.h",
    "nmod.h",
    "nmod_mat.h",
    "nmod_mpoly.h",
    "nmod_mpoly_factor.h",
    "nmod_poly.h",
    "nmod_poly_factor.h",
    "nmod_poly_mat.h",
    "nmod_types.h",
    "nmod_vec.h",
    "padic.h",
    "padic_mat.h",
    "padic_poly.h",
    "padic_types.h",
    "partitions.h",
    "perm.h",
    "profiler.h",
    "qadic.h",
    "qfb.h",
    "qqbar.h",
    "qsieve.h",
    "templates.h",
    "thread_pool.h",
    "thread_support.h",
    "ulong_extras.h",
];

const COMMON: &str = 
    "#![allow(non_camel_case_types)]\n\
    use crate::deps::*;\n\
    use libc::{c_char, c_int, c_uint, c_void, size_t, FILE};\n";

fn generate_bindings(header: &str, include_path: &PathBuf, out_path: &PathBuf) -> Result<(), bindgen::BindgenError> {
    let include_arg = format!("-I{}", include_path.display());
    let include_fp = include_path.join("flint").join(header);
    let mut out_fp = out_path.join(header);
    out_fp.set_extension("rs");
    
    let bindings = bindgen::Builder::default()
        .header(include_fp.to_string_lossy())
        .allowlist_file(include_fp.to_string_lossy())
        .allowlist_recursively(false)
        .clang_args([include_arg])
        .raw_line(COMMON)
        .generate_cstr(true)
        .derive_debug(true)
        .derive_copy(true)
        .derive_default(true)
        //.impl_debug(true)
        //.derive_default(true)
        //.derive_hash(true)
        .merge_extern_blocks(true)
        .blocklist_item("FP_NAN")
        .blocklist_item("FP_INFINITE")
        .blocklist_item("FP_ZERO")
        .blocklist_item("FP_SUBNORMAL")
        .blocklist_item("FP_NORMAL")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()?;
        //.expect("Unable to generate bindings");

    bindings.write_to_file(out_fp).expect("Couldn't write bindings!");
    Ok(())
}

fn main() {
    println!("cargo:rustc-link-lib=flint");
        
    // Use INCLUDE_DIR env variable to pass flint include dir if needed
    let include_path = PathBuf::from(env::var("INCLUDE_DIR").unwrap());
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let mut ok = true;
    for h in FLINT_HEADERS {
        ok = ok && generate_bindings(h, &include_path, &out_path).is_ok();
    }

    if !ok {
        panic!("Error generating bindings!")
    }
}
