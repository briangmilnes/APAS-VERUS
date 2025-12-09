//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for FibonacciScheduler - parallel Fibonacci computation.

use apas_verus::Chap02::FibonacciScheduler::FibonacciScheduler::{
    fib_single_task, fib_parallel_tasks, fib_seq,
};

/// Expected Fibonacci values for verification
fn expected_fib(n: u64) -> u64 {
    match n {
        0 => 0, 1 => 1, 2 => 1, 3 => 2, 4 => 3, 5 => 5,
        6 => 8, 7 => 13, 8 => 21, 9 => 34, 10 => 55,
        11 => 89, 12 => 144, 13 => 233, 14 => 377, 15 => 610,
        16 => 987, 17 => 1597, 18 => 2584, 19 => 4181, 20 => 6765,
        _ => panic!("not precomputed"),
    }
}

#[test]
fn test_fib_seq_values() {
    for n in 0..=20 {
        assert_eq!(fib_seq(n), expected_fib(n), "fib_seq({}) failed", n);
    }
}

#[test]
fn test_fib_single_task_1_thread() {
    for n in 0..=20 {
        let result = fib_single_task(n, 1);
        assert_eq!(result, expected_fib(n), "fib_single_task({}, 1) failed", n);
    }
}

#[test]
fn test_fib_single_task_10_threads() {
    for n in 0..=20 {
        let result = fib_single_task(n, 10);
        assert_eq!(result, expected_fib(n), "fib_single_task({}, 10) failed", n);
    }
}

#[test]
fn test_fib_parallel_tasks_10_threads() {
    // Compute fib(0) through fib(20) in parallel with 10 threads
    let results = fib_parallel_tasks(20, 10);
    
    assert_eq!(results.len(), 21, "Should have 21 results for fib(0)..fib(20)");
    
    // Results may be in any order due to thread scheduling,
    // but they should contain all expected values
    let mut found = [false; 21];
    for &r in &results {
        // Find which fib value this is
        for n in 0..=20u64 {
            if r == expected_fib(n) && !found[n as usize] {
                found[n as usize] = true;
                break;
            }
        }
    }
    
    // All values should be found
    for n in 0..=20 {
        assert!(found[n], "Missing fib({}) = {} in results", n, expected_fib(n as u64));
    }
}

#[test]
fn test_fib_parallel_tasks_1_thread() {
    // With 1 thread, still should work correctly
    let results = fib_parallel_tasks(15, 1);
    assert_eq!(results.len(), 16);
    
    // Verify sum matches expected
    let sum: u64 = results.iter().sum();
    let expected_sum: u64 = (0..=15).map(expected_fib).sum();
    assert_eq!(sum, expected_sum);
}

#[test]
fn test_fib_parallel_tasks_2_threads() {
    let results = fib_parallel_tasks(10, 2);
    assert_eq!(results.len(), 11);
    
    let sum: u64 = results.iter().sum();
    let expected_sum: u64 = (0..=10).map(expected_fib).sum();
    assert_eq!(sum, expected_sum);
}

#[test]
fn test_fib_parallel_tasks_4_threads() {
    let results = fib_parallel_tasks(15, 4);
    assert_eq!(results.len(), 16);
    
    let sum: u64 = results.iter().sum();
    let expected_sum: u64 = (0..=15).map(expected_fib).sum();
    assert_eq!(sum, expected_sum);
}

#[test]
fn test_fib_10_threads_small_n() {
    // More threads than tasks
    let results = fib_parallel_tasks(5, 10);
    assert_eq!(results.len(), 6);
    
    let sum: u64 = results.iter().sum();
    assert_eq!(sum, 0 + 1 + 1 + 2 + 3 + 5);  // fib(0) + ... + fib(5)
}

