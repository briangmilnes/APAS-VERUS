//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Runtime tests for Chapter 21 — Comprehensions and tabulate+flatten algorithms.

use apas_verus::Chap18::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerBaseTrait;
use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerTrait;
use apas_verus::Chap21::Algorithm21_1::Algorithm21_1::*;
use apas_verus::Chap21::Algorithm21_2::Algorithm21_2::*;
use apas_verus::Chap21::Algorithm21_5::Algorithm21_5::*;
use apas_verus::Chap21::Algorithm21_6::Algorithm21_6::*;
use apas_verus::Chap21::Exercise21_5::Exercise21_5::*;
use apas_verus::Chap21::Exercise21_7::Exercise21_7::*;
use apas_verus::Chap21::Exercise21_8::Exercise21_8::*;
use apas_verus::Chap21::Problem21_1::Problem21_1::*;
use apas_verus::Chap21::Problem21_4::Problem21_4::*;
use apas_verus::Types::Types::*;

// Algorithm 21.1: 2D Points (tabulate + flatten)

#[test]
fn points2d_tab_flat_n0() {
    let result = points2d_tab_flat(0);
    assert_eq!(result.length(), 0);
}

#[test]
fn points2d_tab_flat_n1() {
    let result = points2d_tab_flat(1);
    assert_eq!(result.length(), 0);
}

#[test]
fn points2d_tab_flat_n3() {
    let result = points2d_tab_flat(3);
    assert_eq!(result.length(), 6); // 3 * 2 = 6
}

#[test]
fn points2d_tab_flat_n5() {
    let result = points2d_tab_flat(5);
    assert_eq!(result.length(), 20); // 5 * 4 = 20
}

// Algorithm 21.2: 3D Points (nested tabulate + flatten)

#[test]
fn points3d_tab_flat_n0() {
    let result = points3d_tab_flat(0);
    assert_eq!(result.length(), 0);
}

#[test]
fn points3d_tab_flat_n1() {
    let result = points3d_tab_flat(1);
    assert_eq!(result.length(), 1); // 1^3 = 1
}

#[test]
fn points3d_tab_flat_n3() {
    let result = points3d_tab_flat(3);
    assert_eq!(result.length(), 27); // 3^3 = 27
}

// Algorithm 21.5: Brute Force Primes

#[test]
fn primes_bf_n0() {
    let result = primes_bf(0);
    assert_eq!(result.length(), 0);
}

#[test]
fn primes_bf_n2() {
    let result = primes_bf(2);
    assert_eq!(result.length(), 0);
}

#[test]
fn primes_bf_n10() {
    let result = primes_bf(10);
    let primes: Vec<usize> = (0..result.length()).map(|i| *result.nth(i)).collect();
    assert_eq!(primes, vec![2, 3, 5, 7]);
}

#[test]
fn primes_bf_n20() {
    let result = primes_bf(20);
    let primes: Vec<usize> = (0..result.length()).map(|i| *result.nth(i)).collect();
    assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19]);
}

// Algorithm 21.6: Prime Sieve

#[test]
fn prime_sieve_n0() {
    let result = prime_sieve(0);
    assert_eq!(result.length(), 0);
}

#[test]
fn prime_sieve_n2() {
    let result = prime_sieve(2);
    assert_eq!(result.length(), 0);
}

#[test]
fn prime_sieve_n10() {
    let result = prime_sieve(10);
    let sieved: Vec<usize> = (0..result.length()).map(|i| *result.nth(i)).collect();
    assert_eq!(sieved, vec![2, 3, 5, 7]);
}

#[test]
fn prime_sieve_n20() {
    let result = prime_sieve(20);
    let sieved: Vec<usize> = (0..result.length()).map(|i| *result.nth(i)).collect();
    assert_eq!(sieved, vec![2, 3, 5, 7, 11, 13, 17, 19]);
}

#[test]
fn prime_sieve_matches_primes_bf() {
    // prime_sieve finds primes in 2..=n; primes_bf finds primes in 2..n.
    // Compare using n+1 for bf to align ranges.
    for n in [10, 30, 100] {
        let bf = primes_bf(n + 1);
        let sieve = prime_sieve(n);
        let bf_vec: Vec<usize> = (0..bf.length()).map(|i| *bf.nth(i)).collect();
        let sieve_vec: Vec<usize> = (0..sieve.length()).map(|i| *sieve.nth(i)).collect();
        assert_eq!(bf_vec, sieve_vec, "mismatch at n={n}");
    }
}

// Exercise 21.5: All Contiguous Subsequences

#[test]
fn all_contiguous_subseqs_empty() {
    use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    let a = ArraySeqStPerS::<N>::from_vec(Vec::new());
    let result = all_contiguous_subseqs(&a);
    assert_eq!(result.length(), 0);
}

#[test]
fn all_contiguous_subseqs_small() {
    use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    let a = ArraySeqStPerS::from_vec(vec![10, 20, 30]);
    let result = all_contiguous_subseqs(&a);
    // n=3: 3 + 2 + 1 = 6 subsequences
    assert_eq!(result.length(), 6);
}

// Exercise 21.7: Pair Even with Vowels

#[test]
fn is_even_tests() {
    assert!(is_even(&0));
    assert!(!is_even(&1));
    assert!(is_even(&2));
    assert!(!is_even(&3));
    assert!(is_even(&100));
}

#[test]
fn is_vowel_tests() {
    assert!(is_vowel(&'a'));
    assert!(is_vowel(&'e'));
    assert!(is_vowel(&'i'));
    assert!(is_vowel(&'o'));
    assert!(is_vowel(&'u'));
    assert!(is_vowel(&'A'));
    assert!(!is_vowel(&'b'));
    assert!(!is_vowel(&'z'));
}

#[test]
fn pair_even_with_vowels_basic() {
    use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    let a = ArraySeqStPerS::from_vec(vec![1, 2, 3, 4]);
    let b = ArraySeqStPerS::from_vec(vec!['a', 'b', 'e']);
    let result = pair_even_with_vowels(&a, &b);
    // Even elements: [2, 4]. Vowels: ['a', 'e']. Pairs: 2*2 = 4.
    assert_eq!(result.length(), 4);
}

#[test]
fn pair_even_with_vowels_no_evens() {
    use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    let a = ArraySeqStPerS::from_vec(vec![1, 3, 5]);
    let b = ArraySeqStPerS::from_vec(vec!['a', 'e']);
    let result = pair_even_with_vowels(&a, &b);
    assert_eq!(result.length(), 0);
}

#[test]
fn pair_even_with_vowels_no_vowels() {
    use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    let a = ArraySeqStPerS::from_vec(vec![2, 4]);
    let b = ArraySeqStPerS::from_vec(vec!['b', 'c', 'd']);
    let result = pair_even_with_vowels(&a, &b);
    assert_eq!(result.length(), 0);
}

// Exercise 21.8: isPrime

#[test]
fn is_prime_known_primes() {
    assert!(is_prime(2));
    assert!(is_prime(3));
    assert!(is_prime(5));
    assert!(is_prime(7));
    assert!(is_prime(11));
    assert!(is_prime(13));
    assert!(is_prime(97));
}

#[test]
fn is_prime_known_composites() {
    assert!(!is_prime(0));
    assert!(!is_prime(1));
    assert!(!is_prime(4));
    assert!(!is_prime(6));
    assert!(!is_prime(9));
    assert!(!is_prime(10));
    assert!(!is_prime(100));
}

#[test]
fn is_divisible_tests() {
    assert!(is_divisible(10, 2));
    assert!(is_divisible(10, 5));
    assert!(!is_divisible(10, 3));
    assert!(is_divisible(0, 1));
    assert!(is_divisible(7, 1));
}

// Problem 21.1: Points in 2D (imperative)

#[test]
fn points2d_n0() {
    let result = points2d(0);
    assert_eq!(result.length(), 0);
}

#[test]
fn points2d_n3() {
    let result = points2d(3);
    assert_eq!(result.length(), 6);
    for i in 0..result.length() {
        let p = result.nth(i);
        assert!(p.0 < 3);
        assert!(1 <= p.1 && p.1 < 3);
    }
}

#[test]
fn points2d_matches_tab_flat() {
    for n in [0, 1, 3, 5] {
        let imp = points2d(n);
        let func = points2d_tab_flat(n);
        assert_eq!(imp.length(), func.length(), "length mismatch at n={n}");
    }
}

// Problem 21.3: commented out in lib.rs — no tests

// Problem 21.4: Cartesian Product

#[test]
fn cartesian_loops_basic() {
    use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    let a = ArraySeqStPerS::from_vec(vec![1, 2]);
    let b = ArraySeqStPerS::from_vec(vec![10, 20, 30]);
    let result = cartesian_loops(&a, &b);
    assert_eq!(result.length(), 6);
}

#[test]
fn cartesian_loops_empty() {
    use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    let a = ArraySeqStPerS::<N>::from_vec(Vec::new());
    let b = ArraySeqStPerS::from_vec(vec![1, 2, 3]);
    let result = cartesian_loops(&a, &b);
    assert_eq!(result.length(), 0);
}

#[test]
fn cartesian_tab_flat_basic() {
    use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    let a = ArraySeqStPerS::from_vec(vec![1, 2]);
    let b = ArraySeqStPerS::from_vec(vec![10, 20, 30]);
    let result = cartesian_tab_flat(&a, &b);
    assert_eq!(result.length(), 6);
}

#[test]
fn cartesian_matches() {
    use apas_verus::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    let a = ArraySeqStPerS::from_vec(vec![1, 2, 3]);
    let b = ArraySeqStPerS::from_vec(vec![10, 20]);
    let r1 = cartesian_loops(&a, &b);
    let r2 = cartesian_tab_flat(&a, &b);
    assert_eq!(r1.length(), r2.length());
    for i in 0..r1.length() {
        assert_eq!(r1.nth(i).0, r2.nth(i).0);
        assert_eq!(r1.nth(i).1, r2.nth(i).1);
    }
}
