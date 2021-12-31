#![feature(test)]

extern crate test;

use rug::Integer;
use rug_fft::*;
use std::str::FromStr;
use test::Bencher;

fn bench_fft<F: Fn(&mut [Integer], &Integer, &Integer)>(fft: F, b: &mut Bencher) {
    // BLS12-381 Scalar field.
    let p = Integer::from_str(
        "52435875175126190479447740508185965837690552500527637822603658699938581184513",
    )
    .unwrap();
    let w_base = Integer::from_str(
        "10238227357739495823651030575849232062558860180284477541189508159991286009131",
    )
    .unwrap();
    let s = 32;
    let n = 8;
    assert!(n <= s);
    let w = w_base.pow_mod(&Integer::from(s - n), &p).unwrap();
    let mut rng = rug::rand::RandState::new();
    let mut ys = std::iter::repeat_with(|| Integer::from(p.random_below_ref(&mut rng)))
        .take(1 << n)
        .collect::<Vec<_>>();
    b.iter(|| {
        fft(&mut ys, &p, &w);
    });
}

#[bench]
fn bit_rev(b: &mut Bencher) {
    bench_fft(rug_fft::bit_rev_radix_2_ntt, b);
}

#[bench]
fn cooley_tukey(b: &mut Bencher) {
    bench_fft(rug_fft::cooley_tukey_radix_2_ntt, b);
}

#[bench]
fn naive(b: &mut Bencher) {
    bench_fft(rug_fft::naive_ntt, b);
}
