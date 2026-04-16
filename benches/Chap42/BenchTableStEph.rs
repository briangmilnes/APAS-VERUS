// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 42: Table (sequential, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap42::TableStEph::TableStEph::*;

fn bench_table_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("TableStEphInsert");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| {
                let mut t = <TableStEph<u64, u64> as TableStEphTrait<u64, u64>>::empty();
                for i in 0..n as u64 {
                    t.insert(i, i, |_old, new| *new);
                }
                t
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_table_insert);
criterion_main!(benches);
