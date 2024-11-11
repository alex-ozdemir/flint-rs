#![allow(non_camel_case_types)]

//! *See the [Arb documentation](https://arblib.org/).

use crate::acb::{acb_ptr, acb_srcptr, acb_struct};
use crate::acb_poly::acb_poly_struct;
use crate::arf::arf_struct;
use crate::deps::*;
use crate::flint::*;
use crate::fmpz::fmpz;
use crate::fmpz_poly::fmpz_poly_struct;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct psl2z_struct {
    pub a: fmpz,
    pub b: fmpz,
    pub c: fmpz,
    pub d: fmpz,
}

pub type psl2z_t = [psl2z_struct; 1usize];

extern "C" {
    pub fn psl2z_mul(h: *mut psl2z_struct, f: *mut psl2z_struct, g: *mut psl2z_struct);
    pub fn psl2z_inv(h: *mut psl2z_struct, g: *mut psl2z_struct);
    pub fn psl2z_is_one(g: *mut psl2z_struct) -> ::std::os::raw::c_int;
    pub fn psl2z_is_correct(g: *mut psl2z_struct) -> ::std::os::raw::c_int;
    pub fn psl2z_randtest(g: *mut psl2z_struct, state: *mut flint_rand_s, bits: mp_limb_signed_t);
    pub fn acb_modular_transform(
        w: *mut acb_struct,
        g: *mut psl2z_struct,
        z: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_modular_fundamental_domain_approx_d(
        g: *mut psl2z_struct,
        x: f64,
        y: f64,
        one_minus_eps: f64,
    );
    pub fn acb_modular_fundamental_domain_approx_arf(
        g: *mut psl2z_struct,
        xx: *mut arf_struct,
        yy: *mut arf_struct,
        one_minus_eps: *mut arf_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_modular_fundamental_domain_approx(
        w: *mut acb_struct,
        g: *mut psl2z_struct,
        z: *mut acb_struct,
        one_minus_eps: *mut arf_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_modular_is_in_fundamental_domain(
        z: *mut acb_struct,
        tol: *mut arf_struct,
        prec: mp_limb_signed_t,
    ) -> ::std::os::raw::c_int;
    pub fn acb_modular_addseq_theta(
        exponents: *mut mp_limb_signed_t,
        aindex: *mut mp_limb_signed_t,
        bindex: *mut mp_limb_signed_t,
        num: mp_limb_signed_t,
    );
    pub fn acb_modular_addseq_eta(
        exponents: *mut mp_limb_signed_t,
        aindex: *mut mp_limb_signed_t,
        bindex: *mut mp_limb_signed_t,
        num: mp_limb_signed_t,
    );
    pub fn acb_modular_fill_addseq(tab: *mut mp_limb_signed_t, len: mp_limb_signed_t);
    pub fn acb_modular_theta_transform(
        R: *mut ::std::os::raw::c_int,
        S: *mut ::std::os::raw::c_int,
        C: *mut ::std::os::raw::c_int,
        g: *mut psl2z_struct,
    );
    pub fn acb_modular_theta_const_sum(
        theta2: *mut acb_struct,
        theta3: *mut acb_struct,
        theta4: *mut acb_struct,
        q: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_modular_theta_const_sum_basecase(
        theta2: *mut acb_struct,
        theta3: *mut acb_struct,
        theta4: *mut acb_struct,
        q: *mut acb_struct,
        N: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_modular_theta_const_sum_rs(
        theta2: *mut acb_struct,
        theta3: *mut acb_struct,
        theta4: *mut acb_struct,
        q: *mut acb_struct,
        N: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_modular_theta_sum(
        theta1: acb_ptr,
        theta2: acb_ptr,
        theta3: acb_ptr,
        theta4: acb_ptr,
        w: *mut acb_struct,
        w_is_unit: ::std::os::raw::c_int,
        q: *mut acb_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_modular_theta_notransform(
        theta1: *mut acb_struct,
        theta2: *mut acb_struct,
        theta3: *mut acb_struct,
        theta4: *mut acb_struct,
        z: *mut acb_struct,
        tau: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_modular_theta(
        theta1: *mut acb_struct,
        theta2: *mut acb_struct,
        theta3: *mut acb_struct,
        theta4: *mut acb_struct,
        z: *mut acb_struct,
        tau: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_modular_theta_jet_notransform(
        theta1: acb_ptr,
        theta2: acb_ptr,
        theta3: acb_ptr,
        theta4: acb_ptr,
        z: *mut acb_struct,
        tau: *mut acb_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_modular_theta_jet(
        theta1: acb_ptr,
        theta2: acb_ptr,
        theta3: acb_ptr,
        theta4: acb_ptr,
        z: *mut acb_struct,
        tau: *mut acb_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_modular_theta_series(
        theta1: acb_ptr,
        theta2: acb_ptr,
        theta3: acb_ptr,
        theta4: acb_ptr,
        z: acb_srcptr,
        zlen: mp_limb_signed_t,
        tau: *mut acb_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_modular_theta_series(
        theta1: *mut acb_poly_struct,
        theta2: *mut acb_poly_struct,
        theta3: *mut acb_poly_struct,
        theta4: *mut acb_poly_struct,
        z: *mut acb_poly_struct,
        tau: *mut acb_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_modular_j(z: *mut acb_struct, tau: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_modular_epsilon_arg(g: *mut psl2z_struct) -> ::std::os::raw::c_int;
    pub fn acb_modular_eta_sum(eta: *mut acb_struct, q: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_modular_eta(z: *mut acb_struct, tau: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_modular_lambda(r: *mut acb_struct, tau: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_modular_delta(r: *mut acb_struct, tau: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_modular_eisenstein(
        r: acb_ptr,
        tau: *mut acb_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_modular_elliptic_p(
        r: *mut acb_struct,
        z: *mut acb_struct,
        tau: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_modular_elliptic_p_zpx(
        r: acb_ptr,
        z: *mut acb_struct,
        tau: *mut acb_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_modular_elliptic_k(k: *mut acb_struct, m: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_modular_elliptic_k_cpx(
        w: acb_ptr,
        m: *mut acb_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_modular_elliptic_e(res: *mut acb_struct, m: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_modular_hilbert_class_poly(res: *mut fmpz_poly_struct, D: mp_limb_signed_t);
    pub fn _acb_modular_mul(
        z: *mut acb_struct,
        tmp1: *mut acb_struct,
        tmp2: *mut acb_struct,
        x: *mut acb_struct,
        y: *mut acb_struct,
        wprec: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
}
