//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Chapter 12 — Exercise 12.5: lock-free concurrent stack using compare-and-swap.
//!
//! Note: Concurrent data structures cannot have simple sequential specs because
//! state changes concurrently. Raw pointers and AtomicPtr require external_body.
//! Specs here are trusted documentation of linearizable behavior.

//  Table of Contents
//	1. module
//	4. type definitions
//	8. traits
//	9. impls

//		1. module


pub mod Exercise12_5 {
    use vstd::prelude::*;
    use std::ptr::null_mut;
    use std::sync::atomic::{AtomicPtr, Ordering};

verus! {

//		4. type definitions

/// Node for the lock-free stack. External due to raw pointer field.
#[verifier::external]
struct Node<T> {
    value: T,
    next: *mut Node<T>,
}

/// Lock-free concurrent stack using AtomicPtr and CAS.
/// External due to AtomicPtr (no vstd specs) and raw pointers.
#[verifier::external_body]
#[verifier::reject_recursive_types(T)]
pub struct ConcurrentStackMt<T: Send> {
    head: AtomicPtr<Node<T>>,
}


//		8. traits

/// Trait for lock-free concurrent stack operations.
/// 
/// Specs describe linearizable behavior: each operation appears to take effect
/// atomically at some point between its invocation and response.
pub trait ConcurrentStackMtTrait<T: Send>: Sized {
    /// Spec: is this stack instance well-formed?
    spec fn wf(&self) -> bool;
    
    /// Create a new empty stack.
    fn new() -> (stack: Self)
        ensures stack.wf();
    
    /// Push a value onto the stack. Always succeeds (may spin under contention).
    fn push(&self, value: T)
        requires self.wf();
    
    /// Pop a value from the stack.
    /// Returns Some(v) where v was the top element at the linearization point,
    /// or None if the stack was empty at that point.
    fn pop(&self) -> (possible_top: Option<T>)
        requires self.wf();
    
    /// Check if the stack is empty at this instant.
    /// Note: Result may be stale by the time caller acts on it.
    fn is_empty(&self) -> (empty: bool)
        requires self.wf();
    
    /// Drain all elements from the stack into a Vec.
    /// Elements are returned in LIFO order (most recently pushed first).
    /// Note: Concurrent pushes during drain may or may not be included.
    fn drain(&self) -> (items: Vec<T>)
        requires self.wf();
}


//		9. impls

impl<T: Send> ConcurrentStackMt<T> {
    /// Spec: the stack is always well-formed after construction.
    pub closed spec fn wf(&self) -> bool {
        true  // Opaque - trusted to be correct
    }
}

impl<T: Send> ConcurrentStackMtTrait<T> for ConcurrentStackMt<T> {
    open spec fn wf(&self) -> bool {
        ConcurrentStackMt::wf(self)
    }

    #[verifier::external_body]
    fn new() -> (stack: Self) {
        ConcurrentStackMt {
            head: AtomicPtr::new(null_mut()),
        }
    }

    #[verifier::external_body]
    fn push(&self, value: T) {
        let mut new_node = Box::new(Node { value, next: null_mut() });
        loop {
            let head = self.head.load(Ordering::Acquire);
            new_node.next = head;
            let node_ptr = Box::into_raw(new_node);
            if self.head.compare_exchange_weak(head, node_ptr, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                break;
            }
            new_node = unsafe { Box::from_raw(node_ptr) };
        }
    }

    #[verifier::external_body]
    fn pop(&self) -> (possible_top: Option<T>) {
        loop {
            let head = self.head.load(Ordering::Acquire);
            if head.is_null() {
                return None;
            }
            let next = unsafe { (*head).next };
            if self.head.compare_exchange_weak(head, next, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                let boxed = unsafe { Box::from_raw(head) };
                return Some(boxed.value);
            }
        }
    }

    #[verifier::external_body]
    fn is_empty(&self) -> (empty: bool) {
        self.head.load(Ordering::Acquire).is_null()
    }

    #[verifier::external_body]
    fn drain(&self) -> (items: Vec<T>) {
        let mut items = Vec::new();
        while let Some(value) = self.pop() {
            items.push(value);
        }
        items
    }
}

impl<T: Send> Default for ConcurrentStackMt<T> {
    #[verifier::external_body]
    fn default() -> Self { 
        ConcurrentStackMt::new() 
    }
}

} // verus!

impl<T: Send> Drop for ConcurrentStackMt<T> {
    fn drop(&mut self) {
        let mut current = self.head.load(Ordering::Relaxed);
        while !current.is_null() {
            unsafe {
                let node = Box::from_raw(current);
                current = node.next;
            }
        }
    }
}

} // mod
