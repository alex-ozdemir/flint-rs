/* automatically generated by rust-bindgen 0.70.1 */

use crate::deps::*;
use crate::flint::*;
use crate::limb_types::*;
use crate::nmod_types::*;


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nmod_poly_interval_poly_arg_t {
    pub baby: *mut nmod_poly_struct,
    pub res: *mut nmod_poly_struct,
    pub H: *mut nmod_poly_struct,
    pub v: *mut nmod_poly_struct,
    pub vinv: *mut nmod_poly_struct,
    pub tmp: mp_ptr,
    pub m: mp_limb_signed_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of nmod_poly_interval_poly_arg_t"]
        [::std::mem::size_of::<nmod_poly_interval_poly_arg_t>() - 56usize];
    ["Alignment of nmod_poly_interval_poly_arg_t"]
        [::std::mem::align_of::<nmod_poly_interval_poly_arg_t>() - 8usize];
    ["Offset of field: nmod_poly_interval_poly_arg_t::baby"]
        [::std::mem::offset_of!(nmod_poly_interval_poly_arg_t, baby) - 0usize];
    ["Offset of field: nmod_poly_interval_poly_arg_t::res"]
        [::std::mem::offset_of!(nmod_poly_interval_poly_arg_t, res) - 8usize];
    ["Offset of field: nmod_poly_interval_poly_arg_t::H"]
        [::std::mem::offset_of!(nmod_poly_interval_poly_arg_t, H) - 16usize];
    ["Offset of field: nmod_poly_interval_poly_arg_t::v"]
        [::std::mem::offset_of!(nmod_poly_interval_poly_arg_t, v) - 24usize];
    ["Offset of field: nmod_poly_interval_poly_arg_t::vinv"]
        [::std::mem::offset_of!(nmod_poly_interval_poly_arg_t, vinv) - 32usize];
    ["Offset of field: nmod_poly_interval_poly_arg_t::tmp"]
        [::std::mem::offset_of!(nmod_poly_interval_poly_arg_t, tmp) - 40usize];
    ["Offset of field: nmod_poly_interval_poly_arg_t::m"]
        [::std::mem::offset_of!(nmod_poly_interval_poly_arg_t, m) - 48usize];
};
impl Default for nmod_poly_interval_poly_arg_t {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
extern "C" {
    pub fn nmod_poly_factor_init(fac: *mut nmod_poly_factor_struct);
    pub fn nmod_poly_factor_clear(fac: *mut nmod_poly_factor_struct);
    pub fn nmod_poly_factor_realloc(fac: *mut nmod_poly_factor_struct, alloc: mp_limb_signed_t);
    pub fn nmod_poly_factor_fit_length(fac: *mut nmod_poly_factor_struct, len: mp_limb_signed_t);
    pub fn nmod_poly_factor_set(
        res: *mut nmod_poly_factor_struct,
        fac: *const nmod_poly_factor_struct,
    );
    #[link_name = "nmod_poly_factor_swap__extern"]
    pub fn nmod_poly_factor_swap(a: *mut nmod_poly_factor_struct, b: *mut nmod_poly_factor_struct);
    pub fn nmod_poly_factor_get_poly(
        a: *mut nmod_poly_struct,
        b: *const nmod_poly_factor_struct,
        i: mp_limb_signed_t,
    );
    pub fn nmod_poly_factor_insert(
        fac: *mut nmod_poly_factor_struct,
        poly: *const nmod_poly_struct,
        exp: mp_limb_signed_t,
    );
    pub fn nmod_poly_factor_print(fac: *const nmod_poly_factor_struct);
    pub fn nmod_poly_factor_print_pretty(
        fac: *const nmod_poly_factor_struct,
        var: *const libc::c_char,
    );
    pub fn nmod_poly_factor_concat(
        res: *mut nmod_poly_factor_struct,
        fac: *const nmod_poly_factor_struct,
    );
    pub fn nmod_poly_factor_pow(fac: *mut nmod_poly_factor_struct, exp: mp_limb_signed_t);
    pub fn nmod_poly_factor_equal_deg(
        factors: *mut nmod_poly_factor_struct,
        pol: *const nmod_poly_struct,
        d: mp_limb_signed_t,
    );
    pub fn nmod_poly_factor_equal_deg_prob(
        factor: *mut nmod_poly_struct,
        state: *mut flint_rand_s,
        pol: *const nmod_poly_struct,
        d: mp_limb_signed_t,
    ) -> libc::c_int;
    pub fn nmod_poly_factor_distinct_deg(
        res: *mut nmod_poly_factor_struct,
        poly: *const nmod_poly_struct,
        degs: *const *mut mp_limb_signed_t,
    );
    pub fn nmod_poly_factor_distinct_deg_threaded(
        res: *mut nmod_poly_factor_struct,
        poly: *const nmod_poly_struct,
        degs: *const *mut mp_limb_signed_t,
    );
    pub fn nmod_poly_is_irreducible(f: *const nmod_poly_struct) -> libc::c_int;
    pub fn nmod_poly_is_irreducible_rabin(f: *const nmod_poly_struct) -> libc::c_int;
    pub fn nmod_poly_is_irreducible_ddf(f: *const nmod_poly_struct) -> libc::c_int;
    pub fn _nmod_poly_is_squarefree(
        f: mp_srcptr,
        len: mp_limb_signed_t,
        mod_: nmod_t,
    ) -> libc::c_int;
    pub fn nmod_poly_is_squarefree(f: *const nmod_poly_struct) -> libc::c_int;
    pub fn nmod_poly_factor_cantor_zassenhaus(
        res: *mut nmod_poly_factor_struct,
        f: *const nmod_poly_struct,
    );
    pub fn nmod_poly_factor_berlekamp(
        factors: *mut nmod_poly_factor_struct,
        f: *const nmod_poly_struct,
    );
    pub fn nmod_poly_factor_kaltofen_shoup(
        res: *mut nmod_poly_factor_struct,
        poly: *const nmod_poly_struct,
    );
    pub fn nmod_poly_factor_squarefree(
        res: *mut nmod_poly_factor_struct,
        f: *const nmod_poly_struct,
    );
    pub fn nmod_poly_factor_with_berlekamp(
        result: *mut nmod_poly_factor_struct,
        input: *const nmod_poly_struct,
    ) -> mp_limb_t;
    pub fn nmod_poly_factor_with_cantor_zassenhaus(
        result: *mut nmod_poly_factor_struct,
        input: *const nmod_poly_struct,
    ) -> mp_limb_t;
    pub fn nmod_poly_factor_with_kaltofen_shoup(
        result: *mut nmod_poly_factor_struct,
        input: *const nmod_poly_struct,
    ) -> mp_limb_t;
    pub fn nmod_poly_factor(
        result: *mut nmod_poly_factor_struct,
        input: *const nmod_poly_struct,
    ) -> mp_limb_t;
    pub fn _nmod_poly_interval_poly_worker(arg_ptr: *mut libc::c_void);
    pub fn nmod_poly_roots(
        r: *mut nmod_poly_factor_struct,
        f: *const nmod_poly_struct,
        with_multiplicity: libc::c_int,
    );
    pub fn nmod_poly_roots_factored(
        r: *mut nmod_poly_factor_struct,
        f: *const nmod_poly_struct,
        with_multiplicity: libc::c_int,
        n: *const n_factor_t,
    ) -> libc::c_int;
}
