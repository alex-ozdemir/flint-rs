# flint-sys

Bindings to the [FLINT](http://flintlib.org/) library.

FLINT (Fast Library for Number Theory) is a C library which provides a number
of number theoretic and algebraic functions and types.

## Binding Coverage

Most of FLINT is not covered (feel free to file and issue or submit a PR if
you'd like!). The following are:

   * Most of `fmpz_mod_poly.h`.
   * Most of `fmpz.h`.

## Dependencies

You must install the following before installing this crate:

   * [FLINT](http://flintlib.org/) version 2.7.0 or greater

### Arch Linux

```
pacman -S flint
```

### Ubuntu

```
apt install libflint-dev
```
