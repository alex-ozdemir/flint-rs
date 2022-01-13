#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
// TODO: sort out d_mat_t and mpf_mat_t, then fmpz_gram_union etc.

//! See the [FLINT documentation](http://flintlib.org/doc/fmpz_lll.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpz::fmpz;
use crate::fmpz_mat::fmpz_mat_struct;
use libc::{c_int, c_uint};

pub const rep_type_GRAM: rep_type = 0;
pub const rep_type_Z_BASIS: rep_type = 1;
pub type rep_type = c_uint;
pub const gram_type_APPROX: gram_type = 0;
pub const gram_type_EXACT: gram_type = 1;
pub type gram_type = c_uint;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_lll_struct {
    pub delta: f64,
    pub eta: f64,
    pub rt: rep_type,
    pub gt: gram_type,
}

pub type fmpz_lll_t = [fmpz_lll_struct; 1usize];

/*
#[repr(C)]
#[derive(Copy, Clone)]
pub union fmpz_gram_union {
    pub appSP: d_mat_t,
    pub appSP2: mpf_mat_t,
    pub exactSP: fmpz_mat_t,
}

pub type fmpz_gram_t = [fmpz_gram_union; 1usize];
*/

extern "C" {
    pub fn fmpz_lll_context_init_default(fl: *mut fmpz_lll_struct);
    pub fn fmpz_lll_context_init(
        fl: *mut fmpz_lll_struct,
        delta: f64,
        eta: f64,
        rt: rep_type,
        gt: gram_type,
    );
    pub fn fmpz_lll_randtest(fl: *mut fmpz_lll_struct, state: *const flint_rand_s);
    pub fn fmpz_lll_heuristic_dot(
        vec1: *const f64,
        vec2: *const f64,
        len2: mp_limb_signed_t,
        B: *const fmpz_mat_struct,
        k: mp_limb_signed_t,
        j: mp_limb_signed_t,
        exp_adj: mp_limb_signed_t,
    ) -> f64;
    /*
    pub fn fmpz_lll_check_babai(
        kappa: c_int,
        B: *mut fmpz_mat_struct,
        U: *mut fmpz_mat_struct,
        mu: *mut d_mat_struct,
        r: *mut d_mat_struct,
        s: *mut f64,
        appB: *mut d_mat_struct,
        expo: *mut c_int,
        A: *mut fmpz_gram_union,
        a: c_int,
        zeros: c_int,
        kappamax: c_int,
        n: c_int,
        fl: *const fmpz_lll_struct,
    ) -> c_int;
    pub fn fmpz_lll_check_babai_heuristic_d(
        kappa: c_int,
        B: *mut fmpz_mat_struct,
        U: *mut fmpz_mat_struct,
        mu: *mut d_mat_struct,
        r: *mut d_mat_struct,
        s: *mut f64,
        appB: *mut d_mat_struct,
        expo: *mut c_int,
        A: *mut fmpz_gram_union,
        a: c_int,
        zeros: c_int,
        kappamax: c_int,
        n: c_int,
        fl: *const fmpz_lll_struct,
    ) -> c_int;
    */
    pub fn fmpz_lll_shift(B: *const fmpz_mat_struct) -> c_int;
    pub fn fmpz_lll_d(
        B: *mut fmpz_mat_struct,
        U: *mut fmpz_mat_struct,
        fl: *const fmpz_lll_struct,
    ) -> c_int;
    pub fn fmpz_lll_d_heuristic(
        B: *mut fmpz_mat_struct,
        U: *mut fmpz_mat_struct,
        fl: *const fmpz_lll_struct,
    ) -> c_int;
    /*
    pub fn fmpz_lll_check_babai_heuristic(
        kappa: c_int,
        B: *mut fmpz_mat_struct,
        U: *mut fmpz_mat_struct,
        mu: *mut mpf_mat_struct,
        r: *mut mpf_mat_struct,
        s: *mut mpf,
        appB: *mut mpf_mat_struct,
        A: *mut fmpz_gram_union,
        a: c_int,
        zeros: c_int,
        kappamax: c_int,
        n: c_int,
        tmp: *mut __mpf_struct,
        rtmp: *mut __mpf_struct,
        prec: mp_limb_t,
        fl: *const fmpz_lll_struct,
    ) -> c_int;
    */
    pub fn fmpz_lll_mpf2(
        B: *mut fmpz_mat_struct,
        U: *mut fmpz_mat_struct,
        prec: mp_limb_t,
        fl: *const fmpz_lll_struct,
    ) -> c_int;
    pub fn fmpz_lll_mpf(
        B: *mut fmpz_mat_struct,
        U: *mut fmpz_mat_struct,
        fl: *const fmpz_lll_struct,
    ) -> c_int;
    pub fn fmpz_lll_wrapper(
        B: *mut fmpz_mat_struct,
        U: *mut fmpz_mat_struct,
        fl: *const fmpz_lll_struct,
    ) -> c_int;
    /*
    pub fn fmpz_lll_advance_check_babai(
        cur_kappa: c_int,
        kappa: c_int,
        B: *mut fmpz_mat_struct,
        U: *mut fmpz_mat_struct,
        mu: *mut d_mat_struct,
        r: *mut d_mat_struct,
        s: *mut f64,
        appB: *mut d_mat_struct,
        expo: *mut c_int,
        A: *mut fmpz_gram_union,
        a: c_int,
        zeros: c_int,
        kappamax: c_int,
        n: c_int,
        fl: *const fmpz_lll_struct,
    ) -> c_int;
    pub fn fmpz_lll_advance_check_babai_heuristic_d(
        cur_kappa: c_int,
        kappa: c_int,
        B: *mut fmpz_mat_struct,
        U: *mut fmpz_mat_struct,
        mu: *mut d_mat_struct,
        r: *mut d_mat_struct,
        s: *mut f64,
        appB: *mut d_mat_struct,
        expo: *mut c_int,
        A: *mut fmpz_gram_union,
        a: c_int,
        zeros: c_int,
        kappamax: c_int,
        n: c_int,
        fl: *const fmpz_lll_struct,
    ) -> c_int;
    */
    pub fn fmpz_lll_d_with_removal(
        B: *mut fmpz_mat_struct,
        U: *mut fmpz_mat_struct,
        gs_B: *const fmpz,
        fl: *const fmpz_lll_struct,
    ) -> c_int;
    pub fn fmpz_lll_d_heuristic_with_removal(
        B: *mut fmpz_mat_struct,
        U: *mut fmpz_mat_struct,
        gs_B: *const fmpz,
        fl: *const fmpz_lll_struct,
    ) -> c_int;
    pub fn fmpz_lll_mpf2_with_removal(
        B: *mut fmpz_mat_struct,
        U: *mut fmpz_mat_struct,
        prec: mp_limb_t,
        gs_B: *const fmpz,
        fl: *const fmpz_lll_struct,
    ) -> c_int;
    pub fn fmpz_lll_mpf_with_removal(
        B: *mut fmpz_mat_struct,
        U: *mut fmpz_mat_struct,
        gs_B: *const fmpz,
        fl: *const fmpz_lll_struct,
    ) -> c_int;
    pub fn fmpz_lll_wrapper_with_removal(
        B: *mut fmpz_mat_struct,
        U: *mut fmpz_mat_struct,
        gs_B: *const fmpz,
        fl: *const fmpz_lll_struct,
    ) -> c_int;
    pub fn fmpz_lll_d_with_removal_knapsack(
        B: *mut fmpz_mat_struct,
        U: *mut fmpz_mat_struct,
        gs_B: *const fmpz,
        fl: *const fmpz_lll_struct,
    ) -> c_int;
    pub fn fmpz_lll_wrapper_with_removal_knapsack(
        B: *mut fmpz_mat_struct,
        U: *mut fmpz_mat_struct,
        gs_B: *const fmpz,
        fl: *const fmpz_lll_struct,
    ) -> c_int;
    pub fn fmpz_lll_with_removal_ulll(
        FM: *mut fmpz_mat_struct,
        UM: *mut fmpz_mat_struct,
        new_size: mp_limb_signed_t,
        gs_B: *const fmpz,
        fl: *const fmpz_lll_struct,
    ) -> c_int;
    pub fn fmpz_lll_is_reduced_d(B: *const fmpz_mat_struct, fl: *const fmpz_lll_struct) -> c_int;
    pub fn fmpz_lll_is_reduced_mpfr(
        B: *const fmpz_mat_struct,
        fl: *const fmpz_lll_struct,
        prec: mp_limb_t,
    ) -> c_int;
    pub fn fmpz_lll_is_reduced(
        B: *const fmpz_mat_struct,
        fl: *const fmpz_lll_struct,
        prec: mp_limb_t,
    ) -> c_int;
    pub fn fmpz_lll_is_reduced_d_with_removal(
        B: *const fmpz_mat_struct,
        fl: *const fmpz_lll_struct,
        gs_B: *const fmpz,
        newd: c_int,
    ) -> c_int;
    pub fn fmpz_lll_is_reduced_mpfr_with_removal(
        B: *const fmpz_mat_struct,
        fl: *const fmpz_lll_struct,
        gs_B: *const fmpz,
        newd: c_int,
        prec: mp_limb_t,
    ) -> c_int;
    pub fn fmpz_lll_is_reduced_with_removal(
        B: *const fmpz_mat_struct,
        fl: *const fmpz_lll_struct,
        gs_B: *const fmpz,
        newd: c_int,
        prec: mp_limb_t,
    ) -> c_int;
    pub fn fmpz_lll(B: *mut fmpz_mat_struct, U: *mut fmpz_mat_struct, fl: *const fmpz_lll_struct);
    pub fn fmpz_lll_with_removal(
        B: *mut fmpz_mat_struct,
        U: *mut fmpz_mat_struct,
        gs_B: *const fmpz,
        fl: *const fmpz_lll_struct,
    ) -> c_int;
    pub fn fmpz_lll_storjohann_ulll(
        FM: *mut fmpz_mat_struct,
        new_size: mp_limb_signed_t,
        fl: *const fmpz_lll_struct,
    );
}
