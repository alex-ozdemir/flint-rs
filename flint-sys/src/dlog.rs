#![allow(non_camel_case_types)]

//! *See the [Arb documentation](https://arblib.org/).

use crate::deps::*;
use crate::nmod_vec::nmod_t;
use crate::ulong_extras::n_factor_t;
use libc::{c_int, c_uint};

pub const DLOG_MODPE: c_uint = 0;
pub const DLOG_CRT: c_uint = 1;
pub const DLOG_POWER: c_uint = 2;
pub const DLOG_BSGS: c_uint = 3;
pub const DLOG_TABLE: c_uint = 4;
pub const DLOG_23: c_uint = 5;
pub type _bindgen_ty_22 = c_uint;
pub type dlog_precomp_ptr = *mut dlog_precomp_struct;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dlog_1modpe_struct {
    pub inv1p: mp_limb_t,
    pub invloga1: mp_limb_t,
}

pub type dlog_1modpe_t = [dlog_1modpe_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dlog_modpe_struct {
    pub p: mp_limb_t,
    pub e: mp_limb_t,
    pub pe1: mp_limb_t,
    pub inva: mp_limb_t,
    pub pe: nmod_t,
    pub modp: *mut dlog_precomp_struct,
    pub modpe: dlog_1modpe_t,
}

pub type dlog_modpe_t = [dlog_modpe_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dlog_table_struct {
    pub mod_: mp_limb_t,
    pub table: *mut mp_limb_t,
}

pub type dlog_table_t = [dlog_table_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct apow {
    pub k: mp_limb_t,
    pub ak: mp_limb_t,
}

pub type apow_t = apow;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dlog_bsgs_struct {
    pub mod_: nmod_t,
    pub m: mp_limb_t,
    pub am: mp_limb_t,
    pub g: mp_limb_t,
    pub table: *mut apow_t,
}

pub type dlog_bsgs_t = [dlog_bsgs_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dlog_rho_struct {
    pub a: mp_limb_t,
    pub n: nmod_t,
    pub mod_: nmod_t,
    pub nisprime: c_int,
}

pub type dlog_rho_t = [dlog_rho_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dlog_crt_struct {
    pub mod_: nmod_t,
    pub n: nmod_t,
    pub num: mp_limb_t,
    pub expo: *mut mp_limb_t,
    pub crt_coeffs: *mut mp_limb_t,
    pub pre: dlog_precomp_ptr,
}

pub type dlog_crt_t = [dlog_crt_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dlog_power_struct {
    pub mod_: nmod_t,
    pub p: mp_limb_t,
    pub e: mp_limb_t,
    pub apk: *mut mp_limb_t,
    pub pre: *mut dlog_precomp_struct,
}

pub type dlog_power_t = [dlog_power_struct; 1usize];
pub type dlog_order23_t = [mp_limb_t; 1usize];

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlog_precomp_struct {
    pub type_: c_int,
    pub cost: mp_limb_t,
    pub t: dlog_precomp_struct__bindgen_ty_1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dlog_precomp_struct__bindgen_ty_1 {
    pub table: dlog_table_t,
    pub bsgs: dlog_bsgs_t,
    pub crt: dlog_crt_t,
    pub power: dlog_power_t,
    pub modpe: dlog_modpe_t,
    pub order23: dlog_order23_t,
}

pub type dlog_precomp_t = [dlog_precomp_struct; 1usize];

extern "C" {
    pub fn apow_cmp(x: *const apow_t, y: *const apow_t) -> c_int;
    pub fn dlog_precomp_modpe_init(
        pre: *mut dlog_precomp_struct,
        a: mp_limb_t,
        p: mp_limb_t,
        e: mp_limb_t,
        pe: mp_limb_t,
        num: mp_limb_t,
    );
    pub fn dlog_precomp_small_init(
        pre: *mut dlog_precomp_struct,
        a: mp_limb_t,
        mod_: mp_limb_t,
        n: mp_limb_t,
        num: mp_limb_t,
    );
    pub fn dlog_precomp_n_init(
        pre: *mut dlog_precomp_struct,
        a: mp_limb_t,
        mod_: mp_limb_t,
        n: mp_limb_t,
        num: mp_limb_t,
    );
    pub fn dlog_precomp_p_init(
        pre: *mut dlog_precomp_struct,
        a: mp_limb_t,
        mod_: mp_limb_t,
        p: mp_limb_t,
        num: mp_limb_t,
    );
    pub fn dlog_precomp_pe_init(
        pre: *mut dlog_precomp_struct,
        a: mp_limb_t,
        mod_: mp_limb_t,
        p: mp_limb_t,
        e: mp_limb_t,
        pe: mp_limb_t,
        num: mp_limb_t,
    );
    pub fn dlog_precomp_clear(pre: *mut dlog_precomp_struct);
    pub fn dlog_precomp(pre: *mut dlog_precomp_struct, b: mp_limb_t) -> mp_limb_t;
    pub fn dlog_order23_init(t: *mut mp_limb_t, a: mp_limb_t) -> mp_limb_t;
    pub fn dlog_table_init(t: *mut dlog_table_struct, a: mp_limb_t, mod_: mp_limb_t) -> mp_limb_t;
    pub fn dlog_crt_init(
        t: *mut dlog_crt_struct,
        a: mp_limb_t,
        mod_: mp_limb_t,
        n: mp_limb_t,
        num: mp_limb_t,
    ) -> mp_limb_t;
    pub fn dlog_power_init(
        t: *mut dlog_power_struct,
        a: mp_limb_t,
        mod_: mp_limb_t,
        p: mp_limb_t,
        e: mp_limb_t,
        num: mp_limb_t,
    ) -> mp_limb_t;
    pub fn dlog_modpe_init(
        t: *mut dlog_modpe_struct,
        a: mp_limb_t,
        p: mp_limb_t,
        e: mp_limb_t,
        pe: mp_limb_t,
        num: mp_limb_t,
    ) -> mp_limb_t;
    pub fn dlog_bsgs_init(
        t: *mut dlog_bsgs_struct,
        a: mp_limb_t,
        mod_: mp_limb_t,
        n: mp_limb_t,
        m: mp_limb_t,
    ) -> mp_limb_t;
    pub fn dlog_1modpe_init(
        t: *mut dlog_1modpe_struct,
        a1: mp_limb_t,
        p: mp_limb_t,
        e: mp_limb_t,
        pe: nmod_t,
    );
    pub fn dlog_rho_init(t: *mut dlog_rho_struct, a: mp_limb_t, mod_: mp_limb_t, n: mp_limb_t);
    pub fn dlog_once(b: mp_limb_t, a: mp_limb_t, mod_: nmod_t, n: mp_limb_t) -> mp_limb_t;
    pub fn dlog_crt_clear(t: *mut dlog_crt_struct);
    pub fn dlog_order23(t: *mut mp_limb_t, b: mp_limb_t) -> mp_limb_t;
    pub fn dlog_table(t: *mut dlog_table_struct, b: mp_limb_t) -> mp_limb_t;
    pub fn dlog_crt(t: *mut dlog_crt_struct, b: mp_limb_t) -> mp_limb_t;
    pub fn dlog_power(t: *mut dlog_power_struct, b: mp_limb_t) -> mp_limb_t;
    pub fn dlog_modpe(t: *mut dlog_modpe_struct, b: mp_limb_t) -> mp_limb_t;
    pub fn dlog_bsgs(t: *mut dlog_bsgs_struct, b: mp_limb_t) -> mp_limb_t;
    pub fn dlog_rho(t: *mut dlog_rho_struct, b: mp_limb_t) -> mp_limb_t;
    pub fn dlog_1modpe_1modp(
        b1: mp_limb_t,
        p: mp_limb_t,
        e: mp_limb_t,
        inv1p: mp_limb_t,
        pe: nmod_t,
    ) -> mp_limb_t;
    pub fn dlog_1modpe(
        t: *mut dlog_1modpe_struct,
        b1: mp_limb_t,
        p: mp_limb_t,
        e: mp_limb_t,
        pe: nmod_t,
    ) -> mp_limb_t;
    pub fn dlog_mod2e_1mod4(b1: mp_limb_t, e: mp_limb_t, inva: mp_limb_t, pe: nmod_t) -> mp_limb_t;
    pub fn dlog_mod2e(t: *mut dlog_modpe_struct, b: mp_limb_t) -> mp_limb_t;
    pub fn dlog_n_factor_group(fac: *mut n_factor_t, bound: mp_limb_t);
    pub fn dlog_vec_pindex_factorgcd(
        v: *mut mp_limb_t,
        nv: mp_limb_t,
        p: mp_limb_t,
        mod_: nmod_t,
        a: mp_limb_t,
        na: mp_limb_t,
        loga: mp_limb_t,
        logm1: mp_limb_t,
        order: nmod_t,
        maxtry: c_int,
    ) -> mp_limb_t;
    pub fn dlog_vec_fill(v: *mut mp_limb_t, nv: mp_limb_t, x: mp_limb_t);
    pub fn dlog_vec_set_not_found(v: *mut mp_limb_t, nv: mp_limb_t, mod_: nmod_t);
    pub fn dlog_vec_loop(
        v: *mut mp_limb_t,
        nv: mp_limb_t,
        a: mp_limb_t,
        va: mp_limb_t,
        mod_: nmod_t,
        na: mp_limb_t,
        order: nmod_t,
    );
    pub fn dlog_vec_loop_add(
        v: *mut mp_limb_t,
        nv: mp_limb_t,
        a: mp_limb_t,
        va: mp_limb_t,
        mod_: nmod_t,
        na: mp_limb_t,
        order: nmod_t,
    );
    pub fn dlog_vec_eratos_add(
        v: *mut mp_limb_t,
        nv: mp_limb_t,
        a: mp_limb_t,
        va: mp_limb_t,
        mod_: nmod_t,
        na: mp_limb_t,
        order: nmod_t,
    );
    pub fn dlog_vec_eratos(
        v: *mut mp_limb_t,
        nv: mp_limb_t,
        a: mp_limb_t,
        va: mp_limb_t,
        mod_: nmod_t,
        na: mp_limb_t,
        order: nmod_t,
    );
    pub fn dlog_vec_sieve_add(
        v: *mut mp_limb_t,
        nv: mp_limb_t,
        a: mp_limb_t,
        va: mp_limb_t,
        mod_: nmod_t,
        na: mp_limb_t,
        order: nmod_t,
    );
    pub fn dlog_vec_sieve(
        v: *mut mp_limb_t,
        nv: mp_limb_t,
        a: mp_limb_t,
        va: mp_limb_t,
        mod_: nmod_t,
        na: mp_limb_t,
        order: nmod_t,
    );
    pub fn dlog_vec_add(
        v: *mut mp_limb_t,
        nv: mp_limb_t,
        a: mp_limb_t,
        va: mp_limb_t,
        mod_: nmod_t,
        na: mp_limb_t,
        order: nmod_t,
    );
    pub fn dlog_vec(
        v: *mut mp_limb_t,
        nv: mp_limb_t,
        a: mp_limb_t,
        va: mp_limb_t,
        mod_: nmod_t,
        na: mp_limb_t,
        order: nmod_t,
    );
    pub fn dlog_vec_sieve_precomp(
        v: *mut mp_limb_t,
        nv: mp_limb_t,
        pre: *mut dlog_precomp_struct,
        a: mp_limb_t,
        va: mp_limb_t,
        mod_: nmod_t,
        na: mp_limb_t,
        order: nmod_t,
    );
    pub fn dlog_vec_sieve_add_precomp(
        v: *mut mp_limb_t,
        nv: mp_limb_t,
        pre: *mut dlog_precomp_struct,
        a: mp_limb_t,
        va: mp_limb_t,
        mod_: nmod_t,
        na: mp_limb_t,
        order: nmod_t,
    );
    pub fn dlog_vec_add_precomp(
        v: *mut mp_limb_t,
        nv: mp_limb_t,
        pre: *mut dlog_precomp_struct,
        a: mp_limb_t,
        va: mp_limb_t,
        mod_: nmod_t,
        na: mp_limb_t,
        order: nmod_t,
    );
}
