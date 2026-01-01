// Copyright (c) 2025 Brian G. Milnes
//! Tests for WSSchedulerMtEph work-stealing pool

use apas_verus::Chap02::WSSchedulerMtEph::WSSchedulerMtEph::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

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

// ============ spawn/wait tests ============

#[test]
fn test_spawn_wait_simple() {
    let pool = Pool::new(4);
    let h1 = pool.spawn(|| 1 + 1);
    let h2 = pool.spawn(|| 2 + 2);
    let a = Pool::wait(h1);
    let b = Pool::wait(h2);
    assert_eq!(a, 2);
    assert_eq!(b, 4);
}

#[test]
fn test_spawn_wait_n_tasks() {
    let pool = Pool::new(4);
    let n = 10;
    
    // Spawn N tasks
    let mut handles = Vec::new();
    for i in 0..n {
        let h = pool.spawn(move || i * i);
        handles.push(h);
    }
    
    // Wait for all and collect results
    let mut results = Vec::new();
    for h in handles {
        results.push(Pool::wait(h));
    }
    
    // Verify results
    for i in 0..n {
        assert_eq!(results[i], i * i);
    }
}

#[test]
fn test_spawn_wait_parallel() {
    // Verify spawn/wait actually runs in parallel
    let pool = Pool::new(4);
    let burn_ms = 200;
    
    let start = Instant::now();
    
    // Spawn 4 tasks that each burn 200ms
    let h1 = pool.spawn(move || { std::thread::sleep(Duration::from_millis(burn_ms)); 1 });
    let h2 = pool.spawn(move || { std::thread::sleep(Duration::from_millis(burn_ms)); 2 });
    let h3 = pool.spawn(move || { std::thread::sleep(Duration::from_millis(burn_ms)); 3 });
    let h4 = pool.spawn(move || { std::thread::sleep(Duration::from_millis(burn_ms)); 4 });
    
    let r1 = Pool::wait(h1);
    let r2 = Pool::wait(h2);
    let r3 = Pool::wait(h3);
    let r4 = Pool::wait(h4);
    
    let elapsed = start.elapsed();
    
    assert_eq!(r1 + r2 + r3 + r4, 10);
    
    // If parallel: ~200ms. If sequential: ~800ms.
    // Allow up to 400ms for parallel (some overhead).
    assert!(
        elapsed < Duration::from_millis(500),
        "spawn/wait not parallel! Took {:?}, expected ~200ms", elapsed
    );
}

#[test]
fn test_spawn_wait_respects_budget() {
    // With budget=2, only 2 tasks run concurrently
    let pool = Pool::new(2);
    let active = Arc::new(AtomicUsize::new(0));
    let max_active = Arc::new(AtomicUsize::new(0));
    
    let mut handles = Vec::new();
    for _ in 0..6 {
        let a = Arc::clone(&active);
        let m = Arc::clone(&max_active);
        let h = pool.spawn(move || {
            let current = a.fetch_add(1, Ordering::SeqCst) + 1;
            // Update max
            loop {
                let old = m.load(Ordering::SeqCst);
                if current <= old || m.compare_exchange(old, current, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                    break;
                }
            }
            std::thread::sleep(Duration::from_millis(50));
            a.fetch_sub(1, Ordering::SeqCst);
            current
        });
        handles.push(h);
    }
    
    for h in handles {
        Pool::wait(h);
    }
    
    let max = max_active.load(Ordering::SeqCst);
    assert!(
        max <= 3, // budget + 1 for timing slack
        "Exceeded budget! Max concurrent: {}, budget: 2", max
    );
}
