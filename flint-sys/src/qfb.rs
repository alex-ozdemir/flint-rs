use crate::deps::*;
use crate::fmpz::*;

use libc::c_int;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct qfb {
    pub a: fmpz_t,
    pub b: fmpz_t,
    pub c: fmpz_t,
}

pub type qfb_t = [qfb; 1usize];

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct qfb_hash_t {
    pub q: qfb_t,
    pub q2: qfb_t,
    pub iter: mp_limb_signed_t,
}

extern "C" {
    pub fn qfb_hash_init(depth: mp_limb_signed_t) -> *const qfb_hash_t;
    pub fn qfb_hash_clear(qhash: *mut qfb_hash_t, depth: mp_limb_signed_t);
    pub fn qfb_hash_insert(
        qhash: *mut qfb_hash_t,
        q: *const qfb,
        q2: *const qfb,
        iter: mp_limb_signed_t,
        depth: mp_limb_signed_t,
    );
    pub fn qfb_hash_find(
        qhash: *mut qfb_hash_t,
        q: *const qfb,
        depth: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn qfb_reduce(r: *mut qfb, f: *const qfb, D: *const fmpz);
    pub fn qfb_is_reduced(r: *const qfb) -> c_int;
    pub fn qfb_reduced_forms(forms: *mut *mut qfb, d: mp_limb_signed_t) -> mp_limb_signed_t;
    pub fn qfb_reduced_forms_large(forms: *mut *mut qfb, d: mp_limb_signed_t) -> mp_limb_signed_t;
    pub fn qfb_nucomp(r: *mut qfb, f: *const qfb, g: *const qfb, D: *const fmpz, L: *const fmpz);
    pub fn qfb_nudupl(r: *mut qfb, f: *const qfb, D: *const fmpz, L: *const fmpz);
    pub fn qfb_pow_ui(r: *mut qfb, f: *const qfb, D: *const fmpz, exp: mp_limb_t);
    pub fn qfb_pow(r: *mut qfb, f: *const qfb, D: *const fmpz, exp: *const fmpz);
    pub fn qfb_pow_with_root(
        r: *mut qfb,
        f: *const qfb,
        D: *const fmpz,
        e: *const fmpz,
        L: *const fmpz,
    );
    pub fn qfb_prime_form(r: *mut qfb, D: *const fmpz, p: *const fmpz);
    pub fn qfb_exponent_element(
        exponent: *mut fmpz,
        f: *const qfb,
        n: *const fmpz,
        B1: mp_limb_t,
        B2_sqrt: mp_limb_t,
    ) -> c_int;
    pub fn qfb_exponent(
        exponent: *mut fmpz,
        n: *const fmpz,
        B1: mp_limb_t,
        B2_sqrt: mp_limb_t,
        c: mp_limb_signed_t,
    ) -> c_int;
    pub fn qfb_exponent_grh(
        exponent: *mut fmpz,
        n: *const fmpz,
        B1: mp_limb_t,
        B2_sqrt: mp_limb_t,
    ) -> c_int;
}

#[inline]
pub unsafe fn qfb_init(r: *mut qfb) {
    fmpz_init(&mut (*r).a[0]);
    fmpz_init(&mut (*r).b[0]);
    fmpz_init(&mut (*r).c[0]);
}

#[inline]
pub unsafe fn qfb_clear(r: *mut qfb) {
    fmpz_clear(&mut (*r).a[0]);
    fmpz_clear(&mut (*r).b[0]);
    fmpz_clear(&mut (*r).c[0]);
}

#[inline]
pub unsafe fn qfb_equal(f: *const qfb, g: *const qfb) -> c_int {
    let b = fmpz_equal(&(*f).a[0], &(*g).a[0]) != 0
        && fmpz_equal(&(*f).b[0], &(*g).b[0]) != 0
        && fmpz_equal(&(*f).c[0], &(*g).c[0]) != 0;
    b as i32
}

#[inline]
pub unsafe fn qfb_set(f: *mut qfb, g: *const qfb) {
    fmpz_set(&mut (*f).a[0], &(*g).a[0]);
    fmpz_set(&mut (*f).b[0], &(*g).b[0]);
    fmpz_set(&mut (*f).c[0], &(*g).c[0]);
}

/*
pub unsafe fn qfb_discriminant(D: fmpz_t, f: *const qfb) {}
pub unsafe fn qfb_print(r: *mut qfb) {}
pub unsafe fn qfb_array_clear(r: *mut qfb) {}
pub unsafe fn qfb_inverse(r: *mut qfb) {}
pub unsafe fn qfb_is_principal_from(r: *mut qfb) {}
pub unsafe fn qfb_principal_from(r: *mut qfb) {}
pub unsafe fn qfb_is_primitive(r: *mut qfb) {}
*/
