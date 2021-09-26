#![allow(non_camel_case_types)]

//! *See the [FLINT documentation](http://flintlib.org/doc/fft.html).

use crate::deps::*;

extern "C" {
    pub fn fft_combine_limbs(
        res: *mut mp_limb_t,
        poly: *mut *mut mp_limb_t,
        length: mp_limb_signed_t,
        coeff_limbs: mp_size_t,
        output_limbs: mp_size_t,
        total_limbs: mp_size_t,
    );
    pub fn fft_combine_bits(
        res: *mut mp_limb_t,
        poly: *mut *mut mp_limb_t,
        length: mp_limb_signed_t,
        bits: mp_limb_t,
        output_limbs: mp_size_t,
        total_limbs: mp_size_t,
    );
    pub fn fft_split_limbs(
        poly: *mut *mut mp_limb_t,
        limbs: mp_srcptr,
        total_limbs: mp_size_t,
        coeff_limbs: mp_size_t,
        output_limbs: mp_size_t,
    ) -> mp_size_t;
    pub fn fft_split_bits(
        poly: *mut *mut mp_limb_t,
        limbs: mp_srcptr,
        total_limbs: mp_size_t,
        bits: mp_limb_t,
        output_limbs: mp_size_t,
    ) -> mp_size_t;
    pub fn fermat_to_mpz(m: *mut __mpz_struct, i: *mut mp_limb_t, limbs: mp_size_t);
    pub fn mpn_normmod_2expp1(t: *mut mp_limb_t, limbs: mp_size_t);
    pub fn butterfly_lshB(
        t: *mut mp_limb_t,
        u: *mut mp_limb_t,
        i1: *mut mp_limb_t,
        i2: *mut mp_limb_t,
        limbs: mp_size_t,
        x: mp_size_t,
        y: mp_size_t,
    );
    pub fn butterfly_rshB(
        t: *mut mp_limb_t,
        u: *mut mp_limb_t,
        i1: *mut mp_limb_t,
        i2: *mut mp_limb_t,
        limbs: mp_size_t,
        x: mp_size_t,
        y: mp_size_t,
    );
    pub fn mpn_mul_2expmod_2expp1(
        t: *mut mp_limb_t,
        i1: *mut mp_limb_t,
        limbs: mp_size_t,
        d: mp_limb_t,
    );
    pub fn mpn_div_2expmod_2expp1(
        t: *mut mp_limb_t,
        i1: *mut mp_limb_t,
        limbs: mp_size_t,
        d: mp_limb_t,
    );
    pub fn fft_adjust(
        r: *mut mp_limb_t,
        i1: *mut mp_limb_t,
        i: mp_size_t,
        limbs: mp_size_t,
        w: mp_limb_t,
    );
    pub fn fft_butterfly(
        s: *mut mp_limb_t,
        t: *mut mp_limb_t,
        i1: *mut mp_limb_t,
        i2: *mut mp_limb_t,
        i: mp_size_t,
        limbs: mp_size_t,
        w: mp_limb_t,
    );
    pub fn ifft_butterfly(
        s: *mut mp_limb_t,
        t: *mut mp_limb_t,
        i1: *mut mp_limb_t,
        i2: *mut mp_limb_t,
        i: mp_size_t,
        limbs: mp_size_t,
        w: mp_limb_t,
    );
    pub fn fft_radix2(
        ii: *mut *mut mp_limb_t,
        n: mp_size_t,
        w: mp_limb_t,
        t1: *mut *mut mp_limb_t,
        t2: *mut *mut mp_limb_t,
    );
    pub fn fft_truncate1(
        ii: *mut *mut mp_limb_t,
        n: mp_size_t,
        w: mp_limb_t,
        t1: *mut *mut mp_limb_t,
        t2: *mut *mut mp_limb_t,
        trunc: mp_size_t,
    );
    pub fn fft_truncate(
        ii: *mut *mut mp_limb_t,
        n: mp_size_t,
        w: mp_limb_t,
        t1: *mut *mut mp_limb_t,
        t2: *mut *mut mp_limb_t,
        trunc: mp_size_t,
    );
    pub fn ifft_radix2(
        ii: *mut *mut mp_limb_t,
        n: mp_size_t,
        w: mp_limb_t,
        t1: *mut *mut mp_limb_t,
        t2: *mut *mut mp_limb_t,
    );
    pub fn ifft_truncate1(
        ii: *mut *mut mp_limb_t,
        n: mp_size_t,
        w: mp_limb_t,
        t1: *mut *mut mp_limb_t,
        t2: *mut *mut mp_limb_t,
        trunc: mp_size_t,
    );
    pub fn ifft_truncate(
        ii: *mut *mut mp_limb_t,
        n: mp_size_t,
        w: mp_limb_t,
        t1: *mut *mut mp_limb_t,
        t2: *mut *mut mp_limb_t,
        trunc: mp_size_t,
    );
    pub fn fft_butterfly_sqrt2(
        s: *mut mp_limb_t,
        t: *mut mp_limb_t,
        i1: *mut mp_limb_t,
        i2: *mut mp_limb_t,
        i: mp_size_t,
        limbs: mp_size_t,
        w: mp_limb_t,
        temp: *mut mp_limb_t,
    );
    pub fn ifft_butterfly_sqrt2(
        s: *mut mp_limb_t,
        t: *mut mp_limb_t,
        i1: *mut mp_limb_t,
        i2: *mut mp_limb_t,
        i: mp_size_t,
        limbs: mp_size_t,
        w: mp_limb_t,
        temp: *mut mp_limb_t,
    );
    pub fn fft_adjust_sqrt2(
        r: *mut mp_limb_t,
        i1: *mut mp_limb_t,
        i: mp_size_t,
        limbs: mp_size_t,
        w: mp_limb_t,
        temp: *mut mp_limb_t,
    );
    pub fn fft_truncate_sqrt2(
        ii: *mut *mut mp_limb_t,
        n: mp_size_t,
        w: mp_limb_t,
        t1: *mut *mut mp_limb_t,
        t2: *mut *mut mp_limb_t,
        temp: *mut *mut mp_limb_t,
        trunc: mp_size_t,
    );
    pub fn ifft_truncate_sqrt2(
        ii: *mut *mut mp_limb_t,
        n: mp_size_t,
        w: mp_limb_t,
        t1: *mut *mut mp_limb_t,
        t2: *mut *mut mp_limb_t,
        temp: *mut *mut mp_limb_t,
        trunc: mp_size_t,
    );
    pub fn mul_truncate_sqrt2(
        r1: mp_ptr,
        i1: mp_srcptr,
        n1: mp_size_t,
        i2: mp_srcptr,
        n2: mp_size_t,
        depth: mp_limb_t,
        w: mp_limb_t,
    );
    pub fn fft_butterfly_twiddle(
        u: *mut mp_limb_t,
        v: *mut mp_limb_t,
        s: *mut mp_limb_t,
        t: *mut mp_limb_t,
        limbs: mp_size_t,
        b1: mp_limb_t,
        b2: mp_limb_t,
    );
    pub fn ifft_butterfly_twiddle(
        u: *mut mp_limb_t,
        v: *mut mp_limb_t,
        s: *mut mp_limb_t,
        t: *mut mp_limb_t,
        limbs: mp_size_t,
        b1: mp_limb_t,
        b2: mp_limb_t,
    );
    pub fn fft_radix2_twiddle(
        ii: *mut *mut mp_limb_t,
        is: mp_size_t,
        n: mp_size_t,
        w: mp_limb_t,
        t1: *mut *mut mp_limb_t,
        t2: *mut *mut mp_limb_t,
        ws: mp_size_t,
        r: mp_size_t,
        c: mp_size_t,
        rs: mp_size_t,
    );
    pub fn ifft_radix2_twiddle(
        ii: *mut *mut mp_limb_t,
        is: mp_size_t,
        n: mp_size_t,
        w: mp_limb_t,
        t1: *mut *mut mp_limb_t,
        t2: *mut *mut mp_limb_t,
        ws: mp_size_t,
        r: mp_size_t,
        c: mp_size_t,
        rs: mp_size_t,
    );
    pub fn fft_truncate1_twiddle(
        ii: *mut *mut mp_limb_t,
        is: mp_size_t,
        n: mp_size_t,
        w: mp_limb_t,
        t1: *mut *mut mp_limb_t,
        t2: *mut *mut mp_limb_t,
        ws: mp_size_t,
        r: mp_size_t,
        c: mp_size_t,
        rs: mp_size_t,
        trunc: mp_size_t,
    );
    pub fn ifft_truncate1_twiddle(
        ii: *mut *mut mp_limb_t,
        is: mp_size_t,
        n: mp_size_t,
        w: mp_limb_t,
        t1: *mut *mut mp_limb_t,
        t2: *mut *mut mp_limb_t,
        ws: mp_size_t,
        r: mp_size_t,
        c: mp_size_t,
        rs: mp_size_t,
        trunc: mp_size_t,
    );
    pub fn fft_mfa_truncate_sqrt2(
        ii: *mut *mut mp_limb_t,
        n: mp_size_t,
        w: mp_limb_t,
        t1: *mut *mut mp_limb_t,
        t2: *mut *mut mp_limb_t,
        temp: *mut *mut mp_limb_t,
        n1: mp_size_t,
        trunc: mp_size_t,
    );
    pub fn ifft_mfa_truncate_sqrt2(
        ii: *mut *mut mp_limb_t,
        n: mp_size_t,
        w: mp_limb_t,
        t1: *mut *mut mp_limb_t,
        t2: *mut *mut mp_limb_t,
        temp: *mut *mut mp_limb_t,
        n1: mp_size_t,
        trunc: mp_size_t,
    );
    pub fn mul_mfa_truncate_sqrt2(
        r1: mp_ptr,
        i1: mp_srcptr,
        n1: mp_size_t,
        i2: mp_srcptr,
        n2: mp_size_t,
        depth: mp_limb_t,
        w: mp_limb_t,
    );
    pub fn fft_mfa_truncate_sqrt2_outer(
        ii: *mut *mut mp_limb_t,
        n: mp_size_t,
        w: mp_limb_t,
        t1: *mut *mut mp_limb_t,
        t2: *mut *mut mp_limb_t,
        temp: *mut *mut mp_limb_t,
        n1: mp_size_t,
        trunc: mp_size_t,
    );
    pub fn fft_mfa_truncate_sqrt2_inner(
        ii: *mut *mut mp_limb_t,
        jj: *mut *mut mp_limb_t,
        n: mp_size_t,
        w: mp_limb_t,
        t1: *mut *mut mp_limb_t,
        t2: *mut *mut mp_limb_t,
        temp: *mut *mut mp_limb_t,
        n1: mp_size_t,
        trunc: mp_size_t,
        tt: *mut *mut mp_limb_t,
    );
    pub fn ifft_mfa_truncate_sqrt2_outer(
        ii: *mut *mut mp_limb_t,
        n: mp_size_t,
        w: mp_limb_t,
        t1: *mut *mut mp_limb_t,
        t2: *mut *mut mp_limb_t,
        temp: *mut *mut mp_limb_t,
        n1: mp_size_t,
        trunc: mp_size_t,
    );
    pub fn fft_negacyclic(
        ii: *mut *mut mp_limb_t,
        n: mp_size_t,
        w: mp_limb_t,
        t1: *mut *mut mp_limb_t,
        t2: *mut *mut mp_limb_t,
        temp: *mut *mut mp_limb_t,
    );
    pub fn ifft_negacyclic(
        ii: *mut *mut mp_limb_t,
        n: mp_size_t,
        w: mp_limb_t,
        t1: *mut *mut mp_limb_t,
        t2: *mut *mut mp_limb_t,
        temp: *mut *mut mp_limb_t,
    );
    pub fn fft_naive_convolution_1(
        r: *mut mp_limb_t,
        ii: *mut mp_limb_t,
        jj: *mut mp_limb_t,
        m: mp_size_t,
    );
    pub fn _fft_mulmod_2expp1(
        r1: *mut mp_limb_t,
        i1: *mut mp_limb_t,
        i2: *mut mp_limb_t,
        r_limbs: mp_size_t,
        depth: mp_limb_t,
        w: mp_limb_t,
    );
    pub fn fft_adjust_limbs(limbs: mp_size_t) -> mp_limb_signed_t;
    pub fn fft_mulmod_2expp1(
        r: *mut mp_limb_t,
        i1: *mut mp_limb_t,
        i2: *mut mp_limb_t,
        n: mp_size_t,
        w: mp_size_t,
        tt: *mut mp_limb_t,
    );
    pub fn flint_mpn_mul_fft_main(
        r1: mp_ptr,
        i1: mp_srcptr,
        n1: mp_size_t,
        i2: mp_srcptr,
        n2: mp_size_t,
    );
    pub fn fft_convolution_basic(
        ii: *mut *mut mp_limb_t,
        jj: *mut *mut mp_limb_t,
        depth: mp_limb_signed_t,
        limbs: mp_limb_signed_t,
        trunc: mp_limb_signed_t,
        t1: *mut *mut mp_limb_t,
        t2: *mut *mut mp_limb_t,
        s1: *mut *mut mp_limb_t,
        tt: *mut *mut mp_limb_t,
    );
    pub fn fft_convolution(
        ii: *mut *mut mp_limb_t,
        jj: *mut *mut mp_limb_t,
        depth: mp_limb_signed_t,
        limbs: mp_limb_signed_t,
        trunc: mp_limb_signed_t,
        t1: *mut *mut mp_limb_t,
        t2: *mut *mut mp_limb_t,
        s1: *mut *mut mp_limb_t,
        tt: *mut *mut mp_limb_t,
    );
    pub fn fft_precache(
        jj: *mut *mut mp_limb_t,
        depth: mp_limb_signed_t,
        limbs: mp_limb_signed_t,
        trunc: mp_limb_signed_t,
        t1: *mut *mut mp_limb_t,
        t2: *mut *mut mp_limb_t,
        s1: *mut *mut mp_limb_t,
    );
    pub fn fft_convolution_precache(
        ii: *mut *mut mp_limb_t,
        jj: *mut *mut mp_limb_t,
        depth: mp_limb_signed_t,
        limbs: mp_limb_signed_t,
        trunc: mp_limb_signed_t,
        t1: *mut *mut mp_limb_t,
        t2: *mut *mut mp_limb_t,
        s1: *mut *mut mp_limb_t,
        tt: *mut *mut mp_limb_t,
    );
}
