#!/bin/sh
set -euo pipefail
# Invoke from flint-sys as flint-bindgen/scripts/gen_bindings.sh
#
# Override the bundled FLINT source version with either:
#   flint-bindgen/scripts/gen_bindings.sh 3.5.0
#   FLINT_VERSION=3.5.0 flint-bindgen/scripts/gen_bindings.sh
FLINT_VERSION=${FLINT_VERSION:-3.5.0}

if [ "${1:-}" = "-h" ] || [ "${1:-}" = "--help" ]; then
    echo "Usage: $0 [FLINT_VERSION]"
    echo
    echo "Defaults to FLINT_VERSION=${FLINT_VERSION}."
    exit 0
fi

if [ "$#" -gt 1 ]; then
    echo "Usage: $0 [FLINT_VERSION]" >&2
    exit 2
fi

if [ "$#" -eq 1 ]; then
    FLINT_VERSION=$1
fi

FLINT_SRC_DIR="flint-${FLINT_VERSION}"

logdir=$(realpath flint-bindgen/scripts/logs)
mkdir -p ${logdir}
# rm ${logdir}/*

if [ ! -x "${FLINT_SRC_DIR}/configure" ]; then
    echo "Expected ${FLINT_SRC_DIR}/configure to exist and be executable." >&2
    exit 1
fi

echo "Removing old build artifacts..."
cargo clean
rm -f C/extern.c src/__mod_statements.rs
pushd flint-bindgen
cargo clean
rm -f scripts/bindgen_headers.rs scripts/build_headers.rs
popd
echo "Starting flint-sys build to get gmp and mpfr..."
cargo b >${logdir}/00-gmp_mpfr.log 2>&1 || true
gmp_mpfr_dir=$(realpath $(find target/debug/build -type d -name "gmp-mpfr-sys-*" -execdir test -d "{}/out" \; -print))
echo "Building FLINT ${FLINT_VERSION}..."
rm -rf flint-bindgen/flint-build
rm -rf flint-bindgen/flint-out
mkdir -p flint-bindgen/flint-build
mkdir -p flint-bindgen/flint-out
pushd flint-bindgen/flint-build

echo "  Running configure..."
"../../${FLINT_SRC_DIR}/configure" --disable-shared \
    --with-mpfr=${gmp_mpfr_dir}/out \
    --with-gmp=${gmp_mpfr_dir}/out \
    CFLAGS="-fPIC" \
    --prefix=$(realpath ../flint-out) > ${logdir}/01-flint-configure.log
echo "  Running make..."
make -j $(nproc) > ${logdir}/02-flint-make.log
echo "  Running make install to get public header files..."
make install > ${logdir}/03-flint-install.log

popd

echo "  FLINT build success"
echo "Generating header list..."
pushd flint-bindgen/scripts
python headers.py ../flint-out/include/flint
popd
echo "Generating bindings..."
deps_include_dir=${gmp_mpfr_dir}/out/include
pushd flint-bindgen
if [ -d "${deps_include_dir}" ]; then
    INCLUDE_DIR=flint-out/include DEPS_INCLUDE_DIR=${deps_include_dir} cargo b -vv >${logdir}/04-bindgen.log 2>&1
else
    INCLUDE_DIR=flint-out/include cargo b -vv >${logdir}/04-bindgen.log 2>&1
fi
bindings_dir=$(realpath $(find target/debug/build -type d -name "flint-bindgen-*" -execdir test -d "{}/out" \; -print))
popd
echo "Consolidating function wrappers..."
python flint-bindgen/scripts/externs.py ${bindings_dir}/out/extern > C/extern.c
echo "Removing stale binding modules..."
find src -type f -name "*.rs" -not \( -name "lib.rs" -o -name "bindgen.rs" -o -name "deps.rs" \) -print0 | xargs -0 rm
echo "Installing new modules..."
mv flint-bindgen/scripts/__mod_statements.rs src/
cp ${bindings_dir}/out/*.rs src/
echo "Fixing warnings (unused imports)..."
cargo fix --lib -p flint-sys --allow-dirty --broken-code >${logdir}/05-warnings.log 2>&1
