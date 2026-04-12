//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 27: Reduce by contraction (parallel) benchmark.

use std::sync::Arc;
use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap27::ReduceContractMtEph::ReduceContractMtEph::*;
use apas_verus::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
use vstd::prelude::Ghost;
use apas_verus::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::set_parallelism;

fn build_seq(n: usize) -> ArraySeqMtEphS<usize> {
    ArraySeqMtEphS::<usize>::from_vec((1..=n).collect())
}

fn bench_reduce_parallel(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("ReduceContractParallel");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[64usize, 256] {
        let a = build_seq(n);
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, _| {
            b.iter(|| {
                <ArraySeqMtEphS<usize> as ReduceContractMtEphTrait<usize>>::reduce_contract_parallel(
                    &a,
                    Arc::new(|x: &usize, y: &usize| x.wrapping_add(*y)),
                    Ghost::assume_new(),
                    0usize,
                )
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_reduce_parallel);
criterion_main!(benches);
