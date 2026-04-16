// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 49: Subset sum (multi-threaded, persistent) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use apas_verus::Chap49::SubsetSumMtPer::SubsetSumMtPer::*;
use apas_verus::Chap18::ArraySeqMtPer::ArraySeqMtPer::ArraySeqMtPerS;
use apas_verus::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::set_parallelism;

fn build_solver(n: usize) -> SubsetSumMtPerS<i32> {
    let multiset = ArraySeqMtPerS::<i32> { seq: (1..=n as i32).collect() };
    SubsetSumMtPerS::<i32>::from_multiset(multiset)
}

fn bench_subset_sum_mt_per(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("SubsetSumMtPer");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &(n, target) in &[(10usize, 40i32), (15, 80)] {
        group.bench_with_input(
            BenchmarkId::new(format!("n{}_t{}", n, target), n),
            &(n, target),
            |b, &(n, target)| {
                b.iter_batched(
                    || build_solver(n),
                    |solver| solver.subset_sum(target),
                    BatchSize::SmallInput,
                );
            },
        );
    }
    group.finish();
}

criterion_group!(benches, bench_subset_sum_mt_per);
criterion_main!(benches);
