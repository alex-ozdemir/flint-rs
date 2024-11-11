#![allow(non_camel_case_types)]

//! *See the [Arb documentation](https://arblib.org/).

use crate::deps::*;
use crate::acb::{acb_ptr, acb_srcptr, acb_struct};
use crate::acb_poly::acb_poly_struct;
use crate::mag::mag_struct;

extern "C" {
    pub fn acb_hypgeom_rising_ui_rs(
        res: *mut acb_struct,
        x: *mut acb_struct,
        n: mp_limb_t,
        m: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_pfq_bound_factor(
        C: *mut mag_struct,
        a: acb_srcptr,
        p: mp_limb_signed_t,
        b: acb_srcptr,
        q: mp_limb_signed_t,
        z: *mut acb_struct,
        n: mp_limb_t,
    );
    pub fn acb_hypgeom_pfq_choose_n(
        a: acb_srcptr,
        p: mp_limb_signed_t,
        b: acb_srcptr,
        q: mp_limb_signed_t,
        z: *mut acb_struct,
        prec: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn acb_hypgeom_pfq_sum_forward(
        s: *mut acb_struct,
        t: *mut acb_struct,
        a: acb_srcptr,
        p: mp_limb_signed_t,
        b: acb_srcptr,
        q: mp_limb_signed_t,
        z: *mut acb_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_pfq_sum_rs(
        s: *mut acb_struct,
        t: *mut acb_struct,
        a: acb_srcptr,
        p: mp_limb_signed_t,
        b: acb_srcptr,
        q: mp_limb_signed_t,
        z: *mut acb_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_pfq_sum_bs(
        s: *mut acb_struct,
        t: *mut acb_struct,
        a: acb_srcptr,
        p: mp_limb_signed_t,
        b: acb_srcptr,
        q: mp_limb_signed_t,
        z: *mut acb_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_pfq_sum_fme(
        s: *mut acb_struct,
        t: *mut acb_struct,
        a: acb_srcptr,
        p: mp_limb_signed_t,
        b: acb_srcptr,
        q: mp_limb_signed_t,
        z: *mut acb_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_pfq_sum(
        s: *mut acb_struct,
        t: *mut acb_struct,
        a: acb_srcptr,
        p: mp_limb_signed_t,
        b: acb_srcptr,
        q: mp_limb_signed_t,
        z: *mut acb_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_pfq_sum_bs_invz(
        s: *mut acb_struct,
        t: *mut acb_struct,
        a: acb_srcptr,
        p: mp_limb_signed_t,
        b: acb_srcptr,
        q: mp_limb_signed_t,
        z: *mut acb_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_pfq_sum_invz(
        s: *mut acb_struct,
        t: *mut acb_struct,
        a: acb_srcptr,
        p: mp_limb_signed_t,
        b: acb_srcptr,
        q: mp_limb_signed_t,
        z: *mut acb_struct,
        zinv: *mut acb_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_pfq_direct(
        res: *mut acb_struct,
        a: acb_srcptr,
        p: mp_limb_signed_t,
        b: acb_srcptr,
        q: mp_limb_signed_t,
        z: *mut acb_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_pfq_series_choose_n(
        a: *const acb_poly_struct,
        p: mp_limb_signed_t,
        b: *const acb_poly_struct,
        q: mp_limb_signed_t,
        z: *mut acb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    ) -> mp_limb_signed_t;
    pub fn acb_hypgeom_pfq_series_sum_forward(
        s: *mut acb_poly_struct,
        t: *mut acb_poly_struct,
        a: *const acb_poly_struct,
        p: mp_limb_signed_t,
        b: *const acb_poly_struct,
        q: mp_limb_signed_t,
        z: *mut acb_poly_struct,
        regularized: ::std::os::raw::c_int,
        n: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_pfq_series_sum_bs(
        s: *mut acb_poly_struct,
        t: *mut acb_poly_struct,
        a: *const acb_poly_struct,
        p: mp_limb_signed_t,
        b: *const acb_poly_struct,
        q: mp_limb_signed_t,
        z: *mut acb_poly_struct,
        regularized: ::std::os::raw::c_int,
        n: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_pfq_series_sum_rs(
        s: *mut acb_poly_struct,
        t: *mut acb_poly_struct,
        a: *const acb_poly_struct,
        p: mp_limb_signed_t,
        b: *const acb_poly_struct,
        q: mp_limb_signed_t,
        z: *mut acb_poly_struct,
        regularized: ::std::os::raw::c_int,
        n: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_pfq_series_sum(
        s: *mut acb_poly_struct,
        t: *mut acb_poly_struct,
        a: *const acb_poly_struct,
        p: mp_limb_signed_t,
        b: *const acb_poly_struct,
        q: mp_limb_signed_t,
        z: *mut acb_poly_struct,
        regularized: ::std::os::raw::c_int,
        n: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_pfq_series_direct(
        res: *mut acb_poly_struct,
        a: *const acb_poly_struct,
        p: mp_limb_signed_t,
        b: *const acb_poly_struct,
        q: mp_limb_signed_t,
        z: *mut acb_poly_struct,
        regularized: ::std::os::raw::c_int,
        n: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_pfq(
        res: *mut acb_struct,
        a: acb_srcptr,
        p: mp_limb_signed_t,
        b: acb_srcptr,
        q: mp_limb_signed_t,
        z: *mut acb_struct,
        regularized: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_u_asymp(
        res: *mut acb_struct,
        a: *mut acb_struct,
        b: *mut acb_struct,
        z: *mut acb_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_u_1f1_series(
        res: *mut acb_poly_struct,
        a: *mut acb_poly_struct,
        b: *mut acb_poly_struct,
        z: *mut acb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_u_1f1(
        res: *mut acb_struct,
        a: *mut acb_struct,
        b: *mut acb_struct,
        z: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_u(
        res: *mut acb_struct,
        a: *mut acb_struct,
        b: *mut acb_struct,
        z: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_u_use_asymp(
        z: *mut acb_struct,
        prec: mp_limb_signed_t,
    ) -> ::std::os::raw::c_int;
    pub fn acb_hypgeom_m_asymp(
        res: *mut acb_struct,
        a: *mut acb_struct,
        b: *mut acb_struct,
        z: *mut acb_struct,
        regularized: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_m_1f1(
        res: *mut acb_struct,
        a: *mut acb_struct,
        b: *mut acb_struct,
        z: *mut acb_struct,
        regularized: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_m(
        res: *mut acb_struct,
        a: *mut acb_struct,
        b: *mut acb_struct,
        z: *mut acb_struct,
        regularized: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_1f1(
        res: *mut acb_struct,
        a: *mut acb_struct,
        b: *mut acb_struct,
        z: *mut acb_struct,
        regularized: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_bessel_j_0f1(
        res: *mut acb_struct,
        nu: *mut acb_struct,
        z: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_bessel_j_asymp(
        res: *mut acb_struct,
        nu: *mut acb_struct,
        z: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_bessel_j(
        res: *mut acb_struct,
        nu: *mut acb_struct,
        z: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_bessel_i_0f1(
        res: *mut acb_struct,
        nu: *mut acb_struct,
        z: *mut acb_struct,
        scaled: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_bessel_i_asymp(
        res: *mut acb_struct,
        nu: *mut acb_struct,
        z: *mut acb_struct,
        scaled: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_bessel_i(
        res: *mut acb_struct,
        nu: *mut acb_struct,
        z: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_bessel_i_scaled(
        res: *mut acb_struct,
        nu: *mut acb_struct,
        z: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_bessel_k_0f1(
        res: *mut acb_struct,
        nu: *mut acb_struct,
        z: *mut acb_struct,
        scaled: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_bessel_k_0f1_series(
        res: *mut acb_poly_struct,
        n: *mut acb_poly_struct,
        z: *mut acb_poly_struct,
        scaled: ::std::os::raw::c_int,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_bessel_k_asymp(
        res: *mut acb_struct,
        nu: *mut acb_struct,
        z: *mut acb_struct,
        scaled: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_bessel_k(
        res: *mut acb_struct,
        nu: *mut acb_struct,
        z: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_bessel_k_scaled(
        res: *mut acb_struct,
        nu: *mut acb_struct,
        z: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_bessel_y(
        res: *mut acb_struct,
        nu: *mut acb_struct,
        z: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_bessel_jy(
        res1: *mut acb_struct,
        res2: *mut acb_struct,
        nu: *mut acb_struct,
        z: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_0f1_asymp(
        res: *mut acb_struct,
        a: *mut acb_struct,
        z: *mut acb_struct,
        regularized: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_0f1_direct(
        res: *mut acb_struct,
        a: *mut acb_struct,
        z: *mut acb_struct,
        regularized: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_0f1(
        res: *mut acb_struct,
        a: *mut acb_struct,
        z: *mut acb_struct,
        regularized: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_airy_bound(
        ai: *mut mag_struct,
        aip: *mut mag_struct,
        bi: *mut mag_struct,
        bip: *mut mag_struct,
        z: *mut acb_struct,
    );
    pub fn acb_hypgeom_airy_asymp(
        ai: *mut acb_struct,
        aip: *mut acb_struct,
        bi: *mut acb_struct,
        bip: *mut acb_struct,
        z: *mut acb_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_airy_direct(
        ai: *mut acb_struct,
        aip: *mut acb_struct,
        bi: *mut acb_struct,
        bip: *mut acb_struct,
        z: *mut acb_struct,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_airy(
        ai: *mut acb_struct,
        aip: *mut acb_struct,
        bi: *mut acb_struct,
        bip: *mut acb_struct,
        z: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_airy_jet(
        ai: acb_ptr,
        bi: acb_ptr,
        z: *mut acb_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_hypgeom_airy_series(
        ai: acb_ptr,
        ai_prime: acb_ptr,
        bi: acb_ptr,
        bi_prime: acb_ptr,
        z: acb_srcptr,
        zlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_airy_series(
        ai: *mut acb_poly_struct,
        ai_prime: *mut acb_poly_struct,
        bi: *mut acb_poly_struct,
        bi_prime: *mut acb_poly_struct,
        z: *mut acb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_coulomb(
        F: *mut acb_struct,
        G: *mut acb_struct,
        Hpos: *mut acb_struct,
        Hneg: *mut acb_struct,
        l: *mut acb_struct,
        eta: *mut acb_struct,
        z: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_coulomb_jet(
        F: acb_ptr,
        G: acb_ptr,
        Hpos: acb_ptr,
        Hneg: acb_ptr,
        l: *mut acb_struct,
        eta: *mut acb_struct,
        z: *mut acb_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_hypgeom_coulomb_series(
        F: acb_ptr,
        G: acb_ptr,
        Hpos: acb_ptr,
        Hneg: acb_ptr,
        l: *mut acb_struct,
        eta: *mut acb_struct,
        z: acb_srcptr,
        zlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_coulomb_series(
        F: *mut acb_poly_struct,
        G: *mut acb_poly_struct,
        Hpos: *mut acb_poly_struct,
        Hneg: *mut acb_poly_struct,
        l: *mut acb_struct,
        eta: *mut acb_struct,
        z: *mut acb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_gamma_upper_asymp(
        res: *mut acb_struct,
        s: *mut acb_struct,
        z: *mut acb_struct,
        modified: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_gamma_upper_1f1a(
        res: *mut acb_struct,
        s: *mut acb_struct,
        z: *mut acb_struct,
        modified: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_gamma_upper_1f1b(
        res: *mut acb_struct,
        s: *mut acb_struct,
        z: *mut acb_struct,
        modified: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_gamma_upper_singular(
        res: *mut acb_struct,
        s: mp_limb_signed_t,
        z: *mut acb_struct,
        modified: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_gamma_upper(
        res: *mut acb_struct,
        s: *mut acb_struct,
        z: *mut acb_struct,
        modified: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_hypgeom_gamma_upper_series(
        g: acb_ptr,
        s: *mut acb_struct,
        h: acb_srcptr,
        hlen: mp_limb_signed_t,
        regularized: ::std::os::raw::c_int,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_gamma_upper_series(
        g: *mut acb_poly_struct,
        s: *mut acb_struct,
        h: *mut acb_poly_struct,
        regularized: ::std::os::raw::c_int,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_gamma_lower(
        res: *mut acb_struct,
        s: *mut acb_struct,
        z: *mut acb_struct,
        modified: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_hypgeom_gamma_lower_series(
        g: acb_ptr,
        s: *mut acb_struct,
        h: acb_srcptr,
        hlen: mp_limb_signed_t,
        regularized: ::std::os::raw::c_int,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_gamma_lower_series(
        g: *mut acb_poly_struct,
        s: *mut acb_struct,
        h: *mut acb_poly_struct,
        regularized: ::std::os::raw::c_int,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_beta_lower(
        res: *mut acb_struct,
        a: *mut acb_struct,
        b: *mut acb_struct,
        z: *mut acb_struct,
        regularized: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_hypgeom_beta_lower_series(
        res: acb_ptr,
        a: *mut acb_struct,
        b: *mut acb_struct,
        z: acb_srcptr,
        zlen: mp_limb_signed_t,
        regularized: ::std::os::raw::c_int,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_beta_lower_series(
        res: *mut acb_poly_struct,
        a: *mut acb_struct,
        b: *mut acb_struct,
        z: *mut acb_poly_struct,
        regularized: ::std::os::raw::c_int,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_expint(
        res: *mut acb_struct,
        s: *mut acb_struct,
        z: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_erf_propagated_error(
        re: *mut mag_struct,
        im: *mut mag_struct,
        z: *mut acb_struct,
    );
    pub fn acb_hypgeom_erf_1f1a(res: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_hypgeom_erf_1f1b(res: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_hypgeom_erf_asymp(
        res: *mut acb_struct,
        z: *mut acb_struct,
        complementary: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
        prec2: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_erf(res: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn _acb_hypgeom_erf_series(
        g: acb_ptr,
        h: acb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_erf_series(
        g: *mut acb_poly_struct,
        h: *mut acb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_erfc(res: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn _acb_hypgeom_erfc_series(
        g: acb_ptr,
        h: acb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_erfc_series(
        g: *mut acb_poly_struct,
        h: *mut acb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_erfi(res: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn _acb_hypgeom_erfi_series(
        g: acb_ptr,
        h: acb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_erfi_series(
        g: *mut acb_poly_struct,
        h: *mut acb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_fresnel(
        res1: *mut acb_struct,
        res2: *mut acb_struct,
        z: *mut acb_struct,
        normalized: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_hypgeom_fresnel_series(
        s: acb_ptr,
        c: acb_ptr,
        h: acb_srcptr,
        hlen: mp_limb_signed_t,
        normalized: ::std::os::raw::c_int,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_fresnel_series(
        s: *mut acb_poly_struct,
        c: *mut acb_poly_struct,
        h: *mut acb_poly_struct,
        normalized: ::std::os::raw::c_int,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_ei_asymp(res: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_hypgeom_ei_2f2(res: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_hypgeom_ei(res: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn _acb_hypgeom_ei_series(
        g: acb_ptr,
        h: acb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_ei_series(
        g: *mut acb_poly_struct,
        h: *mut acb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_si_asymp(res: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_hypgeom_si_1f2(res: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_hypgeom_si(res: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn _acb_hypgeom_si_series(
        g: acb_ptr,
        h: acb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_si_series(
        g: *mut acb_poly_struct,
        h: *mut acb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_ci_asymp(res: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_hypgeom_ci_2f3(res: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_hypgeom_ci(res: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn _acb_hypgeom_ci_series(
        g: acb_ptr,
        h: acb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_ci_series(
        g: *mut acb_poly_struct,
        h: *mut acb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_shi(res: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn _acb_hypgeom_shi_series(
        g: acb_ptr,
        h: acb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_shi_series(
        g: *mut acb_poly_struct,
        h: *mut acb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_chi_asymp(res: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_hypgeom_chi_2f3(res: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_hypgeom_chi(res: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn _acb_hypgeom_chi_series(
        g: acb_ptr,
        h: acb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_chi_series(
        g: *mut acb_poly_struct,
        h: *mut acb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_li(
        res: *mut acb_struct,
        z: *mut acb_struct,
        offset: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn _acb_hypgeom_li_series(
        g: acb_ptr,
        h: acb_srcptr,
        hlen: mp_limb_signed_t,
        offset: ::std::os::raw::c_int,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_li_series(
        g: *mut acb_poly_struct,
        h: *mut acb_poly_struct,
        offset: ::std::os::raw::c_int,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_2f1_continuation(
        res0: *mut acb_struct,
        res1: *mut acb_struct,
        a: *mut acb_struct,
        b: *mut acb_struct,
        c: *mut acb_struct,
        z0: *mut acb_struct,
        z1: *mut acb_struct,
        f0: *mut acb_struct,
        f1: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_2f1_series_direct(
        res: *mut acb_poly_struct,
        a: *mut acb_poly_struct,
        b: *mut acb_poly_struct,
        c: *mut acb_poly_struct,
        z: *mut acb_poly_struct,
        regularized: ::std::os::raw::c_int,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_2f1_direct(
        res: *mut acb_struct,
        a: *mut acb_struct,
        b: *mut acb_struct,
        c: *mut acb_struct,
        z: *mut acb_struct,
        regularized: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_2f1_transform(
        res: *mut acb_struct,
        a: *mut acb_struct,
        b: *mut acb_struct,
        c: *mut acb_struct,
        z: *mut acb_struct,
        regularized: ::std::os::raw::c_int,
        which: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_2f1_transform_limit(
        res: *mut acb_struct,
        a: *mut acb_struct,
        b: *mut acb_struct,
        c: *mut acb_struct,
        z: *mut acb_struct,
        regularized: ::std::os::raw::c_int,
        which: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_2f1_corner(
        res: *mut acb_struct,
        a: *mut acb_struct,
        b: *mut acb_struct,
        c: *mut acb_struct,
        z: *mut acb_struct,
        regularized: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_2f1_choose(z: *mut acb_struct) -> ::std::os::raw::c_int;
    pub fn acb_hypgeom_2f1(
        res: *mut acb_struct,
        a: *mut acb_struct,
        b: *mut acb_struct,
        c: *mut acb_struct,
        z: *mut acb_struct,
        regularized: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_legendre_p_uiui_rec(
        res: *mut acb_struct,
        n: mp_limb_t,
        m: mp_limb_t,
        z: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_legendre_p(
        res: *mut acb_struct,
        n: *mut acb_struct,
        m: *mut acb_struct,
        z: *mut acb_struct,
        type_: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_legendre_q(
        res: *mut acb_struct,
        n: *mut acb_struct,
        m: *mut acb_struct,
        z: *mut acb_struct,
        type_: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_jacobi_p(
        res: *mut acb_struct,
        n: *mut acb_struct,
        a: *mut acb_struct,
        b: *mut acb_struct,
        z: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_gegenbauer_c(
        res: *mut acb_struct,
        n: *mut acb_struct,
        m: *mut acb_struct,
        z: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_laguerre_l(
        res: *mut acb_struct,
        n: *mut acb_struct,
        m: *mut acb_struct,
        z: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_hermite_h(
        res: *mut acb_struct,
        n: *mut acb_struct,
        z: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_chebyshev_t(
        res: *mut acb_struct,
        n: *mut acb_struct,
        z: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_chebyshev_u(
        res: *mut acb_struct,
        n: *mut acb_struct,
        z: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_spherical_y(
        res: *mut acb_struct,
        n: mp_limb_signed_t,
        m: mp_limb_signed_t,
        theta: *mut acb_struct,
        phi: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_dilog_bernoulli(
        res: *mut acb_struct,
        z: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_dilog_continuation(
        res: *mut acb_struct,
        a: *mut acb_struct,
        z: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_dilog_bitburst(
        res: *mut acb_struct,
        z0: *mut acb_struct,
        z: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_dilog_transform(
        res: *mut acb_struct,
        z: *mut acb_struct,
        algorithm: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_dilog_zero_taylor(
        res: *mut acb_struct,
        z: *mut acb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn acb_hypgeom_dilog_zero(res: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
    pub fn acb_hypgeom_dilog(res: *mut acb_struct, z: *mut acb_struct, prec: mp_limb_signed_t);
}
