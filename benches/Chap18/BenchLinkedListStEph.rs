//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 18: Linked list (ephemeral) benchmark — append and nth.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap18::LinkedListStEph::LinkedListStEph::*;

fn build_list(n: usize) -> LinkedListStEphS<u64> {
    LinkedListStEphS::<u64>::from_vec((0..n as u64).collect())
}

fn bench_linked_list_append(c: &mut Criterion) {
    let mut group = c.benchmark_group("LinkedListAppend");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[64usize, 256] {
        let a = build_list(n);
        let b = build_list(n);
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b_crit, _| {
            b_crit.iter(|| LinkedListStEphS::<u64>::append(&a, &b));
        });
    }
    group.finish();
}

fn bench_linked_list_nth(c: &mut Criterion) {
    let mut group = c.benchmark_group("LinkedListNth");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[64usize, 256] {
        let list = build_list(n);
        group.bench_with_input(BenchmarkId::new("last", n), &n, |b, _| {
            b.iter(|| list.nth(n - 1));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_linked_list_append, bench_linked_list_nth);
criterion_main!(benches);
