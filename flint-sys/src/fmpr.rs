#![allow(non_camel_case_types)]

//! *See the [Arb documentation](https://arblib.org/).

use flint_sys::deps::*;
use flint_sys::flint::*;
use flint_sys::fmpq::fmpq;
use flint_sys::fmpz::fmpz;
use libc::c_int;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpr_struct {
    pub man: fmpz,
    pub exp: fmpz,
}

pub type fmpr_t = [fmpr_struct; 1usize];
pub type fmpr_ptr = *mut fmpr_struct;
pub type fmpr_srcptr = *const fmpr_struct;

extern "C" {
    pub fn _fmpr_normalise_naive(
        man: *mut fmpz,
        exp: *mut fmpz,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn _fmpr_set_round(
        rman: *mut fmpz,
        rexp: *mut fmpz,
        man: *mut fmpz,
        exp: *mut fmpz,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn _fmpr_set_round_mpn(
        shift: *mut mp_limb_signed_t,
        man: *mut fmpz,
        x: mp_srcptr,
        xn: mp_size_t,
        negative: c_int,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_set_round_ui_2exp_fmpz(
        z: *mut fmpr_struct,
        lo: mp_limb_t,
        exp: *mut fmpz,
        negative: c_int,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_set_round_uiui_2exp_fmpz(
        z: *mut fmpr_struct,
        hi: mp_limb_t,
        lo: mp_limb_t,
        exp: *mut fmpz,
        negative: c_int,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_ulp(u: *mut fmpr_struct, x: *mut fmpr_struct, prec: mp_limb_signed_t);
    pub fn fmpr_check_ulp(
        result: *mut fmpr_struct,
        r: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn fmpr_cmp(x: *mut fmpr_struct, y: *mut fmpr_struct) -> c_int;
    pub fn fmpr_cmpabs(x: *mut fmpr_struct, y: *mut fmpr_struct) -> c_int;
    pub fn fmpr_cmpabs_ui(x: *mut fmpr_struct, y: mp_limb_t) -> c_int;
    pub fn fmpr_randtest(
        x: *mut fmpr_struct,
        state: *mut flint_rand_s,
        bits: mp_limb_signed_t,
        exp_bits: mp_limb_signed_t,
    );
    pub fn fmpr_randtest_not_zero(
        x: *mut fmpr_struct,
        state: *mut flint_rand_s,
        bits: mp_limb_signed_t,
        exp_bits: mp_limb_signed_t,
    );
    pub fn fmpr_randtest_special(
        x: *mut fmpr_struct,
        state: *mut flint_rand_s,
        bits: mp_limb_signed_t,
        exp_bits: mp_limb_signed_t,
    );
    pub fn fmpr_get_mpfr(x: *mut __mpfr_struct, y: *mut fmpr_struct, rnd: mpfr_rnd_t) -> c_int;
    pub fn fmpr_set_mpfr(x: *mut fmpr_struct, y: *mut __mpfr_struct);
    pub fn fmpr_get_d(x: *mut fmpr_struct, rnd: c_int) -> f64;
    pub fn fmpr_set_d(x: *mut fmpr_struct, v: f64);
    pub fn _fmpr_add_eps(
        z: *mut fmpr_struct,
        x: *mut fmpr_struct,
        sign: c_int,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn _fmpr_add_mpn(
        z: *mut fmpr_struct,
        xman: mp_srcptr,
        xn: mp_size_t,
        xsign: c_int,
        xexp: *mut fmpz,
        yman: mp_srcptr,
        yn: mp_size_t,
        ysign: c_int,
        yexp: *mut fmpz,
        shift: mp_limb_signed_t,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn _fmpr_add_1x1(
        z: *mut fmpr_struct,
        x: mp_limb_t,
        xsign: c_int,
        xexp: *mut fmpz,
        y: mp_limb_t,
        ysign: c_int,
        yexp: *mut fmpz,
        shift: mp_limb_signed_t,
        prec: mp_limb_signed_t,
        rnd: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn fmpr_add_naive(
        z: *mut fmpr_struct,
        x: *mut fmpr_struct,
        y: *mut fmpr_struct,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn _fmpr_mul_mpn(
        z: *mut fmpr_struct,
        xman: mp_srcptr,
        xn: mp_size_t,
        xexp: *mut fmpz,
        yman: mp_srcptr,
        yn: mp_size_t,
        yexp: *mut fmpz,
        negative: c_int,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn _fmpr_mul_1x1(
        z: *mut fmpr_struct,
        u: mp_limb_t,
        xexp: *mut fmpz,
        v: mp_limb_t,
        yexp: *mut fmpz,
        negative: c_int,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_mul_naive(
        z: *mut fmpr_struct,
        x: *mut fmpr_struct,
        y: *mut fmpr_struct,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_mul(
        z: *mut fmpr_struct,
        x: *mut fmpr_struct,
        y: *mut fmpr_struct,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_mul_ui(
        z: *mut fmpr_struct,
        x: *mut fmpr_struct,
        y: mp_limb_t,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_mul_si(
        z: *mut fmpr_struct,
        x: *mut fmpr_struct,
        y: mp_limb_signed_t,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_mul_fmpz(
        z: *mut fmpr_struct,
        x: *mut fmpr_struct,
        y: *mut fmpz,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_add(
        z: *mut fmpr_struct,
        x: *mut fmpr_struct,
        y: *mut fmpr_struct,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_add_ui(
        z: *mut fmpr_struct,
        x: *mut fmpr_struct,
        y: mp_limb_t,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_add_si(
        z: *mut fmpr_struct,
        x: *mut fmpr_struct,
        y: mp_limb_signed_t,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_add_fmpz(
        z: *mut fmpr_struct,
        x: *mut fmpr_struct,
        y: *mut fmpz,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_sub(
        z: *mut fmpr_struct,
        x: *mut fmpr_struct,
        y: *mut fmpr_struct,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_sub_ui(
        z: *mut fmpr_struct,
        x: *mut fmpr_struct,
        y: mp_limb_t,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_sub_si(
        z: *mut fmpr_struct,
        x: *mut fmpr_struct,
        y: mp_limb_signed_t,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_sub_fmpz(
        z: *mut fmpr_struct,
        x: *mut fmpr_struct,
        y: *mut fmpz,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_div(
        z: *mut fmpr_struct,
        x: *mut fmpr_struct,
        y: *mut fmpr_struct,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_div_ui(
        z: *mut fmpr_struct,
        x: *mut fmpr_struct,
        y: mp_limb_t,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_ui_div(
        z: *mut fmpr_struct,
        x: mp_limb_t,
        y: *mut fmpr_struct,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_div_si(
        z: *mut fmpr_struct,
        x: *mut fmpr_struct,
        y: mp_limb_signed_t,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_si_div(
        z: *mut fmpr_struct,
        x: mp_limb_signed_t,
        y: *mut fmpr_struct,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_div_fmpz(
        z: *mut fmpr_struct,
        x: *mut fmpr_struct,
        y: *mut fmpz,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_fmpz_div(
        z: *mut fmpr_struct,
        x: *mut fmpz,
        y: *mut fmpr_struct,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_fmpz_div_fmpz(
        z: *mut fmpr_struct,
        x: *mut fmpz,
        y: *mut fmpz,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_addmul(
        z: *mut fmpr_struct,
        x: *mut fmpr_struct,
        y: *mut fmpr_struct,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_addmul_ui(
        z: *mut fmpr_struct,
        x: *mut fmpr_struct,
        y: mp_limb_t,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_addmul_si(
        z: *mut fmpr_struct,
        x: *mut fmpr_struct,
        y: mp_limb_signed_t,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_addmul_fmpz(
        z: *mut fmpr_struct,
        x: *mut fmpr_struct,
        y: *mut fmpz,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_submul(
        z: *mut fmpr_struct,
        x: *mut fmpr_struct,
        y: *mut fmpr_struct,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_submul_ui(
        z: *mut fmpr_struct,
        x: *mut fmpr_struct,
        y: mp_limb_t,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_submul_si(
        z: *mut fmpr_struct,
        x: *mut fmpr_struct,
        y: mp_limb_signed_t,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_submul_fmpz(
        z: *mut fmpr_struct,
        x: *mut fmpr_struct,
        y: *mut fmpz,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_sqrt(
        y: *mut fmpr_struct,
        x: *mut fmpr_struct,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_sqrt_ui(
        z: *mut fmpr_struct,
        x: mp_limb_t,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_sqrt_fmpz(
        z: *mut fmpr_struct,
        x: *mut fmpz,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_rsqrt(
        y: *mut fmpr_struct,
        x: *mut fmpr_struct,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_root(
        y: *mut fmpr_struct,
        x: *mut fmpr_struct,
        k: mp_limb_t,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_log(
        y: *mut fmpr_struct,
        x: *mut fmpr_struct,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_log1p(
        y: *mut fmpr_struct,
        x: *mut fmpr_struct,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_exp(
        y: *mut fmpr_struct,
        x: *mut fmpr_struct,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_expm1(
        y: *mut fmpr_struct,
        x: *mut fmpr_struct,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_print(x: *mut fmpr_struct);
    pub fn fmpr_printd(x: *mut fmpr_struct, digits: mp_limb_signed_t);
    pub fn fmpr_get_fmpq(y: *mut fmpq, x: *mut fmpr_struct);
    pub fn fmpr_set_fmpq(
        x: *mut fmpr_struct,
        y: *mut fmpq,
        prec: mp_limb_signed_t,
        rnd: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpr_get_fmpz(z: *mut fmpz, x: *mut fmpr_struct, rnd: c_int);
    pub fn fmpr_get_si(x: *mut fmpr_struct, rnd: c_int) -> mp_limb_signed_t;
    pub fn fmpr_set_fmpz_2exp(x: *mut fmpr_struct, man: *mut fmpz, exp: *mut fmpz);
    pub fn fmpr_get_fmpz_2exp(man: *mut fmpz, exp: *mut fmpz, x: *mut fmpr_struct);
    pub fn fmpr_get_fmpz_fixed_fmpz(y: *mut fmpz, x: *mut fmpr_struct, e: *mut fmpz) -> c_int;
    pub fn fmpr_get_fmpz_fixed_si(y: *mut fmpz, x: *mut fmpr_struct, e: mp_limb_signed_t) -> c_int;
    pub fn fmpr_cmp_2exp_si(x: *mut fmpr_struct, e: mp_limb_signed_t) -> c_int;
    pub fn fmpr_cmpabs_2exp_si(x: *mut fmpr_struct, e: mp_limb_signed_t) -> c_int;
    pub fn fmpr_pow_sloppy_fmpz(
        y: *mut fmpr_struct,
        b: *mut fmpr_struct,
        e: *mut fmpz,
        prec: mp_limb_signed_t,
        rnd: c_int,
    );
    pub fn fmpr_pow_sloppy_ui(
        y: *mut fmpr_struct,
        b: *mut fmpr_struct,
        e: mp_limb_t,
        prec: mp_limb_signed_t,
        rnd: c_int,
    );
    pub fn fmpr_pow_sloppy_si(
        y: *mut fmpr_struct,
        b: *mut fmpr_struct,
        e: mp_limb_signed_t,
        prec: mp_limb_signed_t,
        rnd: c_int,
    );
}
