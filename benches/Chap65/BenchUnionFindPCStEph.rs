// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 65: Union-Find with path compression benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use apas_verus::Chap65::UnionFindPCStEph::UnionFindPCStEph::*;

/// Build a fresh UnionFind with n elements already inserted.
fn build_uf(n: usize) -> UnionFindPC<usize> {
    let mut uf = UnionFindPC::<usize>::new();
    for i in 0..n {
        uf.insert(i);
    }
    uf
}

fn bench_union_find_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("UnionFindInsert");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(250));
    for &n in &[64usize, 256] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter_batched(
                || UnionFindPC::<usize>::new(),
                |mut uf| {
                    for i in 0..n {
                        uf.insert(i);
                    }
                    uf
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_union_find_union(c: &mut Criterion) {
    let mut group = c.benchmark_group("UnionFindUnion");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(250));
    for &n in &[64usize, 256] {
        group.bench_with_input(BenchmarkId::new("chain_n", n), &n, |b, &n| {
            b.iter_batched(
                || build_uf(n),
                |mut uf| {
                    for i in 1..n {
                        uf.union(&(i - 1), &i);
                    }
                    uf
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

criterion_group!(benches, bench_union_find_insert, bench_union_find_union);
criterion_main!(benches);
