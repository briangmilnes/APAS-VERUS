// Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Chapter 19 slice-backed array sequence (multi-threaded ephemeral).
//! O(1) slicing via shared `Arc<Vec<T>>` backing with offset/length window.
//! Multiple slices can share the same backing storage; `slice()` and
//! `subseq_copy()` are O(1) (just an Arc ref-count bump + window adjust).
//! Iterators delegate to vstd's fully-specified `std::slice::Iter`.

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
    };

    // 4. type definitions

    #[verifier::reject_recursive_types(T)]
    pub struct ArraySeqMtEphSliceS<T> {
        data: Arc<Vec<T>>,
        start: usize,
        len: usize,
    }

    // 5. view impls

    impl<T: View> View for ArraySeqMtEphSliceS<T> {
        type V = Seq<T::V>;

        closed spec fn view(&self) -> Seq<T::V> {
            (*self.data)@.map(|_i: int, t: T| t@)
                .subrange(self.start as int, (self.start + self.len) as int)
        }
    }

    // 6. spec fns

    impl<T> ArraySeqMtEphSliceS<T> {
        /// The raw backing subrange as a Seq<T> (not View-mapped).
        /// Connects arc_vec_as_slice result to spec_len/spec_index.
        closed spec fn spec_backing_seq(&self) -> Seq<T> {
            (*self.data)@.subrange(self.start as int, (self.start + self.len) as int)
        }
    }

    // 8. traits

    pub trait ArraySeqMtEphSliceTrait<T: Eq + Clone>: Sized {
        spec fn slice_wf(&self) -> bool;

        spec fn spec_len(&self) -> nat;

        spec fn spec_index(&self, i: int) -> T
            recommends self.slice_wf(), i < self.spec_len();

        fn length(&self) -> (len: usize)
            requires self.slice_wf(),
            ensures len as int == self.spec_len();

        fn nth_cloned(&self, index: usize) -> (elem: T)
            requires
                self.slice_wf(),
                index < self.spec_len(),
                obeys_feq_clone::<T>(),
            ensures elem == self.spec_index(index as int);

        /// O(1) slice: shares backing storage, adjusts window.
        fn slice(&self, start: usize, length: usize) -> (sliced: Self)
            requires
                self.slice_wf(),
                start + length <= usize::MAX,
                start + length <= self.spec_len(),
            ensures
                sliced.slice_wf(),
                sliced.spec_len() == length as int,
                forall|i: int| #![trigger sliced.spec_index(i)]
                    0 <= i < length ==> sliced.spec_index(i) == self.spec_index(start as int + i);

        /// O(1) subseq: same as slice (shares backing storage).
        fn subseq_copy(&self, start: usize, length: usize) -> (subseq: Self)
            requires
                self.slice_wf(),
                start + length <= usize::MAX,
                start + length <= self.spec_len(),
            ensures
                subseq.slice_wf(),
                subseq.spec_len() == length as int,
                forall|i: int| #![trigger subseq.spec_index(i)]
                    0 <= i < length ==> subseq.spec_index(i) == self.spec_index(start as int + i);

        fn from_vec(data: Vec<T>) -> (seq: Self)
            ensures
                seq.slice_wf(),
                seq.spec_len() == data@.len(),
                forall|i: int| #![trigger seq.spec_index(i)]
                    0 <= i < data@.len() ==> seq.spec_index(i) == data@[i];

        fn empty() -> (empty_seq: Self)
            ensures
                empty_seq.slice_wf(),
                empty_seq.spec_len() == 0;

        fn singleton(item: T) -> (s: Self)
            requires obeys_feq_clone::<T>(),
            ensures
                s.slice_wf(),
                s.spec_len() == 1,
                s.spec_index(0) == item;

        fn new(length: usize, init_value: T) -> (new_seq: Self)
            requires
                length <= usize::MAX,
                obeys_feq_clone::<T>(),
            ensures
                new_seq.slice_wf(),
                new_seq.spec_len() == length as int,
                forall|i: int| #![trigger new_seq.spec_index(i)]
                    0 <= i < length ==> new_seq.spec_index(i) == init_value;
    }

    // 9. impls

    impl<T: Eq + Clone> ArraySeqMtEphSliceTrait<T> for ArraySeqMtEphSliceS<T> {
        closed spec fn slice_wf(&self) -> bool {
            self.start + self.len <= (*self.data)@.len()
            && self.start + self.len <= usize::MAX
        }

        closed spec fn spec_len(&self) -> nat {
            self.len as nat
        }

        closed spec fn spec_index(&self, i: int) -> T {
            (*self.data)@[self.start as int + i]
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
    }

    // 10. iterators

    // No custom iterator type. We return vstd's std::slice::Iter<'a, T>
    // directly, inheriting all of its specs (View, ForLoopGhostIterator, etc.).

    impl<'a, T: Eq + Clone> ArraySeqMtEphSliceS<T> {
        pub fn iter(&'a self) -> (it: std::slice::Iter<'a, T>)
            requires self.slice_wf(),
            ensures
                it@.0 == 0,
                it@.1.len() == self.spec_len(),
                forall|i: int| #![trigger it@.1[i]]
                    0 <= i < self.spec_len() ==> it@.1[i] == self.spec_index(i),
        {
            let sl: &[T] = arc_vec_as_slice(&self.data, self.start, self.len);
            assert(sl@.len() == self.len);
            assert(sl@ == self.spec_backing_seq());
            sl.iter()
        }
    }

    impl<'a, T: Eq + Clone> std::iter::IntoIterator for &'a ArraySeqMtEphSliceS<T> {
        type Item = &'a T;
        type IntoIter = std::slice::Iter<'a, T>;

        fn into_iter(self) -> (it: std::slice::Iter<'a, T>)
            requires self.slice_wf(),
            ensures
                it@.0 == 0,
                it@.1.len() == self.spec_len(),
                forall|i: int| #![trigger it@.1[i]]
                    0 <= i < self.spec_len() ==> it@.1[i] == self.spec_index(i),
        {
            let sl: &[T] = arc_vec_as_slice(&self.data, self.start, self.len);
            assert(sl@.len() == self.len);
            assert(sl@ == self.spec_backing_seq());
            sl.iter()
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
