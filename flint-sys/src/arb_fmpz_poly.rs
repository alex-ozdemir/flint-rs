#![allow(non_camel_case_types)]

//! *See the [Arb documentation](https://arblib.org/).

use crate::acb::{acb_ptr, acb_struct};
use crate::arb::arb_struct;
use crate::deps::*;
use crate::fmpz::fmpz;
use crate::fmpz_poly::fmpz_poly_struct;

extern "C" {
    pub fn _arb_fmpz_poly_evaluate_acb_horner(
        res: *mut acb_struct,
        f: *const fmpz,
        len: mp_limb_signed_t,
        x: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_fmpz_poly_evaluate_acb_horner(
        res: *mut acb_struct,
        f: *mut fmpz_poly_struct,
        a: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_fmpz_poly_evaluate_acb_rectangular(
        res: *mut acb_struct,
        f: *const fmpz,
        len: mp_limb_signed_t,
        x: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_fmpz_poly_evaluate_acb_rectangular(
        res: *mut acb_struct,
        f: *mut fmpz_poly_struct,
        a: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_fmpz_poly_evaluate_acb(
        res: *mut acb_struct,
        f: *const fmpz,
        len: mp_limb_signed_t,
        x: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_fmpz_poly_evaluate_acb(
        res: *mut acb_struct,
        f: *mut fmpz_poly_struct,
        a: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_fmpz_poly_evaluate_arb_horner(
        res: *mut arb_struct,
        f: *const fmpz,
        len: mp_limb_signed_t,
        x: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_fmpz_poly_evaluate_arb_horner(
        res: *mut arb_struct,
        f: *mut fmpz_poly_struct,
        a: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_fmpz_poly_evaluate_arb_rectangular(
        res: *mut arb_struct,
        f: *const fmpz,
        len: mp_limb_signed_t,
        x: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_fmpz_poly_evaluate_arb_rectangular(
        res: *mut arb_struct,
        f: *mut fmpz_poly_struct,
        a: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_fmpz_poly_evaluate_arb(
        res: *mut arb_struct,
        f: *const fmpz,
        len: mp_limb_signed_t,
        x: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_fmpz_poly_evaluate_arb(
        res: *mut arb_struct,
        f: *mut fmpz_poly_struct,
        a: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_fmpz_poly_deflate(
        result: *mut fmpz_poly_struct,
        input: *mut fmpz_poly_struct,
        deflation: mp_limb_t,
    );
    pub fn arb_fmpz_poly_deflation(input: *mut fmpz_poly_struct) -> mp_limb_t;
    pub fn arb_fmpz_poly_complex_roots(
        roots: acb_ptr,
        poly: *mut fmpz_poly_struct,
        flags: ::std::os::raw::c_int,
        target_prec: mp_limb_signed_t,
    );
    pub fn arb_fmpz_poly_gauss_period_minpoly(
        res: *mut fmpz_poly_struct,
        q: mp_limb_t,
        n: mp_limb_t,
    );
}
