//! Tests for WorkStealingSchedulerMtEph

use apas_verus::Chap02::WSSchedulerMtEph::WSSchedulerMtEph::*;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;

#[test]
fn test_join_simple() {
    let (a, b) = join(
        || 1 + 1,
        || 2 + 2,
    );
    assert_eq!(a, 2);
    assert_eq!(b, 4);
}

#[test]
fn test_join_different_types() {
    let (a, b) = join(
        || "hello".to_string(),
        || 42u64,
    );
    assert_eq!(a, "hello");
    assert_eq!(b, 42);
}

#[test]
fn test_join_nested() {
    let (a, b) = join(
        || {
            let (x, y) = join(|| 1, || 2);
            x + y
        },
        || {
            let (x, y) = join(|| 3, || 4);
            x + y
        },
    );
    assert_eq!(a, 3);
    assert_eq!(b, 7);
}

#[test]
fn test_fib_sequential() {
    assert_eq!(fib_sequential(0), 0);
    assert_eq!(fib_sequential(1), 1);
    assert_eq!(fib_sequential(2), 1);
    assert_eq!(fib_sequential(5), 5);
    assert_eq!(fib_sequential(10), 55);
    assert_eq!(fib_sequential(20), 6765);
}

#[test]
fn test_fib_parallel() {
    assert_eq!(fib_parallel(0), 0);
    assert_eq!(fib_parallel(1), 1);
    assert_eq!(fib_parallel(2), 1);
    assert_eq!(fib_parallel(5), 5);
    assert_eq!(fib_parallel(10), 55);
    assert_eq!(fib_parallel(20), 6765);
}

#[test]
fn test_fib_parallel_larger() {
    // fib(30) = 832040
    assert_eq!(fib_parallel(30), 832040);
}

#[test]
fn test_fib_parallel_bounded() {
    let budget = Arc::new(AtomicUsize::new(4));
    assert_eq!(fib_parallel_bounded(budget.clone(), 0), 0);
    assert_eq!(fib_parallel_bounded(budget.clone(), 1), 1);
    assert_eq!(fib_parallel_bounded(budget.clone(), 10), 55);
    assert_eq!(fib_parallel_bounded(budget.clone(), 20), 6765);
}

#[test]
fn test_fib_parallel_bounded_single_thread() {
    // Budget of 0 means all sequential
    let budget = Arc::new(AtomicUsize::new(0));
    assert_eq!(fib_parallel_bounded(budget.clone(), 20), 6765);
}

#[test]
fn test_thread_pool_creation() {
    let pool = ThreadPool::new(4);
    assert_eq!(pool.num_workers(), 4);
    pool.shutdown();
}

#[test]
fn test_thread_pool_different_sizes() {
    for n in [1, 2, 4, 8] {
        let pool = ThreadPool::new(n);
        assert_eq!(pool.num_workers(), n);
        pool.shutdown();
    }
}

#[test]
fn test_join_with_computation() {
    let (a, b) = join(
        || {
            let mut sum = 0u64;
            for i in 0..1000 {
                sum += i;
            }
            sum
        },
        || {
            let mut prod = 1u64;
            for i in 1..20 {
                prod *= i;
            }
            prod
        },
    );
    assert_eq!(a, 499500);  // sum 0..1000
    assert_eq!(b, 121645100408832000);  // 19!
}

