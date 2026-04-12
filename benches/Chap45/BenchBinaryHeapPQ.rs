//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 45: Binary heap priority queue benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use apas_verus::Chap45::BinaryHeapPQ::BinaryHeapPQ::*;

fn bench_binary_heap_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("BinaryHeapPQInsert");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter_batched(
                || <BinaryHeapPQ<u64> as BinaryHeapPQTrait<u64>>::empty(),
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

fn bench_binary_heap_delete_min(c: &mut Criterion) {
    let mut group = c.benchmark_group("BinaryHeapPQDeleteMin");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter_batched(
                || {
                    let mut pq = <BinaryHeapPQ<u64> as BinaryHeapPQTrait<u64>>::empty();
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

criterion_group!(benches, bench_binary_heap_insert, bench_binary_heap_delete_min);
criterion_main!(benches);
