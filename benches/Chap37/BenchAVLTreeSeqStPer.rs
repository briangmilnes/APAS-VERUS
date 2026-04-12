//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 37: AVL tree sequence (sequential, persistent) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;

fn build_tree(n: usize) -> AVLTreeSeqStPerS<u64> {
    let values: Vec<u64> = (0..n as u64).collect();
    AVLTreeSeqStPerS::<u64>::from_vec(values)
}

fn bench_avltreeseq_stper_build(c: &mut Criterion) {
    let mut group = c.benchmark_group("AVLTreeSeqStPerBuild");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| build_tree(n));
        });
    }
    group.finish();
}

fn bench_avltreeseq_stper_nth(c: &mut Criterion) {
    let mut group = c.benchmark_group("AVLTreeSeqStPerNth");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[32usize, 64] {
        let tree = build_tree(n);
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| tree.nth(n / 2));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_avltreeseq_stper_build, bench_avltreeseq_stper_nth);
criterion_main!(benches);
