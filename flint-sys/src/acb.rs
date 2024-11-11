#![allow(non_camel_case_types)]

//! *See the [Arb documentation](https://arblib.org/).

use crate::arb::{arb_ptr, arb_struct};
use crate::arf_types::{arf_srcptr, arf_struct};
use crate::mag::{mag_srcptr, mag_struct};
use crate::deps::*;
use crate::flint::*;
use crate::fmpq::fmpq;
use crate::fmpz::fmpz;
use libc::{c_int, FILE};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct acb_struct {
    pub real: arb_struct,
    pub imag: arb_struct,
}

pub type acb_t = [acb_struct; 1usize];
pub type acb_ptr = *mut acb_struct;
pub type acb_srcptr = *const acb_struct;

extern "C" {
    pub fn acb_init(x: *mut acb_struct);
    pub fn acb_clear(x: *mut acb_struct);
    pub fn _acb_vec_init(n: mp_limb_signed_t) -> acb_ptr;
    pub fn _acb_vec_clear(v: acb_ptr, n: mp_limb_signed_t);
    pub fn acb_real_ptr(z: *mut acb_struct) -> arb_ptr;
    pub fn acb_imag_ptr(z: *mut acb_struct) -> arb_ptr;
    pub fn acb_get_real(re: *mut arb_struct, z: *const acb_struct);
    pub fn acb_get_imag(im: *mut arb_struct, z: *const acb_struct);
    pub fn acb_get_mid(res: *mut acb_struct, x: *const acb_struct);
    pub fn acb_is_zero(z: *const acb_struct) -> c_int;
    pub fn acb_is_one(z: *const acb_struct) -> c_int;
    pub fn acb_is_exact(z: *const acb_struct) -> c_int;
    pub fn acb_is_int(z: *const acb_struct) -> c_int;
    pub fn acb_is_int_2exp_si(z: *const acb_struct, e: mp_limb_signed_t) -> c_int;
    pub fn acb_zero(z: *mut acb_struct);
    pub fn acb_one(z: *mut acb_struct);
    pub fn acb_onei(z: *mut acb_struct);
    pub fn acb_set(z: *mut acb_struct, x: *const acb_struct);
    pub fn acb_set_round(z: *mut acb_struct, x: *const acb_struct, prec: mp_limb_signed_t);
    pub fn acb_neg_round(z: *mut acb_struct, x: *const acb_struct, prec: mp_limb_signed_t);
    pub fn acb_swap(z: *mut acb_struct, x: *mut acb_struct);
    pub fn acb_equal(x: *const acb_struct, y: *const acb_struct) -> c_int;
    pub fn acb_equal_si(x: *const acb_struct, y: mp_limb_signed_t) -> c_int;
    pub fn acb_eq(x: *const acb_struct, y: *const acb_struct) -> c_int;
    pub fn acb_ne(x: *const acb_struct, y: *const acb_struct) -> c_int;
    pub fn acb_overlaps(x: *const acb_struct, y: *const acb_struct) -> c_int;
    pub fn acb_contains_zero(x: *const acb_struct) -> c_int;
    pub fn acb_contains_fmpq(x: *const acb_struct, y: *const fmpq) -> c_int;
    pub fn acb_contains_fmpz(x: *const acb_struct, y: *const fmpz) -> c_int;
    pub fn acb_contains(x: *const acb_struct, y: *const acb_struct) -> c_int;
    pub fn acb_contains_interior(x: *const acb_struct, y: *const acb_struct) -> c_int;
    pub fn acb_set_ui(z: *mut acb_struct, c: mp_limb_t);
    pub fn acb_set_d(z: *mut acb_struct, c: f64);
    pub fn acb_set_si(z: *mut acb_struct, c: mp_limb_signed_t);
    pub fn acb_set_si_si(z: *mut acb_struct, x: mp_limb_signed_t, y: mp_limb_signed_t);
    pub fn acb_set_d_d(z: *mut acb_struct, x: f64, y: f64);
    pub fn acb_set_fmpz(z: *mut acb_struct, c: *const fmpz);
    pub fn acb_set_fmpz_fmpz(z: *mut acb_struct, x: *const fmpz, y: *const fmpz);
    pub fn acb_set_round_fmpz(z: *mut acb_struct, y: *const fmpz, prec: mp_limb_signed_t);
    pub fn acb_contains_int(x: *const acb_struct) -> c_int;
    pub fn acb_get_unique_fmpz(z: *mut fmpz, x: *const acb_struct) -> c_int;
    pub fn acb_set_fmpq(z: *mut acb_struct, c: *const fmpq, prec: mp_limb_signed_t);
    pub fn acb_set_arb(z: *mut acb_struct, c: *const arb_struct);
    pub fn acb_set_arb_arb(z: *mut acb_struct, x: *const arb_struct, y: *const arb_struct);
    pub fn acb_set_round_arb(z: *mut acb_struct, x: *const arb_struct, prec: mp_limb_signed_t);
    pub fn acb_trim(z: *mut acb_struct, x: *const acb_struct);
    pub fn acb_add_error_arf(x: *mut acb_struct, err: *const arf_struct);
    pub fn acb_add_error_mag(x: *mut acb_struct, err: *const mag_struct);
    pub fn acb_get_mag(z: *mut mag_struct, x: *const acb_struct);
    pub fn acb_get_mag_lower(z: *mut mag_struct, x: *const acb_struct);
    pub fn acb_get_abs_ubound_arf(u: *mut arf_struct, z: *const acb_struct, prec: mp_limb_signed_t);
    pub fn acb_get_abs_lbound_arf(u: *mut arf_struct, z: *const acb_struct, prec: mp_limb_signed_t);
    pub fn acb_get_rad_ubound_arf(u: *mut arf_struct, z: *const acb_struct, prec: mp_limb_signed_t);
    pub fn acb_union(
        res: *mut acb_struct,
        x: *mut acb_struct,
        y: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_arg(r: *mut arb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_sgn(res: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_csgn(res: *mut arb_struct, z: *mut acb_struct);
    pub fn acb_real_abs(
        res: *mut acb_struct,
        z: *mut acb_struct,
        analytic: c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_real_sgn(
        res: *mut acb_struct,
        z: *mut acb_struct,
        analytic: c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_real_heaviside(
        res: *mut acb_struct,
        z: *mut acb_struct,
        analytic: c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_real_floor(
        res: *mut acb_struct,
        z: *mut acb_struct,
        analytic: c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_real_ceil(
        res: *mut acb_struct,
        z: *mut acb_struct,
        analytic: c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_real_max(
        res: *mut acb_struct,
        x: *mut acb_struct,
        y: *mut acb_struct,
        analytic: c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_real_min(
        res: *mut acb_struct,
        x: *mut acb_struct,
        y: *mut acb_struct,
        analytic: c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_real_sqrtpos(
        res: *mut acb_struct,
        z: *mut acb_struct,
        analytic: c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_sqrt_analytic(
        res: *mut acb_struct,
        z: *mut acb_struct,
        analytic: c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_rsqrt_analytic(
        res: *mut acb_struct,
        z: *mut acb_struct,
        analytic: c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_log_analytic(
        res: *mut acb_struct,
        z: *mut acb_struct,
        analytic: c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_pow_analytic(
        res: *mut acb_struct,
        z: *mut acb_struct,
        w: *mut acb_struct,
        analytic: c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_add(
        z: *mut acb_struct,
        x: *mut acb_struct,
        y: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_sub(
        z: *mut acb_struct,
        x: *mut acb_struct,
        y: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_add_si(z: *mut acb_struct, x: *mut acb_struct, c: mp_limb_t, prec: mp_limb_signed_t);
    pub fn acb_add_ui(z: *mut acb_struct, x: *mut acb_struct, c: mp_limb_t, prec: mp_limb_signed_t);
    pub fn acb_sub_si(z: *mut acb_struct, x: *mut acb_struct, c: mp_limb_t, prec: mp_limb_signed_t);
    pub fn acb_sub_ui(z: *mut acb_struct, x: *mut acb_struct, c: mp_limb_t, prec: mp_limb_signed_t);
    pub fn acb_add_fmpz(
        z: *mut acb_struct,
        x: *mut acb_struct,
        y: *mut fmpz,
        prec: mp_limb_signed_t,
    );
    pub fn acb_add_arb(
        z: *mut acb_struct,
        x: *mut acb_struct,
        y: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_sub_fmpz(
        z: *mut acb_struct,
        x: *mut acb_struct,
        y: *mut fmpz,
        prec: mp_limb_signed_t,
    );
    pub fn acb_sub_arb(
        z: *mut acb_struct,
        x: *mut acb_struct,
        y: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_neg(z: *mut acb_struct, x: *mut acb_struct);
    pub fn acb_conj(z: *mut acb_struct, x: *mut acb_struct);
    pub fn acb_abs(u: *mut arb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_mul_ui(z: *mut acb_struct, x: *mut acb_struct, y: mp_limb_t, prec: mp_limb_signed_t);
    pub fn acb_mul_si(
        z: *mut acb_struct,
        x: *mut acb_struct,
        y: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_mul_fmpz(
        z: *mut acb_struct,
        x: *mut acb_struct,
        y: *mut fmpz,
        prec: mp_limb_signed_t,
    );
    pub fn acb_mul_arb(
        z: *mut acb_struct,
        x: *mut acb_struct,
        y: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_mul_onei(z: *mut acb_struct, x: *mut acb_struct);
    pub fn acb_div_onei(z: *mut acb_struct, x: *mut acb_struct);
    pub fn acb_mul(
        z: *mut acb_struct,
        x: *mut acb_struct,
        y: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_mul_naive(
        z: *mut acb_struct,
        x: *mut acb_struct,
        y: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_mul_2exp_si(z: *mut acb_struct, x: *mut acb_struct, e: mp_limb_signed_t);
    pub fn acb_mul_2exp_fmpz(z: *mut acb_struct, x: *mut acb_struct, c: *mut fmpz);
    pub fn acb_addmul(
        z: *mut acb_struct,
        x: *mut acb_struct,
        y: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_submul(
        z: *mut acb_struct,
        x: *mut acb_struct,
        y: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_addmul_ui(
        z: *mut acb_struct,
        x: *mut acb_struct,
        y: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_addmul_si(
        z: *mut acb_struct,
        x: *mut acb_struct,
        y: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_submul_ui(
        z: *mut acb_struct,
        x: *mut acb_struct,
        y: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_submul_si(
        z: *mut acb_struct,
        x: *mut acb_struct,
        y: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_addmul_fmpz(
        z: *mut acb_struct,
        x: *mut acb_struct,
        y: *mut fmpz,
        prec: mp_limb_signed_t,
    );
    pub fn acb_submul_fmpz(
        z: *mut acb_struct,
        x: *mut acb_struct,
        y: *mut fmpz,
        prec: mp_limb_signed_t,
    );
    pub fn acb_addmul_arb(
        z: *mut acb_struct,
        x: *mut acb_struct,
        y: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_submul_arb(
        z: *mut acb_struct,
        x: *mut acb_struct,
        y: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dot_simple(
        res: *mut acb_struct,
        initial: *mut acb_struct,
        subtract: c_int,
        x: acb_srcptr,
        xstep: mp_limb_signed_t,
        y: acb_srcptr,
        ystep: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dot_precise(
        res: *mut acb_struct,
        initial: *mut acb_struct,
        subtract: c_int,
        x: acb_srcptr,
        xstep: mp_limb_signed_t,
        y: acb_srcptr,
        ystep: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dot(
        res: *mut acb_struct,
        initial: *mut acb_struct,
        subtract: c_int,
        x: acb_srcptr,
        xstep: mp_limb_signed_t,
        y: acb_srcptr,
        ystep: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_approx_dot(
        res: *mut acb_struct,
        initial: *mut acb_struct,
        subtract: c_int,
        x: acb_srcptr,
        xstep: mp_limb_signed_t,
        y: acb_srcptr,
        ystep: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dot_ui(
        res: *mut acb_struct,
        initial: *mut acb_struct,
        subtract: c_int,
        x: acb_srcptr,
        xstep: mp_limb_signed_t,
        y: *const mp_limb_t,
        ystep: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dot_si(
        res: *mut acb_struct,
        initial: *mut acb_struct,
        subtract: c_int,
        x: acb_srcptr,
        xstep: mp_limb_signed_t,
        y: *const mp_limb_signed_t,
        ystep: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dot_uiui(
        res: *mut acb_struct,
        initial: *mut acb_struct,
        subtract: c_int,
        x: acb_srcptr,
        xstep: mp_limb_signed_t,
        y: *const mp_limb_t,
        ystep: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dot_siui(
        res: *mut acb_struct,
        initial: *mut acb_struct,
        subtract: c_int,
        x: acb_srcptr,
        xstep: mp_limb_signed_t,
        y: *const mp_limb_t,
        ystep: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dot_fmpz(
        res: *mut acb_struct,
        initial: *mut acb_struct,
        subtract: c_int,
        x: acb_srcptr,
        xstep: mp_limb_signed_t,
        y: *const fmpz,
        ystep: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_inv(z: *mut acb_struct, x: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_div(
        z: *mut acb_struct,
        x: *mut acb_struct,
        y: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_div_ui(z: *mut acb_struct, x: *mut acb_struct, c: mp_limb_t, prec: mp_limb_signed_t);
    pub fn acb_div_si(
        z: *mut acb_struct,
        x: *mut acb_struct,
        c: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_div_arb(
        z: *mut acb_struct,
        x: *mut acb_struct,
        c: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_div_fmpz(
        z: *mut acb_struct,
        x: *mut acb_struct,
        c: *mut fmpz,
        prec: mp_limb_signed_t,
    );
    pub fn acb_cube(y: *mut acb_struct, x: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_pow_fmpz(
        y: *mut acb_struct,
        b: *mut acb_struct,
        e: *mut fmpz,
        prec: mp_limb_signed_t,
    );
    pub fn acb_pow_ui(y: *mut acb_struct, b: *mut acb_struct, e: mp_limb_t, prec: mp_limb_signed_t);
    pub fn acb_pow_si(
        y: *mut acb_struct,
        b: *mut acb_struct,
        e: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_const_pi(x: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_log(r: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_log1p(r: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_exp(r: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_exp_pi_i(r: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_exp_invexp(
        r: *mut acb_struct,
        s: *mut acb_struct,
        z: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_expm1(r: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_sin(r: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_cos(r: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_sin_cos(
        s: *mut acb_struct,
        c: *mut acb_struct,
        z: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_tan(r: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_cot(r: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_asin(r: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_acos(r: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_atan(r: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_asinh(r: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_acosh(r: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_atanh(r: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_sinh(y: *mut acb_struct, x: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_cosh(y: *mut acb_struct, x: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_sinh_cosh(
        y: *mut acb_struct,
        z: *mut acb_struct,
        x: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_tanh(y: *mut acb_struct, x: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_coth(y: *mut acb_struct, x: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_sech(r: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_csch(r: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_sec(y: *mut acb_struct, x: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_csc(y: *mut acb_struct, x: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_sin_pi(r: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_cos_pi(r: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_sin_cos_pi(
        s: *mut acb_struct,
        c: *mut acb_struct,
        z: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_tan_pi(r: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_cot_pi(r: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_csc_pi(y: *mut acb_struct, x: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_sinc(res: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_sinc_pi(res: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_pow_arb(
        z: *mut acb_struct,
        x: *mut acb_struct,
        y: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_pow(
        r: *mut acb_struct,
        x: *mut acb_struct,
        y: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_sqrt(y: *mut acb_struct, x: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_rsqrt(y: *mut acb_struct, x: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_root_ui(
        y: *mut acb_struct,
        x: *mut acb_struct,
        k: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_quadratic_roots_fmpz(
        r1: *mut acb_struct,
        r2: *mut acb_struct,
        a: *mut fmpz,
        b: *mut fmpz,
        c: *mut fmpz,
        prec: mp_limb_signed_t,
    );
    pub fn acb_chebyshev_t_ui(
        a: *mut acb_struct,
        n: mp_limb_t,
        x: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_chebyshev_t2_ui(
        a: *mut acb_struct,
        b: *mut acb_struct,
        n: mp_limb_t,
        x: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_chebyshev_u_ui(
        a: *mut acb_struct,
        n: mp_limb_t,
        x: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_chebyshev_u2_ui(
        a: *mut acb_struct,
        b: *mut acb_struct,
        n: mp_limb_t,
        x: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_rising_ui_bs(
        y: *mut acb_struct,
        x: *mut acb_struct,
        n: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_rising_ui_rs(
        y: *mut acb_struct,
        x: *mut acb_struct,
        n: mp_limb_t,
        m: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_rising_ui_rec(
        y: *mut acb_struct,
        x: *mut acb_struct,
        n: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_rising_ui(
        z: *mut acb_struct,
        x: *mut acb_struct,
        n: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_rising(
        z: *mut acb_struct,
        x: *mut acb_struct,
        n: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_rising2_ui_bs(
        u: *mut acb_struct,
        v: *mut acb_struct,
        x: *mut acb_struct,
        n: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_rising2_ui_rs(
        u: *mut acb_struct,
        v: *mut acb_struct,
        x: *mut acb_struct,
        n: mp_limb_t,
        m: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_rising2_ui(
        u: *mut acb_struct,
        v: *mut acb_struct,
        x: *mut acb_struct,
        n: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_rising_ui_get_mag(bound: *mut mag_struct, s: *mut acb_struct, n: mp_limb_t);
    pub fn acb_gamma(y: *mut acb_struct, x: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_rgamma(y: *mut acb_struct, x: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_lgamma(y: *mut acb_struct, x: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_log_sin_pi(res: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_digamma(y: *mut acb_struct, x: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_zeta(z: *mut acb_struct, s: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_hurwitz_zeta(
        z: *mut acb_struct,
        s: *mut acb_struct,
        a: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_polygamma(
        res: *mut acb_struct,
        s: *mut acb_struct,
        z: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_bernoulli_poly_ui(
        res: *mut acb_struct,
        n: mp_limb_t,
        x: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_log_barnes_g(res: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_barnes_g(res: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_polylog(
        w: *mut acb_struct,
        s: *mut acb_struct,
        z: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_polylog_si(
        w: *mut acb_struct,
        s: mp_limb_signed_t,
        z: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_agm1(m: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_agm1_cpx(
        m: acb_ptr,
        z: *mut acb_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_agm(
        res: *mut acb_struct,
        a: *mut acb_struct,
        b: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_lambertw_asymp(
        res: *mut acb_struct,
        z: *mut acb_struct,
        k: *mut fmpz,
        L: mp_limb_signed_t,
        M: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_lambertw_check_branch(
        w: *mut acb_struct,
        k: *mut fmpz,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn acb_lambertw_bound_deriv(
        res: *mut mag_struct,
        z: *mut acb_struct,
        ez1: *mut acb_struct,
        k: *mut fmpz,
    );
    pub fn acb_lambertw(
        res: *mut acb_struct,
        z: *mut acb_struct,
        k: *mut fmpz,
        flags: c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_sqr(res: *mut acb_struct, val: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_is_finite(x: *mut acb_struct) -> c_int;
    pub fn acb_indeterminate(x: *mut acb_struct);
    pub fn _acb_vec_entry_ptr(vec: acb_ptr, i: mp_limb_signed_t) -> acb_ptr;
    pub fn _acb_vec_zero(A: acb_ptr, n: mp_limb_signed_t);
    pub fn _acb_vec_is_zero(vec: acb_srcptr, len: mp_limb_signed_t) -> c_int;
    pub fn _acb_vec_set(res: acb_ptr, vec: acb_srcptr, len: mp_limb_signed_t);
    pub fn _acb_vec_set_round(
        res: acb_ptr,
        vec: acb_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_vec_neg(res: acb_ptr, vec: acb_srcptr, len: mp_limb_signed_t);
    pub fn _acb_vec_add(
        res: acb_ptr,
        vec1: acb_srcptr,
        vec2: acb_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_vec_sub(
        res: acb_ptr,
        vec1: acb_srcptr,
        vec2: acb_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_vec_scalar_submul(
        res: acb_ptr,
        vec: acb_srcptr,
        len: mp_limb_signed_t,
        c: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_vec_scalar_addmul(
        res: acb_ptr,
        vec: acb_srcptr,
        len: mp_limb_signed_t,
        c: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_vec_scalar_mul(
        res: acb_ptr,
        vec: acb_srcptr,
        len: mp_limb_signed_t,
        c: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_vec_scalar_mul_ui(
        res: acb_ptr,
        vec: acb_srcptr,
        len: mp_limb_signed_t,
        c: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_vec_scalar_mul_2exp_si(
        res: acb_ptr,
        vec: acb_srcptr,
        len: mp_limb_signed_t,
        c: mp_limb_signed_t,
    );
    pub fn _acb_vec_scalar_mul_onei(res: acb_ptr, vec: acb_srcptr, len: mp_limb_signed_t);
    pub fn _acb_vec_scalar_div_ui(
        res: acb_ptr,
        vec: acb_srcptr,
        len: mp_limb_signed_t,
        c: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_vec_scalar_div(
        res: acb_ptr,
        vec: acb_srcptr,
        len: mp_limb_signed_t,
        c: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_vec_scalar_mul_arb(
        res: acb_ptr,
        vec: acb_srcptr,
        len: mp_limb_signed_t,
        c: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_vec_scalar_div_arb(
        res: acb_ptr,
        vec: acb_srcptr,
        len: mp_limb_signed_t,
        c: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_vec_scalar_mul_fmpz(
        res: acb_ptr,
        vec: acb_srcptr,
        len: mp_limb_signed_t,
        c: *mut fmpz,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_vec_scalar_div_fmpz(
        res: acb_ptr,
        vec: acb_srcptr,
        len: mp_limb_signed_t,
        c: *mut fmpz,
        prec: mp_limb_signed_t,
    );
    pub fn acb_fprint(file: *mut FILE, x: *mut acb_struct);
    pub fn acb_print(x: *mut acb_struct);
    pub fn acb_fprintd(file: *mut FILE, z: *mut acb_struct, digits: mp_limb_signed_t);
    pub fn acb_printd(z: *mut acb_struct, digits: mp_limb_signed_t);
    pub fn acb_fprintn(
        fp: *mut FILE,
        z: *mut acb_struct,
        digits: mp_limb_signed_t,
        flags: mp_limb_t,
    );
    pub fn acb_printn(x: *mut acb_struct, digits: mp_limb_signed_t, flags: mp_limb_t);
    pub fn acb_randtest(
        z: *mut acb_struct,
        state: *mut flint_rand_s,
        prec: mp_limb_signed_t,
        mag_bits: mp_limb_signed_t,
    );
    pub fn acb_randtest_special(
        z: *mut acb_struct,
        state: *mut flint_rand_s,
        prec: mp_limb_signed_t,
        mag_bits: mp_limb_signed_t,
    );
    pub fn acb_randtest_precise(
        z: *mut acb_struct,
        state: *mut flint_rand_s,
        prec: mp_limb_signed_t,
        mag_bits: mp_limb_signed_t,
    );
    pub fn acb_randtest_param(
        z: *mut acb_struct,
        state: *mut flint_rand_s,
        prec: mp_limb_signed_t,
        mag_bits: mp_limb_signed_t,
    );
    pub fn acb_rel_error_bits(x: *mut acb_struct) -> mp_limb_signed_t;
    pub fn acb_rel_accuracy_bits(x: *mut acb_struct) -> mp_limb_signed_t;
    pub fn acb_rel_one_accuracy_bits(x: *mut acb_struct) -> mp_limb_signed_t;
    pub fn acb_bits(x: *mut acb_struct) -> mp_limb_signed_t;
    pub fn acb_is_real(x: *mut acb_struct) -> c_int;
    pub fn _acb_vec_is_real(v: acb_srcptr, len: mp_limb_signed_t) -> c_int;
    pub fn _acb_vec_bits(vec: acb_srcptr, len: mp_limb_signed_t) -> mp_limb_signed_t;
    pub fn _acb_vec_set_powers(
        xs: acb_ptr,
        x: *mut acb_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_vec_add_error_arf_vec(res: acb_ptr, err: arf_srcptr, len: mp_limb_signed_t);
    pub fn _acb_vec_add_error_mag_vec(res: acb_ptr, err: mag_srcptr, len: mp_limb_signed_t);
    pub fn _acb_vec_indeterminate(vec: acb_ptr, len: mp_limb_signed_t);
    pub fn _acb_vec_trim(res: acb_ptr, vec: acb_srcptr, len: mp_limb_signed_t);
    pub fn _acb_vec_get_unique_fmpz_vec(
        res: *mut fmpz,
        vec: acb_srcptr,
        len: mp_limb_signed_t,
    ) -> c_int;
    pub fn _acb_vec_sort_pretty(vec: acb_ptr, len: mp_limb_signed_t);
    pub fn acb_unit_root(res: *mut acb_struct, order: mp_limb_t, prec: mp_limb_signed_t);
    pub fn _acb_vec_unit_roots(
        z: acb_ptr,
        order: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_allocated_bytes(x: *mut acb_struct) -> mp_limb_signed_t;
    pub fn _acb_vec_allocated_bytes(vec: acb_srcptr, len: mp_limb_signed_t) -> mp_limb_signed_t;
    pub fn _acb_vec_estimate_allocated_bytes(len: mp_limb_signed_t, prec: mp_limb_signed_t) -> f64;
}
