//! Bindings for the [FLINT](http://flintlib.org/sphinx/index.html) library.
//! Crates marked with an asterisk have functions which may require mutable borrows where const
//! borrows will suffice (these need to be corrected but the bindings will still work as expected).

pub mod deps;
pub mod flint;

pub mod perm;
pub mod mpoly;

pub mod ulong_extras;
pub mod fmpz;
pub mod fmpz_vec;
pub mod fmpz_factor;
pub mod fmpz_mat;
pub mod fmpz_lll;
pub mod fmpz_poly;
pub mod fmpz_poly_mat;
pub mod fmpz_poly_factor;
pub mod fmpz_mpoly;
pub mod fmpz_mpoly_factor;
pub mod long_extras;
pub mod arith;
pub mod fft;
pub mod qsieve;

pub mod fmpq;
pub mod fmpq_vec;
pub mod fmpq_mat;
pub mod fmpq_poly;
pub mod fmpq_mpoly;
pub mod fmpz_poly_q;

pub mod n_poly;
pub mod nmod_vec;
pub mod nmod_mat;
pub mod nmod_poly;
pub mod nmod_poly_mat;
pub mod nmod_poly_factor;
pub mod nmod_mpoly;
pub mod fmpz_mod;
pub mod fmpz_mod_mat;
pub mod fmpz_mod_poly;
pub mod fmpz_mod_poly_factor;

pub mod fq;
pub mod fq_vec;
pub mod fq_mat;
pub mod fq_poly;
pub mod fq_poly_factor;
pub mod fq_nmod;
pub mod fq_nmod_vec;
pub mod fq_nmod_mat;
pub mod fq_nmod_poly;
pub mod fq_nmod_poly_factor;
pub mod fq_zech;
pub mod fq_zech_vec;
pub mod fq_zech_mat;
pub mod fq_zech_poly;
pub mod fq_zech_poly_factor;

pub mod padic;
pub mod padic_poly;
pub mod padic_mat;
pub mod qadic;

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::MaybeUninit;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn try_it() {
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
        }
    }

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
                fmpz::fmpz_clear(&mut f_u);
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
    }
}
