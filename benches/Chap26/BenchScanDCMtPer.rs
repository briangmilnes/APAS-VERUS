//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 26: Divide-and-conquer scan (multi-threaded, persistent) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap26::ScanDCMtPer::ScanDCMtPer::*;
use apas_verus::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
use apas_verus::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::set_parallelism;

fn build_seq(n: usize) -> ArraySeqMtPerS<usize> {
    ArraySeqMtPerS::<usize>::from_vec((0..n).collect())
}

fn bench_prefix_sums_dc_parallel(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("ScanDCPrefixSumsMt");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[64usize, 256] {
        let a = build_seq(n);
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, _| {
            b.iter(|| <ArraySeqMtPerS<usize> as ScanDCMtTrait>::prefix_sums_dc_parallel(&a));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_prefix_sums_dc_parallel);
criterion_main!(benches);
