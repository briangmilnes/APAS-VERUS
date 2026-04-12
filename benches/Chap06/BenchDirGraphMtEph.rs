//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 06: Directed graph (multi-threaded, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap06::DirGraphMtEph::DirGraphMtEph::*;
use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Types::Types::Edge;
use apas_verus::SetLit;
use apas_verus::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::set_parallelism;

fn build_path_graph(n: usize) -> DirGraphMtEph<u64> {
    let mut verts: SetStEph<u64> = SetLit![];
    let mut arcs: SetStEph<Edge<u64>> = SetLit![];
    for i in 0..n as u64 { let _ = verts.insert(i); }
    for i in 0..(n as u64 - 1) { let _ = arcs.insert(Edge(i, i + 1)); }
    <DirGraphMtEph<u64> as DirGraphMtEphTrait<u64>>::from_sets(verts, arcs)
}

fn bench_dir_graph_mt_build(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("DirGraphMtEphBuild");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("path_n", n), &n, |b, &n| {
            b.iter(|| build_path_graph(n));
        });
    }
    group.finish();
}

fn bench_dir_graph_mt_neighbors(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("DirGraphMtEphNeighbors");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[32usize, 64] {
        let g = build_path_graph(n);
        let mid = n as u64 / 2;
        group.bench_with_input(BenchmarkId::new("n_plus", n), &n, |b, _| {
            b.iter(|| g.n_plus(&mid));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_dir_graph_mt_build, bench_dir_graph_mt_neighbors);
criterion_main!(benches);
