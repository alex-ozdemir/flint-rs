#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

//! *See the [FLINT documentation](http://flintlib.org/doc/fmpz_mod_poly_factor.html).

use crate::deps::*;
use crate::flint::*;
use crate::fmpz::fmpz;
use crate::fmpz_factor::fmpz_factor_struct;
use crate::fmpz_mod::fmpz_mod_ctx_struct;
use crate::fmpz_mod_poly::fmpz_mod_poly_struct;
use libc::{c_int, c_void};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mod_poly_factor_struct {
    pub poly: *mut fmpz_mod_poly_struct,
    pub exp: *mut mp_limb_signed_t,
    pub num: mp_limb_signed_t,
    pub alloc: mp_limb_signed_t,
}

pub type fmpz_mod_poly_factor_t = [fmpz_mod_poly_factor_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fmpz_mod_poly_interval_poly_arg_t {
    pub baby: *mut fmpz_mod_poly_struct,
    pub res: *mut fmpz_mod_poly_struct,
    pub H: *mut fmpz_mod_poly_struct,
    pub v: *mut fmpz_mod_poly_struct,
    pub vinv: *mut fmpz_mod_poly_struct,
    pub ctx: *const fmpz_mod_ctx_struct,
    pub tmp: *mut fmpz,
    pub m: mp_limb_signed_t,
}

extern "C" {
    pub fn fmpz_mod_poly_factor_init(
        fac: *mut fmpz_mod_poly_factor_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_factor_clear(
        fac: *mut fmpz_mod_poly_factor_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_factor_realloc(
        fac: *mut fmpz_mod_poly_factor_struct,
        alloc: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_factor_fit_length(
        fac: *mut fmpz_mod_poly_factor_struct,
        len: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_factor_set(
        res: *mut fmpz_mod_poly_factor_struct,
        fac: *const fmpz_mod_poly_factor_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_factor_swap(
        a: *mut fmpz_mod_poly_factor_struct,
        b: *mut fmpz_mod_poly_factor_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_factor_insert(
        fac: *mut fmpz_mod_poly_factor_struct,
        poly: *const fmpz_mod_poly_struct,
        exp: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_factor_print(
        fac: *const fmpz_mod_poly_factor_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_factor_concat(
        res: *mut fmpz_mod_poly_factor_struct,
        fac: *const fmpz_mod_poly_factor_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_factor_pow(
        fac: *mut fmpz_mod_poly_factor_struct,
        exp: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_is_irreducible(
        f: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mod_poly_is_irreducible_ddf(
        f: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mod_poly_is_irreducible_rabin(
        f: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mod_poly_is_irreducible_rabin_f(
        fac: *const fmpz,
        f: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    ) -> c_int;
    pub fn _fmpz_mod_poly_is_squarefree(
        f: *const fmpz,
        len: mp_limb_signed_t,
        p: *const fmpz,
    ) -> c_int;
    pub fn _fmpz_mod_poly_is_squarefree_f(
        fac: *const fmpz,
        f: *const fmpz,
        len: mp_limb_signed_t,
        p: *const fmpz,
    ) -> c_int;
    pub fn fmpz_mod_poly_is_squarefree(
        f: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mod_poly_is_squarefree_f(
        fac: *const fmpz,
        f: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mod_poly_factor_equal_deg_prob(
        factor: *mut fmpz_mod_poly_struct,
        state: *const flint_rand_s,
        pol: *const fmpz_mod_poly_struct,
        d: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mod_poly_factor_equal_deg_with_frob(
        factors: *mut fmpz_mod_poly_factor_struct,
        f: *const fmpz_mod_poly_struct,
        d: mp_limb_signed_t,
        frob: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_factor_equal_deg(
        factors: *mut fmpz_mod_poly_factor_struct,
        pol: *const fmpz_mod_poly_struct,
        d: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_factor_distinct_deg_with_frob(
        res: *mut fmpz_mod_poly_factor_struct,
        poly: *const fmpz_mod_poly_struct,
        polyinv: *const fmpz_mod_poly_struct,
        frob: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_factor_distinct_deg(
        res: *mut fmpz_mod_poly_factor_struct,
        poly: *const fmpz_mod_poly_struct,
        degs: *const *const mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_factor_distinct_deg_threaded_with_frob(
        res: *mut fmpz_mod_poly_factor_struct,
        poly: *const fmpz_mod_poly_struct,
        polyinv: *const fmpz_mod_poly_struct,
        frob: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_factor_distinct_deg_threaded(
        res: *mut fmpz_mod_poly_factor_struct,
        poly: *const fmpz_mod_poly_struct,
        degs: *const *const mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_factor_squarefree(
        res: *mut fmpz_mod_poly_factor_struct,
        f: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_factor(
        res: *mut fmpz_mod_poly_factor_struct,
        f: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_factor_cantor_zassenhaus(
        res: *mut fmpz_mod_poly_factor_struct,
        f: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_factor_kaltofen_shoup(
        res: *mut fmpz_mod_poly_factor_struct,
        poly: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_factor_berlekamp(
        factors: *mut fmpz_mod_poly_factor_struct,
        f: *const fmpz_mod_poly_struct,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn _fmpz_mod_poly_interval_poly_worker(arg_ptr: *mut c_void);
    pub fn fmpz_mod_poly_roots(
        r: *mut fmpz_mod_poly_factor_struct,
        f: *const fmpz_mod_poly_struct,
        with_multiplicity: c_int,
        ctx: *const fmpz_mod_ctx_struct,
    );
    pub fn fmpz_mod_poly_roots_factored(
        r: *mut fmpz_mod_poly_factor_struct,
        f: *const fmpz_mod_poly_struct,
        with_multiplicity: c_int,
        n: *const fmpz_factor_struct,
        ctx: *const fmpz_mod_ctx_struct,
    ) -> c_int;
    pub fn fmpz_mod_poly_factor_get_fmpz_mod_poly(
        z: *mut fmpz_mod_poly_struct,
        fac: *const fmpz_mod_poly_factor_struct,
        i: mp_limb_signed_t,
        ctx: *const fmpz_mod_ctx_struct,
    );
}
