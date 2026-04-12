//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 43: Augmented ordered table (multi-threaded, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap43::AugOrderedTableMtEph::AugOrderedTableMtEph::*;
use apas_verus::Concurrency::Concurrency::*;

type AugFn = fn(&u64, &u64) -> u64;

fn bench_aug_table_mt_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("AugOrdTableMtEphInsert");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    let reducer: AugFn = |a, b| a + b;
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| {
                let mut t = <AugOrderedTableMtEph<u64, u64, AugFn> as AugOrderedTableMtEphTrait<u64, u64, AugFn>>::empty(reducer, 0u64);
                for i in 0..n as u64 {
                    t.insert(i, i, |_old, new| *new);
                }
                t
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_aug_table_mt_insert);
criterion_main!(benches);
