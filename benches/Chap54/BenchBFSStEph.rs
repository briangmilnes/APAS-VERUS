// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 54: BFS (sequential, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;
use apas_verus::Chap54::BFSStEph::BFSStEph::*;

fn make_ring_graph(n: usize) -> ArraySeqStEphS<ArraySeqStEphS<usize>> {
    let adj: Vec<ArraySeqStEphS<usize>> = (0..n)
        .map(|i| ArraySeqStEphS::from_vec(vec![(i + 1) % n]))
        .collect();
    ArraySeqStEphS::from_vec(adj)
}

fn bench_bfs(c: &mut Criterion) {
    let mut group = c.benchmark_group("BFSStEph");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[64usize, 128] {
        group.bench_with_input(BenchmarkId::new("ring_n", n), &n, |b, &n| {
            let graph = make_ring_graph(n);
            b.iter(|| BFSStEph::bfs(&graph, 0));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_bfs);
criterion_main!(benches);
