#![allow(non_camel_case_types)]

//! See the [FLINT documentation](http://flintlib.org/doc/fmpq_mat.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpq::fmpq;
use crate::fmpq_poly::fmpq_poly_struct;
use crate::fmpz::fmpz;
use crate::fmpz_mat::fmpz_mat_struct;
use libc::c_int;

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash)]
pub struct fmpq_mat_struct {
    pub entries: *mut fmpq,
    pub r: mp_limb_signed_t,
    pub c: mp_limb_signed_t,
    pub rows: *mut *mut fmpq,
}

pub type fmpq_mat_t = [fmpq_mat_struct; 1usize];

extern "C" {
    pub fn fmpq_mat_entry(
        mat: *const fmpq_mat_struct,
        i: mp_limb_signed_t,
        j: mp_limb_signed_t,
    ) -> *mut fmpq;
    pub fn fmpq_mat_entry_num(
        mat: *const fmpq_mat_struct,
        i: mp_limb_signed_t,
        j: mp_limb_signed_t,
    ) -> *mut fmpz;
    pub fn fmpq_mat_entry_den(
        mat: *const fmpq_mat_struct,
        i: mp_limb_signed_t,
        j: mp_limb_signed_t,
    ) -> *mut fmpz;
    pub fn fmpq_mat_nrows(mat: *const fmpq_mat_struct) -> mp_limb_signed_t;
    pub fn fmpq_mat_ncols(mat: *const fmpq_mat_struct) -> mp_limb_signed_t;
    pub fn fmpq_mat_init(mat: *mut fmpq_mat_struct, rows: mp_limb_signed_t, cols: mp_limb_signed_t);
    pub fn fmpq_mat_init_set(mat1: *mut fmpq_mat_struct, mat2: *const fmpq_mat_struct);
    pub fn fmpq_mat_clear(mat: *mut fmpq_mat_struct);
    pub fn fmpq_mat_swap(mat1: *mut fmpq_mat_struct, mat2: *mut fmpq_mat_struct);
    pub fn fmpq_mat_window_init(
        window: *mut fmpq_mat_struct,
        mat: *const fmpq_mat_struct,
        r1: mp_limb_signed_t,
        c1: mp_limb_signed_t,
        r2: mp_limb_signed_t,
        c2: mp_limb_signed_t,
    );
    pub fn fmpq_mat_window_clear(window: *mut fmpq_mat_struct);
    pub fn fmpq_mat_concat_horizontal(
        res: *mut fmpq_mat_struct,
        mat1: *const fmpq_mat_struct,
        mat2: *const fmpq_mat_struct,
    );
    pub fn fmpq_mat_concat_vertical(
        res: *mut fmpq_mat_struct,
        mat1: *const fmpq_mat_struct,
        mat2: *const fmpq_mat_struct,
    );
    pub fn fmpq_mat_print(mat: *const fmpq_mat_struct);
    pub fn fmpq_mat_randbits(
        mat: *mut fmpq_mat_struct,
        state: *const flint_rand_s,
        bits: mp_limb_t,
    );
    pub fn fmpq_mat_randtest(
        mat: *mut fmpq_mat_struct,
        state: *const flint_rand_s,
        bits: mp_limb_t,
    );
    pub fn fmpq_mat_hilbert_matrix(mat: *const fmpq_mat_struct);
    pub fn fmpq_mat_set(dest: *mut fmpq_mat_struct, src: *const fmpq_mat_struct);
    pub fn fmpq_mat_zero(mat: *mut fmpq_mat_struct);
    pub fn fmpq_mat_one(mat: *mut fmpq_mat_struct);
    pub fn fmpq_mat_transpose(rop: *mut fmpq_mat_struct, op: *const fmpq_mat_struct);
    pub fn fmpq_mat_add(
        mat: *mut fmpq_mat_struct,
        mat1: *const fmpq_mat_struct,
        mat2: *const fmpq_mat_struct,
    );
    pub fn fmpq_mat_sub(
        mat: *mut fmpq_mat_struct,
        mat1: *const fmpq_mat_struct,
        mat2: *const fmpq_mat_struct,
    );
    pub fn fmpq_mat_neg(rop: *mut fmpq_mat_struct, op: *const fmpq_mat_struct);
    pub fn fmpq_mat_scalar_mul_fmpq(
        rop: *mut fmpq_mat_struct,
        op: *const fmpq_mat_struct,
        x: *const fmpq,
    );
    pub fn fmpq_mat_scalar_mul_fmpz(
        rop: *mut fmpq_mat_struct,
        op: *const fmpq_mat_struct,
        x: *const fmpz,
    );
    pub fn fmpq_mat_scalar_div_fmpz(
        rop: *mut fmpq_mat_struct,
        op: *const fmpq_mat_struct,
        x: *const fmpz,
    );
    pub fn fmpq_mat_equal(mat1: *const fmpq_mat_struct, mat2: *const fmpq_mat_struct) -> c_int;
    pub fn fmpq_mat_is_integral(mat: *const fmpq_mat_struct) -> c_int;
    pub fn fmpq_mat_is_zero(mat: *const fmpq_mat_struct) -> c_int;
    pub fn fmpq_mat_is_one(mat: *const fmpq_mat_struct) -> c_int;
    pub fn fmpq_mat_is_empty(mat: *const fmpq_mat_struct) -> c_int;
    pub fn fmpq_mat_is_square(mat: *const fmpq_mat_struct) -> c_int;
    pub fn fmpq_mat_get_fmpz_mat(dest: *mut fmpz_mat_struct, mat: *const fmpq_mat_struct) -> c_int;
    pub fn fmpq_mat_get_fmpz_mat_entrywise(
        num: *mut fmpz_mat_struct,
        den: *mut fmpz_mat_struct,
        mat: *const fmpq_mat_struct,
    );
    pub fn fmpq_mat_get_fmpz_mat_matwise(
        num: *mut fmpz_mat_struct,
        den: *mut fmpz,
        mat: *const fmpq_mat_struct,
    );
    pub fn fmpq_mat_get_fmpz_mat_rowwise(
        num: *mut fmpz_mat_struct,
        den: *mut fmpz,
        mat: *const fmpq_mat_struct,
    );
    pub fn fmpq_mat_get_fmpz_mat_colwise(
        num: *mut fmpz_mat_struct,
        den: *mut fmpz,
        mat: *const fmpq_mat_struct,
    );
    pub fn fmpq_mat_get_fmpz_mat_rowwise_2(
        num: *mut fmpz_mat_struct,
        num2: *mut fmpz_mat_struct,
        den: *mut fmpz,
        mat: *const fmpq_mat_struct,
        mat2: *const fmpq_mat_struct,
    );
    pub fn fmpq_mat_get_fmpz_mat_mod_fmpz(
        dest: *mut fmpz_mat_struct,
        mat: *const fmpq_mat_struct,
        mod_: *const fmpz,
    );
    pub fn fmpq_mat_set_fmpz_mat(dest: *mut fmpq_mat_struct, src: *const fmpz_mat_struct);
    pub fn fmpq_mat_set_fmpz_mat_div_fmpz(
        X: *mut fmpq_mat_struct,
        Xmod: *const fmpz_mat_struct,
        div: *const fmpz,
    );
    pub fn fmpq_mat_set_fmpz_mat_mod_fmpz(
        X: *mut fmpq_mat_struct,
        Xmod: *const fmpz_mat_struct,
        mod_: *const fmpz,
    ) -> c_int;
    pub fn fmpq_mat_mul_direct(
        C: *mut fmpq_mat_struct,
        A: *const fmpq_mat_struct,
        B: *const fmpq_mat_struct,
    );
    pub fn fmpq_mat_mul_cleared(
        C: *mut fmpq_mat_struct,
        A: *const fmpq_mat_struct,
        B: *const fmpq_mat_struct,
    );
    pub fn fmpq_mat_mul(
        C: *mut fmpq_mat_struct,
        A: *const fmpq_mat_struct,
        B: *const fmpq_mat_struct,
    );
    pub fn fmpq_mat_mul_fmpz_mat(
        C: *mut fmpq_mat_struct,
        A: *const fmpq_mat_struct,
        B: *const fmpz_mat_struct,
    );
    pub fn fmpq_mat_mul_r_fmpz_mat(
        C: *mut fmpq_mat_struct,
        A: *const fmpz_mat_struct,
        B: *const fmpq_mat_struct,
    );
    pub fn fmpq_mat_kronecker_product(
        C: *mut fmpq_mat_struct,
        A: *const fmpq_mat_struct,
        B: *const fmpq_mat_struct,
    );
    pub fn fmpq_mat_swap_rows(
        mat: *mut fmpq_mat_struct,
        perm: *const mp_limb_signed_t,
        r: mp_limb_signed_t,
        s: mp_limb_signed_t,
    );
    pub fn fmpq_mat_invert_rows(mat: *mut fmpq_mat_struct, perm: *const mp_limb_signed_t);
    pub fn fmpq_mat_swap_cols(
        mat: *mut fmpq_mat_struct,
        perm: *const mp_limb_signed_t,
        r: mp_limb_signed_t,
        s: mp_limb_signed_t,
    );
    pub fn fmpq_mat_invert_cols(mat: *mut fmpq_mat_struct, perm: *const mp_limb_signed_t);
    pub fn fmpq_mat_trace(trace: *mut fmpq, mat: *const fmpq_mat_struct);
    pub fn fmpq_mat_det(det: *mut fmpq, mat: *const fmpq_mat_struct);
    pub fn fmpq_mat_solve_fmpz_mat_fraction_free(
        X: *mut fmpq_mat_struct,
        A: *const fmpz_mat_struct,
        B: *const fmpz_mat_struct,
    ) -> c_int;
    pub fn fmpq_mat_solve_fraction_free(
        X: *mut fmpq_mat_struct,
        A: *const fmpq_mat_struct,
        B: *const fmpq_mat_struct,
    ) -> c_int;
    pub fn fmpq_mat_solve_fmpz_mat_dixon(
        X: *mut fmpq_mat_struct,
        A: *const fmpz_mat_struct,
        B: *const fmpz_mat_struct,
    ) -> c_int;
    pub fn fmpq_mat_solve_dixon(
        X: *mut fmpq_mat_struct,
        A: *const fmpq_mat_struct,
        B: *const fmpq_mat_struct,
    ) -> c_int;
    pub fn fmpq_mat_solve_fmpz_mat_multi_mod(
        X: *mut fmpq_mat_struct,
        A: *const fmpz_mat_struct,
        B: *const fmpz_mat_struct,
    ) -> c_int;
    pub fn fmpq_mat_solve_multi_mod(
        X: *mut fmpq_mat_struct,
        A: *const fmpq_mat_struct,
        B: *const fmpq_mat_struct,
    ) -> c_int;
    pub fn fmpq_mat_can_solve_fmpz_mat_multi_mod(
        X: *mut fmpq_mat_struct,
        A: *const fmpz_mat_struct,
        B: *const fmpz_mat_struct,
    ) -> c_int;
    pub fn fmpq_mat_can_solve_multi_mod(
        X: *mut fmpq_mat_struct,
        A: *const fmpq_mat_struct,
        B: *const fmpq_mat_struct,
    ) -> c_int;
    pub fn fmpq_mat_solve_fmpz_mat(
        X: *mut fmpq_mat_struct,
        A: *const fmpz_mat_struct,
        B: *const fmpz_mat_struct,
    ) -> c_int;
    pub fn fmpq_mat_solve(
        X: *mut fmpq_mat_struct,
        A: *const fmpq_mat_struct,
        B: *const fmpq_mat_struct,
    ) -> c_int;
    pub fn fmpq_mat_inv(B: *mut fmpq_mat_struct, A: *const fmpq_mat_struct) -> c_int;
    pub fn fmpq_mat_pivot(
        perm: *mut mp_limb_signed_t,
        mat: *mut fmpq_mat_struct,
        r: mp_limb_signed_t,
        c: mp_limb_signed_t,
    ) -> c_int;
    pub fn fmpq_mat_rref_classical(
        B: *mut fmpq_mat_struct,
        A: *const fmpq_mat_struct,
    ) -> mp_limb_signed_t;
    pub fn fmpq_mat_rref_fraction_free(
        B: *mut fmpq_mat_struct,
        A: *const fmpq_mat_struct,
    ) -> mp_limb_signed_t;
    pub fn fmpq_mat_rref(B: *mut fmpq_mat_struct, A: *const fmpq_mat_struct) -> mp_limb_signed_t;
    pub fn fmpq_mat_gso(B: *mut fmpq_mat_struct, A: *const fmpq_mat_struct);
    pub fn fmpq_mat_similarity(A: *mut fmpq_mat_struct, r: mp_limb_signed_t, d: *const fmpq);
    pub fn _fmpq_mat_charpoly(coeffs: *mut fmpz, den: *mut fmpz, mat: *const fmpq_mat_struct);
    pub fn fmpq_mat_charpoly(pol: *mut fmpq_poly_struct, mat: *const fmpq_mat_struct);
    pub fn _fmpq_mat_minpoly(
        coeffs: *mut fmpz,
        den: *mut fmpz,
        mat: *const fmpq_mat_struct,
    ) -> mp_limb_signed_t;
    pub fn fmpq_mat_minpoly(pol: *mut fmpq_poly_struct, mat: *const fmpq_mat_struct);
}
