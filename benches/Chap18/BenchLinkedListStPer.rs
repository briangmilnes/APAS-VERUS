//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 18: Linked list (persistent) benchmark — append and nth.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap18::LinkedListStPer::LinkedListStPer::*;

fn build_list(n: usize) -> LinkedListStPerS<u64> {
    LinkedListStPerS::<u64>::from_vec((0..n as u64).collect())
}

fn bench_linked_list_per_append(c: &mut Criterion) {
    let mut group = c.benchmark_group("LinkedListPerAppend");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[64usize, 256] {
        let a = build_list(n);
        let b = build_list(n);
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b_crit, _| {
            b_crit.iter(|| LinkedListStPerS::<u64>::append(&a, &b));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_linked_list_per_append);
criterion_main!(benches);
