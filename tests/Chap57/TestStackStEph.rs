//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! Tests for StackStEph

use apas_verus::Chap57::StackStEph::StackStEph::*;
use apas_verus::Types::Types::*;

#[test]
fn test_new_stack_is_empty() {
    let stack = StackStEph::<i32>::new();
    assert!(stack.is_empty());
    assert_eq!(stack.size(), 0);
}

#[test]
fn test_push_single_element() {
    let mut stack = StackStEph::new();
    stack.push(42);
    assert!(!stack.is_empty());
    assert_eq!(stack.size(), 1);
}

#[test]
fn test_push_and_pop() {
    let mut stack = StackStEph::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    assert_eq!(stack.size(), 3);
    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), None);
    assert!(stack.is_empty());
}

#[test]
fn test_peek() {
    let mut stack = StackStEph::new();
    assert_eq!(stack.peek(), None);

    stack.push(10);
    assert_eq!(stack.peek(), Some(&10));
    assert_eq!(stack.size(), 1); // Peek doesn't remove

    stack.push(20);
    assert_eq!(stack.peek(), Some(&20));
    assert_eq!(stack.size(), 2);

    stack.pop();
    assert_eq!(stack.peek(), Some(&10));
}

#[test]
fn test_pop_from_empty() {
    let mut stack = StackStEph::<String>::new();
    assert_eq!(stack.pop(), None);
    assert!(stack.is_empty());
}

#[test]
fn test_multiple_operations() {
    let mut stack = StackStEph::new();

    // Push some elements
    for i in 0..10 {
        stack.push(i);
        assert_eq!(stack.size(), i + 1);
    }

    // Pop half
    for i in (5..10).rev() {
        assert_eq!(stack.pop(), Some(i));
    }

    assert_eq!(stack.size(), 5);

    // Push more
    stack.push(100);
    stack.push(200);

    assert_eq!(stack.size(), 7);
    assert_eq!(stack.peek(), Some(&200));
}

#[test]
fn test_lifo_order() {
    let mut stack = StackStEph::new();
    let items = vec!["first", "second", "third", "fourth"];

    for &item in &items {
        stack.push(item);
    }

    // Should come out in reverse order (LIFO)
    for &item in items.iter().rev() {
        assert_eq!(stack.pop(), Some(item));
    }
}

#[test]
fn test_size_tracking() {
    let mut stack = StackStEph::new();
    assert_eq!(stack.size(), 0);

    stack.push(1);
    assert_eq!(stack.size(), 1);

    stack.push(2);
    assert_eq!(stack.size(), 2);

    stack.pop();
    assert_eq!(stack.size(), 1);

    stack.pop();
    assert_eq!(stack.size(), 0);

    stack.pop(); // Pop from empty
    assert_eq!(stack.size(), 0);
}

#[test]
fn test_default_trait() {
    let stack = StackStEph::<i32>::default();
    assert!(stack.is_empty());
    assert_eq!(stack.size(), 0);
}

#[test]
fn test_clone() {
    let mut stack1 = StackStEph::new();
    stack1.push(1);
    stack1.push(2);
    stack1.push(3);

    let mut stack2 = stack1.clone();

    // Both stacks should have same elements
    assert_eq!(stack1.size(), stack2.size());
    assert_eq!(stack1.pop(), stack2.pop());

    // After popping, they're independent
    stack1.push(99);
    assert_eq!(stack1.peek(), Some(&99));
    assert_eq!(stack2.peek(), Some(&2));
}
