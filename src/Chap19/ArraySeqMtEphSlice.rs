//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! MtEph slice-oriented Array sequence variant.
//!
//! Under verus_keep_ghost: Vec-backed with full Verus specs and proofs.
//! Under normal compilation: Arc<Mutex<Box<[T]>>> with range metadata for O(1) slicing.

pub mod ArraySeqMtEphSlice {

    use vstd::prelude::*;

    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_clone;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;

    #[cfg(not(verus_keep_ghost))]
    use std::ops::Range;
    #[cfg(not(verus_keep_ghost))]
    use std::sync::{Arc, Mutex};
    #[cfg(not(verus_keep_ghost))]
    use crate::ParaPair;
    #[cfg(not(verus_keep_ghost))]
    use crate::Types::Types::*;

    verus! {

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
        vstd::seq_lib::group_seq_properties,
        crate::vstdplus::feq::feq::group_feq_axioms,
    };

    // ─── Verus struct ───

    #[cfg(verus_keep_ghost)]
    #[verifier::reject_recursive_types(T)]
    pub struct ArraySeqMtEphSliceS<T> {
        pub seq: Vec<T>,
    }

    #[cfg(verus_keep_ghost)]
    impl<T: View> View for ArraySeqMtEphSliceS<T> {
        type V = Seq<T::V>;
        open spec fn view(&self) -> Seq<T::V> {
            self.seq@.map(|_i: int, t: T| t@)
        }
    }

    // ─── Trait with specs ───

    pub trait ArraySeqMtEphSliceTrait<T: Eq + Clone>: Sized {
        spec fn spec_len(&self) -> nat;
        spec fn spec_index(&self, i: int) -> T
            recommends i < self.spec_len();

        fn length(&self) -> (len: usize)
            ensures len as int == self.spec_len();

        fn nth_cloned(&self, index: usize) -> (elem: T)
            requires
                index < self.spec_len(),
                obeys_feq_clone::<T>(),
            ensures elem == self.spec_index(index as int);

        fn slice(&self, start: usize, length: usize) -> (sliced: Self)
            requires
                start + length <= usize::MAX,
                start + length <= self.spec_len(),
                obeys_feq_clone::<T>(),
            ensures
                sliced.spec_len() == length as int,
                forall|i: int| #![trigger sliced.spec_index(i)]
                    0 <= i < length ==> sliced.spec_index(i) == self.spec_index(start as int + i);

        fn subseq_copy(&self, start: usize, length: usize) -> (subseq: Self)
            requires
                start + length <= usize::MAX,
                start + length <= self.spec_len(),
                obeys_feq_clone::<T>(),
            ensures
                subseq.spec_len() == length as int,
                forall|i: int| #![trigger subseq.spec_index(i)]
                    0 <= i < length ==> subseq.spec_index(i) == self.spec_index(start as int + i);

        fn from_vec(data: Vec<T>) -> (seq: Self)
            ensures
                seq.spec_len() == data@.len(),
                forall|i: int| #![trigger seq.spec_index(i)]
                    0 <= i < data@.len() ==> seq.spec_index(i) == data@[i];

        fn empty() -> (empty_seq: Self)
            ensures empty_seq.spec_len() == 0;

        fn singleton(item: T) -> (s: Self)
            requires obeys_feq_clone::<T>(),
            ensures
                s.spec_len() == 1,
                s.spec_index(0) == item;

        fn new(length: usize, init_value: T) -> (new_seq: Self)
            requires
                length <= usize::MAX,
                obeys_feq_clone::<T>(),
            ensures
                new_seq.spec_len() == length as int,
                forall|i: int| #![trigger new_seq.spec_index(i)]
                    0 <= i < length ==> new_seq.spec_index(i) == init_value;
    }

    // ─── Verified impl ───

    #[cfg(verus_keep_ghost)]
    impl<T: Eq + Clone> ArraySeqMtEphSliceTrait<T> for ArraySeqMtEphSliceS<T> {
        open spec fn spec_len(&self) -> nat {
            self.seq@.len()
        }

        open spec fn spec_index(&self, i: int) -> T {
            self.seq[i]
        }

        fn length(&self) -> (len: usize) {
            self.seq.len()
        }

        fn nth_cloned(&self, index: usize) -> (elem: T) {
            self.seq[index].clone_plus()
        }

        fn slice(&self, start: usize, length: usize) -> (sliced: Self) {
            let mut v: Vec<T> = Vec::with_capacity(length);
            let mut i: usize = 0;
            while i < length
                invariant
                    0 <= i <= length,
                    start + length <= self.seq@.len(),
                    start as int + length as int <= usize::MAX as int,
                    obeys_feq_clone::<T>(),
                    v@.len() == i as int,
                    forall|j: int| #![trigger v@[j]]
                        0 <= j < i ==> v@[j] == self.seq@[start as int + j],
                decreases length - i,
            {
                v.push(self.seq[start + i].clone_plus());
                i = i + 1;
            }
            ArraySeqMtEphSliceS { seq: v }
        }

        fn subseq_copy(&self, start: usize, length: usize) -> (subseq: Self) {
            self.slice(start, length)
        }

        fn from_vec(data: Vec<T>) -> (seq: Self) {
            ArraySeqMtEphSliceS { seq: data }
        }

        fn empty() -> (empty_seq: Self) {
            ArraySeqMtEphSliceS { seq: Vec::new() }
        }

        fn singleton(item: T) -> (s: Self) {
            let mut v: Vec<T> = Vec::new();
            v.push(item);
            ArraySeqMtEphSliceS { seq: v }
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
            ArraySeqMtEphSliceS { seq: v }
        }
    }

    } // verus!

    // ═══════════════════════════════════════════════════════════════
    // Runtime: Arc<Mutex<Box<[T]>>> backed struct with O(1) slicing
    // ═══════════════════════════════════════════════════════════════

    #[cfg(not(verus_keep_ghost))]
    #[derive(Debug)]
    struct Inner<T: StTInMtT> {
        data: Mutex<Box<[T]>>,
    }

    #[cfg(not(verus_keep_ghost))]
    impl<T: StTInMtT> Inner<T> {
        fn new(data: Box<[T]>) -> Self { Inner { data: Mutex::new(data) } }
    }

    #[cfg(not(verus_keep_ghost))]
    pub struct ArraySeqMtEphSliceS<T: StTInMtT> {
        inner: Arc<Inner<T>>,
        range: Range<usize>,
    }

    #[cfg(not(verus_keep_ghost))]
    fn clamp_subrange<T: StTInMtT + 'static>(a: &ArraySeqMtEphSliceS<T>, start: usize, length: usize) -> Range<usize> {
        let local_len = a.length();
        let clamped_start = start.min(local_len);
        let clamped_end = clamped_start.saturating_add(length).min(local_len);
        let base = a.range.start;
        (base + clamped_start)..(base + clamped_end)
    }

    #[cfg(not(verus_keep_ghost))]
    impl<T: StTInMtT + Eq + 'static> ArraySeqMtEphSliceTrait<T> for ArraySeqMtEphSliceS<T> {
        fn length(&self) -> usize {
            self.range.end - self.range.start
        }

        fn nth_cloned(&self, index: usize) -> T {
            let guard = self.inner.data.lock().unwrap();
            guard[self.range.start + index].clone()
        }

        fn slice(&self, start: usize, length: usize) -> Self {
            let sub = clamp_subrange(self, start, length);
            ArraySeqMtEphSliceS {
                inner: Arc::clone(&self.inner),
                range: sub,
            }
        }

        fn subseq_copy(&self, start: usize, length: usize) -> Self {
            let sub = clamp_subrange(self, start, length);
            let guard = self.inner.data.lock().unwrap();
            let data: Vec<T> = guard[sub.start..sub.end].to_vec();
            Self::from_vec(data)
        }

        fn from_vec(data: Vec<T>) -> Self {
            let len = data.len();
            ArraySeqMtEphSliceS {
                inner: Arc::new(Inner::new(data.into_boxed_slice())),
                range: 0..len,
            }
        }

        fn empty() -> Self {
            Self::from_vec(Vec::new())
        }

        fn singleton(item: T) -> Self {
            Self::from_vec(vec![item])
        }

        fn new(length: usize, init_value: T) -> Self {
            let mut data = Vec::with_capacity(length);
            for _ in 0..length { data.push(init_value.clone()); }
            Self::from_vec(data)
        }
    }

    // ─── Runtime trait impls ───

    #[cfg(not(verus_keep_ghost))]
    impl<T: StTInMtT> Clone for ArraySeqMtEphSliceS<T> {
        fn clone(&self) -> Self {
            ArraySeqMtEphSliceS {
                inner: Arc::clone(&self.inner),
                range: self.range.clone(),
            }
        }
    }

    #[cfg(not(verus_keep_ghost))]
    impl<T: StTInMtT + 'static> PartialEq for ArraySeqMtEphSliceS<T> {
        fn eq(&self, other: &Self) -> bool {
            if Arc::ptr_eq(&self.inner, &other.inner) && self.range == other.range {
                return true;
            }
            if self.length() != other.length() { return false; }
            let left = self.to_vec();
            let right = other.to_vec();
            left.iter().zip(right.iter()).all(|(a, b)| a == b)
        }
    }

    #[cfg(not(verus_keep_ghost))]
    impl<T: StTInMtT + 'static> Eq for ArraySeqMtEphSliceS<T> {}

    #[cfg(not(verus_keep_ghost))]
    impl<T: StTInMtT> std::fmt::Debug for ArraySeqMtEphSliceS<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let guard = self.inner.data.lock().unwrap();
            f.debug_list()
                .entries(guard[self.range.start..self.range.end].iter())
                .finish()
        }
    }

    #[cfg(not(verus_keep_ghost))]
    impl<T: StTInMtT> std::fmt::Display for ArraySeqMtEphSliceS<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let guard = self.inner.data.lock().unwrap();
            let mut first = true;
            write!(f, "[")?;
            for item in &guard[self.range.start..self.range.end] {
                if !first { write!(f, ", ")?; }
                first = false;
                write!(f, "{item}")?;
            }
            write!(f, "]")
        }
    }

    #[cfg(not(verus_keep_ghost))]
    impl<T: StTInMtT + 'static> ArraySeqMtEphSliceS<T> {
        pub fn to_vec(&self) -> Vec<T> {
            let guard = self.inner.data.lock().unwrap();
            guard[self.range.start..self.range.end].to_vec()
        }
    }

    #[cfg(not(verus_keep_ghost))]
    #[macro_export]
    macro_rules! ArraySeqMtEphSliceSLit {
        () => { $crate::Chap19::ArraySeqMtEphSlice::ArraySeqMtEphSlice::ArraySeqMtEphSliceS::from_vec(Vec::new()) };
        ($x:expr; $n:expr) => { $crate::Chap19::ArraySeqMtEphSlice::ArraySeqMtEphSlice::ArraySeqMtEphSliceS::from_vec(vec![$x; $n]) };
        ($($x:expr),* $(,)?) => { $crate::Chap19::ArraySeqMtEphSlice::ArraySeqMtEphSlice::ArraySeqMtEphSliceS::from_vec(vec![$($x),*]) };
    }
}
