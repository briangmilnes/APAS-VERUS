//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 65: Union-find with array (sequential, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use apas_verus::Chap65::UnionFindArrayStEph::UnionFindArrayStEph::*;

fn bench_union_find_array_union(c: &mut Criterion) {
    let mut group = c.benchmark_group("UnionFindArrayStEphUnion");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter_batched(
                || <UnionFindArray as UnionFindArrayStEphTrait>::new(n),
                |mut uf| {
                    for i in 0..n - 1 { uf.union(i, i + 1); }
                    uf
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_union_find_array_find(c: &mut Criterion) {
    let mut group = c.benchmark_group("UnionFindArrayStEphFind");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[32usize, 64] {
        let mut uf = <UnionFindArray as UnionFindArrayStEphTrait>::new(n);
        for i in 0..n - 1 { uf.union(i, i + 1); }
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| uf.find(n - 1));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_union_find_array_union, bench_union_find_array_find);
criterion_main!(benches);
