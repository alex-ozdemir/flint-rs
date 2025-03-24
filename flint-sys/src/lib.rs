#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
//#![allow(unused_imports)] // remove

/// Bindings for the [FLINT](http://flintlib.org/sphinx/index.html) library.

pub mod bindgen;
pub mod deps;

pub mod acb;
pub mod acb_calc;
pub mod acb_dft;
pub mod acb_dirichlet;
pub mod acb_elliptic;
pub mod acb_hypgeom;
pub mod acb_mat;
pub mod acb_modular;
pub mod acb_poly;
pub mod acb_theta;
pub mod acb_types;
pub mod acf;
pub mod acf_types;
pub mod aprcl;
pub mod arb;
pub mod arb_calc;
pub mod arb_fmpz_poly;
pub mod arb_fpwrap;
pub mod arb_hypgeom;
pub mod arb_mat;
pub mod arb_poly;
pub mod arb_types;
pub mod arf;
pub mod arf_types;
pub mod arith;
pub mod bernoulli;
pub mod bool_mat;
pub mod ca;
pub mod ca_ext;
pub mod ca_field;
pub mod ca_mat;
pub mod ca_poly;
pub mod ca_vec;
pub mod calcium;
pub mod d_mat;
pub mod d_vec;
pub mod dirichlet;
pub mod dlog;
pub mod double_extras;
pub mod double_interval;
pub mod fexpr;
pub mod fexpr_builtin;
pub mod fft;
pub mod fft_tuning;
pub mod flint_config;
pub mod flint;
pub mod fmpq;
pub mod fmpq_mat;
pub mod fmpq_mpoly;
pub mod fmpq_mpoly_factor;
pub mod fmpq_poly;
pub mod fmpq_types;
pub mod fmpq_vec;
pub mod fmpz;
pub mod fmpz_extras;
pub mod fmpz_factor;
pub mod fmpz_lll;
pub mod fmpz_mat;
pub mod fmpz_mod;
pub mod fmpz_mod_mat;
pub mod fmpz_mod_mpoly;
pub mod fmpz_mod_mpoly_factor;
pub mod fmpz_mod_poly;
pub mod fmpz_mod_poly_factor;
pub mod fmpz_mod_types;
pub mod fmpz_mod_vec;
pub mod fmpz_mpoly;
pub mod fmpz_mpoly_factor;
pub mod fmpz_mpoly_q;
pub mod fmpz_poly;
pub mod fmpz_poly_factor;
pub mod fmpz_poly_mat;
pub mod fmpz_poly_q;
pub mod fmpz_types;
pub mod fmpz_vec;
pub mod fmpzi;
pub mod fq;
pub mod fq_default;
pub mod fq_default_mat;
pub mod fq_default_poly;
pub mod fq_default_poly_factor;
pub mod fq_embed;
pub mod fq_embed_templates;
pub mod fq_mat;
pub mod fq_mat_templates;
pub mod fq_nmod;
pub mod fq_nmod_embed;
pub mod fq_nmod_mat;
pub mod fq_nmod_mpoly;
pub mod fq_nmod_mpoly_factor;
pub mod fq_nmod_poly;
pub mod fq_nmod_poly_factor;
pub mod fq_nmod_types;
pub mod fq_nmod_vec;
pub mod fq_poly;
pub mod fq_poly_factor;
pub mod fq_poly_factor_templates;
pub mod fq_poly_templates;
pub mod fq_templates;
pub mod fq_types;
pub mod fq_vec;
pub mod fq_vec_templates;
pub mod fq_zech;
pub mod fq_zech_embed;
pub mod fq_zech_mat;
pub mod fq_zech_mpoly;
pub mod fq_zech_mpoly_factor;
pub mod fq_zech_poly;
pub mod fq_zech_poly_factor;
pub mod fq_zech_types;
pub mod fq_zech_vec;
pub mod gmpcompat;
pub mod gr;
pub mod gr_generic;
pub mod gr_mat;
pub mod gr_mpoly;
pub mod gr_poly;
pub mod gr_special;
pub mod gr_vec;
pub mod hypgeom;
pub mod limb_types;
pub mod long_extras;
pub mod longlong;
pub mod mag;
pub mod mpf_impl;
pub mod mpfr_mat;
pub mod mpfr_vec;
pub mod mpoly;
pub mod mpoly_types;
pub mod n_poly;
pub mod n_poly_types;
pub mod nf;
pub mod nf_elem;
pub mod nmod;
pub mod nmod_mat;
pub mod nmod_mpoly;
pub mod nmod_mpoly_factor;
pub mod nmod_poly;
pub mod nmod_poly_factor;
pub mod nmod_poly_mat;
pub mod nmod_types;
pub mod nmod_vec;
pub mod padic;
pub mod padic_mat;
pub mod padic_poly;
pub mod padic_types;
pub mod partitions;
pub mod perm;
pub mod qadic;
pub mod qfb;
pub mod qqbar;
pub mod qsieve;
pub mod templates;
pub mod thread_pool;
pub mod thread_support;
pub mod ulong_extras;

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::MaybeUninit;
    
    #[test]
    fn test_fmpz() { 
        let mut p: flint::fmpz = flint::fmpz::default();
        unsafe{
            fmpz::fmpz_init(&mut p);
            fmpz::fmpz_set_ui(&mut p, 17);
            debug_assert!(fmpz::fmpz_print(&mut p) > 0);            
            fmpz::fmpz_clear(&mut p);
        }
    }

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
}

/*
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
