// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 26: Divide-and-conquer reduce (multi-threaded, persistent) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap26::DivConReduceMtPer::DivConReduceMtPer::*;
use apas_verus::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
use apas_verus::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::set_parallelism;

fn build_seq(n: usize) -> ArraySeqMtPerS<usize> {
    ArraySeqMtPerS::<usize>::from_vec((0..n).collect())
}

fn bench_max_element_parallel(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("DivConMaxElementMt");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[64usize, 256] {
        let a = build_seq(n);
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, _| {
            b.iter(|| <ArraySeqMtPerS<usize> as DivConReduceMtTrait>::max_element_parallel(&a));
        });
    }
    group.finish();
}

fn bench_sum_parallel(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("DivConSumMt");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[64usize, 256] {
        let a = build_seq(n);
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, _| {
            b.iter(|| <ArraySeqMtPerS<usize> as DivConReduceMtTrait>::sum_parallel(&a));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_max_element_parallel, bench_sum_parallel);
criterion_main!(benches);
