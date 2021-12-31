#![allow(non_camel_case_types)]

//! *See the [FLINT documentation](http://flintlib.org/doc/fq_poly_factor.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpz_poly::fmpz_poly_struct;
use crate::fq::{fq_ctx_struct, fq_struct};
use crate::fq_poly::{fq_poly_struct, fq_poly_t};
use libc::{c_char, c_int};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fq_poly_factor_struct {
    pub poly: *mut fq_poly_struct,
    pub exp: *mut mp_limb_signed_t,
    pub num: mp_limb_signed_t,
    pub alloc: mp_limb_signed_t,
}

pub type fq_poly_factor_t = [fq_poly_factor_struct; 1usize];

extern "C" {
    pub fn FQ_POLY_ITERATED_FROBENIUS_CUTOFF(
        ctx: *mut fq_ctx_struct,
        length: mp_limb_signed_t,
    ) -> c_int;
    pub fn fq_poly_factor_init(fac: *mut fq_poly_factor_struct, ctx: *mut fq_ctx_struct);
    pub fn fq_poly_factor_clear(fac: *mut fq_poly_factor_struct, ctx: *mut fq_ctx_struct);
    pub fn fq_poly_factor_realloc(
        fac: *mut fq_poly_factor_struct,
        alloc: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_factor_fit_length(
        fac: *mut fq_poly_factor_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_factor_set(
        res: *mut fq_poly_factor_struct,
        fac: *mut fq_poly_factor_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_factor_insert(
        fac: *mut fq_poly_factor_struct,
        poly: *mut fq_poly_struct,
        exp: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_factor_print(fac: *mut fq_poly_factor_struct, ctx: *mut fq_ctx_struct);
    pub fn fq_poly_factor_print_pretty(
        fac: *mut fq_poly_factor_struct,
        var: *const c_char,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_factor_concat(
        res: *mut fq_poly_factor_struct,
        fac: *mut fq_poly_factor_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_factor_pow(
        fac: *mut fq_poly_factor_struct,
        exp: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn _fq_poly_is_squarefree(
        f: *const fq_struct,
        len: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    ) -> c_int;
    pub fn fq_poly_is_squarefree(f: *mut fq_poly_struct, ctx: *mut fq_ctx_struct) -> c_int;
    pub fn fq_poly_factor_squarefree(
        res: *mut fq_poly_factor_struct,
        f: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_is_irreducible(f: *mut fq_poly_struct, ctx: *mut fq_ctx_struct) -> c_int;
    pub fn fq_poly_is_irreducible_ddf(f: *mut fq_poly_struct, ctx: *mut fq_ctx_struct) -> c_int;
    pub fn fq_poly_is_irreducible_ben_or(f: *mut fq_poly_struct, ctx: *mut fq_ctx_struct) -> c_int;
    pub fn fq_poly_factor_distinct_deg(
        res: *mut fq_poly_factor_struct,
        poly: *mut fq_poly_struct,
        degs: *const *mut mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_factor_equal_deg_prob(
        factor: *mut fq_poly_struct,
        state: *mut flint_rand_s,
        pol: *mut fq_poly_struct,
        d: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    ) -> c_int;
    pub fn fq_poly_factor_equal_deg(
        factors: *mut fq_poly_factor_struct,
        pol: *mut fq_poly_struct,
        d: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_factor_cantor_zassenhaus(
        res: *mut fq_poly_factor_struct,
        f: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_factor_kaltofen_shoup(
        res: *mut fq_poly_factor_struct,
        poly: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_factor_berlekamp(
        factors: *mut fq_poly_factor_struct,
        f: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_factor_with_berlekamp(
        result: *mut fq_poly_factor_struct,
        leading_coeff: *mut fmpz_poly_struct,
        input: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_factor_with_cantor_zassenhaus(
        result: *mut fq_poly_factor_struct,
        leading_coeff: *mut fmpz_poly_struct,
        input: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_factor_with_kaltofen_shoup(
        result: *mut fq_poly_factor_struct,
        leading_coeff: *mut fmpz_poly_struct,
        input: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_factor(
        result: *mut fq_poly_factor_struct,
        leading_coeff: *mut fmpz_poly_struct,
        input: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_iterated_frobenius_preinv(
        rop: *mut fq_poly_t,
        n: mp_limb_signed_t,
        v: *mut fq_poly_struct,
        vinv: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_factor_split_single(
        linfactor: *mut fq_poly_struct,
        input: *mut fq_poly_struct,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_roots(
        r: *mut fq_poly_factor_struct,
        f: *mut fq_poly_struct,
        with_multiplicity: c_int,
        ctx: *mut fq_ctx_struct,
    );
    pub fn fq_poly_factor_get_poly(
        z: *mut fq_poly_struct,
        fac: *mut fq_poly_factor_struct,
        i: mp_limb_signed_t,
        ctx: *mut fq_ctx_struct,
    );
}
