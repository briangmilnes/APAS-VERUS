// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 23: Balanced binary tree (ephemeral) benchmark — build and traversal.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap23::BalBinTreeStEph::BalBinTreeStEph::*;

/// Build a balanced tree of depth d by repeatedly joining leaves.
fn build_tree(depth: usize) -> BalBinTree<u64> {
    if depth == 0 {
        BalBinTree::<u64>::leaf()
    } else {
        let left = build_tree(depth - 1);
        let right = build_tree(depth - 1);
        BalBinTree::<u64>::node(left, depth as u64, right)
    }
}

fn bench_tree_build(c: &mut Criterion) {
    let mut group = c.benchmark_group("BalBinTreeBuild");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &d in &[6usize, 8] {
        group.bench_with_input(BenchmarkId::new("depth", d), &d, |b, &d| {
            b.iter(|| build_tree(d));
        });
    }
    group.finish();
}

fn bench_tree_in_order(c: &mut Criterion) {
    let mut group = c.benchmark_group("BalBinTreeInOrder");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &d in &[6usize, 8] {
        let tree = build_tree(d);
        group.bench_with_input(BenchmarkId::new("depth", d), &d, |b, _| {
            b.iter(|| tree.in_order());
        });
    }
    group.finish();
}

criterion_group!(benches, bench_tree_build, bench_tree_in_order);
criterion_main!(benches);
