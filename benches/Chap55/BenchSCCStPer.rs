//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 55: Strongly connected components (sequential, persistent) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
use apas_verus::Chap55::SCCStPer::SCCStPer::*;

fn make_scc_graph(n: usize) -> ArraySeqStPerS<ArraySeqStPerS<usize>> {
    let adj: Vec<ArraySeqStPerS<usize>> = (0..n)
        .map(|i| {
            let group_size = 4;
            let next_in_group = if (i + 1) % group_size == 0 {
                i + 1 - group_size
            } else {
                i + 1
            };
            if next_in_group < n {
                ArraySeqStPerS { seq: vec![next_in_group] }
            } else {
                ArraySeqStPerS { seq: vec![] }
            }
        })
        .collect();
    ArraySeqStPerS { seq: adj }
}

fn bench_scc_per(c: &mut Criterion) {
    let mut group = c.benchmark_group("SCCStPer");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            let graph = make_scc_graph(n);
            b.iter(|| SCCStPer::scc(&graph));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_scc_per);
criterion_main!(benches);
