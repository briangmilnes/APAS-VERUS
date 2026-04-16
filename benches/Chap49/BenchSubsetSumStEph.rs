// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 49: Subset sum (memoized recursive DP) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use apas_verus::Chap49::SubsetSumStEph::SubsetSumStEph::*;
use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS;

fn build_solver(n: usize) -> SubsetSumStEphS<i32> {
    let seq: Vec<i32> = (1..=n as i32).collect();
    let multiset = ArraySeqStEphS { seq };
    SubsetSumStEphS::<i32>::from_multiset(multiset)
}

fn bench_subset_sum(c: &mut Criterion) {
    let mut group = c.benchmark_group("SubsetSum");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    // n=10 means values 1..10, target = sum of last 5 — forces full DP.
    for &(n, target) in &[(10usize, 40i32), (15, 80)] {
        group.bench_with_input(
            BenchmarkId::new(format!("n{}_t{}", n, target), n),
            &(n, target),
            |b, &(n, target)| {
                b.iter_batched(
                    || build_solver(n),
                    |mut solver| solver.subset_sum(target),
                    BatchSize::SmallInput,
                );
            },
        );
    }
    group.finish();
}

criterion_group!(benches, bench_subset_sum);
criterion_main!(benches);
