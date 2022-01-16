# flint-sys

Rust bindings to the [FLINT](http://flintlib.org/) library. 

FLINT (Fast Library for Number Theory) is a C library which provides a number
of number theoretic and algebraic functions and types.

## Usage

See the [documentation](https://docs.rs/flint-sys/latest/flint_sys/). This crate is available on [crates.io](https://crates.io/crates/flint-sys).

## Optional features

  * `disable-make-check`: this can reduce compilation time significantly.

## Notes

  * As of version 0.6.0 the FLINT source files are now included and the library is compiled automatically. The files are cached to avoid unnecessary compilations.

  * Some binding arguments may be marked mutable instead of `const`. These are being manually updated over time, but feel free to correct any bindings and make a pull request if you would like certain ones fixed right away.
