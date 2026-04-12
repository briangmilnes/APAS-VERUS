//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 52: Edge-set graph (sequential, persistent) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap52::EdgeSetGraphStPer::EdgeSetGraphStPer::*;

fn bench_edgesetgraph_stper_build(c: &mut Criterion) {
    let mut group = c.benchmark_group("EdgeSetGraphStPerBuild");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(400));
    for &n in &[16usize, 32] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| {
                let mut g = <EdgeSetGraphStPer<i32> as EdgeSetGraphStPerTrait<i32>>::empty();
                for i in 0..n as i32 { g = g.insert_edge(i, (i + 1) % n as i32); }
                g
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_edgesetgraph_stper_build);
criterion_main!(benches);
