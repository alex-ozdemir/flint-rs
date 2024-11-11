#![allow(non_camel_case_types)]

//! *See the [Arb documentation](https://arblib.org/).

use crate::acb::{acb_srcptr, acb_struct};
use crate::arb::*;
use crate::arf::arf_struct;
use crate::mag::mag_struct;
use crate::deps::*;
use crate::flint::*;
use crate::fmpq_poly::fmpq_poly_struct;
use crate::fmpz_poly::fmpz_poly_struct;
use libc::{c_int, FILE};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct arb_poly_struct {
    pub coeffs: arb_ptr,
    pub length: mp_limb_signed_t,
    pub alloc: mp_limb_signed_t,
}

pub type arb_poly_t = [arb_poly_struct; 1usize];

extern "C" {
    pub fn arb_poly_init(poly: *mut arb_poly_struct);
    pub fn arb_poly_init2(poly: *mut arb_poly_struct, len: mp_limb_signed_t);
    pub fn arb_poly_clear(poly: *mut arb_poly_struct);
    pub fn arb_poly_fit_length(poly: *mut arb_poly_struct, len: mp_limb_signed_t);
    pub fn _arb_poly_set_length(poly: *mut arb_poly_struct, len: mp_limb_signed_t);
    pub fn _arb_poly_normalise(poly: *mut arb_poly_struct);
    pub fn arb_poly_set(poly: *mut arb_poly_struct, src: *const arb_poly_struct);
    pub fn arb_poly_set_round(
        poly: *mut arb_poly_struct,
        src: *const arb_poly_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_set_trunc(
        res: *mut arb_poly_struct,
        poly: *const arb_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn arb_poly_set_trunc_round(
        res: *mut arb_poly_struct,
        poly: *const arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_valuation(poly: *mut arb_poly_struct) -> mp_limb_signed_t;
    pub fn arb_poly_set_coeff_si(
        poly: *mut arb_poly_struct,
        n: mp_limb_signed_t,
        x: mp_limb_signed_t,
    );
    pub fn arb_poly_set_coeff_arb(
        poly: *mut arb_poly_struct,
        n: mp_limb_signed_t,
        x: *const arb_struct,
    );
    pub fn arb_poly_get_coeff_arb(
        x: *mut arb_struct,
        poly: *const arb_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _arb_poly_reverse(
        res: arb_ptr,
        poly: arb_srcptr,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn _arb_poly_shift_right(
        res: arb_ptr,
        poly: arb_srcptr,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn arb_poly_shift_right(
        res: *mut arb_poly_struct,
        poly: *const arb_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _arb_poly_shift_left(
        res: arb_ptr,
        poly: arb_srcptr,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
    );
    pub fn arb_poly_shift_left(
        res: *mut arb_poly_struct,
        poly: *const arb_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn arb_poly_set_fmpz_poly(
        poly: *mut arb_poly_struct,
        src: *const fmpz_poly_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_set_fmpq_poly(
        poly: *mut arb_poly_struct,
        src: *const fmpq_poly_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_set_si(poly: *mut arb_poly_struct, c: mp_limb_signed_t);
    pub fn arb_poly_get_unique_fmpz_poly(
        res: *mut fmpz_poly_struct,
        src: *const arb_poly_struct,
    ) -> c_int;
    pub fn arb_poly_contains(poly1: *const arb_poly_struct, poly2: *const arb_poly_struct)
        -> c_int;
    pub fn arb_poly_contains_fmpz_poly(
        poly1: *const arb_poly_struct,
        poly2: *const fmpz_poly_struct,
    ) -> c_int;
    pub fn arb_poly_contains_fmpq_poly(
        poly1: *const arb_poly_struct,
        poly2: *const fmpq_poly_struct,
    ) -> c_int;
    pub fn arb_poly_equal(A: *const arb_poly_struct, B: *const arb_poly_struct) -> c_int;
    pub fn _arb_poly_overlaps(
        poly1: arb_srcptr,
        len1: mp_limb_signed_t,
        poly2: arb_srcptr,
        len2: mp_limb_signed_t,
    ) -> c_int;
    pub fn arb_poly_overlaps(poly1: *const arb_poly_struct, poly2: *const arb_poly_struct)
        -> c_int;
    pub fn _arb_poly_majorant(
        res: arb_ptr,
        vec: arb_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_majorant(
        res: *mut arb_poly_struct,
        poly: *const arb_poly_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_fprintd(
        file: *mut FILE,
        poly: *const arb_poly_struct,
        digits: mp_limb_signed_t,
    );
    pub fn arb_poly_randtest(
        poly: *mut arb_poly_struct,
        state: *const flint_rand_s,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
        mag_bits: mp_limb_signed_t,
    );
    pub fn _arb_poly_add(
        res: arb_ptr,
        poly1: arb_srcptr,
        len1: mp_limb_signed_t,
        poly2: arb_srcptr,
        len2: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_add(
        res: *mut arb_poly_struct,
        poly1: *const arb_poly_struct,
        poly2: *const arb_poly_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_add_si(
        res: *mut arb_poly_struct,
        poly: *const arb_poly_struct,
        c: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_sub(
        res: arb_ptr,
        poly1: arb_srcptr,
        len1: mp_limb_signed_t,
        poly2: arb_srcptr,
        len2: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_sub(
        res: *mut arb_poly_struct,
        poly1: *const arb_poly_struct,
        poly2: *const arb_poly_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_add_series(
        res: *mut arb_poly_struct,
        poly1: *const arb_poly_struct,
        poly2: *const arb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_sub_series(
        res: *mut arb_poly_struct,
        poly1: *const arb_poly_struct,
        poly2: *const arb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_mullow_ztrunc(
        res: arb_ptr,
        poly1: arb_srcptr,
        len1: mp_limb_signed_t,
        poly2: arb_srcptr,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_mullow_ztrunc(
        res: *mut arb_poly_struct,
        poly1: *const arb_poly_struct,
        poly2: *const arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_mullow_classical(
        res: arb_ptr,
        poly1: arb_srcptr,
        len1: mp_limb_signed_t,
        poly2: arb_srcptr,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_mullow_classical(
        res: *mut arb_poly_struct,
        poly1: *const arb_poly_struct,
        poly2: *const arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_mullow_block(
        C: arb_ptr,
        A: arb_srcptr,
        lenA: mp_limb_signed_t,
        B: arb_srcptr,
        lenB: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_mullow_block(
        res: *mut arb_poly_struct,
        poly1: *const arb_poly_struct,
        poly2: *const arb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_mullow(
        C: arb_ptr,
        A: arb_srcptr,
        lenA: mp_limb_signed_t,
        B: arb_srcptr,
        lenB: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_mullow(
        res: *mut arb_poly_struct,
        poly1: *const arb_poly_struct,
        poly2: *const arb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_mul(
        C: arb_ptr,
        A: arb_srcptr,
        lenA: mp_limb_signed_t,
        B: arb_srcptr,
        lenB: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_mul(
        res: *mut arb_poly_struct,
        poly1: *const arb_poly_struct,
        poly2: *const arb_poly_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_inv_series(
        Qinv: arb_ptr,
        Q: arb_srcptr,
        Qlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_inv_series(
        Qinv: *mut arb_poly_struct,
        Q: *const arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_div_series(
        Q: arb_ptr,
        A: arb_srcptr,
        Alen: mp_limb_signed_t,
        B: arb_srcptr,
        Blen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_div_series(
        Q: *mut arb_poly_struct,
        A: *const arb_poly_struct,
        B: *const arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_div(
        Q: arb_ptr,
        A: arb_srcptr,
        lenA: mp_limb_signed_t,
        B: arb_srcptr,
        lenB: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_divrem(
        Q: arb_ptr,
        R: arb_ptr,
        A: arb_srcptr,
        lenA: mp_limb_signed_t,
        B: arb_srcptr,
        lenB: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_rem(
        R: arb_ptr,
        A: arb_srcptr,
        lenA: mp_limb_signed_t,
        B: arb_srcptr,
        lenB: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_divrem(
        Q: *mut arb_poly_struct,
        R: *mut arb_poly_struct,
        A: *const arb_poly_struct,
        B: *const arb_poly_struct,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn _arb_poly_div_root(
        Q: arb_ptr,
        R: *mut arb_struct,
        A: arb_srcptr,
        len: mp_limb_signed_t,
        c: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_product_roots(
        poly: arb_ptr,
        xs: arb_srcptr,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_product_roots(
        poly: *mut arb_poly_struct,
        xs: arb_srcptr,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_product_roots_complex(
        poly: arb_ptr,
        r: arb_srcptr,
        rn: mp_limb_signed_t,
        c: acb_srcptr,
        cn: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_product_roots_complex(
        poly: *mut arb_poly_struct,
        r: arb_srcptr,
        rn: mp_limb_signed_t,
        c: acb_srcptr,
        cn: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_tree_alloc(len: mp_limb_signed_t) -> *mut arb_ptr;
    pub fn _arb_poly_tree_free(tree: *mut arb_ptr, len: mp_limb_signed_t);
    pub fn _arb_poly_tree_build(
        tree: *mut arb_ptr,
        roots: arb_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_taylor_shift_horner(
        poly: arb_ptr,
        c: *mut arb_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_taylor_shift_horner(
        g: *mut arb_poly_struct,
        f: *mut arb_poly_struct,
        c: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_taylor_shift_divconquer(
        poly: arb_ptr,
        c: *mut arb_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_taylor_shift_divconquer(
        g: *mut arb_poly_struct,
        f: *mut arb_poly_struct,
        c: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_taylor_shift_convolution(
        poly: arb_ptr,
        c: *mut arb_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_taylor_shift_convolution(
        g: *mut arb_poly_struct,
        f: *mut arb_poly_struct,
        c: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_taylor_shift(
        poly: arb_ptr,
        c: *mut arb_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_taylor_shift(
        g: *mut arb_poly_struct,
        f: *mut arb_poly_struct,
        c: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_compose(
        res: arb_ptr,
        poly1: arb_srcptr,
        len1: mp_limb_signed_t,
        poly2: arb_srcptr,
        len2: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_compose(
        res: *mut arb_poly_struct,
        poly1: *mut arb_poly_struct,
        poly2: *mut arb_poly_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_compose_horner(
        res: arb_ptr,
        poly1: arb_srcptr,
        len1: mp_limb_signed_t,
        poly2: arb_srcptr,
        len2: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_compose_horner(
        res: *mut arb_poly_struct,
        poly1: *mut arb_poly_struct,
        poly2: *mut arb_poly_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_compose_divconquer(
        res: arb_ptr,
        poly1: arb_srcptr,
        len1: mp_limb_signed_t,
        poly2: arb_srcptr,
        len2: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_compose_divconquer(
        res: *mut arb_poly_struct,
        poly1: *mut arb_poly_struct,
        poly2: *mut arb_poly_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_compose_series_horner(
        res: arb_ptr,
        poly1: arb_srcptr,
        len1: mp_limb_signed_t,
        poly2: arb_srcptr,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_compose_series_horner(
        res: *mut arb_poly_struct,
        poly1: *mut arb_poly_struct,
        poly2: *mut arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_compose_series(
        res: arb_ptr,
        poly1: arb_srcptr,
        len1: mp_limb_signed_t,
        poly2: arb_srcptr,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_compose_series(
        res: *mut arb_poly_struct,
        poly1: *mut arb_poly_struct,
        poly2: *mut arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_revert_series_lagrange(
        Qinv: arb_ptr,
        Q: arb_srcptr,
        Qlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_revert_series_lagrange(
        Qinv: *mut arb_poly_struct,
        Q: *mut arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_revert_series_newton(
        Qinv: arb_ptr,
        Q: arb_srcptr,
        Qlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_revert_series_newton(
        Qinv: *mut arb_poly_struct,
        Q: *mut arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_revert_series_lagrange_fast(
        Qinv: arb_ptr,
        Q: arb_srcptr,
        Qlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_revert_series_lagrange_fast(
        Qinv: *mut arb_poly_struct,
        Q: *mut arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_revert_series(
        Qinv: arb_ptr,
        Q: arb_srcptr,
        Qlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_revert_series(
        Qinv: *mut arb_poly_struct,
        Q: *mut arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_evaluate_horner(
        res: *mut arb_struct,
        f: arb_srcptr,
        len: mp_limb_signed_t,
        a: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_evaluate_horner(
        res: *mut arb_struct,
        f: *mut arb_poly_struct,
        a: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_evaluate_rectangular(
        y: *mut arb_struct,
        poly: arb_srcptr,
        len: mp_limb_signed_t,
        x: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_evaluate_rectangular(
        res: *mut arb_struct,
        f: *mut arb_poly_struct,
        a: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_evaluate(
        res: *mut arb_struct,
        f: arb_srcptr,
        len: mp_limb_signed_t,
        a: *const arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_evaluate(
        res: *mut arb_struct,
        f: *mut arb_poly_struct,
        a: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_evaluate2_horner(
        y: *mut arb_struct,
        z: *mut arb_struct,
        f: arb_srcptr,
        len: mp_limb_signed_t,
        x: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_evaluate2_horner(
        y: *mut arb_struct,
        z: *mut arb_struct,
        f: *mut arb_poly_struct,
        x: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_evaluate2_rectangular(
        y: *mut arb_struct,
        z: *mut arb_struct,
        f: arb_srcptr,
        len: mp_limb_signed_t,
        x: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_evaluate2_rectangular(
        y: *mut arb_struct,
        z: *mut arb_struct,
        f: *mut arb_poly_struct,
        x: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_evaluate2(
        y: *mut arb_struct,
        z: *mut arb_struct,
        f: arb_srcptr,
        len: mp_limb_signed_t,
        x: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_evaluate2(
        y: *mut arb_struct,
        z: *mut arb_struct,
        f: *mut arb_poly_struct,
        x: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_evaluate_vec_iter(
        ys: arb_ptr,
        poly: arb_srcptr,
        plen: mp_limb_signed_t,
        xs: arb_srcptr,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_evaluate_vec_iter(
        ys: arb_ptr,
        poly: *mut arb_poly_struct,
        xs: arb_srcptr,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_evaluate_vec_fast_precomp(
        vs: arb_ptr,
        poly: arb_srcptr,
        plen: mp_limb_signed_t,
        tree: *mut arb_ptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_evaluate_vec_fast(
        ys: arb_ptr,
        poly: arb_srcptr,
        plen: mp_limb_signed_t,
        xs: arb_srcptr,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_evaluate_vec_fast(
        ys: arb_ptr,
        poly: *const arb_poly_struct,
        xs: arb_srcptr,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_interpolate_newton(
        poly: arb_ptr,
        xs: arb_srcptr,
        ys: arb_srcptr,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_interpolate_newton(
        poly: *mut arb_poly_struct,
        xs: arb_srcptr,
        ys: arb_srcptr,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_interpolate_barycentric(
        poly: arb_ptr,
        xs: arb_srcptr,
        ys: arb_srcptr,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_interpolate_barycentric(
        poly: *mut arb_poly_struct,
        xs: arb_srcptr,
        ys: arb_srcptr,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_interpolation_weights(
        w: arb_ptr,
        tree: *mut arb_ptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_interpolate_fast_precomp(
        poly: arb_ptr,
        ys: arb_srcptr,
        tree: *mut arb_ptr,
        weights: arb_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_interpolate_fast(
        poly: arb_ptr,
        xs: arb_srcptr,
        ys: arb_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_interpolate_fast(
        poly: *mut arb_poly_struct,
        xs: arb_srcptr,
        ys: arb_srcptr,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_derivative(
        res: arb_ptr,
        poly: arb_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_derivative(
        res: *mut arb_poly_struct,
        poly: *const arb_poly_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_integral(
        res: arb_ptr,
        poly: arb_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_integral(
        res: *mut arb_poly_struct,
        poly: *const arb_poly_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_borel_transform(
        res: *mut arb_poly_struct,
        poly: *const arb_poly_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_borel_transform(
        res: arb_ptr,
        poly: arb_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_inv_borel_transform(
        res: *mut arb_poly_struct,
        poly: *const arb_poly_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_inv_borel_transform(
        res: arb_ptr,
        poly: arb_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_binomial_transform_basecase(
        b: arb_ptr,
        a: arb_srcptr,
        alen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_binomial_transform_basecase(
        b: *mut arb_poly_struct,
        a: *const arb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_binomial_transform_convolution(
        b: arb_ptr,
        a: arb_srcptr,
        alen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_binomial_transform_convolution(
        b: *mut arb_poly_struct,
        a: *const arb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_binomial_transform(
        b: arb_ptr,
        a: arb_srcptr,
        alen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_binomial_transform(
        b: *mut arb_poly_struct,
        a: *const arb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_pow_ui_trunc_binexp(
        res: arb_ptr,
        f: arb_srcptr,
        flen: mp_limb_signed_t,
        exp: mp_limb_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_pow_ui_trunc_binexp(
        res: *mut arb_poly_struct,
        poly: *const arb_poly_struct,
        exp: mp_limb_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_pow_ui(
        res: arb_ptr,
        f: arb_srcptr,
        flen: mp_limb_signed_t,
        exp: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_pow_ui(
        res: *mut arb_poly_struct,
        poly: *const arb_poly_struct,
        exp: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_pow_series(
        h: arb_ptr,
        f: arb_srcptr,
        flen: mp_limb_signed_t,
        g: arb_srcptr,
        glen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_pow_series(
        h: *mut arb_poly_struct,
        f: *const arb_poly_struct,
        g: *const arb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_pow_arb_series(
        h: arb_ptr,
        f: arb_srcptr,
        flen: mp_limb_signed_t,
        g: *const arb_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_pow_arb_series(
        h: *mut arb_poly_struct,
        f: *const arb_poly_struct,
        g: *const arb_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_binomial_pow_arb_series(
        h: arb_ptr,
        f: arb_srcptr,
        flen: mp_limb_signed_t,
        g: *const arb_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_rsqrt_series(
        g: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_rsqrt_series(
        g: *mut arb_poly_struct,
        h: *const arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_sqrt_series(
        g: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_sqrt_series(
        g: *mut arb_poly_struct,
        h: *const arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_log_series(
        res: arb_ptr,
        f: arb_srcptr,
        flen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_log_series(
        res: *mut arb_poly_struct,
        f: *const arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_log1p_series(
        res: arb_ptr,
        f: arb_srcptr,
        flen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_log1p_series(
        res: *mut arb_poly_struct,
        f: *const arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_atan_series(
        res: arb_ptr,
        f: arb_srcptr,
        flen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_atan_series(
        res: *mut arb_poly_struct,
        f: *const arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_asin_series(
        res: arb_ptr,
        f: arb_srcptr,
        flen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_asin_series(
        res: *mut arb_poly_struct,
        f: *const arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_acos_series(
        res: arb_ptr,
        f: arb_srcptr,
        flen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_acos_series(
        res: *mut arb_poly_struct,
        f: *const arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_exp_series_basecase(
        f: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_exp_series_basecase(
        f: *mut arb_poly_struct,
        h: *mut arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_exp_series(
        f: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_exp_series(
        f: *mut arb_poly_struct,
        h: *mut arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_sinh_cosh_series_basecase(
        s: arb_ptr,
        c: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_sinh_cosh_series_basecase(
        s: *mut arb_poly_struct,
        c: *mut arb_poly_struct,
        h: *mut arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_sinh_cosh_series_exponential(
        s: arb_ptr,
        c: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_sinh_cosh_series_exponential(
        s: *mut arb_poly_struct,
        c: *mut arb_poly_struct,
        h: *mut arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_sinh_cosh_series(
        s: arb_ptr,
        c: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_sinh_cosh_series(
        s: *mut arb_poly_struct,
        c: *mut arb_poly_struct,
        h: *mut arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_sinh_series(
        s: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_sinh_series(
        s: *mut arb_poly_struct,
        h: *mut arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_cosh_series(
        c: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_cosh_series(
        c: *mut arb_poly_struct,
        h: *mut arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_sin_cos_series_basecase(
        s: arb_ptr,
        c: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
        times_pi: c_int,
    );
    pub fn arb_poly_sin_cos_series_basecase(
        s: *mut arb_poly_struct,
        c: *mut arb_poly_struct,
        h: *mut arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
        times_pi: c_int,
    );
    pub fn _arb_poly_sin_cos_series_tangent(
        s: arb_ptr,
        c: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
        times_pi: c_int,
    );
    pub fn arb_poly_sin_cos_series_tangent(
        s: *mut arb_poly_struct,
        c: *mut arb_poly_struct,
        h: *mut arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
        times_pi: c_int,
    );
    pub fn _arb_poly_sin_cos_series(
        s: arb_ptr,
        c: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_sin_cos_series(
        s: *mut arb_poly_struct,
        c: *mut arb_poly_struct,
        h: *mut arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_sin_cos_pi_series(
        s: arb_ptr,
        c: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_sin_cos_pi_series(
        s: *mut arb_poly_struct,
        c: *mut arb_poly_struct,
        h: *mut arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_sin_series(
        g: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_sin_series(
        g: *mut arb_poly_struct,
        h: *mut arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_cos_series(
        g: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_cos_series(
        g: *mut arb_poly_struct,
        h: *mut arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_sin_pi_series(
        g: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_sin_pi_series(
        g: *mut arb_poly_struct,
        h: *mut arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_cos_pi_series(
        g: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_cos_pi_series(
        g: *mut arb_poly_struct,
        h: *mut arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_cot_pi_series(
        g: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_cot_pi_series(
        g: *mut arb_poly_struct,
        h: *mut arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_tan_series(
        g: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_tan_series(
        g: *mut arb_poly_struct,
        h: *mut arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_sinc_series(
        g: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_sinc_series(
        g: *mut arb_poly_struct,
        h: *mut arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_sinc_pi_series(
        g: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_sinc_pi_series(
        g: *mut arb_poly_struct,
        h: *mut arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_compose_series_brent_kung(
        res: arb_ptr,
        poly1: arb_srcptr,
        len1: mp_limb_signed_t,
        poly2: arb_srcptr,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_compose_series_brent_kung(
        res: *mut arb_poly_struct,
        poly1: *mut arb_poly_struct,
        poly2: *mut arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_evaluate_acb_horner(
        res: *mut acb_struct,
        f: arb_srcptr,
        len: mp_limb_signed_t,
        x: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_evaluate_acb_horner(
        res: *mut acb_struct,
        f: *mut arb_poly_struct,
        a: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_evaluate_acb_rectangular(
        y: *mut acb_struct,
        poly: arb_srcptr,
        len: mp_limb_signed_t,
        x: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_evaluate_acb_rectangular(
        res: *mut acb_struct,
        f: *mut arb_poly_struct,
        a: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_evaluate_acb(
        res: *mut acb_struct,
        f: arb_srcptr,
        len: mp_limb_signed_t,
        x: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_evaluate_acb(
        res: *mut acb_struct,
        f: *mut arb_poly_struct,
        a: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_evaluate2_acb_horner(
        y: *mut acb_struct,
        z: *mut acb_struct,
        f: arb_srcptr,
        len: mp_limb_signed_t,
        x: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_evaluate2_acb_horner(
        y: *mut acb_struct,
        z: *mut acb_struct,
        f: *mut arb_poly_struct,
        x: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_evaluate2_acb_rectangular(
        y: *mut acb_struct,
        z: *mut acb_struct,
        f: arb_srcptr,
        len: mp_limb_signed_t,
        x: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_evaluate2_acb_rectangular(
        y: *mut acb_struct,
        z: *mut acb_struct,
        f: *mut arb_poly_struct,
        x: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_evaluate2_acb(
        y: *mut acb_struct,
        z: *mut acb_struct,
        f: arb_srcptr,
        len: mp_limb_signed_t,
        x: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_evaluate2_acb(
        y: *mut acb_struct,
        z: *mut acb_struct,
        f: *mut arb_poly_struct,
        x: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_lambertw_series(
        res: arb_ptr,
        z: arb_srcptr,
        zlen: mp_limb_signed_t,
        flags: c_int,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_lambertw_series(
        res: *mut arb_poly_struct,
        z: *mut arb_poly_struct,
        flags: c_int,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_gamma_series(
        res: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_gamma_series(
        res: *mut arb_poly_struct,
        f: *mut arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_rgamma_series(
        res: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_rgamma_series(
        res: *mut arb_poly_struct,
        f: *mut arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_lgamma_series(
        res: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_lgamma_series(
        res: *mut arb_poly_struct,
        f: *mut arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_digamma_series(
        res: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_digamma_series(
        res: *mut arb_poly_struct,
        f: *mut arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_rising_ui_series(
        res: arb_ptr,
        f: arb_srcptr,
        flen: mp_limb_signed_t,
        r: mp_limb_t,
        trunc: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_rising_ui_series(
        res: *mut arb_poly_struct,
        f: *mut arb_poly_struct,
        r: mp_limb_t,
        trunc: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_zeta_series(
        res: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        a: *mut arb_struct,
        deflate: c_int,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_zeta_series(
        res: *mut arb_poly_struct,
        f: *mut arb_poly_struct,
        a: *mut arb_struct,
        deflate: c_int,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_riemann_siegel_theta_series(
        res: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_riemann_siegel_theta_series(
        res: *mut arb_poly_struct,
        h: *mut arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_riemann_siegel_z_series(
        res: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_riemann_siegel_z_series(
        res: *mut arb_poly_struct,
        h: *mut arb_poly_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_swinnerton_dyer_ui_prec(n: mp_limb_t) -> mp_limb_signed_t;
    pub fn _arb_poly_swinnerton_dyer_ui(
        T: arb_ptr,
        n: mp_limb_t,
        trunc: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_poly_swinnerton_dyer_ui(
        poly: *mut arb_poly_struct,
        n: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_newton_convergence_factor(
        convergence_factor: *mut arf_struct,
        poly: arb_srcptr,
        len: mp_limb_signed_t,
        convergence_interval: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_newton_step(
        xnew: *mut arb_struct,
        poly: arb_srcptr,
        len: mp_limb_signed_t,
        x: *mut arb_struct,
        convergence_interval: *mut arb_struct,
        convergence_factor: *mut arf_struct,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn _arb_poly_newton_refine_root(
        r: *mut arb_struct,
        poly: arb_srcptr,
        len: mp_limb_signed_t,
        start: *mut arb_struct,
        convergence_interval: *mut arb_struct,
        convergence_factor: *mut arf_struct,
        eval_extra_prec: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_poly_root_bound_fujiwara(
        bound: *mut mag_struct,
        poly: arb_srcptr,
        len: mp_limb_signed_t,
    );
    pub fn arb_poly_root_bound_fujiwara(bound: *mut mag_struct, poly: *mut arb_poly_struct);
}

pub unsafe fn arb_poly_length(poly: *const arb_poly_struct) -> mp_limb_signed_t {
    (*poly).length
}

pub unsafe fn arb_poly_degree(poly: *const arb_poly_struct) -> mp_limb_signed_t {
    (*poly).length - 1
}

pub unsafe fn arb_poly_is_zero(poly: *const arb_poly_struct) -> bool {
    (*poly).length == 0
}

pub unsafe fn arb_poly_is_one(poly: *const arb_poly_struct) -> bool {
    ((*poly).length == 1) && (arb_is_one((*poly).coeffs) != 0)
}

/*
pub unsafe fn arb_poly_is_x(poly: *const arb_poly_struct) -> bool {
    ((*poly).length == 2) && (arb_is_zero((*poly).coeffs) != 0)
        && (arb_is_one((*poly).coeffs + 1) != 0)
}*/
