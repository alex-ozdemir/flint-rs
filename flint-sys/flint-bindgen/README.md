# flint-bindgen

Generate bindings to the FLINT library with bindgen. These are used to create the `flint-sys` bindings.

Run the binding generation script from the `flint-sys` directory:

```sh
flint-bindgen/scripts/gen_bindings.sh
```

The script builds GMP/MPFR via `gmp-mpfr-sys`, builds and installs the bundled
FLINT source, generates the FLINT header lists, runs bindgen, installs the
generated Rust modules into `src`, consolidates static function wrappers into
`C/extern.c`, and runs `cargo fix`.
