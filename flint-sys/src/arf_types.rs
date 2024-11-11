#![allow(non_camel_case_types)]

//! *See the [Arb documentation](https://arblib.org/).

use derivative::Derivative;

use crate::deps::*;
use crate::flint::*;
use crate::fmpz::fmpz;

use libc::{c_char, c_int, FILE};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mantissa_noptr_struct {
    pub d: [mp_limb_t; 2usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct mantissa_ptr_struct {
    pub alloc: mp_size_t,
    pub d: mp_ptr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mantissa_struct {
    pub noptr: mantissa_noptr_struct,
    pub ptr: mantissa_ptr_struct,
}

#[repr(C)]
#[derive(Copy, Clone, Derivative)]
#[derivative(Debug)]
pub struct arf_struct {
    pub exp: fmpz,
    pub size: mp_size_t,
    #[derivative(Debug="ignore")]
    pub d: mantissa_struct,
}

pub type arf_t = [arf_struct; 1usize];
pub type arf_ptr = *mut arf_struct;
pub type arf_srcptr = *const arf_struct;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arf_interval_struct {
    pub a: arf_struct,
    pub b: arf_struct,
}
pub type arf_interval_t = [arf_interval_struct; 1usize];
pub type arf_interval_ptr = *mut arf_interval_struct;
pub type arf_interval_srcptr = *const arf_interval_struct;
