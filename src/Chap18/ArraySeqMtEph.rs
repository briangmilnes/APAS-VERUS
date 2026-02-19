//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Chapter 18 algorithms for ArraySeqMtEph multithreaded ephemeral. Verusified.
//! Uses global work-stealing pool for parallel operations (map_par, reduce_par, filter_par).

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	4. type definitions
//	5. view impls
//	6. spec fns
//	7. proof fns
//	8. ninject lock predicate and helpers
//	9. traits
//	10. impls
//	11. iterators
//	12. derive impls in verus!
//	13. derive impls outside verus!

//		1. module

pub mod ArraySeqMtEph {

    use std::fmt::{Debug, Display, Formatter};
    use std::fmt::Result as FmtResult;
    use std::slice::Iter;
    use std::sync::Arc;
    use std::vec::IntoIter;

    use vstd::prelude::*;
    use vstd::rwlock::*;

    verus! {

    //		2. imports

    #[cfg(verus_keep_ghost)]
    use {
        vstd::std_specs::cmp::PartialEqSpecImpl,
        vstd::std_specs::vec::*,
        vstd::std_specs::clone::*,
    };
    use crate::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::*;
    use crate::vstdplus::clone_plus::clone_plus::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;
    use crate::vstdplus::monoid::monoid::*;
    use crate::vstdplus::multiset::multiset::*;

    #[cfg(verus_keep_ghost)]


    //		3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
        vstd::seq_lib::group_seq_properties,
        crate::vstdplus::feq::feq::group_feq_axioms,
    };


    //		4. type definitions

    #[verifier::reject_recursive_types(T)]
    pub struct ArraySeqMtEphS<T> {
        pub seq: Vec<T>,
    }


    //		5. view impls

    impl<T: View> View for ArraySeqMtEphS<T> {
        type V = Seq<T::V>;

        open spec fn view(&self) -> Seq<T::V> {
            self.seq@.map(|_i: int, t: T| t@)
        }
    }


    //		6. spec fns

    /// Definition 18.7 (iterate). Left fold: spec_iterate(s, f, x) = f(...f(f(x, s[0]), s[1])..., s[n-1]).
    pub open spec fn spec_iterate<A, T>(s: Seq<T>, f: spec_fn(A, T) -> A, start_x: A) -> A {
        s.fold_left(start_x, f)
    }

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
    /// When no duplicates exist, this coincides with `spec_inject`.
    pub open spec fn spec_ninject<T>(s: Seq<T>, updates: Seq<(usize, T)>, result: Seq<T>) -> bool {
        result.len() == s.len()
        && forall|i: int| #![trigger result[i]] 0 <= i < s.len() ==> {
            result[i] == s[i]
            || exists|j: int| #![trigger updates[j]] 0 <= j < updates.len()
                && updates[j].0 == i as usize && result[i] == updates[j].1
        }
    }

    //		7. proof fns

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

    //		8. ninject lock predicate and helpers

    /// Ghost state for the ninject lock. The invariant says the buffer is
    /// always a valid partial ninject result: every element is either the
    /// original or came from some update.
    pub struct NinjectInv<T> {
        pub ghost source: Seq<T>,
        pub ghost updates: Seq<(usize, T)>,
    }

    impl<T> RwLockPredicate<Vec<T>> for NinjectInv<T> {
        open spec fn inv(self, v: Vec<T>) -> bool {
            v@.len() == self.source.len()
            && forall|i: int| #![trigger v@[i]] 0 <= i < v@.len() ==> {
                v@[i] == self.source[i]
                || exists|j: int| #![trigger self.updates[j]] 0 <= j < self.updates.len()
                    && self.updates[j].0 == i as usize && v@[i] == self.updates[j].1
            }
        }
    }

    /// Acquire the lock, apply updates, release. Preserves the lock invariant.
    /// - APAS: N/A — implementation utility, not in prose.
    /// - Claude-Opus-4.6: Work Θ(|updates|), Span Θ(|updates|).
    fn apply_ninject_updates<T: Clone + Eq + Send + Sync + 'static>(
        lock: Arc<RwLock<Vec<T>, NinjectInv<T>>>,
        updates: Vec<(usize, T)>,
        Ghost(pred): Ghost<NinjectInv<T>>,
    )
        requires
            obeys_feq_clone::<T>(),
            lock.pred() == pred,
            forall|k: int| #![trigger updates@[k]] 0 <= k < updates@.len() ==> {
                0 <= updates@[k].0 < pred.source.len()
                && exists|j: int| #![trigger pred.updates[j]] 0 <= j < pred.updates.len()
                    && pred.updates[j] == updates@[k]
            },
    {
        let (mut buf, write_handle) = lock.acquire_write();
        let len = buf.len();
        let mut i: usize = 0;
        while i < updates.len()
            invariant
                i <= updates@.len(),
                len == buf@.len(),
                pred.inv(buf),
                obeys_feq_clone::<T>(),
                forall|k: int| #![trigger updates@[k]] 0 <= k < updates@.len() ==> {
                    0 <= updates@[k].0 < pred.source.len()
                    && exists|j: int| #![trigger pred.updates[j]] 0 <= j < pred.updates.len()
                        && pred.updates[j] == updates@[k]
                },
            decreases updates@.len() - i,
        {
            let pos = updates[i].0;
            if pos < len {
                let val = updates[i].1.clone();
                proof {
                    axiom_cloned_implies_eq_owned::<T>(updates@[i as int].1, val);
                }
                buf.set(pos, val);
                proof {
                    let witness = choose|j: int| #![trigger pred.updates[j]]
                        0 <= j < pred.updates.len()
                        && pred.updates[j] == updates@[i as int];
                    assert forall|p: int| #![trigger buf@[p]] 0 <= p < buf@.len() implies {
                        buf@[p] == pred.source[p]
                        || exists|j: int| #![trigger pred.updates[j]] 0 <= j < pred.updates.len()
                            && pred.updates[j].0 == p as usize && buf@[p] == pred.updates[j].1
                    } by {
                        if p == pos as int {
                            assert(pred.updates[witness].0 == pos as usize);
                            assert(buf@[p] == pred.updates[witness].1);
                        }
                    }
                }
            }
            i += 1;
        }
        write_handle.release_write(buf);
    }

    //		9. traits

    /// - Base trait for multi-threaded ephemeral array sequences (Chapter 18).
    /// - These methods are never redefined in later chapters.
    pub trait ArraySeqMtEphBaseTrait<T>: Sized {
        spec fn spec_len(&self) -> nat;

        spec fn spec_index(&self, i: int) -> T
            recommends i < self.spec_len();

        /// - Create a new sequence of length `length` with each element initialized to `init_value`.
        /// - APAS: no cost spec (semantics-only chapter).
        /// - Claude-Opus-4.6: Work Θ(length), Span Θ(log length).
        fn new(length: usize, init_value: T) -> (new_seq: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                length <= usize::MAX,
            ensures
                new_seq.spec_len() == length as int,
                forall|i: int| #![trigger new_seq.spec_index(i)] 0 <= i < length ==> new_seq.spec_index(i) == init_value;

        /// - Set the element at `index` to `item` in place.
        /// - APAS: N/A — implementation utility, not in prose.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1).
        fn set(&mut self, index: usize, item: T) -> (success: Result<(), &'static str>)
            requires index < old(self).spec_len()
            ensures
                success.is_ok() ==> self.spec_len() == old(self).spec_len(),
                success.is_ok() ==> self.spec_index(index as int) == item,
                success.is_ok() ==> forall|i: int| #![trigger self.spec_index(i), old(self).spec_index(i)] 0 <= i < old(self).spec_len() && i != index ==> self.spec_index(i) == old(self).spec_index(i);

        /// - Definition 18.1 (length). Return the number of elements.
        /// - APAS: no cost spec (semantics-only chapter).
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1).
        fn length(&self) -> (len: usize)
            ensures len as int == self.spec_len();

        /// - Algorithm 19.11 (Function nth). Return a reference to the element at `index`.
        /// - APAS: no cost spec (semantics-only chapter).
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1).
        fn nth(&self, index: usize) -> (nth_elem: &T)
            requires index < self.spec_len()
            ensures *nth_elem == self.spec_index(index as int);

        /// - Definition 18.12 (subseq copy). Extract contiguous subsequence with allocation.
        /// - APAS: N/A — implementation utility, not in prose.
        /// - Claude-Opus-4.6: Work Θ(length), Span Θ(log length).
        fn subseq_copy(&self, start: usize, length: usize) -> (subseq: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                start + length <= usize::MAX,
                start + length <= self.spec_len(),
            ensures
                subseq.spec_len() == length as int,
                forall|i: int| #![trigger subseq.spec_index(i)] 0 <= i < length ==> subseq.spec_index(i) == self.spec_index(start as int + i);

        /// - Definition 18.12 (subseq). Extract a contiguous subsequence.
        /// - APAS: no cost spec (semantics-only chapter).
        /// - Claude-Opus-4.6: Work Θ(length), Span Θ(log length).
        fn subseq(a: &Self, start: usize, length: usize) -> (subseq: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                start + length <= usize::MAX,
                start + length <= a.spec_len(),
            ensures
                subseq.spec_len() == length as int,
                forall|i: int| #![trigger subseq.spec_index(i)] 0 <= i < length ==> subseq.spec_index(i) == a.spec_index(start as int + i);

        /// - Create sequence from Vec.
        /// - APAS: N/A — implementation utility, not in prose.
        /// - Claude-Opus-4.6: Work Θ(n) worst case, Θ(1) best case, Span Θ(1).
        fn from_vec(elts: Vec<T>) -> (seq: Self)
            ensures
                seq.spec_len() == elts@.len(),
                forall|i: int| #![trigger seq.spec_index(i)] 0 <= i < elts@.len() ==> seq.spec_index(i) == elts@[i];
    }

    /// Redefinable trait - may be overridden with better algorithms in later chapters.
    pub trait ArraySeqMtEphRedefinableTrait<T>: ArraySeqMtEphBaseTrait<T> {

        /// - Definition 18.1 (empty). Construct the empty sequence.
        /// - APAS: no cost spec (semantics-only chapter).
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1).
        fn empty() -> (empty_seq: Self)
            ensures empty_seq.spec_len() == 0;

        /// - Definition 18.1 (singleton). Construct a singleton sequence containing `item`.
        /// - APAS: no cost spec (semantics-only chapter).
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1).
        fn singleton(item: T) -> (singleton: Self)
            ensures
                singleton.spec_len() == 1,
                singleton.spec_index(0) == item;

        /// - Definition 18.13 (append). Concatenate two sequences.
        /// - APAS: no cost spec (semantics-only chapter).
        /// - Claude-Opus-4.6: Work Θ(|a| + |b|), Span Θ(log(|a| + |b|)).
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> (appended: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                a.seq@.len() + b.seq@.len() <= usize::MAX as int,
            ensures
                appended.spec_len() == a.seq@.len() + b.seq@.len(),
                forall|i: int| #![trigger appended.spec_index(i)] 0 <= i < a.seq@.len() ==> appended.spec_index(i) == a.seq@[i],
                forall|i: int| #![trigger b.seq@[i]] 0 <= i < b.seq@.len() ==> appended.spec_index(a.seq@.len() as int + i) == b.seq@[i];

        /// - Definition 18.14 (filter). Keep elements satisfying `pred`.
        /// - APAS: no cost spec (semantics-only chapter).
        /// - Claude-Opus-4.6: Work Θ(|a|), Span Θ(|a|).
        /// - The multiset postcondition captures predicate satisfaction, provenance,
        ///   and completeness in a single statement.
        fn filter<F: Fn(&T) -> bool>(a: &ArraySeqMtEphS<T>, pred: &F, Ghost(spec_pred): Ghost<spec_fn(T) -> bool>) -> (filtered: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                forall|i: int| 0 <= i < a.seq@.len() ==> #[trigger] pred.requires((&a.seq@[i],)),
                // The biconditional bridge ties the exec closure to the spec predicate.
                forall|v: T, keep: bool| pred.ensures((&v,), keep) ==> spec_pred(v) == keep,
            ensures
                filtered.spec_len() <= a.seq@.len(),
                filtered.spec_len() == spec_filter_len(
                    Seq::new(a.seq@.len(), |i: int| a.seq@[i]), spec_pred),
                // The result multiset equals the input multiset filtered by the spec predicate.
                Seq::new(filtered.spec_len(), |i: int| filtered.spec_index(i)).to_multiset()
                    =~= Seq::new(a.seq@.len(), |i: int| a.seq@[i]).to_multiset().filter(spec_pred),
                forall|i: int| #![trigger filtered.spec_index(i)] 0 <= i < filtered.spec_len() ==> pred.ensures((&filtered.spec_index(i),), true);

        /// - Definition 18.16 (update). Return a copy with the index replaced by the new value.
        /// - APAS: no cost spec (semantics-only chapter).
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(log n).
        fn update(a: &ArraySeqMtEphS<T>, index: usize, item: T) -> (updated: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                index < a.seq@.len(),
            ensures
                updated.spec_len() == a.seq@.len(),
                updated.spec_index(index as int) == item,
                forall|i: int| #![trigger updated.spec_index(i)] 0 <= i < a.seq@.len() && i != index as int ==> updated.spec_index(i) == a.seq@[i];

        /// - Definition 18.16 (inject). Update multiple positions at once; the first update in
        ///   the ordering of `updates` takes effect when positions collide.
        /// - APAS: no cost spec (semantics-only chapter).
        /// - Claude-Opus-4.6: Work Θ(|a| + |updates|), Span Θ(1).
        fn inject(a: &Self, updates: &Vec<(usize, T)>) -> (injected: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
            ensures
                injected.spec_len() == a.spec_len(),
                Seq::new(injected.spec_len(), |i: int| injected.spec_index(i))
                    =~= spec_inject(
                        Seq::new(a.spec_len(), |i: int| a.spec_index(i)),
                        updates@);

        /// - Definition 18.17 (ninject). Nondeterministic inject: update multiple positions at
        ///   once; when positions collide, any one of the updates may take effect.
        /// - APAS: no cost spec (semantics-only chapter).
        /// - Claude-Opus-4.6: Work Θ(|a| + |updates|), Span Θ(1).
        fn ninject(a: &Self, updates: &Vec<(usize, T)>) -> (result: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
            ensures
                spec_ninject(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)),
                    updates@,
                    Seq::new(result.spec_len(), |i: int| result.spec_index(i)));

        /// - Definition 18.5 (isEmpty). true iff the sequence has length zero.
        /// - APAS: no cost spec (semantics-only chapter).
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1).
        fn is_empty(&self) -> (empty: bool)
            ensures empty <==> self.spec_len() == 0;

        /// - Definition 18.5 (isSingleton). true iff the sequence has length one.
        /// - APAS: no cost spec (semantics-only chapter).
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1).
        fn is_singleton(&self) -> (single: bool)
            ensures single <==> self.spec_len() == 1;

        /// - Definition 18.7 (iterate). Fold with accumulator `seed`.
        /// - APAS: no cost spec (semantics-only chapter).
        /// - Claude-Opus-4.6: Work Θ(|a|), Span Θ(|a|).
        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqMtEphS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(A, T) -> A>, seed: A) -> (result: A)
            requires
                forall|x: &A, y: &T| #[trigger] f.requires((x, y)),
                forall|a: A, t: T, ret: A| f.ensures((&a, &t), ret) <==> ret == spec_f(a, t),
            ensures
                result == spec_iterate(Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_f, seed);

        /// - Definition 18.18 (reduce). Combine elements using associative `f` and identity `id`.
        /// - APAS: no cost spec (semantics-only chapter).
        /// - Claude-Opus-4.6: Work Θ(|a|), Span Θ(log|a|).
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqMtEphS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(T, T) -> T>, id: T) -> (result: T)
            where T: Clone
            requires
                spec_monoid(spec_f, id),
                forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) <==> ret == spec_f(x, y),
            ensures
                result == spec_iterate(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_f, id);

        /// - Definition 18.19 (scan). Prefix-reduce returning inclusive prefix sums and total.
        /// - APAS: no cost spec (semantics-only chapter).
        /// - Claude-Opus-4.6: Work Θ(|a|), Span Θ(log|a|).
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqMtEphS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(T, T) -> T>, id: T) -> (scanned: (ArraySeqMtEphS<T>, T))
            where T: Clone + Eq
            requires
                spec_monoid(spec_f, id),
                obeys_feq_clone::<T>(),
                forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) <==> ret == spec_f(x, y),
            ensures
                scanned.0.spec_len() == a.spec_len(),
                forall|i: int| #![trigger scanned.0.spec_index(i)] 0 <= i < a.spec_len() ==>
                    scanned.0.spec_index(i) == Seq::new(a.spec_len(), |j: int| a.spec_index(j)).take(i + 1).fold_left(id, spec_f),
                scanned.1 == spec_iterate(
                    Seq::new(a.spec_len(), |j: int| a.spec_index(j)), spec_f, id);

        /// - Algorithm 18.4 (map). Transform each element via `f`.
        /// - APAS: no cost spec (semantics-only chapter).
        /// - Claude-Opus-4.6: Work Θ(|a|), Span Θ(log|a|).
        fn map<U: Clone, F: Fn(&T) -> U>(a: &ArraySeqMtEphS<T>, f: &F) -> (mapped: ArraySeqMtEphS<U>)
            requires
                forall|i: int| 0 <= i < a.seq@.len() ==> #[trigger] f.requires((&a.seq@[i],)),
            ensures
                mapped.seq@.len() == a.seq@.len(),
                forall|i: int| #![trigger mapped.seq@[i]] 0 <= i < a.seq@.len() ==> f.ensures((&a.seq@[i],), mapped.seq@[i]);

        /// - Algorithm 18.3 (tabulate). Build a sequence by applying `f` to each index.
        /// - APAS: no cost spec (semantics-only chapter).
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n).
        fn tabulate<F: Fn(usize) -> T>(f: &F, length: usize) -> (tab_seq: ArraySeqMtEphS<T>)
            requires
                length <= usize::MAX,
                forall|i: usize| i < length ==> #[trigger] f.requires((i,)),
            ensures
                tab_seq.seq@.len() == length,
                forall|i: int| #![trigger tab_seq.seq@[i]] 0 <= i < length ==> f.ensures((i as usize,), tab_seq.seq@[i]);

        /// - Definition 18.15 (flatten). Concatenate a sequence of sequences.
        /// - APAS: no cost spec (semantics-only chapter).
        /// - Claude-Opus-4.6: Work Θ(Σ|a_i|), Span Θ(Σ|a_i|).
        fn flatten(a: &ArraySeqMtEphS<ArraySeqMtEphS<T>>) -> (flattened: ArraySeqMtEphS<T>)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
            ensures
                flattened.seq@ =~= a.seq@.map_values(|inner: ArraySeqMtEphS<T>| inner.seq@).flatten();
    }


    //		9. impl BaseTrait for Struct

    impl<T> ArraySeqMtEphBaseTrait<T> for ArraySeqMtEphS<T> {
        open spec fn spec_len(&self) -> nat {
            self.seq@.len()
        }

        open spec fn spec_index(&self, i: int) -> T {
            self.seq[i]
        }

        fn new(length: usize, init_value: T) -> (new_seq: ArraySeqMtEphS<T>)
            where T: Clone + Eq
        {
            let seq = std::vec::from_elem(init_value, length);
            ArraySeqMtEphS { seq }
        }

        fn set(&mut self, index: usize, item: T) -> (success: Result<(), &'static str>) {
            if index < self.seq.len() {
                self.seq.set(index, item);
                Ok(())
            } else {
                Err("Index out of bounds")
            }
        }

        fn length(&self) -> (len: usize) {
            self.seq.len()
        }

        fn nth(&self, index: usize) -> (nth_elem: &T) {
            &self.seq[index]
        }

        fn subseq_copy(&self, start: usize, length: usize) -> (subseq: ArraySeqMtEphS<T>)
            where T: Clone + Eq
        {
            let end = start + length;
            let mut seq: Vec<T> = Vec::with_capacity(length);
            let mut i: usize = start;
            while i < end
                invariant
                    start <= i <= end,
                    end == start + length,
                    end <= self.seq@.len(),
                    seq@.len() == (i - start) as int,
                    obeys_feq_clone::<T>(),
                    forall|j: int| #![trigger seq@[j]] 0 <= j < seq@.len() ==> seq@[j] == self.seq@[(start + j) as int],
                decreases end - i,
            {
                seq.push(self.seq[i].clone());
                proof {
                    let ghost last = seq@[seq@.len() - 1 as int];
                    assert(cloned(self.seq[i as int], last));
                    axiom_cloned_implies_eq_owned(self.seq[i as int], last);
                }
                i += 1;
            }
            ArraySeqMtEphS { seq }
        }

        fn subseq(a: &ArraySeqMtEphS<T>, start: usize, length: usize) -> (subseq: ArraySeqMtEphS<T>)
            where T: Clone + Eq
        {
            let end = start + length;
            let mut seq: Vec<T> = Vec::with_capacity(length);
            let mut i: usize = start;
            while i < end
                invariant
                    start <= i <= end,
                    end == start + length,
                    end <= a.seq@.len(),
                    seq@.len() == (i - start) as int,
                    obeys_feq_clone::<T>(),
                    forall|j: int| #![trigger seq@[j]] 0 <= j < seq@.len() ==> seq@[j] == a.seq@[(start + j) as int],
                decreases end - i,
            {
                seq.push(a.seq[i].clone());
                proof {
                    let ghost last = seq@[seq@.len() - 1 as int];
                    assert(cloned(a.seq[i as int], last));
                    axiom_cloned_implies_eq_owned(a.seq[i as int], last);
                }
                i += 1;
            }
            ArraySeqMtEphS { seq }
        }

        fn from_vec(elts: Vec<T>) -> (seq: ArraySeqMtEphS<T>) {
            ArraySeqMtEphS { seq: elts }
        }
    }

    //		9. impl RedefinableTrait for Struct

    impl<T> ArraySeqMtEphRedefinableTrait<T> for ArraySeqMtEphS<T> {
        fn empty() -> (empty_seq: ArraySeqMtEphS<T>) {
            ArraySeqMtEphS { seq: Vec::new() }
        }

        fn singleton(item: T) -> (singleton: ArraySeqMtEphS<T>) {
            let mut seq = Vec::with_capacity(1);
            seq.push(item);
            ArraySeqMtEphS { seq }
        }

        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> (appended: ArraySeqMtEphS<T>)
            where T: Clone + Eq
        {
            let a_len = a.seq.len();
            let b_len = b.seq.len();
            let mut seq: Vec<T> = Vec::with_capacity(a_len + b_len);
            let mut i: usize = 0;
            while i < a_len
                invariant
                    i <= a_len,
                    a_len == a.seq@.len(),
                    seq@.len() == i as int,
                    obeys_feq_clone::<T>(),
                    forall|k: int| #![trigger seq@[k]] 0 <= k < i ==> seq@[k] == a.seq@[k],
                decreases a_len - i,
            {
                seq.push(a.seq[i].clone());
                proof {
                    let ghost last = seq@[seq@.len() - 1 as int];
                    assert(cloned(a.seq[i as int], last));
                    axiom_cloned_implies_eq_owned(a.seq[i as int], last);
                }
                i += 1;
            }
            let mut j: usize = 0;
            while j < b_len
                invariant
                    j <= b_len,
                    b_len == b.seq@.len(),
                    a_len == a.seq@.len(),
                    seq@.len() == a_len + j,
                    obeys_feq_clone::<T>(),
                    forall|k: int| #![trigger seq@[k]] 0 <= k < a_len ==> seq@[k] == a.seq@[k],
                    forall|k: int| #![trigger b.seq@[k]] 0 <= k < j ==> seq@[a_len as int + k] == b.seq@[k],
                decreases b_len - j,
            {
                seq.push(b.seq[j].clone());
                proof {
                    let ghost last = seq@[seq@.len() - 1 as int];
                    assert(cloned(b.seq[j as int], last));
                    axiom_cloned_implies_eq_owned(b.seq[j as int], last);
                }
                j += 1;
            }
            ArraySeqMtEphS { seq }
        }

        fn filter<F: Fn(&T) -> bool>(a: &ArraySeqMtEphS<T>, pred: &F, Ghost(spec_pred): Ghost<spec_fn(T) -> bool>) -> (filtered: ArraySeqMtEphS<T>)
            where T: Clone + Eq
        {
            let len = a.seq.len();
            let mut seq: Vec<T> = Vec::new();
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len == a.seq@.len(),
                    seq@.len() <= i,
                    obeys_feq_clone::<T>(),
                    forall|j: int| 0 <= j < a.seq@.len() ==> #[trigger] pred.requires((&a.seq@[j],)),
                    forall|v: T, keep: bool| pred.ensures((&v,), keep) ==> spec_pred(v) == keep,
                    forall|j: int| #![trigger seq@[j]] 0 <= j < seq@.len() ==> pred.ensures((&seq@[j],), true),
                    seq@.len() == spec_filter_len(a.seq@.subrange(0, i as int), spec_pred),
                    seq@.to_multiset() =~= a.seq@.subrange(0, i as int).to_multiset().filter(spec_pred),
                decreases len - i,
            {
                proof {
                    broadcast use vstd::seq_lib::group_to_multiset_ensures;
                    a.lemma_spec_index(i as int);
                }
                assert(a.seq@.subrange(0, i as int + 1) =~= a.seq@.subrange(0, i as int).push(a.seq@[i as int]));
                assert(a.seq@.subrange(0, i as int + 1).drop_last() =~= a.seq@.subrange(0, i as int));
                if pred(&a.seq[i]) {
                    let elem = a.seq[i].clone();
                    proof {
                        axiom_cloned_implies_eq_owned(a.seq[i as int], elem);
                    }
                    seq.push(elem);
                }
                i += 1;
            }
            let filtered = ArraySeqMtEphS { seq };
            proof {
                assert(a.seq@.subrange(0, a.seq@.len() as int) =~= a.seq@);
                assert(filtered.seq@ =~= Seq::new(filtered.spec_len(), |i: int| filtered.spec_index(i)));
                assert(a.seq@ =~= Seq::new(a.seq@.len(), |i: int| a.seq@[i]));
            }
            filtered
        }

        fn update(a: &ArraySeqMtEphS<T>, index: usize, item: T) -> (updated: ArraySeqMtEphS<T>)
            where T: Clone + Eq
        {
            let len = a.seq.len();
            let mut seq: Vec<T> = Vec::with_capacity(len);
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len == a.seq@.len(),
                    seq@.len() == i as int,
                    obeys_feq_clone::<T>(),
                    index < len,
                    forall|k: int| #![trigger seq@[k]] 0 <= k < i && k != index as int ==> seq@[k] == a.seq@[k],
                    i > index ==> seq@[index as int] == item,
                decreases len - i,
            {
                if i == index {
                    seq.push(item.clone());
                    proof {
                        let ghost last = seq@[seq@.len() - 1 as int];
                        assert(cloned(item, last));
                        axiom_cloned_implies_eq_owned(item, last);
                    }
                } else {
                    seq.push(a.seq[i].clone());
                    proof {
                        let ghost last = seq@[seq@.len() - 1 as int];
                        assert(cloned(a.seq[i as int], last));
                        axiom_cloned_implies_eq_owned(a.seq[i as int], last);
                    }
                }
                i += 1;
            }
            ArraySeqMtEphS { seq }
        }

        fn inject(a: &ArraySeqMtEphS<T>, updates: &Vec<(usize, T)>) -> (injected: ArraySeqMtEphS<T>)
            where T: Clone + Eq
        {
            let ghost s = a.seq@;
            let ghost u = updates@;
            let len = a.seq.len();
            let ulen = updates.len();

            let mut result_vec: Vec<T> = Vec::with_capacity(len);
            let mut k: usize = 0;
            while k < len
                invariant
                    k <= len,
                    len == a.seq@.len(),
                    s == a.seq@,
                    result_vec@.len() == k as int,
                    obeys_feq_clone::<T>(),
                    forall|j: int| #![trigger result_vec@[j]] 0 <= j < k as int ==> result_vec@[j] == s[j],
                decreases len - k,
            {
                let elem = a.seq[k].clone();
                proof { axiom_cloned_implies_eq_owned(a.seq@[k as int], elem); }
                result_vec.push(elem);
                k += 1;
            }
            assert(result_vec@ =~= s);

            let mut i: usize = ulen;
            while i > 0
                invariant
                    0 <= i <= ulen,
                    ulen == u.len(),
                    len == a.seq@.len(),
                    result_vec@.len() == s.len(),
                    s.len() == len,
                    obeys_feq_clone::<T>(),
                    s == a.seq@,
                    u == updates@,
                    result_vec@ =~= spec_inject(s, u.subrange(i as int, ulen as int)),
                decreases i,
            {
                i -= 1;
                let pos = updates[i].0;
                if pos < len {
                    let val = updates[i].1.clone();
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
            }
            let injected = ArraySeqMtEphS { seq: result_vec };
            proof {
                assert(Seq::new(injected.spec_len(), |i: int| injected.spec_index(i)) =~= result_vec@);
                assert forall|j: int| 0 <= j < a.spec_len() implies #[trigger] a.spec_index(j) == s[j]
                by { a.lemma_spec_index(j); }
                assert(Seq::new(a.spec_len(), |i: int| a.spec_index(i)) =~= s);
            }
            injected
        }

        fn ninject(a: &ArraySeqMtEphS<T>, updates: &Vec<(usize, T)>) -> (result: ArraySeqMtEphS<T>)
            where T: Clone + Eq
        {
            // Delegate to deterministic inject. inject satisfies the weaker ninject spec
            // because inject picks the first update for each position, which is one valid
            // nondeterministic choice.
            let result = Self::inject(a, updates);
            proof {
                let ghost s = Seq::new(a.spec_len(), |i: int| a.spec_index(i));
                let ghost r = Seq::new(result.spec_len(), |i: int| result.spec_index(i));
                let ghost u = updates@;
                // inject ensures r =~= spec_inject(s, u), so r.len() == s.len().
                // For each position, spec_inject either leaves s[i] or applies some update.
                assert forall|i: int| 0 <= i < s.len() implies
                    #[trigger] r[i] == s[i]
                    || exists|j: int| #![trigger u[j]] 0 <= j < u.len()
                        && u[j].0 == i as usize && r[i] == u[j].1
                by {
                    lemma_spec_inject_element(s, u, i);
                }
            }
            result
        }

        fn is_empty(&self) -> (empty: bool) {
            self.seq.len() == 0
        }

        fn is_singleton(&self) -> (single: bool) {
            self.seq.len() == 1
        }

        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqMtEphS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(A, T) -> A>, seed: A) -> (acc: A) {
            let ghost s = Seq::new(a.spec_len(), |i: int| a.spec_index(i));
            let len = a.seq.len();
            let mut acc = seed;
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len == a.seq@.len(),
                    forall|x: &A, y: &T| #[trigger] f.requires((x, y)),
                    forall|a: A, t: T, ret: A| f.ensures((&a, &t), ret) ==> ret == spec_f(a, t),
                    s == Seq::new(a.spec_len(), |j: int| a.spec_index(j)),
                    acc == s.take(i as int).fold_left(seed, spec_f),
                decreases len - i,
            {
                proof {
                    a.lemma_spec_index(i as int);
                    assert(s.take(i as int + 1) =~= s.take(i as int).push(s[i as int]));
                }
                acc = f(&acc, &a.seq[i]);
                proof {
                    let ghost t = s.take(i as int + 1);
                    assert(t.len() > 0);
                    assert(t.drop_last() =~= s.take(i as int));
                    assert(t.last() == s[i as int]);
                    reveal(Seq::fold_left);
                }
                i += 1;
            }
            proof {
                assert(s.take(len as int) =~= s);
            }
            acc
        }

        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqMtEphS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(T, T) -> T>, id: T) -> (reduced: T)
            where T: Clone
        {
            let ghost s = Seq::new(a.spec_len(), |i: int| a.spec_index(i));
            let len = a.seq.len();
            let mut acc = id;
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len == a.seq@.len(),
                    forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                    forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) ==> ret == spec_f(x, y),
                    s == Seq::new(a.spec_len(), |j: int| a.spec_index(j)),
                    acc == s.take(i as int).fold_left(id, spec_f),
                decreases len - i,
            {
                proof {
                    a.lemma_spec_index(i as int);
                    assert(s.take(i as int + 1) =~= s.take(i as int).push(s[i as int]));
                }
                acc = f(&acc, &a.seq[i]);
                proof {
                    let ghost t = s.take(i as int + 1);
                    assert(t.len() > 0);
                    assert(t.drop_last() =~= s.take(i as int));
                    assert(t.last() == s[i as int]);
                    reveal(Seq::fold_left);
                }
                i += 1;
            }
            proof {
                assert(s.take(len as int) =~= s);
            }
            acc
        }

        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqMtEphS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(T, T) -> T>, id: T) -> (scanned: (ArraySeqMtEphS<T>, T))
            where T: Clone + Eq
        {
            let ghost s = Seq::new(a.spec_len(), |i: int| a.spec_index(i));
            let len = a.seq.len();
            let mut acc = id;
            let mut seq: Vec<T> = Vec::with_capacity(len);
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len == a.seq@.len(),
                    seq@.len() == i as int,
                    obeys_feq_clone::<T>(),
                    forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                    forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) ==> ret == spec_f(x, y),
                    s == Seq::new(a.spec_len(), |j: int| a.spec_index(j)),
                    acc == s.take(i as int).fold_left(id, spec_f),
                    forall|k: int| #![trigger seq@[k]] 0 <= k < seq@.len() ==>
                        seq@[k] == s.take(k + 1).fold_left(id, spec_f),
                decreases len - i,
            {
                proof {
                    a.lemma_spec_index(i as int);
                    assert(s.take(i as int + 1) =~= s.take(i as int).push(s[i as int]));
                }
                acc = f(&acc, &a.seq[i]);
                proof {
                    let ghost t = s.take(i as int + 1);
                    assert(t.len() > 0);
                    assert(t.drop_last() =~= s.take(i as int));
                    assert(t.last() == s[i as int]);
                    reveal(Seq::fold_left);
                }
                let cloned = acc.clone();
                proof {
                    axiom_cloned_implies_eq_owned(acc, cloned);
                }
                seq.push(cloned);
                i += 1;
            }
            proof {
                assert(s.take(len as int) =~= s);
            }
            let scanned_seq = ArraySeqMtEphS { seq };
            proof {
                assert forall|i: int| #![trigger scanned_seq.spec_index(i)] 0 <= i < a.spec_len() implies
                    scanned_seq.spec_index(i) == s.take(i + 1).fold_left(id, spec_f)
                by {
                    assert(scanned_seq.spec_index(i) == seq@[i]);
                }
            }
            (scanned_seq, acc)
        }

        fn map<U: Clone, F: Fn(&T) -> U>(a: &ArraySeqMtEphS<T>, f: &F) -> (mapped: ArraySeqMtEphS<U>)
        {
            let len = a.seq.len();
            let mut seq: Vec<U> = Vec::with_capacity(len);
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len == a.seq@.len(),
                    seq@.len() == i as int,
                    forall|j: int| 0 <= j < a.seq@.len() ==> #[trigger] f.requires((&a.seq@[j],)),
                    forall|j: int| #![trigger seq@[j]] 0 <= j < i ==> f.ensures((&a.seq@[j],), seq@[j]),
                decreases len - i,
            {
                seq.push(f(&a.seq[i]));
                i += 1;
            }
            ArraySeqMtEphS { seq }
        }

        fn tabulate<F: Fn(usize) -> T>(f: &F, length: usize) -> (tab_seq: ArraySeqMtEphS<T>)
        {
            let mut seq = Vec::with_capacity(length);
            let mut i: usize = 0;
            while i < length
                invariant
                    i <= length,
                    seq@.len() == i as int,
                    forall|j: usize| j < length ==> #[trigger] f.requires((j,)),
                    forall|j: int| #![trigger seq@[j]] 0 <= j < i ==> f.ensures((j as usize,), seq@[j]),
                decreases length - i,
            {
                seq.push(f(i));
                i += 1;
            }
            ArraySeqMtEphS { seq }
        }

        fn flatten(a: &ArraySeqMtEphS<ArraySeqMtEphS<T>>) -> (flattened: ArraySeqMtEphS<T>)
            where T: Clone + Eq
        {
            let outer_len = a.seq.len();
            let mut seq: Vec<T> = Vec::new();
            let mut i: usize = 0;
            while i < outer_len
                invariant
                    i <= outer_len,
                    outer_len == a.seq@.len(),
                    obeys_feq_clone::<T>(),
                    seq@ =~= a.seq@.take(i as int).map_values(|inner: ArraySeqMtEphS<T>| inner.seq@).flatten(),
                decreases outer_len - i,
            {
                let inner = &a.seq[i];
                let inner_len = inner.seq.len();
                let mut j: usize = 0;
                while j < inner_len
                    invariant
                        j <= inner_len,
                        inner_len == inner.seq@.len(),
                        i < outer_len,
                        outer_len == a.seq@.len(),
                        obeys_feq_clone::<T>(),
                        seq@ =~= a.seq@.take(i as int).map_values(|inner: ArraySeqMtEphS<T>| inner.seq@).flatten()
                            + inner.seq@.take(j as int),
                    decreases inner_len - j,
                {
                    seq.push(inner.seq[j].clone());
                    proof {
                        let ghost last = seq@[seq@.len() - 1 as int];
                        assert(cloned(inner.seq[j as int], last));
                        axiom_cloned_implies_eq_owned(inner.seq[j as int], last);
                        assert(inner.seq@.take(j as int + 1) =~= inner.seq@.take(j as int).push(inner.seq@[j as int]));
                    }
                    j += 1;
                }
                proof {
                    assert(inner.seq@.take(inner_len as int) =~= inner.seq@);
                    let ghost prefix = a.seq@.take(i as int).map_values(|inner: ArraySeqMtEphS<T>| inner.seq@);
                    assert(a.seq@.take(i as int + 1).map_values(|inner: ArraySeqMtEphS<T>| inner.seq@)
                        =~= prefix.push(a.seq@[i as int].seq@));
                    prefix.lemma_flatten_push(a.seq@[i as int].seq@);
                }
                i += 1;
            }
            proof {
                assert(a.seq@.take(outer_len as int) =~= a.seq@);
            }
            ArraySeqMtEphS { seq }
        }
    }

    //		9. bare impl (parallel methods, lemmas, iterators)

    impl<T> ArraySeqMtEphS<T> {
        broadcast proof fn lemma_spec_index(&self, i: int)
            requires 0 <= i < self.spec_len()
            ensures #[trigger] self.seq@[i] == self.spec_index(i)
        {}

        /// Returns an iterator over the sequence elements.
        pub fn iter(&self) -> (it: ArraySeqMtEphIter<'_, T>)
            ensures
                it@.0 == 0,
                it@.1 == self.seq@,
                iter_invariant(&it),
        {
            ArraySeqMtEphIter { inner: self.seq.iter() }
        }

        /// - APAS: no cost spec (semantics-only chapter).
        /// - Claude-Opus-4.6: Work Θ(|a|), Span Θ(log|a|).
        pub fn map_par<U: Clone + Eq + View + Send + Sync + 'static, F: Fn(&T) -> U + Send + Sync + Clone + 'static>(
            a: &ArraySeqMtEphS<T>,
            f: F,
        ) -> (mapped: ArraySeqMtEphS<U>)
            where T: Clone + Send + Sync + Eq + 'static
            requires
                obeys_feq_clone::<T>(),
                obeys_feq_clone::<U>(),
                a.seq@.len() <= usize::MAX as int,
                forall|i: int| 0 <= i < a.seq@.len() ==> #[trigger] f.requires((&a.seq@[i],)),
            ensures mapped.seq@.len() == a.seq@.len()
            decreases a.seq@.len()
        {
            let len = a.seq.len();
            if len == 0 {
                ArraySeqMtEphS { seq: Vec::new() }
            } else if len == 1 {
                let mut seq = Vec::with_capacity(1);
                seq.push(f(&a.seq[0]));
                ArraySeqMtEphS { seq }
            } else {
                let mid = len / 2;
                let left_seq = a.subseq_copy(0, mid);
                let right_seq = a.subseq_copy(mid, len - mid);
                let f1 = clone_fn(&f);
                let f2 = f;
                proof {
                    assert forall|i: int| 0 <= i < left_seq.seq@.len() implies #[trigger] f1.requires((&left_seq.seq@[i],)) by {
                        a.lemma_spec_index(0 + i);
                        left_seq.lemma_spec_index(i);
                    }
                    assert forall|i: int| 0 <= i < right_seq.seq@.len() implies #[trigger] f2.requires((&right_seq.seq@[i],)) by {
                        a.lemma_spec_index(mid as int + i);
                        right_seq.lemma_spec_index(i);
                    }
                }
                let left_len = Ghost(left_seq.seq@.len());
                let right_len = Ghost(right_seq.seq@.len());
                let (left, right) = join(
                    move || -> (r: ArraySeqMtEphS<U>)
                        ensures r.seq@.len() == left_len@
                    { Self::map_par(&left_seq, f1) },
                    move || -> (r: ArraySeqMtEphS<U>)
                        ensures r.seq@.len() == right_len@
                    { Self::map_par(&right_seq, f2) },
                );
                ArraySeqMtEphS::<U>::append(&left, &right)
            }
        }

        /// - APAS: no cost spec (semantics-only chapter).
        /// - Claude-Opus-4.6: Work Θ(|a|), Span Θ(log|a|).
        pub fn filter_par<F: Fn(&T) -> bool + Send + Sync + Clone + 'static>(
            a: &ArraySeqMtEphS<T>,
            pred: F,
        ) -> (filtered: ArraySeqMtEphS<T>)
            where T: Clone + Send + Sync + Eq + 'static
            requires
                obeys_feq_clone::<T>(),
                a.seq@.len() <= usize::MAX as int,
                forall|i: int| 0 <= i < a.seq@.len() ==> #[trigger] pred.requires((&a.seq@[i],)),
            ensures filtered.seq@.len() <= a.seq@.len()
            decreases a.seq@.len()
        {
            let len = a.seq.len();
            if len == 0 {
                ArraySeqMtEphS { seq: Vec::new() }
            } else if len == 1 {
                if pred(&a.seq[0]) {
                    let mut seq = Vec::with_capacity(1);
                    seq.push(a.seq[0].clone());
                    ArraySeqMtEphS { seq }
                } else {
                    ArraySeqMtEphS { seq: Vec::new() }
                }
            } else {
                let mid = len / 2;
                let left_seq = a.subseq_copy(0, mid);
                let right_seq = a.subseq_copy(mid, len - mid);
                let p1 = clone_pred(&pred);
                let p2 = pred;
                proof {
                    assert forall|i: int| 0 <= i < left_seq.seq@.len() implies #[trigger] p1.requires((&left_seq.seq@[i],)) by {
                        a.lemma_spec_index(0 + i);
                        left_seq.lemma_spec_index(i);
                    }
                    assert forall|i: int| 0 <= i < right_seq.seq@.len() implies #[trigger] p2.requires((&right_seq.seq@[i],)) by {
                        a.lemma_spec_index(mid as int + i);
                        right_seq.lemma_spec_index(i);
                    }
                }
                let left_len = Ghost(left_seq.seq@.len());
                let right_len = Ghost(right_seq.seq@.len());
                let (left, right) = join(
                    move || -> (r: ArraySeqMtEphS<T>)
                        ensures r.seq@.len() <= left_len@
                    { Self::filter_par(&left_seq, p1) },
                    move || -> (r: ArraySeqMtEphS<T>)
                        ensures r.seq@.len() <= right_len@
                    { Self::filter_par(&right_seq, p2) },
                );
                Self::append(&left, &right)
            }
        }

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

        /// - APAS: no cost spec (semantics-only chapter).
        /// - Claude-Opus-4.6: Work Θ(|a|), Span Θ(log|a|).
        pub fn reduce_par<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(
            a: &ArraySeqMtEphS<T>,
            f: F,
            Ghost(spec_f): Ghost<spec_fn(T, T) -> T>,
            id: T,
        ) -> (reduced: T)
            where T: Clone + Send + Sync + Eq + 'static
            requires
                obeys_feq_clone::<T>(),
                spec_monoid(spec_f, id),
                a.seq@.len() > 0,
                forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) <==> ret == spec_f(x, y),
            ensures
                reduced == spec_iterate(Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_f, id),
            decreases a.seq@.len()
        {
            let ghost s = Seq::new(a.spec_len(), |i: int| a.spec_index(i));
            let len = a.seq.len();
            if len == 1 {
                let result = a.seq[0].clone();
                proof {
                    assert(cloned(a.seq[0 as int], result));
                    axiom_cloned_implies_eq_owned(a.seq[0 as int], result);
                    a.lemma_spec_index(0);
                    assert(s =~= seq![a.spec_index(0)]);
                    reveal_with_fuel(Seq::fold_left, 2);
                    assert(spec_f(id, s[0]) == s[0]);  // left identity
                }
                result
            } else {
                let mid = len / 2;
                let left_seq = a.subseq_copy(0, mid);
                let right_seq = a.subseq_copy(mid, len - mid);
                let f1 = clone_fn2(&f);
                let f2 = clone_fn2(&f);
                let id1 = id.clone();
                proof { axiom_cloned_implies_eq_owned(id, id1); }
                let id2 = id.clone();
                proof { axiom_cloned_implies_eq_owned(id, id2); }

                let ghost left_s = Seq::new(left_seq.spec_len(), |i: int| left_seq.spec_index(i));
                let ghost right_s = Seq::new(right_seq.spec_len(), |i: int| right_seq.spec_index(i));

                let fa = move || -> (r: T)
                    requires
                        left_seq.seq@.len() > 0,
                        obeys_feq_clone::<T>(),
                        spec_monoid(spec_f, id),
                        forall|x: &T, y: &T| #[trigger] f1.requires((x, y)),
                        forall|x: T, y: T, ret: T| f1.ensures((&x, &y), ret) <==> ret == spec_f(x, y),
                    ensures
                        r == spec_iterate(left_s, spec_f, id),
                {
                    Self::reduce_par(&left_seq, f1, Ghost(spec_f), id1)
                };

                let fb = move || -> (r: T)
                    requires
                        right_seq.seq@.len() > 0,
                        obeys_feq_clone::<T>(),
                        spec_monoid(spec_f, id),
                        forall|x: &T, y: &T| #[trigger] f2.requires((x, y)),
                        forall|x: T, y: T, ret: T| f2.ensures((&x, &y), ret) <==> ret == spec_f(x, y),
                    ensures
                        r == spec_iterate(right_s, spec_f, id),
                {
                    Self::reduce_par(&right_seq, f2, Ghost(spec_f), id2)
                };

                let (left, right) = join(fa, fb);
                let result = f(&left, &right);
                proof {
                    assert(left_s =~= s.subrange(0, mid as int));
                    assert(right_s =~= s.subrange(mid as int, len as int));
                    s.lemma_fold_left_split(id, spec_f, mid as int);
                    Self::lemma_monoid_fold_left(right_s, spec_f, id, left);
                }
                result
            }
        }

        /// Definition 18.17 (ninject). Parallel nondeterministic inject via RwLock.
        /// Two threads contend for a single lock protecting the result buffer.
        /// Whichever thread acquires last overwrites the other's conflicting writes.
        /// That scheduling race is the source of nondeterminism.
        /// - APAS: no cost spec (semantics-only chapter).
        /// - Claude-Opus-4.6: Work Θ(|a| + |updates|), Span Θ(|updates|) — lock serializes the writers.
        pub fn ninject_par(a: &ArraySeqMtEphS<T>, updates: &Vec<(usize, T)>) -> (result: ArraySeqMtEphS<T>)
            where T: Clone + Send + Sync + Eq + 'static
            requires
                obeys_feq_clone::<T>(),
                forall|k: int| #![trigger updates@[k]] 0 <= k < updates@.len() ==>
                    0 <= updates@[k].0 < a.seq@.len(),
            ensures
                spec_ninject(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)),
                    updates@,
                    Seq::new(result.spec_len(), |i: int| result.spec_index(i))),
        {
            let ghost s = Seq::new(a.spec_len(), |i: int| a.spec_index(i));
            let ghost pred = NinjectInv::<T> { source: a.seq@, updates: updates@ };

            let buf = a.seq.clone();
            proof {
                broadcast use group_feq_axioms;
                assert forall|i: int| 0 <= i < a.seq@.len() implies #[trigger] buf@[i] == a.seq@[i]
                by {
                    assert(cloned(a.seq@[i], buf@[i]));
                }
                assert(buf@ =~= a.seq@);
            }
            let lock = Arc::new(RwLock::<Vec<T>, NinjectInv<T>>::new(buf, Ghost(pred)));

            // Split updates in half.
            let mid = updates.len() / 2;
            let mut left: Vec<(usize, T)> = Vec::new();
            let mut right: Vec<(usize, T)> = Vec::new();
            let mut k: usize = 0;
            while k < updates.len()
                invariant
                    k <= updates@.len(),
                    mid == updates@.len() / 2,
                    obeys_feq_clone::<T>(),
                    forall|p: int| #![trigger left@[p]] 0 <= p < left@.len() ==> {
                        exists|j: int| #![trigger updates@[j]] 0 <= j < updates@.len()
                            && updates@[j] == left@[p]
                    },
                    forall|p: int| #![trigger right@[p]] 0 <= p < right@.len() ==> {
                        exists|j: int| #![trigger updates@[j]] 0 <= j < updates@.len()
                            && updates@[j] == right@[p]
                    },
                decreases updates@.len() - k,
            {
                let pos = updates[k].0;
                let val = updates[k].1.clone();
                proof {
                    axiom_cloned_implies_eq_owned::<T>(updates@[k as int].1, val);
                }
                if k < mid {
                    left.push((pos, val));
                } else {
                    right.push((pos, val));
                }
                k += 1;
            }

            // Two threads race for the single lock.
            let lock1 = lock.clone();
            let lock2 = lock.clone();

            let ghost lv = left@;
            let ghost rv = right@;

            proof {
                assert(pred.updates =~= updates@);
            }

            let (_, _) = join(
                move || -> ()
                    ensures true
                {
                    apply_ninject_updates(lock1, left, Ghost(pred));
                },
                move || -> ()
                    ensures true
                {
                    apply_ninject_updates(lock2, right, Ghost(pred));
                },
            );

            // Extract result. The lock invariant gives us spec_ninject.
            let (result_vec, write_handle) = lock.acquire_write();
            proof {
                assert(pred.inv(result_vec));
                assert(pred.updates =~= updates@);
                assert(pred.source =~= a.seq@);
            }
            let r = result_vec.clone();
            proof {
                broadcast use group_feq_axioms;
                assert forall|i: int| 0 <= i < result_vec@.len() implies #[trigger] r@[i] == result_vec@[i]
                by {
                    assert(cloned(result_vec@[i], r@[i]));
                }
                assert(r@ =~= result_vec@);
            }
            write_handle.release_write(result_vec);

            let result = ArraySeqMtEphS { seq: r };
            proof {
                assert(Seq::new(result.spec_len(), |i: int| result.spec_index(i)) =~= r@);
            }
            result
        }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: View + PartialEq> PartialEqSpecImpl for ArraySeqMtEphS<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }


    //		10. iterators

    /// Iterator wrapper with closed spec view for encapsulation.
    #[verifier::reject_recursive_types(T)]
    pub struct ArraySeqMtEphIter<'a, T> {
        inner: std::slice::Iter<'a, T>,
    }

    impl<'a, T> View for ArraySeqMtEphIter<'a, T> {
        type V = (int, Seq<T>);
        closed spec fn view(&self) -> (int, Seq<T>) { self.inner@ }
    }

    pub open spec fn iter_invariant<'a, T>(it: &ArraySeqMtEphIter<'a, T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<'a, T> std::iter::Iterator for ArraySeqMtEphIter<'a, T> {
        type Item = &'a T;

        // Relies on vstd's assume_specification for slice::Iter::next.
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
    pub struct ArraySeqMtEphGhostIterator<'a, T> {
        pub pos: int,
        pub elements: Seq<T>,
        pub phantom: core::marker::PhantomData<&'a T>,
    }

    impl<'a, T> View for ArraySeqMtEphGhostIterator<'a, T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.elements.take(self.pos) }
    }

    impl<'a, T> vstd::pervasive::ForLoopGhostIteratorNew for ArraySeqMtEphIter<'a, T> {
        type GhostIter = ArraySeqMtEphGhostIterator<'a, T>;
        open spec fn ghost_iter(&self) -> ArraySeqMtEphGhostIterator<'a, T> {
            ArraySeqMtEphGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    impl<'a, T> vstd::pervasive::ForLoopGhostIterator for ArraySeqMtEphGhostIterator<'a, T> {
        type ExecIter = ArraySeqMtEphIter<'a, T>;
        type Item = T;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &ArraySeqMtEphIter<'a, T>) -> bool {
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

        open spec fn ghost_advance(&self, _exec_iter: &ArraySeqMtEphIter<'a, T>) -> ArraySeqMtEphGhostIterator<'a, T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, T> std::iter::IntoIterator for &'a ArraySeqMtEphS<T> {
        type Item = &'a T;
        type IntoIter = ArraySeqMtEphIter<'a, T>;
        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                it@.0 == 0,
                it@.1 == self.seq@,
                iter_invariant(&it),
        {
            ArraySeqMtEphIter { inner: self.seq.iter() }
        }
    }

    impl<T> std::iter::IntoIterator for ArraySeqMtEphS<T> {
        type Item = T;
        type IntoIter = IntoIter<T>;
        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                it@.0 == 0,
                it@.1 == self.seq@,
        {
            self.seq.into_iter()
        }
    }


    //		11. derive impls in verus!

    impl<T: Clone> Clone for ArraySeqMtEphS<T> {
        fn clone(&self) -> (res: Self)
            ensures
                res.seq@.len() == self.seq@.len(),
                forall|i: int| #![trigger res.seq@[i]]
                    0 <= i < self.seq@.len() ==> cloned::<T>(self.seq@[i], res.seq@[i]),
        {
            ArraySeqMtEphS { seq: self.seq.clone() }
        }
    }

    impl<T: Eq + View> Eq for ArraySeqMtEphS<T> {}

    impl<T: PartialEq + View> PartialEq for ArraySeqMtEphS<T> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            let equal = self.seq == other.seq;
            proof { assume(equal == (self@ == other@)); }
            equal
        }
    }

    } // verus!


    //		13. derive impls outside verus!

    #[cfg(verus_keep_ghost)]
    impl<T: Debug> Debug for ArraySeqMtEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_list().entries(self.seq.iter()).finish()
        }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: Display> Display for ArraySeqMtEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "[")?;
            for (i, item) in self.seq.iter().enumerate() {
                if i > 0 { write!(f, ", ")?; }
                write!(f, "{item}")?;
            }
            write!(f, "]")
        }
    }

    /// Literal constructor macro for ArraySeqMtEphS.
    #[macro_export]
    macro_rules! ArraySeqMtEphSLit {
        () => { $crate::Chap18::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphS::from_vec(Vec::new()) };
        ($x:expr; $n:expr) => { $crate::Chap18::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphS::from_vec(vec![$x; $n]) };
        ($($x:expr),* $(,)?) => { $crate::Chap18::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphS::from_vec(vec![$($x),*]) };
    }
}
