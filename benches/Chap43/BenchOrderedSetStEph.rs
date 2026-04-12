//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 43: Ordered set (sequential, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap43::OrderedSetStEph::OrderedSetStEph::*;

fn bench_ordered_set_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrdSetStEphInsert");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| {
                let mut s = <OrderedSetStEph<u64> as OrderedSetStEphTrait<u64>>::empty();
                for i in 0..n as u64 {
                    s.insert(i);
                }
                s
            });
        });
    }
    group.finish();
}

fn bench_ordered_set_first_last(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrdSetStEphFirstLast");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[32usize, 64] {
        let mut s = <OrderedSetStEph<u64> as OrderedSetStEphTrait<u64>>::empty();
        for i in 0..n as u64 {
            s.insert(i);
        }
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, _| {
            b.iter(|| {
                let f = s.first();
                let l = s.last();
                (f, l)
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_ordered_set_insert, bench_ordered_set_first_last);
criterion_main!(benches);
