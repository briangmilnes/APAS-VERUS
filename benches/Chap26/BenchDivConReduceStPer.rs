//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 26: Divide-and-conquer reduce (sequential, persistent) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap26::DivConReduceStPer::DivConReduceStPer::*;
use apas_verus::Chap18::ArraySeqStPer::ArraySeqStPer::*;

fn build_seq(n: usize) -> ArraySeqStPerS<usize> {
    ArraySeqStPerS::<usize>::from_vec((0..n).collect())
}

fn bench_max_element(c: &mut Criterion) {
    let mut group = c.benchmark_group("DivConMaxElement");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[64usize, 256] {
        let a = build_seq(n);
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, _| {
            b.iter(|| <ArraySeqStPerS<usize> as DivConReduceStTrait>::max_element(&a));
        });
    }
    group.finish();
}

fn bench_sum(c: &mut Criterion) {
    let mut group = c.benchmark_group("DivConSum");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[64usize, 256] {
        let a = build_seq(n);
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, _| {
            b.iter(|| <ArraySeqStPerS<usize> as DivConReduceStTrait>::sum(&a));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_max_element, bench_sum);
criterion_main!(benches);
