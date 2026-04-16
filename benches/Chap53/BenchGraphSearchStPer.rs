// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 53: Graph search (sequential, persistent) benchmark.

use std::time::Duration;
use vstd::prelude::Ghost;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
use apas_verus::Chap53::GraphSearchStPer::GraphSearchStPer::*;

fn build_chain_graph(n: usize) -> impl Fn(&usize) -> AVLTreeSetStPer<usize> {
    move |v: &usize| {
        if *v + 1 < n {
            AVLTreeSetStPer::singleton(*v + 1)
        } else {
            AVLTreeSetStPer::empty()
        }
    }
}

fn bench_graph_search_reachable_st_per(c: &mut Criterion) {
    let mut group = c.benchmark_group("GraphSearchStPerReachable");
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

criterion_group!(benches, bench_graph_search_reachable_st_per);
criterion_main!(benches);
