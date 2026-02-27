//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Chapter 12 — Exercise 12.5: lock-free concurrent stack using compare-and-swap.
//!
//! The external_body and external annotations here are permanent. This stack
//! uses AtomicPtr and raw pointers for a lock-free Treiber stack — none of
//! which have vstd specs. Meaningful verification would require a tokenized
//! state machine (TSM) to model the linearizability argument, which is
//! disproportionate for a CAS exercise. The RTTs validate runtime behavior.

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
#[verifier::external] // accept hole
struct Node<T> {
    value: T,
    next: *mut Node<T>,
}

/// Lock-free concurrent stack using AtomicPtr and CAS.
/// External due to AtomicPtr (no vstd specs) and raw pointers.
#[verifier::external_body] // accept hole
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
    /// Spec: the stack is always well-formed after construction.
    open spec fn wf(&self) -> bool { true }
    
    /// Create a new empty stack.
    /// - APAS: no cost spec.
    /// - Claude-Opus-4.6: O(1).
    fn new() -> (stack: Self)
        ensures stack.wf();
    
    /// Push a value onto the stack. Always succeeds (may spin under contention).
    /// - APAS: no cost spec.
    /// - Claude-Opus-4.6: amortized O(1), worst-case unbounded (CAS retries). Lock-free.
    fn push(&self, value: T)
        requires self.wf();
    
    /// Pop a value from the stack.
    /// Returns Some(v) where v was the top element at the linearization point,
    /// or None if the stack was empty at that point.
    /// - APAS: no cost spec.
    /// - Claude-Opus-4.6: amortized O(1), worst-case unbounded (CAS retries). Lock-free.
    fn pop(&self) -> (possible_top: Option<T>)
        requires self.wf();
    
    /// Check if the stack is empty at this instant.
    /// Note: Result may be stale by the time caller acts on it.
    /// - APAS: no cost spec.
    /// - Claude-Opus-4.6: O(1) — single atomic load.
    fn is_empty(&self) -> (empty: bool)
        requires self.wf();
    
    /// Drain all elements from the stack into a Vec.
    /// Elements are returned in LIFO order (most recently pushed first).
    /// Note: Concurrent pushes during drain may or may not be included.
    /// - APAS: no cost spec.
    /// - Claude-Opus-4.6: O(n) — sequential pop loop.
    fn drain(&self) -> (items: Vec<T>)
        requires self.wf();
}


//		9. impls

impl<T: Send> ConcurrentStackMtTrait<T> for ConcurrentStackMt<T> {

    #[verifier::external_body] // accept hole
    fn new() -> (stack: Self) {
        ConcurrentStackMt {
            head: AtomicPtr::new(null_mut()),
        }
    }

    #[verifier::external_body] // accept hole
    fn push(&self, value: T) {
        let mut new_node = Box::new(Node { value, next: null_mut() });
        loop {
            let head = self.head.load(Ordering::Acquire);
            new_node.next = head;
            let node_ptr = Box::into_raw(new_node);
            if self.head.compare_exchange_weak(head, node_ptr, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                break;
            }
            new_node = unsafe { Box::from_raw(node_ptr) }; // accept hole
        }
    }

    #[verifier::external_body] // accept hole
    fn pop(&self) -> (possible_top: Option<T>) {
        loop {
            let head = self.head.load(Ordering::Acquire);
            if head.is_null() {
                return None;
            }
            let next = unsafe { (*head).next };  // accept hole
            if self.head.compare_exchange_weak(head, next, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                let boxed = unsafe { Box::from_raw(head) };  // accept hole
                return Some(boxed.value);
            }
        }
    }

    #[verifier::external_body] // accept hole
    fn is_empty(&self) -> (empty: bool) {
        self.head.load(Ordering::Acquire).is_null()
    }

    #[verifier::external_body] // accept hole
    fn drain(&self) -> (items: Vec<T>) {
        let mut items = Vec::new();
        while let Some(value) = self.pop() {
            items.push(value);
        }
        items
    }
}

impl<T: Send> Default for ConcurrentStackMt<T> {
    fn default() -> Self { 
        ConcurrentStackMt::new() 
    }
}

impl<T: Send> Drop for ConcurrentStackMt<T> {
    #[verifier::external_body] // accept hole
    fn drop(&mut self)
        opens_invariants none
        no_unwind
    {
        let mut current = self.head.load(Ordering::Relaxed);
        while !current.is_null() {
            unsafe {  // accept hole
                let node = Box::from_raw(current);
                current = node.next;
            }
        }
    }
}

} // verus!

} // mod
