// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 06: Labeled undirected graph (sequential, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::*;
use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Types::Types::LabEdge;
use apas_verus::SetLit;

fn build_labeled_ring(n: usize) -> LabUnDirGraphStEph<u64, u64> {
    let mut verts: SetStEph<u64> = SetLit![];
    let mut edges: SetStEph<LabEdge<u64, u64>> = SetLit![];
    for i in 0..n as u64 { let _ = verts.insert(i); }
    for i in 0..n as u64 { let _ = edges.insert(LabEdge(i, (i + 1) % n as u64, i + 1)); }
    <LabUnDirGraphStEph<u64, u64> as LabUnDirGraphStEphTrait<u64, u64>>::from_vertices_and_labeled_edges(verts, edges)
}

fn bench_lab_undir_graph_build(c: &mut Criterion) {
    let mut group = c.benchmark_group("LabUnDirGraphStEphBuild");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[32usize, 128] {
        group.bench_with_input(BenchmarkId::new("ring_n", n), &n, |b, &n| {
            b.iter(|| build_labeled_ring(n));
        });
    }
    group.finish();
}

fn bench_lab_undir_graph_ng(c: &mut Criterion) {
    let mut group = c.benchmark_group("LabUnDirGraphStEphNg");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[32usize, 128] {
        let g = build_labeled_ring(n);
        let mid = n as u64 / 2;
        group.bench_with_input(BenchmarkId::new("ring_n", n), &n, |b, _| {
            b.iter(|| g.ng(&mid));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_lab_undir_graph_build, bench_lab_undir_graph_ng);
criterion_main!(benches);
