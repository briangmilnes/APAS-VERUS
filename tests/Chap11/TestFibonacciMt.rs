// Copyright (c) 2025 Brian G. Milnes
//! Tests for Chap11 Fibonacci implementations.
//!
//! Only FibonacciStEph is testable at runtime. The four Mt variants
//! (MtPerAllThreads, MtPerTSM, MtEph2Threads, MtEphRecomputes) are gated
//! behind `#![cfg(verus_keep_ghost)]` because they use TSM tokens and
//! Tracked types — they exist only under Verus compilation and are
//! verified at proof time, not tested at runtime.

use apas_verus::Chap11::FibonacciStEph::FibonacciStEph::fib;

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
