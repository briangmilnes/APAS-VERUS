//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 28: Maximum contiguous subsequence sum (parallel variants) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap28::MaxContigSubSumDivConMtEph::MaxContigSubSumDivConMtEph::*;
use apas_verus::Chap28::MaxContigSubSumOptMtEph::MaxContigSubSumOptMtEph::*;
use apas_verus::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
use apas_verus::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::set_parallelism;

fn build_seq(n: usize) -> ArraySeqMtEphS<i32> {
    ArraySeqMtEphS::<i32>::from_vec((0..n as i32).map(|i| i - n as i32 / 2).collect())
}

fn bench_mcss_divcon_mt(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("MCSSdivconMt");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[64usize, 256] {
        let a = build_seq(n);
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, _| {
            b.iter(|| <ArraySeqMtEphS<i32> as MaxContigSubSumDivConMtTrait>::max_contig_sub_sum_divcon_mt(&a));
        });
    }
    group.finish();
}

fn bench_mcss_opt_mt(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("MCSSOptMt");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[64usize, 256] {
        let a = build_seq(n);
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, _| {
            b.iter(|| <ArraySeqMtEphS<i32> as MaxContigSubSumOptMtTrait>::max_contig_sub_sum_opt_mt(&a));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_mcss_divcon_mt, bench_mcss_opt_mt);
criterion_main!(benches);
