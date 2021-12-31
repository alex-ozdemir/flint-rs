#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

//! *See the [FLINT documentation](http://flintlib.org/doc/nmod_poly.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpz::fmpz;
use crate::nmod_mat::nmod_mat_struct;
use crate::nmod_vec::nmod_t;
use libc::{c_char, c_int, c_void, FILE};

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash)]
pub struct nmod_poly_struct {
    pub coeffs: mp_ptr,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
    pub mod_: nmod_t,
}

pub type nmod_poly_t = [nmod_poly_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash)]
pub struct nmod_poly_res_struct {
    pub res: mp_limb_t,
    pub lc: mp_limb_t,
    pub len0: mp_limb_signed_t,
    pub len1: mp_limb_signed_t,
    pub off: mp_limb_signed_t,
}

pub type nmod_poly_res_t = [nmod_poly_res_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nmod_poly_matrix_precompute_arg_t {
    pub A: *mut nmod_mat_struct,
    pub poly1: *mut nmod_poly_struct,
    pub poly2: *mut nmod_poly_struct,
    pub poly2inv: *mut nmod_poly_struct,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nmod_poly_compose_mod_precomp_preinv_arg_t {
    pub A: *mut nmod_mat_struct,
    pub res: *mut nmod_poly_struct,
    pub poly1: *mut nmod_poly_struct,
    pub poly3: *mut nmod_poly_struct,
    pub poly3inv: *mut nmod_poly_struct,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _nmod_poly_multi_crt_prog_instr {
    pub a_idx: mp_limb_signed_t,
    pub b_idx: mp_limb_signed_t,
    pub c_idx: mp_limb_signed_t,
    pub idem: nmod_poly_t,
    pub modulus: nmod_poly_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nmod_poly_multi_crt_struct {
    pub prog: *mut _nmod_poly_multi_crt_prog_instr,
    pub length: mp_limb_signed_t,
    pub alloc: mp_limb_signed_t,
    pub localsize: mp_limb_signed_t,
    pub temp1loc: mp_limb_signed_t,
    pub temp2loc: mp_limb_signed_t,
    pub good: c_int,
}

pub type nmod_poly_multi_crt_t = [nmod_poly_multi_crt_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nmod_berlekamp_massey_struct {
    pub npoints: mp_limb_signed_t,
    pub R0: nmod_poly_t,
    pub R1: nmod_poly_t,
    pub V0: nmod_poly_t,
    pub V1: nmod_poly_t,
    pub qt: nmod_poly_t,
    pub rt: nmod_poly_t,
    pub points: nmod_poly_t,
}

pub type nmod_berlekamp_massey_t = [nmod_berlekamp_massey_struct; 1usize];

extern "C" {
    pub fn signed_mpn_sub_n(
        res: mp_ptr,
        op1: mp_srcptr,
        op2: mp_srcptr,
        n: mp_limb_signed_t,
    ) -> c_int;
    pub fn nmod_poly_init(poly: *mut nmod_poly_struct, n: mp_limb_t);
    pub fn nmod_poly_init_preinv(poly: *mut nmod_poly_struct, n: mp_limb_t, ninv: mp_limb_t);
    pub fn nmod_poly_init2(poly: *mut nmod_poly_struct, n: mp_limb_t, alloc: mp_limb_signed_t);
    pub fn nmod_poly_init2_preinv(
        poly: *mut nmod_poly_struct,
        n: mp_limb_t,
        ninv: mp_limb_t,
        alloc: mp_limb_signed_t,
    );
    pub fn nmod_poly_realloc(poly: *mut nmod_poly_struct, alloc: mp_limb_signed_t);
    pub fn nmod_poly_clear(poly: *mut nmod_poly_struct);
    pub fn nmod_poly_fit_length(poly: *mut nmod_poly_struct, alloc: mp_limb_signed_t);
    pub fn nmod_poly_init_mod(poly: *mut nmod_poly_struct, mod_: nmod_t);
    pub fn nmod_poly_set_mod(poly: *mut nmod_poly_struct, mod_: nmod_t);
    pub fn _nmod_poly_set_length(poly: *mut nmod_poly_struct, len: mp_limb_signed_t);
    pub fn _nmod_poly_normalise(poly: *mut nmod_poly_struct);
    pub fn nmod_poly_length(poly: *mut nmod_poly_struct) -> mp_limb_signed_t;
    pub fn nmod_poly_degree(poly: *mut nmod_poly_struct) -> mp_limb_signed_t;
    pub fn nmod_poly_modulus(poly: *mut nmod_poly_struct) -> mp_limb_t;
    pub fn nmod_poly_max_bits(poly: *mut nmod_poly_struct) -> mp_limb_t;
    pub fn nmod_poly_lead(poly: *mut nmod_poly_struct) -> mp_ptr;
    pub fn nmod_poly_set(a: *mut nmod_poly_struct, b: *mut nmod_poly_struct);
    pub fn nmod_poly_swap(poly1: *mut nmod_poly_struct, poly2: *mut nmod_poly_struct);
    pub fn nmod_poly_zero(res: *mut nmod_poly_struct);
    pub fn nmod_poly_one(res: *mut nmod_poly_struct);
    pub fn nmod_poly_truncate(poly: *mut nmod_poly_struct, len: mp_limb_signed_t);
    pub fn nmod_poly_set_trunc(
        res: *mut nmod_poly_struct,
        poly: *mut nmod_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_reverse(
        output: mp_ptr,
        input: mp_srcptr,
        len: mp_limb_signed_t,
        m: mp_limb_signed_t,
    );
    pub fn nmod_poly_reverse(
        output: *mut nmod_poly_struct,
        input: *mut nmod_poly_struct,
        m: mp_limb_signed_t,
    );
    pub fn nmod_poly_equal(a: *mut nmod_poly_struct, b: *mut nmod_poly_struct) -> c_int;
    pub fn nmod_poly_equal_trunc(
        poly1: *mut nmod_poly_struct,
        poly2: *mut nmod_poly_struct,
        n: mp_limb_signed_t,
    ) -> c_int;
    pub fn nmod_poly_is_zero(poly: *mut nmod_poly_struct) -> c_int;
    pub fn nmod_poly_is_one(poly: *mut nmod_poly_struct) -> c_int;
    pub fn nmod_poly_randtest(
        poly: *mut nmod_poly_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
    );
    pub fn nmod_poly_randtest_not_zero(
        poly: *mut nmod_poly_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
    );
    pub fn nmod_poly_randtest_irreducible(
        poly: *mut nmod_poly_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
    );
    pub fn nmod_poly_randtest_monic(
        poly: *mut nmod_poly_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
    );
    pub fn nmod_poly_randtest_monic_irreducible(
        poly: *mut nmod_poly_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
    );
    pub fn nmod_poly_randtest_monic_primitive(
        poly: *mut nmod_poly_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
    );
    pub fn nmod_poly_randtest_trinomial(
        poly: *mut nmod_poly_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
    );
    pub fn nmod_poly_randtest_trinomial_irreducible(
        poly: *mut nmod_poly_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
        max_attempts: mp_limb_signed_t,
    ) -> c_int;
    pub fn nmod_poly_randtest_pentomial(
        poly: *mut nmod_poly_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
    );
    pub fn nmod_poly_randtest_pentomial_irreducible(
        poly: *mut nmod_poly_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
        max_attempts: mp_limb_signed_t,
    ) -> c_int;
    pub fn nmod_poly_randtest_sparse_irreducible(
        poly: *mut nmod_poly_struct,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
    );
    pub fn nmod_poly_get_coeff_ui(poly: *mut nmod_poly_struct, j: mp_limb_signed_t) -> mp_limb_t;
    pub fn nmod_poly_set_coeff_ui(poly: *mut nmod_poly_struct, j: mp_limb_signed_t, c: mp_limb_t);
    pub fn nmod_poly_get_str(poly: *mut nmod_poly_struct) -> *mut c_char;
    pub fn nmod_poly_get_str_pretty(poly: *mut nmod_poly_struct, x: *const c_char) -> *mut c_char;
    pub fn nmod_poly_set_str(poly: *mut nmod_poly_struct, s: *const c_char) -> c_int;
    pub fn nmod_poly_fread(f: *mut FILE, poly: *mut nmod_poly_struct) -> c_int;
    pub fn nmod_poly_fprint(f: *mut FILE, poly: *mut nmod_poly_struct) -> c_int;
    pub fn nmod_poly_fprint_pretty(
        f: *mut FILE,
        a: *mut nmod_poly_struct,
        x: *const c_char,
    ) -> c_int;
    pub fn nmod_poly_print(a: *mut nmod_poly_struct) -> c_int;
    pub fn nmod_poly_print_pretty(a: *mut nmod_poly_struct, x: *const c_char) -> c_int;
    pub fn nmod_poly_read(poly: *mut nmod_poly_struct) -> c_int;
    pub fn _nmod_poly_shift_left(
        res: mp_ptr,
        poly: mp_srcptr,
        len: mp_limb_signed_t,
        k: mp_limb_signed_t,
    );
    pub fn nmod_poly_shift_left(
        res: *mut nmod_poly_struct,
        poly: *mut nmod_poly_struct,
        k: mp_limb_signed_t,
    );
    pub fn _nmod_poly_shift_right(
        res: mp_ptr,
        poly: mp_srcptr,
        len: mp_limb_signed_t,
        k: mp_limb_signed_t,
    );
    pub fn nmod_poly_shift_right(
        res: *mut nmod_poly_struct,
        poly: *mut nmod_poly_struct,
        k: mp_limb_signed_t,
    );
    pub fn _nmod_poly_add(
        res: mp_ptr,
        poly1: mp_srcptr,
        len1: mp_limb_signed_t,
        poly2: mp_srcptr,
        len2: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_add(
        res: *mut nmod_poly_struct,
        poly1: *mut nmod_poly_struct,
        poly2: *mut nmod_poly_struct,
    );
    pub fn nmod_poly_add_ui(res: *mut nmod_poly_struct, poly: *mut nmod_poly_struct, c: mp_limb_t);
    pub fn nmod_poly_add_series(
        res: *mut nmod_poly_struct,
        poly1: *mut nmod_poly_struct,
        poly2: *mut nmod_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_sub(
        res: mp_ptr,
        poly1: mp_srcptr,
        len1: mp_limb_signed_t,
        poly2: mp_srcptr,
        len2: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_sub(
        res: *mut nmod_poly_struct,
        poly1: *mut nmod_poly_struct,
        poly2: *mut nmod_poly_struct,
    );
    pub fn nmod_poly_sub_series(
        res: *mut nmod_poly_struct,
        poly1: *mut nmod_poly_struct,
        poly2: *mut nmod_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn nmod_poly_sub_ui(res: *mut nmod_poly_struct, poly: *mut nmod_poly_struct, c: mp_limb_t);
    pub fn nmod_poly_neg(res: *mut nmod_poly_struct, poly1: *mut nmod_poly_struct);
    pub fn nmod_poly_scalar_mul_nmod(
        res: *mut nmod_poly_struct,
        poly1: *mut nmod_poly_struct,
        c: mp_limb_t,
    );
    pub fn _nmod_poly_make_monic(
        output: mp_ptr,
        input: mp_srcptr,
        len: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_make_monic(output: *mut nmod_poly_struct, input: *mut nmod_poly_struct);
    pub fn _nmod_poly_KS2_pack1(
        res: mp_ptr,
        op: mp_srcptr,
        n: mp_limb_signed_t,
        s: mp_limb_signed_t,
        b: mp_limb_t,
        k: mp_limb_t,
        r: mp_limb_signed_t,
    );
    pub fn _nmod_poly_KS2_pack(
        res: mp_ptr,
        op: mp_srcptr,
        n: mp_limb_signed_t,
        s: mp_limb_signed_t,
        b: mp_limb_t,
        k: mp_limb_t,
        r: mp_limb_signed_t,
    );
    pub fn _nmod_poly_KS2_unpack1(
        res: mp_ptr,
        op: mp_srcptr,
        n: mp_limb_signed_t,
        b: mp_limb_t,
        k: mp_limb_t,
    );
    pub fn _nmod_poly_KS2_unpack2(
        res: mp_ptr,
        op: mp_srcptr,
        n: mp_limb_signed_t,
        b: mp_limb_t,
        k: mp_limb_t,
    );
    pub fn _nmod_poly_KS2_unpack3(
        res: mp_ptr,
        op: mp_srcptr,
        n: mp_limb_signed_t,
        b: mp_limb_t,
        k: mp_limb_t,
    );
    pub fn _nmod_poly_KS2_unpack(
        res: mp_ptr,
        op: mp_srcptr,
        n: mp_limb_signed_t,
        b: mp_limb_t,
        k: mp_limb_t,
    );
    pub fn _nmod_poly_KS2_reduce(
        res: mp_ptr,
        s: mp_limb_signed_t,
        op: mp_srcptr,
        n: mp_limb_signed_t,
        w: mp_limb_t,
        mod_: nmod_t,
    );
    pub fn _nmod_poly_KS2_recover_reduce1(
        res: mp_ptr,
        s: mp_limb_signed_t,
        op1: mp_srcptr,
        op2: mp_srcptr,
        n: mp_limb_signed_t,
        b: mp_limb_t,
        mod_: nmod_t,
    );
    pub fn _nmod_poly_KS2_recover_reduce2(
        res: mp_ptr,
        s: mp_limb_signed_t,
        op1: mp_srcptr,
        op2: mp_srcptr,
        n: mp_limb_signed_t,
        b: mp_limb_t,
        mod_: nmod_t,
    );
    pub fn _nmod_poly_KS2_recover_reduce2b(
        res: mp_ptr,
        s: mp_limb_signed_t,
        op1: mp_srcptr,
        op2: mp_srcptr,
        n: mp_limb_signed_t,
        b: mp_limb_t,
        mod_: nmod_t,
    );
    pub fn _nmod_poly_KS2_recover_reduce3(
        res: mp_ptr,
        s: mp_limb_signed_t,
        op1: mp_srcptr,
        op2: mp_srcptr,
        n: mp_limb_signed_t,
        b: mp_limb_t,
        mod_: nmod_t,
    );
    pub fn _nmod_poly_KS2_recover_reduce(
        res: mp_ptr,
        s: mp_limb_signed_t,
        op1: mp_srcptr,
        op2: mp_srcptr,
        n: mp_limb_signed_t,
        b: mp_limb_t,
        mod_: nmod_t,
    );
    pub fn _nmod_poly_bit_pack(
        res: mp_ptr,
        poly: mp_srcptr,
        len: mp_limb_signed_t,
        bits: mp_limb_t,
    );
    pub fn _nmod_poly_bit_unpack(
        res: mp_ptr,
        len: mp_limb_signed_t,
        mpn: mp_srcptr,
        bits: mp_limb_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_bit_pack(f: *mut fmpz, poly: *mut nmod_poly_struct, bit_size: mp_limb_t);
    pub fn nmod_poly_bit_unpack(poly: *mut nmod_poly_struct, f: *mut fmpz, bit_size: mp_limb_t);
    pub fn _nmod_poly_mul_classical(
        res: mp_ptr,
        poly1: mp_srcptr,
        len1: mp_limb_signed_t,
        poly2: mp_srcptr,
        len2: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_mul_classical(
        res: *mut nmod_poly_struct,
        poly1: *mut nmod_poly_struct,
        poly2: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_mullow_classical(
        res: mp_ptr,
        poly1: mp_srcptr,
        len1: mp_limb_signed_t,
        poly2: mp_srcptr,
        len2: mp_limb_signed_t,
        trunc: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_mullow_classical(
        res: *mut nmod_poly_struct,
        poly1: *mut nmod_poly_struct,
        poly2: *mut nmod_poly_struct,
        trunc: mp_limb_signed_t,
    );
    pub fn _nmod_poly_mulhigh_classical(
        res: mp_ptr,
        poly1: mp_srcptr,
        len1: mp_limb_signed_t,
        poly2: mp_srcptr,
        len2: mp_limb_signed_t,
        start: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_mulhigh_classical(
        res: *mut nmod_poly_struct,
        poly1: *mut nmod_poly_struct,
        poly2: *mut nmod_poly_struct,
        start: mp_limb_signed_t,
    );
    pub fn _nmod_poly_mul_KS(
        out: mp_ptr,
        in1: mp_srcptr,
        len1: mp_limb_signed_t,
        in2: mp_srcptr,
        len2: mp_limb_signed_t,
        bits: mp_limb_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_mul_KS(
        res: *mut nmod_poly_struct,
        poly1: *mut nmod_poly_struct,
        poly2: *mut nmod_poly_struct,
        bits: mp_limb_t,
    );
    pub fn _nmod_poly_mul_KS2(
        res: mp_ptr,
        op1: mp_srcptr,
        n1: mp_limb_signed_t,
        op2: mp_srcptr,
        n2: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_mul_KS2(
        res: *mut nmod_poly_struct,
        poly1: *mut nmod_poly_struct,
        poly2: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_mul_KS4(
        res: mp_ptr,
        op1: mp_srcptr,
        n1: mp_limb_signed_t,
        op2: mp_srcptr,
        n2: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_mul_KS4(
        res: *mut nmod_poly_struct,
        poly1: *mut nmod_poly_struct,
        poly2: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_mullow_KS(
        out: mp_ptr,
        in1: mp_srcptr,
        len1: mp_limb_signed_t,
        in2: mp_srcptr,
        len2: mp_limb_signed_t,
        bits: mp_limb_t,
        n: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_mullow_KS(
        res: *mut nmod_poly_struct,
        poly1: *mut nmod_poly_struct,
        poly2: *mut nmod_poly_struct,
        bits: mp_limb_t,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_mul(
        res: mp_ptr,
        poly1: mp_srcptr,
        len1: mp_limb_signed_t,
        poly2: mp_srcptr,
        len2: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_mul(
        res: *mut nmod_poly_struct,
        poly1: *mut nmod_poly_struct,
        poly2: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_mullow(
        res: mp_ptr,
        poly1: mp_srcptr,
        len1: mp_limb_signed_t,
        poly2: mp_srcptr,
        len2: mp_limb_signed_t,
        trunc: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_mullow(
        res: *mut nmod_poly_struct,
        poly1: *mut nmod_poly_struct,
        poly2: *mut nmod_poly_struct,
        trunc: mp_limb_signed_t,
    );
    pub fn _nmod_poly_mulhigh(
        res: mp_ptr,
        poly1: mp_srcptr,
        len1: mp_limb_signed_t,
        poly2: mp_srcptr,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_mulhigh(
        res: *mut nmod_poly_struct,
        poly1: *mut nmod_poly_struct,
        poly2: *mut nmod_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_mulmod(
        res: mp_ptr,
        poly1: mp_srcptr,
        len1: mp_limb_signed_t,
        poly2: mp_srcptr,
        len2: mp_limb_signed_t,
        f: mp_srcptr,
        lenf: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_mulmod(
        res: *mut nmod_poly_struct,
        poly1: *mut nmod_poly_struct,
        poly2: *mut nmod_poly_struct,
        f: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_mulmod_preinv(
        res: mp_ptr,
        poly1: mp_srcptr,
        len1: mp_limb_signed_t,
        poly2: mp_srcptr,
        len2: mp_limb_signed_t,
        f: mp_srcptr,
        lenf: mp_limb_signed_t,
        finv: mp_srcptr,
        lenfinv: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_mulmod_preinv(
        res: *mut nmod_poly_struct,
        poly1: *mut nmod_poly_struct,
        poly2: *mut nmod_poly_struct,
        f: *mut nmod_poly_struct,
        finv: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_invmod(
        A: *mut mp_limb_t,
        B: *const mp_limb_t,
        lenB: mp_limb_signed_t,
        P: *const mp_limb_t,
        lenP: mp_limb_signed_t,
        mod_: nmod_t,
    ) -> c_int;
    pub fn nmod_poly_invmod(
        A: *mut nmod_poly_struct,
        B: *mut nmod_poly_struct,
        P: *mut nmod_poly_struct,
    ) -> c_int;
    pub fn _nmod_poly_pow_binexp(
        res: mp_ptr,
        poly: mp_srcptr,
        len: mp_limb_signed_t,
        e: mp_limb_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_pow_binexp(
        res: *mut nmod_poly_struct,
        poly: *mut nmod_poly_struct,
        e: mp_limb_t,
    );
    pub fn _nmod_poly_pow(
        res: mp_ptr,
        poly: mp_srcptr,
        len: mp_limb_signed_t,
        e: mp_limb_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_pow(res: *mut nmod_poly_struct, poly: *mut nmod_poly_struct, e: mp_limb_t);
    pub fn _nmod_poly_pow_trunc_binexp(
        res: mp_ptr,
        poly: mp_srcptr,
        e: mp_limb_t,
        trunc: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_pow_trunc_binexp(
        res: *mut nmod_poly_struct,
        poly: *mut nmod_poly_struct,
        e: mp_limb_t,
        trunc: mp_limb_signed_t,
    );
    pub fn _nmod_poly_pow_trunc(
        res: mp_ptr,
        poly: mp_srcptr,
        e: mp_limb_t,
        trunc: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_pow_trunc(
        res: *mut nmod_poly_struct,
        poly: *mut nmod_poly_struct,
        e: mp_limb_t,
        trunc: mp_limb_signed_t,
    );
    pub fn nmod_poly_powmod_ui_binexp(
        res: *mut nmod_poly_struct,
        poly: *mut nmod_poly_struct,
        e: mp_limb_t,
        f: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_powmod_ui_binexp(
        res: mp_ptr,
        poly: mp_srcptr,
        e: mp_limb_t,
        f: mp_srcptr,
        lenf: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_powmod_fmpz_binexp(
        res: *mut nmod_poly_struct,
        poly: *mut nmod_poly_struct,
        e: *mut fmpz,
        f: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_powmod_fmpz_binexp(
        res: mp_ptr,
        poly: mp_srcptr,
        e: *mut fmpz,
        f: mp_srcptr,
        lenf: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn _nmod_poly_powmod_mpz_binexp(
        res: mp_ptr,
        poly: mp_srcptr,
        e: mpz_srcptr,
        f: mp_srcptr,
        lenf: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_powmod_mpz_binexp(
        res: *mut nmod_poly_struct,
        poly: *mut nmod_poly_struct,
        e: mpz_srcptr,
        f: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_powmod_ui_binexp_preinv(
        res: mp_ptr,
        poly: mp_srcptr,
        e: mp_limb_t,
        f: mp_srcptr,
        lenf: mp_limb_signed_t,
        finv: mp_srcptr,
        lenfinv: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_powmod_ui_binexp_preinv(
        res: *mut nmod_poly_struct,
        poly: *mut nmod_poly_struct,
        e: mp_limb_t,
        f: *mut nmod_poly_struct,
        finv: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_powmod_fmpz_binexp_preinv(
        res: mp_ptr,
        poly: mp_srcptr,
        e: *mut fmpz,
        f: mp_srcptr,
        lenf: mp_limb_signed_t,
        finv: mp_srcptr,
        lenfinv: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_powmod_fmpz_binexp_preinv(
        res: *mut nmod_poly_struct,
        poly: *mut nmod_poly_struct,
        e: *mut fmpz,
        f: *mut nmod_poly_struct,
        finv: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_powmod_x_ui_preinv(
        res: mp_ptr,
        e: mp_limb_t,
        f: mp_srcptr,
        lenf: mp_limb_signed_t,
        finv: mp_srcptr,
        lenfinv: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_powmod_x_ui_preinv(
        res: *mut nmod_poly_struct,
        e: mp_limb_t,
        f: *mut nmod_poly_struct,
        finv: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_powmod_mpz_binexp_preinv(
        res: mp_ptr,
        poly: mp_srcptr,
        e: mpz_srcptr,
        f: mp_srcptr,
        lenf: mp_limb_signed_t,
        finv: mp_srcptr,
        lenfinv: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_powmod_mpz_binexp_preinv(
        res: *mut nmod_poly_struct,
        poly: *mut nmod_poly_struct,
        e: mpz_srcptr,
        f: *mut nmod_poly_struct,
        finv: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_powmod_x_fmpz_preinv(
        res: mp_ptr,
        e: *mut fmpz,
        f: mp_srcptr,
        lenf: mp_limb_signed_t,
        finv: mp_srcptr,
        lenfinv: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_powmod_x_fmpz_preinv(
        res: *mut nmod_poly_struct,
        e: *mut fmpz,
        f: *mut nmod_poly_struct,
        finv: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_powers_mod_preinv_naive(
        res: *mut mp_ptr,
        f: mp_srcptr,
        flen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        g: mp_srcptr,
        glen: mp_limb_signed_t,
        ginv: mp_srcptr,
        ginvlen: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_powers_mod_naive(
        res: *mut nmod_poly_struct,
        f: *mut nmod_poly_struct,
        n: mp_limb_signed_t,
        g: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_powers_mod_preinv_threaded_pool(
        res: *mut mp_ptr,
        f: mp_srcptr,
        flen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        g: mp_srcptr,
        glen: mp_limb_signed_t,
        ginv: mp_srcptr,
        ginvlen: mp_limb_signed_t,
        mod_: nmod_t,
        threads: *mut thread_pool_handle,
        num_threads: mp_limb_signed_t,
    );
    pub fn _nmod_poly_powers_mod_preinv_threaded(
        res: *mut mp_ptr,
        f: mp_srcptr,
        flen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        g: mp_srcptr,
        glen: mp_limb_signed_t,
        ginv: mp_srcptr,
        ginvlen: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_powers_mod_bsgs(
        res: *mut nmod_poly_struct,
        f: *mut nmod_poly_struct,
        n: mp_limb_signed_t,
        g: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_divrem_basecase(
        Q: mp_ptr,
        R: mp_ptr,
        W: mp_ptr,
        A: mp_srcptr,
        A_len: mp_limb_signed_t,
        B: mp_srcptr,
        B_len: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_divrem_basecase(
        Q: *mut nmod_poly_struct,
        R: *mut nmod_poly_struct,
        A: *mut nmod_poly_struct,
        B: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_divrem_divconquer_recursive(
        Q: mp_ptr,
        BQ: mp_ptr,
        W: mp_ptr,
        V: mp_ptr,
        A: mp_srcptr,
        B: mp_srcptr,
        lenB: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn _nmod_poly_divrem_divconquer(
        Q: mp_ptr,
        R: mp_ptr,
        A: mp_srcptr,
        lenA: mp_limb_signed_t,
        B: mp_srcptr,
        lenB: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_divrem_divconquer(
        Q: *mut nmod_poly_struct,
        R: *mut nmod_poly_struct,
        A: *mut nmod_poly_struct,
        B: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_divrem_q0(
        Q: mp_ptr,
        R: mp_ptr,
        A: mp_srcptr,
        B: mp_srcptr,
        lenA: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn _nmod_poly_divrem_q1(
        Q: mp_ptr,
        R: mp_ptr,
        A: mp_srcptr,
        lenA: mp_limb_signed_t,
        B: mp_srcptr,
        lenB: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn _nmod_poly_divrem(
        Q: mp_ptr,
        R: mp_ptr,
        A: mp_srcptr,
        lenA: mp_limb_signed_t,
        B: mp_srcptr,
        lenB: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_divrem(
        Q: *mut nmod_poly_struct,
        R: *mut nmod_poly_struct,
        A: *mut nmod_poly_struct,
        B: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_div_basecase(
        Q: mp_ptr,
        W: mp_ptr,
        A: mp_srcptr,
        A_len: mp_limb_signed_t,
        B: mp_srcptr,
        B_len: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_div_basecase(
        Q: *mut nmod_poly_struct,
        A: *mut nmod_poly_struct,
        B: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_div_divconquer_recursive(
        Q: mp_ptr,
        W: mp_ptr,
        V: mp_ptr,
        A: mp_srcptr,
        B: mp_srcptr,
        lenB: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn _nmod_poly_div_divconquer(
        Q: mp_ptr,
        A: mp_srcptr,
        lenA: mp_limb_signed_t,
        B: mp_srcptr,
        lenB: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_div_divconquer(
        Q: *mut nmod_poly_struct,
        A: *mut nmod_poly_struct,
        B: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_div(
        Q: mp_ptr,
        A: mp_srcptr,
        lenA: mp_limb_signed_t,
        B: mp_srcptr,
        lenB: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_div(
        Q: *mut nmod_poly_struct,
        A: *mut nmod_poly_struct,
        B: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_rem_basecase(
        R: mp_ptr,
        W: mp_ptr,
        A: mp_srcptr,
        lenA: mp_limb_signed_t,
        B: mp_srcptr,
        lenB: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_rem_basecase(
        R: *mut nmod_poly_struct,
        A: *mut nmod_poly_struct,
        B: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_rem_q1(
        R: mp_ptr,
        A: mp_srcptr,
        lenA: mp_limb_signed_t,
        B: mp_srcptr,
        lenB: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn _nmod_poly_rem(
        R: mp_ptr,
        A: mp_srcptr,
        lenA: mp_limb_signed_t,
        B: mp_srcptr,
        lenB: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_rem(
        R: *mut nmod_poly_struct,
        A: *mut nmod_poly_struct,
        B: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_inv_series_basecase(
        Qinv: mp_ptr,
        Q: mp_srcptr,
        Qlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_inv_series_basecase(
        Qinv: *mut nmod_poly_struct,
        Q: *mut nmod_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_inv_series_newton(
        Qinv: mp_ptr,
        Q: mp_srcptr,
        Qlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_inv_series_newton(
        Qinv: *mut nmod_poly_struct,
        Q: *mut nmod_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_inv_series(
        Qinv: mp_ptr,
        Q: mp_srcptr,
        Qlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_inv_series(
        Qinv: *mut nmod_poly_struct,
        Q: *mut nmod_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_div_series_basecase(
        Q: mp_ptr,
        A: mp_srcptr,
        Alen: mp_limb_signed_t,
        B: mp_srcptr,
        Blen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_div_series_basecase(
        Q: *mut nmod_poly_struct,
        A: *mut nmod_poly_struct,
        B: *mut nmod_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_div_series(
        Q: mp_ptr,
        A: mp_srcptr,
        Alen: mp_limb_signed_t,
        B: mp_srcptr,
        Blen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_div_series(
        Q: *mut nmod_poly_struct,
        A: *mut nmod_poly_struct,
        B: *mut nmod_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_div_newton(
        Q: mp_ptr,
        A: mp_srcptr,
        Alen: mp_limb_signed_t,
        B: mp_srcptr,
        Blen: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_div_newton(
        Q: *mut nmod_poly_struct,
        A: *mut nmod_poly_struct,
        B: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_divrem_newton(
        Q: mp_ptr,
        R: mp_ptr,
        A: mp_srcptr,
        Alen: mp_limb_signed_t,
        B: mp_srcptr,
        Blen: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_divrem_newton(
        Q: *mut nmod_poly_struct,
        R: *mut nmod_poly_struct,
        A: *mut nmod_poly_struct,
        B: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_div_newton_n_preinv(
        Q: mp_ptr,
        A: mp_srcptr,
        lenA: mp_limb_signed_t,
        B: mp_srcptr,
        lenB: mp_limb_signed_t,
        Binv: mp_srcptr,
        lenBinv: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_div_newton_n_preinv(
        Q: *mut nmod_poly_struct,
        A: *mut nmod_poly_struct,
        B: *mut nmod_poly_struct,
        Binv: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_divrem_newton_n_preinv(
        Q: mp_ptr,
        R: mp_ptr,
        A: mp_srcptr,
        lenA: mp_limb_signed_t,
        B: mp_srcptr,
        lenB: mp_limb_signed_t,
        Binv: mp_srcptr,
        lenBinv: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_divrem_newton_n_preinv(
        Q: *mut nmod_poly_struct,
        R: *mut nmod_poly_struct,
        A: *mut nmod_poly_struct,
        B: *mut nmod_poly_struct,
        Binv: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_div_root(
        Q: mp_ptr,
        A: mp_srcptr,
        len: mp_limb_signed_t,
        c: mp_limb_t,
        mod_: nmod_t,
    ) -> mp_limb_t;
    pub fn nmod_poly_div_root(
        Q: *mut nmod_poly_struct,
        A: *mut nmod_poly_struct,
        c: mp_limb_t,
    ) -> mp_limb_t;
    pub fn _nmod_poly_derivative(
        x_prime: mp_ptr,
        x: mp_srcptr,
        len: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_derivative(x_prime: *mut nmod_poly_struct, x: *mut nmod_poly_struct);
    pub fn _nmod_poly_integral(x_int: mp_ptr, x: mp_srcptr, len: mp_limb_signed_t, mod_: nmod_t);
    pub fn nmod_poly_integral(x_int: *mut nmod_poly_struct, x: *mut nmod_poly_struct);
    pub fn _nmod_poly_evaluate_fmpz(
        rop: *mut fmpz,
        poly: mp_srcptr,
        len: mp_limb_signed_t,
        c: *mut fmpz,
    );
    pub fn nmod_poly_evaluate_fmpz(rop: *mut fmpz, poly: *mut nmod_poly_struct, c: *mut fmpz);
    pub fn _nmod_poly_evaluate_nmod(
        poly: mp_srcptr,
        len: mp_limb_signed_t,
        c: mp_limb_t,
        mod_: nmod_t,
    ) -> mp_limb_t;
    pub fn nmod_poly_evaluate_nmod(poly: *mut nmod_poly_struct, c: mp_limb_t) -> mp_limb_t;
    pub fn _nmod_poly_evaluate_nmod_vec(
        ys: mp_ptr,
        coeffs: mp_srcptr,
        len: mp_limb_signed_t,
        xs: mp_srcptr,
        n: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_evaluate_nmod_vec(
        ys: mp_ptr,
        poly: *mut nmod_poly_struct,
        xs: mp_srcptr,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_evaluate_nmod_vec_iter(
        ys: mp_ptr,
        coeffs: mp_srcptr,
        len: mp_limb_signed_t,
        xs: mp_srcptr,
        n: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_evaluate_nmod_vec_iter(
        ys: mp_ptr,
        poly: *mut nmod_poly_struct,
        xs: mp_srcptr,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_evaluate_nmod_vec_fast_precomp(
        vs: mp_ptr,
        poly: mp_srcptr,
        plen: mp_limb_signed_t,
        tree: *const mp_ptr,
        len: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn _nmod_poly_evaluate_nmod_vec_fast(
        ys: mp_ptr,
        coeffs: mp_srcptr,
        len: mp_limb_signed_t,
        xs: mp_srcptr,
        n: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_evaluate_nmod_vec_fast(
        ys: mp_ptr,
        poly: *mut nmod_poly_struct,
        xs: mp_srcptr,
        n: mp_limb_signed_t,
    );
    pub fn nmod_mat_one_addmul(dest: *mut nmod_mat_struct, mat: *mut nmod_mat_struct, c: mp_limb_t);
    pub fn nmod_poly_evaluate_mat_horner(
        dest: *mut nmod_mat_struct,
        poly: *mut nmod_poly_struct,
        c: *mut nmod_mat_struct,
    );
    pub fn nmod_poly_evaluate_mat_paterson_stockmeyer(
        dest: *mut nmod_mat_struct,
        poly: *mut nmod_poly_struct,
        c: *mut nmod_mat_struct,
    );
    pub fn nmod_poly_evaluate_mat(
        dest: *mut nmod_mat_struct,
        poly: *mut nmod_poly_struct,
        c: *mut nmod_mat_struct,
    );
    pub fn _nmod_poly_tree_alloc(len: mp_limb_signed_t) -> *mut mp_ptr;
    pub fn _nmod_poly_tree_free(tree: *mut mp_ptr, len: mp_limb_signed_t);
    pub fn _nmod_poly_tree_build(
        tree: *mut mp_ptr,
        roots: mp_srcptr,
        len: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn _nmod_poly_interpolate_nmod_vec_newton(
        poly: mp_ptr,
        xs: mp_srcptr,
        ys: mp_srcptr,
        n: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_interpolate_nmod_vec_newton(
        poly: *mut nmod_poly_struct,
        xs: mp_srcptr,
        ys: mp_srcptr,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_interpolate_nmod_vec_barycentric(
        poly: mp_ptr,
        xs: mp_srcptr,
        ys: mp_srcptr,
        n: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_interpolate_nmod_vec_barycentric(
        poly: *mut nmod_poly_struct,
        xs: mp_srcptr,
        ys: mp_srcptr,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_interpolate_nmod_vec(
        poly: mp_ptr,
        xs: mp_srcptr,
        ys: mp_srcptr,
        n: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_interpolate_nmod_vec(
        poly: *mut nmod_poly_struct,
        xs: mp_srcptr,
        ys: mp_srcptr,
        n: mp_limb_signed_t,
    );
    pub fn nmod_poly_interpolate_nmod_vec_fast(
        poly: *mut nmod_poly_struct,
        xs: mp_srcptr,
        ys: mp_srcptr,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_interpolate_nmod_vec_fast(
        poly: mp_ptr,
        xs: mp_srcptr,
        ys: mp_srcptr,
        len: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn _nmod_poly_interpolate_nmod_vec_fast_precomp(
        poly: mp_ptr,
        ys: mp_srcptr,
        tree: *const mp_ptr,
        weights: mp_srcptr,
        len: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn _nmod_poly_interpolation_weights(
        w: mp_ptr,
        tree: *const mp_ptr,
        len: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn _nmod_poly_compose_horner(
        res: mp_ptr,
        poly1: mp_srcptr,
        len1: mp_limb_signed_t,
        poly2: mp_srcptr,
        len2: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_compose_horner(
        res: *mut nmod_poly_struct,
        poly1: *mut nmod_poly_struct,
        poly2: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_compose_divconquer(
        res: mp_ptr,
        poly1: mp_srcptr,
        len1: mp_limb_signed_t,
        poly2: mp_srcptr,
        len2: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_compose_divconquer(
        res: *mut nmod_poly_struct,
        poly1: *mut nmod_poly_struct,
        poly2: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_compose(
        res: mp_ptr,
        poly1: mp_srcptr,
        len1: mp_limb_signed_t,
        poly2: mp_srcptr,
        len2: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_compose(
        res: *mut nmod_poly_struct,
        poly1: *mut nmod_poly_struct,
        poly2: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_taylor_shift_horner(
        poly: mp_ptr,
        c: mp_limb_t,
        len: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_taylor_shift_horner(
        g: *mut nmod_poly_struct,
        f: *mut nmod_poly_struct,
        c: mp_limb_t,
    );
    pub fn _nmod_poly_taylor_shift_convolution(
        poly: mp_ptr,
        c: mp_limb_t,
        len: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_taylor_shift_convolution(
        g: *mut nmod_poly_struct,
        f: *mut nmod_poly_struct,
        c: mp_limb_t,
    );
    pub fn _nmod_poly_taylor_shift(poly: mp_ptr, c: mp_limb_t, len: mp_limb_signed_t, mod_: nmod_t);
    pub fn nmod_poly_taylor_shift(g: *mut nmod_poly_struct, f: *mut nmod_poly_struct, c: mp_limb_t);
    pub fn _nmod_poly_compose_mod_brent_kung(
        res: mp_ptr,
        f: mp_srcptr,
        lenf: mp_limb_signed_t,
        g: mp_srcptr,
        h: mp_srcptr,
        lenh: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_compose_mod_brent_kung(
        res: *mut nmod_poly_struct,
        f: *mut nmod_poly_struct,
        g: *mut nmod_poly_struct,
        h: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_reduce_matrix_mod_poly(
        A: *mut nmod_mat_struct,
        B: *mut nmod_mat_struct,
        f: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_precompute_matrix(
        A: *mut nmod_mat_struct,
        poly1: mp_srcptr,
        poly2: mp_srcptr,
        len2: mp_limb_signed_t,
        poly2inv: mp_srcptr,
        len2inv: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn _nmod_poly_precompute_matrix_worker(arg_ptr: *mut c_void);
    pub fn nmod_poly_precompute_matrix(
        A: *mut nmod_mat_struct,
        poly1: *mut nmod_poly_struct,
        poly2: *mut nmod_poly_struct,
        poly2inv: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_compose_mod_brent_kung_precomp_preinv(
        res: mp_ptr,
        poly1: mp_srcptr,
        len1: mp_limb_signed_t,
        A: *mut nmod_mat_struct,
        poly3: mp_srcptr,
        len3: mp_limb_signed_t,
        poly3inv: mp_srcptr,
        len3inv: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn _nmod_poly_compose_mod_brent_kung_precomp_preinv_worker(arg_ptr: *mut c_void);
    pub fn nmod_poly_compose_mod_brent_kung_precomp_preinv(
        res: *mut nmod_poly_struct,
        poly1: *mut nmod_poly_struct,
        A: *mut nmod_mat_struct,
        poly3: *mut nmod_poly_struct,
        poly3inv: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_compose_mod_brent_kung_preinv(
        res: mp_ptr,
        poly1: mp_srcptr,
        len1: mp_limb_signed_t,
        poly2: mp_srcptr,
        poly3: mp_srcptr,
        len3: mp_limb_signed_t,
        poly3inv: mp_srcptr,
        len3inv: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_compose_mod_brent_kung_preinv(
        res: *mut nmod_poly_struct,
        poly1: *mut nmod_poly_struct,
        poly2: *mut nmod_poly_struct,
        poly3: *mut nmod_poly_struct,
        poly3inv: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_compose_mod_brent_kung_vec_preinv(
        res: *mut nmod_poly_struct,
        polys: *const nmod_poly_struct,
        len1: mp_limb_signed_t,
        l: mp_limb_signed_t,
        g: mp_srcptr,
        glen: mp_limb_signed_t,
        poly: mp_srcptr,
        len: mp_limb_signed_t,
        polyinv: mp_srcptr,
        leninv: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_compose_mod_brent_kung_vec_preinv(
        res: *mut nmod_poly_struct,
        polys: *const nmod_poly_struct,
        len1: mp_limb_signed_t,
        n: mp_limb_signed_t,
        g: *mut nmod_poly_struct,
        poly: *mut nmod_poly_struct,
        polyinv: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_compose_mod_brent_kung_vec_preinv_worker(arg_ptr: *mut c_void);
    pub fn nmod_poly_compose_mod_brent_kung_vec_preinv_threaded_pool(
        res: *mut nmod_poly_struct,
        polys: *const nmod_poly_struct,
        len1: mp_limb_signed_t,
        n: mp_limb_signed_t,
        g: *mut nmod_poly_struct,
        poly: *mut nmod_poly_struct,
        polyinv: *mut nmod_poly_struct,
        threads: *mut thread_pool_handle,
        num_threads: mp_limb_signed_t,
    );
    pub fn _nmod_poly_compose_mod_brent_kung_vec_preinv_threaded_pool(
        res: *mut nmod_poly_struct,
        polys: *const nmod_poly_struct,
        lenpolys: mp_limb_signed_t,
        l: mp_limb_signed_t,
        g: mp_srcptr,
        glen: mp_limb_signed_t,
        poly: mp_srcptr,
        len: mp_limb_signed_t,
        polyinv: mp_srcptr,
        leninv: mp_limb_signed_t,
        mod_: nmod_t,
        threads: *mut thread_pool_handle,
        num_threads: mp_limb_signed_t,
    );
    pub fn nmod_poly_compose_mod_brent_kung_vec_preinv_threaded(
        res: *mut nmod_poly_struct,
        polys: *const nmod_poly_struct,
        len1: mp_limb_signed_t,
        n: mp_limb_signed_t,
        g: *mut nmod_poly_struct,
        poly: *mut nmod_poly_struct,
        polyinv: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_compose_mod_horner(
        res: mp_ptr,
        f: mp_srcptr,
        lenf: mp_limb_signed_t,
        g: mp_srcptr,
        h: mp_srcptr,
        lenh: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_compose_mod_horner(
        res: *mut nmod_poly_struct,
        f: *mut nmod_poly_struct,
        g: *mut nmod_poly_struct,
        h: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_compose_mod(
        res: mp_ptr,
        f: mp_srcptr,
        lenf: mp_limb_signed_t,
        g: mp_srcptr,
        h: mp_srcptr,
        lenh: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_compose_mod(
        res: *mut nmod_poly_struct,
        f: *mut nmod_poly_struct,
        g: *mut nmod_poly_struct,
        h: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_compose_series_horner(
        res: mp_ptr,
        poly1: mp_srcptr,
        len1: mp_limb_signed_t,
        poly2: mp_srcptr,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_compose_series_horner(
        res: *mut nmod_poly_struct,
        poly1: *mut nmod_poly_struct,
        poly2: *mut nmod_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_compose_series_brent_kung(
        res: mp_ptr,
        poly1: mp_srcptr,
        len1: mp_limb_signed_t,
        poly2: mp_srcptr,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_compose_series_brent_kung(
        res: *mut nmod_poly_struct,
        poly1: *mut nmod_poly_struct,
        poly2: *mut nmod_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_compose_series(
        res: mp_ptr,
        poly1: mp_srcptr,
        len1: mp_limb_signed_t,
        poly2: mp_srcptr,
        len2: mp_limb_signed_t,
        n: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_compose_series(
        res: *mut nmod_poly_struct,
        poly1: *mut nmod_poly_struct,
        poly2: *mut nmod_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_revert_series_lagrange(
        Qinv: mp_ptr,
        Q: mp_srcptr,
        n: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_revert_series_lagrange(
        Qinv: *mut nmod_poly_struct,
        Q: *mut nmod_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_revert_series_lagrange_fast(
        Qinv: mp_ptr,
        Q: mp_srcptr,
        n: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_revert_series_lagrange_fast(
        Qinv: *mut nmod_poly_struct,
        Q: *mut nmod_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_revert_series_newton(
        Qinv: mp_ptr,
        Q: mp_srcptr,
        n: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_revert_series_newton(
        Qinv: *mut nmod_poly_struct,
        Q: *mut nmod_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_revert_series(Qinv: mp_ptr, Q: mp_srcptr, n: mp_limb_signed_t, mod_: nmod_t);
    pub fn nmod_poly_revert_series(
        Qinv: *mut nmod_poly_struct,
        Q: *mut nmod_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_compose_series_divconquer(
        res: mp_ptr,
        poly1: mp_srcptr,
        len1: mp_limb_signed_t,
        poly2: mp_srcptr,
        len2: mp_limb_signed_t,
        N: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_compose_series_divconquer(
        res: *mut nmod_poly_struct,
        poly1: *mut nmod_poly_struct,
        poly2: *mut nmod_poly_struct,
        N: mp_limb_signed_t,
    );
    pub fn _nmod_poly_gcd_euclidean(
        G: mp_ptr,
        A: mp_srcptr,
        lenA: mp_limb_signed_t,
        B: mp_srcptr,
        lenB: mp_limb_signed_t,
        mod_: nmod_t,
    ) -> mp_limb_signed_t;
    pub fn nmod_poly_gcd_euclidean(
        G: *mut nmod_poly_struct,
        A: *mut nmod_poly_struct,
        B: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_hgcd_recursive(
        M: *mut mp_ptr,
        lenM: *mut mp_limb_signed_t,
        A: mp_ptr,
        lenA: *mut mp_limb_signed_t,
        B: mp_ptr,
        lenB: *mut mp_limb_signed_t,
        a: mp_srcptr,
        lena: mp_limb_signed_t,
        b: mp_srcptr,
        lenb: mp_limb_signed_t,
        P: mp_ptr,
        mod_: nmod_t,
        flag: c_int,
        res: *mut nmod_poly_res_struct,
    ) -> mp_limb_signed_t;
    pub fn _nmod_poly_hgcd(
        M: *mut mp_ptr,
        lenM: *mut mp_limb_signed_t,
        A: mp_ptr,
        lenA: *mut mp_limb_signed_t,
        B: mp_ptr,
        lenB: *mut mp_limb_signed_t,
        a: mp_srcptr,
        lena: mp_limb_signed_t,
        b: mp_srcptr,
        lenb: mp_limb_signed_t,
        mod_: nmod_t,
    ) -> mp_limb_signed_t;
    pub fn nmod_poly_hgcd_ref(
        m11: *mut nmod_poly_struct,
        m12: *mut nmod_poly_struct,
        m21: *mut nmod_poly_struct,
        m22: *mut nmod_poly_struct,
        A: *mut nmod_poly_struct,
        B: *mut nmod_poly_struct,
        a: *mut nmod_poly_struct,
        b: *mut nmod_poly_struct,
    ) -> mp_limb_signed_t;
    pub fn nmod_poly_hgcd(
        m11: *mut nmod_poly_struct,
        m12: *mut nmod_poly_struct,
        m21: *mut nmod_poly_struct,
        m22: *mut nmod_poly_struct,
        A: *mut nmod_poly_struct,
        B: *mut nmod_poly_struct,
        a: *mut nmod_poly_struct,
        b: *mut nmod_poly_struct,
    ) -> mp_limb_signed_t;
    pub fn _nmod_poly_gcd_hgcd(
        G: mp_ptr,
        A: mp_srcptr,
        lenA: mp_limb_signed_t,
        B: mp_srcptr,
        lenB: mp_limb_signed_t,
        mod_: nmod_t,
    ) -> mp_limb_signed_t;
    pub fn nmod_poly_gcd_hgcd(
        G: *mut nmod_poly_struct,
        A: *mut nmod_poly_struct,
        B: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_gcd(
        G: mp_ptr,
        A: mp_srcptr,
        lenA: mp_limb_signed_t,
        B: mp_srcptr,
        lenB: mp_limb_signed_t,
        mod_: nmod_t,
    ) -> mp_limb_signed_t;
    pub fn nmod_poly_gcd(
        G: *mut nmod_poly_struct,
        A: *mut nmod_poly_struct,
        B: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_xgcd_euclidean(
        res: mp_ptr,
        s: mp_ptr,
        t: mp_ptr,
        poly1: mp_srcptr,
        len1: mp_limb_signed_t,
        poly2: mp_srcptr,
        len2: mp_limb_signed_t,
        mod_: nmod_t,
    ) -> mp_limb_signed_t;
    pub fn nmod_poly_xgcd_euclidean(
        G: *mut nmod_poly_struct,
        S: *mut nmod_poly_struct,
        T: *mut nmod_poly_struct,
        A: *mut nmod_poly_struct,
        B: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_xgcd_hgcd(
        G: mp_ptr,
        S: mp_ptr,
        T: mp_ptr,
        A: mp_srcptr,
        lenA: mp_limb_signed_t,
        B: mp_srcptr,
        lenB: mp_limb_signed_t,
        mod_: nmod_t,
    ) -> mp_limb_signed_t;
    pub fn nmod_poly_xgcd_hgcd(
        G: *mut nmod_poly_struct,
        S: *mut nmod_poly_struct,
        T: *mut nmod_poly_struct,
        A: *mut nmod_poly_struct,
        B: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_xgcd(
        G: mp_ptr,
        S: mp_ptr,
        T: mp_ptr,
        A: mp_srcptr,
        lenA: mp_limb_signed_t,
        B: mp_srcptr,
        lenB: mp_limb_signed_t,
        mod_: nmod_t,
    ) -> mp_limb_signed_t;
    pub fn nmod_poly_xgcd(
        G: *mut nmod_poly_struct,
        S: *mut nmod_poly_struct,
        T: *mut nmod_poly_struct,
        A: *mut nmod_poly_struct,
        B: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_resultant_euclidean(
        poly1: mp_srcptr,
        len1: mp_limb_signed_t,
        poly2: mp_srcptr,
        len2: mp_limb_signed_t,
        mod_: nmod_t,
    ) -> mp_limb_t;
    pub fn nmod_poly_resultant_euclidean(
        f: *mut nmod_poly_struct,
        g: *mut nmod_poly_struct,
    ) -> mp_limb_t;
    pub fn _nmod_poly_resultant_hgcd(
        A: mp_srcptr,
        lenA: mp_limb_signed_t,
        B: mp_srcptr,
        lenB: mp_limb_signed_t,
        mod_: nmod_t,
    ) -> mp_limb_t;
    pub fn nmod_poly_resultant_hgcd(
        A: *mut nmod_poly_struct,
        B: *mut nmod_poly_struct,
    ) -> mp_limb_t;
    pub fn _nmod_poly_resultant(
        poly1: mp_srcptr,
        len1: mp_limb_signed_t,
        poly2: mp_srcptr,
        len2: mp_limb_signed_t,
        mod_: nmod_t,
    ) -> mp_limb_t;
    pub fn nmod_poly_resultant(f: *mut nmod_poly_struct, g: *mut nmod_poly_struct) -> mp_limb_t;
    pub fn _nmod_poly_gcdinv(
        G: *mut mp_limb_t,
        S: *mut mp_limb_t,
        A: *const mp_limb_t,
        lenA: mp_limb_signed_t,
        B: *const mp_limb_t,
        lenB: mp_limb_signed_t,
        mod_: nmod_t,
    ) -> mp_limb_signed_t;
    pub fn nmod_poly_gcdinv(
        G: *mut nmod_poly_struct,
        S: *mut nmod_poly_struct,
        A: *mut nmod_poly_struct,
        B: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_discriminant(
        poly: mp_srcptr,
        len: mp_limb_signed_t,
        mod_: nmod_t,
    ) -> mp_limb_t;
    pub fn nmod_poly_discriminant(f: *mut nmod_poly_struct) -> mp_limb_t;
    pub fn _nmod_poly_invsqrt_series(g: mp_ptr, h: mp_srcptr, n: mp_limb_signed_t, mod_: nmod_t);
    pub fn nmod_poly_invsqrt_series(
        g: *mut nmod_poly_struct,
        h: *mut nmod_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_sqrt_series(g: mp_ptr, h: mp_srcptr, n: mp_limb_signed_t, mod_: nmod_t);
    pub fn nmod_poly_sqrt_series(
        g: *mut nmod_poly_struct,
        h: *mut nmod_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_sqrt(s: mp_ptr, p: mp_srcptr, len: mp_limb_signed_t, mod_: nmod_t) -> c_int;
    pub fn nmod_poly_sqrt(b: *mut nmod_poly_struct, a: *mut nmod_poly_struct) -> c_int;
    pub fn _nmod_poly_power_sums_naive(
        res: mp_ptr,
        poly: mp_srcptr,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_power_sums_naive(
        res: *mut nmod_poly_struct,
        poly: *mut nmod_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_power_sums_schoenhage(
        res: mp_ptr,
        poly: mp_srcptr,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_power_sums_schoenhage(
        res: *mut nmod_poly_struct,
        poly: *mut nmod_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_power_sums(
        res: mp_ptr,
        poly: mp_srcptr,
        len: mp_limb_signed_t,
        n: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_power_sums(
        res: *mut nmod_poly_struct,
        poly: *mut nmod_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_power_sums_to_poly_naive(
        res: mp_ptr,
        poly: mp_srcptr,
        len: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_power_sums_to_poly_naive(res: *mut nmod_poly_struct, Q: *mut nmod_poly_struct);
    pub fn _nmod_poly_power_sums_to_poly_schoenhage(
        res: mp_ptr,
        poly: mp_srcptr,
        len: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_power_sums_to_poly_schoenhage(
        res: *mut nmod_poly_struct,
        Q: *mut nmod_poly_struct,
    );
    pub fn _nmod_poly_power_sums_to_poly(
        res: mp_ptr,
        poly: mp_srcptr,
        len: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_power_sums_to_poly(res: *mut nmod_poly_struct, Q: *mut nmod_poly_struct);
    pub fn _nmod_poly_atan_series(g: mp_ptr, h: mp_srcptr, n: mp_limb_signed_t, mod_: nmod_t);
    pub fn nmod_poly_atan_series(
        g: *mut nmod_poly_struct,
        h: *mut nmod_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_tan_series(g: mp_ptr, h: mp_srcptr, n: mp_limb_signed_t, mod_: nmod_t);
    pub fn nmod_poly_tan_series(
        g: *mut nmod_poly_struct,
        h: *mut nmod_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_asin_series(g: mp_ptr, h: mp_srcptr, n: mp_limb_signed_t, mod_: nmod_t);
    pub fn nmod_poly_asin_series(
        g: *mut nmod_poly_struct,
        h: *mut nmod_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_sin_series(g: mp_ptr, h: mp_srcptr, n: mp_limb_signed_t, mod_: nmod_t);
    pub fn nmod_poly_sin_series(
        g: *mut nmod_poly_struct,
        h: *mut nmod_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_cos_series(g: mp_ptr, h: mp_srcptr, n: mp_limb_signed_t, mod_: nmod_t);
    pub fn nmod_poly_cos_series(
        g: *mut nmod_poly_struct,
        h: *mut nmod_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_asinh_series(g: mp_ptr, h: mp_srcptr, n: mp_limb_signed_t, mod_: nmod_t);
    pub fn nmod_poly_asinh_series(
        g: *mut nmod_poly_struct,
        h: *mut nmod_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_atanh_series(g: mp_ptr, h: mp_srcptr, n: mp_limb_signed_t, mod_: nmod_t);
    pub fn nmod_poly_atanh_series(
        g: *mut nmod_poly_struct,
        h: *mut nmod_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_sinh_series(g: mp_ptr, h: mp_srcptr, n: mp_limb_signed_t, mod_: nmod_t);
    pub fn nmod_poly_sinh_series(
        g: *mut nmod_poly_struct,
        h: *mut nmod_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_cosh_series(g: mp_ptr, h: mp_srcptr, n: mp_limb_signed_t, mod_: nmod_t);
    pub fn nmod_poly_cosh_series(
        g: *mut nmod_poly_struct,
        h: *mut nmod_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_tanh_series(g: mp_ptr, h: mp_srcptr, n: mp_limb_signed_t, mod_: nmod_t);
    pub fn nmod_poly_tanh_series(
        g: *mut nmod_poly_struct,
        h: *mut nmod_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_log_series_monomial_ui(
        res: mp_ptr,
        coeff: mp_limb_t,
        power: mp_limb_t,
        n: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_log_series_monomial_ui(
        res: *mut nmod_poly_struct,
        coeff: mp_limb_t,
        power: mp_limb_t,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_log_series(
        res: mp_ptr,
        f: mp_srcptr,
        flen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_log_series(
        res: *mut nmod_poly_struct,
        f: *mut nmod_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_exp_series_monomial_ui(
        res: mp_ptr,
        coeff: mp_limb_t,
        power: mp_limb_t,
        n: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_exp_series_monomial_ui(
        res: *mut nmod_poly_struct,
        coeff: mp_limb_t,
        power: mp_limb_t,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_exp_series_basecase(
        f: mp_ptr,
        h: mp_srcptr,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_exp_series_basecase(
        f: *mut nmod_poly_struct,
        h: *mut nmod_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_exp_expinv_series(
        f: mp_ptr,
        g: mp_ptr,
        h: mp_srcptr,
        n: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn _nmod_poly_exp_series(f: mp_ptr, h: mp_srcptr, n: mp_limb_signed_t, mod_: nmod_t);
    pub fn _nmod_poly_exp_series2(
        f: mp_ptr,
        h: mp_srcptr,
        hlen: mp_limb_signed_t,
        n: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn nmod_poly_exp_series(
        f: *mut nmod_poly_struct,
        h: *mut nmod_poly_struct,
        n: mp_limb_signed_t,
    );
    pub fn nmod_poly_product_roots_nmod_vec(
        poly: *mut nmod_poly_struct,
        xs: mp_srcptr,
        n: mp_limb_signed_t,
    );
    pub fn _nmod_poly_product_roots_nmod_vec(
        poly: mp_ptr,
        xs: mp_srcptr,
        n: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn _nmod_poly_split_rabin(
        a: *mut nmod_poly_struct,
        b: *mut nmod_poly_struct,
        f: *mut nmod_poly_struct,
        t: *mut nmod_poly_struct,
        t2: *mut nmod_poly_struct,
        randstate: *mut flint_rand_s,
    );
    pub fn nmod_poly_find_distinct_nonzero_roots(
        roots: *mut mp_limb_t,
        P: *mut nmod_poly_struct,
    ) -> c_int;
    pub fn nmod_poly_multi_crt_init(CRT: *mut nmod_poly_multi_crt_struct);
    pub fn nmod_poly_multi_crt_precompute(
        CRT: *mut nmod_poly_multi_crt_struct,
        moduli: *const nmod_poly_struct,
        len: mp_limb_signed_t,
    ) -> c_int;
    pub fn nmod_poly_multi_crt_precompute_p(
        CRT: *mut nmod_poly_multi_crt_struct,
        moduli: *const *const nmod_poly_struct,
        len: mp_limb_signed_t,
    ) -> c_int;
    pub fn nmod_poly_multi_crt_precomp(
        output: *mut nmod_poly_struct,
        CRT: *mut nmod_poly_multi_crt_struct,
        values: *const nmod_poly_struct,
    );
    pub fn nmod_poly_multi_crt_precomp_p(
        output: *mut nmod_poly_struct,
        CRT: *mut nmod_poly_multi_crt_struct,
        values: *const *const nmod_poly_struct,
    );
    pub fn nmod_poly_multi_crt(
        output: *mut nmod_poly_struct,
        moduli: *const nmod_poly_struct,
        values: *const nmod_poly_struct,
        len: mp_limb_signed_t,
    ) -> c_int;
    pub fn nmod_poly_multi_crt_clear(CRT: *mut nmod_poly_multi_crt_struct);
    pub fn _nmod_poly_multi_crt_local_size(
        CRT: *mut nmod_poly_multi_crt_struct,
    ) -> mp_limb_signed_t;
    pub fn _nmod_poly_multi_crt_run(
        outputs: *mut nmod_poly_struct,
        CRT: *mut nmod_poly_multi_crt_struct,
        inputs: *const nmod_poly_struct,
    );
    pub fn _nmod_poly_multi_crt_run_p(
        outputs: *mut nmod_poly_struct,
        CRT: *mut nmod_poly_multi_crt_struct,
        inputs: *const *const nmod_poly_struct,
    );
    pub fn nmod_poly_deflation(input: *mut nmod_poly_struct) -> mp_limb_t;
    pub fn nmod_poly_deflate(
        result: *mut nmod_poly_struct,
        input: *mut nmod_poly_struct,
        deflation: mp_limb_t,
    );
    pub fn nmod_poly_inflate(
        result: *mut nmod_poly_struct,
        input: *mut nmod_poly_struct,
        inflation: mp_limb_t,
    );
    pub fn nmod_mat_charpoly_danilevsky(p: *mut nmod_poly_struct, M: *mut nmod_mat_struct);
    pub fn nmod_mat_charpoly(p: *mut nmod_poly_struct, M: *mut nmod_mat_struct);
    pub fn nmod_mat_minpoly_with_gens(
        p: *mut nmod_poly_struct,
        X: *mut nmod_mat_struct,
        P: *mut mp_limb_t,
    );
    pub fn nmod_mat_minpoly(p: *mut nmod_poly_struct, M: *mut nmod_mat_struct);
    pub fn nmod_berlekamp_massey_init(B: *mut nmod_berlekamp_massey_struct, p: mp_limb_t);
    pub fn nmod_berlekamp_massey_start_over(B: *mut nmod_berlekamp_massey_struct);
    pub fn nmod_berlekamp_massey_clear(B: *mut nmod_berlekamp_massey_struct);
    pub fn nmod_berlekamp_massey_set_prime(B: *mut nmod_berlekamp_massey_struct, p: mp_limb_t);
    pub fn nmod_berlekamp_massey_print(B: *mut nmod_berlekamp_massey_struct);
    pub fn nmod_berlekamp_massey_add_points(
        B: *mut nmod_berlekamp_massey_struct,
        a: *const mp_limb_t,
        count: mp_limb_signed_t,
    );
    pub fn nmod_berlekamp_massey_add_zeros(
        B: *mut nmod_berlekamp_massey_struct,
        count: mp_limb_signed_t,
    );
    pub fn nmod_berlekamp_massey_add_point(B: *mut nmod_berlekamp_massey_struct, a: mp_limb_t);
    pub fn nmod_berlekamp_massey_reduce(B: *mut nmod_berlekamp_massey_struct) -> c_int;
    pub fn nmod_berlekamp_massey_points(B: *mut nmod_berlekamp_massey_struct) -> *const mp_limb_t;
    pub fn nmod_berlekamp_massey_point_count(
        B: *mut nmod_berlekamp_massey_struct,
    ) -> mp_limb_signed_t;
    pub fn nmod_berlekamp_massey_V_poly(
        B: *mut nmod_berlekamp_massey_struct,
    ) -> *const nmod_poly_struct;
    pub fn nmod_berlekamp_massey_R_poly(
        B: *mut nmod_berlekamp_massey_struct,
    ) -> *const nmod_poly_struct;
}
