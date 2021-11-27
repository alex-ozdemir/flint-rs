//! Bridge between FLINT and rug (GMP-based) integer types.

use std::mem::transmute;

pub fn fmpz_to_int(p: *const flint_sys::fmpz::fmpz) -> rug::Integer {
    unsafe {
        // Create a rug integer
        let mut res = rug::Integer::new();
        // Get GMP ptr
        let mpz_t_ptr: *mut gmp_mpfr_sys::gmp::mpz_t = res.as_raw_mut();
        // Cast to flint stub ptr. Safe b/c same layout
        let stub_ptr: *mut flint_sys::deps::__mpz_struct = transmute(mpz_t_ptr);
        // Copy flint integer's value into stub ptr
        flint_sys::fmpz::fmpz_get_mpz(stub_ptr, p);
        // return rug integer
        res
    }
}

pub fn int_to_fmpz(i: &rug::Integer) -> flint_sys::fmpz::fmpz {
    unsafe {
        // create flint integer
        let mut out = flint_sys::fmpz::fmpz::default();
        // initialize it
        flint_sys::fmpz::fmpz_init(&mut out);
        // Get GMP ptr
        let mpz_t_ptr: *const gmp_mpfr_sys::gmp::mpz_t = i.as_raw();
        // Cast to flint stub ptr. Safe b/c same layout
        let stub_ptr: *const flint_sys::deps::__mpz_struct = transmute(mpz_t_ptr);
        flint_sys::fmpz::fmpz_set_mpz(&mut out, stub_ptr);
        out
    }
}
