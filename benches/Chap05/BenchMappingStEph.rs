// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 05: Mapping (functional relation, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap05::MappingStEph::MappingStEph::*;
use apas_verus::Types::Types::Pair;

fn bench_mapping_from_vec(c: &mut Criterion) {
    let mut group = c.benchmark_group("MappingFromVec");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[64usize, 256] {
        let pairs: Vec<Pair<u64, u64>> = (0..n as u64).map(|i| Pair(i, i * 2)).collect();
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, _| {
            b.iter(|| MappingStEph::<u64, u64>::from_vec(pairs.clone()));
        });
    }
    group.finish();
}

fn bench_mapping_mem(c: &mut Criterion) {
    let mut group = c.benchmark_group("MappingMem");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[64usize, 256] {
        let pairs: Vec<Pair<u64, u64>> = (0..n as u64).map(|i| Pair(i, i * 2)).collect();
        let m = MappingStEph::<u64, u64>::from_vec(pairs);
        let query = Pair(n as u64 / 2, (n as u64 / 2) * 2);
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, _| {
            b.iter(|| m.mem(&query));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_mapping_from_vec, bench_mapping_mem);
criterion_main!(benches);
