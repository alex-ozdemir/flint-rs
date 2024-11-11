#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//! *See the [Arb documentation](https://arblib.org/).

use crate::acb::{acb_ptr, acb_srcptr};
use crate::deps::*;
use crate::nmod_vec::nmod_t;
use crate::ulong_extras::n_factor_t;
use libc::c_int;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct crt_struct {
    pub num: ::std::os::raw::c_int,
    pub n: nmod_t,
    pub m: [mp_limb_signed_t; 15usize],
    pub M: [mp_limb_t; 15usize],
    pub vM: [mp_limb_t; 15usize],
}
pub type crt_t = [crt_struct; 1usize];

pub type acb_dft_step_ptr = *mut acb_dft_step_struct;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acb_dft_cyc_struct {
    pub n: mp_limb_signed_t,
    pub z: acb_ptr,
    pub zclear: ::std::os::raw::c_int,
    pub num: mp_limb_signed_t,
    pub cyc: acb_dft_step_ptr,
}

pub type acb_dft_cyc_t = [acb_dft_cyc_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acb_dft_rad2_struct {
    pub e: ::std::os::raw::c_int,
    pub n: mp_limb_signed_t,
    pub dv: mp_limb_signed_t,
    pub nz: mp_limb_signed_t,
    pub z: acb_ptr,
}

pub type acb_dft_rad2_t = [acb_dft_rad2_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acb_dft_bluestein_struct {
    pub n: mp_limb_signed_t,
    pub dv: mp_limb_signed_t,
    pub z: acb_ptr,
    pub g: acb_ptr,
    pub rad2: acb_dft_rad2_t,
}

pub type acb_dft_bluestein_t = [acb_dft_bluestein_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acb_dft_prod_struct {
    pub n: mp_limb_signed_t,
    pub num: mp_limb_signed_t,
    pub cyc: acb_dft_step_ptr,
}

pub type acb_dft_prod_t = [acb_dft_prod_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acb_dft_crt_struct {
    pub n: mp_limb_signed_t,
    pub c: crt_t,
    pub dv: mp_limb_signed_t,
    pub cyc: acb_dft_step_ptr,
}

pub type acb_dft_crt_t = [acb_dft_crt_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct acb_dft_naive_struct {
    pub n: mp_limb_signed_t,
    pub dv: mp_limb_signed_t,
    pub zclear: ::std::os::raw::c_int,
    pub z: acb_ptr,
    pub dz: mp_limb_signed_t,
}

pub type acb_dft_naive_t = [acb_dft_naive_struct; 1usize];

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acb_dft_pre_struct {
    pub n: mp_limb_signed_t,
    pub type_: ::std::os::raw::c_int,
    pub t: acb_dft_pre_struct__bindgen_ty_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union acb_dft_pre_struct__bindgen_ty_1 {
    pub rad2: acb_dft_rad2_t,
    pub cyc: acb_dft_cyc_t,
    pub prod: acb_dft_prod_t,
    pub crt: acb_dft_crt_t,
    pub naive: acb_dft_naive_t,
    pub bluestein: acb_dft_bluestein_t,
}

pub type acb_dft_pre_t = [acb_dft_pre_struct; 1usize];

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acb_dft_step_struct {
    pub m: mp_limb_signed_t,
    pub M: mp_limb_signed_t,
    pub dv: mp_limb_signed_t,
    pub z: acb_srcptr,
    pub dz: mp_limb_signed_t,
    pub pre: acb_dft_pre_t,
}

pub const DFT_NAIVE: ::std::os::raw::c_uint = 0;
pub const DFT_CYC: ::std::os::raw::c_uint = 1;
pub const DFT_PROD: ::std::os::raw::c_uint = 2;
pub const DFT_CRT: ::std::os::raw::c_uint = 3;
pub const DFT_RAD2: ::std::os::raw::c_uint = 4;
pub const DFT_CONV: ::std::os::raw::c_uint = 5;
pub type _bindgen_ty_23 = ::std::os::raw::c_uint;

extern "C" {
    pub fn _acb_dft_naive(
        w: acb_ptr,
        v: acb_srcptr,
        dv: mp_limb_signed_t,
        z: acb_srcptr,
        dz: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_naive(w: acb_ptr, v: acb_srcptr, len: mp_limb_signed_t, prec: mp_limb_signed_t);
    pub fn acb_dft_crt(w: acb_ptr, v: acb_srcptr, len: mp_limb_signed_t, prec: mp_limb_signed_t);
    pub fn acb_dft_cyc(w: acb_ptr, v: acb_srcptr, len: mp_limb_signed_t, prec: mp_limb_signed_t);
    pub fn acb_dft_rad2_inplace(v: acb_ptr, e: c_int, prec: mp_limb_signed_t);
    pub fn acb_dft_rad2(w: acb_ptr, v: acb_srcptr, e: c_int, prec: mp_limb_signed_t);
    pub fn acb_dft_bluestein(
        w: acb_ptr,
        v: acb_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_prod(
        w: acb_ptr,
        v: acb_srcptr,
        cyc: *mut mp_limb_signed_t,
        num: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_rad2_inplace_threaded(v: acb_ptr, e: c_int, prec: mp_limb_signed_t);
    pub fn acb_dft_convol_naive(
        w: acb_ptr,
        f: acb_srcptr,
        g: acb_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_convol_dft(
        w: acb_ptr,
        f: acb_srcptr,
        g: acb_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_convol_rad2(
        w: acb_ptr,
        f: acb_srcptr,
        g: acb_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_convol_mullow(
        w: acb_ptr,
        f: acb_srcptr,
        g: acb_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_convol(
        w: acb_ptr,
        f: acb_srcptr,
        g: acb_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn crt_init(c: *mut crt_struct, n: mp_limb_t);
    pub fn crt_decomp(
        y: acb_ptr,
        x: acb_srcptr,
        dx: mp_limb_signed_t,
        c: *mut crt_struct,
        len: mp_limb_t,
    );
    pub fn crt_recomp(y: acb_ptr, x: acb_srcptr, c: *mut crt_struct, len: mp_limb_t);
    pub fn acb_dft_step(
        w: acb_ptr,
        v: acb_srcptr,
        cyc: acb_dft_step_ptr,
        num: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_precomp(
        w: acb_ptr,
        v: acb_srcptr,
        pre: *mut acb_dft_pre_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_inverse_precomp(
        w: acb_ptr,
        v: acb_srcptr,
        pre: *mut acb_dft_pre_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_naive_precomp(
        w: acb_ptr,
        v: acb_srcptr,
        pol: *mut acb_dft_naive_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_cyc_precomp(
        w: acb_ptr,
        v: acb_srcptr,
        cyc: *mut acb_dft_cyc_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_rad2_precomp_inplace(
        v: acb_ptr,
        rad2: *mut acb_dft_rad2_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_rad2_precomp(
        w: acb_ptr,
        v: acb_srcptr,
        rad2: *mut acb_dft_rad2_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_crt_precomp(
        w: acb_ptr,
        v: acb_srcptr,
        crt: *mut acb_dft_crt_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_prod_precomp(
        w: acb_ptr,
        v: acb_srcptr,
        prod: *mut acb_dft_prod_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_bluestein_precomp(
        w: acb_ptr,
        v: acb_srcptr,
        t: *mut acb_dft_bluestein_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_rad2_precomp_inplace_threaded(
        v: acb_ptr,
        rad2: *mut acb_dft_rad2_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_inverse_rad2_precomp_inplace(
        v: acb_ptr,
        rad2: *mut acb_dft_rad2_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_inverse_rad2_precomp(
        w: acb_ptr,
        v: acb_srcptr,
        rad2: *mut acb_dft_rad2_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_convol_rad2_precomp(
        w: acb_ptr,
        f: acb_srcptr,
        g: acb_srcptr,
        len: mp_limb_signed_t,
        arg1: *mut acb_dft_rad2_struct,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_dft_precomp_init(
        pre: *mut acb_dft_pre_struct,
        dv: mp_limb_signed_t,
        z: acb_ptr,
        dz: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_precomp_init(
        pre: *mut acb_dft_pre_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_precomp_clear(pre: *mut acb_dft_pre_struct);
    pub fn acb_dft(w: acb_ptr, v: acb_srcptr, len: mp_limb_signed_t, prec: mp_limb_signed_t);
    pub fn acb_dft_inverse(
        w: acb_ptr,
        v: acb_srcptr,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_dft_steps_prod(
        m: *mut mp_limb_signed_t,
        num: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    ) -> acb_dft_step_ptr;
    pub fn acb_dft_prod_clear(t: *mut acb_dft_prod_struct);
    pub fn _acb_dft_cyc_init_z_fac(
        t: *mut acb_dft_cyc_struct,
        fac: n_factor_t,
        dv: mp_limb_signed_t,
        z: acb_ptr,
        dz: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_dft_cyc_init(
        t: *mut acb_dft_cyc_struct,
        dv: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_cyc_clear(t: *mut acb_dft_cyc_struct);
    pub fn _acb_dft_naive_init(
        pol: *mut acb_dft_naive_struct,
        dv: mp_limb_signed_t,
        z: acb_ptr,
        dz: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_dft_rad2_init(
        t: *mut acb_dft_rad2_struct,
        dv: mp_limb_signed_t,
        e: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_dft_bluestein_init(
        t: *mut acb_dft_bluestein_struct,
        dv: mp_limb_signed_t,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_dft_crt_init(
        crt: *mut acb_dft_crt_struct,
        dv: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_crt_init(
        crt: *mut acb_dft_crt_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_dft_crt_clear(crt: *mut acb_dft_crt_struct);
}
