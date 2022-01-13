#![allow(non_camel_case_types)]
// TODO: nmod

//! See the [FLINT documentation](http://flintlib.org/doc/fmpz_vec.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpz::fmpz;
use libc::{c_int, FILE};

extern "C" {
    pub fn _fmpz_vec_init(len: mp_limb_signed_t) -> *mut fmpz;
    pub fn _fmpz_vec_clear(vec: *mut fmpz, len: mp_limb_signed_t);
    pub fn _fmpz_vec_randtest(
        f: *mut fmpz,
        state: *const flint_rand_s,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
    );
    pub fn _fmpz_vec_randtest_unsigned(
        f: *mut fmpz,
        state: *const flint_rand_s,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
    );
    pub fn _fmpz_vec_max_bits(vec: *const fmpz, len: mp_limb_signed_t) -> mp_limb_signed_t;
    pub fn _fmpz_vec_max_bits_ref(vec: *const fmpz, len: mp_limb_signed_t) -> mp_limb_signed_t;
    pub fn _fmpz_vec_sum_max_bits(
        sumabs: *mut mp_limb_signed_t,
        maxabs: *mut mp_limb_signed_t,
        coeffs: *const fmpz,
        length: mp_limb_signed_t,
    );
    pub fn _fmpz_vec_max_limbs(vec: *const fmpz, len: mp_limb_signed_t) -> mp_size_t;
    pub fn _fmpz_vec_height(height: *mut fmpz, vec: *const fmpz, len: mp_limb_signed_t);
    pub fn _fmpz_vec_height_index(vec: *const fmpz, len: mp_limb_signed_t) -> mp_limb_signed_t;
    pub fn _fmpz_vec_fprint(file: *mut FILE, vec: *const fmpz, len: mp_limb_signed_t) -> c_int;
    pub fn _fmpz_vec_print(vec: *const fmpz, len: mp_limb_signed_t) -> c_int;
    pub fn _fmpz_vec_fread(
        file: *const FILE,
        vec: *mut *mut fmpz,
        len: *mut mp_limb_signed_t,
    ) -> c_int;
    pub fn _fmpz_vec_read(vec: *mut *mut fmpz, len: *mut mp_limb_signed_t) -> c_int;
    /*
    pub fn _fmpz_vec_set_nmod_vec(
        res: *mut fmpz,
        poly: mp_srcptr,
        len: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn _fmpz_vec_get_nmod_vec(
        res: mp_ptr,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        mod_: nmod_t,
    );
    */
    pub fn _fmpz_vec_get_fft(
        coeffs_f: *mut *mut mp_limb_t,
        coeffs_m: *const fmpz,
        l: mp_limb_signed_t,
        length: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn _fmpz_vec_set_fft(
        coeffs_m: *mut fmpz,
        length: mp_limb_signed_t,
        coeffs_f: *const mp_ptr,
        limbs: mp_limb_signed_t,
        sign: mp_limb_signed_t,
    );
    pub fn _fmpz_vec_get_d_vec_2exp(
        appv: *mut f64,
        vec: *const fmpz,
        len: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    //pub fn _fmpz_vec_get_mpf_vec(appv: *mut mpf, vec: *const fmpz, len: mp_limb_signed_t);
    pub fn _fmpz_vec_set(vec1: *mut fmpz, vec2: *const fmpz, len2: mp_limb_signed_t);
    pub fn _fmpz_vec_swap(vec1: *mut fmpz, vec2: *mut fmpz, len2: mp_limb_signed_t);
    pub fn _fmpz_vec_zero(vec: *mut fmpz, len: mp_limb_signed_t);
    pub fn _fmpz_vec_neg(vec1: *mut fmpz, vec2: *const fmpz, len2: mp_limb_signed_t);
    pub fn _fmpz_vec_scalar_abs(vec1: *mut fmpz, vec2: *const fmpz, len2: mp_limb_signed_t);
    pub fn _fmpz_vec_equal(vec1: *const fmpz, vec2: *const fmpz, len: mp_limb_signed_t) -> c_int;
    pub fn _fmpz_vec_is_zero(vec: *const fmpz, len: mp_limb_signed_t) -> c_int;
    pub fn _fmpz_vec_max(
        vec1: *mut fmpz,
        vec2: *const fmpz,
        vec3: *const fmpz,
        len: mp_limb_signed_t,
    );
    pub fn _fmpz_vec_max_inplace(vec1: *mut fmpz, vec2: *const fmpz, len: mp_limb_signed_t);
    pub fn _fmpz_vec_min(
        vec1: *mut fmpz,
        vec2: *const fmpz,
        vec3: *const fmpz,
        len: mp_limb_signed_t,
    );
    pub fn _fmpz_vec_min_inplace(vec1: *mut fmpz, vec2: *const fmpz, len: mp_limb_signed_t);
    pub fn _fmpz_vec_sort(vec: *mut fmpz, len: mp_limb_signed_t);
    pub fn _fmpz_vec_add(
        res: *mut fmpz,
        vec1: *const fmpz,
        vec2: *const fmpz,
        len2: mp_limb_signed_t,
    );
    pub fn _fmpz_vec_sub(
        res: *mut fmpz,
        vec1: *const fmpz,
        vec2: *const fmpz,
        len2: mp_limb_signed_t,
    );
    pub fn _fmpz_vec_scalar_mul_si(
        vec1: *mut fmpz,
        vec2: *const fmpz,
        len2: mp_limb_signed_t,
        c: mp_limb_signed_t,
    );
    pub fn _fmpz_vec_scalar_mul_ui(
        vec1: *mut fmpz,
        vec2: *const fmpz,
        len2: mp_limb_signed_t,
        c: mp_limb_t,
    );
    pub fn _fmpz_vec_scalar_mul_fmpz(
        vec1: *mut fmpz,
        vec2: *const fmpz,
        len2: mp_limb_signed_t,
        x: *const fmpz,
    );
    pub fn _fmpz_vec_scalar_mul_2exp(
        vec1: *mut fmpz,
        vec2: *const fmpz,
        len2: mp_limb_signed_t,
        exp: mp_limb_t,
    );
    pub fn _fmpz_vec_scalar_divexact_fmpz(
        vec1: *mut fmpz,
        vec2: *const fmpz,
        len2: mp_limb_signed_t,
        x: *const fmpz,
    );
    pub fn _fmpz_vec_scalar_divexact_si(
        vec1: *mut fmpz,
        vec2: *const fmpz,
        len2: mp_limb_signed_t,
        c: mp_limb_signed_t,
    );
    pub fn _fmpz_vec_scalar_divexact_ui(
        vec1: *mut fmpz,
        vec2: *const fmpz,
        len2: mp_limb_signed_t,
        c: mp_limb_t,
    );
    pub fn _fmpz_vec_scalar_fdiv_q_fmpz(
        vec1: *mut fmpz,
        vec2: *const fmpz,
        len2: mp_limb_signed_t,
        c: *const fmpz,
    );
    pub fn _fmpz_vec_scalar_fdiv_q_si(
        vec1: *mut fmpz,
        vec2: *const fmpz,
        len2: mp_limb_signed_t,
        c: mp_limb_signed_t,
    );
    pub fn _fmpz_vec_scalar_fdiv_q_ui(
        vec1: *mut fmpz,
        vec2: *const fmpz,
        len2: mp_limb_signed_t,
        c: mp_limb_t,
    );
    pub fn _fmpz_vec_scalar_fdiv_q_2exp(
        vec1: *mut fmpz,
        vec2: *const fmpz,
        len2: mp_limb_signed_t,
        exp: mp_limb_t,
    );
    pub fn _fmpz_vec_scalar_fdiv_r_2exp(
        vec1: *mut fmpz,
        vec2: *const fmpz,
        len2: mp_limb_signed_t,
        exp: mp_limb_t,
    );
    pub fn _fmpz_vec_scalar_tdiv_q_fmpz(
        vec1: *mut fmpz,
        vec2: *const fmpz,
        len2: mp_limb_signed_t,
        c: *const fmpz,
    );
    pub fn _fmpz_vec_scalar_tdiv_q_si(
        vec1: *mut fmpz,
        vec2: *const fmpz,
        len2: mp_limb_signed_t,
        c: mp_limb_signed_t,
    );
    pub fn _fmpz_vec_scalar_tdiv_q_ui(
        vec1: *mut fmpz,
        vec2: *const fmpz,
        len2: mp_limb_signed_t,
        c: mp_limb_t,
    );
    pub fn _fmpz_vec_scalar_tdiv_q_2exp(
        vec1: *mut fmpz,
        vec2: *const fmpz,
        len2: mp_limb_signed_t,
        exp: mp_limb_t,
    );
    pub fn _fmpz_vec_scalar_addmul_si(
        vec1: *mut fmpz,
        vec2: *const fmpz,
        len2: mp_limb_signed_t,
        c: mp_limb_signed_t,
    );
    pub fn _fmpz_vec_scalar_addmul_fmpz(
        poly1: *mut fmpz,
        poly2: *const fmpz,
        len2: mp_limb_signed_t,
        x: *const fmpz,
    );
    pub fn _fmpz_vec_scalar_addmul_si_2exp(
        vec1: *mut fmpz,
        vec2: *const fmpz,
        len2: mp_limb_signed_t,
        c: mp_limb_signed_t,
        exp: mp_limb_t,
    );
    pub fn _fmpz_vec_scalar_submul_si(
        vec1: *mut fmpz,
        vec2: *const fmpz,
        len2: mp_limb_signed_t,
        c: mp_limb_signed_t,
    );
    pub fn _fmpz_vec_scalar_submul_fmpz(
        vec1: *mut fmpz,
        vec2: *const fmpz,
        len2: mp_limb_signed_t,
        x: *const fmpz,
    );
    pub fn _fmpz_vec_scalar_submul_si_2exp(
        vec1: *mut fmpz,
        vec2: *const fmpz,
        len2: mp_limb_signed_t,
        c: mp_limb_signed_t,
        exp: mp_limb_t,
    );
    pub fn _fmpz_vec_sum(res: *mut fmpz, vec: *const fmpz, len: mp_limb_signed_t);
    pub fn _fmpz_vec_prod(res: *mut fmpz, vec: *const fmpz, len: mp_limb_signed_t);
    pub fn _fmpz_vec_scalar_mod_fmpz(
        res: *mut fmpz,
        vec: *const fmpz,
        len: mp_limb_signed_t,
        p: *const fmpz,
    );
    pub fn _fmpz_vec_scalar_smod_fmpz(
        res: *mut fmpz,
        vec: *const fmpz,
        len: mp_limb_signed_t,
        p: *const fmpz,
    );
    pub fn _fmpz_vec_content(res: *mut fmpz, vec: *const fmpz, len: mp_limb_signed_t);
    pub fn _fmpz_vec_content_chained(res: *mut fmpz, vec: *const fmpz, len: mp_limb_signed_t);
    pub fn _fmpz_vec_lcm(res: *mut fmpz, vec: *const fmpz, len: mp_limb_signed_t);
    pub fn _fmpz_vec_dot(
        res: *mut fmpz,
        vec1: *const fmpz,
        vec2: *const fmpz,
        len2: mp_limb_signed_t,
    );
    pub fn _fmpz_vec_dot_ptr(
        c: *mut fmpz,
        vec1: *const fmpz,
        vec2: *mut *mut fmpz,
        offset: mp_limb_signed_t,
        len: mp_limb_signed_t,
    );
}
