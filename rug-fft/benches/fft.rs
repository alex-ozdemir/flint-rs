#![feature(test)]

extern crate test;

use rug_fft::*;
use rug::Integer;
use std::str::FromStr;
use test::Bencher;

#[bench]
fn bit_rev(b: &mut Bencher) {
    let p = Integer::from_str(
        "52435875175126190479447740508185965837690552500527637822603658699938581184513",
    )
    .unwrap();
    let w_base = Integer::from_str(
        "10238227357739495823651030575849232062558860180284477541189508159991286009131",
    )
    .unwrap();
    let s = 32;
    let n = 10;
    assert!(n <= s);
    let w = w_base.pow_mod(&Integer::from(s - n), &p).unwrap();
    let mut rng = rug::rand::RandState::new();
    let mut ys = std::iter::repeat_with(|| Integer::from(p.random_below_ref(&mut rng)))
        .take(1 << n)
        .collect::<Vec<_>>();
    b.iter(|| {
        bit_rev_radix_2_ntt(&mut ys, &p, &w);
    });
}

#[bench]
fn cooley_tukey(b: &mut Bencher) {
    let p = Integer::from_str(
        "52435875175126190479447740508185965837690552500527637822603658699938581184513",
    )
    .unwrap();
    let w_base = Integer::from_str(
        "10238227357739495823651030575849232062558860180284477541189508159991286009131",
    )
    .unwrap();
    let s = 32;
    let n = 10;
    assert!(n <= s);
    let w = w_base.pow_mod(&Integer::from(s - n), &p).unwrap();
    let mut rng = rug::rand::RandState::new();
    let mut ys = std::iter::repeat_with(|| Integer::from(p.random_below_ref(&mut rng)))
        .take(1 << n)
        .collect::<Vec<_>>();
    b.iter(|| {
        cooley_tukey_radix_2_ntt(&mut ys, &p, &w);
    });
}

#[bench]
fn naive(b: &mut Bencher) {
    let p = Integer::from_str(
        "52435875175126190479447740508185965837690552500527637822603658699938581184513",
    )
    .unwrap();
    let w_base = Integer::from_str(
        "10238227357739495823651030575849232062558860180284477541189508159991286009131",
    )
    .unwrap();
    let s = 32;
    let n = 5;
    assert!(n <= s);
    let w = w_base.pow_mod(&Integer::from(s - n), &p).unwrap();
    let mut rng = rug::rand::RandState::new();
    let mut ys = std::iter::repeat_with(|| Integer::from(p.random_below_ref(&mut rng)))
        .take(1 << n)
        .collect::<Vec<_>>();
    b.iter(|| {
        naive_ntt(&ys, &p, &w);
    });
}
