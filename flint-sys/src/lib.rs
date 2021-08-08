//! Bindings for the [FLINT](http://flintlib.org/sphinx/index.html) library.
//! Crates marked with an asterisk have functions which may require mutable borrows where const
//! borrows will suffice (these need to be corrected but the bindings will still work as expected).

pub mod deps;
pub mod flint;

pub mod perm;
pub mod mpoly;

pub mod ulong_extras;
pub mod fmpz;
pub mod fmpz_vec;
pub mod fmpz_factor;
pub mod fmpz_mat;
pub mod fmpz_lll;
pub mod fmpz_poly;
pub mod fmpz_poly_mat;
pub mod fmpz_poly_factor;
pub mod fmpz_mpoly;
pub mod fmpz_mpoly_factor;
pub mod long_extras;
pub mod arith;
pub mod fft;
pub mod qsieve;

pub mod fmpq;
pub mod fmpq_vec;
pub mod fmpq_mat;
pub mod fmpq_poly;
pub mod fmpq_mpoly;
pub mod fmpz_poly_q;

pub mod n_poly;
pub mod nmod_vec;
pub mod nmod_mat;
pub mod nmod_poly;
pub mod nmod_poly_mat;
pub mod nmod_poly_factor;
pub mod nmod_mpoly;
pub mod fmpz_mod;
pub mod fmpz_mod_mat;
pub mod fmpz_mod_poly;
pub mod fmpz_mod_poly_factor;

pub mod fq;
pub mod fq_vec;
pub mod fq_mat;
pub mod fq_poly;
pub mod fq_poly_factor;
pub mod fq_nmod;
pub mod fq_nmod_vec;
pub mod fq_nmod_mat;
pub mod fq_nmod_poly;
pub mod fq_nmod_poly_factor;
pub mod fq_zech;
pub mod fq_zech_vec;
pub mod fq_zech_mat;
pub mod fq_zech_poly;
pub mod fq_zech_poly_factor;

pub mod padic;
pub mod padic_poly;
pub mod padic_mat;
pub mod qadic;
