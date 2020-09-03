use flint_sys::{self, fmpz};

use std::default::Default;
use std::mem::MaybeUninit;
use std::convert::From;

struct Integer {
    raw: fmpz,
}

impl Default for Integer {
    fn default() -> Self {
        Self::new()
    }
}

impl Integer {
    fn new() -> Self {
        unsafe {
            let mut raw = MaybeUninit::uninit();
            flint_sys::fmpz_init(raw.as_mut_ptr());
            Integer { raw: raw.assume_init() }
        }
    }
    fn set_ui(&mut self, n: flint_sys::ulong) {
        unsafe {
            flint_sys::fmpz_set_ui(&mut self.raw, n)
        }
    }
    fn set_si(&mut self, i: flint_sys::slong) {
        unsafe {
            flint_sys::fmpz_set_si(&mut self.raw, i)
        }
    }
}

macro_rules! impl_int_from {
    ($small:ty, $set_fn:ident, $intermediate:ty) => {
        impl From<$small> for Integer {
            fn from(small: $small) -> Self {
                let mut i = Integer::new();
                Integer::$set_fn(&mut i, small as $intermediate);
                i
            }
        }
        impl From<&$small> for Integer {
            fn from(small: &$small) -> Self {
                Self::from(*small)
            }
        }
    }
}

impl_int_from!(u8, set_ui, flint_sys::ulong);
impl_int_from!(i8, set_si, flint_sys::slong);
impl_int_from!(u16, set_ui, flint_sys::ulong);
impl_int_from!(i16, set_si, flint_sys::slong);
impl_int_from!(u32, set_ui, flint_sys::ulong);
impl_int_from!(i32, set_si, flint_sys::slong);
impl_int_from!(u64, set_ui, flint_sys::ulong);
impl_int_from!(i64, set_si, flint_sys::slong);
impl_int_from!(usize, set_ui, flint_sys::ulong);
impl_int_from!(isize, set_si, flint_sys::slong);
