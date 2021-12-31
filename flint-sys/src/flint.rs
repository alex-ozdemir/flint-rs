#![allow(non_camel_case_types)]

use crate::deps::*;
use libc::{c_char, c_int, c_uint, c_void, size_t, FILE};

pub type __builtin_va_list = [__va_list_tag; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: c_uint,
    pub fp_offset: c_uint,
    pub overflow_arg_area: *mut c_void,
    pub reg_save_area: *mut c_void,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __locale_data {
    pub _address: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flint_rand_s {
    pub gmp_state: gmp_randstate_t,
    pub gmp_init: c_int,
    pub __randval: mp_limb_t,
    pub __randval2: mp_limb_t,
}

pub type flint_rand_t = [flint_rand_s; 1usize];
//pub type flint_rand_t = flint_rand_s;

pub type flint_bitcnt_t = mp_limb_t;

pub type flint_mpfr = __mpfr_struct;

pub type thread_pool_handle = c_int;

pub type flint_cleanup_function_t = ::std::option::Option<unsafe extern "C" fn()>;

extern "C" {
    pub static rec_word_tab: [c_int; 256usize];
    pub static mut flint_version: [c_char; 0usize];
    pub fn flint_malloc(size: size_t) -> *mut c_void;
    pub fn flint_realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    pub fn flint_calloc(num: size_t, size: size_t) -> *mut c_void;
    pub fn flint_free(ptr: *mut c_void);
    pub fn flint_register_cleanup_function(cleanup_function: flint_cleanup_function_t);
    pub fn flint_cleanup();
    pub fn flint_cleanup_master();
    pub fn __flint_set_memory_functions(
        alloc_func: ::std::option::Option<unsafe extern "C" fn(arg1: size_t) -> *mut c_void>,
        calloc_func: ::std::option::Option<
            unsafe extern "C" fn(arg1: size_t, arg2: size_t) -> *mut c_void,
        >,
        realloc_func: ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut c_void, arg2: size_t) -> *mut c_void,
        >,
        free_func: ::std::option::Option<unsafe extern "C" fn(arg1: *mut c_void)>,
    );
    pub fn flint_abort();
    pub fn flint_get_num_threads() -> c_int;
    pub fn flint_set_num_threads(num_threads: c_int);
    pub fn _flint_set_num_workers(num_workers: c_int);
    pub fn flint_set_num_workers(num_workers: c_int) -> c_int;
    pub fn flint_reset_num_workers(max_workers: c_int);
    pub fn flint_set_thread_affinity(cpus: *mut c_int, length: mp_limb_signed_t) -> c_int;
    pub fn flint_restore_thread_affinity() -> c_int;
    pub fn flint_test_multiplier() -> c_int;
    pub fn flint_randinit(state: *mut flint_rand_s);
    pub fn flint_randseed(state: *mut flint_rand_s, seed1: mp_limb_t, seed2: mp_limb_t);
    pub fn flint_get_randseed(
        seed1: *mut mp_limb_t,
        seed2: *mut mp_limb_t,
        state: *mut flint_rand_s,
    );
    pub fn _flint_rand_init_gmp(state: *mut flint_rand_s);
    pub fn flint_randclear(state: *mut flint_rand_s);
    pub fn flint_rand_alloc() -> *mut flint_rand_s;
    pub fn flint_rand_free(state: *mut flint_rand_s);
    pub fn parse_fmt(floating: *mut c_int, fmt: *const c_char) -> c_int;
    pub fn flint_printf(str_: *const c_char, ...) -> c_int;
    pub fn flint_vprintf(str_: *const c_char, ap: *mut __va_list_tag) -> c_int;
    pub fn flint_fprintf(f: *mut FILE, str_: *const c_char, ...) -> c_int;
    pub fn flint_sprintf(s: *mut c_char, str_: *const c_char, ...) -> c_int;
    pub fn flint_scanf(str_: *const c_char, ...) -> c_int;
    pub fn flint_fscanf(f: *mut FILE, str_: *const c_char, ...) -> c_int;
    pub fn flint_sscanf(s: *const c_char, str_: *const c_char, ...) -> c_int;
    pub fn flint_mul_sizes(x: mp_limb_signed_t, y: mp_limb_signed_t) -> mp_limb_signed_t;
}
