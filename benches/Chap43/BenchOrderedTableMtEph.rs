// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 43: Ordered table (multi-threaded, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap43::OrderedTableMtEph::OrderedTableMtEph::*;
use apas_verus::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::set_parallelism;

fn bench_ord_table_mt_insert(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("OrdTableMtEphInsert");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| {
                let mut t = <OrderedTableMtEph<u64, u64> as OrderedTableMtEphTrait<u64, u64>>::empty();
                for i in 0..n as u64 {
                    t.insert(i, i, |_old, new| *new);
                }
                t
            });
        });
    }
    group.finish();
}

fn bench_ord_table_mt_lookup(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("OrdTableMtEphLookup");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[32usize, 64] {
        let mut t = <OrderedTableMtEph<u64, u64> as OrderedTableMtEphTrait<u64, u64>>::empty();
        for i in 0..n as u64 {
            t.insert(i, i, |_old, new| *new);
        }
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| t.lookup(&(n as u64 / 2)));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_ord_table_mt_insert, bench_ord_table_mt_lookup);
criterion_main!(benches);
