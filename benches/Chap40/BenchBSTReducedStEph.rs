//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 40: Reduced BST (sequential, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap40::BSTReducedStEph::BSTReducedStEph::*;

fn build_sum_tree(n: usize) -> BSTSumStEph<u64, u64> {
    let mut tree = <BSTSumStEph<u64, u64> as BSTReducedStEphTrait<u64, u64, u64, SumOp<u64>>>::new();
    for i in 0..n as u64 {
        tree.insert(i, i, i.wrapping_mul(6364136223846793005));
    }
    tree
}

fn bench_bst_reduced_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("BSTReducedStEphInsert");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| build_sum_tree(n));
        });
    }
    group.finish();
}

fn bench_bst_reduced_range_reduce(c: &mut Criterion) {
    let mut group = c.benchmark_group("BSTReducedStEphRangeReduce");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[32usize, 64] {
        let tree = build_sum_tree(n);
        let lo = n as u64 / 4;
        let hi = 3 * n as u64 / 4;
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, _| {
            b.iter(|| {
                <BSTSumStEph<u64, u64> as BSTReducedStEphTrait<u64, u64, u64, SumOp<u64>>>::range_reduce(&tree, &lo, &hi)
            });
        });
    }
    group.finish();
}

criterion_group!(benches, bench_bst_reduced_insert, bench_bst_reduced_range_reduce);
criterion_main!(benches);
