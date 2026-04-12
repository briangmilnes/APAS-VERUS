//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 41: Array-backed set (sequential, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap41::ArraySetStEph::ArraySetStEph::*;

fn build_set(n: usize) -> ArraySetStEph<u64> {
    let mut s = <ArraySetStEph<u64> as ArraySetStEphTrait<u64>>::empty();
    for i in 0..n as u64 { s.insert(i); }
    s
}

fn bench_arrayset_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("ArraySetInsert");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| build_set(n));
        });
    }
    group.finish();
}

fn bench_arrayset_union(c: &mut Criterion) {
    let mut group = c.benchmark_group("ArraySetUnion");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[32usize, 64] {
        let s1 = build_set(n);
        let s2 = build_set(n);
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, _| {
            b.iter(|| s1.union(&s2));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_arrayset_insert, bench_arrayset_union);
criterion_main!(benches);
