// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 28: Maximum contiguous subsequence sum (reduced) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap28::MaxContigSubSumReducedStEph::MaxContigSubSumReducedStEph::*;
use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;

fn build_seq(n: usize) -> ArraySeqStEphS<i32> {
    ArraySeqStEphS::<i32>::from_vec((0..n as i32).map(|i| i - n as i32 / 2).collect())
}

fn bench_mcss_reduced(c: &mut Criterion) {
    let mut group = c.benchmark_group("MCSSReduced");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[64usize, 256] {
        let a = build_seq(n);
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, _| {
            b.iter(|| <ArraySeqStEphS<i32> as MaxContigSubSumReducedTrait>::max_contig_sub_sum_reduced(&a));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_mcss_reduced);
criterion_main!(benches);
