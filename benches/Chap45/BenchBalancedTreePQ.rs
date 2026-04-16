// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 45: Balanced tree priority queue benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use apas_verus::Chap45::BalancedTreePQ::BalancedTreePQ::*;

fn bench_balanced_tree_pq_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("BalancedTreePQInsert");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter_batched(
                || <BalancedTreePQ<u64> as BalancedTreePQTrait<u64>>::empty(),
                |pq| {
                    let mut cur = pq;
                    for i in 0..n as u64 {
                        cur = cur.insert(n as u64 - i);
                    }
                    cur
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_balanced_tree_pq_delete_min(c: &mut Criterion) {
    let mut group = c.benchmark_group("BalancedTreePQDeleteMin");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter_batched(
                || {
                    let mut pq = <BalancedTreePQ<u64> as BalancedTreePQTrait<u64>>::empty();
                    for i in 0..n as u64 { pq = pq.insert(i); }
                    pq
                },
                |pq| pq.delete_min(),
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

criterion_group!(benches, bench_balanced_tree_pq_insert, bench_balanced_tree_pq_delete_min);
criterion_main!(benches);
