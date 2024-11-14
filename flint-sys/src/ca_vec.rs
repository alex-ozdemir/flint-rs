/* automatically generated by rust-bindgen 0.70.1 */

use crate::deps::*;
use crate::ca::*;
use crate::flint::*;
use crate::gr::*;


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ca_vec_struct {
    pub entries: ca_ptr,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ca_vec_struct"][::std::mem::size_of::<ca_vec_struct>() - 24usize];
    ["Alignment of ca_vec_struct"][::std::mem::align_of::<ca_vec_struct>() - 8usize];
    ["Offset of field: ca_vec_struct::entries"]
        [::std::mem::offset_of!(ca_vec_struct, entries) - 0usize];
    ["Offset of field: ca_vec_struct::alloc"]
        [::std::mem::offset_of!(ca_vec_struct, alloc) - 8usize];
    ["Offset of field: ca_vec_struct::length"]
        [::std::mem::offset_of!(ca_vec_struct, length) - 16usize];
};
impl Default for ca_vec_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type ca_vec_t = [ca_vec_struct; 1usize];
extern "C" {
    #[link_name = "ca_vec_entry_ptr__extern"]
    pub fn ca_vec_entry_ptr(vec: *mut ca_vec_struct, i: mp_limb_signed_t) -> ca_ptr;
    pub fn _ca_vec_init(len: mp_limb_signed_t, ctx: *mut ca_ctx_struct) -> ca_ptr;
    pub fn ca_vec_init(vec: *mut ca_vec_struct, len: mp_limb_signed_t, ctx: *mut ca_ctx_struct);
    pub fn _ca_vec_clear(v: ca_ptr, len: mp_limb_signed_t, ctx: *mut ca_ctx_struct);
    pub fn ca_vec_clear(vec: *mut ca_vec_struct, ctx: *mut ca_ctx_struct);
    pub fn _ca_vec_swap(vec1: ca_ptr, vec2: ca_ptr, len: mp_limb_signed_t, ctx: *mut ca_ctx_struct);
    #[link_name = "ca_vec_swap__extern"]
    pub fn ca_vec_swap(vec1: *mut ca_vec_struct, vec2: *mut ca_vec_struct, ctx: *mut ca_ctx_struct);
    #[link_name = "ca_vec_length__extern"]
    pub fn ca_vec_length(vec: *const ca_vec_struct, ctx: *mut ca_ctx_struct) -> mp_limb_signed_t;
    pub fn _ca_vec_fit_length(
        vec: *mut ca_vec_struct,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn ca_vec_set_length(
        res: *mut ca_vec_struct,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_vec_set(res: ca_ptr, src: ca_srcptr, len: mp_limb_signed_t, ctx: *mut ca_ctx_struct);
    pub fn ca_vec_set(res: *mut ca_vec_struct, src: *const ca_vec_struct, ctx: *mut ca_ctx_struct);
    pub fn _ca_vec_zero(res: ca_ptr, len: mp_limb_signed_t, ctx: *mut ca_ctx_struct);
    pub fn ca_vec_zero(res: *mut ca_vec_struct, len: mp_limb_signed_t, ctx: *mut ca_ctx_struct);
    #[link_name = "_ca_vec_unknown__extern"]
    pub fn _ca_vec_unknown(vec: ca_ptr, len: mp_limb_signed_t, ctx: *mut ca_ctx_struct);
    #[link_name = "_ca_vec_undefined__extern"]
    pub fn _ca_vec_undefined(vec: ca_ptr, len: mp_limb_signed_t, ctx: *mut ca_ctx_struct);
    pub fn ca_vec_print(vec: *const ca_vec_struct, ctx: *mut ca_ctx_struct);
    pub fn ca_vec_printn(
        vec: *const ca_vec_struct,
        digits: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    #[link_name = "ca_vec_append__extern"]
    pub fn ca_vec_append(vec: *mut ca_vec_struct, f: *const ca_struct, ctx: *mut ca_ctx_struct);
    pub fn _ca_vec_neg(res: ca_ptr, src: ca_srcptr, len: mp_limb_signed_t, ctx: *mut ca_ctx_struct);
    pub fn ca_vec_neg(res: *mut ca_vec_struct, src: *const ca_vec_struct, ctx: *mut ca_ctx_struct);
    pub fn _ca_vec_add(
        res: ca_ptr,
        vec1: ca_srcptr,
        vec2: ca_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_vec_sub(
        res: ca_ptr,
        vec1: ca_srcptr,
        vec2: ca_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_vec_scalar_mul_ca(
        res: ca_ptr,
        src: ca_srcptr,
        len: mp_limb_signed_t,
        c: *const ca_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_vec_scalar_div_ca(
        res: ca_ptr,
        src: ca_srcptr,
        len: mp_limb_signed_t,
        c: *const ca_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_vec_scalar_addmul_ca(
        res: ca_ptr,
        vec: ca_srcptr,
        len: mp_limb_signed_t,
        c: *const ca_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_vec_scalar_submul_ca(
        res: ca_ptr,
        vec: ca_srcptr,
        len: mp_limb_signed_t,
        c: *const ca_struct,
        ctx: *mut ca_ctx_struct,
    );
    pub fn _ca_vec_check_is_zero(
        vec: ca_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    ) -> truth_t;
    #[link_name = "_ca_vec_is_fmpq_vec__extern"]
    pub fn _ca_vec_is_fmpq_vec(
        vec: ca_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    ) -> libc::c_int;
    #[link_name = "_ca_vec_fmpq_vec_is_fmpz_vec__extern"]
    pub fn _ca_vec_fmpq_vec_is_fmpz_vec(
        vec: ca_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    ) -> libc::c_int;
    #[link_name = "_ca_vec_fmpq_vec_get_fmpz_vec_den__extern"]
    pub fn _ca_vec_fmpq_vec_get_fmpz_vec_den(
        c: *mut fmpz,
        den: *mut fmpz,
        vec: ca_srcptr,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
    #[link_name = "_ca_vec_set_fmpz_vec_div_fmpz__extern"]
    pub fn _ca_vec_set_fmpz_vec_div_fmpz(
        res: ca_ptr,
        v: *const fmpz,
        den: *const fmpz,
        len: mp_limb_signed_t,
        ctx: *mut ca_ctx_struct,
    );
}
