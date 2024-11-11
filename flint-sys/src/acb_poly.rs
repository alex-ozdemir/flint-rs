#![allow(non_camel_case_types)]

//! *See the [Arb documentation](https://arblib.org/).

use crate::acb::{acb_ptr, acb_srcptr, acb_struct};
use crate::arb::arb_ptr;
use crate::arb_poly::arb_poly_struct;
use crate::mag::mag_struct;
use crate::deps::*;
use crate::flint::*;
use crate::fmpq_poly::fmpq_poly_struct;
use crate::fmpz::fmpz;
use crate::fmpz_poly::fmpz_poly_struct;
use libc::{c_int, FILE};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acb_poly_struct {
    pub coeffs: acb_ptr,
    pub length: mp_limb_signed_t,
    pub alloc: mp_limb_signed_t,
}

pub type acb_poly_t = [acb_poly_struct; 1usize];

extern "C" {
    pub fn acb_poly_init(poly: *mut acb_poly_struct);
    pub fn acb_poly_init2(poly: *mut acb_poly_struct, len: mp_limb_signed_t);
    pub fn acb_poly_clear(poly: *mut acb_poly_struct);
    pub fn acb_poly_fit_length(poly: *mut acb_poly_struct, len: mp_limb_signed_t);
    pub fn _acb_poly_set_length(poly: *mut acb_poly_struct, len: mp_limb_signed_t);
    pub fn _acb_poly_normalise(poly: *mut acb_poly_struct);
    pub fn acb_poly_valuation(poly: *mut acb_poly_struct) -> mp_limb_signed_t;
    pub fn acb_poly_set_coeff_si(
        poly: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        x: mp_limb_signed_t,
    );
    pub fn acb_poly_set_coeff_acb(
        poly: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        x: *mut acb_struct,
    );
    pub fn acb_poly_get_coeff_acb(
        x: *mut acb_struct,
        poly: *mut acb_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _acb_poly_shift_right(
        res: acb_ptr,
        poly: acb_srcptr,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn acb_poly_shift_right(
        res: *mut acb_poly_struct,
        poly: *mut acb_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _acb_poly_shift_left(
        res: acb_ptr,
        poly: acb_srcptr,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn acb_poly_shift_left(
        res: *mut acb_poly_struct,
        poly: *mut acb_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _acb_poly_majorant(
        res: arb_ptr,
        vec: acb_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_majorant(
        res: *mut arb_poly_struct,
        poly: *mut acb_poly_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_fprintd(file: *mut FILE, poly: *mut acb_poly_struct, digits: mp_limb_signed_t);
    pub fn _acb_poly_evaluate_horner(
        res: *mut acb_struct,
        f: acb_srcptr,
        len: mp_limb_signed_t,
        a: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_evaluate_horner(
        res: *mut acb_struct,
        f: *mut acb_poly_struct,
        a: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_evaluate_rectangular(
        y: *mut acb_struct,
        poly: acb_srcptr,
        len: mp_limb_signed_t,
        x: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_evaluate_rectangular(
        res: *mut acb_struct,
        f: *mut acb_poly_struct,
        a: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_evaluate(
        res: *mut acb_struct,
        f: acb_srcptr,
        len: mp_limb_signed_t,
        a: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_evaluate(
        res: *mut acb_struct,
        f: *mut acb_poly_struct,
        a: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_evaluate2_horner(
        y: *mut acb_struct,
        z: *mut acb_struct,
        f: acb_srcptr,
        len: mp_limb_signed_t,
        x: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_evaluate2_horner(
        y: *mut acb_struct,
        z: *mut acb_struct,
        f: *mut acb_poly_struct,
        x: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_evaluate2_rectangular(
        y: *mut acb_struct,
        z: *mut acb_struct,
        f: acb_srcptr,
        len: mp_limb_signed_t,
        x: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_evaluate2_rectangular(
        y: *mut acb_struct,
        z: *mut acb_struct,
        f: *mut acb_poly_struct,
        x: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_evaluate2(
        y: *mut acb_struct,
        z: *mut acb_struct,
        f: acb_srcptr,
        len: mp_limb_signed_t,
        x: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_evaluate2(
        y: *mut acb_struct,
        z: *mut acb_struct,
        f: *mut acb_poly_struct,
        x: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_derivative(
        res: acb_ptr,
        poly: acb_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_derivative(
        res: *mut acb_poly_struct,
        poly: *mut acb_poly_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_integral(
        res: acb_ptr,
        poly: acb_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_integral(
        res: *mut acb_poly_struct,
        poly: *mut acb_poly_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_borel_transform(
        res: *mut acb_poly_struct,
        poly: *mut acb_poly_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_borel_transform(
        res: acb_ptr,
        poly: acb_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_inv_borel_transform(
        res: *mut acb_poly_struct,
        poly: *mut acb_poly_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_inv_borel_transform(
        res: acb_ptr,
        poly: acb_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_binomial_transform_basecase(
        b: acb_ptr,
        a: acb_srcptr,
        alen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_binomial_transform_basecase(
        b: *mut acb_poly_struct,
        a: *mut acb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_binomial_transform_convolution(
        b: acb_ptr,
        a: acb_srcptr,
        alen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_binomial_transform_convolution(
        b: *mut acb_poly_struct,
        a: *mut acb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_binomial_transform(
        b: acb_ptr,
        a: acb_srcptr,
        alen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_binomial_transform(
        b: *mut acb_poly_struct,
        a: *mut acb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_set(dest: *mut acb_poly_struct, src: *mut acb_poly_struct);
    pub fn acb_poly_set_round(
        dest: *mut acb_poly_struct,
        src: *mut acb_poly_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_set_trunc(
        res: *mut acb_poly_struct,
        poly: *mut acb_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn acb_poly_set_trunc_round(
        res: *mut acb_poly_struct,
        poly: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_set_arb_poly(poly: *mut acb_poly_struct, re: *mut arb_poly_struct);
    pub fn acb_poly_set2_arb_poly(
        poly: *mut acb_poly_struct,
        re: *mut arb_poly_struct,
        im: *mut arb_poly_struct,
    );
    pub fn acb_poly_set_fmpq_poly(
        poly: *mut acb_poly_struct,
        re: *mut fmpq_poly_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_set2_fmpq_poly(
        poly: *mut acb_poly_struct,
        re: *mut fmpq_poly_struct,
        im: *mut fmpq_poly_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_set_fmpz_poly(
        poly: *mut acb_poly_struct,
        src: *mut fmpz_poly_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_set2_fmpz_poly(
        poly: *mut acb_poly_struct,
        re: *mut fmpz_poly_struct,
        im: *mut fmpz_poly_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_get_unique_fmpz_poly(
        res: *mut fmpz_poly_struct,
        src: *mut acb_poly_struct,
    ) -> c_int;
    pub fn acb_poly_set_si(poly: *mut acb_poly_struct, c: mp_limb_signed_t);
    pub fn acb_poly_randtest(
        poly: *mut acb_poly_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
        mag_bits: mp_limb_signed_t,
    );
    pub fn acb_poly_equal(A: *mut acb_poly_struct, B: *mut acb_poly_struct) -> c_int;
    pub fn acb_poly_contains_fmpz_poly(
        poly1: *mut acb_poly_struct,
        poly2: *mut fmpz_poly_struct,
    ) -> c_int;
    pub fn acb_poly_contains_fmpq_poly(
        poly1: *mut acb_poly_struct,
        poly2: *mut fmpq_poly_struct,
    ) -> c_int;
    pub fn _acb_poly_overlaps(
        poly1: acb_srcptr,
        len1: mp_limb_signed_t,
        poly2: acb_srcptr,
        len2: mp_limb_signed_t,
    ) -> c_int;
    pub fn acb_poly_overlaps(poly1: *mut acb_poly_struct, poly2: *mut acb_poly_struct) -> c_int;
    pub fn acb_poly_contains(poly1: *mut acb_poly_struct, poly2: *mut acb_poly_struct) -> c_int;
    pub fn _acb_poly_add(
        res: acb_ptr,
        poly1: acb_srcptr,
        len1: mp_limb_signed_t,
        poly2: acb_srcptr,
        len2: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_add(
        res: *mut acb_poly_struct,
        poly1: *mut acb_poly_struct,
        poly2: *mut acb_poly_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_add_si(
        res: *mut acb_poly_struct,
        poly: *mut acb_poly_struct,
        c: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_sub(
        res: acb_ptr,
        poly1: acb_srcptr,
        len1: mp_limb_signed_t,
        poly2: acb_srcptr,
        len2: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_sub(
        res: *mut acb_poly_struct,
        poly1: *mut acb_poly_struct,
        poly2: *mut acb_poly_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_add_series(
        res: *mut acb_poly_struct,
        poly1: *mut acb_poly_struct,
        poly2: *mut acb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_sub_series(
        res: *mut acb_poly_struct,
        poly1: *mut acb_poly_struct,
        poly2: *mut acb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_mullow_classical(
        res: *mut acb_poly_struct,
        poly1: *mut acb_poly_struct,
        poly2: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_mullow_classical(
        res: acb_ptr,
        poly1: acb_srcptr,
        len1: mp_limb_signed_t,
        poly2: acb_srcptr,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_mullow_transpose(
        res: acb_ptr,
        poly1: acb_srcptr,
        len1: mp_limb_signed_t,
        poly2: acb_srcptr,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_mullow_transpose(
        res: *mut acb_poly_struct,
        poly1: *mut acb_poly_struct,
        poly2: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_mullow_transpose_gauss(
        res: acb_ptr,
        poly1: acb_srcptr,
        len1: mp_limb_signed_t,
        poly2: acb_srcptr,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_mullow_transpose_gauss(
        res: *mut acb_poly_struct,
        poly1: *mut acb_poly_struct,
        poly2: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_mullow(
        res: acb_ptr,
        poly1: acb_srcptr,
        len1: mp_limb_signed_t,
        poly2: acb_srcptr,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_mullow(
        res: *mut acb_poly_struct,
        poly1: *mut acb_poly_struct,
        poly2: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_mul(
        C: acb_ptr,
        A: acb_srcptr,
        lenA: mp_limb_signed_t,
        B: acb_srcptr,
        lenB: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_mul(
        res: *mut acb_poly_struct,
        poly1: *mut acb_poly_struct,
        poly2: *mut acb_poly_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_inv_series(
        Qinv: acb_ptr,
        Q: acb_srcptr,
        Qlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_inv_series(
        Qinv: *mut acb_poly_struct,
        Q: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_div_series(
        Q: acb_ptr,
        A: acb_srcptr,
        Alen: mp_limb_signed_t,
        B: acb_srcptr,
        Blen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_div_series(
        Q: *mut acb_poly_struct,
        A: *mut acb_poly_struct,
        B: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_reverse(
        res: acb_ptr,
        poly: acb_srcptr,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn _acb_poly_div(
        Q: acb_ptr,
        A: acb_srcptr,
        lenA: mp_limb_signed_t,
        B: acb_srcptr,
        lenB: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_divrem(
        Q: acb_ptr,
        R: acb_ptr,
        A: acb_srcptr,
        lenA: mp_limb_signed_t,
        B: acb_srcptr,
        lenB: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_rem(
        R: acb_ptr,
        A: acb_srcptr,
        lenA: mp_limb_signed_t,
        B: acb_srcptr,
        lenB: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_divrem(
        Q: *mut acb_poly_struct,
        R: *mut acb_poly_struct,
        A: *mut acb_poly_struct,
        B: *mut acb_poly_struct,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn _acb_poly_div_root(
        Q: acb_ptr,
        R: *mut acb_struct,
        A: acb_srcptr,
        len: mp_limb_signed_t,
        c: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_taylor_shift_horner(
        poly: acb_ptr,
        c: *mut acb_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_taylor_shift_horner(
        g: *mut acb_poly_struct,
        f: *mut acb_poly_struct,
        c: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_taylor_shift_divconquer(
        poly: acb_ptr,
        c: *mut acb_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_taylor_shift_divconquer(
        g: *mut acb_poly_struct,
        f: *mut acb_poly_struct,
        c: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_taylor_shift_convolution(
        poly: acb_ptr,
        c: *mut acb_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_taylor_shift_convolution(
        g: *mut acb_poly_struct,
        f: *mut acb_poly_struct,
        c: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_taylor_shift(
        poly: acb_ptr,
        c: *mut acb_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_taylor_shift(
        g: *mut acb_poly_struct,
        f: *mut acb_poly_struct,
        c: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_compose(
        res: acb_ptr,
        poly1: acb_srcptr,
        len1: mp_limb_signed_t,
        poly2: acb_srcptr,
        len2: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_compose(
        res: *mut acb_poly_struct,
        poly1: *mut acb_poly_struct,
        poly2: *mut acb_poly_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_compose_horner(
        res: acb_ptr,
        poly1: acb_srcptr,
        len1: mp_limb_signed_t,
        poly2: acb_srcptr,
        len2: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_compose_horner(
        res: *mut acb_poly_struct,
        poly1: *mut acb_poly_struct,
        poly2: *mut acb_poly_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_compose_divconquer(
        res: acb_ptr,
        poly1: acb_srcptr,
        len1: mp_limb_signed_t,
        poly2: acb_srcptr,
        len2: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_compose_divconquer(
        res: *mut acb_poly_struct,
        poly1: *mut acb_poly_struct,
        poly2: *mut acb_poly_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_compose_series_horner(
        res: acb_ptr,
        poly1: acb_srcptr,
        len1: mp_limb_signed_t,
        poly2: acb_srcptr,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_compose_series_horner(
        res: *mut acb_poly_struct,
        poly1: *mut acb_poly_struct,
        poly2: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_compose_series_brent_kung(
        res: acb_ptr,
        poly1: acb_srcptr,
        len1: mp_limb_signed_t,
        poly2: acb_srcptr,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_compose_series_brent_kung(
        res: *mut acb_poly_struct,
        poly1: *mut acb_poly_struct,
        poly2: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_compose_series(
        res: acb_ptr,
        poly1: acb_srcptr,
        len1: mp_limb_signed_t,
        poly2: acb_srcptr,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_compose_series(
        res: *mut acb_poly_struct,
        poly1: *mut acb_poly_struct,
        poly2: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_revert_series_lagrange(
        Qinv: acb_ptr,
        Q: acb_srcptr,
        Qlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_revert_series_lagrange(
        Qinv: *mut acb_poly_struct,
        Q: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_revert_series_newton(
        Qinv: acb_ptr,
        Q: acb_srcptr,
        Qlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_revert_series_newton(
        Qinv: *mut acb_poly_struct,
        Q: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_revert_series_lagrange_fast(
        Qinv: acb_ptr,
        Q: acb_srcptr,
        Qlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_revert_series_lagrange_fast(
        Qinv: *mut acb_poly_struct,
        Q: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_revert_series(
        Qinv: acb_ptr,
        Q: acb_srcptr,
        Qlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_revert_series(
        Qinv: *mut acb_poly_struct,
        Q: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_evaluate_vec_fast_precomp(
        vs: acb_ptr,
        poly: acb_srcptr,
        plen: mp_limb_signed_t,
        tree: *mut acb_ptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_evaluate_vec_fast(
        ys: acb_ptr,
        poly: acb_srcptr,
        plen: mp_limb_signed_t,
        xs: acb_srcptr,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_evaluate_vec_fast(
        ys: acb_ptr,
        poly: *mut acb_poly_struct,
        xs: acb_srcptr,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_evaluate_vec_iter(
        ys: acb_ptr,
        poly: acb_srcptr,
        plen: mp_limb_signed_t,
        xs: acb_srcptr,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_evaluate_vec_iter(
        ys: acb_ptr,
        poly: *mut acb_poly_struct,
        xs: acb_srcptr,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_interpolate_barycentric(
        poly: acb_ptr,
        xs: acb_srcptr,
        ys: acb_srcptr,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_interpolate_barycentric(
        poly: *mut acb_poly_struct,
        xs: acb_srcptr,
        ys: acb_srcptr,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_interpolation_weights(
        w: acb_ptr,
        tree: *mut acb_ptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_interpolate_fast_precomp(
        poly: acb_ptr,
        ys: acb_srcptr,
        tree: *mut acb_ptr,
        weights: acb_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_interpolate_fast(
        poly: acb_ptr,
        xs: acb_srcptr,
        ys: acb_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_interpolate_fast(
        poly: *mut acb_poly_struct,
        xs: acb_srcptr,
        ys: acb_srcptr,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_interpolate_newton(
        poly: acb_ptr,
        xs: acb_srcptr,
        ys: acb_srcptr,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_interpolate_newton(
        poly: *mut acb_poly_struct,
        xs: acb_srcptr,
        ys: acb_srcptr,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_product_roots(
        poly: acb_ptr,
        xs: acb_srcptr,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_product_roots(
        poly: *mut acb_poly_struct,
        xs: acb_srcptr,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_tree_alloc(len: mp_limb_signed_t) -> *mut acb_ptr;
    pub fn _acb_poly_tree_free(tree: *mut acb_ptr, len: mp_limb_signed_t);
    pub fn _acb_poly_tree_build(
        tree: *mut acb_ptr,
        roots: acb_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_root_inclusion(
        r: *mut acb_struct,
        m: *mut acb_struct,
        poly: acb_srcptr,
        polyder: acb_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_validate_roots(
        roots: acb_ptr,
        poly: acb_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn _acb_poly_refine_roots_durand_kerner(
        roots: acb_ptr,
        poly: acb_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_find_roots(
        roots: acb_ptr,
        poly: acb_srcptr,
        initial: acb_srcptr,
        len: mp_limb_signed_t,
        maxiter: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn acb_poly_find_roots(
        roots: acb_ptr,
        poly: *mut acb_poly_struct,
        initial: acb_srcptr,
        maxiter: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn _acb_poly_root_bound_fujiwara(
        bound: *mut mag_struct,
        poly: acb_srcptr,
        len: mp_limb_signed_t,
    );
    pub fn acb_poly_root_bound_fujiwara(bound: *mut mag_struct, poly: *mut acb_poly_struct);
    pub fn _acb_poly_validate_real_roots(
        roots: acb_srcptr,
        poly: acb_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn acb_poly_validate_real_roots(
        roots: acb_srcptr,
        poly: *mut acb_poly_struct,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn _acb_poly_pow_ui_trunc_binexp(
        res: acb_ptr,
        f: acb_srcptr,
        flen: mp_limb_signed_t,
        exp: mp_limb_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_pow_ui_trunc_binexp(
        res: *mut acb_poly_struct,
        poly: *mut acb_poly_struct,
        exp: mp_limb_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_pow_ui(
        res: acb_ptr,
        f: acb_srcptr,
        flen: mp_limb_signed_t,
        exp: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_pow_ui(
        res: *mut acb_poly_struct,
        poly: *mut acb_poly_struct,
        exp: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_rsqrt_series(
        g: acb_ptr,
        h: acb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_rsqrt_series(
        g: *mut acb_poly_struct,
        h: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_sqrt_series(
        g: acb_ptr,
        h: acb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_sqrt_series(
        g: *mut acb_poly_struct,
        h: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_log_series(
        res: acb_ptr,
        f: acb_srcptr,
        flen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_log_series(
        res: *mut acb_poly_struct,
        f: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_log1p_series(
        res: acb_ptr,
        f: acb_srcptr,
        flen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_log1p_series(
        res: *mut acb_poly_struct,
        f: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_atan_series(
        res: acb_ptr,
        f: acb_srcptr,
        flen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_atan_series(
        res: *mut acb_poly_struct,
        f: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_exp_series_basecase(
        f: acb_ptr,
        h: acb_srcptr,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_exp_series_basecase(
        f: *mut acb_poly_struct,
        h: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_exp_series(
        f: acb_ptr,
        h: acb_srcptr,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_exp_series(
        f: *mut acb_poly_struct,
        h: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_exp_pi_i_series(
        f: acb_ptr,
        h: acb_srcptr,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_exp_pi_i_series(
        f: *mut acb_poly_struct,
        h: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_sinh_cosh_series_basecase(
        s: acb_ptr,
        c: acb_ptr,
        h: acb_srcptr,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_sinh_cosh_series_basecase(
        s: *mut acb_poly_struct,
        c: *mut acb_poly_struct,
        h: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_sinh_cosh_series_exponential(
        s: acb_ptr,
        c: acb_ptr,
        h: acb_srcptr,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_sinh_cosh_series_exponential(
        s: *mut acb_poly_struct,
        c: *mut acb_poly_struct,
        h: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_sinh_cosh_series(
        s: acb_ptr,
        c: acb_ptr,
        h: acb_srcptr,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_sinh_cosh_series(
        s: *mut acb_poly_struct,
        c: *mut acb_poly_struct,
        h: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_sinh_series(
        s: acb_ptr,
        h: acb_srcptr,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_sinh_series(
        s: *mut acb_poly_struct,
        h: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_cosh_series(
        c: acb_ptr,
        h: acb_srcptr,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_cosh_series(
        c: *mut acb_poly_struct,
        h: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_sin_cos_series_basecase(
        s: acb_ptr,
        c: acb_ptr,
        h: acb_srcptr,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
        times_pi: c_int,
    );
    pub fn acb_poly_sin_cos_series_basecase(
        s: *mut acb_poly_struct,
        c: *mut acb_poly_struct,
        h: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
        times_pi: c_int,
    );
    pub fn _acb_poly_sin_cos_series_tangent(
        s: acb_ptr,
        c: acb_ptr,
        h: acb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
        times_pi: c_int,
    );
    pub fn acb_poly_sin_cos_series_tangent(
        s: *mut acb_poly_struct,
        c: *mut acb_poly_struct,
        h: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
        times_pi: c_int,
    );
    pub fn _acb_poly_sin_cos_series(
        s: acb_ptr,
        c: acb_ptr,
        h: acb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_sin_cos_series(
        s: *mut acb_poly_struct,
        c: *mut acb_poly_struct,
        h: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_sin_series(
        g: acb_ptr,
        h: acb_srcptr,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_sin_series(
        g: *mut acb_poly_struct,
        h: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_cos_series(
        g: acb_ptr,
        h: acb_srcptr,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_cos_series(
        g: *mut acb_poly_struct,
        h: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_sin_cos_pi_series(
        s: acb_ptr,
        c: acb_ptr,
        h: acb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_sin_cos_pi_series(
        s: *mut acb_poly_struct,
        c: *mut acb_poly_struct,
        h: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_sin_pi_series(
        g: acb_ptr,
        h: acb_srcptr,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_sin_pi_series(
        g: *mut acb_poly_struct,
        h: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_cos_pi_series(
        g: acb_ptr,
        h: acb_srcptr,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_cos_pi_series(
        g: *mut acb_poly_struct,
        h: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_cot_pi_series(
        g: acb_ptr,
        h: acb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_cot_pi_series(
        res: *mut acb_poly_struct,
        f: *mut acb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_tan_series(
        g: acb_ptr,
        h: acb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_tan_series(
        g: *mut acb_poly_struct,
        h: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_sinc_series(
        g: acb_ptr,
        h: acb_srcptr,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_sinc_series(
        g: *mut acb_poly_struct,
        h: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_lambertw_series(
        res: acb_ptr,
        z: acb_srcptr,
        zlen: mp_limb_signed_t,
        k: *mut fmpz,
        flags: c_int,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_lambertw_series(
        res: *mut acb_poly_struct,
        z: *mut acb_poly_struct,
        k: *mut fmpz,
        flags: c_int,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_gamma_series(
        res: acb_ptr,
        h: acb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_gamma_series(
        res: *mut acb_poly_struct,
        f: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_rgamma_series(
        res: acb_ptr,
        h: acb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_rgamma_series(
        res: *mut acb_poly_struct,
        f: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_lgamma_series(
        res: acb_ptr,
        h: acb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_lgamma_series(
        res: *mut acb_poly_struct,
        f: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_digamma_series(
        res: acb_ptr,
        h: acb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_digamma_series(
        res: *mut acb_poly_struct,
        f: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_rising_ui_series(
        res: acb_ptr,
        f: acb_srcptr,
        flen: mp_limb_signed_t,
        r: mp_limb_t,
        trunc: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_rising_ui_series(
        res: *mut acb_poly_struct,
        f: *mut acb_poly_struct,
        r: mp_limb_t,
        trunc: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_pow_acb_series(
        h: acb_ptr,
        f: acb_srcptr,
        flen: mp_limb_signed_t,
        g: *mut acb_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_pow_acb_series(
        h: *mut acb_poly_struct,
        f: *mut acb_poly_struct,
        g: *mut acb_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_pow_series(
        h: acb_ptr,
        f: acb_srcptr,
        flen: mp_limb_signed_t,
        g: acb_srcptr,
        glen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_pow_series(
        h: *mut acb_poly_struct,
        f: *mut acb_poly_struct,
        g: *mut acb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_binomial_pow_acb_series(
        h: acb_ptr,
        f: acb_srcptr,
        flen: mp_limb_signed_t,
        g: *mut acb_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_acb_invpow_cpx(
        res: acb_ptr,
        N: *mut acb_struct,
        c: *mut acb_struct,
        trunc: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_mullow_cpx(
        res: acb_ptr,
        src: acb_srcptr,
        len: mp_limb_signed_t,
        c: *mut acb_struct,
        trunc: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_powsum_series_naive(
        z: acb_ptr,
        s: *mut acb_struct,
        a: *mut acb_struct,
        q: *mut acb_struct,
        n: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_powsum_series_naive_threaded(
        z: acb_ptr,
        s: *mut acb_struct,
        a: *mut acb_struct,
        q: *mut acb_struct,
        n: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_powsum_one_series_sieved(
        z: acb_ptr,
        s: *mut acb_struct,
        n: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_zeta_em_sum(
        z: acb_ptr,
        s: *mut acb_struct,
        a: *mut acb_struct,
        deflate: c_int,
        N: mp_limb_t,
        M: mp_limb_t,
        d: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_zeta_em_choose_param(
        bound: *mut mag_struct,
        N: *mut mp_limb_t,
        M: *mut mp_limb_t,
        s: *mut acb_struct,
        a: *mut acb_struct,
        d: mp_limb_signed_t,
        target: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_zeta_em_bound1(
        bound: *mut mag_struct,
        s: *mut acb_struct,
        a: *mut acb_struct,
        N: mp_limb_signed_t,
        M: mp_limb_signed_t,
        d: mp_limb_signed_t,
        wp: mp_limb_signed_t,
    );
    pub fn _acb_poly_zeta_em_bound(
        vec: arb_ptr,
        s: *mut acb_struct,
        a: *mut acb_struct,
        N: mp_limb_t,
        M: mp_limb_t,
        d: mp_limb_signed_t,
        wp: mp_limb_signed_t,
    );
    pub fn _acb_poly_zeta_em_tail_naive(
        sum: acb_ptr,
        s: *mut acb_struct,
        Na: *mut acb_struct,
        Nasx: acb_srcptr,
        M: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_zeta_em_tail_bsplit(
        z: acb_ptr,
        s: *mut acb_struct,
        Na: *mut acb_struct,
        Nasx: acb_srcptr,
        M: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_zeta_cpx_series(
        z: acb_ptr,
        s: *mut acb_struct,
        a: *mut acb_struct,
        deflate: c_int,
        d: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_zeta_series(
        res: acb_ptr,
        h: acb_srcptr,
        hlen: mp_limb_signed_t,
        a: *mut acb_struct,
        deflate: c_int,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_zeta_series(
        res: *mut acb_poly_struct,
        f: *mut acb_poly_struct,
        a: *mut acb_struct,
        deflate: c_int,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_polylog_cpx_zeta(
        w: acb_ptr,
        s: *mut acb_struct,
        z: *mut acb_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_polylog_cpx_small(
        w: acb_ptr,
        s: *mut acb_struct,
        z: *mut acb_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_polylog_cpx(
        w: acb_ptr,
        s: *mut acb_struct,
        z: *mut acb_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_polylog_series(
        res: acb_ptr,
        s: acb_srcptr,
        slen: mp_limb_signed_t,
        z: *mut acb_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_polylog_series(
        res: *mut acb_poly_struct,
        s: *mut acb_poly_struct,
        z: *mut acb_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_agm1_series(
        res: acb_ptr,
        z: acb_srcptr,
        zlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_agm1_series(
        res: *mut acb_poly_struct,
        z: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_elliptic_k_series(
        res: acb_ptr,
        z: acb_srcptr,
        zlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_elliptic_k_series(
        res: *mut acb_poly_struct,
        z: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_elliptic_p_series(
        res: acb_ptr,
        z: acb_srcptr,
        zlen: mp_limb_signed_t,
        tau: *mut acb_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_elliptic_p_series(
        res: *mut acb_poly_struct,
        z: *mut acb_poly_struct,
        tau: *mut acb_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_poly_erf_series(
        g: acb_ptr,
        h: acb_srcptr,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_poly_erf_series(
        g: *mut acb_poly_struct,
        h: *mut acb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
}
