/* automatically generated by rust-bindgen 0.70.1 */

use crate::deps::*;


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct n_poly_struct {
    pub coeffs: *mut mp_limb_t,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of n_poly_struct"][::std::mem::size_of::<n_poly_struct>() - 24usize];
    ["Alignment of n_poly_struct"][::std::mem::align_of::<n_poly_struct>() - 8usize];
    ["Offset of field: n_poly_struct::coeffs"]
        [::std::mem::offset_of!(n_poly_struct, coeffs) - 0usize];
    ["Offset of field: n_poly_struct::alloc"]
        [::std::mem::offset_of!(n_poly_struct, alloc) - 8usize];
    ["Offset of field: n_poly_struct::length"]
        [::std::mem::offset_of!(n_poly_struct, length) - 16usize];
};
impl Default for n_poly_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type n_poly_t = [n_poly_struct; 1usize];
pub type n_fq_poly_struct = n_poly_struct;
pub type n_fq_poly_t = n_poly_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct n_bpoly_struct {
    pub coeffs: *mut n_poly_struct,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of n_bpoly_struct"][::std::mem::size_of::<n_bpoly_struct>() - 24usize];
    ["Alignment of n_bpoly_struct"][::std::mem::align_of::<n_bpoly_struct>() - 8usize];
    ["Offset of field: n_bpoly_struct::coeffs"]
        [::std::mem::offset_of!(n_bpoly_struct, coeffs) - 0usize];
    ["Offset of field: n_bpoly_struct::alloc"]
        [::std::mem::offset_of!(n_bpoly_struct, alloc) - 8usize];
    ["Offset of field: n_bpoly_struct::length"]
        [::std::mem::offset_of!(n_bpoly_struct, length) - 16usize];
};
impl Default for n_bpoly_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type n_bpoly_t = [n_bpoly_struct; 1usize];
pub type n_fq_bpoly_struct = n_bpoly_struct;
pub type n_fq_bpoly_t = n_bpoly_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct n_tpoly_struct {
    pub coeffs: *mut n_bpoly_struct,
    pub alloc: mp_limb_signed_t,
    pub length: mp_limb_signed_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of n_tpoly_struct"][::std::mem::size_of::<n_tpoly_struct>() - 24usize];
    ["Alignment of n_tpoly_struct"][::std::mem::align_of::<n_tpoly_struct>() - 8usize];
    ["Offset of field: n_tpoly_struct::coeffs"]
        [::std::mem::offset_of!(n_tpoly_struct, coeffs) - 0usize];
    ["Offset of field: n_tpoly_struct::alloc"]
        [::std::mem::offset_of!(n_tpoly_struct, alloc) - 8usize];
    ["Offset of field: n_tpoly_struct::length"]
        [::std::mem::offset_of!(n_tpoly_struct, length) - 16usize];
};
impl Default for n_tpoly_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type n_tpoly_t = [n_tpoly_struct; 1usize];
pub type n_fq_tpoly_struct = n_tpoly_struct;
pub type n_fq_tpoly_t = n_tpoly_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct n_polyu_struct {
    pub exps: *mut mp_limb_t,
    pub coeffs: *mut mp_limb_t,
    pub length: mp_limb_signed_t,
    pub alloc: mp_limb_signed_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of n_polyu_struct"][::std::mem::size_of::<n_polyu_struct>() - 32usize];
    ["Alignment of n_polyu_struct"][::std::mem::align_of::<n_polyu_struct>() - 8usize];
    ["Offset of field: n_polyu_struct::exps"]
        [::std::mem::offset_of!(n_polyu_struct, exps) - 0usize];
    ["Offset of field: n_polyu_struct::coeffs"]
        [::std::mem::offset_of!(n_polyu_struct, coeffs) - 8usize];
    ["Offset of field: n_polyu_struct::length"]
        [::std::mem::offset_of!(n_polyu_struct, length) - 16usize];
    ["Offset of field: n_polyu_struct::alloc"]
        [::std::mem::offset_of!(n_polyu_struct, alloc) - 24usize];
};
impl Default for n_polyu_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type n_polyu_t = [n_polyu_struct; 1usize];
pub type n_fq_polyu_struct = n_polyu_struct;
pub type n_fq_polyu_t = n_polyu_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct n_polyun_struct {
    pub coeffs: *mut n_poly_struct,
    pub exps: *mut mp_limb_t,
    pub length: mp_limb_signed_t,
    pub alloc: mp_limb_signed_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of n_polyun_struct"][::std::mem::size_of::<n_polyun_struct>() - 32usize];
    ["Alignment of n_polyun_struct"][::std::mem::align_of::<n_polyun_struct>() - 8usize];
    ["Offset of field: n_polyun_struct::coeffs"]
        [::std::mem::offset_of!(n_polyun_struct, coeffs) - 0usize];
    ["Offset of field: n_polyun_struct::exps"]
        [::std::mem::offset_of!(n_polyun_struct, exps) - 8usize];
    ["Offset of field: n_polyun_struct::length"]
        [::std::mem::offset_of!(n_polyun_struct, length) - 16usize];
    ["Offset of field: n_polyun_struct::alloc"]
        [::std::mem::offset_of!(n_polyun_struct, alloc) - 24usize];
};
impl Default for n_polyun_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type n_polyun_t = [n_polyun_struct; 1usize];
pub type n_fq_polyun_struct = n_polyun_struct;
pub type n_fq_polyun_t = n_polyun_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct n_poly_stack_struct {
    pub array: *mut *mut n_poly_struct,
    pub alloc: mp_limb_signed_t,
    pub top: mp_limb_signed_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of n_poly_stack_struct"][::std::mem::size_of::<n_poly_stack_struct>() - 24usize];
    ["Alignment of n_poly_stack_struct"][::std::mem::align_of::<n_poly_stack_struct>() - 8usize];
    ["Offset of field: n_poly_stack_struct::array"]
        [::std::mem::offset_of!(n_poly_stack_struct, array) - 0usize];
    ["Offset of field: n_poly_stack_struct::alloc"]
        [::std::mem::offset_of!(n_poly_stack_struct, alloc) - 8usize];
    ["Offset of field: n_poly_stack_struct::top"]
        [::std::mem::offset_of!(n_poly_stack_struct, top) - 16usize];
};
impl Default for n_poly_stack_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type n_poly_stack_t = [n_poly_stack_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct n_bpoly_stack_struct {
    pub array: *mut *mut n_bpoly_struct,
    pub alloc: mp_limb_signed_t,
    pub top: mp_limb_signed_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of n_bpoly_stack_struct"][::std::mem::size_of::<n_bpoly_stack_struct>() - 24usize];
    ["Alignment of n_bpoly_stack_struct"][::std::mem::align_of::<n_bpoly_stack_struct>() - 8usize];
    ["Offset of field: n_bpoly_stack_struct::array"]
        [::std::mem::offset_of!(n_bpoly_stack_struct, array) - 0usize];
    ["Offset of field: n_bpoly_stack_struct::alloc"]
        [::std::mem::offset_of!(n_bpoly_stack_struct, alloc) - 8usize];
    ["Offset of field: n_bpoly_stack_struct::top"]
        [::std::mem::offset_of!(n_bpoly_stack_struct, top) - 16usize];
};
impl Default for n_bpoly_stack_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type n_bpoly_stack_t = [n_bpoly_stack_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct n_poly_bpoly_stack_struct {
    pub poly_stack: n_poly_stack_t,
    pub bpoly_stack: n_bpoly_stack_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of n_poly_bpoly_stack_struct"]
        [::std::mem::size_of::<n_poly_bpoly_stack_struct>() - 48usize];
    ["Alignment of n_poly_bpoly_stack_struct"]
        [::std::mem::align_of::<n_poly_bpoly_stack_struct>() - 8usize];
    ["Offset of field: n_poly_bpoly_stack_struct::poly_stack"]
        [::std::mem::offset_of!(n_poly_bpoly_stack_struct, poly_stack) - 0usize];
    ["Offset of field: n_poly_bpoly_stack_struct::bpoly_stack"]
        [::std::mem::offset_of!(n_poly_bpoly_stack_struct, bpoly_stack) - 24usize];
};
impl Default for n_poly_bpoly_stack_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type n_poly_bpoly_stack_t = [n_poly_bpoly_stack_struct; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct nmod_eval_interp_struct {
    pub M: *mut mp_limb_t,
    pub T: *mut mp_limb_t,
    pub Q: *mut mp_limb_t,
    pub array: *mut mp_limb_t,
    pub alloc: mp_limb_signed_t,
    pub d: mp_limb_signed_t,
    pub radix: mp_limb_signed_t,
    pub w: mp_limb_t,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of nmod_eval_interp_struct"][::std::mem::size_of::<nmod_eval_interp_struct>() - 64usize];
    ["Alignment of nmod_eval_interp_struct"]
        [::std::mem::align_of::<nmod_eval_interp_struct>() - 8usize];
    ["Offset of field: nmod_eval_interp_struct::M"]
        [::std::mem::offset_of!(nmod_eval_interp_struct, M) - 0usize];
    ["Offset of field: nmod_eval_interp_struct::T"]
        [::std::mem::offset_of!(nmod_eval_interp_struct, T) - 8usize];
    ["Offset of field: nmod_eval_interp_struct::Q"]
        [::std::mem::offset_of!(nmod_eval_interp_struct, Q) - 16usize];
    ["Offset of field: nmod_eval_interp_struct::array"]
        [::std::mem::offset_of!(nmod_eval_interp_struct, array) - 24usize];
    ["Offset of field: nmod_eval_interp_struct::alloc"]
        [::std::mem::offset_of!(nmod_eval_interp_struct, alloc) - 32usize];
    ["Offset of field: nmod_eval_interp_struct::d"]
        [::std::mem::offset_of!(nmod_eval_interp_struct, d) - 40usize];
    ["Offset of field: nmod_eval_interp_struct::radix"]
        [::std::mem::offset_of!(nmod_eval_interp_struct, radix) - 48usize];
    ["Offset of field: nmod_eval_interp_struct::w"]
        [::std::mem::offset_of!(nmod_eval_interp_struct, w) - 56usize];
};
impl Default for nmod_eval_interp_struct {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
pub type nmod_eval_interp_t = [nmod_eval_interp_struct; 1usize];
