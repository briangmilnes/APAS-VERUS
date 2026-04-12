//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 54: BFS (multi-threaded, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
use apas_verus::Chap54::BFSMtEph::BFSMtEph::*;
use apas_verus::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::set_parallelism;

fn make_ring_graph(n: usize) -> ArraySeqMtEphS<ArraySeqMtEphS<usize>> {
    let adj: Vec<ArraySeqMtEphS<usize>> = (0..n)
        .map(|i| ArraySeqMtEphS { seq: vec![(i + 1) % n] })
        .collect();
    ArraySeqMtEphS { seq: adj }
}

fn bench_bfs_mt(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("BFSMtEph");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[64usize, 128] {
        group.bench_with_input(BenchmarkId::new("ring_n", n), &n, |b, &n| {
            let graph = make_ring_graph(n);
            b.iter(|| BFSMtEph::bfs(&graph, 0));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_bfs_mt);
criterion_main!(benches);
