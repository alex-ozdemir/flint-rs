#![allow(non_camel_case_types)]

use gmp_mpfr_sys::{gmp, mpfr};
use libc::{c_char, c_double, c_int, c_long, c_ulong, FILE};

//include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
#[repr(C)]
#[derive(Default, Debug)]
pub struct fmpz(slong);

type slong = c_long;
type ulong = c_ulong;
type mp_limb_t = c_ulong;

#[repr(C)]
pub struct fmpz_mod_poly {
    coeffs: *mut fmpz,
    alloc: slong,
    length: slong,
    p: fmpz,
}

#[repr(C)]
pub struct fmpz_poly {
    coeffs: *mut fmpz,
    alloc: slong,
    length: slong,
}

#[repr(C)]
pub struct flint_rand {
    gmp_rand: gmp::randstate_t,
    gmp_init: c_int,
    randval: ulong,
    randval2: ulong,
}

extern "C" {
    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_init) for this function.
    #[link_name = "__fmpz_init"]
    pub fn fmpz_init(f: *mut fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_init2) for this function.
    #[link_name = "fmpz_init2"]
    pub fn fmpz_init2(f: *mut fmpz, limbs: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_clear) for this function.
    #[link_name = "__fmpz_clear"]
    pub fn fmpz_clear(f: *mut fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_init_set) for this function.
    #[link_name = "__fmpz_init_set"]
    pub fn fmpz_init_set(f: *mut fmpz, g: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_init_set_ui) for this function.
    #[link_name = "__fmpz_init_set_ui"]
    pub fn fmpz_init_set_ui(f: *mut fmpz, g: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_init_set_si) for this function.
    #[link_name = "fmpz_init_set_si"]
    pub fn fmpz_init_set_si(f: *mut fmpz, g: slong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_randbits) for this function.
    #[link_name = "fmpz_randbits"]
    pub fn fmpz_randbits(f: *mut fmpz, state: *mut flint_rand, bits: usize);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_randtest) for this function.
    #[link_name = "fmpz_randtest"]
    pub fn fmpz_randtest(f: *mut fmpz, state: *mut flint_rand, bits: usize);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_randtest_unsigned) for this function.
    #[link_name = "fmpz_randtest_unsigned"]
    pub fn fmpz_randtest_unsigned(f: *mut fmpz, state: *mut flint_rand, bits: usize);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_randtest_not_zero) for this function.
    #[link_name = "fmpz_randtest_not_zero"]
    pub fn fmpz_randtest_not_zero(f: *mut fmpz, state: *mut flint_rand, bits: usize);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_randm) for this function.
    #[link_name = "fmpz_randm"]
    pub fn fmpz_randm(f: *mut fmpz, state: *mut flint_rand, m: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_randtest_mod) for this function.
    #[link_name = "fmpz_randtest_mod"]
    pub fn fmpz_randtest_mod(f: *mut fmpz, state: *mut flint_rand, m: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_randtest_mod_signed) for this function.
    #[link_name = "fmpz_randtest_mod_signed"]
    pub fn fmpz_randtest_mod_signed(f: *mut fmpz, state: *mut flint_rand, m: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_randprime) for this function.
    #[link_name = "fmpz_randprime"]
    pub fn fmpz_randprime(f: *mut fmpz, state: *mut flint_rand, bits: usize, proved: c_int);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_get_si) for this function.
    #[link_name = "fmpz_get_si"]
    pub fn fmpz_get_si(f: *const fmpz) -> slong;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_get_ui) for this function.
    #[link_name = "fmpz_get_ui"]
    pub fn fmpz_get_ui(f: *const fmpz) -> ulong;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_get_uiui) for this function.
    #[link_name = "fmpz_get_uiui"]
    pub fn fmpz_get_uiui(hi: *mut mp_limb_t, low: *mut mp_limb_t, f: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_get_d) for this function.
    #[link_name = "fmpz_get_d"]
    pub fn fmpz_get_d(f: *const fmpz) -> c_double;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_set_mpf) for this function.
    #[link_name = "fmpz_set_mpf"]
    pub fn fmpz_set_mpf(f: *mut fmpz, x: *const gmp::mpf_t);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_get_mpf) for this function.
    #[link_name = "fmpz_get_mpf"]
    pub fn fmpz_get_mpf(x: *mut gmp::mpf_t, f: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_get_mpfr) for this function.
    #[link_name = "fmpz_get_mpfr"]
    pub fn fmpz_get_mpfr(x: *mut mpfr::mpfr_t, f: *const fmpz, rnd: mpfr::rnd_t);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_get_d_2exp) for this function.
    #[link_name = "fmpz_get_d_2exp"]
    pub fn fmpz_get_d_2exp(exp: *const isize, f: *const fmpz) -> c_double;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_get_mpz) for this function.
    #[link_name = "fmpz_get_mpz"]
    pub fn fmpz_get_mpz(x: *mut gmp::mpz_t, f: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_get_mpn) for this function.
    #[link_name = "fmpz_get_mpn"]
    pub fn fmpz_get_mpn(n: *mut *const mp_limb_t, n_in: *const fmpz) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_get_str) for this function.
    #[link_name = "fmpz_get_str"]
    pub fn fmpz_get_str(str: *mut c_char, b: c_int, f: *const fmpz) -> *mut c_char;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_set_si) for this function.
    #[link_name = "__fmpz_set_si"]
    pub fn fmpz_set_si(f: *mut fmpz, val: slong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_set_ui) for this function.
    #[link_name = "__fmpz_set_ui"]
    pub fn fmpz_set_ui(f: *mut fmpz, val: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_set_d) for this function.
    #[link_name = "fmpz_set_d"]
    pub fn fmpz_set_d(f: *mut fmpz, c: c_double);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_set_d_2exp) for this function.
    #[link_name = "fmpz_set_d_2exp"]
    pub fn fmpz_set_d_2exp(f: *mut fmpz, d: c_double, exp: slong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_neg_ui) for this function.
    #[link_name = "fmpz_neg_ui"]
    pub fn fmpz_neg_ui(f: *mut fmpz, val: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_set_uiui) for this function.
    #[link_name = "fmpz_set_uiui"]
    pub fn fmpz_set_uiui(f: *mut fmpz, hi: mp_limb_t, lo: mp_limb_t);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_neg_uiui) for this function.
    #[link_name = "fmpz_neg_uiui"]
    pub fn fmpz_neg_uiui(f: *mut fmpz, hi: mp_limb_t, lo: mp_limb_t);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_set_signed_uiui) for this function.
    #[link_name = "fmpz_set_signed_uiui"]
    pub fn fmpz_set_signed_uiui(f: *mut fmpz, hi: ulong, lo: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_set_signed_uiuiui) for this function.
    #[link_name = "fmpz_set_signed_uiuiui"]
    pub fn fmpz_set_signed_uiuiui(f: *mut fmpz, hi: ulong, mid: ulong, lo: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_set_ui_array) for this function.
    #[link_name = "fmpz_set_ui_array"]
    pub fn fmpz_set_ui_array(out: *mut fmpz, in_: *const ulong, in_len: slong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_get_ui_array) for this function.
    #[link_name = "fmpz_get_ui_array"]
    pub fn fmpz_get_ui_array(out: *mut ulong, out_len: slong, in_: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_set_mpz) for this function.
    #[link_name = "fmpz_set_mpz"]
    pub fn fmpz_set_mpz(f: *mut fmpz, x: *const gmp::mpz_t);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_set_str) for this function.
    #[link_name = "fmpz_set_str"]
    pub fn fmpz_set_str(f: *mut fmpz, str: *const c_char, b: c_int) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_set_ui_smod) for this function.
    #[link_name = "fmpz_set_ui_smod"]
    pub fn fmpz_set_ui_smod(f: *mut fmpz, x: mp_limb_t, m: mp_limb_t);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.flint_mpz_init_set_readonly) for this function.
    #[link_name = "flint_mpz_init_set_readonly"]
    pub fn flint_mpz_init_set_readonly(z: *mut gmp::mpz_t, f: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.flint_mpz_clear_readonly) for this function.
    #[link_name = "flint_mpz_clear_readonly"]
    pub fn flint_mpz_clear_readonly(z: *mut gmp::mpz_t);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_init_set_readonly) for this function.
    #[link_name = "fmpz_init_set_readonly"]
    pub fn fmpz_init_set_readonly(f: *mut fmpz, z: *const gmp::mpz_t);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_clear_readonly) for this function.
    #[link_name = "fmpz_clear_readonly"]
    pub fn fmpz_clear_readonly(f: *mut fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_read) for this function.
    #[link_name = "fmpz_read"]
    pub fn fmpz_read(f: *mut fmpz) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_fread) for this function.
    #[link_name = "fmpz_fread"]
    pub fn fmpz_fread(file: *mut FILE, f: *const fmpz) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_inp_raw) for this function.
    #[link_name = "fmpz_inp_raw"]
    pub fn fmpz_inp_raw(x: *mut fmpz, fin: *mut FILE) -> usize;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_print) for this function.
    #[link_name = "fmpz_print"]
    pub fn fmpz_print(x: *mut fmpz) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_fprint) for this function.
    #[link_name = "fmpz_fprint"]
    pub fn fmpz_fprint(file: *mut FILE, x: *const fmpz) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_out_raw) for this function.
    #[link_name = "fmpz_out_raw"]
    pub fn fmpz_out_raw(fout: *mut FILE, x: *const fmpz) -> usize;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_sizeinbase) for this function.
    #[link_name = "fmpz_sizeinbase"]
    pub fn fmpz_sizeinbase(f: *const fmpz, b: c_int) -> usize;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_bits) for this function.
    #[link_name = "fmpz_bits"]
    pub fn fmpz_bits(f: *const fmpz) -> usize;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_size) for this function.
    #[link_name = "fmpz_size"]
    pub fn fmpz_size(f: *const fmpz) -> c_long;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_sgn) for this function.
    #[link_name = "fmpz_sgn"]
    pub fn fmpz_sgn(f: *const fmpz) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_val2) for this function.
    #[link_name = "fmpz_val2"]
    pub fn fmpz_val2(f: *const fmpz) -> usize;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_swap) for this function.
    #[link_name = "fmpz_swap"]
    pub fn fmpz_swap(f: *mut fmpz, g: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_set) for this function.
    #[link_name = "fmpz_set"]
    pub fn fmpz_set(f: *mut fmpz, g: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_zero) for this function.
    #[link_name = "fmpz_zero"]
    pub fn fmpz_zero(f: *mut fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_one) for this function.
    #[link_name = "fmpz_one"]
    pub fn fmpz_one(f: *mut fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_abs_fits_ui) for this function.
    #[link_name = "fmpz_abs_fits_ui"]
    pub fn fmpz_abs_fits_ui(f: *const fmpz) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_fits_si) for this function.
    #[link_name = "fmpz_fits_si"]
    pub fn fmpz_fits_si(f: *const fmpz) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_setbit) for this function.
    #[link_name = "fmpz_setbit"]
    pub fn fmpz_setbit(f: *mut fmpz, i: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_tstbit) for this function.
    #[link_name = "fmpz_tstbit"]
    pub fn fmpz_tstbit(f: *const fmpz, i: ulong) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_abs_lbound_ui_2exp) for this function.
    #[link_name = "fmpz_abs_lbound_ui_2exp"]
    pub fn fmpz_abs_lbound_ui_2exp(exp: *const isize, x: *const fmpz, bits: c_int) -> mp_limb_t;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_abs_ubound_ui_2exp) for this function.
    #[link_name = "fmpz_abs_ubound_ui_2exp"]
    pub fn fmpz_abs_ubound_ui_2exp(exp: *const isize, x: *const fmpz, bits: c_int) -> mp_limb_t;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_cmp) for this function.
    #[link_name = "fmpz_cmp"]
    pub fn fmpz_cmp(f: *const fmpz, g: *const fmpz) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_cmp_ui) for this function.
    #[link_name = "fmpz_cmp_ui"]
    pub fn fmpz_cmp_ui(f: *const fmpz, g: ulong) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_cmp_si) for this function.
    #[link_name = "fmpz_cmp_si"]
    pub fn fmpz_cmp_si(f: *const fmpz, g: slong) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_cmpabs) for this function.
    #[link_name = "fmpz_cmpabs"]
    pub fn fmpz_cmpabs(f: *const fmpz, g: *const fmpz) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_equal) for this function.
    #[link_name = "fmpz_equal"]
    pub fn fmpz_equal(f: *const fmpz, g: *const fmpz) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_equal_ui) for this function.
    #[link_name = "fmpz_equal_ui"]
    pub fn fmpz_equal_ui(f: *const fmpz, g: ulong) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_equal_si) for this function.
    #[link_name = "fmpz_equal_si"]
    pub fn fmpz_equal_si(f: *const fmpz, g: slong) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_is_zero) for this function.
    #[link_name = "fmpz_is_zero"]
    pub fn fmpz_is_zero(f: *const fmpz) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_is_one) for this function.
    #[link_name = "fmpz_is_one"]
    pub fn fmpz_is_one(f: *const fmpz) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_is_pm1) for this function.
    #[link_name = "fmpz_is_pm1"]
    pub fn fmpz_is_pm1(f: *const fmpz) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_is_even) for this function.
    #[link_name = "fmpz_is_even"]
    pub fn fmpz_is_even(f: *const fmpz) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_is_odd) for this function.
    #[link_name = "fmpz_is_odd"]
    pub fn fmpz_is_odd(f: *const fmpz) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_neg) for this function.
    #[link_name = "__fmpz_neg"]
    pub fn fmpz_neg(f1: *mut fmpz, f2: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_abs) for this function.
    #[link_name = "fmpz_abs"]
    pub fn fmpz_abs(f1: *mut fmpz, f2: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_add) for this function.
    #[link_name = "fmpz_add"]
    pub fn fmpz_add(f: *mut fmpz, g: *const fmpz, h: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_add_ui) for this function.
    #[link_name = "fmpz_add_ui"]
    pub fn fmpz_add_ui(f: *mut fmpz, g: *const fmpz, x: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_add_si) for this function.
    #[link_name = "fmpz_add_si"]
    pub fn fmpz_add_si(f: *mut fmpz, g: *const fmpz, x: slong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_sub) for this function.
    #[link_name = "fmpz_sub"]
    pub fn fmpz_sub(f: *mut fmpz, g: *const fmpz, h: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_sub_ui) for this function.
    #[link_name = "fmpz_sub_ui"]
    pub fn fmpz_sub_ui(f: *mut fmpz, g: *const fmpz, x: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_sub_si) for this function.
    #[link_name = "fmpz_sub_si"]
    pub fn fmpz_sub_si(f: *mut fmpz, g: *const fmpz, x: slong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_mul) for this function.
    #[link_name = "fmpz_mul"]
    pub fn fmpz_mul(f: *mut fmpz, g: *const fmpz, h: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_mul_si) for this function.
    #[link_name = "fmpz_mul_si"]
    pub fn fmpz_mul_si(f: *mut fmpz, g: *const fmpz, x: slong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_mul_ui) for this function.
    #[link_name = "fmpz_mul_ui"]
    pub fn fmpz_mul_ui(f: *mut fmpz, g: *const fmpz, x: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_mul2_uiui) for this function.
    #[link_name = "fmpz_mul2_uiui"]
    pub fn fmpz_mul2_uiui(f: *mut fmpz, g: *const fmpz, x: ulong, y: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_mul_2exp) for this function.
    #[link_name = "fmpz_mul_2exp"]
    pub fn fmpz_mul_2exp(f: *mut fmpz, g: *const fmpz, e: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_addmul) for this function.
    #[link_name = "fmpz_addmul"]
    pub fn fmpz_addmul(f: *mut fmpz, g: *const fmpz, h: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_addmul_ui) for this function.
    #[link_name = "fmpz_addmul_ui"]
    pub fn fmpz_addmul_ui(f: *mut fmpz, g: *const fmpz, x: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_submul) for this function.
    #[link_name = "fmpz_submul"]
    pub fn fmpz_submul(f: *mut fmpz, g: *const fmpz, h: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_submul_ui) for this function.
    #[link_name = "fmpz_submul_ui"]
    pub fn fmpz_submul_ui(f: *mut fmpz, g: *const fmpz, x: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_fmma) for this function.
    #[link_name = "fmpz_fmma"]
    pub fn fmpz_fmma(f: *mut fmpz, a: *const fmpz, b: *const fmpz, c: *const fmpz, d: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_fmms) for this function.
    #[link_name = "fmpz_fmms"]
    pub fn fmpz_fmms(f: *mut fmpz, a: *const fmpz, b: *const fmpz, c: *const fmpz, d: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_cdiv_qr) for this function.
    #[link_name = "fmpz_cdiv_qr"]
    pub fn fmpz_cdiv_qr(f: *mut fmpz, s: *const fmpz, g: *const fmpz, h: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_cdiv_q) for this function.
    #[link_name = "fmpz_cdiv_q"]
    pub fn fmpz_cdiv_q(f: *mut fmpz, g: *const fmpz, h: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_cdiv_q_si) for this function.
    #[link_name = "fmpz_cdiv_q_si"]
    pub fn fmpz_cdiv_q_si(f: *mut fmpz, g: *const fmpz, h: slong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_cdiv_q_ui) for this function.
    #[link_name = "fmpz_cdiv_q_ui"]
    pub fn fmpz_cdiv_q_ui(f: *mut fmpz, g: *const fmpz, h: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_cdiv_q_2exp) for this function.
    #[link_name = "fmpz_cdiv_q_2exp"]
    pub fn fmpz_cdiv_q_2exp(f: *mut fmpz, g: *const fmpz, exp: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_cdiv_r_2exp) for this function.
    #[link_name = "fmpz_cdiv_r_2exp"]
    pub fn fmpz_cdiv_r_2exp(f: *mut fmpz, g: *const fmpz, exp: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_cdiv_ui) for this function.
    #[link_name = "fmpz_cdiv_ui"]
    pub fn fmpz_cdiv_ui(g: *const fmpz, h: ulong) -> ulong;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_fdiv_q_2exp) for this function.
    #[link_name = "fmpz_fdiv_q_2exp"]
    pub fn fmpz_fdiv_q_2exp(f: *mut fmpz, g: *const fmpz, exp: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_fdiv_q) for this function.
    #[link_name = "fmpz_fdiv_q"]
    pub fn fmpz_fdiv_q(f: *mut fmpz, g: *const fmpz, h: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_fdiv_q_si) for this function.
    #[link_name = "fmpz_fdiv_q_si"]
    pub fn fmpz_fdiv_q_si(f: *mut fmpz, g: *const fmpz, h: slong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_fdiv_q_ui) for this function.
    #[link_name = "fmpz_fdiv_q_ui"]
    pub fn fmpz_fdiv_q_ui(f: *mut fmpz, g: *const fmpz, h: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_fdiv_qr) for this function.
    #[link_name = "fmpz_fdiv_qr"]
    pub fn fmpz_fdiv_qr(f: *mut fmpz, s: *const fmpz, g: *const fmpz, h: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_fdiv_r) for this function.
    #[link_name = "fmpz_fdiv_r"]
    pub fn fmpz_fdiv_r(f: *mut fmpz, g: *const fmpz, h: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_fdiv_r_2exp) for this function.
    #[link_name = "fmpz_fdiv_r_2exp"]
    pub fn fmpz_fdiv_r_2exp(f: *mut fmpz, g: *const fmpz, exp: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_fdiv_ui) for this function.
    #[link_name = "fmpz_fdiv_ui"]
    pub fn fmpz_fdiv_ui(g: *const fmpz, x: ulong) -> ulong;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_tdiv_q) for this function.
    #[link_name = "fmpz_tdiv_q"]
    pub fn fmpz_tdiv_q(f: *mut fmpz, g: *const fmpz, h: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_tdiv_qr) for this function.
    #[link_name = "fmpz_tdiv_qr"]
    pub fn fmpz_tdiv_qr(f: *mut fmpz, s: *const fmpz, g: *const fmpz, h: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_tdiv_q_si) for this function.
    #[link_name = "fmpz_tdiv_q_si"]
    pub fn fmpz_tdiv_q_si(f: *mut fmpz, g: *const fmpz, h: slong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_tdiv_q_ui) for this function.
    #[link_name = "fmpz_tdiv_q_ui"]
    pub fn fmpz_tdiv_q_ui(f: *mut fmpz, g: *const fmpz, h: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_tdiv_r_2exp) for this function.
    #[link_name = "fmpz_tdiv_r_2exp"]
    pub fn fmpz_tdiv_r_2exp(f: *mut fmpz, g: *const fmpz, exp: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_tdiv_ui) for this function.
    #[link_name = "fmpz_tdiv_ui"]
    pub fn fmpz_tdiv_ui(g: *const fmpz, h: ulong) -> ulong;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_tdiv_q_2exp) for this function.
    #[link_name = "fmpz_tdiv_q_2exp"]
    pub fn fmpz_tdiv_q_2exp(f: *mut fmpz, g: *const fmpz, exp: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_divexact) for this function.
    #[link_name = "fmpz_divexact"]
    pub fn fmpz_divexact(f: *mut fmpz, g: *const fmpz, h: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_divexact_si) for this function.
    #[link_name = "fmpz_divexact_si"]
    pub fn fmpz_divexact_si(f: *mut fmpz, g: *const fmpz, h: slong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_divexact_ui) for this function.
    #[link_name = "fmpz_divexact_ui"]
    pub fn fmpz_divexact_ui(f: *mut fmpz, g: *const fmpz, h: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_divexact2_uiui) for this function.
    #[link_name = "fmpz_divexact2_uiui"]
    pub fn fmpz_divexact2_uiui(f: *mut fmpz, g: *const fmpz, x: ulong, y: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_divisible) for this function.
    #[link_name = "fmpz_divisible"]
    pub fn fmpz_divisible(f: *const fmpz, g: *const fmpz) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_divisible_si) for this function.
    #[link_name = "fmpz_divisible_si"]
    pub fn fmpz_divisible_si(f: *const fmpz, g: slong) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_mod) for this function.
    #[link_name = "fmpz_mod"]
    pub fn fmpz_mod(f: *mut fmpz, g: *const fmpz, h: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_mod_ui) for this function.
    #[link_name = "fmpz_mod_ui"]
    pub fn fmpz_mod_ui(f: *mut fmpz, g: *const fmpz, x: ulong) -> ulong;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_smod) for this function.
    #[link_name = "fmpz_smod"]
    pub fn fmpz_smod(f: *mut fmpz, g: *const fmpz, h: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_pow_ui) for this function.
    #[link_name = "fmpz_pow_ui"]
    pub fn fmpz_pow_ui(f: *mut fmpz, g: *const fmpz, x: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_pow_fmpz) for this function.
    #[link_name = "fmpz_pow_fmpz"]
    pub fn fmpz_pow_fmpz(f: *mut fmpz, g: *const fmpz, x: *const fmpz) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_powm_ui) for this function.
    #[link_name = "fmpz_powm_ui"]
    pub fn fmpz_powm_ui(f: *mut fmpz, g: *const fmpz, e: ulong, m: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_powm) for this function.
    #[link_name = "fmpz_powm"]
    pub fn fmpz_powm(f: *mut fmpz, g: *const fmpz, e: *const fmpz, m: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_clog) for this function.
    #[link_name = "fmpz_clog"]
    pub fn fmpz_clog(x: *const fmpz, b: *const fmpz) -> slong;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_clog_ui) for this function.
    #[link_name = "fmpz_clog_ui"]
    pub fn fmpz_clog_ui(x: *const fmpz, b: ulong) -> slong;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_flog) for this function.
    #[link_name = "fmpz_flog"]
    pub fn fmpz_flog(x: *const fmpz, b: *const fmpz) -> slong;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_flog_ui) for this function.
    #[link_name = "fmpz_flog_ui"]
    pub fn fmpz_flog_ui(x: *const fmpz, b: ulong) -> slong;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_dlog) for this function.
    #[link_name = "fmpz_dlog"]
    pub fn fmpz_dlog(x: *const fmpz) -> c_double;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_sqrtmod) for this function.
    #[link_name = "fmpz_sqrtmod"]
    pub fn fmpz_sqrtmod(b: *mut fmpz, a: *const fmpz, p: *const fmpz) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_sqrt) for this function.
    #[link_name = "fmpz_sqrt"]
    pub fn fmpz_sqrt(f: *mut fmpz, g: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_sqrtrem) for this function.
    #[link_name = "fmpz_sqrtrem"]
    pub fn fmpz_sqrtrem(f: *mut fmpz, r: *const fmpz, g: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_is_square) for this function.
    #[link_name = "fmpz_is_square"]
    pub fn fmpz_is_square(f: *const fmpz) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_root) for this function.
    #[link_name = "fmpz_root"]
    pub fn fmpz_root(r: *mut fmpz, f: *const fmpz, n: slong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_is_perfect_power) for this function.
    #[link_name = "fmpz_is_perfect_power"]
    pub fn fmpz_is_perfect_power(root: *mut fmpz, f: *const fmpz) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_fac_ui) for this function.
    #[link_name = "fmpz_fac_ui"]
    pub fn fmpz_fac_ui(f: *mut fmpz, n: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_fib_ui) for this function.
    #[link_name = "fmpz_fib_ui"]
    pub fn fmpz_fib_ui(f: *mut fmpz, n: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_bin_uiui) for this function.
    #[link_name = "fmpz_bin_uiui"]
    pub fn fmpz_bin_uiui(f: *mut fmpz, n: ulong, k: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c._fmpz_rfac_ui) for this function.
    #[link_name = "_fmpz_rfac_ui"]
    pub fn _fmpz_rfac_ui(r: *mut fmpz, x: *const fmpz, a: ulong, b: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_rfac_ui) for this function.
    #[link_name = "fmpz_rfac_ui"]
    pub fn fmpz_rfac_ui(r: *mut fmpz, x: *const fmpz, k: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_rfac_uiui) for this function.
    #[link_name = "fmpz_rfac_uiui"]
    pub fn fmpz_rfac_uiui(r: *mut fmpz, x: ulong, k: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_mul_tdiv_q_2exp) for this function.
    #[link_name = "fmpz_mul_tdiv_q_2exp"]
    pub fn fmpz_mul_tdiv_q_2exp(f: *mut fmpz, g: *const fmpz, h: *const fmpz, exp: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_mul_si_tdiv_q_2exp) for this function.
    #[link_name = "fmpz_mul_si_tdiv_q_2exp"]
    pub fn fmpz_mul_si_tdiv_q_2exp(f: *mut fmpz, g: *const fmpz, x: slong, exp: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_gcd) for this function.
    #[link_name = "fmpz_gcd"]
    pub fn fmpz_gcd(f: *mut fmpz, g: *const fmpz, h: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_lcm) for this function.
    #[link_name = "fmpz_lcm"]
    pub fn fmpz_lcm(f: *mut fmpz, g: *const fmpz, h: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_gcdinv) for this function.
    #[link_name = "fmpz_gcdinv"]
    pub fn fmpz_gcdinv(d: *mut fmpz, a: *const fmpz, f: *const fmpz, g: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_xgcd) for this function.
    #[link_name = "fmpz_xgcd"]
    pub fn fmpz_xgcd(d: *mut fmpz, a: *const fmpz, b: *const fmpz, f: *const fmpz, g: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_xgcd_partial) for this function.
    #[link_name = "fmpz_xgcd_partial"]
    pub fn fmpz_xgcd_partial(
        co2: *mut fmpz,
        co1: *const fmpz,
        r2: *const fmpz,
        r1: *const fmpz,
        L: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c._fmpz_remove) for this function.
    #[link_name = "_fmpz_remove"]
    pub fn _fmpz_remove(x: *mut fmpz, f: *const fmpz, finv: c_double) -> slong;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_remove) for this function.
    #[link_name = "fmpz_remove"]
    pub fn fmpz_remove(rop: *mut fmpz, op: *const fmpz, f: *const fmpz) -> slong;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_invmod) for this function.
    #[link_name = "fmpz_invmod"]
    pub fn fmpz_invmod(f: *mut fmpz, g: *const fmpz, h: *const fmpz) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_negmod) for this function.
    #[link_name = "fmpz_negmod"]
    pub fn fmpz_negmod(f: *mut fmpz, g: *const fmpz, h: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_jacobi) for this function.
    #[link_name = "fmpz_jacobi"]
    pub fn fmpz_jacobi(a: *const fmpz, p: *const fmpz) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_divides_mod_list) for this function.
    #[link_name = "fmpz_divides_mod_list"]
    pub fn fmpz_divides_mod_list(
        xstart: *mut fmpz,
        xstride: *const fmpz,
        xlength: *const fmpz,
        a: *const fmpz,
        b: *const fmpz,
        n: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_bit_pack) for this function.
    #[link_name = "fmpz_bit_pack"]
    pub fn fmpz_bit_pack(
        arr: *mut mp_limb_t,
        shift: usize,
        bits: usize,
        coeff: *const fmpz,
        negate: c_int,
        borrow: c_int,
    ) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_bit_unpack) for this function.
    #[link_name = "fmpz_bit_unpack"]
    pub fn fmpz_bit_unpack(
        coeff: *mut fmpz,
        arr: *mut mp_limb_t,
        shift: usize,
        bits: usize,
        negate: c_int,
        borrow: c_int,
    ) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_bit_unpack_unsigned) for this function.
    #[link_name = "fmpz_bit_unpack_unsigned"]
    pub fn fmpz_bit_unpack_unsigned(
        coeff: *mut fmpz,
        arr: *const mp_limb_t,
        shift: usize,
        bits: usize,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_complement) for this function.
    #[link_name = "fmpz_complement"]
    pub fn fmpz_complement(r: *mut fmpz, f: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_clrbit) for this function.
    #[link_name = "fmpz_clrbit"]
    pub fn fmpz_clrbit(f: *mut fmpz, i: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_combit) for this function.
    #[link_name = "fmpz_combit"]
    pub fn fmpz_combit(f: *mut fmpz, i: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_and) for this function.
    #[link_name = "fmpz_and"]
    pub fn fmpz_and(r: *mut fmpz, a: *const fmpz, b: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_or) for this function.
    #[link_name = "fmpz_or"]
    pub fn fmpz_or(r: *mut fmpz, a: *const fmpz, b: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_xor) for this function.
    #[link_name = "fmpz_xor"]
    pub fn fmpz_xor(r: *mut fmpz, a: *const fmpz, b: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_popcnt) for this function.
    #[link_name = "fmpz_popcnt"]
    pub fn fmpz_popcnt(a: *const fmpz) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_CRT_ui) for this function.
    #[link_name = "fmpz_CRT_ui"]
    pub fn fmpz_CRT_ui(
        out: *mut fmpz,
        r1: *const fmpz,
        m1: *const fmpz,
        r2: ulong,
        m2: ulong,
        sign: c_int,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_CRT) for this function.
    #[link_name = "fmpz_CRT"]
    pub fn fmpz_CRT(
        out: *mut fmpz,
        r1: *const fmpz,
        m1: *const fmpz,
        r2: *const fmpz,
        m2: *const fmpz,
        sign: c_int,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_multi_crt) for this function.
    #[link_name = "fmpz_multi_crt"]
    pub fn fmpz_multi_crt(
        output: *mut fmpz,
        moduli: *const fmpz,
        values: *const fmpz,
        len: slong,
    ) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_is_strong_probabprime) for this function.
    #[link_name = "fmpz_is_strong_probabprime"]
    pub fn fmpz_is_strong_probabprime(n: *const fmpz, a: *const fmpz) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_is_probabprime_lucas) for this function.
    #[link_name = "fmpz_is_probabprime_lucas"]
    pub fn fmpz_is_probabprime_lucas(n: *const fmpz) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_is_probabprime_BPSW) for this function.
    #[link_name = "fmpz_is_probabprime_BPSW"]
    pub fn fmpz_is_probabprime_BPSW(n: *const fmpz) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_is_probabprime) for this function.
    #[link_name = "fmpz_is_probabprime"]
    pub fn fmpz_is_probabprime(p: *const fmpz) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_is_prime_pseudosquare) for this function.
    #[link_name = "fmpz_is_prime_pseudosquare"]
    pub fn fmpz_is_prime_pseudosquare(n: *const fmpz) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_is_prime_pocklington) for this function.
    #[link_name = "fmpz_is_prime_pocklington"]
    pub fn fmpz_is_prime_pocklington(
        F: *mut fmpz,
        R: *const fmpz,
        n: *const fmpz,
        pm1: *mut mp_limb_t,
        num_pm1: slong,
    ) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c._fmpz_nm1_trial_factors) for this function.
    #[link_name = "_fmpz_nm1_trial_factors"]
    pub fn _fmpz_nm1_trial_factors(
        n: *const fmpz,
        pm1: *mut mp_limb_t,
        num_pm1: *const isize,
        limit: ulong,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_is_prime_morrison) for this function.
    #[link_name = "fmpz_is_prime_morrison"]
    pub fn fmpz_is_prime_morrison(
        F: *mut fmpz,
        R: *const fmpz,
        n: *const fmpz,
        pp1: *mut mp_limb_t,
        num_pp1: slong,
    ) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c._fmpz_np1_trial_factors) for this function.
    #[link_name = "_fmpz_np1_trial_factors"]
    pub fn _fmpz_np1_trial_factors(
        n: *const fmpz,
        pp1: *mut mp_limb_t,
        num_pp1: *const isize,
        limit: ulong,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_is_prime) for this function.
    #[link_name = "fmpz_is_prime"]
    pub fn fmpz_is_prime(n: *const fmpz) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_lucas_chain) for this function.
    #[link_name = "fmpz_lucas_chain"]
    pub fn fmpz_lucas_chain(
        Vm: *mut fmpz,
        Vm1: *const fmpz,
        A: *const fmpz,
        m: *const fmpz,
        n: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_lucas_chain_full) for this function.
    #[link_name = "fmpz_lucas_chain_full"]
    pub fn fmpz_lucas_chain_full(
        Vm: *mut fmpz,
        Vm1: *const fmpz,
        A: *const fmpz,
        B: *const fmpz,
        m: *const fmpz,
        n: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_lucas_chain_double) for this function.
    #[link_name = "fmpz_lucas_chain_double"]
    pub fn fmpz_lucas_chain_double(
        U2m: *mut fmpz,
        U2m1: *const fmpz,
        Um: *const fmpz,
        Um1: *const fmpz,
        A: *const fmpz,
        B: *const fmpz,
        n: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_lucas_chain_add) for this function.
    #[link_name = "fmpz_lucas_chain_add"]
    pub fn fmpz_lucas_chain_add(
        Umn: *mut fmpz,
        Umn1: *const fmpz,
        Um: *const fmpz,
        Um1: *const fmpz,
        Un: *const fmpz,
        Un1: *const fmpz,
        A: *const fmpz,
        B: *const fmpz,
        n: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_lucas_chain_mul) for this function.
    #[link_name = "fmpz_lucas_chain_mul"]
    pub fn fmpz_lucas_chain_mul(
        Ukm: *mut fmpz,
        Ukm1: *const fmpz,
        Um: *const fmpz,
        Um1: *const fmpz,
        A: *const fmpz,
        B: *const fmpz,
        k: *const fmpz,
        n: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_lucas_chain_VtoU) for this function.
    #[link_name = "fmpz_lucas_chain_VtoU"]
    pub fn fmpz_lucas_chain_VtoU(
        Um: *mut fmpz,
        Um1: *const fmpz,
        Vm: *const fmpz,
        Vm1: *const fmpz,
        A: *const fmpz,
        B: *const fmpz,
        Dinv: *const fmpz,
        n: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_divisor_in_residue_class_lenstra) for this function.
    #[link_name = "fmpz_divisor_in_residue_class_lenstra"]
    pub fn fmpz_divisor_in_residue_class_lenstra(
        fac: *mut fmpz,
        n: *const fmpz,
        r: *const fmpz,
        s: *const fmpz,
    ) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_nextprime) for this function.
    #[link_name = "fmpz_nextprime"]
    pub fn fmpz_nextprime(res: *mut fmpz, n: *const fmpz, proved: c_int);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_primorial) for this function.
    #[link_name = "fmpz_primorial"]
    pub fn fmpz_primorial(res: *mut fmpz, n: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_euler_phi) for this function.
    #[link_name = "fmpz_euler_phi"]
    pub fn fmpz_euler_phi(res: *mut fmpz, n: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_moebius_mu) for this function.
    #[link_name = "fmpz_moebius_mu"]
    pub fn fmpz_moebius_mu(n: *const fmpz) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz.html#c.fmpz_divisor_sigma) for this function.
    #[link_name = "fmpz_divisor_sigma"]
    pub fn fmpz_divisor_sigma(res: *mut fmpz, n: *const fmpz, k: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_init) for this function.
    #[link_name = "fmpz_mod_poly_init"]
    pub fn fmpz_mod_poly_init(poly: *const fmpz_mod_poly, p: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_init2) for this function.
    #[link_name = "fmpz_mod_poly_init2"]
    pub fn fmpz_mod_poly_init2(poly: *const fmpz_mod_poly, p: *const fmpz, alloc: slong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_clear) for this function.
    #[link_name = "fmpz_mod_poly_clear"]
    pub fn fmpz_mod_poly_clear(poly: *const fmpz_mod_poly);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_realloc) for this function.
    #[link_name = "fmpz_mod_poly_realloc"]
    pub fn fmpz_mod_poly_realloc(poly: *const fmpz_mod_poly, alloc: slong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_fit_length) for this function.
    #[link_name = "fmpz_mod_poly_fit_length"]
    pub fn fmpz_mod_poly_fit_length(poly: *const fmpz_mod_poly, len: slong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_normalise) for this function.
    #[link_name = "_fmpz_mod_poly_normalise"]
    pub fn _fmpz_mod_poly_normalise(poly: *const fmpz_mod_poly);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_set_length) for this function.
    #[link_name = "_fmpz_mod_poly_set_length"]
    pub fn _fmpz_mod_poly_set_length(poly: *const fmpz_mod_poly, len: slong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_truncate) for this function.
    #[link_name = "fmpz_mod_poly_truncate"]
    pub fn fmpz_mod_poly_truncate(poly: *const fmpz_mod_poly, len: slong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_set_trunc) for this function.
    #[link_name = "fmpz_mod_poly_set_trunc"]
    pub fn fmpz_mod_poly_set_trunc(res: *const fmpz_mod_poly, poly: *const fmpz_mod_poly, n: slong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_randtest) for this function.
    #[link_name = "fmpz_mod_poly_randtest"]
    pub fn fmpz_mod_poly_randtest(f: *const fmpz_mod_poly, state: *mut flint_rand, len: slong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_randtest_irreducible) for this function.
    #[link_name = "fmpz_mod_poly_randtest_irreducible"]
    pub fn fmpz_mod_poly_randtest_irreducible(
        f: *const fmpz_mod_poly,
        state: *mut flint_rand,
        len: slong,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_randtest_not_zero) for this function.
    #[link_name = "fmpz_mod_poly_randtest_not_zero"]
    pub fn fmpz_mod_poly_randtest_not_zero(
        f: *const fmpz_mod_poly,
        state: *mut flint_rand,
        len: slong,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_randtest_monic) for this function.
    #[link_name = "fmpz_mod_poly_randtest_monic"]
    pub fn fmpz_mod_poly_randtest_monic(
        poly: *const fmpz_mod_poly,
        state: *mut flint_rand,
        len: slong,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_randtest_monic_irreducible) for this function.
    #[link_name = "fmpz_mod_poly_randtest_monic_irreducible"]
    pub fn fmpz_mod_poly_randtest_monic_irreducible(
        poly: *const fmpz_mod_poly,
        state: *mut flint_rand,
        len: slong,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_randtest_monic_primitive) for this function.
    #[link_name = "fmpz_mod_poly_randtest_monic_primitive"]
    pub fn fmpz_mod_poly_randtest_monic_primitive(
        poly: *const fmpz_mod_poly,
        state: *mut flint_rand,
        len: slong,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_randtest_trinomial) for this function.
    #[link_name = "fmpz_mod_poly_randtest_trinomial"]
    pub fn fmpz_mod_poly_randtest_trinomial(
        poly: *const fmpz_mod_poly,
        state: *mut flint_rand,
        len: slong,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_randtest_trinomial_irreducible) for this function.
    #[link_name = "fmpz_mod_poly_randtest_trinomial_irreducible"]
    pub fn fmpz_mod_poly_randtest_trinomial_irreducible(
        poly: *const fmpz_mod_poly,
        state: *mut flint_rand,
        len: slong,
        max_attempts: slong,
    ) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_randtest_pentomial) for this function.
    #[link_name = "fmpz_mod_poly_randtest_pentomial"]
    pub fn fmpz_mod_poly_randtest_pentomial(
        poly: *const fmpz_mod_poly,
        state: *mut flint_rand,
        len: slong,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_randtest_pentomial_irreducible) for this function.
    #[link_name = "fmpz_mod_poly_randtest_pentomial_irreducible"]
    pub fn fmpz_mod_poly_randtest_pentomial_irreducible(
        poly: *const fmpz_mod_poly,
        state: *mut flint_rand,
        len: slong,
        max_attempts: slong,
    ) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_randtest_sparse_irreducible) for this function.
    #[link_name = "fmpz_mod_poly_randtest_sparse_irreducible"]
    pub fn fmpz_mod_poly_randtest_sparse_irreducible(
        poly: *const fmpz_mod_poly,
        state: *mut flint_rand,
        len: slong,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_modulus) for this function.
    #[link_name = "fmpz_mod_poly_modulus"]
    pub fn fmpz_mod_poly_modulus(poly: *const fmpz_mod_poly) -> *mut fmpz;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_degree) for this function.
    #[link_name = "fmpz_mod_poly_degree"]
    pub fn fmpz_mod_poly_degree(poly: *const fmpz_mod_poly) -> slong;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_length) for this function.
    #[link_name = "fmpz_mod_poly_length"]
    pub fn fmpz_mod_poly_length(poly: *const fmpz_mod_poly) -> slong;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_lead) for this function.
    #[link_name = "fmpz_mod_poly_lead"]
    pub fn fmpz_mod_poly_lead(poly: *const fmpz_mod_poly) -> *mut fmpz;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_set) for this function.
    #[link_name = "fmpz_mod_poly_set"]
    pub fn fmpz_mod_poly_set(poly1: *const fmpz_mod_poly, poly2: *const fmpz_mod_poly);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_swap) for this function.
    #[link_name = "fmpz_mod_poly_swap"]
    pub fn fmpz_mod_poly_swap(poly1: *const fmpz_mod_poly, poly2: *const fmpz_mod_poly);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_zero) for this function.
    #[link_name = "fmpz_mod_poly_zero"]
    pub fn fmpz_mod_poly_zero(poly: *const fmpz_mod_poly);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_one) for this function.
    #[link_name = "fmpz_mod_poly_one"]
    pub fn fmpz_mod_poly_one(poly: *const fmpz_mod_poly);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_zero_coeffs) for this function.
    #[link_name = "fmpz_mod_poly_zero_coeffs"]
    pub fn fmpz_mod_poly_zero_coeffs(poly: *const fmpz_mod_poly, i: slong, j: slong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_reverse) for this function.
    #[link_name = "fmpz_mod_poly_reverse"]
    pub fn fmpz_mod_poly_reverse(res: *const fmpz_mod_poly, poly: *const fmpz_mod_poly, n: slong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_set_ui) for this function.
    #[link_name = "fmpz_mod_poly_set_ui"]
    pub fn fmpz_mod_poly_set_ui(f: *const fmpz_mod_poly, c: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_set_fmpz) for this function.
    #[link_name = "fmpz_mod_poly_set_fmpz"]
    pub fn fmpz_mod_poly_set_fmpz(f: *const fmpz_mod_poly, c: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_set_fmpz_poly) for this function.
    #[link_name = "fmpz_mod_poly_set_fmpz_poly"]
    pub fn fmpz_mod_poly_set_fmpz_poly(f: *const fmpz_mod_poly, g: *const fmpz_poly);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_get_fmpz_poly) for this function.
    #[link_name = "fmpz_mod_poly_get_fmpz_poly"]
    pub fn fmpz_mod_poly_get_fmpz_poly(f: *mut fmpz_poly, g: *const fmpz_mod_poly);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_equal) for this function.
    #[link_name = "fmpz_mod_poly_equal"]
    pub fn fmpz_mod_poly_equal(poly1: *const fmpz_mod_poly, poly2: *const fmpz_mod_poly) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_equal_trunc) for this function.
    #[link_name = "fmpz_mod_poly_equal_trunc"]
    pub fn fmpz_mod_poly_equal_trunc(
        poly1: *const fmpz_mod_poly,
        poly2: *const fmpz_mod_poly,
        n: slong,
    ) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_is_zero) for this function.
    #[link_name = "fmpz_mod_poly_is_zero"]
    pub fn fmpz_mod_poly_is_zero(poly: *const fmpz_mod_poly) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_is_one) for this function.
    #[link_name = "fmpz_mod_poly_is_one"]
    pub fn fmpz_mod_poly_is_one(poly: *const fmpz_mod_poly) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_is_gen) for this function.
    #[link_name = "fmpz_mod_poly_is_gen"]
    pub fn fmpz_mod_poly_is_gen(poly: *const fmpz_mod_poly) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_set_coeff_fmpz) for this function.
    #[link_name = "fmpz_mod_poly_set_coeff_fmpz"]
    pub fn fmpz_mod_poly_set_coeff_fmpz(poly: *const fmpz_mod_poly, n: slong, x: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_set_coeff_ui) for this function.
    #[link_name = "fmpz_mod_poly_set_coeff_ui"]
    pub fn fmpz_mod_poly_set_coeff_ui(poly: *const fmpz_mod_poly, n: slong, x: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_get_coeff_fmpz) for this function.
    #[link_name = "fmpz_mod_poly_get_coeff_fmpz"]
    pub fn fmpz_mod_poly_get_coeff_fmpz(x: *mut fmpz, poly: *const fmpz_mod_poly, n: slong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_set_coeff_mpz) for this function.
    #[link_name = "fmpz_mod_poly_set_coeff_mpz"]
    pub fn fmpz_mod_poly_set_coeff_mpz(poly: *const fmpz_mod_poly, n: slong, x: *const gmp::mpz_t);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_get_coeff_mpz) for this function.
    #[link_name = "fmpz_mod_poly_get_coeff_mpz"]
    pub fn fmpz_mod_poly_get_coeff_mpz(x: *mut gmp::mpz_t, poly: *const fmpz_mod_poly, n: slong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_shift_left) for this function.
    #[link_name = "_fmpz_mod_poly_shift_left"]
    pub fn _fmpz_mod_poly_shift_left(res: *mut fmpz, poly: *const fmpz, len: slong, n: slong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_shift_left) for this function.
    #[link_name = "fmpz_mod_poly_shift_left"]
    pub fn fmpz_mod_poly_shift_left(f: *const fmpz_mod_poly, g: *const fmpz_mod_poly, n: slong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_shift_right) for this function.
    #[link_name = "_fmpz_mod_poly_shift_right"]
    pub fn _fmpz_mod_poly_shift_right(res: *mut fmpz, poly: *const fmpz, len: slong, n: slong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_shift_right) for this function.
    #[link_name = "fmpz_mod_poly_shift_right"]
    pub fn fmpz_mod_poly_shift_right(f: *const fmpz_mod_poly, g: *const fmpz_mod_poly, n: slong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_add) for this function.
    #[link_name = "_fmpz_mod_poly_add"]
    pub fn _fmpz_mod_poly_add(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: slong,
        poly2: *const fmpz,
        len2: slong,
        p: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_add) for this function.
    #[link_name = "fmpz_mod_poly_add"]
    pub fn fmpz_mod_poly_add(
        res: *const fmpz_mod_poly,
        poly1: *const fmpz_mod_poly,
        poly2: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_add_series) for this function.
    #[link_name = "fmpz_mod_poly_add_series"]
    pub fn fmpz_mod_poly_add_series(
        res: *const fmpz_mod_poly,
        poly1: *const fmpz_mod_poly,
        poly2: *const fmpz_mod_poly,
        n: slong,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_sub) for this function.
    #[link_name = "_fmpz_mod_poly_sub"]
    pub fn _fmpz_mod_poly_sub(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: slong,
        poly2: *const fmpz,
        len2: slong,
        p: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_sub) for this function.
    #[link_name = "fmpz_mod_poly_sub"]
    pub fn fmpz_mod_poly_sub(
        res: *const fmpz_mod_poly,
        poly1: *const fmpz_mod_poly,
        poly2: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_sub_series) for this function.
    #[link_name = "fmpz_mod_poly_sub_series"]
    pub fn fmpz_mod_poly_sub_series(
        res: *const fmpz_mod_poly,
        poly1: *const fmpz_mod_poly,
        poly2: *const fmpz_mod_poly,
        n: slong,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_neg) for this function.
    #[link_name = "_fmpz_mod_poly_neg"]
    pub fn _fmpz_mod_poly_neg(res: *mut fmpz, poly: *const fmpz, len: slong, p: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_neg) for this function.
    #[link_name = "fmpz_mod_poly_neg"]
    pub fn fmpz_mod_poly_neg(res: *const fmpz_mod_poly, poly: *const fmpz_mod_poly);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_scalar_mul_fmpz) for this function.
    #[link_name = "_fmpz_mod_poly_scalar_mul_fmpz"]
    pub fn _fmpz_mod_poly_scalar_mul_fmpz(
        res: *mut fmpz,
        poly: *const fmpz,
        len: slong,
        x: *const fmpz,
        p: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_scalar_mul_fmpz) for this function.
    #[link_name = "fmpz_mod_poly_scalar_mul_fmpz"]
    pub fn fmpz_mod_poly_scalar_mul_fmpz(
        res: *const fmpz_mod_poly,
        poly: *const fmpz_mod_poly,
        x: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_scalar_div_fmpz) for this function.
    #[link_name = "_fmpz_mod_poly_scalar_div_fmpz"]
    pub fn _fmpz_mod_poly_scalar_div_fmpz(
        res: *mut fmpz,
        poly: *const fmpz,
        len: slong,
        x: *const fmpz,
        p: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_scalar_div_fmpz) for this function.
    #[link_name = "fmpz_mod_poly_scalar_div_fmpz"]
    pub fn fmpz_mod_poly_scalar_div_fmpz(
        res: *const fmpz_mod_poly,
        poly: *const fmpz_mod_poly,
        x: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_mul) for this function.
    #[link_name = "_fmpz_mod_poly_mul"]
    pub fn _fmpz_mod_poly_mul(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: slong,
        poly2: *const fmpz,
        len2: slong,
        p: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_mul) for this function.
    #[link_name = "fmpz_mod_poly_mul"]
    pub fn fmpz_mod_poly_mul(
        res: *const fmpz_mod_poly,
        poly1: *const fmpz_mod_poly,
        poly2: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_mullow) for this function.
    #[link_name = "_fmpz_mod_poly_mullow"]
    pub fn _fmpz_mod_poly_mullow(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: slong,
        poly2: *const fmpz,
        len2: slong,
        p: *const fmpz,
        n: slong,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_mullow) for this function.
    #[link_name = "fmpz_mod_poly_mullow"]
    pub fn fmpz_mod_poly_mullow(
        res: *const fmpz_mod_poly,
        poly1: *const fmpz_mod_poly,
        poly2: *const fmpz_mod_poly,
        n: slong,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_sqr) for this function.
    #[link_name = "_fmpz_mod_poly_sqr"]
    pub fn _fmpz_mod_poly_sqr(res: *mut fmpz, poly: *const fmpz, len: slong, p: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_sqr) for this function.
    #[link_name = "fmpz_mod_poly_sqr"]
    pub fn fmpz_mod_poly_sqr(res: *const fmpz_mod_poly, poly: *const fmpz_mod_poly);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_mulmod) for this function.
    #[link_name = "_fmpz_mod_poly_mulmod"]
    pub fn _fmpz_mod_poly_mulmod(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: slong,
        poly2: *const fmpz,
        len2: slong,
        f: *const fmpz,
        lenf: slong,
        p: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_mulmod) for this function.
    #[link_name = "fmpz_mod_poly_mulmod"]
    pub fn fmpz_mod_poly_mulmod(
        res: *const fmpz_mod_poly,
        poly1: *const fmpz_mod_poly,
        poly2: *const fmpz_mod_poly,
        f: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_mulmod_preinv) for this function.
    #[link_name = "_fmpz_mod_poly_mulmod_preinv"]
    pub fn _fmpz_mod_poly_mulmod_preinv(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: slong,
        poly2: *const fmpz,
        len2: slong,
        f: *const fmpz,
        lenf: slong,
        finv: *const fmpz,
        lenfinv: slong,
        p: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_mulmod_preinv) for this function.
    #[link_name = "fmpz_mod_poly_mulmod_preinv"]
    pub fn fmpz_mod_poly_mulmod_preinv(
        res: *const fmpz_mod_poly,
        poly1: *const fmpz_mod_poly,
        poly2: *const fmpz_mod_poly,
        f: *const fmpz_mod_poly,
        finv: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_product_roots_fmpz_vec) for this function.
    #[link_name = "_fmpz_mod_poly_product_roots_fmpz_vec"]
    pub fn _fmpz_mod_poly_product_roots_fmpz_vec(
        poly: *mut fmpz,
        xs: *const fmpz,
        n: slong,
        f: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_product_roots_fmpz_vec) for this function.
    #[link_name = "fmpz_mod_poly_product_roots_fmpz_vec"]
    pub fn fmpz_mod_poly_product_roots_fmpz_vec(
        poly: *mut fmpz_poly,
        xs: *const fmpz,
        n: slong,
        f: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_find_distinct_nonzero_roots) for this function.
    #[link_name = "fmpz_mod_poly_find_distinct_nonzero_roots"]
    pub fn fmpz_mod_poly_find_distinct_nonzero_roots(
        roots: *mut fmpz,
        A: *const fmpz_mod_poly,
    ) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_pow) for this function.
    #[link_name = "_fmpz_mod_poly_pow"]
    pub fn _fmpz_mod_poly_pow(
        rop: *mut fmpz,
        op: *const fmpz,
        len: slong,
        e: ulong,
        p: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_pow) for this function.
    #[link_name = "fmpz_mod_poly_pow"]
    pub fn fmpz_mod_poly_pow(rop: *const fmpz_mod_poly, op: *const fmpz_mod_poly, e: ulong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_pow_trunc) for this function.
    #[link_name = "_fmpz_mod_poly_pow_trunc"]
    pub fn _fmpz_mod_poly_pow_trunc(
        res: *mut fmpz,
        poly: *const fmpz,
        e: ulong,
        trunc: slong,
        p: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_pow_trunc) for this function.
    #[link_name = "fmpz_mod_poly_pow_trunc"]
    pub fn fmpz_mod_poly_pow_trunc(
        res: *const fmpz_mod_poly,
        poly: *const fmpz_mod_poly,
        e: ulong,
        trunc: slong,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_pow_trunc_binexp) for this function.
    #[link_name = "_fmpz_mod_poly_pow_trunc_binexp"]
    pub fn _fmpz_mod_poly_pow_trunc_binexp(
        res: *mut fmpz,
        poly: *const fmpz,
        e: ulong,
        trunc: slong,
        p: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_pow_trunc_binexp) for this function.
    #[link_name = "fmpz_mod_poly_pow_trunc_binexp"]
    pub fn fmpz_mod_poly_pow_trunc_binexp(
        res: *const fmpz_mod_poly,
        poly: *const fmpz_mod_poly,
        e: ulong,
        trunc: slong,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_powmod_ui_binexp) for this function.
    #[link_name = "_fmpz_mod_poly_powmod_ui_binexp"]
    pub fn _fmpz_mod_poly_powmod_ui_binexp(
        res: *mut fmpz,
        poly: *const fmpz,
        e: ulong,
        f: *const fmpz,
        lenf: slong,
        p: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_powmod_ui_binexp) for this function.
    #[link_name = "fmpz_mod_poly_powmod_ui_binexp"]
    pub fn fmpz_mod_poly_powmod_ui_binexp(
        res: *const fmpz_mod_poly,
        poly: *const fmpz_mod_poly,
        e: ulong,
        f: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_powmod_ui_binexp_preinv) for this function.
    #[link_name = "_fmpz_mod_poly_powmod_ui_binexp_preinv"]
    pub fn _fmpz_mod_poly_powmod_ui_binexp_preinv(
        res: *mut fmpz,
        poly: *const fmpz,
        e: ulong,
        f: *const fmpz,
        lenf: slong,
        finv: *const fmpz,
        lenfinv: slong,
        p: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_powmod_ui_binexp_preinv) for this function.
    #[link_name = "fmpz_mod_poly_powmod_ui_binexp_preinv"]
    pub fn fmpz_mod_poly_powmod_ui_binexp_preinv(
        res: *const fmpz_mod_poly,
        poly: *const fmpz_mod_poly,
        e: ulong,
        f: *const fmpz_mod_poly,
        finv: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_powmod_fmpz_binexp) for this function.
    #[link_name = "_fmpz_mod_poly_powmod_fmpz_binexp"]
    pub fn _fmpz_mod_poly_powmod_fmpz_binexp(
        res: *mut fmpz,
        poly: *const fmpz,
        e: *const fmpz,
        f: *const fmpz,
        lenf: slong,
        p: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_powmod_fmpz_binexp) for this function.
    #[link_name = "fmpz_mod_poly_powmod_fmpz_binexp"]
    pub fn fmpz_mod_poly_powmod_fmpz_binexp(
        res: *const fmpz_mod_poly,
        poly: *const fmpz_mod_poly,
        e: *const fmpz,
        f: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_powmod_fmpz_binexp_preinv) for this function.
    #[link_name = "_fmpz_mod_poly_powmod_fmpz_binexp_preinv"]
    pub fn _fmpz_mod_poly_powmod_fmpz_binexp_preinv(
        res: *mut fmpz,
        poly: *const fmpz,
        e: *const fmpz,
        f: *const fmpz,
        lenf: slong,
        finv: *const fmpz,
        lenfinv: slong,
        p: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_powmod_fmpz_binexp_preinv) for this function.
    #[link_name = "fmpz_mod_poly_powmod_fmpz_binexp_preinv"]
    pub fn fmpz_mod_poly_powmod_fmpz_binexp_preinv(
        res: *const fmpz_mod_poly,
        poly: *const fmpz_mod_poly,
        e: *const fmpz,
        f: *const fmpz_mod_poly,
        finv: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_powmod_x_fmpz_preinv) for this function.
    #[link_name = "_fmpz_mod_poly_powmod_x_fmpz_preinv"]
    pub fn _fmpz_mod_poly_powmod_x_fmpz_preinv(
        res: *mut fmpz,
        e: *const fmpz,
        f: *const fmpz,
        lenf: slong,
        finv: *const fmpz,
        lenfinv: slong,
        p: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_powmod_x_fmpz_preinv) for this function.
    #[link_name = "fmpz_mod_poly_powmod_x_fmpz_preinv"]
    pub fn fmpz_mod_poly_powmod_x_fmpz_preinv(
        res: *const fmpz_mod_poly,
        e: *const fmpz,
        f: *const fmpz_mod_poly,
        finv: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_powers_mod_preinv_naive) for this function.
    #[link_name = "_fmpz_mod_poly_powers_mod_preinv_naive"]
    pub fn _fmpz_mod_poly_powers_mod_preinv_naive(
        res: *mut *mut fmpz,
        f: *const fmpz,
        flen: slong,
        n: slong,
        g: *const fmpz,
        glen: slong,
        ginv: *const fmpz,
        ginvlen: slong,
        p: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_powers_mod_naive) for this function.
    #[link_name = "fmpz_mod_poly_powers_mod_naive"]
    pub fn fmpz_mod_poly_powers_mod_naive(
        res: *mut fmpz_mod_poly,
        f: *const fmpz_mod_poly,
        n: slong,
        g: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_powers_mod_bsgs) for this function.
    #[link_name = "fmpz_mod_poly_powers_mod_bsgs"]
    pub fn fmpz_mod_poly_powers_mod_bsgs(
        res: *mut fmpz_mod_poly,
        f: *const fmpz_mod_poly,
        n: slong,
        g: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_divrem_basecase) for this function.
    #[link_name = "_fmpz_mod_poly_divrem_basecase"]
    pub fn _fmpz_mod_poly_divrem_basecase(
        Q: *mut fmpz,
        R: *mut fmpz,
        A: *const fmpz,
        lenA: slong,
        B: *const fmpz,
        lenB: slong,
        invB: *const fmpz,
        p: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_divrem_basecase) for this function.
    #[link_name = "fmpz_mod_poly_divrem_basecase"]
    pub fn fmpz_mod_poly_divrem_basecase(
        Q: *const fmpz_mod_poly,
        R: *const fmpz_mod_poly,
        A: *const fmpz_mod_poly,
        B: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_divrem_newton_n_preinv) for this function.
    #[link_name = "_fmpz_mod_poly_divrem_newton_n_preinv"]
    pub fn _fmpz_mod_poly_divrem_newton_n_preinv(
        Q: *mut fmpz,
        R: *mut fmpz,
        A: *const fmpz,
        lenA: slong,
        B: *const fmpz,
        lenB: slong,
        Binv: *const fmpz,
        lenBinv: slong,
        mod_: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_divrem_newton_n_preinv) for this function.
    #[link_name = "fmpz_mod_poly_divrem_newton_n_preinv"]
    pub fn fmpz_mod_poly_divrem_newton_n_preinv(
        Q: *const fmpz_mod_poly,
        R: *const fmpz_mod_poly,
        A: *const fmpz_mod_poly,
        B: *const fmpz_mod_poly,
        Binv: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_div_basecase) for this function.
    #[link_name = "_fmpz_mod_poly_div_basecase"]
    pub fn _fmpz_mod_poly_div_basecase(
        Q: *mut fmpz,
        R: *mut fmpz,
        A: *const fmpz,
        lenA: slong,
        B: *const fmpz,
        lenB: slong,
        invB: *const fmpz,
        p: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_div_basecase) for this function.
    #[link_name = "fmpz_mod_poly_div_basecase"]
    pub fn fmpz_mod_poly_div_basecase(
        Q: *const fmpz_mod_poly,
        A: *const fmpz_mod_poly,
        B: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_div_newton_n_preinv) for this function.
    #[link_name = "_fmpz_mod_poly_div_newton_n_preinv"]
    pub fn _fmpz_mod_poly_div_newton_n_preinv(
        Q: *mut fmpz,
        A: *const fmpz,
        lenA: slong,
        B: *const fmpz,
        lenB: slong,
        Binv: *const fmpz,
        lenBinv: slong,
        mod_: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_div_newton_n_preinv) for this function.
    #[link_name = "fmpz_mod_poly_div_newton_n_preinv"]
    pub fn fmpz_mod_poly_div_newton_n_preinv(
        Q: *const fmpz_mod_poly,
        A: *const fmpz_mod_poly,
        B: *const fmpz_mod_poly,
        Binv: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_remove) for this function.
    #[link_name = "fmpz_mod_poly_remove"]
    pub fn fmpz_mod_poly_remove(f: *const fmpz_mod_poly, g: *const fmpz_mod_poly) -> ulong;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_rem_basecase) for this function.
    #[link_name = "_fmpz_mod_poly_rem_basecase"]
    pub fn _fmpz_mod_poly_rem_basecase(
        R: *mut fmpz,
        A: *const fmpz,
        lenA: slong,
        B: *const fmpz,
        lenB: slong,
        invB: *const fmpz,
        p: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_rem_basecase) for this function.
    #[link_name = "fmpz_mod_poly_rem_basecase"]
    pub fn fmpz_mod_poly_rem_basecase(
        R: *const fmpz_mod_poly,
        A: *const fmpz_mod_poly,
        B: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_divrem_divconquer_recursive) for this function.
    #[link_name = "_fmpz_mod_poly_divrem_divconquer_recursive"]
    pub fn _fmpz_mod_poly_divrem_divconquer_recursive(
        Q: *mut fmpz,
        BQ: *mut fmpz,
        W: *mut fmpz,
        A: *const fmpz,
        B: *const fmpz,
        lenB: slong,
        invB: *const fmpz,
        p: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_divrem_divconquer) for this function.
    #[link_name = "_fmpz_mod_poly_divrem_divconquer"]
    pub fn _fmpz_mod_poly_divrem_divconquer(
        Q: *mut fmpz,
        R: *mut fmpz,
        A: *const fmpz,
        lenA: slong,
        B: *const fmpz,
        lenB: slong,
        invB: *const fmpz,
        p: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_divrem_divconquer) for this function.
    #[link_name = "fmpz_mod_poly_divrem_divconquer"]
    pub fn fmpz_mod_poly_divrem_divconquer(
        Q: *const fmpz_mod_poly,
        R: *const fmpz_mod_poly,
        A: *const fmpz_mod_poly,
        B: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_divrem) for this function.
    #[link_name = "_fmpz_mod_poly_divrem"]
    pub fn _fmpz_mod_poly_divrem(
        Q: *mut fmpz,
        R: *mut fmpz,
        A: *const fmpz,
        lenA: slong,
        B: *const fmpz,
        lenB: slong,
        invB: *const fmpz,
        p: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_divrem) for this function.
    #[link_name = "fmpz_mod_poly_divrem"]
    pub fn fmpz_mod_poly_divrem(
        Q: *const fmpz_mod_poly,
        R: *const fmpz_mod_poly,
        A: *const fmpz_mod_poly,
        B: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_divrem_f) for this function.
    #[link_name = "fmpz_mod_poly_divrem_f"]
    pub fn fmpz_mod_poly_divrem_f(
        f: *mut fmpz,
        Q: *const fmpz_mod_poly,
        R: *const fmpz_mod_poly,
        A: *const fmpz_mod_poly,
        B: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_rem) for this function.
    #[link_name = "_fmpz_mod_poly_rem"]
    pub fn _fmpz_mod_poly_rem(
        R: *mut fmpz,
        A: *const fmpz,
        lenA: slong,
        B: *const fmpz,
        lenB: slong,
        invB: *const fmpz,
        p: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_rem_f) for this function.
    #[link_name = "_fmpz_mod_poly_rem_f"]
    pub fn _fmpz_mod_poly_rem_f(
        f: *mut fmpz,
        R: *mut fmpz,
        A: *const fmpz,
        lenA: slong,
        B: *const fmpz,
        lenB: slong,
        invB: *const fmpz,
        p: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_rem) for this function.
    #[link_name = "fmpz_mod_poly_rem"]
    pub fn fmpz_mod_poly_rem(
        R: *const fmpz_mod_poly,
        A: *const fmpz_mod_poly,
        B: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_inv_series_newton) for this function.
    #[link_name = "_fmpz_mod_poly_inv_series_newton"]
    pub fn _fmpz_mod_poly_inv_series_newton(
        Qinv: *mut fmpz,
        Q: *const fmpz,
        n: slong,
        cinv: *const fmpz,
        p: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_inv_series_newton) for this function.
    #[link_name = "fmpz_mod_poly_inv_series_newton"]
    pub fn fmpz_mod_poly_inv_series_newton(
        Qinv: *const fmpz_mod_poly,
        Q: *const fmpz_mod_poly,
        n: slong,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_inv_series_newton_f) for this function.
    #[link_name = "fmpz_mod_poly_inv_series_newton_f"]
    pub fn fmpz_mod_poly_inv_series_newton_f(
        f: *mut fmpz,
        Qinv: *const fmpz_mod_poly,
        Q: *const fmpz_mod_poly,
        n: slong,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_inv_series) for this function.
    #[link_name = "_fmpz_mod_poly_inv_series"]
    pub fn _fmpz_mod_poly_inv_series(
        Qinv: *mut fmpz,
        Q: *const fmpz,
        n: slong,
        cinv: *const fmpz,
        p: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_inv_series) for this function.
    #[link_name = "fmpz_mod_poly_inv_series"]
    pub fn fmpz_mod_poly_inv_series(Qinv: *const fmpz_mod_poly, Q: *const fmpz_mod_poly, n: slong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_inv_series_f) for this function.
    #[link_name = "fmpz_mod_poly_inv_series_f"]
    pub fn fmpz_mod_poly_inv_series_f(
        f: *mut fmpz,
        Qinv: *const fmpz_mod_poly,
        Q: *const fmpz_mod_poly,
        n: slong,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_div_series) for this function.
    #[link_name = "_fmpz_mod_poly_div_series"]
    pub fn _fmpz_mod_poly_div_series(
        Q: *mut fmpz,
        A: *const fmpz,
        Alen: slong,
        B: *const fmpz,
        Blen: slong,
        p: *const fmpz,
        n: slong,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_div_series) for this function.
    #[link_name = "fmpz_mod_poly_div_series"]
    pub fn fmpz_mod_poly_div_series(
        Q: *const fmpz_mod_poly,
        A: *const fmpz_mod_poly,
        B: *const fmpz_mod_poly,
        n: slong,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_make_monic) for this function.
    #[link_name = "fmpz_mod_poly_make_monic"]
    pub fn fmpz_mod_poly_make_monic(res: *const fmpz_mod_poly, poly: *const fmpz_mod_poly);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_make_monic_f) for this function.
    #[link_name = "fmpz_mod_poly_make_monic_f"]
    pub fn fmpz_mod_poly_make_monic_f(
        f: *mut fmpz,
        res: *const fmpz_mod_poly,
        poly: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_gcd_euclidean) for this function.
    #[link_name = "_fmpz_mod_poly_gcd_euclidean"]
    pub fn _fmpz_mod_poly_gcd_euclidean(
        G: *mut fmpz,
        A: *const fmpz,
        lenA: slong,
        B: *const fmpz,
        lenB: slong,
        invB: *const fmpz,
        p: *const fmpz,
    ) -> slong;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_gcd_euclidean) for this function.
    #[link_name = "fmpz_mod_poly_gcd_euclidean"]
    pub fn fmpz_mod_poly_gcd_euclidean(
        G: *const fmpz_mod_poly,
        A: *const fmpz_mod_poly,
        B: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_gcd) for this function.
    #[link_name = "_fmpz_mod_poly_gcd"]
    pub fn _fmpz_mod_poly_gcd(
        G: *mut fmpz,
        A: *const fmpz,
        lenA: slong,
        B: *const fmpz,
        lenB: slong,
        invB: *const fmpz,
        p: *const fmpz,
    ) -> slong;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_gcd) for this function.
    #[link_name = "fmpz_mod_poly_gcd"]
    pub fn fmpz_mod_poly_gcd(
        G: *const fmpz_mod_poly,
        A: *const fmpz_mod_poly,
        B: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_gcd_euclidean_f) for this function.
    #[link_name = "_fmpz_mod_poly_gcd_euclidean_f"]
    pub fn _fmpz_mod_poly_gcd_euclidean_f(
        f: *mut fmpz,
        G: *mut fmpz,
        A: *const fmpz,
        lenA: slong,
        B: *const fmpz,
        lenB: slong,
        p: *const fmpz,
    ) -> slong;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_gcd_euclidean_f) for this function.
    #[link_name = "fmpz_mod_poly_gcd_euclidean_f"]
    pub fn fmpz_mod_poly_gcd_euclidean_f(
        f: *mut fmpz,
        G: *const fmpz_mod_poly,
        A: *const fmpz_mod_poly,
        B: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_gcd_f) for this function.
    #[link_name = "_fmpz_mod_poly_gcd_f"]
    pub fn _fmpz_mod_poly_gcd_f(
        f: *mut fmpz,
        G: *mut fmpz,
        A: *const fmpz,
        lenA: slong,
        B: *const fmpz,
        lenB: slong,
        p: *const fmpz,
    ) -> slong;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_gcd_f) for this function.
    #[link_name = "fmpz_mod_poly_gcd_f"]
    pub fn fmpz_mod_poly_gcd_f(
        f: *mut fmpz,
        G: *const fmpz_mod_poly,
        A: *const fmpz_mod_poly,
        B: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_hgcd) for this function.
    #[link_name = "_fmpz_mod_poly_hgcd"]
    pub fn _fmpz_mod_poly_hgcd(
        M: *mut *mut fmpz,
        lenM: *const isize,
        A: *mut fmpz,
        lenA: *const isize,
        B: *mut fmpz,
        lenB: *const isize,
        a: *const fmpz,
        lena: slong,
        b: *const fmpz,
        lenb: slong,
        mod_: *const fmpz,
    ) -> slong;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_gcd_hgcd) for this function.
    #[link_name = "_fmpz_mod_poly_gcd_hgcd"]
    pub fn _fmpz_mod_poly_gcd_hgcd(
        G: *mut fmpz,
        A: *const fmpz,
        lenA: slong,
        B: *const fmpz,
        lenB: slong,
        mod_: *const fmpz,
    ) -> slong;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_gcd_hgcd) for this function.
    #[link_name = "fmpz_mod_poly_gcd_hgcd"]
    pub fn fmpz_mod_poly_gcd_hgcd(
        G: *const fmpz_mod_poly,
        A: *const fmpz_mod_poly,
        B: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_xgcd_euclidean) for this function.
    #[link_name = "_fmpz_mod_poly_xgcd_euclidean"]
    pub fn _fmpz_mod_poly_xgcd_euclidean(
        G: *mut fmpz,
        S: *mut fmpz,
        T: *mut fmpz,
        A: *const fmpz,
        lenA: slong,
        B: *const fmpz,
        lenB: slong,
        invB: *const fmpz,
        p: *const fmpz,
    ) -> slong;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_xgcd_euclidean_f) for this function.
    #[link_name = "_fmpz_mod_poly_xgcd_euclidean_f"]
    pub fn _fmpz_mod_poly_xgcd_euclidean_f(
        f: *mut fmpz,
        G: *mut fmpz,
        S: *mut fmpz,
        T: *mut fmpz,
        A: *const fmpz,
        lenA: slong,
        B: *const fmpz,
        lenB: slong,
        invB: *const fmpz,
        p: *const fmpz,
    ) -> slong;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_xgcd_euclidean) for this function.
    #[link_name = "fmpz_mod_poly_xgcd_euclidean"]
    pub fn fmpz_mod_poly_xgcd_euclidean(
        G: *const fmpz_mod_poly,
        S: *const fmpz_mod_poly,
        T: *const fmpz_mod_poly,
        A: *const fmpz_mod_poly,
        B: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_xgcd_euclidean_f) for this function.
    #[link_name = "fmpz_mod_poly_xgcd_euclidean_f"]
    pub fn fmpz_mod_poly_xgcd_euclidean_f(
        f: *mut fmpz,
        G: *const fmpz_mod_poly,
        S: *const fmpz_mod_poly,
        T: *const fmpz_mod_poly,
        A: *const fmpz_mod_poly,
        B: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_xgcd_hgcd) for this function.
    #[link_name = "_fmpz_mod_poly_xgcd_hgcd"]
    pub fn _fmpz_mod_poly_xgcd_hgcd(
        G: *mut fmpz,
        S: *mut fmpz,
        T: *mut fmpz,
        A: *const fmpz,
        lenA: slong,
        B: *const fmpz,
        lenB: slong,
        mod_: *const fmpz,
    ) -> slong;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_xgcd_hgcd) for this function.
    #[link_name = "fmpz_mod_poly_xgcd_hgcd"]
    pub fn fmpz_mod_poly_xgcd_hgcd(
        G: *const fmpz_mod_poly,
        S: *const fmpz_mod_poly,
        T: *const fmpz_mod_poly,
        A: *const fmpz_mod_poly,
        B: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_xgcd) for this function.
    #[link_name = "_fmpz_mod_poly_xgcd"]
    pub fn _fmpz_mod_poly_xgcd(
        G: *mut fmpz,
        S: *mut fmpz,
        T: *mut fmpz,
        A: *const fmpz,
        lenA: slong,
        B: *const fmpz,
        lenB: slong,
        invB: *const fmpz,
        p: *const fmpz,
    ) -> slong;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_xgcd) for this function.
    #[link_name = "fmpz_mod_poly_xgcd"]
    pub fn fmpz_mod_poly_xgcd(
        G: *const fmpz_mod_poly,
        S: *const fmpz_mod_poly,
        T: *const fmpz_mod_poly,
        A: *const fmpz_mod_poly,
        B: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_xgcd_f) for this function.
    #[link_name = "fmpz_mod_poly_xgcd_f"]
    pub fn fmpz_mod_poly_xgcd_f(
        f: *mut fmpz,
        G: *const fmpz_mod_poly,
        S: *const fmpz_mod_poly,
        T: *const fmpz_mod_poly,
        A: *const fmpz_mod_poly,
        B: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_gcdinv_euclidean) for this function.
    #[link_name = "_fmpz_mod_poly_gcdinv_euclidean"]
    pub fn _fmpz_mod_poly_gcdinv_euclidean(
        G: *mut fmpz,
        S: *mut fmpz,
        A: *const fmpz,
        lenA: slong,
        B: *const fmpz,
        lenB: slong,
        p: *const fmpz,
    ) -> slong;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_gcdinv_euclidean) for this function.
    #[link_name = "fmpz_mod_poly_gcdinv_euclidean"]
    pub fn fmpz_mod_poly_gcdinv_euclidean(
        G: *const fmpz_mod_poly,
        S: *const fmpz_mod_poly,
        A: *const fmpz_mod_poly,
        B: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_gcdinv_euclidean_f) for this function.
    #[link_name = "_fmpz_mod_poly_gcdinv_euclidean_f"]
    pub fn _fmpz_mod_poly_gcdinv_euclidean_f(
        f: *mut fmpz,
        G: *mut fmpz,
        S: *mut fmpz,
        A: *const fmpz,
        lenA: slong,
        B: *const fmpz,
        lenB: slong,
        p: *const fmpz,
    ) -> slong;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_gcdinv_euclidean_f) for this function.
    #[link_name = "fmpz_mod_poly_gcdinv_euclidean_f"]
    pub fn fmpz_mod_poly_gcdinv_euclidean_f(
        f: *mut fmpz,
        G: *const fmpz_mod_poly,
        S: *const fmpz_mod_poly,
        A: *const fmpz_mod_poly,
        B: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_gcdinv) for this function.
    #[link_name = "_fmpz_mod_poly_gcdinv"]
    pub fn _fmpz_mod_poly_gcdinv(
        G: *mut fmpz,
        S: *mut fmpz,
        A: *const fmpz,
        lenA: slong,
        B: *const fmpz,
        lenB: slong,
        p: *const fmpz,
    ) -> slong;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_gcdinv_f) for this function.
    #[link_name = "_fmpz_mod_poly_gcdinv_f"]
    pub fn _fmpz_mod_poly_gcdinv_f(
        f: *mut fmpz,
        G: *mut fmpz,
        S: *mut fmpz,
        A: *const fmpz,
        lenA: slong,
        B: *const fmpz,
        lenB: slong,
        p: *const fmpz,
    ) -> slong;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_gcdinv) for this function.
    #[link_name = "fmpz_mod_poly_gcdinv"]
    pub fn fmpz_mod_poly_gcdinv(
        G: *const fmpz_mod_poly,
        S: *const fmpz_mod_poly,
        A: *const fmpz_mod_poly,
        B: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_gcdinv_f) for this function.
    #[link_name = "fmpz_mod_poly_gcdinv_f"]
    pub fn fmpz_mod_poly_gcdinv_f(
        f: *mut fmpz,
        G: *const fmpz_mod_poly,
        S: *const fmpz_mod_poly,
        A: *const fmpz_mod_poly,
        B: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_invmod) for this function.
    #[link_name = "_fmpz_mod_poly_invmod"]
    pub fn _fmpz_mod_poly_invmod(
        A: *mut fmpz,
        B: *const fmpz,
        lenB: slong,
        P: *const fmpz,
        lenP: slong,
        p: *const fmpz,
    ) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_invmod_f) for this function.
    #[link_name = "_fmpz_mod_poly_invmod_f"]
    pub fn _fmpz_mod_poly_invmod_f(
        f: *mut fmpz,
        A: *mut fmpz,
        B: *const fmpz,
        lenB: slong,
        P: *const fmpz,
        lenP: slong,
        p: *const fmpz,
    ) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_invmod) for this function.
    #[link_name = "fmpz_mod_poly_invmod"]
    pub fn fmpz_mod_poly_invmod(
        A: *const fmpz_mod_poly,
        B: *const fmpz_mod_poly,
        P: *const fmpz_mod_poly,
    ) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_invmod_f) for this function.
    #[link_name = "fmpz_mod_poly_invmod_f"]
    pub fn fmpz_mod_poly_invmod_f(
        f: *mut fmpz,
        A: *const fmpz_mod_poly,
        B: *const fmpz_mod_poly,
        P: *const fmpz_mod_poly,
    ) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_minpoly_bm) for this function.
    #[link_name = "_fmpz_mod_poly_minpoly_bm"]
    pub fn _fmpz_mod_poly_minpoly_bm(
        poly: *mut fmpz,
        seq: *const fmpz,
        len: slong,
        p: *const fmpz,
    ) -> slong;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_minpoly_bm) for this function.
    #[link_name = "fmpz_mod_poly_minpoly_bm"]
    pub fn fmpz_mod_poly_minpoly_bm(poly: *const fmpz_mod_poly, seq: *const fmpz, len: slong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_minpoly_hgcd) for this function.
    #[link_name = "_fmpz_mod_poly_minpoly_hgcd"]
    pub fn _fmpz_mod_poly_minpoly_hgcd(
        poly: *mut fmpz,
        seq: *const fmpz,
        len: slong,
        p: *const fmpz,
    ) -> slong;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_minpoly_hgcd) for this function.
    #[link_name = "fmpz_mod_poly_minpoly_hgcd"]
    pub fn fmpz_mod_poly_minpoly_hgcd(poly: *const fmpz_mod_poly, seq: *const fmpz, len: slong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_minpoly) for this function.
    #[link_name = "_fmpz_mod_poly_minpoly"]
    pub fn _fmpz_mod_poly_minpoly(
        poly: *mut fmpz,
        seq: *const fmpz,
        len: slong,
        p: *const fmpz,
    ) -> slong;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_minpoly) for this function.
    #[link_name = "fmpz_mod_poly_minpoly"]
    pub fn fmpz_mod_poly_minpoly(poly: *const fmpz_mod_poly, seq: *const fmpz, len: slong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_resultant_euclidean) for this function.
    #[link_name = "_fmpz_mod_poly_resultant_euclidean"]
    pub fn _fmpz_mod_poly_resultant_euclidean(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: slong,
        poly2: *const fmpz,
        len2: slong,
        mod_: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_resultant_euclidean) for this function.
    #[link_name = "fmpz_mod_poly_resultant_euclidean"]
    pub fn fmpz_mod_poly_resultant_euclidean(
        r: *mut fmpz,
        f: *const fmpz_mod_poly,
        g: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_resultant_hgcd) for this function.
    #[link_name = "_fmpz_mod_poly_resultant_hgcd"]
    pub fn _fmpz_mod_poly_resultant_hgcd(
        res: *mut fmpz,
        A: *const fmpz,
        lenA: slong,
        B: *const fmpz,
        lenB: slong,
        mod_: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_resultant_hgcd) for this function.
    #[link_name = "fmpz_mod_poly_resultant_hgcd"]
    pub fn fmpz_mod_poly_resultant_hgcd(
        res: *mut fmpz,
        f: *const fmpz_mod_poly,
        g: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_resultant) for this function.
    #[link_name = "_fmpz_mod_poly_resultant"]
    pub fn _fmpz_mod_poly_resultant(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: slong,
        poly2: *const fmpz,
        len2: slong,
        mod_: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_resultant) for this function.
    #[link_name = "fmpz_mod_poly_resultant"]
    pub fn fmpz_mod_poly_resultant(
        res: *mut fmpz,
        f: *const fmpz_mod_poly,
        g: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_discriminant) for this function.
    #[link_name = "_fmpz_mod_poly_discriminant"]
    pub fn _fmpz_mod_poly_discriminant(
        d: *mut fmpz,
        poly: *const fmpz,
        len: slong,
        mod_: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_discriminant) for this function.
    #[link_name = "fmpz_mod_poly_discriminant"]
    pub fn fmpz_mod_poly_discriminant(d: *mut fmpz, f: *const fmpz_mod_poly);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_derivative) for this function.
    #[link_name = "_fmpz_mod_poly_derivative"]
    pub fn _fmpz_mod_poly_derivative(res: *mut fmpz, poly: *const fmpz, len: slong, p: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_derivative) for this function.
    #[link_name = "fmpz_mod_poly_derivative"]
    pub fn fmpz_mod_poly_derivative(res: *const fmpz_mod_poly, poly: *const fmpz_mod_poly);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_evaluate_fmpz) for this function.
    #[link_name = "_fmpz_mod_poly_evaluate_fmpz"]
    pub fn _fmpz_mod_poly_evaluate_fmpz(
        res: *mut fmpz,
        poly: *const fmpz,
        len: slong,
        a: *const fmpz,
        p: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_evaluate_fmpz) for this function.
    #[link_name = "fmpz_mod_poly_evaluate_fmpz"]
    pub fn fmpz_mod_poly_evaluate_fmpz(res: *mut fmpz, poly: *const fmpz_mod_poly, a: *const fmpz);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_evaluate_fmpz_vec_iter) for this function.
    #[link_name = "_fmpz_mod_poly_evaluate_fmpz_vec_iter"]
    pub fn _fmpz_mod_poly_evaluate_fmpz_vec_iter(
        ys: *mut fmpz,
        coeffs: *const fmpz,
        len: slong,
        xs: *const fmpz,
        n: slong,
        mod_: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_evaluate_fmpz_vec_iter) for this function.
    #[link_name = "fmpz_mod_poly_evaluate_fmpz_vec_iter"]
    pub fn fmpz_mod_poly_evaluate_fmpz_vec_iter(
        ys: *mut fmpz,
        poly: *const fmpz_mod_poly,
        xs: *const fmpz,
        n: slong,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_evaluate_fmpz_vec_fast_precomp) for this function.
    #[link_name = "_fmpz_mod_poly_evaluate_fmpz_vec_fast_precomp"]
    pub fn _fmpz_mod_poly_evaluate_fmpz_vec_fast_precomp(
        vs: *mut fmpz,
        poly: *const fmpz,
        plen: slong,
        tree: *const *const fmpz_poly,
        len: slong,
        mod_: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_evaluate_fmpz_vec_fast) for this function.
    #[link_name = "_fmpz_mod_poly_evaluate_fmpz_vec_fast"]
    pub fn _fmpz_mod_poly_evaluate_fmpz_vec_fast(
        ys: *mut fmpz,
        poly: *const fmpz,
        plen: slong,
        xs: *const fmpz,
        n: slong,
        mod_: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_evaluate_fmpz_vec_fast) for this function.
    #[link_name = "fmpz_mod_poly_evaluate_fmpz_vec_fast"]
    pub fn fmpz_mod_poly_evaluate_fmpz_vec_fast(
        ys: *mut fmpz,
        poly: *const fmpz_mod_poly,
        xs: *const fmpz,
        n: slong,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_evaluate_fmpz_vec) for this function.
    #[link_name = "_fmpz_mod_poly_evaluate_fmpz_vec"]
    pub fn _fmpz_mod_poly_evaluate_fmpz_vec(
        ys: *mut fmpz,
        coeffs: *const fmpz,
        len: slong,
        xs: *const fmpz,
        n: slong,
        mod_: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_evaluate_fmpz_vec) for this function.
    #[link_name = "fmpz_mod_poly_evaluate_fmpz_vec"]
    pub fn fmpz_mod_poly_evaluate_fmpz_vec(
        ys: *mut fmpz,
        poly: *const fmpz_mod_poly,
        xs: *const fmpz,
        n: slong,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_compose_horner) for this function.
    #[link_name = "_fmpz_mod_poly_compose_horner"]
    pub fn _fmpz_mod_poly_compose_horner(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: slong,
        poly2: *const fmpz,
        len2: slong,
        p: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_compose_horner) for this function.
    #[link_name = "fmpz_mod_poly_compose_horner"]
    pub fn fmpz_mod_poly_compose_horner(
        res: *const fmpz_mod_poly,
        poly1: *const fmpz_mod_poly,
        poly2: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_compose_divconquer) for this function.
    #[link_name = "_fmpz_mod_poly_compose_divconquer"]
    pub fn _fmpz_mod_poly_compose_divconquer(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: slong,
        poly2: *const fmpz,
        len2: slong,
        p: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_compose_divconquer) for this function.
    #[link_name = "fmpz_mod_poly_compose_divconquer"]
    pub fn fmpz_mod_poly_compose_divconquer(
        res: *const fmpz_mod_poly,
        poly1: *const fmpz_mod_poly,
        poly2: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_compose) for this function.
    #[link_name = "_fmpz_mod_poly_compose"]
    pub fn _fmpz_mod_poly_compose(
        res: *mut fmpz,
        poly1: *const fmpz,
        len1: slong,
        poly2: *const fmpz,
        len2: slong,
        p: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_compose) for this function.
    #[link_name = "fmpz_mod_poly_compose"]
    pub fn fmpz_mod_poly_compose(
        res: *const fmpz_mod_poly,
        poly1: *const fmpz_mod_poly,
        poly2: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_compose_mod) for this function.
    #[link_name = "_fmpz_mod_poly_compose_mod"]
    pub fn _fmpz_mod_poly_compose_mod(
        res: *mut fmpz,
        f: *const fmpz,
        lenf: slong,
        g: *const fmpz,
        h: *const fmpz,
        lenh: slong,
        p: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_compose_mod) for this function.
    #[link_name = "fmpz_mod_poly_compose_mod"]
    pub fn fmpz_mod_poly_compose_mod(
        res: *const fmpz_mod_poly,
        f: *const fmpz_mod_poly,
        g: *const fmpz_mod_poly,
        h: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_compose_mod_horner) for this function.
    #[link_name = "_fmpz_mod_poly_compose_mod_horner"]
    pub fn _fmpz_mod_poly_compose_mod_horner(
        res: *mut fmpz,
        f: *const fmpz,
        lenf: slong,
        g: *const fmpz,
        h: *const fmpz,
        lenh: slong,
        p: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_compose_mod_horner) for this function.
    #[link_name = "fmpz_mod_poly_compose_mod_horner"]
    pub fn fmpz_mod_poly_compose_mod_horner(
        res: *const fmpz_mod_poly,
        f: *const fmpz_mod_poly,
        g: *const fmpz_mod_poly,
        h: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_compose_mod_brent_kung) for this function.
    #[link_name = "_fmpz_mod_poly_compose_mod_brent_kung"]
    pub fn _fmpz_mod_poly_compose_mod_brent_kung(
        res: *mut fmpz,
        f: *const fmpz,
        len1: slong,
        g: *const fmpz,
        h: *const fmpz,
        len3: slong,
        p: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_compose_mod_brent_kung) for this function.
    #[link_name = "fmpz_mod_poly_compose_mod_brent_kung"]
    pub fn fmpz_mod_poly_compose_mod_brent_kung(
        res: *const fmpz_mod_poly,
        f: *const fmpz_mod_poly,
        g: *const fmpz_mod_poly,
        h: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_compose_mod_brent_kung_preinv) for this function.
    #[link_name = "_fmpz_mod_poly_compose_mod_brent_kung_preinv"]
    pub fn _fmpz_mod_poly_compose_mod_brent_kung_preinv(
        res: *mut fmpz,
        f: *const fmpz,
        lenf: slong,
        g: *const fmpz,
        h: *const fmpz,
        lenh: slong,
        hinv: *const fmpz,
        lenhinv: slong,
        p: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_compose_mod_brent_kung_preinv) for this function.
    #[link_name = "fmpz_mod_poly_compose_mod_brent_kung_preinv"]
    pub fn fmpz_mod_poly_compose_mod_brent_kung_preinv(
        res: *const fmpz_mod_poly,
        f: *const fmpz_mod_poly,
        g: *const fmpz_mod_poly,
        h: *const fmpz_mod_poly,
        hinv: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_compose_mod_brent_kung_vec_preinv) for this function.
    #[link_name = "_fmpz_mod_poly_compose_mod_brent_kung_vec_preinv"]
    pub fn _fmpz_mod_poly_compose_mod_brent_kung_vec_preinv(
        res: *mut fmpz_mod_poly,
        polys: *const fmpz_mod_poly,
        len1: slong,
        l: slong,
        g: *const fmpz,
        glen: slong,
        h: *const fmpz,
        lenh: slong,
        hinv: *const fmpz,
        lenhinv: slong,
        p: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_compose_mod_brent_kung_vec_preinv) for this function.
    #[link_name = "fmpz_mod_poly_compose_mod_brent_kung_vec_preinv"]
    pub fn fmpz_mod_poly_compose_mod_brent_kung_vec_preinv(
        res: *mut fmpz_mod_poly,
        polys: *const fmpz_mod_poly,
        len1: slong,
        n: slong,
        g: *const fmpz_mod_poly,
        h: *const fmpz_mod_poly,
        hinv: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_compose_mod_brent_kung_vec_preinv_threaded) for this function.
    #[link_name = "fmpz_mod_poly_compose_mod_brent_kung_vec_preinv_threaded"]
    pub fn fmpz_mod_poly_compose_mod_brent_kung_vec_preinv_threaded(
        res: *mut fmpz_mod_poly,
        polys: *const fmpz_mod_poly,
        len1: slong,
        n: slong,
        g: *const fmpz_mod_poly,
        poly: *const fmpz_mod_poly,
        polyinv: *const fmpz_mod_poly,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_tree_alloc) for this function.
    #[link_name = "_fmpz_mod_poly_tree_alloc"]
    pub fn _fmpz_mod_poly_tree_alloc(len: slong) -> *mut *mut fmpz_poly;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_tree_free) for this function.
    #[link_name = "_fmpz_mod_poly_tree_free"]
    pub fn _fmpz_mod_poly_tree_free(tree: *mut *mut fmpz_poly, len: slong);

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_tree_build) for this function.
    #[link_name = "_fmpz_mod_poly_tree_build"]
    pub fn _fmpz_mod_poly_tree_build(
        tree: *mut *mut fmpz_poly,
        roots: *const fmpz,
        len: slong,
        mod_: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_radix_init) for this function.
    #[link_name = "_fmpz_mod_poly_radix_init"]
    pub fn _fmpz_mod_poly_radix_init(
        Rpow: *mut *mut fmpz,
        Rinv: *mut *mut fmpz,
        R: *const fmpz,
        lenR: slong,
        k: slong,
        invL: *const fmpz,
        p: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_radix) for this function.
    #[link_name = "_fmpz_mod_poly_radix"]
    pub fn _fmpz_mod_poly_radix(
        B: *mut *mut fmpz,
        F: *const fmpz,
        Rpow: *mut *mut fmpz,
        Rinv: *mut *mut fmpz,
        degR: slong,
        k: slong,
        i: slong,
        W: *mut fmpz,
        p: *const fmpz,
    );

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c._fmpz_mod_poly_fprint) for this function.
    #[link_name = "_fmpz_mod_poly_fprint"]
    pub fn _fmpz_mod_poly_fprint(
        file: *mut FILE,
        poly: *const fmpz,
        len: slong,
        p: *const fmpz,
    ) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_fprint) for this function.
    #[link_name = "fmpz_mod_poly_fprint"]
    pub fn fmpz_mod_poly_fprint(file: *mut FILE, poly: *const fmpz_mod_poly) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_fprint_pretty) for this function.
    #[link_name = "fmpz_mod_poly_fprint_pretty"]
    pub fn fmpz_mod_poly_fprint_pretty(
        file: *mut FILE,
        poly: *const fmpz_mod_poly,
        x: *const c_char,
    ) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_print) for this function.
    #[link_name = "fmpz_mod_poly_print"]
    pub fn fmpz_mod_poly_print(poly: *const fmpz_mod_poly) -> c_int;

    /// See the [FLINT Documentation](http://flintlib.org/doc/fmpz_mod_poly.html#c.fmpz_mod_poly_print_pretty) for this function.
    #[link_name = "fmpz_mod_poly_print_pretty"]
    pub fn fmpz_mod_poly_print_pretty(poly: *const fmpz_mod_poly, x: *const c_char) -> c_int;

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::MaybeUninit;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn try_it() {
        let mut p: fmpz = fmpz::default();
        unsafe {
            fmpz_init(&mut p);
            fmpz_set_ui(&mut p, 17);
            debug_assert!(fmpz_print(&mut p) > 0);
            let mut f = MaybeUninit::uninit();
            fmpz_mod_poly_init(f.as_mut_ptr(), &p);
            let mut f = f.assume_init();
            fmpz_mod_poly_set_coeff_ui(&mut f, 1, 5);
            fmpz_mod_poly_set_coeff_ui(&mut f, 0, 5);
            let x = std::ffi::CString::new("x").unwrap();
            debug_assert!(fmpz_mod_poly_print_pretty(&f, x.as_ptr()) > 0);
        }
    }
}
