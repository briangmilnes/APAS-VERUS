//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for ConcurrentScheduler - verified concurrent execute() with RwLock.

use apas_verus::Chap02::ConcurrentScheduler::ConcurrentScheduler::{
    ConcurrentScheduler, join,
};

#[test]
fn test_single_thread_execute() {
    let sched = ConcurrentScheduler::<i32>::new(4);
    sched.execute(|| 42);
    sched.execute(|| 100);
    
    let results = join(&sched);
    
    assert_eq!(results.len(), 2);
    let sum: i32 = results.iter().sum();
    assert_eq!(sum, 142);
}

#[test]
fn test_many_tasks() {
    let sched = ConcurrentScheduler::<usize>::new(4);
    
    for i in 0..20 {
        sched.execute(move || i * i);
    }
    
    let results = join(&sched);
    
    assert_eq!(results.len(), 20);
    let sum: usize = results.iter().sum();
    assert_eq!(sum, 2470);  // sum of squares 0..20
}

#[test]
fn test_with_computation() {
    let sched = ConcurrentScheduler::<u64>::new(8);
    
    for i in 0u64..10 {
        sched.execute(move || {
            let mut acc = 0u64;
            for j in 0..1000 {
                acc += j;
            }
            acc + i
        });
    }
    
    let results = join(&sched);
    
    assert_eq!(results.len(), 10);
    
    let base: u64 = (0..1000u64).sum();
    for r in &results {
        assert!(*r >= base && *r < base + 10);
    }
}

#[test]
fn test_no_tasks() {
    let sched = ConcurrentScheduler::<i32>::new(4);
    let results = join(&sched);
    assert_eq!(results.len(), 0);
}

#[test]
fn test_one_task() {
    let sched = ConcurrentScheduler::<String>::new(2);
    sched.execute(|| "hello".to_string());
    
    let results = join(&sched);
    
    assert_eq!(results.len(), 1);
    assert_eq!(results[0], "hello");
}

#[test]
fn test_is_closed() {
    let sched = ConcurrentScheduler::<i32>::new(4);
    assert!(!sched.is_closed());
    
    sched.execute(|| 1);
    assert!(!sched.is_closed());
    
    let _ = join(&sched);
    assert!(sched.is_closed());
}

#[test]
fn test_pending_count() {
    let sched = ConcurrentScheduler::<i32>::new(4);
    assert_eq!(sched.pending_count(), 0);
    
    sched.execute(|| 1);
    assert_eq!(sched.pending_count(), 1);
    
    sched.execute(|| 2);
    assert_eq!(sched.pending_count(), 2);
}
