// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes

use apas_verus::Chap02::FibonacciHFScheduler::FibonacciHFScheduler::{fib_seq, fib_par};

fn expected_fib(n: u64) -> u64 {
    match n {
        0 => 0, 1 => 1, 2 => 1, 3 => 2, 4 => 3, 5 => 5,
        6 => 8, 7 => 13, 8 => 21, 9 => 34, 10 => 55,
        11 => 89, 12 => 144, 13 => 233, 14 => 377, 15 => 610,
        16 => 987, 17 => 1597, 18 => 2584, 19 => 4181, 20 => 6765,
        25 => 75025, 30 => 832040,
        _ => panic!("not precomputed"),
    }
}

#[test]
fn test_fib_seq() {
    for n in 0..=20 {
        assert_eq!(fib_seq(n), expected_fib(n));
    }
}

#[test]
fn test_fib_par() {
    for n in 0..=20 {
        assert_eq!(fib_par(n), expected_fib(n));
    }
}

#[test]
fn test_fib_par_larger() {
    assert_eq!(fib_par(25), expected_fib(25));
    assert_eq!(fib_par(30), expected_fib(30));
}

#[test]
fn test_fib_seq_base_cases() {
    assert_eq!(fib_seq(0), 0);
    assert_eq!(fib_seq(1), 1);
}

#[test]
fn test_fib_par_base_cases() {
    assert_eq!(fib_par(0), 0);
    assert_eq!(fib_par(1), 1);
}

#[test]
fn test_fib_seq_recurrence() {
    for n in 2..=20 {
        assert_eq!(fib_seq(n), fib_seq(n - 1) + fib_seq(n - 2), "recurrence fails at n={n}");
    }
}

#[test]
fn test_fib_par_recurrence() {
    for n in 2..=20 {
        assert_eq!(fib_par(n), fib_par(n - 1) + fib_par(n - 2), "recurrence fails at n={n}");
    }
}

#[test]
fn test_fib_seq_par_agree() {
    for n in 0..=30 {
        assert_eq!(fib_seq(n), fib_par(n), "seq != par at n={n}");
    }
}

#[test]
fn test_fib_seq_monotonic() {
    for n in 1..=20 {
        assert!(fib_seq(n) >= fib_seq(n - 1), "not monotonic at n={n}");
    }
}

#[test]
fn test_fib_known_large() {
    assert_eq!(fib_seq(46), 1836311903);
    assert_eq!(fib_par(46), 1836311903);
}

#[test]
fn test_fib_seq_n2() {
    assert_eq!(fib_seq(2), 1);
}

#[test]
fn test_fib_par_n2() {
    assert_eq!(fib_par(2), 1);
}

#[test]
fn test_fib_seq_n10() {
    assert_eq!(fib_seq(10), 55);
}

#[test]
fn test_fib_par_n10() {
    assert_eq!(fib_par(10), 55);
}

#[test]
fn test_fib_seq_strictly_increasing_from_n2() {
    // fib(1) = fib(2) = 1, then strictly increasing from n=3.
    for n in 3..=20 {
        assert!(fib_seq(n) > fib_seq(n - 1), "not strictly increasing at n={n}");
    }
}
