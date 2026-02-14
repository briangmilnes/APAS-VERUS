//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Single-threaded ephemeral array sequence (mutable) implementation. Verusified.

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
    use vstd::std_specs::cmp::PartialEqSpecImpl;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::vec::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::clone::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;
    use crate::vstdplus::multiset::multiset::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap18::ArraySeq::ArraySeq::{spec_iterate, spec_monoid};

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

    /// - Base trait for single-threaded ephemeral array sequences (Chapter 18).
    /// - These methods are never redefined in later chapters.
    pub trait ArraySeqStEphBaseTrait<T>: Sized {
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

        /// - Set the element at `index` to `item` in place.
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
    }

    /// Redefinable trait - may be overridden with better algorithms in later chapters.
    pub trait ArraySeqStEphRedefinableTrait<T>: ArraySeqStEphBaseTrait<T> {

        /// - Definition 18.1 (empty). Construct the empty sequence.
        /// - Work Θ(1), Span Θ(1).
        fn empty() -> (empty_seq: Self)
            ensures empty_seq.spec_len() == 0;

        /// - Definition 18.1 (singleton). Construct a singleton sequence containing `item`.
        /// - Work Θ(1), Span Θ(1).
        fn singleton(item: T) -> (singleton: Self)
            ensures
                singleton.spec_len() == 1,
                singleton.spec_index(0) == item;

        /// - Definition 18.13 (append). Concatenate two sequences.
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

        /// - Definition 18.14 (filter). Keep elements satisfying `pred`.
        /// - Work Θ(|a|), Span Θ(1).
        /// - The multiset postcondition captures predicate satisfaction, provenance,
        ///   and completeness in a single statement.
        fn filter<F: Fn(&T) -> bool>(a: &ArraySeqStEphS<T>, pred: &F, Ghost(spec_pred): Ghost<spec_fn(T) -> bool>) -> (filtered: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                forall|i: int| 0 <= i < a.seq@.len() ==> #[trigger] pred.requires((&a.seq@[i],)),
                // The biconditional bridge ties the exec closure to the spec predicate.
                forall|v: T, ret: bool| pred.ensures((&v,), ret) <==> spec_pred(v) == ret,
            ensures
                filtered.spec_len() <= a.seq@.len(),
                filtered.spec_len() == spec_filter_len(
                    Seq::new(a.seq@.len(), |i: int| a.seq@[i]), spec_pred),
                // The result multiset equals the input multiset filtered by the spec predicate.
                Seq::new(filtered.spec_len(), |i: int| filtered.spec_index(i)).to_multiset()
                    =~= Seq::new(a.seq@.len(), |i: int| a.seq@[i]).to_multiset().filter(spec_pred),
                forall|i: int| #![trigger filtered.spec_index(i)] 0 <= i < filtered.spec_len() ==> pred.ensures((&filtered.spec_index(i),), true);

        /// - Definition 18.16 (update). Return a copy with the index replaced by the new value.
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

        /// - Definition 18.5 (isEmpty). true iff the sequence has length zero.
        /// - Work Θ(1), Span Θ(1).
        fn is_empty(&self) -> (empty: bool)
            ensures empty <==> self.spec_len() == 0;

        /// - Definition 18.5 (isSingleton). true iff the sequence has length one.
        /// - Work Θ(1), Span Θ(1).
        fn is_singleton(&self) -> (single: bool)
            ensures single <==> self.spec_len() == 1;

        /// - Definition 18.7 (iterate). Fold with accumulator `seed`.
        /// - Work Θ(|a|), Span Θ(1).
        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqStEphS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(A, T) -> A>, seed: A) -> (result: A)
            requires
                forall|x: &A, y: &T| #[trigger] f.requires((x, y)),
                forall|a: A, t: T, ret: A| f.ensures((&a, &t), ret) <==> ret == spec_f(a, t),
            ensures
                result == spec_iterate(Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_f, seed);

        /// - Definition 18.18 (reduce). Combine elements using associative `f` and identity `id`.
        /// - Work Θ(|a|), Span Θ(1).
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(T, T) -> T>, id: T) -> (result: T)
            where T: Clone
            requires
                spec_monoid(spec_f, id),
                forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) <==> ret == spec_f(x, y),
            ensures
                result == spec_iterate(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_f, id);

        /// - Definition 18.19 (scan). Prefix-reduce returning inclusive prefix sums and total.
        /// - Work Θ(|a|), Span Θ(1).
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(T, T) -> T>, id: T) -> (scanned: (ArraySeqStEphS<T>, T))
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
        /// - Work Θ(|a|), Span Θ(1).
        fn map<U: Clone, F: Fn(&T) -> U>(a: &ArraySeqStEphS<T>, f: &F) -> (mapped: ArraySeqStEphS<U>)
            requires
                forall|i: int| 0 <= i < a.seq@.len() ==> #[trigger] f.requires((&a.seq@[i],)),
            ensures
                mapped.seq@.len() == a.seq@.len(),
                forall|i: int| #![trigger mapped.seq@[i]] 0 <= i < a.seq@.len() ==> f.ensures((&a.seq@[i],), mapped.seq@[i]);

        /// - Algorithm 18.3 (tabulate). Build a sequence by applying `f` to each index.
        /// - Work Θ(n), Span Θ(1).
        fn tabulate<F: Fn(usize) -> T>(f: &F, length: usize) -> (tab_seq: ArraySeqStEphS<T>)
            requires
                length <= usize::MAX,
                forall|i: usize| i < length ==> #[trigger] f.requires((i,)),
            ensures
                tab_seq.seq@.len() == length,
                forall|i: int| #![trigger tab_seq.seq@[i]] 0 <= i < length ==> f.ensures((i as usize,), tab_seq.seq@[i]);

        /// - Definition 18.15 (flatten). Concatenate a sequence of sequences.
        /// - Work Θ(Σ|a_i|), Span Θ(1).
        fn flatten(a: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> (flattened: ArraySeqStEphS<T>)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
            ensures
                flattened.seq@ =~= a.seq@.map_values(|inner: ArraySeqStEphS<T>| inner.seq@).flatten();
    }


    //		9. impl BaseTrait for Struct

    impl<T> ArraySeqStEphBaseTrait<T> for ArraySeqStEphS<T> {
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
    }

    //		9. impl RedefinableTrait for Struct

    impl<T> ArraySeqStEphRedefinableTrait<T> for ArraySeqStEphS<T> {
        fn empty() -> (empty_seq: ArraySeqStEphS<T>) {
            ArraySeqStEphS { seq: Vec::new() }
        }

        fn singleton(item: T) -> (singleton: ArraySeqStEphS<T>) {
            let mut seq = Vec::with_capacity(1);
            seq.push(item);
            ArraySeqStEphS { seq }
        }

        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> (appended: ArraySeqStEphS<T>)
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
            ArraySeqStEphS { seq }
        }

        fn filter<F: Fn(&T) -> bool>(a: &ArraySeqStEphS<T>, pred: &F, Ghost(spec_pred): Ghost<spec_fn(T) -> bool>) -> (filtered: ArraySeqStEphS<T>)
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
                    forall|v: T, ret: bool| pred.ensures((&v,), ret) <==> spec_pred(v) == ret,
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
            let filtered = ArraySeqStEphS { seq };
            proof {
                assert(a.seq@.subrange(0, a.seq@.len() as int) =~= a.seq@);
                assert(filtered.seq@ =~= Seq::new(filtered.spec_len(), |i: int| filtered.spec_index(i)));
                assert(a.seq@ =~= Seq::new(a.seq@.len(), |i: int| a.seq@[i]));
            }
            filtered
        }

        fn update(a: &ArraySeqStEphS<T>, index: usize, item: T) -> (updated: ArraySeqStEphS<T>)
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
            ArraySeqStEphS { seq }
        }

        fn is_empty(&self) -> (empty: bool) {
            self.seq.len() == 0
        }

        fn is_singleton(&self) -> (single: bool) {
            self.seq.len() == 1
        }

        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqStEphS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(A, T) -> A>, seed: A) -> (acc: A) {
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

        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(T, T) -> T>, id: T) -> (reduced: T)
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

        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(T, T) -> T>, id: T) -> (scanned: (ArraySeqStEphS<T>, T))
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
            let scanned_seq = ArraySeqStEphS { seq };
            proof {
                assert forall|i: int| #![trigger scanned_seq.spec_index(i)] 0 <= i < a.spec_len() implies
                    scanned_seq.spec_index(i) == s.take(i + 1).fold_left(id, spec_f)
                by {
                    assert(scanned_seq.spec_index(i) == seq@[i]);
                }
            }
            (scanned_seq, acc)
        }

        fn map<U: Clone, F: Fn(&T) -> U>(a: &ArraySeqStEphS<T>, f: &F) -> (mapped: ArraySeqStEphS<U>)
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
            ArraySeqStEphS { seq }
        }

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
    }

    //		9. bare impl (lemmas and iterators not in any trait)

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

    /// Ghost iterator for ForLoopGhostIterator support.
    #[verifier::reject_recursive_types(T)]
    pub struct ArraySeqStEphGhostIterator<'a, T> {
        pub pos: int,
        pub elements: Seq<T>,
        pub phantom: core::marker::PhantomData<&'a T>,
    }

    impl<'a, T> View for ArraySeqStEphGhostIterator<'a, T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.elements.take(self.pos) }
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
        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                it@.0 == 0,
                it@.1 == self.seq@,
                iter_invariant(&it),
        {
            ArraySeqStEphIter { inner: self.seq.iter() }
        }
    }

    impl<T> std::iter::IntoIterator for ArraySeqStEphS<T> {
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

    impl<T: Clone> Clone for ArraySeqStEphS<T> {
        fn clone(&self) -> Self { ArraySeqStEphS { seq: self.seq.clone() } }
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
