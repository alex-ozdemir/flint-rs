#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// fmpz is a type, fmpz == mp_limb_signed_t == c_long == i32 or i64

pub struct my_fmpz {
    x: u32,
}

impl my_fmpz {
    pub fn new(modulus: u32) -> Self {
        unsafe { my_fmpz { x: modulus } }
    }
}
