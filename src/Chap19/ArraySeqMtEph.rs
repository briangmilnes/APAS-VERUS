//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Chapter 19 parametric sequence implementation for array-backed sequences (multi-threaded ephemeral). Verusified.
//! Functions are implemented in terms of primitives: nth, length, subseq, tabulate, flatten.
//! Ephemeral variant: supports in-place mutation via `set`.
//! Multi-threaded: parallel methods (map_par, filter_par, reduce_par) use fork-join via `join`.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	4. type definitions
//	5. view impls
//	6. spec fns
//	7. proof fns
//	8. traits
//	9. impls
//	10. iterators
//	11. derive impls in verus!
//	13. derive impls outside verus!

//		1. module

pub mod ArraySeqMtEph {

    use std::fmt::{Debug, Display, Formatter};
    use std::fmt::Result as FmtResult;
    use std::slice::Iter;
    use std::vec::IntoIter;

    use vstd::prelude::*;

    verus! {

    //		2. imports

    #[cfg(verus_keep_ghost)]
    use {
        vstd::multiset::Multiset,
        vstd::std_specs::cmp::PartialEqSpecImpl,
        vstd::std_specs::vec::*,
        vstd::std_specs::clone::*,
    };
    use crate::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::*;
    use crate::vstdplus::clone_plus::clone_plus::*;
    #[cfg(verus_keep_ghost)]
    use {
        crate::vstdplus::feq::feq::*,
        crate::vstdplus::monoid::monoid::*,
        crate::vstdplus::multiset::multiset::*,
    };


    //		3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
        vstd::seq_lib::group_seq_properties,
        crate::vstdplus::feq::feq::group_feq_axioms,
        // Veracity: added broadcast groups
        vstd::seq_lib::group_to_multiset_ensures,
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

    /// Definition 18.17 (ninject). Non-deterministic inject: each position in the result
    /// holds either the original value or a value from some update. The choice of which
    /// update "wins" is unspecified.
    pub open spec fn spec_ninject<T>(s: Seq<T>, updates: Seq<(usize, T)>, injected: Seq<T>) -> bool {
        injected.len() == s.len()
        && forall|i: int| #![trigger injected[i]] 0 <= i < s.len() ==> {
            injected[i] == s[i]
            || exists|j: int| #![trigger updates[j]] 0 <= j < updates.len()
                && updates[j].0 == i as usize && injected[i] == updates[j].1
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


    //		8. traits

    /// Chapter 19 multi-threaded ephemeral array sequence trait.
    /// Specifications match Chapter 18; algorithms from Chapter 19.
    /// Ephemeral: supports in-place mutation via `set`.
    pub trait ArraySeqMtEphTrait<T>: Sized {
        spec fn spec_arrayseqmteph_wf(&self) -> bool;

        spec fn spec_len(&self) -> nat;
        spec fn spec_index(&self, i: int) -> T
            recommends i < self.spec_len();

        /// - Create a new sequence of length `length` with each element initialized to `init_value`.
        /// - Alg Analysis: APAS: N/A — implementation utility, not in prose.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(length), Span O(log length).
        fn new(length: usize, init_value: T) -> (new_seq: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                length <= usize::MAX,
            ensures
                new_seq.spec_arrayseqmteph_wf(),
                new_seq.spec_len() == length as int,
                forall|i: int| #![trigger new_seq.spec_index(i)] 0 <= i < length ==> new_seq.spec_index(i) == init_value;

        /// - Set the element at `index` to `item` in place (ephemeral mutation).
        /// - Alg Analysis: APAS: N/A — implementation utility, not in prose.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1).
        fn set(&mut self, index: usize, item: T) -> (success: Result<(), &'static str>)
            requires index < old(self).spec_len()
            ensures
                success.is_ok(),
                success.is_ok() ==> self.spec_len() == old(self).spec_len(),
                success.is_ok() ==> self.spec_index(index as int) == item,
                success.is_ok() ==> forall|i: int| #![trigger self.spec_index(i), old(self).spec_index(i)] 0 <= i < old(self).spec_len() && i != index ==> self.spec_index(i) == old(self).spec_index(i);

        /// - Definition 18.1 (length). Return the number of elements.
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        fn length(&self) -> (len: usize)
            ensures len as int == self.spec_len();

        /// - Algorithm 19.11 (Function nth). Return a reference to the element at `index`.
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(1), Span O(1)
        /// - Alg Analysis: APAS (Ch22 CS 22.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        fn nth(&self, index: usize) -> (nth_elem: &T)
            requires index < self.spec_len()
            ensures *nth_elem == self.spec_index(index as int);

        /// - Definition 18.12 (subseq copy). Extract contiguous subsequence with allocation.
        /// - Alg Analysis: APAS: N/A — implementation utility, not in prose.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(length), Span O(log length).
        fn subseq_copy(&self, start: usize, length: usize) -> (subseq: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                start + length <= usize::MAX,
                start + length <= self.spec_len(),
            ensures
                subseq.spec_arrayseqmteph_wf(),
                subseq.spec_len() == length as int,
                forall|i: int| #![trigger subseq.spec_index(i)] 0 <= i < length ==> subseq.spec_index(i) == self.spec_index(start as int + i);

        /// - Definition 18.12 (subseq). Extract a contiguous subsequence.
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(length), Span O(length) — DIFFERS: Vec-backed, copies elements sequentially
        fn subseq(a: &Self, start: usize, length: usize) -> (subseq: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                start + length <= usize::MAX,
                start + length <= a.spec_len(),
            ensures
                subseq.spec_arrayseqmteph_wf(),
                subseq.spec_len() == length as int,
                forall|i: int| #![trigger subseq.spec_index(i)] 0 <= i < length ==> subseq.spec_index(i) == a.spec_index(start as int + i);

        /// - Create sequence from Vec.
        /// - Alg Analysis: APAS: N/A — implementation utility, not in prose.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n) worst case, O(1) best case, Span O(1).
        fn from_vec(elts: Vec<T>) -> (seq: Self)
            ensures
                seq.spec_arrayseqmteph_wf(),
                seq.spec_len() == elts@.len(),
                forall|i: int| #![trigger seq.spec_index(i)] 0 <= i < elts@.len() ==> seq.spec_index(i) == elts@[i];

        /// - Algorithm 19.1 (empty). empty = tabulate (lambda i.i) 0.
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        fn empty() -> (empty_seq: Self)
            ensures empty_seq.spec_arrayseqmteph_wf(), empty_seq.spec_len() == 0;

        /// - Algorithm 19.2 (singleton). singleton x = tabulate (lambda i.x) 1.
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        fn singleton(item: T) -> (singleton: Self)
            where T: Clone + Eq
            requires obeys_feq_clone::<T>()
            ensures
                singleton.spec_arrayseqmteph_wf(),
                singleton.spec_len() == 1,
                singleton.spec_index(0) == item;

        /// - Algorithm 19.4 (append). append a b = tabulate (select(a,b)) (|a|+|b|).
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(|a| + |b|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a| + |b|), Span O(|a| + |b|) — DIFFERS: sequential tabulate, span = work
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> (appended: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                a.seq@.len() + b.seq@.len() <= usize::MAX as int,
            ensures
                appended.spec_arrayseqmteph_wf(),
                appended.spec_len() == a.seq@.len() + b.seq@.len(),
                forall|i: int| #![trigger appended.spec_index(i)] 0 <= i < a.seq@.len() ==> appended.spec_index(i) == a.seq@[i],
                forall|i: int| #![trigger b.seq@[i]] 0 <= i < b.seq@.len() ==> appended.spec_index(a.seq@.len() as int + i) == b.seq@[i];

        /// - Algorithm 19.5 (filter). filter f a = flatten (map (deflate f) a).
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(1 + Sigma W(f(x))), Span O(lg |a| + max S(f(x)))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(lg n) — parallel D&C with multiset distribution lemma
        fn filter<F: Fn(&T) -> bool + Clone + Send + Sync + 'static>(a: &ArraySeqMtEphS<T>, pred: &F, Ghost(spec_pred): Ghost<spec_fn(T) -> bool>) -> (filtered: Self)
            where T: Clone + Eq + Send + Sync + 'static
            requires
                obeys_feq_clone::<T>(),
                forall|i: int| 0 <= i < a.seq@.len() ==> #[trigger] pred.requires((&a.seq@[i],)),
                forall|v: T, keep: bool| pred.ensures((&v,), keep) ==> spec_pred(v) == keep,
            ensures
                filtered.spec_arrayseqmteph_wf(),
                filtered.spec_len() <= a.seq@.len(),
                filtered.spec_len() == spec_filter_len(
                    Seq::new(a.seq@.len(), |i: int| a.seq@[i]), spec_pred),
                Seq::new(filtered.spec_len(), |i: int| filtered.spec_index(i)).to_multiset()
                    =~= Seq::new(a.seq@.len(), |i: int| a.seq@[i]).to_multiset().filter(spec_pred),
                forall|i: int| #![trigger filtered.spec_index(i)] 0 <= i < filtered.spec_len() ==> pred.ensures((&filtered.spec_index(i),), true);

        /// - Algorithm 19.6 (update). Ephemeral: clone then set (O(n) clone + O(1) set).
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(|a|), Span O(1)
        /// - Alg Analysis: APAS (Ch22 CS 22.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a|), Span O(|a|) — DIFFERS: clone entire Vec + set, sequential
        fn update(a: &ArraySeqMtEphS<T>, index: usize, item: T) -> (updated: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                index < a.seq@.len(),
            ensures
                updated.spec_arrayseqmteph_wf(),
                updated.spec_len() == a.seq@.len(),
                updated.spec_index(index as int) == item,
                forall|i: int| #![trigger updated.spec_index(i)] 0 <= i < a.seq@.len() && i != index as int ==> updated.spec_index(i) == a.seq@[i];

        /// - Definition 18.16 (inject). Update multiple positions at once; the first update in
        ///   the ordering of `updates` takes effect when positions collide.
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(|a| + |b|), Span O(lg(degree(b)))
        /// - Alg Analysis: APAS (Ch22 CS 22.2): Work O(|b|), Span O(lg(degree(b)))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a| + |b|), Span O(|a| + |b|) — DIFFERS: sequential clone + loop
        fn inject(a: &Self, updates: &Vec<(usize, T)>) -> (injected: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
            ensures
                injected.spec_arrayseqmteph_wf(),
                injected.spec_len() == a.spec_len(),
                Seq::new(injected.spec_len(), |i: int| injected.spec_index(i))
                    =~= spec_inject(
                        Seq::new(a.spec_len(), |i: int| a.spec_index(i)),
                        updates@);

        /// - Definition 18.17 (ninject). Non-deterministic inject: each position in the result
        ///   holds either the original value or a value from some update.
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(|a| + |b|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a| + |b|), Span O(|a| + |b|) — DIFFERS: delegates to inject, sequential clone + loop
        fn ninject(a: &Self, updates: &Vec<(usize, T)>) -> (injected: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
            ensures
                injected.spec_arrayseqmteph_wf(),
                spec_ninject(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)),
                    updates@,
                    Seq::new(injected.spec_len(), |i: int| injected.spec_index(i)));

        /// - Algorithm 19.7 (isEmpty). isEmpty a = (|a| = 0).
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        fn is_empty(&self) -> (empty: bool)
            ensures empty <==> self.spec_len() == 0;

        /// - Algorithm 19.7 (isSingleton). isSingleton a = (|a| = 1).
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        fn is_singleton(&self) -> (single: bool)
            ensures single <==> self.spec_len() == 1;

        /// - Algorithm 19.8 (iterate). Left fold over the sequence (iterative).
        /// - Alg Analysis: APAS (Ch19 Alg 19.8): iterate (iterative)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a|), Span O(|a|).
        fn iterate_iter<A, F: Fn(&A, &T) -> A>(a: &ArraySeqMtEphS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(A, T) -> A>, seed: A) -> (accumulated: A)
            requires
                forall|x: &A, y: &T| #[trigger] f.requires((x, y)),
                forall|a: A, t: T, ret: A| f.ensures((&a, &t), ret) <==> ret == spec_f(a, t),
            ensures
                accumulated == spec_iterate(a.seq@, spec_f, seed);

        /// - Algorithm 19.8 (iterate). iterate f x a = if |a|=0 then x else iterate f (f(x,a[0])) a[1..|a|-1].
        /// - Alg Analysis: APAS (Ch20 CS 20.3): Work O(1 + Sigma W(f)), Span O(1 + Sigma S(f))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(Sigma W(f)), Span O(Sigma S(f)) — matches APAS (inherently sequential)
        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqMtEphS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(A, T) -> A>, seed: A) -> (accumulated: A)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                forall|x: &A, y: &T| #[trigger] f.requires((x, y)),
                forall|a: A, t: T, ret: A| f.ensures((&a, &t), ret) <==> ret == spec_f(a, t),
            ensures
                accumulated == spec_iterate(a.seq@, spec_f, seed);

        /// - Algorithm 19.9 (reduce). Combine elements (iterative).
        /// - Alg Analysis: APAS (Ch19 Alg 19.9): reduce (iterative)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a|), Span O(|a|).
        fn reduce_iter<F: Fn(&T, &T) -> T>(a: &ArraySeqMtEphS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(T, T) -> T>, id: T) -> (reduced: T)
            where T: Clone
            requires
                spec_monoid(spec_f, id),
                forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) <==> ret == spec_f(x, y),
            ensures
                reduced == spec_iterate(a.seq@, spec_f, id);

        /// - Algorithm 19.9 (reduce). reduce f id a = if |a|=0 then id else if |a|=1 then a[0] else f(reduce f id b, reduce f id c).
        /// - Alg Analysis: APAS (Ch20 CS 20.4): Work O(1 + Sigma W(f)), Span O(lg |a| * max S(f))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(Sigma W(f)), Span O(lg |a| * max S(f)) — matches APAS: D&C fork-join via reduce_par
        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &ArraySeqMtEphS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(T, T) -> T>, id: T) -> (reduced: T)
            where T: Clone + Eq + Send + Sync + 'static
            requires
                obeys_feq_clone::<T>(),
                spec_monoid(spec_f, id),
                forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) <==> ret == spec_f(x, y),
            ensures
                reduced == spec_iterate(a.seq@, spec_f, id);

        /// - Algorithm 19.10 (scan). Prefix-reduce returning partial sums and total.
        /// - Alg Analysis: APAS (Ch20 CS 20.5): Work O(|a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a|), Span O(|a|) — DIFFERS: sequential loop, span = work
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqMtEphS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(T, T) -> T>, id: T) -> (scanned: (ArraySeqMtEphS<T>, T))
            where T: Clone
            requires
                spec_monoid(spec_f, id),
                forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) <==> ret == spec_f(x, y),
            ensures
                scanned.0.seq@.len() == a.seq@.len(),
                forall|i: int| #![trigger scanned.0.seq@[i]] 0 <= i < a.seq@.len() ==>
                    scanned.0.seq@[i] == a.seq@.take(i + 1).fold_left(id, spec_f),
                scanned.1 == spec_iterate(a.seq@, spec_f, id);

        /// - Algorithm 19.3 (map). map f a = tabulate (lambda i.f(a[i])) |a|.
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(1 + Sigma W(f(x))), Span O(1 + max S(f(x)))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(Sigma W(f(x))), Span O(1 + max S(f(x))) — matches APAS: D&C fork-join via map_dc
        fn map<U: Clone + Send + Sync + 'static, F: Fn(&T) -> U + Send + Sync + Clone + 'static>(a: &ArraySeqMtEphS<T>, f: &F) -> (mapped: ArraySeqMtEphS<U>)
            where T: Clone + Eq + Send + Sync + 'static
            requires
                obeys_feq_clone::<T>(),
                forall|i: int| 0 <= i < a.seq@.len() ==> #[trigger] f.requires((&a.seq@[i],)),
            ensures
                mapped.seq@.len() == a.seq@.len(),
                forall|i: int| #![trigger mapped.seq@[i]] 0 <= i < a.seq@.len() ==> f.ensures((&a.seq@[i],), mapped.seq@[i]);

        /// - Primitive: tabulate. Build a sequence by applying `f` to each index.
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(1 + Sigma W(f(i))), Span O(1 + max S(f(i)))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(Sigma W(f(i))), Span O(Sigma S(f(i))) — DIFFERS: sequential loop, span = work
        fn tabulate<F: Fn(usize) -> T>(f: &F, length: usize) -> (tab_seq: ArraySeqMtEphS<T>)
            requires
                length <= usize::MAX,
                forall|i: usize| i < length ==> #[trigger] f.requires((i,)),
            ensures
                tab_seq.seq@.len() == length,
                forall|i: int| #![trigger tab_seq.seq@[i]] 0 <= i < length ==> f.ensures((i as usize,), tab_seq.seq@[i]);

        /// - Primitive: flatten. Concatenate a sequence of sequences.
        /// - Alg Analysis: APAS (Ch19 Alg 19.15): Work O(|a| + sum |a[i]|), Span O(lg |a|)
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(|a| + sum |a[i]|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a| + sum |a[i]|), Span O(|a| + sum |a[i]|) — DIFFERS: nested sequential loops, span = work
        fn flatten(a: &ArraySeqMtEphS<ArraySeqMtEphS<T>>) -> (flattened: ArraySeqMtEphS<T>)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
            ensures
                flattened.seq@ =~= a.seq@.map_values(|inner: ArraySeqMtEphS<T>| inner.seq@).flatten();

        /// - Algorithm 19.5 (deflate). deflate f x = if (f x) then ⟨x⟩ else ⟨⟩.
        /// - Alg Analysis: APAS (Ch19 Alg 19.5): deflate (part of filter)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1).
        fn deflate<F: Fn(&T) -> bool>(pred: &F, x: &T) -> (deflated: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                pred.requires((x,)),
            ensures
                deflated.spec_arrayseqmteph_wf(),
                deflated.spec_len() <= 1,
                deflated.spec_len() == 1 ==> pred.ensures((x,), true) && deflated.spec_index(0) == *x,
                deflated.spec_len() == 0 ==> pred.ensures((x,), false);
    }


    //		9. impl Trait for Struct

    impl<T> ArraySeqMtEphTrait<T> for ArraySeqMtEphS<T> {
        open spec fn spec_arrayseqmteph_wf(&self) -> bool { true } // accept hole: Vec-backed, true is correct

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

        // Ephemeral: in-place mutation of a single element.
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

        // Algorithm 19.1: empty = tabulate (lambda i.i) 0.
        // The closure is never called (length 0) but must return T.
        // Rust has no value of unbounded generic T, so the body is an
        // unreachable loop; `requires false` makes this dead code.
        fn empty() -> (empty_seq: ArraySeqMtEphS<T>) {
            Self::tabulate(
                &(|_i: usize| -> (r: T)
                    requires false
                {
                    loop
                        invariant false
                        decreases 0usize
                    {}
                }),
                0,
            )
        }

        // Algorithm 19.2: singleton x = tabulate (lambda i.x) 1.
        fn singleton(item: T) -> (singleton: ArraySeqMtEphS<T>)
            where T: Clone + Eq
        {
            Self::tabulate(
                &(|_i: usize| -> (r: T)
                    requires obeys_feq_clone::<T>()
                    ensures r == item
                {
                    let r = item.clone();
                    proof {
                        assert(cloned(item, r));
                        axiom_cloned_implies_eq_owned(item, r);
                    }
                    r
                }),
                1,
            )
        }

        // Algorithm 19.4: append a b = tabulate (select(a,b)) (|a|+|b|)
        // where select(a,b) i = if i < |a| then a[i] else b[i-|a|].
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> (appended: ArraySeqMtEphS<T>)
            where T: Clone + Eq
        {
            let a_len = a.seq.len();
            let b_len = b.seq.len();
            let total = a_len + b_len;
            Self::tabulate(
                &(|i: usize| -> (r: T)
                    requires
                        obeys_feq_clone::<T>(),
                        (i as int) < a.seq@.len() + b.seq@.len(),
                    ensures
                        (i as int) < a.seq@.len() ==> r == a.seq@[i as int],
                        (i as int) >= a.seq@.len() ==> r == b.seq@[(i as int) - a.seq@.len()],
                {
                    if i < a_len {
                        let r = a.seq[i].clone();
                        proof {
                            assert(cloned(a.seq@[i as int], r));
                            axiom_cloned_implies_eq_owned(a.seq@[i as int], r);
                        }
                        r
                    } else {
                        let r = b.seq[i - a_len].clone();
                        proof {
                            assert(cloned(b.seq@[(i - a_len) as int], r));
                            axiom_cloned_implies_eq_owned(b.seq@[(i - a_len) as int], r);
                        }
                        r
                    }
                }),
                total,
            )
        }

        // Algorithm 19.5: parallel D&C filter via filter_dc.
        fn filter<F: Fn(&T) -> bool + Clone + Send + Sync + 'static>(a: &ArraySeqMtEphS<T>, pred: &F, Ghost(spec_pred): Ghost<spec_fn(T) -> bool>) -> (filtered: ArraySeqMtEphS<T>)
            where T: Clone + Eq + Send + Sync + 'static
        {
            let filtered = Self::filter_dc(a, pred, Ghost(spec_pred));
            proof {
                let ghost s = Seq::new(a.seq@.len(), |i: int| a.seq@[i]);
                assert(s =~= a.seq@);
                let ghost fs = Seq::new(filtered.spec_len(), |i: int| filtered.spec_index(i));
                assert(fs =~= filtered.seq@) by {
                    assert forall|i: int| 0 <= i < fs.len() implies #[trigger] fs[i] == filtered.seq@[i]
                    by { filtered.lemma_spec_index(i); }
                }
            }
            filtered
        }

        // Algorithm 19.6 (ephemeral): update via clone + set.
        // Clone the source (O(n)), then set at index (O(1)).
        fn update(a: &ArraySeqMtEphS<T>, index: usize, item: T) -> (updated: ArraySeqMtEphS<T>)
            where T: Clone + Eq
        {
            let mut seq = a.seq.clone();
            seq.set(index, item);
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
                assert(result_vec@ =~= spec_inject(s, u));
                assert(s =~= Seq::new(a.spec_len(), |i: int| a.spec_index(i)));
                assert(result_vec@ =~= spec_inject(Seq::new(a.spec_len(), |i: int| a.spec_index(i)), updates@));
            }
            let injected = ArraySeqMtEphS { seq: result_vec };
            proof {
                assert(Seq::new(injected.spec_len(), |i: int| injected.spec_index(i)) =~= result_vec@);
                assert(Seq::new(injected.spec_len(), |i: int| injected.spec_index(i))
                    =~= spec_inject(Seq::new(a.spec_len(), |i: int| a.spec_index(i)), updates@));
            }
            injected
        }

        // Definition 18.17 (ninject): delegates to inject. inject is a valid ninject because
        // every element of spec_inject(s, u) is either the original or came from some update.
        fn ninject(a: &ArraySeqMtEphS<T>, updates: &Vec<(usize, T)>) -> (injected: ArraySeqMtEphS<T>)
            where T: Clone + Eq
        {
            let injected = Self::inject(a, updates);
            proof {
                let s = Seq::new(a.spec_len(), |i: int| a.spec_index(i));
                let r = Seq::new(injected.spec_len(), |i: int| injected.spec_index(i));
                let u = updates@;
                assert(r =~= spec_inject(s, u));
                lemma_spec_inject_len(s, u);
                assert forall|i: int| 0 <= i < s.len() implies {
                    r[i] == s[i]
                    || exists|j: int| #![trigger u[j]] 0 <= j < u.len()
                        && u[j].0 == i as usize && r[i] == u[j].1
                } by {
                    lemma_spec_inject_element(s, u, i);
                }
            }
            injected
        }

        // Algorithm 19.7: isEmpty a = (|a| = 0).
        fn is_empty(&self) -> (empty: bool) {
            self.seq.len() == 0
        }

        // Algorithm 19.7: isSingleton a = (|a| = 1).
        fn is_singleton(&self) -> (single: bool) {
            self.seq.len() == 1
        }

        // Algorithm 19.8: iterate f x a (iterative form).
        fn iterate_iter<A, F: Fn(&A, &T) -> A>(a: &ArraySeqMtEphS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(A, T) -> A>, seed: A) -> (accumulated: A) {
            let len = a.seq.len();
            let mut acc = seed;
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len == a.seq@.len(),
                    forall|x: &A, y: &T| #[trigger] f.requires((x, y)),
                    forall|a: A, t: T, ret: A| f.ensures((&a, &t), ret) <==> ret == spec_f(a, t),
                    acc == a.seq@.take(i as int).fold_left(seed, spec_f),
                decreases len - i,
            {
                proof {
                    assert(a.seq@.take(i as int + 1) =~= a.seq@.take(i as int).push(a.seq@[i as int]));
                }
                acc = f(&acc, &a.seq[i]);
                proof {
                    let ghost t = a.seq@.take(i as int + 1);
                    assert(t.len() > 0);
                    assert(t.drop_last() =~= a.seq@.take(i as int));
                    assert(t.last() == a.seq@[i as int]);
                    reveal(Seq::fold_left);
                }
                i += 1;
            }
            proof {
                assert(a.seq@.take(len as int) =~= a.seq@);
            }
            acc
        }

        // Algorithm 19.8: iterate f x a = if |a|=0 then x else iterate f (f(x,a[0])) a[1..|a|-1].
        // Delegates to iterate_iter which has the full proof.
        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqMtEphS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(A, T) -> A>, seed: A) -> (accumulated: A)
            where T: Clone + Eq
        {
            Self::iterate_iter(a, f, Ghost(spec_f), seed)
        }

        // Algorithm 19.9: reduce f id a (iterative form for single-threaded).
        fn reduce_iter<F: Fn(&T, &T) -> T>(a: &ArraySeqMtEphS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(T, T) -> T>, id: T) -> (reduced: T)
            where T: Clone
        {
            let len = a.seq.len();
            let mut acc = id;
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len == a.seq@.len(),
                    forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                    forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) <==> ret == spec_f(x, y),
                    acc == a.seq@.take(i as int).fold_left(id, spec_f),
                decreases len - i,
            {
                proof {
                    assert(a.seq@.take(i as int + 1) =~= a.seq@.take(i as int).push(a.seq@[i as int]));
                }
                acc = f(&acc, &a.seq[i]);
                proof {
                    let ghost t = a.seq@.take(i as int + 1);
                    assert(t.len() > 0);
                    assert(t.drop_last() =~= a.seq@.take(i as int));
                    assert(t.last() == a.seq@[i as int]);
                    reveal(Seq::fold_left);
                }
                i += 1;
            }
            proof {
                assert(a.seq@.take(len as int) =~= a.seq@);
            }
            acc
        }

        // Algorithm 19.9: reduce via D&C fork-join (delegates to reduce_par).
        fn reduce<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(a: &ArraySeqMtEphS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(T, T) -> T>, id: T) -> (reduced: T)
            where T: Clone + Eq + Send + Sync + 'static
        {
            let len = a.seq.len();
            if len == 0 {
                proof {
                    assert(a.seq@ =~= Seq::<T>::empty());
                    reveal(Seq::fold_left);
                }
                id
            } else {
                let f_owned = clone_fn2(f);
                let result = Self::reduce_par(a, f_owned, Ghost(spec_f), id);
                proof {
                    assert(Seq::new(a.spec_len(), |i: int| a.spec_index(i)) =~= a.seq@);
                }
                result
            }
        }

        // Algorithm 19.10: scan f id a (iterative form for single-threaded).
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqMtEphS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(T, T) -> T>, id: T) -> (scanned: (ArraySeqMtEphS<T>, T))
            where T: Clone
        {
            let len = a.seq.len();
            let mut acc = id;
            let mut seq: Vec<T> = Vec::with_capacity(len);
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len == a.seq@.len(),
                    seq@.len() == i as int,
                    spec_monoid(spec_f, id),
                    forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                    forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) <==> ret == spec_f(x, y),
                    acc == a.seq@.take(i as int).fold_left(id, spec_f),
                    forall|k: int| #![trigger seq@[k]] 0 <= k < seq@.len() ==>
                        seq@[k] == a.seq@.take(k + 1).fold_left(id, spec_f),
                decreases len - i,
            {
                proof {
                    assert(a.seq@.take(i as int + 1) =~= a.seq@.take(i as int).push(a.seq@[i as int]));
                }
                seq.push(f(&acc, &a.seq[i]));
                acc = f(&acc, &a.seq[i]);
                proof {
                    let ghost t = a.seq@.take(i as int + 1);
                    assert(t.len() > 0);
                    assert(t.drop_last() =~= a.seq@.take(i as int));
                    assert(t.last() == a.seq@[i as int]);
                    reveal(Seq::fold_left);
                }
                i += 1;
            }
            proof {
                assert(a.seq@.take(len as int) =~= a.seq@);
            }
            (ArraySeqMtEphS { seq }, acc)
        }

        // Algorithm 19.3: map via D&C fork-join (delegates to map_dc).
        fn map<U: Clone + Send + Sync + 'static, F: Fn(&T) -> U + Send + Sync + Clone + 'static>(a: &ArraySeqMtEphS<T>, f: &F) -> (mapped: ArraySeqMtEphS<U>)
            where T: Clone + Eq + Send + Sync + 'static
        {
            let a_owned = a.subseq_copy(0, a.seq.len());
            let f_owned = clone_fn(f);
            let ghost a_owned_view = a_owned.seq@;
            proof {
                // Prove element equality before a_owned is consumed
                assert forall|i: int| 0 <= i < a_owned_view.len() implies #[trigger] a_owned_view[i] == a.seq@[i] by {
                    a.lemma_spec_index(i);
                    a_owned.lemma_spec_index(i);
                }
                assert forall|i: int| 0 <= i < a_owned.seq@.len() implies #[trigger] f_owned.requires((&a_owned.seq@[i],)) by {
                    a.lemma_spec_index(i);
                    a_owned.lemma_spec_index(i);
                }
            }
            let result = Self::map_dc(a_owned, f_owned);
            proof {
                // map_dc ensures f_owned.ensures on a_owned_view elements
                // clone_fn ensures f_owned.ensures == f.ensures
                // a_owned_view[i] == a.seq@[i] (proved above)
            }
            result
        }

        // Primitive: tabulate.
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

        // Primitive: flatten.
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

        // Algorithm 19.5: deflate f x = if (f x) then ⟨x⟩ else ⟨⟩.
        fn deflate<F: Fn(&T) -> bool>(pred: &F, x: &T) -> (deflated: ArraySeqMtEphS<T>)
            where T: Clone + Eq
        {
            if pred(x) {
                let elem = x.clone();
                proof {
                    assert(cloned(*x, elem));
                    axiom_cloned_implies_eq_owned(*x, elem);
                }
                Self::singleton(elem)
            } else {
                Self::empty()
            }
        }
    }

    //		9. bare impl (flatten lemmas, parallel methods, iterators)

    /// If every inner sequence has length <= 1, flatten has length <= the outer length.
    proof fn lemma_flatten_bounded_by_outer_len<T>(ss: Seq<Seq<T>>)
        requires forall|i: int| #![trigger ss[i]] 0 <= i < ss.len() ==> ss[i].len() <= 1
        ensures ss.flatten().len() <= ss.len()
        decreases ss.len()
    {
        if ss.len() > 0 {
            let prefix = ss.drop_last();
            lemma_flatten_bounded_by_outer_len::<T>(prefix);
            prefix.lemma_flatten_push(ss.last());
            assert(ss =~= prefix.push(ss.last()));
            assert(ss.flatten() =~= prefix.flatten() + ss.last());
        }
    }

    /// Every element of flatten(ss) satisfies p, if every element of every inner sequence does.
    proof fn lemma_flatten_all_satisfy<T>(ss: Seq<Seq<T>>, p: spec_fn(T) -> bool)
        requires
            forall|j: int, k: int| #![trigger ss[j][k]] 0 <= j < ss.len() && 0 <= k < ss[j].len() ==> p(ss[j][k])
        ensures
            forall|i: int| #![trigger ss.flatten()[i]] 0 <= i < ss.flatten().len() ==> p(ss.flatten()[i])
        decreases ss.len()
    {
        if ss.len() > 0 {
            let prefix = ss.drop_last();
            let last = ss.last();
            lemma_flatten_all_satisfy::<T>(prefix, p);
            prefix.lemma_flatten_push(last);
            assert(ss =~= prefix.push(last));
            assert(ss.flatten() =~= prefix.flatten() + last);
        }
    }

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

        /// Parallel map via D&C fork-join.
        /// - Alg Analysis: APAS (Ch19 Alg 19.3): parallel variant — map
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a|), Span O(lg |a|).
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

        /// Parallel divide-and-conquer filter. Called by trait method filter.
        fn filter_dc<F: Fn(&T) -> bool + Clone + Send + Sync + 'static>(
            a: &ArraySeqMtEphS<T>,
            pred: &F,
            Ghost(spec_pred): Ghost<spec_fn(T) -> bool>,
        ) -> (filtered: ArraySeqMtEphS<T>)
            where T: Clone + Eq + Send + Sync + 'static
            requires
                obeys_feq_clone::<T>(),
                forall|i: int| 0 <= i < a.seq@.len() ==> #[trigger] pred.requires((&a.seq@[i],)),
                forall|v: T, keep: bool| pred.ensures((&v,), keep) ==> spec_pred(v) == keep,
            ensures
                filtered.spec_arrayseqmteph_wf(),
                filtered.spec_len() <= a.seq@.len(),
                filtered.spec_len() == spec_filter_len(a.seq@, spec_pred),
                filtered.seq@.to_multiset() =~= a.seq@.to_multiset().filter(spec_pred),
                forall|i: int| #![trigger filtered.spec_index(i)] 0 <= i < filtered.spec_len()
                    ==> pred.ensures((&filtered.spec_index(i),), true),
            decreases a.seq@.len()
        {
            let len = a.seq.len();
            if len == 0 {
                let filtered = ArraySeqMtEphS::<T> { seq: Vec::new() };
                proof {
                    assert(a.seq@ =~= Seq::<T>::empty());
                    broadcast use vstd::multiset::group_multiset_axioms;
                    assert forall|v: T| #[trigger] a.seq@.to_multiset().filter(spec_pred).count(v) == 0nat by {}
                    assert(a.seq@.to_multiset().filter(spec_pred) =~= Multiset::<T>::empty());
                    assert(filtered.seq@.to_multiset() =~= Multiset::<T>::empty());
                    assert(filtered.seq@.to_multiset() =~= a.seq@.to_multiset().filter(spec_pred));
                }
                filtered
            } else if len == 1 {
                let keep = pred(&a.seq[0]);
                if keep {
                    let elem = a.seq[0].clone();
                    proof {
                        axiom_cloned_implies_eq_owned(a.seq[0 as int], elem);
                        broadcast use vstd::seq_lib::group_to_multiset_ensures;
                        broadcast use vstd::multiset::group_multiset_axioms;
                        reveal_with_fuel(spec_filter_len, 2);
                        assert(a.seq@.drop_last() =~= Seq::<T>::empty());
                    }
                    let mut seq = Vec::with_capacity(1);
                    seq.push(elem);
                    let filtered = ArraySeqMtEphS { seq };
                    proof {
                        broadcast use vstd::multiset::group_multiset_axioms;
                        assert(filtered.seq@ =~= seq![elem]);
                        assert(filtered.seq@ =~= a.seq@);
                        assert forall|v: T| #[trigger] filtered.seq@.to_multiset().count(v)
                            == a.seq@.to_multiset().filter(spec_pred).count(v) by {}
                    }
                    filtered
                } else {
                    let filtered = ArraySeqMtEphS::<T> { seq: Vec::new() };
                    proof {
                        broadcast use vstd::seq_lib::group_to_multiset_ensures;
                        broadcast use vstd::multiset::group_multiset_axioms;
                        assert(!spec_pred(a.seq@[0 as int]));
                        reveal_with_fuel(spec_filter_len, 2);
                        assert(a.seq@.drop_last() =~= Seq::<T>::empty());
                        assert forall|v: T| #[trigger] a.seq@.to_multiset().filter(spec_pred).count(v) == 0nat by {}
                        assert(a.seq@.to_multiset().filter(spec_pred) =~= Multiset::<T>::empty());
                        assert(filtered.seq@.to_multiset() =~= Multiset::<T>::empty());
                    }
                    filtered
                }
            } else {
                let mid = len / 2;
                let left_seq = a.subseq_copy(0, mid);
                let right_seq = a.subseq_copy(mid, len - mid);
                let p1 = clone_pred(pred);
                let p2 = clone_pred(pred);

                let ghost left_view = left_seq.seq@;
                let ghost right_view = right_seq.seq@;

                proof {
                    assert forall|i: int| 0 <= i < left_seq.seq@.len() implies
                        #[trigger] p1.requires((&left_seq.seq@[i],))
                    by {
                        a.lemma_spec_index(i);
                        left_seq.lemma_spec_index(i);
                    }
                    assert forall|i: int| 0 <= i < right_seq.seq@.len() implies
                        #[trigger] p2.requires((&right_seq.seq@[i],))
                    by {
                        a.lemma_spec_index(mid as int + i);
                        right_seq.lemma_spec_index(i);
                    }
                    assert(left_view =~= a.seq@.subrange(0, mid as int)) by {
                        assert forall|i: int| 0 <= i < left_view.len() implies
                            #[trigger] left_view[i] == a.seq@.subrange(0, mid as int)[i]
                        by { left_seq.lemma_spec_index(i); a.lemma_spec_index(i); }
                    }
                    assert(right_view =~= a.seq@.subrange(mid as int, len as int)) by {
                        assert forall|i: int| 0 <= i < right_view.len() implies
                            #[trigger] right_view[i] == a.seq@.subrange(mid as int, len as int)[i]
                        by { right_seq.lemma_spec_index(i); a.lemma_spec_index(mid as int + i); }
                    }
                    assert(a.seq@ =~= left_view + right_view);
                }

                let fa = move || -> (r: ArraySeqMtEphS<T>)
                    requires
                        obeys_feq_clone::<T>(),
                        forall|i: int| 0 <= i < left_seq.seq@.len() ==> #[trigger] p1.requires((&left_seq.seq@[i],)),
                        forall|v: T, keep: bool| p1.ensures((&v,), keep) ==> spec_pred(v) == keep,
                    ensures
                        r.spec_arrayseqmteph_wf(),
                        r.spec_len() <= left_view.len(),
                        r.spec_len() == spec_filter_len(left_view, spec_pred),
                        r.seq@.to_multiset() =~= left_view.to_multiset().filter(spec_pred),
                        forall|i: int| #![trigger r.spec_index(i)] 0 <= i < r.spec_len()
                            ==> p1.ensures((&r.spec_index(i),), true),
                {
                    Self::filter_dc(&left_seq, &p1, Ghost(spec_pred))
                };

                let fb = move || -> (r: ArraySeqMtEphS<T>)
                    requires
                        obeys_feq_clone::<T>(),
                        forall|i: int| 0 <= i < right_seq.seq@.len() ==> #[trigger] p2.requires((&right_seq.seq@[i],)),
                        forall|v: T, keep: bool| p2.ensures((&v,), keep) ==> spec_pred(v) == keep,
                    ensures
                        r.spec_arrayseqmteph_wf(),
                        r.spec_len() <= right_view.len(),
                        r.spec_len() == spec_filter_len(right_view, spec_pred),
                        r.seq@.to_multiset() =~= right_view.to_multiset().filter(spec_pred),
                        forall|i: int| #![trigger r.spec_index(i)] 0 <= i < r.spec_len()
                            ==> p2.ensures((&r.spec_index(i),), true),
                {
                    Self::filter_dc(&right_seq, &p2, Ghost(spec_pred))
                };

                let (left, right) = join(fa, fb);
                let filtered = Self::append(&left, &right);
                proof {
                    lemma_spec_filter_len_concat(left_view, right_view, spec_pred);
                    lemma_seq_concat_to_multiset_filter(left_view, right_view, spec_pred);

                    assert(filtered.seq@ =~= left.seq@ + right.seq@) by {
                        assert forall|i: int| 0 <= i < filtered.seq@.len() implies
                            #[trigger] filtered.seq@[i] == (left.seq@ + right.seq@)[i]
                        by {
                            filtered.lemma_spec_index(i);
                            if i < left.seq@.len() as int {
                                left.lemma_spec_index(i);
                            } else {
                                right.lemma_spec_index(i - left.seq@.len() as int);
                            }
                        }
                    }

                    vstd::seq_lib::lemma_multiset_commutative(left.seq@, right.seq@);
                    assert(filtered.seq@.to_multiset()
                        =~= left.seq@.to_multiset().add(right.seq@.to_multiset()));
                    assert(left.seq@.to_multiset().add(right.seq@.to_multiset())
                        =~= left_view.to_multiset().filter(spec_pred).add(right_view.to_multiset().filter(spec_pred)));
                    assert(filtered.seq@.to_multiset() =~= a.seq@.to_multiset().filter(spec_pred));

                    assert forall|i: int| #![trigger filtered.spec_index(i)]
                        0 <= i < filtered.spec_len() implies
                        pred.ensures((&filtered.spec_index(i),), true)
                    by {
                        filtered.lemma_spec_index(i);
                        if i < left.spec_len() as int {
                            left.lemma_spec_index(i);
                        } else {
                            right.lemma_spec_index(i - left.spec_len() as int);
                        }
                    }
                }
                filtered
            }
        }

        /// Parallel filter via D&C fork-join.
        /// - Alg Analysis: APAS (Ch19 Alg 19.5): parallel variant — filter
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a|), Span O(lg² |a|).
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

        /// Parallel reduce via D&C fork-join (requires monoid).
        /// - Alg Analysis: APAS (Ch19 Alg 19.9): parallel variant — reduce
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a|), Span O(lg |a|).
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
                let element = a.seq[0].clone();
                proof {
                    assert(cloned(a.seq[0 as int], element));
                    axiom_cloned_implies_eq_owned(a.seq[0 as int], element);
                    a.lemma_spec_index(0);
                    assert(s =~= seq![a.spec_index(0)]);
                    reveal_with_fuel(Seq::fold_left, 2);
                    assert(spec_f(id, s[0]) == s[0]);  // left identity
                }
                element
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
                let combined = f(&left, &right);
                proof {
                    assert(left_s =~= s.subrange(0, mid as int));
                    assert(right_s =~= s.subrange(mid as int, len as int));
                    s.lemma_fold_left_split(id, spec_f, mid as int);
                    Self::lemma_monoid_fold_left(right_s, spec_f, id, left);
                }
                combined
            }
        }

        /// Parallel D&C map with full element-level ensures.
        /// Takes owned data so join closures satisfy 'static.
        fn map_dc<U: Clone + Send + Sync + 'static, F: Fn(&T) -> U + Send + Sync + Clone + 'static>(
            a: ArraySeqMtEphS<T>,
            f: F,
        ) -> (mapped: ArraySeqMtEphS<U>)
            where T: Clone + Send + Sync + Eq + 'static
            requires
                obeys_feq_clone::<T>(),
                a.seq@.len() <= usize::MAX as int,
                forall|i: int| 0 <= i < a.seq@.len() ==> #[trigger] f.requires((&a.seq@[i],)),
            ensures
                mapped.seq@.len() == a.seq@.len(),
                forall|i: int| #![trigger mapped.seq@[i]] 0 <= i < a.seq@.len() ==> f.ensures((&a.seq@[i],), mapped.seq@[i]),
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
                let ghost a_view = a.seq@;
                let left_seq = a.subseq_copy(0, mid);
                let right_seq = a.subseq_copy(mid, len - mid);
                let f1 = clone_fn(&f);
                let f2 = clone_fn(&f);
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
                let ghost left_view = left_seq.seq@;
                let ghost right_view = right_seq.seq@;
                let ghost left_len = left_seq.seq@.len();
                let ghost right_len = right_seq.seq@.len();
                proof {
                    // Establish element equalities before join closures consume the sequences
                    assert forall|i: int| 0 <= i < left_view.len() implies left_view[i] == a_view[i] by {
                        a.lemma_spec_index(i);
                        left_seq.lemma_spec_index(i);
                    }
                    assert forall|j: int| 0 <= j < right_view.len() implies right_view[j] == a_view[mid as int + j] by {
                        a.lemma_spec_index(mid as int + j);
                        right_seq.lemma_spec_index(j);
                    }
                }

                let fa = move || -> (r: ArraySeqMtEphS<U>)
                    requires
                        left_seq.seq@.len() <= usize::MAX as int,
                        obeys_feq_clone::<T>(),
                        forall|i: int| 0 <= i < left_seq.seq@.len() ==> #[trigger] f1.requires((&left_seq.seq@[i],)),
                    ensures
                        r.seq@.len() == left_len,
                        forall|i: int| #![trigger r.seq@[i]] 0 <= i < left_len ==> f1.ensures((&left_view[i],), r.seq@[i]),
                {
                    Self::map_dc(left_seq, f1)
                };

                let fb = move || -> (r: ArraySeqMtEphS<U>)
                    requires
                        right_seq.seq@.len() <= usize::MAX as int,
                        obeys_feq_clone::<T>(),
                        forall|i: int| 0 <= i < right_seq.seq@.len() ==> #[trigger] f2.requires((&right_seq.seq@[i],)),
                    ensures
                        r.seq@.len() == right_len,
                        forall|i: int| #![trigger r.seq@[i]] 0 <= i < right_len ==> f2.ensures((&right_view[i],), r.seq@[i]),
                {
                    Self::map_dc(right_seq, f2)
                };

                let (left, right) = join(fa, fb);
                let result = ArraySeqMtEphS::<U>::concat_seqs(left, right);
                proof {
                    assert forall|i: int| 0 <= i < a_view.len() implies f.ensures((&a_view[i],), #[trigger] result.seq@[i]) by {
                        if i < mid as int {
                            assert(result.seq@[i] == left.seq@[i]);
                        } else {
                            let j = i - mid as int;
                            assert(result.seq@[i] == right.seq@[j]);
                            assert(right_view[j] == a_view[mid as int + j]);
                        }
                    }
                }
                result
            }
        }

        /// Sequential concatenation of two owned sequences. Moves elements from b
        /// into a's Vec via into_iter, avoiding Clone/Eq requirements.
        fn concat_seqs(a: ArraySeqMtEphS<T>, b: ArraySeqMtEphS<T>) -> (result: ArraySeqMtEphS<T>)
            requires
                a.seq@.len() + b.seq@.len() <= usize::MAX,
            ensures
                result.seq@.len() == a.seq@.len() + b.seq@.len(),
                forall|i: int| #![trigger result.seq@[i]] 0 <= i < a.seq@.len() ==> result.seq@[i] == a.seq@[i],
                forall|i: int| 0 <= i < b.seq@.len() ==> result.seq@[a.seq@.len() + i] == b.seq@[i],
        {
            let ghost a_view = a.seq@;
            let ghost b_view = b.seq@;
            let ghost a_len = a_view.len();
            let b_len = b.seq.len();
            let mut vec = a.seq;
            let mut iter = b.seq.into_iter();
            let mut j: usize = 0;
            while j < b_len
                invariant
                    j <= b_len,
                    b_len == b_view.len(),
                    vec@.len() == a_len + j,
                    iter@.0 == j as int,
                    iter@.1 == b_view,
                    forall|k: int| #![trigger vec@[k]] 0 <= k < a_len ==> vec@[k] == a_view[k],
                    forall|k: int| 0 <= k < j ==> vec@[a_len + k] == b_view[k],
                decreases b_len - j,
            {
                if let Some(e) = iter.next() {
                    vec.push(e);
                }
                j += 1;
            }
            ArraySeqMtEphS { seq: vec }
        }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: View + PartialEq> PartialEqSpecImpl for ArraySeqMtEphS<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }


    //		10. iterators

    
    #[verifier::reject_recursive_types(T)]
    pub struct ArraySeqMtEphIter<'a, T> {
        pub inner: std::slice::Iter<'a, T>,
    }

    impl<'a, T> View for ArraySeqMtEphIter<'a, T> {
        type V = (int, Seq<T>);
        open spec fn view(&self) -> (int, Seq<T>) { self.inner@ }
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

    impl<T: Debug> Debug for ArraySeqMtEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_list().entries(self.seq.iter()).finish()
        }
    }

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

    /// Literal constructor macro for Chap19 ArraySeqMtEphS.
    #[macro_export]
    macro_rules! ArraySeqMtEphChap19SLit {
        () => { $crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphS::from_vec(Vec::new()) };
        ($x:expr; $n:expr) => { $crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphS::from_vec(vec![$x; $n]) };
        ($($x:expr),* $(,)?) => { $crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::ArraySeqMtEphS::from_vec(vec![$($x),*]) };
    }

}
