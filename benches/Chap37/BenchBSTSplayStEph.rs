//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 37: Splay BST (sequential, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, BatchSize};
use apas_verus::Chap37::BSTSplayStEph::BSTSplayStEph::*;

fn bench_bstsplay_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("BSTSplayInsert");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter_batched(
                || <BSTSplayStEph<u64> as BSTSplayStEphTrait<u64>>::new(),
                |mut tree| {
                    for i in 0..n as u64 { tree.insert(i); }
                    tree
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_bstsplay_contains(c: &mut Criterion) {
    let mut group = c.benchmark_group("BSTSplayContains");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[32usize, 64] {
        let mut tree = <BSTSplayStEph<u64> as BSTSplayStEphTrait<u64>>::new();
        for i in 0..n as u64 { tree.insert(i); }
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| tree.contains(&(n as u64 / 2)));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_bstsplay_insert, bench_bstsplay_contains);
criterion_main!(benches);
