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

// Veracity: added broadcast group
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
    vstd::seq_lib::group_to_multiset_ensures,
};

    // Table of Contents
    // 1. module (StackStEph)
    // 2. imports
    // 4. type definitions
    // 5. view impls
    // 8. traits
    // 9. impls
    // 11. derive impls in verus!
    // 13. derive impls outside verus!

    // 2. imports

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::clone::*;

    // 4. type definitions

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

    pub trait StackStEphTrait<T: StT>: View<V = Seq<T>> + Sized {
        spec fn spec_stacksteph_wf(&self) -> bool;

        /// - APAS: (no cost stated) — standard stack.
        /// - Claude-Opus-4.6: Work O(1), Span O(1).
        fn new() -> (empty: Self)
            ensures empty.spec_stacksteph_wf();

        /// - APAS: (no cost stated) — standard stack.
        /// - Claude-Opus-4.6: Work O(1) amortized, Span O(1) amortized.
        fn push(&mut self, item: T)
            requires
                old(self).spec_stacksteph_wf(),
                old(self)@.len() < usize::MAX as int,
            ensures self.spec_stacksteph_wf();

        /// - APAS: (no cost stated) — standard stack.
        /// - Claude-Opus-4.6: Work O(1) amortized, Span O(1) amortized.
        fn pop(&mut self) -> (popped: Option<T>)
            requires old(self).spec_stacksteph_wf()
            ensures self.spec_stacksteph_wf();

        /// - APAS: (no cost stated) — standard stack.
        /// - Claude-Opus-4.6: Work O(1), Span O(1).
        fn peek(&self) -> (top: Option<&T>)
            requires self.spec_stacksteph_wf();

        /// - APAS: (no cost stated) — standard stack.
        /// - Claude-Opus-4.6: Work O(1), Span O(1).
        fn is_empty(&self) -> (is_empty: bool)
            requires self.spec_stacksteph_wf();

        /// - APAS: (no cost stated) — standard stack.
        /// - Claude-Opus-4.6: Work O(1), Span O(1).
        fn size(&self) -> (count: N)
            requires self.spec_stacksteph_wf();
    }

    // 9. impls

    impl<T: StT> StackStEphTrait<T> for StackStEph<T> {
        open spec fn spec_stacksteph_wf(&self) -> bool {
            self@.len() <= usize::MAX as int
        }

        fn new() -> (empty: Self)
            ensures empty@ == Seq::<T>::empty(),
        { StackStEph { elements: Vec::new() } }

        fn push(&mut self, item: T)
            ensures self@ == old(self)@.push(item),
        { self.elements.push(item); }

        fn pop(&mut self) -> (popped: Option<T>)
            ensures
                old(self)@.len() > 0 ==> popped == Some(old(self)@.last()) && self@ == old(self)@.drop_last(),
                old(self)@.len() == 0 ==> (popped matches Option::None),
                old(self)@.len() == 0 ==> self@ == old(self)@,
        { self.elements.pop() }

        fn peek(&self) -> (top: Option<&T>)
            ensures
                self@.len() > 0 ==> (top matches Option::Some(_)),
                self@.len() == 0 ==> (top matches Option::None),
        { self.elements.last() }

        fn is_empty(&self) -> (is_empty: bool)
            ensures is_empty == (self@.len() == 0),
        { self.elements.is_empty() }

        fn size(&self) -> (count: N)
            ensures count == self@.len(),
        { self.elements.len() }
    }

    // 11. derive impls in verus!

    impl<T: StT> Clone for StackStEph<T> {
        fn clone(&self) -> (res: Self)
            ensures
                res.elements@.len() == self.elements@.len(),
                forall|i: int| #![trigger res.elements@[i]]
                    0 <= i < self.elements@.len()
                        ==> cloned::<T>(self.elements@[i], res.elements@[i]),
        {
            StackStEph { elements: self.elements.clone() }
        }
    }

    impl<T: StT> Default for StackStEph<T> {
        fn default() -> (default_val: Self)
            ensures default_val@ == Seq::<T>::empty(),
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
