//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 35: Order-statistic selection (multi-threaded, persistent) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use apas_verus::Chap35::OrderStatSelectMtPer::OrderStatSelectMtPer::*;
use apas_verus::Chap18::ArraySeqMtPer::ArraySeqMtPer::ArraySeqMtPerS;
use apas_verus::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::set_parallelism;

fn build_array(n: usize) -> ArraySeqMtPerS<u64> {
    ArraySeqMtPerS { seq: (0..n as u64).rev().collect() }
}

fn bench_select_mt_per(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("OrderStatSelectMtPer");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[64usize, 256] {
        let k = n / 2;
        group.bench_with_input(BenchmarkId::new("median", n), &n, |b, &n| {
            b.iter_batched(
                || build_array(n),
                |a| <ArraySeqMtPerS<u64> as OrderStatSelectMtPerTrait<u64>>::select(&a, k),
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

criterion_group!(benches, bench_select_mt_per);
criterion_main!(benches);
