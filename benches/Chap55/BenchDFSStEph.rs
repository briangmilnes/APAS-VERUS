// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 55: DFS (sequential, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;
use apas_verus::Chap55::DFSStEph::DFSStEph::*;

fn make_chain_graph(n: usize) -> ArraySeqStEphS<ArraySeqStEphS<usize>> {
    let adj: Vec<ArraySeqStEphS<usize>> = (0..n)
        .map(|i| {
            if i + 1 < n {
                ArraySeqStEphS::from_vec(vec![i + 1])
            } else {
                ArraySeqStEphS::from_vec(vec![])
            }
        })
        .collect();
    ArraySeqStEphS::from_vec(adj)
}

fn bench_dfs(c: &mut Criterion) {
    let mut group = c.benchmark_group("DFSStEph");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("chain_n", n), &n, |b, &n| {
            let graph = make_chain_graph(n);
            b.iter(|| DFSStEph::dfs(&graph, 0));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_dfs);
criterion_main!(benches);
