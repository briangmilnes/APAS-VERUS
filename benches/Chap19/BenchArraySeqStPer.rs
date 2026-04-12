//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 19: ArraySeq (sequential, persistent) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;

fn build_seq(n: usize) -> ArraySeqStPerS<u64> {
    ArraySeqStPerS::<u64>::from_vec((0..n as u64).collect())
}

fn bench_append(c: &mut Criterion) {
    let mut group = c.benchmark_group("Chap19ArraySeqStPerAppend");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[256usize, 1024] {
        let a = build_seq(n);
        let b = build_seq(n);
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b_crit, _| {
            b_crit.iter(|| ArraySeqStPerS::<u64>::append(&a, &b));
        });
    }
    group.finish();
}

fn bench_subseq(c: &mut Criterion) {
    let mut group = c.benchmark_group("Chap19ArraySeqStPerSubseq");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[256usize, 1024] {
        let a = build_seq(n);
        group.bench_with_input(BenchmarkId::new("half", n), &n, |b, _| {
            b.iter(|| ArraySeqStPerS::<u64>::subseq(&a, 0, n / 2));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_append, bench_subseq);
criterion_main!(benches);
