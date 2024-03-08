#![feature(portable_simd)]

use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use simd_playground::BitSet;

const N: usize = 1000_000;

fn bench_union_usize(c: &mut Criterion) {
    let s1: BitSet<usize> = BitSet::zeros(N);
    let s2: BitSet<usize> = BitSet::ones(N);

    c.bench_function("union with usize", |b| {
        b.iter_batched(
            || (s1.clone(), s2.clone()),
            |(x, y)| {
                black_box(x | y);
            },
            BatchSize::SmallInput,
        );
    });
}

fn bench_union_simd(c: &mut Criterion) {
    use std::simd::u64x4;
    let s1: BitSet<u64x4> = BitSet::zeros(N);
    let s2: BitSet<u64x4> = BitSet::ones(N);

    c.bench_function("union with std SIMD", |b| {
        b.iter_batched(
            || (s1.clone(), s2.clone()),
            |(x, y)| {
                black_box(x | y);
            },
            BatchSize::SmallInput,
        );
    });
}

fn bench_union_wide(c: &mut Criterion) {
    use wide::u64x4;
    let s1: BitSet<u64x4> = BitSet::zeros(N);
    let s2: BitSet<u64x4> = BitSet::ones(N);

    c.bench_function("union with wide SIMD", |b| {
        b.iter_batched(
            || (s1.clone(), s2.clone()),
            |(x, y)| {
                black_box(x | y);
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(
    benches,
    bench_union_usize,
    bench_union_simd,
    bench_union_wide
);
criterion_main!(benches);
