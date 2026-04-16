// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 52: Edge-set graph (sequential, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap52::EdgeSetGraphStEph::EdgeSetGraphStEph::*;

fn bench_edge_set_graph(c: &mut Criterion) {
    let mut group = c.benchmark_group("EdgeSetGraphStEph");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[16usize, 32] {
        group.bench_with_input(BenchmarkId::new("insert_edge", n), &n, |b, &n| {
            b.iter(|| {
                let mut g = EdgeSetGraphStEph::<i32>::empty();
                for i in 0..n as i32 {
                    g.insert_edge(i, (i + 1) % n as i32);
                }
                g.num_edges()
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_edge_set_graph);
criterion_main!(benches);
