#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//! *See the [Arb documentation](https://arblib.org/).

use crate::dlog::dlog_precomp_struct;
use crate::deps::*;
use crate::nmod_vec::nmod_t;
use libc::c_int;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dirichlet_prime_group_struct {
    pub p: mp_limb_t,
    pub e: c_int,
    pub pe: nmod_t,
    pub phi: nmod_t,
    pub g: mp_limb_t,
    pub dlog: *mut dlog_precomp_struct,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dirichlet_group_struct {
    pub q: mp_limb_t,
    pub q_even: mp_limb_t,
    pub mod_: nmod_t,
    pub rad_q: mp_limb_t,
    pub phi_q: mp_limb_t,
    pub neven: mp_limb_signed_t,
    pub num: mp_limb_signed_t,
    pub expo: mp_limb_t,
    pub P: *mut dirichlet_prime_group_struct,
    pub generators: *mut mp_limb_t,
    pub PHI: *mut mp_limb_t,
}

pub type dirichlet_group_t = [dirichlet_group_struct; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dirichlet_char_struct {
    pub n: mp_limb_t,
    pub log: *mut mp_limb_t,
}

pub type dirichlet_char_t = [dirichlet_char_struct; 1usize];

extern "C" {
    pub fn dirichlet_group_num_primitive(G: *mut dirichlet_group_struct) -> mp_limb_t;
    pub fn dirichlet_group_init(G: *mut dirichlet_group_struct, q: mp_limb_t);
    pub fn dirichlet_subgroup_init(
        H: *mut dirichlet_group_struct,
        G: *mut dirichlet_group_struct,
        h: mp_limb_t,
    );
    pub fn dirichlet_group_clear(G: *mut dirichlet_group_struct);
    pub fn dirichlet_group_dlog_precompute(G: *mut dirichlet_group_struct, num: mp_limb_t);
    pub fn dirichlet_group_dlog_clear(G: *mut dirichlet_group_struct);
    pub fn dirichlet_conductor_ui(G: *mut dirichlet_group_struct, a: mp_limb_t) -> mp_limb_t;
    pub fn dirichlet_parity_ui(G: *mut dirichlet_group_struct, a: mp_limb_t) -> c_int;
    pub fn dirichlet_order_ui(G: *mut dirichlet_group_struct, a: mp_limb_t) -> mp_limb_t;
    pub fn dirichlet_char_init(x: *mut dirichlet_char_struct, G: *mut dirichlet_group_struct);
    pub fn dirichlet_char_clear(x: *mut dirichlet_char_struct);
    pub fn dirichlet_char_print(G: *mut dirichlet_group_struct, x: *mut dirichlet_char_struct);
    pub fn dirichlet_char_eq_deep(
        G: *mut dirichlet_group_struct,
        x: *mut dirichlet_char_struct,
        y: *mut dirichlet_char_struct,
    ) -> c_int;
    pub fn dirichlet_parity_char(
        G: *mut dirichlet_group_struct,
        x: *mut dirichlet_char_struct,
    ) -> c_int;
    pub fn dirichlet_conductor_char(
        G: *mut dirichlet_group_struct,
        x: *mut dirichlet_char_struct,
    ) -> mp_limb_t;
    pub fn dirichlet_order_char(
        G: *mut dirichlet_group_struct,
        x: *mut dirichlet_char_struct,
    ) -> mp_limb_t;
    pub fn dirichlet_char_log(
        x: *mut dirichlet_char_struct,
        G: *mut dirichlet_group_struct,
        m: mp_limb_t,
    );
    pub fn _dirichlet_char_exp(
        x: *mut dirichlet_char_struct,
        G: *mut dirichlet_group_struct,
    ) -> mp_limb_t;
    pub fn dirichlet_char_index(
        x: *mut dirichlet_char_struct,
        G: *mut dirichlet_group_struct,
        j: mp_limb_t,
    );
    pub fn dirichlet_index_char(
        G: *mut dirichlet_group_struct,
        x: *mut dirichlet_char_struct,
    ) -> mp_limb_t;
    pub fn dirichlet_char_one(x: *mut dirichlet_char_struct, G: *mut dirichlet_group_struct);
    pub fn dirichlet_char_first_primitive(
        x: *mut dirichlet_char_struct,
        G: *mut dirichlet_group_struct,
    );
    pub fn dirichlet_char_next(
        x: *mut dirichlet_char_struct,
        G: *mut dirichlet_group_struct,
    ) -> c_int;
    pub fn dirichlet_char_next_primitive(
        x: *mut dirichlet_char_struct,
        G: *mut dirichlet_group_struct,
    ) -> c_int;
    pub fn dirichlet_char_mul(
        c: *mut dirichlet_char_struct,
        G: *mut dirichlet_group_struct,
        a: *mut dirichlet_char_struct,
        b: *mut dirichlet_char_struct,
    );
    pub fn dirichlet_char_pow(
        c: *mut dirichlet_char_struct,
        G: *mut dirichlet_group_struct,
        a: *mut dirichlet_char_struct,
        n: mp_limb_t,
    );
    pub fn dirichlet_char_lower(
        y: *mut dirichlet_char_struct,
        H: *mut dirichlet_group_struct,
        x: *mut dirichlet_char_struct,
        G: *mut dirichlet_group_struct,
    );
    pub fn dirichlet_char_lift(
        y: *mut dirichlet_char_struct,
        G: *mut dirichlet_group_struct,
        x: *mut dirichlet_char_struct,
        H: *mut dirichlet_group_struct,
    );
    pub fn dirichlet_pairing(
        G: *mut dirichlet_group_struct,
        m: mp_limb_t,
        n: mp_limb_t,
    ) -> mp_limb_t;
    pub fn dirichlet_pairing_char(
        G: *mut dirichlet_group_struct,
        a: *mut dirichlet_char_struct,
        b: *mut dirichlet_char_struct,
    ) -> mp_limb_t;
    pub fn dirichlet_chi(
        G: *mut dirichlet_group_struct,
        chi: *mut dirichlet_char_struct,
        n: mp_limb_t,
    ) -> mp_limb_t;
    pub fn dirichlet_vec_set_null(
        v: *mut mp_limb_t,
        G: *mut dirichlet_group_struct,
        nv: mp_limb_signed_t,
    );
    pub fn dirichlet_chi_vec_loop(
        v: *mut mp_limb_t,
        G: *mut dirichlet_group_struct,
        chi: *mut dirichlet_char_struct,
        nv: mp_limb_signed_t,
    );
    pub fn dirichlet_chi_vec_primeloop(
        v: *mut mp_limb_t,
        G: *mut dirichlet_group_struct,
        chi: *mut dirichlet_char_struct,
        nv: mp_limb_signed_t,
    );
    pub fn dirichlet_chi_vec(
        v: *mut mp_limb_t,
        G: *mut dirichlet_group_struct,
        chi: *mut dirichlet_char_struct,
        nv: mp_limb_signed_t,
    );
    pub fn dirichlet_chi_vec_loop_order(
        v: *mut mp_limb_t,
        G: *mut dirichlet_group_struct,
        chi: *mut dirichlet_char_struct,
        order: mp_limb_t,
        nv: mp_limb_signed_t,
    );
    pub fn dirichlet_chi_vec_primeloop_order(
        v: *mut mp_limb_t,
        G: *mut dirichlet_group_struct,
        chi: *mut dirichlet_char_struct,
        order: mp_limb_t,
        nv: mp_limb_signed_t,
    );
    pub fn dirichlet_chi_vec_order(
        v: *mut mp_limb_t,
        G: *mut dirichlet_group_struct,
        chi: *mut dirichlet_char_struct,
        order: mp_limb_t,
        nv: mp_limb_signed_t,
    );
}
