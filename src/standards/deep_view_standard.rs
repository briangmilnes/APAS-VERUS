// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! DeepView Standard: how to implement vstd's DeepView trait in APAS-VERUS.
//!
//! DeepView recursively maps through nested container types. While View maps
//! Vec<T> to Seq<T> (keeping T concrete), DeepView maps Vec<T> to Seq<T::V>
//! (recursively abstracting elements).
//!
//! This file shows three patterns:
//! - Pattern A: Simple struct (identity — same as View).
//! - Pattern B: Generic collection (recursive deep_view on elements).
//! - Pattern C: Tuple deep_view (vstd built-in).
//!
//! References:
//! - src/experiments/deep_view_struct.rs
//! - src/experiments/deep_view_2_tuple.rs
//! - src/Chap18/ArraySeq.rs
// 1. module
pub mod deep_view_standard {

    use std::fmt::{Debug, Display, Formatter};

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::iter::*;

    verus! {

    // 4. type definitions

    // Pattern A: Simple Struct
    //
    // For non-generic structs with primitive fields, DeepView is identical to View.
    // Both project to the same type.
    pub struct SimpleS {
        pub val: Option<usize>,
    }

    // Pattern B: Generic Collection
    //
    // For a generic collection wrapping Vec<T>, View and DeepView differ:
    // - View:     type V = Seq<T>       (elements stay concrete)
    // - DeepView: type V = Seq<T::V>    (elements recursively abstracted)
    //
    // The body uses Seq::new with deep_view() on each element.
    #[verifier::reject_recursive_types(T)]
    pub struct CollectionS<T> {
        pub seq: Vec<T>,
    }

    // 5. view impls

    /// View and DeepView both project to the same type for simple structs.
    impl View for SimpleS {
        type V = Option<usize>;

        open spec fn view(&self) -> Option<usize> {
            self.val
        }
    }

    /// View keeps elements concrete: Seq<T>.
    impl<T: View> View for CollectionS<T> {
        type V = Seq<T::V>;

        open spec fn view(&self) -> Seq<T::V> {
            self.seq@.map(|_i: int, t: T| t@)
        }
    }

    // 7. proof fns

    // Pattern C: Tuple DeepView
    //
    // vstd provides built-in DeepView for tuples. For Vec<(K, Vec<V>)>,
    // deep_view() produces Seq<(K::V, Seq<V::V>)> automatically.
    // No custom implementation needed — just use it in specs.

    /// Prove that deep_view preserves length (key bridge lemma).
    proof fn lemma_deep_view_len<T: DeepView>(v: &Vec<T>)
        ensures
            v.deep_view().len() == v@.len(),
    {
    }

    /// Prove that tuple deep_view works on Vec<(u32, Vec<u32>)>.
    proof fn test_tuple_deep_view(v: Vec<(u32, Vec<u32>)>)
        ensures
            v.deep_view().len() == v@.len(),
    {
    }

    // 9. impls

    impl DeepView for SimpleS {
        type V = Option<usize>;

        open spec fn deep_view(&self) -> Option<usize> {
            self.val
        }
    }

    /// DeepView recursively abstracts elements: Seq<T::V> where V = T's DeepView.
    impl<T: DeepView> DeepView for CollectionS<T> {
        type V = Seq<T::V>;

        open spec fn deep_view(&self) -> Seq<T::V> {
            let v = self.seq@;
            Seq::new(v.len(), |i: int| v[i].deep_view())
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
            write!(f, "SimpleS({:?})", self.val)
        }
    }

    impl Display for SimpleS {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            match self.val {
                None => write!(f, "None"),
                Some(v) => write!(f, "{}", v),
            }
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
} // pub mod deep_view_standard
