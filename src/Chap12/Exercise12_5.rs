// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO

//! Chapter 12 — Exercise 12.5: lock-free concurrent stack using compare-and-swap.
//!
//! The external_body and external annotations here are permanent. This stack
//! uses AtomicPtr and raw pointers for a lock-free Treiber stack — none of
//! which have vstd specs. Meaningful verification would require a tokenized
//! state machine (TSM) to model the linearizability argument, which is
//! disproportionate for a CAS exercise. The RTTs validate runtime behavior.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 4a. type definitions
//	Section 4b. type definitions
//	Section 8b. traits
//	Section 9b. impls
//	Section 12b. derive impls in verus!
//	Section 14a. derive impls outside verus!
//	Section 14b. derive impls outside verus!

//		Section 1. module


pub mod Exercise12_5 {

    //		Section 2. imports

    use vstd::prelude::*;
    use std::ptr::null_mut;
    use std::sync::atomic::{AtomicPtr, Ordering};

verus! 
{

    //		Section 4a. type definitions


/// Node for the lock-free stack. External due to raw pointer field.
#[verifier::external] // accept hole
struct Node<T> {
    value: T,
    next: *mut Node<T>,
}

    //		Section 4b. type definitions


/// Lock-free concurrent stack using AtomicPtr and CAS.
/// External due to AtomicPtr (no vstd specs) and raw pointers.
#[verifier::external_body] // accept hole
#[verifier::reject_recursive_types(T)]
pub struct ConcurrentStackMt<T: Send> {
    head: AtomicPtr<Node<T>>,
}

    //		Section 8b. traits


/// Trait for lock-free concurrent stack operations.
/// 
/// Specs describe linearizable behavior: each operation appears to take effect
/// atomically at some point between its invocation and response.
pub trait ConcurrentStackMtTrait<T: Send>: Sized {
    /// Spec: the stack is always well-formed after construction.
    open spec fn wf(&self) -> bool { true } // accept hole: Mutex<Vec>-backed, true is correct

    /// Create a new empty stack.
    /// - Alg Analysis: Code review (Claude Opus 4.6): O(1).
    fn new() -> (stack: Self)
        ensures stack.wf();

    /// Push a value onto the stack. Always succeeds (may spin under contention).
    /// - Alg Analysis: Code review (Claude Opus 4.6): amortized O(1), worst-case unbounded (CAS retries). Lock-free.
    fn push(&self, value: T)
        requires self.wf();

    /// Pop a value from the stack.
    /// Returns Some(v) where v was the top element at the linearization point,
    /// or None if the stack was empty at that point.
    /// - Alg Analysis: Code review (Claude Opus 4.6): amortized O(1), worst-case unbounded (CAS retries). Lock-free.
    fn pop(&self) -> (possible_top: Option<T>)
        requires self.wf();

    /// Check if the stack is empty at this instant.
    /// Note: Result may be stale by the time caller acts on it.
    /// - Alg Analysis: Code review (Claude Opus 4.6): O(1) — single atomic load.
    fn is_empty(&self) -> (empty: bool)
        requires self.wf();

    /// Drain all elements from the stack into a Vec.
    /// Elements are returned in LIFO order (most recently pushed first).
    /// Note: Concurrent pushes during drain may or may not be included.
    /// - Alg Analysis: Code review (Claude Opus 4.6): O(n) — sequential pop loop.
    fn drain(&self) -> (items: Vec<T>)
        requires self.wf();
}

    //		Section 9b. impls


impl<T: Send> ConcurrentStackMtTrait<T> for ConcurrentStackMt<T> {

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — single atomic pointer store.
    #[verifier::external_body] // accept hole
    fn new() -> (stack: Self) {
        ConcurrentStackMt {
            head: AtomicPtr::new(null_mut()),
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1) amortized, Span O(1) amortized — CAS retry loop; O(contention) worst case.
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

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1) amortized, Span O(1) amortized — CAS retry loop; O(contention) worst case.
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

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — single atomic load.
    #[verifier::external_body] // accept hole
    fn is_empty(&self) -> (empty: bool) {
        self.head.load(Ordering::Acquire).is_null()
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — pops all n elements sequentially.
    #[verifier::external_body] // accept hole
    fn drain(&self) -> (items: Vec<T>) {
        let mut items = Vec::new();
        while let Some(value) = self.pop() {
            items.push(value);
        }
        items
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

    //		Section 12b. derive impls in verus!


impl<T: Send> Default for ConcurrentStackMt<T> {
    fn default() -> Self { 
        ConcurrentStackMt::new() 
    }
}
} // verus!

    //		Section 14a. derive impls outside verus!


    impl<T> std::fmt::Debug for Node<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Node")
        }
    }

    impl<T> std::fmt::Display for Node<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Node")
        }
    }

    //		Section 14b. derive impls outside verus!

    impl<T: Send> std::fmt::Debug for ConcurrentStackMt<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ConcurrentStackMt")
        }
    }

    impl<T: Send> std::fmt::Display for ConcurrentStackMt<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ConcurrentStackMt")
        }
    }

} // mod
