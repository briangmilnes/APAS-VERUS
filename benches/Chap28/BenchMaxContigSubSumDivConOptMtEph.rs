// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 28: MCSS divide-and-conquer optimized (multi-threaded) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap28::MaxContigSubSumDivConOptMtEph::MaxContigSubSumDivConOptMtEph::*;
use apas_verus::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
use apas_verus::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::set_parallelism;

fn build_seq(n: usize) -> ArraySeqMtEphS<i32> {
    ArraySeqMtEphS::<i32>::from_vec((0..n as i32).map(|i| i - n as i32 / 2).collect())
}

fn bench_mcss_divcon_opt_mt(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("MCSSdivconOptMt");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[64usize, 256] {
        let a = build_seq(n);
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, _| {
            b.iter(|| <ArraySeqMtEphS<i32> as MaxContigSubSumDivConOptMtTrait>::max_contig_sub_sum_divcon_opt_mt(&a));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_mcss_divcon_opt_mt);
criterion_main!(benches);
