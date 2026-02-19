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

    pub trait StackStEphTrait<T: StT>: Sized {
        fn new() -> (result: Self);

        fn push(&mut self, item: T);

        fn pop(&mut self) -> (result: Option<T>);

        fn peek(&self) -> (result: Option<&T>);

        fn is_empty(&self) -> (result: bool);

        fn size(&self) -> (result: N);
    }

    // 9. impls

    impl<T: StT> StackStEphTrait<T> for StackStEph<T> {
        fn new() -> (result: Self)
            ensures result@ == Seq::<T>::empty(),
        { StackStEph { elements: Vec::new() } }

        fn push(&mut self, item: T)
            ensures self@ == old(self)@.push(item),
        { self.elements.push(item); }

        fn pop(&mut self) -> (result: Option<T>)
            ensures
                old(self)@.len() > 0 ==> result == Some(old(self)@.last()) && self@ == old(self)@.drop_last(),
                old(self)@.len() == 0 ==> (result matches Option::None),
                old(self)@.len() == 0 ==> self@ == old(self)@,
        { self.elements.pop() }

        fn peek(&self) -> (result: Option<&T>)
            ensures
                self@.len() > 0 ==> (result matches Option::Some(_)),
                self@.len() == 0 ==> (result matches Option::None),
        { self.elements.last() }

        fn is_empty(&self) -> (result: bool)
            ensures result == (self@.len() == 0),
        { self.elements.is_empty() }

        fn size(&self) -> (result: N)
            ensures result == self@.len(),
        { self.elements.len() }
    }

    // 11. derive impls in verus!

    impl<T: StT> Default for StackStEph<T> {
        fn default() -> (result: Self)
            ensures result@ == Seq::<T>::empty(),
        { Self::new() }
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
