#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

//! *See the [FLINT documentation](http://flintlib.org/doc/qsieve.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpz::{fmpz, fmpz_t};
use crate::fmpz_factor::fmpz_factor_struct;
use libc::{c_char, c_int, c_uchar, c_void, pthread_mutex_t, FILE};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct prime_t {
    pub pinv: mp_limb_t,
    pub p: c_int,
    pub size: c_char,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fac_t {
    pub ind: mp_limb_signed_t,
    pub exp: mp_limb_signed_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct la_col_t {
    pub data: *mut mp_limb_signed_t,
    pub weight: mp_limb_signed_t,
    pub orig: mp_limb_signed_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hash_t {
    pub prime: mp_limb_t,
    pub next: mp_limb_t,
    pub count: mp_limb_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct relation_t {
    pub lp: mp_limb_t,
    pub num_factors: mp_limb_signed_t,
    pub small_primes: mp_limb_signed_t,
    pub small: *mut mp_limb_signed_t,
    pub factor: *mut fac_t,
    pub Y: fmpz_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct qs_poly_s {
    pub B: fmpz_t,
    pub soln1: *mut c_int,
    pub soln2: *mut c_int,
    pub posn1: *mut c_int,
    pub posn2: *mut c_int,
    pub small: *mut mp_limb_signed_t,
    pub factor: *mut fac_t,
    pub num_factors: mp_limb_signed_t,
}

pub type qs_poly_t = [qs_poly_s; 1usize];

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qs_s {
    pub index_j: mp_limb_signed_t,
    pub mutex: pthread_mutex_t,
    pub handles: *mut thread_pool_handle,
    pub num_handles: mp_limb_signed_t,
    pub n: fmpz_t,
    pub bits: mp_limb_t,
    pub ks_primes: mp_limb_t,
    pub k: mp_limb_t,
    pub kn: fmpz_t,
    pub num_primes: mp_limb_signed_t,
    pub factor_base: *mut prime_t,
    pub sqrts: *mut c_int,
    pub small_primes: mp_limb_signed_t,
    pub second_prime: mp_limb_signed_t,
    pub sieve_size: mp_limb_signed_t,
    pub sieve_bits: c_uchar,
    pub sieve_fill: c_uchar,
    pub A: fmpz_t,
    pub B: fmpz_t,
    pub A_ind: *mut mp_limb_t,
    pub A_divp: *mut fmpz_t,
    pub B0_terms: *mut mp_limb_t,
    pub B_terms: *mut fmpz_t,
    pub A_inv: *mut mp_limb_t,
    pub A_inv2B: *mut *mut mp_limb_t,
    pub soln1: *mut c_int,
    pub soln2: *mut c_int,
    pub target_A: fmpz_t,
    pub upp_bound: fmpz_t,
    pub low_bound: fmpz_t,
    pub s: mp_limb_signed_t,
    pub low: mp_limb_signed_t,
    pub high: mp_limb_signed_t,
    pub span: mp_limb_signed_t,
    pub h: mp_limb_signed_t,
    pub m: mp_limb_signed_t,
    pub A_ind_diff: mp_limb_signed_t,
    pub curr_subset: *mut mp_limb_t,
    pub first_subset: *mut mp_limb_t,
    pub j: mp_limb_t,
    pub poly: *mut qs_poly_s,
    pub siqs: *mut FILE,
    pub fname: *mut c_char,
    pub full_relation: mp_limb_signed_t,
    pub num_cycles: mp_limb_signed_t,
    pub vertices: mp_limb_signed_t,
    pub components: mp_limb_signed_t,
    pub edges: mp_limb_signed_t,
    pub table_size: mp_limb_signed_t,
    pub table: *mut hash_t,
    pub hash_table: *mut mp_limb_t,
    pub extra_rels: mp_limb_signed_t,
    pub max_factors: mp_limb_signed_t,
    pub Y_arr: *mut fmpz,
    pub curr_rel: *mut mp_limb_signed_t,
    pub relation: *mut mp_limb_signed_t,
    pub buffer_size: mp_limb_signed_t,
    pub num_relations: mp_limb_signed_t,
    pub small_factor: mp_limb_t,
    pub matrix: *mut la_col_t,
    pub qsort_arr: *mut *mut la_col_t,
    pub columns: mp_limb_signed_t,
    pub prime_count: *mut mp_limb_signed_t,
}

pub type qs_t = [qs_s; 1usize];

extern "C" {
    pub static mut qsieve_tune: [[mp_limb_t; 6usize]; 30usize];
    pub fn qsieve_init(qs_inf: *mut qs_s, n: *mut fmpz);
    pub fn qsieve_knuth_schroeppel(qs_inf: *mut qs_s) -> mp_limb_t;
    pub fn qsieve_clear(qs_inf: *mut qs_s);
    pub fn qsieve_factor(factors: *mut fmpz_factor_struct, n: *mut fmpz);
    pub fn compute_factor_base(
        small_factor: *mut mp_limb_t,
        qs_inf: *mut qs_s,
        num_primes: mp_limb_signed_t,
    ) -> *mut prime_t;
    pub fn qsieve_primes_init(qs_inf: *mut qs_s) -> mp_limb_t;
    pub fn qsieve_primes_increment(qs_inf: *mut qs_s, delta: mp_limb_t) -> mp_limb_t;
    pub fn qsieve_poly_init(qs_inf: *mut qs_s) -> mp_limb_t;
    pub fn qsieve_init_A(qs_inf: *mut qs_s) -> c_int;
    pub fn qsieve_reinit_A(qs_inf: *mut qs_s);
    pub fn qsieve_next_A(qs_inf: *mut qs_s) -> c_int;
    pub fn qsieve_init_poly_first(qs_inf: *mut qs_s);
    pub fn qsieve_init_poly_next(qs_inf: *mut qs_s, i: mp_limb_signed_t);
    pub fn qsieve_compute_C(C: *mut fmpz, qs_inf: *mut qs_s, poly: *mut qs_poly_s);
    pub fn qsieve_poly_copy(poly: *mut qs_poly_s, qs_inf: *mut qs_s);
    pub fn qsieve_poly_clear(qs_inf: *mut qs_s);
    pub fn qsieve_do_sieving(qs_inf: *mut qs_s, sieve: *mut c_uchar, poly: *mut qs_poly_s);
    pub fn qsieve_do_sieving2(qs_inf: *mut qs_s, sieve: *mut c_uchar, poly: *mut qs_poly_s);
    pub fn qsieve_evaluate_candidate(
        qs_inf: *mut qs_s,
        i: mp_limb_t,
        sieve: *mut c_uchar,
        poly: *mut qs_poly_s,
    ) -> mp_limb_signed_t;
    pub fn qsieve_evaluate_sieve(
        qs_inf: *mut qs_s,
        sieve: *mut c_uchar,
        poly: *mut qs_poly_s,
    ) -> mp_limb_signed_t;
    pub fn qsieve_collect_relations(qs_inf: *mut qs_s, sieve: *mut c_uchar) -> mp_limb_signed_t;
    pub fn qsieve_linalg_init(qs_inf: *mut qs_s);
    pub fn qsieve_linalg_realloc(qs_inf: *mut qs_s);
    pub fn qsieve_linalg_clear(qs_inf: *mut qs_s);
    pub fn qsieve_relations_cmp(a: *const c_void, b: *const c_void) -> c_int;
    pub fn qsieve_merge_relations(qs_inf: *mut qs_s) -> mp_limb_signed_t;
    pub fn qsieve_write_to_file(
        qs_inf: *mut qs_s,
        prime: mp_limb_t,
        Y: *mut fmpz,
        poly: *mut qs_poly_s,
    );
    pub fn qsieve_get_table_entry(qs_inf: *mut qs_s, prime: mp_limb_t) -> *mut hash_t;
    pub fn qsieve_add_to_hashtable(qs_inf: *mut qs_s, prime: mp_limb_t);
    pub fn qsieve_parse_relation(qs_inf: *mut qs_s, str_: *mut c_char) -> relation_t;
    pub fn qsieve_merge_relation(qs_inf: *mut qs_s, a: relation_t, b: relation_t) -> relation_t;
    pub fn qsieve_compare_relation(a: *const c_void, b: *const c_void) -> c_int;
    pub fn qsieve_remove_duplicates(
        rel_list: *mut relation_t,
        num_relations: mp_limb_signed_t,
    ) -> c_int;
    pub fn qsieve_insert_relation(
        qs_inf: *mut qs_s,
        rel_list: *mut relation_t,
        num_relations: mp_limb_signed_t,
    );
    pub fn qsieve_process_relation(qs_inf: *mut qs_s) -> c_int;
    pub fn get_null_entry(nullrows: *mut u64, i: mp_limb_signed_t, l: mp_limb_signed_t) -> u64;
    pub fn reduce_matrix(
        qs_inf: *mut qs_s,
        nrows: *mut mp_limb_signed_t,
        ncols: *mut mp_limb_signed_t,
        cols: *mut la_col_t,
    );
    pub fn block_lanczos(
        state: *mut flint_rand_s,
        nrows: mp_limb_signed_t,
        dense_rows: mp_limb_signed_t,
        ncols: mp_limb_signed_t,
        B: *mut la_col_t,
    ) -> *mut u64;
    pub fn qsieve_square_root(
        X: *mut fmpz,
        Y: *mut fmpz,
        qs_inf: *mut qs_s,
        nullrows: *mut u64,
        ncols: mp_limb_signed_t,
        l: mp_limb_signed_t,
        N: *mut fmpz,
    );
}
