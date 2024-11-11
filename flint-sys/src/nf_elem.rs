use crate::deps::*;
use crate::flint::*;
use crate::fmpq::fmpq;
use crate::fmpq_mat::fmpq_mat_struct;
use crate::fmpq_poly::{fmpq_poly_struct, fmpq_poly_t};
use crate::fmpz::{fmpz, fmpz_t};
use crate::fmpz_mat::fmpz_mat_struct;
use crate::fmpz_mod::fmpz_mod_ctx_struct;
use crate::fmpz_mod_poly::fmpz_mod_poly_struct;
use crate::nmod_poly::nmod_poly_struct;

use libc::{c_char, c_int};

use crate::nf::nf_struct;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lnf_elem_struct {
    pub num: fmpz_t,
    pub den: fmpz_t,
}

pub type lnf_elem_t = [lnf_elem_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct qnf_elem_struct {
    pub num: [fmpz; 3usize],
    pub den: fmpz_t,
}

pub type qnf_elem_t = [qnf_elem_struct; 1usize];

#[repr(C)]
#[derive(Copy, Clone)]
pub union nf_elem_struct {
    pub elem: fmpq_poly_t,
    pub lelem: lnf_elem_t,
    pub qelem: qnf_elem_t,
}

pub type nf_elem_t = [nf_elem_struct; 1usize];

extern "C" {
    pub fn nf_elem_init(a: *mut nf_elem_struct, nf: *const nf_struct);
    pub fn nf_elem_clear(a: *mut nf_elem_struct, nf: *const nf_struct);
    pub fn nf_elem_randtest(
        a: *mut nf_elem_struct,
        state: *const flint_rand_s,
        bits: mp_bitcnt_t,
        nf: *const nf_struct,
    );
    pub fn nf_elem_randtest_not_zero(
        a: *mut nf_elem_struct,
        state: *const flint_rand_s,
        bits: mp_bitcnt_t,
        nf: *const nf_struct,
    );
    pub fn _nf_elem_reduce(a: *mut nf_elem_struct, nf: *const nf_struct);
    pub fn nf_elem_reduce(a: *mut nf_elem_struct, nf: *const nf_struct);
    pub fn _nf_elem_invertible_check(a: *const nf_elem_struct, nf: *const nf_struct) -> c_int;
    pub fn _nf_elem_equal(
        a: *const nf_elem_struct,
        b: *const nf_elem_struct,
        nf: *const nf_struct,
    ) -> c_int;
    pub fn nf_elem_equal(
        a: *const nf_elem_struct,
        b: *const nf_elem_struct,
        nf: *const nf_struct,
    ) -> c_int;
    pub fn nf_elem_is_gen(a: *const nf_elem_struct, nf: *const nf_struct) -> c_int;
    pub fn nf_elem_print_pretty(a: *const nf_elem_struct, nf: *const nf_struct, var: *const c_char);
    pub fn nf_elem_get_str_pretty(
        a: *const nf_elem_struct,
        var: *const c_char,
        nf: *const nf_struct,
    ) -> *const c_char;
    pub fn nf_elem_set_fmpq_poly(
        a: *mut nf_elem_struct,
        pol: *const fmpq_poly_struct,
        nf: *const nf_struct,
    );
    pub fn nf_elem_set_fmpz_mat_row(
        b: *mut nf_elem_struct,
        M: *const fmpz_mat_struct,
        i: mp_limb_signed_t,
        den: *const fmpz,
        nf: *const nf_struct,
    );
    pub fn nf_elem_get_fmpz_mat_row(
        M: *mut fmpz_mat_struct,
        i: mp_limb_signed_t,
        den: *const fmpz,
        b: *const nf_elem_struct,
        nf: *const nf_struct,
    );
    pub fn nf_elem_get_fmpq_poly(
        pol: *mut fmpq_poly_struct,
        a: *const nf_elem_struct,
        nf: *const nf_struct,
    );
    pub fn _nf_elem_get_nmod_poly(
        pol: *mut nmod_poly_struct,
        a: *const nf_elem_struct,
        nf: *const nf_struct,
    );
    pub fn nf_elem_get_nmod_poly_den(
        pol: *mut nmod_poly_struct,
        a: *const nf_elem_struct,
        nf: *const nf_struct,
        den: c_int,
    );
    pub fn nf_elem_get_nmod_poly(
        pol: *mut nmod_poly_struct,
        a: *const nf_elem_struct,
        nf: *const nf_struct,
    );
    pub fn _nf_elem_get_fmpz_mod_poly(
        pol: *mut fmpz_mod_poly_struct,
        a: *const nf_elem_struct,
        nf: *const nf_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn nf_elem_get_fmpz_mod_poly_den(
        pol: *mut fmpz_mod_poly_struct,
        a: *const nf_elem_struct,
        nf: *const nf_struct,
        den: c_int,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn nf_elem_get_fmpz_mod_poly(
        pol: *mut fmpz_mod_poly_struct,
        a: *const nf_elem_struct,
        nf: *const nf_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn nf_elem_get_coeff_fmpq(
        a: *mut fmpq,
        b: *const nf_elem_struct,
        i: mp_limb_signed_t,
        nf: *const nf_struct,
    );
    pub fn nf_elem_get_coeff_fmpz(
        a: *mut fmpz,
        b: *const nf_elem_struct,
        i: mp_limb_signed_t,
        nf: *const nf_struct,
    );
    pub fn _nf_elem_set_coeff_num_fmpz(
        a: *mut nf_elem_struct,
        i: mp_limb_signed_t,
        b: *const fmpz,
        nf: *const nf_struct,
    );
    pub fn nf_elem_add_si(
        a: *mut nf_elem_struct,
        b: *const nf_elem_struct,
        c: mp_limb_signed_t,
        nf: *const nf_struct,
    );
    pub fn nf_elem_add_fmpz(
        a: *mut nf_elem_struct,
        b: *const nf_elem_struct,
        c: *const fmpz,
        nf: *const nf_struct,
    );
    pub fn nf_elem_add_fmpq(
        a: *mut nf_elem_struct,
        b: *const nf_elem_struct,
        c: *const fmpq,
        nf: *const nf_struct,
    );
    pub fn nf_elem_sub_si(
        a: *mut nf_elem_struct,
        b: *const nf_elem_struct,
        c: mp_limb_signed_t,
        nf: *const nf_struct,
    );
    pub fn nf_elem_sub_fmpz(
        a: *mut nf_elem_struct,
        b: *const nf_elem_struct,
        c: *const fmpz,
        nf: *const nf_struct,
    );
    pub fn nf_elem_sub_fmpq(
        a: *mut nf_elem_struct,
        b: *const nf_elem_struct,
        c: *const fmpq,
        nf: *const nf_struct,
    );
    pub fn nf_elem_si_sub(
        a: *mut nf_elem_struct,
        c: mp_limb_signed_t,
        b: *const nf_elem_struct,
        nf: *const nf_struct,
    );
    pub fn nf_elem_fmpz_sub(
        a: *mut nf_elem_struct,
        c: *const fmpz,
        b: *const nf_elem_struct,
        nf: *const nf_struct,
    );
    pub fn nf_elem_fmpq_sub(
        a: *mut nf_elem_struct,
        c: *const fmpq,
        b: *const nf_elem_struct,
        nf: *const nf_struct,
    );
    pub fn nf_elem_scalar_mul_si(
        a: *mut nf_elem_struct,
        b: *const nf_elem_struct,
        c: mp_limb_signed_t,
        nf: *const nf_struct,
    );
    pub fn nf_elem_scalar_mul_fmpz(
        a: *mut nf_elem_struct,
        b: *const nf_elem_struct,
        c: *const fmpz,
        nf: *const nf_struct,
    );
    pub fn nf_elem_scalar_mul_fmpq(
        a: *mut nf_elem_struct,
        b: *const nf_elem_struct,
        c: *const fmpq,
        nf: *const nf_struct,
    );
    pub fn nf_elem_scalar_div_si(
        a: *mut nf_elem_struct,
        b: *const nf_elem_struct,
        c: mp_limb_signed_t,
        nf: *const nf_struct,
    );
    pub fn nf_elem_scalar_div_fmpz(
        a: *mut nf_elem_struct,
        b: *const nf_elem_struct,
        c: *const fmpz,
        nf: *const nf_struct,
    );
    pub fn nf_elem_scalar_div_fmpq(
        a: *mut nf_elem_struct,
        b: *const nf_elem_struct,
        c: *const fmpq,
        nf: *const nf_struct,
    );
    pub fn _nf_elem_add_lf(
        a: *mut nf_elem_struct,
        b: *const nf_elem_struct,
        c: *const nf_elem_struct,
        nf: *const nf_struct,
        can: c_int,
    );
    pub fn _nf_elem_sub_lf(
        a: *mut nf_elem_struct,
        b: *const nf_elem_struct,
        c: *const nf_elem_struct,
        nf: *const nf_struct,
        can: c_int,
    );
    pub fn _nf_elem_add_qf(
        a: *mut nf_elem_struct,
        b: *const nf_elem_struct,
        c: *const nf_elem_struct,
        nf: *const nf_struct,
        can: c_int,
    );
    pub fn _nf_elem_sub_qf(
        a: *mut nf_elem_struct,
        b: *const nf_elem_struct,
        c: *const nf_elem_struct,
        nf: *const nf_struct,
        can: c_int,
    );
    pub fn nf_elem_add_qf(
        a: *mut nf_elem_struct,
        b: *const nf_elem_struct,
        c: *const nf_elem_struct,
        nf: *const nf_struct,
    );
    pub fn nf_elem_sub_qf(
        a: *mut nf_elem_struct,
        b: *const nf_elem_struct,
        c: *const nf_elem_struct,
        nf: *const nf_struct,
    );
    pub fn nf_elem_mul_gen(a: *mut nf_elem_struct, b: *const nf_elem_struct, nf: *const nf_struct);
    pub fn _nf_elem_mul(
        a: *mut nf_elem_struct,
        b: *const nf_elem_struct,
        c: *const nf_elem_struct,
        nf: *const nf_struct,
    );
    pub fn nf_elem_mul(
        a: *mut nf_elem_struct,
        b: *const nf_elem_struct,
        c: *const nf_elem_struct,
        nf: *const nf_struct,
    );
    pub fn _nf_elem_mul_red(
        a: *mut nf_elem_struct,
        b: *const nf_elem_struct,
        c: *const nf_elem_struct,
        nf: *const nf_struct,
        red: c_int,
    );
    pub fn nf_elem_mul_red(
        a: *mut nf_elem_struct,
        b: *const nf_elem_struct,
        c: *const nf_elem_struct,
        nf: *const nf_struct,
        red: c_int,
    );
    pub fn _nf_elem_inv(a: *mut nf_elem_struct, b: *const nf_elem_struct, nf: *const nf_struct);
    pub fn nf_elem_inv(a: *mut nf_elem_struct, b: *const nf_elem_struct, nf: *const nf_struct);
    pub fn _nf_elem_div(
        a: *mut nf_elem_struct,
        b: *const nf_elem_struct,
        c: *const nf_elem_struct,
        nf: *const nf_struct,
    );
    pub fn nf_elem_div(
        a: *mut nf_elem_struct,
        b: *const nf_elem_struct,
        c: *const nf_elem_struct,
        nf: *const nf_struct,
    );
    pub fn _nf_elem_pow(
        res: *mut nf_elem_struct,
        b: *const nf_elem_struct,
        e: mp_limb_t,
        nf: *const nf_struct,
    );
    pub fn nf_elem_pow(
        res: *mut nf_elem_struct,
        a: *const nf_elem_struct,
        e: mp_limb_t,
        nf: *const nf_struct,
    );
    pub fn _nf_elem_norm(
        rnum: *mut fmpz,
        rden: *const fmpz,
        a: *const nf_elem_struct,
        nf: *const nf_struct,
    );
    pub fn nf_elem_norm(res: *mut fmpq, a: *const nf_elem_struct, nf: *const nf_struct);
    pub fn _nf_elem_norm_div(
        rnum: *mut fmpz,
        rden: *const fmpz,
        a: *const nf_elem_struct,
        nf: *const nf_struct,
        divisor: *const fmpz,
        nbits: mp_limb_signed_t,
    );
    pub fn nf_elem_norm_div(
        res: *mut fmpq,
        a: *const nf_elem_struct,
        nf: *const nf_struct,
        divisor: *const fmpz,
        nbits: mp_limb_signed_t,
    );
    pub fn _nf_elem_trace(
        rnum: *mut fmpz,
        rden: *mut fmpz,
        a: *const nf_elem_struct,
        nf: *const nf_struct,
    );
    pub fn nf_elem_trace(res: *mut fmpq, a: *const nf_elem_struct, nf: *const nf_struct);
    pub fn nf_elem_rep_mat(
        res: *mut fmpq_mat_struct,
        a: *const nf_elem_struct,
        nf: *const nf_struct,
    );
    pub fn nf_elem_rep_mat_fmpz_mat_den(
        res: *mut fmpz_mat_struct,
        den: *const fmpz,
        a: *const nf_elem_struct,
        nf: *const nf_struct,
    );
    pub fn _nf_elem_mod_fmpz(
        res: *mut nf_elem_struct,
        a: *const nf_elem_struct,
        mod_: *const fmpz,
        nf: *const nf_struct,
        sign: c_int,
    );
    pub fn nf_elem_mod_fmpz_den(
        res: *mut nf_elem_struct,
        a: *const nf_elem_struct,
        mod_: *const fmpz,
        nf: *const nf_struct,
        den: c_int,
    );
    pub fn nf_elem_smod_fmpz_den(
        res: *mut nf_elem_struct,
        a: *const nf_elem_struct,
        mod_: *const fmpz,
        nf: *const nf_struct,
        den: c_int,
    );
    pub fn nf_elem_mod_fmpz(
        res: *mut nf_elem_struct,
        a: *const nf_elem_struct,
        mod_: *const fmpz,
        nf: *const nf_struct,
    );
    pub fn nf_elem_smod_fmpz(
        res: *mut nf_elem_struct,
        a: *const nf_elem_struct,
        mod_: *const fmpz,
        nf: *const nf_struct,
    );
    pub fn nf_elem_coprime_den(
        res: *mut nf_elem_struct,
        a: *const nf_elem_struct,
        mod_: *const fmpz,
        nf: *const nf_struct,
    );
    pub fn nf_elem_coprime_den_signed(
        res: *mut nf_elem_struct,
        a: *const nf_elem_struct,
        mod_: *const fmpz,
        nf: *const nf_struct,
    );
}
