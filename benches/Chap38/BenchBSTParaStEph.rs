//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 38: Parallel BST (sequential, ephemeral) benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap38::BSTParaStEph::BSTParaStEph::*;

fn bench_bstpara_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("BSTParaInsert");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| {
                let mut tree = <ParamBST<u64> as ParamBSTTrait<u64>>::new();
                for i in 0..n as u64 { tree.insert(i); }
                tree
            });
        });
    }
    group.finish();
}

fn bench_bstpara_find(c: &mut Criterion) {
    let mut group = c.benchmark_group("BSTParaFind");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[32usize, 64] {
        let mut tree = <ParamBST<u64> as ParamBSTTrait<u64>>::new();
        for i in 0..n as u64 { tree.insert(i); }
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| tree.find(&(n as u64 / 2)));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_bstpara_insert, bench_bstpara_find);
criterion_main!(benches);
