//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use apas_verus::Chap02::FibonacciWSScheduler::FibonacciWSScheduler::{fib_seq, fib_pool};
use apas_verus::Chap02::WSSchedulerMtEph::WSSchedulerMtEph::{Pool, PoolTrait};

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
fn test_fib_pool() {
    let pool = Pool::new(6);
    for n in 0..=20 {
        assert_eq!(fib_pool(&pool, n), expected_fib(n));
    }
}

#[test]
fn test_fib_pool_larger() {
    let pool = Pool::new(6);
    assert_eq!(fib_pool(&pool, 25), expected_fib(25));
    assert_eq!(fib_pool(&pool, 30), expected_fib(30));
}
