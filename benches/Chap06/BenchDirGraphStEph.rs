// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 06: Directed graph (ephemeral, hash-backed) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap06::DirGraphStEph::DirGraphStEph::*;
use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Types::Types::Edge;
use apas_verus::SetLit;

/// Build a directed path graph 0→1→2→…→n-1.
fn build_path_graph(n: usize) -> DirGraphStEph<u64> {
    let mut verts: SetStEph<u64> = SetLit![];
    let mut arcs: SetStEph<Edge<u64>> = SetLit![];
    for i in 0..n as u64 { verts.insert(i); }
    for i in 0..(n as u64 - 1) { arcs.insert(Edge(i, i + 1)); }
    DirGraphStEph::<u64>::from_sets(verts, arcs)
}

fn bench_dir_graph_build(c: &mut Criterion) {
    let mut group = c.benchmark_group("DirGraphBuild");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[32usize, 128] {
        group.bench_with_input(BenchmarkId::new("path_n", n), &n, |b, &n| {
            b.iter(|| build_path_graph(n));
        });
    }
    group.finish();
}

fn bench_dir_graph_neighbors(c: &mut Criterion) {
    let mut group = c.benchmark_group("DirGraphNeighbors");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[32usize, 128] {
        let g = build_path_graph(n);
        let mid = n as u64 / 2;
        group.bench_with_input(BenchmarkId::new("n_plus", n), &n, |b, _| {
            b.iter(|| g.n_plus(&mid));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_dir_graph_build, bench_dir_graph_neighbors);
criterion_main!(benches);
