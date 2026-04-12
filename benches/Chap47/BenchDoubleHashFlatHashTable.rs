//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 47: Double hashing flat hash table benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use vstd::prelude::Ghost;
use apas_verus::Chap47::DoubleHashFlatHashTableStEph::DoubleHashFlatHashTableStEph::*;
use apas_verus::Chap47::FlatHashTable::FlatHashTable::*;
use apas_verus::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;

type HashFn = fn(&u64, usize) -> usize;
type DblTable = HashTable<u64, u64, FlatEntry<u64, u64>, (), HashFn>;

fn hash_fn(k: &u64, size: usize) -> usize {
    (*k as usize) % size
}

fn make_table(capacity: usize) -> DblTable {
    let mut table: DblTable =
        <DoubleHashFlatHashTableStEph as ParaHashTableStEphTrait<u64, u64, FlatEntry<u64, u64>, (), HashFn>>::createTable(
            hash_fn,
            capacity,
            Ghost::assume_new(),
        );
    for _ in 0..capacity {
        table.table.push(FlatEntry::Empty);
    }
    table
}

fn bench_dblhash_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("DblHashInsert");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    // Use 2*n+1 capacity (prime-like) to keep load factor low.
    for &n in &[16usize, 32] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| {
                let mut table = make_table(2 * n + 1);
                for i in 0..n as u64 {
                    DoubleHashFlatHashTableStEph::insert(&mut table, i, i);
                }
                table
            });
        });
    }
    group.finish();
}

fn bench_dblhash_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("DblHashLookup");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[16usize, 32] {
        let mut table = make_table(2 * n + 1);
        for i in 0..n as u64 {
            DoubleHashFlatHashTableStEph::insert(&mut table, i, i);
        }
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| DoubleHashFlatHashTableStEph::lookup(&table, &(n as u64 / 2)));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_dblhash_insert, bench_dblhash_lookup);
criterion_main!(benches);
