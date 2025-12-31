// Copyright (c) 2025 Brian G. Milnes
//! Tests for WSSchedulerMtEph work-stealing pool

use apas_verus::Chap02::WSSchedulerMtEph::WSSchedulerMtEph::*;

#[test]
fn test_spawn_join_simple() {
    let (a, b) = Pool::spawn_join(
        || 1 + 1,
        || 2 + 2,
    );
    assert_eq!(a, 2);
    assert_eq!(b, 4);
}

#[test]
fn test_spawn_join_different_types() {
    let (a, b) = Pool::spawn_join(
        || "hello".to_string(),
        || 42u64,
    );
    assert_eq!(a, "hello");
    assert_eq!(b, 42);
}

#[test]
fn test_spawn_join_nested() {
    let (a, b) = Pool::spawn_join(
        || {
            let (x, y) = Pool::spawn_join(|| 1, || 2);
            x + y
        },
        || {
            let (x, y) = Pool::spawn_join(|| 3, || 4);
            x + y
        },
    );
    assert_eq!(a, 3);
    assert_eq!(b, 7);
}

#[test]
fn test_pool_join_simple() {
    let pool = Pool::new(4);
    let (a, b) = pool.join(
        || 10 + 10,
        || 20 + 20,
    );
    assert_eq!(a, 20);
    assert_eq!(b, 40);
}

#[test]
fn test_pool_join_heavy() {
    let pool = Pool::new(4);
    let (a, b) = pool.join(
        || (1..=100).sum::<i32>(),
        || (1..=200).sum::<i32>(),
    );
    assert_eq!(a, 5050);
    assert_eq!(b, 20100);
}

#[test]
fn test_spawn_join_with_computation() {
    let (a, b) = Pool::spawn_join(
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
    assert_eq!(a, 499500);
    assert_eq!(b, 121645100408832000);
}
