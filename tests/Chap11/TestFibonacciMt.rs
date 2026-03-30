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

#[test]
fn test_fib_even_indices_divisible_by_fib2() {
    // fib(2) = 1, fib(4) = 3, fib(6) = 8, fib(8) = 21, fib(10) = 55
    // Every third Fibonacci number is even.
    for n in (3..=30).step_by(3) {
        assert_eq!(fib(n) % 2, 0, "fib({n}) should be even");
    }
}

#[test]
fn test_fib_sum_of_first_n() {
    // sum(fib(0)..fib(n)) = fib(n+2) - 1
    for n in 0..=25u64 {
        let sum: u64 = (0..=n).map(|i| fib(i)).sum();
        assert_eq!(sum, fib(n + 2) - 1, "Sum formula fails at n={n}");
    }
}

#[test]
fn test_fib_gcd_property() {
    // gcd(fib(m), fib(n)) = fib(gcd(m, n))
    fn gcd(a: u64, b: u64) -> u64 { if b == 0 { a } else { gcd(b, a % b) } }
    for m in 1..=15u64 {
        for n in 1..=15u64 {
            assert_eq!(
                gcd(fib(m), fib(n)),
                fib(gcd(m, n)),
                "GCD property fails for m={m}, n={n}"
            );
        }
    }
}

#[test]
fn test_fib_cassini_identity() {
    // fib(n-1)*fib(n+1) - fib(n)^2 = (-1)^n for n >= 1
    for n in 1..=25u64 {
        let product = fib(n - 1) as i64 * fib(n + 1) as i64 - (fib(n) as i64).pow(2);
        let expected = if n % 2 == 0 { 1i64 } else { -1i64 };
        assert_eq!(product, expected, "Cassini identity fails at n={n}");
    }
}

#[test]
fn test_fib_recursive_agrees_larger() {
    // Check agreement for a few more values beyond 25.
    for n in [26, 27, 28, 29, 30] {
        assert_eq!(fib(n), fib_recursive(n), "Mismatch at n={n}");
    }
}

#[test]
fn test_fib_boundary_values() {
    // Test specific boundary values.
    assert_eq!(fib(46), 1836311903);
    assert_eq!(fib(47), 2971215073);
    assert_eq!(fib(50), 12586269025);
}

#[test]
fn test_fib_last_digit_pisano_period() {
    // Pisano period for mod 10 is 60.
    // fib(60) mod 10 == fib(0) mod 10 == 0.
    assert_eq!(fib(60) % 10, 0);
    // fib(61) mod 10 == fib(1) mod 10 == 1.
    assert_eq!(fib(61) % 10, 1);
}

#[test]
fn test_fib_ratio_approaches_golden() {
    // For large n, fib(n)/fib(n-1) approaches the golden ratio (1.618...).
    let ratio = fib(40) as f64 / fib(39) as f64;
    let golden = (1.0 + 5.0f64.sqrt()) / 2.0;
    assert!((ratio - golden).abs() < 1e-8, "Ratio {ratio} not close to golden ratio");
}

#[test]
fn test_fib_recursive_base_case_zero() {
    assert_eq!(fib_recursive(0), 0);
}

#[test]
fn test_fib_squares_sum() {
    // fib(0)^2 + fib(1)^2 + ... + fib(n)^2 = fib(n)*fib(n+1)
    for n in 0..=20u64 {
        let sum_squares: u64 = (0..=n).map(|i| fib(i) * fib(i)).sum();
        assert_eq!(sum_squares, fib(n) * fib(n + 1), "Squares sum fails at n={n}");
    }
}

#[test]
fn test_fib_adjacent_coprime() {
    // Consecutive Fibonacci numbers are coprime.
    fn gcd(a: u64, b: u64) -> u64 { if b == 0 { a } else { gcd(b, a % b) } }
    for n in 1..=30u64 {
        assert_eq!(gcd(fib(n), fib(n + 1)), 1, "fib({n}) and fib({}) not coprime", n + 1);
    }
}

#[test]
fn test_fib_divisibility_property() {
    // If m divides n, then fib(m) divides fib(n).
    for m in 2..=10u64 {
        for k in 1..=5u64 {
            let n = m * k;
            if n <= 46 {
                assert_eq!(fib(n) % fib(m), 0, "fib({n}) not divisible by fib({m})");
            }
        }
    }
}
