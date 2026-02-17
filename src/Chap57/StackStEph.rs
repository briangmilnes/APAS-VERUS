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

    use crate::Types::Types::*;

    /// Trait for stack operations
    pub trait StackStEphTrait<T: StT> {
        /// Create new empty stack
        /// APAS: Work Θ(1), Span Θ(1)
        fn new()           -> Self;

        /// Push element onto stack
        /// APAS: Work Θ(1), Span Θ(1)
        fn push(&mut self, item: T);

        /// Pop element from stack
        /// APAS: Work Θ(1), Span Θ(1)
        fn pop(&mut self)  -> Option<T>;

        /// Check if stack is empty
        /// APAS: Work Θ(1), Span Θ(1)
        fn is_empty(&self) -> B;
    }

    #[derive(Debug, Clone)]
    pub struct StackStEph<T: StT> {
        /// Backing storage using Vec for efficient push/pop
        elements: Vec<T>,
    }

    impl<T: StT> StackStEph<T> {
        /// Creates a new empty stack
        /// Work O(1), Span O(1)
        pub fn new() -> Self { StackStEph { elements: Vec::new() } }

        /// Pushes an item onto the stack
        /// Work O(1) amortized, Span O(1) amortized
        pub fn push(&mut self, item: T) { self.elements.push(item); }

        /// Pops and returns the top item from the stack
        /// Returns None if the stack is empty
        /// Work O(1) amortized, Span O(1) amortized
        pub fn pop(&mut self) -> Option<T> { self.elements.pop() }

        /// Returns a reference to the top item without removing it
        /// Returns None if the stack is empty
        /// Work O(1), Span O(1)
        pub fn peek(&self) -> Option<&T> { self.elements.last() }

        /// Checks if the stack is empty
        /// Work O(1), Span O(1)
        pub fn is_empty(&self) -> bool { self.elements.is_empty() }

        /// Returns the number of elements in the stack
        /// Work O(1), Span O(1)
        pub fn size(&self) -> N { self.elements.len() }
    }

    impl<T: StT> Default for StackStEph<T> {
        fn default() -> Self { Self::new() }
    }
}
