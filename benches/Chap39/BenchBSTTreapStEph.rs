// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 39: Treap BST (sequential, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap39::BSTTreapStEph::BSTTreapStEph::*;

fn bench_treap_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("TreapInsert");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| {
                let mut tree = <BSTTreapStEph<u64> as BSTTreapStEphTrait<u64>>::new();
                for i in 0..n as u64 { tree.insert(i, i * 6364136223846793005); }
                tree
            });
        });
    }
    group.finish();
}

fn bench_treap_contains(c: &mut Criterion) {
    let mut group = c.benchmark_group("TreapContains");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[32usize, 64] {
        let mut tree = <BSTTreapStEph<u64> as BSTTreapStEphTrait<u64>>::new();
        for i in 0..n as u64 { tree.insert(i, i * 6364136223846793005); }
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| tree.contains(&(n as u64 / 2)));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_treap_insert, bench_treap_contains);
criterion_main!(benches);
