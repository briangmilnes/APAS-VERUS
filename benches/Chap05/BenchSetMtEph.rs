// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 05: Set (multi-threaded, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap05::SetMtEph::SetMtEph::*;
use apas_verus::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::set_parallelism;

fn bench_set_mt_insert(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("SetMtEphInsert");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[64usize, 256] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| {
                let mut s = SetMtEph::<u64>::empty();
                for i in 0..n as u64 {
                    let _ = s.insert(i);
                }
                s
            });
        });
    }
    group.finish();
}

fn bench_set_mt_union(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("SetMtEphUnion");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[64usize, 256] {
        let mut s1 = SetMtEph::<u64>::empty();
        let mut s2 = SetMtEph::<u64>::empty();
        for i in 0..n as u64 { let _ = s1.insert(i); }
        for i in n as u64..2 * n as u64 { let _ = s2.insert(i); }
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, _| {
            b.iter(|| s1.union(&s2));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_set_mt_insert, bench_set_mt_union);
criterion_main!(benches);
