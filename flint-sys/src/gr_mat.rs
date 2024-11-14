/* automatically generated by rust-bindgen 0.70.1 */

use crate::deps::*;
use crate::flint::*;
use crate::fmpq_types::*;
use crate::fmpz_types::*;
use crate::gr::*;
use crate::gr_poly::*;


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gr_mat_struct {
    pub entries: gr_ptr,
    pub r: mp_limb_signed_t,
    pub c: mp_limb_signed_t,
    pub rows: *mut gr_ptr,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of gr_mat_struct"][::std::mem::size_of::<gr_mat_struct>() - 32usize];
    ["Alignment of gr_mat_struct"][::std::mem::align_of::<gr_mat_struct>() - 8usize];
    ["Offset of field: gr_mat_struct::entries"]
        [::std::mem::offset_of!(gr_mat_struct, entries) - 0usize];
    ["Offset of field: gr_mat_struct::r"][::std::mem::offset_of!(gr_mat_struct, r) - 8usize];
    ["Offset of field: gr_mat_struct::c"][::std::mem::offset_of!(gr_mat_struct, c) - 16usize];
    ["Offset of field: gr_mat_struct::rows"][::std::mem::offset_of!(gr_mat_struct, rows) - 24usize];
};
impl Default for gr_mat_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type gr_mat_t = [gr_mat_struct; 1usize];
pub type gr_method_mat_unary_op_get_scalar = ::std::option::Option<
    unsafe extern "C" fn(arg1: gr_ptr, arg2: *const gr_mat_struct, arg3: gr_ctx_ptr) -> libc::c_int,
>;
pub type gr_method_mat_unary_op = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut gr_mat_struct,
        arg2: *const gr_mat_struct,
        arg3: gr_ctx_ptr,
    ) -> libc::c_int,
>;
pub type gr_method_mat_binary_op = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut gr_mat_struct,
        arg2: *const gr_mat_struct,
        arg3: *const gr_mat_struct,
        arg4: gr_ctx_ptr,
    ) -> libc::c_int,
>;
pub type gr_method_mat_pivot_op = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut mp_limb_signed_t,
        arg2: *mut gr_mat_struct,
        arg3: mp_limb_signed_t,
        arg4: mp_limb_signed_t,
        arg5: mp_limb_signed_t,
        arg6: gr_ctx_ptr,
    ) -> libc::c_int,
>;
pub type gr_method_mat_diagonalization_op = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut gr_vec_struct,
        arg2: *mut gr_mat_struct,
        arg3: *mut gr_mat_struct,
        arg4: *const gr_mat_struct,
        arg5: libc::c_int,
        arg6: gr_ctx_ptr,
    ) -> libc::c_int,
>;
pub type gr_method_mat_lu_op = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *mut mp_limb_signed_t,
        arg2: *mut mp_limb_signed_t,
        arg3: *mut gr_mat_struct,
        arg4: *const gr_mat_struct,
        arg5: libc::c_int,
        arg6: gr_ctx_ptr,
    ) -> libc::c_int,
>;
extern "C" {
    #[link_name = "gr_mat_entry_ptr__extern"]
    pub fn gr_mat_entry_ptr(
        mat: *mut gr_mat_struct,
        i: mp_limb_signed_t,
        j: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> gr_ptr;
    #[link_name = "gr_mat_entry_srcptr__extern"]
    pub fn gr_mat_entry_srcptr(
        mat: *const gr_mat_struct,
        i: mp_limb_signed_t,
        j: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> gr_srcptr;
    pub fn gr_mat_init(
        mat: *mut gr_mat_struct,
        rows: mp_limb_signed_t,
        cols: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    );
    pub fn gr_mat_init_set(
        res: *mut gr_mat_struct,
        mat: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_clear(mat: *mut gr_mat_struct, ctx: *mut gr_ctx_struct);
    #[link_name = "gr_mat_swap__extern"]
    pub fn gr_mat_swap(mat1: *mut gr_mat_struct, mat2: *mut gr_mat_struct, ctx: *mut gr_ctx_struct);
    pub fn gr_mat_swap_rows(
        mat: *mut gr_mat_struct,
        perm: *mut mp_limb_signed_t,
        r: mp_limb_signed_t,
        s: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_invert_rows(
        mat: *mut gr_mat_struct,
        perm: *mut mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_swap_cols(
        mat: *mut gr_mat_struct,
        perm: *mut mp_limb_signed_t,
        r: mp_limb_signed_t,
        s: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_invert_cols(
        mat: *mut gr_mat_struct,
        perm: *mut mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_window_init(
        window: *mut gr_mat_struct,
        mat: *const gr_mat_struct,
        r1: mp_limb_signed_t,
        c1: mp_limb_signed_t,
        r2: mp_limb_signed_t,
        c2: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    );
    #[link_name = "gr_mat_window_clear__extern"]
    pub fn gr_mat_window_clear(window: *mut gr_mat_struct, ctx: *mut gr_ctx_struct);
    pub fn gr_mat_concat_horizontal(
        res: *mut gr_mat_struct,
        mat1: *const gr_mat_struct,
        mat2: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_concat_vertical(
        res: *mut gr_mat_struct,
        mat1: *const gr_mat_struct,
        mat2: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_write(
        out: *mut gr_stream_struct,
        mat: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_print(mat: *const gr_mat_struct, ctx: *mut gr_ctx_struct) -> libc::c_int;
    pub fn gr_mat_randtest(
        mat: *mut gr_mat_struct,
        state: *mut flint_rand_s,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_randops(
        mat: *mut gr_mat_struct,
        state: *mut flint_rand_s,
        count: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_randpermdiag(
        parity: *mut libc::c_int,
        mat: *mut gr_mat_struct,
        state: *mut flint_rand_s,
        diag: gr_ptr,
        n: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_randrank(
        mat: *mut gr_mat_struct,
        state: *mut flint_rand_s,
        rank: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    #[link_name = "gr_mat_is_empty__extern"]
    pub fn gr_mat_is_empty(mat: *const gr_mat_struct, ctx: *mut gr_ctx_struct) -> truth_t;
    #[link_name = "gr_mat_is_square__extern"]
    pub fn gr_mat_is_square(mat: *const gr_mat_struct, ctx: *mut gr_ctx_struct) -> truth_t;
    pub fn gr_mat_equal(
        mat1: *const gr_mat_struct,
        mat2: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> truth_t;
    pub fn gr_mat_is_zero(mat: *const gr_mat_struct, ctx: *mut gr_ctx_struct) -> truth_t;
    pub fn gr_mat_is_one(mat: *const gr_mat_struct, ctx: *mut gr_ctx_struct) -> truth_t;
    pub fn gr_mat_is_neg_one(mat: *const gr_mat_struct, ctx: *mut gr_ctx_struct) -> truth_t;
    pub fn gr_mat_zero(res: *mut gr_mat_struct, ctx: *mut gr_ctx_struct) -> libc::c_int;
    pub fn gr_mat_one(res: *mut gr_mat_struct, ctx: *mut gr_ctx_struct) -> libc::c_int;
    pub fn gr_mat_set(
        res: *mut gr_mat_struct,
        mat: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_set_scalar(
        res: *mut gr_mat_struct,
        c: gr_srcptr,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_set_ui(
        res: *mut gr_mat_struct,
        v: mp_limb_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_set_si(
        res: *mut gr_mat_struct,
        v: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_set_fmpz(
        res: *mut gr_mat_struct,
        v: *const fmpz,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_set_fmpq(
        res: *mut gr_mat_struct,
        v: *const fmpq,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_set_fmpz_mat(
        res: *mut gr_mat_struct,
        mat: *const fmpz_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_set_fmpq_mat(
        res: *mut gr_mat_struct,
        mat: *const fmpq_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_neg(
        res: *mut gr_mat_struct,
        mat: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_swap_entrywise(
        mat1: *mut gr_mat_struct,
        mat2: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_add(
        res: *mut gr_mat_struct,
        mat1: *const gr_mat_struct,
        mat2: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_sub(
        res: *mut gr_mat_struct,
        mat1: *const gr_mat_struct,
        mat2: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_add_scalar(
        res: *mut gr_mat_struct,
        mat: *const gr_mat_struct,
        x: gr_srcptr,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_sub_scalar(
        res: *mut gr_mat_struct,
        mat: *const gr_mat_struct,
        x: gr_srcptr,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_mul_scalar(
        res: *mut gr_mat_struct,
        mat: *const gr_mat_struct,
        x: gr_srcptr,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_addmul_scalar(
        res: *mut gr_mat_struct,
        mat: *const gr_mat_struct,
        x: gr_srcptr,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_submul_scalar(
        res: *mut gr_mat_struct,
        mat: *const gr_mat_struct,
        x: gr_srcptr,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_div_scalar(
        res: *mut gr_mat_struct,
        mat: *const gr_mat_struct,
        x: gr_srcptr,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_mul_classical(
        C: *mut gr_mat_struct,
        A: *const gr_mat_struct,
        B: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_mul_strassen(
        C: *mut gr_mat_struct,
        A: *const gr_mat_struct,
        B: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_mul_generic(
        C: *mut gr_mat_struct,
        A: *const gr_mat_struct,
        B: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_mul(
        C: *mut gr_mat_struct,
        A: *const gr_mat_struct,
        B: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    #[link_name = "gr_mat_sqr__extern"]
    pub fn gr_mat_sqr(
        res: *mut gr_mat_struct,
        mat: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn _gr_mat_gr_poly_evaluate(
        y: *mut gr_mat_struct,
        poly: gr_srcptr,
        len: mp_limb_signed_t,
        x: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_gr_poly_evaluate(
        res: *mut gr_mat_struct,
        f: *const gr_poly_struct,
        a: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_find_nonzero_pivot_large_abs(
        pivot_row: *mut mp_limb_signed_t,
        mat: *mut gr_mat_struct,
        start_row: mp_limb_signed_t,
        end_row: mp_limb_signed_t,
        column: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_find_nonzero_pivot_generic(
        pivot_row: *mut mp_limb_signed_t,
        mat: *mut gr_mat_struct,
        start_row: mp_limb_signed_t,
        end_row: mp_limb_signed_t,
        column: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_find_nonzero_pivot(
        pivot_row: *mut mp_limb_signed_t,
        mat: *mut gr_mat_struct,
        start_row: mp_limb_signed_t,
        end_row: mp_limb_signed_t,
        column: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_lu_recursive(
        rank: *mut mp_limb_signed_t,
        P: *mut mp_limb_signed_t,
        LU: *mut gr_mat_struct,
        A: *const gr_mat_struct,
        rank_check: libc::c_int,
        cutoff: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_lu_classical(
        rank: *mut mp_limb_signed_t,
        P: *mut mp_limb_signed_t,
        LU: *mut gr_mat_struct,
        A: *const gr_mat_struct,
        rank_check: libc::c_int,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_lu_generic(
        rank: *mut mp_limb_signed_t,
        P: *mut mp_limb_signed_t,
        LU: *mut gr_mat_struct,
        A: *const gr_mat_struct,
        rank_check: libc::c_int,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_lu(
        rank: *mut mp_limb_signed_t,
        P: *mut mp_limb_signed_t,
        LU: *mut gr_mat_struct,
        A: *const gr_mat_struct,
        rank_check: libc::c_int,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_fflu(
        res_rank: *mut mp_limb_signed_t,
        P: *mut mp_limb_signed_t,
        LU: *mut gr_mat_struct,
        den: gr_ptr,
        A: *const gr_mat_struct,
        rank_check: libc::c_int,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_nonsingular_solve_fflu(
        X: *mut gr_mat_struct,
        A: *const gr_mat_struct,
        B: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_nonsingular_solve_lu(
        X: *mut gr_mat_struct,
        A: *const gr_mat_struct,
        B: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_nonsingular_solve(
        X: *mut gr_mat_struct,
        A: *const gr_mat_struct,
        B: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_nonsingular_solve_fflu_precomp(
        X: *mut gr_mat_struct,
        perm: *const mp_limb_signed_t,
        A: *const gr_mat_struct,
        B: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_nonsingular_solve_lu_precomp(
        X: *mut gr_mat_struct,
        perm: *const mp_limb_signed_t,
        A: *const gr_mat_struct,
        B: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_nonsingular_solve_den_fflu(
        X: *mut gr_mat_struct,
        den: gr_ptr,
        A: *const gr_mat_struct,
        B: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_nonsingular_solve_den(
        X: *mut gr_mat_struct,
        den: gr_ptr,
        A: *const gr_mat_struct,
        B: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_solve_field(
        X: *mut gr_mat_struct,
        A: *const gr_mat_struct,
        B: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_det_berkowitz(
        res: gr_ptr,
        A: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_det_fflu(
        res: gr_ptr,
        A: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_det_lu(
        res: gr_ptr,
        A: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_det_cofactor(
        res: gr_ptr,
        A: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_det_generic_field(
        res: gr_ptr,
        A: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_det_generic_integral_domain(
        res: gr_ptr,
        A: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_det_generic(
        res: gr_ptr,
        A: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_det(res: gr_ptr, A: *const gr_mat_struct, ctx: *mut gr_ctx_struct)
        -> libc::c_int;
    pub fn gr_mat_inv(
        res: *mut gr_mat_struct,
        mat: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_adjugate_charpoly(
        adj: *mut gr_mat_struct,
        det: gr_ptr,
        A: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_adjugate_cofactor(
        adj: *mut gr_mat_struct,
        det: gr_ptr,
        A: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_adjugate(
        adj: *mut gr_mat_struct,
        det: gr_ptr,
        A: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_rank_lu(
        rank: *mut mp_limb_signed_t,
        A: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_rank_fflu(
        rank: *mut mp_limb_signed_t,
        A: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_rank(
        rank: *mut mp_limb_signed_t,
        A: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_rref_lu(
        res_rank: *mut mp_limb_signed_t,
        R: *mut gr_mat_struct,
        A: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_rref_fflu(
        res_rank: *mut mp_limb_signed_t,
        R: *mut gr_mat_struct,
        A: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_rref(
        res_rank: *mut mp_limb_signed_t,
        R: *mut gr_mat_struct,
        A: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_rref_den_fflu(
        res_rank: *mut mp_limb_signed_t,
        R: *mut gr_mat_struct,
        den: gr_ptr,
        A: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_rref_den(
        res_rank: *mut mp_limb_signed_t,
        R: *mut gr_mat_struct,
        den: gr_ptr,
        A: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_nullspace(
        X: *mut gr_mat_struct,
        A: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_ones(mat: *mut gr_mat_struct, ctx: *mut gr_ctx_struct) -> libc::c_int;
    pub fn gr_mat_pascal(
        mat: *mut gr_mat_struct,
        triangular: libc::c_int,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_stirling(
        mat: *mut gr_mat_struct,
        kind: libc::c_int,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_hilbert(mat: *mut gr_mat_struct, ctx: *mut gr_ctx_struct) -> libc::c_int;
    pub fn gr_mat_hadamard(mat: *mut gr_mat_struct, ctx: *mut gr_ctx_struct) -> libc::c_int;
    pub fn gr_mat_transpose(
        B: *mut gr_mat_struct,
        A: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_nonsingular_solve_tril_classical(
        X: *mut gr_mat_struct,
        L: *const gr_mat_struct,
        B: *const gr_mat_struct,
        unit: libc::c_int,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_nonsingular_solve_tril_recursive(
        X: *mut gr_mat_struct,
        L: *const gr_mat_struct,
        B: *const gr_mat_struct,
        unit: libc::c_int,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_nonsingular_solve_tril(
        X: *mut gr_mat_struct,
        L: *const gr_mat_struct,
        B: *const gr_mat_struct,
        unit: libc::c_int,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_nonsingular_solve_triu_classical(
        X: *mut gr_mat_struct,
        U: *const gr_mat_struct,
        B: *const gr_mat_struct,
        unit: libc::c_int,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_nonsingular_solve_triu_recursive(
        X: *mut gr_mat_struct,
        U: *const gr_mat_struct,
        B: *const gr_mat_struct,
        unit: libc::c_int,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_nonsingular_solve_triu(
        X: *mut gr_mat_struct,
        U: *const gr_mat_struct,
        B: *const gr_mat_struct,
        unit: libc::c_int,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_trace(
        res: gr_ptr,
        mat: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn _gr_mat_charpoly_berkowitz(
        res: gr_ptr,
        mat: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_charpoly_berkowitz(
        res: *mut gr_poly_struct,
        mat: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn _gr_mat_charpoly_danilevsky_inplace(
        res: gr_ptr,
        mat: *mut gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn _gr_mat_charpoly_danilevsky(
        res: gr_ptr,
        mat: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_charpoly_danilevsky(
        res: *mut gr_poly_struct,
        mat: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn _gr_mat_charpoly_faddeev(
        res: gr_ptr,
        adj: *mut gr_mat_struct,
        mat: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_charpoly_faddeev(
        res: *mut gr_poly_struct,
        adj: *mut gr_mat_struct,
        mat: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn _gr_mat_charpoly_faddeev_bsgs(
        res: gr_ptr,
        adj: *mut gr_mat_struct,
        mat: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_charpoly_faddeev_bsgs(
        res: *mut gr_poly_struct,
        adj: *mut gr_mat_struct,
        mat: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn _gr_mat_charpoly_from_hessenberg(
        res: gr_ptr,
        mat: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_charpoly_from_hessenberg(
        cp: *mut gr_poly_struct,
        mat: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn _gr_mat_charpoly_gauss(
        res: gr_ptr,
        mat: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_charpoly_gauss(
        cp: *mut gr_poly_struct,
        mat: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn _gr_mat_charpoly_householder(
        res: gr_ptr,
        mat: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_charpoly_householder(
        cp: *mut gr_poly_struct,
        mat: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn _gr_mat_charpoly(
        res: gr_ptr,
        mat: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_charpoly(
        res: *mut gr_poly_struct,
        mat: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_hessenberg(
        res: *mut gr_mat_struct,
        mat: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_hessenberg_gauss(
        res: *mut gr_mat_struct,
        mat: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_hessenberg_householder(
        res: *mut gr_mat_struct,
        mat: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_is_hessenberg(mat: *const gr_mat_struct, ctx: *mut gr_ctx_struct) -> truth_t;
    pub fn gr_mat_reduce_row(
        column: *mut mp_limb_signed_t,
        A: *mut gr_mat_struct,
        P: *mut mp_limb_signed_t,
        L: *mut mp_limb_signed_t,
        m: mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_apply_row_similarity(
        A: *mut gr_mat_struct,
        r: mp_limb_signed_t,
        d: gr_ptr,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_minpoly_field(
        p: *mut gr_poly_struct,
        X: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_eigenvalues(
        lambda: *mut gr_vec_struct,
        mult: *mut gr_vec_struct,
        mat: *const gr_mat_struct,
        flags: libc::c_int,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_eigenvalues_other(
        lambda: *mut gr_vec_struct,
        mult: *mut gr_vec_struct,
        mat: *const gr_mat_struct,
        mat_ctx: *mut gr_ctx_struct,
        flags: libc::c_int,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_diagonalization_precomp(
        D: *mut gr_vec_struct,
        L: *mut gr_mat_struct,
        R: *mut gr_mat_struct,
        A: *const gr_mat_struct,
        eigenvalues: *const gr_vec_struct,
        mult: *const gr_vec_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_diagonalization_generic(
        D: *mut gr_vec_struct,
        L: *mut gr_mat_struct,
        R: *mut gr_mat_struct,
        A: *const gr_mat_struct,
        flags: libc::c_int,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_diagonalization(
        D: *mut gr_vec_struct,
        L: *mut gr_mat_struct,
        R: *mut gr_mat_struct,
        A: *const gr_mat_struct,
        flags: libc::c_int,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_set_jordan_blocks(
        mat: *mut gr_mat_struct,
        lambda: *const gr_vec_struct,
        num_blocks: mp_limb_signed_t,
        block_lambda: *mut mp_limb_signed_t,
        block_size: *mut mp_limb_signed_t,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_jordan_blocks(
        lambda: *mut gr_vec_struct,
        num_blocks: *mut mp_limb_signed_t,
        block_lambda: *mut mp_limb_signed_t,
        block_size: *mut mp_limb_signed_t,
        A: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_jordan_transformation(
        mat: *mut gr_mat_struct,
        lambda: *const gr_vec_struct,
        num_blocks: mp_limb_signed_t,
        block_lambda: *mut mp_limb_signed_t,
        block_size: *mut mp_limb_signed_t,
        A: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_jordan_form(
        J: *mut gr_mat_struct,
        P: *mut gr_mat_struct,
        A: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_is_scalar(mat: *const gr_mat_struct, ctx: *mut gr_ctx_struct) -> truth_t;
    pub fn gr_mat_is_diagonal(mat: *const gr_mat_struct, ctx: *mut gr_ctx_struct) -> truth_t;
    pub fn gr_mat_is_lower_triangular(
        mat: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> truth_t;
    pub fn gr_mat_is_upper_triangular(
        mat: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> truth_t;
    pub fn gr_mat_mul_diag(
        C: *mut gr_mat_struct,
        A: *const gr_mat_struct,
        D: *const gr_vec_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_diag_mul(
        C: *mut gr_mat_struct,
        D: *const gr_vec_struct,
        A: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_exp_jordan(
        res: *mut gr_mat_struct,
        A: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_exp(
        res: *mut gr_mat_struct,
        A: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_log_jordan(
        res: *mut gr_mat_struct,
        A: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
    pub fn gr_mat_log(
        res: *mut gr_mat_struct,
        A: *const gr_mat_struct,
        ctx: *mut gr_ctx_struct,
    ) -> libc::c_int;
}
