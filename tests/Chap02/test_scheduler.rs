//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for SchedulerMtEph at the scheduler level.

use apas_verus::Chap02::SchedulerMtEph::SchedulerMtEph::SchedulerMtEph;

#[test]
fn test_scheduler_single_task_max1() {
    let mut sched: SchedulerMtEph<i32> = SchedulerMtEph::new(1);
    sched.execute(|| 42);
    let results = sched.join();
    
    assert_eq!(results.len(), 1);
    assert_eq!(results[0], 42);
}

#[test]
fn test_scheduler_single_task_max4() {
    let mut sched: SchedulerMtEph<i32> = SchedulerMtEph::new(4);
    sched.execute(|| 42);
    let results = sched.join();
    
    assert_eq!(results.len(), 1);
    assert_eq!(results[0], 42);
}

#[test]
fn test_scheduler_two_tasks_max1() {
    let mut sched: SchedulerMtEph<i32> = SchedulerMtEph::new(1);
    sched.execute(|| 1);
    sched.execute(|| 2);
    let results = sched.join();
    
    assert_eq!(results.len(), 2);
    // Order might vary depending on polling
    assert!(results.contains(&1));
    assert!(results.contains(&2));
}

#[test]
fn test_scheduler_two_tasks_max2() {
    let mut sched: SchedulerMtEph<i32> = SchedulerMtEph::new(2);
    sched.execute(|| 1);
    sched.execute(|| 2);
    let results = sched.join();
    
    assert_eq!(results.len(), 2);
    assert!(results.contains(&1));
    assert!(results.contains(&2));
}

#[test]
fn test_scheduler_three_tasks_max2() {
    let mut sched: SchedulerMtEph<i32> = SchedulerMtEph::new(2);
    sched.execute(|| 10);
    sched.execute(|| 20);
    sched.execute(|| 30);
    let results = sched.join();
    
    assert_eq!(results.len(), 3);
    let sum: i32 = results.iter().sum();
    assert_eq!(sum, 60);
}

#[test]
fn test_scheduler_many_tasks_max1() {
    let mut sched: SchedulerMtEph<usize> = SchedulerMtEph::new(1);
    for i in 0..10 {
        sched.execute(move || i * i);
    }
    let results = sched.join();
    
    assert_eq!(results.len(), 10);
    let sum: usize = results.iter().sum();
    assert_eq!(sum, 0 + 1 + 4 + 9 + 16 + 25 + 36 + 49 + 64 + 81);
}

#[test]
fn test_scheduler_many_tasks_max4() {
    let mut sched: SchedulerMtEph<usize> = SchedulerMtEph::new(4);
    for i in 0..20 {
        sched.execute(move || i);
    }
    let results = sched.join();
    
    assert_eq!(results.len(), 20);
    let sum: usize = results.iter().sum();
    assert_eq!(sum, (0..20).sum());
}

#[test]
fn test_scheduler_tasks_with_computation() {
    let mut sched: SchedulerMtEph<u64> = SchedulerMtEph::new(4);
    
    // Each task does a small computation
    for i in 0u64..8 {
        sched.execute(move || {
            let mut acc = 0u64;
            for j in 0..1000 {
                acc += j;
            }
            acc + i
        });
    }
    
    let results = sched.join();
    assert_eq!(results.len(), 8);
    
    // Each result should be 499500 + i
    let base: u64 = (0..1000u64).sum();
    for r in &results {
        assert!(*r >= base && *r < base + 8);
    }
}

#[test]
fn test_scheduler_string_results() {
    let mut sched: SchedulerMtEph<String> = SchedulerMtEph::new(2);
    sched.execute(|| "hello".to_string());
    sched.execute(|| "world".to_string());
    let results = sched.join();
    
    assert_eq!(results.len(), 2);
    assert!(results.contains(&"hello".to_string()));
    assert!(results.contains(&"world".to_string()));
}

#[test]
fn test_scheduler_vec_results() {
    let mut sched: SchedulerMtEph<Vec<i32>> = SchedulerMtEph::new(3);
    sched.execute(|| vec![1, 2, 3]);
    sched.execute(|| vec![4, 5]);
    sched.execute(|| vec![6]);
    let results = sched.join();
    
    assert_eq!(results.len(), 3);
    let total_elements: usize = results.iter().map(|v| v.len()).sum();
    assert_eq!(total_elements, 6);
}

#[test]
fn test_scheduler_no_tasks() {
    let mut sched: SchedulerMtEph<i32> = SchedulerMtEph::new(4);
    let results = sched.join();
    
    assert_eq!(results.len(), 0);
}

#[test]
fn test_scheduler_max_threads_respected() {
    // With max_threads=1, tasks should run sequentially
    // We can't directly test thread count, but we can test correctness
    let mut sched: SchedulerMtEph<u32> = SchedulerMtEph::new(1);
    
    for i in 0..5 {
        sched.execute(move || i);
    }
    
    let results = sched.join();
    assert_eq!(results.len(), 5);
}

