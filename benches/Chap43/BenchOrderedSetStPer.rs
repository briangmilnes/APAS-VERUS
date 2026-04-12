//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 43: Ordered set (sequential, persistent) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use apas_verus::Chap43::OrderedSetStPer::OrderedSetStPer::*;

fn bench_ord_set_per_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrdSetStPerInsert");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter_batched(
                || <OrderedSetStPer<u64> as OrderedSetStPerTrait<u64>>::empty(),
                |s| {
                    let mut cur = s;
                    for i in 0..n as u64 {
                        cur = cur.insert(i);
                    }
                    cur
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

criterion_group!(benches, bench_ord_set_per_insert);
criterion_main!(benches);
