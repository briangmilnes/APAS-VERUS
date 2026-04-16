// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 06: Labeled directed graph (multi-threaded, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap06::LabDirGraphMtEph::LabDirGraphMtEph::*;
use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Types::Types::LabEdge;
use apas_verus::SetLit;
use apas_verus::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::set_parallelism;

fn build_labeled_path(n: usize) -> LabDirGraphMtEph<u64, u64> {
    let mut verts: SetStEph<u64> = SetLit![];
    let mut arcs: SetStEph<LabEdge<u64, u64>> = SetLit![];
    for i in 0..n as u64 { let _ = verts.insert(i); }
    for i in 0..(n as u64 - 1) { let _ = arcs.insert(LabEdge(i, i + 1, i + 1)); }
    <LabDirGraphMtEph<u64, u64> as LabDirGraphMtEphTrait<u64, u64>>::from_vertices_and_labeled_arcs(verts, arcs)
}

fn bench_lab_dir_graph_mt_build(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("LabDirGraphMtEphBuild");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("path_n", n), &n, |b, &n| {
            b.iter(|| build_labeled_path(n));
        });
    }
    group.finish();
}

fn bench_lab_dir_graph_mt_n_plus(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("LabDirGraphMtEphNPlus");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[32usize, 64] {
        let g = build_labeled_path(n);
        let mid = n as u64 / 2;
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, _| {
            b.iter(|| g.n_plus(&mid));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_lab_dir_graph_mt_build, bench_lab_dir_graph_mt_n_plus);
criterion_main!(benches);
