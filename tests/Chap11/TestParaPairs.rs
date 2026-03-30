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

#[test]
fn test_para_pair_zero_values() {
    let Pair(a, b) = ParaPair!(move || 0u64, move || 0u64);
    assert_eq!(a, 0);
    assert_eq!(b, 0);
}

#[test]
fn test_para_pair_max_values() {
    let Pair(a, b) = ParaPair!(move || u64::MAX, move || u64::MAX);
    assert_eq!(a, u64::MAX);
    assert_eq!(b, u64::MAX);
}

#[test]
fn test_para_pair_bool_results() {
    let Pair(a, b) = ParaPair!(move || true, move || false);
    assert!(a);
    assert!(!b);
}

#[test]
fn test_para_pair_vec_results() {
    let Pair(v1, v2) = ParaPair!(
        move || vec![1, 2, 3],
        move || vec![4, 5, 6]
    );
    assert_eq!(v1, vec![1, 2, 3]);
    assert_eq!(v2, vec![4, 5, 6]);
}

#[test]
fn test_para_pair_closure_captures() {
    let x = 10;
    let y = 20;
    let Pair(a, b) = ParaPair!(
        move || x * 2,
        move || y * 3
    );
    assert_eq!(a, 20);
    assert_eq!(b, 60);
}

#[test]
fn test_para_pair_heavy_compute() {
    // Both arms do substantial work.
    let Pair(sum1, sum2) = ParaPair!(
        move || (0..1000u64).sum::<u64>(),
        move || (1000..2000u64).sum::<u64>()
    );
    assert_eq!(sum1, 999 * 1000 / 2);
    assert_eq!(sum2, (0..2000u64).sum::<u64>() - sum1);
}

#[test]
fn test_para_pair_option_results() {
    let Pair(a, b) = ParaPair!(
        move || Some(42),
        move || None::<i32>
    );
    assert_eq!(a, Some(42));
    assert_eq!(b, None);
}
