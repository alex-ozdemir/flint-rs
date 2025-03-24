/* automatically generated by rust-bindgen 0.70.1 */

use crate::deps::*;
use crate::acf_types::*;
use crate::arb_types::*;
use crate::arf::*;
use crate::arf_types::*;
use crate::flint::*;


extern "C" {
    #[link_name = "acf_init__extern"]
    pub fn acf_init(x: *mut acf_struct);
    #[link_name = "acf_clear__extern"]
    pub fn acf_clear(x: *mut acf_struct);
    #[link_name = "_acf_vec_init__extern"]
    pub fn _acf_vec_init(n: mp_limb_signed_t) -> acf_ptr;
    #[link_name = "_acf_vec_clear__extern"]
    pub fn _acf_vec_clear(v: acf_ptr, n: mp_limb_signed_t);
    #[link_name = "acf_real_ptr__extern"]
    pub fn acf_real_ptr(z: *mut acf_struct) -> arf_ptr;
    #[link_name = "acf_imag_ptr__extern"]
    pub fn acf_imag_ptr(z: *mut acf_struct) -> arf_ptr;
    #[link_name = "acf_set__extern"]
    pub fn acf_set(z: *mut acf_struct, x: *const acf_struct);
    #[link_name = "acf_swap__extern"]
    pub fn acf_swap(z: *mut acf_struct, x: *mut acf_struct);
    #[link_name = "acf_equal__extern"]
    pub fn acf_equal(x: *const acf_struct, y: *const acf_struct) -> libc::c_int;
    #[link_name = "acf_printd__extern"]
    pub fn acf_printd(x: *const acf_struct, n: mp_limb_signed_t);
    #[link_name = "acf_bits__extern"]
    pub fn acf_bits(x: *const acf_struct) -> mp_limb_signed_t;
    #[link_name = "acf_allocated_bytes__extern"]
    pub fn acf_allocated_bytes(x: *const acf_struct) -> mp_limb_signed_t;
    #[link_name = "acf_randtest__extern"]
    pub fn acf_randtest(
        x: *mut acf_struct,
        state: *mut flint_rand_s,
        bits: mp_limb_signed_t,
        mag_bits: mp_limb_signed_t,
    );
    #[link_name = "acf_get_mag__extern"]
    pub fn acf_get_mag(res: *mut mag_struct, x: *const acf_struct);
    #[link_name = "acf_neg__extern"]
    pub fn acf_neg(z: *mut acf_struct, x: *const acf_struct);
    #[link_name = "acf_set_round__extern"]
    pub fn acf_set_round(
        res: *mut acf_struct,
        x: *const acf_struct,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    #[link_name = "acf_neg_round__extern"]
    pub fn acf_neg_round(
        res: *mut acf_struct,
        x: *const acf_struct,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    #[link_name = "acf_add__extern"]
    pub fn acf_add(
        res: *mut acf_struct,
        x: *const acf_struct,
        y: *const acf_struct,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    #[link_name = "acf_sub__extern"]
    pub fn acf_sub(
        res: *mut acf_struct,
        x: *const acf_struct,
        y: *const acf_struct,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    #[link_name = "acf_mul__extern"]
    pub fn acf_mul(
        res: *mut acf_struct,
        x: *const acf_struct,
        y: *const acf_struct,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    ) -> libc::c_int;
    pub fn acf_approx_inv(
        res: *mut acf_struct,
        x: *const acf_struct,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    );
    pub fn acf_approx_div(
        res: *mut acf_struct,
        x: *const acf_struct,
        y: *const acf_struct,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    );
    pub fn acf_approx_sqrt(
        res: *mut acf_struct,
        x: *const acf_struct,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    );
    pub fn acf_approx_dot(
        res: *mut acf_struct,
        initial: *const acf_struct,
        subtract: libc::c_int,
        x: acf_srcptr,
        xstep: mp_limb_signed_t,
        y: acf_srcptr,
        ystep: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
        rnd: arf_rnd_t,
    );
}
