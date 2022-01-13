#![allow(non_camel_case_types)]

//! *See the [FLINT documentation](http://flintlib.org/doc/arith.html).

use crate::deps::*;
use crate::fmpq::fmpq;
use crate::fmpq_poly::fmpq_poly_struct;
use crate::fmpz::fmpz;
use crate::fmpz_mat::fmpz_mat_struct;
use crate::fmpz_poly::fmpz_poly_struct;
use libc::{c_int, c_uint};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct trig_prod_struct {
    pub n: c_int,
    pub prefactor: c_int,
    pub sqrt_p: mp_limb_t,
    pub sqrt_q: mp_limb_t,
    pub cos_p: [mp_limb_signed_t; 64usize],
    pub cos_q: [mp_limb_t; 64usize],
}
pub type trig_prod_t = [trig_prod_struct; 1usize];

extern "C" {
    pub static mut bell_number_tab: [mp_limb_t; 0usize];
    pub static euler_number_small: [mp_limb_t; 13usize];
    pub static _bernoulli_numer_small: [mp_limb_signed_t; 18usize];
    pub static partitions_lookup: [c_uint; 128usize];

    pub fn mpfr_zeta_inv_euler_product(res: *mut __mpfr_struct, s: mp_limb_t, char_4: c_int);
    pub fn _arith_harmonic_number(num: *mut fmpz, den: *mut fmpz, n: mp_limb_signed_t);
    pub fn arith_harmonic_number(x: *mut fmpq, n: mp_limb_signed_t);
    pub fn arith_ramanujan_tau(res: *mut fmpz, n: *mut fmpz);
    pub fn arith_ramanujan_tau_series(res: *mut fmpz_poly_struct, n: mp_limb_signed_t);
    pub fn arith_divisors(res: *mut fmpz_poly_struct, n: *mut fmpz);
    pub fn arith_stirling_number_1u(s: *mut fmpz, n: mp_limb_signed_t, k: mp_limb_signed_t);
    pub fn arith_stirling_number_1(s: *mut fmpz, n: mp_limb_signed_t, k: mp_limb_signed_t);
    pub fn arith_stirling_number_2(s: *mut fmpz, n: mp_limb_signed_t, k: mp_limb_signed_t);
    pub fn arith_stirling_number_1u_vec(
        row: *mut fmpz,
        n: mp_limb_signed_t,
        klen: mp_limb_signed_t,
    );
    pub fn arith_stirling_number_1_vec(row: *mut fmpz, n: mp_limb_signed_t, klen: mp_limb_signed_t);
    pub fn arith_stirling_number_2_vec(row: *mut fmpz, n: mp_limb_signed_t, klen: mp_limb_signed_t);
    pub fn arith_stirling_number_1u_vec_next(
        row: *mut fmpz,
        prev: *const fmpz,
        n: mp_limb_signed_t,
        klen: mp_limb_signed_t,
    );
    pub fn arith_stirling_number_1_vec_next(
        row: *mut fmpz,
        prev: *const fmpz,
        n: mp_limb_signed_t,
        klen: mp_limb_signed_t,
    );
    pub fn arith_stirling_number_2_vec_next(
        row: *mut fmpz,
        prev: *const fmpz,
        n: mp_limb_signed_t,
        klen: mp_limb_signed_t,
    );
    pub fn arith_stirling_matrix_1u(mat: *mut fmpz_mat_struct);
    pub fn arith_stirling_matrix_1(mat: *mut fmpz_mat_struct);
    pub fn arith_stirling_matrix_2(mat: *mut fmpz_mat_struct);
    pub fn arith_bell_number_size(n: mp_limb_t) -> f64;
    pub fn arith_bell_number(b: *mut fmpz, n: mp_limb_t);
    pub fn arith_bell_number_bsplit(res: *mut fmpz, n: mp_limb_t);
    pub fn arith_bell_number_multi_mod(res: *mut fmpz, n: mp_limb_t);
    pub fn arith_bell_number_vec(b: *mut fmpz, n: mp_limb_signed_t);
    pub fn arith_bell_number_vec_recursive(b: *mut fmpz, n: mp_limb_signed_t);
    pub fn arith_bell_number_vec_multi_mod(b: *mut fmpz, n: mp_limb_signed_t);
    /*
    pub fn arith_bell_number_nmod(n: mp_limb_t, mod_: nmod_t) -> mp_limb_t;
    pub fn arith_bell_number_nmod_vec(b: mp_ptr, n: mp_limb_signed_t, mod_: nmod_t);
    pub fn arith_bell_number_nmod_vec_recursive(b: mp_ptr, n: mp_limb_signed_t, mod_: nmod_t);
    pub fn arith_bell_number_nmod_vec_series(b: mp_ptr, n: mp_limb_signed_t, mod_: nmod_t);
    */
    pub fn arith_euler_number_size(n: mp_limb_t) -> f64;
    pub fn arith_euler_number_vec(res: *mut fmpz, n: mp_limb_signed_t);
    pub fn _arith_euler_number_zeta(res: *mut fmpz, n: mp_limb_t);
    pub fn arith_euler_number(res: *mut fmpz, n: mp_limb_t);
    pub fn arith_euler_polynomial(poly: *mut fmpq_poly_struct, n: mp_limb_t);
    pub fn _arith_bernoulli_number(num: *mut fmpz, den: *mut fmpz, n: mp_limb_t);
    pub fn arith_bernoulli_number(x: *mut fmpq, n: mp_limb_t);
    pub fn _arith_bernoulli_number_vec(num: *mut fmpz, den: *mut fmpz, n: mp_limb_signed_t);
    pub fn arith_bernoulli_number_vec(num: *mut fmpq, n: mp_limb_signed_t);
    pub fn arith_bernoulli_number_denom(den: *mut fmpz, n: mp_limb_t);
    pub fn arith_bernoulli_number_size(n: mp_limb_t) -> f64;
    pub fn arith_bernoulli_polynomial(poly: *mut fmpq_poly_struct, n: mp_limb_t);
    pub fn _arith_bernoulli_number_zeta(num: *mut fmpz, den: *mut fmpz, n: mp_limb_t);
    pub fn _arith_bernoulli_number_vec_multi_mod(
        num: *mut fmpz,
        den: *mut fmpz,
        n: mp_limb_signed_t,
    );
    pub fn _arith_bernoulli_number_vec_recursive(
        num: *mut fmpz,
        den: *mut fmpz,
        n: mp_limb_signed_t,
    );
    pub fn _arith_bernoulli_number_vec_zeta(num: *mut fmpz, den: *mut fmpz, n: mp_limb_signed_t);
    pub fn _arith_cos_minpoly(coeffs: *mut fmpz, d: mp_limb_signed_t, n: mp_limb_t);
    pub fn arith_cos_minpoly(poly: *mut fmpz_poly_struct, n: mp_limb_t);
    pub fn arith_landau_function_vec(res: *mut fmpz, len: mp_limb_signed_t);
    pub fn trig_prod_init(sum: *mut trig_prod_struct);
    pub fn arith_hrr_expsum_factored(prod: *mut trig_prod_struct, k: mp_limb_t, n: mp_limb_t);
    //pub fn arith_number_of_partitions_nmod_vec(res: mp_ptr, len: mp_limb_signed_t, mod_: nmod_t);
    pub fn arith_number_of_partitions_vec(res: *mut fmpz, len: mp_limb_signed_t);
    pub fn arith_number_of_partitions_mpfr(x: *mut __mpfr_struct, n: mp_limb_t);
    pub fn arith_number_of_partitions(x: *mut fmpz, n: mp_limb_t);
    pub fn arith_sum_of_squares(r: *mut fmpz, k: mp_limb_t, n: *mut fmpz);
    pub fn arith_sum_of_squares_vec(r: *mut fmpz, k: mp_limb_t, n: mp_limb_signed_t);
}
