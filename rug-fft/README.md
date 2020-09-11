# rug-fft

Implementations of the FFT for [rug][rug] integers.

That is, implementation of the Number Theoretic Transform.

## Algorithms

   * Cooley-Tukey, in-place, with bit reversals, iteratively.
   * Cooley-Tukey, partially in-place, recursively
   * Naive

The two Cooley-Tukey variations perform similarly in my (unrigorous tests).

[rug]: https://crates.io/crates/rug
