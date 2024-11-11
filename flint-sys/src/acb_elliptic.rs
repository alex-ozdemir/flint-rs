#![allow(non_camel_case_types)]

//! *See the [Arb documentation](https://arblib.org/).

use crate::deps::*;
use crate::acb::{acb_ptr, acb_srcptr, acb_struct};
use crate::acb_poly::acb_poly_struct;

extern "C" {
    pub fn acb_elliptic_k(k: *mut acb_struct, m: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_elliptic_k_jet(
        w: acb_ptr,
        m: *mut acb_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_elliptic_k_series(
        res: acb_ptr,
        m: acb_srcptr,
        zlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_elliptic_k_series(
        res: *mut acb_poly_struct,
        m: *mut acb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_elliptic_e(res: *mut acb_struct, m: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_elliptic_rf(
        res: *mut acb_struct,
        x: *mut acb_struct,
        y: *mut acb_struct,
        z: *mut acb_struct,
        flags: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_elliptic_rj(
        res: *mut acb_struct,
        x: *mut acb_struct,
        y: *mut acb_struct,
        z: *mut acb_struct,
        p: *mut acb_struct,
        flags: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_elliptic_rj_carlson(
        res: *mut acb_struct,
        x: *mut acb_struct,
        y: *mut acb_struct,
        z: *mut acb_struct,
        p: *mut acb_struct,
        flags: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_elliptic_rj_integration(
        res: *mut acb_struct,
        x: *mut acb_struct,
        y: *mut acb_struct,
        z: *mut acb_struct,
        p: *mut acb_struct,
        flags: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_elliptic_rg(
        res: *mut acb_struct,
        x: *mut acb_struct,
        y: *mut acb_struct,
        z: *mut acb_struct,
        flags: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_elliptic_rc1(res: *mut acb_struct, x: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_elliptic_f(
        res: *mut acb_struct,
        phi: *mut acb_struct,
        m: *mut acb_struct,
        times_pi: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_elliptic_e_inc(
        res: *mut acb_struct,
        phi: *mut acb_struct,
        m: *mut acb_struct,
        times_pi: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_elliptic_pi(
        r: *mut acb_struct,
        n: *mut acb_struct,
        m: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_elliptic_pi_inc(
        res: *mut acb_struct,
        n: *mut acb_struct,
        phi: *mut acb_struct,
        m: *mut acb_struct,
        times_pi: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_elliptic_p(
        r: *mut acb_struct,
        z: *mut acb_struct,
        tau: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_elliptic_p_prime(
        r: *mut acb_struct,
        z: *mut acb_struct,
        tau: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_elliptic_p_jet(
        r: acb_ptr,
        z: *mut acb_struct,
        tau: *mut acb_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_elliptic_p_series(
        res: acb_ptr,
        z: acb_srcptr,
        zlen: mp_limb_signed_t,
        tau: *mut acb_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_elliptic_p_series(
        res: *mut acb_poly_struct,
        z: *mut acb_poly_struct,
        tau: *mut acb_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_elliptic_zeta(
        res: *mut acb_struct,
        z: *mut acb_struct,
        tau: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_elliptic_sigma(
        res: *mut acb_struct,
        z: *mut acb_struct,
        tau: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_elliptic_roots(
        e1: *mut acb_struct,
        e2: *mut acb_struct,
        e3: *mut acb_struct,
        tau: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_elliptic_invariants(
        g2: *mut acb_struct,
        g3: *mut acb_struct,
        tau: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_elliptic_inv_p(
        res: *mut acb_struct,
        z: *mut acb_struct,
        tau: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
}
