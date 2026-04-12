//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 26: Merge sort (multi-threaded, persistent) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap26::MergeSortMtPer::MergeSortMtPer::*;
use apas_verus::Chap18::ArraySeqMtPer::ArraySeqMtPer::ArraySeqMtPerS;
use apas_verus::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::set_parallelism;

fn build_reverse(n: usize) -> ArraySeqMtPerS<usize> {
    ArraySeqMtPerS { seq: (0..n).rev().collect() }
}

fn bench_merge_sort_parallel(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("MergeSortMtPer");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[64usize, 256] {
        let arr = build_reverse(n);
        group.bench_with_input(BenchmarkId::new("reverse", n), &n, |b, _| {
            b.iter(|| <ArraySeqMtPerS<usize> as MergeSortMtTrait>::merge_sort_parallel(&arr));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_merge_sort_parallel);
criterion_main!(benches);
