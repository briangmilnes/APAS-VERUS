//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! Stack - Sequential Ephemeral implementation
//!
//! A stack is a Last-In-First-Out (LIFO) data structure.
//!
//! **Algorithmic Analysis:**
//! - `new`: Work O(1), Span O(1)
//! - `push`: Work O(1) amortized, Span O(1) amortized
//! - `pop`: Work O(1) amortized, Span O(1) amortized
//! - `peek`: Work O(1), Span O(1)
//! - `is_empty`: Work O(1), Span O(1)
//! - `size`: Work O(1), Span O(1)

pub mod StackStEph {

    use vstd::prelude::*;
    use crate::Types::Types::*;

    verus! {

    // Table of Contents
    // 1. module (StackStEph)
    // 2. imports
    // 4. type definitions
    // 5. view impls
    // 8. traits
    // 9. impls
    // 11. derive impls in verus!
    // 13. derive impls outside verus!

    // 4. type definitions

    #[derive(Clone)]
    pub struct StackStEph<T: StT> {
        /// Backing storage using Vec for efficient push/pop
        pub elements: Vec<T>,
    }

    // 5. view impls

    impl<T: StT> View for StackStEph<T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> {
            self.elements@
        }
    }

    // 8. traits

    /// Trait for stack operations
    pub trait StackStEphTrait<T: StT>: Sized {
        /// Create new empty stack
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        fn new() -> Self;

        /// Push element onto stack
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1) amortized, Span Θ(1) amortized — Vec::push amortized.
        fn push(&mut self, item: T);

        /// Pop element from stack
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1) amortized, Span Θ(1) amortized — Vec::pop amortized.
        fn pop(&mut self) -> Option<T>;

        /// Check if stack is empty
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        fn is_empty(&self) -> B;
    }

    // 9. impls

    impl<T: StT> StackStEph<T> {
        /// Creates a new empty stack.
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        #[verifier::external_body]
        pub fn new() -> Self { StackStEph { elements: Vec::new() } }

        /// Pushes an item onto the stack.
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1) amortized, Span Θ(1) amortized — Vec::push amortized.
        #[verifier::external_body]
        pub fn push(&mut self, item: T) { self.elements.push(item); }

        /// Pops and returns the top item from the stack.
        /// Returns None if the stack is empty.
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1) amortized, Span Θ(1) amortized — Vec::pop amortized.
        #[verifier::external_body]
        pub fn pop(&mut self) -> Option<T> { self.elements.pop() }

        /// Returns a reference to the top item without removing it.
        /// Returns None if the stack is empty.
        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — Vec::last.
        #[verifier::external_body]
        pub fn peek(&self) -> Option<&T> { self.elements.last() }

        /// Checks if the stack is empty.
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        #[verifier::external_body]
        pub fn is_empty(&self) -> bool { self.elements.is_empty() }

        /// Returns the number of elements in the stack.
        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — Vec::len.
        #[verifier::external_body]
        pub fn size(&self) -> N { self.elements.len() }
    }

    // 11. derive impls in verus!

    impl<T: StT> Default for StackStEph<T> {
        /// - APAS: N/A — Rust Default trait scaffolding.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — delegates to new().
        #[verifier::external_body]
        fn default() -> Self { Self::new() }
    }

    } // verus!

    // 13. derive impls outside verus!

    impl<T: StT> std::fmt::Debug for StackStEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("StackStEph")
                .field("elements", &self.elements)
                .finish()
        }
    }
}
