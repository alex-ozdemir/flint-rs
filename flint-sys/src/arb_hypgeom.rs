#![allow(non_camel_case_types)]

//! *See the [Arb documentation](https://arblib.org/).

use crate::arb::{arb_ptr, arb_srcptr, arb_struct};
use crate::arb_poly::arb_poly_struct;
use crate::mag::mag_struct;
use crate::deps::*;
use crate::fmpz::fmpz;

extern "C" {
    pub fn _arb_hypgeom_rising_coeffs_1(c: *mut mp_limb_t, k: mp_limb_t, l: mp_limb_signed_t);
    pub fn _arb_hypgeom_rising_coeffs_2(c: *mut mp_limb_t, k: mp_limb_t, l: mp_limb_signed_t);
    pub fn _arb_hypgeom_rising_coeffs_fmpz(c: *mut fmpz, k: mp_limb_t, l: mp_limb_signed_t);
    pub fn arb_hypgeom_rising_ui_forward(
        res: *mut arb_struct,
        x: *mut arb_struct,
        n: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_rising_ui_rs(
        res: *mut arb_struct,
        x: *mut arb_struct,
        n: mp_limb_t,
        m: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_rising_ui_bs(
        res: *mut arb_struct,
        x: *mut arb_struct,
        n: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_rising_ui_rec(
        res: *mut arb_struct,
        x: *mut arb_struct,
        n: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_rising_ui(
        y: *mut arb_struct,
        x: *mut arb_struct,
        n: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_rising(
        y: *mut arb_struct,
        x: *mut arb_struct,
        n: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_rising_ui_jet_powsum(
        res: arb_ptr,
        x: *mut arb_struct,
        n: mp_limb_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_rising_ui_jet_rs(
        res: arb_ptr,
        x: *mut arb_struct,
        n: mp_limb_t,
        m: mp_limb_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_rising_ui_jet_bs(
        res: arb_ptr,
        x: *mut arb_struct,
        n: mp_limb_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_rising_ui_jet(
        res: arb_ptr,
        x: *mut arb_struct,
        n: mp_limb_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_pfq(
        res: *mut arb_struct,
        a: arb_srcptr,
        p: mp_limb_signed_t,
        b: arb_srcptr,
        q: mp_limb_signed_t,
        z: *mut arb_struct,
        regularized: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_0f1(
        res: *mut arb_struct,
        a: *mut arb_struct,
        z: *mut arb_struct,
        regularized: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_m(
        res: *mut arb_struct,
        a: *mut arb_struct,
        b: *mut arb_struct,
        z: *mut arb_struct,
        regularized: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_1f1(
        res: *mut arb_struct,
        a: *mut arb_struct,
        b: *mut arb_struct,
        z: *mut arb_struct,
        regularized: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_u(
        res: *mut arb_struct,
        a: *mut arb_struct,
        b: *mut arb_struct,
        z: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_2f1(
        res: *mut arb_struct,
        a: *mut arb_struct,
        b: *mut arb_struct,
        c: *mut arb_struct,
        z: *mut arb_struct,
        regularized: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_erf(res: *mut arb_struct, z: *mut arb_struct, prec: mp_limb_signed_t);
    pub fn _arb_hypgeom_erf_series(
        g: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_erf_series(
        g: *mut arb_poly_struct,
        h: *mut arb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_erfc(res: *mut arb_struct, z: *mut arb_struct, prec: mp_limb_signed_t);
    pub fn _arb_hypgeom_erfc_series(
        g: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_erfc_series(
        g: *mut arb_poly_struct,
        h: *mut arb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_erfi(res: *mut arb_struct, z: *mut arb_struct, prec: mp_limb_signed_t);
    pub fn _arb_hypgeom_erfi_series(
        g: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_erfi_series(
        g: *mut arb_poly_struct,
        h: *mut arb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_fresnel(
        res1: *mut arb_struct,
        res2: *mut arb_struct,
        z: *mut arb_struct,
        normalized: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_hypgeom_fresnel_series(
        s: arb_ptr,
        c: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        normalized: ::std::os::raw::c_int,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_fresnel_series(
        s: *mut arb_poly_struct,
        c: *mut arb_poly_struct,
        h: *mut arb_poly_struct,
        normalized: ::std::os::raw::c_int,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_ei(res: *mut arb_struct, z: *mut arb_struct, prec: mp_limb_signed_t);
    pub fn _arb_hypgeom_ei_series(
        g: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_ei_series(
        g: *mut arb_poly_struct,
        h: *mut arb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_si(res: *mut arb_struct, z: *mut arb_struct, prec: mp_limb_signed_t);
    pub fn _arb_hypgeom_si_series(
        g: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_si_series(
        g: *mut arb_poly_struct,
        h: *mut arb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_ci(res: *mut arb_struct, z: *mut arb_struct, prec: mp_limb_signed_t);
    pub fn _arb_hypgeom_ci_series(
        g: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_ci_series(
        g: *mut arb_poly_struct,
        h: *mut arb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_shi(res: *mut arb_struct, z: *mut arb_struct, prec: mp_limb_signed_t);
    pub fn _arb_hypgeom_shi_series(
        g: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_shi_series(
        g: *mut arb_poly_struct,
        h: *mut arb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_chi(res: *mut arb_struct, z: *mut arb_struct, prec: mp_limb_signed_t);
    pub fn _arb_hypgeom_chi_series(
        g: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_chi_series(
        g: *mut arb_poly_struct,
        h: *mut arb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_li(
        res: *mut arb_struct,
        z: *mut arb_struct,
        offset: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_hypgeom_li_series(
        g: arb_ptr,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        offset: ::std::os::raw::c_int,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_li_series(
        g: *mut arb_poly_struct,
        h: *mut arb_poly_struct,
        offset: ::std::os::raw::c_int,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_bessel_j(
        res: *mut arb_struct,
        nu: *mut arb_struct,
        z: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_bessel_y(
        res: *mut arb_struct,
        nu: *mut arb_struct,
        z: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_bessel_jy(
        res1: *mut arb_struct,
        res2: *mut arb_struct,
        nu: *mut arb_struct,
        z: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_bessel_i(
        res: *mut arb_struct,
        nu: *mut arb_struct,
        z: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_bessel_k(
        res: *mut arb_struct,
        nu: *mut arb_struct,
        z: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_bessel_i_scaled(
        res: *mut arb_struct,
        nu: *mut arb_struct,
        z: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_bessel_k_scaled(
        res: *mut arb_struct,
        nu: *mut arb_struct,
        z: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_airy(
        ai: *mut arb_struct,
        aip: *mut arb_struct,
        bi: *mut arb_struct,
        bip: *mut arb_struct,
        z: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_airy_jet(
        ai: arb_ptr,
        bi: arb_ptr,
        z: *mut arb_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_airy_series(
        ai: *mut arb_poly_struct,
        ai_prime: *mut arb_poly_struct,
        bi: *mut arb_poly_struct,
        bi_prime: *mut arb_poly_struct,
        z: *mut arb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_hypgeom_airy_series(
        ai: arb_ptr,
        ai_prime: arb_ptr,
        bi: arb_ptr,
        bi_prime: arb_ptr,
        z: arb_srcptr,
        zlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_airy_zero(
        ai: *mut arb_struct,
        aip: *mut arb_struct,
        bi: *mut arb_struct,
        bip: *mut arb_struct,
        n: *mut fmpz,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_coulomb(
        F: *mut arb_struct,
        G: *mut arb_struct,
        l: *mut arb_struct,
        eta: *mut arb_struct,
        z: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_coulomb_jet(
        F: arb_ptr,
        G: arb_ptr,
        l: *mut arb_struct,
        eta: *mut arb_struct,
        z: *mut arb_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_hypgeom_coulomb_series(
        F: arb_ptr,
        G: arb_ptr,
        l: *mut arb_struct,
        eta: *mut arb_struct,
        z: arb_srcptr,
        zlen: mp_limb_signed_t,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_coulomb_series(
        F: *mut arb_poly_struct,
        G: *mut arb_poly_struct,
        l: *mut arb_struct,
        eta: *mut arb_struct,
        z: *mut arb_poly_struct,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_expint(
        res: *mut arb_struct,
        s: *mut arb_struct,
        z: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_gamma_lower(
        res: *mut arb_struct,
        s: *mut arb_struct,
        z: *mut arb_struct,
        regularized: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_hypgeom_gamma_lower_series(
        g: arb_ptr,
        s: *mut arb_struct,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        regularized: ::std::os::raw::c_int,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_gamma_lower_series(
        g: *mut arb_poly_struct,
        s: *mut arb_struct,
        h: *mut arb_poly_struct,
        regularized: ::std::os::raw::c_int,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_gamma_upper(
        res: *mut arb_struct,
        s: *mut arb_struct,
        z: *mut arb_struct,
        regularized: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_hypgeom_gamma_upper_series(
        g: arb_ptr,
        s: *mut arb_struct,
        h: arb_srcptr,
        hlen: mp_limb_signed_t,
        regularized: ::std::os::raw::c_int,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_gamma_upper_series(
        g: *mut arb_poly_struct,
        s: *mut arb_struct,
        h: *mut arb_poly_struct,
        regularized: ::std::os::raw::c_int,
        n: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_beta_lower(
        res: *mut arb_struct,
        a: *mut arb_struct,
        c: *mut arb_struct,
        z: *mut arb_struct,
        regularized: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_beta_lower_series(
        res: *mut arb_poly_struct,
        a: *mut arb_struct,
        b: *mut arb_struct,
        z: *mut arb_poly_struct,
        regularized: ::std::os::raw::c_int,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn _arb_hypgeom_beta_lower_series(
        res: arb_ptr,
        a: *mut arb_struct,
        b: *mut arb_struct,
        z: arb_srcptr,
        zlen: mp_limb_signed_t,
        regularized: ::std::os::raw::c_int,
        len: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_chebyshev_t(
        res: *mut arb_struct,
        nu: *mut arb_struct,
        z: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_chebyshev_u(
        res: *mut arb_struct,
        nu: *mut arb_struct,
        z: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_jacobi_p(
        res: *mut arb_struct,
        n: *mut arb_struct,
        a: *mut arb_struct,
        b: *mut arb_struct,
        z: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_gegenbauer_c(
        res: *mut arb_struct,
        n: *mut arb_struct,
        m: *mut arb_struct,
        z: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_laguerre_l(
        res: *mut arb_struct,
        n: *mut arb_struct,
        m: *mut arb_struct,
        z: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_hermite_h(
        res: *mut arb_struct,
        nu: *mut arb_struct,
        z: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_legendre_p(
        res: *mut arb_struct,
        n: *mut arb_struct,
        m: *mut arb_struct,
        z: *mut arb_struct,
        type_: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_legendre_q(
        res: *mut arb_struct,
        n: *mut arb_struct,
        m: *mut arb_struct,
        z: *mut arb_struct,
        type_: ::std::os::raw::c_int,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_legendre_p_ui_deriv_bound(
        dp: *mut mag_struct,
        dp2: *mut mag_struct,
        n: mp_limb_t,
        x: *mut arb_struct,
        x2sub1: *mut arb_struct,
    );
    pub fn arb_hypgeom_legendre_p_ui_rec(
        res: *mut arb_struct,
        res_prime: *mut arb_struct,
        n: mp_limb_t,
        x: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_legendre_p_ui_asymp(
        res: *mut arb_struct,
        res2: *mut arb_struct,
        n: mp_limb_t,
        x: *mut arb_struct,
        K: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_legendre_p_ui_one(
        res: *mut arb_struct,
        res2: *mut arb_struct,
        n: mp_limb_t,
        x: *mut arb_struct,
        K: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_legendre_p_ui_zero(
        res: *mut arb_struct,
        res2: *mut arb_struct,
        n: mp_limb_t,
        x: *mut arb_struct,
        K: mp_limb_signed_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_legendre_p_ui(
        res: *mut arb_struct,
        res_prime: *mut arb_struct,
        n: mp_limb_t,
        x: *mut arb_struct,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_legendre_p_ui_root(
        res: *mut arb_struct,
        weight: *mut arb_struct,
        n: mp_limb_t,
        k: mp_limb_t,
        prec: mp_limb_signed_t,
    );
    pub fn arb_hypgeom_central_bin_ui(res: *mut arb_struct, n: mp_limb_t, prec: mp_limb_signed_t);
    pub fn arb_hypgeom_dilog(res: *mut arb_struct, z: *mut arb_struct, prec: mp_limb_signed_t);
}
