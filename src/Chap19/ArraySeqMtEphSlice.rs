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

        fn iter<'a>(&'a self) -> (it: ArraySeqMtEphSliceIter<'a, T>)
            ensures
                it@.0 == 0int,
                it@.1.len() == self.spec_len(),
                forall|i: int| #![trigger it@.1[i]]
                    0 <= i < self.spec_len() ==> it@.1[i] == self.spec_index(i),
                iter_invariant(&it);

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

        /// - APAS: primitive (Section 19.2).
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1).
        fn length(&self) -> (len: usize) {
            self.seq.len()
        }

        fn iter<'a>(&'a self) -> (it: ArraySeqMtEphSliceIter<'a, T>)
            ensures
                it@.0 == 0int,
                it@.1 == self.seq@,
                iter_invariant(&it),
        {
            ArraySeqMtEphSliceIter { inner: self.seq.iter() }
        }

        /// - APAS: primitive (Section 19.2).
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1).
        fn nth_cloned(&self, index: usize) -> (elem: T) {
            self.seq[index].clone_plus()
        }

        /// - APAS: primitive (Section 19.2) — subseq variant.
        /// - Claude-Opus-4.6: Work Θ(length), Span Θ(length).
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

        /// - APAS: N/A — implementation utility.
        /// - Claude-Opus-4.6: Work Θ(length), Span Θ(length).
        fn subseq_copy(&self, start: usize, length: usize) -> (subseq: Self) {
            self.slice(start, length)
        }

        /// - APAS: N/A — implementation utility.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1).
        fn from_vec(data: Vec<T>) -> (seq: Self) {
            ArraySeqMtEphSliceS { seq: data }
        }

        /// - APAS: Algorithm 19.1 — empty.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1).
        fn empty() -> (empty_seq: Self) {
            ArraySeqMtEphSliceS { seq: Vec::new() }
        }

        /// - APAS: Algorithm 19.2 — singleton.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1).
        fn singleton(item: T) -> (s: Self) {
            let mut v: Vec<T> = Vec::new();
            v.push(item);
            ArraySeqMtEphSliceS { seq: v }
        }

        /// - APAS: N/A — implementation utility.
        /// - Claude-Opus-4.6: Work Θ(length), Span Θ(length).
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

    // 10. iterators

    #[verifier::reject_recursive_types(T)]
    pub struct ArraySeqMtEphSliceIter<'a, T> {
        inner: std::slice::Iter<'a, T>,
    }

    impl<'a, T> View for ArraySeqMtEphSliceIter<'a, T> {
        type V = (int, Seq<T>);
        closed spec fn view(&self) -> (int, Seq<T>) {
            self.inner@
        }
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

    #[verifier::reject_recursive_types(T)]
    pub struct ArraySeqMtEphSliceGhostIterator<'a, T> {
        pub pos: int,
        pub elements: Seq<T>,
        pub phantom: core::marker::PhantomData<&'a T>,
    }

    impl<'a, T> View for ArraySeqMtEphSliceGhostIterator<'a, T> {
        type V = Seq<T>;

        open spec fn view(&self) -> Seq<T> {
            self.elements.take(self.pos)
        }
    }

    impl<'a, T> vstd::pervasive::ForLoopGhostIteratorNew for ArraySeqMtEphSliceIter<'a, T> {
        type GhostIter = ArraySeqMtEphSliceGhostIterator<'a, T>;
        open spec fn ghost_iter(&self) -> ArraySeqMtEphSliceGhostIterator<'a, T> {
            ArraySeqMtEphSliceGhostIterator {
                pos: self@.0,
                elements: self@.1,
                phantom: core::marker::PhantomData,
            }
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
            if 0 <= self.pos < self.elements.len() {
                Some(self.elements[self.pos])
            } else {
                None
            }
        }

        open spec fn ghost_advance(
            &self,
            _exec_iter: &ArraySeqMtEphSliceIter<'a, T>,
        ) -> ArraySeqMtEphSliceGhostIterator<'a, T> {
            Self {
                pos: self.pos + 1,
                ..*self
            }
        }
    }

    impl<'a, T> std::iter::IntoIterator for &'a ArraySeqMtEphSliceS<T> {
        type Item = &'a T;
        type IntoIter = ArraySeqMtEphSliceIter<'a, T>;
        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                it@.0 == 0,
                it@.1 == self.seq@,
                iter_invariant(&it),
        {
            ArraySeqMtEphSliceIter { inner: self.seq.iter() }
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    impl<T: std::fmt::Debug> std::fmt::Debug for ArraySeqMtEphSliceS<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_list().entries(self.seq.iter()).finish()
        }
    }

    impl<T: std::fmt::Display> std::fmt::Display for ArraySeqMtEphSliceS<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut first = true;
            write!(f, "[")?;
            for item in self.seq.iter() {
                if !first { write!(f, ", ")?; }
                first = false;
                write!(f, "{item}")?;
            }
            write!(f, "]")
        }
    }

    #[macro_export]
    macro_rules! ArraySeqMtEphSliceSLit {
        () => { $crate::Chap19::ArraySeqMtEphSlice::ArraySeqMtEphSlice::ArraySeqMtEphSliceS::from_vec(Vec::new()) };
        ($x:expr; $n:expr) => { $crate::Chap19::ArraySeqMtEphSlice::ArraySeqMtEphSlice::ArraySeqMtEphSliceS::from_vec(vec![$x; $n]) };
        ($($x:expr),* $(,)?) => { $crate::Chap19::ArraySeqMtEphSlice::ArraySeqMtEphSlice::ArraySeqMtEphSliceS::from_vec(vec![$($x),*]) };
    }
}
