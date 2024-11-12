
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)] // remove

pub mod deps;
pub mod flint;

pub mod fmpz_types;
pub mod fmpz;
pub mod fmpz_extras;
pub mod fmpzi;
pub mod fmpz_factor;
pub mod fmpz_lll;
pub mod fmpz_poly;
pub mod fmpz_poly_factor;
pub mod fmpz_poly_mat;
pub mod fmpz_poly_q;
pub mod fmpz_mat;
pub mod fmpz_vec;

pub mod fmpq_types;
pub mod fmpq;
pub mod fmpq_poly;
pub mod fmpq_mat;
pub mod fmpq_vec;

pub mod nmod_types;
pub mod nmod;
pub mod nmod_poly;
pub mod nmod_mat;
pub mod nmod_poly_factor;
pub mod nmod_poly_mat;
pub mod nmod_vec;

pub mod fmpz_mod_types;
pub mod fmpz_mod;
pub mod fmpz_mod_mat;
pub mod fmpz_mod_poly;
pub mod fmpz_mod_poly_factor;
pub mod fmpz_mod_vec;

pub mod fq_types;
pub mod fq;
pub mod fq_embed;
pub mod fq_mat;
pub mod fq_poly;
pub mod fq_poly_factor;

pub mod fq_nmod_types;
pub mod fq_nmod;
pub mod fq_nmod_embed;
pub mod fq_nmod_mat;
pub mod fq_nmod_poly;
pub mod fq_nmod_poly_factor;

pub mod fq_zech_types;
pub mod fq_zech;
pub mod fq_zech_embed;
pub mod fq_zech_mat;
pub mod fq_zech_poly;
pub mod fq_zech_poly_factor;

pub mod fq_default;
pub mod fq_default_mat;
pub mod fq_default_poly;
pub mod fq_default_poly_factor;

//pub mod padic;
//pub mod padic_types;
//pub mod padic_mat;
//pub mod padic_poly;
//pub mod qadic;

pub mod gr;
pub mod gr_generic;
pub mod gr_poly;
pub mod gr_mat;
pub mod gr_special;
pub mod gr_vec;

//pub mod mpoly_types;
//pub mod mpoly;
//pub mod fmpz_mpoly;
//pub mod fmpz_mpoly_factor;
//pub mod fmpz_mpoly_q;
//pub mod nmod_mpoly;
//pub mod nmod_mpoly_factor;
//pub mod fmpz_mod_mpoly;
//pub mod fmpz_mod_mpoly_factor;
//pub mod fmpq_mpoly;
//pub mod fmpq_mpoly_factor;
//pub mod fq_nmod_mpoly;
//pub mod fq_nmod_mpoly_factor;
//pub mod fq_zech_mpoly;
//pub mod fq_zech_mpoly_factor;
//pub mod gr_mpoly;

pub mod n_poly_types;
//pub mod n_poly; needs fq_nmod

pub mod d_mat;
pub mod mpf_impl;
pub mod limb_types;
pub mod arith;
pub mod fft;
pub mod fft_tuning;
pub mod perm;
//pub mod qsieve;
//pub mod long_extras;
//pub mod ulong_extras;

//pub mod profiler;
//pub mod templates;
//pub mod thread_pool;
//pub mod thread_support;
//pub mod mpfr_mat;
//pub mod mpfr_vec;
//pub mod mpn_extras;




#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::MaybeUninit;
    
    #[test]
    fn test_fmpz() { 
        let mut p: fmpz::fmpz = fmpz::fmpz::default();
        unsafe{
            fmpz::fmpz_init(&mut p);
            fmpz::fmpz_set_ui(&mut p, 17);
            debug_assert!(fmpz::fmpz_print(&mut p) > 0);            
            fmpz::fmpz_clear(&mut p);
        }
    }
}

/*
use libc::{c_int, c_ulong};

/// Bindings for the [FLINT](http://flintlib.org/sphinx/index.html) library.







// Arb //


pub mod mag;
pub mod arf;

pub mod arb;
pub mod arb_calc;
pub mod arb_fmpz_poly;
//pub mod arb_fpwrap;
pub mod arb_hypgeom;
pub mod arb_mat;
pub mod arb_poly;
//pub mod arb_types;

//pub mod acf;
//pub mod acf_types;

pub mod acb;
//pub mod acb_types;
pub mod acb_calc;
pub mod acb_dft;
pub mod acb_dirichlet;
pub mod acb_elliptic;
pub mod acb_hypgeom;
pub mod acb_mat;
pub mod acb_modular;
pub mod acb_poly;
//pub mod acb_theta;

//pub mod bernoulli;
pub mod bool_mat;

//pub mod d_mat;
//pub mod d_vec;

pub mod dirichlet;
pub mod dlog;
//pub mod double_extras;
//pub mod double_interval;

//pub mod hypgeom;
//pub mod limb_types;
//pub mod longlong;
//pub mod partition;

// Calcium //


/*
pub mod ca;
pub mod ca_ext;
pub mod ca_field;
pub mod ca_mat;
pub mod ca_poly;
pub mod ca_vec;
pub mod calcium;
//pub mod qqbar;
//pub mod fexpr;
//pub mod fexpr_builtin;

*/


// Antic //


pub mod nf;
pub mod nf_elem;
pub mod qfb;


// (some) FLINT constants // 

pub const FMPZ_MOD_MAT_MUL_TRANSPOSE_CUTOFF: u32 = 10;
pub const FMPZ_MOD_POLY_HGCD_CUTOFF: u32 = 128;
pub const FMPZ_MOD_POLY_GCD_CUTOFF: u32 = 256;
pub const FMPZ_MOD_POLY_INV_NEWTON_CUTOFF: u32 = 64;
pub const FMPZ_MOD_POLY_EVALUATE_FMPZ_VEC: u32 = 32;
pub const GR_SUCCESS: u32 = 0;
pub const GR_DOMAIN: u32 = 1;
pub const GR_UNABLE: u32 = 2;
pub const GR_TEST_FAIL: u32 = 4;
pub const GR_TMP_VEC_ALLOC_MAX_STACK: u32 = 1024;
pub const GR_TEST_VERBOSE: u32 = 8;
pub const GR_TEST_ALWAYS_ABLE: u32 = 16;
pub const FQ_DEFAULT_FQ_ZECH: u32 = 1;
pub const FQ_DEFAULT_FQ_NMOD: u32 = 2;
pub const FQ_DEFAULT_FQ: u32 = 3;
pub const FQ_DEFAULT_NMOD: u32 = 4;
pub const FQ_DEFAULT_FMPZ_MOD: u32 = 5;
pub const FQ_MAT_SOLVE_TRI_ROWS_CUTOFF: u32 = 64;
pub const FQ_MAT_SOLVE_TRI_COLS_CUTOFF: u32 = 64;
pub const FQ_MAT_LU_RECURSIVE_CUTOFF: u32 = 4;
pub const FQ_NMOD_MAT_SOLVE_TRI_ROWS_CUTOFF: u32 = 64;
pub const FQ_NMOD_MAT_SOLVE_TRI_COLS_CUTOFF: u32 = 64;
pub const FQ_NMOD_MAT_LU_RECURSIVE_CUTOFF: u32 = 4;
pub const FQ_ZECH_MAT_SOLVE_TRI_ROWS_CUTOFF: u32 = 64;
pub const FQ_ZECH_MAT_SOLVE_TRI_COLS_CUTOFF: u32 = 64;
pub const FQ_ZECH_MAT_LU_RECURSIVE_CUTOFF: u32 = 4;
pub const FQ_POLY_DIVREM_DIVCONQUER_CUTOFF: u32 = 16;
pub const FQ_COMPOSE_MOD_LENH_CUTOFF: u32 = 6;
pub const FQ_COMPOSE_MOD_PREINV_LENH_CUTOFF: u32 = 6;
pub const FQ_MUL_CLASSICAL_CUTOFF: u32 = 6;
pub const FQ_MULLOW_CLASSICAL_CUTOFF: u32 = 6;
pub const FQ_SQR_CLASSICAL_CUTOFF: u32 = 6;
pub const FQ_POLY_HGCD_CUTOFF: u32 = 30;
pub const FQ_POLY_SMALL_GCD_CUTOFF: u32 = 80;
pub const FQ_POLY_GCD_CUTOFF: u32 = 90;
pub const FQ_NMOD_POLY_DIVREM_DIVCONQUER_CUTOFF: u32 = 16;
pub const FQ_NMOD_COMPOSE_MOD_LENH_CUTOFF: u32 = 6;
pub const FQ_NMOD_COMPOSE_MOD_PREINV_LENH_CUTOFF: u32 = 6;
pub const FQ_NMOD_MUL_CLASSICAL_CUTOFF: u32 = 6;
pub const FQ_NMOD_SQR_CLASSICAL_CUTOFF: u32 = 6;
pub const FQ_NMOD_MULLOW_CLASSICAL_CUTOFF: u32 = 6;
pub const FQ_NMOD_POLY_HGCD_CUTOFF: u32 = 25;
pub const FQ_NMOD_POLY_SMALL_GCD_CUTOFF: u32 = 110;
pub const FQ_NMOD_POLY_GCD_CUTOFF: u32 = 120;
pub const FQ_ZECH_POLY_DIVREM_DIVCONQUER_CUTOFF: u32 = 16;
pub const FQ_ZECH_COMPOSE_MOD_LENH_CUTOFF: u32 = 6;
pub const FQ_ZECH_COMPOSE_MOD_PREINV_LENH_CUTOFF: u32 = 6;
pub const FQ_ZECH_SQR_CLASSICAL_CUTOFF: u32 = 100;
pub const FQ_ZECH_MUL_CLASSICAL_CUTOFF: u32 = 90;
pub const FQ_ZECH_MULLOW_CLASSICAL_CUTOFF: u32 = 90;
pub const FQ_ZECH_POLY_HGCD_CUTOFF: u32 = 35;
pub const FQ_ZECH_POLY_GCD_CUTOFF: u32 = 96;
pub const FQ_ZECH_POLY_SMALL_GCD_CUTOFF: u32 = 96;
pub const GR_GENERIC_DEBUG_RINGS: u32 = 0;
pub const GR_PARSE_BALANCE_ADDITIONS: u32 = 1;
pub const GR_PARSE_RING_EXPONENTS: u32 = 2;
pub const MPOLY_GCD_USE_HENSEL: u32 = 1;
pub const MPOLY_GCD_USE_BROWN: u32 = 2;
pub const MPOLY_GCD_USE_ZIPPEL: u32 = 4;
pub const MPOLY_GCD_USE_ZIPPEL2: u32 = 8;
pub const MPOLY_GCD_USE_PRS: u32 = 16;
pub const MPOLY_GCD_USE_ALL: u32 = 31;
pub const MPOLY_FACTOR_USE_ZAS: u32 = 1;
pub const MPOLY_FACTOR_USE_WANG: u32 = 2;
pub const MPOLY_FACTOR_USE_ZIP: u32 = 4;
pub const MPOLY_FACTOR_USE_ALL: u32 = 7;
pub const FLINT_FFT_MUL_THRESHOLD: u32 = 1540;
pub const FLINT_FFT_SQR_THRESHOLD: u32 = 3080;
pub const FLINT_MPN_MUL_FUNC_TAB_WIDTH: u32 = 17;
pub const FLINT_MPN_SQR_FUNC_TAB_WIDTH: u32 = 14;
pub const FLINT_MUL_USE_FUNC_TAB: u32 = 1;
pub const FLINT_MPN_MULHIGH_FUNC_TAB_WIDTH: u32 = 12;
pub const FLINT_MPN_SQRHIGH_FUNC_TAB_WIDTH: u32 = 8;
pub const FLINT_MPN_MULHIGH_NORMALISED_FUNC_TAB_WIDTH: u32 = 12;
pub const FLINT_HAVE_NATIVE_MPN_MULHIGH_BASECASE: u32 = 1;
pub const FLINT_HAVE_NATIVE_MPN_SQRHIGH_BASECASE: u32 = 1;
pub const N_FQ_REDUCE_ITCH: u32 = 2;
pub const N_FQ_MUL_ITCH: u32 = 4;
pub const N_FQ_LAZY_ITCH: u32 = 6;
pub const N_FQ_INV_ITCH: u32 = 1;
pub const N_FQ_POLY_DIVREM_DIVCONQUER_CUTOFF: u32 = 20;



// Arb constants //

pub const FMPR_RND_DOWN: c_int = 0;
pub const FMPR_RND_UP: c_int = 1;
pub const FMPR_RND_FLOOR: c_int = 2;
pub const FMPR_RND_CEIL: c_int = 3;
pub const FMPR_RND_NEAR: c_int = 4;
pub const MAG_BITS: c_int = 30;
pub const ARF_RND_DOWN: c_int = 0;
pub const ARF_RND_UP: c_int = 1;
pub const ARF_RND_FLOOR: c_int = 2;
pub const ARF_RND_CEIL: c_int = 3;
pub const ARF_RND_NEAR: c_int = 4;
pub const ARF_RESULT_EXACT: c_int = 0;
pub const ARF_RESULT_INEXACT: c_int = 1;
pub const ARF_EXP_ZERO: c_int = 0;
pub const ARF_NOPTR_LIMBS: c_int = 2;
pub const MUL_MPFR_MIN_LIMBS: c_int = 25;
pub const MUL_MPFR_MAX_LIMBS: c_int = 10000;
pub const ARF_MUL_STACK_ALLOC: c_int = 40;
pub const ARF_MUL_TLS_ALLOC: c_int = 1000;
pub const ARF_ADD_STACK_ALLOC: c_int = 40;
pub const ARF_ADD_TLS_ALLOC: c_int = 1000;
pub const __ARB_VERSION: c_int = 2;
pub const __ARB_VERSION_MINOR: c_int = 20;
pub const __ARB_VERSION_PATCHLEVEL: c_int = 0;
pub const ARB_VERSION: &'static [u8; 7usize] = b"2.20.0\0";
pub const __ARB_RELEASE: c_int = 22000;
pub const ARB_STR_MORE: c_ulong = 1;
pub const ARB_STR_NO_RADIUS: c_ulong = 2;
pub const ARB_STR_CONDENSE: c_ulong = 16;
pub const ARB_RND: c_int = 0;
pub const ARB_ATAN_TAB1_BITS: c_int = 8;
pub const ARB_ATAN_TAB1_PREC: c_int = 512;
pub const ARB_ATAN_TAB1_LIMBS: c_int = 8;
pub const ARB_ATAN_TAB21_BITS: c_int = 5;
pub const ARB_ATAN_TAB22_BITS: c_int = 5;
pub const ARB_ATAN_TAB2_PREC: c_int = 4608;
pub const ARB_ATAN_TAB2_LIMBS: c_int = 72;
pub const ARB_LOG_TAB11_BITS: c_int = 7;
pub const ARB_LOG_TAB12_BITS: c_int = 7;
pub const ARB_LOG_TAB1_PREC: c_int = 512;
pub const ARB_LOG_TAB1_LIMBS: c_int = 8;
pub const ARB_LOG_TAB21_BITS: c_int = 5;
pub const ARB_LOG_TAB22_BITS: c_int = 5;
pub const ARB_LOG_TAB2_PREC: c_int = 4608;
pub const ARB_LOG_TAB2_LIMBS: c_int = 72;
pub const ARB_EXP_TAB1_NUM: c_int = 178;
pub const ARB_EXP_TAB1_BITS: c_int = 8;
pub const ARB_EXP_TAB1_PREC: c_int = 512;
pub const ARB_EXP_TAB1_LIMBS: c_int = 8;
pub const ARB_EXP_TAB21_NUM: c_int = 23;
pub const ARB_EXP_TAB21_BITS: c_int = 5;
pub const ARB_EXP_TAB22_BITS: c_int = 5;
pub const ARB_EXP_TAB2_PREC: c_int = 4608;
pub const ARB_EXP_TAB2_LIMBS: c_int = 72;
pub const ARB_SIN_COS_TAB1_NUM: c_int = 203;
pub const ARB_SIN_COS_TAB1_BITS: c_int = 8;
pub const ARB_SIN_COS_TAB1_PREC: c_int = 512;
pub const ARB_SIN_COS_TAB1_LIMBS: c_int = 8;
pub const ARB_SIN_COS_TAB21_NUM: c_int = 26;
pub const ARB_SIN_COS_TAB21_BITS: c_int = 5;
pub const ARB_SIN_COS_TAB22_BITS: c_int = 5;
pub const ARB_SIN_COS_TAB2_PREC: c_int = 4608;
pub const ARB_SIN_COS_TAB2_LIMBS: c_int = 72;
pub const ARB_PI4_TAB_LIMBS: c_int = 72;
pub const ACB_LAMBERTW_LEFT: c_int = 2;
pub const ACB_LAMBERTW_MIDDLE: c_int = 4;
pub const FLINT_DEFAULT_THREAD_LIMIT: c_int = 99999;
pub const NMOD_MAT_MUL_TRANSPOSE_CUTOFF: c_int = 20;
pub const NMOD_MAT_SOLVE_TRI_ROWS_CUTOFF: c_int = 64;
pub const NMOD_MAT_SOLVE_TRI_COLS_CUTOFF: c_int = 64;
pub const NMOD_MAT_LU_RECURSIVE_CUTOFF: c_int = 4;
pub const NMOD_MAT_OPTIMAL_MODULUS_BITS: c_int = 59;
pub const NMOD_DIVREM_DIVCONQUER_CUTOFF: c_int = 300;
pub const NMOD_DIV_DIVCONQUER_CUTOFF: c_int = 300;
pub const NMOD_POLY_HGCD_CUTOFF: c_int = 100;
pub const NMOD_POLY_GCD_CUTOFF: c_int = 340;
pub const NMOD_POLY_SMALL_GCD_CUTOFF: c_int = 200;
pub const FMPZ_POLY_INV_NEWTON_CUTOFF: c_int = 32;
pub const FMPZ_POLY_SQRT_DIVCONQUER_CUTOFF: c_int = 16;
pub const FMPZ_POLY_SQRTREM_DIVCONQUER_CUTOFF: c_int = 16;
pub const D_BITS: c_int = 53;
pub const WEAK_CANONICALISE_CUTOFF: c_int = 25600;
pub const ARB_CALC_SUCCESS: c_int = 0;
pub const ARB_CALC_IMPRECISE_INPUT: c_int = 1;
pub const ARB_CALC_NO_CONVERGENCE: c_int = 2;
pub const DLOG_SMALL_LIM: c_int = 50;
pub const DLOG_TABLE_LIM: c_int = 50;
pub const DLOG_TABLE_P_LIM: c_int = 50;
pub const DLOG_TABLE_MODPE_LIM: c_int = 50;
pub const DLOG_TABLE_PE_LIM: c_int = 50;
pub const DLOG_TABLE_N_LIM: c_int = 50;
pub const DLOG_BSGS_LIM: c_int = 500;
pub const DLOG_LOOP_MAX_FACTOR: c_int = 6;
pub const DLOG_G_SMALL: c_int = 0;
pub const DLOG_G_BIG: c_int = 1;
pub const CRT_MAX: c_int = 15;
pub const DFT_VERB: c_int = 0;
pub const MAX_FACTORS: c_int = 15;
pub const ACB_HYPGEOM_2F1_REGULARIZED: c_int = 1;
pub const ACB_HYPGEOM_2F1_AB: c_int = 2;
pub const ACB_HYPGEOM_2F1_AC: c_int = 4;
pub const ACB_HYPGEOM_2F1_BC: c_int = 8;
pub const ACB_HYPGEOM_2F1_ABC: c_int = 16;
pub const ARB_FMPZ_POLY_ROOTS_VERBOSE: c_int = 1;
pub const BELL_NUMBER_TAB_SIZE: c_int = 26;
pub const SMALL_EULER_LIMIT: c_int = 25;
pub const BERNOULLI_SMALL_NUMER_LIMIT: c_int = 35;
pub const BERNOULLI_REV_MIN: c_int = 32;

// Antic constants //

pub const NF_POWERS_CUTOFF: c_ulong = 30;
pub const NF_GENERIC: c_ulong = 0;
pub const NF_MONIC: c_ulong = 1;
pub const NF_LINEAR: c_ulong = 2;
pub const NF_QUADRATIC: c_ulong = 4;
pub const NF_GAUSSIAN: c_ulong = 8;

// Calcium constants //

pub const QQBAR_DEFAULT_PREC: u32 = 128;
pub const QQBAR_ROOTS_IRREDUCIBLE: u32 = 1;
pub const QQBAR_ROOTS_UNSORTED: u32 = 2;
pub const FEXPR_TYPE_BITS: u32 = 4;
pub const FEXPR_SMALL_SYMBOL_LEN: u32 = 7;
pub const FEXPR_LATEX_SMALL: u32 = 1;
pub const FEXPR_LATEX_LOGIC: u32 = 2;
pub const CA_TRIG_DIRECT: u32 = 0;
pub const CA_TRIG_EXPONENTIAL: u32 = 1;
pub const CA_TRIG_SINE_COSINE: u32 = 2;
pub const CA_TRIG_TANGENT: u32 = 3;
pub const CA_FEXPR_SERIALIZATION: u32 = 1;
pub const CA_FACTOR_ZZ_NONE: u32 = 0;
pub const CA_FACTOR_ZZ_SMOOTH: u32 = 2;
pub const CA_FACTOR_ZZ_FULL: u32 = 4;
pub const CA_FACTOR_POLY_NONE: u32 = 0;
pub const CA_FACTOR_POLY_CONTENT: u32 = 64;
pub const CA_FACTOR_POLY_SQF: u32 = 128;
pub const CA_FACTOR_POLY_FULL: u32 = 256;


#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::MaybeUninit;
    
    #[test]
    fn test_fmpz() {
        let mut p: fmpz::fmpz = fmpz::fmpz::default();
        unsafe{
            fmpz::fmpz_init(&mut p);
            fmpz::fmpz_set_ui(&mut p, 17);
            debug_assert!(fmpz::fmpz_print(&mut p) > 0);            
            fmpz::fmpz_clear(&mut p);
        }
    }

    /*
    #[test]
    fn test_fmpz_mod_poly() {
        let mut p: fmpz::fmpz = fmpz::fmpz::default();
        let mut ctx = MaybeUninit::uninit();
        unsafe {
            fmpz::fmpz_init(&mut p);
            fmpz::fmpz_set_ui(&mut p, 17);
            debug_assert!(fmpz::fmpz_print(&mut p) > 0);
            let mut f = MaybeUninit::uninit();
            fmpz_mod::fmpz_mod_ctx_init(ctx.as_mut_ptr(), &p);
            let mut ctx = ctx.assume_init();
            fmpz_mod_poly::fmpz_mod_poly_init(f.as_mut_ptr(), &mut ctx);
            let mut f = f.assume_init();
            fmpz_mod_poly::fmpz_mod_poly_set_coeff_ui(&mut f, 1, 5, &mut ctx);
            fmpz_mod_poly::fmpz_mod_poly_set_coeff_ui(&mut f, 0, 5, &mut ctx);
            let x = std::ffi::CString::new("x").unwrap();
            debug_assert!(
                fmpz_mod_poly::fmpz_mod_poly_print_pretty(&mut f, x.as_ptr(), &mut ctx) > 0
            );
            fmpz_mod_poly::fmpz_mod_poly_clear(&mut f, &mut ctx);
            fmpz_mod::fmpz_mod_ctx_clear(&mut ctx);
            fmpz::fmpz_clear(&mut p);
        }
    }*/

    
    // Checks the norm of the element 2*x^2 - 1 of Q(x)/f where f = x^4 + 1
    #[test]
    fn test_antic() {
        let mut a = MaybeUninit::uninit();
        let mut b = MaybeUninit::uninit();
        let mut res = MaybeUninit::uninit();
        let mut pol = MaybeUninit::uninit();

        let mut x = MaybeUninit::uninit();
        let mut nf = MaybeUninit::uninit();
        
        unsafe {
            fmpz::fmpz_init_set_si(a.as_mut_ptr(), -1);
            let mut a = a.assume_init();
            fmpz::fmpz_init_set_si(b.as_mut_ptr(), 2);
            let mut b = b.assume_init();

            fmpq_poly::fmpq_poly_init(pol.as_mut_ptr());
            let mut pol = pol.assume_init();

            fmpq_poly::fmpq_poly_set_coeff_ui(&mut pol, 0, 1);
            fmpq_poly::fmpq_poly_set_coeff_ui(&mut pol, 4, 1);

            nf::nf_init(nf.as_mut_ptr(), &pol);
            let mut nf = nf.assume_init();

            nf_elem::nf_elem_init(x.as_mut_ptr(), &nf);
            let mut x = x.assume_init();
        
            nf_elem::_nf_elem_set_coeff_num_fmpz(&mut x, 0, &a, &nf);
            nf_elem::_nf_elem_set_coeff_num_fmpz(&mut x, 2, &b, &nf);

            fmpq::fmpq_init(res.as_mut_ptr());
            let mut res = res.assume_init();
            nf_elem::nf_elem_norm(&mut res, &x, &nf);

            assert!(fmpq::fmpq_equal_ui(&res, 25) != 0);
            println!("Success!");

            fmpz::fmpz_clear(&mut a);
            fmpz::fmpz_clear(&mut b);
            fmpq::fmpq_clear(&mut res);
            fmpq_poly::fmpq_poly_clear(&mut pol);
            nf_elem::nf_elem_clear(&mut x, &nf);
            nf::nf_clear(&mut nf);
        }
    }

    /*
    mod random {
        use crate::*;
        use gmp_mpfr_sys::gmp;
        use quickcheck_macros;
        use std::mem::MaybeUninit;

        #[quickcheck_macros::quickcheck]
        fn mult_same_as_gmp(v: u64, u: u64) -> bool {
            unsafe {
                let mut g_v = MaybeUninit::uninit();
                gmp::mpz_init(g_v.as_mut_ptr());
                let mut g_v = g_v.assume_init();
                gmp::mpz_set_ui(&mut g_v, v);

                let mut g_u = MaybeUninit::uninit();
                gmp::mpz_init(g_u.as_mut_ptr());
                let mut g_u = g_u.assume_init();
                gmp::mpz_set_ui(&mut g_u, u);

                let mut f_u = fmpz::fmpz::default();
                fmpz::fmpz_init(&mut f_u);
                fmpz::fmpz_set_ui(&mut f_u, u);

                let mut f_v = fmpz::fmpz::default();
                fmpz::fmpz_init(&mut f_v);
                fmpz::fmpz_set_ui(&mut f_v, v);

                gmp::mpz_mul(&mut g_v, &g_v, &g_u);
                fmpz::fmpz_mul(&mut f_v, &f_v, &f_u);
                let f_as_g = fmpz::_fmpz_promote_val(&mut f_v);
                let eq = gmp::mpz_cmp(flint_to_gmp::mpz_srcptr(f_as_g), &g_v) == 0;

                gmp::mpz_clear(&mut g_v);
                gmp::mpz_clear(&mut g_u);
                gmp::mpz_clear(flint_to_gmp::mpz_ptr(f_as_g));
                fmpz::_fmpz_demote_val(&mut f_v);
                fmpz::fmpz_clear(&mut f_u);
                fmpz::fmpz_clear(&mut f_v);
                eq
            }
        }
        pub mod flint_to_gmp {
            /// Safe because [gmp_mpfr_sys::gmp::mpz_t] and [flint_sys::deps::__mpz_struct] have identical
            /// layouts.
            pub fn mpz_srcptr(
                p: *const super::deps::__mpz_struct
            ) ->
                *const gmp_mpfr_sys::gmp::mpz_t
            {
                unsafe { &std::mem::transmute(*p) }
            }

            /// Safe because [gmp_mpfr_sys::gmp::mpz_t] and [flint_sys::deps::mp_srcptr] have identical
            /// layouts.
            pub fn mpz_ptr(
                p: *mut super::deps::__mpz_struct
                ) ->
                *mut gmp_mpfr_sys::gmp::mpz_t
            {
                unsafe { &mut std::mem::transmute(*p) }
            }
        }
    }*/
}
*/
