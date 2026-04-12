//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 43: Augmented ordered table (sequential, persistent) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use apas_verus::Chap43::AugOrderedTableStPer::AugOrderedTableStPer::*;

type AugFn = fn(&u64, &u64) -> u64;

fn bench_aug_table_per_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("AugOrdTableStPerInsert");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    let reducer: AugFn = |a, b| a + b;
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter_batched(
                || <AugOrderedTableStPer<u64, u64, AugFn> as AugOrderedTableStPerTrait<u64, u64, AugFn>>::empty(reducer, 0u64),
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

criterion_group!(benches, bench_aug_table_per_insert);
criterion_main!(benches);
