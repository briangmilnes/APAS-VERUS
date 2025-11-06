//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
use std::time::Duration;

use criterion::*;

use apas_verus::Chap03::InsertionSortStEph::InsertionSortStEph::*;

fn build_vec(len: usize) -> Vec<u64> {
    (0..len as u64).rev().collect()
}

fn bench_insertion_sort(c: &mut Criterion) {
    let mut group = c.benchmark_group("InsertionSortSt");
    group.sample_size(30);
    group.warm_up_time(Duration::from_millis(300));
    group.measurement_time(Duration::from_secs(1));

    for &n in &[32usize, 64, 128] {
        group.bench_with_input(BenchmarkId::new("reverse", n), &n, |b, &len| {
            b.iter_batched(
                || build_vec(len),
                |mut data| {
                    insertion_sort(&mut data);
                },
                BatchSize::SmallInput,
            );
        });
    }

    group.finish();
}

criterion_group!(benches, bench_insertion_sort);
criterion_main!(benches);

