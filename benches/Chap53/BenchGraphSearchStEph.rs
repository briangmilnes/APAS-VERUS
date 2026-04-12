//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 53: Graph search (sequential, ephemeral) benchmark.

use std::time::Duration;
use vstd::prelude::Ghost;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
use apas_verus::Chap53::GraphSearchStEph::GraphSearchStEph::*;

fn build_chain_graph(n: usize) -> impl Fn(&usize) -> AVLTreeSetStEph<usize> {
    move |v: &usize| {
        if *v + 1 < n {
            AVLTreeSetStEph::singleton(*v + 1)
        } else {
            AVLTreeSetStEph::empty()
        }
    }
}

fn bench_graph_search_reachable(c: &mut Criterion) {
    let mut group = c.benchmark_group("GraphSearchStEphReachable");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[16usize, 64] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            let graph = build_chain_graph(n);
            b.iter(|| reachable(&graph, 0, Ghost::assume_new()));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_graph_search_reachable);
criterion_main!(benches);
