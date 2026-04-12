//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 65: Union-find without path compression (sequential, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use apas_verus::Chap65::UnionFindNoPCStEph::UnionFindNoPCStEph::*;

fn bench_union_find_no_pc_union(c: &mut Criterion) {
    let mut group = c.benchmark_group("UnionFindNoPCStEphUnion");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[16usize, 32] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter_batched(
                || <UnionFind<usize> as UnionFindStEphTrait<usize>>::new(),
                |mut uf| {
                    for i in 0..n {
                        let v = i;
                        let u = (i + 1) % n;
                        uf.union_sets(&v, &u);
                    }
                    uf
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_union_find_no_pc_find(c: &mut Criterion) {
    let mut group = c.benchmark_group("UnionFindNoPCStEphFind");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[16usize, 32] {
        let mut uf = <UnionFind<usize> as UnionFindStEphTrait<usize>>::new();
        for i in 0..n { uf.union_sets(&i, &((i + 1) % n)); }
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, _| {
            b.iter(|| uf.find(&0));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_union_find_no_pc_union, bench_union_find_no_pc_find);
criterion_main!(benches);
