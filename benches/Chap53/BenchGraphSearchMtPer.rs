//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 53: Graph search (multi-threaded, persistent) benchmark.

use std::time::Duration;
use vstd::prelude::Ghost;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap41::AVLTreeSetMtPer::AVLTreeSetMtPer::*;
use apas_verus::Chap53::GraphSearchMtPer::GraphSearchMtPer::*;
use apas_verus::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::set_parallelism;

fn build_chain_graph(n: usize) -> impl Fn(&usize) -> AVLTreeSetMtPer<usize> {
    move |v: &usize| {
        if *v + 1 < n {
            AVLTreeSetMtPer::singleton(*v + 1)
        } else {
            AVLTreeSetMtPer::empty()
        }
    }
}

fn bench_graph_search_reachable_mt_per(c: &mut Criterion) {
    set_parallelism(10);
    let mut group = c.benchmark_group("GraphSearchMtPerReachable");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[16usize, 64] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            let graph = build_chain_graph(n);
            b.iter(|| reachable(&graph, 0, Ghost::assume_new()));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_graph_search_reachable_mt_per);
criterion_main!(benches);
