#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use rug::{integer::IsPrime, Integer};
use std::mem::replace;

fn check_ntt_params(xs: &[Integer], p: &Integer, w: &Integer) {
    debug_assert_ne!(p.is_probably_prime(100), IsPrime::No);
    let n = xs.len();
    for i in 1..n {
        debug_assert_ne!(
            w.clone().pow_mod(&Integer::from(i), p).unwrap(),
            Integer::from(1)
        );
    }
    debug_assert_eq!(
        w.clone().pow_mod(&Integer::from(n), p).unwrap(),
        Integer::from(1)
    );
}

/// Computes, for `i` in `0..n`, `sum_{j=0}^{n-1} w^{ij}*x_j`, modulo `p`.
///
/// # Example
///
/// ```
/// use rug::Integer;
/// use rug_fft::naive_ntt;
///
/// let xs = vec![1, 4]
///     .into_iter()
///     .map(Integer::from)
///     .collect::<Vec<_>>();
/// let p = Integer::from(7);
/// let w = Integer::from(6);
/// let ys = naive_ntt(&xs, &p, &w);
/// let ys_ex = vec![5, 4]
///     .into_iter()
///     .map(Integer::from)
///     .collect::<Vec<_>>();
/// assert_eq!(ys, ys_ex);
/// ```
pub fn naive_ntt(xs: &[Integer], p: &Integer, w: &Integer) -> Vec<Integer> {
    check_ntt_params(xs, p, w);
    let n = xs.len();
    (0..n)
        .map(|i| {
            let wi = w.clone().pow_mod(&Integer::from(i), p).unwrap();
            // Horner
            let mut acc = xs[n - 1].clone();
            for j in (0..(n - 1)).rev() {
                acc *= &wi;
                acc += &xs[j];
                acc %= p;
            }
            acc
        })
        .collect()
}

/// Computes, for `i` in `0..n`, `x_i` such that `y_i = sum_{j=0}^{n-1} w^{ij}*x_j`, modulo `p`.
///
/// # Example
///
/// ```
/// use rug::Integer;
/// use rug_fft::naive_intt;
///
/// let xs = vec![5, 4]
///     .into_iter()
///     .map(Integer::from)
///     .collect::<Vec<_>>();
/// let p = Integer::from(7);
/// let w = Integer::from(6);
/// let ys = naive_intt(&xs, &p, &w);
/// let ys_ex = vec![1, 4]
///     .into_iter()
///     .map(Integer::from)
///     .collect::<Vec<_>>();
/// assert_eq!(ys, ys_ex);
/// ```
pub fn naive_intt(ys: &[Integer], p: &Integer, w: &Integer) -> Vec<Integer> {
    let n_inv = {
        let mut t = Integer::from(ys.len());
        t.invert_mut(p).unwrap();
        t
    };
    let ntt = naive_ntt(ys, p, &Integer::from(w.invert_ref(p).unwrap()));
    ntt.into_iter().map(|i| i * &n_inv % p).collect()
}

/// Computes, for `i` in `0..n`, `sum_{j=0}^{n-1} w^{ij}*x_j`, modulo `p`.
///
/// Requires `n` to be a power of two, and `w` to be an `n`th root of unity.
///
/// # Example
///
/// ```
/// use rug::Integer;
/// use rug_fft::cooley_tukey_radix_2_ntt;
///
/// let mut xs = vec![1, 4]
///     .into_iter()
///     .map(Integer::from)
///     .collect::<Vec<_>>();
/// let p = Integer::from(7);
/// let w = Integer::from(6);
/// cooley_tukey_radix_2_ntt(&mut xs, &p, &w);
/// let ys_ex = vec![5, 4]
///     .into_iter()
///     .map(Integer::from)
///     .collect::<Vec<_>>();
/// assert_eq!(xs, ys_ex);
/// ```
pub fn cooley_tukey_radix_2_ntt(xs: &mut [Integer], p: &Integer, w: &Integer) {
    assert!(Integer::from(xs.len()).is_power_of_two());
    let mut ws = Vec::with_capacity(xs.len());
    ws.push(Integer::from(1));
    for _ in 0..(xs.len() - 1) {
        ws.push(Integer::from(ws.last().unwrap() * w) % p);
        debug_assert_ne!(ws.last().unwrap(), &Integer::from(1));
    }
    debug_assert_eq!(w.clone() * ws.last().unwrap() % p, Integer::from(1));
    cooley_tukey_radix_2_ntt_h(xs, p, &ws, 1);
}

/// Computes, for `i` in `0..n`, `x_i` such that `y_i = sum_{j=0}^{n-1} w^{ij}*x_j`, modulo `p`.
///
/// Requires `n` to be a power of two, and `w` to be an `n`th root of unity.
///
/// # Example
///
/// ```
/// use rug::Integer;
/// use rug_fft::cooley_tukey_radix_2_intt;
///
/// let mut xs = vec![5, 4]
///     .into_iter()
///     .map(Integer::from)
///     .collect::<Vec<_>>();
/// let p = Integer::from(7);
/// let w = Integer::from(6);
/// cooley_tukey_radix_2_intt(&mut xs, &p, &w);
/// let ys_ex = vec![1, 4]
///     .into_iter()
///     .map(Integer::from)
///     .collect::<Vec<_>>();
/// assert_eq!(xs, ys_ex);
/// ```
pub fn cooley_tukey_radix_2_intt(ys: &mut [Integer], p: &Integer, w: &Integer) {
    let n_inv = {
        let mut t = Integer::from(ys.len());
        t.invert_mut(p).unwrap();
        t
    };
    cooley_tukey_radix_2_ntt(ys, p, &Integer::from(w.invert_ref(p).unwrap()));
    for y in ys {
        *y *= &n_inv;
        *y %= p;
    }
}

fn cooley_tukey_radix_2_ntt_h(xs: &mut [Integer], p: &Integer, ws: &[Integer], wi: usize) {
    let n = xs.len();
    if n < 2 {
        return;
    }

    // Split
    let mut odd = (0..n / 2)
        .map(|i| replace(&mut xs[2 * i + 1], Integer::new()))
        .collect::<Vec<_>>();
    for i in 1..n / 2 {
        xs.swap(i, 2 * i);
    }

    // Recurse
    cooley_tukey_radix_2_ntt_h(&mut odd, p, ws, 2 * wi);
    cooley_tukey_radix_2_ntt_h(&mut xs[..n / 2], p, ws, 2 * wi);

    // Merge
    for (i, o) in odd.into_iter().enumerate() {
        xs[n / 2 + i] = xs[i].clone();
        let f = o * &ws[wi * i % ws.len()] % p;
        xs[i] += &f;
        xs[i] %= p;
        xs[n / 2 + i] -= &f;
        xs[n / 2 + i] += p;
        xs[n / 2 + i] %= p;
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use quickcheck::{Arbitrary, Gen};

    // Thanks: https://www.nayuki.io/page/number-theoretic-transform-integer-dft
    #[test]
    fn naive_ntt_5() {
        let xs = vec![6, 0, 10, 7, 2]
            .into_iter()
            .map(Integer::from)
            .collect::<Vec<_>>();
        let p = Integer::from(11);
        let w = Integer::from(3);
        let ys = naive_ntt(&xs, &p, &w);
        let ys_ex = vec![3, 7, 0, 5, 4]
            .into_iter()
            .map(Integer::from)
            .collect::<Vec<_>>();
        assert_eq!(ys, ys_ex);
    }

    #[test]
    fn naive_ntt_8() {
        let xs = vec![4, 1, 4, 2, 1, 3, 5, 6]
            .into_iter()
            .map(Integer::from)
            .collect::<Vec<_>>();
        let p = Integer::from(673);
        let w = Integer::from(326);
        let ys = naive_ntt(&xs, &p, &w);
        let ys_ex = vec![26, 338, 228, 115, 2, 457, 437, 448]
            .into_iter()
            .map(Integer::from)
            .collect::<Vec<_>>();
        assert_eq!(ys, ys_ex);
    }

    #[test]
    fn naive_ntt_2() {
        let xs = vec![1, 4]
            .into_iter()
            .map(Integer::from)
            .collect::<Vec<_>>();
        let p = Integer::from(7);
        let w = Integer::from(6);
        let ys = naive_ntt(&xs, &p, &w);
        let ys_ex = vec![5, 4]
            .into_iter()
            .map(Integer::from)
            .collect::<Vec<_>>();
        assert_eq!(ys, ys_ex);
    }

    #[test]
    fn naive_round_trip_8() {
        let xs = vec![4, 1, 4, 2, 1, 3, 5, 6]
            .into_iter()
            .map(Integer::from)
            .collect::<Vec<_>>();
        let p = Integer::from(673);
        let w = Integer::from(326);
        let ys = naive_ntt(&xs, &p, &w);
        let xs2 = naive_intt(&ys, &p, &w);
        assert_eq!(xs, xs2);
    }

    #[test]
    fn ct_ntt_2() {
        let xs = vec![1, 4]
            .into_iter()
            .map(Integer::from)
            .collect::<Vec<_>>();
        let p = Integer::from(7);
        let w = Integer::from(6);
        let mut ys = xs;
        cooley_tukey_radix_2_ntt(&mut ys, &p, &w);
        let ys_ex = vec![5, 4]
            .into_iter()
            .map(Integer::from)
            .collect::<Vec<_>>();
        assert_eq!(ys, ys_ex);
    }

    #[test]
    fn ct_ntt_8() {
        let xs = vec![4, 1, 4, 2, 1, 3, 5, 6]
            .into_iter()
            .map(Integer::from)
            .collect::<Vec<_>>();
        let p = Integer::from(673);
        let w = Integer::from(326);
        let mut ys = xs;
        cooley_tukey_radix_2_ntt(&mut ys, &p, &w);
        let ys_ex = vec![26, 338, 228, 115, 2, 457, 437, 448]
            .into_iter()
            .map(Integer::from)
            .collect::<Vec<_>>();
        assert_eq!(ys, ys_ex);
    }

    #[test]
    fn ct_round_trip_8() {
        let xs = vec![4, 1, 4, 2, 1, 3, 5, 6]
            .into_iter()
            .map(Integer::from)
            .collect::<Vec<_>>();
        let mut ys = xs.clone();
        let p = Integer::from(673);
        let w = Integer::from(326);
        cooley_tukey_radix_2_ntt(&mut ys, &p, &w);
        cooley_tukey_radix_2_intt(&mut ys, &p, &w);
        assert_eq!(xs, ys);
    }

    #[derive(Clone, Debug)]
    struct NttInput {
        p: Integer,
        w: Integer,
        xs: Vec<Integer>,
    }

    impl Arbitrary for NttInput {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let p = Integer::from(673);
            let mut w = Integer::from(118); // Order 32
            let lg_size = g.next_u32() % 6;
            let size = 1 << lg_size;
            w = w.pow_mod(&Integer::from(32 / size), &p).unwrap();
            let xs = std::iter::repeat_with(|| Integer::from(g.next_u32()) % &p)
                .take(size)
                .collect::<Vec<_>>();
            NttInput { p, w, xs }
        }
    }

    #[quickcheck]
    fn ct_round_trip_32(input: NttInput) -> bool {
        let mut ys = input.xs.clone();
        cooley_tukey_radix_2_ntt(&mut ys, &input.p, &input.w);
        cooley_tukey_radix_2_intt(&mut ys, &input.p, &input.w);
        ys == input.xs
    }
}
