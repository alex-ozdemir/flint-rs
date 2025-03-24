/* automatically generated by rust-bindgen 0.70.1 */

use libc::*;
use crate::deps::*;
use crate::flint::*;
use crate::fq_nmod_types::*;
use crate::fq_zech_types::*;
use crate::nmod_types::*;


extern "C" {
    pub fn fq_zech_ctx_init_ui(
        ctx: *mut fq_zech_ctx_struct,
        p: mp_limb_t,
        d: mp_limb_signed_t,
        var: *const libc::c_char,
    );
    pub fn _fq_zech_ctx_init_conway_ui(
        ctx: *mut fq_zech_ctx_struct,
        p: mp_limb_t,
        d: mp_limb_signed_t,
        var: *const libc::c_char,
    ) -> libc::c_int;
    pub fn fq_zech_ctx_init_conway_ui(
        ctx: *mut fq_zech_ctx_struct,
        p: mp_limb_t,
        d: mp_limb_signed_t,
        var: *const libc::c_char,
    );
    pub fn fq_zech_ctx_init_random_ui(
        ctx: *mut fq_zech_ctx_struct,
        p: mp_limb_t,
        d: mp_limb_signed_t,
        var: *const libc::c_char,
    );
    pub fn fq_zech_ctx_init_fq_nmod_ctx_check(
        ctx: *mut fq_zech_ctx_struct,
        ctxn: *mut fq_nmod_ctx_struct,
    ) -> libc::c_int;
    pub fn fq_zech_ctx_init_fq_nmod_ctx(
        ctx: *mut fq_zech_ctx_struct,
        ctxn: *mut fq_nmod_ctx_struct,
    );
    pub fn fq_zech_ctx_init_modulus_check(
        ctx: *mut fq_zech_ctx_struct,
        modulus: *const nmod_poly_struct,
        var: *const libc::c_char,
    ) -> libc::c_int;
    pub fn fq_zech_ctx_init_modulus(
        ctx: *mut fq_zech_ctx_struct,
        modulus: *const nmod_poly_struct,
        var: *const libc::c_char,
    );
    pub fn fq_zech_ctx_init_randtest(
        ctx: *mut fq_zech_ctx_struct,
        state: *mut flint_rand_s,
        type_: libc::c_int,
    );
    pub fn fq_zech_ctx_init_randtest_reducible(
        ctx: *mut fq_zech_ctx_struct,
        state: *mut flint_rand_s,
        type_: libc::c_int,
    );
    pub fn fq_zech_ctx_clear(ctx: *mut fq_zech_ctx_struct);
    #[link_name = "fq_zech_ctx_modulus__extern"]
    pub fn fq_zech_ctx_modulus(ctx: *const fq_zech_ctx_struct) -> *const nmod_poly_struct;
    #[link_name = "fq_zech_ctx_degree__extern"]
    pub fn fq_zech_ctx_degree(ctx: *const fq_zech_ctx_struct) -> mp_limb_signed_t;
    #[link_name = "fq_zech_ctx_prime__extern"]
    pub fn fq_zech_ctx_prime(ctx: *const fq_zech_ctx_struct) -> mp_limb_t;
    #[link_name = "fq_zech_ctx_order_ui__extern"]
    pub fn fq_zech_ctx_order_ui(ctx: *const fq_zech_ctx_struct) -> mp_limb_t;
    pub fn fq_zech_ctx_fprint(file: *mut FILE, ctx: *const fq_zech_ctx_struct) -> libc::c_int;
    pub fn fq_zech_ctx_print(ctx: *const fq_zech_ctx_struct);
    #[link_name = "fq_zech_init__extern"]
    pub fn fq_zech_init(rop: *mut fq_zech_struct, ctx: *const fq_zech_ctx_struct);
    #[link_name = "fq_zech_init2__extern"]
    pub fn fq_zech_init2(rop: *mut fq_zech_struct, ctx: *const fq_zech_ctx_struct);
    #[link_name = "fq_zech_clear__extern"]
    pub fn fq_zech_clear(rop: *mut fq_zech_struct, ctx: *const fq_zech_ctx_struct);
    #[link_name = "fq_zech_reduce__extern"]
    pub fn fq_zech_reduce(rop: *mut fq_zech_struct, ctx: *const fq_zech_ctx_struct);
    pub fn fq_zech_add(
        rop: *mut fq_zech_struct,
        op1: *const fq_zech_struct,
        op2: *const fq_zech_struct,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_sub(
        rop: *mut fq_zech_struct,
        op1: *const fq_zech_struct,
        op2: *const fq_zech_struct,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_sub_one(
        rop: *mut fq_zech_struct,
        op1: *const fq_zech_struct,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_neg(
        rop: *mut fq_zech_struct,
        op1: *const fq_zech_struct,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_mul(
        rop: *mut fq_zech_struct,
        op1: *const fq_zech_struct,
        op2: *const fq_zech_struct,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_mul_fmpz(
        rop: *mut fq_zech_struct,
        op: *const fq_zech_struct,
        x: *const fmpz,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_mul_si(
        rop: *mut fq_zech_struct,
        op: *const fq_zech_struct,
        x: mp_limb_signed_t,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_mul_ui(
        rop: *mut fq_zech_struct,
        op: *const fq_zech_struct,
        x: mp_limb_t,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_sqr(
        rop: *mut fq_zech_struct,
        op: *const fq_zech_struct,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_inv(
        rop: *mut fq_zech_struct,
        op1: *const fq_zech_struct,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn _fq_zech_pow(
        rop: *mut fmpz,
        op: *const fmpz,
        len: mp_limb_signed_t,
        e: *const fmpz,
        a: *const fmpz,
        j: *const mp_limb_signed_t,
        lena: mp_limb_signed_t,
        p: *const fmpz,
    );
    pub fn fq_zech_pow(
        rop: *mut fq_zech_struct,
        op1: *const fq_zech_struct,
        e: *const fmpz,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_pow_ui(
        rop: *mut fq_zech_struct,
        op1: *const fq_zech_struct,
        e: mp_limb_t,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_sqrt(
        rop: *mut fq_zech_struct,
        op1: *const fq_zech_struct,
        ctx: *const fq_zech_ctx_struct,
    ) -> libc::c_int;
    pub fn fq_zech_pth_root(
        rop: *mut fq_zech_struct,
        op1: *const fq_zech_struct,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_is_square(
        op1: *const fq_zech_struct,
        ctx: *const fq_zech_ctx_struct,
    ) -> libc::c_int;
    pub fn fq_zech_randtest(
        rop: *mut fq_zech_struct,
        state: *mut flint_rand_s,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_randtest_not_zero(
        rop: *mut fq_zech_struct,
        state: *mut flint_rand_s,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_rand(
        rop: *mut fq_zech_struct,
        state: *mut flint_rand_s,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_rand_not_zero(
        rop: *mut fq_zech_struct,
        state: *mut flint_rand_s,
        ctx: *const fq_zech_ctx_struct,
    );
    #[link_name = "fq_zech_equal__extern"]
    pub fn fq_zech_equal(
        op1: *const fq_zech_struct,
        op2: *const fq_zech_struct,
        ctx: *const fq_zech_ctx_struct,
    ) -> libc::c_int;
    #[link_name = "fq_zech_is_zero__extern"]
    pub fn fq_zech_is_zero(
        op: *const fq_zech_struct,
        ctx: *const fq_zech_ctx_struct,
    ) -> libc::c_int;
    #[link_name = "fq_zech_is_one__extern"]
    pub fn fq_zech_is_one(op: *const fq_zech_struct, ctx: *const fq_zech_ctx_struct)
        -> libc::c_int;
    #[link_name = "fq_zech_set__extern"]
    pub fn fq_zech_set(
        rop: *mut fq_zech_struct,
        op: *const fq_zech_struct,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_set_fmpz(
        rop: *mut fq_zech_struct,
        x: *const fmpz,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_set_si(
        rop: *mut fq_zech_struct,
        x: mp_limb_signed_t,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_set_ui(rop: *mut fq_zech_struct, x: mp_limb_t, ctx: *const fq_zech_ctx_struct);
    #[link_name = "fq_zech_swap__extern"]
    pub fn fq_zech_swap(
        op1: *mut fq_zech_struct,
        op2: *mut fq_zech_struct,
        ctx: *const fq_zech_ctx_struct,
    );
    #[link_name = "fq_zech_zero__extern"]
    pub fn fq_zech_zero(rop: *mut fq_zech_struct, ctx: *const fq_zech_ctx_struct);
    #[link_name = "fq_zech_one__extern"]
    pub fn fq_zech_one(rop: *mut fq_zech_struct, ctx: *const fq_zech_ctx_struct);
    #[link_name = "fq_zech_gen__extern"]
    pub fn fq_zech_gen(rop: *mut fq_zech_struct, ctx: *const fq_zech_ctx_struct);
    pub fn fq_zech_get_fmpz(
        a: *mut fmpz,
        op: *const fq_zech_struct,
        ctx: *const fq_zech_ctx_struct,
    ) -> libc::c_int;
    pub fn fq_zech_set_fq_nmod(
        rop: *mut fq_zech_struct,
        op: *const nmod_poly_struct,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_get_fq_nmod(
        rop: *mut nmod_poly_struct,
        op: *const fq_zech_struct,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_get_nmod_poly(
        a: *mut nmod_poly_struct,
        b: *const fq_zech_struct,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_set_nmod_poly(
        a: *mut fq_zech_struct,
        b: *const nmod_poly_struct,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_fprint(
        file: *mut FILE,
        op: *const fq_zech_struct,
        ctx: *const fq_zech_ctx_struct,
    ) -> libc::c_int;
    pub fn fq_zech_fprint_pretty(
        file: *mut FILE,
        op: *const fq_zech_struct,
        ctx: *const fq_zech_ctx_struct,
    ) -> libc::c_int;
    pub fn fq_zech_print(op: *const fq_zech_struct, ctx: *const fq_zech_ctx_struct);
    pub fn fq_zech_print_pretty(op: *const fq_zech_struct, ctx: *const fq_zech_ctx_struct);
    pub fn fq_zech_get_str(
        op: *const fq_zech_struct,
        ctx: *const fq_zech_ctx_struct,
    ) -> *mut libc::c_char;
    pub fn fq_zech_get_str_pretty(
        op: *const fq_zech_struct,
        ctx: *const fq_zech_ctx_struct,
    ) -> *mut libc::c_char;
    pub fn fq_zech_trace(rop: *mut fmpz, op: *const fq_zech_struct, ctx: *const fq_zech_ctx_struct);
    pub fn fq_zech_frobenius(
        rop: *mut fq_zech_struct,
        op: *const fq_zech_struct,
        e: mp_limb_signed_t,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_norm(rop: *mut fmpz, op: *const fq_zech_struct, ctx: *const fq_zech_ctx_struct);
    pub fn fq_zech_bit_pack(
        f: *mut fmpz,
        op: *const fq_zech_struct,
        bit_size: mp_limb_t,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_bit_unpack(
        rop: *mut fq_zech_struct,
        f: *const fmpz,
        bit_size: mp_limb_t,
        ctx: *const fq_zech_ctx_struct,
    );
    pub fn fq_zech_ctx_init(
        arg1: *mut fq_zech_ctx_struct,
        arg2: *mut fmpz,
        arg3: mp_limb_signed_t,
        arg4: *const libc::c_char,
    );
    pub fn _fq_zech_ctx_init_conway(
        arg1: *mut fq_zech_ctx_struct,
        arg2: *mut fmpz,
        arg3: mp_limb_signed_t,
        arg4: *const libc::c_char,
    ) -> libc::c_int;
    pub fn fq_zech_ctx_init_conway(
        arg1: *mut fq_zech_ctx_struct,
        arg2: *mut fmpz,
        arg3: mp_limb_signed_t,
        arg4: *const libc::c_char,
    );
    pub fn fq_zech_ctx_init_random(
        arg1: *mut fq_zech_ctx_struct,
        arg2: *mut fmpz,
        arg3: mp_limb_signed_t,
        arg4: *const libc::c_char,
    );
    pub fn fq_zech_ctx_order(arg1: *mut fmpz, arg2: *const fq_zech_ctx_struct);
}
