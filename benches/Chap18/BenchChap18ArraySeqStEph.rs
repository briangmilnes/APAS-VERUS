// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 18: ArraySeq (sequential, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap18::ArraySeqStEph::ArraySeqStEph::*;

fn build_seq(n: usize) -> ArraySeqStEphS<u64> {
    ArraySeqStEphS { seq: (0..n as u64).collect() }
}

fn bench_append(c: &mut Criterion) {
    let mut group = c.benchmark_group("Chap18ArraySeqStEphAppend");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[256usize, 1024] {
        let a = build_seq(n);
        let b = build_seq(n);
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b_crit, _| {
            b_crit.iter(|| ArraySeqStEphS::<u64>::append(&a, &b));
        });
    }
    group.finish();
}

fn bench_length(c: &mut Criterion) {
    let mut group = c.benchmark_group("Chap18ArraySeqStEphLength");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[256usize, 1024] {
        let a = build_seq(n);
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, _| {
            b.iter(|| a.length());
        });
    }
    group.finish();
}

criterion_group!(benches, bench_append, bench_length);
criterion_main!(benches);
