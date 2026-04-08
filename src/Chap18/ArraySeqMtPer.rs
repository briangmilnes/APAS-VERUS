//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO

//! Chapter 18 algorithms for ArraySeqMtPer multithreaded persistent. Verusified.
//! Uses global work-stealing pool for parallel operations (map_par, reduce_par).

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 5. view impls
//	Section 6. spec fns
//	Section 8. traits
//	Section 9. impls
//	Section 10. iterators
//	Section 12. derive impls in verus!
//	Section 13. macros
//	Section 14. derive impls outside verus!

//		Section 1. module


pub mod ArraySeqMtPer {


    //		Section 2. imports

    use std::fmt::{Debug, Display, Formatter};
    use std::fmt::Result as FmtResult;
    use std::slice::Iter;
    use std::vec::IntoIter;

    use vstd::prelude::*;
    use crate::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::*;
    use crate::vstdplus::clone_plus::clone_plus::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

    verus! 
{


    #[cfg(verus_keep_ghost)]
    use {
        vstd::multiset::Multiset,
        vstd::std_specs::vec::*,
        vstd::std_specs::clone::*,
    };
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;
    use crate::vstdplus::monoid::monoid::*;
    use crate::vstdplus::multiset::multiset::*;

    //		Section 3. broadcast use


    #[cfg(verus_keep_ghost)]


    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
        vstd::seq_lib::group_seq_properties,
        crate::vstdplus::feq::feq::group_feq_axioms,
        vstd::seq_lib::group_to_multiset_ensures,
    };

    //		Section 4. type definitions


    #[verifier::reject_recursive_types(T)]
    pub struct ArraySeqMtPerS<T> {
        pub seq: Vec<T>,
    }

    //		Section 5. view impls


    impl<T: View> View for ArraySeqMtPerS<T> {
        type V = Seq<T::V>;

        open spec fn view(&self) -> Seq<T::V> {
            self.seq@.map(|_i: int, t: T| t@)
        }
    }

    //		Section 6. spec fns


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

    //		Section 8. traits


    /// - Base trait for multi-threaded persistent array sequences (Chapter 18).
    /// - These methods are never redefined in later chapters.
    pub trait ArraySeqMtPerBaseTrait<T>: Sized {
        spec fn spec_arrayseqmtper_wf(&self) -> bool;

        spec fn spec_len(&self) -> nat;

        spec fn spec_index(&self, i: int) -> T
            recommends i < self.spec_len();

        /// - Create a new sequence of length `length` with each element initialized to `init_value`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(length), Span O(log length).
        fn new(length: usize, init_value: T) -> (new_seq: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                length <= usize::MAX,
            ensures
                new_seq.spec_arrayseqmtper_wf(),
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
        /// - Alg Analysis: APAS: N/A — implementation utility, not in prose.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(length), Span O(log length).
        fn subseq_copy(&self, start: usize, length: usize) -> (subseq: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                start + length <= usize::MAX,
                start + length <= self.spec_len(),
            ensures
                subseq.spec_arrayseqmtper_wf(),
                subseq.spec_len() == length as int,
                forall|i: int| #![trigger subseq.spec_index(i)] 0 <= i < length ==> subseq.spec_index(i) == self.spec_index(start as int + i);

        /// - Definition 18.12 (subseq). Extract a contiguous subsequence.
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(j), Span O(j) — ACCEPTED DIFFERENCE: Vec-backed; Vec-backed, sequential clone loop; O(1) requires tree representation
        fn subseq(a: &Self, start: usize, length: usize) -> (subseq: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                start + length <= usize::MAX,
                start + length <= a.spec_len(),
            ensures
                subseq.spec_arrayseqmtper_wf(),
                subseq.spec_len() == length as int,
                forall|i: int| #![trigger subseq.spec_index(i)] 0 <= i < length ==> subseq.spec_index(i) == a.spec_index(start as int + i);

        /// - Create sequence from Vec.
        /// - Alg Analysis: APAS: N/A — implementation utility, not in prose.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n) worst case, O(1) best case, Span O(1).
        fn from_vec(elts: Vec<T>) -> (seq: Self)
            ensures
                seq.spec_arrayseqmtper_wf(),
                seq.spec_len() == elts@.len(),
                forall|i: int| #![trigger seq.spec_index(i)] 0 <= i < elts@.len() ==> seq.spec_index(i) == elts@[i];
    }

    /// Redefinable trait - may be overridden with better algorithms in later chapters.
    pub trait ArraySeqMtPerRedefinableTrait<T>: ArraySeqMtPerBaseTrait<T> {

        /// - Definition 18.1 (empty). Construct the empty sequence.
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (empty_seq: Self)
            ensures empty_seq.spec_arrayseqmtper_wf(), empty_seq.spec_len() == 0;

        /// - Definition 18.1 (singleton). Construct a singleton sequence containing `item`.
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(item: T) -> (singleton: Self)
            ensures
                singleton.spec_arrayseqmtper_wf(),
                singleton.spec_len() == 1,
                singleton.spec_index(0) == item;

        /// - Definition 18.13 (append). Concatenate two sequences.
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(|a| + |b|), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a| + |b|), Span O(|a| + |b|) — ACCEPTED DIFFERENCE: Vec-backed; Vec-backed, sequential clone loops; O(1) requires tree representation
        fn append(a: &ArraySeqMtPerS<T>, b: &ArraySeqMtPerS<T>) -> (appended: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                a.seq@.len() + b.seq@.len() <= usize::MAX as int,
            ensures
                appended.spec_arrayseqmtper_wf(),
                appended.spec_len() == a.seq@.len() + b.seq@.len(),
                forall|i: int| #![trigger appended.spec_index(i)] 0 <= i < a.seq@.len() ==> appended.spec_index(i) == a.seq@[i],
                forall|i: int| #![trigger b.seq@[i]] 0 <= i < b.seq@.len() ==> appended.spec_index(a.seq@.len() as int + i) == b.seq@[i];

        /// - Definition 18.14 (filter). Keep elements satisfying `pred`.
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(1 + Sigma W(f(x))), Span O(lg |a| + max S(f(x)))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(lg n) — ACCEPTED DIFFERENCE: Vec-backed; parallel D&C with multiset distribution lemma
        /// - The multiset postcondition captures predicate satisfaction, provenance,
        ///   and completeness in a single statement.
        fn filter<F: Fn(&T) -> bool + Clone + Send + Sync + 'static>(a: &ArraySeqMtPerS<T>, pred: &F, Ghost(spec_pred): Ghost<spec_fn(T) -> bool>) -> (filtered: Self)
            where T: Clone + Eq + Send + Sync + 'static
            requires
                obeys_feq_clone::<T>(),
                forall|i: int| 0 <= i < a.seq@.len() ==> #[trigger] pred.requires((&a.seq@[i],)),
                // The biconditional bridge ties the exec closure to the spec predicate.
                forall|v: T, keep: bool| pred.ensures((&v,), keep) ==> spec_pred(v) == keep,
            ensures
                filtered.spec_arrayseqmtper_wf(),
                filtered.spec_len() <= a.seq@.len(),
                filtered.spec_len() == spec_filter_len(
                    Seq::new(a.seq@.len(), |i: int| a.seq@[i]), spec_pred),
                // The result multiset equals the input multiset filtered by the spec predicate.
                Seq::new(filtered.spec_len(), |i: int| filtered.spec_index(i)).to_multiset()
                    =~= Seq::new(a.seq@.len(), |i: int| a.seq@[i]).to_multiset().filter(spec_pred),
                forall|i: int| #![trigger filtered.spec_index(i)] 0 <= i < filtered.spec_len() ==> pred.ensures((&filtered.spec_index(i),), true);

        /// - Definition 18.16 (update). Return a copy with the index replaced by the new value.
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(|a|), Span O(1)
        /// - Alg Analysis: APAS (Ch22 CS 22.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — ACCEPTED DIFFERENCE: Vec-backed; Vec-backed, sequential clone loop; O(1) requires tree representation
        fn update(a: &ArraySeqMtPerS<T>, index: usize, item: T) -> (updated: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                index < a.seq@.len(),
            ensures
                updated.spec_arrayseqmtper_wf(),
                updated.spec_len() == a.seq@.len(),
                updated.spec_index(index as int) == item,
                forall|i: int| #![trigger updated.spec_index(i)] 0 <= i < a.seq@.len() && i != index as int ==> updated.spec_index(i) == a.seq@[i];

        /// - Definition 18.16 (inject). Update multiple positions at once; the first update in
        ///   the ordering of `updates` takes effect when positions collide.
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(|a| + |b|), Span O(lg(degree(b)))
        /// - Alg Analysis: APAS (Ch22 CS 22.2): Work O(|b|), Span O(lg(degree(b)))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + m), Span O(n + m) — ACCEPTED DIFFERENCE: Vec-backed; sequential apply; parallel inject requires sort-by-position
        fn inject(a: &Self, updates: &Vec<(usize, T)>) -> (injected: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
            ensures
                injected.spec_arrayseqmtper_wf(),
                injected.spec_len() == a.spec_len(),
                Seq::new(injected.spec_len(), |i: int| injected.spec_index(i))
                    =~= spec_inject(
                        Seq::new(a.spec_len(), |i: int| a.spec_index(i)),
                        updates@);

        /// - Definition 18.5 (isEmpty). true iff the sequence has length zero.
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_empty(&self) -> (empty: bool)
            ensures empty <==> self.spec_len() == 0;

        /// - Definition 18.5 (isSingleton). true iff the sequence has length one.
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_singleton(&self) -> (single: bool)
            ensures single <==> self.spec_len() == 1;

        /// - Definition 18.7 (iterate). Fold with accumulator `seed`.
        /// - Alg Analysis: APAS (Ch20 CS 20.3): Work O(1 + Sigma W(f)), Span O(1 + Sigma S(f))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) (iterate is sequential)
        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqMtPerS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(A, T) -> A>, seed: A) -> (accumulated: A)
            requires
                forall|x: &A, y: &T| #[trigger] f.requires((x, y)),
                forall|a: A, t: T, ret: A| f.ensures((&a, &t), ret) ==> ret == spec_f(a, t),
            ensures
                accumulated == spec_iterate(Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_f, seed);

        /// - Definition 18.18 (reduce). Combine elements using associative `f` and identity `id`.
        /// - Alg Analysis: APAS (Ch20 CS 20.4): Work O(1 + Sigma W(f)), Span O(lg |a| * max S(f))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — ACCEPTED DIFFERENCE: Vec-backed; sequential fold; parallel reduce_inner available via bare impl
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqMtPerS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(T, T) -> T>, id: T) -> (reduced: T)
            where T: Clone
            requires
                spec_monoid(spec_f, id),
                forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) ==> ret == spec_f(x, y),
            ensures
                reduced == spec_iterate(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_f, id);

        /// - Definition 18.19 (scan). Prefix-reduce returning inclusive prefix sums and total.
        /// - Alg Analysis: APAS (Ch20 CS 20.5): Work O(|a|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — ACCEPTED DIFFERENCE: Vec-backed; sequential loop; parallel scan requires upsweep/downsweep
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqMtPerS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(T, T) -> T>, id: T) -> (scanned: (ArraySeqMtPerS<T>, T))
            where T: Clone + Eq
            requires
                spec_monoid(spec_f, id),
                obeys_feq_clone::<T>(),
                forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) ==> ret == spec_f(x, y),
            ensures
                scanned.0.spec_len() == a.spec_len(),
                forall|i: int| #![trigger scanned.0.spec_index(i)] 0 <= i < a.spec_len() ==>
                    scanned.0.spec_index(i) == Seq::new(a.spec_len(), |j: int| a.spec_index(j)).take(i + 1).fold_left(id, spec_f),
                scanned.1 == spec_iterate(
                    Seq::new(a.spec_len(), |j: int| a.spec_index(j)), spec_f, id);

        /// - Algorithm 18.4 (map). Transform each element via `f`.
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(1 + Sigma W(f(x))), Span O(1 + max S(f(x)))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — ACCEPTED DIFFERENCE: Vec-backed; sequential loop; parallel map_inner available via bare impl
        fn map<U: Clone, F: Fn(&T) -> U>(a: &ArraySeqMtPerS<T>, f: &F) -> (mapped: ArraySeqMtPerS<U>)
            requires
                forall|i: int| 0 <= i < a.seq@.len() ==> #[trigger] f.requires((&a.seq@[i],)),
            ensures
                mapped.seq@.len() == a.seq@.len(),
                forall|i: int| #![trigger mapped.seq@[i]] 0 <= i < a.seq@.len() ==> f.ensures((&a.seq@[i],), mapped.seq@[i]);

        /// - Algorithm 18.3 (tabulate). Build a sequence by applying `f` to each index.
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(1 + Sigma W(f(i))), Span O(1 + max S(f(i)))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — ACCEPTED DIFFERENCE: Vec-backed; sequential loop; parallel tabulate_inner available via bare impl
        fn tabulate<F: Fn(usize) -> T>(f: &F, length: usize) -> (tab_seq: ArraySeqMtPerS<T>)
            requires
                length <= usize::MAX,
                forall|i: usize| i < length ==> #[trigger] f.requires((i,)),
            ensures
                tab_seq.seq@.len() == length,
                forall|i: int| #![trigger tab_seq.seq@[i]] 0 <= i < length ==> f.ensures((i as usize,), tab_seq.seq@[i]);

        /// - Definition 18.15 (flatten). Concatenate a sequence of sequences.
        /// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(|a| + sum |a[i]|), Span O(lg |a|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(Σ|a_i|), Span O(Σ|a_i|) — ACCEPTED DIFFERENCE: Vec-backed; sequential nested loops; D&C flatten requires outer array cloning proof
        fn flatten(a: &ArraySeqMtPerS<ArraySeqMtPerS<T>>) -> (flattened: ArraySeqMtPerS<T>)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
            ensures
                flattened.seq@ =~= a.seq@.map_values(|inner: ArraySeqMtPerS<T>| inner.seq@).flatten();
    }

    //		Section 9. impls


    impl<T> ArraySeqMtPerBaseTrait<T> for ArraySeqMtPerS<T> {
        open spec fn spec_arrayseqmtper_wf(&self) -> bool { true } // accept hole: Vec-backed, true is correct

        open spec fn spec_len(&self) -> nat {
            self.seq@.len()
        }

        open spec fn spec_index(&self, i: int) -> T {
            self.seq[i]
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn new(length: usize, init_value: T) -> (new_seq: ArraySeqMtPerS<T>)
            where T: Clone + Eq
        {
            let seq = std::vec::from_elem(init_value, length);
            ArraySeqMtPerS { seq }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn length(&self) -> (len: usize) {
            self.seq.len()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn nth(&self, index: usize) -> (nth_elem: &T) {
            &self.seq[index]
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(j - i), Span O(j - i)
        fn subseq_copy(&self, start: usize, length: usize) -> (subseq: ArraySeqMtPerS<T>)
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
            ArraySeqMtPerS { seq }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(j - i), Span O(j - i)
        fn subseq(a: &ArraySeqMtPerS<T>, start: usize, length: usize) -> (subseq: ArraySeqMtPerS<T>)
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
                // Veracity: NEEDED proof block
                proof {
                    let ghost last = seq@[seq@.len() - 1 as int];
                    axiom_cloned_implies_eq_owned(a.seq[i as int], last);
                }
                i += 1;
            }
            ArraySeqMtPerS { seq }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn from_vec(elts: Vec<T>) -> (seq: ArraySeqMtPerS<T>) {
            ArraySeqMtPerS { seq: elts }
        }
    }


    impl<T> ArraySeqMtPerRedefinableTrait<T> for ArraySeqMtPerS<T> {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (empty_seq: ArraySeqMtPerS<T>) {
            ArraySeqMtPerS { seq: Vec::new() }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(item: T) -> (singleton: ArraySeqMtPerS<T>) {
            let mut seq = Vec::with_capacity(1);
            seq.push(item);
            ArraySeqMtPerS { seq }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a| + |b|), Span O(|a| + |b|)
        fn append(a: &ArraySeqMtPerS<T>, b: &ArraySeqMtPerS<T>) -> (appended: ArraySeqMtPerS<T>)
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
                // Veracity: NEEDED proof block
                proof {
                    let ghost last = seq@[seq@.len() - 1 as int];
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
                // Veracity: NEEDED proof block
                proof {
                    let ghost last = seq@[seq@.len() - 1 as int];
                    axiom_cloned_implies_eq_owned(b.seq[j as int], last);
                }
                j += 1;
            }
            ArraySeqMtPerS { seq }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn filter<F: Fn(&T) -> bool + Clone + Send + Sync + 'static>(a: &ArraySeqMtPerS<T>, pred: &F, Ghost(spec_pred): Ghost<spec_fn(T) -> bool>) -> (filtered: ArraySeqMtPerS<T>)
            where T: Clone + Eq + Send + Sync + 'static
        {
            let filtered = Self::filter_dc(a, pred, Ghost(spec_pred));
            // Veracity: NEEDED proof block
            proof {
                // Bridge filter_dc ensures (a.seq@) to trait ensures (Seq::new(...)).
                let ghost s = Seq::new(a.seq@.len(), |i: int| a.seq@[i]);
                // Veracity: NEEDED assert
                assert(s =~= a.seq@);
                let ghost fs = Seq::new(filtered.spec_len(), |i: int| filtered.spec_index(i));
                // Veracity: NEEDED assert
                assert(fs =~= filtered.seq@) by {
                    // Veracity: NEEDED assert
                    assert forall|i: int| 0 <= i < fs.len() implies #[trigger] fs[i] == filtered.seq@[i]
                    by { filtered.lemma_spec_index(i); }
                }
            }
            filtered
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn update(a: &ArraySeqMtPerS<T>, index: usize, item: T) -> (updated: ArraySeqMtPerS<T>)
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
                    // Veracity: NEEDED proof block
                    proof {
                        let ghost last = seq@[seq@.len() - 1 as int];
                        axiom_cloned_implies_eq_owned(item, last);
                    }
                } else {
                    seq.push(a.seq[i].clone());
                    // Veracity: NEEDED proof block
                    proof {
                        let ghost last = seq@[seq@.len() - 1 as int];
                        axiom_cloned_implies_eq_owned(a.seq[i as int], last);
                    }
                }
                i += 1;
            }
            ArraySeqMtPerS { seq }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn inject(a: &ArraySeqMtPerS<T>, updates: &Vec<(usize, T)>) -> (injected: ArraySeqMtPerS<T>)
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
                // Veracity: NEEDED proof block
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
                    s == a.seq@,
                    u == updates@,
                    result_vec@ =~= spec_inject(s, u.subrange(i as int, ulen as int)),
                decreases i,
            {
                i -= 1;
                let pos = updates[i].0;
                if pos < len {
                    let val = updates[i].1.clone();
                    // Veracity: NEEDED proof block
                    proof {
                        axiom_cloned_implies_eq_owned(u[i as int].1, val);
                    }
                    result_vec.set(pos, val);
                }
                // Veracity: NEEDED proof block
                proof {
                    let ghost sub = u.subrange(i as int, ulen as int);
                    // Veracity: NEEDED assert
                    assert(sub.drop_first() =~= u.subrange(i as int + 1, ulen as int));
                    reveal(spec_inject);
                }
            }

            // Veracity: NEEDED proof block
            proof {
            }
            let injected = ArraySeqMtPerS { seq: result_vec };
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                assert forall|j: int| 0 <= j < a.spec_len() implies #[trigger] a.spec_index(j) == s[j]
                by { a.lemma_spec_index(j); }
                // Veracity: NEEDED assert
                assert(Seq::new(a.spec_len(), |i: int| a.spec_index(i)) =~= s);
            }
            injected
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_empty(&self) -> (empty: bool) {
            self.seq.len() == 0
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_singleton(&self) -> (single: bool) {
            self.seq.len() == 1
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqMtPerS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(A, T) -> A>, seed: A) -> (accumulated: A) {
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
                // Veracity: NEEDED proof block
                proof {
                    a.lemma_spec_index(i as int);
                }
                acc = f(&acc, &a.seq[i]);
                // Veracity: NEEDED proof block
                proof {
                    let ghost t = s.take(i as int + 1);
                    // Veracity: NEEDED assert
                    assert(t.drop_last() =~= s.take(i as int));
                    reveal(Seq::fold_left);
                }
                i += 1;
            }
            // Veracity: NEEDED proof block
            proof {
            }
            acc
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqMtPerS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(T, T) -> T>, id: T) -> (reduced: T)
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
                // Veracity: NEEDED proof block
                proof {
                    a.lemma_spec_index(i as int);
                }
                acc = f(&acc, &a.seq[i]);
                // Veracity: NEEDED proof block
                proof {
                    let ghost t = s.take(i as int + 1);
                    // Veracity: NEEDED assert
                    assert(t.drop_last() =~= s.take(i as int));
                    reveal(Seq::fold_left);
                }
                i += 1;
            }
            // Veracity: NEEDED proof block
            proof {
            }
            acc
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqMtPerS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(T, T) -> T>, id: T) -> (scanned: (ArraySeqMtPerS<T>, T))
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
                // Veracity: NEEDED proof block
                proof {
                    a.lemma_spec_index(i as int);
                }
                acc = f(&acc, &a.seq[i]);
                // Veracity: NEEDED proof block
                proof {
                    let ghost t = s.take(i as int + 1);
                    // Veracity: NEEDED assert
                    assert(t.drop_last() =~= s.take(i as int));
                    reveal(Seq::fold_left);
                }
                let cloned = acc.clone();
                // Veracity: NEEDED proof block
                proof {
                    axiom_cloned_implies_eq_owned(acc, cloned);
                }
                seq.push(cloned);
                i += 1;
            }
            // Veracity: NEEDED proof block
            proof {
            }
            let scanned_seq = ArraySeqMtPerS { seq };
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                assert forall|i: int| #![trigger scanned_seq.spec_index(i)] 0 <= i < a.spec_len() implies
                    scanned_seq.spec_index(i) == s.take(i + 1).fold_left(id, spec_f)
                by {
                }
            }
            (scanned_seq, acc)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn map<U: Clone, F: Fn(&T) -> U>(a: &ArraySeqMtPerS<T>, f: &F) -> (mapped: ArraySeqMtPerS<U>)
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
            ArraySeqMtPerS { seq }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn tabulate<F: Fn(usize) -> T>(f: &F, length: usize) -> (tab_seq: ArraySeqMtPerS<T>)
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
            ArraySeqMtPerS { seq }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(total length), Span O(total length)
        fn flatten(a: &ArraySeqMtPerS<ArraySeqMtPerS<T>>) -> (flattened: ArraySeqMtPerS<T>)
            where T: Clone + Eq
        {
            // Sequential: D&C flatten requires outer array cloning with complex proof.
            let outer_len = a.seq.len();
            let mut seq: Vec<T> = Vec::new();
            let mut i: usize = 0;
            while i < outer_len
                invariant
                    i <= outer_len,
                    outer_len == a.seq@.len(),
                    obeys_feq_clone::<T>(),
                    seq@ =~= a.seq@.take(i as int).map_values(|inner: ArraySeqMtPerS<T>| inner.seq@).flatten(),
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
                        seq@ =~= a.seq@.take(i as int).map_values(|inner: ArraySeqMtPerS<T>| inner.seq@).flatten()
                            + inner.seq@.take(j as int),
                    decreases inner_len - j,
                {
                    seq.push(inner.seq[j].clone());
                    // Veracity: NEEDED proof block
                    proof {
                        let ghost last = seq@[seq@.len() - 1 as int];
                        axiom_cloned_implies_eq_owned(inner.seq[j as int], last);
                    }
                    j += 1;
                }
                // Veracity: NEEDED proof block
                proof {
                    let ghost prefix = a.seq@.take(i as int).map_values(|inner: ArraySeqMtPerS<T>| inner.seq@);
                    // Veracity: NEEDED assert
                    assert(a.seq@.take(i as int + 1).map_values(|inner: ArraySeqMtPerS<T>| inner.seq@)
                        =~= prefix.push(a.seq@[i as int].seq@));
                    prefix.lemma_flatten_push(a.seq@[i as int].seq@);
                }
                i += 1;
            }
            // Veracity: NEEDED proof block
            proof {
            }
            ArraySeqMtPerS { seq }
        }
    }


    impl<T> ArraySeqMtPerS<T> {
        broadcast proof fn lemma_spec_index(&self, i: int)
            requires 0 <= i < self.spec_len()
            ensures #[trigger] self.seq@[i] == self.spec_index(i)
        {}

        /// Returns an iterator over the sequence elements.
        pub fn iter(&self) -> (it: ArraySeqMtPerIter<'_, T>)
            ensures
                it@.0 == 0,
                it@.1 == self.seq@,
                iter_invariant(&it),
        {
            ArraySeqMtPerIter { inner: self.seq.iter() }
        }

        /// - Parallel map. Transform each element via `f` using fork-join.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a|), Span O(log|a|).
        pub fn map_par<U: Clone + Eq + View + Send + Sync + 'static, F: Fn(&T) -> U + Send + Sync + Clone + 'static>(
            a: &ArraySeqMtPerS<T>,
            f: F,
        ) -> (mapped: ArraySeqMtPerS<U>)
            where T: Clone + Send + Sync + Eq + 'static
            requires
                obeys_feq_clone::<T>(),
                obeys_feq_clone::<U>(),
                forall|i: int| 0 <= i < a.seq@.len() ==> #[trigger] f.requires((&a.seq@[i],)),
            ensures mapped.seq@.len() == a.seq@.len()
            decreases a.seq@.len()
        {
            let len = a.seq.len();
            if len == 0 {
                ArraySeqMtPerS { seq: Vec::new() }
            } else if len == 1 {
                ArraySeqMtPerS { seq: vec![f(&a.seq[0])] }
            } else {
                let mid = len / 2;
                let left_seq = a.subseq_copy(0, mid);
                let right_seq = a.subseq_copy(mid, len - mid);
                let f1 = clone_fn(&f);
                let f2 = f;
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    assert forall|i: int| 0 <= i < left_seq.seq@.len() implies #[trigger] f1.requires((&left_seq.seq@[i],)) by {
                        a.lemma_spec_index(0 + i);
                        left_seq.lemma_spec_index(i);
                    }
                    // Veracity: NEEDED assert
                    assert forall|i: int| 0 <= i < right_seq.seq@.len() implies #[trigger] f2.requires((&right_seq.seq@[i],)) by {
                        a.lemma_spec_index(mid as int + i);
                        right_seq.lemma_spec_index(i);
                    }
                }

                let fa = move || -> (r: ArraySeqMtPerS<U>)
                    requires
                        forall|i: int| 0 <= i < left_seq.seq@.len() ==> #[trigger] f1.requires((&left_seq.seq@[i],)),
                    ensures r.seq@.len() == left_seq.seq@.len(),
                {
                    Self::map_par(&left_seq, f1)
                };

                let fb = move || -> (r: ArraySeqMtPerS<U>)
                    requires
                        forall|i: int| 0 <= i < right_seq.seq@.len() ==> #[trigger] f2.requires((&right_seq.seq@[i],)),
                    ensures r.seq@.len() == right_seq.seq@.len(),
                {
                    Self::map_par(&right_seq, f2)
                };

                let (left, right) = join(fa, fb);
                ArraySeqMtPerS::<U>::append(&left, &right)
            }
        }

        /// Parallel divide-and-conquer filter. Called by trait method filter.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn filter_dc<F: Fn(&T) -> bool + Clone + Send + Sync + 'static>(
            a: &ArraySeqMtPerS<T>,
            pred: &F,
            Ghost(spec_pred): Ghost<spec_fn(T) -> bool>,
        ) -> (filtered: ArraySeqMtPerS<T>)
            where T: Clone + Eq + Send + Sync + 'static
            requires
                obeys_feq_clone::<T>(),
                forall|i: int| 0 <= i < a.seq@.len() ==> #[trigger] pred.requires((&a.seq@[i],)),
                forall|v: T, keep: bool| pred.ensures((&v,), keep) ==> spec_pred(v) == keep,
            ensures
                filtered.spec_arrayseqmtper_wf(),
                filtered.spec_len() <= a.seq@.len(),
                filtered.spec_len() == spec_filter_len(a.seq@, spec_pred),
                filtered.seq@.to_multiset() =~= a.seq@.to_multiset().filter(spec_pred),
                forall|i: int| #![trigger filtered.spec_index(i)] 0 <= i < filtered.spec_len()
                    ==> pred.ensures((&filtered.spec_index(i),), true),
            decreases a.seq@.len()
        {
            let len = a.seq.len();
            if len == 0 {
                let filtered = ArraySeqMtPerS::<T> { seq: Vec::new() };
                // Veracity: NEEDED proof block
                proof {
                    broadcast use vstd::multiset::group_multiset_axioms;
                }
                filtered
            } else if len == 1 {
                let keep = pred(&a.seq[0]);
                if keep {
                    let elem = a.seq[0].clone();
                    // Veracity: NEEDED proof block
                    proof {
                        axiom_cloned_implies_eq_owned(a.seq[0 as int], elem);
                        broadcast use vstd::seq_lib::group_to_multiset_ensures;
                        broadcast use vstd::multiset::group_multiset_axioms;
                        reveal_with_fuel(spec_filter_len, 2);
                    }
                    let mut seq = Vec::with_capacity(1);
                    seq.push(elem);
                    let filtered = ArraySeqMtPerS { seq };
                    // Veracity: NEEDED proof block
                    proof {
                        broadcast use vstd::multiset::group_multiset_axioms;
                        // Veracity: NEEDED assert
                        assert(filtered.seq@ =~= a.seq@);
                    }
                    filtered
                } else {
                    let filtered = ArraySeqMtPerS::<T> { seq: Vec::new() };
                    // Veracity: NEEDED proof block
                    proof {
                        broadcast use vstd::seq_lib::group_to_multiset_ensures;
                        broadcast use vstd::multiset::group_multiset_axioms;
                        reveal_with_fuel(spec_filter_len, 2);
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

                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    assert forall|i: int| 0 <= i < left_seq.seq@.len() implies
                        #[trigger] p1.requires((&left_seq.seq@[i],))
                    by {
                        a.lemma_spec_index(i);
                        left_seq.lemma_spec_index(i);
                    }
                    // Veracity: NEEDED assert
                    assert forall|i: int| 0 <= i < right_seq.seq@.len() implies
                        #[trigger] p2.requires((&right_seq.seq@[i],))
                    by {
                        a.lemma_spec_index(mid as int + i);
                        right_seq.lemma_spec_index(i);
                    }
                    // Veracity: NEEDED assert
                    assert(left_view =~= a.seq@.subrange(0, mid as int)) by {
                        // Veracity: NEEDED assert
                        assert forall|i: int| 0 <= i < left_view.len() implies
                            #[trigger] left_view[i] == a.seq@.subrange(0, mid as int)[i]
                        by { left_seq.lemma_spec_index(i); a.lemma_spec_index(i); }
                    }
                    // Veracity: NEEDED assert
                    assert(right_view =~= a.seq@.subrange(mid as int, len as int)) by {
                        // Veracity: NEEDED assert
                        assert forall|i: int| 0 <= i < right_view.len() implies
                            #[trigger] right_view[i] == a.seq@.subrange(mid as int, len as int)[i]
                        by { right_seq.lemma_spec_index(i); a.lemma_spec_index(mid as int + i); }
                    }
                    // Veracity: NEEDED assert
                    assert(a.seq@ =~= left_view + right_view);
                }

                let fa = move || -> (r: ArraySeqMtPerS<T>)
                    requires
                        obeys_feq_clone::<T>(),
                        forall|i: int| 0 <= i < left_seq.seq@.len() ==> #[trigger] p1.requires((&left_seq.seq@[i],)),
                        forall|v: T, keep: bool| p1.ensures((&v,), keep) ==> spec_pred(v) == keep,
                    ensures
                        r.spec_arrayseqmtper_wf(),
                        r.spec_len() <= left_view.len(),
                        r.spec_len() == spec_filter_len(left_view, spec_pred),
                        r.seq@.to_multiset() =~= left_view.to_multiset().filter(spec_pred),
                        forall|i: int| #![trigger r.spec_index(i)] 0 <= i < r.spec_len()
                            ==> p1.ensures((&r.spec_index(i),), true),
                {
                    Self::filter_dc(&left_seq, &p1, Ghost(spec_pred))
                };

                let fb = move || -> (r: ArraySeqMtPerS<T>)
                    requires
                        obeys_feq_clone::<T>(),
                        forall|i: int| 0 <= i < right_seq.seq@.len() ==> #[trigger] p2.requires((&right_seq.seq@[i],)),
                        forall|v: T, keep: bool| p2.ensures((&v,), keep) ==> spec_pred(v) == keep,
                    ensures
                        r.spec_arrayseqmtper_wf(),
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
                // Veracity: NEEDED proof block
                proof {
                    lemma_spec_filter_len_concat(left_view, right_view, spec_pred);
                    lemma_seq_concat_to_multiset_filter(left_view, right_view, spec_pred);

                    // Veracity: NEEDED assert
                    assert(filtered.seq@ =~= left.seq@ + right.seq@) by {
                        // Veracity: NEEDED assert
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

                    // Veracity: NEEDED assert
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

        /// - Parallel filter. Keep elements satisfying `pred` using fork-join.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a|), Span O(log|a|).
        pub fn filter_par<F: Fn(&T) -> bool + Send + Sync + Clone + 'static>(
            a: &ArraySeqMtPerS<T>,
            pred: F,
        ) -> (filtered: ArraySeqMtPerS<T>)
            where T: Clone + Send + Sync + Eq + 'static
            requires
                obeys_feq_clone::<T>(),
                forall|i: int| 0 <= i < a.seq@.len() ==> #[trigger] pred.requires((&a.seq@[i],)),
            ensures filtered.seq@.len() <= a.seq@.len()
            decreases a.seq@.len()
        {
            let len = a.seq.len();
            if len == 0 {
                ArraySeqMtPerS { seq: Vec::new() }
            } else if len == 1 {
                if pred(&a.seq[0]) {
                    ArraySeqMtPerS { seq: vec![a.seq[0].clone()] }
                } else {
                    ArraySeqMtPerS { seq: Vec::new() }
                }
            } else {
                let mid = len / 2;
                let left_seq = a.subseq_copy(0, mid);
                let right_seq = a.subseq_copy(mid, len - mid);
                let p1 = clone_pred(&pred);
                let p2 = pred;
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    assert forall|i: int| 0 <= i < left_seq.seq@.len() implies #[trigger] p1.requires((&left_seq.seq@[i],)) by {
                        a.lemma_spec_index(0 + i);
                        left_seq.lemma_spec_index(i);
                    }
                    // Veracity: NEEDED assert
                    assert forall|i: int| 0 <= i < right_seq.seq@.len() implies #[trigger] p2.requires((&right_seq.seq@[i],)) by {
                        a.lemma_spec_index(mid as int + i);
                        right_seq.lemma_spec_index(i);
                    }
                }

                let fa = move || -> (r: ArraySeqMtPerS<T>)
                    requires forall|i: int| 0 <= i < left_seq.seq@.len() ==> #[trigger] p1.requires((&left_seq.seq@[i],)),
                    ensures r.seq@.len() <= left_seq.seq@.len(),
                {
                    Self::filter_par(&left_seq, p1)
                };

                let fb = move || -> (r: ArraySeqMtPerS<T>)
                    requires forall|i: int| 0 <= i < right_seq.seq@.len() ==> #[trigger] p2.requires((&right_seq.seq@[i],)),
                    ensures r.seq@.len() <= right_seq.seq@.len(),
                {
                    Self::filter_par(&right_seq, p2)
                };

                let (left, right) = join(fa, fb);
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

                // Split s into s1 ++ [a_last] via lemma_fold_left_split
                s.lemma_fold_left_split(id, f, n);
                s.lemma_fold_left_split(x, f, n);
                // tail.fold_left(s1.fold_left(id, f), f) == s.fold_left(id, f)
                // tail.fold_left(s1.fold_left(x, f), f) == s.fold_left(x, f)

                reveal_with_fuel(Seq::fold_left, 2);
                // tail.fold_left(y, f) == f(y, a_last)

                let lid = s1.fold_left(id, f);
                let lx = s1.fold_left(x, f);
                // s.fold_left(id, f) == f(lid, a_last)
                // s.fold_left(x, f) == f(lx, a_last)

                // IH: f(x, lid) == lx
                Self::lemma_monoid_fold_left(s1, f, id, x);

                // Chain: f(x, s.fold_left(id, f)) == f(x, f(lid, a_last))
                //   == f(f(x, lid), a_last) [associativity]
                //   == f(lx, a_last)         [IH]
                //   == s.fold_left(x, f)
            }
        }

        /// - Parallel reduce. Combine elements using associative `f` and identity `id` via fork-join.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a|), Span O(log|a|).
        pub fn reduce_par<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(
            a: &ArraySeqMtPerS<T>,
            f: F,
            Ghost(spec_f): Ghost<spec_fn(T, T) -> T>,
            id: T,
        ) -> (reduced: T)
            where T: Clone + Send + Sync + Eq + 'static
            requires
                obeys_feq_clone::<T>(),
                spec_monoid(spec_f, id),
                forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) <==> ret == spec_f(x, y),
            ensures
                reduced == spec_iterate(Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_f, id),
            decreases a.seq@.len()
        {
            let ghost s = Seq::new(a.spec_len(), |i: int| a.spec_index(i));
            let len = a.seq.len();
            if len == 0 {
                // Veracity: NEEDED proof block
                proof {
                    reveal_with_fuel(Seq::fold_left, 1);
                }
                id
            } else if len == 1 {
                let element = a.seq[0].clone();
                // Veracity: NEEDED proof block
                proof {
                    axiom_cloned_implies_eq_owned(a.seq[0 as int], element);
                    a.lemma_spec_index(0);
                    reveal_with_fuel(Seq::fold_left, 2);
                }
                element
            } else {
                let mid = len / 2;
                let left_seq = a.subseq_copy(0, mid);
                let right_seq = a.subseq_copy(mid, len - mid);
                let f1 = clone_fn2(&f);
                let f2 = clone_fn2(&f);
                let id1 = id.clone();
                // Veracity: NEEDED proof block
                proof { axiom_cloned_implies_eq_owned(id, id1); }
                let id2 = id.clone();
                // Veracity: NEEDED proof block
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
                // Veracity: NEEDED proof block
                proof {
                    // left == left_s.fold_left(id, spec_f)
                    // right == right_s.fold_left(id, spec_f)
                    // combined == spec_f(left, right)
                    // Veracity: NEEDED assert
                    assert(left_s =~= s.subrange(0, mid as int));
                    // Veracity: NEEDED assert
                    assert(right_s =~= s.subrange(mid as int, len as int));
                    s.lemma_fold_left_split(id, spec_f, mid as int);
                    // s.fold_left(id, spec_f) == right_s.fold_left(left_s.fold_left(id, spec_f), spec_f)
                    //                         == right_s.fold_left(left, spec_f)
                    Self::lemma_monoid_fold_left(right_s, spec_f, id, left);
                    // spec_f(left, right_s.fold_left(id, spec_f)) == right_s.fold_left(left, spec_f)
                    // i.e., spec_f(left, right) == right_s.fold_left(left, spec_f)
                    //                           == s.fold_left(id, spec_f)
                }
                combined
            }
        }

        /// Parallel map inner recursive helper with decreases.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        pub fn map_inner<U: Clone + Eq + Send + Sync + 'static, F: Fn(&T) -> U + Clone + Send + Sync + 'static>(
            a: &ArraySeqMtPerS<T>,
            f: &F,
        ) -> (mapped: ArraySeqMtPerS<U>)
            where T: Clone + Eq + Send + Sync + 'static
            requires
                obeys_feq_clone::<T>(),
                obeys_feq_clone::<U>(),
                forall|i: int| 0 <= i < a.seq@.len() ==> #[trigger] f.requires((&a.seq@[i],)),
            ensures
                mapped.seq@.len() == a.seq@.len(),
                forall|i: int| #![trigger mapped.seq@[i]] 0 <= i < a.seq@.len() ==> f.ensures((&a.seq@[i],), mapped.seq@[i]),
            decreases a.seq@.len(),
        {
            let len = a.seq.len();
            if len == 0 {
                ArraySeqMtPerS { seq: Vec::new() }
            } else if len == 1 {
                let mut seq = Vec::with_capacity(1);
                seq.push(f(&a.seq[0]));
                ArraySeqMtPerS { seq }
            } else {
                let mid = len / 2;
                let left_seq = a.subseq_copy(0, mid);
                let right_seq = a.subseq_copy(mid, len - mid);
                let f1 = clone_fn(f);
                let f2 = clone_fn(f);
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    assert forall|i: int| 0 <= i < left_seq.seq@.len()
                        implies #[trigger] f1.requires((&left_seq.seq@[i],)) by {
                        left_seq.lemma_spec_index(i);
                        a.lemma_spec_index(i);
                    }
                    // Veracity: NEEDED assert
                    assert forall|i: int| 0 <= i < right_seq.seq@.len()
                        implies #[trigger] f2.requires((&right_seq.seq@[i],)) by {
                        right_seq.lemma_spec_index(i);
                        a.lemma_spec_index(mid as int + i);
                    }
                }

                let ghost left_g = left_seq.seq@;
                let ghost right_g = right_seq.seq@;

                let fa = move || -> (r: ArraySeqMtPerS<U>)
                    requires
                        obeys_feq_clone::<T>(),
                        obeys_feq_clone::<U>(),
                        forall|i: int| 0 <= i < left_seq.seq@.len()
                            ==> #[trigger] f1.requires((&left_seq.seq@[i],)),
                    ensures
                        r.seq@.len() == left_g.len(),
                        forall|i: int| #![trigger r.seq@[i]] 0 <= i < left_g.len()
                            ==> f1.ensures((&left_g[i],), r.seq@[i]),
                {
                    Self::map_inner(&left_seq, &f1)
                };

                let fb = move || -> (r: ArraySeqMtPerS<U>)
                    requires
                        obeys_feq_clone::<T>(),
                        obeys_feq_clone::<U>(),
                        forall|i: int| 0 <= i < right_seq.seq@.len()
                            ==> #[trigger] f2.requires((&right_seq.seq@[i],)),
                    ensures
                        r.seq@.len() == right_g.len(),
                        forall|i: int| #![trigger r.seq@[i]] 0 <= i < right_g.len()
                            ==> f2.ensures((&right_g[i],), r.seq@[i]),
                {
                    Self::map_inner(&right_seq, &f2)
                };

                let (left_mapped, right_mapped) = join(fa, fb);
                let combined = ArraySeqMtPerS::<U>::append(&left_mapped, &right_mapped);

                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    assert forall|i: int| #![trigger combined.seq@[i]] 0 <= i < a.seq@.len()
                        implies f.ensures((&a.seq@[i],), combined.seq@[i]) by {
                        combined.lemma_spec_index(i);
                        if i < mid as int {
                            left_mapped.lemma_spec_index(i);
                            left_seq.lemma_spec_index(i);
                            a.lemma_spec_index(i);
                        } else {
                            let j = i - mid as int;
                            right_mapped.lemma_spec_index(j);
                            right_seq.lemma_spec_index(j);
                            a.lemma_spec_index(i);
                        }
                    }
                }

                combined
            }
        }

        /// Parallel filter inner recursive helper with decreases.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        pub fn filter_inner<F: Fn(&T) -> bool + Clone + Send + Sync + 'static>(
            a: &ArraySeqMtPerS<T>,
            pred: &F,
            Ghost(spec_pred): Ghost<spec_fn(T) -> bool>,
        ) -> (filtered: ArraySeqMtPerS<T>)
            where T: Clone + Eq + Send + Sync + 'static
            requires
                obeys_feq_clone::<T>(),
                forall|i: int| 0 <= i < a.seq@.len() ==> #[trigger] pred.requires((&a.seq@[i],)),
                forall|v: T, keep: bool| pred.ensures((&v,), keep) ==> spec_pred(v) == keep,
            ensures
                filtered.spec_arrayseqmtper_wf(),
                filtered.spec_len() <= a.seq@.len(),
                forall|i: int| #![trigger filtered.spec_index(i)] 0 <= i < filtered.spec_len()
                    ==> pred.ensures((&filtered.spec_index(i),), true),
            decreases a.seq@.len(),
        {
            let len = a.seq.len();
            if len == 0 {
                ArraySeqMtPerS { seq: Vec::new() }
            } else if len == 1 {
                if pred(&a.seq[0]) {
                    let elem = a.seq[0].clone();
                    // Veracity: NEEDED proof block
                    proof { axiom_cloned_implies_eq_owned(a.seq[0 as int], elem); }
                    let mut seq = Vec::with_capacity(1);
                    seq.push(elem);
                    ArraySeqMtPerS { seq }
                } else {
                    ArraySeqMtPerS { seq: Vec::new() }
                }
            } else {
                let mid = len / 2;
                let left_seq = a.subseq_copy(0, mid);
                let right_seq = a.subseq_copy(mid, len - mid);
                let p1 = clone_pred(pred);
                let p2 = clone_pred(pred);
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    assert forall|i: int| 0 <= i < left_seq.seq@.len()
                        implies #[trigger] p1.requires((&left_seq.seq@[i],)) by {
                        left_seq.lemma_spec_index(i);
                        a.lemma_spec_index(i);
                    }
                    // Veracity: NEEDED assert
                    assert forall|i: int| 0 <= i < right_seq.seq@.len()
                        implies #[trigger] p2.requires((&right_seq.seq@[i],)) by {
                        right_seq.lemma_spec_index(i);
                        a.lemma_spec_index(mid as int + i);
                    }
                }

                let fa = move || -> (r: ArraySeqMtPerS<T>)
                    requires
                        obeys_feq_clone::<T>(),
                        forall|i: int| 0 <= i < left_seq.seq@.len()
                            ==> #[trigger] p1.requires((&left_seq.seq@[i],)),
                        forall|v: T, keep: bool| p1.ensures((&v,), keep) ==> spec_pred(v) == keep,
                    ensures
                        r.spec_arrayseqmtper_wf(),
                        r.spec_len() <= left_seq.spec_len(),
                        forall|i: int| #![trigger r.spec_index(i)] 0 <= i < r.spec_len()
                            ==> p1.ensures((&r.spec_index(i),), true),
                {
                    Self::filter_inner(&left_seq, &p1, Ghost(spec_pred))
                };

                let fb = move || -> (r: ArraySeqMtPerS<T>)
                    requires
                        obeys_feq_clone::<T>(),
                        forall|i: int| 0 <= i < right_seq.seq@.len()
                            ==> #[trigger] p2.requires((&right_seq.seq@[i],)),
                        forall|v: T, keep: bool| p2.ensures((&v,), keep) ==> spec_pred(v) == keep,
                    ensures
                        r.spec_arrayseqmtper_wf(),
                        r.spec_len() <= right_seq.spec_len(),
                        forall|i: int| #![trigger r.spec_index(i)] 0 <= i < r.spec_len()
                            ==> p2.ensures((&r.spec_index(i),), true),
                {
                    Self::filter_inner(&right_seq, &p2, Ghost(spec_pred))
                };

                let (left_filtered, right_filtered) = join(fa, fb);
                let combined = Self::append(&left_filtered, &right_filtered);

                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    assert forall|i: int| #![trigger combined.spec_index(i)] 0 <= i < combined.spec_len()
                        implies pred.ensures((&combined.spec_index(i),), true) by {
                        combined.lemma_spec_index(i);
                        if i < left_filtered.spec_len() {
                            left_filtered.lemma_spec_index(i);
                        } else {
                            right_filtered.lemma_spec_index(i - left_filtered.spec_len());
                        }
                    }
                }

                combined
            }
        }

        /// Parallel reduce inner recursive helper with decreases.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(log n)
        pub fn reduce_inner<F: Fn(&T, &T) -> T + Clone + Send + Sync + 'static>(
            a: &ArraySeqMtPerS<T>,
            f: &F,
            Ghost(spec_f): Ghost<spec_fn(T, T) -> T>,
            id: T,
        ) -> (reduced: T)
            where T: Clone + Eq + Send + Sync + 'static
            requires
                obeys_feq_clone::<T>(),
                spec_monoid(spec_f, id),
                forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) ==> ret == spec_f(x, y),
            ensures
                reduced == spec_iterate(Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_f, id),
            decreases a.seq@.len(),
        {
            let ghost s = Seq::new(a.spec_len(), |i: int| a.spec_index(i));
            let len = a.seq.len();
            if len == 0 {
                // Veracity: NEEDED proof block
                proof {
                    reveal_with_fuel(Seq::fold_left, 1);
                }
                id
            } else if len == 1 {
                let element = a.seq[0].clone();
                // Veracity: NEEDED proof block
                proof {
                    axiom_cloned_implies_eq_owned(a.seq[0 as int], element);
                    a.lemma_spec_index(0);
                    reveal_with_fuel(Seq::fold_left, 2);
                }
                element
            } else {
                let mid = len / 2;
                let left_seq = a.subseq_copy(0, mid);
                let right_seq = a.subseq_copy(mid, len - mid);
                let f1 = clone_fn2(f);
                let f2 = clone_fn2(f);
                let id1 = id.clone();
                // Veracity: NEEDED proof block
                proof { axiom_cloned_implies_eq_owned(id, id1); }
                let id2 = id.clone();
                // Veracity: NEEDED proof block
                proof { axiom_cloned_implies_eq_owned(id, id2); }

                let ghost left_s = Seq::new(left_seq.spec_len(), |i: int| left_seq.spec_index(i));
                let ghost right_s = Seq::new(right_seq.spec_len(), |i: int| right_seq.spec_index(i));

                let fa = move || -> (r: T)
                    requires
                        left_seq.seq@.len() > 0,
                        obeys_feq_clone::<T>(),
                        spec_monoid(spec_f, id),
                        forall|x: &T, y: &T| #[trigger] f1.requires((x, y)),
                        forall|x: T, y: T, ret: T| f1.ensures((&x, &y), ret) ==> ret == spec_f(x, y),
                    ensures
                        r == spec_iterate(left_s, spec_f, id),
                {
                    Self::reduce_inner(&left_seq, &f1, Ghost(spec_f), id1)
                };

                let fb = move || -> (r: T)
                    requires
                        right_seq.seq@.len() > 0,
                        obeys_feq_clone::<T>(),
                        spec_monoid(spec_f, id),
                        forall|x: &T, y: &T| #[trigger] f2.requires((x, y)),
                        forall|x: T, y: T, ret: T| f2.ensures((&x, &y), ret) ==> ret == spec_f(x, y),
                    ensures
                        r == spec_iterate(right_s, spec_f, id),
                {
                    Self::reduce_inner(&right_seq, &f2, Ghost(spec_f), id2)
                };

                let (left_result, right_result) = join(fa, fb);
                let combined = f(&left_result, &right_result);
                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    assert(left_s =~= s.subrange(0, mid as int));
                    // Veracity: NEEDED assert
                    assert(right_s =~= s.subrange(mid as int, len as int));
                    s.lemma_fold_left_split(id, spec_f, mid as int);
                    Self::lemma_monoid_fold_left(right_s, spec_f, id, left_result);
                }
                combined
            }
        }

        /// Parallel tabulate inner recursive helper with decreases.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        pub fn tabulate_inner<F: Fn(usize) -> T + Clone + Send + Sync + 'static>(
            f: &F,
            offset: usize,
            length: usize,
        ) -> (tab_seq: ArraySeqMtPerS<T>)
            where T: Clone + Eq + Send + Sync + 'static
            requires
                obeys_feq_clone::<T>(),
                offset + length <= usize::MAX,
                forall|i: usize| offset <= i < offset + length ==> #[trigger] f.requires((i,)),
            ensures
                tab_seq.seq@.len() == length,
                forall|i: int| #![trigger tab_seq.seq@[i]] 0 <= i < length
                    ==> f.ensures(((offset + i) as usize,), tab_seq.seq@[i]),
            decreases length,
        {
            if length == 0 {
                ArraySeqMtPerS { seq: Vec::new() }
            } else if length == 1 {
                let mut seq = Vec::with_capacity(1);
                seq.push(f(offset));
                ArraySeqMtPerS { seq }
            } else {
                let mid = length / 2;
                let f1 = clone_fn_usize(f);
                let f2 = clone_fn_usize(f);

                let fa = move || -> (r: ArraySeqMtPerS<T>)
                    requires
                        obeys_feq_clone::<T>(),
                        offset + length <= usize::MAX,
                        forall|i: usize| offset <= i < offset + length ==> #[trigger] f1.requires((i,)),
                    ensures
                        r.seq@.len() == mid as int,
                        forall|i: int| #![trigger r.seq@[i]] 0 <= i < mid
                            ==> f1.ensures(((offset + i) as usize,), r.seq@[i]),
                {
                    Self::tabulate_inner(&f1, offset, mid)
                };

                let fb = move || -> (r: ArraySeqMtPerS<T>)
                    requires
                        obeys_feq_clone::<T>(),
                        offset + length <= usize::MAX,
                        mid <= length,
                        forall|i: usize| offset <= i < offset + length ==> #[trigger] f2.requires((i,)),
                    ensures
                        r.seq@.len() == (length - mid) as int,
                        forall|i: int| #![trigger r.seq@[i]] 0 <= i < (length - mid)
                            ==> f2.ensures(((offset + mid + i) as usize,), r.seq@[i]),
                {
                    Self::tabulate_inner(&f2, offset + mid, length - mid)
                };

                let (left, right) = join(fa, fb);
                let combined = Self::append(&left, &right);

                // Veracity: NEEDED proof block
                proof {
                    // Veracity: NEEDED assert
                    assert forall|i: int| #![trigger combined.seq@[i]] 0 <= i < length
                        implies f.ensures(((offset + i) as usize,), combined.seq@[i]) by {
                        combined.lemma_spec_index(i);
                        if i < mid as int {
                            left.lemma_spec_index(i);
                        } else {
                            let j = i - mid as int;
                            right.lemma_spec_index(j);
                        }
                    }
                }

                combined
            }
        }

        // BYPASSED: flatten_inner — D&C flatten requires outer array cloning proof
        // that is incompatible with subseq_copy's element-level ensures.
        // The sequential flatten in the trait impl is correct and verified.
    }

    //		Section 10. iterators


    #[verifier::reject_recursive_types(T)]
    pub struct ArraySeqMtPerIter<'a, T> {
        pub inner: std::slice::Iter<'a, T>,
    }

    impl<'a, T> View for ArraySeqMtPerIter<'a, T> {
        type V = (int, Seq<T>);
        open spec fn view(&self) -> (int, Seq<T>) { self.inner@ }
    }

    pub open spec fn iter_invariant<'a, T>(it: &ArraySeqMtPerIter<'a, T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<'a, T> std::iter::Iterator for ArraySeqMtPerIter<'a, T> {
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
    pub struct ArraySeqMtPerGhostIterator<'a, T> {
        pub pos: int,
        pub elements: Seq<T>,
        pub phantom: core::marker::PhantomData<&'a T>,
    }

    impl<'a, T> View for ArraySeqMtPerGhostIterator<'a, T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.elements.take(self.pos) }
    }

    impl<'a, T> vstd::pervasive::ForLoopGhostIteratorNew for ArraySeqMtPerIter<'a, T> {
        type GhostIter = ArraySeqMtPerGhostIterator<'a, T>;
        open spec fn ghost_iter(&self) -> ArraySeqMtPerGhostIterator<'a, T> {
            ArraySeqMtPerGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    impl<'a, T> vstd::pervasive::ForLoopGhostIterator for ArraySeqMtPerGhostIterator<'a, T> {
        type ExecIter = ArraySeqMtPerIter<'a, T>;
        type Item = T;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &ArraySeqMtPerIter<'a, T>) -> bool {
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

        open spec fn ghost_advance(&self, _exec_iter: &ArraySeqMtPerIter<'a, T>) -> ArraySeqMtPerGhostIterator<'a, T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, T> std::iter::IntoIterator for &'a ArraySeqMtPerS<T> {
        type Item = &'a T;
        type IntoIter = ArraySeqMtPerIter<'a, T>;
        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                it@.0 == 0,
                it@.1 == self.seq@,
                iter_invariant(&it),
        {
            ArraySeqMtPerIter { inner: self.seq.iter() }
        }
    }

    impl<T> std::iter::IntoIterator for ArraySeqMtPerS<T> {
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


    #[cfg(verus_keep_ghost)]
    impl<T: View + PartialEq> PartialEqSpecImpl for ArraySeqMtPerS<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }


    impl<T: Clone> Clone for ArraySeqMtPerS<T> {
        fn clone(&self) -> (res: Self)
            ensures
                res.seq@.len() == self.seq@.len(),
                forall|i: int| #![trigger res.seq@[i]]
                    0 <= i < self.seq@.len() ==> cloned::<T>(self.seq@[i], res.seq@[i]),
        {
            ArraySeqMtPerS { seq: self.seq.clone() }
        }
    }

    impl<T: Eq + View> Eq for ArraySeqMtPerS<T> {}

    impl<T: PartialEq + View> PartialEq for ArraySeqMtPerS<T> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            let equal = self.seq == other.seq;
            // Veracity: NEEDED proof block
            proof { assume(equal == (self@ == other@)); }
            equal
        }
    }

    } // verus!

    //		Section 13. macros


    /// Literal constructor macro for ArraySeqMtPerS.
    #[macro_export]
    macro_rules! ArraySeqMtPerSLit {
        () => { $crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::ArraySeqMtPerS::from_vec(Vec::new()) };
        ($x:expr; $n:expr) => { $crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::ArraySeqMtPerS::from_vec(vec![$x; $n]) };
        ($($x:expr),* $(,)?) => { $crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::ArraySeqMtPerS::from_vec(vec![$($x),*]) };
    }

    #[macro_export]
    macro_rules! ArrayMtPerSLit {
        () => { $crate::ArraySeqMtPerSLit![] };
        ($x:expr; $n:expr) => { $crate::ArraySeqMtPerSLit![$x; $n] };
        ($($x:expr),* $(,)?) => { $crate::ArraySeqMtPerSLit![$($x),*] };
    }

    //		Section 14. derive impls outside verus!

    impl<T: Debug> Debug for ArraySeqMtPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_list().entries(self.seq.iter()).finish()
        }
    }

    impl<T: Display> Display for ArraySeqMtPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "[")?;
            for (i, item) in self.seq.iter().enumerate() {
                if i > 0 { write!(f, ", ")?; }
                write!(f, "{item}")?;
            }
            write!(f, "]")
        }
    }

    impl<'a, T: Debug> Debug for ArraySeqMtPerIter<'a, T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "ArraySeqMtPerIter({:?})", self.inner)
        }
    }

    impl<'a, T> Display for ArraySeqMtPerIter<'a, T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "ArraySeqMtPerIter")
        }
    }

    impl<'a, T> Debug for ArraySeqMtPerGhostIterator<'a, T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "ArraySeqMtPerGhostIterator")
        }
    }

    impl<'a, T> Display for ArraySeqMtPerGhostIterator<'a, T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "ArraySeqMtPerGhostIterator")
        }
    }
}
