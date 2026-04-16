// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 49: Subset sum (sequential, persistent) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap49::SubsetSumStPer::SubsetSumStPer::*;
use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS;

fn bench_subset_sum_per(c: &mut Criterion) {
    let mut group = c.benchmark_group("SubsetSumStPer");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &(n, target) in &[(10usize, 40i32), (15, 80)] {
        group.bench_with_input(
            BenchmarkId::new(format!("n{}_t{}", n, target), n),
            &(n, target),
            |b, &(n, target)| {
                let seq: Vec<i32> = (1..=n as i32).collect();
                let multiset = ArraySeqStPerS { seq };
                let solver = SubsetSumStPerS::<i32>::from_multiset(multiset);
                b.iter(|| solver.subset_sum(target));
            },
        );
    }
    group.finish();
}

criterion_group!(benches, bench_subset_sum_per);
criterion_main!(benches);
