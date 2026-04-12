//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 66: Borůvka MST (multi-threaded, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap05::SetStEph::SetStEph::*;
use apas_verus::Chap66::BoruvkaMtEph::BoruvkaMtEph::*;
use apas_verus::SetLit;
use apas_verus::vstdplus::float::float::*;

fn w(v: f64) -> WrappedF64 { WrappedF64 { val: v } }

fn make_chain_graph(n: usize) -> (SetStEph<usize>, SetStEph<LabeledEdge<usize>>) {
    let mut vertices: SetStEph<usize> = SetLit![];
    for i in 0..n { let _ = vertices.insert(i); }
    let mut edges: SetStEph<LabeledEdge<usize>> = SetLit![];
    for i in 0..n - 1 {
        let _ = edges.insert(LabeledEdge(i, i + 1, w((i + 1) as f64), i));
    }
    (vertices, edges)
}

fn bench_boruvka_mt(c: &mut Criterion) {
    let mut group = c.benchmark_group("BoruvkaMtEph");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[16usize, 32] {
        let (vertices, edges) = make_chain_graph(n);
        group.bench_with_input(BenchmarkId::new("chain_n", n), &n, |b, _| {
            b.iter(|| boruvka_mst_mt_with_seed(&vertices, &edges, 42));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_boruvka_mt);
criterion_main!(benches);
