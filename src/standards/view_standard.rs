// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
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
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::iter::*;

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

    // Delegated iteration: the collection returns the std slice iterator that
    // vstd already specifies. Postconditions per iterators_standard.rs.
    impl<T> CollectionS<T> {
        pub fn iter(&self) -> (it: std::slice::Iter<'_, T>)
            ensures
                IteratorSpec::remaining(&it) == self.seq@.as_ref(),
                vstd::std_specs::slice::into_iter_elts(it) == self.seq@,
                IteratorSpec::decrease(&it) is Some,
        {
            self.seq.iter()
        }
    }

    // 10. iterators

    impl<'a, T> std::iter::IntoIterator for &'a CollectionS<T> {
        type Item = &'a T;

        type IntoIter = std::slice::Iter<'a, T>;

        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                IteratorSpec::remaining(&it) == self.seq@.as_ref(),
                vstd::std_specs::slice::into_iter_elts(it) == self.seq@,
                IteratorSpec::decrease(&it) is Some,
        {
            self.seq.iter()
        }
    }

    impl<T> std::iter::IntoIterator for CollectionS<T> {
        type Item = T;

        type IntoIter = std::vec::IntoIter<T>;

        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                IteratorSpec::remaining(&it) == self.seq@,
                vstd::std_specs::vec::into_iter_elts(it) == self.seq@,
                IteratorSpec::decrease(&it) is Some,
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
} // pub mod view_standard
