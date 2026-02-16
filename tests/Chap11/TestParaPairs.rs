// Copyright (c) 2025 Brian G. Milnes
//! Tests for ParaPairs module - work-stealing parallelism with global pool.

use apas_verus::Types::Types::Pair;
use apas_verus::ParaPair;
use apas_verus::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::set_parallelism;
use apas_verus::Chap11::FibonacciStEph::FibonacciStEph::fib;

#[test]
fn test_set_parallelism() {
    // Set parallelism before pool initialization.
    set_parallelism(4);
}

#[test]
fn test_para_pair_simple() {
    let Pair(a, b) = ParaPair!(
        move || 1 + 1,
        move || 2 + 2
    );
    assert_eq!(a, 2);
    assert_eq!(b, 4);
}

#[test]
fn test_para_pair_strings() {
    let Pair(a, b) = ParaPair!(
        move || "hello".to_string(),
        move || "world".to_string()
    );
    assert_eq!(a, "hello");
    assert_eq!(b, "world");
}

#[test]
fn test_para_pair_compute() {
    let Pair(a, b) = ParaPair!(
        move || fib(10),
        move || fib(15)
    );
    assert_eq!(a, 55);
    assert_eq!(b, 610);
}

#[test]
fn test_para_pair_different_types() {
    let Pair(num, text) = ParaPair!(
        move || 42_i32,
        move || "answer".to_string()
    );
    assert_eq!(num, 42);
    assert_eq!(text, "answer");
}

#[test]
fn test_para_pair_nested() {
    // Test nested parallel pairs to exercise help-first strategy.
    let Pair(ab, cd) = ParaPair!(
        move || {
            let Pair(a, b) = ParaPair!(move || 1, move || 2);
            a + b
        },
        move || {
            let Pair(c, d) = ParaPair!(move || 3, move || 4);
            c + d
        }
    );
    assert_eq!(ab, 3);
    assert_eq!(cd, 7);
}
