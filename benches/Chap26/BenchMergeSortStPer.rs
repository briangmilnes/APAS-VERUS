//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 26: Merge sort (persistent, sequential) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap26::MergeSortStPer::MergeSortStPer::*;
use apas_verus::Chap18::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS;

fn build_reverse(n: usize) -> ArraySeqStPerS<usize> {
    ArraySeqStPerS { seq: (0..n).rev().collect() }
}

fn bench_merge_sort(c: &mut Criterion) {
    let mut group = c.benchmark_group("MergeSortStPer");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[64usize, 256] {
        let arr = build_reverse(n);
        group.bench_with_input(BenchmarkId::new("reverse", n), &n, |b, _| {
            b.iter(|| ArraySeqStPerS::<usize>::merge_sort(&arr));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_merge_sort);
criterion_main!(benches);
