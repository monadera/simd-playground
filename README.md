# Faster Rust with SIMD

This repository contains the code for the
[Faster Rust with SIMD](https://monadera.com/blog/faster-rust-with-simd/)
blog post. It contains an incomplete bitset implementation using
an abstract block for the bits. The block is implemented in three
different ways - using a simple `usize`, using `u64x4` from
[the standard library](https://doc.rust-lang.org/std/simd/index.html)
(nightly-only), and using `u64x4` from
[wide](https://docs.rs/wide/latest/wide/).

## Running the benchmarks

Make sure to run the benchmarks with all features enabled:

```shell
cargo bench --all-features
```

As the benchmarks use the experimental SIMD APIs in the
standard library, they require `nightly` Rust.
