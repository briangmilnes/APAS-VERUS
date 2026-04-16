// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 43: Ordered table (sequential, persistent) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use apas_verus::Chap43::OrderedTableStPer::OrderedTableStPer::*;

fn bench_ord_table_per_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrdTableStPerInsert");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter_batched(
                || <OrderedTableStPer<u64, u64> as OrderedTableStPerTrait<u64, u64>>::empty(),
                |t| {
                    let mut cur = t;
                    for i in 0..n as u64 {
                        cur = cur.insert(i, i);
                    }
                    cur
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

criterion_group!(benches, bench_ord_table_per_insert);
criterion_main!(benches);
