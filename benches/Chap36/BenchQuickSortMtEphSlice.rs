// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 36: Quicksort slice variant (multi-threaded, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use apas_verus::Chap36::QuickSortMtEphSlice::QuickSortMtEphSlice::*;
use apas_verus::Chap19::ArraySeqMtEphSlice::ArraySeqMtEphSlice::*;
use apas_verus::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::set_parallelism;

fn build_seq(n: usize) -> ArraySeqMtEphSliceS<u64> {
    ArraySeqMtEphSliceS::<u64>::from_vec((0..n as u64).rev().collect())
}

fn bench_quick_sort_first_slice(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("QuickSortFirstMtSlice");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[64usize, 256] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter_batched(
                || build_seq(n),
                |mut a| <ArraySeqMtEphSliceS<u64> as QuickSortMtEphSliceTrait<u64>>::quick_sort_first(&mut a),
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

criterion_group!(benches, bench_quick_sort_first_slice);
criterion_main!(benches);
