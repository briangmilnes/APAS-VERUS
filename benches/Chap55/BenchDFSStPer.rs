//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 55: DFS (sequential, persistent) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_verus::Chap55::DFSStPer::DFSStPer::*;

fn make_chain_graph(n: usize) -> ArraySeqStPerS<ArraySeqStPerS<usize>> {
    let adj: Vec<ArraySeqStPerS<usize>> = (0..n)
        .map(|i| {
            if i + 1 < n {
                ArraySeqStPerS { seq: vec![i + 1] }
            } else {
                ArraySeqStPerS { seq: vec![] }
            }
        })
        .collect();
    ArraySeqStPerS { seq: adj }
}

fn bench_dfs_per(c: &mut Criterion) {
    let mut group = c.benchmark_group("DFSStPer");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("chain_n", n), &n, |b, &n| {
            let graph = make_chain_graph(n);
            b.iter(|| DFSStPer::dfs(&graph, 0));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_dfs_per);
criterion_main!(benches);
