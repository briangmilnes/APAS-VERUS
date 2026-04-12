//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 44: Document index benchmark.

use std::time::Duration;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use apas_verus::Chap44::DocumentIndex::DocumentIndex::*;
use apas_verus::DocumentCollectionLit;

fn bench_make_index(c: &mut Criterion) {
    let mut group = c.benchmark_group("DocIndexMake");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(300));
    for &n in &[4usize, 8] {
        group.bench_with_input(BenchmarkId::new("n_docs", n), &n, |b, &n| {
            b.iter(|| {
                let docs = if n <= 4 {
                    DocumentCollectionLit![
                        "doc1" => "hello world programming rust",
                        "doc2" => "world peace and rust love",
                        "doc3" => "programming is fun with rust",
                        "doc4" => "hello programming world again"
                    ]
                } else {
                    DocumentCollectionLit![
                        "doc1" => "hello world programming rust",
                        "doc2" => "world peace and rust love",
                        "doc3" => "programming is fun with rust",
                        "doc4" => "hello programming world again",
                        "doc5" => "algorithms and data structures",
                        "doc6" => "parallel and sequential computing",
                        "doc7" => "formal verification with verus",
                        "doc8" => "rust systems programming language"
                    ]
                };
                DocumentIndex::make_index(&docs)
            });
        });
    }
    group.finish();
}

fn bench_find(c: &mut Criterion) {
    let mut group = c.benchmark_group("DocIndexFind");
    group.sample_size(20);
    group.warm_up_time(Duration::from_millis(100));
    group.measurement_time(Duration::from_millis(200));
    let docs = DocumentCollectionLit![
        "doc1" => "hello world programming rust",
        "doc2" => "world peace and rust love",
        "doc3" => "programming is fun with rust",
        "doc4" => "hello programming world again"
    ];
    let index = DocumentIndex::make_index(&docs);
    group.bench_function("find_rust", |b| {
        b.iter(|| index.find(&"rust".to_string()));
    });
    group.finish();
}

criterion_group!(benches, bench_make_index, bench_find);
criterion_main!(benches);
