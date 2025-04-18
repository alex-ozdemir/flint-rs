/* automatically generated by rust-bindgen 0.70.1 */

use crate::deps::*;
use crate::flint::*;


extern "C" {
    #[link_name = "fmpz_add_inline__extern"]
    pub fn fmpz_add_inline(z: *mut fmpz, x: *const fmpz, y: *const fmpz);
    #[link_name = "fmpz_add_si_inline__extern"]
    pub fn fmpz_add_si_inline(z: *mut fmpz, x: *const fmpz, y: mp_limb_signed_t);
    #[link_name = "fmpz_sub_si_inline__extern"]
    pub fn fmpz_sub_si_inline(z: *mut fmpz, x: *const fmpz, y: mp_limb_signed_t);
    #[link_name = "fmpz_add_ui_inline__extern"]
    pub fn fmpz_add_ui_inline(z: *mut fmpz, x: *const fmpz, y: mp_limb_t);
    #[link_name = "fmpz_add2_fmpz_si_inline__extern"]
    pub fn fmpz_add2_fmpz_si_inline(
        z: *mut fmpz,
        x: *const fmpz,
        y: *const fmpz,
        c: mp_limb_signed_t,
    );
    #[link_name = "fmpz_set_mpn_large__extern"]
    pub fn fmpz_set_mpn_large(z: *mut fmpz, src: mp_srcptr, n: mp_size_t, negative: libc::c_int);
    #[link_name = "fmpz_adiv_q_2exp__extern"]
    pub fn fmpz_adiv_q_2exp(z: *mut fmpz, x: *const fmpz, exp: mp_limb_t);
    #[link_name = "_fmpz_set_si_small__extern"]
    pub fn _fmpz_set_si_small(x: *mut fmpz, v: mp_limb_signed_t);
    pub fn _fmpz_sub_small_large(x: *const fmpz, y: *const fmpz) -> mp_limb_signed_t;
    #[link_name = "_fmpz_sub_small__extern"]
    pub fn _fmpz_sub_small(x: *const fmpz, y: *const fmpz) -> mp_limb_signed_t;
    #[link_name = "_fmpz_size__extern"]
    pub fn _fmpz_size(f: *const fmpz) -> mp_size_t;
    #[link_name = "fmpz_ui_mul_ui__extern"]
    pub fn fmpz_ui_mul_ui(r: *mut fmpz, a: mp_limb_t, b: mp_limb_t);
    #[link_name = "fmpz_max__extern"]
    pub fn fmpz_max(z: *mut fmpz, x: *const fmpz, y: *const fmpz);
    #[link_name = "fmpz_min__extern"]
    pub fn fmpz_min(z: *mut fmpz, x: *const fmpz, y: *const fmpz);
    pub fn fmpz_lshift_mpn(
        z: *mut fmpz,
        d: mp_srcptr,
        dn: mp_size_t,
        sgnbit: libc::c_int,
        shift: mp_limb_t,
    );
    #[link_name = "fmpz_allocated_bytes__extern"]
    pub fn fmpz_allocated_bytes(x: *const fmpz) -> mp_limb_signed_t;
}
