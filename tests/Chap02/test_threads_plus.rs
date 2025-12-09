//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Tests for threads_plus at the raw thread level.

use apas_verus::vstdplus::threads_plus::threads_plus::{spawn_plus, JoinHandlePlus};

#[test]
fn test_single_thread_returns_value() {
    let handle = spawn_plus(|| 42);
    let result = handle.join();
    assert!(matches!(result, Ok(42)));
}

#[test]
fn test_single_thread_with_computation() {
    let handle = spawn_plus(|| {
        let mut sum = 0;
        for i in 0..10 {
            sum += i;
        }
        sum
    });
    let result = handle.join();
    assert!(matches!(result, Ok(45)));
}

#[test]
fn test_two_threads_parallel() {
    let h1 = spawn_plus(|| 1);
    let h2 = spawn_plus(|| 2);
    
    let r1 = h1.join();
    let r2 = h2.join();
    
    assert!(matches!(r1, Ok(1)));
    assert!(matches!(r2, Ok(2)));
}

#[test]
fn test_three_threads_parallel() {
    let h1 = spawn_plus(|| "one");
    let h2 = spawn_plus(|| "two");
    let h3 = spawn_plus(|| "three");
    
    let r1 = h1.join();
    let r2 = h2.join();
    let r3 = h3.join();
    
    assert!(matches!(r1, Ok("one")));
    assert!(matches!(r2, Ok("two")));
    assert!(matches!(r3, Ok("three")));
}

#[test]
fn test_is_finished_eventually_true() {
    let handle = spawn_plus(|| {
        // Quick computation
        1 + 1
    });
    
    // Spin until finished (should be very fast)
    let mut iterations = 0;
    while !handle.is_finished() {
        iterations += 1;
        if iterations > 1_000_000 {
            panic!("Thread didn't finish in reasonable time");
        }
        std::hint::spin_loop();
    }
    
    let result = handle.join();
    assert!(matches!(result, Ok(2)));
}

#[test]
fn test_thread_with_sleep() {
    let handle = spawn_plus(|| {
        std::thread::sleep(std::time::Duration::from_millis(10));
        "done"
    });
    
    // Initially might not be finished
    // (though on fast machines it might be)
    
    let result = handle.join();
    assert!(matches!(result, Ok("done")));
}

#[test]
fn test_many_threads() {
    let mut handles: Vec<JoinHandlePlus<usize>> = Vec::new();
    
    for i in 0..10 {
        handles.push(spawn_plus(move || i * i));
    }
    
    let mut results: Vec<usize> = Vec::new();
    for h in handles {
        if let Ok(v) = h.join() {
            results.push(v);
        }
    }
    
    assert_eq!(results, vec![0, 1, 4, 9, 16, 25, 36, 49, 64, 81]);
}

