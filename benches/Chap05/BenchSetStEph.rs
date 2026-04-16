// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 05: Set (ephemeral, hash-backed) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap05::SetStEph::SetStEph::*;

fn bench_set_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("SetInsert");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[64usize, 256] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| {
                let mut s = SetStEph::<u64>::empty();
                for i in 0..n as u64 {
                    s.insert(i);
                }
                s
            });
        });
    }
    group.finish();
}

fn bench_set_union(c: &mut Criterion) {
    let mut group = c.benchmark_group("SetUnion");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[64usize, 256] {
        let mut s1 = SetStEph::<u64>::empty();
        let mut s2 = SetStEph::<u64>::empty();
        for i in 0..n as u64 { s1.insert(i); }
        for i in n as u64..2 * n as u64 { s2.insert(i); }
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, _| {
            b.iter(|| s1.union(&s2));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_set_insert, bench_set_union);
criterion_main!(benches);
