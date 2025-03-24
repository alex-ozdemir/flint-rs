#include "acb.h"
#include "acb_dft.h"
#include "acb_dirichlet.h"
#include "acb_mat.h"
#include "acb_modular.h"
#include "acb_poly.h"
#include "acb_theta.h"
#include "acf.h"
#include "arb.h"
#include "arb_calc.h"
#include "arb_mat.h"
#include "arb_poly.h"
#include "arf.h"
#include "arith.h"
#include "bernoulli.h"
#include "bool_mat.h"
#include "ca.h"
#include "ca_ext.h"
#include "ca_mat.h"
#include "ca_poly.h"
#include "ca_vec.h"
#include "calcium.h"
#include "d_mat.h"
#include "d_vec.h"
#include "dirichlet.h"
#include "dlog.h"
#include "double_extras.h"
#include "double_interval.h"
#include "fexpr.h"
#include "fexpr_builtin.h"
#include "fft.h"
#include "flint.h"
#include "fmpq.h"
#include "fmpq_mat.h"
#include "fmpq_mpoly.h"
#include "fmpq_mpoly_factor.h"
#include "fmpq_poly.h"
#include "fmpz.h"
#include "fmpz_extras.h"
#include "fmpz_mat.h"
#include "fmpz_mod.h"
#include "fmpz_mod_mat.h"
#include "fmpz_mod_mpoly.h"
#include "fmpz_mod_mpoly_factor.h"
#include "fmpz_mod_poly.h"
#include "fmpz_mod_poly_factor.h"
#include "fmpz_mpoly.h"
#include "fmpz_mpoly_factor.h"
#include "fmpz_mpoly_q.h"
#include "fmpz_poly.h"
#include "fmpz_poly_factor.h"
#include "fmpz_poly_mat.h"
#include "fmpz_poly_q.h"
#include "fmpz_vec.h"
#include "fmpzi.h"
#include "fq.h"
#include "fq_default.h"
#include "fq_default_mat.h"
#include "fq_default_poly.h"
#include "fq_default_poly_factor.h"
#include "fq_nmod.h"
#include "fq_nmod_mpoly.h"
#include "fq_nmod_mpoly_factor.h"
#include "fq_zech.h"
#include "fq_zech_mpoly.h"
#include "fq_zech_mpoly_factor.h"
#include "gmpcompat.h"
#include "gr.h"
#include "gr_mat.h"
#include "gr_mpoly.h"
#include "gr_poly.h"
#include "gr_special.h"
#include "gr_vec.h"
#include "long_extras.h"
#include "mag.h"
#include "mpf-impl.h"
#include "mpfr_mat.h"
#include "mpoly.h"
#include "n_poly.h"
#include "nf_elem.h"
#include "nmod.h"
#include "nmod_mat.h"
#include "nmod_mpoly.h"
#include "nmod_mpoly_factor.h"
#include "nmod_poly.h"
#include "nmod_poly_factor.h"
#include "nmod_poly_mat.h"
#include "nmod_vec.h"
#include "padic.h"
#include "padic_mat.h"
#include "padic_poly.h"
#include "perm.h"
#include "qadic.h"
#include "qfb.h"
#include "qqbar.h"
#include "qsieve.h"
#include "ulong_extras.h"


void acb_init__extern(acb_t x) { acb_init(x); }
arb_ptr acb_real_ptr__extern(acb_t z) { return acb_real_ptr(z); }
arb_ptr acb_imag_ptr__extern(acb_t z) { return acb_imag_ptr(z); }
void acb_get_real__extern(arb_t re, const acb_t z) { acb_get_real(re, z); }
void acb_get_imag__extern(arb_t im, const acb_t z) { acb_get_imag(im, z); }
void acb_get_mid__extern(acb_t res, const acb_t x) { acb_get_mid(res, x); }
int acb_is_zero__extern(const acb_t z) { return acb_is_zero(z); }
int acb_is_one__extern(const acb_t z) { return acb_is_one(z); }
int acb_is_exact__extern(const acb_t z) { return acb_is_exact(z); }
int acb_is_int__extern(const acb_t z) { return acb_is_int(z); }
int acb_is_int_2exp_si__extern(const acb_t z, mp_limb_signed_t e) { return acb_is_int_2exp_si(z, e); }
void acb_zero__extern(acb_t z) { acb_zero(z); }
void acb_one__extern(acb_t z) { acb_one(z); }
void acb_onei__extern(acb_t z) { acb_onei(z); }
void acb_set__extern(acb_t z, const acb_t x) { acb_set(z, x); }
void acb_set_round__extern(acb_t z, const acb_t x, mp_limb_signed_t prec) { acb_set_round(z, x, prec); }
void acb_neg_round__extern(acb_t z, const acb_t x, mp_limb_signed_t prec) { acb_neg_round(z, x, prec); }
void acb_swap__extern(acb_t z, acb_t x) { acb_swap(z, x); }
int acb_equal__extern(const acb_t x, const acb_t y) { return acb_equal(x, y); }
int acb_equal_si__extern(const acb_t x, mp_limb_signed_t y) { return acb_equal_si(x, y); }
int acb_eq__extern(const acb_t x, const acb_t y) { return acb_eq(x, y); }
int acb_ne__extern(const acb_t x, const acb_t y) { return acb_ne(x, y); }
int acb_overlaps__extern(const acb_t x, const acb_t y) { return acb_overlaps(x, y); }
int acb_contains_zero__extern(const acb_t x) { return acb_contains_zero(x); }
int acb_contains_fmpq__extern(const acb_t x, const fmpq_t y) { return acb_contains_fmpq(x, y); }
int acb_contains_fmpz__extern(const acb_t x, const fmpz_t y) { return acb_contains_fmpz(x, y); }
int acb_contains__extern(const acb_t x, const acb_t y) { return acb_contains(x, y); }
int acb_contains_interior__extern(const acb_t x, const acb_t y) { return acb_contains_interior(x, y); }
void acb_set_ui__extern(acb_t z, mp_limb_t c) { acb_set_ui(z, c); }
void acb_set_d__extern(acb_t z, double c) { acb_set_d(z, c); }
void acb_set_si__extern(acb_t z, mp_limb_signed_t c) { acb_set_si(z, c); }
void acb_set_si_si__extern(acb_t z, mp_limb_signed_t x, mp_limb_signed_t y) { acb_set_si_si(z, x, y); }
void acb_set_d_d__extern(acb_t z, double x, double y) { acb_set_d_d(z, x, y); }
void acb_set_fmpz__extern(acb_t z, const fmpz_t c) { acb_set_fmpz(z, c); }
void acb_set_fmpz_fmpz__extern(acb_t z, const fmpz_t x, const fmpz_t y) { acb_set_fmpz_fmpz(z, x, y); }
void acb_set_round_fmpz__extern(acb_t z, const fmpz_t y, mp_limb_signed_t prec) { acb_set_round_fmpz(z, y, prec); }
void acb_set_fmpq__extern(acb_t z, const fmpq_t c, mp_limb_signed_t prec) { acb_set_fmpq(z, c, prec); }
void acb_set_arb__extern(acb_t z, const arb_t c) { acb_set_arb(z, c); }
void acb_set_arb_arb__extern(acb_t z, const arb_t x, const arb_t y) { acb_set_arb_arb(z, x, y); }
void acb_set_round_arb__extern(acb_t z, const arb_t x, mp_limb_signed_t prec) { acb_set_round_arb(z, x, prec); }
void acb_trim__extern(acb_t z, const acb_t x) { acb_trim(z, x); }
void acb_add_error_arf__extern(acb_t x, const arf_t err) { acb_add_error_arf(x, err); }
void acb_add_error_mag__extern(acb_t x, const mag_t err) { acb_add_error_mag(x, err); }
void acb_add_error_arb__extern(acb_t x, const arb_t err) { acb_add_error_arb(x, err); }
void acb_union__extern(acb_t res, const acb_t x, const acb_t y, mp_limb_signed_t prec) { acb_union(res, x, y, prec); }
void acb_add__extern(acb_t z, const acb_t x, const acb_t y, mp_limb_signed_t prec) { acb_add(z, x, y, prec); }
void acb_sub__extern(acb_t z, const acb_t x, const acb_t y, mp_limb_signed_t prec) { acb_sub(z, x, y, prec); }
void acb_add_si__extern(acb_t z, const acb_t x, mp_limb_signed_t c, mp_limb_signed_t prec) { acb_add_si(z, x, c, prec); }
void acb_add_ui__extern(acb_t z, const acb_t x, mp_limb_t c, mp_limb_signed_t prec) { acb_add_ui(z, x, c, prec); }
void acb_sub_si__extern(acb_t z, const acb_t x, mp_limb_signed_t c, mp_limb_signed_t prec) { acb_sub_si(z, x, c, prec); }
void acb_sub_ui__extern(acb_t z, const acb_t x, mp_limb_t c, mp_limb_signed_t prec) { acb_sub_ui(z, x, c, prec); }
void acb_add_fmpz__extern(acb_t z, const acb_t x, const fmpz_t y, mp_limb_signed_t prec) { acb_add_fmpz(z, x, y, prec); }
void acb_add_arb__extern(acb_t z, const acb_t x, const arb_t y, mp_limb_signed_t prec) { acb_add_arb(z, x, y, prec); }
void acb_sub_fmpz__extern(acb_t z, const acb_t x, const fmpz_t y, mp_limb_signed_t prec) { acb_sub_fmpz(z, x, y, prec); }
void acb_sub_arb__extern(acb_t z, const acb_t x, const arb_t y, mp_limb_signed_t prec) { acb_sub_arb(z, x, y, prec); }
void acb_neg__extern(acb_t z, const acb_t x) { acb_neg(z, x); }
void acb_conj__extern(acb_t z, const acb_t x) { acb_conj(z, x); }
void acb_abs__extern(arb_t u, const acb_t z, mp_limb_signed_t prec) { acb_abs(u, z, prec); }
void acb_mul_ui__extern(acb_t z, const acb_t x, mp_limb_t y, mp_limb_signed_t prec) { acb_mul_ui(z, x, y, prec); }
void acb_mul_si__extern(acb_t z, const acb_t x, mp_limb_signed_t y, mp_limb_signed_t prec) { acb_mul_si(z, x, y, prec); }
void acb_mul_fmpz__extern(acb_t z, const acb_t x, const fmpz_t y, mp_limb_signed_t prec) { acb_mul_fmpz(z, x, y, prec); }
void acb_mul_arb__extern(acb_t z, const acb_t x, const arb_t y, mp_limb_signed_t prec) { acb_mul_arb(z, x, y, prec); }
void acb_mul_onei__extern(acb_t z, const acb_t x) { acb_mul_onei(z, x); }
void acb_div_onei__extern(acb_t z, const acb_t x) { acb_div_onei(z, x); }
void acb_mul_i_pow_si__extern(acb_t z, const acb_t x, mp_limb_signed_t k) { acb_mul_i_pow_si(z, x, k); }
void acb_mul_2exp_si__extern(acb_t z, const acb_t x, mp_limb_signed_t e) { acb_mul_2exp_si(z, x, e); }
void acb_mul_2exp_fmpz__extern(acb_t z, const acb_t x, const fmpz_t c) { acb_mul_2exp_fmpz(z, x, c); }
void acb_addmul_ui__extern(acb_t z, const acb_t x, mp_limb_t y, mp_limb_signed_t prec) { acb_addmul_ui(z, x, y, prec); }
void acb_addmul_si__extern(acb_t z, const acb_t x, mp_limb_signed_t y, mp_limb_signed_t prec) { acb_addmul_si(z, x, y, prec); }
void acb_submul_ui__extern(acb_t z, const acb_t x, mp_limb_t y, mp_limb_signed_t prec) { acb_submul_ui(z, x, y, prec); }
void acb_submul_si__extern(acb_t z, const acb_t x, mp_limb_signed_t y, mp_limb_signed_t prec) { acb_submul_si(z, x, y, prec); }
void acb_addmul_fmpz__extern(acb_t z, const acb_t x, const fmpz_t y, mp_limb_signed_t prec) { acb_addmul_fmpz(z, x, y, prec); }
void acb_submul_fmpz__extern(acb_t z, const acb_t x, const fmpz_t y, mp_limb_signed_t prec) { acb_submul_fmpz(z, x, y, prec); }
void acb_addmul_arb__extern(acb_t z, const acb_t x, const arb_t y, mp_limb_signed_t prec) { acb_addmul_arb(z, x, y, prec); }
void acb_submul_arb__extern(acb_t z, const acb_t x, const arb_t y, mp_limb_signed_t prec) { acb_submul_arb(z, x, y, prec); }
void acb_div_ui__extern(acb_t z, const acb_t x, mp_limb_t c, mp_limb_signed_t prec) { acb_div_ui(z, x, c, prec); }
void acb_div_si__extern(acb_t z, const acb_t x, mp_limb_signed_t c, mp_limb_signed_t prec) { acb_div_si(z, x, c, prec); }
void acb_div_arb__extern(acb_t z, const acb_t x, const arb_t c, mp_limb_signed_t prec) { acb_div_arb(z, x, c, prec); }
void acb_div_fmpz__extern(acb_t z, const acb_t x, const fmpz_t c, mp_limb_signed_t prec) { acb_div_fmpz(z, x, c, prec); }
void acb_const_pi__extern(acb_t x, mp_limb_signed_t prec) { acb_const_pi(x, prec); }
void acb_sinh__extern(acb_t y, const acb_t x, mp_limb_signed_t prec) { acb_sinh(y, x, prec); }
void acb_cosh__extern(acb_t y, const acb_t x, mp_limb_signed_t prec) { acb_cosh(y, x, prec); }
void acb_sinh_cosh__extern(acb_t y, acb_t z, const acb_t x, mp_limb_signed_t prec) { acb_sinh_cosh(y, z, x, prec); }
void acb_tanh__extern(acb_t y, const acb_t x, mp_limb_signed_t prec) { acb_tanh(y, x, prec); }
void acb_coth__extern(acb_t y, const acb_t x, mp_limb_signed_t prec) { acb_coth(y, x, prec); }
void acb_sec__extern(acb_t y, const acb_t x, mp_limb_signed_t prec) { acb_sec(y, x, prec); }
void acb_csc__extern(acb_t y, const acb_t x, mp_limb_signed_t prec) { acb_csc(y, x, prec); }
void acb_sqr__extern(acb_t res, const acb_t val, mp_limb_signed_t prec) { acb_sqr(res, val, prec); }
int acb_is_finite__extern(const acb_t x) { return acb_is_finite(x); }
void acb_indeterminate__extern(acb_t x) { acb_indeterminate(x); }
acb_ptr _acb_vec_entry_ptr__extern(acb_ptr vec, mp_limb_signed_t i) { return _acb_vec_entry_ptr(vec, i); }
void _acb_vec_zero__extern(acb_ptr A, mp_limb_signed_t n) { _acb_vec_zero(A, n); }
int _acb_vec_is_zero__extern(acb_srcptr vec, mp_limb_signed_t len) { return _acb_vec_is_zero(vec, len); }
void _acb_vec_set__extern(acb_ptr res, acb_srcptr vec, mp_limb_signed_t len) { _acb_vec_set(res, vec, len); }
void _acb_vec_set_round__extern(acb_ptr res, acb_srcptr vec, mp_limb_signed_t len, mp_limb_signed_t prec) { _acb_vec_set_round(res, vec, len, prec); }
void _acb_vec_swap__extern(acb_ptr res, acb_ptr vec, mp_limb_signed_t len) { _acb_vec_swap(res, vec, len); }
void _acb_vec_neg__extern(acb_ptr res, acb_srcptr vec, mp_limb_signed_t len) { _acb_vec_neg(res, vec, len); }
void _acb_vec_add__extern(acb_ptr res, acb_srcptr vec1, acb_srcptr vec2, mp_limb_signed_t len, mp_limb_signed_t prec) { _acb_vec_add(res, vec1, vec2, len, prec); }
void _acb_vec_sub__extern(acb_ptr res, acb_srcptr vec1, acb_srcptr vec2, mp_limb_signed_t len, mp_limb_signed_t prec) { _acb_vec_sub(res, vec1, vec2, len, prec); }
void _acb_vec_scalar_submul__extern(acb_ptr res, acb_srcptr vec, mp_limb_signed_t len, const acb_t c, mp_limb_signed_t prec) { _acb_vec_scalar_submul(res, vec, len, c, prec); }
void _acb_vec_scalar_addmul__extern(acb_ptr res, acb_srcptr vec, mp_limb_signed_t len, const acb_t c, mp_limb_signed_t prec) { _acb_vec_scalar_addmul(res, vec, len, c, prec); }
void _acb_vec_scalar_mul__extern(acb_ptr res, acb_srcptr vec, mp_limb_signed_t len, const acb_t c, mp_limb_signed_t prec) { _acb_vec_scalar_mul(res, vec, len, c, prec); }
void _acb_vec_scalar_mul_ui__extern(acb_ptr res, acb_srcptr vec, mp_limb_signed_t len, mp_limb_t c, mp_limb_signed_t prec) { _acb_vec_scalar_mul_ui(res, vec, len, c, prec); }
void _acb_vec_scalar_mul_2exp_si__extern(acb_ptr res, acb_srcptr vec, mp_limb_signed_t len, mp_limb_signed_t c) { _acb_vec_scalar_mul_2exp_si(res, vec, len, c); }
void _acb_vec_scalar_mul_onei__extern(acb_ptr res, acb_srcptr vec, mp_limb_signed_t len) { _acb_vec_scalar_mul_onei(res, vec, len); }
void _acb_vec_scalar_div_ui__extern(acb_ptr res, acb_srcptr vec, mp_limb_signed_t len, mp_limb_t c, mp_limb_signed_t prec) { _acb_vec_scalar_div_ui(res, vec, len, c, prec); }
void _acb_vec_scalar_div__extern(acb_ptr res, acb_srcptr vec, mp_limb_signed_t len, const acb_t c, mp_limb_signed_t prec) { _acb_vec_scalar_div(res, vec, len, c, prec); }
void _acb_vec_scalar_mul_arb__extern(acb_ptr res, acb_srcptr vec, mp_limb_signed_t len, const arb_t c, mp_limb_signed_t prec) { _acb_vec_scalar_mul_arb(res, vec, len, c, prec); }
void _acb_vec_scalar_div_arb__extern(acb_ptr res, acb_srcptr vec, mp_limb_signed_t len, const arb_t c, mp_limb_signed_t prec) { _acb_vec_scalar_div_arb(res, vec, len, c, prec); }
void _acb_vec_scalar_mul_fmpz__extern(acb_ptr res, acb_srcptr vec, mp_limb_signed_t len, const fmpz_t c, mp_limb_signed_t prec) { _acb_vec_scalar_mul_fmpz(res, vec, len, c, prec); }
void _acb_vec_scalar_div_fmpz__extern(acb_ptr res, acb_srcptr vec, mp_limb_signed_t len, const fmpz_t c, mp_limb_signed_t prec) { _acb_vec_scalar_div_fmpz(res, vec, len, c, prec); }
void _acb_vec_sqr__extern(acb_ptr res, acb_srcptr vec, mp_limb_signed_t len, mp_limb_signed_t prec) { _acb_vec_sqr(res, vec, len, prec); }
mp_limb_signed_t acb_rel_accuracy_bits__extern(const acb_t x) { return acb_rel_accuracy_bits(x); }
mp_limb_signed_t acb_bits__extern(const acb_t x) { return acb_bits(x); }
int acb_is_real__extern(const acb_t x) { return acb_is_real(x); }
int _acb_vec_is_real__extern(acb_srcptr v, mp_limb_signed_t len) { return _acb_vec_is_real(v, len); }
int _acb_vec_is_finite__extern(acb_srcptr vec, mp_limb_signed_t len) { return _acb_vec_is_finite(vec, len); }
int _acb_vec_equal__extern(acb_srcptr vec1, acb_srcptr vec2, mp_limb_signed_t len) { return _acb_vec_equal(vec1, vec2, len); }
int _acb_vec_overlaps__extern(acb_srcptr vec1, acb_srcptr vec2, mp_limb_signed_t len) { return _acb_vec_overlaps(vec1, vec2, len); }
int _acb_vec_contains__extern(acb_srcptr vec1, acb_srcptr vec2, mp_limb_signed_t len) { return _acb_vec_contains(vec1, vec2, len); }
void _acb_vec_get_real__extern(arb_ptr re, acb_srcptr vec, mp_limb_signed_t len) { _acb_vec_get_real(re, vec, len); }
void _acb_vec_get_imag__extern(arb_ptr im, acb_srcptr vec, mp_limb_signed_t len) { _acb_vec_get_imag(im, vec, len); }
void _acb_vec_set_real_imag__extern(acb_ptr vec, arb_srcptr re, arb_srcptr im, mp_limb_signed_t len) { _acb_vec_set_real_imag(vec, re, im, len); }
mp_limb_signed_t _acb_vec_bits__extern(acb_srcptr vec, mp_limb_signed_t len) { return _acb_vec_bits(vec, len); }
void _acb_vec_add_error_arf_vec__extern(acb_ptr res, arf_srcptr err, mp_limb_signed_t len) { _acb_vec_add_error_arf_vec(res, err, len); }
void _acb_vec_add_error_mag_vec__extern(acb_ptr res, mag_srcptr err, mp_limb_signed_t len) { _acb_vec_add_error_mag_vec(res, err, len); }
void _acb_vec_indeterminate__extern(acb_ptr vec, mp_limb_signed_t len) { _acb_vec_indeterminate(vec, len); }
void _acb_vec_trim__extern(acb_ptr res, acb_srcptr vec, mp_limb_signed_t len) { _acb_vec_trim(res, vec, len); }
int _acb_vec_get_unique_fmpz_vec__extern(fmpz *res, acb_srcptr vec, mp_limb_signed_t len) { return _acb_vec_get_unique_fmpz_vec(res, vec, len); }
mp_limb_signed_t acb_allocated_bytes__extern(const acb_t x) { return acb_allocated_bytes(x); }
mp_limb_signed_t _acb_vec_allocated_bytes__extern(acb_srcptr vec, mp_limb_signed_t len) { return _acb_vec_allocated_bytes(vec, len); }
double _acb_vec_estimate_allocated_bytes__extern(mp_limb_signed_t len, mp_limb_signed_t prec) { return _acb_vec_estimate_allocated_bytes(len, prec); }

void acb_dft_prod_init__extern(acb_dft_prod_t t, mp_limb_signed_t *cyc, mp_limb_signed_t num, mp_limb_signed_t prec) { acb_dft_prod_init(t, cyc, num, prec); }
void acb_dft_cyc_init__extern(acb_dft_cyc_t t, mp_limb_signed_t len, mp_limb_signed_t prec) { acb_dft_cyc_init(t, len, prec); }
void acb_dft_naive_init__extern(acb_dft_naive_t pol, mp_limb_signed_t len, mp_limb_signed_t prec) { acb_dft_naive_init(pol, len, prec); }
void acb_dft_naive_clear__extern(acb_dft_naive_t pol) { acb_dft_naive_clear(pol); }
void acb_dft_rad2_init__extern(acb_dft_rad2_t t, int e, mp_limb_signed_t prec) { acb_dft_rad2_init(t, e, prec); }
void acb_dft_rad2_clear__extern(acb_dft_rad2_t t) { acb_dft_rad2_clear(t); }
void acb_dft_bluestein_init__extern(acb_dft_bluestein_t t, mp_limb_signed_t n, mp_limb_signed_t prec) { acb_dft_bluestein_init(t, n, prec); }
void acb_dft_bluestein_clear__extern(acb_dft_bluestein_t t) { acb_dft_bluestein_clear(t); }
void acb_swap_ri__extern(acb_t x) { acb_swap_ri(x); }
void acb_vec_swap_ri__extern(acb_ptr v, mp_limb_signed_t len) { acb_vec_swap_ri(v, len); }
void _acb_vec_kronecker_mul__extern(acb_ptr z, acb_srcptr x, acb_srcptr y, mp_limb_signed_t len, mp_limb_signed_t prec) { _acb_vec_kronecker_mul(z, x, y, len, prec); }
void _acb_vec_kronecker_mul_step__extern(acb_ptr z, acb_srcptr x, acb_srcptr y, mp_limb_signed_t step, mp_limb_signed_t len, mp_limb_signed_t prec) { _acb_vec_kronecker_mul_step(z, x, y, step, len, prec); }
void acb_vec_printd_index__extern(acb_srcptr vec, mp_limb_signed_t len, mp_limb_signed_t digits) { acb_vec_printd_index(vec, len, digits); }

void acb_dirichlet_hardy_z_zero__extern(arb_t res, const fmpz_t n, mp_limb_signed_t prec) { acb_dirichlet_hardy_z_zero(res, n, prec); }
void acb_dirichlet_zeta_zero__extern(acb_t res, const fmpz_t n, mp_limb_signed_t prec) { acb_dirichlet_zeta_zero(res, n, prec); }

acb_ptr acb_mat_entry_ptr__extern(acb_mat_t mat, mp_limb_signed_t i, mp_limb_signed_t j) { return acb_mat_entry_ptr(mat, i, j); }
void acb_mat_swap__extern(acb_mat_t mat1, acb_mat_t mat2) { acb_mat_swap(mat1, mat2); }
void acb_mat_swap_entrywise__extern(acb_mat_t mat1, acb_mat_t mat2) { acb_mat_swap_entrywise(mat1, mat2); }
void acb_mat_window_clear__extern(acb_mat_t window) { acb_mat_window_clear(window); }
int acb_mat_is_empty__extern(const acb_mat_t mat) { return acb_mat_is_empty(mat); }
int acb_mat_is_square__extern(const acb_mat_t mat) { return acb_mat_is_square(mat); }
int acb_mat_is_diag__extern(const acb_mat_t mat) { return acb_mat_is_diag(mat); }
void acb_mat_get_mid__extern(acb_mat_t B, const acb_mat_t A) { acb_mat_get_mid(B, A); }
void acb_mat_add_error_mag__extern(acb_mat_t mat, const mag_t err) { acb_mat_add_error_mag(mat, err); }
void acb_mat_conjugate_transpose__extern(acb_mat_t mat1, const acb_mat_t mat2) { acb_mat_conjugate_transpose(mat1, mat2); }
void acb_mat_scalar_mul_2exp_si__extern(acb_mat_t B, const acb_mat_t A, mp_limb_signed_t c) { acb_mat_scalar_mul_2exp_si(B, A, c); }
void acb_mat_scalar_addmul_si__extern(acb_mat_t B, const acb_mat_t A, mp_limb_signed_t c, mp_limb_signed_t prec) { acb_mat_scalar_addmul_si(B, A, c, prec); }
void acb_mat_scalar_mul_si__extern(acb_mat_t B, const acb_mat_t A, mp_limb_signed_t c, mp_limb_signed_t prec) { acb_mat_scalar_mul_si(B, A, c, prec); }
void acb_mat_scalar_div_si__extern(acb_mat_t B, const acb_mat_t A, mp_limb_signed_t c, mp_limb_signed_t prec) { acb_mat_scalar_div_si(B, A, c, prec); }
void acb_mat_scalar_addmul_fmpz__extern(acb_mat_t B, const acb_mat_t A, const fmpz_t c, mp_limb_signed_t prec) { acb_mat_scalar_addmul_fmpz(B, A, c, prec); }
void acb_mat_scalar_mul_fmpz__extern(acb_mat_t B, const acb_mat_t A, const fmpz_t c, mp_limb_signed_t prec) { acb_mat_scalar_mul_fmpz(B, A, c, prec); }
void acb_mat_scalar_div_fmpz__extern(acb_mat_t B, const acb_mat_t A, const fmpz_t c, mp_limb_signed_t prec) { acb_mat_scalar_div_fmpz(B, A, c, prec); }
void acb_mat_scalar_addmul_acb__extern(acb_mat_t B, const acb_mat_t A, const acb_t c, mp_limb_signed_t prec) { acb_mat_scalar_addmul_acb(B, A, c, prec); }
void acb_mat_scalar_mul_acb__extern(acb_mat_t B, const acb_mat_t A, const acb_t c, mp_limb_signed_t prec) { acb_mat_scalar_mul_acb(B, A, c, prec); }
void acb_mat_scalar_div_acb__extern(acb_mat_t B, const acb_mat_t A, const acb_t c, mp_limb_signed_t prec) { acb_mat_scalar_div_acb(B, A, c, prec); }
void acb_mat_scalar_addmul_arb__extern(acb_mat_t B, const acb_mat_t A, const arb_t c, mp_limb_signed_t prec) { acb_mat_scalar_addmul_arb(B, A, c, prec); }
void acb_mat_scalar_mul_arb__extern(acb_mat_t B, const acb_mat_t A, const arb_t c, mp_limb_signed_t prec) { acb_mat_scalar_mul_arb(B, A, c, prec); }
void acb_mat_scalar_div_arb__extern(acb_mat_t B, const acb_mat_t A, const arb_t c, mp_limb_signed_t prec) { acb_mat_scalar_div_arb(B, A, c, prec); }
void acb_mat_swap_rows__extern(acb_mat_t mat, mp_limb_signed_t *perm, mp_limb_signed_t r, mp_limb_signed_t s) { acb_mat_swap_rows(mat, perm, r, s); }
mp_limb_signed_t acb_mat_allocated_bytes__extern(const acb_mat_t x) { return acb_mat_allocated_bytes(x); }

void psl2z_init__extern(psl2z_t g) { psl2z_init(g); }
void psl2z_clear__extern(psl2z_t g) { psl2z_clear(g); }
void psl2z_swap__extern(psl2z_t f, psl2z_t g) { psl2z_swap(f, g); }
void psl2z_set__extern(psl2z_t h, const psl2z_t g) { psl2z_set(h, g); }
void psl2z_one__extern(psl2z_t g) { psl2z_one(g); }
int psl2z_equal__extern(const psl2z_t f, const psl2z_t g) { return psl2z_equal(f, g); }

void acb_poly_swap__extern(acb_poly_t poly1, acb_poly_t poly2) { acb_poly_swap(poly1, poly2); }
mp_limb_signed_t acb_poly_length__extern(const acb_poly_t poly) { return acb_poly_length(poly); }
mp_limb_signed_t acb_poly_degree__extern(const acb_poly_t poly) { return acb_poly_degree(poly); }
int acb_poly_is_zero__extern(const acb_poly_t z) { return acb_poly_is_zero(z); }
int acb_poly_is_one__extern(const acb_poly_t z) { return acb_poly_is_one(z); }
int acb_poly_is_x__extern(const acb_poly_t z) { return acb_poly_is_x(z); }
void acb_poly_zero__extern(acb_poly_t poly) { acb_poly_zero(poly); }
void acb_poly_one__extern(acb_poly_t poly) { acb_poly_one(poly); }
void acb_poly_truncate__extern(acb_poly_t poly, mp_limb_signed_t newlen) { acb_poly_truncate(poly, newlen); }
void acb_poly_set_acb__extern(acb_poly_t poly, const acb_t c) { acb_poly_set_acb(poly, c); }
int acb_poly_is_real__extern(const acb_poly_t poly) { return acb_poly_is_real(poly); }
void acb_poly_neg__extern(acb_poly_t res, const acb_poly_t poly) { acb_poly_neg(res, poly); }
void acb_poly_scalar_mul_2exp_si__extern(acb_poly_t res, const acb_poly_t poly, mp_limb_signed_t c) { acb_poly_scalar_mul_2exp_si(res, poly, c); }
void acb_poly_scalar_mul__extern(acb_poly_t res, const acb_poly_t poly, const acb_t c, mp_limb_signed_t prec) { acb_poly_scalar_mul(res, poly, c, prec); }
void acb_poly_scalar_div__extern(acb_poly_t res, const acb_poly_t poly, const acb_t c, mp_limb_signed_t prec) { acb_poly_scalar_div(res, poly, c, prec); }
void _acb_poly_mul_monic__extern(acb_ptr res, acb_srcptr poly1, mp_limb_signed_t len1, acb_srcptr poly2, mp_limb_signed_t len2, mp_limb_signed_t prec) { _acb_poly_mul_monic(res, poly1, len1, poly2, len2, prec); }
void _acb_poly_acb_pow_cpx__extern(acb_ptr w, const acb_t a, const acb_t b, mp_limb_signed_t len, mp_limb_signed_t prec) { _acb_poly_acb_pow_cpx(w, a, b, len, prec); }
mp_limb_signed_t acb_poly_allocated_bytes__extern(const acb_poly_t x) { return acb_poly_allocated_bytes(x); }

mp_limb_signed_t sp2gz_dim__extern(const fmpz_mat_t mat) { return sp2gz_dim(mat); }

void acf_init__extern(acf_t x) { acf_init(x); }
void acf_clear__extern(acf_t x) { acf_clear(x); }
acf_ptr _acf_vec_init__extern(mp_limb_signed_t n) { return _acf_vec_init(n); }
void _acf_vec_clear__extern(acf_ptr v, mp_limb_signed_t n) { _acf_vec_clear(v, n); }
arf_ptr acf_real_ptr__extern(acf_t z) { return acf_real_ptr(z); }
arf_ptr acf_imag_ptr__extern(acf_t z) { return acf_imag_ptr(z); }
void acf_set__extern(acf_t z, const acf_t x) { acf_set(z, x); }
void acf_swap__extern(acf_t z, acf_t x) { acf_swap(z, x); }
int acf_equal__extern(const acf_t x, const acf_t y) { return acf_equal(x, y); }
void acf_printd__extern(const acf_t x, mp_limb_signed_t n) { acf_printd(x, n); }
mp_limb_signed_t acf_bits__extern(const acf_t x) { return acf_bits(x); }
mp_limb_signed_t acf_allocated_bytes__extern(const acf_t x) { return acf_allocated_bytes(x); }
void acf_randtest__extern(acf_t x, flint_rand_t state, mp_limb_signed_t bits, mp_limb_signed_t mag_bits) { acf_randtest(x, state, bits, mag_bits); }
void acf_get_mag__extern(mag_t res, const acf_t x) { acf_get_mag(res, x); }
void acf_neg__extern(acf_t z, const acf_t x) { acf_neg(z, x); }
int acf_set_round__extern(acf_t res, const acf_t x, mp_limb_signed_t prec, arf_rnd_t rnd) { return acf_set_round(res, x, prec, rnd); }
int acf_neg_round__extern(acf_t res, const acf_t x, mp_limb_signed_t prec, arf_rnd_t rnd) { return acf_neg_round(res, x, prec, rnd); }
int acf_add__extern(acf_t res, const acf_t x, const acf_t y, mp_limb_signed_t prec, arf_rnd_t rnd) { return acf_add(res, x, y, prec, rnd); }
int acf_sub__extern(acf_t res, const acf_t x, const acf_t y, mp_limb_signed_t prec, arf_rnd_t rnd) { return acf_sub(res, x, y, prec, rnd); }
int acf_mul__extern(acf_t res, const acf_t x, const acf_t y, mp_limb_signed_t prec, arf_rnd_t rnd) { return acf_mul(res, x, y, prec, rnd); }

void arb_init__extern(arb_t x) { arb_init(x); }
arf_ptr arb_mid_ptr__extern(arb_t z) { return arb_mid_ptr(z); }
mag_ptr arb_rad_ptr__extern(arb_t z) { return arb_rad_ptr(z); }
int arb_is_exact__extern(const arb_t x) { return arb_is_exact(x); }
int arb_equal__extern(const arb_t x, const arb_t y) { return arb_equal(x, y); }
int arb_equal_si__extern(const arb_t x, mp_limb_signed_t y) { return arb_equal_si(x, y); }
int arb_is_zero__extern(const arb_t x) { return arb_is_zero(x); }
void arb_pos_inf__extern(arb_t x) { arb_pos_inf(x); }
void arb_neg_inf__extern(arb_t x) { arb_neg_inf(x); }
void arb_zero_pm_inf__extern(arb_t x) { arb_zero_pm_inf(x); }
void arb_zero_pm_one__extern(arb_t x) { arb_zero_pm_one(x); }
void arb_unit_interval__extern(arb_t x) { arb_unit_interval(x); }
void arb_swap__extern(arb_t x, arb_t y) { arb_swap(x, y); }
void arb_set_arf__extern(arb_t x, const arf_t y) { arb_set_arf(x, y); }
void arb_set_fmpz_2exp__extern(arb_t x, const fmpz_t y, const fmpz_t exp) { arb_set_fmpz_2exp(x, y, exp); }
int arb_is_one__extern(const arb_t f) { return arb_is_one(f); }
void arb_mul_2exp_fmpz__extern(arb_t y, const arb_t x, const fmpz_t e) { arb_mul_2exp_fmpz(y, x, e); }
int arb_is_int__extern(const arb_t x) { return arb_is_int(x); }
int arb_is_int_2exp_si__extern(const arb_t x, mp_limb_signed_t e) { return arb_is_int_2exp_si(x, e); }
void arb_get_mag__extern(mag_t z, const arb_t x) { arb_get_mag(z, x); }
void arb_get_mid_arb__extern(arb_t z, const arb_t x) { arb_get_mid_arb(z, x); }
void arb_get_rad_arb__extern(arb_t z, const arb_t x) { arb_get_rad_arb(z, x); }
mp_limb_signed_t arb_rel_accuracy_bits__extern(const arb_t x) { return arb_rel_accuracy_bits(x); }
mp_limb_signed_t arb_bits__extern(const arb_t x) { return arb_bits(x); }
void arb_add_error_mag__extern(arb_t x, const mag_t err) { arb_add_error_mag(x, err); }
void arb_inv__extern(arb_t y, const arb_t x, mp_limb_signed_t prec) { arb_inv(y, x, prec); }
void arb_set_fmpq__extern(arb_t y, const fmpq_t x, mp_limb_signed_t prec) { arb_set_fmpq(y, x, prec); }
void arb_sqr__extern(arb_t res, const arb_t val, mp_limb_signed_t prec) { arb_sqr(res, val, prec); }
arb_ptr _arb_vec_entry_ptr__extern(arb_ptr vec, mp_limb_signed_t i) { return _arb_vec_entry_ptr(vec, i); }
void _arb_vec_zero__extern(arb_ptr A, mp_limb_signed_t n) { _arb_vec_zero(A, n); }
int _arb_vec_is_zero__extern(arb_srcptr vec, mp_limb_signed_t len) { return _arb_vec_is_zero(vec, len); }
int _arb_vec_is_finite__extern(arb_srcptr x, mp_limb_signed_t len) { return _arb_vec_is_finite(x, len); }
int _arb_vec_equal__extern(arb_srcptr vec1, arb_srcptr vec2, mp_limb_signed_t len) { return _arb_vec_equal(vec1, vec2, len); }
int _arb_vec_overlaps__extern(arb_srcptr vec1, arb_srcptr vec2, mp_limb_signed_t len) { return _arb_vec_overlaps(vec1, vec2, len); }
int _arb_vec_contains__extern(arb_srcptr vec1, arb_srcptr vec2, mp_limb_signed_t len) { return _arb_vec_contains(vec1, vec2, len); }
void _arb_vec_set__extern(arb_ptr res, arb_srcptr vec, mp_limb_signed_t len) { _arb_vec_set(res, vec, len); }
void _arb_vec_set_round__extern(arb_ptr res, arb_srcptr vec, mp_limb_signed_t len, mp_limb_signed_t prec) { _arb_vec_set_round(res, vec, len, prec); }
void _arb_vec_swap__extern(arb_ptr res, arb_ptr vec, mp_limb_signed_t len) { _arb_vec_swap(res, vec, len); }
void _arb_vec_neg__extern(arb_ptr B, arb_srcptr A, mp_limb_signed_t n) { _arb_vec_neg(B, A, n); }
void _arb_vec_sub__extern(arb_ptr C, arb_srcptr A, arb_srcptr B, mp_limb_signed_t n, mp_limb_signed_t prec) { _arb_vec_sub(C, A, B, n, prec); }
void _arb_vec_add__extern(arb_ptr C, arb_srcptr A, arb_srcptr B, mp_limb_signed_t n, mp_limb_signed_t prec) { _arb_vec_add(C, A, B, n, prec); }
void _arb_vec_scalar_mul__extern(arb_ptr res, arb_srcptr vec, mp_limb_signed_t len, const arb_t c, mp_limb_signed_t prec) { _arb_vec_scalar_mul(res, vec, len, c, prec); }
void _arb_vec_scalar_div__extern(arb_ptr res, arb_srcptr vec, mp_limb_signed_t len, const arb_t c, mp_limb_signed_t prec) { _arb_vec_scalar_div(res, vec, len, c, prec); }
void _arb_vec_scalar_mul_fmpz__extern(arb_ptr res, arb_srcptr vec, mp_limb_signed_t len, const fmpz_t c, mp_limb_signed_t prec) { _arb_vec_scalar_mul_fmpz(res, vec, len, c, prec); }
void _arb_vec_scalar_mul_2exp_si__extern(arb_ptr res, arb_srcptr src, mp_limb_signed_t len, mp_limb_signed_t c) { _arb_vec_scalar_mul_2exp_si(res, src, len, c); }
void _arb_vec_scalar_addmul__extern(arb_ptr res, arb_srcptr vec, mp_limb_signed_t len, const arb_t c, mp_limb_signed_t prec) { _arb_vec_scalar_addmul(res, vec, len, c, prec); }
mp_limb_signed_t _arb_vec_bits__extern(arb_srcptr x, mp_limb_signed_t len) { return _arb_vec_bits(x, len); }
void _arb_vec_add_error_arf_vec__extern(arb_ptr res, arf_srcptr err, mp_limb_signed_t len) { _arb_vec_add_error_arf_vec(res, err, len); }
void _arb_vec_add_error_mag_vec__extern(arb_ptr res, mag_srcptr err, mp_limb_signed_t len) { _arb_vec_add_error_mag_vec(res, err, len); }
void _arb_vec_indeterminate__extern(arb_ptr vec, mp_limb_signed_t len) { _arb_vec_indeterminate(vec, len); }
void _arb_vec_trim__extern(arb_ptr res, arb_srcptr vec, mp_limb_signed_t len) { _arb_vec_trim(res, vec, len); }
int _arb_vec_get_unique_fmpz_vec__extern(fmpz *res, arb_srcptr vec, mp_limb_signed_t len) { return _arb_vec_get_unique_fmpz_vec(res, vec, len); }
mp_limb_t _arb_mpn_leading_zeros__extern(mp_srcptr d, mp_size_t n) { return _arb_mpn_leading_zeros(d, n); }
mp_limb_signed_t arb_allocated_bytes__extern(const arb_t x) { return arb_allocated_bytes(x); }
mp_limb_signed_t _arb_vec_allocated_bytes__extern(arb_srcptr vec, mp_limb_signed_t len) { return _arb_vec_allocated_bytes(vec, len); }
double _arb_vec_estimate_allocated_bytes__extern(mp_limb_signed_t len, mp_limb_signed_t prec) { return _arb_vec_estimate_allocated_bytes(len, prec); }

void arf_interval_init__extern(arf_interval_t v) { arf_interval_init(v); }
void arf_interval_clear__extern(arf_interval_t v) { arf_interval_clear(v); }
arf_interval_ptr _arf_interval_vec_init__extern(mp_limb_signed_t n) { return _arf_interval_vec_init(n); }
void _arf_interval_vec_clear__extern(arf_interval_ptr v, mp_limb_signed_t n) { _arf_interval_vec_clear(v, n); }
void arf_interval_set__extern(arf_interval_t v, const arf_interval_t u) { arf_interval_set(v, u); }
void arf_interval_swap__extern(arf_interval_t v, arf_interval_t u) { arf_interval_swap(v, u); }
void arf_interval_get_arb__extern(arb_t x, const arf_interval_t v, mp_limb_signed_t prec) { arf_interval_get_arb(x, v, prec); }

arb_ptr arb_mat_entry_ptr__extern(arb_mat_t mat, mp_limb_signed_t i, mp_limb_signed_t j) { return arb_mat_entry_ptr(mat, i, j); }
void arb_mat_swap__extern(arb_mat_t mat1, arb_mat_t mat2) { arb_mat_swap(mat1, mat2); }
void arb_mat_swap_entrywise__extern(arb_mat_t mat1, arb_mat_t mat2) { arb_mat_swap_entrywise(mat1, mat2); }
void arb_mat_window_clear__extern(arb_mat_t window) { arb_mat_window_clear(window); }
int arb_mat_is_empty__extern(const arb_mat_t mat) { return arb_mat_is_empty(mat); }
int arb_mat_is_square__extern(const arb_mat_t mat) { return arb_mat_is_square(mat); }
int arb_mat_is_diag__extern(const arb_mat_t mat) { return arb_mat_is_diag(mat); }
void arb_mat_get_mid__extern(arb_mat_t B, const arb_mat_t A) { arb_mat_get_mid(B, A); }
void arb_mat_add_error_mag__extern(arb_mat_t mat, const mag_t err) { arb_mat_add_error_mag(mat, err); }
void arb_mat_scalar_mul_2exp_si__extern(arb_mat_t B, const arb_mat_t A, mp_limb_signed_t c) { arb_mat_scalar_mul_2exp_si(B, A, c); }
void arb_mat_scalar_addmul_si__extern(arb_mat_t B, const arb_mat_t A, mp_limb_signed_t c, mp_limb_signed_t prec) { arb_mat_scalar_addmul_si(B, A, c, prec); }
void arb_mat_scalar_mul_si__extern(arb_mat_t B, const arb_mat_t A, mp_limb_signed_t c, mp_limb_signed_t prec) { arb_mat_scalar_mul_si(B, A, c, prec); }
void arb_mat_scalar_div_si__extern(arb_mat_t B, const arb_mat_t A, mp_limb_signed_t c, mp_limb_signed_t prec) { arb_mat_scalar_div_si(B, A, c, prec); }
void arb_mat_scalar_addmul_fmpz__extern(arb_mat_t B, const arb_mat_t A, const fmpz_t c, mp_limb_signed_t prec) { arb_mat_scalar_addmul_fmpz(B, A, c, prec); }
void arb_mat_scalar_mul_fmpz__extern(arb_mat_t B, const arb_mat_t A, const fmpz_t c, mp_limb_signed_t prec) { arb_mat_scalar_mul_fmpz(B, A, c, prec); }
void arb_mat_scalar_div_fmpz__extern(arb_mat_t B, const arb_mat_t A, const fmpz_t c, mp_limb_signed_t prec) { arb_mat_scalar_div_fmpz(B, A, c, prec); }
void arb_mat_scalar_addmul_arb__extern(arb_mat_t B, const arb_mat_t A, const arb_t c, mp_limb_signed_t prec) { arb_mat_scalar_addmul_arb(B, A, c, prec); }
void arb_mat_scalar_mul_arb__extern(arb_mat_t B, const arb_mat_t A, const arb_t c, mp_limb_signed_t prec) { arb_mat_scalar_mul_arb(B, A, c, prec); }
void arb_mat_scalar_div_arb__extern(arb_mat_t B, const arb_mat_t A, const arb_t c, mp_limb_signed_t prec) { arb_mat_scalar_div_arb(B, A, c, prec); }
void arb_mat_swap_rows__extern(arb_mat_t mat, mp_limb_signed_t *perm, mp_limb_signed_t r, mp_limb_signed_t s) { arb_mat_swap_rows(mat, perm, r, s); }
mp_limb_signed_t arb_mat_count_not_is_zero__extern(const arb_mat_t mat) { return arb_mat_count_not_is_zero(mat); }
mp_limb_signed_t arb_mat_allocated_bytes__extern(const arb_mat_t x) { return arb_mat_allocated_bytes(x); }

void arb_poly_swap__extern(arb_poly_t poly1, arb_poly_t poly2) { arb_poly_swap(poly1, poly2); }
mp_limb_signed_t arb_poly_length__extern(const arb_poly_t poly) { return arb_poly_length(poly); }
mp_limb_signed_t arb_poly_degree__extern(const arb_poly_t poly) { return arb_poly_degree(poly); }
int arb_poly_is_zero__extern(const arb_poly_t z) { return arb_poly_is_zero(z); }
int arb_poly_is_one__extern(const arb_poly_t z) { return arb_poly_is_one(z); }
int arb_poly_is_x__extern(const arb_poly_t z) { return arb_poly_is_x(z); }
void arb_poly_zero__extern(arb_poly_t poly) { arb_poly_zero(poly); }
void arb_poly_one__extern(arb_poly_t poly) { arb_poly_one(poly); }
void arb_poly_truncate__extern(arb_poly_t poly, mp_limb_signed_t newlen) { arb_poly_truncate(poly, newlen); }
void arb_poly_set_arb__extern(arb_poly_t poly, const arb_t c) { arb_poly_set_arb(poly, c); }
void arb_poly_neg__extern(arb_poly_t res, const arb_poly_t poly) { arb_poly_neg(res, poly); }
void arb_poly_scalar_mul_2exp_si__extern(arb_poly_t res, const arb_poly_t poly, mp_limb_signed_t c) { arb_poly_scalar_mul_2exp_si(res, poly, c); }
void arb_poly_scalar_mul__extern(arb_poly_t res, const arb_poly_t poly, const arb_t c, mp_limb_signed_t prec) { arb_poly_scalar_mul(res, poly, c, prec); }
void arb_poly_scalar_div__extern(arb_poly_t res, const arb_poly_t poly, const arb_t c, mp_limb_signed_t prec) { arb_poly_scalar_div(res, poly, c, prec); }
void _arb_poly_mul_monic__extern(arb_ptr res, arb_srcptr poly1, mp_limb_signed_t len1, arb_srcptr poly2, mp_limb_signed_t len2, mp_limb_signed_t prec) { _arb_poly_mul_monic(res, poly1, len1, poly2, len2, prec); }
mp_limb_signed_t arb_poly_allocated_bytes__extern(const arb_poly_t x) { return arb_poly_allocated_bytes(x); }
int n_zerobits__extern(mp_limb_t e) { return n_zerobits(e); }
mp_limb_signed_t poly_pow_length__extern(mp_limb_signed_t poly_len, mp_limb_t exp, mp_limb_signed_t trunc) { return poly_pow_length(poly_len, exp, trunc); }

int arf_rounds_down__extern(arf_rnd_t rnd, int sgnbit) { return arf_rounds_down(rnd, sgnbit); }
int arf_rounds_up__extern(arf_rnd_t rnd, int sgnbit) { return arf_rounds_up(rnd, sgnbit); }
void arf_init__extern(arf_t x) { arf_init(x); }
void arf_zero__extern(arf_t x) { arf_zero(x); }
void arf_pos_inf__extern(arf_t x) { arf_pos_inf(x); }
void arf_neg_inf__extern(arf_t x) { arf_neg_inf(x); }
void arf_nan__extern(arf_t x) { arf_nan(x); }
int arf_is_special__extern(const arf_t x) { return arf_is_special(x); }
int arf_is_zero__extern(const arf_t x) { return arf_is_zero(x); }
int arf_is_pos_inf__extern(const arf_t x) { return arf_is_pos_inf(x); }
int arf_is_neg_inf__extern(const arf_t x) { return arf_is_neg_inf(x); }
int arf_is_nan__extern(const arf_t x) { return arf_is_nan(x); }
int arf_is_normal__extern(const arf_t x) { return arf_is_normal(x); }
int arf_is_finite__extern(const arf_t x) { return arf_is_finite(x); }
int arf_is_inf__extern(const arf_t x) { return arf_is_inf(x); }
void arf_one__extern(arf_t x) { arf_one(x); }
int arf_is_one__extern(const arf_t x) { return arf_is_one(x); }
int arf_sgn__extern(const arf_t x) { return arf_sgn(x); }
void arf_swap__extern(arf_t y, arf_t x) { arf_swap(y, x); }
void arf_neg__extern(arf_t y, const arf_t x) { arf_neg(y, x); }
void arf_init_set_ui__extern(arf_t x, mp_limb_t v) { arf_init_set_ui(x, v); }
void arf_init_set_si__extern(arf_t x, mp_limb_signed_t v) { arf_init_set_si(x, v); }
void arf_set_ui__extern(arf_t x, mp_limb_t v) { arf_set_ui(x, v); }
void arf_set_si__extern(arf_t x, mp_limb_signed_t v) { arf_set_si(x, v); }
void arf_init_set_shallow__extern(arf_t z, const arf_t x) { arf_init_set_shallow(z, x); }
void arf_init_neg_shallow__extern(arf_t z, const arf_t x) { arf_init_neg_shallow(z, x); }
void arf_init_set_mag_shallow__extern(arf_t y, const mag_t x) { arf_init_set_mag_shallow(y, x); }
void arf_init_neg_mag_shallow__extern(arf_t z, const mag_t x) { arf_init_neg_mag_shallow(z, x); }
int arf_cmpabs_mag__extern(const arf_t x, const mag_t y) { return arf_cmpabs_mag(x, y); }
int arf_mag_cmpabs__extern(const mag_t x, const arf_t y) { return arf_mag_cmpabs(x, y); }
void arf_set_mpz__extern(arf_t y, const mpz_t x) { arf_set_mpz(y, x); }
void arf_set_fmpz__extern(arf_t y, const fmpz_t x) { arf_set_fmpz(y, x); }
int arf_set_round_ui__extern(arf_t x, mp_limb_t v, mp_limb_signed_t prec, arf_rnd_t rnd) { return arf_set_round_ui(x, v, prec, rnd); }
int arf_set_round_si__extern(arf_t x, mp_limb_signed_t v, mp_limb_signed_t prec, arf_rnd_t rnd) { return arf_set_round_si(x, v, prec, rnd); }
int arf_set_round_mpz__extern(arf_t y, const mpz_t x, mp_limb_signed_t prec, arf_rnd_t rnd) { return arf_set_round_mpz(y, x, prec, rnd); }
int arf_set_round_fmpz__extern(arf_t y, const fmpz_t x, mp_limb_signed_t prec, arf_rnd_t rnd) { return arf_set_round_fmpz(y, x, prec, rnd); }
void arf_min__extern(arf_t z, const arf_t a, const arf_t b) { arf_min(z, a, b); }
void arf_max__extern(arf_t z, const arf_t a, const arf_t b) { arf_max(z, a, b); }
void arf_abs__extern(arf_t y, const arf_t x) { arf_abs(y, x); }
mp_limb_signed_t arf_bits__extern(const arf_t x) { return arf_bits(x); }
void arf_bot__extern(fmpz_t e, const arf_t x) { arf_bot(e, x); }
void arf_set_si_2exp_si__extern(arf_t x, mp_limb_signed_t man, mp_limb_signed_t exp) { arf_set_si_2exp_si(x, man, exp); }
void arf_set_ui_2exp_si__extern(arf_t x, mp_limb_t man, mp_limb_signed_t exp) { arf_set_ui_2exp_si(x, man, exp); }
void arf_mul_2exp_si__extern(arf_t y, const arf_t x, mp_limb_signed_t e) { arf_mul_2exp_si(y, x, e); }
void arf_mul_2exp_fmpz__extern(arf_t y, const arf_t x, const fmpz_t e) { arf_mul_2exp_fmpz(y, x, e); }
int arf_set_round_fmpz_2exp__extern(arf_t y, const fmpz_t x, const fmpz_t exp, mp_limb_signed_t prec, arf_rnd_t rnd) { return arf_set_round_fmpz_2exp(y, x, exp, prec, rnd); }
void arf_abs_bound_lt_2exp_fmpz__extern(fmpz_t b, const arf_t x) { arf_abs_bound_lt_2exp_fmpz(b, x); }
void arf_abs_bound_le_2exp_fmpz__extern(fmpz_t b, const arf_t x) { arf_abs_bound_le_2exp_fmpz(b, x); }
void arf_set_fmpz_2exp__extern(arf_t x, const fmpz_t man, const fmpz_t exp) { arf_set_fmpz_2exp(x, man, exp); }
int arf_neg_mul__extern(arf_t z, const arf_t x, const arf_t y, mp_limb_signed_t prec, arf_rnd_t rnd) { return arf_neg_mul(z, x, y, prec, rnd); }
int arf_mul_ui__extern(arf_ptr z, arf_srcptr x, mp_limb_t y, mp_limb_signed_t prec, arf_rnd_t rnd) { return arf_mul_ui(z, x, y, prec, rnd); }
int arf_mul_si__extern(arf_ptr z, arf_srcptr x, mp_limb_signed_t y, mp_limb_signed_t prec, arf_rnd_t rnd) { return arf_mul_si(z, x, y, prec, rnd); }
int arf_mul_fmpz__extern(arf_ptr z, arf_srcptr x, const fmpz_t y, mp_limb_signed_t prec, arf_rnd_t rnd) { return arf_mul_fmpz(z, x, y, prec, rnd); }
int arf_addmul_ui__extern(arf_ptr z, arf_srcptr x, mp_limb_t y, mp_limb_signed_t prec, arf_rnd_t rnd) { return arf_addmul_ui(z, x, y, prec, rnd); }
int arf_addmul_si__extern(arf_ptr z, arf_srcptr x, mp_limb_signed_t y, mp_limb_signed_t prec, arf_rnd_t rnd) { return arf_addmul_si(z, x, y, prec, rnd); }
int arf_addmul_fmpz__extern(arf_ptr z, arf_srcptr x, const fmpz_t y, mp_limb_signed_t prec, arf_rnd_t rnd) { return arf_addmul_fmpz(z, x, y, prec, rnd); }
int arf_submul_ui__extern(arf_ptr z, arf_srcptr x, mp_limb_t y, mp_limb_signed_t prec, arf_rnd_t rnd) { return arf_submul_ui(z, x, y, prec, rnd); }
int arf_submul_si__extern(arf_ptr z, arf_srcptr x, mp_limb_signed_t y, mp_limb_signed_t prec, arf_rnd_t rnd) { return arf_submul_si(z, x, y, prec, rnd); }
int arf_submul_fmpz__extern(arf_ptr z, arf_srcptr x, const fmpz_t y, mp_limb_signed_t prec, arf_rnd_t rnd) { return arf_submul_fmpz(z, x, y, prec, rnd); }
int arf_div_ui__extern(arf_ptr z, arf_srcptr x, mp_limb_t y, mp_limb_signed_t prec, arf_rnd_t rnd) { return arf_div_ui(z, x, y, prec, rnd); }
int arf_ui_div__extern(arf_ptr z, mp_limb_t x, arf_srcptr y, mp_limb_signed_t prec, arf_rnd_t rnd) { return arf_ui_div(z, x, y, prec, rnd); }
int arf_div_si__extern(arf_ptr z, arf_srcptr x, mp_limb_signed_t y, mp_limb_signed_t prec, arf_rnd_t rnd) { return arf_div_si(z, x, y, prec, rnd); }
int arf_si_div__extern(arf_ptr z, mp_limb_signed_t x, arf_srcptr y, mp_limb_signed_t prec, arf_rnd_t rnd) { return arf_si_div(z, x, y, prec, rnd); }
int arf_div_fmpz__extern(arf_ptr z, arf_srcptr x, const fmpz_t y, mp_limb_signed_t prec, arf_rnd_t rnd) { return arf_div_fmpz(z, x, y, prec, rnd); }
int arf_fmpz_div__extern(arf_ptr z, const fmpz_t x, arf_srcptr y, mp_limb_signed_t prec, arf_rnd_t rnd) { return arf_fmpz_div(z, x, y, prec, rnd); }
int arf_fmpz_div_fmpz__extern(arf_ptr z, const fmpz_t x, const fmpz_t y, mp_limb_signed_t prec, arf_rnd_t rnd) { return arf_fmpz_div_fmpz(z, x, y, prec, rnd); }
void arf_set_mag__extern(arf_t y, const mag_t x) { arf_set_mag(y, x); }
void mag_init_set_arf__extern(mag_t y, const arf_t x) { mag_init_set_arf(y, x); }
void mag_fast_init_set_arf__extern(mag_t y, const arf_t x) { mag_fast_init_set_arf(y, x); }
void arf_mag_fast_add_ulp__extern(mag_t z, const mag_t x, const arf_t y, mp_limb_signed_t prec) { arf_mag_fast_add_ulp(z, x, y, prec); }
void arf_mag_add_ulp__extern(mag_t z, const mag_t x, const arf_t y, mp_limb_signed_t prec) { arf_mag_add_ulp(z, x, y, prec); }
void arf_mag_set_ulp__extern(mag_t z, const arf_t y, mp_limb_signed_t prec) { arf_mag_set_ulp(z, y, prec); }
int arf_set_fmpq__extern(arf_t y, const fmpq_t x, mp_limb_signed_t prec, arf_rnd_t rnd) { return arf_set_fmpq(y, x, prec, rnd); }
mp_limb_signed_t arf_allocated_bytes__extern(const arf_t x) { return arf_allocated_bytes(x); }

void trig_prod_init__extern(trig_prod_t sum) { trig_prod_init(sum); }

mp_limb_signed_t bernoulli_denom_size__extern(mp_limb_signed_t n) { return bernoulli_denom_size(n); }
mp_limb_signed_t bernoulli_zeta_terms__extern(mp_limb_t s, mp_limb_signed_t prec) { return bernoulli_zeta_terms(s, prec); }
mp_limb_signed_t bernoulli_power_prec__extern(mp_limb_signed_t i, mp_limb_t s1, mp_limb_signed_t wp) { return bernoulli_power_prec(i, s1, wp); }
mp_limb_signed_t bernoulli_global_prec__extern(mp_limb_t nmax) { return bernoulli_global_prec(nmax); }

int bool_mat_get_entry__extern(const bool_mat_t mat, mp_limb_signed_t i, mp_limb_signed_t j) { return bool_mat_get_entry(mat, i, j); }
void bool_mat_set_entry__extern(bool_mat_t mat, mp_limb_signed_t i, mp_limb_signed_t j, int value) { bool_mat_set_entry(mat, i, j, value); }
void bool_mat_swap__extern(bool_mat_t mat1, bool_mat_t mat2) { bool_mat_swap(mat1, mat2); }
int bool_mat_is_empty__extern(const bool_mat_t mat) { return bool_mat_is_empty(mat); }
int bool_mat_is_square__extern(const bool_mat_t mat) { return bool_mat_is_square(mat); }
void bool_mat_sqr__extern(bool_mat_t B, const bool_mat_t A) { bool_mat_sqr(B, A); }

mp_limb_signed_t ca_ctx_get_option__extern(ca_ctx_t ctx, mp_limb_signed_t i) { return ca_ctx_get_option(ctx, i); }
void ca_ctx_set_option__extern(ca_ctx_t ctx, mp_limb_signed_t i, mp_limb_signed_t value) { ca_ctx_set_option(ctx, i, value); }
void _ca_make_fmpq__extern(ca_t x, ca_ctx_t ctx) { _ca_make_fmpq(x, ctx); }
void _ca_function_fx__extern(ca_t res, calcium_func_code func, const ca_t x, ca_ctx_t ctx) { _ca_function_fx(res, func, x, ctx); }
void _ca_function_fxy__extern(ca_t res, calcium_func_code func, const ca_t x, const ca_t y, ca_ctx_t ctx) { _ca_function_fxy(res, func, x, y, ctx); }
int ca_is_special__extern(const ca_t x, ca_ctx_t ctx) { return ca_is_special(x, ctx); }
int ca_is_unknown__extern(const ca_t x, ca_ctx_t ctx) { return ca_is_unknown(x, ctx); }
int ca_is_qq_elem__extern(const ca_t x, ca_ctx_t ctx) { return ca_is_qq_elem(x, ctx); }
int ca_is_qq_elem_zero__extern(const ca_t x, ca_ctx_t ctx) { return ca_is_qq_elem_zero(x, ctx); }
int ca_is_qq_elem_one__extern(const ca_t x, ca_ctx_t ctx) { return ca_is_qq_elem_one(x, ctx); }
int ca_is_qq_elem_integer__extern(const ca_t x, ca_ctx_t ctx) { return ca_is_qq_elem_integer(x, ctx); }
int ca_is_nf_elem__extern(const ca_t x, ca_ctx_t ctx) { return ca_is_nf_elem(x, ctx); }
int ca_is_generic_elem__extern(const ca_t x, ca_ctx_t ctx) { return ca_is_generic_elem(x, ctx); }
void ca_sqr__extern(ca_t res, const ca_t x, ca_ctx_t ctx) { ca_sqr(res, x, ctx); }
void ca_sqrt_ui__extern(ca_t res, mp_limb_t n, ca_ctx_t ctx) { ca_sqrt_ui(res, n, ctx); }

void ca_ext_init_set__extern(ca_ext_t res, const ca_ext_t x, ca_ctx_t ctx) { ca_ext_init_set(res, x, ctx); }
mp_limb_signed_t ca_ext_nargs__extern(const ca_ext_t x, ca_ctx_t ctx) { return ca_ext_nargs(x, ctx); }
void ca_ext_get_arg__extern(ca_t res, const ca_ext_t x, mp_limb_signed_t i, ca_ctx_t ctx) { ca_ext_get_arg(res, x, i, ctx); }
mp_limb_t ca_ext_hash__extern(const ca_ext_t x, ca_ctx_t ctx) { return ca_ext_hash(x, ctx); }

ca_ptr ca_mat_entry_ptr__extern(ca_mat_t mat, mp_limb_signed_t i, mp_limb_signed_t j) { return ca_mat_entry_ptr(mat, i, j); }
void ca_mat_swap__extern(ca_mat_t mat1, ca_mat_t mat2, ca_ctx_t ctx) { ca_mat_swap(mat1, mat2, ctx); }
void ca_mat_window_clear__extern(ca_mat_t window, ca_ctx_t ctx) { ca_mat_window_clear(window, ctx); }
int ca_mat_is_empty__extern(const ca_mat_t mat) { return ca_mat_is_empty(mat); }
int ca_mat_is_square__extern(const ca_mat_t mat) { return ca_mat_is_square(mat); }
void ca_mat_mul_si__extern(ca_mat_t B, const ca_mat_t A, mp_limb_signed_t c, ca_ctx_t ctx) { ca_mat_mul_si(B, A, c, ctx); }
void ca_mat_mul_fmpz__extern(ca_mat_t B, const ca_mat_t A, const fmpz_t c, ca_ctx_t ctx) { ca_mat_mul_fmpz(B, A, c, ctx); }
void ca_mat_mul_fmpq__extern(ca_mat_t B, const ca_mat_t A, const fmpq_t c, ca_ctx_t ctx) { ca_mat_mul_fmpq(B, A, c, ctx); }
void ca_mat_mul_ca__extern(ca_mat_t B, const ca_mat_t A, const ca_t c, ca_ctx_t ctx) { ca_mat_mul_ca(B, A, c, ctx); }
void ca_mat_div_si__extern(ca_mat_t B, const ca_mat_t A, mp_limb_signed_t c, ca_ctx_t ctx) { ca_mat_div_si(B, A, c, ctx); }
void ca_mat_div_fmpz__extern(ca_mat_t B, const ca_mat_t A, const fmpz_t c, ca_ctx_t ctx) { ca_mat_div_fmpz(B, A, c, ctx); }
void ca_mat_div_fmpq__extern(ca_mat_t B, const ca_mat_t A, const fmpq_t c, ca_ctx_t ctx) { ca_mat_div_fmpq(B, A, c, ctx); }
void ca_mat_div_ca__extern(ca_mat_t B, const ca_mat_t A, const ca_t c, ca_ctx_t ctx) { ca_mat_div_ca(B, A, c, ctx); }
void ca_mat_sqr__extern(ca_mat_t res, const ca_mat_t A, ca_ctx_t ctx) { ca_mat_sqr(res, A, ctx); }
void _ca_mat_swap_rows__extern(ca_mat_t mat, mp_limb_signed_t *perm, mp_limb_signed_t r, mp_limb_signed_t s) { _ca_mat_swap_rows(mat, perm, r, s); }

ca_ptr ca_poly_coeff_ptr__extern(ca_poly_t poly, mp_limb_signed_t i) { return ca_poly_coeff_ptr(poly, i); }
void ca_poly_swap__extern(ca_poly_t poly1, ca_poly_t poly2, ca_ctx_t ctx) { ca_poly_swap(poly1, poly2, ctx); }
void ca_poly_zero__extern(ca_poly_t poly, ca_ctx_t ctx) { ca_poly_zero(poly, ctx); }
void ca_poly_x__extern(ca_poly_t poly, ca_ctx_t ctx) { ca_poly_x(poly, ctx); }
void ca_poly_one__extern(ca_poly_t poly, ca_ctx_t ctx) { ca_poly_one(poly, ctx); }
void ca_poly_mul_ca__extern(ca_poly_t res, const ca_poly_t poly, const ca_t c, ca_ctx_t ctx) { ca_poly_mul_ca(res, poly, c, ctx); }
void ca_poly_div_ca__extern(ca_poly_t res, const ca_poly_t poly, const ca_t c, ca_ctx_t ctx) { ca_poly_div_ca(res, poly, c, ctx); }
void ca_poly_div_fmpz__extern(ca_poly_t res, const ca_poly_t poly, const fmpz_t c, ca_ctx_t ctx) { ca_poly_div_fmpz(res, poly, c, ctx); }

ca_ptr ca_vec_entry_ptr__extern(ca_vec_t vec, mp_limb_signed_t i) { return ca_vec_entry_ptr(vec, i); }
void ca_vec_swap__extern(ca_vec_t vec1, ca_vec_t vec2, ca_ctx_t ctx) { ca_vec_swap(vec1, vec2, ctx); }
mp_limb_signed_t ca_vec_length__extern(const ca_vec_t vec, ca_ctx_t ctx) { return ca_vec_length(vec, ctx); }
void _ca_vec_unknown__extern(ca_ptr vec, mp_limb_signed_t len, ca_ctx_t ctx) { _ca_vec_unknown(vec, len, ctx); }
void _ca_vec_undefined__extern(ca_ptr vec, mp_limb_signed_t len, ca_ctx_t ctx) { _ca_vec_undefined(vec, len, ctx); }
void ca_vec_append__extern(ca_vec_t vec, const ca_t f, ca_ctx_t ctx) { ca_vec_append(vec, f, ctx); }
int _ca_vec_is_fmpq_vec__extern(ca_srcptr vec, mp_limb_signed_t len, ca_ctx_t ctx) { return _ca_vec_is_fmpq_vec(vec, len, ctx); }
int _ca_vec_fmpq_vec_is_fmpz_vec__extern(ca_srcptr vec, mp_limb_signed_t len, ca_ctx_t ctx) { return _ca_vec_fmpq_vec_is_fmpz_vec(vec, len, ctx); }
void _ca_vec_fmpq_vec_get_fmpz_vec_den__extern(fmpz *c, fmpz_t den, ca_srcptr vec, mp_limb_signed_t len, ca_ctx_t ctx) { _ca_vec_fmpq_vec_get_fmpz_vec_den(c, den, vec, len, ctx); }
void _ca_vec_set_fmpz_vec_div_fmpz__extern(ca_ptr res, const fmpz *v, const fmpz_t den, mp_limb_signed_t len, ca_ctx_t ctx) { _ca_vec_set_fmpz_vec_div_fmpz(res, v, den, len, ctx); }

void calcium_stream_init_str__extern(calcium_stream_t out) { calcium_stream_init_str(out); }
void calcium_write_free__extern(calcium_stream_t out, char *s) { calcium_write_free(out, s); }
void truth_print__extern(truth_t t) { truth_print(t); }
mp_limb_t calcium_fmpz_hash__extern(const fmpz_t x) { return calcium_fmpz_hash(x); }

mp_limb_signed_t d_mat_nrows__extern(const d_mat_t mat) { return d_mat_nrows(mat); }
mp_limb_signed_t d_mat_ncols__extern(const d_mat_t mat) { return d_mat_ncols(mat); }
void d_mat_swap_entrywise__extern(d_mat_t mat1, d_mat_t mat2) { d_mat_swap_entrywise(mat1, mat2); }
int d_mat_is_square__extern(const d_mat_t mat) { return d_mat_is_square(mat); }

void _d_vec_mul_2exp__extern(double *res, const double *x, mp_limb_signed_t len, int e) { _d_vec_mul_2exp(res, x, len, e); }

mp_limb_t dirichlet_group_size__extern(const dirichlet_group_t G) { return dirichlet_group_size(G); }
void dirichlet_char_set__extern(dirichlet_char_t x, const dirichlet_group_t G, const dirichlet_char_t y) { dirichlet_char_set(x, G, y); }
int dirichlet_char_eq__extern(const dirichlet_char_t x, const dirichlet_char_t y) { return dirichlet_char_eq(x, y); }
mp_limb_t dirichlet_char_exp__extern(const dirichlet_group_t G, const dirichlet_char_t x) { return dirichlet_char_exp(G, x); }
int dirichlet_char_is_principal__extern(const dirichlet_group_t G, const dirichlet_char_t chi) { return dirichlet_char_is_principal(G, chi); }
int dirichlet_char_is_real__extern(const dirichlet_group_t G, const dirichlet_char_t chi) { return dirichlet_char_is_real(G, chi); }
int dirichlet_char_is_primitive__extern(const dirichlet_group_t G, const dirichlet_char_t chi) { return dirichlet_char_is_primitive(G, chi); }

void dlog_order23_clear__extern(dlog_order23_t t) { dlog_order23_clear(t); }
void dlog_table_clear__extern(dlog_table_t t) { dlog_table_clear(t); }
void dlog_power_clear__extern(dlog_power_t t) { dlog_power_clear(t); }
void dlog_modpe_clear__extern(dlog_modpe_t t) { dlog_modpe_clear(t); }
void dlog_bsgs_clear__extern(dlog_bsgs_t t) { dlog_bsgs_clear(t); }
void dlog_rho_clear__extern(dlog_rho_t t) { dlog_rho_clear(t); }
mp_limb_t dlog_bsgs_size__extern(mp_limb_t n, mp_limb_t num) { return dlog_bsgs_size(n, num); }

double d_polyval__extern(const double *poly, int len, double x) { return d_polyval(poly, len, x); }
int d_is_nan__extern(double x) { return d_is_nan(x); }
double d_mul_2exp_inrange__extern(double x, int i) { return d_mul_2exp_inrange(x, i); }
double d_mul_2exp_inrange2__extern(double x, int i) { return d_mul_2exp_inrange2(x, i); }
double d_mul_2exp__extern(double x, int i) { return d_mul_2exp(x, i); }

di_t di_interval__extern(double a, double b) { return di_interval(a, b); }
double _di_below__extern(double x) { return _di_below(x); }
double _di_above__extern(double x) { return _di_above(x); }
di_t di_neg__extern(di_t x) { return di_neg(x); }
di_t di_fast_add__extern(di_t x, di_t y) { return di_fast_add(x, y); }
di_t di_fast_sub__extern(di_t x, di_t y) { return di_fast_sub(x, y); }
di_t di_fast_add_d__extern(di_t x, double y) { return di_fast_add_d(x, y); }
di_t di_fast_sub_d__extern(di_t x, double y) { return di_fast_sub_d(x, y); }
di_t di_fast_mul_d__extern(di_t x, double y) { return di_fast_mul_d(x, y); }
di_t di_fast_div_d__extern(di_t x, double y) { return di_fast_div_d(x, y); }
di_t di_fast_mid__extern(di_t x) { return di_fast_mid(x); }
double di_fast_ubound_radius__extern(di_t x) { return di_fast_ubound_radius(x); }
void di_print__extern(di_t x) { di_print(x); }
double d_randtest2__extern(flint_rand_t state) { return d_randtest2(state); }
di_t di_randtest__extern(flint_rand_t state) { return di_randtest(state); }

void fexpr_init__extern(fexpr_t expr) { fexpr_init(expr); }
void fexpr_clear__extern(fexpr_t expr) { fexpr_clear(expr); }
fexpr_ptr _fexpr_vec_init__extern(mp_limb_signed_t len) { return _fexpr_vec_init(len); }
void _fexpr_vec_clear__extern(fexpr_ptr vec, mp_limb_signed_t len) { _fexpr_vec_clear(vec, len); }
void fexpr_fit_size__extern(fexpr_t expr, mp_limb_signed_t size) { fexpr_fit_size(expr, size); }
mp_limb_signed_t _fexpr_size__extern(const mp_limb_t *expr) { return _fexpr_size(expr); }
mp_limb_signed_t fexpr_size__extern(const fexpr_t expr) { return fexpr_size(expr); }
void fexpr_set__extern(fexpr_t res, const fexpr_t expr) { fexpr_set(res, expr); }
void fexpr_swap__extern(fexpr_t a, fexpr_t b) { fexpr_swap(a, b); }
int _mpn_equal__extern(mp_srcptr a, mp_srcptr b, mp_limb_signed_t len) { return _mpn_equal(a, b, len); }
int fexpr_equal__extern(const fexpr_t a, const fexpr_t b) { return fexpr_equal(a, b); }
int _fexpr_is_integer__extern(const mp_limb_t *expr) { return _fexpr_is_integer(expr); }
int fexpr_is_integer__extern(const fexpr_t expr) { return fexpr_is_integer(expr); }
int _fexpr_is_symbol__extern(const mp_limb_t *expr) { return _fexpr_is_symbol(expr); }
int fexpr_is_symbol__extern(const fexpr_t expr) { return fexpr_is_symbol(expr); }
int _fexpr_is_string__extern(const mp_limb_t *expr) { return _fexpr_is_string(expr); }
int fexpr_is_string__extern(const fexpr_t expr) { return fexpr_is_string(expr); }
int _fexpr_is_atom__extern(const mp_limb_t *expr) { return _fexpr_is_atom(expr); }
int fexpr_is_atom__extern(const fexpr_t expr) { return fexpr_is_atom(expr); }
void fexpr_zero__extern(fexpr_t res) { fexpr_zero(res); }
int fexpr_is_zero__extern(const fexpr_t expr) { return fexpr_is_zero(expr); }
void fexpr_set_symbol_builtin__extern(fexpr_t res, mp_limb_signed_t id) { fexpr_set_symbol_builtin(res, id); }
mp_limb_signed_t fexpr_size_bytes__extern(const fexpr_t expr) { return fexpr_size_bytes(expr); }
mp_limb_signed_t fexpr_allocated_bytes__extern(const fexpr_t expr) { return fexpr_allocated_bytes(expr); }
int fexpr_is_any_builtin_symbol__extern(const fexpr_t expr) { return fexpr_is_any_builtin_symbol(expr); }
int fexpr_is_builtin_symbol__extern(const fexpr_t expr, mp_limb_signed_t i) { return fexpr_is_builtin_symbol(expr, i); }
mp_limb_signed_t fexpr_nargs__extern(const fexpr_t expr) { return fexpr_nargs(expr); }
void fexpr_view_next__extern(fexpr_t view) { fexpr_view_next(view); }
void fexpr_vec_init__extern(fexpr_vec_t vec, mp_limb_signed_t len) { fexpr_vec_init(vec, len); }
void fexpr_vec_print__extern(const fexpr_vec_t F) { fexpr_vec_print(F); }
void fexpr_vec_swap__extern(fexpr_vec_t x, fexpr_vec_t y) { fexpr_vec_swap(x, y); }
void fexpr_vec_fit_length__extern(fexpr_vec_t vec, mp_limb_signed_t len) { fexpr_vec_fit_length(vec, len); }
void fexpr_vec_clear__extern(fexpr_vec_t vec) { fexpr_vec_clear(vec); }
void fexpr_vec_set__extern(fexpr_vec_t dest, const fexpr_vec_t src) { fexpr_vec_set(dest, src); }
void fexpr_vec_append__extern(fexpr_vec_t vec, const fexpr_t f) { fexpr_vec_append(vec, f); }
mp_limb_signed_t fexpr_vec_insert_unique__extern(fexpr_vec_t vec, const fexpr_t f) { return fexpr_vec_insert_unique(vec, f); }
void fexpr_vec_set_length__extern(fexpr_vec_t vec, mp_limb_signed_t len) { fexpr_vec_set_length(vec, len); }

const char * fexpr_builtin_name__extern(mp_limb_signed_t n) { return fexpr_builtin_name(n); }
mp_limb_signed_t fexpr_builtin_length__extern(void) { return fexpr_builtin_length(); }

void mpn_addmod_2expp1_1__extern(mp_limb_t *r, mp_size_t limbs, mp_limb_signed_t c) { mpn_addmod_2expp1_1(r, limbs, c); }

void flint_randinit__extern(flint_rand_t state) { flint_randinit(state); }
void flint_randseed__extern(flint_rand_t state, mp_limb_t seed1, mp_limb_t seed2) { flint_randseed(state, seed1, seed2); }
void flint_get_randseed__extern(mp_limb_t *seed1, mp_limb_t *seed2, flint_rand_t state) { flint_get_randseed(seed1, seed2, state); }
void _flint_rand_init_gmp__extern(flint_rand_t state) { _flint_rand_init_gmp(state); }
void flint_randclear__extern(flint_rand_t state) { flint_randclear(state); }
flint_rand_s * flint_rand_alloc__extern(void) { return flint_rand_alloc(); }
void flint_rand_free__extern(flint_rand_s *state) { flint_rand_free(state); }
mp_limb_t FLINT_BIT_COUNT__extern(mp_limb_t x) { return FLINT_BIT_COUNT(x); }
mp_limb_signed_t flint_mul_sizes__extern(mp_limb_signed_t x, mp_limb_signed_t y) { return flint_mul_sizes(x, y); }

void fmpq_init__extern(fmpq_t x) { fmpq_init(x); }
void fmpq_clear__extern(fmpq_t x) { fmpq_clear(x); }
void fmpq_zero__extern(fmpq_t res) { fmpq_zero(res); }
void fmpq_one__extern(fmpq_t res) { fmpq_one(res); }
int fmpq_equal__extern(const fmpq_t x, const fmpq_t y) { return fmpq_equal(x, y); }
int fmpq_sgn__extern(const fmpq_t x) { return fmpq_sgn(x); }
int fmpq_is_zero__extern(const fmpq_t x) { return fmpq_is_zero(x); }
int fmpq_is_one__extern(const fmpq_t x) { return fmpq_is_one(x); }
int fmpq_is_pm1__extern(const fmpq_t x) { return fmpq_is_pm1(x); }
void fmpq_set__extern(fmpq_t dest, const fmpq_t src) { fmpq_set(dest, src); }
void fmpq_swap__extern(fmpq_t op1, fmpq_t op2) { fmpq_swap(op1, op2); }
void fmpq_neg__extern(fmpq_t dest, const fmpq_t src) { fmpq_neg(dest, src); }
void fmpq_abs__extern(fmpq_t dest, const fmpq_t src) { fmpq_abs(dest, src); }
int fmpq_equal_ui__extern(fmpq_t q, mp_limb_t n) { return fmpq_equal_ui(q, n); }
int fmpq_equal_si__extern(fmpq_t q, mp_limb_signed_t n) { return fmpq_equal_si(q, n); }
void fmpq_set_fmpz__extern(fmpq_t q, const fmpz_t n) { fmpq_set_fmpz(q, n); }
void fmpq_set_mpq__extern(fmpq_t dest, const mpq_t src) { fmpq_set_mpq(dest, src); }
void fmpq_get_mpq__extern(mpq_t dest, const fmpq_t src) { fmpq_get_mpq(dest, src); }
void _fmpq_ball_swap__extern(_fmpq_ball_t x, _fmpq_ball_t y) { _fmpq_ball_swap(x, y); }

fmpq * fmpq_mat_entry__extern(const fmpq_mat_t mat, mp_limb_signed_t i, mp_limb_signed_t j) { return fmpq_mat_entry(mat, i, j); }
fmpz * fmpq_mat_entry_num__extern(const fmpq_mat_t mat, mp_limb_signed_t i, mp_limb_signed_t j) { return fmpq_mat_entry_num(mat, i, j); }
fmpz * fmpq_mat_entry_den__extern(const fmpq_mat_t mat, mp_limb_signed_t i, mp_limb_signed_t j) { return fmpq_mat_entry_den(mat, i, j); }
mp_limb_signed_t fmpq_mat_nrows__extern(const fmpq_mat_t mat) { return fmpq_mat_nrows(mat); }
mp_limb_signed_t fmpq_mat_ncols__extern(const fmpq_mat_t mat) { return fmpq_mat_ncols(mat); }
void fmpq_mat_swap__extern(fmpq_mat_t mat1, fmpq_mat_t mat2) { fmpq_mat_swap(mat1, mat2); }
int fmpq_mat_is_empty__extern(const fmpq_mat_t mat) { return fmpq_mat_is_empty(mat); }
int fmpq_mat_is_square__extern(const fmpq_mat_t mat) { return fmpq_mat_is_square(mat); }

void fmpq_mpoly_ctx_init__extern(fmpq_mpoly_ctx_t ctx, mp_limb_signed_t nvars, const ordering_t ord) { fmpq_mpoly_ctx_init(ctx, nvars, ord); }
void fmpq_mpoly_ctx_init_rand__extern(fmpq_mpoly_ctx_t ctx, flint_rand_t state, mp_limb_signed_t max_nvars) { fmpq_mpoly_ctx_init_rand(ctx, state, max_nvars); }
void fmpq_mpoly_ctx_clear__extern(fmpq_mpoly_ctx_t ctx) { fmpq_mpoly_ctx_clear(ctx); }
mp_limb_signed_t fmpq_mpoly_ctx_nvars__extern(const fmpq_mpoly_ctx_t ctx) { return fmpq_mpoly_ctx_nvars(ctx); }
ordering_t fmpq_mpoly_ctx_ord__extern(const fmpq_mpoly_ctx_t ctx) { return fmpq_mpoly_ctx_ord(ctx); }
fmpq * fmpq_mpoly_content_ref__extern(fmpq_mpoly_t A, const fmpq_mpoly_ctx_t ctx) { return fmpq_mpoly_content_ref(A, ctx); }
fmpz_mpoly_struct * fmpq_mpoly_zpoly_ref__extern(fmpq_mpoly_t A, const fmpq_mpoly_ctx_t ctx) { return fmpq_mpoly_zpoly_ref(A, ctx); }
fmpz * fmpq_mpoly_zpoly_term_coeff_ref__extern(fmpq_mpoly_t A, mp_limb_signed_t i, const fmpq_mpoly_ctx_t ctx) { return fmpq_mpoly_zpoly_term_coeff_ref(A, i, ctx); }
void fmpq_mpoly_init__extern(fmpq_mpoly_t A, const fmpq_mpoly_ctx_t ctx) { fmpq_mpoly_init(A, ctx); }
void fmpq_mpoly_init2__extern(fmpq_mpoly_t A, mp_limb_signed_t alloc, const fmpq_mpoly_ctx_t ctx) { fmpq_mpoly_init2(A, alloc, ctx); }
void fmpq_mpoly_init3__extern(fmpq_mpoly_t A, mp_limb_signed_t alloc, mp_limb_t bits, const fmpq_mpoly_ctx_t ctx) { fmpq_mpoly_init3(A, alloc, bits, ctx); }
void fmpq_mpoly_realloc__extern(fmpq_mpoly_t A, mp_limb_signed_t alloc, const fmpq_mpoly_ctx_t ctx) { fmpq_mpoly_realloc(A, alloc, ctx); }
void fmpq_mpoly_fit_length__extern(fmpq_mpoly_t A, mp_limb_signed_t len, const fmpq_mpoly_ctx_t ctx) { fmpq_mpoly_fit_length(A, len, ctx); }
void fmpq_mpoly_clear__extern(fmpq_mpoly_t A, const fmpq_mpoly_ctx_t ctx) { fmpq_mpoly_clear(A, ctx); }
void fmpq_mpoly_fit_bits__extern(fmpq_mpoly_t A, mp_limb_t bits, const fmpq_mpoly_ctx_t ctx) { fmpq_mpoly_fit_bits(A, bits, ctx); }
void fmpq_mpoly_gen__extern(fmpq_mpoly_t A, mp_limb_signed_t var, const fmpq_mpoly_ctx_t ctx) { fmpq_mpoly_gen(A, var, ctx); }
int fmpq_mpoly_is_gen__extern(const fmpq_mpoly_t A, mp_limb_signed_t var, const fmpq_mpoly_ctx_t ctx) { return fmpq_mpoly_is_gen(A, var, ctx); }
void fmpq_mpoly_set__extern(fmpq_mpoly_t A, const fmpq_mpoly_t B, const fmpq_mpoly_ctx_t ctx) { fmpq_mpoly_set(A, B, ctx); }
int fmpq_mpoly_equal__extern(const fmpq_mpoly_t A, const fmpq_mpoly_t B, const fmpq_mpoly_ctx_t ctx) { return fmpq_mpoly_equal(A, B, ctx); }
void fmpq_mpoly_swap__extern(fmpq_mpoly_t A, fmpq_mpoly_t B, const fmpq_mpoly_ctx_t ctx) { fmpq_mpoly_swap(A, B, ctx); }
int fmpq_mpoly_is_fmpq__extern(const fmpq_mpoly_t A, const fmpq_mpoly_ctx_t ctx) { return fmpq_mpoly_is_fmpq(A, ctx); }
void fmpq_mpoly_zero__extern(fmpq_mpoly_t A, const fmpq_mpoly_ctx_t ctx) { fmpq_mpoly_zero(A, ctx); }
void fmpq_mpoly_one__extern(fmpq_mpoly_t A, const fmpq_mpoly_ctx_t ctx) { fmpq_mpoly_one(A, ctx); }
int fmpq_mpoly_is_zero__extern(const fmpq_mpoly_t A, const fmpq_mpoly_ctx_t ctx) { return fmpq_mpoly_is_zero(A, ctx); }
int fmpq_mpoly_is_one__extern(const fmpq_mpoly_t A, const fmpq_mpoly_ctx_t ctx) { return fmpq_mpoly_is_one(A, ctx); }
void fmpq_mpoly_used_vars__extern(int *used, const fmpq_mpoly_t A, const fmpq_mpoly_ctx_t ctx) { fmpq_mpoly_used_vars(used, A, ctx); }
void fmpq_mpoly_get_denominator__extern(fmpz_t d, const fmpq_mpoly_t A, const fmpq_mpoly_ctx_t ctx) { fmpq_mpoly_get_denominator(d, A, ctx); }
mp_limb_signed_t fmpq_mpoly_length__extern(const fmpq_mpoly_t A, const fmpq_mpoly_ctx_t ctx) { return fmpq_mpoly_length(A, ctx); }
void fmpq_mpoly_resize__extern(fmpq_mpoly_t A, mp_limb_signed_t new_length, const fmpq_mpoly_ctx_t ctx) { fmpq_mpoly_resize(A, new_length, ctx); }
void fmpq_mpoly_set_term_exp_fmpz__extern(fmpq_mpoly_t A, mp_limb_signed_t i, fmpz *const *exps, const fmpq_mpoly_ctx_t ctx) { fmpq_mpoly_set_term_exp_fmpz(A, i, exps, ctx); }
void fmpq_mpoly_set_term_exp_ui__extern(fmpq_mpoly_t A, mp_limb_signed_t i, const mp_limb_t *exps, const fmpq_mpoly_ctx_t ctx) { fmpq_mpoly_set_term_exp_ui(A, i, exps, ctx); }
void fmpq_mpoly_sort_terms__extern(fmpq_mpoly_t A, const fmpq_mpoly_ctx_t ctx) { fmpq_mpoly_sort_terms(A, ctx); }
void fmpq_mpoly_combine_like_terms__extern(fmpq_mpoly_t A, const fmpq_mpoly_ctx_t ctx) { fmpq_mpoly_combine_like_terms(A, ctx); }
void fmpq_mpoly_randtest_bounds__extern(fmpq_mpoly_t A, flint_rand_t state, mp_limb_signed_t length, mp_limb_t coeff_bits, mp_limb_t *exp_bounds, const fmpq_mpoly_ctx_t ctx) { fmpq_mpoly_randtest_bounds(A, state, length, coeff_bits, exp_bounds, ctx); }
void fmpq_mpoly_randtest_bound__extern(fmpq_mpoly_t A, flint_rand_t state, mp_limb_signed_t length, mp_limb_t coeff_bits, mp_limb_t exp_bound, const fmpq_mpoly_ctx_t ctx) { fmpq_mpoly_randtest_bound(A, state, length, coeff_bits, exp_bound, ctx); }
void fmpq_mpoly_randtest_bits__extern(fmpq_mpoly_t A, flint_rand_t state, mp_limb_signed_t length, mp_limb_t coeff_bits, mp_limb_t exp_bits, const fmpq_mpoly_ctx_t ctx) { fmpq_mpoly_randtest_bits(A, state, length, coeff_bits, exp_bits, ctx); }
void fmpq_mpoly_neg__extern(fmpq_mpoly_t A, const fmpq_mpoly_t B, const fmpq_mpoly_ctx_t ctx) { fmpq_mpoly_neg(A, B, ctx); }
int fmpq_mpoly_is_square__extern(const fmpq_mpoly_t A, const fmpq_mpoly_ctx_t ctx) { return fmpq_mpoly_is_square(A, ctx); }
void fmpq_mpoly_content__extern(fmpq_t g, const fmpq_mpoly_t A, const fmpq_mpoly_ctx_t ctx) { fmpq_mpoly_content(g, A, ctx); }
void fmpq_mpoly_univar_swap__extern(fmpq_mpoly_univar_t A, fmpq_mpoly_univar_t B, const fmpq_mpoly_ctx_t ctx) { fmpq_mpoly_univar_swap(A, B, ctx); }
int fmpq_mpoly_univar_degree_fits_si__extern(const fmpq_mpoly_univar_t A, const fmpq_mpoly_ctx_t ctx) { return fmpq_mpoly_univar_degree_fits_si(A, ctx); }
mp_limb_signed_t fmpq_mpoly_univar_length__extern(const fmpq_mpoly_univar_t A, const fmpq_mpoly_ctx_t ctx) { return fmpq_mpoly_univar_length(A, ctx); }
mp_limb_signed_t fmpq_mpoly_univar_get_term_exp_si__extern(fmpq_mpoly_univar_t A, mp_limb_signed_t i, const fmpq_mpoly_ctx_t ctx) { return fmpq_mpoly_univar_get_term_exp_si(A, i, ctx); }
void fmpq_mpoly_univar_get_term_coeff__extern(fmpq_mpoly_t c, const fmpq_mpoly_univar_t A, mp_limb_signed_t i, const fmpq_mpoly_ctx_t ctx) { fmpq_mpoly_univar_get_term_coeff(c, A, i, ctx); }
void fmpq_mpoly_univar_swap_term_coeff__extern(fmpq_mpoly_t c, fmpq_mpoly_univar_t A, mp_limb_signed_t i, const fmpq_mpoly_ctx_t ctx) { fmpq_mpoly_univar_swap_term_coeff(c, A, i, ctx); }

mp_limb_signed_t fmpq_mpoly_factor_length__extern(const fmpq_mpoly_factor_t f, const fmpq_mpoly_ctx_t ctx) { return fmpq_mpoly_factor_length(f, ctx); }
void fmpq_mpoly_factor_get_constant_fmpq__extern(fmpq_t c, const fmpq_mpoly_factor_t f, const fmpq_mpoly_ctx_t ctx) { fmpq_mpoly_factor_get_constant_fmpq(c, f, ctx); }
void fmpq_mpoly_factor_get_base__extern(fmpq_mpoly_t p, const fmpq_mpoly_factor_t f, mp_limb_signed_t i, const fmpq_mpoly_ctx_t ctx) { fmpq_mpoly_factor_get_base(p, f, i, ctx); }
void fmpq_mpoly_factor_swap_base__extern(fmpq_mpoly_t p, fmpq_mpoly_factor_t f, mp_limb_signed_t i, const fmpq_mpoly_ctx_t ctx) { fmpq_mpoly_factor_swap_base(p, f, i, ctx); }
mp_limb_signed_t fmpq_mpoly_factor_get_exp_si__extern(fmpq_mpoly_factor_t f, mp_limb_signed_t i, const fmpq_mpoly_ctx_t ctx) { return fmpq_mpoly_factor_get_exp_si(f, i, ctx); }

mp_limb_signed_t fmpq_poly_degree__extern(const fmpq_poly_t poly) { return fmpq_poly_degree(poly); }
mp_limb_signed_t fmpq_poly_length__extern(const fmpq_poly_t poly) { return fmpq_poly_length(poly); }
int fmpq_poly_is_zero__extern(const fmpq_poly_t poly) { return fmpq_poly_is_zero(poly); }
int fmpq_poly_is_gen__extern(const fmpq_poly_t op) { return fmpq_poly_is_gen(op); }
void fmpq_poly_addmul__extern(fmpq_poly_t rop, const fmpq_poly_t op1, const fmpq_poly_t op2) { fmpq_poly_addmul(rop, op1, op2); }
void fmpq_poly_submul__extern(fmpq_poly_t rop, const fmpq_poly_t op1, const fmpq_poly_t op2) { fmpq_poly_submul(rop, op1, op2); }
void _fmpq_poly_inv_series__extern(fmpz *Qinv, fmpz_t Qinvden, const fmpz *Q, const fmpz_t Qden, mp_limb_signed_t Qlen, mp_limb_signed_t n) { _fmpq_poly_inv_series(Qinv, Qinvden, Q, Qden, Qlen, n); }
void fmpq_poly_inv_series__extern(fmpq_poly_t Qinv, const fmpq_poly_t Q, mp_limb_signed_t n) { fmpq_poly_inv_series(Qinv, Q, n); }

void _fmpz_demote__extern(fmpz_t f) { _fmpz_demote(f); }
void fmpz_init__extern(fmpz_t f) { fmpz_init(f); }
void fmpz_clear__extern(fmpz_t f) { fmpz_clear(f); }
void fmpz_init_set__extern(fmpz_t f, const fmpz_t g) { fmpz_init_set(f, g); }
void fmpz_init_set_ui__extern(fmpz_t f, mp_limb_t g) { fmpz_init_set_ui(f, g); }
void fmpz_init_set_si__extern(fmpz_t f, mp_limb_signed_t g) { fmpz_init_set_si(f, g); }
void fmpz_zero__extern(fmpz_t f) { fmpz_zero(f); }
void fmpz_one__extern(fmpz_t f) { fmpz_one(f); }
void fmpz_swap__extern(fmpz_t f, fmpz_t g) { fmpz_swap(f, g); }
void fmpz_set_si__extern(fmpz_t f, mp_limb_signed_t val) { fmpz_set_si(f, val); }
void fmpz_set_ui__extern(fmpz_t f, mp_limb_t val) { fmpz_set_ui(f, val); }
void fmpz_neg_ui__extern(fmpz_t f, mp_limb_t val) { fmpz_neg_ui(f, val); }
void fmpz_get_uiui__extern(mp_limb_t *hi, mp_limb_t *low, const fmpz_t f) { fmpz_get_uiui(hi, low, f); }
void fmpz_set_uiui__extern(fmpz_t f, mp_limb_t hi, mp_limb_t lo) { fmpz_set_uiui(f, hi, lo); }
void fmpz_neg_uiui__extern(fmpz_t f, mp_limb_t hi, mp_limb_t lo) { fmpz_neg_uiui(f, hi, lo); }
void fmpz_set_signed_uiui__extern(fmpz_t r, mp_limb_t hi, mp_limb_t lo) { fmpz_set_signed_uiui(r, hi, lo); }
int fmpz_is_zero__extern(const fmpz_t f) { return fmpz_is_zero(f); }
int fmpz_is_one__extern(const fmpz_t f) { return fmpz_is_one(f); }
int fmpz_is_pm1__extern(const fmpz_t f) { return fmpz_is_pm1(f); }
int fmpz_is_even__extern(const fmpz_t f) { return fmpz_is_even(f); }
int fmpz_is_odd__extern(const fmpz_t f) { return fmpz_is_odd(f); }
void fmpz_add_si__extern(fmpz_t f, const fmpz_t g, mp_limb_signed_t x) { fmpz_add_si(f, g, x); }
void fmpz_sub_si__extern(fmpz_t f, const fmpz_t g, mp_limb_signed_t x) { fmpz_sub_si(f, g, x); }
void fmpz_mul2_uiui__extern(fmpz_t f, const fmpz_t g, mp_limb_t h1, mp_limb_t h2) { fmpz_mul2_uiui(f, g, h1, h2); }
int fmpz_divisible_si__extern(const fmpz_t f, mp_limb_signed_t g) { return fmpz_divisible_si(f, g); }
void fmpz_divexact2_uiui__extern(fmpz_t f, const fmpz_t g, mp_limb_t h1, mp_limb_t h2) { fmpz_divexact2_uiui(f, g, h1, h2); }
mp_limb_t fmpz_mod_ui__extern(fmpz_t f, const fmpz_t g, mp_limb_t h) { return fmpz_mod_ui(f, g, h); }
void fmpz_set_ui_smod__extern(fmpz_t f, mp_limb_t x, mp_limb_t m) { fmpz_set_ui_smod(f, x, m); }
void fmpz_negmod__extern(fmpz_t r, const fmpz_t a, const fmpz_t mod) { fmpz_negmod(r, a, mod); }

void fmpz_add_inline__extern(fmpz_t z, const fmpz_t x, const fmpz_t y) { fmpz_add_inline(z, x, y); }
void fmpz_add_si_inline__extern(fmpz_t z, const fmpz_t x, mp_limb_signed_t y) { fmpz_add_si_inline(z, x, y); }
void fmpz_sub_si_inline__extern(fmpz_t z, const fmpz_t x, mp_limb_signed_t y) { fmpz_sub_si_inline(z, x, y); }
void fmpz_add_ui_inline__extern(fmpz_t z, const fmpz_t x, mp_limb_t y) { fmpz_add_ui_inline(z, x, y); }
void fmpz_add2_fmpz_si_inline__extern(fmpz_t z, const fmpz_t x, const fmpz_t y, mp_limb_signed_t c) { fmpz_add2_fmpz_si_inline(z, x, y, c); }
void fmpz_set_mpn_large__extern(fmpz_t z, mp_srcptr src, mp_size_t n, int negative) { fmpz_set_mpn_large(z, src, n, negative); }
void fmpz_adiv_q_2exp__extern(fmpz_t z, const fmpz_t x, mp_limb_t exp) { fmpz_adiv_q_2exp(z, x, exp); }
void _fmpz_set_si_small__extern(fmpz_t x, mp_limb_signed_t v) { _fmpz_set_si_small(x, v); }
mp_limb_signed_t _fmpz_sub_small__extern(const fmpz_t x, const fmpz_t y) { return _fmpz_sub_small(x, y); }
mp_size_t _fmpz_size__extern(const fmpz_t f) { return _fmpz_size(f); }
void fmpz_ui_mul_ui__extern(fmpz_t r, mp_limb_t a, mp_limb_t b) { fmpz_ui_mul_ui(r, a, b); }
void fmpz_max__extern(fmpz_t z, const fmpz_t x, const fmpz_t y) { fmpz_max(z, x, y); }
void fmpz_min__extern(fmpz_t z, const fmpz_t x, const fmpz_t y) { fmpz_min(z, x, y); }
mp_limb_signed_t fmpz_allocated_bytes__extern(const fmpz_t x) { return fmpz_allocated_bytes(x); }

fmpz * fmpz_mat_entry__extern(const fmpz_mat_t mat, mp_limb_signed_t i, mp_limb_signed_t j) { return fmpz_mat_entry(mat, i, j); }
mp_limb_signed_t fmpz_mat_nrows__extern(const fmpz_mat_t mat) { return fmpz_mat_nrows(mat); }
mp_limb_signed_t fmpz_mat_ncols__extern(const fmpz_mat_t mat) { return fmpz_mat_ncols(mat); }
int fmpz_mat_is_empty__extern(const fmpz_mat_t mat) { return fmpz_mat_is_empty(mat); }
int fmpz_mat_is_square__extern(const fmpz_mat_t mat) { return fmpz_mat_is_square(mat); }
void fmpz_mat_swap_rows__extern(fmpz_mat_t mat, mp_limb_signed_t *perm, mp_limb_signed_t r, mp_limb_signed_t s) { fmpz_mat_swap_rows(mat, perm, r, s); }
void _fmpz_mat_charpoly__extern(fmpz *cp, const fmpz_mat_t mat) { _fmpz_mat_charpoly(cp, mat); }
void fmpz_mat_charpoly__extern(fmpz_poly_t cp, const fmpz_mat_t mat) { fmpz_mat_charpoly(cp, mat); }
mp_limb_signed_t _fmpz_mat_minpoly__extern(fmpz *cp, const fmpz_mat_t mat) { return _fmpz_mat_minpoly(cp, mat); }
void fmpz_mat_minpoly__extern(fmpz_poly_t cp, const fmpz_mat_t mat) { fmpz_mat_minpoly(cp, mat); }

const fmpz * fmpz_mod_ctx_modulus__extern(const fmpz_mod_ctx_t ctx) { return fmpz_mod_ctx_modulus(ctx); }
void fmpz_mod_add__extern(fmpz_t a, const fmpz_t b, const fmpz_t c, const fmpz_mod_ctx_t ctx) { fmpz_mod_add(a, b, c, ctx); }
void fmpz_mod_sub__extern(fmpz_t a, const fmpz_t b, const fmpz_t c, const fmpz_mod_ctx_t ctx) { fmpz_mod_sub(a, b, c, ctx); }
void fmpz_mod_mul__extern(fmpz_t a, const fmpz_t b, const fmpz_t c, const fmpz_mod_ctx_t ctx) { fmpz_mod_mul(a, b, c, ctx); }
const fmpz * fmpz_mod_discrete_log_pohlig_hellman_primitive_root__extern(fmpz_mod_discrete_log_pohlig_hellman_t L) { return fmpz_mod_discrete_log_pohlig_hellman_primitive_root(L); }

fmpz * fmpz_mod_mat_entry__extern(const fmpz_mod_mat_t mat, mp_limb_signed_t i, mp_limb_signed_t j) { return fmpz_mod_mat_entry(mat, i, j); }
mp_limb_signed_t fmpz_mod_mat_nrows__extern(const fmpz_mod_mat_t mat, const fmpz_mod_ctx_t ctx) { return fmpz_mod_mat_nrows(mat, ctx); }
mp_limb_signed_t fmpz_mod_mat_ncols__extern(const fmpz_mod_mat_t mat, const fmpz_mod_ctx_t ctx) { return fmpz_mod_mat_ncols(mat, ctx); }
void fmpz_mod_mat_one__extern(fmpz_mod_mat_t mat, const fmpz_mod_ctx_t ctx) { fmpz_mod_mat_one(mat, ctx); }
void fmpz_mod_mat_zero__extern(fmpz_mod_mat_t mat, const fmpz_mod_ctx_t ctx) { fmpz_mod_mat_zero(mat, ctx); }
int fmpz_mod_mat_is_empty__extern(const fmpz_mod_mat_t mat, const fmpz_mod_ctx_t ctx) { return fmpz_mod_mat_is_empty(mat, ctx); }
int fmpz_mod_mat_is_square__extern(const fmpz_mod_mat_t mat, const fmpz_mod_ctx_t ctx) { return fmpz_mod_mat_is_square(mat, ctx); }
void fmpz_mod_mat_set_nmod_mat__extern(fmpz_mod_mat_t A, const nmod_mat_t B, const fmpz_mod_ctx_t ctx) { fmpz_mod_mat_set_nmod_mat(A, B, ctx); }
void _fmpz_mod_mat_reduce__extern(fmpz_mod_mat_t mat, const fmpz_mod_ctx_t ctx) { _fmpz_mod_mat_reduce(mat, ctx); }
void fmpz_mod_mat_randops__extern(fmpz_mod_mat_t mat, flint_rand_t state, mp_limb_signed_t count, const fmpz_mod_ctx_t ctx) { fmpz_mod_mat_randops(mat, state, count, ctx); }
void fmpz_mod_mat_concat_horizontal__extern(fmpz_mod_mat_t res, const fmpz_mod_mat_t mat1, const fmpz_mod_mat_t mat2, const fmpz_mod_ctx_t ctx) { fmpz_mod_mat_concat_horizontal(res, mat1, mat2, ctx); }
void fmpz_mod_mat_concat_vertical__extern(fmpz_mod_mat_t res, const fmpz_mod_mat_t mat1, const fmpz_mod_mat_t mat2, const fmpz_mod_ctx_t ctx) { fmpz_mod_mat_concat_vertical(res, mat1, mat2, ctx); }
void fmpz_mod_mat_transpose__extern(fmpz_mod_mat_t B, const fmpz_mod_mat_t A, const fmpz_mod_ctx_t ctx) { fmpz_mod_mat_transpose(B, A, ctx); }
void fmpz_mod_mat_swap_rows__extern(fmpz_mod_mat_t mat, mp_limb_signed_t *perm, mp_limb_signed_t r, mp_limb_signed_t s, const fmpz_mod_ctx_t ctx) { fmpz_mod_mat_swap_rows(mat, perm, r, s, ctx); }
void fmpz_mod_mat_invert_rows__extern(fmpz_mod_mat_t mat, mp_limb_signed_t *perm, const fmpz_mod_ctx_t ctx) { fmpz_mod_mat_invert_rows(mat, perm, ctx); }
void fmpz_mod_mat_swap_cols__extern(fmpz_mod_mat_t mat, mp_limb_signed_t *perm, mp_limb_signed_t r, mp_limb_signed_t s, const fmpz_mod_ctx_t ctx) { fmpz_mod_mat_swap_cols(mat, perm, r, s, ctx); }
void fmpz_mod_mat_invert_cols__extern(fmpz_mod_mat_t mat, mp_limb_signed_t *perm, const fmpz_mod_ctx_t ctx) { fmpz_mod_mat_invert_cols(mat, perm, ctx); }

mp_limb_signed_t fmpz_mod_mpoly_ctx_nvars__extern(const fmpz_mod_mpoly_ctx_t ctx) { return fmpz_mod_mpoly_ctx_nvars(ctx); }
ordering_t fmpz_mod_mpoly_ctx_ord__extern(const fmpz_mod_mpoly_ctx_t ctx) { return fmpz_mod_mpoly_ctx_ord(ctx); }
const fmpz * fmpz_mod_mpoly_ctx_modulus__extern(const fmpz_mod_mpoly_ctx_t ctx) { return fmpz_mod_mpoly_ctx_modulus(ctx); }
void fmpz_mod_mpoly_ctx_get_modulus__extern(fmpz_t m, const fmpz_mod_mpoly_ctx_t ctx) { fmpz_mod_mpoly_ctx_get_modulus(m, ctx); }
void fmpz_mod_mpoly_init__extern(fmpz_mod_mpoly_t A, const fmpz_mod_mpoly_ctx_t ctx) { fmpz_mod_mpoly_init(A, ctx); }
void _fmpz_mod_mpoly_fit_length__extern(fmpz **coeffs, mp_limb_signed_t *coeffs_alloc, mp_limb_t **exps, mp_limb_signed_t *exps_alloc, mp_limb_signed_t N, mp_limb_signed_t length) { _fmpz_mod_mpoly_fit_length(coeffs, coeffs_alloc, exps, exps_alloc, N, length); }
void _fmpz_mod_mpoly_set_length__extern(fmpz_mod_mpoly_t A, mp_limb_signed_t newlen, const fmpz_mod_mpoly_ctx_t ctx) { _fmpz_mod_mpoly_set_length(A, newlen, ctx); }
void fmpz_mod_mpoly_truncate__extern(fmpz_mod_mpoly_t A, mp_limb_signed_t newlen, const fmpz_mod_mpoly_ctx_t ctx) { fmpz_mod_mpoly_truncate(A, newlen, ctx); }
void fmpz_mod_mpoly_swap__extern(fmpz_mod_mpoly_t A, fmpz_mod_mpoly_t B, const fmpz_mod_mpoly_ctx_t ctx) { fmpz_mod_mpoly_swap(A, B, ctx); }
void fmpz_mod_mpoly_zero__extern(fmpz_mod_mpoly_t A, const fmpz_mod_mpoly_ctx_t ctx) { fmpz_mod_mpoly_zero(A, ctx); }
void fmpz_mod_mpoly_one__extern(fmpz_mod_mpoly_t A, const fmpz_mod_mpoly_ctx_t ctx) { fmpz_mod_mpoly_one(A, ctx); }
int fmpz_mod_mpoly_is_zero__extern(const fmpz_mod_mpoly_t A, const fmpz_mod_mpoly_ctx_t ctx) { return fmpz_mod_mpoly_is_zero(A, ctx); }
int fmpz_mod_mpoly_is_one__extern(const fmpz_mod_mpoly_t A, const fmpz_mod_mpoly_ctx_t ctx) { return fmpz_mod_mpoly_is_one(A, ctx); }
fmpz * fmpz_mod_mpoly_leadcoeff__extern(fmpz_mod_mpoly_t A) { return fmpz_mod_mpoly_leadcoeff(A); }
mp_limb_signed_t fmpz_mod_mpoly_length__extern(const fmpz_mod_mpoly_t A, const fmpz_mod_mpoly_ctx_t ctx) { return fmpz_mod_mpoly_length(A, ctx); }
void fmpz_mod_mpoly_divexact__extern(fmpz_mod_mpoly_t Q, const fmpz_mod_mpoly_t A, const fmpz_mod_mpoly_t B, const fmpz_mod_mpoly_ctx_t ctx) { fmpz_mod_mpoly_divexact(Q, A, B, ctx); }
int fmpz_mod_mpoly_sqrt__extern(fmpz_mod_mpoly_t Q, const fmpz_mod_mpoly_t A, const fmpz_mod_mpoly_ctx_t ctx) { return fmpz_mod_mpoly_sqrt(Q, A, ctx); }
int fmpz_mod_mpoly_is_square__extern(const fmpz_mod_mpoly_t A, const fmpz_mod_mpoly_ctx_t ctx) { return fmpz_mod_mpoly_is_square(A, ctx); }
void fmpz_mod_mpoly_univar_zero__extern(fmpz_mod_mpoly_univar_t A, const fmpz_mod_mpoly_ctx_t ctx) { fmpz_mod_mpoly_univar_zero(A, ctx); }
void fmpz_mod_mpoly_univar_swap__extern(fmpz_mod_mpoly_univar_t A, fmpz_mod_mpoly_univar_t B, const fmpz_mod_mpoly_ctx_t ctx) { fmpz_mod_mpoly_univar_swap(A, B, ctx); }
int fmpz_mod_mpoly_univar_degree_fits_si__extern(const fmpz_mod_mpoly_univar_t A, const fmpz_mod_mpoly_ctx_t ctx) { return fmpz_mod_mpoly_univar_degree_fits_si(A, ctx); }
mp_limb_signed_t fmpz_mod_mpoly_univar_length__extern(const fmpz_mod_mpoly_univar_t A, const fmpz_mod_mpoly_ctx_t ctx) { return fmpz_mod_mpoly_univar_length(A, ctx); }
mp_limb_signed_t fmpz_mod_mpoly_univar_get_term_exp_si__extern(fmpz_mod_mpoly_univar_t A, mp_limb_signed_t i, const fmpz_mod_mpoly_ctx_t ctx) { return fmpz_mod_mpoly_univar_get_term_exp_si(A, i, ctx); }
void fmpz_mod_mpoly_univar_get_term_coeff__extern(fmpz_mod_mpoly_t c, const fmpz_mod_mpoly_univar_t A, mp_limb_signed_t i, const fmpz_mod_mpoly_ctx_t ctx) { fmpz_mod_mpoly_univar_get_term_coeff(c, A, i, ctx); }
void fmpz_mod_mpoly_univar_swap_term_coeff__extern(fmpz_mod_mpoly_t c, fmpz_mod_mpoly_univar_t A, mp_limb_signed_t i, const fmpz_mod_mpoly_ctx_t ctx) { fmpz_mod_mpoly_univar_swap_term_coeff(c, A, i, ctx); }
void _fmpz_mod_mpoly_clear_dense_mock__extern(fmpz_mod_poly_t D) { _fmpz_mod_mpoly_clear_dense_mock(D); }

void fmpz_mod_mpoly_factor_init__extern(fmpz_mod_mpoly_factor_t f, const fmpz_mod_mpoly_ctx_t ctx) { fmpz_mod_mpoly_factor_init(f, ctx); }
mp_limb_signed_t fmpz_mod_mpoly_factor_length__extern(const fmpz_mod_mpoly_factor_t f, const fmpz_mod_mpoly_ctx_t ctx) { return fmpz_mod_mpoly_factor_length(f, ctx); }
void fmpz_mod_mpoly_factor_get_constant_fmpz__extern(fmpz_t c, const fmpz_mod_mpoly_factor_t f, const fmpz_mod_mpoly_ctx_t ctx) { fmpz_mod_mpoly_factor_get_constant_fmpz(c, f, ctx); }
void fmpz_mod_mpoly_factor_get_base__extern(fmpz_mod_mpoly_t p, const fmpz_mod_mpoly_factor_t f, mp_limb_signed_t i, const fmpz_mod_mpoly_ctx_t ctx) { fmpz_mod_mpoly_factor_get_base(p, f, i, ctx); }
void fmpz_mod_mpoly_factor_swap_base__extern(fmpz_mod_mpoly_t p, fmpz_mod_mpoly_factor_t f, mp_limb_signed_t i, const fmpz_mod_mpoly_ctx_t ctx) { fmpz_mod_mpoly_factor_swap_base(p, f, i, ctx); }
mp_limb_signed_t fmpz_mod_mpoly_factor_get_exp_si__extern(fmpz_mod_mpoly_factor_t f, mp_limb_signed_t i, const fmpz_mod_mpoly_ctx_t ctx) { return fmpz_mod_mpoly_factor_get_exp_si(f, i, ctx); }
void fmpz_mod_mpoly_factor_swap__extern(fmpz_mod_mpoly_factor_t f, fmpz_mod_mpoly_factor_t g, const fmpz_mod_mpoly_ctx_t ctx) { fmpz_mod_mpoly_factor_swap(f, g, ctx); }
mp_limb_signed_t _fmpz_mod_poly_degree__extern(const fmpz_mod_poly_t a) { return _fmpz_mod_poly_degree(a); }
int fmpz_mod_mpoly_factor_matches__extern(const fmpz_mod_mpoly_t a, const fmpz_mod_mpoly_factor_t f, const fmpz_mod_mpoly_ctx_t ctx) { return fmpz_mod_mpoly_factor_matches(a, f, ctx); }
void fmpz_mod_mpoly_factor_append_fmpz_swap__extern(fmpz_mod_mpoly_factor_t f, fmpz_mod_mpoly_t A, const fmpz_t e, const fmpz_mod_mpoly_ctx_t ctx) { fmpz_mod_mpoly_factor_append_fmpz_swap(f, A, e, ctx); }
void fmpz_mod_mpoly_factor_one__extern(fmpz_mod_mpoly_factor_t f, const fmpz_mod_mpoly_ctx_t ctx) { fmpz_mod_mpoly_factor_one(f, ctx); }
fmpz_mod_poly_struct ** fmpz_mod_poly_stack_request__extern(fmpz_mod_poly_stack_t S, mp_limb_signed_t k) { return fmpz_mod_poly_stack_request(S, k); }
fmpz_mod_poly_struct * fmpz_mod_poly_stack_take_top__extern(fmpz_mod_poly_stack_t S) { return fmpz_mod_poly_stack_take_top(S); }
void fmpz_mod_poly_stack_give_back__extern(fmpz_mod_poly_stack_t S, mp_limb_signed_t k) { fmpz_mod_poly_stack_give_back(S, k); }
mp_limb_signed_t fmpz_mod_poly_stack_size__extern(const fmpz_mod_poly_stack_t S) { return fmpz_mod_poly_stack_size(S); }
fmpz_mod_bpoly_struct ** fmpz_mod_bpoly_stack_request__extern(fmpz_mod_bpoly_stack_t S, mp_limb_signed_t k) { return fmpz_mod_bpoly_stack_request(S, k); }
fmpz_mod_bpoly_struct * fmpz_mod_bpoly_stack_take_top__extern(fmpz_mod_bpoly_stack_t S) { return fmpz_mod_bpoly_stack_take_top(S); }
void fmpz_mod_bpoly_stack_give_back__extern(fmpz_mod_bpoly_stack_t S, mp_limb_signed_t k) { fmpz_mod_bpoly_stack_give_back(S, k); }
mp_limb_signed_t fmpz_mod_bpoly_stack_size__extern(const fmpz_mod_bpoly_stack_t S) { return fmpz_mod_bpoly_stack_size(S); }
fmpz_mod_polyun_struct ** fmpz_mod_polyun_stack_request__extern(fmpz_mod_polyun_stack_t S, mp_limb_signed_t k) { return fmpz_mod_polyun_stack_request(S, k); }
fmpz_mod_polyun_struct * fmpz_mod_polyun_stack_take_top__extern(fmpz_mod_polyun_stack_t S) { return fmpz_mod_polyun_stack_take_top(S); }
void fmpz_mod_polyun_stack_give_back__extern(fmpz_mod_polyun_stack_t S, mp_limb_signed_t k) { fmpz_mod_polyun_stack_give_back(S, k); }
mp_limb_signed_t fmpz_mod_polyun_stack_size__extern(const fmpz_mod_polyun_stack_t S) { return fmpz_mod_polyun_stack_size(S); }
fmpz_mod_mpolyn_struct ** fmpz_mod_mpolyn_stack_request__extern(fmpz_mod_mpolyn_stack_t S, mp_limb_signed_t k, const fmpz_mod_mpoly_ctx_t ctx) { return fmpz_mod_mpolyn_stack_request(S, k, ctx); }
fmpz_mod_mpolyn_struct * fmpz_mod_mpolyn_stack_take_top__extern(fmpz_mod_mpolyn_stack_t S) { return fmpz_mod_mpolyn_stack_take_top(S); }
void fmpz_mod_mpolyn_stack_give_back__extern(fmpz_mod_mpolyn_stack_t S, mp_limb_signed_t k) { fmpz_mod_mpolyn_stack_give_back(S, k); }
mp_limb_signed_t fmpz_mod_mpolyn_stack_size__extern(const fmpz_mod_mpolyn_stack_t S) { return fmpz_mod_mpolyn_stack_size(S); }
mp_limb_t fmpz_mod_polyu1n_bidegree__extern(const fmpz_mod_polyun_t A) { return fmpz_mod_polyu1n_bidegree(A); }
const fmpz * fmpz_mod_polyun_leadcoeff__extern(const fmpz_mod_polyun_t A) { return fmpz_mod_polyun_leadcoeff(A); }
void fmpz_mod_polyun_swap__extern(fmpz_mod_polyun_t A, fmpz_mod_polyun_t B) { fmpz_mod_polyun_swap(A, B); }
void fmpz_mod_polyun_init__extern(fmpz_mod_polyun_t A, const fmpz_mod_ctx_t ctx) { fmpz_mod_polyun_init(A, ctx); }
void fmpz_mod_polyun_fit_length__extern(fmpz_mod_polyun_t A, mp_limb_signed_t len, const fmpz_mod_ctx_t ctx) { fmpz_mod_polyun_fit_length(A, len, ctx); }
void fmpz_mod_mpolyn_swap__extern(fmpz_mod_mpolyn_t A, fmpz_mod_mpolyn_t B, const fmpz_mod_mpoly_ctx_t ctx) { fmpz_mod_mpolyn_swap(A, B, ctx); }
const fmpz * fmpz_mod_mpolyn_leadcoeff__extern(const fmpz_mod_mpolyn_t A) { return fmpz_mod_mpolyn_leadcoeff(A); }
void fmpz_mod_polyu_swap__extern(fmpz_mod_polyu_t A, fmpz_mod_polyu_t B) { fmpz_mod_polyu_swap(A, B); }
void fmpz_mod_polyu_fit_length__extern(fmpz_mod_polyu_t a, mp_limb_signed_t len, const fmpz_mod_ctx_t ctx) { fmpz_mod_polyu_fit_length(a, len, ctx); }
void fmpz_mod_bpoly_init__extern(fmpz_mod_bpoly_t A, const fmpz_mod_ctx_t ctx) { fmpz_mod_bpoly_init(A, ctx); }
void fmpz_mod_bpoly_swap__extern(fmpz_mod_bpoly_t A, fmpz_mod_bpoly_t B, const fmpz_mod_ctx_t ctx) { fmpz_mod_bpoly_swap(A, B, ctx); }
void fmpz_mod_bpoly_get_coeff__extern(fmpz_t c, const fmpz_mod_bpoly_t A, mp_limb_signed_t e0, mp_limb_signed_t e1, const fmpz_mod_ctx_t ctx) { fmpz_mod_bpoly_get_coeff(c, A, e0, e1, ctx); }
mp_limb_signed_t fmpz_mod_bpoly_degree0__extern(const fmpz_mod_bpoly_t A, const fmpz_mod_ctx_t ctx) { return fmpz_mod_bpoly_degree0(A, ctx); }
void fmpz_mod_bpoly_normalise__extern(fmpz_mod_bpoly_t A, const fmpz_mod_ctx_t ctx) { fmpz_mod_bpoly_normalise(A, ctx); }
int fmpz_mod_bpoly_is_one__extern(const fmpz_mod_bpoly_t A, const fmpz_mod_ctx_t ctx) { return fmpz_mod_bpoly_is_one(A, ctx); }
void fmpz_mod_tpoly_init__extern(fmpz_mod_tpoly_t A, const fmpz_mod_ctx_t ctx) { fmpz_mod_tpoly_init(A, ctx); }
void fmpz_mod_tpoly_swap__extern(fmpz_mod_tpoly_t A, fmpz_mod_tpoly_t B, const fmpz_mod_ctx_t ctx) { fmpz_mod_tpoly_swap(A, B, ctx); }
void fmpz_mod_mpolyv_init__extern(fmpz_mod_mpolyv_t A, const fmpz_mod_mpoly_ctx_t ctx) { fmpz_mod_mpolyv_init(A, ctx); }
void fmpz_mod_mpolyv_swap__extern(fmpz_mod_mpolyv_t A, fmpz_mod_mpolyv_t B, const fmpz_mod_mpoly_ctx_t ctx) { fmpz_mod_mpolyv_swap(A, B, ctx); }

void fmpz_mod_poly_init__extern(fmpz_mod_poly_t poly, const fmpz_mod_ctx_t ctx) { fmpz_mod_poly_init(poly, ctx); }
void fmpz_mod_poly_fit_length__extern(fmpz_mod_poly_t poly, mp_limb_signed_t len, const fmpz_mod_ctx_t ctx) { fmpz_mod_poly_fit_length(poly, len, ctx); }
void _fmpz_mod_poly_normalise__extern(fmpz_mod_poly_t poly) { _fmpz_mod_poly_normalise(poly); }
mp_limb_signed_t fmpz_mod_poly_length__extern(const fmpz_mod_poly_t poly, const fmpz_mod_ctx_t ctx) { return fmpz_mod_poly_length(poly, ctx); }
mp_limb_signed_t fmpz_mod_poly_degree__extern(const fmpz_mod_poly_t poly, const fmpz_mod_ctx_t ctx) { return fmpz_mod_poly_degree(poly, ctx); }
fmpz * fmpz_mod_poly_lead__extern(const fmpz_mod_poly_t poly, const fmpz_mod_ctx_t ctx) { return fmpz_mod_poly_lead(poly, ctx); }
int fmpz_mod_poly_is_monic__extern(const fmpz_mod_poly_t f, const fmpz_mod_ctx_t ctx) { return fmpz_mod_poly_is_monic(f, ctx); }
int fmpz_mod_poly_is_one__extern(const fmpz_mod_poly_t poly, const fmpz_mod_ctx_t ctx) { return fmpz_mod_poly_is_one(poly, ctx); }
int fmpz_mod_poly_is_gen__extern(const fmpz_mod_poly_t op, const fmpz_mod_ctx_t ctx) { return fmpz_mod_poly_is_gen(op, ctx); }
void fmpz_mod_poly_swap__extern(fmpz_mod_poly_t poly1, fmpz_mod_poly_t poly2, const fmpz_mod_ctx_t ctx) { fmpz_mod_poly_swap(poly1, poly2, ctx); }
void fmpz_mod_poly_zero__extern(fmpz_mod_poly_t poly, const fmpz_mod_ctx_t ctx) { fmpz_mod_poly_zero(poly, ctx); }
mp_limb_signed_t _fmpz_mod_poly_gcd_f__extern(fmpz_t f, fmpz *G, const fmpz *A, mp_limb_signed_t lenA, const fmpz *B, mp_limb_signed_t lenB, const fmpz_mod_ctx_t ctx) { return _fmpz_mod_poly_gcd_f(f, G, A, lenA, B, lenB, ctx); }
void fmpz_mod_poly_gcd_f__extern(fmpz_t f, fmpz_mod_poly_t G, const fmpz_mod_poly_t A, const fmpz_mod_poly_t B, const fmpz_mod_ctx_t ctx) { fmpz_mod_poly_gcd_f(f, G, A, B, ctx); }
mp_limb_signed_t _fmpz_mod_poly_xgcd_f__extern(fmpz_t f, fmpz *G, fmpz *S, fmpz *T, const fmpz *A, mp_limb_signed_t lenA, const fmpz *B, mp_limb_signed_t lenB, const fmpz_t invB, const fmpz_mod_ctx_t ctx) { return _fmpz_mod_poly_xgcd_f(f, G, S, T, A, lenA, B, lenB, invB, ctx); }
void fmpz_mod_poly_xgcd_f__extern(fmpz_t f, fmpz_mod_poly_t G, fmpz_mod_poly_t S, fmpz_mod_poly_t T, const fmpz_mod_poly_t A, const fmpz_mod_poly_t B, const fmpz_mod_ctx_t ctx) { fmpz_mod_poly_xgcd_f(f, G, S, T, A, B, ctx); }
void fmpz_mod_mat_charpoly__extern(fmpz_mod_poly_t p, const fmpz_mod_mat_t M, const fmpz_mod_ctx_t ctx) { fmpz_mod_mat_charpoly(p, M, ctx); }
const fmpz * fmpz_mod_berlekamp_massey_points__extern(const fmpz_mod_berlekamp_massey_t B) { return fmpz_mod_berlekamp_massey_points(B); }
mp_limb_signed_t fmpz_mod_berlekamp_massey_point_count__extern(const fmpz_mod_berlekamp_massey_t B) { return fmpz_mod_berlekamp_massey_point_count(B); }
const fmpz_mod_poly_struct * fmpz_mod_berlekamp_massey_V_poly__extern(const fmpz_mod_berlekamp_massey_t B) { return fmpz_mod_berlekamp_massey_V_poly(B); }
const fmpz_mod_poly_struct * fmpz_mod_berlekamp_massey_R_poly__extern(const fmpz_mod_berlekamp_massey_t B) { return fmpz_mod_berlekamp_massey_R_poly(B); }

void fmpz_mod_poly_factor_swap__extern(fmpz_mod_poly_factor_t a, fmpz_mod_poly_factor_t b, const fmpz_mod_ctx_t ctx) { fmpz_mod_poly_factor_swap(a, b, ctx); }

fmpz * fmpz_mpoly_term_coeff_ref__extern(fmpz_mpoly_t A, mp_limb_signed_t i, const fmpz_mpoly_ctx_t ctx) { return fmpz_mpoly_term_coeff_ref(A, i, ctx); }
fmpz * fmpz_mpoly_leadcoeff__extern(const fmpz_mpoly_t A) { return fmpz_mpoly_leadcoeff(A); }
mp_limb_signed_t fmpz_mpoly_ctx_nvars__extern(const fmpz_mpoly_ctx_t ctx) { return fmpz_mpoly_ctx_nvars(ctx); }
ordering_t fmpz_mpoly_ctx_ord__extern(const fmpz_mpoly_ctx_t ctx) { return fmpz_mpoly_ctx_ord(ctx); }
void fmpz_mpoly_swap__extern(fmpz_mpoly_t A, fmpz_mpoly_t B, const fmpz_mpoly_ctx_t ctx) { fmpz_mpoly_swap(A, B, ctx); }
int _fmpz_mpoly_fits_small__extern(const fmpz *poly, mp_limb_signed_t len) { return _fmpz_mpoly_fits_small(poly, len); }
void fmpz_mpoly_zero__extern(fmpz_mpoly_t A, const fmpz_mpoly_ctx_t ctx) { fmpz_mpoly_zero(A, ctx); }
void fmpz_mpoly_one__extern(fmpz_mpoly_t A, const fmpz_mpoly_ctx_t ctx) { fmpz_mpoly_one(A, ctx); }
int fmpz_mpoly_is_zero__extern(const fmpz_mpoly_t A, const fmpz_mpoly_ctx_t ctx) { return fmpz_mpoly_is_zero(A, ctx); }
int fmpz_mpoly_is_one__extern(const fmpz_mpoly_t A, const fmpz_mpoly_ctx_t ctx) { return fmpz_mpoly_is_one(A, ctx); }
mp_limb_signed_t fmpz_mpoly_length__extern(const fmpz_mpoly_t A, const fmpz_mpoly_ctx_t ctx) { return fmpz_mpoly_length(A, ctx); }
void fmpz_mpoly_divexact__extern(fmpz_mpoly_t Q, const fmpz_mpoly_t A, const fmpz_mpoly_t B, const fmpz_mpoly_ctx_t ctx) { fmpz_mpoly_divexact(Q, A, B, ctx); }
int fmpz_mpoly_sqrt__extern(fmpz_mpoly_t q, const fmpz_mpoly_t poly2, const fmpz_mpoly_ctx_t ctx) { return fmpz_mpoly_sqrt(q, poly2, ctx); }
int fmpz_mpoly_is_square__extern(const fmpz_mpoly_t poly2, const fmpz_mpoly_ctx_t ctx) { return fmpz_mpoly_is_square(poly2, ctx); }
void fmpz_mpoly_univar_zero__extern(fmpz_mpoly_univar_t A, const fmpz_mpoly_ctx_t ctx) { fmpz_mpoly_univar_zero(A, ctx); }
void fmpz_mpoly_univar_swap__extern(fmpz_mpoly_univar_t A, fmpz_mpoly_univar_t B, const fmpz_mpoly_ctx_t ctx) { fmpz_mpoly_univar_swap(A, B, ctx); }
mp_limb_signed_t fmpz_mpoly_univar_length__extern(const fmpz_mpoly_univar_t A, const fmpz_mpoly_ctx_t ctx) { return fmpz_mpoly_univar_length(A, ctx); }
void fmpz_mpoly_univar_get_term_coeff__extern(fmpz_mpoly_t c, const fmpz_mpoly_univar_t A, mp_limb_signed_t i, const fmpz_mpoly_ctx_t ctx) { fmpz_mpoly_univar_get_term_coeff(c, A, i, ctx); }
void fmpz_mpoly_univar_swap_term_coeff__extern(fmpz_mpoly_t c, fmpz_mpoly_univar_t A, mp_limb_signed_t i, const fmpz_mpoly_ctx_t ctx) { fmpz_mpoly_univar_swap_term_coeff(c, A, i, ctx); }
void _fmpz_mpoly_submul_uiuiui_fmpz__extern(mp_limb_t *c, mp_limb_signed_t d1, mp_limb_signed_t d2) { _fmpz_mpoly_submul_uiuiui_fmpz(c, d1, d2); }
void _fmpz_mpoly_addmul_uiuiui_fmpz__extern(mp_limb_t *c, mp_limb_signed_t d1, mp_limb_signed_t d2) { _fmpz_mpoly_addmul_uiuiui_fmpz(c, d1, d2); }

void fmpz_mpoly_factor_init__extern(fmpz_mpoly_factor_t f, const fmpz_mpoly_ctx_t ctx) { fmpz_mpoly_factor_init(f, ctx); }
mp_limb_signed_t fmpz_mpoly_factor_length__extern(const fmpz_mpoly_factor_t f, const fmpz_mpoly_ctx_t ctx) { return fmpz_mpoly_factor_length(f, ctx); }
void fmpz_mpoly_factor_get_base__extern(fmpz_mpoly_t p, const fmpz_mpoly_factor_t f, mp_limb_signed_t i, const fmpz_mpoly_ctx_t ctx) { fmpz_mpoly_factor_get_base(p, f, i, ctx); }
void fmpz_mpoly_factor_swap_base__extern(fmpz_mpoly_t p, fmpz_mpoly_factor_t f, mp_limb_signed_t i, const fmpz_mpoly_ctx_t ctx) { fmpz_mpoly_factor_swap_base(p, f, i, ctx); }
void fmpz_mpoly_factor_swap__extern(fmpz_mpoly_factor_t f, fmpz_mpoly_factor_t g, const fmpz_mpoly_ctx_t ctx) { fmpz_mpoly_factor_swap(f, g, ctx); }
int fmpz_mpoly_factor_matches__extern(const fmpz_mpoly_t A, const fmpz_mpoly_factor_t f, const fmpz_mpoly_ctx_t ctx) { return fmpz_mpoly_factor_matches(A, f, ctx); }
void fmpz_mpolyv_init__extern(fmpz_mpolyv_t A, const fmpz_mpoly_ctx_t ctx) { fmpz_mpolyv_init(A, ctx); }
void fmpz_mpolyv_swap__extern(fmpz_mpolyv_t A, fmpz_mpolyv_t B, const fmpz_mpoly_ctx_t ctx) { fmpz_mpolyv_swap(A, B, ctx); }
void fmpz_bpoly_init__extern(fmpz_bpoly_t A) { fmpz_bpoly_init(A); }
void fmpz_bpoly_swap__extern(fmpz_bpoly_t A, fmpz_bpoly_t B) { fmpz_bpoly_swap(A, B); }
void fmpz_bpoly_fit_length__extern(fmpz_bpoly_t A, mp_limb_signed_t len) { fmpz_bpoly_fit_length(A, len); }
fmpz_poly_struct * fmpz_bpoly_lead__extern(fmpz_bpoly_t A) { return fmpz_bpoly_lead(A); }
void fmpz_bpoly_zero__extern(fmpz_bpoly_t A) { fmpz_bpoly_zero(A); }
mp_limb_signed_t fmpz_bpoly_degree0__extern(const fmpz_bpoly_t A) { return fmpz_bpoly_degree0(A); }
void fmpz_tpoly_init__extern(fmpz_tpoly_t A) { fmpz_tpoly_init(A); }
void fmpz_tpoly_swap__extern(fmpz_tpoly_t A, fmpz_tpoly_t B) { fmpz_tpoly_swap(A, B); }

int fmpz_mpoly_q_is_zero__extern(const fmpz_mpoly_q_t x, const fmpz_mpoly_ctx_t ctx) { return fmpz_mpoly_q_is_zero(x, ctx); }
int fmpz_mpoly_q_is_one__extern(const fmpz_mpoly_q_t x, const fmpz_mpoly_ctx_t ctx) { return fmpz_mpoly_q_is_one(x, ctx); }
int fmpz_mpoly_q_is_fmpz__extern(const fmpz_mpoly_q_t x, const fmpz_mpoly_ctx_t ctx) { return fmpz_mpoly_q_is_fmpz(x, ctx); }
int fmpz_mpoly_q_is_fmpq__extern(const fmpz_mpoly_q_t x, const fmpz_mpoly_ctx_t ctx) { return fmpz_mpoly_q_is_fmpq(x, ctx); }
void fmpz_mpoly_q_zero__extern(fmpz_mpoly_q_t res, const fmpz_mpoly_ctx_t ctx) { fmpz_mpoly_q_zero(res, ctx); }
void fmpz_mpoly_q_one__extern(fmpz_mpoly_q_t res, const fmpz_mpoly_ctx_t ctx) { fmpz_mpoly_q_one(res, ctx); }
void fmpz_mpoly_q_gen__extern(fmpz_mpoly_q_t res, mp_limb_signed_t i, const fmpz_mpoly_ctx_t ctx) { fmpz_mpoly_q_gen(res, i, ctx); }
void fmpz_mpoly_q_add_si__extern(fmpz_mpoly_q_t res, const fmpz_mpoly_q_t x, mp_limb_signed_t c, const fmpz_mpoly_ctx_t ctx) { fmpz_mpoly_q_add_si(res, x, c, ctx); }
void fmpz_mpoly_q_sub_si__extern(fmpz_mpoly_q_t res, const fmpz_mpoly_q_t x, mp_limb_signed_t c, const fmpz_mpoly_ctx_t ctx) { fmpz_mpoly_q_sub_si(res, x, c, ctx); }
void fmpz_mpoly_q_mul_si__extern(fmpz_mpoly_q_t res, const fmpz_mpoly_q_t x, mp_limb_signed_t c, const fmpz_mpoly_ctx_t ctx) { fmpz_mpoly_q_mul_si(res, x, c, ctx); }
void fmpz_mpoly_q_div_si__extern(fmpz_mpoly_q_t res, const fmpz_mpoly_q_t x, mp_limb_signed_t c, const fmpz_mpoly_ctx_t ctx) { fmpz_mpoly_q_div_si(res, x, c, ctx); }
void _fmpz_vec_content2__extern(fmpz_t res, const fmpz *vec, mp_limb_signed_t len, const fmpz_t inp) { _fmpz_vec_content2(res, vec, len, inp); }
void fmpz_mpoly_gcd_assert_successful__extern(fmpz_mpoly_t res, const fmpz_mpoly_t x, const fmpz_mpoly_t y, const fmpz_mpoly_ctx_t ctx) { fmpz_mpoly_gcd_assert_successful(res, x, y, ctx); }
void _fmpz_mpoly_q_mpoly_divexact__extern(fmpz_mpoly_t res, const fmpz_mpoly_t x, const fmpz_mpoly_t y, const fmpz_mpoly_ctx_t ctx) { _fmpz_mpoly_q_mpoly_divexact(res, x, y, ctx); }
void _fmpz_mpoly_q_content__extern(fmpz_t num, fmpz_t den, const fmpz_mpoly_t xnum, const fmpz_mpoly_t xden, const fmpz_mpoly_ctx_t ctx) { _fmpz_mpoly_q_content(num, den, xnum, xden, ctx); }
void fmpz_mpoly_q_content__extern(fmpq_t res, const fmpz_mpoly_q_t x, const fmpz_mpoly_ctx_t ctx) { fmpz_mpoly_q_content(res, x, ctx); }

void fmpz_poly_attach_truncate__extern(fmpz_poly_t trunc, const fmpz_poly_t poly, mp_limb_signed_t n) { fmpz_poly_attach_truncate(trunc, poly, n); }
void fmpz_poly_attach_shift__extern(fmpz_poly_t trunc, const fmpz_poly_t poly, mp_limb_signed_t n) { fmpz_poly_attach_shift(trunc, poly, n); }
mp_limb_signed_t fmpz_poly_length__extern(const fmpz_poly_t poly) { return fmpz_poly_length(poly); }
mp_limb_signed_t fmpz_poly_degree__extern(const fmpz_poly_t poly) { return fmpz_poly_degree(poly); }
void fmpz_poly_zero__extern(fmpz_poly_t poly) { fmpz_poly_zero(poly); }
void fmpz_poly_one__extern(fmpz_poly_t poly) { fmpz_poly_one(poly); }
mp_limb_t fmpz_poly_deflation__extern(const fmpz_poly_t input) { return fmpz_poly_deflation(input); }
int fmpz_poly_is_one__extern(const fmpz_poly_t op) { return fmpz_poly_is_one(op); }
int fmpz_poly_is_unit__extern(const fmpz_poly_t op) { return fmpz_poly_is_unit(op); }
int fmpz_poly_is_gen__extern(const fmpz_poly_t op) { return fmpz_poly_is_gen(op); }
void fmpz_poly_mul_SS_precache__extern(fmpz_poly_t res, const fmpz_poly_t poly1, fmpz_poly_mul_precache_t pre) { fmpz_poly_mul_SS_precache(res, poly1, pre); }
void _fmpz_poly_xgcd__extern(fmpz_t r, fmpz *s, fmpz *t, const fmpz *poly1, mp_limb_signed_t len1, const fmpz *poly2, mp_limb_signed_t len2) { _fmpz_poly_xgcd(r, s, t, poly1, len1, poly2, len2); }
void fmpz_poly_xgcd__extern(fmpz_t r, fmpz_poly_t s, fmpz_poly_t t, const fmpz_poly_t poly1, const fmpz_poly_t poly2) { fmpz_poly_xgcd(r, s, t, poly1, poly2); }
void fmpz_poly_pseudo_divrem__extern(fmpz_poly_t Q, fmpz_poly_t R, mp_limb_t *d, const fmpz_poly_t A, const fmpz_poly_t B) { fmpz_poly_pseudo_divrem(Q, R, d, A, B); }
void fmpz_poly_taylor_shift_multi_mod__extern(fmpz_poly_t g, const fmpz_poly_t f, const fmpz_t c) { fmpz_poly_taylor_shift_multi_mod(g, f, c); }

void zassenhaus_prune_init__extern(zassenhaus_prune_t Z) { zassenhaus_prune_init(Z); }
void zassenhaus_prune_start_add_factors__extern(zassenhaus_prune_t Z) { zassenhaus_prune_start_add_factors(Z); }
int zassenhaus_prune_degree_is_possible__extern(const zassenhaus_prune_t Z, mp_limb_signed_t d) { return zassenhaus_prune_degree_is_possible(Z, d); }

fmpz_poly_struct * fmpz_poly_mat_entry__extern(const fmpz_poly_mat_t mat, mp_limb_signed_t i, mp_limb_signed_t j) { return fmpz_poly_mat_entry(mat, i, j); }
mp_limb_signed_t fmpz_poly_mat_nrows__extern(const fmpz_poly_mat_t mat) { return fmpz_poly_mat_nrows(mat); }
mp_limb_signed_t fmpz_poly_mat_ncols__extern(const fmpz_poly_mat_t mat) { return fmpz_poly_mat_ncols(mat); }
void fmpz_poly_mat_swap_entrywise__extern(fmpz_poly_mat_t mat1, fmpz_poly_mat_t mat2) { fmpz_poly_mat_swap_entrywise(mat1, mat2); }
int fmpz_poly_mat_is_empty__extern(const fmpz_poly_mat_t mat) { return fmpz_poly_mat_is_empty(mat); }
int fmpz_poly_mat_is_square__extern(const fmpz_poly_mat_t mat) { return fmpz_poly_mat_is_square(mat); }

void fmpz_poly_q_zero__extern(fmpz_poly_q_t rop) { fmpz_poly_q_zero(rop); }
void fmpz_poly_q_one__extern(fmpz_poly_q_t rop) { fmpz_poly_q_one(rop); }
void fmpz_poly_q_neg__extern(fmpz_poly_q_t rop, const fmpz_poly_q_t op) { fmpz_poly_q_neg(rop, op); }
int fmpz_poly_q_is_zero__extern(const fmpz_poly_q_t op) { return fmpz_poly_q_is_zero(op); }
int fmpz_poly_q_is_one__extern(const fmpz_poly_q_t op) { return fmpz_poly_q_is_one(op); }
int fmpz_poly_q_equal__extern(const fmpz_poly_q_t op1, const fmpz_poly_q_t op2) { return fmpz_poly_q_equal(op1, op2); }

fmpz * _fmpz_vec_init__extern(mp_limb_signed_t len) { return _fmpz_vec_init(len); }
void _fmpz_vec_dot__extern(fmpz_t res, const fmpz *vec1, const fmpz *vec2, mp_limb_signed_t len2) { _fmpz_vec_dot(res, vec1, vec2, len2); }

void fmpzi_init__extern(fmpzi_t x) { fmpzi_init(x); }
void fmpzi_clear__extern(fmpzi_t x) { fmpzi_clear(x); }
int fmpzi_equal__extern(const fmpzi_t x, const fmpzi_t y) { return fmpzi_equal(x, y); }
void fmpzi_zero__extern(fmpzi_t x) { fmpzi_zero(x); }
void fmpzi_one__extern(fmpzi_t x) { fmpzi_one(x); }
void fmpzi_set__extern(fmpzi_t res, const fmpzi_t x) { fmpzi_set(res, x); }
void fmpzi_conj__extern(fmpzi_t res, const fmpzi_t x) { fmpzi_conj(res, x); }
void fmpzi_swap__extern(fmpzi_t x, fmpzi_t y) { fmpzi_swap(x, y); }
void fmpzi_set_si_si__extern(fmpzi_t res, mp_limb_signed_t a, mp_limb_signed_t b) { fmpzi_set_si_si(res, a, b); }
void fmpzi_randtest__extern(fmpzi_t res, flint_rand_t state, mp_bitcnt_t bits) { fmpzi_randtest(res, state, bits); }
int fmpzi_is_unit__extern(const fmpzi_t x) { return fmpzi_is_unit(x); }
int fmpzi_is_zero__extern(const fmpzi_t x) { return fmpzi_is_zero(x); }
int fmpzi_is_one__extern(const fmpzi_t x) { return fmpzi_is_one(x); }
void fmpzi_norm__extern(fmpz_t res, const fmpzi_t x) { fmpzi_norm(res, x); }
void fmpzi_neg__extern(fmpzi_t res, const fmpzi_t x) { fmpzi_neg(res, x); }
void fmpzi_add__extern(fmpzi_t res, const fmpzi_t x, const fmpzi_t y) { fmpzi_add(res, x, y); }
void fmpzi_sub__extern(fmpzi_t res, const fmpzi_t x, const fmpzi_t y) { fmpzi_sub(res, x, y); }
void fmpzi_canonicalise_unit__extern(fmpzi_t res, const fmpzi_t x) { fmpzi_canonicalise_unit(res, x); }

const fmpz_mod_poly_struct * fq_ctx_modulus__extern(const fq_ctx_t ctx) { return fq_ctx_modulus(ctx); }
mp_limb_signed_t fq_ctx_degree__extern(const fq_ctx_t ctx) { return fq_ctx_degree(ctx); }
const fmpz * fq_ctx_prime__extern(const fq_ctx_t ctx) { return fq_ctx_prime(ctx); }

void fq_default_ctx_init__extern(fq_default_ctx_t ctx, const fmpz_t p, mp_limb_signed_t d, const char *var) { fq_default_ctx_init(ctx, p, d, var); }
void * fq_default_ctx_inner__extern(const fq_default_ctx_t ctx) { return fq_default_ctx_inner(ctx); }
void fq_default_ctx_clear__extern(fq_default_ctx_t ctx) { fq_default_ctx_clear(ctx); }
int fq_default_ctx_type__extern(const fq_default_ctx_t ctx) { return fq_default_ctx_type(ctx); }
mp_limb_signed_t fq_default_ctx_degree__extern(const fq_default_ctx_t ctx) { return fq_default_ctx_degree(ctx); }
void fq_default_ctx_prime__extern(fmpz_t prime, const fq_default_ctx_t ctx) { fq_default_ctx_prime(prime, ctx); }
void fq_default_ctx_order__extern(fmpz_t f, const fq_default_ctx_t ctx) { fq_default_ctx_order(f, ctx); }
void fq_default_init__extern(fq_default_t rop, const fq_default_ctx_t ctx) { fq_default_init(rop, ctx); }
void fq_default_init2__extern(fq_default_t rop, const fq_default_ctx_t ctx) { fq_default_init2(rop, ctx); }
void fq_default_clear__extern(fq_default_t rop, const fq_default_ctx_t ctx) { fq_default_clear(rop, ctx); }
void fq_default_add__extern(fq_default_t rop, const fq_default_t op1, const fq_default_t op2, const fq_default_ctx_t ctx) { fq_default_add(rop, op1, op2, ctx); }
void fq_default_sub__extern(fq_default_t rop, const fq_default_t op1, const fq_default_t op2, const fq_default_ctx_t ctx) { fq_default_sub(rop, op1, op2, ctx); }
void fq_default_sub_one__extern(fq_default_t rop, const fq_default_t op1, const fq_default_ctx_t ctx) { fq_default_sub_one(rop, op1, ctx); }
void fq_default_neg__extern(fq_default_t rop, const fq_default_t op1, const fq_default_ctx_t ctx) { fq_default_neg(rop, op1, ctx); }
void fq_default_mul__extern(fq_default_t rop, const fq_default_t op1, const fq_default_t op2, const fq_default_ctx_t ctx) { fq_default_mul(rop, op1, op2, ctx); }
void fq_default_mul_fmpz__extern(fq_default_t rop, const fq_default_t op, const fmpz_t x, const fq_default_ctx_t ctx) { fq_default_mul_fmpz(rop, op, x, ctx); }
void fq_default_mul_si__extern(fq_default_t rop, const fq_default_t op, mp_limb_signed_t x, const fq_default_ctx_t ctx) { fq_default_mul_si(rop, op, x, ctx); }
void fq_default_mul_ui__extern(fq_default_t rop, const fq_default_t op, mp_limb_t x, const fq_default_ctx_t ctx) { fq_default_mul_ui(rop, op, x, ctx); }
void fq_default_sqr__extern(fq_default_t rop, const fq_default_t op, const fq_default_ctx_t ctx) { fq_default_sqr(rop, op, ctx); }
int fq_default_is_invertible__extern(const fq_default_t op, const fq_default_ctx_t ctx) { return fq_default_is_invertible(op, ctx); }
void fq_default_inv__extern(fq_default_t rop, const fq_default_t op1, const fq_default_ctx_t ctx) { fq_default_inv(rop, op1, ctx); }
void fq_default_div__extern(fq_default_t rop, fq_default_t op1, fq_default_t op2, const fq_default_ctx_t ctx) { fq_default_div(rop, op1, op2, ctx); }
void fq_default_pow__extern(fq_default_t rop, const fq_default_t op1, const fmpz_t e, const fq_default_ctx_t ctx) { fq_default_pow(rop, op1, e, ctx); }
void fq_default_pow_ui__extern(fq_default_t rop, const fq_default_t op, const mp_limb_t e, const fq_default_ctx_t ctx) { fq_default_pow_ui(rop, op, e, ctx); }
int fq_default_is_square__extern(const fq_default_t op, const fq_default_ctx_t ctx) { return fq_default_is_square(op, ctx); }
int fq_default_sqrt__extern(fq_default_t rop, const fq_default_t op, const fq_default_ctx_t ctx) { return fq_default_sqrt(rop, op, ctx); }
void fq_default_pth_root__extern(fq_default_t rop, const fq_default_t op1, const fq_default_ctx_t ctx) { fq_default_pth_root(rop, op1, ctx); }
void fq_default_randtest__extern(fq_default_t rop, flint_rand_t state, const fq_default_ctx_t ctx) { fq_default_randtest(rop, state, ctx); }
void fq_default_randtest_not_zero__extern(fq_default_t rop, flint_rand_t state, const fq_default_ctx_t ctx) { fq_default_randtest_not_zero(rop, state, ctx); }
void fq_default_rand__extern(fq_default_t rop, flint_rand_t state, const fq_default_ctx_t ctx) { fq_default_rand(rop, state, ctx); }
void fq_default_rand_not_zero__extern(fq_default_t rop, flint_rand_t state, const fq_default_ctx_t ctx) { fq_default_rand_not_zero(rop, state, ctx); }
int fq_default_equal__extern(const fq_default_t op1, const fq_default_t op2, const fq_default_ctx_t ctx) { return fq_default_equal(op1, op2, ctx); }
int fq_default_is_zero__extern(const fq_default_t op, const fq_default_ctx_t ctx) { return fq_default_is_zero(op, ctx); }
int fq_default_is_one__extern(const fq_default_t op, const fq_default_ctx_t ctx) { return fq_default_is_one(op, ctx); }
void fq_default_set__extern(fq_default_t rop, const fq_default_t op, const fq_default_ctx_t ctx) { fq_default_set(rop, op, ctx); }
void fq_default_set_fmpz__extern(fq_default_t rop, const fmpz_t x, const fq_default_ctx_t ctx) { fq_default_set_fmpz(rop, x, ctx); }
void fq_default_set_ui__extern(fq_default_t rop, const mp_limb_t x, const fq_default_ctx_t ctx) { fq_default_set_ui(rop, x, ctx); }
void fq_default_set_si__extern(fq_default_t rop, const mp_limb_signed_t x, const fq_default_ctx_t ctx) { fq_default_set_si(rop, x, ctx); }
void fq_default_zero__extern(fq_default_t rop, const fq_default_ctx_t ctx) { fq_default_zero(rop, ctx); }
void fq_default_one__extern(fq_default_t rop, const fq_default_ctx_t ctx) { fq_default_one(rop, ctx); }
void fq_default_swap__extern(fq_default_t op1, fq_default_t op2, const fq_default_ctx_t ctx) { fq_default_swap(op1, op2, ctx); }
void fq_default_gen__extern(fq_default_t rop, const fq_default_ctx_t ctx) { fq_default_gen(rop, ctx); }
void fq_default_get_nmod_poly__extern(nmod_poly_t poly, const fq_default_t op, const fq_default_ctx_t ctx) { fq_default_get_nmod_poly(poly, op, ctx); }
void fq_default_set_nmod_poly__extern(fq_default_t op, const nmod_poly_t poly, const fq_default_ctx_t ctx) { fq_default_set_nmod_poly(op, poly, ctx); }
int fq_default_get_fmpz__extern(fmpz_t z, const fq_default_t op, const fq_default_ctx_t ctx) { return fq_default_get_fmpz(z, op, ctx); }
void fq_default_get_coeff_fmpz__extern(fmpz_t c, fq_default_t op, mp_limb_signed_t n, const fq_default_ctx_t ctx) { fq_default_get_coeff_fmpz(c, op, n, ctx); }
char * fq_default_get_str__extern(const fq_default_t op, const fq_default_ctx_t ctx) { return fq_default_get_str(op, ctx); }
char * fq_default_get_str_pretty__extern(const fq_default_t op, const fq_default_ctx_t ctx) { return fq_default_get_str_pretty(op, ctx); }
void fq_default_trace__extern(fmpz_t rop, const fq_default_t op, const fq_default_ctx_t ctx) { fq_default_trace(rop, op, ctx); }
void fq_default_frobenius__extern(fq_default_t rop, const fq_default_t op, mp_limb_signed_t e, const fq_default_ctx_t ctx) { fq_default_frobenius(rop, op, e, ctx); }
void fq_default_norm__extern(fmpz_t rop, const fq_default_t op, const fq_default_ctx_t ctx) { fq_default_norm(rop, op, ctx); }

void fq_default_mat_init__extern(fq_default_mat_t mat, mp_limb_signed_t rows, mp_limb_signed_t cols, const fq_default_ctx_t ctx) { fq_default_mat_init(mat, rows, cols, ctx); }
void fq_default_mat_init_set__extern(fq_default_mat_t mat, const fq_default_mat_t src, const fq_default_ctx_t ctx) { fq_default_mat_init_set(mat, src, ctx); }
void fq_default_mat_swap__extern(fq_default_mat_t mat1, fq_default_mat_t mat2, const fq_default_ctx_t ctx) { fq_default_mat_swap(mat1, mat2, ctx); }
void fq_default_mat_set__extern(fq_default_mat_t mat1, const fq_default_mat_t mat2, const fq_default_ctx_t ctx) { fq_default_mat_set(mat1, mat2, ctx); }
void fq_default_mat_clear__extern(fq_default_mat_t mat, const fq_default_ctx_t ctx) { fq_default_mat_clear(mat, ctx); }
int fq_default_mat_equal__extern(const fq_default_mat_t mat1, const fq_default_mat_t mat2, const fq_default_ctx_t ctx) { return fq_default_mat_equal(mat1, mat2, ctx); }
int fq_default_mat_is_zero__extern(const fq_default_mat_t mat, const fq_default_ctx_t ctx) { return fq_default_mat_is_zero(mat, ctx); }
int fq_default_mat_is_one__extern(const fq_default_mat_t mat, const fq_default_ctx_t ctx) { return fq_default_mat_is_one(mat, ctx); }
int fq_default_mat_is_empty__extern(const fq_default_mat_t mat, const fq_default_ctx_t ctx) { return fq_default_mat_is_empty(mat, ctx); }
int fq_default_mat_is_square__extern(const fq_default_mat_t mat, const fq_default_ctx_t ctx) { return fq_default_mat_is_square(mat, ctx); }
void fq_default_mat_entry__extern(fq_default_t val, const fq_default_mat_t mat, mp_limb_signed_t i, mp_limb_signed_t j, const fq_default_ctx_t ctx) { fq_default_mat_entry(val, mat, i, j, ctx); }
void fq_default_mat_entry_set__extern(fq_default_mat_t mat, mp_limb_signed_t i, mp_limb_signed_t j, const fq_default_t x, const fq_default_ctx_t ctx) { fq_default_mat_entry_set(mat, i, j, x, ctx); }
void fq_default_mat_entry_set_fmpz__extern(fq_default_mat_t mat, mp_limb_signed_t i, mp_limb_signed_t j, const fmpz_t x, const fq_default_ctx_t ctx) { fq_default_mat_entry_set_fmpz(mat, i, j, x, ctx); }
mp_limb_signed_t fq_default_mat_nrows__extern(const fq_default_mat_t mat, const fq_default_ctx_t ctx) { return fq_default_mat_nrows(mat, ctx); }
mp_limb_signed_t fq_default_mat_ncols__extern(const fq_default_mat_t mat, const fq_default_ctx_t ctx) { return fq_default_mat_ncols(mat, ctx); }
void fq_default_mat_swap_rows__extern(fq_default_mat_t mat, mp_limb_signed_t *perm, mp_limb_signed_t r, mp_limb_signed_t s, const fq_default_ctx_t ctx) { fq_default_mat_swap_rows(mat, perm, r, s, ctx); }
void fq_default_mat_invert_rows__extern(fq_default_mat_t mat, mp_limb_signed_t *perm, const fq_default_ctx_t ctx) { fq_default_mat_invert_rows(mat, perm, ctx); }
void fq_default_mat_swap_cols__extern(fq_default_mat_t mat, mp_limb_signed_t *perm, mp_limb_signed_t r, mp_limb_signed_t s, const fq_default_ctx_t ctx) { fq_default_mat_swap_cols(mat, perm, r, s, ctx); }
void fq_default_mat_invert_cols__extern(fq_default_mat_t mat, mp_limb_signed_t *perm, const fq_default_ctx_t ctx) { fq_default_mat_invert_cols(mat, perm, ctx); }
void fq_default_mat_zero__extern(fq_default_mat_t A, const fq_default_ctx_t ctx) { fq_default_mat_zero(A, ctx); }
void fq_default_mat_one__extern(fq_default_mat_t A, const fq_default_ctx_t ctx) { fq_default_mat_one(A, ctx); }
void fq_default_mat_set_nmod_mat__extern(fq_default_mat_t mat1, const nmod_mat_t mat2, const fq_default_ctx_t ctx) { fq_default_mat_set_nmod_mat(mat1, mat2, ctx); }
void fq_default_mat_set_fmpz_mod_mat__extern(fq_default_mat_t mat1, const fmpz_mod_mat_t mat2, const fq_default_ctx_t ctx) { fq_default_mat_set_fmpz_mod_mat(mat1, mat2, ctx); }
void fq_default_mat_set_fmpz_mat__extern(fq_default_mat_t mat1, const fmpz_mat_t mat2, const fq_default_ctx_t ctx) { fq_default_mat_set_fmpz_mat(mat1, mat2, ctx); }
void fq_default_mat_window_init__extern(fq_default_mat_t window, const fq_default_mat_t mat, mp_limb_signed_t r1, mp_limb_signed_t c1, mp_limb_signed_t r2, mp_limb_signed_t c2, const fq_default_ctx_t ctx) { fq_default_mat_window_init(window, mat, r1, c1, r2, c2, ctx); }
void fq_default_mat_window_clear__extern(fq_default_mat_t window, const fq_default_ctx_t ctx) { fq_default_mat_window_clear(window, ctx); }
void fq_default_mat_concat_horizontal__extern(fq_default_mat_t res, const fq_default_mat_t mat1, const fq_default_mat_t mat2, const fq_default_ctx_t ctx) { fq_default_mat_concat_horizontal(res, mat1, mat2, ctx); }
void fq_default_mat_concat_vertical__extern(fq_default_mat_t res, const fq_default_mat_t mat1, const fq_default_mat_t mat2, const fq_default_ctx_t ctx) { fq_default_mat_concat_vertical(res, mat1, mat2, ctx); }
void fq_default_mat_randtest__extern(fq_default_mat_t mat, flint_rand_t state, const fq_default_ctx_t ctx) { fq_default_mat_randtest(mat, state, ctx); }
void fq_default_mat_randrank__extern(fq_default_mat_t mat, flint_rand_t state, mp_limb_signed_t rank, const fq_default_ctx_t ctx) { fq_default_mat_randrank(mat, state, rank, ctx); }
void fq_default_mat_randops__extern(fq_default_mat_t mat, flint_rand_t state, mp_limb_signed_t count, const fq_default_ctx_t ctx) { fq_default_mat_randops(mat, state, count, ctx); }
void fq_default_mat_randtril__extern(fq_default_mat_t mat, flint_rand_t state, int unit, const fq_default_ctx_t ctx) { fq_default_mat_randtril(mat, state, unit, ctx); }
void fq_default_mat_randtriu__extern(fq_default_mat_t mat, flint_rand_t state, int unit, const fq_default_ctx_t ctx) { fq_default_mat_randtriu(mat, state, unit, ctx); }
void fq_default_mat_add__extern(fq_default_mat_t C, const fq_default_mat_t A, const fq_default_mat_t B, const fq_default_ctx_t ctx) { fq_default_mat_add(C, A, B, ctx); }
void fq_default_mat_sub__extern(fq_default_mat_t C, const fq_default_mat_t A, const fq_default_mat_t B, const fq_default_ctx_t ctx) { fq_default_mat_sub(C, A, B, ctx); }
void fq_default_mat_neg__extern(fq_default_mat_t B, const fq_default_mat_t A, const fq_default_ctx_t ctx) { fq_default_mat_neg(B, A, ctx); }
void fq_default_mat_submul__extern(fq_default_mat_t D, const fq_default_mat_t C, const fq_default_mat_t A, const fq_default_mat_t B, const fq_default_ctx_t ctx) { fq_default_mat_submul(D, C, A, B, ctx); }
void fq_default_mat_mul__extern(fq_default_mat_t C, const fq_default_mat_t A, const fq_default_mat_t B, const fq_default_ctx_t ctx) { fq_default_mat_mul(C, A, B, ctx); }
mp_limb_signed_t fq_default_mat_lu__extern(mp_limb_signed_t *P, fq_default_mat_t A, int rank_check, const fq_default_ctx_t ctx) { return fq_default_mat_lu(P, A, rank_check, ctx); }
int fq_default_mat_inv__extern(fq_default_mat_t B, fq_default_mat_t A, const fq_default_ctx_t ctx) { return fq_default_mat_inv(B, A, ctx); }
mp_limb_signed_t fq_default_mat_rref__extern(fq_default_mat_t B, const fq_default_mat_t A, const fq_default_ctx_t ctx) { return fq_default_mat_rref(B, A, ctx); }
mp_limb_signed_t fq_default_mat_nullspace__extern(fq_default_mat_t X, const fq_default_mat_t A, const fq_default_ctx_t ctx) { return fq_default_mat_nullspace(X, A, ctx); }
mp_limb_signed_t fq_default_mat_rank__extern(const fq_default_mat_t A, const fq_default_ctx_t ctx) { return fq_default_mat_rank(A, ctx); }
void fq_default_mat_solve_tril__extern(fq_default_mat_t X, const fq_default_mat_t L, const fq_default_mat_t B, int unit, const fq_default_ctx_t ctx) { fq_default_mat_solve_tril(X, L, B, unit, ctx); }
void fq_default_mat_solve_triu__extern(fq_default_mat_t X, const fq_default_mat_t U, const fq_default_mat_t B, int unit, const fq_default_ctx_t ctx) { fq_default_mat_solve_triu(X, U, B, unit, ctx); }
int fq_default_mat_solve__extern(fq_default_mat_t X, const fq_default_mat_t A, const fq_default_mat_t C, const fq_default_ctx_t ctx) { return fq_default_mat_solve(X, A, C, ctx); }
int fq_default_mat_can_solve__extern(fq_default_mat_t X, const fq_default_mat_t A, const fq_default_mat_t B, const fq_default_ctx_t ctx) { return fq_default_mat_can_solve(X, A, B, ctx); }
void fq_default_mat_similarity__extern(fq_default_mat_t A, mp_limb_signed_t r, fq_default_t d, const fq_default_ctx_t ctx) { fq_default_mat_similarity(A, r, d, ctx); }

void fq_default_poly_init__extern(fq_default_poly_t poly, const fq_default_ctx_t ctx) { fq_default_poly_init(poly, ctx); }
void fq_default_poly_init2__extern(fq_default_poly_t poly, mp_limb_signed_t alloc, const fq_default_ctx_t ctx) { fq_default_poly_init2(poly, alloc, ctx); }
void fq_default_poly_realloc__extern(fq_default_poly_t poly, mp_limb_signed_t alloc, const fq_default_ctx_t ctx) { fq_default_poly_realloc(poly, alloc, ctx); }
void fq_default_poly_truncate__extern(fq_default_poly_t poly, mp_limb_signed_t len, const fq_default_ctx_t ctx) { fq_default_poly_truncate(poly, len, ctx); }
void fq_default_poly_set_trunc__extern(fq_default_poly_t poly1, fq_default_poly_t poly2, mp_limb_signed_t len, const fq_default_ctx_t ctx) { fq_default_poly_set_trunc(poly1, poly2, len, ctx); }
void fq_default_poly_fit_length__extern(fq_default_poly_t poly, mp_limb_signed_t len, const fq_default_ctx_t ctx) { fq_default_poly_fit_length(poly, len, ctx); }
void _fq_default_poly_set_length__extern(fq_default_poly_t poly, mp_limb_signed_t len, const fq_default_ctx_t ctx) { _fq_default_poly_set_length(poly, len, ctx); }
void fq_default_poly_clear__extern(fq_default_poly_t poly, const fq_default_ctx_t ctx) { fq_default_poly_clear(poly, ctx); }
mp_limb_signed_t fq_default_poly_length__extern(const fq_default_poly_t poly, const fq_default_ctx_t ctx) { return fq_default_poly_length(poly, ctx); }
mp_limb_signed_t fq_default_poly_degree__extern(const fq_default_poly_t poly, const fq_default_ctx_t ctx) { return fq_default_poly_degree(poly, ctx); }
void fq_default_poly_randtest__extern(fq_default_poly_t f, flint_rand_t state, mp_limb_signed_t len, const fq_default_ctx_t ctx) { fq_default_poly_randtest(f, state, len, ctx); }
void fq_default_poly_randtest_not_zero__extern(fq_default_poly_t f, flint_rand_t state, mp_limb_signed_t len, const fq_default_ctx_t ctx) { fq_default_poly_randtest_not_zero(f, state, len, ctx); }
void fq_default_poly_randtest_monic__extern(fq_default_poly_t f, flint_rand_t state, mp_limb_signed_t len, const fq_default_ctx_t ctx) { fq_default_poly_randtest_monic(f, state, len, ctx); }
void fq_default_poly_randtest_irreducible__extern(fq_default_poly_t f, flint_rand_t state, mp_limb_signed_t len, const fq_default_ctx_t ctx) { fq_default_poly_randtest_irreducible(f, state, len, ctx); }
void fq_default_poly_set__extern(fq_default_poly_t rop, const fq_default_poly_t op, const fq_default_ctx_t ctx) { fq_default_poly_set(rop, op, ctx); }
void fq_default_poly_set_fq_default__extern(fq_default_poly_t poly, const fq_default_t c, const fq_default_ctx_t ctx) { fq_default_poly_set_fq_default(poly, c, ctx); }
void fq_default_poly_swap__extern(fq_default_poly_t op1, fq_default_poly_t op2, const fq_default_ctx_t ctx) { fq_default_poly_swap(op1, op2, ctx); }
void fq_default_poly_zero__extern(fq_default_poly_t poly, const fq_default_ctx_t ctx) { fq_default_poly_zero(poly, ctx); }
void fq_default_poly_one__extern(fq_default_poly_t poly, const fq_default_ctx_t ctx) { fq_default_poly_one(poly, ctx); }
void fq_default_poly_gen__extern(fq_default_poly_t f, const fq_default_ctx_t ctx) { fq_default_poly_gen(f, ctx); }
void fq_default_poly_make_monic__extern(fq_default_poly_t rop, const fq_default_poly_t op, const fq_default_ctx_t ctx) { fq_default_poly_make_monic(rop, op, ctx); }
void fq_default_poly_reverse__extern(fq_default_poly_t res, const fq_default_poly_t poly, mp_limb_signed_t n, const fq_default_ctx_t ctx) { fq_default_poly_reverse(res, poly, n, ctx); }
mp_limb_t fq_default_poly_deflation__extern(const fq_default_poly_t input, const fq_default_ctx_t ctx) { return fq_default_poly_deflation(input, ctx); }
void fq_default_poly_deflate__extern(fq_default_poly_t result, const fq_default_poly_t input, mp_limb_t deflation, const fq_default_ctx_t ctx) { fq_default_poly_deflate(result, input, deflation, ctx); }
void fq_default_poly_inflate__extern(fq_default_poly_t result, const fq_default_poly_t input, mp_limb_t inflation, const fq_default_ctx_t ctx) { fq_default_poly_inflate(result, input, inflation, ctx); }
void fq_default_poly_get_coeff__extern(fq_default_t x, const fq_default_poly_t poly, mp_limb_signed_t n, const fq_default_ctx_t ctx) { fq_default_poly_get_coeff(x, poly, n, ctx); }
void fq_default_poly_set_coeff__extern(fq_default_poly_t poly, mp_limb_signed_t n, const fq_default_t x, const fq_default_ctx_t ctx) { fq_default_poly_set_coeff(poly, n, x, ctx); }
void fq_default_poly_set_coeff_fmpz__extern(fq_default_poly_t poly, mp_limb_signed_t n, const fmpz_t x, const fq_default_ctx_t ctx) { fq_default_poly_set_coeff_fmpz(poly, n, x, ctx); }
void fq_default_poly_set_nmod_poly__extern(fq_default_poly_t rop, const nmod_poly_t op, const fq_default_ctx_t ctx) { fq_default_poly_set_nmod_poly(rop, op, ctx); }
void fq_default_poly_set_fmpz_mod_poly__extern(fq_default_poly_t rop, const fmpz_mod_poly_t op, const fq_default_ctx_t ctx) { fq_default_poly_set_fmpz_mod_poly(rop, op, ctx); }
int fq_default_poly_equal__extern(const fq_default_poly_t poly1, const fq_default_poly_t poly2, const fq_default_ctx_t ctx) { return fq_default_poly_equal(poly1, poly2, ctx); }
int fq_default_poly_equal_trunc__extern(const fq_default_poly_t poly1, const fq_default_poly_t poly2, mp_limb_signed_t n, const fq_default_ctx_t ctx) { return fq_default_poly_equal_trunc(poly1, poly2, n, ctx); }
int fq_default_poly_is_zero__extern(const fq_default_poly_t poly, const fq_default_ctx_t ctx) { return fq_default_poly_is_zero(poly, ctx); }
int fq_default_poly_is_one__extern(const fq_default_poly_t op, const fq_default_ctx_t ctx) { return fq_default_poly_is_one(op, ctx); }
int fq_default_poly_is_unit__extern(const fq_default_poly_t op, const fq_default_ctx_t ctx) { return fq_default_poly_is_unit(op, ctx); }
int fq_default_poly_is_gen__extern(const fq_default_poly_t poly, const fq_default_ctx_t ctx) { return fq_default_poly_is_gen(poly, ctx); }
int fq_default_poly_equal_fq_default__extern(const fq_default_poly_t poly, const fq_default_t c, const fq_default_ctx_t ctx) { return fq_default_poly_equal_fq_default(poly, c, ctx); }
void fq_default_poly_add__extern(fq_default_poly_t rop, const fq_default_poly_t op1, const fq_default_poly_t op2, const fq_default_ctx_t ctx) { fq_default_poly_add(rop, op1, op2, ctx); }
void fq_default_poly_add_si__extern(fq_default_poly_t rop, const fq_default_poly_t op1, mp_limb_signed_t c, const fq_default_ctx_t ctx) { fq_default_poly_add_si(rop, op1, c, ctx); }
void fq_default_poly_add_series__extern(fq_default_poly_t rop, const fq_default_poly_t op1, const fq_default_poly_t op2, mp_limb_signed_t n, const fq_default_ctx_t ctx) { fq_default_poly_add_series(rop, op1, op2, n, ctx); }
void fq_default_poly_sub__extern(fq_default_poly_t rop, const fq_default_poly_t op1, const fq_default_poly_t op2, const fq_default_ctx_t ctx) { fq_default_poly_sub(rop, op1, op2, ctx); }
void fq_default_poly_sub_series__extern(fq_default_poly_t rop, const fq_default_poly_t op1, const fq_default_poly_t op2, mp_limb_signed_t n, const fq_default_ctx_t ctx) { fq_default_poly_sub_series(rop, op1, op2, n, ctx); }
void fq_default_poly_neg__extern(fq_default_poly_t rop, const fq_default_poly_t op, const fq_default_ctx_t ctx) { fq_default_poly_neg(rop, op, ctx); }
void fq_default_poly_scalar_mul_fq_default__extern(fq_default_poly_t rop, const fq_default_poly_t op, const fq_default_t x, const fq_default_ctx_t ctx) { fq_default_poly_scalar_mul_fq_default(rop, op, x, ctx); }
void fq_default_poly_scalar_div_fq_default__extern(fq_default_poly_t rop, const fq_default_poly_t op, const fq_default_t x, const fq_default_ctx_t ctx) { fq_default_poly_scalar_div_fq_default(rop, op, x, ctx); }
void fq_default_poly_scalar_addmul_fq_default__extern(fq_default_poly_t rop, const fq_default_poly_t op, const fq_default_t x, const fq_default_ctx_t ctx) { fq_default_poly_scalar_addmul_fq_default(rop, op, x, ctx); }
void fq_default_poly_scalar_submul_fq_default__extern(fq_default_poly_t rop, const fq_default_poly_t op, const fq_default_t x, const fq_default_ctx_t ctx) { fq_default_poly_scalar_submul_fq_default(rop, op, x, ctx); }
void fq_default_poly_mul__extern(fq_default_poly_t rop, const fq_default_poly_t op1, const fq_default_poly_t op2, const fq_default_ctx_t ctx) { fq_default_poly_mul(rop, op1, op2, ctx); }
void fq_default_poly_mullow__extern(fq_default_poly_t rop, const fq_default_poly_t op1, const fq_default_poly_t op2, mp_limb_signed_t n, const fq_default_ctx_t ctx) { fq_default_poly_mullow(rop, op1, op2, n, ctx); }
void fq_default_poly_mulhigh__extern(fq_default_poly_t rop, const fq_default_poly_t op1, const fq_default_poly_t op2, mp_limb_signed_t start, const fq_default_ctx_t ctx) { fq_default_poly_mulhigh(rop, op1, op2, start, ctx); }
void fq_default_poly_mulmod__extern(fq_default_poly_t res, const fq_default_poly_t poly1, const fq_default_poly_t poly2, const fq_default_poly_t f, const fq_default_ctx_t ctx) { fq_default_poly_mulmod(res, poly1, poly2, f, ctx); }
void fq_default_poly_sqr__extern(fq_default_poly_t rop, const fq_default_poly_t op, const fq_default_ctx_t ctx) { fq_default_poly_sqr(rop, op, ctx); }
void fq_default_poly_pow__extern(fq_default_poly_t rop, const fq_default_poly_t op, mp_limb_t e, const fq_default_ctx_t ctx) { fq_default_poly_pow(rop, op, e, ctx); }
void fq_default_poly_pow_trunc__extern(fq_default_poly_t res, const fq_default_poly_t poly, mp_limb_t e, mp_limb_signed_t trunc, const fq_default_ctx_t ctx) { fq_default_poly_pow_trunc(res, poly, e, trunc, ctx); }
void fq_default_poly_powmod_fmpz_binexp__extern(fq_default_poly_t res, const fq_default_poly_t poly, const fmpz_t e, const fq_default_poly_t f, const fq_default_ctx_t ctx) { fq_default_poly_powmod_fmpz_binexp(res, poly, e, f, ctx); }
void fq_default_poly_powmod_ui_binexp__extern(fq_default_poly_t res, const fq_default_poly_t poly, mp_limb_t e, const fq_default_poly_t f, const fq_default_ctx_t ctx) { fq_default_poly_powmod_ui_binexp(res, poly, e, f, ctx); }
void fq_default_poly_shift_left__extern(fq_default_poly_t rop, const fq_default_poly_t op, mp_limb_signed_t n, const fq_default_ctx_t ctx) { fq_default_poly_shift_left(rop, op, n, ctx); }
void fq_default_poly_shift_right__extern(fq_default_poly_t rop, const fq_default_poly_t op, mp_limb_signed_t n, const fq_default_ctx_t ctx) { fq_default_poly_shift_right(rop, op, n, ctx); }
mp_limb_signed_t fq_default_poly_hamming_weight__extern(const fq_default_poly_t op, const fq_default_ctx_t ctx) { return fq_default_poly_hamming_weight(op, ctx); }
void fq_default_poly_gcd__extern(fq_default_poly_t rop, const fq_default_poly_t op1, const fq_default_poly_t op2, const fq_default_ctx_t ctx) { fq_default_poly_gcd(rop, op1, op2, ctx); }
void fq_default_poly_xgcd__extern(fq_default_poly_t G, fq_default_poly_t S, fq_default_poly_t T, const fq_default_poly_t A, const fq_default_poly_t B, const fq_default_ctx_t ctx) { fq_default_poly_xgcd(G, S, T, A, B, ctx); }
mp_limb_t fq_default_poly_remove__extern(fq_default_poly_t f, const fq_default_poly_t g, const fq_default_ctx_t ctx) { return fq_default_poly_remove(f, g, ctx); }
void fq_default_poly_divrem__extern(fq_default_poly_t Q, fq_default_poly_t R, const fq_default_poly_t A, const fq_default_poly_t B, const fq_default_ctx_t ctx) { fq_default_poly_divrem(Q, R, A, B, ctx); }
void fq_default_poly_rem__extern(fq_default_poly_t R, const fq_default_poly_t A, const fq_default_poly_t B, const fq_default_ctx_t ctx) { fq_default_poly_rem(R, A, B, ctx); }
void fq_default_poly_inv_series__extern(fq_default_poly_t Qinv, const fq_default_poly_t Q, mp_limb_signed_t n, const fq_default_ctx_t ctx) { fq_default_poly_inv_series(Qinv, Q, n, ctx); }
void fq_default_poly_div_series__extern(fq_default_poly_t Q, const fq_default_poly_t A, const fq_default_poly_t B, mp_limb_signed_t n, const fq_default_ctx_t ctx) { fq_default_poly_div_series(Q, A, B, n, ctx); }
int fq_default_poly_divides__extern(fq_default_poly_t Q, const fq_default_poly_t A, const fq_default_poly_t B, const fq_default_ctx_t ctx) { return fq_default_poly_divides(Q, A, B, ctx); }
void fq_default_poly_derivative__extern(fq_default_poly_t rop, const fq_default_poly_t op, const fq_default_ctx_t ctx) { fq_default_poly_derivative(rop, op, ctx); }
void fq_default_poly_invsqrt_series__extern(fq_default_poly_t rop, const fq_default_poly_t op, mp_limb_signed_t n, fq_default_ctx_t ctx) { fq_default_poly_invsqrt_series(rop, op, n, ctx); }
void fq_default_poly_sqrt_series__extern(fq_default_poly_t rop, const fq_default_poly_t op, mp_limb_signed_t n, fq_default_ctx_t ctx) { fq_default_poly_sqrt_series(rop, op, n, ctx); }
int fq_default_poly_sqrt__extern(fq_default_poly_t rop, const fq_default_poly_t op, fq_default_ctx_t ctx) { return fq_default_poly_sqrt(rop, op, ctx); }
void fq_default_poly_evaluate_fq_default__extern(fq_default_t res, const fq_default_poly_t f, const fq_default_t a, const fq_default_ctx_t ctx) { fq_default_poly_evaluate_fq_default(res, f, a, ctx); }
void fq_default_poly_compose__extern(fq_default_poly_t rop, const fq_default_poly_t op1, const fq_default_poly_t op2, const fq_default_ctx_t ctx) { fq_default_poly_compose(rop, op1, op2, ctx); }
void fq_default_poly_compose_mod__extern(fq_default_poly_t res, const fq_default_poly_t poly1, const fq_default_poly_t poly2, const fq_default_poly_t poly3, const fq_default_ctx_t ctx) { fq_default_poly_compose_mod(res, poly1, poly2, poly3, ctx); }
char * fq_default_poly_get_str_pretty__extern(const fq_default_poly_t poly, const char *x, const fq_default_ctx_t ctx) { return fq_default_poly_get_str_pretty(poly, x, ctx); }
char * fq_default_poly_get_str__extern(const fq_default_poly_t poly, const fq_default_ctx_t ctx) { return fq_default_poly_get_str(poly, ctx); }
void fq_default_mat_charpoly__extern(fq_default_poly_t p, const fq_default_mat_t M, const fq_default_ctx_t ctx) { fq_default_mat_charpoly(p, M, ctx); }
void fq_default_mat_minpoly__extern(fq_default_poly_t p, const fq_default_mat_t X, const fq_default_ctx_t ctx) { fq_default_mat_minpoly(p, X, ctx); }

void fq_default_poly_factor_init__extern(fq_default_poly_factor_t fac, const fq_default_ctx_t ctx) { fq_default_poly_factor_init(fac, ctx); }
void fq_default_poly_factor_clear__extern(fq_default_poly_factor_t fac, const fq_default_ctx_t ctx) { fq_default_poly_factor_clear(fac, ctx); }
void fq_default_poly_factor_realloc__extern(fq_default_poly_factor_t fac, mp_limb_signed_t alloc, const fq_default_ctx_t ctx) { fq_default_poly_factor_realloc(fac, alloc, ctx); }
void fq_default_poly_factor_fit_length__extern(fq_default_poly_factor_t fac, mp_limb_signed_t len, const fq_default_ctx_t ctx) { fq_default_poly_factor_fit_length(fac, len, ctx); }
mp_limb_signed_t fq_default_poly_factor_length__extern(fq_default_poly_factor_t fac, const fq_default_ctx_t ctx) { return fq_default_poly_factor_length(fac, ctx); }
mp_limb_signed_t fq_default_poly_factor_exp__extern(fq_default_poly_factor_t fac, mp_limb_signed_t i, const fq_default_ctx_t ctx) { return fq_default_poly_factor_exp(fac, i, ctx); }
void fq_default_poly_factor_set__extern(fq_default_poly_factor_t res, const fq_default_poly_factor_t fac, const fq_default_ctx_t ctx) { fq_default_poly_factor_set(res, fac, ctx); }
void fq_default_poly_factor_insert__extern(fq_default_poly_factor_t fac, const fq_default_poly_t poly, mp_limb_signed_t exp, const fq_default_ctx_t ctx) { fq_default_poly_factor_insert(fac, poly, exp, ctx); }
void fq_default_poly_factor_get_poly__extern(fq_default_poly_t poly, const fq_default_poly_factor_t fac, mp_limb_signed_t i, const fq_default_ctx_t ctx) { fq_default_poly_factor_get_poly(poly, fac, i, ctx); }
void fq_default_poly_factor_print__extern(const fq_default_poly_factor_t fac, const fq_default_ctx_t ctx) { fq_default_poly_factor_print(fac, ctx); }
void fq_default_poly_factor_print_pretty__extern(const fq_default_poly_factor_t fac, const char *var, const fq_default_ctx_t ctx) { fq_default_poly_factor_print_pretty(fac, var, ctx); }
void fq_default_poly_factor_concat__extern(fq_default_poly_factor_t res, const fq_default_poly_factor_t fac, const fq_default_ctx_t ctx) { fq_default_poly_factor_concat(res, fac, ctx); }
void fq_default_poly_factor_pow__extern(fq_default_poly_factor_t fac, mp_limb_signed_t exp, const fq_default_ctx_t ctx) { fq_default_poly_factor_pow(fac, exp, ctx); }
int fq_default_poly_is_squarefree__extern(const fq_default_poly_t f, const fq_default_ctx_t ctx) { return fq_default_poly_is_squarefree(f, ctx); }
void fq_default_poly_factor_squarefree__extern(fq_default_poly_factor_t res, const fq_default_poly_t f, const fq_default_ctx_t ctx) { fq_default_poly_factor_squarefree(res, f, ctx); }
int fq_default_poly_is_irreducible__extern(const fq_default_poly_t f, const fq_default_ctx_t ctx) { return fq_default_poly_is_irreducible(f, ctx); }
void fq_default_poly_factor_distinct_deg__extern(fq_default_poly_factor_t res, const fq_default_poly_t poly, mp_limb_signed_t *const *degs, const fq_default_ctx_t ctx) { fq_default_poly_factor_distinct_deg(res, poly, degs, ctx); }
void fq_default_poly_factor_equal_deg__extern(fq_default_poly_factor_t factors, const fq_default_poly_t pol, mp_limb_signed_t d, const fq_default_ctx_t ctx) { fq_default_poly_factor_equal_deg(factors, pol, d, ctx); }
void fq_default_poly_factor__extern(fq_default_poly_factor_t result, fq_default_t leading_coeff, const fq_default_poly_t input, const fq_default_ctx_t ctx) { fq_default_poly_factor(result, leading_coeff, input, ctx); }
void fq_default_poly_factor_split_single__extern(fq_default_poly_t linfactor, const fq_default_poly_t input, const fq_default_ctx_t ctx) { fq_default_poly_factor_split_single(linfactor, input, ctx); }
void fq_default_poly_roots__extern(fq_default_poly_factor_t r, const fq_default_poly_t f, int with_multiplicity, const fq_default_ctx_t ctx) { fq_default_poly_roots(r, f, with_multiplicity, ctx); }

const nmod_poly_struct * fq_nmod_ctx_modulus__extern(const fq_nmod_ctx_t ctx) { return fq_nmod_ctx_modulus(ctx); }
mp_limb_signed_t fq_nmod_ctx_degree__extern(const fq_nmod_ctx_t ctx) { return fq_nmod_ctx_degree(ctx); }
mp_limb_t fq_nmod_ctx_prime__extern(const fq_nmod_ctx_t ctx) { return fq_nmod_ctx_prime(ctx); }

mp_limb_signed_t fq_nmod_mpoly_ctx_nvars__extern(const fq_nmod_mpoly_ctx_t ctx) { return fq_nmod_mpoly_ctx_nvars(ctx); }
ordering_t fq_nmod_mpoly_ctx_ord__extern(const fq_nmod_mpoly_ctx_t ctx) { return fq_nmod_mpoly_ctx_ord(ctx); }
void fq_nmod_mpoly_init__extern(fq_nmod_mpoly_t A, const fq_nmod_mpoly_ctx_t ctx) { fq_nmod_mpoly_init(A, ctx); }
void fq_nmod_mpoly_clear__extern(fq_nmod_mpoly_t A, const fq_nmod_mpoly_ctx_t ctx) { fq_nmod_mpoly_clear(A, ctx); }
void _fq_nmod_mpoly_fit_length__extern(mp_limb_t **coeffs, mp_limb_signed_t *coeffs_alloc, mp_limb_signed_t d, mp_limb_t **exps, mp_limb_signed_t *exps_alloc, mp_limb_signed_t N, mp_limb_signed_t length) { _fq_nmod_mpoly_fit_length(coeffs, coeffs_alloc, d, exps, exps_alloc, N, length); }
void _fq_nmod_mpoly_set_length__extern(fq_nmod_mpoly_t A, mp_limb_signed_t newlen, const fq_nmod_mpoly_ctx_t ctx) { _fq_nmod_mpoly_set_length(A, newlen, ctx); }
void fq_nmod_mpoly_truncate__extern(fq_nmod_mpoly_t A, mp_limb_signed_t newlen, const fq_nmod_mpoly_ctx_t ctx) { fq_nmod_mpoly_truncate(A, newlen, ctx); }
void fq_nmod_mpoly_swap__extern(fq_nmod_mpoly_t A, fq_nmod_mpoly_t B, const fq_nmod_mpoly_ctx_t ctx) { fq_nmod_mpoly_swap(A, B, ctx); }
mp_limb_t * fq_nmod_mpoly_get_nonzero_n_fq__extern(const fq_nmod_mpoly_t A, const fq_nmod_mpoly_ctx_t ctx) { return fq_nmod_mpoly_get_nonzero_n_fq(A, ctx); }
void fq_nmod_mpoly_zero__extern(fq_nmod_mpoly_t A, const fq_nmod_mpoly_ctx_t ctx) { fq_nmod_mpoly_zero(A, ctx); }
int fq_nmod_mpoly_is_zero__extern(const fq_nmod_mpoly_t A, const fq_nmod_mpoly_ctx_t ctx) { return fq_nmod_mpoly_is_zero(A, ctx); }
mp_limb_t * _fq_nmod_mpoly_leadcoeff__extern(const fq_nmod_mpoly_t A, const fq_nmod_mpoly_ctx_t ctx) { return _fq_nmod_mpoly_leadcoeff(A, ctx); }
mp_limb_signed_t fq_nmod_mpoly_length__extern(const fq_nmod_mpoly_t A, const fq_nmod_mpoly_ctx_t ctx) { return fq_nmod_mpoly_length(A, ctx); }
void fq_nmod_mpoly_divexact__extern(fq_nmod_mpoly_t Q, const fq_nmod_mpoly_t A, const fq_nmod_mpoly_t B, const fq_nmod_mpoly_ctx_t ctx) { fq_nmod_mpoly_divexact(Q, A, B, ctx); }
int fq_nmod_mpoly_sqrt__extern(fq_nmod_mpoly_t Q, const fq_nmod_mpoly_t A, const fq_nmod_mpoly_ctx_t ctx) { return fq_nmod_mpoly_sqrt(Q, A, ctx); }
int fq_nmod_mpoly_is_square__extern(const fq_nmod_mpoly_t A, const fq_nmod_mpoly_ctx_t ctx) { return fq_nmod_mpoly_is_square(A, ctx); }
void fq_nmod_mpoly_univar_zero__extern(fq_nmod_mpoly_univar_t A, const fq_nmod_mpoly_ctx_t ctx) { fq_nmod_mpoly_univar_zero(A, ctx); }
void fq_nmod_mpoly_univar_swap__extern(fq_nmod_mpoly_univar_t A, fq_nmod_mpoly_univar_t B, const fq_nmod_mpoly_ctx_t ctx) { fq_nmod_mpoly_univar_swap(A, B, ctx); }
mp_limb_signed_t fq_nmod_mpoly_univar_length__extern(const fq_nmod_mpoly_univar_t A, const fq_nmod_mpoly_ctx_t ctx) { return fq_nmod_mpoly_univar_length(A, ctx); }
void fq_nmod_mpoly_univar_get_term_coeff__extern(fq_nmod_mpoly_t c, const fq_nmod_mpoly_univar_t A, mp_limb_signed_t i, const fq_nmod_mpoly_ctx_t ctx) { fq_nmod_mpoly_univar_get_term_coeff(c, A, i, ctx); }
void fq_nmod_mpoly_univar_swap_term_coeff__extern(fq_nmod_mpoly_t c, fq_nmod_mpoly_univar_t A, mp_limb_signed_t i, const fq_nmod_mpoly_ctx_t ctx) { fq_nmod_mpoly_univar_swap_term_coeff(c, A, i, ctx); }
void fq_nmod_mpolyu_swap__extern(fq_nmod_mpolyu_t A, fq_nmod_mpolyu_t B, const fq_nmod_mpoly_ctx_t uctx) { fq_nmod_mpolyu_swap(A, B, uctx); }
mp_limb_t * fq_nmod_mpolyu_leadcoeff__extern(const fq_nmod_mpolyu_t A, const fq_nmod_mpoly_ctx_t ctx) { return fq_nmod_mpolyu_leadcoeff(A, ctx); }
mp_limb_t * fq_nmod_mpolyn_leadcoeff__extern(fq_nmod_mpolyn_t A, const fq_nmod_mpoly_ctx_t ctx) { return fq_nmod_mpolyn_leadcoeff(A, ctx); }
n_poly_struct * fq_nmod_mpolyn_leadcoeff_poly__extern(const fq_nmod_mpolyn_t A, const fq_nmod_mpoly_ctx_t ctx) { return fq_nmod_mpolyn_leadcoeff_poly(A, ctx); }
n_poly_struct * fq_nmod_mpolyun_leadcoeff_poly__extern(const fq_nmod_mpolyun_t A, const fq_nmod_mpoly_ctx_t ctx) { return fq_nmod_mpolyun_leadcoeff_poly(A, ctx); }

mp_limb_signed_t fq_nmod_mpoly_factor_length__extern(const fq_nmod_mpoly_factor_t f, const fq_nmod_mpoly_ctx_t ctx) { return fq_nmod_mpoly_factor_length(f, ctx); }
void fq_nmod_mpoly_factor_get_base__extern(fq_nmod_mpoly_t p, const fq_nmod_mpoly_factor_t f, mp_limb_signed_t i, const fq_nmod_mpoly_ctx_t ctx) { fq_nmod_mpoly_factor_get_base(p, f, i, ctx); }
void fq_nmod_mpoly_factor_swap_base__extern(fq_nmod_mpoly_t p, const fq_nmod_mpoly_factor_t f, mp_limb_signed_t i, const fq_nmod_mpoly_ctx_t ctx) { fq_nmod_mpoly_factor_swap_base(p, f, i, ctx); }
void fq_nmod_mpoly_factor_swap__extern(fq_nmod_mpoly_factor_t A, fq_nmod_mpoly_factor_t B, const fq_nmod_mpoly_ctx_t ctx) { fq_nmod_mpoly_factor_swap(A, B, ctx); }
int fq_nmod_mpoly_factor_matches__extern(const fq_nmod_mpoly_t a, const fq_nmod_mpoly_factor_t f, const fq_nmod_mpoly_ctx_t ctx) { return fq_nmod_mpoly_factor_matches(a, f, ctx); }
void fq_nmod_mpolyv_init__extern(fq_nmod_mpolyv_t A, const fq_nmod_mpoly_ctx_t ctx) { fq_nmod_mpolyv_init(A, ctx); }
void fq_nmod_mpolyv_swap__extern(fq_nmod_mpolyv_t A, fq_nmod_mpolyv_t B, const fq_nmod_mpoly_ctx_t ctx) { fq_nmod_mpolyv_swap(A, B, ctx); }

const nmod_poly_struct * fq_zech_ctx_modulus__extern(const fq_zech_ctx_t ctx) { return fq_zech_ctx_modulus(ctx); }
mp_limb_signed_t fq_zech_ctx_degree__extern(const fq_zech_ctx_t ctx) { return fq_zech_ctx_degree(ctx); }
mp_limb_t fq_zech_ctx_prime__extern(const fq_zech_ctx_t ctx) { return fq_zech_ctx_prime(ctx); }
mp_limb_t fq_zech_ctx_order_ui__extern(const fq_zech_ctx_t ctx) { return fq_zech_ctx_order_ui(ctx); }
void fq_zech_init__extern(fq_zech_t rop, const fq_zech_ctx_t ctx) { fq_zech_init(rop, ctx); }
void fq_zech_init2__extern(fq_zech_t rop, const fq_zech_ctx_t ctx) { fq_zech_init2(rop, ctx); }
void fq_zech_clear__extern(fq_zech_t rop, const fq_zech_ctx_t ctx) { fq_zech_clear(rop, ctx); }
void fq_zech_reduce__extern(fq_zech_t rop, const fq_zech_ctx_t ctx) { fq_zech_reduce(rop, ctx); }
int fq_zech_equal__extern(const fq_zech_t op1, const fq_zech_t op2, const fq_zech_ctx_t ctx) { return fq_zech_equal(op1, op2, ctx); }
int fq_zech_is_zero__extern(const fq_zech_t op, const fq_zech_ctx_t ctx) { return fq_zech_is_zero(op, ctx); }
int fq_zech_is_one__extern(const fq_zech_t op, const fq_zech_ctx_t ctx) { return fq_zech_is_one(op, ctx); }
void fq_zech_set__extern(fq_zech_t rop, const fq_zech_t op, const fq_zech_ctx_t ctx) { fq_zech_set(rop, op, ctx); }
void fq_zech_swap__extern(fq_zech_t op1, fq_zech_t op2, const fq_zech_ctx_t ctx) { fq_zech_swap(op1, op2, ctx); }
void fq_zech_zero__extern(fq_zech_t rop, const fq_zech_ctx_t ctx) { fq_zech_zero(rop, ctx); }
void fq_zech_one__extern(fq_zech_t rop, const fq_zech_ctx_t ctx) { fq_zech_one(rop, ctx); }
void fq_zech_gen__extern(fq_zech_t rop, const fq_zech_ctx_t ctx) { fq_zech_gen(rop, ctx); }

nmod_t fq_zech_ctx_mod__extern(const fq_zech_ctx_t ctx) { return fq_zech_ctx_mod(ctx); }
mp_limb_signed_t fq_zech_mpoly_ctx_nvars__extern(const fq_zech_mpoly_ctx_t ctx) { return fq_zech_mpoly_ctx_nvars(ctx); }
ordering_t fq_zech_mpoly_ctx_ord__extern(const fq_zech_mpoly_ctx_t ctx) { return fq_zech_mpoly_ctx_ord(ctx); }
void _fq_zech_mpoly_set_length__extern(fq_zech_mpoly_t A, mp_limb_signed_t newlen, const fq_zech_mpoly_ctx_t ctx) { _fq_zech_mpoly_set_length(A, newlen, ctx); }
void fq_zech_mpoly_truncate__extern(fq_zech_mpoly_t A, mp_limb_signed_t newlen, const fq_zech_mpoly_ctx_t ctx) { fq_zech_mpoly_truncate(A, newlen, ctx); }
void fq_zech_mpoly_swap__extern(fq_zech_mpoly_t A, fq_zech_mpoly_t B, const fq_zech_mpoly_ctx_t ctx) { fq_zech_mpoly_swap(A, B, ctx); }
void fq_zech_mpoly_zero__extern(fq_zech_mpoly_t A, const fq_zech_mpoly_ctx_t ctx) { fq_zech_mpoly_zero(A, ctx); }
void fq_zech_mpoly_one__extern(fq_zech_mpoly_t A, const fq_zech_mpoly_ctx_t ctx) { fq_zech_mpoly_one(A, ctx); }
int fq_zech_mpoly_is_zero__extern(const fq_zech_mpoly_t A, const fq_zech_mpoly_ctx_t ctx) { return fq_zech_mpoly_is_zero(A, ctx); }
fq_zech_struct * fq_zech_mpoly_leadcoeff__extern(const fq_zech_mpoly_t A, const fq_zech_mpoly_ctx_t ctx) { return fq_zech_mpoly_leadcoeff(A, ctx); }
mp_limb_signed_t fq_zech_mpoly_length__extern(const fq_zech_mpoly_t A, const fq_zech_mpoly_ctx_t ctx) { return fq_zech_mpoly_length(A, ctx); }
void fq_zech_mpoly_univar_swap__extern(fq_zech_mpoly_univar_t A, fq_zech_mpoly_univar_t B, const fq_zech_mpoly_ctx_t ctx) { fq_zech_mpoly_univar_swap(A, B, ctx); }
mp_limb_signed_t fq_zech_mpoly_univar_length__extern(const fq_zech_mpoly_univar_t A, const fq_zech_mpoly_ctx_t ctx) { return fq_zech_mpoly_univar_length(A, ctx); }
void fq_zech_mpoly_univar_get_term_coeff__extern(fq_zech_mpoly_t c, const fq_zech_mpoly_univar_t A, mp_limb_signed_t i, const fq_zech_mpoly_ctx_t ctx) { fq_zech_mpoly_univar_get_term_coeff(c, A, i, ctx); }
void fq_zech_mpoly_univar_swap_term_coeff__extern(fq_zech_mpoly_t c, fq_zech_mpoly_univar_t A, mp_limb_signed_t i, const fq_zech_mpoly_ctx_t ctx) { fq_zech_mpoly_univar_swap_term_coeff(c, A, i, ctx); }

void fq_zech_bpoly_init__extern(fq_zech_bpoly_t A, const fq_zech_ctx_t ctx) { fq_zech_bpoly_init(A, ctx); }
void fq_zech_bpoly_swap__extern(fq_zech_bpoly_t A, fq_zech_bpoly_t B, const fq_zech_ctx_t ctx) { fq_zech_bpoly_swap(A, B, ctx); }
void fq_zech_bpoly_fit_length__extern(fq_zech_bpoly_t A, mp_limb_signed_t len, const fq_zech_ctx_t ctx) { fq_zech_bpoly_fit_length(A, len, ctx); }
void fq_zech_bpoly_zero__extern(fq_zech_bpoly_t A, const fq_zech_ctx_t ctx) { fq_zech_bpoly_zero(A, ctx); }
int fq_zech_bpoly_is_zero__extern(const fq_zech_bpoly_t A, const fq_zech_ctx_t ctx) { return fq_zech_bpoly_is_zero(A, ctx); }
mp_limb_signed_t fq_zech_bpoly_degree0__extern(const fq_zech_bpoly_t A, const fq_zech_ctx_t ctx) { return fq_zech_bpoly_degree0(A, ctx); }
void fq_zech_tpoly_init__extern(fq_zech_tpoly_t A, const fq_zech_ctx_t ctx) { fq_zech_tpoly_init(A, ctx); }
void fq_zech_tpoly_swap__extern(fq_zech_tpoly_t A, fq_zech_tpoly_t B, const fq_zech_ctx_t ctx) { fq_zech_tpoly_swap(A, B, ctx); }
void fq_zech_polyu_init__extern(fq_zech_polyu_t A, const fq_zech_ctx_t ctx) { fq_zech_polyu_init(A, ctx); }
void fq_zech_polyu_fit_length__extern(fq_zech_polyu_t A, mp_limb_signed_t len, const fq_zech_ctx_t ctx) { fq_zech_polyu_fit_length(A, len, ctx); }
void fq_zech_polyu_swap__extern(fq_zech_polyu_t A, fq_zech_polyu_t B, const fq_zech_ctx_t ctx) { fq_zech_polyu_swap(A, B, ctx); }
void fq_zech_polyun_init__extern(fq_zech_polyun_t A, const fq_zech_ctx_t ctx) { fq_zech_polyun_init(A, ctx); }
void fq_zech_polyun_fit_length__extern(fq_zech_polyun_t A, mp_limb_signed_t len, const fq_zech_ctx_t ctx) { fq_zech_polyun_fit_length(A, len, ctx); }
void fq_zech_polyun_swap__extern(fq_zech_polyun_t A, fq_zech_polyun_t B, const fq_zech_ctx_t ctx) { fq_zech_polyun_swap(A, B, ctx); }
void fq_zech_mpoly_factor_swap__extern(fq_zech_mpoly_factor_t A, fq_zech_mpoly_factor_t B, const fq_zech_mpoly_ctx_t ctx) { fq_zech_mpoly_factor_swap(A, B, ctx); }
//int fq_zech_mpoly_factor_matches__extern(const fq_zech_mpoly_t a, const fq_zech_mpoly_factor_t f, const fq_zech_mpoly_ctx_t ctx) { return fq_zech_mpoly_factor_matches(a, f, ctx); }
void fq_zech_mpolyv_init__extern(fq_zech_mpolyv_t A, const fq_zech_mpoly_ctx_t ctx) { fq_zech_mpolyv_init(A, ctx); }
void fq_zech_mpolyv_swap__extern(fq_zech_mpolyv_t A, fq_zech_mpolyv_t B, const fq_zech_mpoly_ctx_t ctx) { fq_zech_mpolyv_swap(A, B, ctx); }

void flint_mpz_add_uiui__extern(mpz_ptr a, mpz_srcptr b, mp_limb_t c1, mp_limb_t c0) { flint_mpz_add_uiui(a, b, c1, c0); }
void flint_mpz_add_signed_uiui__extern(mpz_ptr a, mpz_srcptr b, mp_limb_t c1, mp_limb_t c0) { flint_mpz_add_signed_uiui(a, b, c1, c0); }
void flint_mpz_add_uiuiui__extern(mpz_ptr a, mpz_srcptr b, mp_limb_t c2, mp_limb_t c1, mp_limb_t c0) { flint_mpz_add_uiuiui(a, b, c2, c1, c0); }
void flint_mpz_add_signed_uiuiui__extern(mpz_ptr a, mpz_srcptr b, mp_limb_t c2, mp_limb_t c1, mp_limb_t c0) { flint_mpz_add_signed_uiuiui(a, b, c2, c1, c0); }

truth_t truth_and__extern(truth_t x, truth_t y) { return truth_and(x, y); }
truth_t truth_or__extern(truth_t x, truth_t y) { return truth_or(x, y); }
truth_t truth_not__extern(truth_t x) { return truth_not(x); }
void truth_println__extern(truth_t x) { truth_println(x); }
int gr_not_implemented__extern(void) { return gr_not_implemented(); }
int gr_not_in_domain__extern(void) { return gr_not_in_domain(); }
void * gr_ctx_data_ptr__extern(gr_ctx_t ctx) { return gr_ctx_data_ptr(ctx); }
void * gr_ctx_data_as_ptr__extern(gr_ctx_t ctx) { return gr_ctx_data_as_ptr(ctx); }
mp_limb_signed_t gr_ctx_sizeof_ctx__extern(void) { return gr_ctx_sizeof_ctx(); }
mp_limb_signed_t gr_ctx_sizeof_elem__extern(gr_ctx_t ctx) { return gr_ctx_sizeof_elem(ctx); }
int gr_ctx_clear__extern(gr_ctx_t ctx) { return gr_ctx_clear(ctx); }
int gr_ctx_write__extern(gr_stream_t out, gr_ctx_t ctx) { return gr_ctx_write(out, ctx); }
truth_t gr_ctx_is_ring__extern(gr_ctx_t ctx) { return gr_ctx_is_ring(ctx); }
truth_t gr_ctx_is_commutative_ring__extern(gr_ctx_t ctx) { return gr_ctx_is_commutative_ring(ctx); }
truth_t gr_ctx_is_integral_domain__extern(gr_ctx_t ctx) { return gr_ctx_is_integral_domain(ctx); }
truth_t gr_ctx_is_field__extern(gr_ctx_t ctx) { return gr_ctx_is_field(ctx); }
truth_t gr_ctx_is_zero_ring__extern(gr_ctx_t ctx) { return gr_ctx_is_zero_ring(ctx); }
truth_t gr_ctx_is_unique_factorization_domain__extern(gr_ctx_t ctx) { return gr_ctx_is_unique_factorization_domain(ctx); }
truth_t gr_ctx_is_finite__extern(gr_ctx_t ctx) { return gr_ctx_is_finite(ctx); }
truth_t gr_ctx_is_finite_characteristic__extern(gr_ctx_t ctx) { return gr_ctx_is_finite_characteristic(ctx); }
truth_t gr_ctx_is_algebraically_closed__extern(gr_ctx_t ctx) { return gr_ctx_is_algebraically_closed(ctx); }
truth_t gr_ctx_is_ordered_ring__extern(gr_ctx_t ctx) { return gr_ctx_is_ordered_ring(ctx); }
truth_t gr_ctx_is_multiplicative_group__extern(gr_ctx_t ctx) { return gr_ctx_is_multiplicative_group(ctx); }
truth_t gr_ctx_is_exact__extern(gr_ctx_t ctx) { return gr_ctx_is_exact(ctx); }
truth_t gr_ctx_is_canonical__extern(gr_ctx_t ctx) { return gr_ctx_is_canonical(ctx); }
truth_t gr_ctx_is_threadsafe__extern(gr_ctx_t ctx) { return gr_ctx_is_threadsafe(ctx); }
truth_t gr_ctx_has_real_prec__extern(gr_ctx_t ctx) { return gr_ctx_has_real_prec(ctx); }
int gr_ctx_set_real_prec__extern(gr_ctx_t ctx, mp_limb_signed_t prec) { return gr_ctx_set_real_prec(ctx, prec); }
int gr_ctx_get_real_prec__extern(mp_limb_signed_t *prec, gr_ctx_t ctx) { return gr_ctx_get_real_prec(prec, ctx); }
int gr_ctx_set_gen_name__extern(gr_ctx_t ctx, const char *s) { return gr_ctx_set_gen_name(ctx, s); }
int gr_ctx_set_gen_names__extern(gr_ctx_t ctx, const char **s) { return gr_ctx_set_gen_names(ctx, s); }
mp_limb_signed_t _gr_ctx_get_real_prec__extern(gr_ctx_t ctx) { return _gr_ctx_get_real_prec(ctx); }
void gr_init__extern(gr_ptr res, gr_ctx_t ctx) { gr_init(res, ctx); }
void gr_clear__extern(gr_ptr res, gr_ctx_t ctx) { gr_clear(res, ctx); }
void gr_swap__extern(gr_ptr x, gr_ptr y, gr_ctx_t ctx) { gr_swap(x, y, ctx); }
void gr_set_shallow__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { gr_set_shallow(res, x, ctx); }
void _gr_length__extern(gr_srcptr x, gr_ctx_t ctx) { _gr_length(x, ctx); }
int gr_randtest__extern(gr_ptr x, flint_rand_t state, gr_ctx_t ctx) { return gr_randtest(x, state, ctx); }
int gr_randtest_not_zero__extern(gr_ptr x, flint_rand_t state, gr_ctx_t ctx) { return gr_randtest_not_zero(x, state, ctx); }
int gr_randtest_small__extern(gr_ptr x, flint_rand_t state, gr_ctx_t ctx) { return gr_randtest_small(x, state, ctx); }
int gr_write__extern(gr_stream_t out, gr_srcptr x, gr_ctx_t ctx) { return gr_write(out, x, ctx); }
int gr_write_n__extern(gr_stream_t out, gr_srcptr x, mp_limb_signed_t n, gr_ctx_t ctx) { return gr_write_n(out, x, n, ctx); }
int gr_zero__extern(gr_ptr res, gr_ctx_t ctx) { return gr_zero(res, ctx); }
int gr_one__extern(gr_ptr res, gr_ctx_t ctx) { return gr_one(res, ctx); }
int gr_neg_one__extern(gr_ptr res, gr_ctx_t ctx) { return gr_neg_one(res, ctx); }
int gr_set__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_set(res, x, ctx); }
int gr_set_si__extern(gr_ptr res, mp_limb_signed_t x, gr_ctx_t ctx) { return gr_set_si(res, x, ctx); }
int gr_set_ui__extern(gr_ptr res, mp_limb_t x, gr_ctx_t ctx) { return gr_set_ui(res, x, ctx); }
int gr_set_fmpz__extern(gr_ptr res, const fmpz_t x, gr_ctx_t ctx) { return gr_set_fmpz(res, x, ctx); }
int gr_set_fmpq__extern(gr_ptr res, const fmpq_t x, gr_ctx_t ctx) { return gr_set_fmpq(res, x, ctx); }
int gr_set_d__extern(gr_ptr res, double x, gr_ctx_t ctx) { return gr_set_d(res, x, ctx); }
int gr_set_other__extern(gr_ptr res, gr_srcptr x, gr_ctx_t x_ctx, gr_ctx_t ctx) { return gr_set_other(res, x, x_ctx, ctx); }
int gr_set_str__extern(gr_ptr res, const char *x, gr_ctx_t ctx) { return gr_set_str(res, x, ctx); }
int gr_get_si__extern(mp_limb_signed_t *res, gr_srcptr x, gr_ctx_t ctx) { return gr_get_si(res, x, ctx); }
int gr_get_ui__extern(mp_limb_t *res, gr_srcptr x, gr_ctx_t ctx) { return gr_get_ui(res, x, ctx); }
int gr_get_fmpz__extern(fmpz_t res, gr_srcptr x, gr_ctx_t ctx) { return gr_get_fmpz(res, x, ctx); }
int gr_get_fmpq__extern(fmpq_t res, gr_srcptr x, gr_ctx_t ctx) { return gr_get_fmpq(res, x, ctx); }
int gr_get_d__extern(double *res, gr_srcptr x, gr_ctx_t ctx) { return gr_get_d(res, x, ctx); }
truth_t gr_is_zero__extern(gr_srcptr x, gr_ctx_t ctx) { return gr_is_zero(x, ctx); }
truth_t gr_is_one__extern(gr_srcptr x, gr_ctx_t ctx) { return gr_is_one(x, ctx); }
truth_t gr_is_neg_one__extern(gr_srcptr x, gr_ctx_t ctx) { return gr_is_neg_one(x, ctx); }
truth_t gr_equal__extern(gr_srcptr x, gr_srcptr y, gr_ctx_t ctx) { return gr_equal(x, y, ctx); }
truth_t gr_not_equal__extern(gr_srcptr x, gr_srcptr y, gr_ctx_t ctx) { return gr_not_equal(x, y, ctx); }
int gr_neg__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_neg(res, x, ctx); }
int gr_add__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_ctx_t ctx) { return gr_add(res, x, y, ctx); }
int gr_add_ui__extern(gr_ptr res, gr_srcptr x, mp_limb_t y, gr_ctx_t ctx) { return gr_add_ui(res, x, y, ctx); }
int gr_add_si__extern(gr_ptr res, gr_srcptr x, mp_limb_signed_t y, gr_ctx_t ctx) { return gr_add_si(res, x, y, ctx); }
int gr_add_fmpz__extern(gr_ptr res, gr_srcptr x, const fmpz_t y, gr_ctx_t ctx) { return gr_add_fmpz(res, x, y, ctx); }
int gr_add_fmpq__extern(gr_ptr res, gr_srcptr x, const fmpq_t y, gr_ctx_t ctx) { return gr_add_fmpq(res, x, y, ctx); }
int gr_add_other__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_ctx_t y_ctx, gr_ctx_t ctx) { return gr_add_other(res, x, y, y_ctx, ctx); }
int gr_other_add__extern(gr_ptr res, gr_srcptr x, gr_ctx_t x_ctx, gr_srcptr y, gr_ctx_t ctx) { return gr_other_add(res, x, x_ctx, y, ctx); }
int gr_sub__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_ctx_t ctx) { return gr_sub(res, x, y, ctx); }
int gr_sub_ui__extern(gr_ptr res, gr_srcptr x, mp_limb_t y, gr_ctx_t ctx) { return gr_sub_ui(res, x, y, ctx); }
int gr_sub_si__extern(gr_ptr res, gr_srcptr x, mp_limb_signed_t y, gr_ctx_t ctx) { return gr_sub_si(res, x, y, ctx); }
int gr_sub_fmpz__extern(gr_ptr res, gr_srcptr x, const fmpz_t y, gr_ctx_t ctx) { return gr_sub_fmpz(res, x, y, ctx); }
int gr_sub_fmpq__extern(gr_ptr res, gr_srcptr x, const fmpq_t y, gr_ctx_t ctx) { return gr_sub_fmpq(res, x, y, ctx); }
int gr_sub_other__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_ctx_t y_ctx, gr_ctx_t ctx) { return gr_sub_other(res, x, y, y_ctx, ctx); }
int gr_other_sub__extern(gr_ptr res, gr_srcptr x, gr_ctx_t x_ctx, gr_srcptr y, gr_ctx_t ctx) { return gr_other_sub(res, x, x_ctx, y, ctx); }
int gr_mul__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_ctx_t ctx) { return gr_mul(res, x, y, ctx); }
int gr_mul_ui__extern(gr_ptr res, gr_srcptr x, mp_limb_t y, gr_ctx_t ctx) { return gr_mul_ui(res, x, y, ctx); }
int gr_mul_si__extern(gr_ptr res, gr_srcptr x, mp_limb_signed_t y, gr_ctx_t ctx) { return gr_mul_si(res, x, y, ctx); }
int gr_mul_fmpz__extern(gr_ptr res, gr_srcptr x, const fmpz_t y, gr_ctx_t ctx) { return gr_mul_fmpz(res, x, y, ctx); }
int gr_mul_fmpq__extern(gr_ptr res, gr_srcptr x, const fmpq_t y, gr_ctx_t ctx) { return gr_mul_fmpq(res, x, y, ctx); }
int gr_mul_other__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_ctx_t y_ctx, gr_ctx_t ctx) { return gr_mul_other(res, x, y, y_ctx, ctx); }
int gr_other_mul__extern(gr_ptr res, gr_srcptr x, gr_ctx_t x_ctx, gr_srcptr y, gr_ctx_t ctx) { return gr_other_mul(res, x, x_ctx, y, ctx); }
int gr_addmul__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_ctx_t ctx) { return gr_addmul(res, x, y, ctx); }
int gr_addmul_ui__extern(gr_ptr res, gr_srcptr x, mp_limb_t y, gr_ctx_t ctx) { return gr_addmul_ui(res, x, y, ctx); }
int gr_addmul_si__extern(gr_ptr res, gr_srcptr x, mp_limb_signed_t y, gr_ctx_t ctx) { return gr_addmul_si(res, x, y, ctx); }
int gr_addmul_fmpz__extern(gr_ptr res, gr_srcptr x, const fmpz_t y, gr_ctx_t ctx) { return gr_addmul_fmpz(res, x, y, ctx); }
int gr_addmul_fmpq__extern(gr_ptr res, gr_srcptr x, const fmpq_t y, gr_ctx_t ctx) { return gr_addmul_fmpq(res, x, y, ctx); }
int gr_addmul_other__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_ctx_t y_ctx, gr_ctx_t ctx) { return gr_addmul_other(res, x, y, y_ctx, ctx); }
int gr_submul__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_ctx_t ctx) { return gr_submul(res, x, y, ctx); }
int gr_submul_ui__extern(gr_ptr res, gr_srcptr x, mp_limb_t y, gr_ctx_t ctx) { return gr_submul_ui(res, x, y, ctx); }
int gr_submul_si__extern(gr_ptr res, gr_srcptr x, mp_limb_signed_t y, gr_ctx_t ctx) { return gr_submul_si(res, x, y, ctx); }
int gr_submul_fmpz__extern(gr_ptr res, gr_srcptr x, const fmpz_t y, gr_ctx_t ctx) { return gr_submul_fmpz(res, x, y, ctx); }
int gr_submul_fmpq__extern(gr_ptr res, gr_srcptr x, const fmpq_t y, gr_ctx_t ctx) { return gr_submul_fmpq(res, x, y, ctx); }
int gr_submul_other__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_ctx_t y_ctx, gr_ctx_t ctx) { return gr_submul_other(res, x, y, y_ctx, ctx); }
int gr_mul_two__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_mul_two(res, x, ctx); }
int gr_sqr__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_sqr(res, x, ctx); }
int gr_mul_2exp_si__extern(gr_ptr res, gr_srcptr x, mp_limb_signed_t y, gr_ctx_t ctx) { return gr_mul_2exp_si(res, x, y, ctx); }
int gr_mul_2exp_fmpz__extern(gr_ptr res, gr_srcptr x, const fmpz_t y, gr_ctx_t ctx) { return gr_mul_2exp_fmpz(res, x, y, ctx); }
int gr_set_fmpz_2exp_fmpz__extern(gr_ptr res, const fmpz_t x, const fmpz_t y, gr_ctx_t ctx) { return gr_set_fmpz_2exp_fmpz(res, x, y, ctx); }
int gr_get_fmpz_2exp_fmpz__extern(fmpz_t res1, fmpz_t res2, gr_srcptr x, gr_ctx_t ctx) { return gr_get_fmpz_2exp_fmpz(res1, res2, x, ctx); }
int gr_set_fmpz_10exp_fmpz__extern(gr_ptr res, const fmpz_t x, const fmpz_t y, gr_ctx_t ctx) { return gr_set_fmpz_10exp_fmpz(res, x, y, ctx); }
int gr_inv__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_inv(res, x, ctx); }
truth_t gr_is_invertible__extern(gr_srcptr x, gr_ctx_t ctx) { return gr_is_invertible(x, ctx); }
int gr_div__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_ctx_t ctx) { return gr_div(res, x, y, ctx); }
int gr_div_ui__extern(gr_ptr res, gr_srcptr x, mp_limb_t y, gr_ctx_t ctx) { return gr_div_ui(res, x, y, ctx); }
int gr_div_si__extern(gr_ptr res, gr_srcptr x, mp_limb_signed_t y, gr_ctx_t ctx) { return gr_div_si(res, x, y, ctx); }
int gr_div_fmpz__extern(gr_ptr res, gr_srcptr x, const fmpz_t y, gr_ctx_t ctx) { return gr_div_fmpz(res, x, y, ctx); }
int gr_div_fmpq__extern(gr_ptr res, gr_srcptr x, const fmpq_t y, gr_ctx_t ctx) { return gr_div_fmpq(res, x, y, ctx); }
int gr_div_other__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_ctx_t y_ctx, gr_ctx_t ctx) { return gr_div_other(res, x, y, y_ctx, ctx); }
int gr_other_div__extern(gr_ptr res, gr_srcptr x, gr_ctx_t x_ctx, gr_srcptr y, gr_ctx_t ctx) { return gr_other_div(res, x, x_ctx, y, ctx); }
int gr_div_nonunique__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_ctx_t ctx) { return gr_div_nonunique(res, x, y, ctx); }
truth_t gr_divides__extern(gr_srcptr x, gr_srcptr y, gr_ctx_t ctx) { return gr_divides(x, y, ctx); }
int gr_divexact__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_ctx_t ctx) { return gr_divexact(res, x, y, ctx); }
int gr_divexact_ui__extern(gr_ptr res, gr_srcptr x, mp_limb_t y, gr_ctx_t ctx) { return gr_divexact_ui(res, x, y, ctx); }
int gr_divexact_si__extern(gr_ptr res, gr_srcptr x, mp_limb_signed_t y, gr_ctx_t ctx) { return gr_divexact_si(res, x, y, ctx); }
int gr_divexact_fmpz__extern(gr_ptr res, gr_srcptr x, const fmpz_t y, gr_ctx_t ctx) { return gr_divexact_fmpz(res, x, y, ctx); }
int gr_divexact_fmpq__extern(gr_ptr res, gr_srcptr x, const fmpq_t y, gr_ctx_t ctx) { return gr_divexact_fmpq(res, x, y, ctx); }
int gr_divexact_other__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_ctx_t y_ctx, gr_ctx_t ctx) { return gr_divexact_other(res, x, y, y_ctx, ctx); }
int gr_other_divexact__extern(gr_ptr res, gr_srcptr x, gr_ctx_t x_ctx, gr_srcptr y, gr_ctx_t ctx) { return gr_other_divexact(res, x, x_ctx, y, ctx); }
int gr_euclidean_div__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_ctx_t ctx) { return gr_euclidean_div(res, x, y, ctx); }
int gr_euclidean_rem__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_ctx_t ctx) { return gr_euclidean_rem(res, x, y, ctx); }
int gr_euclidean_divrem__extern(gr_ptr res1, gr_ptr res2, gr_srcptr x, gr_srcptr y, gr_ctx_t ctx) { return gr_euclidean_divrem(res1, res2, x, y, ctx); }
int gr_gcd__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_ctx_t ctx) { return gr_gcd(res, x, y, ctx); }
int gr_lcm__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_ctx_t ctx) { return gr_lcm(res, x, y, ctx); }
int gr_numerator__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_numerator(res, x, ctx); }
int gr_denominator__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_denominator(res, x, ctx); }
int gr_factor__extern(gr_ptr c, gr_vec_t factors, gr_vec_t exponents, gr_srcptr x, int flags, gr_ctx_t ctx) { return gr_factor(c, factors, exponents, x, flags, ctx); }
int gr_pow__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_ctx_t ctx) { return gr_pow(res, x, y, ctx); }
int gr_pow_ui__extern(gr_ptr res, gr_srcptr x, mp_limb_t y, gr_ctx_t ctx) { return gr_pow_ui(res, x, y, ctx); }
int gr_pow_si__extern(gr_ptr res, gr_srcptr x, mp_limb_signed_t y, gr_ctx_t ctx) { return gr_pow_si(res, x, y, ctx); }
int gr_pow_fmpz__extern(gr_ptr res, gr_srcptr x, const fmpz_t y, gr_ctx_t ctx) { return gr_pow_fmpz(res, x, y, ctx); }
int gr_pow_fmpq__extern(gr_ptr res, gr_srcptr x, const fmpq_t y, gr_ctx_t ctx) { return gr_pow_fmpq(res, x, y, ctx); }
int gr_pow_other__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_ctx_t y_ctx, gr_ctx_t ctx) { return gr_pow_other(res, x, y, y_ctx, ctx); }
int gr_other_pow__extern(gr_ptr res, gr_srcptr x, gr_ctx_t x_ctx, gr_srcptr y, gr_ctx_t ctx) { return gr_other_pow(res, x, x_ctx, y, ctx); }
int gr_sqrt__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_sqrt(res, x, ctx); }
int gr_rsqrt__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_rsqrt(res, x, ctx); }
truth_t gr_is_square__extern(gr_srcptr x, gr_ctx_t ctx) { return gr_is_square(x, ctx); }
int gr_floor__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_floor(res, x, ctx); }
int gr_ceil__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_ceil(res, x, ctx); }
int gr_trunc__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_trunc(res, x, ctx); }
int gr_nint__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_nint(res, x, ctx); }
int gr_abs__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_abs(res, x, ctx); }
int gr_i__extern(gr_ptr res, gr_ctx_t ctx) { return gr_i(res, ctx); }
int gr_conj__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_conj(res, x, ctx); }
int gr_re__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_re(res, x, ctx); }
int gr_im__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_im(res, x, ctx); }
int gr_sgn__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_sgn(res, x, ctx); }
int gr_csgn__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_csgn(res, x, ctx); }
int gr_arg__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_arg(res, x, ctx); }
int gr_pos_inf__extern(gr_ptr res, gr_ctx_t ctx) { return gr_pos_inf(res, ctx); }
int gr_neg_inf__extern(gr_ptr res, gr_ctx_t ctx) { return gr_neg_inf(res, ctx); }
int gr_uinf__extern(gr_ptr res, gr_ctx_t ctx) { return gr_uinf(res, ctx); }
int gr_undefined__extern(gr_ptr res, gr_ctx_t ctx) { return gr_undefined(res, ctx); }
int gr_unknown__extern(gr_ptr res, gr_ctx_t ctx) { return gr_unknown(res, ctx); }
int gr_cmp__extern(int *res, gr_srcptr x, gr_srcptr y, gr_ctx_t ctx) { return gr_cmp(res, x, y, ctx); }
int gr_cmpabs__extern(int *res, gr_srcptr x, gr_srcptr y, gr_ctx_t ctx) { return gr_cmpabs(res, x, y, ctx); }
int gr_cmp_other__extern(int *res, gr_srcptr x, gr_srcptr y, gr_ctx_t y_ctx, gr_ctx_t ctx) { return gr_cmp_other(res, x, y, y_ctx, ctx); }
int gr_cmpabs_other__extern(int *res, gr_srcptr x, gr_srcptr y, gr_ctx_t y_ctx, gr_ctx_t ctx) { return gr_cmpabs_other(res, x, y, y_ctx, ctx); }
int gr_gen__extern(gr_ptr res, gr_ctx_t ctx) { return gr_gen(res, ctx); }
int gr_gens__extern(gr_vec_t res, gr_ctx_t ctx) { return gr_gens(res, ctx); }
int gr_gens_recursive__extern(gr_vec_t res, gr_ctx_t ctx) { return gr_gens_recursive(res, ctx); }
int gr_ctx_fq_prime__extern(fmpz_t res, gr_ctx_t ctx) { return gr_ctx_fq_prime(res, ctx); }
int gr_ctx_fq_degree__extern(mp_limb_signed_t *res, gr_ctx_t ctx) { return gr_ctx_fq_degree(res, ctx); }
int gr_ctx_fq_order__extern(fmpz_t res, gr_ctx_t ctx) { return gr_ctx_fq_order(res, ctx); }
int gr_fq_frobenius__extern(gr_ptr res, gr_srcptr x, mp_limb_signed_t e, gr_ctx_t ctx) { return gr_fq_frobenius(res, x, e, ctx); }
int gr_fq_multiplicative_order__extern(fmpz_t res, gr_srcptr x, gr_ctx_t ctx) { return gr_fq_multiplicative_order(res, x, ctx); }
int gr_fq_norm__extern(fmpz_t res, gr_srcptr x, gr_ctx_t ctx) { return gr_fq_norm(res, x, ctx); }
int gr_fq_trace__extern(fmpz_t res, gr_srcptr x, gr_ctx_t ctx) { return gr_fq_trace(res, x, ctx); }
truth_t gr_fq_is_primitive__extern(gr_srcptr x, gr_ctx_t ctx) { return gr_fq_is_primitive(x, ctx); }
int gr_fq_pth_root__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_fq_pth_root(res, x, ctx); }
int gr_set_interval_mid_rad__extern(gr_ptr res, gr_srcptr m, gr_srcptr r, gr_ctx_t ctx) { return gr_set_interval_mid_rad(res, m, r, ctx); }
void _gr_vec_init__extern(gr_ptr vec, mp_limb_signed_t len, gr_ctx_t ctx) { _gr_vec_init(vec, len, ctx); }
void _gr_vec_clear__extern(gr_ptr vec, mp_limb_signed_t len, gr_ctx_t ctx) { _gr_vec_clear(vec, len, ctx); }
void _gr_vec_swap__extern(gr_ptr vec1, gr_ptr vec2, mp_limb_signed_t len, gr_ctx_t ctx) { _gr_vec_swap(vec1, vec2, len, ctx); }
gr_ptr gr_heap_init__extern(gr_ctx_t ctx) { return gr_heap_init(ctx); }
void gr_heap_clear__extern(gr_ptr x, gr_ctx_t ctx) { gr_heap_clear(x, ctx); }
gr_ptr gr_heap_init_vec__extern(mp_limb_signed_t len, gr_ctx_t ctx) { return gr_heap_init_vec(len, ctx); }
void gr_heap_clear_vec__extern(gr_ptr x, mp_limb_signed_t len, gr_ctx_t ctx) { gr_heap_clear_vec(x, len, ctx); }
void gr_ctx_init_matrix_ring__extern(gr_ctx_t ctx, gr_ctx_t base_ring, mp_limb_signed_t n) { gr_ctx_init_matrix_ring(ctx, base_ring, n); }

gr_ptr gr_mat_entry_ptr__extern(gr_mat_t mat, mp_limb_signed_t i, mp_limb_signed_t j, gr_ctx_t ctx) { return gr_mat_entry_ptr(mat, i, j, ctx); }
gr_srcptr gr_mat_entry_srcptr__extern(const gr_mat_t mat, mp_limb_signed_t i, mp_limb_signed_t j, gr_ctx_t ctx) { return gr_mat_entry_srcptr(mat, i, j, ctx); }
void gr_mat_swap__extern(gr_mat_t mat1, gr_mat_t mat2, gr_ctx_t ctx) { gr_mat_swap(mat1, mat2, ctx); }
void gr_mat_window_clear__extern(gr_mat_t window, gr_ctx_t ctx) { gr_mat_window_clear(window, ctx); }
truth_t gr_mat_is_empty__extern(const gr_mat_t mat, gr_ctx_t ctx) { return gr_mat_is_empty(mat, ctx); }
truth_t gr_mat_is_square__extern(const gr_mat_t mat, gr_ctx_t ctx) { return gr_mat_is_square(mat, ctx); }
int gr_mat_sqr__extern(gr_mat_t res, const gr_mat_t mat, gr_ctx_t ctx) { return gr_mat_sqr(res, mat, ctx); }

void gr_mpoly_init__extern(gr_mpoly_t A, const mpoly_ctx_t mctx, gr_ctx_t cctx) { gr_mpoly_init(A, mctx, cctx); }
void gr_mpoly_clear__extern(gr_mpoly_t A, const mpoly_ctx_t mctx, gr_ctx_t cctx) { gr_mpoly_clear(A, mctx, cctx); }
void _gr_mpoly_set_length__extern(gr_mpoly_t A, mp_limb_signed_t newlen, const mpoly_ctx_t mctx, gr_ctx_t cctx) { _gr_mpoly_set_length(A, newlen, mctx, cctx); }
void gr_mpoly_swap__extern(gr_mpoly_t A, gr_mpoly_t B, const mpoly_ctx_t mctx, gr_ctx_t cctx) { gr_mpoly_swap(A, B, mctx, cctx); }
int gr_mpoly_zero__extern(gr_mpoly_t A, const mpoly_ctx_t mctx, gr_ctx_t cctx) { return gr_mpoly_zero(A, mctx, cctx); }
truth_t gr_mpoly_is_zero__extern(const gr_mpoly_t A, const mpoly_ctx_t mctx, gr_ctx_t cctx) { return gr_mpoly_is_zero(A, mctx, cctx); }
int gr_mpoly_one__extern(gr_mpoly_t A, const mpoly_ctx_t mctx, gr_ctx_t cctx) { return gr_mpoly_one(A, mctx, cctx); }
truth_t gr_mpoly_is_one__extern(const gr_mpoly_t A, const mpoly_ctx_t mctx, gr_ctx_t cctx) { return gr_mpoly_is_one(A, mctx, cctx); }

gr_ptr gr_poly_entry_ptr__extern(gr_poly_t poly, mp_limb_signed_t i, gr_ctx_t ctx) { return gr_poly_entry_ptr(poly, i, ctx); }
gr_srcptr gr_poly_entry_srcptr__extern(const gr_poly_t poly, mp_limb_signed_t i, gr_ctx_t ctx) { return gr_poly_entry_srcptr(poly, i, ctx); }
mp_limb_signed_t gr_poly_length__extern(const gr_poly_t poly, gr_ctx_t ctx) { return gr_poly_length(poly, ctx); }
void gr_poly_swap__extern(gr_poly_t poly1, gr_poly_t poly2, gr_ctx_t ctx) { gr_poly_swap(poly1, poly2, ctx); }
int gr_poly_zero__extern(gr_poly_t poly, gr_ctx_t ctx) { return gr_poly_zero(poly, ctx); }
int _gr_poly_mullow__extern(gr_ptr res, gr_srcptr poly1, mp_limb_signed_t len1, gr_srcptr poly2, mp_limb_signed_t len2, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_poly_mullow(res, poly1, len1, poly2, len2, len, ctx); }
int _gr_poly_divrem__extern(gr_ptr Q, gr_ptr R, gr_srcptr A, mp_limb_signed_t lenA, gr_srcptr B, mp_limb_signed_t lenB, gr_ctx_t ctx) { return _gr_poly_divrem(Q, R, A, lenA, B, lenB, ctx); }
int _gr_poly_div__extern(gr_ptr Q, gr_srcptr A, mp_limb_signed_t lenA, gr_srcptr B, mp_limb_signed_t lenB, gr_ctx_t ctx) { return _gr_poly_div(Q, A, lenA, B, lenB, ctx); }
int _gr_poly_inv_series_basecase__extern(gr_ptr res, gr_srcptr f, mp_limb_signed_t flen, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_poly_inv_series_basecase(res, f, flen, len, ctx); }
int _gr_poly_inv_series__extern(gr_ptr res, gr_srcptr f, mp_limb_signed_t flen, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_poly_inv_series(res, f, flen, len, ctx); }
int _gr_poly_div_series_basecase__extern(gr_ptr res, gr_srcptr f, mp_limb_signed_t flen, gr_srcptr g, mp_limb_signed_t glen, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_poly_div_series_basecase(res, f, flen, g, glen, len, ctx); }
int _gr_poly_div_series__extern(gr_ptr res, gr_srcptr f, mp_limb_signed_t flen, gr_srcptr g, mp_limb_signed_t glen, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_poly_div_series(res, f, flen, g, glen, len, ctx); }
int _gr_poly_divexact__extern(gr_ptr Q, gr_srcptr A, mp_limb_signed_t lenA, gr_srcptr B, mp_limb_signed_t lenB, gr_ctx_t ctx) { return _gr_poly_divexact(Q, A, lenA, B, lenB, ctx); }
int _gr_poly_sqrt_series__extern(gr_ptr res, gr_srcptr f, mp_limb_signed_t flen, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_poly_sqrt_series(res, f, flen, len, ctx); }
int _gr_poly_rsqrt_series__extern(gr_ptr res, gr_srcptr f, mp_limb_signed_t flen, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_poly_rsqrt_series(res, f, flen, len, ctx); }
int _gr_poly_taylor_shift__extern(gr_ptr res, gr_srcptr f, mp_limb_signed_t len, gr_srcptr c, gr_ctx_t ctx) { return _gr_poly_taylor_shift(res, f, len, c, ctx); }
int gr_poly_roots__extern(gr_vec_t roots, gr_vec_t mult, const gr_poly_t poly, int flags, gr_ctx_t ctx) { return gr_poly_roots(roots, mult, poly, flags, ctx); }
int gr_poly_roots_other__extern(gr_vec_t roots, gr_vec_t mult, const gr_poly_t poly, gr_ctx_t poly_ctx, int flags, gr_ctx_t ctx) { return gr_poly_roots_other(roots, mult, poly, poly_ctx, flags, ctx); }
int _gr_poly_exp_series__extern(gr_ptr res, gr_srcptr f, mp_limb_signed_t flen, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_poly_exp_series(res, f, flen, len, ctx); }

int gr_pi__extern(gr_ptr res, gr_ctx_t ctx) { return gr_pi(res, ctx); }
int gr_euler__extern(gr_ptr res, gr_ctx_t ctx) { return gr_euler(res, ctx); }
int gr_catalan__extern(gr_ptr res, gr_ctx_t ctx) { return gr_catalan(res, ctx); }
int gr_khinchin__extern(gr_ptr res, gr_ctx_t ctx) { return gr_khinchin(res, ctx); }
int gr_glaisher__extern(gr_ptr res, gr_ctx_t ctx) { return gr_glaisher(res, ctx); }
int gr_exp__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_exp(res, x, ctx); }
int gr_expm1__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_expm1(res, x, ctx); }
int gr_exp2__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_exp2(res, x, ctx); }
int gr_exp10__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_exp10(res, x, ctx); }
int gr_exp_pi_i__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_exp_pi_i(res, x, ctx); }
int gr_log__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_log(res, x, ctx); }
int gr_log1p__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_log1p(res, x, ctx); }
int gr_log_pi_i__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_log_pi_i(res, x, ctx); }
int gr_log2__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_log2(res, x, ctx); }
int gr_log10__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_log10(res, x, ctx); }
int gr_sin__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_sin(res, x, ctx); }
int gr_cos__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_cos(res, x, ctx); }
int gr_sin_cos__extern(gr_ptr res1, gr_ptr res2, gr_srcptr x, gr_ctx_t ctx) { return gr_sin_cos(res1, res2, x, ctx); }
int gr_tan__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_tan(res, x, ctx); }
int gr_cot__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_cot(res, x, ctx); }
int gr_sec__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_sec(res, x, ctx); }
int gr_csc__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_csc(res, x, ctx); }
int gr_sin_pi__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_sin_pi(res, x, ctx); }
int gr_cos_pi__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_cos_pi(res, x, ctx); }
int gr_sin_cos_pi__extern(gr_ptr res1, gr_ptr res2, gr_srcptr x, gr_ctx_t ctx) { return gr_sin_cos_pi(res1, res2, x, ctx); }
int gr_tan_pi__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_tan_pi(res, x, ctx); }
int gr_cot_pi__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_cot_pi(res, x, ctx); }
int gr_sec_pi__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_sec_pi(res, x, ctx); }
int gr_csc_pi__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_csc_pi(res, x, ctx); }
int gr_sinc__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_sinc(res, x, ctx); }
int gr_sinc_pi__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_sinc_pi(res, x, ctx); }
int gr_sinh__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_sinh(res, x, ctx); }
int gr_cosh__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_cosh(res, x, ctx); }
int gr_sinh_cosh__extern(gr_ptr res1, gr_ptr res2, gr_srcptr x, gr_ctx_t ctx) { return gr_sinh_cosh(res1, res2, x, ctx); }
int gr_tanh__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_tanh(res, x, ctx); }
int gr_coth__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_coth(res, x, ctx); }
int gr_sech__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_sech(res, x, ctx); }
int gr_csch__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_csch(res, x, ctx); }
int gr_asin__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_asin(res, x, ctx); }
int gr_acos__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_acos(res, x, ctx); }
int gr_atan__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_atan(res, x, ctx); }
int gr_atan2__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_ctx_t ctx) { return gr_atan2(res, x, y, ctx); }
int gr_acot__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_acot(res, x, ctx); }
int gr_asec__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_asec(res, x, ctx); }
int gr_acsc__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_acsc(res, x, ctx); }
int gr_asin_pi__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_asin_pi(res, x, ctx); }
int gr_acos_pi__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_acos_pi(res, x, ctx); }
int gr_atan_pi__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_atan_pi(res, x, ctx); }
int gr_acot_pi__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_acot_pi(res, x, ctx); }
int gr_asec_pi__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_asec_pi(res, x, ctx); }
int gr_acsc_pi__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_acsc_pi(res, x, ctx); }
int gr_asinh__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_asinh(res, x, ctx); }
int gr_acosh__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_acosh(res, x, ctx); }
int gr_atanh__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_atanh(res, x, ctx); }
int gr_acoth__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_acoth(res, x, ctx); }
int gr_asech__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_asech(res, x, ctx); }
int gr_acsch__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_acsch(res, x, ctx); }
int gr_lambertw__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_lambertw(res, x, ctx); }
int gr_lambertw_fmpz__extern(gr_ptr res, gr_srcptr x, const fmpz_t k, gr_ctx_t ctx) { return gr_lambertw_fmpz(res, x, k, ctx); }
int gr_fac__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_fac(res, x, ctx); }
int gr_fac_ui__extern(gr_ptr res, mp_limb_t x, gr_ctx_t ctx) { return gr_fac_ui(res, x, ctx); }
int gr_fac_fmpz__extern(gr_ptr res, const fmpz_t x, gr_ctx_t ctx) { return gr_fac_fmpz(res, x, ctx); }
int gr_fac_vec__extern(gr_ptr res, mp_limb_signed_t len, gr_ctx_t ctx) { return gr_fac_vec(res, len, ctx); }
int gr_rfac__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_rfac(res, x, ctx); }
int gr_rfac_ui__extern(gr_ptr res, mp_limb_t x, gr_ctx_t ctx) { return gr_rfac_ui(res, x, ctx); }
int gr_rfac_fmpz__extern(gr_ptr res, const fmpz_t x, gr_ctx_t ctx) { return gr_rfac_fmpz(res, x, ctx); }
int gr_rfac_vec__extern(gr_ptr res, mp_limb_signed_t len, gr_ctx_t ctx) { return gr_rfac_vec(res, len, ctx); }
int gr_bin__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_ctx_t ctx) { return gr_bin(res, x, y, ctx); }
int gr_bin_ui__extern(gr_ptr res, gr_srcptr x, mp_limb_t y, gr_ctx_t ctx) { return gr_bin_ui(res, x, y, ctx); }
int gr_bin_uiui__extern(gr_ptr res, mp_limb_t x, mp_limb_t y, gr_ctx_t ctx) { return gr_bin_uiui(res, x, y, ctx); }
int gr_bin_vec__extern(gr_ptr res, gr_srcptr x, mp_limb_signed_t len, gr_ctx_t ctx) { return gr_bin_vec(res, x, len, ctx); }
int gr_bin_ui_vec__extern(gr_ptr res, mp_limb_t x, mp_limb_signed_t len, gr_ctx_t ctx) { return gr_bin_ui_vec(res, x, len, ctx); }
int gr_rising__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_ctx_t ctx) { return gr_rising(res, x, y, ctx); }
int gr_rising_ui__extern(gr_ptr res, gr_srcptr x, mp_limb_t y, gr_ctx_t ctx) { return gr_rising_ui(res, x, y, ctx); }
int gr_falling__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_ctx_t ctx) { return gr_falling(res, x, y, ctx); }
int gr_falling_ui__extern(gr_ptr res, gr_srcptr x, mp_limb_t y, gr_ctx_t ctx) { return gr_falling_ui(res, x, y, ctx); }
int gr_gamma__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_gamma(res, x, ctx); }
int gr_gamma_fmpz__extern(gr_ptr res, const fmpz_t x, gr_ctx_t ctx) { return gr_gamma_fmpz(res, x, ctx); }
int gr_gamma_fmpq__extern(gr_ptr res, const fmpq_t x, gr_ctx_t ctx) { return gr_gamma_fmpq(res, x, ctx); }
int gr_rgamma__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_rgamma(res, x, ctx); }
int gr_lgamma__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_lgamma(res, x, ctx); }
int gr_digamma__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_digamma(res, x, ctx); }
int gr_beta__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_ctx_t ctx) { return gr_beta(res, x, y, ctx); }
int gr_barnes_g__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_barnes_g(res, x, ctx); }
int gr_log_barnes_g__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_log_barnes_g(res, x, ctx); }
int gr_doublefac__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_doublefac(res, x, ctx); }
int gr_doublefac_ui__extern(gr_ptr res, mp_limb_t x, gr_ctx_t ctx) { return gr_doublefac_ui(res, x, ctx); }
int gr_harmonic__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_harmonic(res, x, ctx); }
int gr_harmonic_ui__extern(gr_ptr res, mp_limb_t x, gr_ctx_t ctx) { return gr_harmonic_ui(res, x, ctx); }
int gr_bernoulli_ui__extern(gr_ptr res, mp_limb_t x, gr_ctx_t ctx) { return gr_bernoulli_ui(res, x, ctx); }
int gr_bernoulli_fmpz__extern(gr_ptr res, const fmpz_t x, gr_ctx_t ctx) { return gr_bernoulli_fmpz(res, x, ctx); }
int gr_bernoulli_vec__extern(gr_ptr res, mp_limb_signed_t len, gr_ctx_t ctx) { return gr_bernoulli_vec(res, len, ctx); }
int gr_fib_ui__extern(gr_ptr res, mp_limb_t x, gr_ctx_t ctx) { return gr_fib_ui(res, x, ctx); }
int gr_fib_fmpz__extern(gr_ptr res, const fmpz_t x, gr_ctx_t ctx) { return gr_fib_fmpz(res, x, ctx); }
int gr_fib_vec__extern(gr_ptr res, mp_limb_signed_t len, gr_ctx_t ctx) { return gr_fib_vec(res, len, ctx); }
int gr_eulernum_ui__extern(gr_ptr res, mp_limb_t x, gr_ctx_t ctx) { return gr_eulernum_ui(res, x, ctx); }
int gr_eulernum_fmpz__extern(gr_ptr res, const fmpz_t x, gr_ctx_t ctx) { return gr_eulernum_fmpz(res, x, ctx); }
int gr_eulernum_vec__extern(gr_ptr res, mp_limb_signed_t len, gr_ctx_t ctx) { return gr_eulernum_vec(res, len, ctx); }
int gr_bernpoly_ui__extern(gr_ptr res, mp_limb_t n, gr_srcptr x, gr_ctx_t ctx) { return gr_bernpoly_ui(res, n, x, ctx); }
int gr_eulerpoly_ui__extern(gr_ptr res, mp_limb_t n, gr_srcptr x, gr_ctx_t ctx) { return gr_eulerpoly_ui(res, n, x, ctx); }
int gr_bellnum_ui__extern(gr_ptr res, mp_limb_t x, gr_ctx_t ctx) { return gr_bellnum_ui(res, x, ctx); }
int gr_bellnum_fmpz__extern(gr_ptr res, const fmpz_t x, gr_ctx_t ctx) { return gr_bellnum_fmpz(res, x, ctx); }
int gr_bellnum_vec__extern(gr_ptr res, mp_limb_signed_t len, gr_ctx_t ctx) { return gr_bellnum_vec(res, len, ctx); }
int gr_stirling_s1u_uiui__extern(gr_ptr res, mp_limb_t x, mp_limb_t y, gr_ctx_t ctx) { return gr_stirling_s1u_uiui(res, x, y, ctx); }
int gr_stirling_s1_uiui__extern(gr_ptr res, mp_limb_t x, mp_limb_t y, gr_ctx_t ctx) { return gr_stirling_s1_uiui(res, x, y, ctx); }
int gr_stirling_s2_uiui__extern(gr_ptr res, mp_limb_t x, mp_limb_t y, gr_ctx_t ctx) { return gr_stirling_s2_uiui(res, x, y, ctx); }
int gr_stirling_s1u_ui_vec__extern(gr_ptr res, mp_limb_t x, mp_limb_signed_t len, gr_ctx_t ctx) { return gr_stirling_s1u_ui_vec(res, x, len, ctx); }
int gr_stirling_s1_ui_vec__extern(gr_ptr res, mp_limb_t x, mp_limb_signed_t len, gr_ctx_t ctx) { return gr_stirling_s1_ui_vec(res, x, len, ctx); }
int gr_stirling_s2_ui_vec__extern(gr_ptr res, mp_limb_t x, mp_limb_signed_t len, gr_ctx_t ctx) { return gr_stirling_s2_ui_vec(res, x, len, ctx); }
int gr_partitions_ui__extern(gr_ptr res, mp_limb_t x, gr_ctx_t ctx) { return gr_partitions_ui(res, x, ctx); }
int gr_partitions_fmpz__extern(gr_ptr res, const fmpz_t x, gr_ctx_t ctx) { return gr_partitions_fmpz(res, x, ctx); }
int gr_partitions_vec__extern(gr_ptr res, mp_limb_signed_t len, gr_ctx_t ctx) { return gr_partitions_vec(res, len, ctx); }
int gr_erf__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_erf(res, x, ctx); }
int gr_erfc__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_erfc(res, x, ctx); }
int gr_erfcx__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_erfcx(res, x, ctx); }
int gr_erfi__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_erfi(res, x, ctx); }
int gr_erfinv__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_erfinv(res, x, ctx); }
int gr_erfcinv__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_erfcinv(res, x, ctx); }
int gr_fresnel_s__extern(gr_ptr res, gr_srcptr x, int normalized, gr_ctx_t ctx) { return gr_fresnel_s(res, x, normalized, ctx); }
int gr_fresnel_c__extern(gr_ptr res, gr_srcptr x, int normalized, gr_ctx_t ctx) { return gr_fresnel_c(res, x, normalized, ctx); }
int gr_fresnel__extern(gr_ptr res1, gr_ptr res2, gr_srcptr x, int normalized, gr_ctx_t ctx) { return gr_fresnel(res1, res2, x, normalized, ctx); }
int gr_gamma_upper__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, int regularized, gr_ctx_t ctx) { return gr_gamma_upper(res, x, y, regularized, ctx); }
int gr_gamma_lower__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, int regularized, gr_ctx_t ctx) { return gr_gamma_lower(res, x, y, regularized, ctx); }
int gr_beta_lower__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_srcptr z, int regularized, gr_ctx_t ctx) { return gr_beta_lower(res, x, y, z, regularized, ctx); }
int gr_exp_integral__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_ctx_t ctx) { return gr_exp_integral(res, x, y, ctx); }
int gr_exp_integral_ei__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_exp_integral_ei(res, x, ctx); }
int gr_sin_integral__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_sin_integral(res, x, ctx); }
int gr_cos_integral__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_cos_integral(res, x, ctx); }
int gr_sinh_integral__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_sinh_integral(res, x, ctx); }
int gr_cosh_integral__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_cosh_integral(res, x, ctx); }
int gr_log_integral__extern(gr_ptr res, gr_srcptr x, int offset, gr_ctx_t ctx) { return gr_log_integral(res, x, offset, ctx); }
int gr_dilog__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_dilog(res, x, ctx); }
int gr_bessel_j__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_ctx_t ctx) { return gr_bessel_j(res, x, y, ctx); }
int gr_bessel_y__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_ctx_t ctx) { return gr_bessel_y(res, x, y, ctx); }
int gr_bessel_i__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_ctx_t ctx) { return gr_bessel_i(res, x, y, ctx); }
int gr_bessel_k__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_ctx_t ctx) { return gr_bessel_k(res, x, y, ctx); }
int gr_bessel_j_y__extern(gr_ptr res1, gr_ptr res2, gr_srcptr x, gr_srcptr y, gr_ctx_t ctx) { return gr_bessel_j_y(res1, res2, x, y, ctx); }
int gr_bessel_i_scaled__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_ctx_t ctx) { return gr_bessel_i_scaled(res, x, y, ctx); }
int gr_bessel_k_scaled__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_ctx_t ctx) { return gr_bessel_k_scaled(res, x, y, ctx); }
int gr_airy__extern(gr_ptr res1, gr_ptr res2, gr_ptr res3, gr_ptr res4, gr_srcptr x, gr_ctx_t ctx) { return gr_airy(res1, res2, res3, res4, x, ctx); }
int gr_airy_ai__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_airy_ai(res, x, ctx); }
int gr_airy_bi__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_airy_bi(res, x, ctx); }
int gr_airy_ai_prime__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_airy_ai_prime(res, x, ctx); }
int gr_airy_bi_prime__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_airy_bi_prime(res, x, ctx); }
int gr_airy_ai_zero__extern(gr_ptr res, const fmpz_t n, gr_ctx_t ctx) { return gr_airy_ai_zero(res, n, ctx); }
int gr_airy_bi_zero__extern(gr_ptr res, const fmpz_t n, gr_ctx_t ctx) { return gr_airy_bi_zero(res, n, ctx); }
int gr_airy_ai_prime_zero__extern(gr_ptr res, const fmpz_t n, gr_ctx_t ctx) { return gr_airy_ai_prime_zero(res, n, ctx); }
int gr_airy_bi_prime_zero__extern(gr_ptr res, const fmpz_t n, gr_ctx_t ctx) { return gr_airy_bi_prime_zero(res, n, ctx); }
int gr_coulomb__extern(gr_ptr res1, gr_ptr res2, gr_ptr res3, gr_ptr res4, gr_srcptr x, gr_srcptr y, gr_srcptr z, gr_ctx_t ctx) { return gr_coulomb(res1, res2, res3, res4, x, y, z, ctx); }
int gr_coulomb_f__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_srcptr z, gr_ctx_t ctx) { return gr_coulomb_f(res, x, y, z, ctx); }
int gr_coulomb_g__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_srcptr z, gr_ctx_t ctx) { return gr_coulomb_g(res, x, y, z, ctx); }
int gr_coulomb_hpos__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_srcptr z, gr_ctx_t ctx) { return gr_coulomb_hpos(res, x, y, z, ctx); }
int gr_coulomb_hneg__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_srcptr z, gr_ctx_t ctx) { return gr_coulomb_hneg(res, x, y, z, ctx); }
int gr_chebyshev_t_fmpz__extern(gr_ptr res, const fmpz_t n, gr_srcptr x, gr_ctx_t ctx) { return gr_chebyshev_t_fmpz(res, n, x, ctx); }
int gr_chebyshev_t__extern(gr_ptr res, gr_srcptr n, gr_srcptr x, gr_ctx_t ctx) { return gr_chebyshev_t(res, n, x, ctx); }
int gr_chebyshev_u_fmpz__extern(gr_ptr res, const fmpz_t n, gr_srcptr x, gr_ctx_t ctx) { return gr_chebyshev_u_fmpz(res, n, x, ctx); }
int gr_chebyshev_u__extern(gr_ptr res, gr_srcptr n, gr_srcptr x, gr_ctx_t ctx) { return gr_chebyshev_u(res, n, x, ctx); }
int gr_jacobi_p__extern(gr_ptr res, gr_srcptr n, gr_srcptr a, gr_srcptr b, gr_srcptr z, gr_ctx_t ctx) { return gr_jacobi_p(res, n, a, b, z, ctx); }
int gr_gegenbauer_c__extern(gr_ptr res, gr_srcptr n, gr_srcptr m, gr_srcptr z, gr_ctx_t ctx) { return gr_gegenbauer_c(res, n, m, z, ctx); }
int gr_laguerre_l__extern(gr_ptr res, gr_srcptr n, gr_srcptr m, gr_srcptr z, gr_ctx_t ctx) { return gr_laguerre_l(res, n, m, z, ctx); }
int gr_hermite_h__extern(gr_ptr res, gr_srcptr n, gr_srcptr z, gr_ctx_t ctx) { return gr_hermite_h(res, n, z, ctx); }
int gr_legendre_p__extern(gr_ptr res, gr_srcptr n, gr_srcptr m, gr_srcptr z, int type, gr_ctx_t ctx) { return gr_legendre_p(res, n, m, z, type, ctx); }
int gr_legendre_q__extern(gr_ptr res, gr_srcptr n, gr_srcptr m, gr_srcptr z, int type, gr_ctx_t ctx) { return gr_legendre_q(res, n, m, z, type, ctx); }
int gr_spherical_y_si__extern(gr_ptr res, mp_limb_signed_t n, mp_limb_signed_t m, gr_srcptr theta, gr_srcptr phi, gr_ctx_t ctx) { return gr_spherical_y_si(res, n, m, theta, phi, ctx); }
int gr_legendre_p_root_ui__extern(gr_ptr root, gr_ptr weight, mp_limb_t n, mp_limb_t k, gr_ctx_t ctx) { return gr_legendre_p_root_ui(root, weight, n, k, ctx); }
int gr_hypgeom_0f1__extern(gr_ptr res, gr_srcptr a, gr_srcptr z, int flags, gr_ctx_t ctx) { return gr_hypgeom_0f1(res, a, z, flags, ctx); }
int gr_hypgeom_1f1__extern(gr_ptr res, gr_srcptr a, gr_srcptr b, gr_srcptr z, int flags, gr_ctx_t ctx) { return gr_hypgeom_1f1(res, a, b, z, flags, ctx); }
int gr_hypgeom_u__extern(gr_ptr res, gr_srcptr a, gr_srcptr b, gr_srcptr z, int flags, gr_ctx_t ctx) { return gr_hypgeom_u(res, a, b, z, flags, ctx); }
int gr_hypgeom_2f1__extern(gr_ptr res, gr_srcptr a, gr_srcptr b, gr_srcptr c, gr_srcptr z, int flags, gr_ctx_t ctx) { return gr_hypgeom_2f1(res, a, b, c, z, flags, ctx); }
int gr_hypgeom_pfq__extern(gr_ptr res, const gr_vec_t a, const gr_vec_t b, gr_srcptr z, int flags, gr_ctx_t ctx) { return gr_hypgeom_pfq(res, a, b, z, flags, ctx); }
int gr_zeta__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_zeta(res, x, ctx); }
int gr_zeta_ui__extern(gr_ptr res, mp_limb_t x, gr_ctx_t ctx) { return gr_zeta_ui(res, x, ctx); }
int gr_hurwitz_zeta__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_ctx_t ctx) { return gr_hurwitz_zeta(res, x, y, ctx); }
int gr_polylog__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_ctx_t ctx) { return gr_polylog(res, x, y, ctx); }
int gr_polygamma__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_ctx_t ctx) { return gr_polygamma(res, x, y, ctx); }
int gr_lerch_phi__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_srcptr z, gr_ctx_t ctx) { return gr_lerch_phi(res, x, y, z, ctx); }
int gr_stieltjes__extern(gr_ptr res, const fmpz_t x, gr_srcptr y, gr_ctx_t ctx) { return gr_stieltjes(res, x, y, ctx); }
int gr_dirichlet_eta__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_dirichlet_eta(res, x, ctx); }
int gr_dirichlet_beta__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_dirichlet_beta(res, x, ctx); }
int gr_riemann_xi__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_riemann_xi(res, x, ctx); }
int gr_zeta_zero__extern(gr_ptr res, const fmpz_t n, gr_ctx_t ctx) { return gr_zeta_zero(res, n, ctx); }
int gr_zeta_zero_vec__extern(gr_ptr res, const fmpz_t n, mp_limb_signed_t len, gr_ctx_t ctx) { return gr_zeta_zero_vec(res, n, len, ctx); }
int gr_zeta_nzeros__extern(gr_ptr res, gr_srcptr t, gr_ctx_t ctx) { return gr_zeta_nzeros(res, t, ctx); }
int gr_jacobi_theta__extern(gr_ptr res1, gr_ptr res2, gr_ptr res3, gr_ptr res4, gr_srcptr z, gr_srcptr tau, gr_ctx_t ctx) { return gr_jacobi_theta(res1, res2, res3, res4, z, tau, ctx); }
int gr_jacobi_theta_1__extern(gr_ptr res, gr_srcptr z, gr_srcptr tau, gr_ctx_t ctx) { return gr_jacobi_theta_1(res, z, tau, ctx); }
int gr_jacobi_theta_2__extern(gr_ptr res, gr_srcptr z, gr_srcptr tau, gr_ctx_t ctx) { return gr_jacobi_theta_2(res, z, tau, ctx); }
int gr_jacobi_theta_3__extern(gr_ptr res, gr_srcptr z, gr_srcptr tau, gr_ctx_t ctx) { return gr_jacobi_theta_3(res, z, tau, ctx); }
int gr_jacobi_theta_4__extern(gr_ptr res, gr_srcptr z, gr_srcptr tau, gr_ctx_t ctx) { return gr_jacobi_theta_4(res, z, tau, ctx); }
int gr_modular_j__extern(gr_ptr res, gr_srcptr tau, gr_ctx_t ctx) { return gr_modular_j(res, tau, ctx); }
int gr_modular_lambda__extern(gr_ptr res, gr_srcptr tau, gr_ctx_t ctx) { return gr_modular_lambda(res, tau, ctx); }
int gr_modular_delta__extern(gr_ptr res, gr_srcptr tau, gr_ctx_t ctx) { return gr_modular_delta(res, tau, ctx); }
int gr_hilbert_class_poly__extern(gr_ptr res, mp_limb_signed_t D, gr_srcptr x, gr_ctx_t ctx) { return gr_hilbert_class_poly(res, D, x, ctx); }
int gr_dedekind_eta__extern(gr_ptr res, gr_srcptr tau, gr_ctx_t ctx) { return gr_dedekind_eta(res, tau, ctx); }
int gr_dedekind_eta_q__extern(gr_ptr res, gr_srcptr tau, gr_ctx_t ctx) { return gr_dedekind_eta_q(res, tau, ctx); }
int gr_eisenstein_e__extern(gr_ptr res, mp_limb_t n, gr_srcptr tau, gr_ctx_t ctx) { return gr_eisenstein_e(res, n, tau, ctx); }
int gr_eisenstein_g__extern(gr_ptr res, mp_limb_t n, gr_srcptr tau, gr_ctx_t ctx) { return gr_eisenstein_g(res, n, tau, ctx); }
int gr_eisenstein_g_vec__extern(gr_ptr res, gr_srcptr tau, mp_limb_signed_t len, gr_ctx_t ctx) { return gr_eisenstein_g_vec(res, tau, len, ctx); }
int gr_agm1__extern(gr_ptr res, gr_srcptr x, gr_ctx_t ctx) { return gr_agm1(res, x, ctx); }
int gr_agm__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_ctx_t ctx) { return gr_agm(res, x, y, ctx); }
int gr_elliptic_k__extern(gr_ptr res, gr_srcptr m, gr_ctx_t ctx) { return gr_elliptic_k(res, m, ctx); }
int gr_elliptic_e__extern(gr_ptr res, gr_srcptr m, gr_ctx_t ctx) { return gr_elliptic_e(res, m, ctx); }
int gr_elliptic_pi__extern(gr_ptr res, gr_srcptr n, gr_srcptr m, gr_ctx_t ctx) { return gr_elliptic_pi(res, n, m, ctx); }
int gr_elliptic_f__extern(gr_ptr res, gr_srcptr phi, gr_srcptr m, int pi, gr_ctx_t ctx) { return gr_elliptic_f(res, phi, m, pi, ctx); }
int gr_elliptic_e_inc__extern(gr_ptr res, gr_srcptr phi, gr_srcptr m, int pi, gr_ctx_t ctx) { return gr_elliptic_e_inc(res, phi, m, pi, ctx); }
int gr_elliptic_pi_inc__extern(gr_ptr res, gr_srcptr n, gr_srcptr phi, gr_srcptr m, int pi, gr_ctx_t ctx) { return gr_elliptic_pi_inc(res, n, phi, m, pi, ctx); }
int gr_carlson_rc__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, int flags, gr_ctx_t ctx) { return gr_carlson_rc(res, x, y, flags, ctx); }
int gr_carlson_rf__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_srcptr z, int flags, gr_ctx_t ctx) { return gr_carlson_rf(res, x, y, z, flags, ctx); }
int gr_carlson_rd__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_srcptr z, int flags, gr_ctx_t ctx) { return gr_carlson_rd(res, x, y, z, flags, ctx); }
int gr_carlson_rg__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_srcptr z, int flags, gr_ctx_t ctx) { return gr_carlson_rg(res, x, y, z, flags, ctx); }
int gr_carlson_rj__extern(gr_ptr res, gr_srcptr x, gr_srcptr y, gr_srcptr z, gr_srcptr w, int flags, gr_ctx_t ctx) { return gr_carlson_rj(res, x, y, z, w, flags, ctx); }
int gr_elliptic_invariants__extern(gr_ptr res1, gr_ptr res2, gr_srcptr tau, gr_ctx_t ctx) { return gr_elliptic_invariants(res1, res2, tau, ctx); }
int gr_elliptic_roots__extern(gr_ptr res1, gr_ptr res2, gr_ptr res3, gr_srcptr tau, gr_ctx_t ctx) { return gr_elliptic_roots(res1, res2, res3, tau, ctx); }
int gr_weierstrass_p__extern(gr_ptr res, gr_srcptr z, gr_srcptr tau, gr_ctx_t ctx) { return gr_weierstrass_p(res, z, tau, ctx); }
int gr_weierstrass_p_prime__extern(gr_ptr res, gr_srcptr z, gr_srcptr tau, gr_ctx_t ctx) { return gr_weierstrass_p_prime(res, z, tau, ctx); }
int gr_weierstrass_p_inv__extern(gr_ptr res, gr_srcptr z, gr_srcptr tau, gr_ctx_t ctx) { return gr_weierstrass_p_inv(res, z, tau, ctx); }
int gr_weierstrass_zeta__extern(gr_ptr res, gr_srcptr z, gr_srcptr tau, gr_ctx_t ctx) { return gr_weierstrass_zeta(res, z, tau, ctx); }
int gr_weierstrass_sigma__extern(gr_ptr res, gr_srcptr z, gr_srcptr tau, gr_ctx_t ctx) { return gr_weierstrass_sigma(res, z, tau, ctx); }

gr_ptr gr_vec_entry_ptr__extern(gr_vec_t vec, mp_limb_signed_t i, gr_ctx_t ctx) { return gr_vec_entry_ptr(vec, i, ctx); }
gr_srcptr gr_vec_entry_srcptr__extern(const gr_vec_t vec, mp_limb_signed_t i, gr_ctx_t ctx) { return gr_vec_entry_srcptr(vec, i, ctx); }
mp_limb_signed_t gr_vec_length__extern(const gr_vec_t vec, gr_ctx_t ctx) { return gr_vec_length(vec, ctx); }
int _gr_vec_zero__extern(gr_ptr vec, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_vec_zero(vec, len, ctx); }
int _gr_vec_set__extern(gr_ptr res, gr_srcptr src, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_vec_set(res, src, len, ctx); }
int _gr_vec_neg__extern(gr_ptr res, gr_srcptr src, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_vec_neg(res, src, len, ctx); }
int _gr_vec_normalise__extern(mp_limb_signed_t *res, gr_srcptr vec, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_vec_normalise(res, vec, len, ctx); }
mp_limb_signed_t _gr_vec_normalise_weak__extern(gr_srcptr vec, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_vec_normalise_weak(vec, len, ctx); }
int _gr_vec_add__extern(gr_ptr res, gr_srcptr src1, gr_srcptr src2, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_vec_add(res, src1, src2, len, ctx); }
int _gr_vec_sub__extern(gr_ptr res, gr_srcptr src1, gr_srcptr src2, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_vec_sub(res, src1, src2, len, ctx); }
int _gr_vec_mul__extern(gr_ptr res, gr_srcptr src1, gr_srcptr src2, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_vec_mul(res, src1, src2, len, ctx); }
int _gr_vec_div__extern(gr_ptr res, gr_srcptr src1, gr_srcptr src2, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_vec_div(res, src1, src2, len, ctx); }
int _gr_vec_divexact__extern(gr_ptr res, gr_srcptr src1, gr_srcptr src2, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_vec_divexact(res, src1, src2, len, ctx); }
int _gr_vec_pow__extern(gr_ptr res, gr_srcptr src1, gr_srcptr src2, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_vec_pow(res, src1, src2, len, ctx); }
int _gr_vec_add_scalar__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, gr_srcptr c, gr_ctx_t ctx) { return _gr_vec_add_scalar(vec1, vec2, len, c, ctx); }
int _gr_vec_sub_scalar__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, gr_srcptr c, gr_ctx_t ctx) { return _gr_vec_sub_scalar(vec1, vec2, len, c, ctx); }
int _gr_vec_mul_scalar__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, gr_srcptr c, gr_ctx_t ctx) { return _gr_vec_mul_scalar(vec1, vec2, len, c, ctx); }
int _gr_vec_div_scalar__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, gr_srcptr c, gr_ctx_t ctx) { return _gr_vec_div_scalar(vec1, vec2, len, c, ctx); }
int _gr_vec_divexact_scalar__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, gr_srcptr c, gr_ctx_t ctx) { return _gr_vec_divexact_scalar(vec1, vec2, len, c, ctx); }
int _gr_vec_pow_scalar__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, gr_srcptr c, gr_ctx_t ctx) { return _gr_vec_pow_scalar(vec1, vec2, len, c, ctx); }
int _gr_vec_add_scalar_si__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, mp_limb_signed_t c, gr_ctx_t ctx) { return _gr_vec_add_scalar_si(vec1, vec2, len, c, ctx); }
int _gr_vec_sub_scalar_si__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, mp_limb_signed_t c, gr_ctx_t ctx) { return _gr_vec_sub_scalar_si(vec1, vec2, len, c, ctx); }
int _gr_vec_mul_scalar_si__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, mp_limb_signed_t c, gr_ctx_t ctx) { return _gr_vec_mul_scalar_si(vec1, vec2, len, c, ctx); }
int _gr_vec_div_scalar_si__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, mp_limb_signed_t c, gr_ctx_t ctx) { return _gr_vec_div_scalar_si(vec1, vec2, len, c, ctx); }
int _gr_vec_divexact_scalar_si__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, mp_limb_signed_t c, gr_ctx_t ctx) { return _gr_vec_divexact_scalar_si(vec1, vec2, len, c, ctx); }
int _gr_vec_pow_scalar_si__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, mp_limb_signed_t c, gr_ctx_t ctx) { return _gr_vec_pow_scalar_si(vec1, vec2, len, c, ctx); }
int _gr_vec_add_scalar_ui__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, mp_limb_t c, gr_ctx_t ctx) { return _gr_vec_add_scalar_ui(vec1, vec2, len, c, ctx); }
int _gr_vec_sub_scalar_ui__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, mp_limb_t c, gr_ctx_t ctx) { return _gr_vec_sub_scalar_ui(vec1, vec2, len, c, ctx); }
int _gr_vec_mul_scalar_ui__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, mp_limb_t c, gr_ctx_t ctx) { return _gr_vec_mul_scalar_ui(vec1, vec2, len, c, ctx); }
int _gr_vec_div_scalar_ui__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, mp_limb_t c, gr_ctx_t ctx) { return _gr_vec_div_scalar_ui(vec1, vec2, len, c, ctx); }
int _gr_vec_divexact_scalar_ui__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, mp_limb_t c, gr_ctx_t ctx) { return _gr_vec_divexact_scalar_ui(vec1, vec2, len, c, ctx); }
int _gr_vec_pow_scalar_ui__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, mp_limb_t c, gr_ctx_t ctx) { return _gr_vec_pow_scalar_ui(vec1, vec2, len, c, ctx); }
int _gr_vec_add_scalar_fmpz__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, const fmpz_t c, gr_ctx_t ctx) { return _gr_vec_add_scalar_fmpz(vec1, vec2, len, c, ctx); }
int _gr_vec_sub_scalar_fmpz__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, const fmpz_t c, gr_ctx_t ctx) { return _gr_vec_sub_scalar_fmpz(vec1, vec2, len, c, ctx); }
int _gr_vec_mul_scalar_fmpz__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, const fmpz_t c, gr_ctx_t ctx) { return _gr_vec_mul_scalar_fmpz(vec1, vec2, len, c, ctx); }
int _gr_vec_div_scalar_fmpz__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, const fmpz_t c, gr_ctx_t ctx) { return _gr_vec_div_scalar_fmpz(vec1, vec2, len, c, ctx); }
int _gr_vec_divexact_scalar_fmpz__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, const fmpz_t c, gr_ctx_t ctx) { return _gr_vec_divexact_scalar_fmpz(vec1, vec2, len, c, ctx); }
int _gr_vec_pow_scalar_fmpz__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, const fmpz_t c, gr_ctx_t ctx) { return _gr_vec_pow_scalar_fmpz(vec1, vec2, len, c, ctx); }
int _gr_vec_add_scalar_fmpq__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, const fmpq_t c, gr_ctx_t ctx) { return _gr_vec_add_scalar_fmpq(vec1, vec2, len, c, ctx); }
int _gr_vec_sub_scalar_fmpq__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, const fmpq_t c, gr_ctx_t ctx) { return _gr_vec_sub_scalar_fmpq(vec1, vec2, len, c, ctx); }
int _gr_vec_mul_scalar_fmpq__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, const fmpq_t c, gr_ctx_t ctx) { return _gr_vec_mul_scalar_fmpq(vec1, vec2, len, c, ctx); }
int _gr_vec_div_scalar_fmpq__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, const fmpq_t c, gr_ctx_t ctx) { return _gr_vec_div_scalar_fmpq(vec1, vec2, len, c, ctx); }
int _gr_vec_divexact_scalar_fmpq__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, const fmpq_t c, gr_ctx_t ctx) { return _gr_vec_divexact_scalar_fmpq(vec1, vec2, len, c, ctx); }
int _gr_vec_pow_scalar_fmpq__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, const fmpq_t c, gr_ctx_t ctx) { return _gr_vec_pow_scalar_fmpq(vec1, vec2, len, c, ctx); }
int _gr_scalar_add_vec__extern(gr_ptr vec1, gr_srcptr c, gr_srcptr vec2, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_scalar_add_vec(vec1, c, vec2, len, ctx); }
int _gr_scalar_sub_vec__extern(gr_ptr vec1, gr_srcptr c, gr_srcptr vec2, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_scalar_sub_vec(vec1, c, vec2, len, ctx); }
int _gr_scalar_mul_vec__extern(gr_ptr vec1, gr_srcptr c, gr_srcptr vec2, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_scalar_mul_vec(vec1, c, vec2, len, ctx); }
int _gr_scalar_div_vec__extern(gr_ptr vec1, gr_srcptr c, gr_srcptr vec2, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_scalar_div_vec(vec1, c, vec2, len, ctx); }
int _gr_scalar_divexact_vec__extern(gr_ptr vec1, gr_srcptr c, gr_srcptr vec2, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_scalar_divexact_vec(vec1, c, vec2, len, ctx); }
int _gr_scalar_pow_vec__extern(gr_ptr vec1, gr_srcptr c, gr_srcptr vec2, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_scalar_pow_vec(vec1, c, vec2, len, ctx); }
int _gr_vec_add_other__extern(gr_ptr vec1, gr_srcptr vec2, gr_srcptr vec3, gr_ctx_t ctx3, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_vec_add_other(vec1, vec2, vec3, ctx3, len, ctx); }
int _gr_vec_sub_other__extern(gr_ptr vec1, gr_srcptr vec2, gr_srcptr vec3, gr_ctx_t ctx3, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_vec_sub_other(vec1, vec2, vec3, ctx3, len, ctx); }
int _gr_vec_mul_other__extern(gr_ptr vec1, gr_srcptr vec2, gr_srcptr vec3, gr_ctx_t ctx3, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_vec_mul_other(vec1, vec2, vec3, ctx3, len, ctx); }
int _gr_vec_div_other__extern(gr_ptr vec1, gr_srcptr vec2, gr_srcptr vec3, gr_ctx_t ctx3, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_vec_div_other(vec1, vec2, vec3, ctx3, len, ctx); }
int _gr_vec_divexact_other__extern(gr_ptr vec1, gr_srcptr vec2, gr_srcptr vec3, gr_ctx_t ctx3, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_vec_divexact_other(vec1, vec2, vec3, ctx3, len, ctx); }
int _gr_vec_pow_other__extern(gr_ptr vec1, gr_srcptr vec2, gr_srcptr vec3, gr_ctx_t ctx3, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_vec_pow_other(vec1, vec2, vec3, ctx3, len, ctx); }
int _gr_other_add_vec__extern(gr_ptr vec1, gr_srcptr vec2, gr_ctx_t ctx2, gr_srcptr vec3, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_other_add_vec(vec1, vec2, ctx2, vec3, len, ctx); }
int _gr_other_sub_vec__extern(gr_ptr vec1, gr_srcptr vec2, gr_ctx_t ctx2, gr_srcptr vec3, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_other_sub_vec(vec1, vec2, ctx2, vec3, len, ctx); }
int _gr_other_mul_vec__extern(gr_ptr vec1, gr_srcptr vec2, gr_ctx_t ctx2, gr_srcptr vec3, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_other_mul_vec(vec1, vec2, ctx2, vec3, len, ctx); }
int _gr_other_div_vec__extern(gr_ptr vec1, gr_srcptr vec2, gr_ctx_t ctx2, gr_srcptr vec3, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_other_div_vec(vec1, vec2, ctx2, vec3, len, ctx); }
int _gr_other_divexact_vec__extern(gr_ptr vec1, gr_srcptr vec2, gr_ctx_t ctx2, gr_srcptr vec3, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_other_divexact_vec(vec1, vec2, ctx2, vec3, len, ctx); }
int _gr_other_pow_vec__extern(gr_ptr vec1, gr_srcptr vec2, gr_ctx_t ctx2, gr_srcptr vec3, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_other_pow_vec(vec1, vec2, ctx2, vec3, len, ctx); }
int _gr_vec_add_scalar_other__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, gr_srcptr c, gr_ctx_t cctx, gr_ctx_t ctx) { return _gr_vec_add_scalar_other(vec1, vec2, len, c, cctx, ctx); }
int _gr_vec_sub_scalar_other__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, gr_srcptr c, gr_ctx_t cctx, gr_ctx_t ctx) { return _gr_vec_sub_scalar_other(vec1, vec2, len, c, cctx, ctx); }
int _gr_vec_mul_scalar_other__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, gr_srcptr c, gr_ctx_t cctx, gr_ctx_t ctx) { return _gr_vec_mul_scalar_other(vec1, vec2, len, c, cctx, ctx); }
int _gr_vec_div_scalar_other__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, gr_srcptr c, gr_ctx_t cctx, gr_ctx_t ctx) { return _gr_vec_div_scalar_other(vec1, vec2, len, c, cctx, ctx); }
int _gr_vec_divexact_scalar_other__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, gr_srcptr c, gr_ctx_t cctx, gr_ctx_t ctx) { return _gr_vec_divexact_scalar_other(vec1, vec2, len, c, cctx, ctx); }
int _gr_vec_pow_scalar_other__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, gr_srcptr c, gr_ctx_t cctx, gr_ctx_t ctx) { return _gr_vec_pow_scalar_other(vec1, vec2, len, c, cctx, ctx); }
int _gr_scalar_other_add_vec__extern(gr_ptr vec1, gr_srcptr c, gr_ctx_t cctx, gr_srcptr vec2, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_scalar_other_add_vec(vec1, c, cctx, vec2, len, ctx); }
int _gr_scalar_other_sub_vec__extern(gr_ptr vec1, gr_srcptr c, gr_ctx_t cctx, gr_srcptr vec2, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_scalar_other_sub_vec(vec1, c, cctx, vec2, len, ctx); }
int _gr_scalar_other_mul_vec__extern(gr_ptr vec1, gr_srcptr c, gr_ctx_t cctx, gr_srcptr vec2, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_scalar_other_mul_vec(vec1, c, cctx, vec2, len, ctx); }
int _gr_scalar_other_div_vec__extern(gr_ptr vec1, gr_srcptr c, gr_ctx_t cctx, gr_srcptr vec2, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_scalar_other_div_vec(vec1, c, cctx, vec2, len, ctx); }
int _gr_scalar_other_divexact_vec__extern(gr_ptr vec1, gr_srcptr c, gr_ctx_t cctx, gr_srcptr vec2, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_scalar_other_divexact_vec(vec1, c, cctx, vec2, len, ctx); }
int _gr_scalar_other_pow_vec__extern(gr_ptr vec1, gr_srcptr c, gr_ctx_t cctx, gr_srcptr vec2, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_scalar_other_pow_vec(vec1, c, cctx, vec2, len, ctx); }
int _gr_vec_mul_scalar_2exp_si__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, mp_limb_signed_t c, gr_ctx_t ctx) { return _gr_vec_mul_scalar_2exp_si(vec1, vec2, len, c, ctx); }
int _gr_vec_addmul_scalar__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, gr_srcptr c, gr_ctx_t ctx) { return _gr_vec_addmul_scalar(vec1, vec2, len, c, ctx); }
int _gr_vec_submul_scalar__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, gr_srcptr c, gr_ctx_t ctx) { return _gr_vec_submul_scalar(vec1, vec2, len, c, ctx); }
int _gr_vec_addmul_scalar_si__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, mp_limb_signed_t c, gr_ctx_t ctx) { return _gr_vec_addmul_scalar_si(vec1, vec2, len, c, ctx); }
int _gr_vec_submul_scalar_si__extern(gr_ptr vec1, gr_srcptr vec2, mp_limb_signed_t len, mp_limb_signed_t c, gr_ctx_t ctx) { return _gr_vec_submul_scalar_si(vec1, vec2, len, c, ctx); }
truth_t _gr_vec_equal__extern(gr_srcptr vec1, gr_srcptr vec2, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_vec_equal(vec1, vec2, len, ctx); }
truth_t _gr_vec_is_zero__extern(gr_srcptr vec, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_vec_is_zero(vec, len, ctx); }
int _gr_vec_sum__extern(gr_ptr res, gr_srcptr vec, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_vec_sum(res, vec, len, ctx); }
int _gr_vec_product__extern(gr_ptr res, gr_srcptr vec, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_vec_product(res, vec, len, ctx); }
int _gr_vec_dot__extern(gr_ptr res, gr_srcptr initial, int subtract, gr_srcptr vec1, gr_srcptr vec2, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_vec_dot(res, initial, subtract, vec1, vec2, len, ctx); }
int _gr_vec_dot_rev__extern(gr_ptr res, gr_srcptr initial, int subtract, gr_srcptr vec1, gr_srcptr vec2, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_vec_dot_rev(res, initial, subtract, vec1, vec2, len, ctx); }
int _gr_vec_dot_si__extern(gr_ptr res, gr_srcptr initial, int subtract, gr_srcptr vec1, const mp_limb_signed_t *vec2, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_vec_dot_si(res, initial, subtract, vec1, vec2, len, ctx); }
int _gr_vec_dot_ui__extern(gr_ptr res, gr_srcptr initial, int subtract, gr_srcptr vec1, const mp_limb_t *vec2, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_vec_dot_ui(res, initial, subtract, vec1, vec2, len, ctx); }
int _gr_vec_dot_fmpz__extern(gr_ptr res, gr_srcptr initial, int subtract, gr_srcptr vec1, const fmpz *vec2, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_vec_dot_fmpz(res, initial, subtract, vec1, vec2, len, ctx); }
int _gr_vec_reciprocals__extern(gr_ptr res, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_vec_reciprocals(res, len, ctx); }
int _gr_vec_set_powers__extern(gr_ptr res, gr_srcptr x, mp_limb_signed_t len, gr_ctx_t ctx) { return _gr_vec_set_powers(res, x, len, ctx); }

int z_mul_checked__extern(mp_limb_signed_t *a, mp_limb_signed_t b, mp_limb_signed_t c) { return z_mul_checked(a, b, c); }
int z_add_checked__extern(mp_limb_signed_t *a, mp_limb_signed_t b, mp_limb_signed_t c) { return z_add_checked(a, b, c); }
int z_mat22_det_is_negative__extern(mp_limb_signed_t m11, mp_limb_signed_t m12, mp_limb_signed_t m21, mp_limb_signed_t m22) { return z_mat22_det_is_negative(m11, m12, m21, m22); }

void _fmpz_set_fast__extern(fmpz_t f, const fmpz_t g) { _fmpz_set_fast(f, g); }
void _fmpz_add_fast__extern(fmpz_t z, const fmpz_t x, mp_limb_signed_t c) { _fmpz_add_fast(z, x, c); }
void _fmpz_add2_fast__extern(fmpz_t z, const fmpz_t x, const fmpz_t y, mp_limb_signed_t c) { _fmpz_add2_fast(z, x, y, c); }
void _fmpz_sub2_fast__extern(fmpz_t z, const fmpz_t x, const fmpz_t y, mp_limb_signed_t c) { _fmpz_sub2_fast(z, x, y, c); }
mp_limb_t __mag_fixmul32__extern(mp_limb_t x, mp_limb_t y) { return __mag_fixmul32(x, y); }
void mag_init__extern(mag_t x) { mag_init(x); }
void mag_init_set__extern(mag_t x, const mag_t y) { mag_init_set(x, y); }
void mag_swap__extern(mag_t x, mag_t y) { mag_swap(x, y); }
void mag_set__extern(mag_t x, const mag_t y) { mag_set(x, y); }
void mag_zero__extern(mag_t x) { mag_zero(x); }
void mag_one__extern(mag_t x) { mag_one(x); }
int mag_is_special__extern(const mag_t x) { return mag_is_special(x); }
int mag_is_zero__extern(const mag_t x) { return mag_is_zero(x); }
void mag_inf__extern(mag_t x) { mag_inf(x); }
int mag_is_inf__extern(const mag_t x) { return mag_is_inf(x); }
int mag_is_finite__extern(const mag_t x) { return mag_is_finite(x); }
int mag_equal__extern(const mag_t x, const mag_t y) { return mag_equal(x, y); }
void mag_inv__extern(mag_t res, const mag_t x) { mag_inv(res, x); }
void mag_inv_lower__extern(mag_t res, const mag_t x) { mag_inv_lower(res, x); }
void mag_fast_init_set__extern(mag_t x, const mag_t y) { mag_fast_init_set(x, y); }
void mag_fast_zero__extern(mag_t x) { mag_fast_zero(x); }
int mag_fast_is_zero__extern(const mag_t x) { return mag_fast_is_zero(x); }
void mag_fast_mul__extern(mag_t z, const mag_t x, const mag_t y) { mag_fast_mul(z, x, y); }
void mag_fast_mul_2exp_si__extern(mag_t z, const mag_t x, mp_limb_signed_t y) { mag_fast_mul_2exp_si(z, x, y); }
void mag_fast_addmul__extern(mag_t z, const mag_t x, const mag_t y) { mag_fast_addmul(z, x, y); }
void mag_fast_add_2exp_si__extern(mag_t z, const mag_t x, mp_limb_signed_t e) { mag_fast_add_2exp_si(z, x, e); }
void mag_min__extern(mag_t z, const mag_t x, const mag_t y) { mag_min(z, x, y); }
void mag_max__extern(mag_t z, const mag_t x, const mag_t y) { mag_max(z, x, y); }
mag_ptr _mag_vec_init__extern(mp_limb_signed_t n) { return _mag_vec_init(n); }
void _mag_vec_clear__extern(mag_ptr v, mp_limb_signed_t n) { _mag_vec_clear(v, n); }
void mag_set_fmpz__extern(mag_t z, const fmpz_t x) { mag_set_fmpz(z, x); }
void mag_set_fmpz_lower__extern(mag_t z, const fmpz_t x) { mag_set_fmpz_lower(z, x); }
void mag_mul_ui__extern(mag_t z, const mag_t x, mp_limb_t y) { mag_mul_ui(z, x, y); }
void mag_mul_ui_lower__extern(mag_t z, const mag_t x, mp_limb_t y) { mag_mul_ui_lower(z, x, y); }
void mag_mul_fmpz__extern(mag_t z, const mag_t x, const fmpz_t y) { mag_mul_fmpz(z, x, y); }
void mag_mul_fmpz_lower__extern(mag_t z, const mag_t x, const fmpz_t y) { mag_mul_fmpz_lower(z, x, y); }
void mag_div_ui__extern(mag_t z, const mag_t x, mp_limb_t y) { mag_div_ui(z, x, y); }
void mag_div_fmpz__extern(mag_t z, const mag_t x, const fmpz_t y) { mag_div_fmpz(z, x, y); }
mp_limb_signed_t mag_allocated_bytes__extern(const mag_t x) { return mag_allocated_bytes(x); }

mpf * mpf_mat_entry__extern(const mpf_mat_t mat, mp_limb_signed_t i, mp_limb_signed_t j) { return mpf_mat_entry(mat, i, j); }

__mpfr_struct * mpfr_mat_entry__extern(const mpfr_mat_t mat, mp_limb_signed_t i, mp_limb_signed_t j) { return mpfr_mat_entry(mat, i, j); }
mp_limb_signed_t mpfr_mat_nrows__extern(const mpfr_mat_t mat) { return mpfr_mat_nrows(mat); }
mp_limb_signed_t mpfr_mat_ncols__extern(const mpfr_mat_t mat) { return mpfr_mat_ncols(mat); }
void mpfr_mat_swap_entrywise__extern(mpfr_mat_t mat1, mpfr_mat_t mat2) { mpfr_mat_swap_entrywise(mat1, mat2); }

mp_limb_signed_t mpoly_divide_threads__extern(mp_limb_signed_t n, double la, double lb) { return mpoly_divide_threads(n, la, lb); }
mp_limb_signed_t mpoly_words_per_exp_sp__extern(mp_limb_t bits, const mpoly_ctx_t mctx) { return mpoly_words_per_exp_sp(bits, mctx); }
mp_limb_signed_t mpoly_words_per_exp_mp__extern(mp_limb_t bits, const mpoly_ctx_t mctx) { return mpoly_words_per_exp_mp(bits, mctx); }
mp_limb_signed_t mpoly_words_per_exp__extern(mp_limb_t bits, const mpoly_ctx_t mctx) { return mpoly_words_per_exp(bits, mctx); }
mp_limb_t mpoly_fix_bits__extern(mp_limb_t bits, const mpoly_ctx_t mctx) { return mpoly_fix_bits(bits, mctx); }
mp_limb_signed_t mpoly_rbtree_ui_head__extern(const mpoly_rbtree_ui_t T) { return mpoly_rbtree_ui_head(T); }
mp_limb_signed_t mpoly_rbtree_fmpz_head__extern(const mpoly_rbtree_fmpz_t T) { return mpoly_rbtree_fmpz_head(T); }
ordering_t mpoly_ordering_randtest__extern(flint_rand_t state) { return mpoly_ordering_randtest(state); }
int mpoly_ordering_isdeg__extern(const mpoly_ctx_t mctx) { return mpoly_ordering_isdeg(mctx); }
int mpoly_ordering_isrev__extern(const mpoly_ctx_t mctx) { return mpoly_ordering_isrev(mctx); }
void mpoly_monomial_zero__extern(mp_limb_t *exp_ptr, mp_limb_signed_t N) { mpoly_monomial_zero(exp_ptr, N); }
void mpoly_monomial_add__extern(mp_limb_t *exp_ptr, const mp_limb_t *exp2, const mp_limb_t *exp3, mp_limb_signed_t N) { mpoly_monomial_add(exp_ptr, exp2, exp3, N); }
void mpoly_monomial_add_mp__extern(mp_limb_t *exp_ptr, const mp_limb_t *exp2, const mp_limb_t *exp3, mp_limb_signed_t N) { mpoly_monomial_add_mp(exp_ptr, exp2, exp3, N); }
void mpoly_monomial_sub__extern(mp_limb_t *exp_ptr, const mp_limb_t *exp2, const mp_limb_t *exp3, mp_limb_signed_t N) { mpoly_monomial_sub(exp_ptr, exp2, exp3, N); }
void mpoly_monomial_sub_mp__extern(mp_limb_t *exp_ptr, const mp_limb_t *exp2, const mp_limb_t *exp3, mp_limb_signed_t N) { mpoly_monomial_sub_mp(exp_ptr, exp2, exp3, N); }
void mpoly_monomial_madd__extern(mp_limb_t *exp1, const mp_limb_t *exp2, mp_limb_t scalar, const mp_limb_t *exp3, mp_limb_signed_t N) { mpoly_monomial_madd(exp1, exp2, scalar, exp3, N); }
void mpoly_monomial_madd_mp__extern(mp_limb_t *exp1, const mp_limb_t *exp2, mp_limb_t scalar, const mp_limb_t *exp3, mp_limb_signed_t N) { mpoly_monomial_madd_mp(exp1, exp2, scalar, exp3, N); }
void mpoly_monomial_madd_inplace_mp__extern(mp_limb_t *exp12, mp_limb_t scalar, const mp_limb_t *exp3, mp_limb_signed_t N) { mpoly_monomial_madd_inplace_mp(exp12, scalar, exp3, N); }
void mpoly_monomial_msub__extern(mp_limb_t *exp1, const mp_limb_t *exp2, mp_limb_t scalar, const mp_limb_t *exp3, mp_limb_signed_t N) { mpoly_monomial_msub(exp1, exp2, scalar, exp3, N); }
void mpoly_monomial_msub_mp__extern(mp_limb_t *exp1, const mp_limb_t *exp2, mp_limb_t scalar, const mp_limb_t *exp3, mp_limb_signed_t N) { mpoly_monomial_msub_mp(exp1, exp2, scalar, exp3, N); }
void mpoly_monomial_msub_ui_array__extern(mp_limb_t *exp1, const mp_limb_t *exp2, const mp_limb_t *scalar, mp_limb_signed_t scalar_limbs, const mp_limb_t *exp3, mp_limb_signed_t N) { mpoly_monomial_msub_ui_array(exp1, exp2, scalar, scalar_limbs, exp3, N); }
void mpoly_monomial_madd_ui_array__extern(mp_limb_t *exp1, const mp_limb_t *exp2, const mp_limb_t *scalar, mp_limb_signed_t scalar_limbs, const mp_limb_t *exp3, mp_limb_signed_t N) { mpoly_monomial_madd_ui_array(exp1, exp2, scalar, scalar_limbs, exp3, N); }
void mpoly_monomial_madd_fmpz__extern(mp_limb_t *exp1, const mp_limb_t *exp2, const fmpz_t scalar, const mp_limb_t *exp3, mp_limb_signed_t N) { mpoly_monomial_madd_fmpz(exp1, exp2, scalar, exp3, N); }
mp_limb_t mpoly_overflow_mask_sp__extern(mp_limb_t bits) { return mpoly_overflow_mask_sp(bits); }
mp_limb_t mpoly_monomial_max1__extern(mp_limb_t exp2, mp_limb_t exp3, mp_limb_t bits, mp_limb_t mask) { return mpoly_monomial_max1(exp2, exp3, bits, mask); }
void mpoly_monomial_max__extern(mp_limb_t *exp1, const mp_limb_t *exp2, const mp_limb_t *exp3, mp_limb_t bits, mp_limb_signed_t N, mp_limb_t mask) { mpoly_monomial_max(exp1, exp2, exp3, bits, N, mask); }
mp_limb_t mpoly_monomial_min1__extern(mp_limb_t exp2, mp_limb_t exp3, mp_limb_t bits, mp_limb_t mask) { return mpoly_monomial_min1(exp2, exp3, bits, mask); }
void mpoly_monomial_min__extern(mp_limb_t *exp1, const mp_limb_t *exp2, const mp_limb_t *exp3, mp_limb_t bits, mp_limb_signed_t N, mp_limb_t mask) { mpoly_monomial_min(exp1, exp2, exp3, bits, N, mask); }
void mpoly_monomial_max_mp__extern(mp_limb_t *exp1, const mp_limb_t *exp2, const mp_limb_t *exp3, mp_limb_t bits, mp_limb_signed_t N) { mpoly_monomial_max_mp(exp1, exp2, exp3, bits, N); }
void mpoly_monomial_min_mp__extern(mp_limb_t *exp1, const mp_limb_t *exp2, const mp_limb_t *exp3, mp_limb_t bits, mp_limb_signed_t N) { mpoly_monomial_min_mp(exp1, exp2, exp3, bits, N); }
int mpoly_monomial_overflows__extern(mp_limb_t *exp2, mp_limb_signed_t N, mp_limb_t mask) { return mpoly_monomial_overflows(exp2, N, mask); }
int mpoly_monomial_overflows_mp__extern(mp_limb_t *exp_ptr, mp_limb_signed_t N, mp_limb_t bits) { return mpoly_monomial_overflows_mp(exp_ptr, N, bits); }
int mpoly_monomial_overflows1__extern(mp_limb_t exp, mp_limb_t mask) { return mpoly_monomial_overflows1(exp, mask); }
int mpoly_monomial_divides__extern(mp_limb_t *exp_ptr, const mp_limb_t *exp2, const mp_limb_t *exp3, mp_limb_signed_t N, mp_limb_t mask) { return mpoly_monomial_divides(exp_ptr, exp2, exp3, N, mask); }
int mpoly_monomial_halves__extern(mp_limb_t *exp_ptr, const mp_limb_t *exp2, mp_limb_signed_t N, mp_limb_t mask) { return mpoly_monomial_halves(exp_ptr, exp2, N, mask); }
int mpoly_monomial_divides_mp__extern(mp_limb_t *exp_ptr, const mp_limb_t *exp2, const mp_limb_t *exp3, mp_limb_signed_t N, mp_limb_t bits) { return mpoly_monomial_divides_mp(exp_ptr, exp2, exp3, N, bits); }
int mpoly_monomial_halves_mp__extern(mp_limb_t *exp_ptr, const mp_limb_t *exp2, mp_limb_signed_t N, mp_limb_t bits) { return mpoly_monomial_halves_mp(exp_ptr, exp2, N, bits); }
int mpoly_monomial_divides_test__extern(const mp_limb_t *exp2, const mp_limb_t *exp3, mp_limb_signed_t N, mp_limb_t mask) { return mpoly_monomial_divides_test(exp2, exp3, N, mask); }
int mpoly_monomial_divides_mp_test__extern(const mp_limb_t *exp2, const mp_limb_t *exp3, mp_limb_signed_t N, mp_limb_t bits) { return mpoly_monomial_divides_mp_test(exp2, exp3, N, bits); }
int mpoly_monomial_divides1__extern(mp_limb_t *exp_ptr, const mp_limb_t exp2, const mp_limb_t exp3, mp_limb_t mask) { return mpoly_monomial_divides1(exp_ptr, exp2, exp3, mask); }
int mpoly_monomial_halves1__extern(mp_limb_t *exp_ptr, const mp_limb_t exp2, mp_limb_t mask) { return mpoly_monomial_halves1(exp_ptr, exp2, mask); }
void mpoly_monomial_set__extern(mp_limb_t *exp2, const mp_limb_t *exp3, mp_limb_signed_t N) { mpoly_monomial_set(exp2, exp3, N); }
void mpoly_monomial_set_extra__extern(mp_limb_t *exp2, const mp_limb_t *exp3, mp_limb_signed_t N, mp_limb_signed_t offset, mp_limb_t extra) { mpoly_monomial_set_extra(exp2, exp3, N, offset, extra); }
void mpoly_copy_monomials__extern(mp_limb_t *exp1, const mp_limb_t *exp2, mp_limb_signed_t len, mp_limb_signed_t N) { mpoly_copy_monomials(exp1, exp2, len, N); }
void mpoly_monomial_swap__extern(mp_limb_t *exp2, mp_limb_t *exp3, mp_limb_signed_t N) { mpoly_monomial_swap(exp2, exp3, N); }
void mpoly_monomial_mul_ui__extern(mp_limb_t *exp2, const mp_limb_t *exp3, mp_limb_signed_t N, mp_limb_t c) { mpoly_monomial_mul_ui(exp2, exp3, N, c); }
void mpoly_monomial_mul_ui_mp__extern(mp_limb_t *exp2, const mp_limb_t *exp3, mp_limb_signed_t N, mp_limb_t c) { mpoly_monomial_mul_ui_mp(exp2, exp3, N, c); }
int mpoly_monomial_is_zero__extern(const mp_limb_t *exp, mp_limb_signed_t N) { return mpoly_monomial_is_zero(exp, N); }
int mpoly_monomial_equal__extern(const mp_limb_t *exp2, const mp_limb_t *exp3, mp_limb_signed_t N) { return mpoly_monomial_equal(exp2, exp3, N); }
int mpoly_monomial_equal_extra__extern(const mp_limb_t *exp2, const mp_limb_t *exp3, mp_limb_signed_t N, mp_limb_signed_t offset, mp_limb_t extra) { return mpoly_monomial_equal_extra(exp2, exp3, N, offset, extra); }
int mpoly_monomial_cmp1__extern(mp_limb_t a, mp_limb_t b, mp_limb_t cmpmask) { return mpoly_monomial_cmp1(a, b, cmpmask); }
int mpoly_monomial_gt1__extern(mp_limb_t a, mp_limb_t b, mp_limb_t cmpmask) { return mpoly_monomial_gt1(a, b, cmpmask); }
int mpoly_monomial_ge1__extern(mp_limb_t a, mp_limb_t b, mp_limb_t cmpmask) { return mpoly_monomial_ge1(a, b, cmpmask); }
int mpoly_monomial_lt__extern(const mp_limb_t *exp3, const mp_limb_t *exp2, mp_limb_signed_t N, const mp_limb_t *cmpmask) { return mpoly_monomial_lt(exp3, exp2, N, cmpmask); }
int mpoly_monomial_gt__extern(const mp_limb_t *exp3, const mp_limb_t *exp2, mp_limb_signed_t N, const mp_limb_t *cmpmask) { return mpoly_monomial_gt(exp3, exp2, N, cmpmask); }
int mpoly_monomial_lt_nomask__extern(const mp_limb_t *exp2, const mp_limb_t *exp3, mp_limb_signed_t N) { return mpoly_monomial_lt_nomask(exp2, exp3, N); }
int mpoly_monomial_gt_nomask__extern(const mp_limb_t *exp2, const mp_limb_t *exp3, mp_limb_signed_t N) { return mpoly_monomial_gt_nomask(exp2, exp3, N); }
int mpoly_monomial_lt_nomask_extra__extern(const mp_limb_t *exp2, const mp_limb_t *exp3, mp_limb_signed_t N, mp_limb_signed_t offset, mp_limb_t extra) { return mpoly_monomial_lt_nomask_extra(exp2, exp3, N, offset, extra); }
int mpoly_monomial_gt_nomask_extra__extern(const mp_limb_t *exp2, const mp_limb_t *exp3, mp_limb_signed_t N, mp_limb_signed_t offset, mp_limb_t extra) { return mpoly_monomial_gt_nomask_extra(exp2, exp3, N, offset, extra); }
int mpoly_monomial_cmp__extern(const mp_limb_t *exp2, const mp_limb_t *exp3, mp_limb_signed_t N, const mp_limb_t *cmpmask) { return mpoly_monomial_cmp(exp2, exp3, N, cmpmask); }
int mpoly_monomial_cmp_nomask__extern(const mp_limb_t *exp2, const mp_limb_t *exp3, mp_limb_signed_t N) { return mpoly_monomial_cmp_nomask(exp2, exp3, N); }
int mpoly_monomial_cmp_nomask_extra__extern(const mp_limb_t *exp2, const mp_limb_t *exp3, mp_limb_signed_t N, mp_limb_signed_t offset, mp_limb_t extra) { return mpoly_monomial_cmp_nomask_extra(exp2, exp3, N, offset, extra); }
int mpoly_monomial_divides_tight__extern(mp_limb_signed_t e1, mp_limb_signed_t e2, mp_limb_signed_t *prods, mp_limb_signed_t num) { return mpoly_monomial_divides_tight(e1, e2, prods, num); }
void mpoly_max_degrees_tight__extern(mp_limb_signed_t *max_exp, mp_limb_t *exps, mp_limb_signed_t len, mp_limb_signed_t *prods, mp_limb_signed_t num) { mpoly_max_degrees_tight(max_exp, exps, len, prods, num); }
mp_limb_signed_t mpoly_geobucket_clog4__extern(mp_limb_signed_t x) { return mpoly_geobucket_clog4(x); }
mp_limb_t pack_exp2__extern(mp_limb_t e0, mp_limb_t e1) { return pack_exp2(e0, e1); }
mp_limb_t pack_exp3__extern(mp_limb_t e0, mp_limb_t e1, mp_limb_t e2) { return pack_exp3(e0, e1, e2); }
mp_limb_t extract_exp__extern(mp_limb_t e, int idx, int nvars) { return extract_exp(e, idx, nvars); }
mp_limb_signed_t mpoly_gen_index__extern(mp_limb_signed_t v, const mpoly_ctx_t mctx) { return mpoly_gen_index(v, mctx); }
mp_limb_t mpoly_gen_pow_exp_bits_required__extern(mp_limb_signed_t v, mp_limb_t e, const mpoly_ctx_t mctx) { return mpoly_gen_pow_exp_bits_required(v, e, mctx); }
void mpoly_get_monomial_ui__extern(mp_limb_t *user_exps, const mp_limb_t *poly_exps, mp_limb_t bits, const mpoly_ctx_t mctx) { mpoly_get_monomial_ui(user_exps, poly_exps, bits, mctx); }
void mpoly_get_monomial_si__extern(mp_limb_signed_t *user_exps, const mp_limb_t *poly_exps, mp_limb_t bits, const mpoly_ctx_t mctx) { mpoly_get_monomial_si(user_exps, poly_exps, bits, mctx); }
mp_limb_t mpoly_get_monomial_var_exp_ui__extern(const mp_limb_t *poly_exps, mp_limb_signed_t var, mp_limb_t bits, const mpoly_ctx_t mctx) { return mpoly_get_monomial_var_exp_ui(poly_exps, var, bits, mctx); }
mp_limb_signed_t mpoly_get_monomial_var_exp_si__extern(const mp_limb_t *poly_exps, mp_limb_signed_t var, mp_limb_t bits, const mpoly_ctx_t mctx) { return mpoly_get_monomial_var_exp_si(poly_exps, var, bits, mctx); }
void _slong_array_fit_length__extern(mp_limb_signed_t **array, mp_limb_signed_t *alloc, mp_limb_signed_t len) { _slong_array_fit_length(array, alloc, len); }
void mpoly_main_variable_terms1__extern(mp_limb_signed_t *i1, mp_limb_signed_t *n1, const mp_limb_t *exp1, mp_limb_signed_t l1, mp_limb_signed_t len1, mp_limb_signed_t k, mp_limb_signed_t num, mp_limb_signed_t bits) { mpoly_main_variable_terms1(i1, n1, exp1, l1, len1, k, num, bits); }

void n_poly_init__extern(n_poly_t A) { n_poly_init(A); }
void n_poly_init2__extern(n_poly_t A, mp_limb_signed_t alloc) { n_poly_init2(A, alloc); }
void n_poly_clear__extern(n_poly_t A) { n_poly_clear(A); }
void n_poly_fit_length__extern(n_poly_t A, mp_limb_signed_t len) { n_poly_fit_length(A, len); }
void nmod_poly_mock__extern(nmod_poly_t a, const n_poly_t b, nmod_t mod) { nmod_poly_mock(a, b, mod); }
void n_poly_mock__extern(n_poly_t a, const nmod_poly_t b) { n_poly_mock(a, b); }
void n_poly_set__extern(n_poly_t A, const n_poly_t B) { n_poly_set(A, B); }
void n_poly_swap__extern(n_poly_t A, n_poly_t B) { n_poly_swap(A, B); }
void _n_poly_normalise__extern(n_poly_t A) { _n_poly_normalise(A); }
mp_limb_signed_t n_poly_degree__extern(const n_poly_t A) { return n_poly_degree(A); }
int n_poly_is_one__extern(const n_poly_t A) { return n_poly_is_one(A); }
mp_limb_t n_poly_lead__extern(const n_poly_t A) { return n_poly_lead(A); }
void n_poly_one__extern(n_poly_t A) { n_poly_one(A); }
void n_poly_set_ui__extern(n_poly_t A, mp_limb_t c) { n_poly_set_ui(A, c); }
int n_poly_is_zero__extern(const n_poly_t poly) { return n_poly_is_zero(poly); }
void n_poly_zero__extern(n_poly_t res) { n_poly_zero(res); }
int n_poly_equal__extern(const n_poly_t a, const n_poly_t b) { return n_poly_equal(a, b); }
void n_poly_mod_make_monic__extern(n_poly_t A, const n_poly_t B, nmod_t mod) { n_poly_mod_make_monic(A, B, mod); }
void n_poly_mod_taylor_shift__extern(n_poly_t g, mp_limb_t c, nmod_t mod) { n_poly_mod_taylor_shift(g, c, mod); }
mp_limb_t n_poly_get_coeff__extern(const n_poly_t poly, mp_limb_signed_t j) { return n_poly_get_coeff(poly, j); }
void n_poly_set_coeff_nonzero__extern(n_poly_t A, mp_limb_signed_t j, mp_limb_t c) { n_poly_set_coeff_nonzero(A, j, c); }
void n_poly_set_nmod_poly__extern(n_poly_t a, const nmod_poly_t b) { n_poly_set_nmod_poly(a, b); }
void nmod_poly_set_n_poly__extern(nmod_poly_t a, const n_poly_t b) { nmod_poly_set_n_poly(a, b); }
void n_poly_shift_left__extern(n_poly_t A, const n_poly_t B, mp_limb_signed_t k) { n_poly_shift_left(A, B, k); }
void n_poly_shift_right__extern(n_poly_t res, const n_poly_t poly, mp_limb_signed_t k) { n_poly_shift_right(res, poly, k); }
void n_poly_truncate__extern(n_poly_t poly, mp_limb_signed_t len) { n_poly_truncate(poly, len); }
void _n_poly_mod_scalar_mul_nmod__extern(n_poly_t A, const n_poly_t B, mp_limb_t c, nmod_t mod) { _n_poly_mod_scalar_mul_nmod(A, B, c, mod); }
void _n_poly_mod_scalar_mul_nmod_inplace__extern(n_poly_t A, mp_limb_t c, nmod_t mod) { _n_poly_mod_scalar_mul_nmod_inplace(A, c, mod); }
mp_limb_t n_poly_mod_evaluate_nmod__extern(const n_poly_t A, mp_limb_t c, nmod_t mod) { return n_poly_mod_evaluate_nmod(A, c, mod); }
void n_poly_mod_neg__extern(n_poly_t A, const n_poly_t B, nmod_t mod) { n_poly_mod_neg(A, B, mod); }
void n_poly_mod_add__extern(n_poly_t A, const n_poly_t B, const n_poly_t C, nmod_t mod) { n_poly_mod_add(A, B, C, mod); }
void n_poly_mod_sub__extern(n_poly_t A, const n_poly_t B, const n_poly_t C, nmod_t mod) { n_poly_mod_sub(A, B, C, mod); }
void n_poly_mod_product_roots_nmod_vec__extern(n_poly_t A, mp_srcptr r, mp_limb_signed_t n, nmod_t mod) { n_poly_mod_product_roots_nmod_vec(A, r, n, mod); }
void _n_poly_mod_mul__extern(n_poly_t A, const n_poly_t B, const n_poly_t C, nmod_t ctx) { _n_poly_mod_mul(A, B, C, ctx); }
void _n_poly_mod_div__extern(n_poly_t Q, const n_poly_t A, const n_poly_t B, nmod_t mod) { _n_poly_mod_div(Q, A, B, mod); }
void _n_poly_mod_divexact__extern(n_poly_t Q, const n_poly_t A, const n_poly_t B, nmod_t mod) { _n_poly_mod_divexact(Q, A, B, mod); }
void _n_poly_mod_rem__extern(n_poly_t R, const n_poly_t A, const n_poly_t B, nmod_t mod) { _n_poly_mod_rem(R, A, B, mod); }
void _n_poly_mod_divrem__extern(n_poly_t Q, n_poly_t R, const n_poly_t A, const n_poly_t B, nmod_t mod) { _n_poly_mod_divrem(Q, R, A, B, mod); }
nmod_t fq_nmod_ctx_mod__extern(const fq_nmod_ctx_t ctx) { return fq_nmod_ctx_mod(ctx); }
int _n_fq_is_zero__extern(const mp_limb_t *a, mp_limb_signed_t d) { return _n_fq_is_zero(a, d); }
void _n_fq_zero__extern(mp_limb_t *a, mp_limb_signed_t d) { _n_fq_zero(a, d); }
int _n_fq_is_one__extern(const mp_limb_t *a, mp_limb_signed_t d) { return _n_fq_is_one(a, d); }
int _n_fq_is_ui__extern(const mp_limb_t *a, mp_limb_signed_t d) { return _n_fq_is_ui(a, d); }
int n_fq_is_one__extern(const mp_limb_t *a, const fq_nmod_ctx_t ctx) { return n_fq_is_one(a, ctx); }
void _n_fq_one__extern(mp_limb_t *a, mp_limb_signed_t d) { _n_fq_one(a, d); }
void _n_fq_set_nmod__extern(mp_limb_t *a, mp_limb_t b, mp_limb_signed_t d) { _n_fq_set_nmod(a, b, d); }
void _n_fq_set__extern(mp_limb_t *a, const mp_limb_t *b, mp_limb_signed_t d) { _n_fq_set(a, b, d); }
void _n_fq_swap__extern(mp_limb_t *a, mp_limb_t *b, mp_limb_signed_t d) { _n_fq_swap(a, b, d); }
int _n_fq_equal__extern(mp_limb_t *a, const mp_limb_t *b, mp_limb_signed_t d) { return _n_fq_equal(a, b, d); }
void n_fq_add__extern(mp_limb_t *a, const mp_limb_t *b, const mp_limb_t *c, const fq_nmod_ctx_t ctx) { n_fq_add(a, b, c, ctx); }
void n_fq_sub__extern(mp_limb_t *a, const mp_limb_t *b, const mp_limb_t *c, const fq_nmod_ctx_t ctx) { n_fq_sub(a, b, c, ctx); }
void _n_fq_add__extern(mp_limb_t *a, const mp_limb_t *b, const mp_limb_t *c, mp_limb_signed_t d, nmod_t mod) { _n_fq_add(a, b, c, d, mod); }
void _n_fq_sub__extern(mp_limb_t *a, const mp_limb_t *b, const mp_limb_t *c, mp_limb_signed_t d, nmod_t mod) { _n_fq_sub(a, b, c, d, mod); }
void _n_fq_neg__extern(mp_limb_t *a, const mp_limb_t *b, mp_limb_signed_t d, nmod_t mod) { _n_fq_neg(a, b, d, mod); }
void _n_fq_reduce2__extern(mp_limb_t *a, mp_limb_t *b, const fq_nmod_ctx_t ctx, mp_limb_t *t) { _n_fq_reduce2(a, b, ctx, t); }
void _n_fq_mul__extern(mp_limb_t *a, const mp_limb_t *b, const mp_limb_t *c, const fq_nmod_ctx_t ctx, mp_limb_t *t) { _n_fq_mul(a, b, c, ctx, t); }
void _n_fq_addmul__extern(mp_limb_t *a, const mp_limb_t *b, const mp_limb_t *c, const mp_limb_t *e, const fq_nmod_ctx_t ctx, mp_limb_t *t) { _n_fq_addmul(a, b, c, e, ctx, t); }
void n_fq_poly_one__extern(n_fq_poly_t A, const fq_nmod_ctx_t ctx) { n_fq_poly_one(A, ctx); }
void _n_fq_poly_normalise__extern(n_fq_poly_t A, mp_limb_signed_t d) { _n_fq_poly_normalise(A, d); }
void n_fq_poly_divrem___extern(n_poly_t Q, n_poly_t R, const n_poly_t A, const n_poly_t B, const fq_nmod_ctx_t ctx, n_poly_stack_t St) { n_fq_poly_divrem_(Q, R, A, B, ctx, St); }
void n_bpoly_init__extern(n_bpoly_t A) { n_bpoly_init(A); }
void n_bpoly_swap__extern(n_bpoly_t A, n_bpoly_t B) { n_bpoly_swap(A, B); }
void n_bpoly_normalise__extern(n_bpoly_t A) { n_bpoly_normalise(A); }
void n_bpoly_fit_length__extern(n_bpoly_t A, mp_limb_signed_t len) { n_bpoly_fit_length(A, len); }
void n_bpoly_zero__extern(n_bpoly_t A) { n_bpoly_zero(A); }
int n_bpoly_is_zero__extern(const n_bpoly_t A) { return n_bpoly_is_zero(A); }
void n_bpoly_set__extern(n_bpoly_t A, const n_bpoly_t B) { n_bpoly_set(A, B); }
mp_limb_t n_bpoly_get_coeff__extern(const n_bpoly_t A, mp_limb_signed_t e0, mp_limb_signed_t e1) { return n_bpoly_get_coeff(A, e0, e1); }
mp_limb_signed_t n_bpoly_degree0__extern(const n_bpoly_t A) { return n_bpoly_degree0(A); }
mp_limb_t n_bpoly_bidegree__extern(const n_bpoly_t A) { return n_bpoly_bidegree(A); }
void n_tpoly_init__extern(n_tpoly_t A) { n_tpoly_init(A); }
void n_tpoly_swap__extern(n_tpoly_t A, n_tpoly_t B) { n_tpoly_swap(A, B); }
void n_polyu_init__extern(n_polyu_t A) { n_polyu_init(A); }
void n_polyu_fit_length__extern(n_polyu_t A, mp_limb_signed_t len) { n_polyu_fit_length(A, len); }
void n_polyu_swap__extern(n_polyu_t A, n_polyu_t B) { n_polyu_swap(A, B); }
mp_limb_signed_t nmod_eval_interp_eval_length__extern(nmod_eval_interp_t E) { return nmod_eval_interp_eval_length(E); }
void nmod_evals_zero__extern(n_poly_t a) { nmod_evals_zero(a); }
void n_fq_evals_zero__extern(n_fq_poly_t a) { n_fq_evals_zero(a); }
void n_polyun_init__extern(n_polyun_t A) { n_polyun_init(A); }
void n_polyun_fit_length__extern(n_polyun_t A, mp_limb_signed_t len) { n_polyun_fit_length(A, len); }
void n_polyun_swap__extern(n_polyun_t A, n_polyun_t B) { n_polyun_swap(A, B); }
void n_polyun_one__extern(n_polyun_t A) { n_polyun_one(A); }
mp_limb_t n_polyu1n_bidegree__extern(n_polyun_t A) { return n_polyu1n_bidegree(A); }
mp_limb_t * n_poly_stack_vec_init__extern(n_poly_stack_t S, mp_limb_signed_t len) { return n_poly_stack_vec_init(S, len); }
void n_poly_stack_vec_clear__extern(n_poly_stack_t S) { n_poly_stack_vec_clear(S); }
n_poly_struct ** n_poly_stack_request__extern(n_poly_stack_t S, mp_limb_signed_t k) { return n_poly_stack_request(S, k); }
n_poly_struct * n_poly_stack_take_top__extern(n_poly_stack_t S) { return n_poly_stack_take_top(S); }
void n_poly_stack_give_back__extern(n_poly_stack_t S, mp_limb_signed_t k) { n_poly_stack_give_back(S, k); }
mp_limb_signed_t n_poly_stack_size__extern(const n_poly_stack_t S) { return n_poly_stack_size(S); }
n_bpoly_struct ** n_bpoly_stack_request__extern(n_bpoly_stack_t S, mp_limb_signed_t k) { return n_bpoly_stack_request(S, k); }
n_bpoly_struct * n_bpoly_stack_take_top__extern(n_bpoly_stack_t S) { return n_bpoly_stack_take_top(S); }
void n_bpoly_stack_give_back__extern(n_bpoly_stack_t S, mp_limb_signed_t k) { n_bpoly_stack_give_back(S, k); }
mp_limb_signed_t n_bpoly_stack_size__extern(const n_bpoly_stack_t S) { return n_bpoly_stack_size(S); }
n_polyun_struct ** n_polyun_stack_request__extern(n_polyun_stack_t S, mp_limb_signed_t k) { return n_polyun_stack_request(S, k); }
n_polyun_struct * n_polyun_stack_take_top__extern(n_polyun_stack_t S) { return n_polyun_stack_take_top(S); }
void n_polyun_stack_give_back__extern(n_polyun_stack_t S, mp_limb_signed_t k) { n_polyun_stack_give_back(S, k); }
mp_limb_signed_t n_polyun_stack_size__extern(const n_polyun_stack_t S) { return n_polyun_stack_size(S); }

void nf_elem_canonicalise__extern(nf_elem_t a, const nf_t nf) { nf_elem_canonicalise(a, nf); }
int nf_elem_is_zero__extern(const nf_elem_t a, const nf_t nf) { return nf_elem_is_zero(a, nf); }
int nf_elem_is_one__extern(const nf_elem_t a, const nf_t nf) { return nf_elem_is_one(a, nf); }
int nf_elem_is_integer__extern(const nf_elem_t a, const nf_t nf) { return nf_elem_is_integer(a, nf); }
int nf_elem_is_rational__extern(const nf_elem_t a, const nf_t nf) { return nf_elem_is_rational(a, nf); }
void nf_elem_get_den__extern(fmpz_t d, const nf_elem_t b, const nf_t nf) { nf_elem_get_den(d, b, nf); }
void nf_elem_set_den__extern(nf_elem_t b, fmpz_t d, const nf_t nf) { nf_elem_set_den(b, d, nf); }
int nf_elem_den_is_one__extern(const nf_elem_t a, const nf_t nf) { return nf_elem_den_is_one(a, nf); }

mp_limb_t nmod_set_ui__extern(mp_limb_t x, nmod_t mod) { return nmod_set_ui(x, mod); }
mp_limb_t nmod_set_si__extern(mp_limb_signed_t x, nmod_t mod) { return nmod_set_si(x, mod); }
mp_limb_t _nmod_add__extern(mp_limb_t a, mp_limb_t b, nmod_t mod) { return _nmod_add(a, b, mod); }
mp_limb_t _nmod_sub__extern(mp_limb_t a, mp_limb_t b, nmod_t mod) { return _nmod_sub(a, b, mod); }
mp_limb_t nmod_add__extern(mp_limb_t a, mp_limb_t b, nmod_t mod) { return nmod_add(a, b, mod); }
mp_limb_t nmod_sub__extern(mp_limb_t a, mp_limb_t b, nmod_t mod) { return nmod_sub(a, b, mod); }
mp_limb_t nmod_neg__extern(mp_limb_t a, nmod_t mod) { return nmod_neg(a, mod); }
mp_limb_t nmod_mul__extern(mp_limb_t a, mp_limb_t b, nmod_t mod) { return nmod_mul(a, b, mod); }
mp_limb_t _nmod_mul_fullword__extern(mp_limb_t a, mp_limb_t b, nmod_t mod) { return _nmod_mul_fullword(a, b, mod); }
mp_limb_t nmod_addmul__extern(mp_limb_t a, mp_limb_t b, mp_limb_t c, nmod_t mod) { return nmod_addmul(a, b, c, mod); }
mp_limb_t nmod_inv__extern(mp_limb_t a, nmod_t mod) { return nmod_inv(a, mod); }
mp_limb_t nmod_div__extern(mp_limb_t a, mp_limb_t b, nmod_t mod) { return nmod_div(a, b, mod); }
mp_limb_t nmod_pow_ui__extern(mp_limb_t a, mp_limb_t exp, nmod_t mod) { return nmod_pow_ui(a, exp, mod); }
mp_limb_t nmod_pow_fmpz__extern(mp_limb_t a, const fmpz_t exp, nmod_t mod) { return nmod_pow_fmpz(a, exp, mod); }
void nmod_init__extern(nmod_t *mod, mp_limb_t n) { nmod_init(mod, n); }
mp_limb_t nmod_discrete_log_pohlig_hellman_primitive_root__extern(const nmod_discrete_log_pohlig_hellman_t L) { return nmod_discrete_log_pohlig_hellman_primitive_root(L); }

mp_limb_t nmod_mat_get_entry__extern(const nmod_mat_t mat, mp_limb_signed_t i, mp_limb_signed_t j) { return nmod_mat_get_entry(mat, i, j); }
mp_limb_t * nmod_mat_entry_ptr__extern(const nmod_mat_t mat, mp_limb_signed_t i, mp_limb_signed_t j) { return nmod_mat_entry_ptr(mat, i, j); }
mp_limb_signed_t nmod_mat_nrows__extern(const nmod_mat_t mat) { return nmod_mat_nrows(mat); }
mp_limb_signed_t nmod_mat_ncols__extern(const nmod_mat_t mat) { return nmod_mat_ncols(mat); }
void nmod_mat_swap_entrywise__extern(nmod_mat_t mat1, nmod_mat_t mat2) { nmod_mat_swap_entrywise(mat1, mat2); }
int nmod_mat_is_empty__extern(const nmod_mat_t mat) { return nmod_mat_is_empty(mat); }
int nmod_mat_is_square__extern(const nmod_mat_t mat) { return nmod_mat_is_square(mat); }
void nmod_mat_swap_rows__extern(nmod_mat_t mat, mp_limb_signed_t *perm, mp_limb_signed_t r, mp_limb_signed_t s) { nmod_mat_swap_rows(mat, perm, r, s); }
void nmod_mat_invert_rows__extern(nmod_mat_t mat, mp_limb_signed_t *perm) { nmod_mat_invert_rows(mat, perm); }
void nmod_mat_swap_cols__extern(nmod_mat_t mat, mp_limb_signed_t *perm, mp_limb_signed_t r, mp_limb_signed_t s) { nmod_mat_swap_cols(mat, perm, r, s); }
void nmod_mat_invert_cols__extern(nmod_mat_t mat, mp_limb_signed_t *perm) { nmod_mat_invert_cols(mat, perm); }

mp_limb_t * nmod_mpoly_term_coeff_ref__extern(nmod_mpoly_t A, mp_limb_signed_t i, const nmod_mpoly_ctx_t ctx) { return nmod_mpoly_term_coeff_ref(A, i, ctx); }
n_poly_struct * evil_cast_nmod_poly_to_n_poly__extern(nmod_poly_struct *a) { return evil_cast_nmod_poly_to_n_poly(a); }
const n_poly_struct * evil_const_cast_nmod_poly_to_n_poly__extern(const nmod_poly_struct *a) { return evil_const_cast_nmod_poly_to_n_poly(a); }
n_poly_struct ** nmod_poly_stack_request_poly__extern(nmod_poly_stack_t S, mp_limb_signed_t k) { return nmod_poly_stack_request_poly(S, k); }
n_poly_struct * nmod_poly_stack_take_top_poly__extern(nmod_poly_stack_t S) { return nmod_poly_stack_take_top_poly(S); }
void nmod_poly_stack_give_back_poly__extern(nmod_poly_stack_t S, mp_limb_signed_t k) { nmod_poly_stack_give_back_poly(S, k); }
mp_limb_signed_t nmod_poly_stack_size_poly__extern(const nmod_poly_stack_t S) { return nmod_poly_stack_size_poly(S); }
nmod_mpolyun_struct ** nmod_poly_stack_request_mpolyun__extern(nmod_poly_stack_t S, mp_limb_signed_t k) { return nmod_poly_stack_request_mpolyun(S, k); }
nmod_mpolyun_struct * nmod_poly_stack_take_top_mpolyun__extern(nmod_poly_stack_t S) { return nmod_poly_stack_take_top_mpolyun(S); }
void nmod_poly_stack_give_back_mpolyun__extern(nmod_poly_stack_t S, mp_limb_signed_t k) { nmod_poly_stack_give_back_mpolyun(S, k); }
mp_limb_signed_t nmod_poly_stack_size_mpolyun__extern(const nmod_poly_stack_t S) { return nmod_poly_stack_size_mpolyun(S); }
nmod_mpolyn_struct ** nmod_poly_stack_request_mpolyn__extern(nmod_poly_stack_t S, mp_limb_signed_t k) { return nmod_poly_stack_request_mpolyn(S, k); }
nmod_mpolyn_struct * nmod_poly_stack_take_top_mpolyn__extern(nmod_poly_stack_t S) { return nmod_poly_stack_take_top_mpolyn(S); }
void nmod_poly_stack_give_back_mpolyn__extern(nmod_poly_stack_t S, mp_limb_signed_t k) { nmod_poly_stack_give_back_mpolyn(S, k); }
mp_limb_signed_t nmod_poly_stack_size_mpolyn__extern(const nmod_poly_stack_t S) { return nmod_poly_stack_size_mpolyn(S); }
mp_limb_signed_t nmod_mpoly_ctx_nvars__extern(const nmod_mpoly_ctx_t ctx) { return nmod_mpoly_ctx_nvars(ctx); }
ordering_t nmod_mpoly_ctx_ord__extern(const nmod_mpoly_ctx_t ctx) { return nmod_mpoly_ctx_ord(ctx); }
mp_limb_t nmod_mpoly_ctx_modulus__extern(const nmod_mpoly_ctx_t ctx) { return nmod_mpoly_ctx_modulus(ctx); }
void nmod_mpoly_init__extern(nmod_mpoly_t A, const nmod_mpoly_ctx_t ctx) { nmod_mpoly_init(A, ctx); }
void nmod_mpoly_clear__extern(nmod_mpoly_t A, const nmod_mpoly_ctx_t ctx) { nmod_mpoly_clear(A, ctx); }
void _nmod_mpoly_fit_length__extern(mp_limb_t **coeffs, mp_limb_signed_t *coeffs_alloc, mp_limb_t **exps, mp_limb_signed_t *exps_alloc, mp_limb_signed_t N, mp_limb_signed_t length) { _nmod_mpoly_fit_length(coeffs, coeffs_alloc, exps, exps_alloc, N, length); }
void _nmod_mpoly_set_length__extern(nmod_mpoly_t A, mp_limb_signed_t newlen, const nmod_mpoly_ctx_t ctx) { _nmod_mpoly_set_length(A, newlen, ctx); }
void nmod_mpoly_truncate__extern(nmod_mpoly_t A, mp_limb_signed_t newlen, const nmod_mpoly_ctx_t ctx) { nmod_mpoly_truncate(A, newlen, ctx); }
void nmod_mpoly_swap__extern(nmod_mpoly_t A, nmod_mpoly_t B, const nmod_mpoly_ctx_t ctx) { nmod_mpoly_swap(A, B, ctx); }
void nmod_mpoly_zero__extern(nmod_mpoly_t A, const nmod_mpoly_ctx_t ctx) { nmod_mpoly_zero(A, ctx); }
void nmod_mpoly_one__extern(nmod_mpoly_t A, const nmod_mpoly_ctx_t ctx) { nmod_mpoly_one(A, ctx); }
int nmod_mpoly_is_zero__extern(const nmod_mpoly_t A, const nmod_mpoly_ctx_t ctx) { return nmod_mpoly_is_zero(A, ctx); }
int nmod_mpoly_is_one__extern(const nmod_mpoly_t A, const nmod_mpoly_ctx_t ctx) { return nmod_mpoly_is_one(A, ctx); }
mp_limb_t nmod_mpoly_leadcoeff__extern(nmod_mpoly_t A, const nmod_mpoly_ctx_t ctx) { return nmod_mpoly_leadcoeff(A, ctx); }
mp_limb_signed_t nmod_mpoly_length__extern(const nmod_mpoly_t A, const nmod_mpoly_ctx_t ctx) { return nmod_mpoly_length(A, ctx); }
void nmod_mpoly_divexact__extern(nmod_mpoly_t Q, const nmod_mpoly_t A, const nmod_mpoly_t B, const nmod_mpoly_ctx_t ctx) { nmod_mpoly_divexact(Q, A, B, ctx); }
int nmod_mpoly_sqrt__extern(nmod_mpoly_t Q, const nmod_mpoly_t A, const nmod_mpoly_ctx_t ctx) { return nmod_mpoly_sqrt(Q, A, ctx); }
int nmod_mpoly_is_square__extern(const nmod_mpoly_t A, const nmod_mpoly_ctx_t ctx) { return nmod_mpoly_is_square(A, ctx); }
void nmod_mpoly_univar_zero__extern(nmod_mpoly_univar_t A, const nmod_mpoly_ctx_t ctx) { nmod_mpoly_univar_zero(A, ctx); }
void nmod_mpoly_univar_swap__extern(nmod_mpoly_univar_t A, nmod_mpoly_univar_t B, const nmod_mpoly_ctx_t ctx) { nmod_mpoly_univar_swap(A, B, ctx); }
mp_limb_signed_t nmod_mpoly_univar_length__extern(const nmod_mpoly_univar_t A, const nmod_mpoly_ctx_t ctx) { return nmod_mpoly_univar_length(A, ctx); }
void nmod_mpoly_univar_get_term_coeff__extern(nmod_mpoly_t c, const nmod_mpoly_univar_t A, mp_limb_signed_t i, const nmod_mpoly_ctx_t ctx) { nmod_mpoly_univar_get_term_coeff(c, A, i, ctx); }
void nmod_mpoly_univar_swap_term_coeff__extern(nmod_mpoly_t c, nmod_mpoly_univar_t A, mp_limb_signed_t i, const nmod_mpoly_ctx_t ctx) { nmod_mpoly_univar_swap_term_coeff(c, A, i, ctx); }
void nmod_mpolyd_swap__extern(nmod_mpolyd_t poly1, nmod_mpolyd_t poly2) { nmod_mpolyd_swap(poly1, poly2); }
void nmod_mpolyu_swap__extern(nmod_mpolyu_t A, nmod_mpolyu_t B, const nmod_mpoly_ctx_t uctx) { nmod_mpolyu_swap(A, B, uctx); }
void nmod_mpolyu_zero__extern(nmod_mpolyu_t A, const nmod_mpoly_ctx_t uctx) { nmod_mpolyu_zero(A, uctx); }
mp_limb_t nmod_mpolyu_leadcoeff__extern(nmod_mpolyu_t A, const nmod_mpoly_ctx_t ctx) { return nmod_mpolyu_leadcoeff(A, ctx); }
mp_limb_t nmod_mpolyn_leadcoeff__extern(nmod_mpolyn_t A, const nmod_mpoly_ctx_t ctx) { return nmod_mpolyn_leadcoeff(A, ctx); }
n_poly_struct * nmod_mpolyn_leadcoeff_poly__extern(nmod_mpolyn_t A, const nmod_mpoly_ctx_t ctx) { return nmod_mpolyn_leadcoeff_poly(A, ctx); }
void nmod_mpolyun_swap__extern(nmod_mpolyun_t A, nmod_mpolyun_t B) { nmod_mpolyun_swap(A, B); }
mp_limb_t nmod_mpolyun_leadcoeff__extern(nmod_mpolyun_t A, const nmod_mpoly_ctx_t ctx) { return nmod_mpolyun_leadcoeff(A, ctx); }
n_poly_struct * nmod_mpolyun_leadcoeff_poly__extern(nmod_mpolyun_t A, const nmod_mpoly_ctx_t ctx) { return nmod_mpolyun_leadcoeff_poly(A, ctx); }

void nmod_mpoly_factor_init__extern(nmod_mpoly_factor_t f, const nmod_mpoly_ctx_t ctx) { nmod_mpoly_factor_init(f, ctx); }
mp_limb_signed_t nmod_mpoly_factor_length__extern(const nmod_mpoly_factor_t f, const nmod_mpoly_ctx_t ctx) { return nmod_mpoly_factor_length(f, ctx); }
mp_limb_t nmod_mpoly_factor_get_constant_ui__extern(const nmod_mpoly_factor_t f, const nmod_mpoly_ctx_t ctx) { return nmod_mpoly_factor_get_constant_ui(f, ctx); }
void nmod_mpoly_factor_get_base__extern(nmod_mpoly_t p, const nmod_mpoly_factor_t f, mp_limb_signed_t i, const nmod_mpoly_ctx_t ctx) { nmod_mpoly_factor_get_base(p, f, i, ctx); }
void nmod_mpoly_factor_swap_base__extern(nmod_mpoly_t p, nmod_mpoly_factor_t f, mp_limb_signed_t i, const nmod_mpoly_ctx_t ctx) { nmod_mpoly_factor_swap_base(p, f, i, ctx); }
int nmod_mpoly_factor_matches__extern(const nmod_mpoly_t a, const nmod_mpoly_factor_t f, const nmod_mpoly_ctx_t ctx) { return nmod_mpoly_factor_matches(a, f, ctx); }
void nmod_mpoly_factor_swap__extern(nmod_mpoly_factor_t f, nmod_mpoly_factor_t g, const nmod_mpoly_ctx_t ctx) { nmod_mpoly_factor_swap(f, g, ctx); }
void nmod_mpoly_factor_one__extern(nmod_mpoly_factor_t f, const nmod_mpoly_ctx_t ctx) { nmod_mpoly_factor_one(f, ctx); }
void nmod_mpolyv_init__extern(nmod_mpolyv_t A, const nmod_mpoly_ctx_t ctx) { nmod_mpolyv_init(A, ctx); }
void nmod_mpolyv_swap__extern(nmod_mpolyv_t A, nmod_mpolyv_t B, const nmod_mpoly_ctx_t ctx) { nmod_mpolyv_swap(A, B, ctx); }

int signed_mpn_sub_n__extern(mp_ptr res, mp_srcptr op1, mp_srcptr op2, mp_limb_signed_t n) { return signed_mpn_sub_n(res, op1, op2, n); }
void nmod_poly_init_mod__extern(nmod_poly_t poly, const nmod_t mod) { nmod_poly_init_mod(poly, mod); }
void nmod_poly_set_mod__extern(nmod_poly_t poly, const nmod_t mod) { nmod_poly_set_mod(poly, mod); }
void _nmod_poly_set_length__extern(nmod_poly_t poly, mp_limb_signed_t len) { _nmod_poly_set_length(poly, len); }
void _nmod_poly_normalise__extern(nmod_poly_t poly) { _nmod_poly_normalise(poly); }
mp_limb_signed_t nmod_poly_length__extern(const nmod_poly_t poly) { return nmod_poly_length(poly); }
mp_limb_signed_t nmod_poly_degree__extern(const nmod_poly_t poly) { return nmod_poly_degree(poly); }
mp_limb_t nmod_poly_modulus__extern(const nmod_poly_t poly) { return nmod_poly_modulus(poly); }
mp_ptr nmod_poly_lead__extern(const nmod_poly_t poly) { return nmod_poly_lead(poly); }
void nmod_poly_set__extern(nmod_poly_t a, const nmod_poly_t b) { nmod_poly_set(a, b); }
void nmod_poly_swap__extern(nmod_poly_t poly1, nmod_poly_t poly2) { nmod_poly_swap(poly1, poly2); }
void nmod_poly_zero__extern(nmod_poly_t res) { nmod_poly_zero(res); }
void nmod_poly_one__extern(nmod_poly_t res) { nmod_poly_one(res); }
void nmod_poly_truncate__extern(nmod_poly_t poly, mp_limb_signed_t len) { nmod_poly_truncate(poly, len); }
int nmod_poly_is_zero__extern(const nmod_poly_t poly) { return nmod_poly_is_zero(poly); }
int nmod_poly_is_one__extern(const nmod_poly_t poly) { return nmod_poly_is_one(poly); }
int nmod_poly_is_unit__extern(const nmod_poly_t poly) { return nmod_poly_is_unit(poly); }
int nmod_poly_is_gen__extern(const nmod_poly_t poly) { return nmod_poly_is_gen(poly); }
int nmod_poly_is_monic__extern(const nmod_poly_t poly) { return nmod_poly_is_monic(poly); }
void nmod_poly_randtest_not_zero__extern(nmod_poly_t poly, flint_rand_t state, mp_limb_signed_t len) { nmod_poly_randtest_not_zero(poly, state, len); }
mp_limb_t nmod_poly_get_coeff_ui__extern(const nmod_poly_t poly, mp_limb_signed_t j) { return nmod_poly_get_coeff_ui(poly, j); }
void nmod_poly_evaluate_mat__extern(nmod_mat_t dest, const nmod_poly_t poly, const nmod_mat_t c) { nmod_poly_evaluate_mat(dest, poly, c); }
mp_limb_signed_t _nmod_poly_hamming_weight__extern(mp_srcptr a, mp_limb_signed_t len) { return _nmod_poly_hamming_weight(a, len); }
mp_limb_signed_t nmod_poly_hamming_weight__extern(const nmod_poly_t A) { return nmod_poly_hamming_weight(A); }
mp_limb_signed_t _nmod_poly_multi_crt_local_size__extern(const nmod_poly_multi_crt_t CRT) { return _nmod_poly_multi_crt_local_size(CRT); }
const mp_limb_t * nmod_berlekamp_massey_points__extern(const nmod_berlekamp_massey_t B) { return nmod_berlekamp_massey_points(B); }
mp_limb_signed_t nmod_berlekamp_massey_point_count__extern(const nmod_berlekamp_massey_t B) { return nmod_berlekamp_massey_point_count(B); }
const nmod_poly_struct * nmod_berlekamp_massey_V_poly__extern(const nmod_berlekamp_massey_t B) { return nmod_berlekamp_massey_V_poly(B); }
const nmod_poly_struct * nmod_berlekamp_massey_R_poly__extern(const nmod_berlekamp_massey_t B) { return nmod_berlekamp_massey_R_poly(B); }

void nmod_poly_factor_swap__extern(nmod_poly_factor_t a, nmod_poly_factor_t b) { nmod_poly_factor_swap(a, b); }

nmod_poly_struct * nmod_poly_mat_entry__extern(const nmod_poly_mat_t mat, mp_limb_signed_t i, mp_limb_signed_t j) { return nmod_poly_mat_entry(mat, i, j); }
mp_limb_signed_t nmod_poly_mat_nrows__extern(const nmod_poly_mat_t mat) { return nmod_poly_mat_nrows(mat); }
mp_limb_signed_t nmod_poly_mat_ncols__extern(const nmod_poly_mat_t mat) { return nmod_poly_mat_ncols(mat); }
void nmod_poly_mat_swap__extern(nmod_poly_mat_t mat1, nmod_poly_mat_t mat2) { nmod_poly_mat_swap(mat1, mat2); }
void nmod_poly_mat_swap_entrywise__extern(nmod_poly_mat_t mat1, nmod_poly_mat_t mat2) { nmod_poly_mat_swap_entrywise(mat1, mat2); }
void nmod_poly_mat_truncate__extern(nmod_poly_mat_t pmat, long len) { nmod_poly_mat_truncate(pmat, len); }
mp_limb_t nmod_poly_mat_modulus__extern(const nmod_poly_mat_t mat) { return nmod_poly_mat_modulus(mat); }
int nmod_poly_mat_is_empty__extern(const nmod_poly_mat_t mat) { return nmod_poly_mat_is_empty(mat); }
int nmod_poly_mat_is_square__extern(const nmod_poly_mat_t mat) { return nmod_poly_mat_is_square(mat); }
mp_limb_signed_t nmod_poly_mat_degree__extern(const nmod_poly_mat_t pmat) { return nmod_poly_mat_degree(pmat); }

mp_ptr _nmod_vec_init__extern(mp_limb_signed_t len) { return _nmod_vec_init(len); }
void _nmod_vec_clear__extern(mp_ptr vec) { _nmod_vec_clear(vec); }
void _nmod_vec_zero__extern(mp_ptr vec, mp_limb_signed_t len) { _nmod_vec_zero(vec, len); }
void _nmod_vec_set__extern(mp_ptr res, mp_srcptr vec, mp_limb_signed_t len) { _nmod_vec_set(res, vec, len); }
void _nmod_vec_swap__extern(mp_ptr a, mp_ptr b, mp_limb_signed_t length) { _nmod_vec_swap(a, b, length); }
int _nmod_vec_equal__extern(mp_srcptr vec, mp_srcptr vec2, mp_limb_signed_t len) { return _nmod_vec_equal(vec, vec2, len); }
int _nmod_vec_is_zero__extern(mp_srcptr vec, mp_limb_signed_t len) { return _nmod_vec_is_zero(vec, len); }

fmpz * padic_unit__extern(const padic_t x) { return padic_unit(x); }
mp_limb_signed_t padic_get_val__extern(const padic_t x) { return padic_get_val(x); }
mp_limb_signed_t padic_get_prec__extern(const padic_t x) { return padic_get_prec(x); }
int _padic_ctx_pow_ui__extern(fmpz_t rop, mp_limb_t e, const padic_ctx_t ctx) { return _padic_ctx_pow_ui(rop, e, ctx); }
void padic_ctx_pow_ui__extern(fmpz_t rop, mp_limb_t e, const padic_ctx_t ctx) { padic_ctx_pow_ui(rop, e, ctx); }
void _padic_canonicalise__extern(padic_t rop, const padic_ctx_t ctx) { _padic_canonicalise(rop, ctx); }
void padic_swap__extern(padic_t op1, padic_t op2) { padic_swap(op1, op2); }
void padic_zero__extern(padic_t rop) { padic_zero(rop); }
void padic_one__extern(padic_t rop) { padic_one(rop); }
int padic_is_zero__extern(const padic_t op) { return padic_is_zero(op); }
int padic_is_one__extern(const padic_t op) { return padic_is_one(op); }
int padic_equal__extern(const padic_t op1, const padic_t op2) { return padic_equal(op1, op2); }

fmpz_mat_struct * padic_mat__extern(const padic_mat_t A) { return padic_mat(A); }
fmpz * padic_mat_entry__extern(const padic_mat_t A, mp_limb_signed_t i, mp_limb_signed_t j) { return padic_mat_entry(A, i, j); }
mp_limb_signed_t padic_mat_get_val__extern(const padic_mat_t A) { return padic_mat_get_val(A); }
mp_limb_signed_t padic_mat_get_prec__extern(const padic_mat_t A) { return padic_mat_get_prec(A); }
mp_limb_signed_t padic_mat_nrows__extern(const padic_mat_t A) { return padic_mat_nrows(A); }
mp_limb_signed_t padic_mat_ncols__extern(const padic_mat_t A) { return padic_mat_ncols(A); }
int padic_mat_is_empty__extern(const padic_mat_t A) { return padic_mat_is_empty(A); }
int padic_mat_is_square__extern(const padic_mat_t A) { return padic_mat_is_square(A); }
void padic_mat_swap_entrywise__extern(padic_mat_t mat1, padic_mat_t mat2) { padic_mat_swap_entrywise(mat1, mat2); }

mp_limb_signed_t _fmpz_vec_ord_p__extern(const fmpz *vec, mp_limb_signed_t len, const fmpz_t p) { return _fmpz_vec_ord_p(vec, len, p); }
void _padic_poly_set_length__extern(padic_poly_t poly, mp_limb_signed_t len) { _padic_poly_set_length(poly, len); }
void padic_poly_truncate__extern(padic_poly_t poly, mp_limb_signed_t n, const fmpz_t p) { padic_poly_truncate(poly, n, p); }
mp_limb_signed_t padic_poly_degree__extern(const padic_poly_t poly) { return padic_poly_degree(poly); }
mp_limb_signed_t padic_poly_length__extern(const padic_poly_t poly) { return padic_poly_length(poly); }
mp_limb_signed_t padic_poly_val__extern(const padic_poly_t poly) { return padic_poly_val(poly); }
void padic_poly_zero__extern(padic_poly_t poly) { padic_poly_zero(poly); }
void padic_poly_one__extern(padic_poly_t poly) { padic_poly_one(poly); }
int padic_poly_is_zero__extern(const padic_poly_t poly) { return padic_poly_is_zero(poly); }
int padic_poly_is_one__extern(const padic_poly_t poly) { return padic_poly_is_one(poly); }

mp_limb_signed_t * _perm_init__extern(mp_limb_signed_t n) { return _perm_init(n); }
void _perm_clear__extern(mp_limb_signed_t *vec) { _perm_clear(vec); }
mp_limb_signed_t _perm_equal__extern(const mp_limb_signed_t *vec1, const mp_limb_signed_t *vec2, mp_limb_signed_t n) { return _perm_equal(vec1, vec2, n); }
void _perm_set__extern(mp_limb_signed_t *res, const mp_limb_signed_t *vec, mp_limb_signed_t n) { _perm_set(res, vec, n); }
void _perm_one__extern(mp_limb_signed_t *vec, mp_limb_signed_t n) { _perm_one(vec, n); }
void _perm_inv__extern(mp_limb_signed_t *res, const mp_limb_signed_t *vec, mp_limb_signed_t n) { _perm_inv(res, vec, n); }
void _perm_compose__extern(mp_limb_signed_t *res, const mp_limb_signed_t *vec1, const mp_limb_signed_t *vec2, mp_limb_signed_t n) { _perm_compose(res, vec1, vec2, n); }

mp_limb_signed_t qadic_val__extern(const qadic_t op) { return qadic_val(op); }
mp_limb_signed_t qadic_prec__extern(const qadic_t op) { return qadic_prec(op); }
mp_limb_signed_t qadic_ctx_degree__extern(const qadic_ctx_t ctx) { return qadic_ctx_degree(ctx); }
void qadic_ctx_print__extern(const qadic_ctx_t ctx) { qadic_ctx_print(ctx); }
void qadic_init__extern(qadic_t x) { qadic_init(x); }
void qadic_init2__extern(qadic_t rop, mp_limb_signed_t prec) { qadic_init2(rop, prec); }
void qadic_clear__extern(qadic_t x) { qadic_clear(x); }
void _fmpz_poly_reduce__extern(fmpz *R, mp_limb_signed_t lenR, const fmpz *a, const mp_limb_signed_t *j, mp_limb_signed_t len) { _fmpz_poly_reduce(R, lenR, a, j, len); }
void _fmpz_mod_poly_reduce__extern(fmpz *R, mp_limb_signed_t lenR, const fmpz *a, const mp_limb_signed_t *j, mp_limb_signed_t len, const fmpz_t p) { _fmpz_mod_poly_reduce(R, lenR, a, j, len, p); }
void qadic_reduce__extern(qadic_t x, const qadic_ctx_t ctx) { qadic_reduce(x, ctx); }
void qadic_randtest__extern(qadic_t x, flint_rand_t state, const qadic_ctx_t ctx) { qadic_randtest(x, state, ctx); }
void qadic_randtest_not_zero__extern(qadic_t x, flint_rand_t state, const qadic_ctx_t ctx) { qadic_randtest_not_zero(x, state, ctx); }
void qadic_randtest_val__extern(qadic_t x, flint_rand_t state, mp_limb_signed_t val, const qadic_ctx_t ctx) { qadic_randtest_val(x, state, val, ctx); }
void qadic_randtest_int__extern(qadic_t x, flint_rand_t state, const qadic_ctx_t ctx) { qadic_randtest_int(x, state, ctx); }
void qadic_zero__extern(qadic_t op) { qadic_zero(op); }
void qadic_one__extern(qadic_t op) { qadic_one(op); }
void qadic_gen__extern(qadic_t x, const qadic_ctx_t ctx) { qadic_gen(x, ctx); }
void qadic_set_ui__extern(qadic_t rop, mp_limb_t op, const qadic_ctx_t ctx) { qadic_set_ui(rop, op, ctx); }
int qadic_get_padic__extern(padic_t rop, const qadic_t op, const qadic_ctx_t ctx) { return qadic_get_padic(rop, op, ctx); }
void qadic_set__extern(qadic_t rop, const qadic_t op, const qadic_ctx_t ctx) { qadic_set(rop, op, ctx); }
int qadic_is_zero__extern(const qadic_t op) { return qadic_is_zero(op); }
int qadic_is_one__extern(const qadic_t op) { return qadic_is_one(op); }
int qadic_equal__extern(const qadic_t op1, const qadic_t op2) { return qadic_equal(op1, op2); }
void qadic_add__extern(qadic_t x, const qadic_t y, const qadic_t z, const qadic_ctx_t ctx) { qadic_add(x, y, z, ctx); }
void qadic_sub__extern(qadic_t x, const qadic_t y, const qadic_t z, const qadic_ctx_t ctx) { qadic_sub(x, y, z, ctx); }
void qadic_neg__extern(qadic_t x, const qadic_t y, const qadic_ctx_t ctx) { qadic_neg(x, y, ctx); }

void qfb_init__extern(qfb_t q) { qfb_init(q); }
void qfb_clear__extern(qfb_t q) { qfb_clear(q); }
int qfb_equal__extern(qfb_t f, qfb_t g) { return qfb_equal(f, g); }
void qfb_set__extern(qfb_t f, qfb_t g) { qfb_set(f, g); }
void qfb_discriminant__extern(fmpz_t D, qfb_t f) { qfb_discriminant(D, f); }
void qfb_array_clear__extern(qfb **forms, mp_limb_signed_t num) { qfb_array_clear(forms, num); }
void qfb_inverse__extern(qfb_t r, qfb_t f) { qfb_inverse(r, f); }
int qfb_is_principal_form__extern(qfb_t f, fmpz_t D) { return qfb_is_principal_form(f, D); }
void qfb_principal_form__extern(qfb_t f, fmpz_t D) { qfb_principal_form(f, D); }
int qfb_is_primitive__extern(qfb_t f) { return qfb_is_primitive(f); }

qqbar_ptr _qqbar_vec_init__extern(mp_limb_signed_t len) { return _qqbar_vec_init(len); }
void _qqbar_vec_clear__extern(qqbar_ptr vec, mp_limb_signed_t len) { _qqbar_vec_clear(vec, len); }
mp_limb_signed_t qqbar_degree__extern(const qqbar_t x) { return qqbar_degree(x); }
int qqbar_is_rational__extern(const qqbar_t x) { return qqbar_is_rational(x); }
int qqbar_is_integer__extern(const qqbar_t x) { return qqbar_is_integer(x); }
int qqbar_is_algebraic_integer__extern(const qqbar_t x) { return qqbar_is_algebraic_integer(x); }
int qqbar_is_zero__extern(const qqbar_t x) { return qqbar_is_zero(x); }
int qqbar_is_one__extern(const qqbar_t x) { return qqbar_is_one(x); }
int qqbar_is_neg_one__extern(const qqbar_t x) { return qqbar_is_neg_one(x); }
int qqbar_is_i__extern(const qqbar_t x) { return qqbar_is_i(x); }
int qqbar_is_neg_i__extern(const qqbar_t x) { return qqbar_is_neg_i(x); }
int qqbar_is_real__extern(const qqbar_t x) { return qqbar_is_real(x); }
int qqbar_within_limits__extern(const qqbar_t x, mp_limb_signed_t deg_limit, mp_limb_signed_t bits_limit) { return qqbar_within_limits(x, deg_limit, bits_limit); }
int qqbar_binop_within_limits__extern(const qqbar_t x, const qqbar_t y, mp_limb_signed_t deg_limit, mp_limb_signed_t bits_limit) { return qqbar_binop_within_limits(x, y, deg_limit, bits_limit); }
void qqbar_zero__extern(qqbar_t res) { qqbar_zero(res); }
void qqbar_one__extern(qqbar_t res) { qqbar_one(res); }
void qqbar_sqr__extern(qqbar_t res, const qqbar_t x) { qqbar_sqr(res, x); }
void qqbar_sqrt__extern(qqbar_t res, const qqbar_t x) { qqbar_sqrt(res, x); }
void qqbar_sqrt_ui__extern(qqbar_t res, mp_limb_t x) { qqbar_sqrt_ui(res, x); }
void qqbar_rsqrt__extern(qqbar_t res, const qqbar_t x) { qqbar_rsqrt(res, x); }

void insert_col_entry__extern(la_col_t *col, mp_limb_signed_t entry) { insert_col_entry(col, entry); }
void swap_cols__extern(la_col_t *col2, la_col_t *col1) { swap_cols(col2, col1); }
void clear_col__extern(la_col_t *col) { clear_col(col); }
void free_col__extern(la_col_t *col) { free_col(col); }

mp_limb_t n_gcd__extern(mp_limb_t x, mp_limb_t y) { return n_gcd(x, y); }
int n_mul_checked__extern(mp_limb_t *a, mp_limb_t b, mp_limb_t c) { return n_mul_checked(a, b, c); }
int n_add_checked__extern(mp_limb_t *a, mp_limb_t b, mp_limb_t c) { return n_add_checked(a, b, c); }
int n_sub_checked__extern(mp_limb_t *a, mp_limb_t b, mp_limb_t c) { return n_sub_checked(a, b, c); }
double n_precompute_inverse__extern(mp_limb_t n) { return n_precompute_inverse(n); }
mp_limb_t n_mulmod_shoup__extern(mp_limb_t w, mp_limb_t t, mp_limb_t w_precomp, mp_limb_t p) { return n_mulmod_shoup(w, t, w_precomp, p); }
mp_limb_t n_mulmod2_preinv__extern(mp_limb_t a, mp_limb_t b, mp_limb_t n, mp_limb_t ninv) { return n_mulmod2_preinv(a, b, n, ninv); }
mp_limb_t n_mulmod2__extern(mp_limb_t a, mp_limb_t b, mp_limb_t n) { return n_mulmod2(a, b, n); }
mp_limb_t n_powmod__extern(mp_limb_t a, mp_limb_signed_t exp, mp_limb_t n) { return n_powmod(a, exp, n); }
mp_limb_t n_powmod2__extern(mp_limb_t a, mp_limb_signed_t exp, mp_limb_t n) { return n_powmod2(a, exp, n); }
mp_limb_t n_addmod__extern(mp_limb_t x, mp_limb_t y, mp_limb_t n) { return n_addmod(x, y, n); }
mp_limb_t n_submod__extern(mp_limb_t x, mp_limb_t y, mp_limb_t n) { return n_submod(x, y, n); }
mp_limb_t n_negmod__extern(mp_limb_t x, mp_limb_t n) { return n_negmod(x, n); }
mp_limb_t n_invmod__extern(mp_limb_t x, mp_limb_t y) { return n_invmod(x, y); }
void n_factor_init__extern(n_factor_t *factors) { n_factor_init(factors); }

