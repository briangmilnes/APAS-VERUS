//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 06: Labeled undirected graph (multi-threaded, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap06::LabUnDirGraphMtEph::LabUnDirGraphMtEph::*;
use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Types::Types::LabEdge;
use apas_verus::SetLit;
use apas_verus::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::set_parallelism;

fn build_labeled_ring(n: usize) -> LabUnDirGraphMtEph<u64, u64> {
    let mut verts: SetStEph<u64> = SetLit![];
    let mut edges: SetStEph<LabEdge<u64, u64>> = SetLit![];
    for i in 0..n as u64 { let _ = verts.insert(i); }
    for i in 0..n as u64 { let _ = edges.insert(LabEdge(i, (i + 1) % n as u64, i + 1)); }
    <LabUnDirGraphMtEph<u64, u64> as LabUnDirGraphMtEphTrait<u64, u64>>::from_vertices_and_labeled_edges(verts, edges)
}

fn bench_lab_undir_graph_mt_build(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("LabUnDirGraphMtEphBuild");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("ring_n", n), &n, |b, &n| {
            b.iter(|| build_labeled_ring(n));
        });
    }
    group.finish();
}

fn bench_lab_undir_graph_mt_ng(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("LabUnDirGraphMtEphNg");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[32usize, 64] {
        let g = build_labeled_ring(n);
        let mid = n as u64 / 2;
        group.bench_with_input(BenchmarkId::new("ring_n", n), &n, |b, _| {
            b.iter(|| g.ng(&mid));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_lab_undir_graph_mt_build, bench_lab_undir_graph_mt_ng);
criterion_main!(benches);
