#![allow(non_camel_case_types)]

//! *See the [Arb documentation](https://arblib.org/).

use crate::deps::*;
use crate::acb::{acb_ptr, acb_struct};
use crate::arb::arb_struct;
use crate::arf_types::arf_struct;
use crate::mag::mag_struct;
use libc::{c_int, c_void};

pub type acb_calc_func_t = ::std::option::Option<
    unsafe extern "C" fn(
        out: acb_ptr,
        inp: *mut acb_struct,
        param: *mut c_void,
        order: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    ) -> c_int,
>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acb_calc_integrate_opt_struct {
    pub deg_limit: mp_limb_signed_t,
    pub eval_limit: mp_limb_signed_t,
    pub depth_limit: mp_limb_signed_t,
    pub use_heap: c_int,
    pub verbose: c_int,
}

pub type acb_calc_integrate_opt_t = [acb_calc_integrate_opt_struct; 1usize];

extern "C" {
    pub fn acb_calc_cauchy_bound(
        bound: *mut arb_struct,
        func: acb_calc_func_t,
        param: *mut c_void,
        x: *mut acb_struct,
        radius: *mut arb_struct,
        maxdepth: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_calc_integrate_taylor(
        res: *mut acb_struct,
        func: acb_calc_func_t,
        param: *mut c_void,
        a: *mut acb_struct,
        b: *mut acb_struct,
        inner_radius: *mut arf_struct,
        outer_radius: *mut arf_struct,
        accuracy_goal: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn acb_calc_integrate_opt_init(options: *mut acb_calc_integrate_opt_struct);
    pub fn acb_calc_integrate(
        res: *mut acb_struct,
        f: acb_calc_func_t,
        param: *mut c_void,
        a: *mut acb_struct,
        b: *mut acb_struct,
        goal: mp_limb_signed_t,
        tol: *mut mag_struct,
        options: *mut acb_calc_integrate_opt_struct,
        prec: mp_limb_signed_t,
    ) -> c_int;
    pub fn acb_calc_integrate_gl_auto_deg(
        res: *mut acb_struct,
        eval_count: *mut mp_limb_signed_t,
        f: acb_calc_func_t,
        param: *mut c_void,
        a: *mut acb_struct,
        b: *mut acb_struct,
        tol: *mut mag_struct,
        deg_limit: mp_limb_signed_t,
        verbose: c_int,
        prec: mp_limb_signed_t,
    ) -> c_int;
}
