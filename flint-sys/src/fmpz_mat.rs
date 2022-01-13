#![allow(non_camel_case_types)]
// TODO: nmod

//! See the [FLINT documentation](http://flintlib.org/doc/fmpz_mat.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpq::fmpq;
use crate::fmpz::fmpz;
use crate::fmpz_poly::fmpz_poly_struct;
use libc::{c_int, FILE};

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash)]
pub struct fmpz_mat_struct {
    pub entries: *mut fmpz,
    pub r: mp_limb_signed_t,
    pub c: mp_limb_signed_t,
    pub rows: *mut *mut fmpz,
}

pub type fmpz_mat_t = [fmpz_mat_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct col_hash_t {
    pub col: mp_limb_t,
    pub hash: mp_limb_t,
}

extern "C" {
    pub fn fmpz_mat_entry(
        mat: *const fmpz_mat_struct,
        i: mp_limb_signed_t,
        j: mp_limb_signed_t,
    ) -> *mut fmpz;
    pub fn fmpz_mat_nrows(mat: *const fmpz_mat_struct) -> mp_limb_signed_t;
    pub fn fmpz_mat_ncols(mat: *const fmpz_mat_struct) -> mp_limb_signed_t;
    pub fn fmpz_mat_init(mat: *mut fmpz_mat_struct, rows: mp_limb_signed_t, cols: mp_limb_signed_t);
    pub fn fmpz_mat_init_set(mat: *mut fmpz_mat_struct, src: *const fmpz_mat_struct);
    pub fn fmpz_mat_swap(mat1: *mut fmpz_mat_struct, mat2: *mut fmpz_mat_struct);
    pub fn fmpz_mat_set(mat1: *mut fmpz_mat_struct, mat2: *const fmpz_mat_struct);
    pub fn fmpz_mat_clear(mat: *mut fmpz_mat_struct);
    pub fn fmpz_mat_equal(mat1: *const fmpz_mat_struct, mat2: *const fmpz_mat_struct) -> c_int;
    pub fn fmpz_mat_is_zero(mat: *const fmpz_mat_struct) -> c_int;
    pub fn fmpz_mat_is_one(mat: *const fmpz_mat_struct) -> c_int;
    pub fn fmpz_mat_is_zero_row(mat: *const fmpz_mat_struct, i: mp_limb_signed_t) -> c_int;
    pub fn fmpz_mat_col_equal(
        M: *const fmpz_mat_struct,
        m: mp_limb_signed_t,
        n: mp_limb_signed_t,
    ) -> c_int;
    pub fn fmpz_mat_row_equal(
        M: *const fmpz_mat_struct,
        m: mp_limb_signed_t,
        n: mp_limb_signed_t,
    ) -> c_int;
    pub fn fmpz_mat_is_empty(mat: *const fmpz_mat_struct) -> c_int;
    pub fn fmpz_mat_is_square(mat: *const fmpz_mat_struct) -> c_int;
    pub fn fmpz_mat_zero(mat: *mut fmpz_mat_struct);
    pub fn fmpz_mat_one(mat: *mut fmpz_mat_struct);
    pub fn fmpz_mat_window_init(
        window: *mut fmpz_mat_struct,
        mat: *const fmpz_mat_struct,
        r1: mp_limb_signed_t,
        c1: mp_limb_signed_t,
        r2: mp_limb_signed_t,
        c2: mp_limb_signed_t,
    );
    pub fn fmpz_mat_window_clear(window: *mut fmpz_mat_struct);
    pub fn fmpz_mat_concat_horizontal(
        res: *mut fmpz_mat_struct,
        mat1: *const fmpz_mat_struct,
        mat2: *const fmpz_mat_struct,
    );
    pub fn fmpz_mat_concat_vertical(
        res: *mut fmpz_mat_struct,
        mat1: *const fmpz_mat_struct,
        mat2: *const fmpz_mat_struct,
    );
    pub fn fmpz_mat_fprint(file: *mut FILE, mat: *const fmpz_mat_struct) -> c_int;
    pub fn fmpz_mat_fprint_pretty(file: *mut FILE, mat: *const fmpz_mat_struct) -> c_int;
    pub fn fmpz_mat_print(mat: *const fmpz_mat_struct) -> c_int;
    pub fn fmpz_mat_print_pretty(mat: *const fmpz_mat_struct) -> c_int;
    pub fn fmpz_mat_fread(file: *const FILE, mat: *mut fmpz_mat_struct) -> c_int;
    pub fn fmpz_mat_read(mat: *mut fmpz_mat_struct) -> c_int;
    pub fn fmpz_mat_randbits(
        mat: *mut fmpz_mat_struct,
        state: *const flint_rand_s,
        bits: mp_limb_t,
    );
    pub fn fmpz_mat_randtest(
        mat: *mut fmpz_mat_struct,
        state: *const flint_rand_s,
        bits: mp_limb_t,
    );
    pub fn fmpz_mat_randtest_unsigned(
        mat: *mut fmpz_mat_struct,
        state: *const flint_rand_s,
        bits: mp_limb_t,
    );
    pub fn fmpz_mat_randintrel(
        mat: *mut fmpz_mat_struct,
        state: *const flint_rand_s,
        bits: mp_limb_t,
    );
    pub fn fmpz_mat_randsimdioph(
        mat: *mut fmpz_mat_struct,
        state: *const flint_rand_s,
        bits: mp_limb_t,
        bits2: mp_limb_t,
    );
    pub fn fmpz_mat_randntrulike(
        mat: *mut fmpz_mat_struct,
        state: *const flint_rand_s,
        bits: mp_limb_t,
        q: mp_limb_t,
    );
    pub fn fmpz_mat_randntrulike2(
        mat: *mut fmpz_mat_struct,
        state: *const flint_rand_s,
        bits: mp_limb_t,
        q: mp_limb_t,
    );
    pub fn fmpz_mat_randajtai(mat: *mut fmpz_mat_struct, state: *const flint_rand_s, alpha: f64);
    pub fn fmpz_mat_randrank(
        mat: *mut fmpz_mat_struct,
        state: *const flint_rand_s,
        rank: mp_limb_signed_t,
        bits: mp_limb_t,
    );
    pub fn fmpz_mat_randdet(
        mat: *mut fmpz_mat_struct,
        state: *const flint_rand_s,
        det: *const fmpz,
    );
    pub fn fmpz_mat_randops(
        mat: *mut fmpz_mat_struct,
        state: *const flint_rand_s,
        count: mp_limb_signed_t,
    );
    pub fn fmpz_mat_randpermdiag(
        mat: *mut fmpz_mat_struct,
        state: *const flint_rand_s,
        diag: *const fmpz,
        n: mp_limb_signed_t,
    ) -> c_int;
    pub fn fmpz_mat_max_bits(mat: *const fmpz_mat_struct) -> mp_limb_signed_t;
    pub fn fmpz_mat_transpose(B: *mut fmpz_mat_struct, A: *const fmpz_mat_struct);
    pub fn fmpz_mat_add(
        C: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        B: *const fmpz_mat_struct,
    );
    pub fn fmpz_mat_sub(
        C: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        B: *const fmpz_mat_struct,
    );
    pub fn fmpz_mat_neg(B: *mut fmpz_mat_struct, A: *const fmpz_mat_struct);
    pub fn fmpz_mat_scalar_mul_fmpz(
        B: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        c: *const fmpz,
    );
    pub fn fmpz_mat_scalar_mul_si(
        B: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        c: mp_limb_signed_t,
    );
    pub fn fmpz_mat_scalar_mul_ui(B: *mut fmpz_mat_struct, A: *const fmpz_mat_struct, c: mp_limb_t);
    pub fn fmpz_mat_scalar_addmul_fmpz(
        B: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        c: *const fmpz,
    );
    pub fn fmpz_mat_scalar_addmul_si(
        B: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        c: mp_limb_signed_t,
    );
    pub fn fmpz_mat_scalar_addmul_ui(
        B: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        c: mp_limb_t,
    );
    pub fn fmpz_mat_scalar_submul_fmpz(
        B: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        c: *const fmpz,
    );
    pub fn fmpz_mat_scalar_submul_si(
        B: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        c: mp_limb_signed_t,
    );
    pub fn fmpz_mat_scalar_submul_ui(
        B: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        c: mp_limb_t,
    );
    /*
    pub fn fmpz_mat_scalar_addmul_nmod_mat_fmpz(
        B: *mut fmpz_mat_struct,
        A: *mut nmod_mat_struct,
        c: *mut fmpz,
    );
    pub fn fmpz_mat_scalar_addmul_nmod_mat_ui(
        B: *mut fmpz_mat_struct,
        A: *mut nmod_mat_struct,
        c: mp_limb_t,
    );
    */
    pub fn fmpz_mat_scalar_divexact_fmpz(
        B: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        c: *const fmpz,
    );
    pub fn fmpz_mat_scalar_divexact_si(
        B: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        c: mp_limb_signed_t,
    );
    pub fn fmpz_mat_scalar_divexact_ui(
        B: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        c: mp_limb_t,
    );
    pub fn fmpz_mat_scalar_mul_2exp(
        B: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        exp: mp_limb_t,
    );
    pub fn fmpz_mat_scalar_tdiv_q_2exp(
        B: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        exp: mp_limb_t,
    );
    pub fn fmpz_mat_scalar_smod(B: *mut fmpz_mat_struct, A: *const fmpz_mat_struct, P: *const fmpz);
    pub fn fmpz_mat_scalar_mod_fmpz(
        B: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        m: *const fmpz,
    );
    pub fn fmpz_mat_mul(
        C: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        B: *const fmpz_mat_struct,
    );
    pub fn fmpz_mat_mul_classical(
        C: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        B: *const fmpz_mat_struct,
    );
    pub fn fmpz_mat_mul_strassen(
        C: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        B: *const fmpz_mat_struct,
    );
    pub fn fmpz_mat_mul_classical_inline(
        C: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        B: *const fmpz_mat_struct,
    );
    pub fn _fmpz_mat_mul_multi_mod(
        C: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        B: *const fmpz_mat_struct,
        bits: mp_limb_t,
    );
    pub fn fmpz_mat_mul_multi_mod(
        C: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        B: *const fmpz_mat_struct,
    );
    pub fn fmpz_mat_sqr_bodrato(B: *mut fmpz_mat_struct, A: *const fmpz_mat_struct);
    pub fn fmpz_mat_sqr(B: *mut fmpz_mat_struct, A: *const fmpz_mat_struct);
    pub fn fmpz_mat_pow(B: *mut fmpz_mat_struct, A: *const fmpz_mat_struct, exp: mp_limb_t);
    pub fn fmpz_mat_kronecker_product(
        C: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        B: *const fmpz_mat_struct,
    );
    pub fn fmpz_mat_content(ret: *mut fmpz, A: *const fmpz_mat_struct);
    pub fn fmpz_mat_swap_rows(
        mat: *mut fmpz_mat_struct,
        perm: *const mp_limb_signed_t,
        r: mp_limb_signed_t,
        s: mp_limb_signed_t,
    );
    pub fn fmpz_mat_invert_rows(mat: *mut fmpz_mat_struct, perm: *const mp_limb_signed_t);
    pub fn fmpz_mat_swap_cols(
        mat: *mut fmpz_mat_struct,
        perm: *const mp_limb_signed_t,
        r: mp_limb_signed_t,
        s: mp_limb_signed_t,
    );
    pub fn fmpz_mat_invert_cols(mat: *mut fmpz_mat_struct, perm: *const mp_limb_signed_t);
    pub fn fmpz_mat_find_pivot_any(
        mat: *const fmpz_mat_struct,
        start_row: mp_limb_signed_t,
        end_row: mp_limb_signed_t,
        c: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mat_find_pivot_smallest(
        mat: *const fmpz_mat_struct,
        start_row: mp_limb_signed_t,
        end_row: mp_limb_signed_t,
        c: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mat_fflu(
        B: *mut fmpz_mat_struct,
        den: *mut fmpz,
        perm: *const mp_limb_signed_t,
        A: *const fmpz_mat_struct,
        rank_check: c_int,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mat_rank_small_inplace(B: *mut fmpz_mat_struct) -> mp_limb_signed_t;
    pub fn fmpz_mat_rref(
        B: *mut fmpz_mat_struct,
        den: *mut fmpz,
        A: *const fmpz_mat_struct,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mat_rref_fflu(
        B: *mut fmpz_mat_struct,
        den: *mut fmpz,
        A: *const fmpz_mat_struct,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mat_rref_mul(
        B: *mut fmpz_mat_struct,
        den: *mut fmpz,
        A: *const fmpz_mat_struct,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mat_is_in_rref_with_rank(
        A: *mut fmpz_mat_struct,
        den: *mut fmpz,
        rank: mp_limb_signed_t,
    ) -> c_int;
    pub fn fmpz_mat_rref_mod(
        perm: *mut mp_limb_signed_t,
        A: *mut fmpz_mat_struct,
        p: *const fmpz,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mat_howell_form_mod(A: *mut fmpz_mat_struct, mod_: *const fmpz)
        -> mp_limb_signed_t;
    pub fn fmpz_mat_strong_echelon_form_mod(A: *mut fmpz_mat_struct, mod_: *const fmpz);
    pub fn fmpz_mat_trace(trace: *mut fmpz, mat: *const fmpz_mat_struct);
    pub fn fmpz_mat_det(det: *mut fmpz, A: *const fmpz_mat_struct);
    pub fn fmpz_mat_det_cofactor(det: *mut fmpz, A: *const fmpz_mat_struct);
    pub fn fmpz_mat_det_bareiss(det: *mut fmpz, A: *const fmpz_mat_struct);
    pub fn fmpz_mat_det_modular(det: *mut fmpz, A: *const fmpz_mat_struct, proved: c_int);
    pub fn fmpz_mat_det_modular_accelerated(
        det: *mut fmpz,
        A: *const fmpz_mat_struct,
        proved: c_int,
    );
    pub fn fmpz_mat_det_modular_given_divisor(
        det: *mut fmpz,
        A: *const fmpz_mat_struct,
        d: *const fmpz,
        proved: c_int,
    );
    pub fn fmpz_mat_det_bound(bound: *mut fmpz, A: *const fmpz_mat_struct);
    pub fn fmpz_mat_det_bound_nonzero(bound: *mut fmpz, A: *const fmpz_mat_struct);
    pub fn fmpz_mat_det_divisor(d: *mut fmpz, A: *const fmpz_mat_struct);
    pub fn fmpz_mat_similarity(A: *mut fmpz_mat_struct, r: mp_limb_signed_t, d: *const fmpz);
    pub fn _fmpz_mat_charpoly_berkowitz(rop: *mut fmpz, op: *const fmpz_mat_struct);
    pub fn fmpz_mat_charpoly_berkowitz(cp: *mut fmpz_poly_struct, mat: *const fmpz_mat_struct);
    pub fn _fmpz_mat_charpoly_modular(rop: *mut fmpz, op: *const fmpz_mat_struct);
    pub fn fmpz_mat_charpoly_modular(cp: *mut fmpz_poly_struct, mat: *const fmpz_mat_struct);
    pub fn _fmpz_mat_charpoly(cp: *mut fmpz, mat: *const fmpz_mat_struct);
    pub fn fmpz_mat_charpoly(cp: *mut fmpz_poly_struct, mat: *const fmpz_mat_struct);
    pub fn _fmpz_mat_minpoly_modular(
        rop: *mut fmpz,
        op: *const fmpz_mat_struct,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mat_minpoly_modular(cp: *mut fmpz_poly_struct, mat: *const fmpz_mat_struct);
    pub fn _fmpz_mat_minpoly(cp: *mut fmpz, mat: *const fmpz_mat_struct) -> mp_limb_signed_t;
    pub fn fmpz_mat_minpoly(cp: *mut fmpz_poly_struct, mat: *const fmpz_mat_struct);
    pub fn fmpz_mat_rank(A: *const fmpz_mat_struct) -> mp_limb_signed_t;
    pub fn fmpz_mat_solve_bound(
        N: *mut fmpz,
        D: *mut fmpz,
        A: *const fmpz_mat_struct,
        B: *const fmpz_mat_struct,
    );
    pub fn fmpz_mat_solve(
        X: *mut fmpz_mat_struct,
        den: *mut fmpz,
        A: *const fmpz_mat_struct,
        B: *const fmpz_mat_struct,
    ) -> c_int;
    pub fn fmpz_mat_solve_cramer(
        X: *mut fmpz_mat_struct,
        den: *mut fmpz,
        A: *const fmpz_mat_struct,
        B: *const fmpz_mat_struct,
    ) -> c_int;
    pub fn fmpz_mat_solve_fflu(
        X: *mut fmpz_mat_struct,
        den: *mut fmpz,
        A: *const fmpz_mat_struct,
        B: *const fmpz_mat_struct,
    ) -> c_int;
    pub fn fmpz_mat_solve_fflu_precomp(
        X: *mut fmpz_mat_struct,
        perm: *const mp_limb_signed_t,
        FFLU: *const fmpz_mat_struct,
        B: *const fmpz_mat_struct,
    );
    /*
    pub fn fmpz_mat_find_good_prime_and_invert(
        Ainv: *mut nmod_mat_struct,
        A: *mut fmpz_mat_struct,
        det_bound: *mut fmpz,
    ) -> mp_limb_t;
    */
    pub fn fmpz_mat_dixon_get_crt_primes(
        num_primes: *mut mp_limb_signed_t,
        A: *mut fmpz_mat_struct,
        p: mp_limb_t,
    ) -> *mut mp_limb_t;
    /*
    pub fn _fmpz_mat_solve_dixon(
        X: *mut fmpz_mat_struct,
        mod_: *mut fmpz,
        A: *mut fmpz_mat_struct,
        B: *mut fmpz_mat_struct,
        Ainv: *mut nmod_mat_struct,
        p: mp_limb_t,
        N: *mut fmpz,
        D: *mut fmpz,
    );*/
    pub fn fmpz_mat_solve_dixon(
        X: *mut fmpz_mat_struct,
        mod_: *mut fmpz,
        A: *const fmpz_mat_struct,
        B: *const fmpz_mat_struct,
    ) -> c_int;
    /*
    pub fn _fmpz_mat_solve_dixon_den(
        X: *mut fmpz_mat_struct,
        den: *mut fmpz,
        A: *mut fmpz_mat_struct,
        B: *mut fmpz_mat_struct,
        Ainv: *mut nmod_mat_struct,
        p: mp_limb_t,
        N: *mut fmpz,
        D: *mut fmpz,
    );*/
    pub fn fmpz_mat_solve_dixon_den(
        X: *mut fmpz_mat_struct,
        den: *mut fmpz,
        A: *const fmpz_mat_struct,
        B: *const fmpz_mat_struct,
    ) -> c_int;
    pub fn fmpz_mat_solve_multi_mod_den(
        X: *mut fmpz_mat_struct,
        den: *mut fmpz,
        A: *const fmpz_mat_struct,
        B: *const fmpz_mat_struct,
    ) -> c_int;
    pub fn fmpz_mat_can_solve_multi_mod_den(
        X: *mut fmpz_mat_struct,
        den: *mut fmpz,
        A: *const fmpz_mat_struct,
        B: *const fmpz_mat_struct,
    ) -> c_int;
    pub fn fmpz_mat_nullspace(
        res: *mut fmpz_mat_struct,
        mat: *const fmpz_mat_struct,
    ) -> mp_limb_signed_t;
    pub fn fmpz_mat_inv(
        B: *mut fmpz_mat_struct,
        den: *mut fmpz,
        A: *const fmpz_mat_struct,
    ) -> c_int;
    /*
    pub fn fmpz_mat_set_nmod_mat(A: *mut fmpz_mat_struct, Amod: *mut nmod_mat_struct);
    pub fn fmpz_mat_set_nmod_mat_unsigned(A: *mut fmpz_mat_struct, Amod: *mut nmod_mat_struct);
    pub fn fmpz_mat_get_nmod_mat(Amod: *mut nmod_mat_struct, A: *mut fmpz_mat_struct);
    pub fn fmpz_mat_CRT_ui(
        res: *mut fmpz_mat_struct,
        mat1: *mut fmpz_mat_struct,
        m1: *mut fmpz,
        mat2: *mut nmod_mat_struct,
        sign: c_int,
    );
    pub fn fmpz_mat_multi_mod_ui_precomp(
        residues: *mut nmod_mat_t,
        nres: mp_limb_signed_t,
        mat: *mut fmpz_mat_struct,
        comb: *mut fmpz_comb_struct,
        temp: *mut fmpz_comb_temp_struct,
    );
    pub fn fmpz_mat_multi_mod_ui(
        residues: *mut nmod_mat_t,
        nres: mp_limb_signed_t,
        mat: *mut fmpz_mat_struct,
    );
    pub fn fmpz_mat_multi_CRT_ui_precomp(
        mat: *mut fmpz_mat_struct,
        residues: *mut nmod_mat_t,
        nres: mp_limb_signed_t,
        comb: *mut fmpz_comb_struct,
        temp: *mut fmpz_comb_temp_struct,
        sign: c_int,
    );
    pub fn fmpz_mat_multi_CRT_ui(
        mat: *mut fmpz_mat_struct,
        residues: *mut nmod_mat_t,
        nres: mp_limb_signed_t,
        sign: c_int,
    );
    */
    pub fn fmpz_mat_hnf(H: *mut fmpz_mat_struct, A: *const fmpz_mat_struct);
    pub fn fmpz_mat_hnf_transform(
        H: *mut fmpz_mat_struct,
        U: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
    );
    pub fn fmpz_mat_hnf_classical(H: *mut fmpz_mat_struct, A: *const fmpz_mat_struct);
    pub fn fmpz_mat_hnf_xgcd(H: *mut fmpz_mat_struct, A: *const fmpz_mat_struct);
    pub fn fmpz_mat_hnf_minors(H: *mut fmpz_mat_struct, A: *const fmpz_mat_struct);
    pub fn fmpz_mat_hnf_minors_transform(
        H: *mut fmpz_mat_struct,
        U: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
    );
    pub fn fmpz_mat_hnf_modular(H: *mut fmpz_mat_struct, A: *const fmpz_mat_struct, D: *const fmpz);
    pub fn fmpz_mat_hnf_modular_eldiv(A: *mut fmpz_mat_struct, D: *const fmpz);
    pub fn fmpz_mat_hnf_pernet_stein(
        H: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        state: *const flint_rand_s,
    );
    pub fn fmpz_mat_is_in_hnf(A: *const fmpz_mat_struct) -> c_int;
    pub fn fmpz_mat_snf(S: *mut fmpz_mat_struct, A: *const fmpz_mat_struct);
    pub fn fmpz_mat_snf_diagonal(S: *mut fmpz_mat_struct, A: *const fmpz_mat_struct);
    pub fn fmpz_mat_snf_kannan_bachem(S: *mut fmpz_mat_struct, A: *const fmpz_mat_struct);
    pub fn fmpz_mat_snf_iliopoulos(
        S: *mut fmpz_mat_struct,
        A: *const fmpz_mat_struct,
        mod_: *const fmpz,
    );
    pub fn fmpz_mat_is_in_snf(A: *const fmpz_mat_struct) -> c_int;
    pub fn fmpz_mat_is_hadamard(A: *const fmpz_mat_struct) -> c_int;
    pub fn fmpz_mat_hadamard(A: *mut fmpz_mat_struct) -> c_int;
    pub fn fmpz_mat_gram(B: *mut fmpz_mat_struct, A: *const fmpz_mat_struct);
    /*
    pub fn fmpz_mat_get_d_mat(
        B: *mut d_mat_struct,
        A: *mut fmpz_mat_struct,
    ) -> c_int;
    pub fn fmpz_mat_get_d_mat_transpose(
        B: *mut d_mat_struct,
        A: *mut fmpz_mat_struct,
    ) -> c_int;
    pub fn fmpz_mat_get_mpf_mat(B: *mut mpf_mat_struct, A: *mut fmpz_mat_struct);
    pub fn fmpz_mat_chol_d(R: *mut d_mat_struct, A: *mut fmpz_mat_struct);
    */
    pub fn fmpz_mat_is_reduced(A: *const fmpz_mat_struct, delta: f64, eta: f64) -> c_int;
    pub fn fmpz_mat_is_reduced_gram(A: *const fmpz_mat_struct, delta: f64, eta: f64) -> c_int;
    pub fn fmpz_mat_is_reduced_with_removal(
        A: *const fmpz_mat_struct,
        delta: f64,
        eta: f64,
        gs_B: *mut fmpz,
        newd: c_int,
    ) -> c_int;
    pub fn fmpz_mat_is_reduced_gram_with_removal(
        A: *const fmpz_mat_struct,
        delta: f64,
        eta: f64,
        gs_B: *const fmpz,
        newd: c_int,
    ) -> c_int;
    pub fn fmpz_mat_lll_original(A: *mut fmpz_mat_struct, delta: *const fmpq, eta: *const fmpq);
    pub fn fmpz_mat_lll_storjohann(A: *mut fmpz_mat_struct, delta: *const fmpq, eta: *const fmpq);
    pub fn fmpz_mat_col_partition(
        part: *mut mp_limb_signed_t,
        M: *const fmpz_mat_struct,
        short_circuit: c_int,
    ) -> c_int;
    pub fn fmpz_mat_next_col_van_hoeij(
        M: *mut fmpz_mat_struct,
        P: *mut fmpz,
        col: *mut fmpz_mat_struct,
        exp: mp_limb_signed_t,
        U_exp: mp_limb_signed_t,
    ) -> c_int;
}
