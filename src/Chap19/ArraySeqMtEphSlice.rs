// Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Chapter 19 slice-backed array sequence (multi-threaded ephemeral).
//! O(1) slicing via shared `Arc<Vec<T>>` backing with offset/length window.
//! Multiple slices can share the same backing storage; `slice()` and
//! `subseq_copy()` are O(1) (just an Arc ref-count bump + window adjust).
//! Iterators wrap vstd's `std::slice::Iter` following the iterator standard.

// Table of Contents
// 1. module
// 2. imports
// 3. broadcast use
// 4. type definitions
// 5. view impls
// 6. spec fns
// 8. traits
// 9. impls
// 10. iterators
// 13. derive impls outside verus!

// 1. module

pub mod ArraySeqMtEphSlice {

    use std::sync::Arc;

    use vstd::prelude::*;

    verus! {

    // 2. imports

    use crate::vstdplus::clone_plus::clone_plus::*;
    use crate::vstdplus::smart_ptrs::smart_ptrs::{arc_deref, arc_vec_as_slice};
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;

    // 3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
        vstd::seq_lib::group_seq_properties,
        vstd::std_specs::slice::group_slice_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms,
        // Veracity: added broadcast groups
        vstd::seq_lib::group_to_multiset_ensures,
    };

    // 4. type definitions

    #[verifier::reject_recursive_types(T)]
    pub struct ArraySeqMtEphSliceS<T> {
        pub data: Arc<Vec<T>>,
        pub start: usize,
        pub len: usize,
    }

    // 5. view impls

    impl<T: View> View for ArraySeqMtEphSliceS<T> {
        type V = Seq<T::V>;

        open spec fn view(&self) -> Seq<T::V> {
            (*self.data)@.map(|_i: int, t: T| t@)
                .subrange(self.start as int, (self.start + self.len) as int)
        }
    }

    // 6. spec fns

    // 8. traits

    pub trait ArraySeqMtEphSliceTrait<T: Eq + Clone>: Sized {
        spec fn spec_arrayseqmtephslice_wf(&self) -> bool;

        spec fn spec_len(&self) -> nat;

        spec fn spec_index(&self, i: int) -> T
            recommends self.spec_arrayseqmtephslice_wf(), i < self.spec_len();

        /// The raw backing subrange as a Seq<T> (not View-mapped).
        spec fn spec_backing_seq(&self) -> Seq<T>;

        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(1), Span O(1)
        /// - Alg Analysis: Claude-Opus-4.6 (1M): NONE
        fn length(&self) -> (len: usize)
            requires self.spec_arrayseqmtephslice_wf(),
            ensures len as int == self.spec_len();

        fn nth_cloned(&self, index: usize) -> (elem: T)
            requires
                self.spec_arrayseqmtephslice_wf(),
                index < self.spec_len(),
                obeys_feq_clone::<T>(),
            ensures elem == self.spec_index(index as int);

        /// O(1) slice: shares backing storage, adjusts window.
        fn slice(&self, start: usize, length: usize) -> (sliced: Self)
            requires
                self.spec_arrayseqmtephslice_wf(),
                start + length <= usize::MAX,
                start + length <= self.spec_len(),
            ensures
                sliced.spec_arrayseqmtephslice_wf(),
                sliced.spec_len() == length as int,
                forall|i: int| #![trigger sliced.spec_index(i)]
                    0 <= i < length ==> sliced.spec_index(i) == self.spec_index(start as int + i);

        /// O(1) subseq: same as slice (shares backing storage).
        fn subseq_copy(&self, start: usize, length: usize) -> (subseq: Self)
            requires
                self.spec_arrayseqmtephslice_wf(),
                start + length <= usize::MAX,
                start + length <= self.spec_len(),
            ensures
                subseq.spec_arrayseqmtephslice_wf(),
                subseq.spec_len() == length as int,
                forall|i: int| #![trigger subseq.spec_index(i)]
                    0 <= i < length ==> subseq.spec_index(i) == self.spec_index(start as int + i);

        fn from_vec(data: Vec<T>) -> (seq: Self)
            ensures
                seq.spec_arrayseqmtephslice_wf(),
                seq.spec_len() == data@.len(),
                forall|i: int| #![trigger seq.spec_index(i)]
                    0 <= i < data@.len() ==> seq.spec_index(i) == data@[i];

        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(1), Span O(1)
        /// - Alg Analysis: Claude-Opus-4.6 (1M): NONE
        fn empty() -> (empty_seq: Self)
            ensures
                empty_seq.spec_arrayseqmtephslice_wf(),
                empty_seq.spec_len() == 0;

        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(1), Span O(1)
        /// - Alg Analysis: Claude-Opus-4.6 (1M): NONE
        fn singleton(item: T) -> (s: Self)
            requires obeys_feq_clone::<T>(),
            ensures
                s.spec_arrayseqmtephslice_wf(),
                s.spec_len() == 1,
                s.spec_index(0) == item;

        fn new(length: usize, init_value: T) -> (new_seq: Self)
            requires
                length <= usize::MAX,
                obeys_feq_clone::<T>(),
            ensures
                new_seq.spec_arrayseqmtephslice_wf(),
                new_seq.spec_len() == length as int,
                forall|i: int| #![trigger new_seq.spec_index(i)]
                    0 <= i < length ==> new_seq.spec_index(i) == init_value;

        /// Materialize the slice window into a freshly-allocated Vec.
        fn to_vec(&self) -> (v: Vec<T>)
            requires
                self.spec_arrayseqmtephslice_wf(),
                obeys_feq_clone::<T>(),
            ensures
                v@.len() == self.spec_len(),
                forall|i: int| #![trigger v@[i]]
                    0 <= i < self.spec_len() ==> v@[i] == self.spec_index(i);

        fn iter(&self) -> (it: ArraySeqMtEphSliceIter<'_, T>)
            requires self.spec_arrayseqmtephslice_wf(),
            ensures
                it@.0 == 0,
                it@.1 == self.spec_backing_seq(),
                iter_invariant(&it);
    }

    // 9. impls

    impl<T: Eq + Clone> ArraySeqMtEphSliceTrait<T> for ArraySeqMtEphSliceS<T> {
        open spec fn spec_arrayseqmtephslice_wf(&self) -> bool {
            self.start + self.len <= (*self.data)@.len()
            && self.start + self.len <= usize::MAX
        }

        open spec fn spec_len(&self) -> nat {
            self.len as nat
        }

        open spec fn spec_index(&self, i: int) -> T {
            (*self.data)@[self.start as int + i]
        }

        open spec fn spec_backing_seq(&self) -> Seq<T> {
            (*self.data)@.subrange(self.start as int, (self.start + self.len) as int)
        }

        fn length(&self) -> (len: usize) {
            self.len
        }

        fn nth_cloned(&self, index: usize) -> (elem: T) {
            let v: &Vec<T> = arc_deref(&self.data);
            v[self.start + index].clone_plus()
        }

        fn slice(&self, start: usize, length: usize) -> (sliced: Self) {
            let new_data = Arc::clone(&self.data);
            ArraySeqMtEphSliceS {
                data: new_data,
                start: self.start + start,
                len: length,
            }
        }

        fn subseq_copy(&self, start: usize, length: usize) -> (subseq: Self) {
            self.slice(start, length)
        }

        fn from_vec(data: Vec<T>) -> (seq: Self) {
            let len = data.len();
            ArraySeqMtEphSliceS {
                data: Arc::new(data),
                start: 0,
                len: len,
            }
        }

        fn empty() -> (empty_seq: Self) {
            ArraySeqMtEphSliceS {
                data: Arc::new(Vec::new()),
                start: 0,
                len: 0,
            }
        }

        fn singleton(item: T) -> (s: Self) {
            let mut v: Vec<T> = Vec::new();
            v.push(item);
            ArraySeqMtEphSliceS {
                data: Arc::new(v),
                start: 0,
                len: 1,
            }
        }

        fn new(length: usize, init_value: T) -> (new_seq: Self) {
            let mut v: Vec<T> = Vec::with_capacity(length);
            let mut i: usize = 0;
            while i < length
                invariant
                    0 <= i <= length,
                    obeys_feq_clone::<T>(),
                    v@.len() == i as int,
                    forall|j: int| #![trigger v@[j]]
                        0 <= j < i ==> v@[j] == init_value,
                decreases length - i,
            {
                v.push(init_value.clone_plus());
                i = i + 1;
            }
            ArraySeqMtEphSliceS {
                data: Arc::new(v),
                start: 0,
                len: length,
            }
        }

        fn to_vec(&self) -> (v: Vec<T>) {
            let mut v: Vec<T> = Vec::with_capacity(self.len);
            let mut i: usize = 0;
            while i < self.len
                invariant
                    0 <= i <= self.len,
                    self.spec_arrayseqmtephslice_wf(),
                    obeys_feq_clone::<T>(),
                    v@.len() == i as int,
                    forall|j: int| #![trigger v@[j]]
                        0 <= j < i as int ==> v@[j] == self.spec_index(j),
                decreases self.len - i,
            {
                let elem = self.nth_cloned(i);
                v.push(elem);
                i = i + 1;
            }
            v
        }

        fn iter(&self) -> (it: ArraySeqMtEphSliceIter<'_, T>) {
            let sl: &[T] = arc_vec_as_slice(&self.data, self.start, self.len);
            assert(sl@.len() == self.len);
            assert(sl@ == self.spec_backing_seq());
            ArraySeqMtEphSliceIter { inner: sl.iter() }
        }
    }

    // 10. iterators

    #[verifier::reject_recursive_types(T)]
    pub struct ArraySeqMtEphSliceIter<'a, T> {
        pub inner: std::slice::Iter<'a, T>,
    }

    impl<'a, T> View for ArraySeqMtEphSliceIter<'a, T> {
        type V = (int, Seq<T>);
        open spec fn view(&self) -> (int, Seq<T>) { self.inner@ }
    }

    pub open spec fn iter_invariant<'a, T>(it: &ArraySeqMtEphSliceIter<'a, T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<'a, T> std::iter::Iterator for ArraySeqMtEphSliceIter<'a, T> {
        type Item = &'a T;

        fn next(&mut self) -> (next: Option<&'a T>)
            ensures ({
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
            })
        {
            self.inner.next()
        }
    }

    /// Ghost iterator for ForLoopGhostIterator support.
    #[verifier::reject_recursive_types(T)]
    pub struct ArraySeqMtEphSliceGhostIterator<'a, T> {
        pub pos: int,
        pub elements: Seq<T>,
        pub phantom: core::marker::PhantomData<&'a T>,
    }

    impl<'a, T> View for ArraySeqMtEphSliceGhostIterator<'a, T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.elements.take(self.pos) }
    }

    impl<'a, T> vstd::pervasive::ForLoopGhostIteratorNew for ArraySeqMtEphSliceIter<'a, T> {
        type GhostIter = ArraySeqMtEphSliceGhostIterator<'a, T>;
        open spec fn ghost_iter(&self) -> ArraySeqMtEphSliceGhostIterator<'a, T> {
            ArraySeqMtEphSliceGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    impl<'a, T> vstd::pervasive::ForLoopGhostIterator for ArraySeqMtEphSliceGhostIterator<'a, T> {
        type ExecIter = ArraySeqMtEphSliceIter<'a, T>;
        type Item = T;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &ArraySeqMtEphSliceIter<'a, T>) -> bool {
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
            if 0 <= self.pos < self.elements.len() { Some(self.elements[self.pos]) } else { None }
        }

        open spec fn ghost_advance(&self, _exec_iter: &ArraySeqMtEphSliceIter<'a, T>) -> ArraySeqMtEphSliceGhostIterator<'a, T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, T: Eq + Clone> std::iter::IntoIterator for &'a ArraySeqMtEphSliceS<T> {
        type Item = &'a T;
        type IntoIter = ArraySeqMtEphSliceIter<'a, T>;

        fn into_iter(self) -> (it: Self::IntoIter)
            requires self.spec_arrayseqmtephslice_wf(),
            ensures
                it@.0 == 0,
                it@.1 == self.spec_backing_seq(),
                iter_invariant(&it),
        {
            let sl: &[T] = arc_vec_as_slice(&self.data, self.start, self.len);
            assert(sl@.len() == self.len);
            assert(sl@ == self.spec_backing_seq());
            ArraySeqMtEphSliceIter { inner: sl.iter() }
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    impl<T: std::fmt::Debug> std::fmt::Debug for ArraySeqMtEphSliceS<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_list()
                .entries(self.data[self.start..self.start + self.len].iter())
                .finish()
        }
    }

    impl<T: std::fmt::Display> std::fmt::Display for ArraySeqMtEphSliceS<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut first = true;
            write!(f, "[")?;
            for item in self.data[self.start..self.start + self.len].iter() {
                if !first {
                    write!(f, ", ")?;
                }
                first = false;
                write!(f, "{item}")?;
            }
            write!(f, "]")
        }
    }

    #[macro_export]
    macro_rules! ArraySeqMtEphSliceSLit {
        () => {
            $crate::Chap19::ArraySeqMtEphSlice::ArraySeqMtEphSlice::ArraySeqMtEphSliceS::from_vec(
                Vec::new(),
            )
        };
        ($x:expr; $n:expr) => {
            $crate::Chap19::ArraySeqMtEphSlice::ArraySeqMtEphSlice::ArraySeqMtEphSliceS::from_vec(
                vec![$x; $n],
            )
        };
        ($($x:expr),* $(,)?) => {
            $crate::Chap19::ArraySeqMtEphSlice::ArraySeqMtEphSlice::ArraySeqMtEphSliceS::from_vec(
                vec![$($x),*],
            )
        };
    }
}
