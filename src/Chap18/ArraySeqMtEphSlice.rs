//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

//! Chapter 18 slice-backed array sequence (multi-threaded ephemeral).
//! O(1) slicing via shared `Arc<Vec<T>>` backing with offset/length window.
//! Multiple slices can share the same backing storage; `slice()` is O(1)
//! (just an Arc ref-count bump + window adjust).
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
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;
    use crate::vstdplus::accept::accept;

    // 3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
        vstd::seq_lib::group_seq_properties,
        vstd::std_specs::slice::group_slice_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms,
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

    /// Well-formedness check on a nested ArraySeqMtEphSliceS<ArraySeqMtEphSliceS<T>>.
    /// True when the outer window is valid and every inner window is valid.
    pub open spec fn spec_nested_wf<T>(a: &ArraySeqMtEphSliceS<ArraySeqMtEphSliceS<T>>) -> bool {
        &&& a.start + a.len <= (*a.data)@.len()
        &&& a.start + a.len <= usize::MAX
        &&& forall|i: int| #![trigger (*a.data)@[a.start as int + i]]
            0 <= i < a.len as int ==> {
                let inner = (*a.data)@[a.start as int + i];
                &&& inner.start + inner.len <= (*inner.data)@.len()
                &&& inner.start + inner.len <= usize::MAX
            }
    }

    /// Inclusive prefix fold: fold of the first `n` elements of the sequence
    /// accessed via `spec_index`. Avoids `Seq::take` and `fold_left` to keep
    /// Z3 reasoning purely arithmetic + function application.
    pub open spec fn spec_prefix_fold<T>(
        seq_fn: spec_fn(int) -> T, spec_f: spec_fn(T, T) -> T, id: T, n: int,
    ) -> T
        decreases n,
    {
        if n <= 0 { id }
        else { spec_f(spec_prefix_fold(seq_fn, spec_f, id, n - 1), seq_fn(n - 1)) }
    }

    /// Sum of inner sequence lengths for a nested ArraySeqMtEphSliceS.
    pub open spec fn spec_sum_inner_lens<T>(
        a: &ArraySeqMtEphSliceS<ArraySeqMtEphSliceS<T>>,
    ) -> nat
        decreases a.len,
    {
        if a.len == 0 { 0 }
        else {
            let inner = (*a.data)@[a.start as int];
            let rest = ArraySeqMtEphSliceS::<ArraySeqMtEphSliceS<T>> {
                data: a.data, start: (a.start + 1) as usize, len: (a.len - 1) as usize,
            };
            inner.len as nat + spec_sum_inner_lens(&rest)
        }
    }

    // 7. proof fns

    /// Definition 18.16 (inject). Apply position-value updates left to right; the first update
    /// to each position wins. Out-of-range positions are ignored.
    pub open spec fn spec_inject<T>(s: Seq<T>, updates: Seq<(usize, T)>) -> Seq<T>
        decreases updates.len()
    {
        if updates.len() == 0 {
            s
        } else {
            let rest = spec_inject(s, updates.drop_first());
            let pos = updates[0].0 as int;
            let val = updates[0].1;
            if 0 <= pos < s.len() { rest.update(pos, val) } else { rest }
        }
    }

    /// Definition 18.17 (ninject). The result has the same length as `s`. For each position i,
    /// the value is either the original `s[i]` or some `updates[j].1` where `updates[j].0 == i`.
    pub open spec fn spec_ninject<T>(s: Seq<T>, updates: Seq<(usize, T)>, injected: Seq<T>) -> bool {
        injected.len() == s.len()
        && forall|i: int| #![trigger injected[i]] 0 <= i < s.len() ==> {
            injected[i] == s[i]
            || exists|j: int| #![trigger updates[j]] 0 <= j < updates.len()
                && updates[j].0 == i as usize && injected[i] == updates[j].1
        }
    }

    /// Each element of `spec_inject(s, u)` is either the original `s[i]` or some update value.
    proof fn lemma_spec_inject_element<T>(s: Seq<T>, u: Seq<(usize, T)>, i: int)
        requires 0 <= i < s.len(),
        ensures ({
            let r = spec_inject(s, u);
            r.len() == s.len()
            && (r[i] == s[i]
                || exists|j: int| #![trigger u[j]] 0 <= j < u.len()
                    && u[j].0 == i as usize && r[i] == u[j].1)
        }),
        decreases u.len(),
    {
        reveal(spec_inject);
        if u.len() > 0 {
            lemma_spec_inject_len(s, u.drop_first());
            lemma_spec_inject_element(s, u.drop_first(), i);
            let rest = spec_inject(s, u.drop_first());
            let pos = u[0].0 as int;
            let val = u[0].1;
            if 0 <= pos < s.len() {
                if i == pos {
                } else {
                    if rest[i] != s[i] {
                        let j = choose|j: int| #![trigger u.drop_first()[j]] 0 <= j < u.drop_first().len()
                            && u.drop_first()[j].0 == i as usize
                            && rest[i] == u.drop_first()[j].1;
                        assert(u[j + 1] == u.drop_first()[j]);
                    }
                }
            } else {
                if rest[i] != s[i] {
                    let j = choose|j: int| #![trigger u.drop_first()[j]] 0 <= j < u.drop_first().len()
                        && u.drop_first()[j].0 == i as usize
                        && rest[i] == u.drop_first()[j].1;
                    assert(u[j + 1] == u.drop_first()[j]);
                }
            }
        }
    }

    /// The length of `spec_inject(s, u)` equals `s.len()`.
    proof fn lemma_spec_inject_len<T>(s: Seq<T>, u: Seq<(usize, T)>)
        ensures spec_inject(s, u).len() == s.len(),
        decreases u.len(),
    {
        reveal(spec_inject);
        if u.len() > 0 {
            lemma_spec_inject_len(s, u.drop_first());
        }
    }

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

        /// Parallel tabulate via D&C with join.
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(n * W(f)), Span O(lg n + S(f))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * W(f)), Span O(lg n * S(f)) — D&C + join, O(n) rejoin.
        fn tabulate<F: MtTabulateFn<T>>(f: &F, length: usize) -> (tab: Self)
            where T: Send + Sync + 'static
            requires
                obeys_feq_clone::<T>(),
                forall|i: usize| i < length ==> #[trigger] f.requires((i,)),
            ensures
                tab.spec_arrayseqmtephslice_wf(),
                tab.spec_len() == length as nat,
                forall|i: int| #![trigger tab.spec_index(i)]
                    0 <= i < length ==> f.ensures((i as usize,), tab.spec_index(i));

        /// Parallel inclusive scan via D&C on O(1) slices.
        /// - Alg Analysis: APAS (Ch20 CS 20.5): Work O(|a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n lg n), Span O(n) — D&C + join, O(1) split, sequential prefix adjustment O(n) at each level.
        fn scan<F: MtReduceFn<T>>(
            &self, f: &F, Ghost(spec_f): Ghost<spec_fn(T, T) -> T>, id: T,
        ) -> (scanned: (Self, T))
            where T: Send + Sync + 'static
            requires
                self.spec_arrayseqmtephslice_wf(),
                obeys_feq_clone::<T>(),
                spec_monoid(spec_f, id),
                forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) <==> ret == spec_f(x, y),
            ensures
                scanned.0.spec_arrayseqmtephslice_wf(),
                scanned.0.spec_len() == self.spec_len(),
                forall|i: int| #![trigger scanned.0.spec_index(i)] 0 <= i < self.spec_len() ==>
                    scanned.0.spec_index(i) == self.spec_backing_seq().take(i + 1).fold_left(id, spec_f),
                scanned.1 == self.spec_backing_seq().fold_left(id, spec_f);

        /// - Definition 18.5 (isEmpty). True iff the sequence has length zero.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_empty(&self) -> (empty: bool)
            requires self.spec_arrayseqmtephslice_wf(),
            ensures empty <==> self.spec_len() == 0;

        /// - Definition 18.5 (isSingleton). True iff the sequence has length one.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_singleton(&self) -> (single: bool)
            requires self.spec_arrayseqmtephslice_wf(),
            ensures single <==> self.spec_len() == 1;

        /// - Definition 18.13 (append). Concatenate two sequences.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a| + |b|), Span O(|a| + |b|)
        fn append(a: &Self, b: &Self) -> (appended: Self)
            requires
                a.spec_arrayseqmtephslice_wf(),
                b.spec_arrayseqmtephslice_wf(),
                obeys_feq_clone::<T>(),
                a.spec_len() + b.spec_len() <= usize::MAX as int,
            ensures
                appended.spec_arrayseqmtephslice_wf(),
                appended.spec_len() == a.spec_len() + b.spec_len(),
                forall|i: int| #![trigger appended.spec_index(i)]
                    0 <= i < a.spec_len() ==> appended.spec_index(i) == a.spec_index(i),
                forall|i: int| #![trigger appended.spec_index(a.spec_len() as int + i)]
                    0 <= i < b.spec_len() ==> appended.spec_index(a.spec_len() as int + i) == b.spec_index(i);

        /// - Definition 18.16 (update). Return a copy with the index replaced by the new value.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn update(&self, index: usize, item: T) -> (updated: Self)
            requires
                self.spec_arrayseqmtephslice_wf(),
                obeys_feq_clone::<T>(),
                index < self.spec_len(),
            ensures
                updated.spec_arrayseqmtephslice_wf(),
                updated.spec_len() == self.spec_len(),
                updated.spec_index(index as int) == item,
                forall|i: int| #![trigger updated.spec_index(i)]
                    0 <= i < self.spec_len() && i != index as int ==> updated.spec_index(i) == self.spec_index(i);

        /// - Definition 18.16 (inject). Update multiple positions at once.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + m), Span O(n + m)
        fn inject(&self, updates: &Vec<(usize, T)>) -> (injected: Self)
            requires
                self.spec_arrayseqmtephslice_wf(),
                obeys_feq_clone::<T>(),
            ensures
                injected.spec_arrayseqmtephslice_wf(),
                injected.spec_len() == self.spec_len(),
                Seq::new(injected.spec_len(), |i: int| injected.spec_index(i))
                    =~= spec_inject(
                        Seq::new(self.spec_len(), |i: int| self.spec_index(i)),
                        updates@);

        /// - Definition 18.17 (ninject). Nondeterministic inject.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + m), Span O(n + m)
        fn ninject(&self, updates: &Vec<(usize, T)>) -> (injected: Self)
            requires
                self.spec_arrayseqmtephslice_wf(),
                obeys_feq_clone::<T>(),
            ensures
                injected.spec_arrayseqmtephslice_wf(),
                spec_ninject(
                    Seq::new(self.spec_len(), |i: int| self.spec_index(i)),
                    updates@,
                    Seq::new(injected.spec_len(), |i: int| injected.spec_index(i)));

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
                reduce_dc(self, f, Ghost(spec_f), id)
            }
        }

        fn map<U: Eq + Clone + Send + Sync + 'static, F: MtMapFn<T, U>>(
            &self, f: &F,
        ) -> (mapped: ArraySeqMtEphSliceS<U>)
            where T: Send + Sync + 'static
        {
            let v = map_dc_vec(self, f);
            ArraySeqMtEphSliceS::<U>::from_vec(v)
        }

        fn filter<F: MtPred<T>>(
            &self, pred: &F, Ghost(spec_pred): Ghost<spec_fn(T) -> bool>,
        ) -> (filtered: Self)
            where T: Send + Sync + 'static
        {
            let v = filter_dc_vec(self, pred, Ghost(spec_pred));
            Self::from_vec(v)
        }

        fn tabulate<F: MtTabulateFn<T>>(f: &F, length: usize) -> (tab: Self)
            where T: Send + Sync + 'static
        {
            let v = tabulate_dc_vec(f, 0, length);
            let ghost v_view = v@;
            let tab = Self::from_vec(v);
            proof {
                assert forall|i: int| 0 <= i < length implies
                    #[trigger] f.ensures((i as usize,), tab.spec_index(i))
                by {
                    assert(tab.spec_index(i) == v_view[i]);
                    assert(f.ensures((i as usize,), v_view[i]));
                }
            }
            tab
        }

        fn scan<F: MtReduceFn<T>>(
            &self, f: &F, Ghost(spec_f): Ghost<spec_fn(T, T) -> T>, id: T,
        ) -> (scanned: (Self, T))
            where T: Send + Sync + 'static
        {
            let len = self.length();
            if len == 0 {
                proof {
                    assert(self.spec_backing_seq() =~= Seq::<T>::empty());
                    reveal(Seq::fold_left);
                }
                (Self::empty(), id)
            } else {
                let (v, total) = scan_dc_vec(self, f, Ghost(spec_f), id);
                let ghost s = self.spec_backing_seq();
                let ghost a_fn = |i: int| self.spec_index(i);
                let result = Self::from_vec(v);
                proof {
                    assert forall|i: int| #![trigger result.spec_index(i)]
                        0 <= i < self.spec_len() implies
                        result.spec_index(i) == s.take(i + 1).fold_left(id, spec_f)
                    by {
                        lemma_prefix_fold_eq_fold_left(s, a_fn, spec_f, id, i + 1);
                    }
                    lemma_prefix_fold_eq_fold_left(s, a_fn, spec_f, id, self.spec_len() as int);
                }
                (result, total)
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_empty(&self) -> (empty: bool) {
            self.len == 0
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_singleton(&self) -> (single: bool) {
            self.len == 1
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a| + |b|), Span O(|a| + |b|)
        fn append(a: &Self, b: &Self) -> (appended: Self) {
            let mut v: Vec<T> = Vec::with_capacity(a.len + b.len);
            let a_backing: &Vec<T> = arc_deref(&a.data);
            let b_backing: &Vec<T> = arc_deref(&b.data);
            let mut i: usize = 0;
            while i < a.len
                invariant
                    i <= a.len,
                    a.spec_arrayseqmtephslice_wf(),
                    obeys_feq_clone::<T>(),
                    v@.len() == i as int,
                    a_backing@ == (*a.data)@,
                    forall|j: int| #![trigger v@[j]]
                        0 <= j < i as int ==> v@[j] == a.spec_index(j),
                decreases a.len - i,
            {
                v.push(a_backing[a.start + i].clone_plus());
                i = i + 1;
            }
            let mut k: usize = 0;
            while k < b.len
                invariant
                    k <= b.len,
                    b.spec_arrayseqmtephslice_wf(),
                    obeys_feq_clone::<T>(),
                    v@.len() == a.len as int + k as int,
                    b_backing@ == (*b.data)@,
                    forall|j: int| #![trigger v@[j]]
                        0 <= j < a.len as int ==> v@[j] == a.spec_index(j),
                    forall|j: int| #![trigger v@[a.len as int + j]]
                        0 <= j < k as int ==> v@[a.len as int + j] == b.spec_index(j),
                decreases b.len - k,
            {
                v.push(b_backing[b.start + k].clone_plus());
                k = k + 1;
            }
            Self::from_vec(v)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn update(&self, index: usize, item: T) -> (updated: Self) {
            let mut v = self.to_vec();
            v.set(index, item);
            let ghost old_v = v@;
            let result = Self::from_vec(v);
            proof {
                assert forall|i: int| #![trigger result.spec_index(i)]
                    0 <= i < self.spec_len() && i != index as int
                implies result.spec_index(i) == self.spec_index(i)
                by {
                    assert(result.spec_index(i) == old_v[i]);
                }
            }
            result
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + m), Span O(n + m)
        fn inject(&self, updates: &Vec<(usize, T)>) -> (injected: Self) {
            let ghost s = Seq::new(self.spec_len(), |i: int| self.spec_index(i));
            let ghost u = updates@;
            let len = self.length();
            let ulen = updates.len();

            // Copy backing into result_vec.
            let mut result_vec: Vec<T> = Vec::with_capacity(len);
            let mut k: usize = 0;
            while k < len
                invariant
                    k <= len,
                    len as int == self.spec_len(),
                    self.spec_arrayseqmtephslice_wf(),
                    obeys_feq_clone::<T>(),
                    s == Seq::new(self.spec_len(), |i: int| self.spec_index(i)),
                    result_vec@.len() == k as int,
                    forall|j: int| #![trigger result_vec@[j]]
                        0 <= j < k as int ==> result_vec@[j] == self.spec_index(j),
                decreases len - k,
            {
                result_vec.push(self.nth_cloned(k));
                k = k + 1;
            }
            proof {
                assert(result_vec@ =~= s);
            }

            // Apply updates from end to front (matches spec_inject recursion).
            let mut i: usize = ulen;
            while i > 0
                invariant
                    0 <= i <= ulen,
                    ulen == u.len(),
                    len as int == self.spec_len(),
                    result_vec@.len() == s.len(),
                    s.len() == len as int,
                    obeys_feq_clone::<T>(),
                    s == Seq::new(self.spec_len(), |j: int| self.spec_index(j)),
                    u == updates@,
                    result_vec@ =~= spec_inject(s, u.subrange(i as int, ulen as int)),
                decreases i,
            {
                i = i - 1;
                let pos = updates[i].0;
                if pos < len {
                    let val = updates[i].1.clone_plus();
                    proof {
                        axiom_cloned_implies_eq_owned(u[i as int].1, val);
                    }
                    result_vec.set(pos, val);
                }
                proof {
                    let ghost sub = u.subrange(i as int, ulen as int);
                    assert(sub.len() > 0);
                    assert(sub[0] == u[i as int]);
                    assert(sub.drop_first() =~= u.subrange(i as int + 1, ulen as int));
                    reveal(spec_inject);
                }
            }

            proof {
                assert(u.subrange(0, ulen as int) =~= u);
                assert(result_vec@ =~= spec_inject(s, u));
            }
            let injected = Self::from_vec(result_vec);
            proof {
                assert(Seq::new(injected.spec_len(), |i: int| injected.spec_index(i)) =~= result_vec@);
            }
            injected
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + m), Span O(n + m)
        fn ninject(&self, updates: &Vec<(usize, T)>) -> (injected: Self) {
            // Delegates to inject; ninject is nondeterministic and inject is a valid choice.
            let result = self.inject(updates);
            proof {
                let ghost s = Seq::new(self.spec_len(), |i: int| self.spec_index(i));
                let ghost r = Seq::new(result.spec_len(), |i: int| result.spec_index(i));
                lemma_spec_inject_len(s, updates@);
                assert forall|i: int| #![trigger r[i]] 0 <= i < s.len() implies {
                    r[i] == s[i]
                    || exists|j: int| #![trigger updates@[j]] 0 <= j < updates@.len()
                        && updates@[j].0 == i as usize && r[i] == updates@[j].1
                } by {
                    lemma_spec_inject_element(s, updates@, i);
                }
            }
            result
        }

    }

    // 9b. free functions — D&C helpers and proof fns

    /// For a monoid (f, id): f(x, s.fold_left(id, f)) == s.fold_left(x, f).
    pub(crate) proof fn lemma_monoid_fold_left<T>(s: Seq<T>, f: spec_fn(T, T) -> T, id: T, x: T)
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
                lemma_monoid_fold_left(s1, f, id, x);

                assert(f(x, f(lid, a_last)) == f(f(x, lid), a_last));
            }
        }


    pub(crate) proof fn lemma_prefix_fold_matching<T>(
        f1: spec_fn(int) -> T, f2: spec_fn(int) -> T,
        spec_f: spec_fn(T, T) -> T, id: T, n: int,
    )
            requires
                0 <= n,
                forall|k: int| 0 <= k < n ==> #[trigger] f1(k) == f2(k),
            ensures
                spec_prefix_fold(f1, spec_f, id, n) == spec_prefix_fold(f2, spec_f, id, n),
            decreases n,
        {
            if n > 0 {
                lemma_prefix_fold_matching(f1, f2, spec_f, id, n - 1);
            }
        }

    /// Monoid split: prefix_fold(a, f, id, m+k) = f(prefix_fold(a, f, id, m), prefix_fold(shifted_a, f, id, k)).
    pub(crate) proof fn lemma_prefix_fold_split<T>(
        a_fn: spec_fn(int) -> T, spec_f: spec_fn(T, T) -> T, id: T, m: int, k: int,
    )
            requires
                spec_monoid(spec_f, id),
                m >= 0, k >= 0,
            ensures
                spec_prefix_fold(a_fn, spec_f, id, m + k)
                    == spec_f(
                        spec_prefix_fold(a_fn, spec_f, id, m),
                        spec_prefix_fold(|j: int| a_fn(m + j), spec_f, id, k),
                    ),
            decreases k,
        {
            if k == 0 {
                // RHS = f(prefix_fold(m), id) = prefix_fold(m) by right identity.
            } else {
                lemma_prefix_fold_split(a_fn, spec_f, id, m, k - 1);
                // prefix_fold(a, f, id, m + k)
                // = f(prefix_fold(a, f, id, m + k - 1), a(m + k - 1))
                // = f(f(prefix_fold(a, f, id, m), prefix_fold(shifted, f, id, k-1)), a(m + k - 1))   [by IH]
                // = f(prefix_fold(a, f, id, m), f(prefix_fold(shifted, f, id, k-1), a(m + k - 1)))    [by assoc]
                // = f(prefix_fold(a, f, id, m), prefix_fold(shifted, f, id, k))
                // since shifted(k-1) = a(m + k - 1)
                let left = spec_prefix_fold(a_fn, spec_f, id, m);
                let right_prev = spec_prefix_fold(|j: int| a_fn(m + j), spec_f, id, k - 1);
                let elem = a_fn(m + k - 1);
                assert(spec_prefix_fold(a_fn, spec_f, id, m + k) == spec_f(spec_f(left, right_prev), elem));
                // shifted_a(k-1) == a_fn(m + (k-1)) == a_fn(m + k - 1) == elem
                assert((|j: int| a_fn(m + j))(k - 1) == elem);
                assert(spec_prefix_fold(|j: int| a_fn(m + j), spec_f, id, k) == spec_f(right_prev, elem));
            }
        }

    /// Connect spec_prefix_fold to spec_backing_seq().take(n).fold_left(id, f).
    pub(crate) proof fn lemma_prefix_fold_eq_fold_left<T>(
        s: Seq<T>, seq_fn: spec_fn(int) -> T, spec_f: spec_fn(T, T) -> T, id: T, n: int,
    )
            requires
                0 <= n <= s.len(),
                forall|k: int| 0 <= k < s.len() ==> #[trigger] seq_fn(k) == s[k],
            ensures
                spec_prefix_fold(seq_fn, spec_f, id, n) == s.take(n).fold_left(id, spec_f),
            decreases n,
        {
            reveal(Seq::fold_left);
            if n > 0 {
                lemma_prefix_fold_eq_fold_left(s, seq_fn, spec_f, id, n - 1);
                assert(s.take(n).drop_last() =~= s.take(n - 1));
            }
        }

    /// D&C reduce on O(1) slices. Called by trait reduce for non-empty sequences.
    pub(crate) fn reduce_dc<T: Eq + Clone + Send + Sync + 'static, F: MtReduceFn<T>>(
        a: &ArraySeqMtEphSliceS<T>, f: &F,
        Ghost(spec_f): Ghost<spec_fn(T, T) -> T>, id: T,
    ) -> (reduced: T)
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
                    reduce_dc(&left, &f1, Ghost(spec_f), id1)
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
                    reduce_dc(&right, &f2, Ghost(spec_f), id2)
                };

                let (lr, rr) = join(fa, fb);
                let combined = f(&lr, &rr);

                proof {
                    s.lemma_fold_left_split(id, spec_f, mid as int);
                    lemma_monoid_fold_left(right_backing, spec_f, id, lr);
                }
                combined
            }
        }

    /// D&C map producing Vec<U>. Called by trait map.
    pub(crate) fn map_dc_vec<T: Eq + Clone + Send + Sync + 'static, U: Eq + Clone + Send + Sync + 'static, F: MtMapFn<T, U>>(
        a: &ArraySeqMtEphSliceS<T>, f: &F,
    ) -> (mapped: Vec<U>)
        requires
                a.spec_arrayseqmtephslice_wf(),
                obeys_feq_clone::<T>(),
                obeys_feq_clone::<U>(),
                forall|x: &T| #[trigger] f.requires((x,)),
            ensures
                mapped@.len() == a.spec_len(),
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
                    map_dc_vec(&left, &f1)
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
                    map_dc_vec(&right, &f2)
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
    pub(crate) fn filter_dc_vec<T: Eq + Clone + Send + Sync + 'static, F: MtPred<T>>(
        a: &ArraySeqMtEphSliceS<T>, pred: &F,
        Ghost(spec_pred): Ghost<spec_fn(T) -> bool>,
    ) -> (filtered: Vec<T>)
        requires
                a.spec_arrayseqmtephslice_wf(),
                obeys_feq_clone::<T>(),
                forall|x: &T| #[trigger] pred.requires((x,)),
                forall|v: T, keep: bool| pred.ensures((&v,), keep) ==> spec_pred(v) == keep,
            ensures
                filtered@.len() <= a.spec_len(),
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
                    filter_dc_vec(&left, &p1, Ghost(spec_pred))
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
                    filter_dc_vec(&right, &p2, Ghost(spec_pred))
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

    /// D&C tabulate producing Vec<T>. Called by trait tabulate.
    pub(crate) fn tabulate_dc_vec<T: Eq + Clone + Send + Sync + 'static, F: MtTabulateFn<T>>(
        f: &F, start: usize, count: usize,
    ) -> (tabulated: Vec<T>)
        requires
                start + count <= usize::MAX,
                obeys_feq_clone::<T>(),
                forall|i: usize| start <= i < start + count ==> #[trigger] f.requires((i,)),
            ensures
                tabulated@.len() == count as int,
                forall|i: int| #![trigger tabulated@[i]]
                    0 <= i < count ==> f.ensures(((start as int + i) as usize,), tabulated@[i]),
            decreases count,
        {
            if count == 0 {
                Vec::new()
            } else if count == 1 {
                let mut v: Vec<T> = Vec::with_capacity(1);
                v.push(f(start));
                v
            } else {
                let mid = count / 2;
                let right_start = start + mid;
                let right_count = count - mid;
                let f1 = clone_fn_usize(f);
                let f2 = clone_fn_usize(f);
                let ghost left_len = mid as nat;
                let ghost right_len = right_count as nat;

                let fa = move || -> (r: Vec<T>)
                    requires
                        start + mid <= usize::MAX,
                        obeys_feq_clone::<T>(),
                        forall|i: usize| start <= i < start + mid ==> #[trigger] f1.requires((i,)),
                    ensures
                        r@.len() == left_len,
                        forall|i: int| #![trigger r@[i]]
                            0 <= i < left_len ==> f1.ensures(((start as int + i) as usize,), r@[i]),
                {
                    tabulate_dc_vec(&f1, start, mid)
                };

                let fb = move || -> (r: Vec<T>)
                    requires
                        right_start + right_count <= usize::MAX,
                        obeys_feq_clone::<T>(),
                        forall|i: usize| right_start <= i < right_start + right_count
                            ==> #[trigger] f2.requires((i,)),
                    ensures
                        r@.len() == right_len,
                        forall|i: int| #![trigger r@[i]]
                            0 <= i < right_len ==> f2.ensures(((right_start as int + i) as usize,), r@[i]),
                {
                    tabulate_dc_vec(&f2, right_start, right_count)
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
                        obeys_feq_clone::<T>(),
                        forall|j: int| #![trigger left_v@[j]]
                            0 <= j < left_len as int
                            ==> f.ensures(((start as int + j) as usize,), left_v@[j]),
                        forall|j: int|
                            0 <= j < i as int
                            ==> #[trigger] left_v@[left_len as int + j] == right_v@[j],
                        forall|j: int| #![trigger right_v@[j]]
                            0 <= j < right_len as int
                            ==> f.ensures(((right_start as int + j) as usize,), right_v@[j]),
                    decreases rlen - i,
                {
                    left_v.push(right_v[i].clone_plus());
                    i = i + 1;
                }
                proof {
                    assert forall|j: int| 0 <= j < count implies
                        #[trigger] f.ensures(((start as int + j) as usize,), left_v@[j])
                    by {
                        if j < left_len as int {
                            assert(f.ensures(((start as int + j) as usize,), left_v@[j]));
                        } else {
                            let k = j - left_len as int;
                            assert(left_v@[left_len as int + k] == right_v@[k]);
                            assert(f.ensures(((right_start as int + k) as usize,), right_v@[k]));
                        }
                    }
                }
                left_v
            }
        }


    pub(crate) fn scan_dc_vec<T: Eq + Clone + Send + Sync + 'static, F: MtReduceFn<T>>(
        a: &ArraySeqMtEphSliceS<T>, f: &F,
        Ghost(spec_f): Ghost<spec_fn(T, T) -> T>, id: T,
    ) -> (scanned: (Vec<T>, T))
        requires
                a.spec_arrayseqmtephslice_wf(),
                a.spec_len() > 0,
                obeys_feq_clone::<T>(),
                spec_monoid(spec_f, id),
                forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) <==> ret == spec_f(x, y),
            ensures ({
                let a_fn = |i: int| a.spec_index(i);
                &&& scanned.0@.len() == a.spec_len()
                &&& forall|i: int| #![trigger scanned.0@[i]] 0 <= i < a.spec_len() ==>
                        scanned.0@[i] == spec_prefix_fold(a_fn, spec_f, id, i + 1)
                &&& scanned.1 == spec_prefix_fold(a_fn, spec_f, id, a.spec_len() as int)
            }),
            decreases a.spec_len(),
        {
            let len = a.length();
            let ghost a_fn = |i: int| a.spec_index(i);
            if len == 1 {
                let elem = a.nth_cloned(0);
                let elem2 = elem.clone_plus();
                proof {
                    axiom_cloned_implies_eq_owned(elem, elem2);
                    // spec_prefix_fold(a_fn, f, id, 1) = f(prefix_fold(a_fn, f, id, 0), a_fn(0)) = f(id, a[0]) = a[0].
                    // Unfold one step: n=1 > 0 so prefix_fold(1) = f(prefix_fold(0), a_fn(0)) = f(id, a[0]).
                    assert(spec_prefix_fold(a_fn, spec_f, id, 0int) == id);
                    assert(spec_prefix_fold(a_fn, spec_f, id, 1int)
                        == spec_f(spec_prefix_fold(a_fn, spec_f, id, 0int), a_fn(0int)));
                    assert(a_fn(0int) == a.spec_index(0));
                }
                let mut v: Vec<T> = Vec::with_capacity(1);
                v.push(elem);
                (v, elem2)
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

                let ghost left_fn = |i: int| left.spec_index(i);
                let ghost right_fn = |i: int| right.spec_index(i);
                let ghost left_len = left.spec_len();
                let ghost right_len = right.spec_len();


                // From slice ensures: left_fn(k) == a_fn(k), right_fn(k) == a_fn(mid + k).

                let fa = move || -> (r: (Vec<T>, T))
                    requires
                        left.spec_arrayseqmtephslice_wf(),
                        left.spec_len() > 0,
                        obeys_feq_clone::<T>(),
                        spec_monoid(spec_f, id),
                        forall|x: &T, y: &T| #[trigger] f1.requires((x, y)),
                        forall|x: T, y: T, ret: T| f1.ensures((&x, &y), ret) <==> ret == spec_f(x, y),
                    ensures ({
                        let lf = |i: int| left.spec_index(i);
                        &&& r.0@.len() == left_len
                        &&& forall|i: int| #![trigger r.0@[i]] 0 <= i < left_len ==>
                                r.0@[i] == spec_prefix_fold(lf, spec_f, id, i + 1)
                        &&& r.1 == spec_prefix_fold(lf, spec_f, id, left_len as int)
                    }),
                {
                    scan_dc_vec(&left, &f1, Ghost(spec_f), id1)
                };

                let fb = move || -> (r: (Vec<T>, T))
                    requires
                        right.spec_arrayseqmtephslice_wf(),
                        right.spec_len() > 0,
                        obeys_feq_clone::<T>(),
                        spec_monoid(spec_f, id),
                        forall|x: &T, y: &T| #[trigger] f2.requires((x, y)),
                        forall|x: T, y: T, ret: T| f2.ensures((&x, &y), ret) <==> ret == spec_f(x, y),
                    ensures ({
                        let rf = |i: int| right.spec_index(i);
                        &&& r.0@.len() == right_len
                        &&& forall|i: int| #![trigger r.0@[i]] 0 <= i < right_len ==>
                                r.0@[i] == spec_prefix_fold(rf, spec_f, id, i + 1)
                        &&& r.1 == spec_prefix_fold(rf, spec_f, id, right_len as int)
                    }),
                {
                    scan_dc_vec(&right, &f2, Ghost(spec_f), id2)
                };

                let ((left_vec, left_total), (right_vec, right_total)) = join(fa, fb);

                // Build result.
                let mut result_vec: Vec<T> = Vec::with_capacity(len);
                let rlen = len - mid;

                // Copy left prefixes.
                let mut i: usize = 0;
                while i < mid
                    invariant
                        i <= mid,
                        mid as int == left_len,
                        result_vec@.len() == i as int,
                        left_vec@.len() == left_len,
                        obeys_feq_clone::<T>(),
                        forall|j: int| #![trigger left_vec@[j]] 0 <= j < left_len ==>
                            left_vec@[j] == spec_prefix_fold(left_fn, spec_f, id, j + 1),
                        forall|j: int| #![trigger result_vec@[j]] 0 <= j < i as int ==>
                            result_vec@[j] == spec_prefix_fold(a_fn, spec_f, id, j + 1),
                        forall|k: int| 0 <= k < mid as int ==> #[trigger] left_fn(k) == a_fn(k),
                    decreases mid - i,
                {
                    let elem = left_vec[i].clone_plus();
                    proof {
                        axiom_cloned_implies_eq_owned(left_vec@[i as int], elem);
                        lemma_prefix_fold_matching(left_fn, a_fn, spec_f, id, i as int + 1);
                    }
                    result_vec.push(elem);
                    i += 1;
                }

                // Adjusted right prefixes.
                let mut j: usize = 0;
                while j < rlen
                    invariant
                        j <= rlen,
                        rlen == len - mid,
                        rlen as int == right_len,
                        mid as int == left_len,
                        len as int == a.spec_len(),
                        result_vec@.len() == mid as int + j as int,
                        right_vec@.len() == right_len,
                        left_total == spec_prefix_fold(left_fn, spec_f, id, left_len as int),
                        spec_monoid(spec_f, id),
                        obeys_feq_clone::<T>(),
                        forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                        forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) <==> ret == spec_f(x, y),
                        forall|k: int| #![trigger right_vec@[k]] 0 <= k < right_len ==>
                            right_vec@[k] == spec_prefix_fold(right_fn, spec_f, id, k + 1),
                        forall|k: int| #![trigger result_vec@[k]] 0 <= k < mid as int + j as int ==>
                            result_vec@[k] == spec_prefix_fold(a_fn, spec_f, id, k + 1),
                        forall|k: int| 0 <= k < mid as int ==> #[trigger] left_fn(k) == a_fn(k),
                        forall|k: int| 0 <= k < right_len ==> #[trigger] right_fn(k) == a_fn(mid as int + k),
                    decreases rlen - j,
                {
                    let adjusted = f(&left_total, &right_vec[j]);
                    proof {
                        // left_total == prefix_fold(left_fn, mid) == prefix_fold(a_fn, mid)
                        lemma_prefix_fold_matching(left_fn, a_fn, spec_f, id, mid as int);
                        // prefix_fold(a_fn, mid + j + 1) = f(prefix_fold(a_fn, mid), prefix_fold(shifted_a, j+1))
                        lemma_prefix_fold_split(a_fn, spec_f, id, mid as int, j as int + 1);
                        // shifted_a(k) == a_fn(mid + k) == right_fn(k)
                        lemma_prefix_fold_matching(|k: int| a_fn(mid as int + k), right_fn, spec_f, id, j as int + 1);
                    }
                    result_vec.push(adjusted);
                    j += 1;
                }

                let total = f(&left_total, &right_total);
                proof {
                    lemma_prefix_fold_matching(left_fn, a_fn, spec_f, id, mid as int);
                    lemma_prefix_fold_split(a_fn, spec_f, id, mid as int, right_len as int);
                    lemma_prefix_fold_matching(|k: int| a_fn(mid as int + k), right_fn, spec_f, id, right_len as int);
                }

                (result_vec, total)
            }
        }

    // 9c. proof fns — flatten

    /// Split lemma: spec_sum_inner_lens over a contiguous window equals the
    /// sum of the left half plus the right half.
    pub(crate) proof fn lemma_sum_inner_lens_split<T>(
        a: &ArraySeqMtEphSliceS<ArraySeqMtEphSliceS<T>>,
        mid: usize,
    )
        requires
            mid <= a.len,
            a.start + a.len <= (*a.data)@.len(),
            a.start + a.len <= usize::MAX,
        ensures ({
            let left = ArraySeqMtEphSliceS::<ArraySeqMtEphSliceS<T>> {
                data: a.data, start: a.start, len: mid,
            };
            let right = ArraySeqMtEphSliceS::<ArraySeqMtEphSliceS<T>> {
                data: a.data, start: (a.start + mid) as usize, len: (a.len - mid) as usize,
            };
            spec_sum_inner_lens(a) == spec_sum_inner_lens(&left) + spec_sum_inner_lens(&right)
        }),
        decreases mid,
    {
        if mid > 0 {
            // Peel first element from both a and left.
            let a_rest = ArraySeqMtEphSliceS::<ArraySeqMtEphSliceS<T>> {
                data: a.data, start: (a.start + 1) as usize, len: (a.len - 1) as usize,
            };
            lemma_sum_inner_lens_split(&a_rest, (mid - 1) as usize);
        }
    }

    // 9d. free functions — flatten

    /// Parallel flatten via D&C on O(1) slices.
    /// - Primitive: flatten. Concatenate a sequence of sequences.
    /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(sum |a[i]|), Span O(lg |a| + max |a[i]|)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(sum |a[i]|), Span O(lg^2 |a| + max |a[i]|) — ACCEPTED DIFFERENCE: Vec concat at each D&C level; O(1) rejoin needs PCell pre-allocated output
    pub fn flatten<T: Eq + Clone + Send + Sync + 'static>(
        a: &ArraySeqMtEphSliceS<ArraySeqMtEphSliceS<T>>,
    ) -> (flattened: ArraySeqMtEphSliceS<T>)
        requires
            spec_nested_wf(a),
            obeys_feq_clone::<T>(),
        ensures
            flattened.spec_arrayseqmtephslice_wf(),
            flattened.spec_len() == spec_sum_inner_lens(a),
    {
        let v = flatten_dc_vec(a);
        let flattened = ArraySeqMtEphSliceS::<T>::from_vec(v);
        flattened
    }

    /// D&C flatten producing Vec<T>. Called by flatten.
    fn flatten_dc_vec<T: Eq + Clone + Send + Sync + 'static>(
        a: &ArraySeqMtEphSliceS<ArraySeqMtEphSliceS<T>>,
    ) -> (flattened: Vec<T>)
        requires
            spec_nested_wf(a),
            obeys_feq_clone::<T>(),
        ensures
            flattened@.len() == spec_sum_inner_lens(a),
        decreases a.len,
    {
        let len = a.len;
        if len == 0 {
            Vec::new()
        } else if len == 1 {
            let backing: &Vec<ArraySeqMtEphSliceS<T>> = arc_deref(&a.data);
            let inner: &ArraySeqMtEphSliceS<T> = &backing[a.start];
            assert(inner == (*a.data)@[a.start as int + 0]);
            let v = inner.to_vec();
            proof {
                // spec_sum_inner_lens(a) with len==1 unfolds to
                // inner.len + spec_sum_inner_lens(rest) where rest.len==0, so inner.len + 0.
                let rest = ArraySeqMtEphSliceS::<ArraySeqMtEphSliceS<T>> {
                    data: a.data, start: (a.start + 1) as usize, len: 0usize,
                };
                assert(spec_sum_inner_lens(&rest) == 0nat);
                assert(spec_sum_inner_lens(a) == inner.len as nat + spec_sum_inner_lens(&rest));
                assert(v@.len() == inner.spec_len());
            }
            v
        } else {
            let mid = len / 2;
            // O(1) slice of the outer sequence.
            let left = ArraySeqMtEphSliceS::<ArraySeqMtEphSliceS<T>> {
                data: Arc::clone(&a.data),
                start: a.start,
                len: mid,
            };
            let right = ArraySeqMtEphSliceS::<ArraySeqMtEphSliceS<T>> {
                data: Arc::clone(&a.data),
                start: a.start + mid,
                len: len - mid,
            };

            let ghost left_sum = spec_sum_inner_lens(&left);
            let ghost right_sum = spec_sum_inner_lens(&right);

            // Propagate nested wf to left and right halves.
            proof {
                assert forall|i: int| #![trigger (*left.data)@[left.start as int + i]]
                    0 <= i < left.len as int implies {
                    let inner = (*left.data)@[left.start as int + i];
                    &&& inner.start + inner.len <= (*inner.data)@.len()
                    &&& inner.start + inner.len <= usize::MAX
                } by {
                    assert((*left.data)@[left.start as int + i]
                        == (*a.data)@[a.start as int + i]);
                }
                assert(spec_nested_wf(&left));

                assert forall|i: int| #![trigger (*right.data)@[right.start as int + i]]
                    0 <= i < right.len as int implies {
                    let inner = (*right.data)@[right.start as int + i];
                    &&& inner.start + inner.len <= (*inner.data)@.len()
                    &&& inner.start + inner.len <= usize::MAX
                } by {
                    assert((*right.data)@[right.start as int + i]
                        == (*a.data)@[a.start as int + (mid as int + i)]);
                }
                assert(spec_nested_wf(&right));

                // Connect Arc::clone-based split to a.data-based split for the lemma.
                lemma_sum_inner_lens_split(a, mid);
            }

            let fa = move || -> (r: Vec<T>)
                requires
                    spec_nested_wf(&left),
                    obeys_feq_clone::<T>(),
                ensures
                    r@.len() == left_sum,
            {
                flatten_dc_vec(&left)
            };

            let fb = move || -> (r: Vec<T>)
                requires
                    spec_nested_wf(&right),
                    obeys_feq_clone::<T>(),
                ensures
                    r@.len() == right_sum,
            {
                flatten_dc_vec(&right)
            };

            let (mut left_v, right_v) = join(fa, fb);
            let rlen = right_v.len();
            let mut i: usize = 0;
            while i < rlen
                invariant
                    i <= rlen,
                    rlen == right_v@.len(),
                    left_v@.len() == left_sum + i as int,
                    obeys_feq_clone::<T>(),
                decreases rlen - i,
            {
                left_v.push(right_v[i].clone_plus());
                i = i + 1;
            }
            left_v
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

    // 9c. PartialEqSpecImpl — struct ArraySeqMtEphSliceS

    #[cfg(verus_keep_ghost)]
    impl<T: View + PartialEq + Eq + Clone> PartialEqSpecImpl for ArraySeqMtEphSliceS<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    // 12. derive impls in verus! — struct ArraySeqMtEphSliceS

    impl<T: Clone> Clone for ArraySeqMtEphSliceS<T> {
        fn clone(&self) -> (cloned: Self)
            ensures
                cloned.data == self.data,
                cloned.start == self.start,
                cloned.len == self.len,
        {
            ArraySeqMtEphSliceS {
                data: Arc::clone(&self.data),
                start: self.start,
                len: self.len,
            }
        }
    }

    impl<T: PartialEq + View + Eq + Clone> PartialEq for ArraySeqMtEphSliceS<T> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            let equal = self.start == other.start && self.len == other.len;
            proof { assume(equal == (self@ == other@)); }
            equal
        }
    }

    impl<T: Eq + View + Clone> Eq for ArraySeqMtEphSliceS<T> {}

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

    impl<'a, T: std::fmt::Debug> std::fmt::Debug for ArraySeqMtEphSliceIter<'a, T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ArraySeqMtEphSliceIter({:?})", self.inner)
        }
    }

    impl<'a, T> std::fmt::Display for ArraySeqMtEphSliceIter<'a, T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ArraySeqMtEphSliceIter")
        }
    }

    impl<'a, T> std::fmt::Debug for ArraySeqMtEphSliceGhostIterator<'a, T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ArraySeqMtEphSliceGhostIterator")
        }
    }

    impl<'a, T> std::fmt::Display for ArraySeqMtEphSliceGhostIterator<'a, T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ArraySeqMtEphSliceGhostIterator")
        }
    }

    #[macro_export]
    macro_rules! Chap18ArraySeqMtEphSliceSLit {
        () => {
            $crate::Chap18::ArraySeqMtEphSlice::ArraySeqMtEphSlice::ArraySeqMtEphSliceS::from_vec(
                Vec::new(),
            )
        };
        ($x:expr; $n:expr) => {
            $crate::Chap18::ArraySeqMtEphSlice::ArraySeqMtEphSlice::ArraySeqMtEphSliceS::from_vec(
                vec![$x; $n],
            )
        };
        ($($x:expr),* $(,)?) => {
            $crate::Chap18::ArraySeqMtEphSlice::ArraySeqMtEphSlice::ArraySeqMtEphSliceS::from_vec(
                vec![$($x),*],
            )
        };
    }
}
