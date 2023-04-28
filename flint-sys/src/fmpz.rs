#![allow(non_camel_case_types)]

//! See the [FLINT documentation](http://flintlib.org/doc/fmpz.html).

use crate::deps::*;
use crate::flint::*;
use libc::{c_char, c_int, size_t, FILE};

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Hash)]
pub struct fmpz(pub slong);

pub type fmpz_t = [fmpz; 1usize];
pub type fmpz_randstate_t = gmp_randstate_t;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_preinvn_struct {
    pub dinv: mp_ptr,
    pub n: mp_limb_signed_t,
    pub norm: mp_limb_t,
}

pub type fmpz_preinvn_t = [fmpz_preinvn_struct; 1usize];

/*
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_comb_struct {
    pub primes: *const mp_limb_t,
    pub num_primes: mp_limb_signed_t,
    pub n: mp_limb_signed_t,
    pub comb: *mut *mut fmpz,
    pub res: *mut *mut fmpz,
    pub mod_: *mut nmod_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_comb_temp_struct {
    pub n: mp_limb_signed_t,
    pub comb_temp: *mut *mut fmpz,
    pub temp: fmpz_t,
    pub temp2: fmpz_t,
}

pub type fmpz_comb_t = [fmpz_comb_struct; 1usize];
pub type fmpz_comb_temp_t = [fmpz_comb_temp_struct; 1usize];
*/

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _fmpz_multi_crt_prog_instr {
    pub a_idx: mp_limb_signed_t,
    pub b_idx: mp_limb_signed_t,
    pub c_idx: mp_limb_signed_t,
    pub idem: fmpz_t,
    pub modulus: fmpz_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_multi_crt_struct {
    pub prog: *mut _fmpz_multi_crt_prog_instr,
    pub length: mp_limb_signed_t,
    pub alloc: mp_limb_signed_t,
    pub localsize: mp_limb_signed_t,
    pub temp1loc: mp_limb_signed_t,
    pub temp2loc: mp_limb_signed_t,
    pub good: c_int,
}

pub type fmpz_multi_crt_t = [fmpz_multi_crt_struct; 1usize];

extern "C" {
    pub static mut fmpz_arr: *mut __mpz_struct;
    pub static mut fmpz_randstate: gmp_randstate_t;
    pub fn _fmpz_new_mpz() -> *mut __mpz_struct;
    pub fn _fmpz_clear_mpz(f: fmpz);
    pub fn _fmpz_cleanup_mpz_content();
    pub fn _fmpz_cleanup();
    pub fn _fmpz_promote(f: *mut fmpz) -> *mut __mpz_struct;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c._fmpz_promote_val) for this function.
    pub fn _fmpz_promote_val(f: *mut fmpz) -> *mut __mpz_struct;
    pub fn _fmpz_demote(f: *mut fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c._fmpz_demote_val) for this function.
    pub fn _fmpz_demote_val(f: *mut fmpz);
    pub fn _fmpz_init_readonly_mpz(f: *mut fmpz, z: *mut __mpz_struct);
    pub fn _fmpz_clear_readonly_mpz(arg1: *mut __mpz_struct);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_init) for this function.
    pub fn fmpz_init(f: *mut fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_init) for this function.
    pub fn fmpz_init2(f: *mut fmpz, limbs: mp_limb_t);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_init) for this function.
    pub fn fmpz_init_set(f: *mut fmpz, g: *const fmpz);
    pub fn fmpz_init_set_ui(f: *mut fmpz, g: mp_limb_t);
    pub fn fmpz_init_set_si(f: *mut fmpz, g: mp_limb_signed_t);
    pub fn fmpz_clear(f: *mut fmpz);
    pub fn fmpz_randbits(f: *mut fmpz, state: *const flint_rand_s, bits: mp_limb_t);
    pub fn fmpz_randm(f: *mut fmpz, state: *const flint_rand_s, m: *const fmpz);
    pub fn fmpz_randtest(f: *mut fmpz, state: *const flint_rand_s, bits: mp_limb_t);
    pub fn fmpz_randtest_unsigned(f: *mut fmpz, state: *const flint_rand_s, bits: mp_limb_t);
    pub fn fmpz_randtest_not_zero(f: *mut fmpz, state: *const flint_rand_s, bits: mp_limb_t);
    pub fn fmpz_randtest_mod(f: *mut fmpz, state: *const flint_rand_s, m: *const fmpz);
    pub fn fmpz_randtest_mod_signed(f: *mut fmpz, state: *const flint_rand_s, m: *const fmpz);
    pub fn fmpz_randprime(f: *mut fmpz, state: *const flint_rand_s, bits: mp_limb_t, proved: c_int);
    pub fn fmpz_get_si(f: *const fmpz) -> mp_limb_signed_t;
    pub fn fmpz_get_ui(f: *const fmpz) -> mp_limb_t;
    pub fn fmpz_get_uiui(hi: *mut mp_limb_t, low: *mut mp_limb_t, f: *const fmpz);
    pub fn fmpz_set_si(f: *mut fmpz, val: mp_limb_signed_t);
    pub fn fmpz_set_ui(f: *mut fmpz, val: mp_limb_t);
    pub fn fmpz_neg_ui(f: *mut fmpz, val: mp_limb_t);
    pub fn fmpz_set_uiui(f: *mut fmpz, hi: mp_limb_t, lo: mp_limb_t);
    pub fn fmpz_neg_uiui(f: *mut fmpz, hi: mp_limb_t, lo: mp_limb_t);
    pub fn fmpz_set_signed_uiui(r: *mut fmpz, hi: mp_limb_t, lo: mp_limb_t);
    pub fn fmpz_set_signed_uiuiui(r: *mut fmpz, hi: mp_limb_t, mid: mp_limb_t, lo: mp_limb_t);
    pub fn fmpz_set_ui_array(out: *mut fmpz, in_: *const mp_limb_t, in_len: mp_limb_signed_t);
    pub fn fmpz_get_ui_array(out: *mut mp_limb_t, out_len: mp_limb_signed_t, in_: *const fmpz);
    pub fn fmpz_get_mpz(x: *mut __mpz_struct, f: *const fmpz);
    pub fn fmpz_set_mpz(f: *mut fmpz, x: *const __mpz_struct);
    pub fn fmpz_get_d(f: *const fmpz) -> f64;
    pub fn fmpz_set_d(f: *mut fmpz, c: f64);
    pub fn fmpz_get_mpf(x: *mut __mpf_struct, f: *const fmpz);
    pub fn fmpz_set_mpf(f: *mut fmpz, x: *const __mpf_struct);
    pub fn fmpz_get_mpfr(x: *mut __mpfr_struct, f: *const fmpz, rnd: mpfr_rnd_t);
    pub fn fmpz_get_mpn(n: *mut mp_ptr, n_in: *const fmpz) -> c_int;
    pub fn fmpz_set_str(f: *mut fmpz, str_: *const c_char, b: c_int) -> c_int;
    pub fn flint_mpz_init_set_readonly(z: *mut __mpz_struct, f: *const fmpz);
    pub fn flint_mpz_clear_readonly(z: *mut __mpz_struct);
    pub fn fmpz_init_set_readonly(f: *mut fmpz, z: *const __mpz_struct);
    pub fn fmpz_clear_readonly(f: *mut fmpz);
    pub fn fmpz_abs_fits_ui(f: *const fmpz) -> c_int;
    pub fn fmpz_fits_si(f: *const fmpz) -> c_int;
    pub fn fmpz_zero(f: *mut fmpz);
    pub fn fmpz_one(f: *mut fmpz);
    pub fn fmpz_is_zero(f: *const fmpz) -> c_int;
    pub fn fmpz_is_one(f: *const fmpz) -> c_int;
    pub fn fmpz_is_pm1(f: *const fmpz) -> c_int;
    pub fn fmpz_set(f: *mut fmpz, g: *const fmpz);
    pub fn fmpz_equal(f: *const fmpz, g: *const fmpz) -> c_int;
    pub fn fmpz_equal_si(f: *const fmpz, g: mp_limb_signed_t) -> c_int;
    pub fn fmpz_equal_ui(f: *const fmpz, g: mp_limb_t) -> c_int;
    pub fn fmpz_read(f: *mut fmpz) -> c_int;
    pub fn fmpz_fread(file: *mut FILE, f: *mut fmpz) -> c_int;
    pub fn fmpz_inp_raw(x: *mut fmpz, fin: *mut FILE) -> size_t;
    pub fn fmpz_print(x: *const fmpz) -> c_int;
    pub fn fmpz_fprint(file: *mut FILE, x: *const fmpz) -> c_int;
    pub fn fmpz_out_raw(fout: *mut FILE, x: *const fmpz) -> size_t;
    pub fn fmpz_sizeinbase(f: *const fmpz, b: c_int) -> size_t;
    pub fn fmpz_get_str(str_: *mut c_char, b: c_int, f: *const fmpz) -> *mut c_char;
    pub fn fmpz_swap(f: *mut fmpz, g: *mut fmpz);
    pub fn fmpz_cmp(f: *const fmpz, g: *const fmpz) -> c_int;
    pub fn fmpz_cmp_ui(f: *const fmpz, g: mp_limb_t) -> c_int;
    pub fn fmpz_cmp_si(f: *const fmpz, g: mp_limb_signed_t) -> c_int;
    pub fn fmpz_cmpabs(f: *const fmpz, g: *const fmpz) -> c_int;
    pub fn fmpz_is_even(f: *const fmpz) -> c_int;
    pub fn fmpz_is_odd(f: *const fmpz) -> c_int;
    pub fn fmpz_size(f: *const fmpz) -> mp_size_t;
    pub fn fmpz_sgn(f: *const fmpz) -> c_int;
    pub fn fmpz_bits(f: *const fmpz) -> mp_limb_t;
    pub fn fmpz_val2(x: *const fmpz) -> mp_limb_t;
    pub fn fmpz_neg(f1: *mut fmpz, f2: *const fmpz);
    pub fn fmpz_abs(f1: *mut fmpz, f2: *const fmpz);
    pub fn fmpz_add(f: *mut fmpz, g: *const fmpz, h: *const fmpz);
    pub fn fmpz_sub(f: *mut fmpz, g: *const fmpz, h: *const fmpz);
    pub fn fmpz_mul_ui(f: *mut fmpz, g: *const fmpz, x: mp_limb_t);
    pub fn fmpz_mul_si(f: *mut fmpz, g: *const fmpz, x: mp_limb_signed_t);
    pub fn fmpz_mul(f: *mut fmpz, g: *const fmpz, h: *const fmpz);
    pub fn fmpz_mul_2exp(f: *mut fmpz, g: *const fmpz, exp: mp_limb_t);
    pub fn fmpz_add_ui(f: *mut fmpz, g: *const fmpz, x: mp_limb_t);
    pub fn fmpz_sub_ui(f: *mut fmpz, g: *const fmpz, x: mp_limb_t);
    pub fn fmpz_add_si(f: *mut fmpz, g: *const fmpz, x: mp_limb_signed_t);
    pub fn fmpz_sub_si(f: *mut fmpz, g: *const fmpz, x: mp_limb_signed_t);
    pub fn fmpz_addmul_ui(f: *mut fmpz, g: *const fmpz, x: mp_limb_t);
    pub fn fmpz_addmul_si(f: *mut fmpz, g: *const fmpz, x: mp_limb_signed_t);
    pub fn fmpz_submul_ui(f: *mut fmpz, g: *const fmpz, x: mp_limb_t);
    pub fn fmpz_submul_si(f: *mut fmpz, g: *const fmpz, x: mp_limb_signed_t);
    pub fn fmpz_addmul(f: *mut fmpz, g: *const fmpz, h: *const fmpz);
    pub fn fmpz_submul(f: *mut fmpz, g: *const fmpz, h: *const fmpz);
    pub fn fmpz_fmma(f: *mut fmpz, a: *const fmpz, b: *const fmpz, c: *const fmpz, d: *const fmpz);
    pub fn fmpz_fmms(f: *mut fmpz, a: *const fmpz, b: *const fmpz, c: *const fmpz, d: *const fmpz);
    pub fn fmpz_pow_ui(f: *mut fmpz, g: *const fmpz, exp: mp_limb_t);
    pub fn fmpz_pow_fmpz(a: *mut fmpz, b: *const fmpz, e: *const fmpz) -> c_int;
    pub fn fmpz_powm_ui(f: *mut fmpz, g: *const fmpz, exp: mp_limb_t, m: *const fmpz);
    pub fn fmpz_powm(f: *mut fmpz, g: *const fmpz, e: *const fmpz, m: *const fmpz);
    pub fn fmpz_setbit(f: *mut fmpz, i: mp_limb_t);
    pub fn fmpz_tstbit(f: *const fmpz, i: mp_limb_t) -> c_int;
    pub fn fmpz_clrbit(f: *mut fmpz, i: mp_limb_t);
    pub fn fmpz_complement(r: *mut fmpz, f: *const fmpz);
    pub fn fmpz_combit(f: *mut fmpz, i: mp_limb_t);
    pub fn fmpz_and(r: *mut fmpz, a: *const fmpz, b: *const fmpz);
    pub fn fmpz_or(r: *mut fmpz, a: *const fmpz, b: *const fmpz);
    pub fn fmpz_xor(r: *mut fmpz, a: *const fmpz, b: *const fmpz);
    pub fn fmpz_popcnt(c: *mut fmpz) -> mp_limb_t;
    pub fn fmpz_dlog(x: *const fmpz) -> f64;
    pub fn fmpz_flog(x: *const fmpz, b: *const fmpz) -> mp_limb_signed_t;
    pub fn fmpz_flog_ui(x: *const fmpz, b: mp_limb_t) -> mp_limb_signed_t;
    pub fn fmpz_clog(x: *const fmpz, b: *const fmpz) -> mp_limb_signed_t;
    pub fn fmpz_clog_ui(x: *const fmpz, b: mp_limb_t) -> mp_limb_signed_t;
    pub fn fmpz_sqrtmod(b: *mut fmpz, a: *const fmpz, p: *const fmpz) -> c_int;
    pub fn fmpz_sqrt(f: *mut fmpz, g: *const fmpz);
    pub fn fmpz_is_square(f: *const fmpz) -> c_int;
    pub fn fmpz_root(r: *mut fmpz, f: *const fmpz, n: mp_limb_signed_t);
    pub fn fmpz_is_perfect_power(root: *mut fmpz, f: *const fmpz) -> c_int;
    pub fn fmpz_sqrtrem(f: *mut fmpz, r: *mut fmpz, g: *const fmpz);
    pub fn fmpz_fdiv_ui(g: *const fmpz, h: mp_limb_t) -> mp_limb_t;
    pub fn fmpz_mod_ui(f: *mut fmpz, g: *const fmpz, h: mp_limb_t) -> mp_limb_t;
    pub fn fmpz_mod(f: *mut fmpz, g: *const fmpz, h: *const fmpz);
    pub fn fmpz_smod(f: *mut fmpz, g: *const fmpz, h: *const fmpz);
    pub fn fmpz_negmod(r: *mut fmpz, a: *const fmpz, mod_: *const fmpz);
    pub fn fmpz_gcd(f: *mut fmpz, g: *const fmpz, h: *const fmpz);
    pub fn fmpz_lcm(f: *mut fmpz, g: *const fmpz, h: *const fmpz);
    pub fn fmpz_gcdinv(d: *mut fmpz, a: *mut fmpz, f: *const fmpz, g: *const fmpz);
    pub fn fmpz_xgcd(d: *mut fmpz, a: *mut fmpz, b: *mut fmpz, f: *const fmpz, g: *const fmpz);
    pub fn fmpz_xgcd_canonical_bezout(
        d: *mut fmpz,
        a: *mut fmpz,
        b: *mut fmpz,
        f: *const fmpz,
        g: *const fmpz,
    );
    pub fn fmpz_xgcd_partial(
        co2: *mut fmpz,
        co1: *mut fmpz,
        r2: *mut fmpz,
        r1: *mut fmpz,
        L: *mut fmpz,
    );
    pub fn fmpz_invmod(f: *mut fmpz, g: *const fmpz, h: *const fmpz) -> c_int;
    pub fn fmpz_jacobi(a: *const fmpz, p: *const fmpz) -> c_int;
    pub fn fmpz_kronecker(a: *const fmpz, n: *const fmpz) -> c_int;
    pub fn fmpz_divides_mod_list(
        xstart: *mut fmpz,
        xstride: *mut fmpz,
        xlength: *mut fmpz,
        a: *const fmpz,
        b: *const fmpz,
        n: *const fmpz,
    );
    pub fn _fmpz_remove(x: *mut fmpz, f: *const fmpz, finv: f64) -> mp_limb_signed_t;
    pub fn fmpz_remove(rop: *mut fmpz, op: *const fmpz, f: *const fmpz) -> mp_limb_signed_t;
    pub fn fmpz_divexact(f: *mut fmpz, g: *const fmpz, h: *const fmpz);
    pub fn fmpz_divexact_si(f: *mut fmpz, g: *const fmpz, h: mp_limb_signed_t);
    pub fn fmpz_divexact_ui(f: *mut fmpz, g: *const fmpz, h: mp_limb_t);
    pub fn fmpz_divisible(f: *const fmpz, g: *const fmpz) -> c_int;
    pub fn fmpz_divisible_si(f: *const fmpz, g: mp_limb_signed_t) -> c_int;
    pub fn fmpz_cdiv_qr(f: *mut fmpz, s: *mut fmpz, g: *const fmpz, h: *const fmpz);
    pub fn fmpz_cdiv_q(f: *mut fmpz, g: *const fmpz, h: *const fmpz);
    pub fn fmpz_cdiv_q_si(f: *mut fmpz, g: *const fmpz, h: mp_limb_signed_t);
    pub fn fmpz_cdiv_q_ui(f: *mut fmpz, g: *const fmpz, h: mp_limb_t);
    pub fn fmpz_cdiv_q_2exp(f: *mut fmpz, g: *const fmpz, exp: mp_limb_t);
    pub fn fmpz_cdiv_r_2exp(f: *mut fmpz, g: *const fmpz, exp: mp_limb_t);
    pub fn fmpz_cdiv_ui(g: *const fmpz, h: mp_limb_t) -> mp_limb_t;
    pub fn fmpz_fdiv_qr(f: *mut fmpz, s: *mut fmpz, g: *const fmpz, h: *const fmpz);
    pub fn fmpz_fdiv_qr_preinvn(
        f: *mut fmpz,
        s: *mut fmpz,
        g: *mut fmpz,
        h: *mut fmpz,
        inv: *mut fmpz_preinvn_struct,
    );
    pub fn fmpz_fdiv_q(f: *mut fmpz, g: *const fmpz, h: *const fmpz);
    pub fn fmpz_fdiv_r(f: *mut fmpz, g: *const fmpz, h: *const fmpz);
    pub fn fmpz_fdiv_q_ui(f: *mut fmpz, g: *const fmpz, h: mp_limb_t);
    pub fn fmpz_fdiv_q_si(f: *mut fmpz, g: *const fmpz, h: mp_limb_signed_t);
    pub fn fmpz_fdiv_q_2exp(f: *mut fmpz, g: *const fmpz, exp: mp_limb_t);
    pub fn fmpz_fdiv_r_2exp(f: *mut fmpz, g: *const fmpz, exp: mp_limb_t);
    pub fn fmpz_tdiv_q(f: *mut fmpz, g: *const fmpz, h: *const fmpz);
    pub fn fmpz_tdiv_qr(f: *mut fmpz, s: *mut fmpz, g: *const fmpz, h: *const fmpz);
    pub fn fmpz_ndiv_qr(q: *mut fmpz, r: *mut fmpz, a: *const fmpz, b: *const fmpz);
    pub fn fmpz_tdiv_q_ui(f: *mut fmpz, g: *const fmpz, h: mp_limb_t);
    pub fn fmpz_tdiv_q_si(f: *mut fmpz, g: *const fmpz, h: mp_limb_signed_t);
    pub fn fmpz_tdiv_r_2exp(f: *mut fmpz, g: *const fmpz, exp: mp_limb_t);
    pub fn fmpz_tdiv_ui(g: *const fmpz, h: mp_limb_t) -> mp_limb_t;
    pub fn fmpz_tdiv_q_2exp(f: *mut fmpz, g: *const fmpz, exp: mp_limb_t);
    pub fn fmpz_preinvn_init(inv: *mut fmpz_preinvn_struct, f: *const fmpz);
    pub fn fmpz_preinvn_clear(inv: *mut fmpz_preinvn_struct);
    pub fn fmpz_get_d_2exp(exp: *const mp_limb_signed_t, f: *const fmpz) -> f64;
    pub fn fmpz_set_d_2exp(f: *mut fmpz, m: f64, exp: mp_limb_signed_t);
    pub fn fmpz_mul2_uiui(f: *mut fmpz, g: *const fmpz, h1: mp_limb_t, h2: mp_limb_t);
    pub fn fmpz_divexact2_uiui(f: *mut fmpz, g: *const fmpz, h1: mp_limb_t, h2: mp_limb_t);
    pub fn fmpz_mul_tdiv_q_2exp(f: *mut fmpz, g: *mut fmpz, h: *mut fmpz, exp: mp_limb_t);
    pub fn fmpz_mul_si_tdiv_q_2exp(f: *mut fmpz, g: *mut fmpz, x: mp_limb_signed_t, exp: mp_limb_t);
    pub fn fmpz_fac_ui(f: *mut fmpz, n: mp_limb_t);
    pub fn fmpz_fib_ui(f: *mut fmpz, n: mp_limb_t);
    pub fn fmpz_bin_uiui(res: *mut fmpz, n: mp_limb_t, k: mp_limb_t);
    pub fn _fmpz_rfac_ui(r: *mut fmpz, x: *const fmpz, a: mp_limb_t, b: mp_limb_t);
    pub fn fmpz_rfac_ui(r: *mut fmpz, x: *const fmpz, n: mp_limb_t);
    pub fn fmpz_rfac_uiui(r: *mut fmpz, x: mp_limb_t, n: mp_limb_t);
    pub fn fmpz_bit_pack(
        arr: mp_ptr,
        shift: mp_limb_t,
        bits: mp_limb_t,
        coeff: *mut fmpz,
        negate: c_int,
        borrow: c_int,
    ) -> c_int;
    pub fn fmpz_bit_unpack(
        coeff: *mut fmpz,
        arr: mp_srcptr,
        shift: mp_limb_t,
        bits: mp_limb_t,
        negate: c_int,
        borrow: c_int,
    ) -> c_int;
    pub fn fmpz_bit_unpack_unsigned(
        coeff: *mut fmpz,
        arr: mp_srcptr,
        shift: mp_limb_t,
        bits: mp_limb_t,
    );
    pub fn _fmpz_CRT_ui_precomp(
        out: *mut fmpz,
        r1: *mut fmpz,
        m1: *mut fmpz,
        r2: mp_limb_t,
        m2: mp_limb_t,
        m2inv: mp_limb_t,
        m1m2: *mut fmpz,
        c: mp_limb_t,
        sign: c_int,
    );
    pub fn fmpz_CRT_ui(
        out: *mut fmpz,
        r1: *const fmpz,
        m1: *const fmpz,
        r2: mp_limb_t,
        m2: mp_limb_t,
        sign: c_int,
    );
    /*
    pub fn fmpz_comb_temp_init(temp: *mut fmpz_comb_temp_struct, comb: *mut fmpz_comb_struct);
    pub fn fmpz_comb_temp_clear(temp: *mut fmpz_comb_temp_struct);
    pub fn fmpz_comb_init(
        comb: *mut fmpz_comb_struct,
        primes: mp_srcptr,
        num_primes: mp_limb_signed_t,
    );
    pub fn fmpz_comb_clear(comb: *mut fmpz_comb_struct);
    pub fn fmpz_multi_mod_ui(
        out: *mut mp_limb_t,
        in_: *mut fmpz,
        comb: *mut fmpz_comb_struct,
        temp: *mut fmpz_comb_temp_struct,
    );
    pub fn fmpz_multi_CRT_ui(
        output: *mut fmpz,
        residues: mp_srcptr,
        comb: *mut fmpz_comb_struct,
        temp: *mut fmpz_comb_temp_struct,
        sign: c_int,
    );
    */
    pub fn fmpz_CRT(
        out: *mut fmpz,
        r1: *const fmpz,
        m1: *const fmpz,
        r2: *const fmpz,
        m2: *const fmpz,
        sign: c_int,
    );
    pub fn fmpz_set_ui_smod(f: *mut fmpz, x: mp_limb_t, m: mp_limb_t);
    pub fn fmpz_multi_crt_init(CRT: *mut fmpz_multi_crt_struct);
    pub fn fmpz_multi_crt_precompute(
        CRT: *mut fmpz_multi_crt_struct,
        moduli: *const fmpz,
        len: mp_limb_signed_t,
    ) -> c_int;
    pub fn fmpz_multi_crt_precompute_p(
        CRT: *mut fmpz_multi_crt_struct,
        moduli: *const *const fmpz,
        len: mp_limb_signed_t,
    ) -> c_int;
    pub fn fmpz_multi_crt_precomp(
        output: *mut fmpz,
        P: *mut fmpz_multi_crt_struct,
        inputs: *const fmpz,
    );
    pub fn fmpz_multi_crt_precomp_p(
        output: *mut fmpz,
        P: *mut fmpz_multi_crt_struct,
        inputs: *const *const fmpz,
    );
    pub fn fmpz_multi_crt(
        output: *mut fmpz,
        moduli: *const fmpz,
        values: *const fmpz,
        len: mp_limb_signed_t,
    ) -> c_int;
    pub fn fmpz_multi_crt_clear(P: *mut fmpz_multi_crt_struct);
    pub fn _fmpz_multi_crt_local_size(CRT: *mut fmpz_multi_crt_struct) -> mp_limb_signed_t;
    pub fn _fmpz_multi_crt_run(
        outputs: *mut fmpz,
        CRT: *mut fmpz_multi_crt_struct,
        inputs: *const fmpz,
    );
    pub fn _fmpz_multi_crt_run_p(
        outputs: *mut fmpz,
        CRT: *mut fmpz_multi_crt_struct,
        inputs: *const *const fmpz,
    );
    pub fn fmpz_abs_ubound_ui_2exp(
        exp: *mut mp_limb_signed_t,
        x: *mut fmpz,
        bits: c_int,
    ) -> mp_limb_t;
    pub fn fmpz_abs_lbound_ui_2exp(
        exp: *mut mp_limb_signed_t,
        x: *mut fmpz,
        bits: c_int,
    ) -> mp_limb_t;
    pub fn fmpz_lucas_chain(
        Vm: *mut fmpz,
        Vm1: *mut fmpz,
        A: *mut fmpz,
        m: *mut fmpz,
        n: *mut fmpz,
    );
    pub fn fmpz_lucas_chain_full(
        Vm: *mut fmpz,
        Vm1: *mut fmpz,
        A: *mut fmpz,
        B: *mut fmpz,
        m: *mut fmpz,
        n: *mut fmpz,
    );
    pub fn fmpz_lucas_chain_double(
        U2m: *mut fmpz,
        U2m1: *mut fmpz,
        Um: *mut fmpz,
        Um1: *mut fmpz,
        A: *mut fmpz,
        B: *mut fmpz,
        n: *mut fmpz,
    );
    pub fn fmpz_lucas_chain_add(
        Umn: *mut fmpz,
        Umn1: *mut fmpz,
        Um: *mut fmpz,
        Um1: *mut fmpz,
        Un: *mut fmpz,
        Un1: *mut fmpz,
        A: *mut fmpz,
        B: *mut fmpz,
        n: *mut fmpz,
    );
    pub fn fmpz_lucas_chain_mul(
        Ukm: *mut fmpz,
        Ukm1: *mut fmpz,
        Um: *mut fmpz,
        Um1: *mut fmpz,
        A: *mut fmpz,
        B: *mut fmpz,
        k: *mut fmpz,
        n: *mut fmpz,
    );
    pub fn fmpz_lucas_chain_VtoU(
        Um: *mut fmpz,
        Um1: *mut fmpz,
        Vm: *mut fmpz,
        Vm1: *mut fmpz,
        A: *mut fmpz,
        B: *mut fmpz,
        Dinv: *mut fmpz,
        n: *mut fmpz,
    );
    pub fn fmpz_is_probabprime_lucas(n: *mut fmpz) -> c_int;
    pub fn fmpz_is_probabprime_BPSW(n: *mut fmpz) -> c_int;
    pub fn fmpz_is_strong_probabprime(n: *mut fmpz, a: *mut fmpz) -> c_int;
    pub fn fmpz_is_probabprime(p: *mut fmpz) -> c_int;
    pub fn fmpz_is_prime_pseudosquare(n: *mut fmpz) -> c_int;
    pub fn _fmpz_nm1_trial_factors(
        n: *mut fmpz,
        pm1: mp_ptr,
        num_pm1: *mut mp_limb_signed_t,
        limit: mp_limb_t,
    );
    pub fn fmpz_is_prime_pocklington(
        F: *mut fmpz,
        R: *mut fmpz,
        n: *mut fmpz,
        pm1: mp_ptr,
        num_pm1: mp_limb_signed_t,
    ) -> c_int;
    pub fn _fmpz_np1_trial_factors(
        n: *mut fmpz,
        pp1: mp_ptr,
        num_pp1: *mut mp_limb_signed_t,
        limit: mp_limb_t,
    );
    pub fn fmpz_is_prime_morrison(
        F: *mut fmpz,
        R: *mut fmpz,
        n: *mut fmpz,
        pm1: mp_ptr,
        num_pm1: mp_limb_signed_t,
    ) -> c_int;
    pub fn fmpz_is_prime(p: *const fmpz) -> c_int;
    pub fn fmpz_divisor_in_residue_class_lenstra(
        fac: *mut fmpz,
        n: *mut fmpz,
        r: *mut fmpz,
        s: *mut fmpz,
    ) -> c_int;
    pub fn fmpz_nextprime(res: *mut fmpz, n: *const fmpz, proved: c_int);
    pub fn fmpz_primorial(res: *mut fmpz, n: mp_limb_t);
    pub fn fmpz_euler_phi(res: *mut fmpz, n: *const fmpz);
    pub fn fmpz_moebius_mu(n: *const fmpz) -> c_int;
    pub fn fmpz_divisor_sigma(res: *mut fmpz, n: *const fmpz, k: mp_limb_t);
    pub fn n_powmod2_fmpz_preinv(
        a: mp_limb_t,
        exp: *mut fmpz,
        n: mp_limb_t,
        ninv: mp_limb_t,
    ) -> mp_limb_t;
    //pub fn nmod_pow_fmpz(a: mp_limb_t, exp: *mut fmpz, mod_: nmod_t) -> mp_limb_t;
    pub fn __new_fmpz() -> *mut fmpz;
    pub fn __free_fmpz(f: *mut fmpz);
    pub fn __fmpz_set_si(f: *mut fmpz, val: mp_limb_signed_t);
    pub fn __fmpz_set_ui(f: *mut fmpz, val: mp_limb_t);
    pub fn __fmpz_init(f: *mut fmpz);
    pub fn __fmpz_init_set_ui(f: *mut fmpz, g: mp_limb_t);
    pub fn __fmpz_clear(f: *mut fmpz);
    pub fn __fmpz_lt(f: *mut fmpz, g: *mut fmpz) -> c_int;
    pub fn __fmpz_gt(f: *mut fmpz, g: *mut fmpz) -> c_int;
    pub fn __fmpz_lte(f: *mut fmpz, g: *mut fmpz) -> c_int;
    pub fn __fmpz_gte(f: *mut fmpz, g: *mut fmpz) -> c_int;
    pub fn __fmpz_eq(f: *mut fmpz, g: *mut fmpz) -> c_int;
    pub fn __fmpz_neq(f: *mut fmpz, g: *mut fmpz) -> c_int;
    pub fn __fmpz_init_set(f: *mut fmpz, g: *mut fmpz);
    pub fn __fmpz_neg(f1: *mut fmpz, f2: *mut fmpz);
}

// Added to simplify operator macros
pub unsafe fn fmpz_tdiv_r(r: *mut fmpz, g: *const fmpz, h: *const fmpz) {
    let mut f: fmpz = fmpz::default();
    fmpz_init(&mut f);
    fmpz_tdiv_qr(&mut f, r, g, h);
    fmpz_clear(&mut f);
}
