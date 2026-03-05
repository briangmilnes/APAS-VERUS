//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Wrapping Iterators Standard: how to wrap a lower-level iterator.
//!
//! When a module wraps another module's collection (e.g., MappingStEph wraps
//! RelationStEph), the wrapper's iterator wraps the inner iterator. Both
//! `next()` and `View` delegate through `self.inner`.
//!
//! This file shows the pattern with two layers:
//! - InnerS / InnerIter: a Vec-backed collection (the foundation).
//! - OuterS / OuterIter: wraps InnerS, delegates iteration.
//!
//! The key insight: OuterIter's `next()` just calls `self.inner.next()`,
//! and OuterIter's View delegates to `self.inner@`. No new iteration
//! logic is needed — only the type wrapper and its View.
//!
//! References:
//! - src/Chap05/MappingStEph.rs (wraps RelationStEphIter)
//! - src/Chap05/RelationStEph.rs (wraps SetStEphIter)

//  Table of Contents
//	1. module
//	4. type definitions
//	5. view impls
//	8. traits
//	9. impls
//	10. iterators
//	13. derive impls outside verus!

//		1. module

pub mod wrapping_iterators_standard {

    use std::fmt::{Debug, Display, Formatter};

    use vstd::prelude::*;

    verus! {

    //		4. type definitions

    // Inner layer: a Vec-backed collection (like SetStEph or ArraySeqStEph).
    #[verifier::reject_recursive_types(T)]
    pub struct InnerS<T> {
        pub seq: Vec<T>,
    }

    // Outer layer: wraps InnerS (like MappingStEph wraps RelationStEph).
    #[verifier::reject_recursive_types(T)]
    pub struct OuterS<T> {
        pub data: InnerS<T>,
    }


    //		5. view impls

    impl<T> View for InnerS<T> {
        type V = Seq<T>;

        open spec fn view(&self) -> Seq<T> {
            self.seq@
        }
    }

    /// OuterS View delegates to the inner collection.
    impl<T> View for OuterS<T> {
        type V = Seq<T>;

        open spec fn view(&self) -> Seq<T> {
            self.data@
        }
    }


    //		8. traits

    pub trait InnerTrait<T>: Sized {
        spec fn spec_len(&self) -> nat;

        fn new(length: usize, init: T) -> (s: Self) where T: Copy
            ensures
                s.spec_len() == length as nat,
        ;
    }

    pub trait OuterTrait<T>: Sized {
        spec fn spec_len(&self) -> nat;

        fn new(length: usize, init: T) -> (s: Self) where T: Copy
            ensures
                s.spec_len() == length as nat,
        ;
    }


    //		9. impls

    impl<T> InnerTrait<T> for InnerS<T> {
        open spec fn spec_len(&self) -> nat {
            self@.len()
        }

        fn new(length: usize, init: T) -> (s: Self) where T: Copy {
            let mut v: Vec<T> = Vec::new();
            let mut i: usize = 0;
            while i < length
                invariant
                    i <= length,
                    v@.len() == i as int,
                decreases length - i,
            {
                v.push(init);
                i += 1;
            }
            InnerS { seq: v }
        }
    }

    impl<T> OuterTrait<T> for OuterS<T> {
        open spec fn spec_len(&self) -> nat {
            self@.len()
        }

        fn new(length: usize, init: T) -> (s: Self) where T: Copy {
            OuterS { data: InnerS::new(length, init) }
        }
    }

    impl<T> InnerS<T> {
        pub fn iter(&self) -> (it: InnerIter<'_, T>)
            ensures
                it@.0 == 0,
                it@.1 == self.seq@,
        {
            InnerIter { inner: self.seq.iter() }
        }
    }

    /// OuterS::iter() wraps the inner iterator.
    impl<T> OuterS<T> {
        pub fn iter(&self) -> (it: OuterIter<'_, T>)
            ensures
                it@.0 == 0,
                it@.1 == self@,
        {
            OuterIter { inner: self.data.iter() }
        }
    }


    //		10. iterators

    #[verifier::reject_recursive_types(T)]
    pub struct InnerIter<'a, T> {
        pub inner: std::slice::Iter<'a, T>,
    }

    #[verifier::reject_recursive_types(T)]
    pub struct InnerGhostIterator<'a, T> {
        pub pos: int,
        pub elements: Seq<T>,
        pub phantom: core::marker::PhantomData<&'a T>,
    }

    // OuterIter wraps InnerIter — the key pattern.
    #[verifier::reject_recursive_types(T)]
    pub struct OuterIter<'a, T> {
        pub inner: InnerIter<'a, T>,
    }

    #[verifier::reject_recursive_types(T)]
    pub struct OuterGhostIterator<'a, T> {
        pub pos: int,
        pub elements: Seq<T>,
        pub phantom: core::marker::PhantomData<&'a T>,
    }

    impl<'a, T> View for InnerIter<'a, T> {
        type V = (int, Seq<T>);

        open spec fn view(&self) -> (int, Seq<T>) {
            self.inner@
        }
    }

    impl<'a, T> View for InnerGhostIterator<'a, T> {
        type V = Seq<T>;

        open spec fn view(&self) -> Seq<T> {
            self.elements.take(self.pos)
        }
    }

    /// OuterIter View delegates to self.inner@ — the wrapping pattern.
    impl<'a, T> View for OuterIter<'a, T> {
        type V = (int, Seq<T>);

        open spec fn view(&self) -> (int, Seq<T>) {
            self.inner@
        }
    }

    impl<'a, T> View for OuterGhostIterator<'a, T> {
        type V = Seq<T>;

        open spec fn view(&self) -> Seq<T> {
            self.elements.take(self.pos)
        }
    }

    // Inner layer iterators (foundation — wraps std::slice::Iter).

    impl<'a, T> std::iter::Iterator for InnerIter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> (next: Option<&'a T>)
            ensures
                ({
                    let (old_index, old_seq) = old(self)@;
                    match next {
                        None => {
                            &&& self@ == old(self)@
                            &&& old_index >= old_seq.len()
                        },
                        Some(element) => {
                            let (new_index, new_seq) = self@;
                            &&& 0 <= old_index < old_seq.len()
                            &&& new_seq == old_seq
                            &&& new_index == old_index + 1
                            &&& element == old_seq[old_index]
                        },
                    }
                }),
        {
            self.inner.next()
        }
    }

    impl<'a, T> vstd::pervasive::ForLoopGhostIteratorNew for InnerIter<'a, T> {
        type GhostIter = InnerGhostIterator<'a, T>;

        open spec fn ghost_iter(&self) -> InnerGhostIterator<'a, T> {
            InnerGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    impl<'a, T> vstd::pervasive::ForLoopGhostIterator for InnerGhostIterator<'a, T> {
        type ExecIter = InnerIter<'a, T>;
        type Item = T;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &InnerIter<'a, T>) -> bool {
            &&& self.pos == exec_iter@.0
            &&& self.elements == exec_iter@.1
        }

        open spec fn ghost_invariant(&self, init: Option<&Self>) -> bool {
            init matches Some(init) ==> {
                &&& init.pos == 0
                &&& init.elements == self.elements
                &&& 0 <= self.pos <= self.elements.len()
            }
        }

        open spec fn ghost_ensures(&self) -> bool {
            self.pos == self.elements.len()
        }

        open spec fn ghost_decrease(&self) -> Option<int> {
            Some(self.elements.len() - self.pos)
        }

        open spec fn ghost_peek_next(&self) -> Option<T> {
            if 0 <= self.pos < self.elements.len() {
                Some(self.elements[self.pos])
            } else {
                None
            }
        }

        open spec fn ghost_advance(&self, _exec_iter: &InnerIter<'a, T>) -> InnerGhostIterator<'a, T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, T> std::iter::IntoIterator for &'a InnerS<T> {
        type Item = &'a T;
        type IntoIter = InnerIter<'a, T>;

        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                it@.0 == 0,
                it@.1 == self.seq@,
        {
            InnerIter { inner: self.seq.iter() }
        }
    }

    // Outer layer iterators (wrapper — delegates to InnerIter).

    /// OuterIter::next() delegates to self.inner.next() — the wrapping pattern.
    impl<'a, T> std::iter::Iterator for OuterIter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> (next: Option<&'a T>)
            ensures
                ({
                    let (old_index, old_seq) = old(self)@;
                    match next {
                        None => {
                            &&& self@ == old(self)@
                            &&& old_index >= old_seq.len()
                        },
                        Some(element) => {
                            let (new_index, new_seq) = self@;
                            &&& 0 <= old_index < old_seq.len()
                            &&& new_seq == old_seq
                            &&& new_index == old_index + 1
                            &&& element == old_seq[old_index]
                        },
                    }
                }),
        {
            self.inner.next()
        }
    }

    impl<'a, T> vstd::pervasive::ForLoopGhostIteratorNew for OuterIter<'a, T> {
        type GhostIter = OuterGhostIterator<'a, T>;

        open spec fn ghost_iter(&self) -> OuterGhostIterator<'a, T> {
            OuterGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    impl<'a, T> vstd::pervasive::ForLoopGhostIterator for OuterGhostIterator<'a, T> {
        type ExecIter = OuterIter<'a, T>;
        type Item = T;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &OuterIter<'a, T>) -> bool {
            &&& self.pos == exec_iter@.0
            &&& self.elements == exec_iter@.1
        }

        open spec fn ghost_invariant(&self, init: Option<&Self>) -> bool {
            init matches Some(init) ==> {
                &&& init.pos == 0
                &&& init.elements == self.elements
                &&& 0 <= self.pos <= self.elements.len()
            }
        }

        open spec fn ghost_ensures(&self) -> bool {
            self.pos == self.elements.len()
        }

        open spec fn ghost_decrease(&self) -> Option<int> {
            Some(self.elements.len() - self.pos)
        }

        open spec fn ghost_peek_next(&self) -> Option<T> {
            if 0 <= self.pos < self.elements.len() {
                Some(self.elements[self.pos])
            } else {
                None
            }
        }

        open spec fn ghost_advance(&self, _exec_iter: &OuterIter<'a, T>) -> OuterGhostIterator<'a, T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, T> std::iter::IntoIterator for &'a OuterS<T> {
        type Item = &'a T;
        type IntoIter = OuterIter<'a, T>;

        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                it@.0 == 0,
                it@.1 == self@,
        {
            OuterIter { inner: self.data.iter() }
        }
    }

    } // verus!

    //		13. derive impls outside verus!

    impl<T: Debug> Debug for InnerS<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "InnerS({:?})", self.seq)
        }
    }

    impl<T: Display> Display for InnerS<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "[")?;
            for (i, item) in self.seq.iter().enumerate() {
                if i > 0 { write!(f, ", ")?; }
                write!(f, "{}", item)?;
            }
            write!(f, "]")
        }
    }

    impl<T: Debug> Debug for OuterS<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "OuterS({:?})", self.data)
        }
    }

    impl<T: Display> Display for OuterS<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "{}", self.data)
        }
    }

    impl<'a, T: Debug> Debug for InnerIter<'a, T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "InnerIter({:?})", self.inner)
        }
    }

    impl<'a, T> Display for InnerIter<'a, T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "InnerIter")
        }
    }

    impl<'a, T> Debug for InnerGhostIterator<'a, T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "InnerGhostIterator")
        }
    }

    impl<'a, T> Display for InnerGhostIterator<'a, T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "InnerGhostIterator")
        }
    }

    impl<'a, T: Debug> Debug for OuterIter<'a, T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "OuterIter({:?})", self.inner)
        }
    }

    impl<'a, T> Display for OuterIter<'a, T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "OuterIter")
        }
    }

    impl<'a, T> Debug for OuterGhostIterator<'a, T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "OuterGhostIterator")
        }
    }

    impl<'a, T> Display for OuterGhostIterator<'a, T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "OuterGhostIterator")
        }
    }
} // pub mod wrapping_iterators_standard
