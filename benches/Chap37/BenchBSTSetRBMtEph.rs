// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 37: Red-black BST set (multi-threaded, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use apas_verus::Chap37::BSTSetRBMtEph::BSTSetRBMtEph::*;
use apas_verus::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::set_parallelism;

fn bench_bstset_rb_mt_insert(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("BSTSetRBMtEphInsert");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter_batched(
                || <BSTSetRBMtEph<u64> as BSTSetRBMtEphTrait<u64>>::empty(),
                |mut set| {
                    for i in 0..n as u64 { let _ = set.insert(i); }
                    set
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_bstset_rb_mt_contains(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("BSTSetRBMtEphContains");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[32usize, 64] {
        let mut set = <BSTSetRBMtEph<u64> as BSTSetRBMtEphTrait<u64>>::empty();
        for i in 0..n as u64 { let _ = set.insert(i); }
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| set.contains(&(n as u64 / 2)));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_bstset_rb_mt_insert, bench_bstset_rb_mt_contains);
criterion_main!(benches);
