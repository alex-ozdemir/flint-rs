use rug_polynomial::ModPoly;
use rug::Integer;
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
    let ys = std::iter::repeat_with(|| Integer::from(p.random_below_ref(&mut rng)))
        .take(1 << n)
        .collect::<Vec<_>>();
    let start = Instant::now();
    let p = ModPoly::interpolate_from_mul_subgroup(ys.clone(), p.clone(), &w);
    let end = Instant::now();
    println!("{}", p.len());
    println!("{}s", (end - start).as_secs_f64());
}
