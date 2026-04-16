// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 23: Primitive tree-backed sequence (persistent) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap23::PrimTreeSeqStPer::PrimTreeSeqStPer::*;

fn build_seq(n: usize) -> PrimTreeSeqStS<u64> {
    PrimTreeSeqStS::<u64>::from_vec((0..n as u64).collect())
}

fn bench_prim_tree_append(c: &mut Criterion) {
    let mut group = c.benchmark_group("PrimTreeSeqAppend");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[64usize, 256] {
        let a = build_seq(n);
        let b = build_seq(n);
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b_crit, _| {
            b_crit.iter(|| PrimTreeSeqStS::<u64>::append(&a, &b));
        });
    }
    group.finish();
}

fn bench_prim_tree_subseq(c: &mut Criterion) {
    let mut group = c.benchmark_group("PrimTreeSeqSubseq");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[64usize, 256] {
        let a = build_seq(n);
        group.bench_with_input(BenchmarkId::new("half", n), &n, |b, _| {
            b.iter(|| a.subseq(0, n / 2));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_prim_tree_append, bench_prim_tree_subseq);
criterion_main!(benches);
