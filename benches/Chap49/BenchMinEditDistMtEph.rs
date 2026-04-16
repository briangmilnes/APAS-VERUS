// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 49: Minimum edit distance (multi-threaded, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use apas_verus::Chap49::MinEditDistMtEph::MinEditDistMtEph::*;
use apas_verus::Chap19::ArraySeqMtEph::ArraySeqMtEph::{ArraySeqMtEphS, ArraySeqMtEphTrait};
use apas_verus::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::set_parallelism;

fn build_solver(src_len: usize, tgt_len: usize) -> MinEditDistMtEphS<u8> {
    let source = ArraySeqMtEphS::<u8>::from_vec((0..src_len as u8).collect());
    let target = ArraySeqMtEphS::<u8>::from_vec((10..10 + tgt_len as u8).collect());
    MinEditDistMtEphS::<u8>::from_sequences(source, target)
}

fn bench_min_edit_dist_mt_eph(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("MinEditDistMtEph");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[20usize, 40] {
        group.bench_with_input(BenchmarkId::new("n_x_n", n), &n, |b, &n| {
            b.iter_batched(
                || build_solver(n, n),
                |mut solver| solver.min_edit_distance(),
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

criterion_group!(benches, bench_min_edit_dist_mt_eph);
criterion_main!(benches);
