// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 52: Adjacency-sequence graph (multi-threaded, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use apas_verus::Chap52::AdjSeqGraphMtEph::AdjSeqGraphMtEph::*;
use apas_verus::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::set_parallelism;

fn bench_adjseqgraph_mt_eph_build(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("AdjSeqGraphMtEphBuild");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[16usize, 32] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter_batched(
                || <AdjSeqGraphMtEph as AdjSeqGraphMtEphTrait>::new(n),
                |g| g,
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_adjseqgraph_mt_eph_neighbors(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("AdjSeqGraphMtEphNeighbors");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[16usize, 32] {
        let g = <AdjSeqGraphMtEph as AdjSeqGraphMtEphTrait>::new(n);
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, _| {
            b.iter(|| g.out_neighbors(0));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_adjseqgraph_mt_eph_build, bench_adjseqgraph_mt_eph_neighbors);
criterion_main!(benches);
