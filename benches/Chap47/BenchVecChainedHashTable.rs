// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! Chapter 47: Vec-chained hash table benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use vstd::prelude::Ghost;
use apas_verus::Chap47::VecChainedHashTableStEph::VecChainedHashTableStEph::*;
use apas_verus::Chap47::ParaHashTableStEph::ParaHashTableStEph::*;

type HashFn = fn(&u64, usize) -> usize;
type VecTable = HashTable<u64, u64, Vec<(u64, u64)>, (), HashFn>;

fn hash_fn(k: &u64, size: usize) -> usize {
    (*k as usize) % size
}

fn bench_vec_chain_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("VecChainHashInsert");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[32usize, 64] {
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| {
                let mut table: VecTable =
                    <VecChainedHashTableStEph as ParaHashTableStEphTrait<u64, u64, Vec<(u64, u64)>, (), HashFn>>::createTable(
                        hash_fn,
                        n,
                        Ghost::assume_new(),
                    );
                for i in 0..n as u64 {
                    VecChainedHashTableStEph::insert(&mut table, i, i);
                }
                table
            });
        });
    }
    group.finish();
}

fn bench_vec_chain_lookup(c: &mut Criterion) {
    let mut group = c.benchmark_group("VecChainHashLookup");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    for &n in &[32usize, 64] {
        let mut table: VecTable =
            <VecChainedHashTableStEph as ParaHashTableStEphTrait<u64, u64, Vec<(u64, u64)>, (), HashFn>>::createTable(
                hash_fn,
                n,
                Ghost::assume_new(),
            );
        for i in 0..n as u64 {
            VecChainedHashTableStEph::insert(&mut table, i, i);
        }
        group.bench_with_input(BenchmarkId::new("n", n), &n, |b, &n| {
            b.iter(|| VecChainedHashTableStEph::lookup(&table, &(n as u64 / 2)));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_vec_chain_insert, bench_vec_chain_lookup);
criterion_main!(benches);
