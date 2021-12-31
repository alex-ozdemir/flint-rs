#![allow(non_camel_case_types)]

//! *See the [FLINT documentation](http://flintlib.org/doc/nmod_vec.html).

use crate::deps::*;
use crate::flint::*;
use libc::c_int;

#[repr(C)]
#[derive(Debug, Copy, Clone, Hash)]
pub struct nmod_t {
    pub n: mp_limb_t,
    pub ninv: mp_limb_t,
    pub norm: mp_limb_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nmod_discrete_log_pohlig_hellman_table_entry_struct {
    pub gammapow: mp_limb_t,
    pub cm: mp_limb_t,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nmod_discrete_log_pohlig_hellman_entry_struct {
    pub exp: mp_limb_signed_t,
    pub prime: mp_limb_t,
    pub gamma: mp_limb_t,
    pub gammainv: mp_limb_t,
    pub startingbeta: mp_limb_t,
    pub co: mp_limb_t,
    pub startinge: mp_limb_t,
    pub idem: mp_limb_t,
    pub cbound: mp_limb_t,
    pub dbound: mp_limb_t,
    pub table: *mut nmod_discrete_log_pohlig_hellman_table_entry_struct,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nmod_discrete_log_pohlig_hellman_struct {
    pub mod_: nmod_t,
    pub alpha: mp_limb_t,
    pub alphainv: mp_limb_t,
    pub num_factors: mp_limb_signed_t,
    pub entries: *mut nmod_discrete_log_pohlig_hellman_entry_struct,
}

pub type nmod_discrete_log_pohlig_hellman_t = [nmod_discrete_log_pohlig_hellman_struct; 1usize];

extern "C" {
    pub fn _nmod_add(a: mp_limb_t, b: mp_limb_t, mod_: nmod_t) -> mp_limb_t;
    pub fn _nmod_sub(a: mp_limb_t, b: mp_limb_t, mod_: nmod_t) -> mp_limb_t;
    pub fn nmod_add(a: mp_limb_t, b: mp_limb_t, mod_: nmod_t) -> mp_limb_t;
    pub fn nmod_sub(a: mp_limb_t, b: mp_limb_t, mod_: nmod_t) -> mp_limb_t;
    pub fn nmod_neg(a: mp_limb_t, mod_: nmod_t) -> mp_limb_t;
    pub fn nmod_mul(a: mp_limb_t, b: mp_limb_t, mod_: nmod_t) -> mp_limb_t;
    pub fn nmod_addmul(a: mp_limb_t, b: mp_limb_t, c: mp_limb_t, mod_: nmod_t) -> mp_limb_t;
    pub fn nmod_inv(a: mp_limb_t, mod_: nmod_t) -> mp_limb_t;
    pub fn nmod_div(a: mp_limb_t, b: mp_limb_t, mod_: nmod_t) -> mp_limb_t;
    pub fn nmod_pow_ui(a: mp_limb_t, exp: mp_limb_t, mod_: nmod_t) -> mp_limb_t;
    pub fn nmod_init(mod_: *mut nmod_t, n: mp_limb_t);
    pub fn _nmod_vec_init(len: mp_limb_signed_t) -> mp_ptr;
    pub fn _nmod_vec_clear(vec: mp_ptr);
    pub fn _nmod_vec_randtest(
        vec: mp_ptr,
        state: *mut flint_rand_s,
        len: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn _nmod_vec_zero(vec: mp_ptr, len: mp_limb_signed_t);
    pub fn _nmod_vec_max_bits(vec: mp_srcptr, len: mp_limb_signed_t) -> mp_limb_t;
    pub fn _nmod_vec_set(res: mp_ptr, vec: mp_srcptr, len: mp_limb_signed_t);
    pub fn _nmod_vec_swap(a: mp_ptr, b: mp_ptr, length: mp_limb_signed_t);
    pub fn _nmod_vec_equal(vec: mp_srcptr, vec2: mp_srcptr, len: mp_limb_signed_t) -> c_int;
    pub fn _nmod_vec_is_zero(vec: mp_srcptr, len: mp_limb_signed_t) -> c_int;
    pub fn _nmod_vec_reduce(res: mp_ptr, vec: mp_srcptr, len: mp_limb_signed_t, mod_: nmod_t);
    pub fn _nmod_vec_add(
        res: mp_ptr,
        vec1: mp_srcptr,
        vec2: mp_srcptr,
        len: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn _nmod_vec_sub(
        res: mp_ptr,
        vec1: mp_srcptr,
        vec2: mp_srcptr,
        len: mp_limb_signed_t,
        mod_: nmod_t,
    );
    pub fn _nmod_vec_neg(res: mp_ptr, vec: mp_srcptr, len: mp_limb_signed_t, mod_: nmod_t);
    pub fn _nmod_vec_scalar_mul_nmod(
        res: mp_ptr,
        vec: mp_srcptr,
        len: mp_limb_signed_t,
        c: mp_limb_t,
        mod_: nmod_t,
    );
    pub fn _nmod_vec_scalar_mul_nmod_shoup(
        res: mp_ptr,
        vec: mp_srcptr,
        len: mp_limb_signed_t,
        c: mp_limb_t,
        mod_: nmod_t,
    );
    pub fn _nmod_vec_scalar_addmul_nmod(
        res: mp_ptr,
        vec: mp_srcptr,
        len: mp_limb_signed_t,
        c: mp_limb_t,
        mod_: nmod_t,
    );
    pub fn _nmod_vec_dot_bound_limbs(len: mp_limb_signed_t, mod_: nmod_t) -> c_int;
    pub fn _nmod_vec_dot(
        vec1: mp_srcptr,
        vec2: mp_srcptr,
        len: mp_limb_signed_t,
        mod_: nmod_t,
        nlimbs: c_int,
    ) -> mp_limb_t;
    pub fn _nmod_vec_dot_ptr(
        vec1: mp_srcptr,
        vec2: *const mp_ptr,
        offset: mp_limb_signed_t,
        len: mp_limb_signed_t,
        mod_: nmod_t,
        nlimbs: c_int,
    ) -> mp_limb_t;
    pub fn nmod_discrete_log_pohlig_hellman_init(L: *mut nmod_discrete_log_pohlig_hellman_struct);
    pub fn nmod_discrete_log_pohlig_hellman_clear(L: *mut nmod_discrete_log_pohlig_hellman_struct);
    pub fn nmod_discrete_log_pohlig_hellman_precompute_prime(
        L: *mut nmod_discrete_log_pohlig_hellman_struct,
        p: mp_limb_t,
    ) -> f64;
    pub fn nmod_discrete_log_pohlig_hellman_run(
        L: *mut nmod_discrete_log_pohlig_hellman_struct,
        y: mp_limb_t,
    ) -> mp_limb_t;
    pub fn nmod_discrete_log_pohlig_hellman_primitive_root(
        L: *mut nmod_discrete_log_pohlig_hellman_struct,
    ) -> mp_limb_t;
}
