//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Chapter 19 parametric sequence implementation for array-backed sequences (ephemeral). Verusified.
//! Functions are implemented in terms of primitives: nth, length, subseq, tabulate, flatten.
//! Ephemeral variant: supports in-place mutation via `set`.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	4. type definitions
//	5. view impls
//	8. traits
//	9. impls
//	10. iterators
//	11. derive impls in verus!
//	13. derive impls outside verus!

//		1. module


pub mod ArraySeqStEph {

    use std::fmt::{Debug, Display, Formatter};
    use std::fmt::Result as FmtResult;
    use std::slice::Iter;
    use std::vec::IntoIter;

    use vstd::prelude::*;

    verus! {

    //		2. imports

    #[cfg(verus_keep_ghost)]
    use {
        vstd::std_specs::cmp::PartialEqSpecImpl,
        vstd::std_specs::vec::*,
        vstd::std_specs::clone::*,
    };
    #[cfg(verus_keep_ghost)]
    use {
        crate::vstdplus::feq::feq::*,
        crate::vstdplus::multiset::multiset::*,
    };


    //		3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
        vstd::seq_lib::group_seq_properties,
        crate::vstdplus::feq::feq::group_feq_axioms,
    };


    //		4. type definitions

    #[verifier::reject_recursive_types(T)]
    pub struct ArraySeqStEphS<T> {
        pub seq: Vec<T>,
    }


    //		5. view impls

    impl<T: View> View for ArraySeqStEphS<T> {
        type V = Seq<T::V>;

        open spec fn view(&self) -> Seq<T::V> {
            self.seq@.map(|_i: int, t: T| t@)
        }
    }


    //		8. traits

    /// Chapter 19 single-threaded ephemeral array sequence trait.
    /// Specifications match Chapter 18; algorithms from Chapter 19.
    /// Ephemeral: supports in-place mutation via `set`.
    pub trait ArraySeqStEphTrait<T>: Sized {
        spec fn spec_len(&self) -> nat;
        spec fn spec_index(&self, i: int) -> T
            recommends i < self.spec_len();

        /// - Create a new sequence of length `length` with each element initialized to `init_value`.
        /// - Work Θ(length), Span Θ(1).
        fn new(length: usize, init_value: T) -> (new_seq: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                length <= usize::MAX,
            ensures
                new_seq.spec_len() == length as int,
                forall|i: int| #![trigger new_seq.spec_index(i)] 0 <= i < length ==> new_seq.spec_index(i) == init_value;

        /// - Set the element at `index` to `item` in place (ephemeral mutation).
        /// - Work Θ(1), Span Θ(1).
        fn set(&mut self, index: usize, item: T) -> (success: Result<(), &'static str>)
            requires index < old(self).spec_len()
            ensures
                success.is_ok() ==> self.spec_len() == old(self).spec_len(),
                success.is_ok() ==> self.spec_index(index as int) == item,
                success.is_ok() ==> forall|i: int| #![trigger self.spec_index(i), old(self).spec_index(i)] 0 <= i < old(self).spec_len() && i != index ==> self.spec_index(i) == old(self).spec_index(i);

        /// - Definition 18.1 (length). Return the number of elements.
        /// - Work Θ(1), Span Θ(1).
        fn length(&self) -> (len: usize)
            ensures len as int == self.spec_len();

        /// - Algorithm 19.11 (Function nth). Return a reference to the element at `index`.
        /// - Work Θ(1), Span Θ(1).
        fn nth(&self, index: usize) -> (nth_elem: &T)
            requires index < self.spec_len()
            ensures *nth_elem == self.spec_index(index as int);

        /// - Definition 18.12 (subseq copy). Extract contiguous subsequence with allocation.
        /// - Work Θ(length), Span Θ(1).
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
        /// - Work Θ(length), Span Θ(1).
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
        /// - Work Θ(n) worst case, Θ(1) best case, Span Θ(1).
        fn from_vec(elts: Vec<T>) -> (seq: Self)
            ensures
                seq.spec_len() == elts@.len(),
                forall|i: int| #![trigger seq.spec_index(i)] 0 <= i < elts@.len() ==> seq.spec_index(i) == elts@[i];

        /// - Algorithm 19.1 (empty). empty = tabulate (lambda i.i) 0.
        /// - Work Θ(1), Span Θ(1).
        fn empty() -> (empty_seq: Self)
            ensures empty_seq.spec_len() == 0;

        /// - Algorithm 19.2 (singleton). singleton x = tabulate (lambda i.x) 1.
        /// - Work Θ(1), Span Θ(1).
        fn singleton(item: T) -> (singleton: Self)
            where T: Clone + Eq
            requires obeys_feq_clone::<T>()
            ensures
                singleton.spec_len() == 1,
                singleton.spec_index(0) == item;

        /// - Algorithm 19.4 (append). append a b = tabulate (select(a,b)) (|a|+|b|).
        /// - Work Θ(|a| + |b|), Span Θ(1).
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> (appended: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                a.seq@.len() + b.seq@.len() <= usize::MAX as int,
            ensures
                appended.spec_len() == a.seq@.len() + b.seq@.len(),
                forall|i: int| #![trigger appended.spec_index(i)] 0 <= i < a.seq@.len() ==> appended.spec_index(i) == a.seq@[i],
                forall|i: int| #![trigger b.seq@[i]] 0 <= i < b.seq@.len() ==> appended.spec_index(a.seq@.len() as int + i) == b.seq@[i];

        /// - Algorithm 19.5 (filter). filter f a = flatten (map (deflate f) a).
        /// - Work Θ(|a|), Span Θ(1).
        fn filter<F: Fn(&T) -> bool>(a: &ArraySeqStEphS<T>, pred: &F, Ghost(spec_pred): Ghost<spec_fn(T) -> bool>) -> (filtered: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                forall|i: int| 0 <= i < a.seq@.len() ==> #[trigger] pred.requires((&a.seq@[i],)),
                forall|v: T, keep: bool| pred.ensures((&v,), keep) ==> spec_pred(v) == keep,
            ensures
                filtered.spec_len() <= a.seq@.len(),
                filtered.spec_len() == spec_filter_len(
                    Seq::new(a.seq@.len(), |i: int| a.seq@[i]), spec_pred),
                Seq::new(filtered.spec_len(), |i: int| filtered.spec_index(i)).to_multiset()
                    =~= Seq::new(a.seq@.len(), |i: int| a.seq@[i]).to_multiset().filter(spec_pred),
                forall|i: int| #![trigger filtered.spec_index(i)] 0 <= i < filtered.spec_len() ==> pred.ensures((&filtered.spec_index(i),), true);

        /// - Algorithm 19.6 (update). Ephemeral: clone then set (O(n) clone + O(1) set).
        /// - Work Θ(|a|), Span Θ(1).
        fn update(a: &ArraySeqStEphS<T>, index: usize, item: T) -> (updated: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                index < a.seq@.len(),
            ensures
                updated.spec_len() == a.seq@.len(),
                updated.spec_index(index as int) == item,
                forall|i: int| #![trigger updated.spec_index(i)] 0 <= i < a.seq@.len() && i != index as int ==> updated.spec_index(i) == a.seq@[i];

        /// - Algorithm 19.7 (isEmpty). isEmpty a = (|a| = 0).
        /// - Work Θ(1), Span Θ(1).
        fn is_empty(&self) -> (empty: bool)
            ensures empty <==> self.spec_len() == 0;

        /// - Algorithm 19.7 (isSingleton). isSingleton a = (|a| = 1).
        /// - Work Θ(1), Span Θ(1).
        fn is_singleton(&self) -> (single: bool)
            ensures single <==> self.spec_len() == 1;

        /// - Algorithm 19.8 (iterate). Left fold over the sequence (iterative).
        /// - Work Θ(|a|), Span Θ(|a|).
        fn iterate_iter<A, F: Fn(&A, &T) -> A>(a: &ArraySeqStEphS<T>, f: &F, seed: A) -> A
            requires forall|x: &A, y: &T| #[trigger] f.requires((x, y));

        /// - Algorithm 19.8 (iterate). iterate f x a = if |a|=0 then x else iterate f (f(x,a[0])) a[1..|a|-1].
        /// - Work Θ(|a|), Span Θ(|a|).
        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqStEphS<T>, f: &F, seed: A) -> A
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                forall|x: &A, y: &T| #[trigger] f.requires((x, y));

        /// - Algorithm 19.9 (reduce). Combine elements (iterative).
        /// - Work Θ(|a|), Span Θ(|a|).
        fn reduce_iter<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> T
            where T: Clone
            requires forall|x: &T, y: &T| #[trigger] f.requires((x, y));

        /// - Algorithm 19.9 (reduce). reduce f id a = if |a|=0 then id else if |a|=1 then a[0] else f(reduce f id b, reduce f id c).
        /// - Work Θ(|a|), Span Θ(lg |a|).
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> T
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                forall|x: &T, y: &T| #[trigger] f.requires((x, y));

        /// - Algorithm 19.10 (scan). Prefix-reduce returning partial sums and total.
        /// - Work Θ(|a|), Span Θ(|a|).
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> (scanned: (ArraySeqStEphS<T>, T))
            where T: Clone
            requires forall|x: &T, y: &T| #[trigger] f.requires((x, y))
            ensures scanned.0.seq@.len() == a.seq@.len();

        /// - Algorithm 19.3 (map). map f a = tabulate (lambda i.f(a[i])) |a|.
        /// - Work Θ(|a|), Span Θ(1).
        fn map<U: Clone, F: Fn(&T) -> U>(a: &ArraySeqStEphS<T>, f: &F) -> (mapped: ArraySeqStEphS<U>)
            requires
                forall|i: int| 0 <= i < a.seq@.len() ==> #[trigger] f.requires((&a.seq@[i],)),
            ensures
                mapped.seq@.len() == a.seq@.len(),
                forall|i: int| #![trigger mapped.seq@[i]] 0 <= i < a.seq@.len() ==> f.ensures((&a.seq@[i],), mapped.seq@[i]);

        /// - Primitive: tabulate. Build a sequence by applying `f` to each index.
        /// - Work Θ(n), Span Θ(1).
        fn tabulate<F: Fn(usize) -> T>(f: &F, length: usize) -> (tab_seq: ArraySeqStEphS<T>)
            requires
                length <= usize::MAX,
                forall|i: usize| i < length ==> #[trigger] f.requires((i,)),
            ensures
                tab_seq.seq@.len() == length,
                forall|i: int| #![trigger tab_seq.seq@[i]] 0 <= i < length ==> f.ensures((i as usize,), tab_seq.seq@[i]);

        /// - Primitive: flatten. Concatenate a sequence of sequences.
        /// - Work Θ(Σ|a_i|), Span Θ(1).
        fn flatten(a: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> (flattened: ArraySeqStEphS<T>)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
            ensures
                flattened.seq@ =~= a.seq@.map_values(|inner: ArraySeqStEphS<T>| inner.seq@).flatten();

        /// - Algorithm 19.5 (deflate). deflate f x = if (f x) then ⟨x⟩ else ⟨⟩.
        /// - Work Θ(1), Span Θ(1).
        fn deflate<F: Fn(&T) -> bool>(pred: &F, x: &T) -> (deflated: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                pred.requires((x,)),
            ensures
                deflated.spec_len() <= 1,
                deflated.spec_len() == 1 ==> pred.ensures((x,), true) && deflated.spec_index(0) == *x,
                deflated.spec_len() == 0 ==> pred.ensures((x,), false);
    }


    //		9. impl Trait for Struct

    impl<T> ArraySeqStEphTrait<T> for ArraySeqStEphS<T> {
        open spec fn spec_len(&self) -> nat {
            self.seq@.len()
        }

        open spec fn spec_index(&self, i: int) -> T {
            self.seq[i]
        }

        fn new(length: usize, init_value: T) -> (new_seq: ArraySeqStEphS<T>)
            where T: Clone + Eq
        {
            let seq = std::vec::from_elem(init_value, length);
            ArraySeqStEphS { seq }
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

        fn subseq_copy(&self, start: usize, length: usize) -> (subseq: ArraySeqStEphS<T>)
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
            ArraySeqStEphS { seq }
        }

        fn subseq(a: &ArraySeqStEphS<T>, start: usize, length: usize) -> (subseq: ArraySeqStEphS<T>)
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
            ArraySeqStEphS { seq }
        }

        fn from_vec(elts: Vec<T>) -> (seq: ArraySeqStEphS<T>) {
            ArraySeqStEphS { seq: elts }
        }

        // Algorithm 19.1: empty = tabulate (lambda i.i) 0.
        // The closure is never called (length 0) but must return T.
        // Rust has no value of unbounded generic T, so the body is an
        // unreachable loop; `requires false` makes this dead code.
        fn empty() -> (empty_seq: ArraySeqStEphS<T>) {
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
        fn singleton(item: T) -> (singleton: ArraySeqStEphS<T>)
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
        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> (appended: ArraySeqStEphS<T>)
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

        // Algorithm 19.5: filter f a = let b = map (deflate f) a in flatten b end.
        fn filter<F: Fn(&T) -> bool>(a: &ArraySeqStEphS<T>, pred: &F, Ghost(spec_pred): Ghost<spec_fn(T) -> bool>) -> (filtered: ArraySeqStEphS<T>)
            where T: Clone + Eq
        {
            // map (deflate pred) a — produces ArraySeqStEphS<ArraySeqStEphS<T>>
            let deflated = Self::map(
                    a,
                    &(|x: &T| -> (d: ArraySeqStEphS<T>)
                        requires
                            obeys_feq_clone::<T>(),
                            pred.requires((x,)),
                        ensures
                            d.seq@.len() <= 1,
                            d.seq@.len() == 1 ==> pred.ensures((x,), true) && d.seq@[0] == *x,
                            d.seq@.len() == 0 ==> pred.ensures((x,), false),
                    {
                        Self::deflate(pred, x)
                    }),
                );
            let filtered = Self::flatten(&deflated);
            proof {
                let ghost ss = deflated.seq@.map_values(|inner: ArraySeqStEphS<T>| inner.seq@);
                assert(ss.len() == a.seq@.len());
                assert(forall|i: int| #![trigger ss[i]] 0 <= i < ss.len() ==> ss[i].len() <= 1);
                lemma_flatten_bounded_by_outer_len::<T>(ss);

                // Every element in every inner seq satisfies pred.
                let ghost p = |x: T| pred.ensures((&x,), true);
                assert forall|j: int, k: int| #![trigger ss[j][k]]
                    0 <= j < ss.len() && 0 <= k < ss[j].len()
                    implies p(ss[j][k])
                by {
                    assert(ss[j] =~= deflated.seq@[j].seq@);
                    assert(deflated.seq@[j].seq@.len() <= 1);
                    assert(deflated.seq@[j].seq@.len() == 1);
                    assert(deflated.seq@[j].seq@[0] == a.seq@[j]);
                };
                lemma_flatten_all_satisfy::<T>(ss, p);

                // Connect flatten length to spec_filter_len via the 0-or-1 lemma.
                let ghost s_view = Seq::new(a.seq@.len(), |i: int| a.seq@[i]);
                assert(s_view =~= a.seq@);
                assert forall|i: int| #![trigger ss[i]] 0 <= i < a.seq@.len()
                    implies (ss[i].len() == 1 <==> spec_pred(a.seq@[i]))
                by {
                    assert(ss[i] =~= deflated.seq@[i].seq@);
                };
                lemma_flatten_01_eq_spec_filter_len(a.seq@, ss, spec_pred);

                // Connect flatten multiset to input multiset filtered by spec_pred.
                assert forall|i: int| #![trigger ss[i]]
                    0 <= i < a.seq@.len() && ss[i].len() == 1
                    implies ss[i][0] == a.seq@[i]
                by {
                    assert(ss[i] =~= deflated.seq@[i].seq@);
                };
                lemma_flatten_01_multiset_eq_filter(a.seq@, ss, spec_pred);

                assert(filtered.seq@ =~= Seq::new(filtered.spec_len(), |i: int| filtered.spec_index(i)));
                assert(a.seq@ =~= Seq::new(a.seq@.len(), |i: int| a.seq@[i]));
            }
            filtered
        }

        // Algorithm 19.6 (ephemeral): update via clone + set.
        // Clone the source (O(n)), then set at index (O(1)).
        fn update(a: &ArraySeqStEphS<T>, index: usize, item: T) -> (updated: ArraySeqStEphS<T>)
            where T: Clone + Eq
        {
            let mut seq = a.seq.clone();
            seq.set(index, item);
            ArraySeqStEphS { seq }
        }

        // Algorithm 19.7: isEmpty a = (|a| = 0).
        fn is_empty(&self) -> (empty: bool) {
            self.seq.len() == 0
        }

        // Algorithm 19.7: isSingleton a = (|a| = 1).
        fn is_singleton(&self) -> (single: bool) {
            self.seq.len() == 1
        }

        // Algorithm 19.8: iterate f x a (iterative form for single-threaded).
        fn iterate_iter<A, F: Fn(&A, &T) -> A>(a: &ArraySeqStEphS<T>, f: &F, seed: A) -> (acc: A) {
            let len = a.seq.len();
            let mut acc = seed;
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len == a.seq@.len(),
                    forall|x: &A, y: &T| #[trigger] f.requires((x, y)),
                decreases len - i,
            {
                acc = f(&acc, &a.seq[i]);
                i += 1;
            }
            acc
        }

        // Algorithm 19.8: iterate f x a = if |a|=0 then x else iterate f (f(x,a[0])) a[1..|a|-1].
        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqStEphS<T>, f: &F, seed: A) -> A
            where T: Clone + Eq
            decreases a.seq@.len(),
        {
            let len = a.seq.len();
            if len == 0 {
                seed
            } else {
                let new_seed = f(&seed, &a.seq[0]);
                let tail = Self::subseq(a, 1, len - 1);
                Self::iterate(&tail, f, new_seed)
            }
        }

        // Algorithm 19.9: reduce f id a (iterative form for single-threaded).
        fn reduce_iter<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> (reduced: T)
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
                decreases len - i,
            {
                acc = f(&acc, &a.seq[i]);
                i += 1;
            }
            acc
        }

        // Algorithm 19.9: reduce f id a = if |a|=0 then id; |a|=1 then a[0]; else f(reduce f id b, reduce f id c).
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> T
            where T: Clone + Eq
            decreases a.seq@.len(),
        {
            let len = a.seq.len();
            if len == 0 {
                id
            } else if len == 1 {
                a.seq[0].clone()
            } else {
                let mid = len / 2;
                let b = Self::subseq(a, 0, mid);
                let c = Self::subseq(a, mid, len - mid);
                let rb = Self::reduce(&b, f, id.clone());
                let rc = Self::reduce(&c, f, id);
                f(&rb, &rc)
            }
        }

        // Algorithm 19.10: scan f id a (iterative form for single-threaded).
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> (scanned: (ArraySeqStEphS<T>, T))
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
                    forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                decreases len - i,
            {
                acc = f(&acc, &a.seq[i]);
                seq.push(acc.clone());
                i += 1;
            }
            (ArraySeqStEphS { seq }, acc)
        }

        // Algorithm 19.3: map f a = tabulate (lambda i.f(a[i])) |a|.
        fn map<U: Clone, F: Fn(&T) -> U>(a: &ArraySeqStEphS<T>, f: &F) -> (mapped: ArraySeqStEphS<U>)
        {
            let n = a.seq.len();
            <ArraySeqStEphS<U> as ArraySeqStEphTrait<U>>::tabulate(
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

        // Primitive: tabulate.
        fn tabulate<F: Fn(usize) -> T>(f: &F, length: usize) -> (tab_seq: ArraySeqStEphS<T>)
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
            ArraySeqStEphS { seq }
        }

        // Primitive: flatten.
        fn flatten(a: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> (flattened: ArraySeqStEphS<T>)
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
                    seq@ =~= a.seq@.take(i as int).map_values(|inner: ArraySeqStEphS<T>| inner.seq@).flatten(),
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
                        seq@ =~= a.seq@.take(i as int).map_values(|inner: ArraySeqStEphS<T>| inner.seq@).flatten()
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
                    let ghost prefix = a.seq@.take(i as int).map_values(|inner: ArraySeqStEphS<T>| inner.seq@);
                    assert(a.seq@.take(i as int + 1).map_values(|inner: ArraySeqStEphS<T>| inner.seq@)
                        =~= prefix.push(a.seq@[i as int].seq@));
                    prefix.lemma_flatten_push(a.seq@[i as int].seq@);
                }
                i += 1;
            }
            proof {
                assert(a.seq@.take(outer_len as int) =~= a.seq@);
            }
            ArraySeqStEphS { seq }
        }

        // Algorithm 19.5: deflate f x = if (f x) then ⟨x⟩ else ⟨⟩.
        fn deflate<F: Fn(&T) -> bool>(pred: &F, x: &T) -> (deflated: ArraySeqStEphS<T>)
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

    //		9. bare impl (lemmas and iterators not in any trait)

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

    impl<T> ArraySeqStEphS<T> {
        broadcast proof fn lemma_spec_index(&self, i: int)
            requires 0 <= i < self.spec_len()
            ensures #[trigger] self.seq@[i] == self.spec_index(i)
        {}

        /// Returns an iterator over the sequence elements.
        pub fn iter(&self) -> (it: ArraySeqStEphIter<'_, T>)
            ensures
                it@.0 == 0,
                it@.1 == self.seq@,
                iter_invariant(&it),
        {
            ArraySeqStEphIter { inner: self.seq.iter() }
        }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: View + PartialEq> PartialEqSpecImpl for ArraySeqStEphS<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }


    //		10. iterators

    /// Iterator wrapper with closed spec view for encapsulation.
    #[verifier::reject_recursive_types(T)]
    pub struct ArraySeqStEphIter<'a, T> {
        inner: std::slice::Iter<'a, T>,
    }

    impl<'a, T> View for ArraySeqStEphIter<'a, T> {
        type V = (int, Seq<T>);
        closed spec fn view(&self) -> (int, Seq<T>) { self.inner@ }
    }

    pub open spec fn iter_invariant<'a, T>(it: &ArraySeqStEphIter<'a, T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<'a, T> std::iter::Iterator for ArraySeqStEphIter<'a, T> {
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
    pub struct ArraySeqStEphGhostIterator<'a, T> {
        pub pos: int,
        pub elements: Seq<T>,
        pub phantom: core::marker::PhantomData<&'a T>,
    }

    impl<'a, T> View for ArraySeqStEphGhostIterator<'a, T> {
        type V = Seq<T>;

        open spec fn view(&self) -> Seq<T> {
            self.elements.take(self.pos)
        }
    }

    impl<'a, T> vstd::pervasive::ForLoopGhostIteratorNew for ArraySeqStEphIter<'a, T> {
        type GhostIter = ArraySeqStEphGhostIterator<'a, T>;
        open spec fn ghost_iter(&self) -> ArraySeqStEphGhostIterator<'a, T> {
            ArraySeqStEphGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    impl<'a, T> vstd::pervasive::ForLoopGhostIterator for ArraySeqStEphGhostIterator<'a, T> {
        type ExecIter = ArraySeqStEphIter<'a, T>;
        type Item = T;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &ArraySeqStEphIter<'a, T>) -> bool {
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

        open spec fn ghost_advance(&self, _exec_iter: &ArraySeqStEphIter<'a, T>) -> ArraySeqStEphGhostIterator<'a, T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, T> std::iter::IntoIterator for &'a ArraySeqStEphS<T> {
        type Item = &'a T;
        type IntoIter = ArraySeqStEphIter<'a, T>;
        fn into_iter(self) -> Self::IntoIter { ArraySeqStEphIter { inner: self.seq.iter() } }
    }

    impl<T> std::iter::IntoIterator for ArraySeqStEphS<T> {
        type Item = T;
        type IntoIter = IntoIter<T>;
        fn into_iter(self) -> Self::IntoIter { self.seq.into_iter() }
    }


    //		11. derive impls in verus!

    impl<T: Clone> Clone for ArraySeqStEphS<T> {
        fn clone(&self) -> Self {
            ArraySeqStEphS { seq: self.seq.clone() }
        }
    }

    impl<T: Eq + View> Eq for ArraySeqStEphS<T> {}

    impl<T: PartialEq + View> PartialEq for ArraySeqStEphS<T> {
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

    impl<T: Debug> Debug for ArraySeqStEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_list().entries(self.seq.iter()).finish()
        }
    }

    impl<T: Display> Display for ArraySeqStEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "[")?;
            for (i, item) in self.seq.iter().enumerate() {
                if i > 0 { write!(f, ", ")?; }
                write!(f, "{item}")?;
            }
            write!(f, "]")
        }
    }

}
