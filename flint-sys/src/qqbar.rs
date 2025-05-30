/* automatically generated by rust-bindgen 0.70.1 */

use crate::deps::*;
use crate::acb_types::*;
use crate::arb_types::*;
use crate::flint::*;
use crate::fmpq_types::*;
use crate::fmpz_types::*;
use crate::mpoly_types::*;


pub const QQBAR_DEFAULT_PREC: u32 = 128;
pub const QQBAR_ROOTS_IRREDUCIBLE: u32 = 1;
pub const QQBAR_ROOTS_UNSORTED: u32 = 2;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct qqbar_struct {
    pub poly: fmpz_poly_struct,
    pub enclosure: acb_struct,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of qqbar_struct"][::std::mem::size_of::<qqbar_struct>() - 120usize];
    ["Alignment of qqbar_struct"][::std::mem::align_of::<qqbar_struct>() - 8usize];
    ["Offset of field: qqbar_struct::poly"][::std::mem::offset_of!(qqbar_struct, poly) - 0usize];
    ["Offset of field: qqbar_struct::enclosure"]
        [::std::mem::offset_of!(qqbar_struct, enclosure) - 24usize];
};
impl Default for qqbar_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type qqbar_t = [qqbar_struct; 1usize];
pub type qqbar_ptr = *mut qqbar_struct;
pub type qqbar_srcptr = *const qqbar_struct;
extern "C" {
    pub fn qqbar_init(res: *mut qqbar_struct);
    pub fn qqbar_clear(res: *mut qqbar_struct);
    #[link_name = "_qqbar_vec_init__extern"]
    pub fn _qqbar_vec_init(len: mp_limb_signed_t) -> qqbar_ptr;
    #[link_name = "_qqbar_vec_clear__extern"]
    pub fn _qqbar_vec_clear(vec: qqbar_ptr, len: mp_limb_signed_t);
    pub fn qqbar_swap(x: *mut qqbar_struct, y: *mut qqbar_struct);
    pub fn qqbar_set(res: *mut qqbar_struct, x: *const qqbar_struct);
    pub fn qqbar_set_si(res: *mut qqbar_struct, x: mp_limb_signed_t);
    pub fn qqbar_set_ui(res: *mut qqbar_struct, x: mp_limb_t);
    pub fn qqbar_set_fmpz(res: *mut qqbar_struct, x: *const fmpz);
    pub fn qqbar_set_fmpq(res: *mut qqbar_struct, x: *const fmpq);
    pub fn qqbar_set_re_im(res: *mut qqbar_struct, x: *const qqbar_struct, y: *const qqbar_struct);
    pub fn qqbar_set_d(res: *mut qqbar_struct, x: f64) -> libc::c_int;
    pub fn qqbar_set_re_im_d(res: *mut qqbar_struct, x: f64, y: f64) -> libc::c_int;
    #[link_name = "qqbar_degree__extern"]
    pub fn qqbar_degree(x: *const qqbar_struct) -> mp_limb_signed_t;
    #[link_name = "qqbar_is_rational__extern"]
    pub fn qqbar_is_rational(x: *const qqbar_struct) -> libc::c_int;
    #[link_name = "qqbar_is_integer__extern"]
    pub fn qqbar_is_integer(x: *const qqbar_struct) -> libc::c_int;
    #[link_name = "qqbar_is_algebraic_integer__extern"]
    pub fn qqbar_is_algebraic_integer(x: *const qqbar_struct) -> libc::c_int;
    #[link_name = "qqbar_is_zero__extern"]
    pub fn qqbar_is_zero(x: *const qqbar_struct) -> libc::c_int;
    #[link_name = "qqbar_is_one__extern"]
    pub fn qqbar_is_one(x: *const qqbar_struct) -> libc::c_int;
    #[link_name = "qqbar_is_neg_one__extern"]
    pub fn qqbar_is_neg_one(x: *const qqbar_struct) -> libc::c_int;
    #[link_name = "qqbar_is_i__extern"]
    pub fn qqbar_is_i(x: *const qqbar_struct) -> libc::c_int;
    #[link_name = "qqbar_is_neg_i__extern"]
    pub fn qqbar_is_neg_i(x: *const qqbar_struct) -> libc::c_int;
    pub fn qqbar_sgn_re(x: *const qqbar_struct) -> libc::c_int;
    pub fn qqbar_sgn_im(x: *const qqbar_struct) -> libc::c_int;
    #[link_name = "qqbar_is_real__extern"]
    pub fn qqbar_is_real(x: *const qqbar_struct) -> libc::c_int;
    pub fn qqbar_height_bits(x: *const qqbar_struct) -> mp_limb_signed_t;
    pub fn qqbar_height(res: *mut fmpz, x: *const qqbar_struct);
    #[link_name = "qqbar_within_limits__extern"]
    pub fn qqbar_within_limits(
        x: *const qqbar_struct,
        deg_limit: mp_limb_signed_t,
        bits_limit: mp_limb_signed_t,
    ) -> libc::c_int;
    #[link_name = "qqbar_binop_within_limits__extern"]
    pub fn qqbar_binop_within_limits(
        x: *const qqbar_struct,
        y: *const qqbar_struct,
        deg_limit: mp_limb_signed_t,
        bits_limit: mp_limb_signed_t,
    ) -> libc::c_int;
    pub fn _qqbar_get_fmpq(num: *mut fmpz, den: *mut fmpz, x: *const qqbar_struct);
    pub fn qqbar_get_fmpq(res: *mut fmpq, x: *const qqbar_struct);
    pub fn qqbar_get_fmpz(res: *mut fmpz, x: *const qqbar_struct);
    #[link_name = "qqbar_zero__extern"]
    pub fn qqbar_zero(res: *mut qqbar_struct);
    #[link_name = "qqbar_one__extern"]
    pub fn qqbar_one(res: *mut qqbar_struct);
    pub fn qqbar_i(res: *mut qqbar_struct);
    pub fn qqbar_phi(res: *mut qqbar_struct);
    pub fn qqbar_randtest(
        res: *mut qqbar_struct,
        state: *mut flint_rand_s,
        deg: mp_limb_signed_t,
        bits: mp_limb_signed_t,
    );
    pub fn qqbar_randtest_real(
        res: *mut qqbar_struct,
        state: *mut flint_rand_s,
        deg: mp_limb_signed_t,
        bits: mp_limb_signed_t,
    );
    pub fn qqbar_randtest_nonreal(
        res: *mut qqbar_struct,
        state: *mut flint_rand_s,
        deg: mp_limb_signed_t,
        bits: mp_limb_signed_t,
    );
    pub fn qqbar_print(x: *const qqbar_struct);
    pub fn qqbar_printn(x: *const qqbar_struct, n: mp_limb_signed_t);
    pub fn qqbar_printnd(x: *const qqbar_struct, n: mp_limb_signed_t);
    pub fn qqbar_equal(x: *const qqbar_struct, y: *const qqbar_struct) -> libc::c_int;
    pub fn qqbar_equal_fmpq_poly_val(
        x: *const qqbar_struct,
        f: *const fmpq_poly_struct,
        y: *const qqbar_struct,
    ) -> libc::c_int;
    pub fn qqbar_cmp_re(x: *const qqbar_struct, y: *const qqbar_struct) -> libc::c_int;
    pub fn qqbar_cmp_im(x: *const qqbar_struct, y: *const qqbar_struct) -> libc::c_int;
    pub fn qqbar_cmpabs_re(x: *const qqbar_struct, y: *const qqbar_struct) -> libc::c_int;
    pub fn qqbar_cmpabs_im(x: *const qqbar_struct, y: *const qqbar_struct) -> libc::c_int;
    pub fn qqbar_cmpabs(x: *const qqbar_struct, y: *const qqbar_struct) -> libc::c_int;
    pub fn qqbar_cmp_root_order(x: *const qqbar_struct, y: *const qqbar_struct) -> libc::c_int;
    pub fn qqbar_hash(x: *const qqbar_struct) -> mp_limb_t;
    pub fn qqbar_conj(res: *mut qqbar_struct, x: *const qqbar_struct);
    pub fn qqbar_re(res: *mut qqbar_struct, x: *const qqbar_struct);
    pub fn qqbar_im(res: *mut qqbar_struct, x: *const qqbar_struct);
    pub fn qqbar_re_im(res1: *mut qqbar_struct, res2: *mut qqbar_struct, x: *const qqbar_struct);
    pub fn qqbar_abs(res: *mut qqbar_struct, x: *const qqbar_struct);
    pub fn qqbar_abs2(res: *mut qqbar_struct, x: *const qqbar_struct);
    pub fn qqbar_sgn(res: *mut qqbar_struct, x: *const qqbar_struct);
    pub fn qqbar_csgn(x: *const qqbar_struct) -> libc::c_int;
    pub fn qqbar_floor(res: *mut fmpz, x: *const qqbar_struct);
    pub fn qqbar_ceil(res: *mut fmpz, x: *const qqbar_struct);
    pub fn qqbar_numerator(res: *mut qqbar_struct, y: *const qqbar_struct);
    pub fn qqbar_denominator(res: *mut fmpz, y: *const qqbar_struct);
    pub fn qqbar_neg(res: *mut qqbar_struct, x: *const qqbar_struct);
    pub fn qqbar_add(res: *mut qqbar_struct, x: *const qqbar_struct, y: *const qqbar_struct);
    pub fn qqbar_add_fmpq(res: *mut qqbar_struct, x: *const qqbar_struct, y: *const fmpq);
    pub fn qqbar_add_fmpz(res: *mut qqbar_struct, x: *const qqbar_struct, y: *const fmpz);
    pub fn qqbar_add_ui(res: *mut qqbar_struct, x: *const qqbar_struct, y: mp_limb_t);
    pub fn qqbar_add_si(res: *mut qqbar_struct, x: *const qqbar_struct, y: mp_limb_signed_t);
    pub fn qqbar_sub(res: *mut qqbar_struct, x: *const qqbar_struct, y: *const qqbar_struct);
    pub fn qqbar_sub_fmpq(res: *mut qqbar_struct, x: *const qqbar_struct, y: *const fmpq);
    pub fn qqbar_sub_fmpz(res: *mut qqbar_struct, x: *const qqbar_struct, y: *const fmpz);
    pub fn qqbar_sub_ui(res: *mut qqbar_struct, x: *const qqbar_struct, y: mp_limb_t);
    pub fn qqbar_sub_si(res: *mut qqbar_struct, x: *const qqbar_struct, y: mp_limb_signed_t);
    pub fn qqbar_fmpq_sub(res: *mut qqbar_struct, x: *const fmpq, y: *const qqbar_struct);
    pub fn qqbar_fmpz_sub(res: *mut qqbar_struct, x: *const fmpz, y: *const qqbar_struct);
    pub fn qqbar_ui_sub(res: *mut qqbar_struct, x: mp_limb_t, y: *const qqbar_struct);
    pub fn qqbar_si_sub(res: *mut qqbar_struct, x: mp_limb_signed_t, y: *const qqbar_struct);
    pub fn qqbar_mul(res: *mut qqbar_struct, x: *const qqbar_struct, y: *const qqbar_struct);
    pub fn qqbar_mul_fmpq(res: *mut qqbar_struct, x: *const qqbar_struct, y: *const fmpq);
    pub fn qqbar_mul_fmpz(res: *mut qqbar_struct, x: *const qqbar_struct, y: *const fmpz);
    pub fn qqbar_mul_ui(res: *mut qqbar_struct, x: *const qqbar_struct, y: mp_limb_t);
    pub fn qqbar_mul_si(res: *mut qqbar_struct, x: *const qqbar_struct, y: mp_limb_signed_t);
    pub fn qqbar_div(res: *mut qqbar_struct, x: *const qqbar_struct, y: *const qqbar_struct);
    pub fn qqbar_div_fmpq(res: *mut qqbar_struct, x: *const qqbar_struct, y: *const fmpq);
    pub fn qqbar_div_fmpz(res: *mut qqbar_struct, x: *const qqbar_struct, y: *const fmpz);
    pub fn qqbar_div_ui(res: *mut qqbar_struct, x: *const qqbar_struct, y: mp_limb_t);
    pub fn qqbar_div_si(res: *mut qqbar_struct, x: *const qqbar_struct, y: mp_limb_signed_t);
    pub fn qqbar_fmpq_div(res: *mut qqbar_struct, x: *const fmpq, y: *const qqbar_struct);
    pub fn qqbar_fmpz_div(res: *mut qqbar_struct, x: *const fmpz, y: *const qqbar_struct);
    pub fn qqbar_ui_div(res: *mut qqbar_struct, x: mp_limb_t, y: *const qqbar_struct);
    pub fn qqbar_si_div(res: *mut qqbar_struct, x: mp_limb_signed_t, y: *const qqbar_struct);
    #[link_name = "qqbar_sqr__extern"]
    pub fn qqbar_sqr(res: *mut qqbar_struct, x: *const qqbar_struct);
    pub fn qqbar_inv(res: *mut qqbar_struct, x: *const qqbar_struct);
    pub fn qqbar_mul_2exp_si(res: *mut qqbar_struct, x: *const qqbar_struct, exp: mp_limb_signed_t);
    pub fn qqbar_pow_ui(res: *mut qqbar_struct, x: *const qqbar_struct, e: mp_limb_t);
    pub fn qqbar_pow_si(res: *mut qqbar_struct, x: *const qqbar_struct, n: mp_limb_signed_t);
    pub fn qqbar_pow_fmpz(res: *mut qqbar_struct, x: *const qqbar_struct, n: *const fmpz);
    pub fn qqbar_pow_fmpq(res: *mut qqbar_struct, x: *const qqbar_struct, n: *const fmpq);
    pub fn qqbar_pow(
        res: *mut qqbar_struct,
        x: *const qqbar_struct,
        e: *const qqbar_struct,
    ) -> libc::c_int;
    pub fn _qqbar_fast_detect_simple_principal_surd(x: *const qqbar_struct) -> libc::c_int;
    pub fn qqbar_root_ui(res: *mut qqbar_struct, x: *const qqbar_struct, n: mp_limb_t);
    #[link_name = "qqbar_sqrt__extern"]
    pub fn qqbar_sqrt(res: *mut qqbar_struct, x: *const qqbar_struct);
    #[link_name = "qqbar_sqrt_ui__extern"]
    pub fn qqbar_sqrt_ui(res: *mut qqbar_struct, x: mp_limb_t);
    #[link_name = "qqbar_rsqrt__extern"]
    pub fn qqbar_rsqrt(res: *mut qqbar_struct, x: *const qqbar_struct);
    pub fn qqbar_fmpq_root_ui(res: *mut qqbar_struct, x: *const fmpq, b: mp_limb_t);
    pub fn qqbar_fmpq_pow_si_ui(
        res: *mut qqbar_struct,
        x: *const fmpq,
        a: mp_limb_signed_t,
        b: mp_limb_t,
    );
    pub fn qqbar_cache_enclosure(res: *mut qqbar_struct, prec: mp_limb_signed_t);
    pub fn qqbar_get_acb(res: *mut acb_struct, x: *const qqbar_struct, prec: mp_limb_signed_t);
    pub fn qqbar_get_arb(res: *mut arb_struct, x: *const qqbar_struct, prec: mp_limb_signed_t);
    pub fn qqbar_get_arb_re(res: *mut arb_struct, x: *const qqbar_struct, prec: mp_limb_signed_t);
    pub fn qqbar_get_arb_im(res: *mut arb_struct, x: *const qqbar_struct, prec: mp_limb_signed_t);
    pub fn qqbar_conjugates(res: qqbar_ptr, x: *const qqbar_struct);
    pub fn _qqbar_evaluate_fmpq_poly(
        res: *mut qqbar_struct,
        poly: *const fmpz,
        den: *const fmpz,
        len: mp_limb_signed_t,
        x: *const qqbar_struct,
    );
    pub fn qqbar_evaluate_fmpq_poly(
        res: *mut qqbar_struct,
        poly: *const fmpq_poly_struct,
        x: *const qqbar_struct,
    );
    pub fn _qqbar_evaluate_fmpz_poly(
        res: *mut qqbar_struct,
        poly: *const fmpz,
        len: mp_limb_signed_t,
        x: *const qqbar_struct,
    );
    pub fn qqbar_evaluate_fmpz_poly(
        res: *mut qqbar_struct,
        poly: *const fmpz_poly_struct,
        x: *const qqbar_struct,
    );
    pub fn qqbar_evaluate_fmpz_mpoly_iter(
        res: *mut qqbar_struct,
        f: *const fmpz_mpoly_struct,
        x: qqbar_srcptr,
        deg_limit: mp_limb_signed_t,
        bits_limit: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> libc::c_int;
    pub fn qqbar_evaluate_fmpz_mpoly_horner(
        res: *mut qqbar_struct,
        f: *const fmpz_mpoly_struct,
        x: qqbar_srcptr,
        deg_limit: mp_limb_signed_t,
        bits_limit: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> libc::c_int;
    pub fn qqbar_evaluate_fmpz_mpoly(
        res: *mut qqbar_struct,
        f: *const fmpz_mpoly_struct,
        x: qqbar_srcptr,
        deg_limit: mp_limb_signed_t,
        bits_limit: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> libc::c_int;
    pub fn qqbar_roots_fmpz_poly(res: qqbar_ptr, poly: *const fmpz_poly_struct, flags: libc::c_int);
    pub fn qqbar_roots_fmpq_poly(res: qqbar_ptr, poly: *const fmpq_poly_struct, flags: libc::c_int);
    pub fn qqbar_eigenvalues_fmpz_mat(
        res: qqbar_ptr,
        mat: *const fmpz_mat_struct,
        flags: libc::c_int,
    );
    pub fn qqbar_eigenvalues_fmpq_mat(
        res: qqbar_ptr,
        mat: *const fmpq_mat_struct,
        flags: libc::c_int,
    );
    pub fn qqbar_root_of_unity(res: *mut qqbar_struct, p: mp_limb_signed_t, q: mp_limb_t);
    pub fn qqbar_is_root_of_unity(
        p: *mut mp_limb_signed_t,
        q: *mut mp_limb_t,
        x: *const qqbar_struct,
    ) -> libc::c_int;
    pub fn qqbar_exp_pi_i(res: *mut qqbar_struct, p: mp_limb_signed_t, q: mp_limb_t);
    pub fn qqbar_cos_pi(res: *mut qqbar_struct, p: mp_limb_signed_t, q: mp_limb_t);
    pub fn qqbar_sin_pi(res: *mut qqbar_struct, p: mp_limb_signed_t, q: mp_limb_t);
    pub fn qqbar_tan_pi(res: *mut qqbar_struct, p: mp_limb_signed_t, q: mp_limb_t) -> libc::c_int;
    pub fn qqbar_cot_pi(res: *mut qqbar_struct, p: mp_limb_signed_t, q: mp_limb_t) -> libc::c_int;
    pub fn qqbar_sec_pi(res: *mut qqbar_struct, p: mp_limb_signed_t, q: mp_limb_t) -> libc::c_int;
    pub fn qqbar_csc_pi(res: *mut qqbar_struct, p: mp_limb_signed_t, q: mp_limb_t) -> libc::c_int;
    pub fn qqbar_log_pi_i(
        p: *mut mp_limb_signed_t,
        q: *mut mp_limb_t,
        x: *const qqbar_struct,
    ) -> libc::c_int;
    pub fn qqbar_atan_pi(
        p: *mut mp_limb_signed_t,
        q: *mut mp_limb_t,
        x: *const qqbar_struct,
    ) -> libc::c_int;
    pub fn qqbar_asin_pi(
        p: *mut mp_limb_signed_t,
        q: *mut mp_limb_t,
        x: *const qqbar_struct,
    ) -> libc::c_int;
    pub fn qqbar_acos_pi(
        p: *mut mp_limb_signed_t,
        q: *mut mp_limb_t,
        x: *const qqbar_struct,
    ) -> libc::c_int;
    pub fn qqbar_acot_pi(
        p: *mut mp_limb_signed_t,
        q: *mut mp_limb_t,
        x: *const qqbar_struct,
    ) -> libc::c_int;
    pub fn qqbar_asec_pi(
        p: *mut mp_limb_signed_t,
        q: *mut mp_limb_t,
        x: *const qqbar_struct,
    ) -> libc::c_int;
    pub fn qqbar_acsc_pi(
        p: *mut mp_limb_signed_t,
        q: *mut mp_limb_t,
        x: *const qqbar_struct,
    ) -> libc::c_int;
    pub fn qqbar_guess(
        res: *mut qqbar_struct,
        z: *const acb_struct,
        max_deg: mp_limb_signed_t,
        max_bits: mp_limb_signed_t,
        flags: libc::c_int,
        prec: mp_limb_signed_t,
    ) -> libc::c_int;
    pub fn qqbar_express_in_field(
        res: *mut fmpq_poly_struct,
        alpha: *const qqbar_struct,
        x: *const qqbar_struct,
        max_bits: mp_limb_signed_t,
        flags: libc::c_int,
        prec: mp_limb_signed_t,
    ) -> libc::c_int;
    pub fn qqbar_get_quadratic(
        res_a: *mut fmpz,
        res_b: *mut fmpz,
        res_c: *mut fmpz,
        res_q: *mut fmpz,
        x: *const qqbar_struct,
        factoring: libc::c_int,
    );
    pub fn qqbar_scalar_op(
        res: *mut qqbar_struct,
        x: *const qqbar_struct,
        a: *const fmpz,
        b: *const fmpz,
        c: *const fmpz,
    );
    pub fn qqbar_fmpz_poly_composed_op(
        res: *mut fmpz_poly_struct,
        A: *const fmpz_poly_struct,
        B: *const fmpz_poly_struct,
        op: libc::c_int,
    );
    pub fn qqbar_binary_op(
        res: *mut qqbar_struct,
        x: *const qqbar_struct,
        y: *const qqbar_struct,
        op: libc::c_int,
    );
    pub fn _qqbar_validate_uniqueness(
        res: *mut acb_struct,
        poly: *const fmpz_poly_struct,
        z: *const acb_struct,
        max_prec: mp_limb_signed_t,
    ) -> libc::c_int;
    pub fn _qqbar_validate_existence_uniqueness(
        res: *mut acb_struct,
        poly: *const fmpz_poly_struct,
        z: *const acb_struct,
        prec: mp_limb_signed_t,
    ) -> libc::c_int;
    pub fn _qqbar_enclosure_raw(
        res: *mut acb_struct,
        poly: *const fmpz_poly_struct,
        zin: *const acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn qqbar_enclosure_raw(
        res: *mut acb_struct,
        x: *const qqbar_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _qqbar_acb_lindep(
        rel: *mut fmpz,
        vec: acb_srcptr,
        len: mp_limb_signed_t,
        check: libc::c_int,
        prec: mp_limb_signed_t,
    ) -> libc::c_int;
}
