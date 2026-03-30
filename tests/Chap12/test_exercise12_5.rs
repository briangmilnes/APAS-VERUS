//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

use std::collections::HashSet;
use std::sync::{Arc, mpsc};
use std::thread;

use apas_verus::Chap12::Exercise12_5::Exercise12_5::*;

#[test]
fn push_pop_lifo_single_thread() {
    let stack = ConcurrentStackMt::<usize>::new();
    for value in 0usize..4 {
        stack.push(value);
    }

    for expected in (0usize..4).rev() {
        assert_eq!(stack.pop(), Some(expected));
    }
    assert_eq!(stack.pop(), None);
    assert!(stack.is_empty());
}

#[test]
fn pop_on_empty_returns_none() {
    let stack = ConcurrentStackMt::<usize>::new();
    assert!(stack.pop().is_none());
}

#[test]
fn multi_thread_push_collects_all_items() {
    let stack = Arc::new(ConcurrentStackMt::<usize>::new());
    let threads = 4;
    let per_thread = 1_000;

    let mut handles = Vec::new();
    for t in 0..threads {
        let stack_clone = Arc::clone(&stack);
        handles.push(thread::spawn(move || {
            let base = t * per_thread;
            for offset in 0..per_thread {
                stack_clone.push(base + offset);
            }
        }))
    }

    for handle in handles {
        handle.join().expect("worker panicked");
    }

    let mut values = stack.drain();
    assert_eq!(values.len(), threads * per_thread);
    values.sort_unstable();
    let expected: Vec<usize> = (0..threads * per_thread).collect();
    assert_eq!(values, expected);
}

#[test]
fn multi_thread_pop_consumes_all_elements() {
    let stack = Arc::new(ConcurrentStackMt::<usize>::new());
    let threads = 4;
    let per_thread = 800;
    for value in 0..threads * per_thread {
        stack.push(value);
    }

    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::new();
    for _ in 0..threads {
        let stack_clone = Arc::clone(&stack);
        let tx_clone = tx.clone();
        handles.push(thread::spawn(move || {
            let mut items = Vec::new();
            while let Some(value) = stack_clone.pop() {
                items.push(value);
            }
            tx_clone.send(items).expect("send items");
        }));
    }
    drop(tx);

    for handle in handles {
        handle.join().expect("worker panicked");
    }

    let mut combined = Vec::new();
    for items in rx {
        combined.extend(items);
    }
    assert_eq!(combined.len(), threads * per_thread);

    let unique: HashSet<usize> = combined.iter().copied().collect();
    assert_eq!(unique.len(), combined.len());
}

#[test]
fn test_default_trait() {
    let stack: ConcurrentStackMt<i32> = Default::default();
    assert!(stack.is_empty());
    assert_eq!(stack.pop(), None);
}

#[test]
fn test_drain() {
    let stack = ConcurrentStackMt::new();
    for i in 0..10 {
        stack.push(i);
    }

    let drained = stack.drain();
    assert_eq!(drained.len(), 10);
    assert!(stack.is_empty());
}

#[test]
fn test_multiple_push_pop_cycles() {
    let stack = ConcurrentStackMt::new();

    stack.push(1);
    stack.push(2);
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), Some(1));

    stack.push(3);
    stack.push(4);
    assert_eq!(stack.pop(), Some(4));
    assert_eq!(stack.pop(), Some(3));

    assert!(stack.is_empty());
}

#[test]
fn test_push_many_pop_all() {
    let stack = ConcurrentStackMt::new();
    for i in 0..100 {
        stack.push(i);
    }
    for i in (0..100).rev() {
        assert_eq!(stack.pop(), Some(i));
    }
    assert!(stack.is_empty());
}

#[test]
fn test_interleaved_push_pop() {
    let stack = ConcurrentStackMt::new();
    for i in 0..50 {
        stack.push(i);
        if i % 2 == 0 {
            assert_eq!(stack.pop(), Some(i));
        }
    }
}

#[test]
fn test_drain_empty_stack() {
    let stack = ConcurrentStackMt::<i32>::new();
    let drained = stack.drain();
    assert!(drained.is_empty());
}

#[test]
fn test_is_empty_after_operations() {
    let stack = ConcurrentStackMt::new();
    assert!(stack.is_empty());
    stack.push(1);
    assert!(!stack.is_empty());
    stack.pop();
    assert!(stack.is_empty());
}

#[test]
fn test_pop_on_empty_multiple_times() {
    let stack = ConcurrentStackMt::<i32>::new();
    for _ in 0..10 {
        assert_eq!(stack.pop(), None);
    }
}

#[test]
fn test_string_values() {
    let stack = ConcurrentStackMt::new();
    stack.push("hello".to_string());
    stack.push("world".to_string());
    assert_eq!(stack.pop(), Some("world".to_string()));
    assert_eq!(stack.pop(), Some("hello".to_string()));
}

#[test]
fn test_drain_preserves_all_elements() {
    let stack = ConcurrentStackMt::new();
    for i in 0..50 {
        stack.push(i);
    }
    let mut drained = stack.drain();
    drained.sort_unstable();
    let expected: Vec<i32> = (0..50).collect();
    assert_eq!(drained, expected);
}

#[test]
fn test_push_large_count() {
    let stack = ConcurrentStackMt::new();
    for i in 0..5000 {
        stack.push(i);
    }
    let drained = stack.drain();
    assert_eq!(drained.len(), 5000);
}

#[test]
fn test_concurrent_push_pop_interleaved() {
    let stack = Arc::new(ConcurrentStackMt::<usize>::new());
    let mut handles = Vec::new();

    // Pushers.
    for t in 0..4 {
        let s = Arc::clone(&stack);
        handles.push(thread::spawn(move || {
            for i in 0..200 {
                s.push(t * 200 + i);
            }
        }));
    }

    // Poppers.
    let (tx, rx) = mpsc::channel();
    for _ in 0..2 {
        let s = Arc::clone(&stack);
        let tx = tx.clone();
        handles.push(thread::spawn(move || {
            let mut count = 0;
            loop {
                match s.pop() {
                    Some(_) => count += 1,
                    None => {
                        thread::yield_now();
                        if s.is_empty() { break; }
                    }
                }
            }
            tx.send(count).unwrap();
        }));
    }
    drop(tx);

    for handle in handles {
        handle.join().unwrap();
    }

    // Remaining items in stack + popped items = 800.
    let popped: usize = rx.iter().sum();
    let remaining = stack.drain().len();
    assert_eq!(popped + remaining, 800);
}

#[test]
fn test_drain_twice() {
    let stack = ConcurrentStackMt::new();
    stack.push(1);
    stack.push(2);
    let first = stack.drain();
    assert_eq!(first.len(), 2);
    let second = stack.drain();
    assert!(second.is_empty());
}
