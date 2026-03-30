// Copyright (c) 2025 Brian G. Milnes
//! Tests for Chap11 Fibonacci implementations.
//!
//! Only FibonacciStEph is testable at runtime. The four Mt variants
//! (MtPerAllThreads, MtPerTSM, MtEph2Threads, MtEphRecomputes) are gated
//! behind `#![cfg(verus_keep_ghost)]` because they use TSM tokens and
//! Tracked types — they exist only under Verus compilation and are
//! verified at proof time, not tested at runtime.

use apas_verus::Chap11::FibonacciStEph::FibonacciStEph::{fib, fib_recursive};

#[test]
fn test_fib_base_cases() {
    assert_eq!(fib(0), 0);
    assert_eq!(fib(1), 1);
}

#[test]
fn test_fib_small() {
    assert_eq!(fib(2), 1);
    assert_eq!(fib(3), 2);
    assert_eq!(fib(4), 3);
    assert_eq!(fib(5), 5);
    assert_eq!(fib(6), 8);
    assert_eq!(fib(7), 13);
    assert_eq!(fib(10), 55);
}

#[test]
fn test_fib_medium() {
    assert_eq!(fib(15), 610);
    assert_eq!(fib(20), 6765);
    assert_eq!(fib(25), 75025);
    assert_eq!(fib(30), 832040);
}

#[test]
fn test_fib_large() {
    assert_eq!(fib(35), 9227465);
    assert_eq!(fib(40), 102334155);
    assert_eq!(fib(45), 1134903170);
    assert_eq!(fib(46), 1836311903);
}

#[test]
fn test_fib_known_values() {
    let known: [(u64, u64); 12] = [
        (0, 0), (1, 1), (2, 1), (5, 5), (10, 55), (15, 610),
        (20, 6765), (25, 75025), (30, 832040), (35, 9227465),
        (40, 102334155), (46, 1836311903),
    ];
    for (n, expected) in known {
        assert_eq!(fib(n), expected, "fib({}) should be {}", n, expected);
    }
}

// Tests for fib_recursive

#[test]
fn test_fib_recursive_base_cases() {
    assert_eq!(fib_recursive(0), 0);
    assert_eq!(fib_recursive(1), 1);
}

#[test]
fn test_fib_recursive_small() {
    assert_eq!(fib_recursive(2), 1);
    assert_eq!(fib_recursive(3), 2);
    assert_eq!(fib_recursive(4), 3);
    assert_eq!(fib_recursive(5), 5);
    assert_eq!(fib_recursive(6), 8);
    assert_eq!(fib_recursive(7), 13);
    assert_eq!(fib_recursive(10), 55);
}

#[test]
fn test_fib_recursive_medium() {
    assert_eq!(fib_recursive(15), 610);
    assert_eq!(fib_recursive(20), 6765);
}

#[test]
fn test_fib_recursive_agrees_with_iterative() {
    for n in 0..=25 {
        assert_eq!(fib(n), fib_recursive(n), "fib({n}) mismatch: iterative vs recursive");
    }
}

#[test]
fn test_fib_consecutive_sum_property() {
    // fib(n) = fib(n-1) + fib(n-2) for n >= 2
    for n in 2..=30u64 {
        assert_eq!(fib(n), fib(n - 1) + fib(n - 2), "Fibonacci recurrence fails at n={n}");
    }
}

#[test]
fn test_fib_monotonically_increasing() {
    let mut prev = 0u64;
    for n in 1..=46 {
        let curr = fib(n);
        assert!(curr >= prev, "fib({n}) = {curr} < fib({}) = {prev}", n - 1);
        prev = curr;
    }
}
