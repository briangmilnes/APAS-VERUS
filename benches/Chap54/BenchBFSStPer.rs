// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 54: BFS (sequential, persistent) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_verus::Chap54::BFSStPer::BFSStPer::*;

fn make_ring_graph(n: usize) -> ArraySeqStPerS<ArraySeqStPerS<usize>> {
    let adj: Vec<ArraySeqStPerS<usize>> = (0..n)
        .map(|i| ArraySeqStPerS { seq: vec![(i + 1) % n] })
        .collect();
    ArraySeqStPerS { seq: adj }
}

fn bench_bfs_per(c: &mut Criterion) {
    let mut group = c.benchmark_group("BFSStPer");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[64usize, 128] {
        group.bench_with_input(BenchmarkId::new("ring_n", n), &n, |b, &n| {
            let graph = make_ring_graph(n);
            b.iter(|| BFSStPer::bfs(&graph, 0));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_bfs_per);
criterion_main!(benches);
