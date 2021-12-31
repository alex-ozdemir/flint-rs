#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

//! *See the [FLINT documentation](http://flintlib.org/doc/n_poly.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpz::fmpz;
use crate::fq_nmod::fq_nmod_ctx_struct;
use crate::fq_nmod_poly::fq_nmod_poly_struct;
use crate::nmod_poly::nmod_poly_struct;
use crate::nmod_vec::nmod_t;
use libc::{c_char, c_int, FILE};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct n_poly_struct {
    pub coeffs: *mut mp_limb_t,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}
pub type n_poly_t = [n_poly_struct; 1usize];
pub type n_fq_poly_struct = n_poly_struct;
pub type n_fq_poly_t = n_poly_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct n_bpoly_struct {
    pub coeffs: *mut n_poly_struct,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}
pub type n_bpoly_t = [n_bpoly_struct; 1usize];
pub type n_fq_bpoly_struct = n_bpoly_struct;
pub type n_fq_bpoly_t = n_bpoly_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct n_tpoly_struct {
    pub coeffs: *mut n_bpoly_struct,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}
pub type n_tpoly_t = [n_tpoly_struct; 1usize];
pub type n_fq_tpoly_struct = n_tpoly_struct;
pub type n_fq_tpoly_t = n_tpoly_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct n_polyu_struct {
    pub exps: *mut mp_limb_t,
    pub coeffs: *mut mp_limb_t,
    pub length: mp_limb_signed_t,
    pub alloc: mp_limb_signed_t,
}
pub type n_polyu_t = [n_polyu_struct; 1usize];
pub type n_fq_polyu_struct = n_polyu_struct;
pub type n_fq_polyu_t = n_polyu_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct n_polyun_struct {
    pub coeffs: *mut n_poly_struct,
    pub exps: *mut mp_limb_t,
    pub length: mp_limb_signed_t,
    pub alloc: mp_limb_signed_t,
}
pub type n_polyun_t = [n_polyun_struct; 1usize];
pub type n_fq_polyun_struct = n_polyun_struct;
pub type n_fq_polyun_t = n_polyun_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct n_poly_stack_struct {
    pub array: *mut *mut n_poly_struct,
    pub alloc: mp_limb_signed_t,
    pub top: mp_limb_signed_t,
}
pub type n_poly_stack_t = [n_poly_stack_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct n_bpoly_stack_struct {
    pub array: *mut *mut n_bpoly_struct,
    pub alloc: mp_limb_signed_t,
    pub top: mp_limb_signed_t,
}
pub type n_bpoly_stack_t = [n_bpoly_stack_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct n_poly_bpoly_stack_struct {
    pub poly_stack: n_poly_stack_t,
    pub bpoly_stack: n_bpoly_stack_t,
}
pub type n_poly_bpoly_stack_t = [n_poly_bpoly_stack_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct n_polyun_stack_struct {
    pub array: *mut *mut n_polyun_struct,
    pub alloc: mp_limb_signed_t,
    pub top: mp_limb_signed_t,
}
pub type n_polyun_stack_t = [n_polyun_stack_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct n_poly_polyun_stack_struct {
    pub poly_stack: n_poly_stack_t,
    pub polyun_stack: n_polyun_stack_t,
}
pub type n_poly_polyun_stack_t = [n_poly_polyun_stack_struct; 1usize];
extern "C" {
    pub fn n_poly_init(A: *mut n_poly_struct);
}
extern "C" {
    pub fn n_poly_init2(A: *mut n_poly_struct, alloc: mp_limb_signed_t);
}
extern "C" {
    pub fn n_poly_clear(A: *mut n_poly_struct);
}
extern "C" {
    pub fn n_poly_is_canonical(A: *mut n_poly_struct) -> c_int;
}
extern "C" {
    pub fn n_poly_realloc(A: *mut n_poly_struct, len: mp_limb_signed_t);
}
extern "C" {
    pub fn n_poly_print_pretty(A: *mut n_poly_struct, x: *const c_char);
}
extern "C" {
    pub fn n_poly_fit_length(A: *mut n_poly_struct, len: mp_limb_signed_t);
}
extern "C" {
    pub fn nmod_poly_mock(a: *mut nmod_poly_struct, b: *mut n_poly_struct, mod_: nmod_t);
}
extern "C" {
    pub fn n_poly_mock(a: *mut n_poly_struct, b: *mut nmod_poly_struct);
}
extern "C" {
    pub fn n_poly_set(A: *mut n_poly_struct, B: *mut n_poly_struct);
}
extern "C" {
    pub fn n_poly_swap(A: *mut n_poly_struct, B: *mut n_poly_struct);
}
extern "C" {
    pub fn _n_poly_normalise(A: *mut n_poly_struct);
}
extern "C" {
    pub fn n_poly_degree(A: *mut n_poly_struct) -> mp_limb_signed_t;
}
extern "C" {
    pub fn n_poly_is_one(A: *mut n_poly_struct) -> c_int;
}
extern "C" {
    pub fn n_poly_lead(A: *mut n_poly_struct) -> mp_limb_t;
}
extern "C" {
    pub fn n_poly_one(A: *mut n_poly_struct);
}
extern "C" {
    pub fn n_poly_set_ui(A: *mut n_poly_struct, c: mp_limb_t);
}
extern "C" {
    pub fn n_poly_is_zero(poly: *mut n_poly_struct) -> c_int;
}
extern "C" {
    pub fn n_poly_zero(res: *mut n_poly_struct);
}
extern "C" {
    pub fn n_poly_equal(a: *mut n_poly_struct, b: *mut n_poly_struct) -> c_int;
}
extern "C" {
    pub fn n_poly_mod_is_canonical(A: *mut n_poly_struct, mod_: nmod_t) -> c_int;
}
extern "C" {
    pub fn n_poly_mod_make_monic(A: *mut n_poly_struct, B: *mut n_poly_struct, mod_: nmod_t);
}
extern "C" {
    pub fn n_poly_mod_taylor_shift(g: *mut n_poly_struct, c: mp_limb_t, mod_: nmod_t);
}
extern "C" {
    pub fn n_poly_get_coeff(poly: *mut n_poly_struct, j: mp_limb_signed_t) -> mp_limb_t;
}
extern "C" {
    pub fn n_poly_set_coeff_nonzero(A: *mut n_poly_struct, j: mp_limb_signed_t, c: mp_limb_t);
}
extern "C" {
    pub fn n_poly_set_coeff(A: *mut n_poly_struct, e: mp_limb_signed_t, c: mp_limb_t);
}
extern "C" {
    pub fn n_poly_mod_set_coeff_ui(
        A: *mut n_poly_struct,
        j: mp_limb_signed_t,
        c: mp_limb_t,
        mod_: nmod_t,
    );
}
extern "C" {
    pub fn n_poly_set_nmod_poly(a: *mut n_poly_struct, b: *mut nmod_poly_struct);
}
extern "C" {
    pub fn nmod_poly_set_n_poly(a: *mut nmod_poly_struct, b: *mut n_poly_struct);
}
extern "C" {
    pub fn n_poly_shift_left(A: *mut n_poly_struct, B: *mut n_poly_struct, k: mp_limb_signed_t);
}
extern "C" {
    pub fn n_poly_shift_right(
        res: *mut n_poly_struct,
        poly: *mut n_poly_struct,
        k: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn n_poly_truncate(poly: *mut n_poly_struct, len: mp_limb_signed_t);
}
extern "C" {
    pub fn _n_poly_mod_scalar_mul_nmod(
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        c: mp_limb_t,
        mod_: nmod_t,
    );
}
extern "C" {
    pub fn _n_poly_mod_scalar_mul_nmod_inplace(A: *mut n_poly_struct, c: mp_limb_t, mod_: nmod_t);
}
extern "C" {
    pub fn n_poly_mod_scalar_mul_ui(
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        c: mp_limb_t,
        ctx: nmod_t,
    );
}
extern "C" {
    pub fn n_poly_mod_eval_step2(
        Acur: *mut n_poly_struct,
        Ainc: *mut n_poly_struct,
        mod_: nmod_t,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn n_poly_mod_evaluate_nmod(A: *mut n_poly_struct, c: mp_limb_t, mod_: nmod_t)
        -> mp_limb_t;
}
extern "C" {
    pub fn n_poly_mod_neg(A: *mut n_poly_struct, B: *mut n_poly_struct, mod_: nmod_t);
}
extern "C" {
    pub fn n_poly_mod_add(
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        C: *mut n_poly_struct,
        mod_: nmod_t,
    );
}
extern "C" {
    pub fn n_poly_mod_add_ui(
        res: *mut n_poly_struct,
        poly: *mut n_poly_struct,
        c: mp_limb_t,
        ctx: nmod_t,
    );
}
extern "C" {
    pub fn n_poly_mod_sub(
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        C: *mut n_poly_struct,
        mod_: nmod_t,
    );
}
extern "C" {
    pub fn n_poly_mod_product_roots_nmod_vec(
        A: *mut n_poly_struct,
        r: mp_srcptr,
        n: mp_limb_signed_t,
        mod_: nmod_t,
    );
}
extern "C" {
    pub fn n_poly_mod_shift_left_scalar_addmul(
        A: *mut n_poly_struct,
        k: mp_limb_signed_t,
        c: mp_limb_t,
        mod_: nmod_t,
    );
}
extern "C" {
    pub fn n_poly_mod_addmul_linear(
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        C: *mut n_poly_struct,
        d1: mp_limb_t,
        d0: mp_limb_t,
        mod_: nmod_t,
    );
}
extern "C" {
    pub fn n_poly_mod_scalar_addmul_nmod(
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        C: *mut n_poly_struct,
        d0: mp_limb_t,
        ctx: nmod_t,
    );
}
extern "C" {
    pub fn _n_poly_eval_pow(
        P: *mut n_poly_struct,
        alphapow: *mut n_poly_struct,
        nlimbs: c_int,
        ctx: nmod_t,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn n_poly_mod_eval_pow(
        P: *mut n_poly_struct,
        alphapow: *mut n_poly_struct,
        ctx: nmod_t,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn n_poly_mod_eval2_pow(
        vp: *mut mp_limb_t,
        vm: *mut mp_limb_t,
        P: *mut n_poly_struct,
        alphapow: *mut n_poly_struct,
        ctx: nmod_t,
    );
}
extern "C" {
    pub fn n_poly_mod_div_root(
        Q: *mut n_poly_struct,
        A: *mut n_poly_struct,
        c: mp_limb_t,
        ctx: nmod_t,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn _n_poly_mod_mul(
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        C: *mut n_poly_struct,
        ctx: nmod_t,
    );
}
extern "C" {
    pub fn _n_poly_mod_div(
        Q: *mut n_poly_struct,
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        mod_: nmod_t,
    );
}
extern "C" {
    pub fn _n_poly_mod_rem(
        R: *mut n_poly_struct,
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        mod_: nmod_t,
    );
}
extern "C" {
    pub fn _n_poly_mod_divrem(
        Q: *mut n_poly_struct,
        R: *mut n_poly_struct,
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        mod_: nmod_t,
    );
}
extern "C" {
    pub fn n_poly_mod_remove(
        f: *mut n_poly_struct,
        p: *mut n_poly_struct,
        ctx: nmod_t,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn n_poly_mod_pow(
        res: *mut n_poly_struct,
        poly: *mut n_poly_struct,
        e: mp_limb_t,
        ctx: nmod_t,
    );
}
extern "C" {
    pub fn n_poly_mod_mul(
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        C: *mut n_poly_struct,
        mod_: nmod_t,
    );
}
extern "C" {
    pub fn n_poly_mod_mullow(
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        C: *mut n_poly_struct,
        n: mp_limb_signed_t,
        mod_: nmod_t,
    );
}
extern "C" {
    pub fn n_poly_mod_div(
        Q: *mut n_poly_struct,
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        mod_: nmod_t,
    );
}
extern "C" {
    pub fn n_poly_mod_rem(
        R: *mut n_poly_struct,
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        mod_: nmod_t,
    );
}
extern "C" {
    pub fn n_poly_mod_divrem(
        Q: *mut n_poly_struct,
        R: *mut n_poly_struct,
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        mod_: nmod_t,
    );
}
extern "C" {
    pub fn n_poly_mod_mulmod(
        res: *mut n_poly_struct,
        poly1: *mut n_poly_struct,
        poly2: *mut n_poly_struct,
        f: *mut n_poly_struct,
        mod_: nmod_t,
    );
}
extern "C" {
    pub fn n_poly_mod_invmod(
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        P: *mut n_poly_struct,
        mod_: nmod_t,
    ) -> c_int;
}
extern "C" {
    pub fn n_poly_mod_gcd(
        G: *mut n_poly_struct,
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        mod_: nmod_t,
    );
}
extern "C" {
    pub fn n_poly_mod_xgcd(
        G: *mut n_poly_struct,
        S: *mut n_poly_struct,
        T: *mut n_poly_struct,
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        mod_: nmod_t,
    );
}
extern "C" {
    pub fn n_poly_mod_inv_series(
        Qinv: *mut n_poly_struct,
        Q: *mut n_poly_struct,
        n: mp_limb_signed_t,
        mod_: nmod_t,
    );
}
extern "C" {
    pub fn n_poly_mod_div_series(
        Q: *mut n_poly_struct,
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        order: mp_limb_signed_t,
        ctx: nmod_t,
    );
}
extern "C" {
    pub fn n_poly_reverse(
        output: *mut n_poly_struct,
        input: *mut n_poly_struct,
        m: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn n_poly_mod_mulmod_preinv(
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        C: *mut n_poly_struct,
        M: *mut n_poly_struct,
        Minv: *mut n_poly_struct,
        ctx: nmod_t,
    );
}
extern "C" {
    pub fn fq_nmod_ctx_mod(ctx: *mut fq_nmod_ctx_struct) -> nmod_t;
}
extern "C" {
    pub fn _n_fq_is_zero(a: *const mp_limb_t, d: mp_limb_signed_t) -> c_int;
}
extern "C" {
    pub fn _n_fq_zero(a: *mut mp_limb_t, d: mp_limb_signed_t);
}
extern "C" {
    pub fn _n_fq_is_one(a: *const mp_limb_t, d: mp_limb_signed_t) -> c_int;
}
extern "C" {
    pub fn _n_fq_is_ui(a: *const mp_limb_t, d: mp_limb_signed_t) -> c_int;
}
extern "C" {
    pub fn n_fq_is_one(a: *const mp_limb_t, ctx: *mut fq_nmod_ctx_struct) -> c_int;
}
extern "C" {
    pub fn _n_fq_one(a: *mut mp_limb_t, d: mp_limb_signed_t);
}
extern "C" {
    pub fn _n_fq_set_nmod(a: *mut mp_limb_t, b: mp_limb_t, d: mp_limb_signed_t);
}
extern "C" {
    pub fn n_fq_gen(a: *mut mp_limb_t, ctx: *mut fq_nmod_ctx_struct);
}
extern "C" {
    pub fn _n_fq_set(a: *mut mp_limb_t, b: *const mp_limb_t, d: mp_limb_signed_t);
}
extern "C" {
    pub fn _n_fq_swap(a: *mut mp_limb_t, b: *mut mp_limb_t, d: mp_limb_signed_t);
}
extern "C" {
    pub fn _n_fq_equal(a: *mut mp_limb_t, b: *const mp_limb_t, d: mp_limb_signed_t) -> c_int;
}
extern "C" {
    pub fn n_fq_equal_fq_nmod(
        a: *const mp_limb_t,
        b: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn n_fq_is_canonical(a: *const mp_limb_t, ctx: *mut fq_nmod_ctx_struct) -> c_int;
}
extern "C" {
    pub fn n_fq_randtest_not_zero(
        a: *mut mp_limb_t,
        state: *mut flint_rand_s,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_get_str_pretty(a: *const mp_limb_t, ctx: *mut fq_nmod_ctx_struct) -> *mut c_char;
}
extern "C" {
    pub fn n_fq_fprint_pretty(
        file: *mut FILE,
        a: *const mp_limb_t,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn n_fq_print_pretty(a: *const mp_limb_t, ctx: *mut fq_nmod_ctx_struct);
}
extern "C" {
    pub fn n_fq_get_fq_nmod(
        a: *mut nmod_poly_struct,
        b: *const mp_limb_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_set_fq_nmod(
        a: *mut mp_limb_t,
        b: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_get_n_poly(
        a: *mut n_poly_struct,
        b: *const mp_limb_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn _n_fq_set_n_poly(
        a: *mut mp_limb_t,
        bcoeffs: *const mp_limb_t,
        blen: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_add_si(
        a: *mut mp_limb_t,
        b: *const mp_limb_t,
        c: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_add(
        a: *mut mp_limb_t,
        b: *const mp_limb_t,
        c: *const mp_limb_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_add_fq_nmod(
        a: *mut mp_limb_t,
        b: *const mp_limb_t,
        c: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_sub_fq_nmod(
        a: *mut mp_limb_t,
        b: *const mp_limb_t,
        c: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_sub(
        a: *mut mp_limb_t,
        b: *const mp_limb_t,
        c: *const mp_limb_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn _n_fq_add(
        a: *mut mp_limb_t,
        b: *const mp_limb_t,
        c: *const mp_limb_t,
        d: mp_limb_signed_t,
        mod_: nmod_t,
    );
}
extern "C" {
    pub fn _n_fq_sub(
        a: *mut mp_limb_t,
        b: *const mp_limb_t,
        c: *const mp_limb_t,
        d: mp_limb_signed_t,
        mod_: nmod_t,
    );
}
extern "C" {
    pub fn _n_fq_neg(a: *mut mp_limb_t, b: *const mp_limb_t, d: mp_limb_signed_t, mod_: nmod_t);
}
extern "C" {
    pub fn _n_fq_mul_ui(
        a: *mut mp_limb_t,
        b: *const mp_limb_t,
        c: mp_limb_t,
        d: mp_limb_signed_t,
        mod_: nmod_t,
    );
}
extern "C" {
    pub fn _n_fq_madd2(
        a: *mut mp_limb_t,
        b: *const mp_limb_t,
        c: *const mp_limb_t,
        ctx: *mut fq_nmod_ctx_struct,
        t: *mut mp_limb_t,
    );
}
extern "C" {
    pub fn _n_fq_mul2(
        t: *mut mp_limb_t,
        b: *const mp_limb_t,
        c: *const mp_limb_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn _n_fq_reduce(
        a: *mut mp_limb_t,
        b: *mut mp_limb_t,
        blen: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
        t: *mut mp_limb_t,
    );
}
extern "C" {
    pub fn _n_fq_reduce2(
        a: *mut mp_limb_t,
        b: *mut mp_limb_t,
        ctx: *mut fq_nmod_ctx_struct,
        t: *mut mp_limb_t,
    );
}
extern "C" {
    pub fn _n_fq_mul(
        a: *mut mp_limb_t,
        b: *const mp_limb_t,
        c: *const mp_limb_t,
        ctx: *mut fq_nmod_ctx_struct,
        t: *mut mp_limb_t,
    );
}
extern "C" {
    pub fn _n_fq_addmul(
        a: *mut mp_limb_t,
        b: *const mp_limb_t,
        c: *const mp_limb_t,
        e: *const mp_limb_t,
        ctx: *mut fq_nmod_ctx_struct,
        t: *mut mp_limb_t,
    );
}
extern "C" {
    pub fn _n_fq_dot_lazy_size(len: mp_limb_signed_t, ctx: *mut fq_nmod_ctx_struct) -> c_int;
}
extern "C" {
    pub fn _n_fq_reduce2_lazy1(a: *mut mp_limb_t, d: mp_limb_signed_t, ctx: nmod_t);
}
extern "C" {
    pub fn _n_fq_madd2_lazy1(
        a: *mut mp_limb_t,
        b: *const mp_limb_t,
        c: *const mp_limb_t,
        d: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _n_fq_mul2_lazy1(
        a: *mut mp_limb_t,
        b: *const mp_limb_t,
        c: *const mp_limb_t,
        d: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _n_fq_reduce2_lazy2(a: *mut mp_limb_t, d: mp_limb_signed_t, ctx: nmod_t);
}
extern "C" {
    pub fn _n_fq_madd2_lazy2(
        a: *mut mp_limb_t,
        b: *const mp_limb_t,
        c: *const mp_limb_t,
        d: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _n_fq_mul2_lazy2(
        a: *mut mp_limb_t,
        b: *const mp_limb_t,
        c: *const mp_limb_t,
        d: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _n_fq_reduce2_lazy3(a: *mut mp_limb_t, d: mp_limb_signed_t, ctx: nmod_t);
}
extern "C" {
    pub fn _n_fq_madd2_lazy3(
        a: *mut mp_limb_t,
        b: *const mp_limb_t,
        c: *const mp_limb_t,
        d: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _n_fq_mul2_lazy3(
        a: *mut mp_limb_t,
        b: *const mp_limb_t,
        c: *const mp_limb_t,
        d: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _n_fq_inv(
        a: *mut mp_limb_t,
        b: *const mp_limb_t,
        ctx: *mut fq_nmod_ctx_struct,
        t: *mut mp_limb_t,
    );
}
extern "C" {
    pub fn _n_fq_pow_ui(
        a: *mut mp_limb_t,
        b: *const mp_limb_t,
        e: mp_limb_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_pow_fmpz(
        a: *mut mp_limb_t,
        b: *const mp_limb_t,
        e: *mut fmpz,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_mul(
        a: *mut mp_limb_t,
        b: *const mp_limb_t,
        c: *const mp_limb_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_mul_fq_nmod(
        a: *mut mp_limb_t,
        b: *const mp_limb_t,
        c: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_addmul(
        a: *mut mp_limb_t,
        b: *const mp_limb_t,
        c: *const mp_limb_t,
        d: *const mp_limb_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_inv(a: *mut mp_limb_t, b: *const mp_limb_t, ctx: *mut fq_nmod_ctx_struct);
}
extern "C" {
    pub fn n_fq_pow_ui(
        a: *mut mp_limb_t,
        b: *const mp_limb_t,
        e: mp_limb_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_poly_init2(
        A: *mut n_poly_struct,
        alloc: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn _n_fq_poly_one(A: *mut n_poly_struct, d: mp_limb_signed_t);
}
extern "C" {
    pub fn n_fq_poly_one(A: *mut n_poly_struct, ctx: *mut fq_nmod_ctx_struct);
}
extern "C" {
    pub fn n_fq_poly_is_one(A: *mut n_poly_struct, ctx: *mut fq_nmod_ctx_struct) -> c_int;
}
extern "C" {
    pub fn n_fq_poly_is_canonical(a: *mut n_poly_struct, ctx: *mut fq_nmod_ctx_struct) -> c_int;
}
extern "C" {
    pub fn _n_fq_poly_normalise(A: *mut n_poly_struct, d: mp_limb_signed_t);
}
extern "C" {
    pub fn n_fq_poly_print_pretty(
        A: *mut n_poly_struct,
        x: *const c_char,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_poly_equal(
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn n_fq_poly_set(
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_poly_randtest(
        A: *mut n_poly_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_poly_make_monic(
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_poly_get_coeff_n_fq(
        c: *mut mp_limb_t,
        A: *mut n_poly_struct,
        e: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_poly_get_coeff_fq_nmod(
        c: *mut nmod_poly_struct,
        A: *mut n_poly_struct,
        e: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_poly_set_coeff_n_fq(
        A: *mut n_poly_struct,
        j: mp_limb_signed_t,
        c: *const mp_limb_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_poly_set_coeff_fq_nmod(
        A: *mut n_poly_struct,
        j: mp_limb_signed_t,
        c: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_poly_scalar_mul_n_fq(
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        c: *const mp_limb_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_poly_scalar_mul_ui(
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        c: mp_limb_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_poly_scalar_addmul_n_fq(
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        C: *mut n_poly_struct,
        d: *const mp_limb_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_poly_shift_left_scalar_submul(
        A: *mut n_poly_struct,
        k: mp_limb_signed_t,
        c: *const mp_limb_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_poly_evaluate_fq_nmod(
        e: *mut nmod_poly_struct,
        A: *mut n_poly_struct,
        c: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_poly_evaluate_n_fq(
        e: *mut mp_limb_t,
        A: *mut n_poly_struct,
        c: *const mp_limb_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_poly_get_fq_nmod_poly(
        A: *mut fq_nmod_poly_struct,
        B: *mut n_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_poly_set_fq_nmod_poly(
        A: *mut n_poly_struct,
        B: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_poly_set_n_fq(
        A: *mut n_poly_struct,
        c: *const mp_limb_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_poly_set_fq_nmod(
        A: *mut n_poly_struct,
        c: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_poly_shift_right(
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_poly_shift_left(
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        n: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_poly_truncate(
        A: *mut n_poly_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_poly_add(
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        C: *mut n_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_poly_sub(
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        C: *mut n_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_poly_neg(
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_poly_add_si(
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        c: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn _n_fq_poly_mul_(
        A: *mut mp_limb_t,
        B: *const mp_limb_t,
        Blen: mp_limb_signed_t,
        C: *const mp_limb_t,
        Clen: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
        St: *mut n_poly_stack_struct,
    );
}
extern "C" {
    pub fn n_fq_poly_mul_(
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        C: *mut n_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
        St: *mut n_poly_stack_struct,
    );
}
extern "C" {
    pub fn n_fq_poly_mul(
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        C: *mut n_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_poly_pow(
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        e: mp_limb_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_poly_remove(
        f: *mut n_poly_struct,
        g: *mut n_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn n_fq_poly_divrem_divconquer_(
        Q: *mut n_poly_struct,
        R: *mut n_poly_struct,
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
        St: *mut n_poly_stack_struct,
    );
}
extern "C" {
    pub fn n_fq_poly_divrem_(
        Q: *mut n_poly_struct,
        R: *mut n_poly_struct,
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
        St: *mut n_poly_stack_struct,
    );
}
extern "C" {
    pub fn n_fq_poly_divrem(
        Q: *mut n_poly_struct,
        R: *mut n_poly_struct,
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_poly_gcd(
        G: *mut n_poly_struct,
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_poly_gcd_(
        G: *mut n_poly_struct,
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
        St: *mut n_poly_stack_struct,
    );
}
extern "C" {
    pub fn n_fq_poly_xgcd(
        G: *mut n_poly_struct,
        S: *mut n_poly_struct,
        T: *mut n_poly_struct,
        B: *mut n_poly_struct,
        C: *mut n_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_poly_mulmod(
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        C: *mut n_poly_struct,
        M: *mut n_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_poly_rem(
        R: *mut n_poly_struct,
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_poly_mullow(
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        C: *mut n_poly_struct,
        order: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_poly_inv_series(
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        order: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_poly_eval_pow(
        ev: *mut mp_limb_t,
        A: *mut n_poly_struct,
        alphapow: *mut n_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_bpoly_init(A: *mut n_bpoly_struct);
}
extern "C" {
    pub fn n_bpoly_clear(A: *mut n_bpoly_struct);
}
extern "C" {
    pub fn n_bpoly_swap(A: *mut n_bpoly_struct, B: *mut n_bpoly_struct);
}
extern "C" {
    pub fn n_bpoly_print_pretty(A: *mut n_bpoly_struct, xvar: *const c_char, yvar: *const c_char);
}
extern "C" {
    pub fn n_bpoly_normalise(A: *mut n_bpoly_struct);
}
extern "C" {
    pub fn n_bpoly_realloc(A: *mut n_bpoly_struct, len: mp_limb_signed_t);
}
extern "C" {
    pub fn n_bpoly_fit_length(A: *mut n_bpoly_struct, len: mp_limb_signed_t);
}
extern "C" {
    pub fn n_bpoly_zero(A: *mut n_bpoly_struct);
}
extern "C" {
    pub fn n_bpoly_is_zero(A: *mut n_bpoly_struct) -> c_int;
}
extern "C" {
    pub fn _n_bpoly_set(A: *mut n_bpoly_struct, B: *mut n_bpoly_struct);
}
extern "C" {
    pub fn n_bpoly_set(A: *mut n_bpoly_struct, B: *mut n_bpoly_struct);
}
extern "C" {
    pub fn n_bpoly_one(A: *mut n_bpoly_struct);
}
extern "C" {
    pub fn n_bpoly_equal(A: *mut n_bpoly_struct, B: *mut n_bpoly_struct) -> c_int;
}
extern "C" {
    pub fn n_bpoly_set_coeff(
        A: *mut n_bpoly_struct,
        e0: mp_limb_signed_t,
        e1: mp_limb_signed_t,
        c: mp_limb_t,
    );
}
extern "C" {
    pub fn n_bpoly_set_coeff_nonzero(
        A: *mut n_bpoly_struct,
        e0: mp_limb_signed_t,
        e1: mp_limb_signed_t,
        c: mp_limb_t,
    );
}
extern "C" {
    pub fn n_bpoly_mod_derivative_gen0(A: *mut n_bpoly_struct, B: *mut n_bpoly_struct, ctx: nmod_t);
}
extern "C" {
    pub fn n_bpoly_get_coeff(
        A: *mut n_bpoly_struct,
        e0: mp_limb_signed_t,
        e1: mp_limb_signed_t,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn n_bpoly_degree0(A: *mut n_bpoly_struct) -> mp_limb_signed_t;
}
extern "C" {
    pub fn n_bpoly_degree1(A: *mut n_bpoly_struct) -> mp_limb_signed_t;
}
extern "C" {
    pub fn n_bpoly_set_poly_gen1(A: *mut n_bpoly_struct, B: *mut n_poly_struct);
}
extern "C" {
    pub fn n_bpoly_set_poly_gen0(A: *mut n_bpoly_struct, B: *mut n_poly_struct);
}
extern "C" {
    pub fn n_bpoly_mod_is_canonical(A: *mut n_bpoly_struct, mod_: nmod_t) -> c_int;
}
extern "C" {
    pub fn n_bpoly_bidegree(A: *mut n_bpoly_struct) -> mp_limb_t;
}
extern "C" {
    pub fn n_bpoly_scalar_mul_nmod(A: *mut n_bpoly_struct, c: mp_limb_t, ctx: nmod_t);
}
extern "C" {
    pub fn n_bpoly_mod_content_last(g: *mut n_poly_struct, A: *mut n_bpoly_struct, ctx: nmod_t);
}
extern "C" {
    pub fn n_bpoly_mod_divexact_last(A: *mut n_bpoly_struct, b: *mut n_poly_struct, ctx: nmod_t);
}
extern "C" {
    pub fn n_bpoly_mod_mul_last(A: *mut n_bpoly_struct, b: *mut n_poly_struct, ctx: nmod_t);
}
extern "C" {
    pub fn n_bpoly_mod_taylor_shift_gen1(
        A: *mut n_bpoly_struct,
        B: *mut n_bpoly_struct,
        c: mp_limb_t,
        ctx: nmod_t,
    );
}
extern "C" {
    pub fn n_bpoly_mod_taylor_shift_gen0(A: *mut n_bpoly_struct, c: mp_limb_t, ctx: nmod_t);
}
extern "C" {
    pub fn n_bpoly_mod_add(
        A: *mut n_bpoly_struct,
        B: *mut n_bpoly_struct,
        C: *mut n_bpoly_struct,
        ctx: nmod_t,
    );
}
extern "C" {
    pub fn n_bpoly_mod_sub(
        A: *mut n_bpoly_struct,
        B: *mut n_bpoly_struct,
        C: *mut n_bpoly_struct,
        ctx: nmod_t,
    );
}
extern "C" {
    pub fn n_bpoly_mod_make_primitive(g: *mut n_poly_struct, A: *mut n_bpoly_struct, ctx: nmod_t);
}
extern "C" {
    pub fn n_bpoly_mod_mul(
        A: *mut n_bpoly_struct,
        B: *mut n_bpoly_struct,
        C: *mut n_bpoly_struct,
        ctx: nmod_t,
    );
}
extern "C" {
    pub fn n_bpoly_mod_divides(
        Q: *mut n_bpoly_struct,
        A: *mut n_bpoly_struct,
        B: *mut n_bpoly_struct,
        ctx: nmod_t,
    ) -> c_int;
}
extern "C" {
    pub fn n_bpoly_mod_mul_series(
        A: *mut n_bpoly_struct,
        B: *mut n_bpoly_struct,
        C: *mut n_bpoly_struct,
        order: mp_limb_signed_t,
        ctx: nmod_t,
    );
}
extern "C" {
    pub fn n_bpoly_mod_divrem_series(
        Q: *mut n_bpoly_struct,
        R: *mut n_bpoly_struct,
        A: *mut n_bpoly_struct,
        B: *mut n_bpoly_struct,
        order: mp_limb_signed_t,
        ctx: nmod_t,
    );
}
extern "C" {
    pub fn n_bpoly_mod_interp_reduce_2sm_poly(
        Ap: *mut n_poly_struct,
        Am: *mut n_poly_struct,
        A: *mut n_bpoly_struct,
        alphapow: *mut n_poly_struct,
        mod_: nmod_t,
    );
}
extern "C" {
    pub fn n_bpoly_mod_interp_lift_2sm_poly(
        deg1: *mut mp_limb_signed_t,
        T: *mut n_bpoly_struct,
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        alpha: mp_limb_t,
        mod_: nmod_t,
    );
}
extern "C" {
    pub fn n_bpoly_mod_interp_crt_2sm_poly(
        deg1: *mut mp_limb_signed_t,
        F: *mut n_bpoly_struct,
        T: *mut n_bpoly_struct,
        A: *mut n_poly_struct,
        B: *mut n_poly_struct,
        modulus: *mut n_poly_struct,
        alphapow: *mut n_poly_struct,
        mod_: nmod_t,
    ) -> c_int;
}
extern "C" {
    pub fn n_bpoly_mod_gcd_brown_smprime(
        G: *mut n_bpoly_struct,
        Abar: *mut n_bpoly_struct,
        Bbar: *mut n_bpoly_struct,
        A: *mut n_bpoly_struct,
        B: *mut n_bpoly_struct,
        ctx: nmod_t,
        Sp: *mut n_poly_bpoly_stack_struct,
    ) -> c_int;
}
extern "C" {
    pub fn n_polyu1n_mod_gcd_brown_smprime(
        G: *mut n_polyun_struct,
        Abar: *mut n_polyun_struct,
        Bbar: *mut n_polyun_struct,
        A: *mut n_polyun_struct,
        B: *mut n_polyun_struct,
        ctx: nmod_t,
        St: *mut n_poly_polyun_stack_struct,
    ) -> c_int;
}
extern "C" {
    pub fn n_fq_bpoly_equal(
        A: *mut n_bpoly_struct,
        B: *mut n_bpoly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn n_fq_bpoly_get_coeff_n_fq(
        c: *mut mp_limb_t,
        A: *mut n_bpoly_struct,
        e0: mp_limb_signed_t,
        e1: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_bpoly_set_coeff_n_fq(
        A: *mut n_bpoly_struct,
        e0: mp_limb_signed_t,
        e1: mp_limb_signed_t,
        c: *const mp_limb_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_bpoly_get_coeff_fq_nmod(
        c: *mut nmod_poly_struct,
        A: *mut n_bpoly_struct,
        e0: mp_limb_signed_t,
        e1: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_bpoly_set_fq_nmod_poly_gen0(
        A: *mut n_bpoly_struct,
        B: *mut fq_nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_bpoly_set_n_fq_poly_gen0(
        A: *mut n_bpoly_struct,
        B: *mut n_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_bpoly_set_n_fq_poly_gen1(
        A: *mut n_bpoly_struct,
        B: *mut n_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_bpoly_derivative_gen0(
        A: *mut n_bpoly_struct,
        B: *mut n_bpoly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_bpoly_scalar_mul_n_fq(
        A: *mut n_bpoly_struct,
        c: *const mp_limb_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_bpoly_taylor_shift_gen1_fq_nmod(
        A: *mut n_bpoly_struct,
        B: *mut n_bpoly_struct,
        c_: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_bpoly_taylor_shift_gen0_fq_nmod(
        A: *mut n_bpoly_struct,
        alpha: *mut nmod_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_bpoly_taylor_shift_gen0_n_fq(
        A: *mut n_bpoly_struct,
        alpha: *const mp_limb_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_bpoly_gcd_brown_smprime(
        G: *mut n_bpoly_struct,
        Abar: *mut n_bpoly_struct,
        Bbar: *mut n_bpoly_struct,
        A: *mut n_bpoly_struct,
        B: *mut n_bpoly_struct,
        ctx: *mut fq_nmod_ctx_struct,
        Sp: *mut n_poly_bpoly_stack_struct,
    ) -> c_int;
}
extern "C" {
    pub fn n_fq_bpoly_print_pretty(
        A: *mut n_bpoly_struct,
        xvar: *const c_char,
        yvar: *const c_char,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_bpoly_one(A: *mut n_bpoly_struct, ctx: *mut fq_nmod_ctx_struct);
}
extern "C" {
    pub fn n_fq_bpoly_set(
        A: *mut n_bpoly_struct,
        B: *mut n_bpoly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_bpoly_is_canonical(A: *mut n_bpoly_struct, ctx: *mut fq_nmod_ctx_struct) -> c_int;
}
extern "C" {
    pub fn n_tpoly_init(A: *mut n_tpoly_struct);
}
extern "C" {
    pub fn n_tpoly_swap(A: *mut n_tpoly_struct, B: *mut n_tpoly_struct);
}
extern "C" {
    pub fn n_tpoly_fit_length(A: *mut n_tpoly_struct, len: mp_limb_signed_t);
}
extern "C" {
    pub fn n_tpoly_clear(A: *mut n_tpoly_struct);
}
extern "C" {
    pub fn n_polyu_init(A: *mut n_polyu_struct);
}
extern "C" {
    pub fn n_polyu_clear(A: *mut n_polyu_struct);
}
extern "C" {
    pub fn n_polyu_realloc(A: *mut n_polyu_struct, len: mp_limb_signed_t);
}
extern "C" {
    pub fn n_polyu_fit_length(A: *mut n_polyu_struct, len: mp_limb_signed_t);
}
extern "C" {
    pub fn n_polyu_swap(A: *mut n_polyu_struct, B: *mut n_polyu_struct);
}
extern "C" {
    pub fn n_polyu3_print_pretty(
        A: *mut n_polyu_struct,
        gen0: *const c_char,
        gen1: *const c_char,
        var2: *const c_char,
    );
}
extern "C" {
    pub fn n_polyu3_degrees(
        deg0: *mut mp_limb_signed_t,
        deg1: *mut mp_limb_signed_t,
        deg2: *mut mp_limb_signed_t,
        A: *mut n_polyu_struct,
    );
}
extern "C" {
    pub fn nmod_pow_cache_start(
        b: mp_limb_t,
        pos_direct: *mut n_poly_struct,
        pos_bin: *mut n_poly_struct,
        neg_direct: *mut n_poly_struct,
    );
}
extern "C" {
    pub fn nmod_pow_cache_mulpow_ui(
        a: mp_limb_t,
        e: mp_limb_t,
        pos_direct: *mut n_poly_struct,
        pos_bin: *mut n_poly_struct,
        neg_direct: *mut n_poly_struct,
        ctx: nmod_t,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn nmod_pow_cache_mulpow_neg_ui(
        a: mp_limb_t,
        e: mp_limb_t,
        pos_direct: *mut n_poly_struct,
        pos_bin: *mut n_poly_struct,
        neg_direct: *mut n_poly_struct,
        ctx: nmod_t,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn nmod_pow_cache_mulpow_fmpz(
        a: mp_limb_t,
        e: *mut fmpz,
        pos_direct: *mut n_poly_struct,
        pos_bin: *mut n_poly_struct,
        neg_direct: *mut n_poly_struct,
        ctx: nmod_t,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn n_fq_pow_cache_start_n_fq(
        b: *const mp_limb_t,
        pos_direct: *mut n_poly_struct,
        pos_bin: *mut n_poly_struct,
        neg_direct: *mut n_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_pow_cache_start_fq_nmod(
        b: *mut nmod_poly_struct,
        pos_direct: *mut n_poly_struct,
        pos_bin: *mut n_poly_struct,
        neg_direct: *mut n_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_pow_cache_mulpow_ui(
        r: *mut mp_limb_t,
        a: *const mp_limb_t,
        e: mp_limb_t,
        pos_direct: *mut n_poly_struct,
        pos_bin: *mut n_poly_struct,
        neg_direct: *mut n_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_pow_cache_mulpow_neg_ui(
        r: *mut mp_limb_t,
        a: *const mp_limb_t,
        e: mp_limb_t,
        pos_direct: *mut n_poly_struct,
        pos_bin: *mut n_poly_struct,
        neg_direct: *mut n_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_pow_cache_mulpow_fmpz(
        r: *mut mp_limb_t,
        a: *const mp_limb_t,
        e: *mut fmpz,
        pos_direct: *mut n_poly_struct,
        pos_bin: *mut n_poly_struct,
        neg_direct: *mut n_poly_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nmod_eval_interp_struct {
    pub M: *mut mp_limb_t,
    pub T: *mut mp_limb_t,
    pub Q: *mut mp_limb_t,
    pub array: *mut mp_limb_t,
    pub alloc: mp_limb_signed_t,
    pub d: mp_limb_signed_t,
    pub radix: mp_limb_signed_t,
    pub w: mp_limb_t,
}
pub type nmod_eval_interp_t = [nmod_eval_interp_struct; 1usize];
extern "C" {
    pub fn nmod_eval_interp_init(E: *mut nmod_eval_interp_struct);
}
extern "C" {
    pub fn nmod_eval_interp_clear(E: *mut nmod_eval_interp_struct);
}
extern "C" {
    pub fn nmod_eval_interp_set_degree_modulus(
        E: *mut nmod_eval_interp_struct,
        deg: mp_limb_signed_t,
        ctx: nmod_t,
    ) -> c_int;
}
extern "C" {
    pub fn nmod_eval_interp_eval_length(E: *mut nmod_eval_interp_struct) -> mp_limb_signed_t;
}
extern "C" {
    pub fn nmod_eval_interp_to_coeffs_poly(
        a: *mut n_poly_struct,
        v: *mut n_poly_struct,
        E: *mut nmod_eval_interp_struct,
        ctx: nmod_t,
    );
}
extern "C" {
    pub fn nmod_eval_interp_from_coeffs_poly(
        v: *mut n_poly_struct,
        a: *mut n_poly_struct,
        E: *mut nmod_eval_interp_struct,
        ctx: nmod_t,
    );
}
extern "C" {
    pub fn nmod_eval_interp_to_coeffs_n_fq_poly(
        a: *mut n_poly_struct,
        v: *mut n_poly_struct,
        E: *mut nmod_eval_interp_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_eval_interp_from_coeffs_n_fq_poly(
        v: *mut n_poly_struct,
        a: *mut n_poly_struct,
        E: *mut nmod_eval_interp_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn nmod_evals_zero(a: *mut n_poly_struct);
}
extern "C" {
    pub fn nmod_evals_add_inplace(
        a: *mut n_poly_struct,
        b: *mut n_poly_struct,
        len: mp_limb_signed_t,
        ctx: nmod_t,
    );
}
extern "C" {
    pub fn nmod_evals_mul(
        a: *mut n_poly_struct,
        b: *mut n_poly_struct,
        c: *mut n_poly_struct,
        len: mp_limb_signed_t,
        ctx: nmod_t,
    );
}
extern "C" {
    pub fn nmod_evals_addmul(
        a: *mut n_poly_struct,
        b: *mut n_poly_struct,
        c: *mut n_poly_struct,
        len: mp_limb_signed_t,
        ctx: nmod_t,
    );
}
extern "C" {
    pub fn nmod_evals_fmma(
        a: *mut n_poly_struct,
        b: *mut n_poly_struct,
        c: *mut n_poly_struct,
        d: *mut n_poly_struct,
        e: *mut n_poly_struct,
        len: mp_limb_signed_t,
        ctx: nmod_t,
    );
}
extern "C" {
    pub fn n_fq_evals_zero(a: *mut n_poly_struct);
}
extern "C" {
    pub fn n_fq_evals_add_inplace(
        a: *mut n_poly_struct,
        b: *mut n_poly_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_evals_mul(
        a: *mut n_poly_struct,
        b: *mut n_poly_struct,
        c: *mut n_poly_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_evals_addmul(
        a: *mut n_poly_struct,
        b: *mut n_poly_struct,
        c: *mut n_poly_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_fq_evals_fmma(
        a: *mut n_poly_struct,
        b: *mut n_poly_struct,
        c: *mut n_poly_struct,
        f: *mut n_poly_struct,
        e: *mut n_poly_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_polyun_init(A: *mut n_polyun_struct);
}
extern "C" {
    pub fn n_polyun_is_canonical(A: *mut n_polyun_struct) -> c_int;
}
extern "C" {
    pub fn n_polyun_clear(A: *mut n_polyun_struct);
}
extern "C" {
    pub fn n_polyun_realloc(A: *mut n_polyun_struct, len: mp_limb_signed_t);
}
extern "C" {
    pub fn n_polyun_fit_length(A: *mut n_polyun_struct, len: mp_limb_signed_t);
}
extern "C" {
    pub fn n_polyun_mod_is_canonical(A: *mut n_polyun_struct, mod_: nmod_t) -> c_int;
}
extern "C" {
    pub fn n_polyun_swap(A: *mut n_polyun_struct, B: *mut n_polyun_struct);
}
extern "C" {
    pub fn n_polyun_set(A: *mut n_polyun_struct, B: *mut n_polyun_struct);
}
extern "C" {
    pub fn n_polyu1n_print_pretty(
        A: *mut n_polyun_struct,
        var0: *const c_char,
        varlast: *const c_char,
    );
}
extern "C" {
    pub fn n_polyu2n_print_pretty(
        A: *mut n_polyun_struct,
        gen0: *const c_char,
        gen1: *const c_char,
        varlast: *const c_char,
    );
}
extern "C" {
    pub fn n_polyu3n_print_pretty(
        A: *mut n_polyun_struct,
        gen0: *const c_char,
        gen1: *const c_char,
        var2: *const c_char,
        varlast: *const c_char,
    );
}
extern "C" {
    pub fn n_fq_polyun_set(
        A: *mut n_polyun_struct,
        B: *mut n_polyun_struct,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn n_polyun_equal(A: *mut n_polyun_struct, B: *mut n_polyun_struct) -> c_int;
}
extern "C" {
    pub fn n_polyun_one(A: *mut n_polyun_struct);
}
extern "C" {
    pub fn n_polyu1n_bidegree(A: *mut n_polyun_struct) -> mp_limb_t;
}
extern "C" {
    pub fn n_fq_poly_product_roots_n_fq(
        M: *mut n_poly_struct,
        H: *const mp_limb_t,
        length: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
        St: *mut n_poly_stack_struct,
    );
}
extern "C" {
    pub fn n_polyun_product_roots(
        M: *mut n_polyun_struct,
        H: *mut n_polyun_struct,
        ctx: nmod_t,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn n_fq_polyun_product_roots(
        M: *mut n_polyun_struct,
        H: *mut n_polyun_struct,
        ctx: *mut fq_nmod_ctx_struct,
        St: *mut n_poly_stack_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn _nmod_zip_eval_step(
        cur: *mut mp_limb_t,
        inc: *const mp_limb_t,
        coeffs: *const mp_limb_t,
        length: mp_limb_signed_t,
        ctx: nmod_t,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn _n_fq_zip_eval_step(
        res: *mut mp_limb_t,
        cur: *mut mp_limb_t,
        inc: *const mp_limb_t,
        coeffs: *const mp_limb_t,
        length: mp_limb_signed_t,
        ctx: *mut fq_nmod_ctx_struct,
    );
}
extern "C" {
    pub fn _n_fqp_zip_eval_step(
        res: *mut mp_limb_t,
        cur: *mut mp_limb_t,
        inc: *const mp_limb_t,
        coeffs: *const mp_limb_t,
        length: mp_limb_signed_t,
        d: mp_limb_signed_t,
        mod_: nmod_t,
    );
}
extern "C" {
    pub fn _nmod_zip_vand_solve(
        coeffs: *mut mp_limb_t,
        monomials: *const mp_limb_t,
        mlength: mp_limb_signed_t,
        evals: *const mp_limb_t,
        elength: mp_limb_signed_t,
        master: *const mp_limb_t,
        scratch: *mut mp_limb_t,
        ctx: nmod_t,
    ) -> c_int;
}
extern "C" {
    pub fn _n_fq_zip_vand_solve(
        coeffs: *mut mp_limb_t,
        monomials: *const mp_limb_t,
        mlength: mp_limb_signed_t,
        evals: *const mp_limb_t,
        elength: mp_limb_signed_t,
        master: *const mp_limb_t,
        scratch: *mut mp_limb_t,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn _n_fqp_zip_vand_solve(
        coeffs: *mut mp_limb_t,
        monomials: *const mp_limb_t,
        mlength: mp_limb_signed_t,
        evals: *const mp_limb_t,
        elength: mp_limb_signed_t,
        master: *const mp_limb_t,
        scratch: *mut mp_limb_t,
        ctx: *mut fq_nmod_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn n_poly_stack_init(S: *mut n_poly_stack_struct);
}
extern "C" {
    pub fn n_poly_stack_clear(S: *mut n_poly_stack_struct);
}
extern "C" {
    pub fn n_poly_stack_fit_request(
        S: *mut n_poly_stack_struct,
        k: mp_limb_signed_t,
    ) -> *mut *mut n_poly_struct;
}
extern "C" {
    pub fn n_poly_stack_vec_init(
        S: *mut n_poly_stack_struct,
        len: mp_limb_signed_t,
    ) -> *mut mp_limb_t;
}
extern "C" {
    pub fn n_poly_stack_vec_clear(S: *mut n_poly_stack_struct);
}
extern "C" {
    pub fn n_poly_stack_request(
        S: *mut n_poly_stack_struct,
        k: mp_limb_signed_t,
    ) -> *mut *mut n_poly_struct;
}
extern "C" {
    pub fn n_poly_stack_take_top(S: *mut n_poly_stack_struct) -> *mut n_poly_struct;
}
extern "C" {
    pub fn n_poly_stack_give_back(S: *mut n_poly_stack_struct, k: mp_limb_signed_t);
}
extern "C" {
    pub fn n_poly_stack_size(S: *mut n_poly_stack_struct) -> mp_limb_signed_t;
}
extern "C" {
    pub fn n_bpoly_stack_init(S: *mut n_bpoly_stack_struct);
}
extern "C" {
    pub fn n_bpoly_stack_clear(S: *mut n_bpoly_stack_struct);
}
extern "C" {
    pub fn n_bpoly_stack_fit_request(
        S: *mut n_bpoly_stack_struct,
        k: mp_limb_signed_t,
    ) -> *mut *mut n_bpoly_struct;
}
extern "C" {
    pub fn n_bpoly_stack_request(
        S: *mut n_bpoly_stack_struct,
        k: mp_limb_signed_t,
    ) -> *mut *mut n_bpoly_struct;
}
extern "C" {
    pub fn n_bpoly_stack_take_top(S: *mut n_bpoly_stack_struct) -> *mut n_bpoly_struct;
}
extern "C" {
    pub fn n_bpoly_stack_give_back(S: *mut n_bpoly_stack_struct, k: mp_limb_signed_t);
}
extern "C" {
    pub fn n_bpoly_stack_size(S: *mut n_bpoly_stack_struct) -> mp_limb_signed_t;
}
extern "C" {
    pub fn n_polyun_stack_init(S: *mut n_polyun_stack_struct);
}
extern "C" {
    pub fn n_polyun_stack_clear(S: *mut n_polyun_stack_struct);
}
extern "C" {
    pub fn n_polyun_stack_fit_request(
        S: *mut n_polyun_stack_struct,
        k: mp_limb_signed_t,
    ) -> *mut *mut n_polyun_struct;
}
extern "C" {
    pub fn n_polyun_stack_request(
        S: *mut n_polyun_stack_struct,
        k: mp_limb_signed_t,
    ) -> *mut *mut n_polyun_struct;
}
extern "C" {
    pub fn n_polyun_stack_take_top(S: *mut n_polyun_stack_struct) -> *mut n_polyun_struct;
}
extern "C" {
    pub fn n_polyun_stack_give_back(S: *mut n_polyun_stack_struct, k: mp_limb_signed_t);
}
extern "C" {
    pub fn n_polyun_stack_size(S: *mut n_polyun_stack_struct) -> mp_limb_signed_t;
}
