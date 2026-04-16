// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 55: Strongly connected components (sequential, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap19::ArraySeqStEph::ArraySeqStEph::*;
use apas_verus::Chap55::SCCStEph::SCCStEph::*;

fn make_scc_graph(n: usize) -> ArraySeqStEphS<ArraySeqStEphS<usize>> {
    // Ring of rings: groups of 4 nodes each forming a cycle.
    let adj: Vec<ArraySeqStEphS<usize>> = (0..n)
        .map(|i| {
            let group_size = 4;
            let next_in_group = if (i + 1) % group_size == 0 {
                i + 1 - group_size
            } else {
                i + 1
            };
            if next_in_group < n {
                ArraySeqStEphS::from_vec(vec![next_in_group])
            } else {
                ArraySeqStEphS::from_vec(vec![])
            }
        })
        .collect();
    ArraySeqStEphS::from_vec(adj)
}

fn bench_scc(c: &mut Criterion) {
    let mut group = c.benchmark_group("SCCStEph");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            let graph = make_scc_graph(n);
            b.iter(|| SCCStEph::scc(&graph));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_scc);
criterion_main!(benches);
