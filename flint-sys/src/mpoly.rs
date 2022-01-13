#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

//! *See the [FLINT documentation](http://flintlib.org/doc/mpoly.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpz::{fmpz, fmpz_t};
use crate::fmpz_mat::fmpz_mat_struct;
use libc::{c_char, c_int, c_uchar, c_uint, c_void};

pub type ordering_t = c_uint;
pub const ordering_t_ORD_LEX: ordering_t = 0;
pub const ordering_t_ORD_DEGLEX: ordering_t = 1;
pub const ordering_t_ORD_DEGREVLEX: ordering_t = 2;

extern "C" {
    pub fn mpoly_divide_threads(n: mp_limb_signed_t, la: f64, lb: f64) -> mp_limb_signed_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mpoly_ctx_struct {
    pub nvars: mp_limb_signed_t,
    pub nfields: mp_limb_signed_t,
    pub ord: ordering_t,
    pub deg: c_int,
    pub rev: c_int,
    pub lut_words_per_exp: [mp_limb_signed_t; 64usize],
    pub lut_fix_bits: [c_uchar; 64usize],
}
pub type mpoly_ctx_t = [mpoly_ctx_struct; 1usize];
extern "C" {
    pub fn mpoly_ctx_init(ctx: *mut mpoly_ctx_struct, nvars: mp_limb_signed_t, ord: ordering_t);
}
extern "C" {
    pub fn mpoly_ctx_init_rand(
        mctx: *mut mpoly_ctx_struct,
        state: *mut flint_rand_s,
        max_nvars: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn mpoly_monomial_randbits_fmpz(
        exp: *mut fmpz,
        state: *mut flint_rand_s,
        exp_bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_ctx_clear(mctx: *mut mpoly_ctx_struct);
}
extern "C" {
    pub fn mpoly_words_per_exp_sp(bits: mp_limb_t, mctx: *mut mpoly_ctx_struct)
        -> mp_limb_signed_t;
}
extern "C" {
    pub fn mpoly_words_per_exp_mp(bits: mp_limb_t, mctx: *mut mpoly_ctx_struct)
        -> mp_limb_signed_t;
}
extern "C" {
    pub fn mpoly_words_per_exp(bits: mp_limb_t, mctx: *mut mpoly_ctx_struct) -> mp_limb_signed_t;
}
extern "C" {
    pub fn mpoly_fix_bits(bits: mp_limb_t, mctx: *mut mpoly_ctx_struct) -> mp_limb_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mpoly_heap_t {
    pub i: mp_limb_t,
    pub j: mp_limb_t,
    pub next: *mut mpoly_heap_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mpoly_nheap_t {
    pub i: mp_limb_t,
    pub j: mp_limb_t,
    pub next: *mut mpoly_nheap_t,
    pub p: mp_limb_signed_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mpoly_heap1_s {
    pub exp: mp_limb_t,
    pub next: *mut c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mpoly_heap_s {
    pub exp: *mut mp_limb_t,
    pub next: *mut c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mpoly_rbnode_ui_struct {
    pub key: mp_limb_t,
    pub up: mp_limb_signed_t,
    pub left: mp_limb_signed_t,
    pub right: mp_limb_signed_t,
    pub color: c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mpoly_rbtree_ui_struct {
    pub length: mp_limb_signed_t,
    pub nodes: *mut mpoly_rbnode_ui_struct,
    pub node_alloc: mp_limb_signed_t,
    pub data: *mut c_char,
    pub data_alloc: mp_limb_signed_t,
    pub data_size: mp_limb_signed_t,
}
pub type mpoly_rbtree_ui_t = [mpoly_rbtree_ui_struct; 1usize];
extern "C" {
    pub fn mpoly_rbtree_ui_init(T: *mut mpoly_rbtree_ui_struct, data_size: mp_limb_signed_t);
}
extern "C" {
    pub fn mpoly_rbtree_ui_clear(T: *mut mpoly_rbtree_ui_struct);
}
extern "C" {
    pub fn mpoly_rbtree_ui_lookup(
        T: *mut mpoly_rbtree_ui_struct,
        its_new: *mut c_int,
        key: mp_limb_t,
    ) -> *mut c_void;
}
extern "C" {
    pub fn mpoly_rbtree_ui_head(T: *mut mpoly_rbtree_ui_struct) -> mp_limb_signed_t;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mpoly_rbnode_fmpz_struct {
    pub key: fmpz_t,
    pub up: mp_limb_signed_t,
    pub left: mp_limb_signed_t,
    pub right: mp_limb_signed_t,
    pub color: c_int,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mpoly_rbtree_fmpz_struct {
    pub length: mp_limb_signed_t,
    pub nodes: *mut mpoly_rbnode_fmpz_struct,
    pub node_alloc: mp_limb_signed_t,
    pub data: *mut c_char,
    pub data_alloc: mp_limb_signed_t,
    pub data_size: mp_limb_signed_t,
}
pub type mpoly_rbtree_fmpz_t = [mpoly_rbtree_fmpz_struct; 1usize];
extern "C" {
    pub fn mpoly_rbtree_fmpz_init(T: *mut mpoly_rbtree_fmpz_struct, data_size: mp_limb_signed_t);
}
extern "C" {
    pub fn mpoly_rbtree_fmpz_clear(T: *mut mpoly_rbtree_fmpz_struct);
}
extern "C" {
    pub fn mpoly_rbtree_fmpz_lookup(
        T: *mut mpoly_rbtree_fmpz_struct,
        its_new: *mut c_int,
        key: *mut fmpz,
    ) -> *mut c_void;
}
extern "C" {
    pub fn mpoly_rbtree_fmpz_head(T: *mut mpoly_rbtree_fmpz_struct) -> mp_limb_signed_t;
}
extern "C" {
    pub fn mpoly_ordering_randtest(state: *mut flint_rand_s) -> ordering_t;
}
extern "C" {
    pub fn mpoly_ordering_isdeg(mctx: *mut mpoly_ctx_struct) -> c_int;
}
extern "C" {
    pub fn mpoly_ordering_isrev(mctx: *mut mpoly_ctx_struct) -> c_int;
}
extern "C" {
    pub fn mpoly_ordering_print(ord: ordering_t);
}
extern "C" {
    pub fn mpoly_monomial_zero(exp_ptr: *mut mp_limb_t, N: mp_limb_signed_t);
}
extern "C" {
    pub fn mpoly_monomial_add(
        exp_ptr: *mut mp_limb_t,
        exp2: *const mp_limb_t,
        exp3: *const mp_limb_t,
        N: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn mpoly_monomial_add_mp(
        exp_ptr: *mut mp_limb_t,
        exp2: *const mp_limb_t,
        exp3: *const mp_limb_t,
        N: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn mpoly_monomial_sub(
        exp_ptr: *mut mp_limb_t,
        exp2: *const mp_limb_t,
        exp3: *const mp_limb_t,
        N: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn mpoly_monomial_sub_mp(
        exp_ptr: *mut mp_limb_t,
        exp2: *const mp_limb_t,
        exp3: *const mp_limb_t,
        N: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn mpoly_monomial_madd(
        exp1: *mut mp_limb_t,
        exp2: *const mp_limb_t,
        scalar: mp_limb_t,
        exp3: *const mp_limb_t,
        N: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn mpoly_monomial_madd_mp(
        exp1: *mut mp_limb_t,
        exp2: *const mp_limb_t,
        scalar: mp_limb_t,
        exp3: *const mp_limb_t,
        N: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn mpoly_monomial_madd_inplace_mp(
        exp12: *mut mp_limb_t,
        scalar: mp_limb_t,
        exp3: *const mp_limb_t,
        N: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn mpoly_monomial_msub(
        exp1: *mut mp_limb_t,
        exp2: *const mp_limb_t,
        scalar: mp_limb_t,
        exp3: *const mp_limb_t,
        N: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn mpoly_monomial_msub_mp(
        exp1: *mut mp_limb_t,
        exp2: *const mp_limb_t,
        scalar: mp_limb_t,
        exp3: *const mp_limb_t,
        N: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn mpoly_monomial_msub_ui_array(
        exp1: *mut mp_limb_t,
        exp2: *const mp_limb_t,
        scalar: *const mp_limb_t,
        scalar_limbs: mp_limb_signed_t,
        exp3: *const mp_limb_t,
        N: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn mpoly_monomial_madd_ui_array(
        exp1: *mut mp_limb_t,
        exp2: *const mp_limb_t,
        scalar: *const mp_limb_t,
        scalar_limbs: mp_limb_signed_t,
        exp3: *const mp_limb_t,
        N: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn mpoly_monomial_madd_fmpz(
        exp1: *mut mp_limb_t,
        exp2: *const mp_limb_t,
        scalar: *mut fmpz,
        exp3: *const mp_limb_t,
        N: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn mpoly_overflow_mask_sp(bits: mp_limb_t) -> mp_limb_t;
}
extern "C" {
    pub fn mpoly_monomial_max1(
        exp2: mp_limb_t,
        exp3: mp_limb_t,
        bits: mp_limb_t,
        mask: mp_limb_t,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn mpoly_monomial_max(
        exp1: *mut mp_limb_t,
        exp2: *const mp_limb_t,
        exp3: *const mp_limb_t,
        bits: mp_limb_t,
        N: mp_limb_signed_t,
        mask: mp_limb_t,
    );
}
extern "C" {
    pub fn mpoly_monomial_min1(
        exp2: mp_limb_t,
        exp3: mp_limb_t,
        bits: mp_limb_t,
        mask: mp_limb_t,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn mpoly_monomial_min(
        exp1: *mut mp_limb_t,
        exp2: *const mp_limb_t,
        exp3: *const mp_limb_t,
        bits: mp_limb_t,
        N: mp_limb_signed_t,
        mask: mp_limb_t,
    );
}
extern "C" {
    pub fn mpoly_monomial_max_mp(
        exp1: *mut mp_limb_t,
        exp2: *const mp_limb_t,
        exp3: *const mp_limb_t,
        bits: mp_limb_t,
        N: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn mpoly_monomial_min_mp(
        exp1: *mut mp_limb_t,
        exp2: *const mp_limb_t,
        exp3: *const mp_limb_t,
        bits: mp_limb_t,
        N: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn mpoly_monomial_overflows(
        exp2: *mut mp_limb_t,
        N: mp_limb_signed_t,
        mask: mp_limb_t,
    ) -> c_int;
}
extern "C" {
    pub fn mpoly_monomial_overflows_mp(
        exp_ptr: *mut mp_limb_t,
        N: mp_limb_signed_t,
        bits: mp_limb_t,
    ) -> c_int;
}
extern "C" {
    pub fn mpoly_monomial_overflows1(exp: mp_limb_t, mask: mp_limb_t) -> c_int;
}
extern "C" {
    pub fn mpoly_monomial_divides(
        exp_ptr: *mut mp_limb_t,
        exp2: *const mp_limb_t,
        exp3: *const mp_limb_t,
        N: mp_limb_signed_t,
        mask: mp_limb_t,
    ) -> c_int;
}
extern "C" {
    pub fn mpoly_monomial_halves(
        exp_ptr: *mut mp_limb_t,
        exp2: *const mp_limb_t,
        N: mp_limb_signed_t,
        mask: mp_limb_t,
    ) -> c_int;
}
extern "C" {
    pub fn mpoly_monomial_divides_mp(
        exp_ptr: *mut mp_limb_t,
        exp2: *const mp_limb_t,
        exp3: *const mp_limb_t,
        N: mp_limb_signed_t,
        bits: mp_limb_t,
    ) -> c_int;
}
extern "C" {
    pub fn mpoly_monomial_halves_mp(
        exp_ptr: *mut mp_limb_t,
        exp2: *const mp_limb_t,
        N: mp_limb_signed_t,
        bits: mp_limb_t,
    ) -> c_int;
}
extern "C" {
    pub fn mpoly_monomial_divides_test(
        exp2: *const mp_limb_t,
        exp3: *const mp_limb_t,
        N: mp_limb_signed_t,
        mask: mp_limb_t,
    ) -> c_int;
}
extern "C" {
    pub fn mpoly_monomial_divides_mp_test(
        exp2: *const mp_limb_t,
        exp3: *const mp_limb_t,
        N: mp_limb_signed_t,
        bits: mp_limb_t,
    ) -> c_int;
}
extern "C" {
    pub fn mpoly_monomial_divides1(
        exp_ptr: *mut mp_limb_t,
        exp2: mp_limb_t,
        exp3: mp_limb_t,
        mask: mp_limb_t,
    ) -> c_int;
}
extern "C" {
    pub fn mpoly_monomial_halves1(
        exp_ptr: *mut mp_limb_t,
        exp2: mp_limb_t,
        mask: mp_limb_t,
    ) -> c_int;
}
extern "C" {
    pub fn mpoly_monomial_set(exp2: *mut mp_limb_t, exp3: *const mp_limb_t, N: mp_limb_signed_t);
}
extern "C" {
    pub fn mpoly_monomial_set_extra(
        exp2: *mut mp_limb_t,
        exp3: *const mp_limb_t,
        N: mp_limb_signed_t,
        offset: mp_limb_signed_t,
        extra: mp_limb_t,
    );
}
extern "C" {
    pub fn mpoly_copy_monomials(
        exp1: *mut mp_limb_t,
        exp2: *const mp_limb_t,
        len: mp_limb_signed_t,
        N: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn mpoly_monomial_swap(exp2: *mut mp_limb_t, exp3: *mut mp_limb_t, N: mp_limb_signed_t);
}
extern "C" {
    pub fn mpoly_monomial_mul_ui(
        exp2: *mut mp_limb_t,
        exp3: *const mp_limb_t,
        N: mp_limb_signed_t,
        c: mp_limb_t,
    );
}
extern "C" {
    pub fn mpoly_monomial_mul_ui_mp(
        exp2: *mut mp_limb_t,
        exp3: *const mp_limb_t,
        N: mp_limb_signed_t,
        c: mp_limb_t,
    );
}
extern "C" {
    pub fn mpoly_monomial_mul_fmpz(
        exp2: *mut mp_limb_t,
        exp3: *const mp_limb_t,
        N: mp_limb_signed_t,
        c: *mut fmpz,
    );
}
extern "C" {
    pub fn mpoly_monomial_is_zero(exp: *const mp_limb_t, N: mp_limb_signed_t) -> c_int;
}
extern "C" {
    pub fn mpoly_monomial_equal(
        exp2: *const mp_limb_t,
        exp3: *const mp_limb_t,
        N: mp_limb_signed_t,
    ) -> c_int;
}
extern "C" {
    pub fn mpoly_monomial_equal_extra(
        exp2: *const mp_limb_t,
        exp3: *const mp_limb_t,
        N: mp_limb_signed_t,
        offset: mp_limb_signed_t,
        extra: mp_limb_t,
    ) -> c_int;
}
extern "C" {
    pub fn mpoly_monomial_cmp1(a: mp_limb_t, b: mp_limb_t, cmpmask: mp_limb_t) -> c_int;
}
extern "C" {
    pub fn mpoly_monomial_gt1(a: mp_limb_t, b: mp_limb_t, cmpmask: mp_limb_t) -> c_int;
}
extern "C" {
    pub fn mpoly_monomial_ge1(a: mp_limb_t, b: mp_limb_t, cmpmask: mp_limb_t) -> c_int;
}
extern "C" {
    pub fn mpoly_monomial_lt(
        exp3: *const mp_limb_t,
        exp2: *const mp_limb_t,
        N: mp_limb_signed_t,
        cmpmask: *const mp_limb_t,
    ) -> c_int;
}
extern "C" {
    pub fn mpoly_monomial_gt(
        exp3: *const mp_limb_t,
        exp2: *const mp_limb_t,
        N: mp_limb_signed_t,
        cmpmask: *const mp_limb_t,
    ) -> c_int;
}
extern "C" {
    pub fn mpoly_monomial_lt_nomask(
        exp2: *const mp_limb_t,
        exp3: *const mp_limb_t,
        N: mp_limb_signed_t,
    ) -> c_int;
}
extern "C" {
    pub fn mpoly_monomial_gt_nomask(
        exp2: *const mp_limb_t,
        exp3: *const mp_limb_t,
        N: mp_limb_signed_t,
    ) -> c_int;
}
extern "C" {
    pub fn mpoly_monomial_lt_nomask_extra(
        exp2: *const mp_limb_t,
        exp3: *const mp_limb_t,
        N: mp_limb_signed_t,
        offset: mp_limb_signed_t,
        extra: mp_limb_t,
    ) -> c_int;
}
extern "C" {
    pub fn mpoly_monomial_gt_nomask_extra(
        exp2: *const mp_limb_t,
        exp3: *const mp_limb_t,
        N: mp_limb_signed_t,
        offset: mp_limb_signed_t,
        extra: mp_limb_t,
    ) -> c_int;
}
extern "C" {
    pub fn mpoly_monomial_cmp(
        exp2: *const mp_limb_t,
        exp3: *const mp_limb_t,
        N: mp_limb_signed_t,
        cmpmask: *const mp_limb_t,
    ) -> c_int;
}
extern "C" {
    pub fn mpoly_monomial_cmp_nomask(
        exp2: *const mp_limb_t,
        exp3: *const mp_limb_t,
        N: mp_limb_signed_t,
    ) -> c_int;
}
extern "C" {
    pub fn mpoly_monomial_cmp_nomask_extra(
        exp2: *const mp_limb_t,
        exp3: *const mp_limb_t,
        N: mp_limb_signed_t,
        offset: mp_limb_signed_t,
        extra: mp_limb_t,
    ) -> c_int;
}
extern "C" {
    pub fn mpoly_monomial_divides_tight(
        e1: mp_limb_signed_t,
        e2: mp_limb_signed_t,
        prods: *mut mp_limb_signed_t,
        num: mp_limb_signed_t,
    ) -> c_int;
}
extern "C" {
    pub fn mpoly_max_degrees_tight(
        max_exp: *mut mp_limb_signed_t,
        exps: *mut mp_limb_t,
        len: mp_limb_signed_t,
        prods: *mut mp_limb_signed_t,
        num: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn mpoly_geobucket_clog4(x: mp_limb_signed_t) -> mp_limb_signed_t;
}
extern "C" {
    pub fn pack_exp2(e0: mp_limb_t, e1: mp_limb_t) -> mp_limb_t;
}
extern "C" {
    pub fn pack_exp3(e0: mp_limb_t, e1: mp_limb_t, e2: mp_limb_t) -> mp_limb_t;
}
extern "C" {
    pub fn extract_exp(e: mp_limb_t, idx: c_int, nvars: c_int) -> mp_limb_t;
}
extern "C" {
    pub fn _mpoly_bidegree(
        Aexps: *const mp_limb_t,
        Abits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn mpoly_gen_fields_ui(
        exp: *mut mp_limb_t,
        var: mp_limb_signed_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_gen_fields_fmpz(
        exp: *mut fmpz,
        var: mp_limb_signed_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_gen_bits_required(var: mp_limb_signed_t, mctx: *mut mpoly_ctx_struct)
        -> mp_limb_t;
}
extern "C" {
    pub fn mpoly_gen_index(v: mp_limb_signed_t, mctx: *mut mpoly_ctx_struct) -> mp_limb_signed_t;
}
extern "C" {
    pub fn mpoly_gen_offset_shift_sp(
        offset: *mut mp_limb_signed_t,
        shift: *mut mp_limb_signed_t,
        var: mp_limb_signed_t,
        bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_gen_monomial_offset_shift_sp(
        mexp: *mut mp_limb_t,
        offset: *mut mp_limb_signed_t,
        shift: *mut mp_limb_signed_t,
        var: mp_limb_signed_t,
        bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_gen_monomial_sp(
        oneexp: *mut mp_limb_t,
        var: mp_limb_signed_t,
        bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_gen_offset_mp(
        var: mp_limb_signed_t,
        bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn mpoly_gen_monomial_offset_mp(
        mexp: *mut mp_limb_t,
        var: mp_limb_signed_t,
        bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn fmpz_mat_mul_vec(v: *mut fmpz, M: *mut fmpz_mat_struct, u: *mut fmpz);
}
extern "C" {
    pub fn mpoly_compose_mat_gen(
        M: *mut fmpz_mat_struct,
        c: *const mp_limb_signed_t,
        mctxB: *mut mpoly_ctx_struct,
        mctxAC: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_compose_mat_fill_column(
        M: *mut fmpz_mat_struct,
        Cexp: *const mp_limb_t,
        Cbits: mp_limb_t,
        Bvar: mp_limb_signed_t,
        mctxB: *mut mpoly_ctx_struct,
        mctxAC: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_get_cmpmask(
        cmpmask: *mut mp_limb_t,
        N: mp_limb_signed_t,
        bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_get_ovfmask(
        ovfmask: *mut mp_limb_t,
        N: mp_limb_signed_t,
        bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_monomials_cmp(
        Aexps: *const mp_limb_t,
        Abits: mp_limb_t,
        Bexps: *const mp_limb_t,
        Bbits: mp_limb_t,
        length: mp_limb_signed_t,
        mctx: *mut mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn mpoly_exp_bits_required_ui(
        user_exp: *const mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn mpoly_exp_bits_required_ffmpz(
        user_exp: *const fmpz,
        mctx: *mut mpoly_ctx_struct,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn mpoly_exp_bits_required_pfmpz(
        user_exp: *const *mut fmpz,
        mctx: *mut mpoly_ctx_struct,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn mpoly_gen_pow_exp_bits_required(
        v: mp_limb_signed_t,
        e: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn mpoly_is_poly(
        Aexps: *const mp_limb_t,
        Alen: mp_limb_signed_t,
        Abits: mp_limb_t,
        var: mp_limb_signed_t,
        mctx: *mut mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn mpoly_pack_vec_ui(
        exp1: *mut mp_limb_t,
        exp2: *const mp_limb_t,
        bits: mp_limb_t,
        nfields: mp_limb_signed_t,
        len: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn mpoly_pack_vec_fmpz(
        exp1: *mut mp_limb_t,
        exp2: *const fmpz,
        bits: mp_limb_t,
        nfields: mp_limb_signed_t,
        len: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn mpoly_unpack_vec_ui(
        exp1: *mut mp_limb_t,
        exp2: *const mp_limb_t,
        bits: mp_limb_t,
        nfields: mp_limb_signed_t,
        len: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn mpoly_unpack_vec_fmpz(
        exp1: *mut fmpz,
        exp2: *const mp_limb_t,
        bits: mp_limb_t,
        nfields: mp_limb_signed_t,
        len: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn mpoly_get_monomial_ui_unpacked_ffmpz(
        user_exps: *mut mp_limb_t,
        poly_exps: *const fmpz,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_get_monomial_ffmpz_unpacked_ffmpz(
        user_exps: *mut fmpz,
        poly_exps: *const fmpz,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_get_monomial_pfmpz_unpacked_ffmpz(
        user_exps: *mut *mut fmpz,
        poly_exps: *const fmpz,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_get_monomial_ui_unpacked_ui(
        user_exps: *mut mp_limb_t,
        poly_exps: *const mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_get_monomial_ui_sp(
        user_exps: *mut mp_limb_t,
        poly_exps: *const mp_limb_t,
        bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_get_monomial_ui_mp(
        user_exps: *mut mp_limb_t,
        poly_exps: *const mp_limb_t,
        bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_get_monomial_si_mp(
        user_exps: *mut mp_limb_signed_t,
        poly_exps: *const mp_limb_t,
        bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_get_monomial_ui(
        user_exps: *mut mp_limb_t,
        poly_exps: *const mp_limb_t,
        bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_get_monomial_si(
        user_exps: *mut mp_limb_signed_t,
        poly_exps: *const mp_limb_t,
        bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_get_monomial_var_exp_ui_sp(
        poly_exps: *const mp_limb_t,
        var: mp_limb_signed_t,
        bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn mpoly_get_monomial_var_exp_ui_mp(
        poly_exps: *const mp_limb_t,
        var: mp_limb_signed_t,
        bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn mpoly_get_monomial_var_exp_si_mp(
        poly_exps: *const mp_limb_t,
        var: mp_limb_signed_t,
        bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn mpoly_get_monomial_var_exp_ui(
        poly_exps: *const mp_limb_t,
        var: mp_limb_signed_t,
        bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    ) -> mp_limb_t;
}
extern "C" {
    pub fn mpoly_get_monomial_var_exp_si(
        poly_exps: *const mp_limb_t,
        var: mp_limb_signed_t,
        bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn mpoly_get_monomial_ffmpz(
        exps: *mut fmpz,
        poly_exps: *const mp_limb_t,
        bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_get_monomial_pfmpz(
        exps: *mut *mut fmpz,
        poly_exps: *const mp_limb_t,
        bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_set_monomial_ui(
        exp1: *mut mp_limb_t,
        exp2: *const mp_limb_t,
        bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_set_monomial_ffmpz(
        exp1: *mut mp_limb_t,
        exp2: *const fmpz,
        bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_set_monomial_pfmpz(
        exp1: *mut mp_limb_t,
        exp2: *const *mut fmpz,
        bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_repack_monomials(
        exps1: *mut mp_limb_t,
        bits1: mp_limb_t,
        exps2: *const mp_limb_t,
        bits2: mp_limb_t,
        len: mp_limb_signed_t,
        mctx: *mut mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn mpoly_pack_monomials_tight(
        exp1: *mut mp_limb_t,
        exp2: *const mp_limb_t,
        len: mp_limb_signed_t,
        mults: *const mp_limb_signed_t,
        num: mp_limb_signed_t,
        bits: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn mpoly_unpack_monomials_tight(
        e1: *mut mp_limb_t,
        e2: *mut mp_limb_t,
        len: mp_limb_signed_t,
        mults: *mut mp_limb_signed_t,
        num: mp_limb_signed_t,
        bits: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn mpoly_monomial_exists(
        index: *mut mp_limb_signed_t,
        poly_exps: *const mp_limb_t,
        exp: *const mp_limb_t,
        len: mp_limb_signed_t,
        N: mp_limb_signed_t,
        cmpmask: *const mp_limb_t,
    ) -> c_int;
}
extern "C" {
    pub fn mpoly_monomial_index1_nomask(
        Aexps: *mut mp_limb_t,
        Alen: mp_limb_signed_t,
        e: mp_limb_t,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn mpoly_monomial_index_ui(
        Aexp: *const mp_limb_t,
        Abits: mp_limb_t,
        Alength: mp_limb_signed_t,
        exp: *const mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn mpoly_monomial_index_pfmpz(
        Aexp: *const mp_limb_t,
        Abits: mp_limb_t,
        Alength: mp_limb_signed_t,
        exp: *const *mut fmpz,
        mctx: *mut mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn mpoly_monomial_index_monomial(
        Aexp: *const mp_limb_t,
        Abits: mp_limb_t,
        Alength: mp_limb_signed_t,
        Mexp: *const mp_limb_t,
        Mbits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn mpoly_min_fields_ui_sp(
        min_fields: *mut mp_limb_t,
        poly_exps: *const mp_limb_t,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_min_fields_fmpz(
        min_fields: *mut fmpz,
        poly_exps: *const mp_limb_t,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_max_fields_ui_sp(
        max_fields: *mut mp_limb_t,
        poly_exps: *const mp_limb_t,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_max_fields_fmpz(
        max_fields: *mut fmpz,
        poly_exps: *const mp_limb_t,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_degrees_fit_si(
        poly_exps: *const mp_limb_t,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn mpoly_degrees_si(
        user_degs: *mut mp_limb_signed_t,
        poly_exps: *const mp_limb_t,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_degrees_si_threaded(
        user_degs: *mut mp_limb_signed_t,
        poly_exps: *const mp_limb_t,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
        handles: *const thread_pool_handle,
        num_handles: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn mpoly_degrees_ffmpz(
        user_degs: *mut fmpz,
        poly_exps: *const mp_limb_t,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_degrees_pfmpz(
        user_degs: *mut *mut fmpz,
        poly_exps: *const mp_limb_t,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_degree_si(
        poly_exps: *const mp_limb_t,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
        var: mp_limb_signed_t,
        mctx: *mut mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn mpoly_degree_fmpz(
        deg: *mut fmpz,
        poly_exps: *const mp_limb_t,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
        var: mp_limb_signed_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_total_degree_fits_si(
        exps: *const mp_limb_t,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn mpoly_total_degree_si(
        exps: *const mp_limb_t,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn mpoly_total_degree_fmpz(
        totdeg: *mut fmpz,
        exps: *const mp_limb_t,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_total_degree_fmpz_ref(
        totdeg: *mut fmpz,
        exps: *const mp_limb_t,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_used_vars_or(
        used: *mut c_int,
        exps: *const mp_limb_t,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_monomial_cmp_general(
        Aexp: *mut mp_limb_t,
        Abits: mp_limb_t,
        Bexp: *mut mp_limb_t,
        Bbits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn mpoly_search_monomials(
        e_ind: *mut *mut mp_limb_signed_t,
        e: *mut mp_limb_t,
        e_score: *mut mp_limb_signed_t,
        t1: *mut mp_limb_signed_t,
        t2: *mut mp_limb_signed_t,
        t3: *mut mp_limb_signed_t,
        lower: mp_limb_signed_t,
        upper: mp_limb_signed_t,
        a: *const mp_limb_t,
        a_len: mp_limb_signed_t,
        b: *const mp_limb_t,
        b_len: mp_limb_signed_t,
        N: mp_limb_signed_t,
        cmpmask: *const mp_limb_t,
    );
}
extern "C" {
    pub fn mpoly_main_variable_split_LEX(
        ind: *mut mp_limb_signed_t,
        pexp: *mut mp_limb_t,
        Aexp: *const mp_limb_t,
        l1: mp_limb_signed_t,
        Alen: mp_limb_signed_t,
        mults: *const mp_limb_t,
        num: mp_limb_signed_t,
        Abits: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn mpoly_main_variable_split_DEG(
        ind: *mut mp_limb_signed_t,
        pexp: *mut mp_limb_t,
        Aexp: *const mp_limb_t,
        l1: mp_limb_signed_t,
        Alen: mp_limb_signed_t,
        deg: mp_limb_t,
        num: mp_limb_signed_t,
        Abits: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn mpoly_term_exp_fits_si(
        exps: *mut mp_limb_t,
        bits: mp_limb_t,
        n: mp_limb_signed_t,
        mctx: *mut mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn mpoly_term_exp_fits_ui(
        exps: *mut mp_limb_t,
        bits: mp_limb_t,
        n: mp_limb_signed_t,
        mctx: *mut mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn mpoly_is_gen(
        exps: *mut mp_limb_t,
        var: mp_limb_signed_t,
        bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn mpoly_monomials_valid_test(
        exps: *mut mp_limb_t,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn mpoly_monomials_overflow_test(
        exps: *mut mp_limb_t,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn mpoly_monomials_inorder_test(
        exps: *mut mp_limb_t,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn mpoly_reverse(
        Aexp: *mut mp_limb_t,
        Bexp: *const mp_limb_t,
        len: mp_limb_signed_t,
        N: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn mpoly_monomials_deflation(
        shift: *mut fmpz,
        stride: *mut fmpz,
        Aexps: *const mp_limb_t,
        Abits: mp_limb_t,
        Alength: mp_limb_signed_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_monomials_deflate(
        Aexps: *mut mp_limb_t,
        Abits: mp_limb_t,
        Bexps: *const mp_limb_t,
        Bbits: mp_limb_t,
        Blength: mp_limb_signed_t,
        shift: *const fmpz,
        stride: *const fmpz,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_monomials_inflate(
        Aexps: *mut mp_limb_t,
        Abits: mp_limb_t,
        Bexps: *const mp_limb_t,
        Bbits: mp_limb_t,
        Blength: mp_limb_signed_t,
        shift: *const fmpz,
        stride: *const fmpz,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _mpoly_gen_shift_right(
        Aexp: *mut mp_limb_t,
        Abits: mp_limb_t,
        Alength: mp_limb_signed_t,
        var: mp_limb_signed_t,
        amount: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _mpoly_gen_shift_right_fmpz(
        Aexp: *mut mp_limb_t,
        Abits: mp_limb_t,
        Alength: mp_limb_signed_t,
        var: mp_limb_signed_t,
        amount: *mut fmpz,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _mpoly_gen_shift_left(
        Aexp: *mut mp_limb_t,
        Abits: mp_limb_t,
        Alength: mp_limb_signed_t,
        var: mp_limb_signed_t,
        amount: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_monomials_shift_right_ui(
        Aexps: *mut mp_limb_t,
        Abits: mp_limb_t,
        Alength: mp_limb_signed_t,
        user_exps: *const mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_monomials_shift_right_ffmpz(
        Aexps: *mut mp_limb_t,
        Abits: mp_limb_t,
        Alength: mp_limb_signed_t,
        user_exps: *const fmpz,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly1_fill_marks(
        Dcoeffs: *mut *mut mp_limb_t,
        Dlen: *mut mp_limb_signed_t,
        Dalloc: *mut mp_limb_signed_t,
        Aexps: *const mp_limb_t,
        Alen: mp_limb_signed_t,
        Abits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly2_fill_marks(
        Dcoeffs: *mut *mut mp_limb_t,
        Dlen: *mut mp_limb_signed_t,
        Dalloc: *mut mp_limb_signed_t,
        Aexps: *const mp_limb_t,
        Alen: mp_limb_signed_t,
        Abits: mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_to_mpolyl_perm_deflate(
        Aexps: *mut mp_limb_t,
        Abits: mp_limb_t,
        Actx: *mut mpoly_ctx_struct,
        Bexps: *mut mp_limb_t,
        Bbits: mp_limb_t,
        Bctx: *mut mpoly_ctx_struct,
        length: mp_limb_signed_t,
        perm: *const mp_limb_signed_t,
        shift: *const mp_limb_t,
        stride: *const mp_limb_t,
    );
}
extern "C" {
    pub fn mpoly_from_mpolyl_perm_inflate(
        Bexps: *mut mp_limb_t,
        Bbits: mp_limb_t,
        Bctx: *mut mpoly_ctx_struct,
        Aexps: *mut mp_limb_t,
        Abits: mp_limb_t,
        Actx: *mut mpoly_ctx_struct,
        length: mp_limb_signed_t,
        perm: *const mp_limb_signed_t,
        shift: *const mp_limb_t,
        stride: *const mp_limb_t,
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mpoly_gcd_info_struct {
    pub Amax_exp: *mut mp_limb_t,
    pub Amin_exp: *mut mp_limb_t,
    pub Astride: *mut mp_limb_t,
    pub Adeflate_deg: *mut mp_limb_signed_t,
    pub Alead_count: *mut mp_limb_signed_t,
    pub Atail_count: *mut mp_limb_signed_t,
    pub Bmax_exp: *mut mp_limb_t,
    pub Bmin_exp: *mut mp_limb_t,
    pub Bstride: *mut mp_limb_t,
    pub Bdeflate_deg: *mut mp_limb_signed_t,
    pub Blead_count: *mut mp_limb_signed_t,
    pub Btail_count: *mut mp_limb_signed_t,
    pub Gmin_exp: *mut mp_limb_t,
    pub Abarmin_exp: *mut mp_limb_t,
    pub Bbarmin_exp: *mut mp_limb_t,
    pub Gstride: *mut mp_limb_t,
    pub Gterm_count_est: *mut mp_limb_signed_t,
    pub Gdeflate_deg_bound: *mut mp_limb_signed_t,
    pub Gbits: mp_limb_t,
    pub Abarbits: mp_limb_t,
    pub Bbarbits: mp_limb_t,
    pub mvars: mp_limb_signed_t,
    pub Adeflate_tdeg: mp_limb_signed_t,
    pub Bdeflate_tdeg: mp_limb_signed_t,
    pub Adensity: f64,
    pub Bdensity: f64,
    pub hensel_time: f64,
    pub brown_time: f64,
    pub zippel_time: f64,
    pub zippel2_time: f64,
    pub hensel_perm: *mut mp_limb_signed_t,
    pub brown_perm: *mut mp_limb_signed_t,
    pub zippel_perm: *mut mp_limb_signed_t,
    pub zippel2_perm: *mut mp_limb_signed_t,
    pub can_use: c_uint,
    pub Gdeflate_deg_bounds_are_nice: c_int,
    pub data: *mut c_char,
}
pub type mpoly_gcd_info_t = [mpoly_gcd_info_struct; 1usize];
extern "C" {
    pub fn mpoly_gcd_info_init(I: *mut mpoly_gcd_info_struct, nvars: mp_limb_signed_t);
}
extern "C" {
    pub fn mpoly_gcd_info_clear(I: *mut mpoly_gcd_info_struct);
}
extern "C" {
    pub fn mpoly_gcd_info_limits(
        Amax_exp: *mut mp_limb_t,
        Amin_exp: *mut mp_limb_t,
        Amax_exp_count: *mut mp_limb_signed_t,
        Amin_exp_count: *mut mp_limb_signed_t,
        Aexps: *const mp_limb_t,
        Abits: mp_limb_t,
        Alength: mp_limb_signed_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_gcd_info_stride(
        strides: *mut mp_limb_t,
        Aexps: *const mp_limb_t,
        Abits: mp_limb_t,
        Alength: mp_limb_signed_t,
        Amax_exp: *const mp_limb_t,
        Amin_exp: *const mp_limb_t,
        Bexps: *const mp_limb_t,
        Bbits: mp_limb_t,
        Blength: mp_limb_signed_t,
        Bmax_exp: *const mp_limb_t,
        Bmin_exp: *const mp_limb_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_gcd_info_set_perm(
        I: *mut mpoly_gcd_info_struct,
        Alength: mp_limb_signed_t,
        Blength: mp_limb_signed_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_gcd_info_get_brown_upper_limit(
        I: *mut mpoly_gcd_info_struct,
        var: mp_limb_signed_t,
        bound: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn mpoly_gcd_info_measure_hensel(
        I: *mut mpoly_gcd_info_struct,
        Alength: mp_limb_signed_t,
        Blength: mp_limb_signed_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_gcd_info_measure_brown(
        I: *mut mpoly_gcd_info_struct,
        Alength: mp_limb_signed_t,
        Blength: mp_limb_signed_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_gcd_info_measure_bma(
        I: *mut mpoly_gcd_info_struct,
        Alength: mp_limb_signed_t,
        Blength: mp_limb_signed_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_gcd_info_measure_zippel(
        I: *mut mpoly_gcd_info_struct,
        Alength: mp_limb_signed_t,
        Blength: mp_limb_signed_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_gcd_info_measure_zippel2(
        I: *mut mpoly_gcd_info_struct,
        Alength: mp_limb_signed_t,
        Blength: mp_limb_signed_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_monomial_cofactors(
        Abarexps: *mut fmpz,
        Bbarexps: *mut fmpz,
        Aexps: *const mp_limb_t,
        Abits: mp_limb_t,
        Bexps: *const mp_limb_t,
        Bbits: mp_limb_t,
        length: mp_limb_signed_t,
        mctx: *mut mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn mpoly_is_proved_not_square(
        Aexps: *const mp_limb_t,
        Alen: mp_limb_signed_t,
        Abits: mp_limb_t,
        N: mp_limb_signed_t,
        t: *mut mp_limb_t,
    ) -> c_int;
}
extern "C" {
    pub fn mpoly_remove_var_powers(
        var_powers: *mut fmpz,
        Aexps: *mut mp_limb_t,
        Abits: mp_limb_t,
        Alen: mp_limb_signed_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn _mpoly_compress_exps(
        V: *mut mp_limb_signed_t,
        D: *mut mp_limb_signed_t,
        deg: *mut mp_limb_signed_t,
        S: *mut mp_limb_signed_t,
        n: mp_limb_signed_t,
        l: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
}
extern "C" {
    pub fn mpoly_test_irreducible(
        Aexps: *mut mp_limb_t,
        Abits: mp_limb_t,
        Alen: mp_limb_signed_t,
        ctx: *mut mpoly_ctx_struct,
    ) -> c_int;
}
extern "C" {
    pub fn _mpoly_test_irreducible(
        Aexps: *mut mp_limb_signed_t,
        stride: mp_limb_signed_t,
        Alen: mp_limb_signed_t,
        nvars: mp_limb_signed_t,
        state: *mut flint_rand_s,
        tries_left: mp_limb_signed_t,
    ) -> c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mpoly_compression_struct {
    pub mvars: mp_limb_signed_t,
    pub nvars: mp_limb_signed_t,
    pub exps: *mut mp_limb_signed_t,
    pub exps_alloc: mp_limb_signed_t,
    pub rest: *mut mp_limb_signed_t,
    pub rest_alloc: mp_limb_signed_t,
    pub umat: *mut mp_limb_signed_t,
    pub deltas: *mut mp_limb_signed_t,
    pub degs: *mut mp_limb_signed_t,
    pub is_trivial: c_int,
    pub is_perm: c_int,
    pub is_irred: c_int,
}
pub type mpoly_compression_t = [mpoly_compression_struct; 1usize];
extern "C" {
    pub fn mpoly_compression_init(M: *mut mpoly_compression_struct);
}
extern "C" {
    pub fn mpoly_compression_clear(M: *mut mpoly_compression_struct);
}
extern "C" {
    pub fn mpoly_compression_set(
        M: *mut mpoly_compression_struct,
        Aexps: *const mp_limb_t,
        Abits: mp_limb_t,
        Alen: mp_limb_signed_t,
        mctx: *mut mpoly_ctx_struct,
    );
}
extern "C" {
    pub fn mpoly_bivar_cld_bounds(l: *mut mp_limb_signed_t, n: mp_limb_signed_t);
}
extern "C" {
    pub fn _slong_array_fit_length(
        array: *mut *mut mp_limb_signed_t,
        alloc: *mut mp_limb_signed_t,
        len: mp_limb_signed_t,
    );
}
extern "C" {
    pub fn _mpoly_heap_pop1(
        heap: *mut mpoly_heap1_s,
        heap_len: *mut mp_limb_signed_t,
        maskhi: mp_limb_t,
    ) -> *mut c_void;
}
extern "C" {
    pub fn _mpoly_heap_insert1(
        heap: *mut mpoly_heap1_s,
        exp: mp_limb_t,
        x: *mut c_void,
        next_loc: *mut mp_limb_signed_t,
        heap_len: *mut mp_limb_signed_t,
        maskhi: mp_limb_t,
    );
}
extern "C" {
    pub fn _mpoly_heap_pop(
        heap: *mut mpoly_heap_s,
        heap_len: *mut mp_limb_signed_t,
        N: mp_limb_signed_t,
        cmpmask: *const mp_limb_t,
    ) -> *mut c_void;
}
extern "C" {
    pub fn _mpoly_heap_insert(
        heap: *mut mpoly_heap_s,
        exp: *mut mp_limb_t,
        x: *mut c_void,
        next_loc: *mut mp_limb_signed_t,
        heap_len: *mut mp_limb_signed_t,
        N: mp_limb_signed_t,
        cmpmask: *const mp_limb_t,
    ) -> c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mpoly_univar_struct {
    pub coeffs: *mut c_char,
    pub exps: *mut fmpz,
    pub length: mp_limb_signed_t,
    pub alloc: mp_limb_signed_t,
}
pub type mpoly_univar_t = [mpoly_univar_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _bindgen_ty_22 {
    pub elem_size: mp_limb_signed_t,
    pub ctx: *const c_void,
    pub init: ::std::option::Option<unsafe extern "C" fn(arg1: *mut c_void, arg2: *const c_void)>,
    pub clear: ::std::option::Option<unsafe extern "C" fn(arg1: *mut c_void, arg2: *const c_void)>,
    pub is_zero: ::std::option::Option<
        unsafe extern "C" fn(arg1: *const c_void, arg2: *const c_void) -> c_int,
    >,
    pub zero: ::std::option::Option<unsafe extern "C" fn(arg1: *mut c_void, arg2: *const c_void)>,
    pub one: ::std::option::Option<unsafe extern "C" fn(arg1: *mut c_void, arg2: *const c_void)>,
    pub set_fmpz: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut c_void, arg2: *mut fmpz, arg3: *const c_void),
    >,
    pub set: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut c_void, arg2: *const c_void, arg3: *const c_void),
    >,
    pub swap: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut c_void, arg2: *mut c_void, arg3: *const c_void),
    >,
    pub neg: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut c_void, arg2: *const c_void, arg3: *const c_void),
    >,
    pub add: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut c_void,
            arg2: *const c_void,
            arg3: *const c_void,
            arg4: *const c_void,
        ),
    >,
    pub sub: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut c_void,
            arg2: *const c_void,
            arg3: *const c_void,
            arg4: *const c_void,
        ),
    >,
    pub mul_fmpz: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut c_void,
            arg2: *const c_void,
            arg3: *mut fmpz,
            arg4: *const c_void,
        ),
    >,
    pub mul: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut c_void,
            arg2: *const c_void,
            arg3: *const c_void,
            arg4: *const c_void,
        ),
    >,
    pub divexact: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut c_void,
            arg2: *const c_void,
            arg3: *const c_void,
            arg4: *const c_void,
        ),
    >,
    pub divides: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut c_void,
            arg2: *const c_void,
            arg3: *const c_void,
            arg4: *const c_void,
        ) -> c_int,
    >,
    pub pow_fmpz: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut c_void,
            arg2: *const c_void,
            arg3: *mut fmpz,
            arg4: *const c_void,
        ) -> c_int,
    >,
    pub length: ::std::option::Option<
        unsafe extern "C" fn(arg1: *const c_void, arg2: *const c_void) -> mp_limb_signed_t,
    >,
}
pub type mpoly_void_ring_t = [_bindgen_ty_22; 1usize];
extern "C" {
    pub fn mpoly_void_ring_elem_init(R: *mut _bindgen_ty_22) -> *mut c_void;
}
extern "C" {
    pub fn mpoly_void_ring_elem_clear(a: *mut c_void, R: *mut _bindgen_ty_22);
}
extern "C" {
    pub fn mpoly_univar_init(A: *mut mpoly_univar_struct, R: *mut _bindgen_ty_22);
}
extern "C" {
    pub fn mpoly_univar_clear(A: *mut mpoly_univar_struct, R: *mut _bindgen_ty_22);
}
extern "C" {
    pub fn mpoly_univar_swap(A: *mut mpoly_univar_struct, B: *mut mpoly_univar_struct);
}
extern "C" {
    pub fn mpoly_univar_fit_length(
        A: *mut mpoly_univar_struct,
        len: mp_limb_signed_t,
        R: *mut _bindgen_ty_22,
    );
}
extern "C" {
    pub fn mpoly_univar_init2(
        A: *mut mpoly_univar_struct,
        len: mp_limb_signed_t,
        R: *mut _bindgen_ty_22,
    );
}
extern "C" {
    pub fn mpoly_univar_pseudo_gcd_ducos(
        G: *mut mpoly_univar_struct,
        B: *mut mpoly_univar_struct,
        A: *mut mpoly_univar_struct,
        R: *mut _bindgen_ty_22,
    ) -> c_int;
}
extern "C" {
    pub fn mpoly_univar_resultant(
        r: *mut c_void,
        fx: *mut mpoly_univar_struct,
        gx: *mut mpoly_univar_struct,
        R: *mut _bindgen_ty_22,
    ) -> c_int;
}
extern "C" {
    pub fn mpoly_univar_discriminant(
        d: *mut c_void,
        fx: *mut mpoly_univar_struct,
        R: *mut _bindgen_ty_22,
    ) -> c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct string_with_length_struct {
    pub str_: *mut c_char,
    pub str_len: mp_limb_signed_t,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mpoly_parse_struct {
    pub R: mpoly_void_ring_t,
    pub stack: *mut mp_limb_signed_t,
    pub stack_len: mp_limb_signed_t,
    pub stack_alloc: mp_limb_signed_t,
    pub estore: *mut c_char,
    pub estore_len: mp_limb_signed_t,
    pub estore_alloc: mp_limb_signed_t,
    pub tmp: *mut c_void,
    pub terminal_strings: *mut string_with_length_struct,
    pub terminal_values: *mut c_char,
    pub terminals_alloc: mp_limb_signed_t,
    pub terminals_len: mp_limb_signed_t,
}
pub type mpoly_parse_t = [mpoly_parse_struct; 1usize];
extern "C" {
    pub fn mpoly_parse_init(E: *mut mpoly_parse_struct);
}
extern "C" {
    pub fn mpoly_parse_clear(E: *mut mpoly_parse_struct);
}
extern "C" {
    pub fn mpoly_parse_add_terminal(E: *mut mpoly_parse_struct, s: *const c_char, v: *const c_void);
}
extern "C" {
    pub fn mpoly_parse_parse(
        E: *mut mpoly_parse_struct,
        res: *mut c_void,
        s: *const c_char,
        len: mp_limb_signed_t,
    ) -> c_int;
}
extern "C" {
    pub fn mpoly_main_variable_terms1(
        i1: *mut mp_limb_signed_t,
        n1: *mut mp_limb_signed_t,
        exp1: *const mp_limb_t,
        l1: mp_limb_signed_t,
        len1: mp_limb_signed_t,
        k: mp_limb_signed_t,
        num: mp_limb_signed_t,
        bits: mp_limb_signed_t,
    );
}
