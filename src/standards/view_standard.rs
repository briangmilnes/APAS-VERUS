//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! View Standard: how to implement vstd's View trait in APAS-VERUS.
//!
//! View maps a concrete (exec-mode) type to an abstract (spec-mode) type,
//! accessed via the `@` operator. Every APAS-VERUS data structure implements
//! View so that specs can reason about abstract values.
//!
//! This file shows two patterns:
//! - Pattern A: Simple struct (single field projection).
//! - Pattern B: Generic collection (Vec<T> to Seq<T::V>).
//!
//! Reference: src/Chap18/ArraySeqStEph.rs section 5.
// 1. module
pub mod view_standard {

    use std::fmt::{Debug, Display, Formatter};

    use vstd::prelude::*;

    verus! {

    // 4. type definitions

    // Pattern A: Simple Struct
    //
    // When a struct has a single meaningful field, View projects to it directly.
    // The View type matches the field type (or its spec equivalent).
    pub struct SimpleS {
        pub val: u64,
    }

    // Pattern B: Generic Collection
    //
    // When a struct wraps Vec<T> and T itself implements View, the collection's
    // View maps each element through T's View. This gives Seq<T::V> — a
    // sequence of abstract element values.
    //
    // Key pieces:
    // - Bound: impl<T: View> View for CollectionS<T>
    // - View type: Seq<T::V>
    // - Body: self.seq@.map(|_i: int, t: T| t@)
    // - Annotation: #[verifier::reject_recursive_types(T)] on the struct
    #[verifier::reject_recursive_types(T)]
    pub struct CollectionS<T> {
        pub seq: Vec<T>,
    }

    #[verifier::reject_recursive_types(T)]
    pub struct CollectionIter<'a, T> {
        pub inner: std::slice::Iter<'a, T>,
    }

    #[verifier::reject_recursive_types(T)]
    pub struct CollectionGhostIterator<'a, T> {
        pub pos: int,
        pub elements: Seq<T>,
        pub phantom: core::marker::PhantomData<&'a T>,
    }

    // 5. view impls

    /// View for a simple single-field struct.
    /// self@ produces the u64 value directly.
    impl View for SimpleS {
        type V = u64;

        open spec fn view(&self) -> u64 {
            self.val
        }
    }

    /// View for a generic collection: maps Vec<T> to Seq<T::V>.
    /// Each element is mapped through its own View via t@.
    impl<T: View> View for CollectionS<T> {
        type V = Seq<T::V>;

        open spec fn view(&self) -> Seq<T::V> {
            self.seq@.map(|_i: int, t: T| t@)
        }
    }

    impl<'a, T> View for CollectionIter<'a, T> {
        type V = (int, Seq<T>);

        open spec fn view(&self) -> (int, Seq<T>) {
            self.inner@
        }
    }

    impl<'a, T> View for CollectionGhostIterator<'a, T> {
        type V = Seq<T>;

        open spec fn view(&self) -> Seq<T> {
            self.elements.take(self.pos)
        }
    }

    // 6. spec fns

    pub open spec fn iter_invariant<'a, T>(it: &CollectionIter<'a, T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    // 8. traits

    pub trait SimpleTrait: Sized {
        spec fn spec_val(&self) -> u64;

        fn new(v: u64) -> (s: Self)
            ensures
                s.spec_val() == v,
        ;

        fn get(&self) -> (v: u64)
            ensures
                v == self.spec_val(),
        ;
    }

    pub trait CollectionTrait<T>: Sized {
        spec fn spec_len(&self) -> nat;

        spec fn spec_index(&self, i: int) -> T
            recommends
                i < self.spec_len(),
        ;

        fn length(&self) -> (len: usize)
            ensures
                len as nat == self.spec_len(),
        ;

        fn nth(&self, i: usize) -> (val: &T)
            requires
                i < self.spec_len(),
            ensures
                *val == self.spec_index(i as int),
        ;
    }

    // 9. impls

    impl SimpleTrait for SimpleS {
        open spec fn spec_val(&self) -> u64 {
            self@
        }

        fn new(v: u64) -> (s: Self) {
            SimpleS { val: v }
        }

        fn get(&self) -> (v: u64) {
            self.val
        }
    }

    impl<T> CollectionTrait<T> for CollectionS<T> {
        open spec fn spec_len(&self) -> nat {
            self.seq@.len()
        }

        open spec fn spec_index(&self, i: int) -> T {
            self.seq@[i]
        }

        fn length(&self) -> (len: usize) {
            self.seq.len()
        }

        fn nth(&self, i: usize) -> (val: &T) {
            &self.seq[i]
        }
    }

    impl<T> CollectionS<T> {
        pub fn iter(&self) -> (it: CollectionIter<'_, T>)
            ensures
                it@.0 == 0,
                it@.1 == self.seq@,
                iter_invariant(&it),
        {
            CollectionIter { inner: self.seq.iter() }
        }
    }

    // 10. iterators

    impl<'a, T> std::iter::Iterator for CollectionIter<'a, T> {
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

    impl<'a, T> vstd::pervasive::ForLoopGhostIteratorNew for CollectionIter<'a, T> {
        type GhostIter = CollectionGhostIterator<'a, T>;

        open spec fn ghost_iter(&self) -> CollectionGhostIterator<'a, T> {
            CollectionGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    impl<'a, T> vstd::pervasive::ForLoopGhostIterator for CollectionGhostIterator<'a, T> {
        type ExecIter = CollectionIter<'a, T>;
        type Item = T;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &CollectionIter<'a, T>) -> bool {
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

        open spec fn ghost_advance(&self, _exec_iter: &CollectionIter<'a, T>) -> CollectionGhostIterator<'a, T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, T> std::iter::IntoIterator for &'a CollectionS<T> {
        type Item = &'a T;

        type IntoIter = CollectionIter<'a, T>;

        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                it@.0 == 0,
                it@.1 == self.seq@,
                iter_invariant(&it),
        {
            CollectionIter { inner: self.seq.iter() }
        }
    }

    impl<T> std::iter::IntoIterator for CollectionS<T> {
        type Item = T;

        type IntoIter = std::vec::IntoIter<T>;

        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                it@.0 == 0,
                it@.1 == self.seq@,
        {
            self.seq.into_iter()
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    impl Debug for SimpleS {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "SimpleS({})", self.val)
        }
    }

    impl Display for SimpleS {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "{}", self.val)
        }
    }

    impl<T: Debug> Debug for CollectionS<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "CollectionS({:?})", self.seq)
        }
    }

    impl<T: Display> Display for CollectionS<T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "[")?;
            for (i, item) in self.seq.iter().enumerate() {
                if i > 0 { write!(f, ", ")?; }
                write!(f, "{}", item)?;
            }
            write!(f, "]")
        }
    }
    impl<'a, T: Debug> Debug for CollectionIter<'a, T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "CollectionIter({:?})", self.inner)
        }
    }

    impl<'a, T> Display for CollectionIter<'a, T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "CollectionIter")
        }
    }

    impl<'a, T> Debug for CollectionGhostIterator<'a, T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "CollectionGhostIterator")
        }
    }

    impl<'a, T> Display for CollectionGhostIterator<'a, T> {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(f, "CollectionGhostIterator")
        }
    }
} // pub mod view_standard
