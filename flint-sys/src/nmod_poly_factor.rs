#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//! *See the [FLINT documentation](http://flintlib.org/doc/nmod_poly_factor.html).

use crate::deps::*;
use crate::flint::*;
use crate::nmod_poly::nmod_poly_struct;
use crate::nmod_vec::nmod_t;
use crate::ulong_extras::n_factor_t;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nmod_poly_factor_struct {
    pub p: *mut nmod_poly_struct,
    pub exp: *mut mp_limb_signed_t,
    pub num: mp_limb_signed_t,
    pub alloc: mp_limb_signed_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nmod_poly_interval_poly_arg_t {
    pub baby: *mut nmod_poly_struct,
    pub res: *mut nmod_poly_struct,
    pub H: *mut nmod_poly_struct,
    pub v: *mut nmod_poly_struct,
    pub vinv: *mut nmod_poly_struct,
    pub tmp: mp_ptr,
    pub m: mp_limb_signed_t,
}

pub type nmod_poly_factor_t = [nmod_poly_factor_struct; 1usize];

extern "C" {
    pub fn nmod_poly_factor_init(fac: *mut nmod_poly_factor_struct);
    pub fn nmod_poly_factor_clear(fac: *mut nmod_poly_factor_struct);
    pub fn nmod_poly_factor_realloc(fac: *mut nmod_poly_factor_struct, alloc: mp_limb_signed_t);
    pub fn nmod_poly_factor_fit_length(fac: *mut nmod_poly_factor_struct, len: mp_limb_signed_t);
    pub fn nmod_poly_factor_set(
        res: *mut nmod_poly_factor_struct,
        fac: *mut nmod_poly_factor_struct,
    );
    pub fn nmod_poly_factor_swap(a: *mut nmod_poly_factor_struct, b: *mut nmod_poly_factor_struct);
    pub fn nmod_poly_factor_insert(
        fac: *mut nmod_poly_factor_struct,
        poly: *mut nmod_poly_struct,
        exp: mp_limb_signed_t,
    );
    pub fn nmod_poly_factor_print(fac: *mut nmod_poly_factor_struct);
    pub fn nmod_poly_factor_concat(
        res: *mut nmod_poly_factor_struct,
        fac: *mut nmod_poly_factor_struct,
    );
    pub fn nmod_poly_factor_pow(fac: *mut nmod_poly_factor_struct, exp: mp_limb_signed_t);
    pub fn nmod_poly_factor_equal_deg(
        factors: *mut nmod_poly_factor_struct,
        pol: *mut nmod_poly_struct,
        d: mp_limb_signed_t,
    );
    pub fn nmod_poly_factor_equal_deg_prob(
        factor: *mut nmod_poly_struct,
        state: *mut flint_rand_s,
        pol: *mut nmod_poly_struct,
        d: mp_limb_signed_t,
    ) -> ::std::os::raw::c_int;
    pub fn nmod_poly_factor_distinct_deg(
        res: *mut nmod_poly_factor_struct,
        poly: *mut nmod_poly_struct,
        degs: *const *mut mp_limb_signed_t,
    );
    pub fn nmod_poly_remove(f: *mut nmod_poly_struct, p: *mut nmod_poly_struct) -> mp_limb_t;
    pub fn nmod_poly_factor_distinct_deg_threaded(
        res: *mut nmod_poly_factor_struct,
        poly: *mut nmod_poly_struct,
        degs: *const *mut mp_limb_signed_t,
    );
    pub fn nmod_poly_is_irreducible(f: *mut nmod_poly_struct) -> ::std::os::raw::c_int;
    pub fn nmod_poly_is_irreducible_rabin(f: *mut nmod_poly_struct) -> ::std::os::raw::c_int;
    pub fn nmod_poly_is_irreducible_ddf(f: *mut nmod_poly_struct) -> ::std::os::raw::c_int;
    pub fn _nmod_poly_is_squarefree(
        f: mp_srcptr,
        len: mp_limb_signed_t,
        mod_: nmod_t,
    ) -> ::std::os::raw::c_int;
    pub fn nmod_poly_is_squarefree(f: *mut nmod_poly_struct) -> ::std::os::raw::c_int;
    pub fn nmod_poly_factor_cantor_zassenhaus(
        res: *mut nmod_poly_factor_struct,
        f: *mut nmod_poly_struct,
    );
    pub fn nmod_poly_factor_berlekamp(
        factors: *mut nmod_poly_factor_struct,
        f: *mut nmod_poly_struct,
    );
    pub fn nmod_poly_factor_kaltofen_shoup(
        res: *mut nmod_poly_factor_struct,
        poly: *mut nmod_poly_struct,
    );
    pub fn nmod_poly_factor_squarefree(res: *mut nmod_poly_factor_struct, f: *mut nmod_poly_struct);
    pub fn nmod_poly_factor_with_berlekamp(
        result: *mut nmod_poly_factor_struct,
        input: *mut nmod_poly_struct,
    ) -> mp_limb_t;
    pub fn nmod_poly_factor_with_cantor_zassenhaus(
        result: *mut nmod_poly_factor_struct,
        input: *mut nmod_poly_struct,
    ) -> mp_limb_t;
    pub fn nmod_poly_factor_with_kaltofen_shoup(
        result: *mut nmod_poly_factor_struct,
        input: *mut nmod_poly_struct,
    ) -> mp_limb_t;
    pub fn nmod_poly_factor(
        result: *mut nmod_poly_factor_struct,
        input: *mut nmod_poly_struct,
    ) -> mp_limb_t;
    pub fn _nmod_poly_interval_poly_worker(arg_ptr: *mut ::std::os::raw::c_void);
    pub fn nmod_poly_roots(
        r: *mut nmod_poly_factor_struct,
        f: *mut nmod_poly_struct,
        with_multiplicity: ::std::os::raw::c_int,
    );
    pub fn nmod_poly_roots_factored(
        r: *mut nmod_poly_factor_struct,
        f: *mut nmod_poly_struct,
        with_multiplicity: ::std::os::raw::c_int,
        n: *const n_factor_t,
    ) -> ::std::os::raw::c_int;
    pub fn nmod_poly_factor_get_nmod_poly(
        z: *mut nmod_poly_struct,
        fac: *mut nmod_poly_factor_struct,
        i: mp_limb_signed_t,
    );
}
