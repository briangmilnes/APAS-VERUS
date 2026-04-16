// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 39: Parallel Treap BST (multi-threaded, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use apas_verus::Chap39::BSTParaTreapMtEph::BSTParaTreapMtEph::*;
use apas_verus::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::set_parallelism;

fn bench_para_treap_insert(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("BSTParaTreapMtEphInsert");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter_batched(
                || <ParamTreap<i32> as ParamTreapTrait<i32>>::new(),
                |mut tree| {
                    for i in 0..n as i32 { tree.insert(i); }
                    tree
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_para_treap_find(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("BSTParaTreapMtEphFind");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[32usize, 64] {
        let mut tree = <ParamTreap<i32> as ParamTreapTrait<i32>>::new();
        for i in 0..n as i32 { tree.insert(i); }
        let mid = n as i32 / 2;
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, _| {
            b.iter(|| tree.find(&mid));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_para_treap_insert, bench_para_treap_find);
criterion_main!(benches);
