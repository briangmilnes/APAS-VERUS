//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 54: BFS (multi-threaded, persistent) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
use apas_verus::Chap54::BFSMtPer::BFSMtPer::*;
use apas_verus::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::set_parallelism;

fn make_ring_graph(n: usize) -> ArraySeqMtPerS<ArraySeqMtPerS<usize>> {
    let adj: Vec<ArraySeqMtPerS<usize>> = (0..n)
        .map(|i| ArraySeqMtPerS::from_vec(vec![(i + 1) % n]))
        .collect();
    ArraySeqMtPerS::from_vec(adj)
}

fn bench_bfs_mt_per(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("BFSMtPer");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[32usize, 64] {
        let graph = make_ring_graph(n);
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, _| {
            b.iter(|| BFSMtPer::bfs(&graph, 0));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_bfs_mt_per);
criterion_main!(benches);
