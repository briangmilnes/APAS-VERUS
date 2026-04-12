//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 37: BST (plain, no balancing) benchmark — insert and contains.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use apas_verus::Chap37::BSTPlainStEph::BSTPlainStEph::*;

/// Build a BST with n elements inserted in random-ish order (interleaved).
fn build_bst(n: usize) -> BSTPlainStEph<u64> {
    let mut tree = BSTPlainStEph::<u64>::new();
    // Insert in a pattern that avoids a degenerate chain.
    for i in 0..n as u64 {
        // Interleave: 0, n-1, 1, n-2, ...
        let v = if i % 2 == 0 { i / 2 } else { n as u64 - 1 - i / 2 };
        tree = tree.insert(v);
    }
    tree
}

fn bench_bst_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("BSTInsert");
    group.sample_size(10);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(250));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("interleaved", n), &n, |b, &n| {
            b.iter_batched(
                || BSTPlainStEph::<u64>::new(),
                |tree| {
                    let mut t = tree;
                    for i in 0..n as u64 {
                        let v = if i % 2 == 0 { i / 2 } else { n as u64 - 1 - i / 2 };
                        t = t.insert(v);
                    }
                    t
                },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}

fn bench_bst_contains(c: &mut Criterion) {
    let mut group = c.benchmark_group("BSTContains");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[32usize, 64] {
        let tree = build_bst(n);
        let target = n as u64 / 2;
        group.bench_with_input(BenchmarkId::new("hit", n), &n, |b, _| {
            b.iter(|| tree.contains(&target));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_bst_insert, bench_bst_contains);
criterion_main!(benches);
