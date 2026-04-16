// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO

//! Chapter 19 parametric sequence implementation for array-backed sequences. Verusified.
//! Functions are implemented in terms of primitives: nth, length, subseq, tabulate, flatten.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 5. view impls
//	Section 6. spec fns
//	Section 7. proof fns/broadcast groups
//	Section 8. traits
//	Section 9. impls
//	Section 10. iterators
//	Section 12. derive impls in verus!
//	Section 13. macros
//	Section 14. derive impls outside verus!

//		Section 1. module


pub mod ArraySeqStPer {


    //		Section 2. imports

    use std::fmt::{Debug, Display, Formatter};
    use std::fmt::Result as FmtResult;
    use std::slice::Iter;
    use std::vec::IntoIter;

    use vstd::prelude::*;
    pub use crate::Chap19::ArraySeqSpecsAndLemmas::ArraySeqSpecsAndLemmas::*;

    verus!
{


    #[cfg(verus_keep_ghost)]
    use {
        vstd::std_specs::cmp::PartialEqSpecImpl,
        vstd::std_specs::vec::*,
        vstd::std_specs::clone::*,
    };
    #[cfg(verus_keep_ghost)]
    use {
        crate::vstdplus::accept::accept,
        crate::vstdplus::feq::feq::*,
        crate::vstdplus::monoid::monoid::*,
        crate::vstdplus::multiset::multiset::*,
    };

    //		Section 3. broadcast use


    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
        vstd::seq_lib::group_seq_properties,
        crate::vstdplus::feq::feq::group_feq_axioms,
        vstd::seq_lib::group_to_multiset_ensures,
    };

    //		Section 4. type definitions


    #[verifier::reject_recursive_types(T)]
    pub struct ArraySeqStPerS<T> {
        pub seq: Vec<T>,
    }

    //		Section 5. view impls


    impl<T: View> View for ArraySeqStPerS<T> {
        type V = Seq<T::V>;

        open spec fn view(&self) -> Seq<T::V> {
            self.seq@.map(|_i: int, t: T| t@)
        }
    }

    //		Section 7. proof fns/broadcast groups


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
            // Veracity: NEEDED assert
            assert(ss =~= prefix.push(ss.last()));
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
            // Veracity: NEEDED assert
            assert(ss =~= prefix.push(last));
        }
    }

    //		Section 8. traits


    /// Chapter 19 single-threaded persistent array sequence trait.
    /// Specifications match Chapter 18; algorithms from Chapter 19.
    pub trait ArraySeqStPerTrait<T>: Sized {
        spec fn spec_arrayseqstper_wf(&self) -> bool;

        spec fn spec_len(&self) -> nat;
        spec fn spec_index(&self, i: int) -> T
            recommends i < self.spec_len();

        /// - Create a new sequence of length `length` with each element initialized to `init_value`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(length), Span O(length) — ACCEPTED DIFFERENCE: St sequential, APAS parallel.
        fn new(length: usize, init_value: T) -> (new_seq: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                length <= usize::MAX,
            ensures
                new_seq.spec_arrayseqstper_wf(),
                new_seq.spec_len() == length as int,
                forall|i: int| #![trigger new_seq.spec_index(i)] 0 <= i < length ==> new_seq.spec_index(i) == init_value;

        /// - Definition 18.1 (length). Return the number of elements.
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn length(&self) -> (len: usize)
            ensures len as int == self.spec_len();

        /// - Algorithm 19.11 (Function nth). Return a reference to the element at `index`.
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(1), Span O(1)
        /// - Alg Analysis: APAS (Ch22 CS 22.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn nth(&self, index: usize) -> (nth_elem: &T)
            requires index < self.spec_len()
            ensures *nth_elem == self.spec_index(index as int);

        /// - Definition 18.12 (subseq copy). Extract contiguous subsequence with allocation.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(length), Span O(length) — ACCEPTED DIFFERENCE: St sequential, APAS parallel.
        fn subseq_copy(&self, start: usize, length: usize) -> (subseq: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                start + length <= usize::MAX,
                start + length <= self.spec_len(),
            ensures
                subseq.spec_arrayseqstper_wf(),
                subseq.spec_len() == length as int,
                forall|i: int| #![trigger subseq.spec_index(i)] 0 <= i < length ==> subseq.spec_index(i) == self.spec_index(start as int + i);

        /// - Definition 18.12 (subseq). Extract a contiguous subsequence.
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(length), Span O(length) — ACCEPTED DIFFERENCE: Vec-backed, copies elements sequentially
        fn subseq(a: &Self, start: usize, length: usize) -> (subseq: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                start + length <= usize::MAX,
                start + length <= a.spec_len(),
            ensures
                subseq.spec_arrayseqstper_wf(),
                subseq.spec_len() == length as int,
                forall|i: int| #![trigger subseq.spec_index(i)] 0 <= i < length ==> subseq.spec_index(i) == a.spec_index(start as int + i);

        /// - Create sequence from Vec.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — wraps Vec in struct, no copy.
        fn from_vec(elts: Vec<T>) -> (seq: Self)
            ensures
                seq.spec_arrayseqstper_wf(),
                seq.spec_len() == elts@.len(),
                forall|i: int| #![trigger seq.spec_index(i)] 0 <= i < elts@.len() ==> seq.spec_index(i) == elts@[i];

        /// - Algorithm 19.1 (empty). empty = tabulate (lambda i.i) 0.
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (empty_seq: Self)
            ensures empty_seq.spec_arrayseqstper_wf(), empty_seq.spec_len() == 0;

        /// - Algorithm 19.2 (singleton). singleton x = tabulate (lambda i.x) 1.
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(item: T) -> (singleton: Self)
            where T: Clone + Eq
            requires obeys_feq_clone::<T>()
            ensures
                singleton.spec_arrayseqstper_wf(),
                singleton.spec_len() == 1,
                singleton.spec_index(0) == item;

        /// - Algorithm 19.4 (append). append a b = tabulate (select(a,b)) (|a|+|b|).
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(|a| + |b|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a| + |b|), Span O(|a| + |b|) — ACCEPTED DIFFERENCE: sequential tabulate, span = work
        fn append(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> (appended: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                a.seq@.len() + b.seq@.len() <= usize::MAX as int,
            ensures
                appended.spec_arrayseqstper_wf(),
                appended.spec_len() == a.seq@.len() + b.seq@.len(),
                forall|i: int| #![trigger appended.spec_index(i)] 0 <= i < a.seq@.len() ==> appended.spec_index(i) == a.seq@[i],
                forall|i: int| #![trigger b.seq@[i]] 0 <= i < b.seq@.len() ==> appended.spec_index(a.seq@.len() as int + i) == b.seq@[i];

        /// - Algorithm 19.5 (filter). filter f a = flatten (map (deflate f) a).
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(1 + Sigma W(f(x))), Span O(lg |a| + max S(f(x)))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a| * (1 + W(f))), Span O(|a| * (1 + S(f))) — ACCEPTED DIFFERENCE: sequential map+flatten, span = work
        fn filter<F: Fn(&T) -> bool>(a: &ArraySeqStPerS<T>, pred: &F, Ghost(spec_pred): Ghost<spec_fn(T) -> bool>) -> (filtered: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                forall|i: int| 0 <= i < a.seq@.len() ==> #[trigger] pred.requires((&a.seq@[i],)),
                forall|v: T, keep: bool| pred.ensures((&v,), keep) ==> spec_pred(v) == keep,
            ensures
                filtered.spec_arrayseqstper_wf(),
                filtered.spec_len() <= a.seq@.len(),
                filtered.spec_len() == spec_filter_len(
                    Seq::new(a.seq@.len(), |i: int| a.seq@[i]), spec_pred),
                Seq::new(filtered.spec_len(), |i: int| filtered.spec_index(i)).to_multiset()
                    =~= Seq::new(a.seq@.len(), |i: int| a.seq@[i]).to_multiset().filter(spec_pred),
                forall|i: int| #![trigger filtered.spec_index(i)] 0 <= i < filtered.spec_len() ==> pred.ensures((&filtered.spec_index(i),), true);

        /// - Algorithm 19.6 (update). update a (i, x) = tabulate (lambda j. if i=j then x else a[j]) |a|.
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(|a|), Span O(1)
        /// - Alg Analysis: APAS (Ch22 CS 22.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a|), Span O(|a|) — ACCEPTED DIFFERENCE: clone entire Vec + set, sequential
        fn update(a: &ArraySeqStPerS<T>, index: usize, item: T) -> (updated: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                index < a.seq@.len(),
            ensures
                updated.spec_arrayseqstper_wf(),
                updated.spec_len() == a.seq@.len(),
                updated.spec_index(index as int) == item,
                forall|i: int| #![trigger updated.spec_index(i)] 0 <= i < a.seq@.len() && i != index as int ==> updated.spec_index(i) == a.seq@[i];

        /// - Definition 18.16 (inject). Update multiple positions at once; the first update in
        ///   the ordering of `updates` takes effect when positions collide.
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(|a| + |b|), Span O(lg(degree(b)))
        /// - Alg Analysis: APAS (Ch22 CS 22.2): Work O(|b|), Span O(lg(degree(b)))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a| + |b|), Span O(|a| + |b|) — ACCEPTED DIFFERENCE: sequential clone + loop
        fn inject(a: &Self, updates: &Vec<(usize, T)>) -> (injected: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
            ensures
                injected.spec_arrayseqstper_wf(),
                injected.spec_len() == a.spec_len(),
                Seq::new(injected.spec_len(), |i: int| injected.spec_index(i))
                    =~= spec_inject(
                        Seq::new(a.spec_len(), |i: int| a.spec_index(i)),
                        updates@);

        /// - Algorithm 19.7 (isEmpty). isEmpty a = (|a| = 0).
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_empty(&self) -> (empty: bool)
            ensures empty <==> self.spec_len() == 0;

        /// - Algorithm 19.7 (isSingleton). isSingleton a = (|a| = 1).
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_singleton(&self) -> (single: bool)
            ensures single <==> self.spec_len() == 1;

        /// - Algorithm 19.8 (iterate). Left fold over the sequence (iterative).
        /// - Alg Analysis: APAS (Ch19 Alg 19.8): iterate (iterative form)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a|), Span O(|a|)
        fn iterate_iter<A, F: Fn(&A, &T) -> A>(a: &ArraySeqStPerS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(A, T) -> A>, seed: A) -> (accumulated: A)
            requires
                forall|x: &A, y: &T| #[trigger] f.requires((x, y)),
                forall|a: A, t: T, ret: A| f.ensures((&a, &t), ret) <==> ret == spec_f(a, t),
            ensures
                accumulated == spec_iterate(a.seq@, spec_f, seed);

        /// - Algorithm 19.8 (iterate). iterate f x a = if |a|=0 then x else iterate f (f(x,a[0])) a[1..|a|-1].
        /// - Alg Analysis: APAS (Ch20 CS 20.3): Work O(1 + Sigma W(f)), Span O(1 + Sigma S(f))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(Sigma W(f)) — ACCEPTED DIFFERENCE: St sequential, APAS parallel
        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqStPerS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(A, T) -> A>, seed: A) -> (accumulated: A)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                forall|x: &A, y: &T| #[trigger] f.requires((x, y)),
                forall|a: A, t: T, ret: A| f.ensures((&a, &t), ret) <==> ret == spec_f(a, t),
            ensures
                accumulated == spec_iterate(a.seq@, spec_f, seed);

        /// - Algorithm 19.9 (reduce). Combine elements (iterative).
        /// - Alg Analysis: APAS (Ch19 Alg 19.9): reduce (iterative form)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a|), Span O(|a|)
        fn reduce_iter<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(T, T) -> T>, id: T) -> (reduced: T)
            where T: Clone
            requires
                spec_monoid(spec_f, id),
                forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) <==> ret == spec_f(x, y),
            ensures
                reduced == spec_iterate(a.seq@, spec_f, id);

        /// - Algorithm 19.9 (reduce). reduce f id a = if |a|=0 then id else if |a|=1 then a[0] else f(reduce f id b, reduce f id c).
        /// - Alg Analysis: APAS (Ch20 CS 20.4): Work O(1 + Sigma W(f)), Span O(lg |a| * max S(f))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(Sigma W(f)), Span O(Sigma S(f)) — ACCEPTED DIFFERENCE: delegates to reduce_iter (sequential), span = work
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(T, T) -> T>, id: T) -> (reduced: T)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                spec_monoid(spec_f, id),
                forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) <==> ret == spec_f(x, y),
            ensures
                reduced == spec_iterate(a.seq@, spec_f, id);

        /// - Algorithm 19.10 (scan). Prefix-reduce returning partial sums and total.
        /// - Alg Analysis: APAS (Ch20 CS 20.5): Work O(|a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a|), Span O(|a|) — ACCEPTED DIFFERENCE: sequential loop, span = work
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(T, T) -> T>, id: T) -> (scanned: (ArraySeqStPerS<T>, T))
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(Sigma W(f(x))), Span O(Sigma S(f(x))) — ACCEPTED DIFFERENCE: sequential tabulate, span = work
        fn map<U: Clone, F: Fn(&T) -> U>(a: &ArraySeqStPerS<T>, f: &F) -> (mapped: ArraySeqStPerS<U>)
            requires
                forall|i: int| 0 <= i < a.seq@.len() ==> #[trigger] f.requires((&a.seq@[i],)),
            ensures
                mapped.seq@.len() == a.seq@.len(),
                forall|i: int| #![trigger mapped.seq@[i]] 0 <= i < a.seq@.len() ==> f.ensures((&a.seq@[i],), mapped.seq@[i]);

        /// - Primitive: tabulate. Build a sequence by applying `f` to each index.
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(1 + Sigma W(f(i))), Span O(1 + max S(f(i)))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(Sigma W(f(i))), Span O(Sigma S(f(i))) — ACCEPTED DIFFERENCE: sequential loop, span = work
        fn tabulate<F: Fn(usize) -> T>(f: &F, length: usize) -> (tab_seq: ArraySeqStPerS<T>)
            requires
                length <= usize::MAX,
                forall|i: usize| i < length ==> #[trigger] f.requires((i,)),
            ensures
                tab_seq.seq@.len() == length,
                forall|i: int| #![trigger tab_seq.seq@[i]] 0 <= i < length ==> f.ensures((i as usize,), tab_seq.seq@[i]);

        /// - Primitive: flatten. Concatenate a sequence of sequences.
        /// - Alg Analysis: APAS (Ch19 Alg 19.15): Work O(|a| + sum |a[i]|), Span O(lg |a|)
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(|a| + sum |a[i]|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a| + sum |a[i]|), Span O(|a| + sum |a[i]|) — ACCEPTED DIFFERENCE: nested sequential loops, span = work
        fn flatten(a: &ArraySeqStPerS<ArraySeqStPerS<T>>) -> (flattened: ArraySeqStPerS<T>)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
            ensures
                flattened.seq@ =~= a.seq@.map_values(|inner: ArraySeqStPerS<T>| inner.seq@).flatten();

        /// - Algorithm 19.5 (deflate). deflate f x = if (f x) then ⟨x⟩ else ⟨⟩.
        /// - Alg Analysis: APAS (Ch19 Alg 19.5): deflate (part of filter)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn deflate<F: Fn(&T) -> bool>(pred: &F, x: &T) -> (deflated: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                pred.requires((x,)),
            ensures
                deflated.spec_arrayseqstper_wf(),
                deflated.spec_len() <= 1,
                deflated.spec_len() == 1 ==> pred.ensures((x,), true) && deflated.spec_index(0) == *x,
                deflated.spec_len() == 0 ==> pred.ensures((x,), false);
    }

    //		Section 9. impls


    impl<T> ArraySeqStPerTrait<T> for ArraySeqStPerS<T> {
        open spec fn spec_arrayseqstper_wf(&self) -> bool { true } // accept hole: Vec-backed, true is correct

        open spec fn spec_len(&self) -> nat {
            self.seq@.len()
        }

        open spec fn spec_index(&self, i: int) -> T {
            self.seq[i]
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn new(length: usize, init_value: T) -> (new_seq: ArraySeqStPerS<T>)
            where T: Clone + Eq
        {
            let seq = std::vec::from_elem(init_value, length);
            ArraySeqStPerS { seq }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn length(&self) -> (len: usize) {
            self.seq.len()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn nth(&self, index: usize) -> (nth_elem: &T) {
            &self.seq[index]
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(length), Span O(length)
        fn subseq_copy(&self, start: usize, length: usize) -> (subseq: ArraySeqStPerS<T>)
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
                // Veracity: NEEDED proof block
                proof {
                    let ghost last = seq@[seq@.len() - 1 as int];
                    axiom_cloned_implies_eq_owned(self.seq[i as int], last);
                }
                i += 1;
            }
            ArraySeqStPerS { seq }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(length), Span O(length)
        fn subseq(a: &ArraySeqStPerS<T>, start: usize, length: usize) -> (subseq: ArraySeqStPerS<T>)
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
                // Veracity: NEEDED proof block
                seq.push(a.seq[i].clone());
                proof {
                    let ghost last = seq@[seq@.len() - 1 as int];
                    axiom_cloned_implies_eq_owned(a.seq[i as int], last);
                }
                i += 1;
            }
            ArraySeqStPerS { seq }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn from_vec(elts: Vec<T>) -> (seq: ArraySeqStPerS<T>) {
            ArraySeqStPerS { seq: elts }
        }

        // Algorithm 19.1: empty = tabulate (lambda i.i) 0.
        // The closure is never called (length 0) but must return T.
        // Rust has no value of unbounded generic T, so the body is an
        // unreachable loop; `requires false` makes this dead code.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (empty_seq: ArraySeqStPerS<T>) {
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(item: T) -> (singleton: ArraySeqStPerS<T>)
            where T: Clone + Eq
        {
            Self::tabulate(
                &(|_i: usize| -> (r: T)
                    requires obeys_feq_clone::<T>()
                    ensures r == item
                // Veracity: NEEDED proof block
                {
                    let r = item.clone();
                    proof {
                        axiom_cloned_implies_eq_owned(item, r);
                    }
                    r
                }),
                1,
            )
        }

        // Algorithm 19.4: append a b = tabulate (select(a,b)) (|a|+|b|)
        // where select(a,b) i = if i < |a| then a[i] else b[i-|a|].
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a|+|b|), Span O(|a|+|b|)
        fn append(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> (appended: ArraySeqStPerS<T>)
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
                // Veracity: NEEDED proof block
                {
                    if i < a_len {
                        let r = a.seq[i].clone();
                        proof {
                            axiom_cloned_implies_eq_owned(a.seq@[i as int], r);
                        // Veracity: NEEDED proof block
                        }
                        r
                    } else {
                        let r = b.seq[i - a_len].clone();
                        proof {
                            axiom_cloned_implies_eq_owned(b.seq@[(i - a_len) as int], r);
                        }
                        r
                    }
                }),
                total,
            )
        }

        // Algorithm 19.5: filter f a = let b = map (deflate f) a in flatten b end.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn filter<F: Fn(&T) -> bool>(a: &ArraySeqStPerS<T>, pred: &F, Ghost(spec_pred): Ghost<spec_fn(T) -> bool>) -> (filtered: ArraySeqStPerS<T>)
            where T: Clone + Eq
        {
            // map (deflate pred) a — produces ArraySeqStPerS<ArraySeqStPerS<T>>
            let deflated = Self::map(
                    a,
                    &(|x: &T| -> (d: ArraySeqStPerS<T>)
                        requires
                            obeys_feq_clone::<T>(),
                            pred.requires((x,)),
                        ensures
                            d.seq@.len() <= 1,
                            d.seq@.len() == 1 ==> pred.ensures((x,), true) && d.seq@[0] == *x,
                            d.seq@.len() == 0 ==> pred.ensures((x,), false),
                    // Veracity: NEEDED proof block
                    {
                        Self::deflate(pred, x)
                    }),
                );
            let filtered = Self::flatten(&deflated);
            proof {
                let ghost ss = deflated.seq@.map_values(|inner: ArraySeqStPerS<T>| inner.seq@);
                lemma_flatten_bounded_by_outer_len::<T>(ss);

                // Every element in every inner seq satisfies pred.
                let ghost p = |x: T| pred.ensures((&x,), true);
                // Veracity: NEEDED assert
                assert forall|j: int, k: int| #![trigger ss[j][k]]
                    0 <= j < ss.len() && 0 <= k < ss[j].len()
                    implies p(ss[j][k])
                by {
                };
                lemma_flatten_all_satisfy::<T>(ss, p);

                // Connect flatten length to spec_filter_len via the 0-or-1 lemma.
                let ghost s_view = Seq::new(a.seq@.len(), |i: int| a.seq@[i]);
                // Veracity: NEEDED assert
                assert forall|i: int| #![trigger ss[i]] 0 <= i < a.seq@.len()
                    implies (ss[i].len() == 1 <==> spec_pred(a.seq@[i]))
                by {
                };
                lemma_flatten_01_eq_spec_filter_len(a.seq@, ss, spec_pred);

                // Connect flatten multiset to input multiset filtered by spec_pred.
                // Veracity: NEEDED assert
                assert forall|i: int| #![trigger ss[i]]
                    0 <= i < a.seq@.len() && ss[i].len() == 1
                    implies ss[i][0] == a.seq@[i]
                by {
                };
                lemma_flatten_01_multiset_eq_filter(a.seq@, ss, spec_pred);

                // Veracity: NEEDED assert
                assert(filtered.seq@ =~= Seq::new(filtered.spec_len(), |i: int| filtered.spec_index(i)));
                // Veracity: NEEDED assert
                assert(a.seq@ =~= Seq::new(a.seq@.len(), |i: int| a.seq@[i]));
            }
            filtered
        }

        // Algorithm 19.6: update a (i, x) = tabulate (lambda j. if i=j then x else a[j]) |a|.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn update(a: &ArraySeqStPerS<T>, index: usize, item: T) -> (updated: ArraySeqStPerS<T>)
            where T: Clone + Eq
        {
            let len = a.seq.len();
            Self::tabulate(
                &(|j: usize| -> (r: T)
                    requires
                        obeys_feq_clone::<T>(),
                        j < a.seq@.len(),
                        index < a.seq@.len(),
                    // Veracity: NEEDED proof block
                    ensures
                        j == index ==> r == item,
                        j != index ==> r == a.seq@[j as int],
                {
                    if j == index {
                        // Veracity: NEEDED proof block
                        let r = item.clone();
                        proof {
                            axiom_cloned_implies_eq_owned(item, r);
                        }
                        r
                    } else {
                        let r = a.seq[j].clone();
                        proof {
                            axiom_cloned_implies_eq_owned(a.seq@[j as int], r);
                        }
                        r
                    }
                }),
                len,
            )
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n+m), Span O(n+m)
        fn inject(a: &ArraySeqStPerS<T>, updates: &Vec<(usize, T)>) -> (injected: ArraySeqStPerS<T>)
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
                    // Veracity: NEEDED proof block
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

            let mut i: usize = ulen;
            while i > 0
                invariant
                    0 <= i <= ulen,
                    ulen == u.len(),
                    len == a.seq@.len(),
                    result_vec@.len() == s.len(),
                    s.len() == len,
                    obeys_feq_clone::<T>(),
                    // Veracity: NEEDED proof block
                    s == a.seq@,
                    u == updates@,
                    result_vec@ =~= spec_inject(s, u.subrange(i as int, ulen as int)),
                decreases i,
            // Veracity: NEEDED proof block
            {
                i -= 1;
                let pos = updates[i].0;
                if pos < len {
                    let val = updates[i].1.clone();
                    proof {
                        axiom_cloned_implies_eq_owned(u[i as int].1, val);
                    // Veracity: NEEDED proof block
                    }
                    result_vec.set(pos, val);
                // Veracity: NEEDED proof block
                }
                proof {
                    let ghost sub = u.subrange(i as int, ulen as int);
                    // Veracity: NEEDED assert
                    assert(sub.drop_first() =~= u.subrange(i as int + 1, ulen as int));
                    reveal(spec_inject);
                }
            }

            proof {
            }
            let injected = ArraySeqStPerS { seq: result_vec };
            proof {
                // Veracity: NEEDED assert
                assert(Seq::new(a.spec_len(), |i: int| a.spec_index(i)) =~= a.seq@) by {
                    // Veracity: NEEDED assert
                    assert forall|i: int| 0 <= i < a.spec_len() implies
                        Seq::new(a.spec_len(), |j: int| a.spec_index(j))[i] == a.seq@[i]
                    by {
                        a.lemma_spec_index(i);
                    };
                }
            }
            injected
        }

        // Algorithm 19.7: isEmpty a = (|a| = 0).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_empty(&self) -> (empty: bool) {
            self.seq.len() == 0
        }

        // Algorithm 19.7: isSingleton a = (|a| = 1).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_singleton(&self) -> (single: bool) {
            self.seq.len() == 1
        }

        // Algorithm 19.8: iterate f x a (iterative form for single-threaded).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        // Veracity: NEEDED proof block
        fn iterate_iter<A, F: Fn(&A, &T) -> A>(a: &ArraySeqStPerS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(A, T) -> A>, seed: A) -> (accumulated: A) {
            let len = a.seq.len();
            // Veracity: NEEDED proof block
            let mut acc = seed;
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len == a.seq@.len(),
                    forall|x: &A, y: &T| #[trigger] f.requires((x, y)),
                    // Veracity: NEEDED proof block
                    forall|a: A, t: T, ret: A| f.ensures((&a, &t), ret) <==> ret == spec_f(a, t),
                    acc == a.seq@.take(i as int).fold_left(seed, spec_f),
                decreases len - i,
            {
                proof {
                }
                acc = f(&acc, &a.seq[i]);
                proof {
                    let ghost t = a.seq@.take(i as int + 1);
                    // Veracity: NEEDED assert
                    assert(t.drop_last() =~= a.seq@.take(i as int));
                    reveal(Seq::fold_left);
                }
                i += 1;
            }
            proof {
            }
            acc
        }

        // Algorithm 19.8: iterate f x a = if |a|=0 then x else iterate f (f(x,a[0])) a[1..|a|-1].
        // Delegates to iterate_iter which has the full proof.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqStPerS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(A, T) -> A>, seed: A) -> (accumulated: A)
            where T: Clone + Eq
        {
            Self::iterate_iter(a, f, Ghost(spec_f), seed)
        }

        // Algorithm 19.9: reduce f id a (iterative form for single-threaded).
        // Veracity: NEEDED proof block
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn reduce_iter<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(T, T) -> T>, id: T) -> (reduced: T)
            // Veracity: NEEDED proof block
            where T: Clone
        {
            let len = a.seq.len();
            let mut acc = id;
            let mut i: usize = 0;
            while i < len
                invariant
                    // Veracity: NEEDED proof block
                    i <= len,
                    len == a.seq@.len(),
                    forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                    forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) <==> ret == spec_f(x, y),
                    acc == a.seq@.take(i as int).fold_left(id, spec_f),
                decreases len - i,
            {
                proof {
                }
                acc = f(&acc, &a.seq[i]);
                proof {
                    let ghost t = a.seq@.take(i as int + 1);
                    // Veracity: NEEDED assert
                    assert(t.drop_last() =~= a.seq@.take(i as int));
                    reveal(Seq::fold_left);
                }
                i += 1;
            }
            proof {
            }
            acc
        }

        // Algorithm 19.9: reduce f id a = if |a|=0 then id; |a|=1 then a[0]; else f(reduce f id b, reduce f id c).
        // Delegates to reduce_iter which has the full proof.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(T, T) -> T>, id: T) -> (reduced: T)
            where T: Clone + Eq
        {
            Self::reduce_iter(a, f, Ghost(spec_f), id)
        }

        // Algorithm 19.10: scan f id a (iterative form for single-threaded).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(T, T) -> T>, id: T) -> (scanned: (ArraySeqStPerS<T>, T))
            // Veracity: NEEDED proof block
            where T: Clone
        {
            let len = a.seq.len();
            // Veracity: NEEDED proof block
            let mut acc = id;
            let mut seq: Vec<T> = Vec::with_capacity(len);
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len == a.seq@.len(),
                    // Veracity: NEEDED proof block
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
                }
                seq.push(f(&acc, &a.seq[i]));
                acc = f(&acc, &a.seq[i]);
                proof {
                    let ghost t = a.seq@.take(i as int + 1);
                    // Veracity: NEEDED assert
                    assert(t.drop_last() =~= a.seq@.take(i as int));
                    reveal(Seq::fold_left);
                }
                i += 1;
            }
            proof {
            }
            (ArraySeqStPerS { seq }, acc)
        }

        // Algorithm 19.3: map f a = tabulate (lambda i.f(a[i])) |a|.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn map<U: Clone, F: Fn(&T) -> U>(a: &ArraySeqStPerS<T>, f: &F) -> (mapped: ArraySeqStPerS<U>)
        {
            let n = a.seq.len();
            <ArraySeqStPerS<U> as ArraySeqStPerTrait<U>>::tabulate(
                &(|i: usize| -> (r: U)
                    requires
                        (i as int) < a.seq@.len(),
                        f.requires((&a.seq@[i as int],)),
                    ensures
                        f.ensures((&a.seq@[i as int],), r),
                {
                    f(&a.seq[i])
                }),
                n,
            )
        }

        // Primitive: tabulate. Same as Chapter 18.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn tabulate<F: Fn(usize) -> T>(f: &F, length: usize) -> (tab_seq: ArraySeqStPerS<T>)
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
            ArraySeqStPerS { seq }
        }

        // Primitive: flatten. Same as Chapter 18.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(total elements), Span O(total elements)
        fn flatten(a: &ArraySeqStPerS<ArraySeqStPerS<T>>) -> (flattened: ArraySeqStPerS<T>)
            where T: Clone + Eq
        {
            let outer_len = a.seq.len();
            let mut seq: Vec<T> = Vec::new();
            let mut i: usize = 0;
            while i < outer_len
                // Veracity: NEEDED proof block
                invariant
                    i <= outer_len,
                    outer_len == a.seq@.len(),
                    obeys_feq_clone::<T>(),
                    seq@ =~= a.seq@.take(i as int).map_values(|inner: ArraySeqStPerS<T>| inner.seq@).flatten(),
                // Veracity: NEEDED proof block
                decreases outer_len - i,
            {
                let inner = &a.seq[i];
                let inner_len = inner.seq.len();
                let mut j: usize = 0;
                while j < inner_len
                    invariant
                        j <= inner_len,
                        // Veracity: NEEDED proof block
                        inner_len == inner.seq@.len(),
                        i < outer_len,
                        outer_len == a.seq@.len(),
                        obeys_feq_clone::<T>(),
                        seq@ =~= a.seq@.take(i as int).map_values(|inner: ArraySeqStPerS<T>| inner.seq@).flatten()
                            + inner.seq@.take(j as int),
                    decreases inner_len - j,
                {
                    seq.push(inner.seq[j].clone());
                    proof {
                        let ghost last = seq@[seq@.len() - 1 as int];
                        // Veracity: NEEDED proof block
                        axiom_cloned_implies_eq_owned(inner.seq[j as int], last);
                    }
                    j += 1;
                }
                proof {
                    let ghost prefix = a.seq@.take(i as int).map_values(|inner: ArraySeqStPerS<T>| inner.seq@);
                    // Veracity: NEEDED assert
                    assert(a.seq@.take(i as int + 1).map_values(|inner: ArraySeqStPerS<T>| inner.seq@)
                        =~= prefix.push(a.seq@[i as int].seq@));
                    prefix.lemma_flatten_push(a.seq@[i as int].seq@);
                }
                i += 1;
            }
            proof {
            }
            ArraySeqStPerS { seq }
        }

        // Algorithm 19.5 (deflate): deflate f x = if (f x) then ⟨x⟩ else ⟨⟩.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn deflate<F: Fn(&T) -> bool>(pred: &F, x: &T) -> (deflated: ArraySeqStPerS<T>)
            where T: Clone + Eq
        {
            if pred(x) {
                let elem = x.clone();
                proof {
                    axiom_cloned_implies_eq_owned(*x, elem);
                }
                Self::singleton(elem)
            } else {
                Self::empty()
            }
        }
    }

    impl<T> ArraySeqStPerS<T> {
        broadcast proof fn lemma_spec_index(&self, i: int)
            requires 0 <= i < self.spec_len()
            ensures #[trigger] self.seq@[i] == self.spec_index(i)
        {}
    }

    impl<T: View> ArraySeqStPerS<T> {
        /// Connects view indexing with spec_index: self@[i] == self.spec_index(i)@.
        pub proof fn lemma_view_index(&self, i: int)
            requires 0 <= i < self.spec_len()
            ensures self@[i] == self.spec_index(i)@
        {}

        /// Returns an iterator over the sequence elements.
        pub fn iter(&self) -> (it: ArraySeqStPerIter<'_, T>)
            ensures
                it@.0 == 0,
                it@.1 == self.seq@,
                iter_invariant(&it),
        {
            ArraySeqStPerIter { inner: self.seq.iter() }
        }
    }

    //		Section 10. iterators


    #[verifier::reject_recursive_types(T)]
    pub struct ArraySeqStPerIter<'a, T> {
        pub inner: std::slice::Iter<'a, T>,
    }

    impl<'a, T> View for ArraySeqStPerIter<'a, T> {
        type V = (int, Seq<T>);
        open spec fn view(&self) -> (int, Seq<T>) { self.inner@ }
    }

    pub open spec fn iter_invariant<'a, T>(it: &ArraySeqStPerIter<'a, T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<'a, T> std::iter::Iterator for ArraySeqStPerIter<'a, T> {
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

    /// Ghost iterator for ForLoopGhostIterator support (for-iter, for-borrow patterns).
    #[verifier::reject_recursive_types(T)]
    pub struct ArraySeqStPerGhostIterator<'a, T> {
        pub pos: int,
        pub elements: Seq<T>,
        pub phantom: core::marker::PhantomData<&'a T>,
    }

    impl<'a, T> View for ArraySeqStPerGhostIterator<'a, T> {
        type V = Seq<T>;

        open spec fn view(&self) -> Seq<T> {
            self.elements.take(self.pos)
        }
    }

    impl<'a, T> vstd::pervasive::ForLoopGhostIteratorNew for ArraySeqStPerIter<'a, T> {
        type GhostIter = ArraySeqStPerGhostIterator<'a, T>;
        open spec fn ghost_iter(&self) -> ArraySeqStPerGhostIterator<'a, T> {
            ArraySeqStPerGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    impl<'a, T> vstd::pervasive::ForLoopGhostIterator for ArraySeqStPerGhostIterator<'a, T> {
        type ExecIter = ArraySeqStPerIter<'a, T>;
        type Item = T;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &ArraySeqStPerIter<'a, T>) -> bool {
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

        open spec fn ghost_advance(&self, _exec_iter: &ArraySeqStPerIter<'a, T>) -> ArraySeqStPerGhostIterator<'a, T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, T> std::iter::IntoIterator for &'a ArraySeqStPerS<T> {
        type Item = &'a T;
        type IntoIter = ArraySeqStPerIter<'a, T>;
        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                it@.0 == 0,
                it@.1 == self.seq@,
                iter_invariant(&it),
        {
            ArraySeqStPerIter { inner: self.seq.iter() }
        }
    }

    impl<T> std::iter::IntoIterator for ArraySeqStPerS<T> {
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

    //		Section 12. derive impls in verus!

// Veracity: UNNEEDED proof block 
    #[cfg(verus_keep_ghost)]
    impl<T: View + PartialEq> PartialEqSpecImpl for ArraySeqStPerS<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }


    impl<T: Clone> Clone for ArraySeqStPerS<T> {
        fn clone(&self) -> (res: Self)
            ensures
                res.seq@.len() == self.seq@.len(),
                forall|i: int| #![trigger res.seq@[i]]
                    0 <= i < self.seq@.len() ==> cloned::<T>(self.seq@[i], res.seq@[i]),
        {
            ArraySeqStPerS { seq: self.seq.clone() }
        }
    }

    impl<T: Eq + View> Eq for ArraySeqStPerS<T> {}

    impl<T: PartialEq + View> PartialEq for ArraySeqStPerS<T> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            let equal = self.seq == other.seq;
            proof { accept(equal == (self@ == other@)); }
            equal
        }
    }

    } // verus!

    //		Section 13. macros


    /// Literal constructor macro for ArraySeqStPerS.
    #[macro_export]
    macro_rules! ArraySeqStPerSLit {
        () => { $crate::Chap19::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS::from_vec(Vec::new()) };
        ($x:expr; $n:expr) => { $crate::Chap19::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS::from_vec(vec![$x; $n]) };
        ($($x:expr),* $(,)?) => { $crate::Chap19::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS::from_vec(vec![$($x),*]) };
    }

    //		Section 14. derive impls outside verus!

    impl<T: Debug> Debug for ArraySeqStPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_list().entries(self.seq.iter()).finish()
        }
    }

    impl<T: Display> Display for ArraySeqStPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "[")?;
            for (i, item) in self.seq.iter().enumerate() {
                if i > 0 { write!(f, ", ")?; }
                write!(f, "{item}")?;
            }
            write!(f, "]")
        }
    }

    impl<'a, T: Debug> Debug for ArraySeqStPerIter<'a, T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "ArraySeqStPerIter({:?})", self.inner)
        }
    }

    impl<'a, T> Display for ArraySeqStPerIter<'a, T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "ArraySeqStPerIter")
        }
    }

    impl<'a, T> Debug for ArraySeqStPerGhostIterator<'a, T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "ArraySeqStPerGhostIterator")
        }
    }

    impl<'a, T> Display for ArraySeqStPerGhostIterator<'a, T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "ArraySeqStPerGhostIterator")
        }
    }
}
