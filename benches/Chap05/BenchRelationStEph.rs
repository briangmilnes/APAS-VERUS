// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 05: Relation (set of pairs, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap05::RelationStEph::RelationStEph::*;
use apas_verus::Types::Types::Pair;

fn bench_relation_from_vec(c: &mut Criterion) {
    let mut group = c.benchmark_group("RelationFromVec");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[64usize, 256] {
        let pairs: Vec<Pair<u64, u64>> = (0..n as u64).map(|i| Pair(i, i + 1)).collect();
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, _| {
            b.iter(|| RelationStEph::<u64, u64>::from_vec(pairs.clone()));
        });
    }
    group.finish();
}

fn bench_relation_mem(c: &mut Criterion) {
    let mut group = c.benchmark_group("RelationMem");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[64usize, 256] {
        let pairs: Vec<Pair<u64, u64>> = (0..n as u64).map(|i| Pair(i, i + 1)).collect();
        let r = RelationStEph::<u64, u64>::from_vec(pairs);
        let k = n as u64 / 2;
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, _| {
            b.iter(|| r.mem(&k, &(k + 1)));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_relation_from_vec, bench_relation_mem);
criterion_main!(benches);
