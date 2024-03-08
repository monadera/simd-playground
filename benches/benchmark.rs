#![feature(portable_simd)]

use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use simd_playground::BitSet;
use std::simd::usizex8;

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

fn bench_union_usizex8(c: &mut Criterion) {
    let s1: BitSet<usizex8> = BitSet::zeros(N);
    let s2: BitSet<usizex8> = BitSet::ones(N);

    c.bench_function("union with SIMD", |b| {
        b.iter_batched(
            || (s1.clone(), s2.clone()),
            |(x, y)| {
                black_box(x | y);
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(benches, bench_union_usize, bench_union_usizex8);
criterion_main!(benches);
