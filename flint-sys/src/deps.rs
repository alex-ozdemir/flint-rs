#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use libc::{c_int, c_long, c_uint, c_ulong, c_void, size_t};

// GMP

pub type slong = c_long;
pub type ulong = c_ulong;
pub type mp_limb_t = c_ulong;
pub type mp_limb_signed_t = c_long;
pub type mp_bitcnt_t = c_ulong;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __mpz_struct {
    pub _mp_alloc: c_int,
    pub _mp_size: c_int,
    pub _mp_d: *mut mp_limb_t,
}

pub type MP_INT = __mpz_struct;
pub type mpz_t = [__mpz_struct; 1usize];
pub type mp_ptr = *mut mp_limb_t;
pub type mp_srcptr = *const mp_limb_t;
pub type mp_size_t = c_long;
pub type mp_exp_t = c_long;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __mpq_struct {
    pub _mp_num: __mpz_struct,
    pub _mp_den: __mpz_struct,
}

pub type MP_RAT = __mpq_struct;
pub type mpq_t = [__mpq_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __mpf_struct {
    pub _mp_prec: c_int,
    pub _mp_size: c_int,
    pub _mp_exp: mp_exp_t,
    pub _mp_d: *mut mp_limb_t,
}

pub type mpf_t = [__mpf_struct; 1usize];
pub type gmp_randalg_t = c_uint;
pub const gmp_randalg_t_GMP_RAND_ALG_DEFAULT: gmp_randalg_t = 0;
pub const gmp_randalg_t_GMP_RAND_ALG_LC: gmp_randalg_t = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub union __gmp_randstate_struct__bindgen_ty_1 {
    pub _mp_lc: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __gmp_randstate_struct {
    pub _mp_seed: mpz_t,
    pub _mp_alg: gmp_randalg_t,
    pub _mp_algdata: __gmp_randstate_struct__bindgen_ty_1,
}

pub type gmp_randstate_t = [__gmp_randstate_struct; 1usize];
pub type mpz_srcptr = *const __mpz_struct;
pub type mpz_ptr = *mut __mpz_struct;
pub type mpf_srcptr = *const __mpf_struct;
pub type mpf_ptr = *mut __mpf_struct;
pub type mpq_srcptr = *const __mpq_struct;
pub type mpq_ptr = *mut __mpq_struct;

// MPFR

pub type mpfr_void = c_void;
pub type mpfr_int = c_int;
pub type mpfr_uint = c_uint;
pub type mpfr_long = c_long;
pub type mpfr_ulong = c_ulong;
pub type mpfr_size_t = size_t;
pub type mpfr_flags_t = c_uint;
pub const mpfr_rnd_t_MPFR_RNDN: mpfr_rnd_t = 0;
pub const mpfr_rnd_t_MPFR_RNDZ: mpfr_rnd_t = 1;
pub const mpfr_rnd_t_MPFR_RNDU: mpfr_rnd_t = 2;
pub const mpfr_rnd_t_MPFR_RNDD: mpfr_rnd_t = 3;
pub const mpfr_rnd_t_MPFR_RNDA: mpfr_rnd_t = 4;
pub const mpfr_rnd_t_MPFR_RNDF: mpfr_rnd_t = 5;
pub const mpfr_rnd_t_MPFR_RNDNA: mpfr_rnd_t = -1;
pub type mpfr_rnd_t = c_int;
pub type mpfr_prec_t = c_long;
pub type mpfr_uprec_t = c_ulong;
pub type mpfr_sign_t = c_int;
pub type mpfr_exp_t = c_long;
pub type mpfr_uexp_t = c_ulong;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __mpfr_struct {
    pub _mpfr_prec: mpfr_prec_t,
    pub _mpfr_sign: mpfr_sign_t,
    pub _mpfr_exp: mpfr_exp_t,
    pub _mpfr_d: *mut mp_limb_t,
}

pub type mpfr_t = [__mpfr_struct; 1usize];
pub type mpfr_ptr = *mut __mpfr_struct;
pub type mpfr_srcptr = *const __mpfr_struct;
pub const mpfr_kind_t_MPFR_NAN_KIND: mpfr_kind_t = 0;
pub const mpfr_kind_t_MPFR_INF_KIND: mpfr_kind_t = 1;
pub const mpfr_kind_t_MPFR_ZERO_KIND: mpfr_kind_t = 2;
pub const mpfr_kind_t_MPFR_REGULAR_KIND: mpfr_kind_t = 3;
pub type mpfr_kind_t = c_uint;
pub const mpfr_free_cache_t_MPFR_FREE_LOCAL_CACHE: mpfr_free_cache_t = 1;
pub const mpfr_free_cache_t_MPFR_FREE_GLOBAL_CACHE: mpfr_free_cache_t = 2;
pub type mpfr_free_cache_t = c_uint;
