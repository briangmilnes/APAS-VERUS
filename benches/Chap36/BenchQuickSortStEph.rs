// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 36: QuickSort benchmarks (first-element pivot, median-of-3 pivot).

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use apas_verus::Chap36::QuickSortStEph::QuickSortStEph::*;
use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS;

fn build_reverse(n: usize) -> ArraySeqStEphS<u64> {
    ArraySeqStEphS { seq: (0..n as u64).rev().collect() }
}

fn bench_quicksort_first(c: &mut Criterion) {
    let mut group = c.benchmark_group("QuickSortFirst");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(250));
    for &n in &[32usize, 128] {
        group.bench_with_input(BenchmarkId::new("reverse", n), &n, |b, &n| {
            b.iter_batched(
                || build_reverse(n),
                |mut a| ArraySeqStEphS::<u64>::quick_sort_first(&mut a),
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_quicksort_median3(c: &mut Criterion) {
    let mut group = c.benchmark_group("QuickSortMedian3");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(250));
    for &n in &[32usize, 128] {
        group.bench_with_input(BenchmarkId::new("reverse", n), &n, |b, &n| {
            b.iter_batched(
                || build_reverse(n),
                |mut a| ArraySeqStEphS::<u64>::quick_sort_median3(&mut a),
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

criterion_group!(benches, bench_quicksort_first, bench_quicksort_median3);
criterion_main!(benches);
