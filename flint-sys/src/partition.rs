#![allow(non_camel_case_types)]

//! *See the [Arb documentation](https://arblib.org/).

use crate::arb::arb_struct;
use crate::arf::arf_struct;
use crate::deps::*;
use crate::fmpz::fmpz;

extern "C" {
    pub fn partitions_rademacher_bound(b: *mut arf_struct, n: *mut fmpz, N: mp_limb_t);
    pub fn partitions_hrr_sum_arb(
        x: *mut arb_struct,
        n: *mut fmpz,
        N0: mp_limb_signed_t,
        N: mp_limb_signed_t,
        use_doubles: ::std::os::raw::c_int,
    );
    pub fn partitions_fmpz_fmpz(p: *mut fmpz, n: *mut fmpz, use_doubles: ::std::os::raw::c_int);
    pub fn partitions_fmpz_ui(p: *mut fmpz, n: mp_limb_t);
    pub fn partitions_fmpz_ui_using_doubles(p: *mut fmpz, n: mp_limb_t);
    pub fn partitions_leading_fmpz(res: *mut arb_struct, n: *mut fmpz, prec: mp_limb_signed_t);
}
