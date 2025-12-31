//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 12 — Exercise 12.5: lock-free concurrent stack using compare-and-swap.
//!
//! Note: Concurrent data structures cannot have simple sequential specs because
//! state changes concurrently. This is external_body with claimed postconditions.

pub mod Exercise12_5 {
    use vstd::prelude::*;
    use std::ptr::null_mut;
    use std::sync::atomic::{AtomicPtr, Ordering};

verus! {

    // Entire struct is external - Verus doesn't support AtomicPtr or raw pointers
    #[verifier::external_body]
    #[verifier::reject_recursive_types(T)]
    pub struct ConcurrentStackMt<T: Send> {
        head: AtomicPtr<Node<T>>,
    }

    // Node is also external
    #[verifier::external]
    struct Node<T> {
        value: T,
        next: *mut Node<T>,
    }

    impl<T: Send> ConcurrentStackMt<T> {
        #[verifier::external_body]
        pub fn new() -> Self {
            ConcurrentStackMt {
                head: AtomicPtr::new(null_mut()),
            }
        }

        #[verifier::external_body]
        pub fn push(&self, value: T) {
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
        pub fn pop(&self) -> Option<T> {
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
        pub fn is_empty(&self) -> bool {
            self.head.load(Ordering::Acquire).is_null()
        }

        #[verifier::external_body]
        pub fn drain(&self) -> Vec<T> {
            let mut items = Vec::new();
            while let Some(value) = self.pop() {
                items.push(value);
            }
            items
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

impl<T: Send> Default for ConcurrentStackMt<T> {
    fn default() -> Self { ConcurrentStackMt::new() }
}

} // mod
