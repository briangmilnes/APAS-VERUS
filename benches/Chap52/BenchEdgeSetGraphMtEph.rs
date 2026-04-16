// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
use std::time::Duration;

use criterion::*;

use apas_verus::Chap52::EdgeSetGraphMtEph::EdgeSetGraphMtEph::*;

fn build_chain(n: usize) -> EdgeSetGraphMtEph<i32> {
    let mut g = EdgeSetGraphMtEph::<i32>::empty();
    for i in 0..(n as i32) {
        g.insert_edge(i, i + 1);
    }
    g
}

fn bench_insert_edge(c: &mut Criterion) {
    let mut group = c.benchmark_group("EdgeSetGraphMtEph_insert_edge");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));

    for &n in &[16usize, 64, 256] {
        group.bench_with_input(BenchmarkId::new("chain", n), &n, |b, &len| {
            b.iter_batched(
                || EdgeSetGraphMtEph::<i32>::empty(),
                |mut g| {
                    for i in 0..(len as i32) {
                        g.insert_edge(i, i + 1);
                    }
                    g
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_out_neighbors(c: &mut Criterion) {
    let mut group = c.benchmark_group("EdgeSetGraphMtEph_out_neighbors");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));

    for &n in &[16usize, 64, 256] {
        group.bench_with_input(BenchmarkId::new("star", n), &n, |b, &deg| {
            // Build a star graph: vertex 0 connects to 1..=deg.
            let mut g = EdgeSetGraphMtEph::<i32>::empty();
            for i in 1..=(deg as i32) {
                g.insert_edge(0, i);
            }
            b.iter_batched(
                || g.clone(),
                |gr| {
                    let _ = gr.out_neighbors(&0);
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_delete_vertex(c: &mut Criterion) {
    let mut group = c.benchmark_group("EdgeSetGraphMtEph_delete_vertex");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));

    for &n in &[16usize, 64, 256] {
        group.bench_with_input(BenchmarkId::new("chain", n), &n, |b, &len| {
            b.iter_batched(
                || build_chain(len),
                |mut g| {
                    g.delete_vertex(&0);
                    g
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

criterion_group!(benches, bench_insert_edge, bench_out_neighbors, bench_delete_vertex);
criterion_main!(benches);
