//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 37: AVL BST set (multi-threaded, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use apas_verus::Chap37::BSTSetAVLMtEph::BSTSetAVLMtEph::*;
use apas_verus::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::set_parallelism;

fn bench_bstset_avl_mt_insert(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("BSTSetAVLMtEphInsert");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter_batched(
                || <BSTSetAVLMtEph<u64> as BSTSetAVLMtEphTrait<u64>>::empty(),
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

fn bench_bstset_avl_mt_contains(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("BSTSetAVLMtEphContains");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[32usize, 64] {
        let mut set = <BSTSetAVLMtEph<u64> as BSTSetAVLMtEphTrait<u64>>::empty();
        for i in 0..n as u64 { let _ = set.insert(i); }
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| set.contains(&(n as u64 / 2)));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_bstset_avl_mt_insert, bench_bstset_avl_mt_contains);
criterion_main!(benches);
