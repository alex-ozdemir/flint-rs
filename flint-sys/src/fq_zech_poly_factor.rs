/* automatically generated by rust-bindgen 0.70.1 */

use crate::deps::*;
use crate::fq_zech_types::*;


extern "C" {
    pub fn FQ_ZECH_POLY_ITERATED_FROBENIUS_CUTOFF(
        ctx: *const fq_zech_ctx_struct,
        length: mp_limb_signed_t,
    ) -> libc::c_int;
    pub fn fq_zech_poly_factor_get_poly(
        z: *mut fq_zech_poly_struct,
        fac: *const fq_zech_poly_factor_struct,
        i: mp_limb_signed_t,
        ctx: *const fq_zech_ctx_struct,
    );
}
