# flint-bindgen

Generate bindings to the FLINT library with bindgen. These are used to create the `flint-sys` bindings.

Run the binding generation script from the `flint-sys` directory:

```sh
flint-bindgen/scripts/gen_bindings.sh
```

By default, the script uses the bundled `flint-3.5.0` source directory. To
generate bindings for a different bundled FLINT source version, pass the version
as the first argument or set `FLINT_VERSION`:

```sh
flint-bindgen/scripts/gen_bindings.sh 3.5.0
FLINT_VERSION=3.5.0 flint-bindgen/scripts/gen_bindings.sh
```

The script builds GMP/MPFR via `gmp-mpfr-sys`, builds and installs the bundled
FLINT source, generates the FLINT header lists, runs bindgen, installs the
generated Rust modules into `src`, consolidates static function wrappers into
`C/extern.c`, and runs `cargo fix`.
