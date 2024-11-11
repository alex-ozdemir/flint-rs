#![allow(non_camel_case_types)]

//! See the [Arb documentation](https://arblib.org/).

use crate::arf_types::{arf_ptr, arf_srcptr, arf_struct};
use crate::mag::{mag_ptr, mag_srcptr, mag_struct};
use crate::deps::*;
use crate::flint::*;
use crate::fmpq::fmpq;
use crate::fmpz::fmpz;
use libc::{c_char, c_int, FILE};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct arb_struct {
    pub mid: arf_struct,
    pub rad: mag_struct,
}

pub type arb_t = [arb_struct; 1usize];
pub type arb_ptr = *mut arb_struct;
pub type arb_srcptr = *const arb_struct;

extern "C" {
    pub fn arb_init(x: *mut arb_struct);
    pub fn arb_clear(x: *mut arb_struct);
    pub fn _arb_vec_init(n: mp_limb_signed_t) -> arb_ptr;
    pub fn _arb_vec_clear(v: arb_ptr, n: mp_limb_signed_t);
    pub fn arb_mid_ptr(z: *const arb_struct) -> arf_ptr;
    pub fn arb_rad_ptr(z: *const arb_struct) -> mag_ptr;
    pub fn arb_is_exact(x: *const arb_struct) -> c_int;
    pub fn arb_equal(x: *const arb_struct, y: *const arb_struct) -> c_int;
    pub fn arb_equal_si(x: *const arb_struct, y: mp_limb_signed_t) -> c_int;
    pub fn arb_eq(x: *const arb_struct, y: *const arb_struct) -> c_int;
    pub fn arb_ne(x: *const arb_struct, y: *const arb_struct) -> c_int;
    pub fn arb_lt(x: *const arb_struct, y: *const arb_struct) -> c_int;
    pub fn arb_le(x: *const arb_struct, y: *const arb_struct) -> c_int;
    pub fn arb_gt(x: *const arb_struct, y: *const arb_struct) -> c_int;
    pub fn arb_ge(x: *const arb_struct, y: *const arb_struct) -> c_int;
    pub fn arb_zero(x: *mut arb_struct);
    pub fn arb_is_zero(x: *const arb_struct) -> c_int;
    pub fn arb_pos_inf(x: *mut arb_struct);
    pub fn arb_neg_inf(x: *mut arb_struct);
    pub fn arb_zero_pm_inf(x: *mut arb_struct);
    pub fn arb_zero_pm_one(x: *mut arb_struct);
    pub fn arb_unit_interval(x: *mut arb_struct);
    pub fn arb_indeterminate(x: *mut arb_struct);
    pub fn arb_is_finite(x: *const arb_struct) -> c_int;
    pub fn arb_set(x: *mut arb_struct, y: *const arb_struct);
    pub fn arb_swap(x: *mut arb_struct, y: *mut arb_struct);
    pub fn arb_set_round(z: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_trim(y: *mut arb_struct, x: *const arb_struct);
    pub fn arb_neg(y: *mut arb_struct, x: *const arb_struct);
    pub fn arb_neg_round(x: *mut arb_struct, y: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_abs(y: *mut arb_struct, x: *const arb_struct);
    pub fn arb_sgn(res: *mut arb_struct, x: *const arb_struct);
    pub fn arb_sgn_nonzero(x: *const arb_struct) -> c_int;
    pub fn _arb_digits_round_inplace(
        s: *mut c_char,
        shift: *const mp_limb_t,
        error: *const fmpz,
        n: mp_limb_signed_t,
        rnd: c_int,
    );
    pub fn arb_set_str(res: *mut arb_struct, inp: *const c_char, prec: mp_limb_signed_t) -> c_int;
    pub fn arb_get_str(x: *const arb_struct, n: mp_limb_signed_t, flags: mp_limb_t) -> *mut c_char;
    pub fn arb_set_arf(x: *mut arb_struct, y: *const arf_struct);
    pub fn arb_set_si(x: *mut arb_struct, y: mp_limb_signed_t);
    pub fn arb_set_ui(x: *mut arb_struct, y: mp_limb_t);
    pub fn arb_set_d(x: *mut arb_struct, y: f64);
    pub fn arb_set_fmpz(x: *mut arb_struct, y: *const fmpz);
    pub fn arb_set_fmpz_2exp(x: *mut arb_struct, y: *const fmpz, exp: *const fmpz);
    pub fn arb_set_round_fmpz_2exp(
        y: *mut arb_struct,
        x: *const fmpz,
        exp: *const fmpz,
        prec: mp_limb_signed_t,
    );
    pub fn arb_set_round_fmpz(y: *mut arb_struct, x: *const fmpz, prec: mp_limb_signed_t);
    pub fn arb_is_one(f: *const arb_struct) -> c_int;
    pub fn arb_one(f: *mut arb_struct);
    pub fn arb_fprint(file: *mut FILE, x: *const arb_struct);
    pub fn arb_fprintd(file: *mut FILE, x: *const arb_struct, digits: mp_limb_signed_t);
    pub fn arb_fprintn(
        file: *mut FILE,
        x: *const arb_struct,
        digits: mp_limb_signed_t,
        flags: mp_limb_t,
    );
    pub fn arb_print(x: *const arb_struct);
    pub fn arb_printd(x: *const arb_struct, digits: mp_limb_signed_t);
    pub fn arb_printn(x: *const arb_struct, digits: mp_limb_signed_t, flags: mp_limb_t);
    pub fn arb_mul_2exp_si(y: *mut arb_struct, x: *const arb_struct, e: mp_limb_signed_t);
    pub fn arb_mul_2exp_fmpz(y: *mut arb_struct, x: *const arb_struct, e: *const fmpz);
    pub fn arb_is_int(x: *const arb_struct) -> c_int;
    pub fn arb_is_int_2exp_si(x: *const arb_struct, e: mp_limb_signed_t) -> c_int;
    pub fn arb_contains_zero(x: *const arb_struct) -> c_int;
    pub fn arb_is_nonzero(x: *const arb_struct) -> c_int;
    pub fn arb_is_positive(x: *const arb_struct) -> c_int;
    pub fn arb_is_nonnegative(x: *const arb_struct) -> c_int;
    pub fn arb_is_negative(x: *const arb_struct) -> c_int;
    pub fn arb_is_nonpositive(x: *const arb_struct) -> c_int;
    pub fn arb_contains_negative(x: *const arb_struct) -> c_int;
    pub fn arb_contains_nonpositive(x: *const arb_struct) -> c_int;
    pub fn arb_contains_positive(x: *const arb_struct) -> c_int;
    pub fn arb_contains_nonnegative(x: *const arb_struct) -> c_int;
    pub fn arb_get_mag_lower(z: *mut mag_struct, x: *const arb_struct);
    pub fn arb_get_mag_lower_nonnegative(z: *mut mag_struct, x: *const arb_struct);
    pub fn arb_get_mag(z: *mut mag_struct, x: *const arb_struct);
    pub fn arb_get_mid_arb(z: *mut arb_struct, x: *const arb_struct);
    pub fn arb_get_rad_arb(z: *mut arb_struct, x: *const arb_struct);
    pub fn arb_get_abs_ubound_arf(u: *mut arf_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_get_abs_lbound_arf(u: *mut arf_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_get_ubound_arf(u: *mut arf_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_get_lbound_arf(u: *mut arf_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_nonnegative_part(res: *mut arb_struct, x: *const arb_struct);
    pub fn arb_rel_error_bits(x: *const arb_struct) -> mp_limb_signed_t;
    pub fn arb_rel_accuracy_bits(x: *const arb_struct) -> mp_limb_signed_t;
    pub fn arb_rel_one_accuracy_bits(x: *const arb_struct) -> mp_limb_signed_t;
    pub fn arb_bits(x: *const arb_struct) -> mp_limb_signed_t;
    pub fn arb_randtest_exact(
        x: *mut arb_struct,
        state: *const flint_rand_s,
        prec: mp_limb_signed_t,
        mag_bits: mp_limb_signed_t,
    );
    pub fn arb_randtest_wide(
        x: *mut arb_struct,
        state: *const flint_rand_s,
        prec: mp_limb_signed_t,
        mag_bits: mp_limb_signed_t,
    );
    pub fn arb_randtest_precise(
        x: *mut arb_struct,
        state: *const flint_rand_s,
        prec: mp_limb_signed_t,
        mag_bits: mp_limb_signed_t,
    );
    pub fn arb_randtest(
        x: *mut arb_struct,
        state: *const flint_rand_s,
        prec: mp_limb_signed_t,
        mag_bits: mp_limb_signed_t,
    );
    pub fn arb_randtest_special(
        x: *mut arb_struct,
        state: *const flint_rand_s,
        prec: mp_limb_signed_t,
        mag_bits: mp_limb_signed_t,
    );
    pub fn arb_urandom(x: *mut arb_struct, state: *const flint_rand_s, prec: mp_limb_signed_t);
    pub fn arb_add_error_arf(x: *mut arb_struct, err: *const arf_struct);
    pub fn arb_add_error_2exp_si(x: *mut arb_struct, err: mp_limb_signed_t);
    pub fn arb_add_error_2exp_fmpz(x: *mut arb_struct, err: *const fmpz);
    pub fn arb_add_error(x: *mut arb_struct, error: *const arb_struct);
    pub fn arb_add_error_mag(x: *mut arb_struct, err: *const mag_struct);
    pub fn arb_contains_arf(x: *const arb_struct, y: *const arf_struct) -> c_int;
    pub fn arb_contains_fmpq(x: *const arb_struct, y: *const fmpq) -> c_int;
    pub fn arb_contains_fmpz(x: *const arb_struct, y: *const fmpz) -> c_int;
    pub fn arb_contains_si(x: *const arb_struct, y: mp_limb_signed_t) -> c_int;
    pub fn arb_contains_mpfr(x: *const arb_struct, y: *const __mpfr_struct) -> c_int;
    pub fn arb_overlaps(x: *const arb_struct, y: *const arb_struct) -> c_int;
    pub fn arb_contains(x: *const arb_struct, y: *const arb_struct) -> c_int;
    pub fn arb_contains_interior(x: *const arb_struct, y: *const arb_struct) -> c_int;
    pub fn arb_contains_int(x: *const arb_struct) -> c_int;
    pub fn arb_get_interval_fmpz_2exp(
        a: *mut fmpz,
        b: *mut fmpz,
        exp: *mut fmpz,
        x: *const arb_struct,
    );
    pub fn arb_get_unique_fmpz(z: *mut fmpz, x: *const arb_struct) -> c_int;
    pub fn arb_get_fmpz_mid_rad_10exp(
        mid: *mut fmpz,
        rad: *mut fmpz,
        exp: *mut fmpz,
        x: *const arb_struct,
        n: mp_limb_signed_t,
    );
    pub fn arb_floor(z: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_ceil(z: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_set_interval_arf(
        x: *mut arb_struct,
        a: *const arf_struct,
        b: *const arf_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_set_interval_mpfr(
        x: *mut arb_struct,
        a: *const __mpfr_struct,
        b: *const __mpfr_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_get_interval_arf(
        a: *mut arf_struct,
        b: *mut arf_struct,
        x: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_get_interval_mpfr(
        a: *mut __mpfr_struct,
        b: *mut __mpfr_struct,
        x: *const arb_struct,
    );
    pub fn arb_set_interval_mag(
        res: *mut arb_struct,
        a: *const mag_struct,
        b: *const mag_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_set_interval_neg_pos_mag(
        res: *mut arb_struct,
        a: *const mag_struct,
        b: *const mag_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_union(
        z: *mut arb_struct,
        x: *const arb_struct,
        y: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_intersection(
        z: *mut arb_struct,
        x: *const arb_struct,
        y: *const arb_struct,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn arb_get_rand_fmpq(
        q: *mut fmpq,
        state: *const flint_rand_s,
        x: *const arb_struct,
        bits: mp_limb_signed_t,
    );
    pub fn arb_min(
        z: *mut arb_struct,
        x: *const arb_struct,
        y: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_max(
        z: *mut arb_struct,
        x: *const arb_struct,
        y: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_can_round_arf(x: *const arb_struct, prec: mp_limb_signed_t, rnd: c_int) -> c_int;
    pub fn arb_can_round_mpfr(
        x: *const arb_struct,
        prec: mp_limb_signed_t,
        rnd: mpfr_rnd_t,
    ) -> c_int;
    pub fn arb_add(
        z: *mut arb_struct,
        x: *const arb_struct,
        y: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_add_arf(
        z: *mut arb_struct,
        x: *const arb_struct,
        y: *const arf_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_add_ui(
        z: *mut arb_struct,
        x: *const arb_struct,
        y: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_add_si(
        z: *mut arb_struct,
        x: *const arb_struct,
        y: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_add_fmpz(
        z: *mut arb_struct,
        x: *const arb_struct,
        y: *const fmpz,
        prec: mp_limb_signed_t,
    );
    pub fn arb_add_fmpz_2exp(
        z: *mut arb_struct,
        x: *const arb_struct,
        man: *const fmpz,
        exp: *const fmpz,
        prec: mp_limb_signed_t,
    );
    pub fn arb_sub(
        z: *mut arb_struct,
        x: *const arb_struct,
        y: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_sub_arf(
        z: *mut arb_struct,
        x: *const arb_struct,
        y: *const arf_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_sub_ui(
        z: *mut arb_struct,
        x: *const arb_struct,
        y: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_sub_si(
        z: *mut arb_struct,
        x: *const arb_struct,
        y: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_sub_fmpz(
        z: *mut arb_struct,
        x: *const arb_struct,
        y: *const fmpz,
        prec: mp_limb_signed_t,
    );
    pub fn arb_mul(
        z: *mut arb_struct,
        x: *const arb_struct,
        y: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_mul_arf(
        z: *mut arb_struct,
        x: *const arb_struct,
        y: *const arf_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_mul_si(
        z: *mut arb_struct,
        x: *const arb_struct,
        y: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_mul_ui(
        z: *mut arb_struct,
        x: *const arb_struct,
        y: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_mul_fmpz(
        z: *mut arb_struct,
        x: *const arb_struct,
        y: *const fmpz,
        prec: mp_limb_signed_t,
    );
    pub fn arb_addmul(
        z: *mut arb_struct,
        x: *const arb_struct,
        y: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_addmul_arf(
        z: *mut arb_struct,
        x: *const arb_struct,
        y: *const arf_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_addmul_si(
        z: *mut arb_struct,
        x: *const arb_struct,
        y: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_addmul_ui(
        z: *mut arb_struct,
        x: *const arb_struct,
        y: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_addmul_fmpz(
        z: *mut arb_struct,
        x: *const arb_struct,
        y: *const fmpz,
        prec: mp_limb_signed_t,
    );
    pub fn arb_submul(
        z: *mut arb_struct,
        x: *const arb_struct,
        y: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_submul_arf(
        z: *mut arb_struct,
        x: *const arb_struct,
        y: *const arf_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_submul_si(
        z: *mut arb_struct,
        x: *const arb_struct,
        y: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_submul_ui(
        z: *mut arb_struct,
        x: *const arb_struct,
        y: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_submul_fmpz(
        z: *mut arb_struct,
        x: *const arb_struct,
        y: *const fmpz,
        prec: mp_limb_signed_t,
    );
    pub fn arb_dot_simple(
        res: *mut arb_struct,
        initial: *const arb_struct,
        subtract: c_int,
        x: arb_srcptr,
        xstep: mp_limb_signed_t,
        y: arb_srcptr,
        ystep: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_dot_precise(
        res: *mut arb_struct,
        initial: *const arb_struct,
        subtract: c_int,
        x: arb_srcptr,
        xstep: mp_limb_signed_t,
        y: arb_srcptr,
        ystep: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_dot(
        res: *mut arb_struct,
        initial: *const arb_struct,
        subtract: c_int,
        x: arb_srcptr,
        xstep: mp_limb_signed_t,
        y: arb_srcptr,
        ystep: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_approx_dot(
        res: *mut arb_struct,
        initial: *const arb_struct,
        subtract: c_int,
        x: arb_srcptr,
        xstep: mp_limb_signed_t,
        y: arb_srcptr,
        ystep: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_dot_ui(
        res: *mut arb_struct,
        initial: *const arb_struct,
        subtract: c_int,
        x: arb_srcptr,
        xstep: mp_limb_signed_t,
        y: *const mp_limb_t,
        ystep: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_dot_si(
        res: *mut arb_struct,
        initial: *const arb_struct,
        subtract: c_int,
        x: arb_srcptr,
        xstep: mp_limb_signed_t,
        y: *const mp_limb_signed_t,
        ystep: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_dot_uiui(
        res: *mut arb_struct,
        initial: *const arb_struct,
        subtract: c_int,
        x: arb_srcptr,
        xstep: mp_limb_signed_t,
        y: *const mp_limb_t,
        ystep: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_dot_siui(
        res: *mut arb_struct,
        initial: *const arb_struct,
        subtract: c_int,
        x: arb_srcptr,
        xstep: mp_limb_signed_t,
        y: *const mp_limb_t,
        ystep: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_dot_fmpz(
        res: *mut arb_struct,
        initial: *const arb_struct,
        subtract: c_int,
        x: arb_srcptr,
        xstep: mp_limb_signed_t,
        y: *const fmpz,
        ystep: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_div(
        z: *mut arb_struct,
        x: *const arb_struct,
        y: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_div_arf(
        z: *mut arb_struct,
        x: *const arb_struct,
        y: *const arf_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_div_si(
        z: *mut arb_struct,
        x: *const arb_struct,
        y: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_div_ui(
        z: *mut arb_struct,
        x: *const arb_struct,
        y: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_div_fmpz(
        z: *mut arb_struct,
        x: *const arb_struct,
        y: *const fmpz,
        prec: mp_limb_signed_t,
    );
    pub fn arb_fmpz_div_fmpz(
        z: *mut arb_struct,
        x: *const fmpz,
        y: *const fmpz,
        prec: mp_limb_signed_t,
    );
    pub fn arb_ui_div(
        z: *mut arb_struct,
        x: mp_limb_t,
        y: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_inv(y: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_set_fmpq(y: *mut arb_struct, x: *const fmpq, prec: mp_limb_signed_t);
    pub fn arb_sqrt(z: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_sqrt_arf(z: *mut arb_struct, x: *const arf_struct, prec: mp_limb_signed_t);
    pub fn arb_sqrt_fmpz(z: *mut arb_struct, x: *const fmpz, prec: mp_limb_signed_t);
    pub fn arb_sqrt_ui(z: *mut arb_struct, x: mp_limb_t, prec: mp_limb_signed_t);
    pub fn arb_sqrtpos(z: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_hypot(
        z: *mut arb_struct,
        x: *const arb_struct,
        y: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_rsqrt(z: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_rsqrt_ui(z: *mut arb_struct, x: mp_limb_t, prec: mp_limb_signed_t);
    pub fn arb_sqrt1pm1(r: *mut arb_struct, z: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_pow_fmpz_binexp(
        y: *mut arb_struct,
        b: *const arb_struct,
        e: *const fmpz,
        prec: mp_limb_signed_t,
    );
    pub fn arb_pow_fmpz(
        y: *mut arb_struct,
        b: *const arb_struct,
        e: *const fmpz,
        prec: mp_limb_signed_t,
    );
    pub fn arb_pow_ui(
        y: *mut arb_struct,
        b: *const arb_struct,
        e: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_ui_pow_ui(y: *mut arb_struct, b: mp_limb_t, e: mp_limb_t, prec: mp_limb_signed_t);
    pub fn arb_si_pow_ui(
        y: *mut arb_struct,
        b: mp_limb_signed_t,
        e: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_pow_fmpq(
        y: *mut arb_struct,
        x: *const arb_struct,
        a: *const fmpq,
        prec: mp_limb_signed_t,
    );
    pub fn arb_div_2expm1_ui(
        z: *mut arb_struct,
        x: *const arb_struct,
        n: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_pow(
        z: *mut arb_struct,
        x: *const arb_struct,
        y: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_root_ui(
        z: *mut arb_struct,
        x: *const arb_struct,
        k: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_root(z: *mut arb_struct, x: *const arb_struct, k: mp_limb_t, prec: mp_limb_signed_t);
    pub fn arb_log(z: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_log_arf(z: *mut arb_struct, x: *const arf_struct, prec: mp_limb_signed_t);
    pub fn arb_log_ui(z: *mut arb_struct, x: mp_limb_t, prec: mp_limb_signed_t);
    pub fn arb_log_fmpz(z: *mut arb_struct, x: *const fmpz, prec: mp_limb_signed_t);
    pub fn arb_log1p(r: *mut arb_struct, z: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_log_base_ui(
        res: *mut arb_struct,
        x: *const arb_struct,
        b: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_log_hypot(
        res: *mut arb_struct,
        x: *const arb_struct,
        y: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_exp(z: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_expm1(z: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_exp_invexp(
        z: *mut arb_struct,
        w: *const arb_struct,
        x: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_sin(s: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_cos(c: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_sin_cos(
        s: *mut arb_struct,
        c: *mut arb_struct,
        x: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_sin_pi(s: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_cos_pi(c: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_sin_cos_pi(
        s: *mut arb_struct,
        c: *mut arb_struct,
        x: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_tan(y: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_cot(y: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_tan_pi(y: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_cot_pi(y: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn _arb_sin_pi_fmpq_algebraic(
        s: *mut arb_struct,
        p: mp_limb_t,
        q: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_cos_pi_fmpq_algebraic(
        c: *mut arb_struct,
        p: mp_limb_t,
        q: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_sin_cos_pi_fmpq_algebraic(
        s: *mut arb_struct,
        c: *mut arb_struct,
        p: mp_limb_t,
        q: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_sin_cos_pi_fmpq(
        s: *mut arb_struct,
        c: *mut arb_struct,
        x: *const fmpq,
        prec: mp_limb_signed_t,
    );
    pub fn arb_sin_pi_fmpq(s: *mut arb_struct, x: *const fmpq, prec: mp_limb_signed_t);
    pub fn arb_cos_pi_fmpq(c: *mut arb_struct, x: *const fmpq, prec: mp_limb_signed_t);
    pub fn arb_sinc(z: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_sinc_pi(z: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_sinh(z: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_cosh(z: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_sinh_cosh(
        s: *mut arb_struct,
        c: *mut arb_struct,
        x: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_tanh(y: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_coth(y: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_atan_arf(z: *mut arb_struct, x: *const arf_struct, prec: mp_limb_signed_t);
    pub fn arb_atan(z: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_atan2(
        z: *mut arb_struct,
        b: *const arb_struct,
        a: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_asin(z: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_acos(z: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_atanh(z: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_asinh(z: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_acosh(z: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_sec(res: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_csc(res: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_csc_pi(res: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_sech(res: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_csch(res: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_fac_ui(z: *mut arb_struct, n: mp_limb_t, prec: mp_limb_signed_t);
    pub fn arb_doublefac_ui(z: *mut arb_struct, n: mp_limb_t, prec: mp_limb_signed_t);
    pub fn arb_bin_ui(
        z: *mut arb_struct,
        n: *const arb_struct,
        k: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_bin_uiui(z: *mut arb_struct, n: mp_limb_t, k: mp_limb_t, prec: mp_limb_signed_t);
    pub fn arb_fib_fmpz(z: *mut arb_struct, n: *const fmpz, prec: mp_limb_signed_t);
    pub fn arb_fib_ui(z: *mut arb_struct, n: mp_limb_t, prec: mp_limb_signed_t);
    pub fn arb_const_pi(z: *mut arb_struct, prec: mp_limb_signed_t);
    pub fn arb_const_sqrt_pi(z: *mut arb_struct, prec: mp_limb_signed_t);
    pub fn arb_const_log_sqrt2pi(z: *mut arb_struct, prec: mp_limb_signed_t);
    pub fn arb_const_log2(z: *mut arb_struct, prec: mp_limb_signed_t);
    pub fn arb_const_log10(z: *mut arb_struct, prec: mp_limb_signed_t);
    pub fn arb_const_euler(z: *mut arb_struct, prec: mp_limb_signed_t);
    pub fn arb_const_catalan(z: *mut arb_struct, prec: mp_limb_signed_t);
    pub fn arb_const_e(z: *mut arb_struct, prec: mp_limb_signed_t);
    pub fn arb_const_khinchin(z: *mut arb_struct, prec: mp_limb_signed_t);
    pub fn arb_const_glaisher(z: *mut arb_struct, prec: mp_limb_signed_t);
    pub fn arb_agm(
        z: *mut arb_struct,
        x: *const arb_struct,
        y: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_lgamma(z: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_rgamma(z: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_gamma(z: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_gamma_fmpq(z: *mut arb_struct, x: *const fmpq, prec: mp_limb_signed_t);
    pub fn arb_gamma_fmpz(z: *mut arb_struct, x: *const fmpz, prec: mp_limb_signed_t);
    pub fn arb_digamma(y: *mut arb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_zeta(z: *mut arb_struct, s: *const arb_struct, prec: mp_limb_signed_t);
    pub fn arb_hurwitz_zeta(
        z: *mut arb_struct,
        s: *const arb_struct,
        a: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_rising_ui_bs(
        y: *mut arb_struct,
        x: *const arb_struct,
        n: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_rising_ui_rs(
        y: *mut arb_struct,
        x: *const arb_struct,
        n: mp_limb_t,
        m: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_rising_ui_rec(
        y: *mut arb_struct,
        x: *const arb_struct,
        n: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_rising_ui(
        z: *mut arb_struct,
        x: *const arb_struct,
        n: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_rising_fmpq_ui(
        y: *mut arb_struct,
        x: *const fmpq,
        n: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_rising(
        z: *mut arb_struct,
        x: *const arb_struct,
        n: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_rising2_ui_rs(
        u: *mut arb_struct,
        v: *mut arb_struct,
        x: *const arb_struct,
        n: mp_limb_t,
        m: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_rising2_ui_bs(
        u: *mut arb_struct,
        v: *mut arb_struct,
        x: *const arb_struct,
        n: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_rising2_ui(
        u: *mut arb_struct,
        v: *mut arb_struct,
        x: *const arb_struct,
        n: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_log_ui_from_prev(
        s: *mut arb_struct,
        k: mp_limb_t,
        log_prev: *const arb_struct,
        prev: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_const_apery(s: *mut arb_struct, prec: mp_limb_signed_t);
    pub fn arb_zeta_ui_asymp(x: *mut arb_struct, s: mp_limb_t, prec: mp_limb_signed_t);
    pub fn arb_zeta_ui_borwein_bsplit(x: *mut arb_struct, s: mp_limb_t, prec: mp_limb_signed_t);
    pub fn arb_zeta_ui_euler_product(z: *mut arb_struct, s: mp_limb_t, prec: mp_limb_signed_t);
    pub fn arb_zeta_ui_bernoulli(x: *mut arb_struct, n: mp_limb_t, prec: mp_limb_signed_t);
    pub fn arb_zeta_ui_vec_borwein(
        z: arb_ptr,
        start: mp_limb_t,
        num: mp_limb_signed_t,
        step: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_zeta_ui(x: *mut arb_struct, n: mp_limb_t, prec: mp_limb_signed_t);
    pub fn arb_zeta_ui_vec_even(
        x: arb_ptr,
        start: mp_limb_t,
        num: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_zeta_ui_vec_odd(
        x: arb_ptr,
        start: mp_limb_t,
        num: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_zeta_ui_vec(
        x: arb_ptr,
        start: mp_limb_t,
        num: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_bernoulli_ui(b: *mut arb_struct, n: mp_limb_t, prec: mp_limb_signed_t);
    pub fn arb_bernoulli_ui_zeta(b: *mut arb_struct, n: mp_limb_t, prec: mp_limb_signed_t);
    pub fn arb_bernoulli_fmpz(b: *mut arb_struct, n: *const fmpz, prec: mp_limb_signed_t);
    pub fn arb_bernoulli_poly_ui(
        res: *mut arb_struct,
        n: mp_limb_t,
        x: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_polylog(
        w: *mut arb_struct,
        s: *const arb_struct,
        z: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_polylog_si(
        w: *mut arb_struct,
        s: mp_limb_signed_t,
        z: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_chebyshev_t_ui(
        a: *mut arb_struct,
        n: mp_limb_t,
        x: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_chebyshev_t2_ui(
        a: *mut arb_struct,
        b: *mut arb_struct,
        n: mp_limb_t,
        x: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_chebyshev_u_ui(
        a: *mut arb_struct,
        n: mp_limb_t,
        x: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_chebyshev_u2_ui(
        a: *mut arb_struct,
        b: *mut arb_struct,
        n: mp_limb_t,
        x: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_power_sum_vec(
        res: arb_ptr,
        a: *const arb_struct,
        b: *const arb_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_bell_sum_taylor(
        res: *mut arb_struct,
        n: *const fmpz,
        a: *const fmpz,
        b: *const fmpz,
        mmag: *const fmpz,
        prec: mp_limb_signed_t,
    );
    pub fn arb_bell_sum_bsplit(
        res: *mut arb_struct,
        n: *const fmpz,
        a: *const fmpz,
        b: *const fmpz,
        mmag: *const fmpz,
        prec: mp_limb_signed_t,
    );
    pub fn arb_bell_fmpz(res: *mut arb_struct, n: *const fmpz, prec: mp_limb_signed_t);
    pub fn arb_bell_ui(res: *mut arb_struct, n: mp_limb_t, prec: mp_limb_signed_t);
    pub fn arb_euler_number_fmpz(res: *mut arb_struct, n: *const fmpz, prec: mp_limb_signed_t);
    pub fn arb_euler_number_ui(res: *mut arb_struct, n: mp_limb_t, prec: mp_limb_signed_t);
    pub fn arb_partitions_fmpz(res: *mut arb_struct, n: *const fmpz, prec: mp_limb_signed_t);
    pub fn arb_partitions_ui(res: *mut arb_struct, n: mp_limb_t, prec: mp_limb_signed_t);
    pub fn arb_lambertw(
        res: *mut arb_struct,
        x: *const arb_struct,
        flags: c_int,
        prec: mp_limb_signed_t,
    );
    pub fn arb_sqr(res: *mut arb_struct, val: *const arb_struct, prec: mp_limb_signed_t);
    pub fn _arb_vec_entry_ptr(vec: arb_ptr, i: mp_limb_signed_t) -> arb_ptr;
    pub fn _arb_vec_printn(
        vec: arb_srcptr,
        len: mp_limb_signed_t,
        ndigits: mp_limb_signed_t,
        flags: mp_limb_t,
    );
    pub fn _arb_vec_zero(A: arb_ptr, n: mp_limb_signed_t);
    pub fn _arb_vec_is_zero(vec: arb_srcptr, len: mp_limb_signed_t) -> c_int;
    pub fn _arb_vec_is_finite(x: arb_srcptr, len: mp_limb_signed_t) -> c_int;
    pub fn _arb_vec_set(res: arb_ptr, vec: arb_srcptr, len: mp_limb_signed_t);
    pub fn _arb_vec_set_round(
        res: arb_ptr,
        vec: arb_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_vec_swap(res: arb_ptr, vec: arb_ptr, len: mp_limb_signed_t);
    pub fn _arb_vec_neg(B: arb_ptr, A: arb_srcptr, n: mp_limb_signed_t);
    pub fn _arb_vec_sub(
        C: arb_ptr,
        A: arb_srcptr,
        B: arb_srcptr,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_vec_add(
        C: arb_ptr,
        A: arb_srcptr,
        B: arb_srcptr,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_vec_scalar_mul(
        res: arb_ptr,
        vec: arb_srcptr,
        len: mp_limb_signed_t,
        c: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_vec_scalar_div(
        res: arb_ptr,
        vec: arb_srcptr,
        len: mp_limb_signed_t,
        c: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_vec_scalar_mul_fmpz(
        res: arb_ptr,
        vec: arb_srcptr,
        len: mp_limb_signed_t,
        c: *const fmpz,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_vec_scalar_mul_2exp_si(
        res: arb_ptr,
        src: arb_srcptr,
        len: mp_limb_signed_t,
        c: mp_limb_signed_t,
    );
    pub fn _arb_vec_scalar_addmul(
        res: arb_ptr,
        vec: arb_srcptr,
        len: mp_limb_signed_t,
        c: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_vec_get_mag(bound: *mut mag_struct, vec: arb_srcptr, len: mp_limb_signed_t);
    pub fn _arb_vec_bits(x: arb_srcptr, len: mp_limb_signed_t) -> mp_limb_signed_t;
    pub fn _arb_vec_set_powers(
        xs: arb_ptr,
        x: *const arb_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_vec_add_error_arf_vec(res: arb_ptr, err: arf_srcptr, len: mp_limb_signed_t);
    pub fn _arb_vec_add_error_mag_vec(res: arb_ptr, err: mag_srcptr, len: mp_limb_signed_t);
    pub fn _arb_vec_indeterminate(vec: arb_ptr, len: mp_limb_signed_t);
    pub fn _arb_vec_trim(res: arb_ptr, vec: arb_srcptr, len: mp_limb_signed_t);
    pub fn _arb_vec_get_unique_fmpz_vec(
        res: *mut fmpz,
        vec: arb_srcptr,
        len: mp_limb_signed_t,
    ) -> c_int;
    pub static mut arb_atan_tab1: [[mp_limb_t; 8usize]; 256usize];
    pub static mut arb_atan_tab21: [[mp_limb_t; 72usize]; 32usize];
    pub static mut arb_atan_tab22: [[mp_limb_t; 72usize]; 32usize];
    pub static arb_atan_pi2_minus_one: [mp_limb_t; 72usize];
    pub fn _arb_atan_taylor_naive(
        y: mp_ptr,
        error: *mut mp_limb_t,
        x: mp_srcptr,
        xn: mp_size_t,
        N: mp_limb_t,
        alternating: c_int,
    );
    pub fn _arb_atan_taylor_rs(
        y: mp_ptr,
        error: *mut mp_limb_t,
        x: mp_srcptr,
        xn: mp_size_t,
        N: mp_limb_t,
        alternating: c_int,
    );
    pub static mut arb_log_tab11: [[mp_limb_t; 8usize]; 128usize];
    pub static mut arb_log_tab12: [[mp_limb_t; 8usize]; 128usize];
    pub static mut arb_log_tab21: [[mp_limb_t; 72usize]; 32usize];
    pub static mut arb_log_tab22: [[mp_limb_t; 72usize]; 32usize];
    pub static arb_log_log2_tab: [mp_limb_t; 72usize];
    pub static mut arb_exp_tab1: [[mp_limb_t; 8usize]; 178usize];
    pub static mut arb_exp_tab21: [[mp_limb_t; 72usize]; 23usize];
    pub static mut arb_exp_tab22: [[mp_limb_t; 72usize]; 32usize];
    pub fn _arb_exp_taylor_naive(
        y: mp_ptr,
        error: *mut mp_limb_t,
        x: mp_srcptr,
        xn: mp_size_t,
        N: mp_limb_t,
    );
    pub fn _arb_exp_taylor_rs(
        y: mp_ptr,
        error: *mut mp_limb_t,
        x: mp_srcptr,
        xn: mp_size_t,
        N: mp_limb_t,
    );
    pub fn arb_exp_arf_bb(
        z: *mut arb_struct,
        x: *const arf_struct,
        prec: mp_limb_signed_t,
        minus_one: c_int,
    );
    pub fn arb_exp_arf_rs_generic(
        res: *mut arb_struct,
        x: *const arf_struct,
        prec: mp_limb_signed_t,
        minus_one: c_int,
    );
    pub fn _arb_get_mpn_fixed_mod_log2(
        w: mp_ptr,
        q: *mut fmpz,
        error: *mut mp_limb_t,
        x: *const arf_struct,
        wn: mp_size_t,
    ) -> c_int;
    pub fn _arb_exp_taylor_bound(mag: mp_limb_signed_t, prec: mp_limb_signed_t)
        -> mp_limb_signed_t;
    pub fn _arb_exp_sum_bs_powtab(
        T: *mut fmpz,
        Q: *mut fmpz,
        Qexp: *mut mp_limb_t,
        x: *const fmpz,
        r: mp_limb_t,
        N: mp_limb_signed_t,
    );
    pub fn _arb_exp_sum_bs_simple(
        T: *mut fmpz,
        Q: *mut fmpz,
        Qexp: *mut mp_limb_t,
        x: *const fmpz,
        r: mp_limb_t,
        N: mp_limb_signed_t,
    );
    pub static mut arb_sin_cos_tab1: [[mp_limb_t; 8usize]; 406usize];
    pub static mut arb_sin_cos_tab21: [[mp_limb_t; 72usize]; 52usize];
    pub static mut arb_sin_cos_tab22: [[mp_limb_t; 72usize]; 64usize];
    pub static arb_pi4_tab: [mp_limb_t; 72usize];
    pub fn _arb_sin_cos_taylor_naive(
        ysin: mp_ptr,
        ycos: mp_ptr,
        error: *mut mp_limb_t,
        x: mp_srcptr,
        xn: mp_size_t,
        N: mp_limb_t,
    );
    pub fn _arb_sin_cos_taylor_rs(
        ysin: mp_ptr,
        ycos: mp_ptr,
        error: *mut mp_limb_t,
        x: mp_srcptr,
        xn: mp_size_t,
        N: mp_limb_t,
        sinonly: c_int,
        alternating: c_int,
    );
    pub fn _arb_get_mpn_fixed_mod_pi4(
        w: mp_ptr,
        q: *mut fmpz,
        octant: *mut c_int,
        error: *mut mp_limb_t,
        x: *const arf_struct,
        wn: mp_size_t,
    ) -> c_int;
    pub fn arb_sin_cos_arf_bb(
        zsin: *mut arb_struct,
        zcos: *mut arb_struct,
        x: *const arf_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_sin_cos_arf_rs_generic(
        res_sin: *mut arb_struct,
        res_cos: *mut arb_struct,
        x: *const arf_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_sin_cos_arf_generic(
        res_sin: *mut arb_struct,
        res_cos: *mut arb_struct,
        x: *const arf_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_sin_cos_wide(
        s: *mut arb_struct,
        c: *mut arb_struct,
        x: *const arf_struct,
        r: *const mag_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_sin_cos_wide(
        s: *mut arb_struct,
        c: *mut arb_struct,
        x: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_sin_cos_generic(
        s: *mut arb_struct,
        c: *mut arb_struct,
        x: *const arf_struct,
        xrad: *const mag_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_sin_cos_generic(
        s: *mut arb_struct,
        c: *mut arb_struct,
        x: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_mpn_leading_zeros(d: mp_srcptr, n: mp_size_t) -> mp_limb_t;
    pub fn _arb_atan_sum_bs_simple(
        T: *mut fmpz,
        Q: *mut fmpz,
        Qexp: *mut mp_limb_t,
        x: *const fmpz,
        r: mp_limb_t,
        N: mp_limb_signed_t,
    );
    pub fn _arb_atan_sum_bs_powtab(
        T: *mut fmpz,
        Q: *mut fmpz,
        Qexp: *mut mp_limb_t,
        x: *const fmpz,
        r: mp_limb_t,
        N: mp_limb_signed_t,
    );
    pub fn arb_atan_arf_bb(z: *mut arb_struct, x: *const arf_struct, prec: mp_limb_signed_t);
    pub fn arb_allocated_bytes(x: *const arb_struct) -> mp_limb_signed_t;
    pub fn _arb_vec_allocated_bytes(vec: arb_srcptr, len: mp_limb_signed_t) -> mp_limb_signed_t;
    pub fn _arb_vec_estimate_allocated_bytes(len: mp_limb_signed_t, prec: mp_limb_signed_t) -> f64;
    pub fn arb_load_str(res: *mut arb_struct, data: *const c_char) -> c_int;
    pub fn arb_dump_str(x: *const arb_struct) -> *mut c_char;
    pub fn arb_load_file(res: *mut arb_struct, stream: *const FILE) -> c_int;
    pub fn arb_dump_file(stream: *mut FILE, x: *const arb_struct) -> c_int;
}
