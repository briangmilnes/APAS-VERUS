// Copyright (c) 2025 Brian G. Milnes
//! Tests for HFSchedulerMtEph global work-stealing pool.

use apas_verus::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::*;
use std::time::{Duration, Instant};

#[test]
fn test_spawn_join_simple() {
    let (a, b) = spawn_join(
        || 1 + 1,
        || 2 + 2,
    );
    assert_eq!(a, 2);
    assert_eq!(b, 4);
}

#[test]
fn test_spawn_join_different_types() {
    let (a, b) = spawn_join(
        || "hello".to_string(),
        || 42u64,
    );
    assert_eq!(a, "hello");
    assert_eq!(b, 42);
}

#[test]
fn test_spawn_join_nested() {
    let (a, b) = spawn_join(
        || {
            let (x, y) = spawn_join(|| 1, || 2);
            x + y
        },
        || {
            let (x, y) = spawn_join(|| 3, || 4);
            x + y
        },
    );
    assert_eq!(a, 3);
    assert_eq!(b, 7);
}

#[test]
fn test_join_simple() {
    let (a, b) = join(
        || 10 + 10,
        || 20 + 20,
    );
    assert_eq!(a, 20);
    assert_eq!(b, 40);
}

#[test]
fn test_join_heavy() {
    let (a, b) = join(
        || (1..=100).sum::<i32>(),
        || (1..=200).sum::<i32>(),
    );
    assert_eq!(a, 5050);
    assert_eq!(b, 20100);
}

#[test]
fn test_spawn_join_with_computation() {
    let (a, b) = spawn_join(
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

// ============ spawn/wait tests ============

#[test]
fn test_spawn_wait_simple() {
    let h1 = spawn(|| 1 + 1);
    let h2 = spawn(|| 2 + 2);
    let a = wait(h1);
    let b = wait(h2);
    assert_eq!(a, 2);
    assert_eq!(b, 4);
}

#[test]
fn test_spawn_wait_n_tasks() {
    let n = 10;
    
    // Spawn N tasks.
    let mut handles = Vec::new();
    for i in 0..n {
        let h = spawn(move || i * i);
        handles.push(h);
    }
    
    // Wait for all and collect results.
    let mut results = Vec::new();
    for h in handles {
        results.push(wait(h));
    }
    
    // Verify results.
    for i in 0..n {
        assert_eq!(results[i], i * i);
    }
}

#[test]
fn test_spawn_wait_parallel() {
    // Verify spawn/wait actually runs in parallel.
    let burn_ms = 50;
    
    let start = Instant::now();
    
    // Spawn 4 tasks that each burn 50ms.
    let h1 = spawn(move || { std::thread::sleep(Duration::from_millis(burn_ms)); 1 });
    let h2 = spawn(move || { std::thread::sleep(Duration::from_millis(burn_ms)); 2 });
    let h3 = spawn(move || { std::thread::sleep(Duration::from_millis(burn_ms)); 3 });
    let h4 = spawn(move || { std::thread::sleep(Duration::from_millis(burn_ms)); 4 });
    
    let r1 = wait(h1);
    let r2 = wait(h2);
    let r3 = wait(h3);
    let r4 = wait(h4);
    
    let elapsed = start.elapsed();
    
    assert_eq!(r1 + r2 + r3 + r4, 10);
    
    // If parallel: ~50ms. If sequential: ~200ms.
    assert!(
        elapsed < Duration::from_millis(150),
        "spawn/wait not parallel! Took {:?}, expected ~50ms", elapsed
    );
}

#[test]
fn test_join_returns_correct_types() {
    let (a, b): (String, Vec<i32>) = join(
        || "hello".to_string(),
        || vec![1, 2, 3],
    );
    assert_eq!(a, "hello");
    assert_eq!(b, vec![1, 2, 3]);
}

#[test]
fn test_spawn_join_identity() {
    let (a, b) = spawn_join(|| 0, || 0);
    assert_eq!(a, 0);
    assert_eq!(b, 0);
}

#[test]
fn test_spawn_wait_string() {
    let h = spawn(|| format!("{} {}", "hello", "world"));
    assert_eq!(wait(h), "hello world");
}

#[test]
fn test_join_large_result() {
    let (a, b) = join(
        || (0..100).collect::<Vec<i32>>(),
        || (100..200).collect::<Vec<i32>>(),
    );
    assert_eq!(a.len(), 100);
    assert_eq!(b.len(), 100);
    assert_eq!(a[0], 0);
    assert_eq!(b[0], 100);
}

#[test]
fn test_spawn_wait_chained() {
    let h1 = spawn(|| 10);
    let v1 = wait(h1);
    let h2 = spawn(move || v1 * 2);
    let v2 = wait(h2);
    assert_eq!(v2, 20);
}

#[test]
fn test_join_closures_capture() {
    let x = 42;
    let (a, b) = join(
        move || x + 1,
        move || x * 2,
    );
    assert_eq!(a, 43);
    assert_eq!(b, 84);
}

#[test]
fn test_spawn_wait_vec_result() {
    let h = spawn(|| (0..50).collect::<Vec<i32>>());
    let v = wait(h);
    assert_eq!(v.len(), 50);
    assert_eq!(v[49], 49);
}

#[test]
fn test_spawn_join_zero_computation() {
    let (a, b) = spawn_join(|| (), || ());
    assert_eq!(a, ());
    assert_eq!(b, ());
}
