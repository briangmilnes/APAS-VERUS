// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 50: Matrix chain multiplication (multi-threaded, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use apas_verus::Chap50::MatrixChainMtEph::MatrixChainMtEph::*;
use apas_verus::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::set_parallelism;

fn bench_matrix_chain_mt_eph(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("MatrixChainMtEph");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[6usize, 10] {
        group.bench_with_input(BenchmarkId::new("n_matrices", n), &n, |b, &n| {
            b.iter_batched(
                || {
                    let dims: Vec<MatrixDim> = (0..n + 1)
                        .map(|i| MatrixDim { rows: (i + 1) as usize, cols: (i + 2) as usize })
                        .collect();
                    MatrixChainMtEphS::from_dimensions(dims)
                },
                |mut mc| mc.optimal_cost(),
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

criterion_group!(benches, bench_matrix_chain_mt_eph);
criterion_main!(benches);
