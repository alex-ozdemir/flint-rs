#![allow(non_camel_case_types)]

//! See the [FLINT documentation](http://flintlib.org/doc/fmpq.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpz::fmpz;
use libc::{c_char, c_int, FILE};

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Hash)]
pub struct fmpq {
    pub num: fmpz,
    pub den: fmpz,
}

pub type fmpq_t = [fmpq; 1usize];

extern "C" {
    pub fn fmpq_init(x: *mut fmpq);
    pub fn fmpq_clear(x: *mut fmpq);
    pub fn fmpq_zero(res: *mut fmpq);
    pub fn fmpq_one(res: *mut fmpq);
    pub fn fmpq_equal(x: *const fmpq, y: *const fmpq) -> c_int;
    pub fn fmpq_sgn(x: *const fmpq) -> c_int;
    pub fn fmpq_is_zero(x: *const fmpq) -> c_int;
    pub fn fmpq_is_one(x: *const fmpq) -> c_int;
    pub fn fmpq_is_pm1(x: *const fmpq) -> c_int;
    pub fn fmpq_set(dest: *mut fmpq, src: *const fmpq);
    pub fn fmpq_swap(op1: *mut fmpq, op2: *mut fmpq);
    pub fn fmpq_neg(dest: *mut fmpq, src: *const fmpq);
    pub fn fmpq_abs(dest: *mut fmpq, src: *const fmpq);
    pub fn _fmpq_cmp(p: *const fmpz, q: *const fmpz, r: *const fmpz, s: *const fmpz) -> c_int;
    pub fn fmpq_cmp(x: *const fmpq, y: *const fmpq) -> c_int;
    pub fn _fmpq_cmp_fmpz(p: *const fmpz, q: *const fmpz, r: *const fmpz) -> c_int;
    pub fn fmpq_cmp_fmpz(x: *const fmpq, y: *const fmpz) -> c_int;
    pub fn _fmpq_cmp_ui(p: *const fmpz, q: *const fmpz, c: mp_limb_t) -> c_int;
    pub fn fmpq_cmp_ui(x: *const fmpq, c: mp_limb_t) -> c_int;
    pub fn _fmpq_cmp_si(p: *const fmpz, q: *const fmpz, c: mp_limb_signed_t) -> c_int;
    pub fn fmpq_cmp_si(x: *const fmpq, c: mp_limb_signed_t) -> c_int;
    pub fn _fmpq_canonicalise(num: *mut fmpz, den: *mut fmpz);
    pub fn fmpq_canonicalise(res: *mut fmpq);
    pub fn _fmpq_is_canonical(num: *const fmpz, den: *const fmpz) -> c_int;
    pub fn fmpq_is_canonical(x: *const fmpq) -> c_int;
    pub fn _fmpq_set_ui(rnum: *mut fmpz, rden: *mut fmpz, p: mp_limb_t, q: mp_limb_t);
    pub fn fmpq_set_ui(res: *mut fmpq, p: mp_limb_t, q: mp_limb_t);
    pub fn _fmpq_set_si(rnum: *mut fmpz, rden: *mut fmpz, p: mp_limb_signed_t, q: mp_limb_t);
    pub fn fmpq_set_si(res: *mut fmpq, p: mp_limb_signed_t, q: mp_limb_t);
    pub fn fmpq_equal_ui(q: *const fmpq, n: mp_limb_t) -> c_int;
    pub fn fmpq_equal_si(q: *const fmpq, n: mp_limb_signed_t) -> c_int;
    pub fn fmpq_set_fmpz_frac(res: *mut fmpq, p: *const fmpz, q: *const fmpz);
    pub fn fmpq_set_str(res: *mut fmpq, str_: *const c_char, base: c_int) -> c_int;
    pub fn fmpq_set_mpq(dest: *mut fmpq, src: *const __mpq_struct);
    pub fn fmpq_get_mpq(dest: *mut __mpq_struct, src: *const fmpq);
    pub fn fmpq_get_d(a: *const fmpq) -> f64;
    pub fn fmpq_get_mpfr(r: *mut __mpfr_struct, x: *const fmpq, rnd: mpfr_rnd_t) -> c_int;
    pub fn fmpq_get_mpz_frac(a: *mut __mpz_struct, b: *mut __mpz_struct, c: *const fmpq);
    pub fn flint_mpq_init_set_readonly(z: *mut __mpq_struct, f: *mut fmpq);
    pub fn flint_mpq_clear_readonly(z: *mut __mpq_struct);
    pub fn fmpq_init_set_readonly(f: *mut fmpq, z: *mut __mpq_struct);
    pub fn fmpq_clear_readonly(f: *mut fmpq);
    pub fn fmpq_init_set_mpz_frac_readonly(
        z: *mut fmpq,
        num: *mut __mpz_struct,
        den: *mut __mpz_struct,
    );
    pub fn _fmpq_get_str(
        str_: *mut c_char,
        b: c_int,
        num: *const fmpz,
        den: *const fmpz,
    ) -> *mut c_char;
    pub fn fmpq_get_str(str_: *mut c_char, b: c_int, x: *const fmpq) -> *mut c_char;
    pub fn _fmpq_fprint(file: *mut FILE, num: *const fmpz, den: *const fmpz) -> c_int;
    pub fn fmpq_fprint(file: *mut FILE, x: *const fmpq) -> c_int;
    pub fn _fmpq_print(num: *const fmpz, den: *const fmpz) -> c_int;
    pub fn fmpq_print(x: *const fmpq) -> c_int;
    pub fn _fmpq_randtest(
        num: *mut fmpz,
        den: *mut fmpz,
        state: *mut flint_rand_s,
        bits: mp_limb_t,
    );
    pub fn fmpq_randtest(res: *mut fmpq, state: *mut flint_rand_s, bits: mp_limb_t);
    pub fn fmpq_randtest_not_zero(res: *mut fmpq, state: *mut flint_rand_s, bits: mp_limb_t);
    pub fn _fmpq_randbits(
        num: *mut fmpz,
        den: *mut fmpz,
        state: *mut flint_rand_s,
        bits: mp_limb_t,
    );
    pub fn fmpq_randbits(res: *mut fmpq, state: *mut flint_rand_s, bits: mp_limb_t);
    pub fn _fmpq_add_small(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        p1: mp_limb_signed_t,
        q1: mp_limb_t,
        p2: mp_limb_signed_t,
        q2: mp_limb_t,
    );
    pub fn _fmpq_mul_small(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        p1: mp_limb_signed_t,
        q1: mp_limb_t,
        p2: mp_limb_signed_t,
        q2: mp_limb_t,
    );
    pub fn _fmpq_add(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        op1num: *const fmpz,
        op1den: *const fmpz,
        op2num: *const fmpz,
        op2den: *const fmpz,
    );
    pub fn fmpq_add(res: *mut fmpq, op1: *const fmpq, op2: *const fmpq);
    pub fn _fmpq_add_si(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        p: *const fmpz,
        q: *const fmpz,
        r: mp_limb_signed_t,
    );
    pub fn fmpq_add_si(res: *mut fmpq, op1: *const fmpq, c: mp_limb_signed_t);
    pub fn _fmpq_add_ui(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        p: *const fmpz,
        q: *const fmpz,
        r: mp_limb_t,
    );
    pub fn fmpq_add_ui(res: *mut fmpq, op1: *const fmpq, c: mp_limb_t);
    pub fn _fmpq_add_fmpz(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        p: *const fmpz,
        q: *const fmpz,
        r: *const fmpz,
    );
    pub fn fmpq_add_fmpz(res: *mut fmpq, op1: *const fmpq, c: *const fmpz);
    pub fn _fmpq_sub(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        op1num: *const fmpz,
        op1den: *const fmpz,
        op2num: *const fmpz,
        op2den: *const fmpz,
    );
    pub fn fmpq_sub(res: *mut fmpq, op1: *const fmpq, op2: *const fmpq);
    pub fn _fmpq_sub_si(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        p: *const fmpz,
        q: *const fmpz,
        r: mp_limb_signed_t,
    );
    pub fn fmpq_sub_si(res: *mut fmpq, op1: *const fmpq, c: mp_limb_signed_t);
    pub fn _fmpq_sub_ui(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        p: *const fmpz,
        q: *const fmpz,
        r: mp_limb_t,
    );
    pub fn fmpq_sub_ui(res: *mut fmpq, op1: *const fmpq, c: mp_limb_t);
    pub fn _fmpq_sub_fmpz(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        p: *const fmpz,
        q: *const fmpz,
        r: *const fmpz,
    );
    pub fn fmpq_sub_fmpz(res: *mut fmpq, op1: *const fmpq, c: *const fmpz);
    pub fn _fmpq_mul_si(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        p: *const fmpz,
        q: *const fmpz,
        r: mp_limb_signed_t,
    );
    pub fn fmpq_mul_si(res: *mut fmpq, op1: *const fmpq, c: mp_limb_signed_t);
    pub fn _fmpq_mul_ui(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        p: *const fmpz,
        q: *const fmpz,
        r: mp_limb_t,
    );
    pub fn fmpq_mul_ui(res: *mut fmpq, op1: *const fmpq, c: mp_limb_t);
    pub fn _fmpq_mul(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        op1num: *const fmpz,
        op1den: *const fmpz,
        op2num: *const fmpz,
        op2den: *const fmpz,
    );
    pub fn fmpq_mul(res: *mut fmpq, op1: *const fmpq, op2: *const fmpq);
    pub fn fmpq_mul_fmpz(res: *mut fmpq, op: *const fmpq, x: *const fmpz);
    pub fn _fmpq_pow_si(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        opnum: *const fmpz,
        opden: *const fmpz,
        e: mp_limb_signed_t,
    );
    pub fn fmpq_pow_si(rop: *mut fmpq, op: *const fmpq, e: mp_limb_signed_t);
    pub fn fmpq_pow_fmpz(a: *mut fmpq, b: *const fmpq, e: *const fmpz) -> c_int;
    pub fn _fmpq_addmul(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        op1num: *const fmpz,
        op1den: *const fmpz,
        op2num: *const fmpz,
        op2den: *const fmpz,
    );
    pub fn fmpq_addmul(res: *mut fmpq, op1: *const fmpq, op2: *const fmpq);
    pub fn _fmpq_submul(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        op1num: *const fmpz,
        op1den: *const fmpz,
        op2num: *const fmpz,
        op2den: *const fmpz,
    );
    pub fn fmpq_submul(res: *mut fmpq, op1: *const fmpq, op2: *const fmpq);
    pub fn fmpq_inv(dest: *mut fmpq, src: *const fmpq);
    pub fn _fmpq_div(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        op1num: *const fmpz,
        op1den: *const fmpz,
        op2num: *const fmpz,
        op2den: *const fmpz,
    );
    pub fn fmpq_div(res: *mut fmpq, op1: *const fmpq, op2: *const fmpq);
    pub fn fmpq_div_fmpz(res: *mut fmpq, op: *const fmpq, x: *const fmpz);
    pub fn fmpq_mul_2exp(res: *mut fmpq, x: *const fmpq, exp: mp_limb_t);
    pub fn fmpq_div_2exp(res: *mut fmpq, x: *const fmpq, exp: mp_limb_t);
    pub fn _fmpq_mod_fmpz(
        res: *mut fmpz,
        num: *const fmpz,
        den: *const fmpz,
        mod_: *const fmpz,
    ) -> c_int;
    pub fn fmpq_mod_fmpz(res: *mut fmpz, x: *const fmpq, mod_: *const fmpz) -> c_int;
    pub fn _fmpq_gcd(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        p: *const fmpz,
        q: *const fmpz,
        r: *const fmpz,
        s: *const fmpz,
    );
    pub fn fmpq_gcd(res: *mut fmpq, op1: *const fmpq, op2: *const fmpq);
    pub fn _fmpq_gcd_cofactors(
        ng: *mut fmpz,
        dg: *mut fmpz,
        A: *mut fmpz,
        B: *mut fmpz,
        na: *const fmpz,
        da: *const fmpz,
        nb: *const fmpz,
        db: *const fmpz,
    );
    pub fn fmpq_gcd_cofactors(
        g: *mut fmpq,
        A: *mut fmpz,
        B: *mut fmpz,
        a: *const fmpq,
        b: *const fmpq,
    );
    pub fn _fmpq_reconstruct_fmpz(
        num: *mut fmpz,
        den: *mut fmpz,
        a: *const fmpz,
        m: *const fmpz,
    ) -> c_int;
    pub fn fmpq_reconstruct_fmpz(res: *mut fmpq, a: *const fmpz, m: *const fmpz) -> c_int;
    pub fn _fmpq_reconstruct_fmpz_2_naive(
        n: *mut fmpz,
        d: *mut fmpz,
        a: *const fmpz,
        m: *const fmpz,
        N: *const fmpz,
        D: *const fmpz,
    ) -> c_int;
    pub fn _fmpq_reconstruct_fmpz_2(
        n: *mut fmpz,
        d: *mut fmpz,
        a: *const fmpz,
        m: *const fmpz,
        N: *const fmpz,
        D: *const fmpz,
    ) -> c_int;
    pub fn fmpq_reconstruct_fmpz_2(
        res: *mut fmpq,
        a: *const fmpz,
        m: *const fmpz,
        N: *const fmpz,
        D: *const fmpz,
    ) -> c_int;
    pub fn fmpq_height_bits(x: *const fmpq) -> mp_limb_t;
    pub fn fmpq_height(height: *mut fmpz, x: *const fmpq);
    pub fn _fmpq_next_calkin_wilf(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        num: *const fmpz,
        den: *const fmpz,
    );
    pub fn fmpq_next_calkin_wilf(res: *mut fmpq, x: *const fmpq);
    pub fn _fmpq_next_signed_calkin_wilf(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        num: *const fmpz,
        den: *const fmpz,
    );
    pub fn fmpq_next_signed_calkin_wilf(res: *mut fmpq, x: *const fmpq);
    pub fn _fmpq_next_minimal(rnum: *mut fmpz, rden: *mut fmpz, num: *const fmpz, den: *const fmpz);
    pub fn fmpq_next_minimal(res: *mut fmpq, x: *const fmpq);
    pub fn _fmpq_next_signed_minimal(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        num: *const fmpz,
        den: *const fmpz,
    );
    pub fn fmpq_next_signed_minimal(res: *mut fmpq, x: *const fmpq);
    pub fn fmpq_farey_neighbors(left: *mut fmpq, right: *mut fmpq, mid: *mut fmpq, Q: *const fmpz);
    pub fn fmpq_simplest_between(mid: *mut fmpq, l: *const fmpq, r: *const fmpq);
    pub fn _fmpq_simplest_between(
        mid_num: *mut fmpz,
        mid_den: *mut fmpz,
        l_num: *const fmpz,
        l_den: *const fmpz,
        r_num: *const fmpz,
        r_den: *const fmpz,
    );
    pub fn fmpq_get_cfrac_naive(
        c: *mut fmpz,
        rem: *mut fmpq,
        x: *const fmpq,
        n: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn fmpq_get_cfrac(
        c: *mut fmpz,
        rem: *mut fmpq,
        x: *const fmpq,
        n: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn fmpq_set_cfrac(x: *mut fmpq, c: *const fmpz, n: mp_limb_signed_t);
    pub fn fmpq_cfrac_bound(x: *mut fmpq) -> mp_limb_signed_t;
    pub fn fmpq_dedekind_sum_naive(s: *mut fmpq, h: *const fmpz, k: *const fmpz);
    pub fn fmpq_dedekind_sum(s: *mut fmpq, h: *const fmpz, k: *const fmpz);
    pub fn _fmpq_harmonic_ui(num: *mut fmpz, den: *mut fmpz, n: mp_limb_t);
    pub fn fmpq_harmonic_ui(x: *mut fmpq, n: mp_limb_t);
    pub fn _fmpq_vec_init(len: mp_limb_signed_t) -> *mut fmpq;
    pub fn _fmpq_vec_clear(vec: *mut fmpq, len: mp_limb_signed_t);
}

pub unsafe fn fmpq_set_ui_den1(res: *mut fmpq, num: mp_limb_t) {
    fmpq_set_ui(res, num, 1 as mp_limb_t);
}

pub unsafe fn fmpq_set_si_den1(res: *mut fmpq, num: mp_limb_signed_t) {
    fmpq_set_si(res, num, 1 as mp_limb_t);
}

pub unsafe fn fmpq_set_fmpz_den1(res: *mut fmpq, num: *const fmpz) {
    fmpq_set_fmpz_frac(res, num, &fmpz(1) as *const fmpz);
}
