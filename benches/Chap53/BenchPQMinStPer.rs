//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 53: Priority-queue graph search (sequential, persistent) benchmark.

use std::time::Duration;
use vstd::prelude::Ghost;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
use apas_verus::Chap53::PQMinStPer::PQMinStPer::*;

fn build_chain_graph(n: usize) -> impl Fn(&usize) -> AVLTreeSetStPer<usize> {
    move |v: &usize| {
        if *v + 1 < n {
            AVLTreeSetStPer::singleton(*v + 1)
        } else {
            AVLTreeSetStPer::empty()
        }
    }
}

fn bench_pq_min_st_per(c: &mut Criterion) {
    let mut group = c.benchmark_group("PQMinStPer");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[16usize, 64] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            let graph = build_chain_graph(n);
            let prio_fn = |v: &usize| *v;
            b.iter(|| pq_min::<usize, usize, _, _>(&graph, 0, &prio_fn, Ghost::assume_new(), Ghost::assume_new()));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_pq_min_st_per);
criterion_main!(benches);
