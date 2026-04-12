//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 41: Array set enum (multi-threaded, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use apas_verus::Chap41::ArraySetEnumMtEph::ArraySetEnumMtEph::*;

fn bench_arraysetenummteph_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("ArraySetEnumMtEphInsert");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter_batched(
                || <ArraySetEnumMtEph as ArraySetEnumMtEphTrait>::new(n),
                |mut s| {
                    for i in 0..n { s.insert(i); }
                    s
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

criterion_group!(benches, bench_arraysetenummteph_insert);
criterion_main!(benches);
