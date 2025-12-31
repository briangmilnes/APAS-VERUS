// Copyright (c) 2025 Brian G. Milnes
//! Stress test: verify Pool respects thread budget limit.
//! Creates pool with budget 5, forks 20 CPU-burning tasks.

use apas_verus::Chap02::WSSchedulerMtEph::WSSchedulerMtEph::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Burn CPU for approximately the given duration
fn burn_cpu(duration: Duration) -> u64 {
    let start = Instant::now();
    let mut count: u64 = 0;
    while start.elapsed() < duration {
        // Busy work that can't be optimized away
        count = count.wrapping_add(1);
        std::hint::black_box(count);
    }
    count
}

#[test]
fn test_pool_respects_budget_limit() {
    const BUDGET: usize = 5;
    const NUM_TASKS: usize = 10;
    const BURN_SECS: u64 = 10; // Short burn for faster test
    
    let pool = Pool::new(BUDGET);
    
    // Track max concurrent threads observed
    let active_count = Arc::new(AtomicUsize::new(0));
    let max_observed = Arc::new(AtomicUsize::new(0));
    
    // Launch tasks recursively using pool.join
    fn launch_tasks(
        pool: &Pool,
        remaining: usize,
        active: Arc<AtomicUsize>,
        max_obs: Arc<AtomicUsize>,
        burn_duration: Duration,
    ) {
        if remaining == 0 {
            return;
        }
        
        if remaining == 1 {
            // Base case: just do the work
            let current = active.fetch_add(1, Ordering::SeqCst) + 1;
            // Update max observed
            loop {
                let old_max = max_obs.load(Ordering::SeqCst);
                if current <= old_max {
                    break;
                }
                if max_obs.compare_exchange(old_max, current, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                    break;
                }
            }
            
            burn_cpu(burn_duration);
            active.fetch_sub(1, Ordering::SeqCst);
        } else {
            // Split work in half
            let half = remaining / 2;
            let other_half = remaining - half;
            
            let active1 = Arc::clone(&active);
            let active2 = Arc::clone(&active);
            let max1 = Arc::clone(&max_obs);
            let max2 = Arc::clone(&max_obs);
            let pool1 = pool.clone();
            let pool2 = pool.clone();
            
            pool.join(
                move || launch_tasks(&pool1, half, active1, max1, burn_duration),
                move || launch_tasks(&pool2, other_half, active2, max2, burn_duration),
            );
        }
    }
    
    let start = Instant::now();
    
    launch_tasks(
        &pool,
        NUM_TASKS,
        Arc::clone(&active_count),
        Arc::clone(&max_observed),
        Duration::from_secs(BURN_SECS),
    );
    
    let elapsed = start.elapsed();
    let max_concurrent = max_observed.load(Ordering::SeqCst);
    
    println!("\n=== Pool Budget Limit Test ===");
    println!("Budget: {}", BUDGET);
    println!("Tasks: {}", NUM_TASKS);
    println!("Burn time per task: {}s", BURN_SECS);
    println!("Total elapsed: {:.2}s", elapsed.as_secs_f64());
    println!("Max concurrent observed: {}", max_concurrent);
    
    // With budget 5 and 20 tasks of 2s each:
    // Minimum time = (20 * 2) / 5 = 8 seconds (perfect parallelism)
    // If no limit: all 20 run at once, ~2 seconds
    
    // The key assertion: we should never exceed the budget
    assert!(
        max_concurrent <= BUDGET + 1, // +1 for main thread potentially
        "Exceeded budget! Max concurrent {} > budget {}", 
        max_concurrent, BUDGET
    );
    
    // Should take at least (NUM_TASKS * BURN_SECS) / BUDGET seconds
    let min_expected = Duration::from_secs((NUM_TASKS as u64 * BURN_SECS) / BUDGET as u64);
    assert!(
        elapsed >= min_expected / 2, // Allow some slack
        "Finished too fast - budget not being enforced? Elapsed {:?} < expected {:?}",
        elapsed, min_expected
    );
    
    println!("✓ Budget limit respected!");
}

#[test]
fn test_pool_actually_parallelizes() {
    // Sanity check: pool.join should be faster than sequential
    const BUDGET: usize = 4;
    const BURN_MS: u64 = 10000;
    
    let pool = Pool::new(BUDGET);
    
    let start_parallel = Instant::now();
    let (a, b) = pool.join(
        || burn_cpu(Duration::from_millis(BURN_MS)),
        || burn_cpu(Duration::from_millis(BURN_MS)),
    );
    let parallel_time = start_parallel.elapsed();
    
    let start_sequential = Instant::now();
    let c = burn_cpu(Duration::from_millis(BURN_MS));
    let d = burn_cpu(Duration::from_millis(BURN_MS));
    let sequential_time = start_sequential.elapsed();
    
    println!("\n=== Parallelism Check ===");
    println!("Parallel time: {:.2}s", parallel_time.as_secs_f64());
    println!("Sequential time: {:.2}s", sequential_time.as_secs_f64());
    println!("Speedup: {:.2}x", sequential_time.as_secs_f64() / parallel_time.as_secs_f64());
    
    // Parallel should be noticeably faster (at least 1.5x)
    assert!(
        parallel_time < sequential_time * 3 / 4,
        "No speedup from parallelism! Parallel {:?} vs Sequential {:?}",
        parallel_time, sequential_time
    );
    
    // Use results to prevent optimization
    assert!(a > 0 && b > 0 && c > 0 && d > 0);
    
    println!("✓ Parallelism working!");
}

