//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 43: Ordered table (sequential, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap43::OrderedTableStEph::OrderedTableStEph::*;

fn bench_ordtable_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrdTableInsert");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| {
                let mut t = <OrderedTableStEph<u64, u64> as OrderedTableStEphTrait<u64, u64>>::empty();
                for i in 0..n as u64 {
                    t.insert(i, i, |_old, new| *new);
                }
                t
            });
        });
    }
    group.finish();
}

fn bench_ordtable_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrdTableLookup");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[32usize, 64] {
        let mut t = <OrderedTableStEph<u64, u64> as OrderedTableStEphTrait<u64, u64>>::empty();
        for i in 0..n as u64 { t.insert(i, i, |_old, new| *new); }
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| t.lookup(&(n as u64 / 2)));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_ordtable_insert, bench_ordtable_lookup);
criterion_main!(benches);
