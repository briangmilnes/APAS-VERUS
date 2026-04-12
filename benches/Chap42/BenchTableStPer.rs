//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 42: Table (sequential, persistent) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, BatchSize};
use apas_verus::Chap42::TableStPer::TableStPer::*;

fn bench_table_per_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("TableStPerInsert");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter_batched(
                || <TableStPer<u64, u64> as TableStPerTrait<u64, u64>>::empty(),
                |t| {
                    let mut cur = t;
                    for i in 0..n as u64 {
                        cur = cur.insert(i, i, |_old, new| *new);
                    }
                    cur
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

criterion_group!(benches, bench_table_per_insert);
criterion_main!(benches);
