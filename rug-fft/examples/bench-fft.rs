use rug::Integer;
use rug_fft::*;
use std::str::FromStr;
use std::time::Instant;

fn main() {
    // Bn12-381, Fr
    let p = Integer::from_str(
        "52435875175126190479447740508185965837690552500527637822603658699938581184513",
    )
    .unwrap();
    let w_base = Integer::from_str(
        "10238227357739495823651030575849232062558860180284477541189508159991286009131",
    )
    .unwrap();
    let s = 32;
    let n = std::env::args()
        .nth(1)
        .and_then(|s| usize::from_str(s.as_str()).ok())
        .expect("Provide the log of the number points of the first argument");
    assert!(n <= s);
    let w = w_base.pow_mod(&Integer::from(s - n), &p).unwrap();
    let mut rng = rug::rand::RandState::new();
    let mut ys = std::iter::repeat_with(|| Integer::from(p.random_below_ref(&mut rng)))
        .take(1 << n)
        .collect::<Vec<_>>();
    let start = Instant::now();
    match std::env::args()
        .nth(2)
        .expect("Provide an FFT name as the second arg")
        .as_str()
    {
        "br" => bit_rev_radix_2_ntt(&mut ys, &p, &w),
        "ct" => cooley_tukey_radix_2_ntt(&mut ys, &p, &w),
        "naive" => naive_ntt(&mut ys, &p, &w),
        _ => panic!("Acceptable FFTs: br, ct, naive"),
    }
    let end = Instant::now();
    println!("{}", (end - start).as_secs_f64());
}
