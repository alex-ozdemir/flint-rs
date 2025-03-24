/* automatically generated by rust-bindgen 0.70.1 */

use crate::deps::*;
use crate::acb_types::*;
use crate::flint::*;
use crate::fmpz_types::*;
use crate::mpoly_types::*;


extern "C" {
    pub fn fmpz_mpoly_q_init(res: *mut fmpz_mpoly_q_struct, ctx: *const fmpz_mpoly_ctx_struct);
    pub fn fmpz_mpoly_q_clear(res: *mut fmpz_mpoly_q_struct, ctx: *const fmpz_mpoly_ctx_struct);
    pub fn fmpz_mpoly_q_swap(
        x: *mut fmpz_mpoly_q_struct,
        y: *mut fmpz_mpoly_q_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_set(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpz_mpoly_q_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_set_fmpq(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpq,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_set_fmpz(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_set_si(
        res: *mut fmpz_mpoly_q_struct,
        x: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_canonicalise(
        x: *mut fmpz_mpoly_q_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_is_canonical(
        res: *const fmpz_mpoly_q_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> libc::c_int;
    #[link_name = "fmpz_mpoly_q_is_zero__extern"]
    pub fn fmpz_mpoly_q_is_zero(
        x: *const fmpz_mpoly_q_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> libc::c_int;
    #[link_name = "fmpz_mpoly_q_is_one__extern"]
    pub fn fmpz_mpoly_q_is_one(
        x: *const fmpz_mpoly_q_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> libc::c_int;
    #[link_name = "fmpz_mpoly_q_is_fmpz__extern"]
    pub fn fmpz_mpoly_q_is_fmpz(
        x: *const fmpz_mpoly_q_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> libc::c_int;
    #[link_name = "fmpz_mpoly_q_is_fmpq__extern"]
    pub fn fmpz_mpoly_q_is_fmpq(
        x: *const fmpz_mpoly_q_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> libc::c_int;
    pub fn fmpz_mpoly_q_used_vars(
        used: *mut libc::c_int,
        f: *const fmpz_mpoly_q_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_used_vars_num(
        used: *mut libc::c_int,
        f: *const fmpz_mpoly_q_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_used_vars_den(
        used: *mut libc::c_int,
        f: *const fmpz_mpoly_q_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    #[link_name = "fmpz_mpoly_q_zero__extern"]
    pub fn fmpz_mpoly_q_zero(res: *mut fmpz_mpoly_q_struct, ctx: *const fmpz_mpoly_ctx_struct);
    #[link_name = "fmpz_mpoly_q_one__extern"]
    pub fn fmpz_mpoly_q_one(res: *mut fmpz_mpoly_q_struct, ctx: *const fmpz_mpoly_ctx_struct);
    #[link_name = "fmpz_mpoly_q_gen__extern"]
    pub fn fmpz_mpoly_q_gen(
        res: *mut fmpz_mpoly_q_struct,
        i: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_print_pretty(
        f: *const fmpz_mpoly_q_struct,
        x: *mut *const libc::c_char,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_get_str_pretty(
        f: *const fmpz_mpoly_q_struct,
        vars: *mut *const libc::c_char,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> *mut libc::c_char;
    pub fn fmpz_mpoly_q_set_str_pretty(
        res: *mut fmpz_mpoly_q_struct,
        s: *const libc::c_char,
        vars: *mut *const libc::c_char,
        ctx: *mut fmpz_mpoly_ctx_struct,
    ) -> libc::c_int;
    pub fn fmpz_mpoly_q_randtest(
        res: *mut fmpz_mpoly_q_struct,
        state: *mut flint_rand_s,
        length: mp_limb_signed_t,
        coeff_bits: mp_limb_t,
        exp_bound: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_equal(
        x: *const fmpz_mpoly_q_struct,
        y: *const fmpz_mpoly_q_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    ) -> libc::c_int;
    pub fn fmpz_mpoly_q_neg(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpz_mpoly_q_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_add(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpz_mpoly_q_struct,
        y: *const fmpz_mpoly_q_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_sub(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpz_mpoly_q_struct,
        y: *const fmpz_mpoly_q_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_mul(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpz_mpoly_q_struct,
        y: *const fmpz_mpoly_q_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_div(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpz_mpoly_q_struct,
        y: *const fmpz_mpoly_q_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_inv(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpz_mpoly_q_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn _fmpz_mpoly_q_add(
        res_num: *mut fmpz_mpoly_struct,
        res_den: *mut fmpz_mpoly_struct,
        x_num: *const fmpz_mpoly_struct,
        x_den: *const fmpz_mpoly_struct,
        y_num: *const fmpz_mpoly_struct,
        y_den: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn _fmpz_mpoly_q_sub(
        res_num: *mut fmpz_mpoly_struct,
        res_den: *mut fmpz_mpoly_struct,
        x_num: *const fmpz_mpoly_struct,
        x_den: *const fmpz_mpoly_struct,
        y_num: *const fmpz_mpoly_struct,
        y_den: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn _fmpz_mpoly_q_mul(
        res_num: *mut fmpz_mpoly_struct,
        res_den: *mut fmpz_mpoly_struct,
        x_num: *const fmpz_mpoly_struct,
        x_den: *const fmpz_mpoly_struct,
        y_num: *const fmpz_mpoly_struct,
        y_den: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn _fmpz_mpoly_q_div(
        res_num: *mut fmpz_mpoly_struct,
        res_den: *mut fmpz_mpoly_struct,
        x_num: *const fmpz_mpoly_struct,
        x_den: *const fmpz_mpoly_struct,
        y_num: *const fmpz_mpoly_struct,
        y_den: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn _fmpz_mpoly_q_add_fmpq(
        res_num: *mut fmpz_mpoly_struct,
        res_den: *mut fmpz_mpoly_struct,
        x_num: *const fmpz_mpoly_struct,
        x_den: *const fmpz_mpoly_struct,
        y_num: *const fmpz,
        y_den: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn _fmpz_mpoly_q_sub_fmpq(
        res_num: *mut fmpz_mpoly_struct,
        res_den: *mut fmpz_mpoly_struct,
        x_num: *const fmpz_mpoly_struct,
        x_den: *const fmpz_mpoly_struct,
        y_num: *const fmpz,
        y_den: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn _fmpz_mpoly_q_mul_fmpq(
        res_num: *mut fmpz_mpoly_struct,
        res_den: *mut fmpz_mpoly_struct,
        x_num: *const fmpz_mpoly_struct,
        x_den: *const fmpz_mpoly_struct,
        y_num: *const fmpz,
        y_den: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_add_fmpz(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpz_mpoly_q_struct,
        y: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_add_fmpq(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpz_mpoly_q_struct,
        y: *const fmpq,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_sub_fmpz(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpz_mpoly_q_struct,
        y: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_sub_fmpq(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpz_mpoly_q_struct,
        y: *const fmpq,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_mul_fmpz(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpz_mpoly_q_struct,
        y: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_mul_fmpq(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpz_mpoly_q_struct,
        y: *const fmpq,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_div_fmpz(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpz_mpoly_q_struct,
        y: *const fmpz,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_div_fmpq(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpz_mpoly_q_struct,
        y: *const fmpq,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    #[link_name = "fmpz_mpoly_q_add_si__extern"]
    pub fn fmpz_mpoly_q_add_si(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpz_mpoly_q_struct,
        c: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    #[link_name = "fmpz_mpoly_q_sub_si__extern"]
    pub fn fmpz_mpoly_q_sub_si(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpz_mpoly_q_struct,
        c: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    #[link_name = "fmpz_mpoly_q_mul_si__extern"]
    pub fn fmpz_mpoly_q_mul_si(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpz_mpoly_q_struct,
        c: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    #[link_name = "fmpz_mpoly_q_div_si__extern"]
    pub fn fmpz_mpoly_q_div_si(
        res: *mut fmpz_mpoly_q_struct,
        x: *const fmpz_mpoly_q_struct,
        c: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    #[link_name = "_fmpz_vec_content2__extern"]
    pub fn _fmpz_vec_content2(
        res: *mut fmpz,
        vec: *const fmpz,
        len: mp_limb_signed_t,
        inp: *const fmpz,
    );
    #[link_name = "fmpz_mpoly_gcd_assert_successful__extern"]
    pub fn fmpz_mpoly_gcd_assert_successful(
        res: *mut fmpz_mpoly_struct,
        x: *const fmpz_mpoly_struct,
        y: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    #[link_name = "_fmpz_mpoly_q_mpoly_divexact__extern"]
    pub fn _fmpz_mpoly_q_mpoly_divexact(
        res: *mut fmpz_mpoly_struct,
        x: *const fmpz_mpoly_struct,
        y: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    #[link_name = "_fmpz_mpoly_q_content__extern"]
    pub fn _fmpz_mpoly_q_content(
        num: *mut fmpz,
        den: *mut fmpz,
        xnum: *const fmpz_mpoly_struct,
        xden: *const fmpz_mpoly_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    #[link_name = "fmpz_mpoly_q_content__extern"]
    pub fn fmpz_mpoly_q_content(
        res: *mut fmpq,
        x: *const fmpz_mpoly_q_struct,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
    pub fn fmpz_mpoly_q_evaluate_acb(
        res: *mut acb_struct,
        f: *const fmpz_mpoly_q_struct,
        x: acb_srcptr,
        prec: mp_limb_signed_t,
        ctx: *const fmpz_mpoly_ctx_struct,
    );
}
