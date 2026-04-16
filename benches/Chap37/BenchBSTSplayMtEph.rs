// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 37: Splay BST (multi-threaded, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use apas_verus::Chap37::BSTSplayMtEph::BSTSplayMtEph::*;
use apas_verus::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::set_parallelism;

fn bench_bstsplay_mt_insert(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("BSTSplayMtEphInsert");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter_batched(
                || <BSTSplayMtEph<u64> as BSTSplayMtEphTrait<u64>>::new(),
                |mut tree| {
                    for i in 0..n as u64 { let _ = tree.insert(i); }
                    tree
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_bstsplay_mt_contains(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("BSTSplayMtEphContains");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[32usize, 64] {
        let mut tree = <BSTSplayMtEph<u64> as BSTSplayMtEphTrait<u64>>::new();
        for i in 0..n as u64 { let _ = tree.insert(i); }
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| tree.contains(&(n as u64 / 2)));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_bstsplay_mt_insert, bench_bstsplay_mt_contains);
criterion_main!(benches);
