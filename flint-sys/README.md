# flint-sys

Bindings to the [FLINT](http://flintlib.org/) library.

FLINT (Fast Library for Number Theory) is a C library which provides a number
of number theoretic and algebraic functions and types.

## Binding Coverage

Most of FLINT is covered with the exception of some functions not listed in the documentation (these are to come).

Internal crates marked with an asterisk in the documentation have functions which may require mutable borrows where const borrows will suffice (these need to be corrected but the bindings will still work as expected).

These bindings have been tested with FLINT 2.8.0. Feedback on other versions of FLINT is appreciated.

## Dependencies

You must install the following before installing this crate:

   * [FLINT](http://flintlib.org/) version 2.8 or greater

### Arch Linux

```
pacman -S flint
```

### Ubuntu

```
apt install libflint-dev
```

Currently if FLINT is installed from source it must be located in `/usr/local/`.
