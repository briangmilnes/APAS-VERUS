// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 37: Weight-balanced BST (sequential, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, BatchSize};
use apas_verus::Chap37::BSTBBAlphaStEph::BSTBBAlphaStEph::*;

fn bench_bstbb_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("BSTBBInsert");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter_batched(
                || <BSTBBAlphaStEph<u64> as BSTBBAlphaStEphTrait<u64>>::new(),
                |tree| {
                    let mut t = tree;
                    for i in 0..n as u64 { t = t.insert(i); }
                    t
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_bstbb_contains(c: &mut Criterion) {
    let mut group = c.benchmark_group("BSTBBContains");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[32usize, 64] {
        let mut tree = <BSTBBAlphaStEph<u64> as BSTBBAlphaStEphTrait<u64>>::new();
        for i in 0..n as u64 { tree = tree.insert(i); }
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| tree.contains(&(n as u64 / 2)));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_bstbb_insert, bench_bstbb_contains);
criterion_main!(benches);
