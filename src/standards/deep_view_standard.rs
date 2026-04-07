//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
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
} // pub mod deep_view_standard
