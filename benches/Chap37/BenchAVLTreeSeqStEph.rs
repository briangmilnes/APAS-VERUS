//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 37: AVL tree sequence (sequential, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::*;

fn build_tree(n: usize) -> AVLTreeSeqStEphS<u64> {
    let mut tree = AVLTreeSeqStEphS::<u64>::new_root();
    for i in 0..n as u64 { tree.insert_value(i); }
    tree
}

fn bench_avltreeseq_build(c: &mut Criterion) {
    let mut group = c.benchmark_group("AVLTreeSeqBuild");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| build_tree(n));
        });
    }
    group.finish();
}

fn bench_avltreeseq_contains(c: &mut Criterion) {
    let mut group = c.benchmark_group("AVLTreeSeqContains");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[32usize, 64] {
        let tree = build_tree(n);
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| tree.contains_value(&(n as u64 / 2)));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_avltreeseq_build, bench_avltreeseq_contains);
criterion_main!(benches);
