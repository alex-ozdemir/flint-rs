#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//! *See the [Arb documentation](https://arblib.org/).

use crate::arb::arb_struct;
use crate::mag::{mag_struct, mag_t};
use crate::deps::*;
use crate::fmpz_poly::fmpz_poly_t;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hypgeom_struct {
    pub A: fmpz_poly_t,
    pub B: fmpz_poly_t,
    pub P: fmpz_poly_t,
    pub Q: fmpz_poly_t,
    pub have_precomputed: ::std::os::raw::c_int,
    pub r: mp_limb_signed_t,
    pub boundC: mp_limb_signed_t,
    pub boundD: mp_limb_signed_t,
    pub boundK: mp_limb_signed_t,
    pub MK: mag_t,
}

pub type hypgeom_t = [hypgeom_struct; 1usize];

extern "C" {
    pub fn hypgeom_init(hyp: *mut hypgeom_struct);
    pub fn hypgeom_clear(hyp: *mut hypgeom_struct);
    pub fn hypgeom_precompute(hyp: *mut hypgeom_struct);
    pub fn hypgeom_estimate_terms(
        z: *mut mag_struct,
        r: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn hypgeom_bound(
        error: *mut mag_struct,
        r: ::std::os::raw::c_int,
        C: mp_limb_signed_t,
        D: mp_limb_signed_t,
        K: mp_limb_signed_t,
        TK: *mut mag_struct,
        z: *mut mag_struct,
        prec: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn arb_hypgeom_sum(
        P: *mut arb_struct,
        Q: *mut arb_struct,
        hyp: *mut hypgeom_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_infsum(
        P: *mut arb_struct,
        Q: *mut arb_struct,
        hyp: *mut hypgeom_struct,
        target_prec: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
}
