// Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

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
    use crate::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::join;
    use crate::Concurrency::Concurrency::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::monoid::monoid::*;

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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        fn length(&self) -> (len: usize)
            requires self.spec_arrayseqmtephslice_wf(),
            ensures len as int == self.spec_len();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — array index + clone.
        fn nth_cloned(&self, index: usize) -> (elem: T)
            requires
                self.spec_arrayseqmtephslice_wf(),
                index < self.spec_len(),
                obeys_feq_clone::<T>(),
            ensures elem == self.spec_index(index as int);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — Arc clone + window adjust.
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn from_vec(data: Vec<T>) -> (seq: Self)
            ensures
                seq.spec_arrayseqmtephslice_wf(),
                seq.spec_len() == data@.len(),
                forall|i: int| #![trigger seq.spec_index(i)]
                    0 <= i < data@.len() ==> seq.spec_index(i) == data@[i];

        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        fn empty() -> (empty_seq: Self)
            ensures
                empty_seq.spec_arrayseqmtephslice_wf(),
                empty_seq.spec_len() == 0;

        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        fn singleton(item: T) -> (s: Self)
            requires obeys_feq_clone::<T>(),
            ensures
                s.spec_arrayseqmtephslice_wf(),
                s.spec_len() == 1,
                s.spec_index(0) == item;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
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

        /// Parallel reduce via D&C on O(1) slices.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(lg n) — D&C + join, O(1) split.
        fn reduce<F: MtReduceFn<T>>(
            &self, f: &F, Ghost(spec_f): Ghost<spec_fn(T, T) -> T>, id: T,
        ) -> (reduced: T)
            where T: Send + Sync + 'static
            requires
                self.spec_arrayseqmtephslice_wf(),
                obeys_feq_clone::<T>(),
                spec_monoid(spec_f, id),
                forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) <==> ret == spec_f(x, y),
            ensures
                reduced == self.spec_backing_seq().fold_left(id, spec_f);

        /// Parallel map via D&C on O(1) slices.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — D&C + join, O(1) split, O(n) rejoin.
        fn map<U: Eq + Clone + Send + Sync + 'static, F: MtMapFn<T, U>>(
            &self, f: &F,
        ) -> (mapped: ArraySeqMtEphSliceS<U>)
            where T: Send + Sync + 'static
            requires
                self.spec_arrayseqmtephslice_wf(),
                obeys_feq_clone::<T>(),
                obeys_feq_clone::<U>(),
                forall|x: &T| #[trigger] f.requires((x,)),
            ensures
                mapped.len as nat == self.spec_len(),
                mapped.start + mapped.len <= (*mapped.data)@.len(),
                mapped.start + mapped.len <= usize::MAX;

        /// Parallel filter via D&C on O(1) slices.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — D&C + join, O(1) split, O(n) rejoin.
        fn filter<F: MtPred<T>>(
            &self, pred: &F, Ghost(spec_pred): Ghost<spec_fn(T) -> bool>,
        ) -> (filtered: Self)
            where T: Send + Sync + 'static
            requires
                self.spec_arrayseqmtephslice_wf(),
                obeys_feq_clone::<T>(),
                forall|x: &T| #[trigger] pred.requires((x,)),
                forall|v: T, keep: bool| pred.ensures((&v,), keep) ==> spec_pred(v) == keep,
            ensures
                filtered.spec_arrayseqmtephslice_wf(),
                filtered.spec_len() <= self.spec_len();
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn length(&self) -> (len: usize) {
            self.len
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn nth_cloned(&self, index: usize) -> (elem: T) {
            let v: &Vec<T> = arc_deref(&self.data);
            v[self.start + index].clone_plus()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn slice(&self, start: usize, length: usize) -> (sliced: Self) {
            let new_data = Arc::clone(&self.data);
            ArraySeqMtEphSliceS {
                data: new_data,
                start: self.start + start,
                len: length,
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn subseq_copy(&self, start: usize, length: usize) -> (subseq: Self) {
            self.slice(start, length)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn from_vec(data: Vec<T>) -> (seq: Self) {
            let len = data.len();
            ArraySeqMtEphSliceS {
                data: Arc::new(data),
                start: 0,
                len: len,
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (empty_seq: Self) {
            ArraySeqMtEphSliceS {
                data: Arc::new(Vec::new()),
                start: 0,
                len: 0,
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(item: T) -> (s: Self) {
            let mut v: Vec<T> = Vec::new();
            v.push(item);
            ArraySeqMtEphSliceS {
                data: Arc::new(v),
                start: 0,
                len: 1,
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
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

        fn reduce<F: MtReduceFn<T>>(
            &self, f: &F, Ghost(spec_f): Ghost<spec_fn(T, T) -> T>, id: T,
        ) -> (reduced: T)
            where T: Send + Sync + 'static
        {
            let len = self.length();
            if len == 0 {
                proof {
                    assert(self.spec_backing_seq() =~= Seq::<T>::empty());
                    reveal(Seq::fold_left);
                }
                id
            } else {
                Self::reduce_dc(self, f, Ghost(spec_f), id)
            }
        }

        fn map<U: Eq + Clone + Send + Sync + 'static, F: MtMapFn<T, U>>(
            &self, f: &F,
        ) -> (mapped: ArraySeqMtEphSliceS<U>)
            where T: Send + Sync + 'static
        {
            let v = Self::map_dc_vec(self, f);
            ArraySeqMtEphSliceS::<U>::from_vec(v)
        }

        fn filter<F: MtPred<T>>(
            &self, pred: &F, Ghost(spec_pred): Ghost<spec_fn(T) -> bool>,
        ) -> (filtered: Self)
            where T: Send + Sync + 'static
        {
            let v = Self::filter_dc_vec(self, pred, Ghost(spec_pred));
            Self::from_vec(v)
        }
    }

    // 9b. bare impl — D&C helpers and proof fns

    impl<T: Eq + Clone> ArraySeqMtEphSliceS<T> {
        /// For a monoid (f, id): f(x, s.fold_left(id, f)) == s.fold_left(x, f).
        proof fn lemma_monoid_fold_left(s: Seq<T>, f: spec_fn(T, T) -> T, id: T, x: T)
            requires spec_monoid(f, id)
            ensures f(x, s.fold_left(id, f)) == s.fold_left(x, f)
            decreases s.len()
        {
            if s.len() > 0 {
                let n = (s.len() - 1) as int;
                let s1 = s.subrange(0, n);
                let tail = s.subrange(n, s.len() as int);
                let a_last = s[n];

                s.lemma_fold_left_split(id, f, n);
                s.lemma_fold_left_split(x, f, n);

                assert(tail =~= seq![a_last]);
                reveal_with_fuel(Seq::fold_left, 2);
                let lid = s1.fold_left(id, f);
                let lx = s1.fold_left(x, f);
                Self::lemma_monoid_fold_left(s1, f, id, x);

                assert(f(x, f(lid, a_last)) == f(f(x, lid), a_last));
            }
        }

        /// D&C reduce on O(1) slices. Called by trait reduce for non-empty sequences.
        fn reduce_dc<F: MtReduceFn<T>>(
            a: &ArraySeqMtEphSliceS<T>, f: &F,
            Ghost(spec_f): Ghost<spec_fn(T, T) -> T>, id: T,
        ) -> (reduced: T)
            where T: Send + Sync + 'static
            requires
                a.spec_arrayseqmtephslice_wf(),
                a.spec_len() > 0,
                obeys_feq_clone::<T>(),
                spec_monoid(spec_f, id),
                forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) <==> ret == spec_f(x, y),
            ensures
                reduced == a.spec_backing_seq().fold_left(id, spec_f),
            decreases a.spec_len(),
        {
            let len = a.length();
            if len == 1 {
                let elem = a.nth_cloned(0);
                proof {
                    let s = a.spec_backing_seq();
                    assert(s =~= seq![a.spec_index(0)]);
                    reveal_with_fuel(Seq::fold_left, 2);
                    assert(spec_f(id, s[0]) == s[0]);
                }
                elem
            } else {
                let mid = len / 2;
                let left = a.slice(0, mid);
                let right = a.slice(mid, len - mid);

                let f1 = clone_fn2(f);
                let f2 = clone_fn2(f);
                let id1 = id.clone_plus();
                proof { axiom_cloned_implies_eq_owned(id, id1); }
                let id2 = id.clone_plus();
                proof { axiom_cloned_implies_eq_owned(id, id2); }

                let ghost s = a.spec_backing_seq();
                let ghost left_backing = left.spec_backing_seq();
                let ghost right_backing = right.spec_backing_seq();

                // Prove backing seq relationships while left/right are still accessible.
                proof {
                    // left_backing[i] == left.spec_index(i) == a.spec_index(i) == s[i]
                    assert forall|i: int| 0 <= i < mid implies
                        #[trigger] left_backing[i] == s.subrange(0, mid as int)[i]
                    by {
                        assert(left.spec_index(i) == a.spec_index(0int + i));
                        assert(left_backing[i] == left.spec_index(i));
                        assert(s[i] == a.spec_index(i));
                    }
                    assert(left_backing =~= s.subrange(0, mid as int));
                    // right_backing[i] == right.spec_index(i) == a.spec_index(mid+i) == s[mid+i]
                    assert forall|i: int| 0 <= i < (len - mid) as int implies
                        #[trigger] right_backing[i] == s.subrange(mid as int, len as int)[i]
                    by {
                        assert(right.spec_index(i) == a.spec_index(mid as int + i));
                        assert(right_backing[i] == right.spec_index(i));
                        assert(s.subrange(mid as int, len as int)[i] == s[mid as int + i]);
                        assert(s[mid as int + i] == a.spec_index(mid as int + i));
                    }
                    assert(right_backing =~= s.subrange(mid as int, len as int));
                }

                let fa = move || -> (r: T)
                    requires
                        left.spec_arrayseqmtephslice_wf(),
                        left.spec_len() > 0,
                        obeys_feq_clone::<T>(),
                        spec_monoid(spec_f, id),
                        forall|x: &T, y: &T| #[trigger] f1.requires((x, y)),
                        forall|x: T, y: T, ret: T| f1.ensures((&x, &y), ret) <==> ret == spec_f(x, y),
                    ensures
                        r == left_backing.fold_left(id, spec_f),
                {
                    Self::reduce_dc(&left, &f1, Ghost(spec_f), id1)
                };

                let fb = move || -> (r: T)
                    requires
                        right.spec_arrayseqmtephslice_wf(),
                        right.spec_len() > 0,
                        obeys_feq_clone::<T>(),
                        spec_monoid(spec_f, id),
                        forall|x: &T, y: &T| #[trigger] f2.requires((x, y)),
                        forall|x: T, y: T, ret: T| f2.ensures((&x, &y), ret) <==> ret == spec_f(x, y),
                    ensures
                        r == right_backing.fold_left(id, spec_f),
                {
                    Self::reduce_dc(&right, &f2, Ghost(spec_f), id2)
                };

                let (lr, rr) = join(fa, fb);
                let combined = f(&lr, &rr);

                proof {
                    s.lemma_fold_left_split(id, spec_f, mid as int);
                    Self::lemma_monoid_fold_left(right_backing, spec_f, id, lr);
                }
                combined
            }
        }

        /// D&C map producing Vec<U>. Called by trait map.
        fn map_dc_vec<U: Eq + Clone + Send + Sync + 'static, F: MtMapFn<T, U>>(
            a: &ArraySeqMtEphSliceS<T>, f: &F,
        ) -> (result: Vec<U>)
            where T: Send + Sync + 'static
            requires
                a.spec_arrayseqmtephslice_wf(),
                obeys_feq_clone::<T>(),
                obeys_feq_clone::<U>(),
                forall|x: &T| #[trigger] f.requires((x,)),
            ensures
                result@.len() == a.spec_len(),
            decreases a.spec_len(),
        {
            let len = a.length();
            if len == 0 {
                Vec::new()
            } else if len == 1 {
                let elem = a.nth_cloned(0);
                let mut v: Vec<U> = Vec::with_capacity(1);
                v.push(f(&elem));
                v
            } else {
                let mid = len / 2;
                let left = a.slice(0, mid);
                let right = a.slice(mid, len - mid);
                let f1 = clone_fn(f);
                let f2 = clone_fn(f);
                let ghost left_len = left.spec_len();
                let ghost right_len = right.spec_len();

                let fa = move || -> (r: Vec<U>)
                    requires
                        left.spec_arrayseqmtephslice_wf(),
                        obeys_feq_clone::<T>(),
                        obeys_feq_clone::<U>(),
                        forall|x: &T| #[trigger] f1.requires((x,)),
                    ensures
                        r@.len() == left_len,
                {
                    Self::map_dc_vec(&left, &f1)
                };

                let fb = move || -> (r: Vec<U>)
                    requires
                        right.spec_arrayseqmtephslice_wf(),
                        obeys_feq_clone::<T>(),
                        obeys_feq_clone::<U>(),
                        forall|x: &T| #[trigger] f2.requires((x,)),
                    ensures
                        r@.len() == right_len,
                {
                    Self::map_dc_vec(&right, &f2)
                };

                let (mut left_v, right_v) = join(fa, fb);
                let rlen = right_v.len();
                let mut i: usize = 0;
                while i < rlen
                    invariant
                        i <= rlen,
                        rlen == right_v@.len(),
                        rlen as int == right_len,
                        left_v@.len() == left_len + i as int,
                        obeys_feq_clone::<U>(),
                    decreases rlen - i,
                {
                    left_v.push(right_v[i].clone_plus());
                    i = i + 1;
                }
                left_v
            }
        }

        /// D&C filter producing Vec<T>. Called by trait filter.
        fn filter_dc_vec<F: MtPred<T>>(
            a: &ArraySeqMtEphSliceS<T>, pred: &F,
            Ghost(spec_pred): Ghost<spec_fn(T) -> bool>,
        ) -> (result: Vec<T>)
            where T: Send + Sync + 'static
            requires
                a.spec_arrayseqmtephslice_wf(),
                obeys_feq_clone::<T>(),
                forall|x: &T| #[trigger] pred.requires((x,)),
                forall|v: T, keep: bool| pred.ensures((&v,), keep) ==> spec_pred(v) == keep,
            ensures
                result@.len() <= a.spec_len(),
            decreases a.spec_len(),
        {
            let len = a.length();
            if len == 0 {
                Vec::new()
            } else if len == 1 {
                let elem = a.nth_cloned(0);
                if pred(&elem) {
                    let mut v: Vec<T> = Vec::with_capacity(1);
                    v.push(elem);
                    v
                } else {
                    Vec::new()
                }
            } else {
                let mid = len / 2;
                let left = a.slice(0, mid);
                let right = a.slice(mid, len - mid);
                let p1 = clone_pred(pred);
                let p2 = clone_pred(pred);
                let ghost left_len = left.spec_len();
                let ghost right_len = right.spec_len();

                let fa = move || -> (r: Vec<T>)
                    requires
                        left.spec_arrayseqmtephslice_wf(),
                        obeys_feq_clone::<T>(),
                        forall|x: &T| #[trigger] p1.requires((x,)),
                        forall|v: T, keep: bool| p1.ensures((&v,), keep) ==> spec_pred(v) == keep,
                    ensures
                        r@.len() <= left_len,
                {
                    Self::filter_dc_vec(&left, &p1, Ghost(spec_pred))
                };

                let fb = move || -> (r: Vec<T>)
                    requires
                        right.spec_arrayseqmtephslice_wf(),
                        obeys_feq_clone::<T>(),
                        forall|x: &T| #[trigger] p2.requires((x,)),
                        forall|v: T, keep: bool| p2.ensures((&v,), keep) ==> spec_pred(v) == keep,
                    ensures
                        r@.len() <= right_len,
                {
                    Self::filter_dc_vec(&right, &p2, Ghost(spec_pred))
                };

                let (mut left_v, right_v) = join(fa, fb);
                let rlen = right_v.len();
                let mut i: usize = 0;
                while i < rlen
                    invariant
                        i <= rlen,
                        rlen == right_v@.len(),
                        left_v@.len() <= left_len + i as int,
                        obeys_feq_clone::<T>(),
                    decreases rlen - i,
                {
                    left_v.push(right_v[i].clone_plus());
                    i = i + 1;
                }
                left_v
            }
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
