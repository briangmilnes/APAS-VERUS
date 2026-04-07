//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
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


//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 5. view impls
//	Section 8. traits
//	Section 9. impls
//	Section 12. derive impls in verus!
//	Section 14. derive impls outside verus!

//		Section 1. module

pub mod StackStEph {


    //		Section 2. imports

    use vstd::prelude::*;
    use crate::Types::Types::*;

    verus! 
{

    //		Section 3. broadcast use


    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::clone::*;

broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
    vstd::seq_lib::group_to_multiset_ensures,
};

    //		Section 4. type definitions


    pub struct StackStEph<T: StT> {
        pub elements: Vec<T>,
    }

    //		Section 5. view impls


    impl<T: StT> View for StackStEph<T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> {
            self.elements@
        }
    }

    //		Section 8. traits


    pub trait StackStEphTrait<T: StT>: View<V = Seq<T>> + Sized {
        spec fn spec_stacksteph_wf(&self) -> bool;

        /// - Alg Analysis: APAS: (no cost stated) — standard stack.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — no APAS cost spec
        fn new() -> (empty: Self)
            ensures empty.spec_stacksteph_wf();

        /// - Alg Analysis: APAS: (no cost stated) — standard stack.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1) amortized, Span O(1) amortized — no APAS cost spec
        fn push(&mut self, item: T)
            requires
                old(self).spec_stacksteph_wf(),
                old(self)@.len() < usize::MAX as int,
            ensures self.spec_stacksteph_wf();

        /// - Alg Analysis: APAS: (no cost stated) — standard stack.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1) amortized, Span O(1) amortized — no APAS cost spec
        fn pop(&mut self) -> (popped: Option<T>)
            requires old(self).spec_stacksteph_wf()
            ensures self.spec_stacksteph_wf();

        /// - Alg Analysis: APAS: (no cost stated) — standard stack.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — no APAS cost spec
        fn peek(&self) -> (top: Option<&T>)
            requires self.spec_stacksteph_wf();

        /// - Alg Analysis: APAS: (no cost stated) — standard stack.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — no APAS cost spec
        fn is_empty(&self) -> (is_empty: bool)
            requires self.spec_stacksteph_wf();

        /// - Alg Analysis: APAS: (no cost stated) — standard stack.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — no APAS cost spec
        fn size(&self) -> (count: usize)
            requires self.spec_stacksteph_wf();
    }

    //		Section 9. impls


    impl<T: StT> StackStEphTrait<T> for StackStEph<T> {
        open spec fn spec_stacksteph_wf(&self) -> bool {
            self@.len() <= usize::MAX as int
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — Vec allocation.
        fn new() -> (empty: Self)
            ensures empty@ == Seq::<T>::empty(),
        { StackStEph { elements: Vec::new() } }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1) amortized, Span O(1) amortized — Vec push.
        fn push(&mut self, item: T)
            ensures self@ == old(self)@.push(item),
        { self.elements.push(item); }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — Vec pop.
        fn pop(&mut self) -> (popped: Option<T>)
            ensures
                old(self)@.len() > 0 ==> popped == Some(old(self)@.last()) && self@ == old(self)@.drop_last(),
                old(self)@.len() == 0 ==> (popped matches Option::None),
                old(self)@.len() == 0 ==> self@ == old(self)@,
        { self.elements.pop() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — Vec last.
        fn peek(&self) -> (top: Option<&T>)
            ensures
                self@.len() > 0 ==> (top matches Option::Some(_)),
                self@.len() == 0 ==> (top matches Option::None),
        { self.elements.last() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — Vec is_empty.
        fn is_empty(&self) -> (is_empty: bool)
            ensures is_empty == (self@.len() == 0),
        { self.elements.is_empty() }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — Vec len.
        fn size(&self) -> (count: usize)
            ensures count == self@.len(),
        { self.elements.len() }
    }

    //		Section 12. derive impls in verus!


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

    //		Section 14. derive impls outside verus!


    impl<T: StT> std::fmt::Debug for StackStEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("StackStEph")
                .field("elements", &self.elements)
                .finish()
        }
    }

    impl<T: StT + std::fmt::Display> std::fmt::Display for StackStEph<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "StackStEph(len: {})", self.elements.len())
        }
    }
}
