// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 26: Divide-and-conquer scan (sequential, persistent) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap26::ScanDCStPer::ScanDCStPer::*;
use apas_verus::Chap18::ArraySeqStPer::ArraySeqStPer::*;

fn build_seq(n: usize) -> ArraySeqStPerS<usize> {
    ArraySeqStPerS::<usize>::from_vec((0..n).collect())
}

fn bench_prefix_sums_dc(c: &mut Criterion) {
    let mut group = c.benchmark_group("ScanDCPrefixSums");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[64usize, 256] {
        let a = build_seq(n);
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, _| {
            b.iter(|| <ArraySeqStPerS<usize> as ScanDCStTrait>::prefix_sums_dc(&a));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_prefix_sums_dc);
criterion_main!(benches);
