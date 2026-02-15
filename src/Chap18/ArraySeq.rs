//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! The simplest possible version, ignoring parallelism. Verusified.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	4. type definitions
//	5. view impls
//	6. spec fns
//	8. traits
//	9. impls
//	10. iterators
//	11. derive impls in verus!
//	13. derive impls outside verus!

//		1. module

pub mod ArraySeq {

    use std::fmt::{Debug, Display, Formatter};
    use std::fmt::Result as FmtResult;
    use std::slice::{Iter, IterMut};
    use std::vec::IntoIter;

    use vstd::prelude::*;

    verus! {

    //		2. imports

    #[cfg(verus_keep_ghost)]
    use {
        vstd::std_specs::cmp::PartialEqSpecImpl,
        vstd::std_specs::cmp::PartialEqSpec,
        vstd::std_specs::vec::*,
        vstd::std_specs::clone::*,
        vstd::laws_eq::obeys_concrete_eq,
        vstd::laws_eq::obeys_deep_eq,
    };

    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;
    use crate::vstdplus::multiset::multiset::*;
    #[cfg(verus_keep_ghost)]
    use vstd::relations::associative;

    //		3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
        vstd::seq_lib::group_seq_properties,
        vstd::multiset::group_multiset_axioms,
        vstd::multiset::group_multiset_properties,
        vstd::seq_lib::group_to_multiset_ensures,
        vstd::seq_lib::group_filter_ensures,
        crate::vstdplus::feq::feq::group_feq_axioms,
    };


    //		4. type definitions

    #[verifier::reject_recursive_types(T)]
    pub struct ArraySeqS<T> {
        pub seq: Vec<T>,
    }


    //		5. view impls

    impl<T: View> View for ArraySeqS<T> {
        type V = Seq<T::V>;

        open spec fn view(&self) -> Seq<T::V> {
            self.seq@.map(|_i: int, t: T| t@)
        }
    }

    impl<T: DeepView> DeepView for ArraySeqS<T> {
        type V = Seq<T::V>;

        open spec fn deep_view(&self) -> Seq<T::V> {
            let v = self.seq@;
            Seq::new(v.len(), |i: int| v[i].deep_view())
        }
    }


    //		6. spec fns

    /// Definition 18.7 (iterate). Left fold over a sequence: applies f to the accumulator
    /// and each element from left to right.  spec_iterate(s, f, start_x) = f(...f(f(start_x, s[0]), s[1])..., s[n-1]).
    pub open spec fn spec_iterate<A, T>(s: Seq<T>, f: spec_fn(A, T) -> A, start_x: A) -> A {
        s.fold_left(start_x, f)
    }

    // spec_associative: use vstd::relations::associative instead.

    /// The value id is a left identity for f: f(id, x) == x for all x.
    pub open spec fn spec_left_identity<T>(f: spec_fn(T, T) -> T, id: T) -> bool {
        forall|x: T| #[trigger] f(id, x) == x
    }

    /// The value id is a right identity for f: f(x, id) == x for all x.
    pub open spec fn spec_right_identity<T>(f: spec_fn(T, T) -> T, id: T) -> bool {
        forall|x: T| #[trigger] f(x, id) == x
    }

    /// The triple (T, f, id) forms a monoid: f is associative and id is a two-sided identity.
    pub open spec fn spec_monoid<T>(f: spec_fn(T, T) -> T, id: T) -> bool {
        associative(f) && spec_left_identity(f, id) && spec_right_identity(f, id)
    }

    /// Definition 18.16 (inject). Apply position-value updates left to right; the first update
    /// to each position wins.  Out-of-range positions are ignored.
    pub open spec fn spec_inject<T>(s: Seq<T>, updates: Seq<(usize, T)>) -> Seq<T>
        decreases updates.len()
    {
        if updates.len() == 0 {
            s
        } else {
            // Process the tail first, then apply the head on top so the
            // leftmost (first) update to any position overwrites later ones.
            let rest = spec_inject(s, updates.drop_first());
            let pos = updates[0].0 as int;
            let val = updates[0].1;
            if 0 <= pos < s.len() { rest.update(pos, val) } else { rest }
        }
    }

    /// Definition 18.19 (iteratePrefixes). Return the exclusive prefix folds and the total.
    /// The sequence contains the accumulator state before processing each element:
    /// prefixes[i] = fold_left(x, f, a[0..i]).  The second component is the final fold.
    pub open spec fn spec_iterate_prefixes<A, T>(s: Seq<T>, f: spec_fn(A, T) -> A, x: A) -> (Seq<A>, A) {
        (Seq::new(s.len(), |i: int| s.take(i).fold_left(x, f)), s.fold_left(x, f))
    }

    /// Find the index of the first group whose key matches `k`, or None.
    pub open spec fn spec_find_key_index<K, V>(groups: Seq<(K, Seq<V>)>, k: K) -> Option<int>
        decreases groups.len()
    {
        if groups.len() == 0 {
            None
        } else if groups[0].0 == k {
            Some(0)
        } else {
            match spec_find_key_index(groups.skip(1), k) {
                Some(i) => Some(i + 1),
                None => None,
            }
        }
    }

    /// Definition 18.18 (collect). Group key-value pairs by key, preserving value order.
    /// Processes pairs left to right: if the key already has a group, append the value;
    /// otherwise create a new group. Keys appear in first-occurrence order.
    pub open spec fn spec_collect<K, V>(pairs: Seq<(K, V)>) -> Seq<(K, Seq<V>)>
        decreases pairs.len()
    {
        if pairs.len() == 0 {
            Seq::empty()
        } else {
            let rest = spec_collect(pairs.drop_last());
            let k = pairs.last().0;
            let v = pairs.last().1;
            match spec_find_key_index(rest, k) {
                Some(i) => rest.update(i, (k, rest[i].1.push(v))),
                None => rest.push((k, seq![v])),
            }
        }
    }

    pub open spec fn obeys_spec_eq<T: PartialEq>() -> bool {
        forall|x: T, y: T| x.eq_spec(&y) <==> x == y
    }


    //		8. traits

    /// Data Type 18.1: Generic sequence trait for array-backed sequences.
    pub trait ArraySeqTrait<T: View>: Sized {
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

        /// - Definition 18.13 (append). Concatenate two sequences.
        /// - Work Θ(|a| + |b|), Span Θ(1).
        fn append(a: &Self, b: &Self) -> (appended: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                a.spec_len() + b.spec_len() <= usize::MAX as int,
            ensures
                appended.spec_len() == a.spec_len() + b.spec_len(),
                forall|i: int| #![trigger appended.spec_index(i)] 0 <= i < a.spec_len() ==> appended.spec_index(i) == a.spec_index(i),
                forall|i: int| #![trigger b.spec_index(i)] 0 <= i < b.spec_len() ==> appended.spec_index(a.spec_len() + i) == b.spec_index(i);

        /// - Definition 18.14 (filter). Keep elements satisfying `pred`.
        /// - Work Θ(|a|), Span Θ(1).
        /// - The multiset postcondition captures predicate satisfaction, provenance,
        ///   and completeness in a single statement.
        fn filter<F: Fn(&T) -> bool>(a: &Self, pred: &F, Ghost(spec_pred): Ghost<spec_fn(T) -> bool>) -> (filtered: Self)
            where T: Clone + Eq
            requires 
              obeys_feq_clone::<T>(),
              forall|i: int| 0 <= i < a.spec_len() ==> #[trigger] pred.requires((&a.spec_index(i),)),
              // The forward bridge ties the exec closure to the spec predicate.
              forall|v: T, keep: bool| pred.ensures((&v,), keep) ==> spec_pred(v) == keep,
            ensures
                filtered.spec_len() <= a.spec_len(),
                // The result length equals the spec_filter_len count.
                filtered.spec_len() == spec_filter_len(Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_pred),
                // The result multiset equals the input multiset filtered by the spec predicate.
                Seq::new(filtered.spec_len(), |i: int| filtered.spec_index(i)).to_multiset()
                    =~= Seq::new(a.spec_len(), |i: int| a.spec_index(i)).to_multiset().filter(spec_pred),
                // Every element of the filtered result satisfies the predicate.
                forall|i: int| #![trigger filtered.spec_index(i)] 0 <= i < filtered.spec_len() ==> pred.ensures((&filtered.spec_index(i),), true);

        /// - Definition 18.16 (update). Return a copy with the index replaced by the new value.
        /// - Work Θ(|a|), Span Θ(1).
        fn update(a: &Self, index: usize, item: T) -> (updated: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                index < a.spec_len(),
            ensures
                updated.spec_len() == a.spec_len(),
                updated.spec_index(index as int) == item,
                forall|i: int| #![trigger updated.spec_index(i)] 0 <= i < a.spec_len() && i != index as int ==> updated.spec_index(i) == a.spec_index(i);

        /// - Definition 18.5 (isEmpty). true iff the sequence has length zero.
        /// - Work Θ(1), Span Θ(1).
        fn is_empty(&self) -> (empty: bool)
            ensures empty <==> self.spec_len() == 0;

        /// - Definition 18.5 (isSingleton). true iff the sequence has length one.
        /// - Work Θ(1), Span Θ(1).
        fn is_singleton(&self) -> (single: bool)
            ensures single <==> self.spec_len() == 1;

        /// - Definition 18.7 (iterate). Left fold with accumulator `start_x`.
        /// - The Rust equivalent is `Iterator::fold`.
        /// - Work Θ(|a|), Span Θ(1).
        fn iterate<A, F: Fn(&A, &T) -> A>(a: &Self, f: &F, Ghost(spec_f): Ghost<spec_fn(A, T) -> A>, start_x: A) -> (result: A)
            requires
                forall|x: &A, y: &T| #[trigger] f.requires((x, y)),
                forall|a: A, t: T, ret: A| f.ensures((&a, &t), ret) <==> ret == spec_f(a, t),
            ensures
                result == spec_iterate(Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_f, start_x);

        /// - Definition 18.18 (reduce). Combine elements using associative function and identity `id`.
        /// - The Rust equivalent is `Iterator::fold` with the accumulator type equal to the element type.
        /// - Work Θ(|a|), Span Θ(1).
        fn reduce<F: Fn(&T, &T) -> T>(a: &Self, f: &F, Ghost(spec_f): Ghost<spec_fn(T, T) -> T>, id: T) -> (result: T)
            where T: Clone
            requires
                spec_monoid(spec_f, id),
                forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) <==> ret == spec_f(x, y),
            ensures
                result == spec_iterate(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)), spec_f, id);

        /// - Definition 18.19 (scan). Prefix-reduce returning inclusive prefix sums and total.
        /// - The Rust equivalent is `Iterator::scan`, which produces similar intermediate state.
        /// - Work Θ(|a|), Span Θ(1).
        fn scan<F: Fn(&T, &T) -> T>(a: &Self, f: &F, Ghost(spec_f): Ghost<spec_fn(T, T) -> T>, id: T) -> (scanned: (Self, T))
            where T: Clone + Eq
            requires
                spec_monoid(spec_f, id),
                obeys_feq_clone::<T>(),
                forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) <==> ret == spec_f(x, y),
            ensures
                scanned.0.spec_len() == a.spec_len(),
                // Each element is the fold of the first i+1 input elements (inclusive prefix sum).
                forall|i: int| #![trigger scanned.0.spec_index(i)] 0 <= i < a.spec_len() ==>
                    scanned.0.spec_index(i) == Seq::new(a.spec_len(), |j: int| a.spec_index(j)).take(i + 1).fold_left(id, spec_f),
                // The total is the fold of the entire sequence.
                scanned.1 == spec_iterate(
                    Seq::new(a.spec_len(), |j: int| a.spec_index(j)), spec_f, id);

        /// - Definition 18.16 (inject). Update multiple positions at once; the first update in
        ///   the ordering of `updates` takes effect when positions collide.
        /// - Work Θ(|a| + |updates|), Span Θ(1).
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

        /// - Definition 18.19 (scanI). Inclusive prefix-reduce: scanI[i] = reduce f id a[0..i].
        /// - Our `scan` currently computes inclusive prefixes; this function makes the intent explicit.
        /// - Work Θ(|a|), Span Θ(1).
        fn scan_inclusive<F: Fn(&T, &T) -> T>(a: &Self, f: &F, Ghost(spec_f): Ghost<spec_fn(T, T) -> T>, id: T) -> (result: Self)
            where T: Clone + Eq
            requires
                spec_monoid(spec_f, id),
                obeys_feq_clone::<T>(),
                forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) <==> ret == spec_f(x, y),
            ensures
                result.spec_len() == a.spec_len(),
                forall|i: int| #![trigger result.spec_index(i)] 0 <= i < a.spec_len() ==>
                    result.spec_index(i) == Seq::new(a.spec_len(), |j: int| a.spec_index(j)).take(i + 1).fold_left(id, spec_f);

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

        /// - Remove the element at `index`, shifting subsequent elements left.
        /// - Work Θ(|self|), Span Θ(1).
        fn remove(&mut self, index: usize) -> (element: T)
            requires
                index < old(self).spec_len(),
            ensures
                element == old(self).spec_index(index as int),
                self.spec_len() == old(self).spec_len() - 1,
                forall|j: int| #![trigger self.spec_index(j)] 0 <= j < index ==> self.spec_index(j) == old(self).spec_index(j),
                forall|j: int| #![trigger self.spec_index(j)] index <= j < self.spec_len() ==> self.spec_index(j) == old(self).spec_index(j + 1);

        /// - Insert `element` at `index`, shifting subsequent elements right.
        /// - Work Θ(|self|), Span Θ(1).
        fn insert(&mut self, index: usize, element: T)
            requires
                index <= old(self).spec_len(),
            ensures
                self.spec_len() == old(self).spec_len() + 1,
                self.spec_index(index as int) == element,
                forall|j: int| #![trigger self.spec_index(j)] 0 <= j < index ==> self.spec_index(j) == old(self).spec_index(j),
                forall|j: int| #![trigger self.spec_index(j)] index < j < self.spec_len() ==> self.spec_index(j) == old(self).spec_index(j - 1);

        /// - Create sequence from Vec.
        /// - Work Θ(n) worst case, Θ(1) best case, Span Θ(1).
        fn from_vec(elts: Vec<T>) -> (seq: Self)
            ensures
                seq.spec_len() == elts@.len(),
                forall|i: int| #![trigger seq.spec_index(i)] 0 <= i < elts@.len() ==> seq.spec_index(i) == elts@[i];


        fn find_key<K: View + Eq + PartialEq, V: View>(
            groups: &ArraySeqS<(K, ArraySeqS<V>)>,
            needle: &K,
        ) -> (found: Option<usize>)
            requires
              obeys_concrete_eq::<K>(),
            ensures
               match found {
                   Some(idx) => idx < groups.seq@.len()
                        && groups.seq@[idx as int].0 == *needle
                        && forall|m: int| #![trigger groups.seq@[m]] 0 <= m < idx as int ==> groups.seq@[m].0 != *needle,
                   None => forall|m: int| #![trigger groups.seq@[m]] 0 <= m < groups.seq@.len() ==> groups.seq@[m].0 != *needle,
               };

    /// Definition 18.18 (collect). Group key-value pairs by key, preserving value order.
    /// This is not Rust style iter().collect(), this is a SQL style collect with group_by.
    /// The Verus limitation of "index for &mut not supported" prevents
    /// groups[idx].1.push(v). So we remove the entry, mutate it, and re-insert.
    fn collect<K: DeepView<V = K> + View + Clone + Eq + PartialEq, V: DeepView<V = V> + View + Clone + Eq>(
        pairs: &ArraySeqS<(K, V)>,
    ) -> (collected: ArraySeqS<(K, ArraySeqS<V>)>)
        requires
            obeys_feq_clone::<K>(),
            obeys_feq_clone::<V>(),
            obeys_concrete_eq::<K>(),
            obeys_deep_eq::<K>(),
            obeys_deep_eq::<V>(),
            obeys_generic_deep_eq::<K>(),
            obeys_generic_deep_eq::<V>(),
        ensures
           collected.seq.deep_view() =~= spec_collect(pairs.seq@);
   }

    //		9. impls

    impl<T: View> ArraySeqTrait<T> for ArraySeqS<T> {
        open spec fn spec_len(&self) -> nat {
            self.seq@.len()
        }

        open spec fn spec_index(&self, i: int) -> T {
            self.seq[i]
        }

        fn new(length: usize, init_value: T) -> (new_seq: ArraySeqS<T>)
            where T: Clone + Eq
        {
            let seq = std::vec::from_elem(init_value, length);
            ArraySeqS { seq }
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

        fn empty() -> (empty_seq: ArraySeqS<T>) {
            ArraySeqS { seq: Vec::new() }
        }

        fn singleton(item: T) -> (singleton: ArraySeqS<T>) {
            let mut seq = Vec::with_capacity(1);
            seq.push(item);
            ArraySeqS { seq }
        }

        fn subseq(a: &ArraySeqS<T>, start: usize, length: usize) -> (subseq: ArraySeqS<T>)
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
            ArraySeqS { seq }
        }

        fn append(a: &ArraySeqS<T>, b: &ArraySeqS<T>) -> (appended: ArraySeqS<T>)
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
                    crate::vstdplus::feq::feq::axiom_cloned_implies_eq_owned(a.seq[i as int], last);
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
                    crate::vstdplus::feq::feq::axiom_cloned_implies_eq_owned(b.seq[j as int], last);
                }
                j += 1;
            }
            ArraySeqS { seq }
        }

        fn filter<F: Fn(&T) -> bool>(a: &ArraySeqS<T>, pred: &F, Ghost(spec_pred): Ghost<spec_fn(T) -> bool>) -> (filtered: ArraySeqS<T>)
            where T: Clone + Eq
        {
            let mut seq: Vec<T> = Vec::new();

            for i in 0..a.seq.len()
                invariant
                    obeys_feq_clone::<T>(),
                    forall|j: int| 0 <= j < a.spec_len() ==> #[trigger] pred.requires((&a.spec_index(j),)),
                    forall|v: T, keep: bool| pred.ensures((&v,), keep) ==> spec_pred(v) == keep,
                    i <= a.seq@.len(),
                    seq@.len() <= i,
                    forall|j: int| #![trigger seq@[j]] 0 <= j < seq@.len() ==> pred.ensures((&seq@[j],), true),
                    // The result length equals spec_filter_len over the prefix seen so far.
                    seq@.len() == spec_filter_len(a.seq@.subrange(0, i as int), spec_pred),
                    // The result multiset equals the filtered multiset of elements seen so far.
                    seq@.to_multiset() =~= a.seq@.subrange(0, i as int).to_multiset().filter(spec_pred),
            {
                proof {
                    broadcast use vstd::seq_lib::group_to_multiset_ensures;
                    a.lemma_spec_index(i as int);
                }
                // Extending the subrange by one element lets the multiset axioms advance the invariant.
                assert(a.seq@.subrange(0, i as int + 1) =~= a.seq@.subrange(0, i as int).push(a.seq@[i as int]));
                // spec_filter_len unfolds via drop_last: subrange(0,i+1).drop_last() == subrange(0,i)
                assert(a.seq@.subrange(0, i as int + 1).drop_last() =~= a.seq@.subrange(0, i as int));
                if pred(&a.seq[i]) {
                    let elem = a.seq[i].clone();
                    proof {
                        // The clone axiom ensures the cloned value equals the original in spec.
                        axiom_cloned_implies_eq_owned(a.seq[i as int], elem);
                    }
                    seq.push(elem);
                }
            }
            // The full subrange equals the original sequence.
            assert(a.seq@.subrange(0, a.seq.len() as int) =~= a.seq@);
            let filtered = ArraySeqS { seq };
            proof {
                // Bridge from the concrete seq@ to the abstract Seq::new(spec_len, spec_index).
                assert(filtered.seq@ =~= Seq::new(filtered.spec_len(), |i: int| filtered.spec_index(i)));
                assert(a.seq@ =~= Seq::new(a.spec_len(), |i: int| a.spec_index(i)));
            }
            filtered
        }

        fn update(a: &ArraySeqS<T>, index: usize, item: T) -> (updated: ArraySeqS<T>)
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
                        crate::vstdplus::feq::feq::axiom_cloned_implies_eq_owned(item, last);
                    }
                } else {
                    seq.push(a.seq[i].clone());
                    proof {
                        let ghost last = seq@[seq@.len() - 1 as int];
                        assert(cloned(a.seq[i as int], last));
                        crate::vstdplus::feq::feq::axiom_cloned_implies_eq_owned(a.seq[i as int], last);
                    }
                }
                i += 1;
            }
            ArraySeqS { seq }
        }

        fn is_empty(&self) -> (empty: bool) {
            self.seq.len() == 0
        }

        fn is_singleton(&self) -> (single: bool) {
            self.seq.len() == 1
        }

        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(A, T) -> A>, start_x: A) -> (acc: A) {
            let ghost s = Seq::new(a.spec_len(), |i: int| a.spec_index(i));
            let len = a.seq.len();
            let mut acc = start_x;
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len == a.seq@.len(),
                    forall|x: &A, y: &T| #[trigger] f.requires((x, y)),
                    forall|a: A, t: T, ret: A| f.ensures((&a, &t), ret) ==> ret == spec_f(a, t),
                    // The accumulator equals the fold of the first i elements.
                    s == Seq::new(a.spec_len(), |j: int| a.spec_index(j)),
                    acc == s.take(i as int).fold_left(start_x, spec_f),
                decreases len - i,
            {
                proof {
                    a.lemma_spec_index(i as int);
                    // take(i+1) == take(i).push(s[i]).
                    assert(s.take(i as int + 1) =~= s.take(i as int).push(s[i as int]));
                }
                acc = f(&acc, &a.seq[i]);
                proof {
                    // Help the solver unfold fold_left on take(i+1).
                    let ghost t = s.take(i as int + 1);
                    assert(t.len() > 0);
                    assert(t.drop_last() =~= s.take(i as int));
                    assert(t.last() == s[i as int]);
                    reveal(Seq::fold_left);
                }
                i += 1;
            }
            proof {
                // At loop exit, i == len so take(len) == s.
                assert(s.take(len as int) =~= s);
            }
            acc
        }

        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(T, T) -> T>, id: T) -> (result: T)
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
                    // The accumulator equals the fold of the first i elements.
                    s == Seq::new(a.spec_len(), |j: int| a.spec_index(j)),
                    acc == s.take(i as int).fold_left(id, spec_f),
                decreases len - i,
            {
                proof {
                    a.lemma_spec_index(i as int);
                    // take(i+1) == take(i).push(s[i]).
                    assert(s.take(i as int + 1) =~= s.take(i as int).push(s[i as int]));
                }
                acc = f(&acc, &a.seq[i]);
                proof {
                    // Help the solver unfold fold_left on take(i+1).
                    let ghost t = s.take(i as int + 1);
                    assert(t.len() > 0);
                    assert(t.drop_last() =~= s.take(i as int));
                    assert(t.last() == s[i as int]);
                    reveal(Seq::fold_left);
                }
                i += 1;
            }
            proof {
                // At loop exit, i == len so take(len) == s.
                assert(s.take(len as int) =~= s);
            }
            acc
        }

        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(T, T) -> T>, id: T) -> (scanned: (ArraySeqS<T>, T))
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
                    // The accumulator equals the fold of the first i elements.
                    s == Seq::new(a.spec_len(), |j: int| a.spec_index(j)),
                    acc == s.take(i as int).fold_left(id, spec_f),
                    // Each element written so far is the fold of the corresponding prefix.
                    forall|k: int| #![trigger seq@[k]] 0 <= k < seq@.len() ==>
                        seq@[k] == s.take(k + 1).fold_left(id, spec_f),
                decreases len - i,
            {
                proof {
                    a.lemma_spec_index(i as int);
                    // take(i+1) == take(i).push(s[i]).
                    assert(s.take(i as int + 1) =~= s.take(i as int).push(s[i as int]));
                }
                acc = f(&acc, &a.seq[i]);
                proof {
                    // Help the solver unfold fold_left on take(i+1).
                    let ghost t = s.take(i as int + 1);
                    assert(t.len() > 0);
                    assert(t.drop_last() =~= s.take(i as int));
                    assert(t.last() == s[i as int]);
                    reveal(Seq::fold_left);
                }
                let cloned = acc.clone();
                proof {
                    // The clone axiom ensures the cloned value equals the original in spec.
                    axiom_cloned_implies_eq_owned(acc, cloned);
                }
                seq.push(cloned);
                i += 1;
            }
            proof {
                // At loop exit, i == len so take(len) == s.
                assert(s.take(len as int) =~= s);
            }
            let scanned_seq = ArraySeqS { seq };
            proof {
                // Bridge from seq@ indices to spec_index for the prefix sum postcondition.
                assert forall|i: int| #![trigger scanned_seq.spec_index(i)] 0 <= i < a.spec_len() implies
                    scanned_seq.spec_index(i) == s.take(i + 1).fold_left(id, spec_f)
                by {
                    assert(scanned_seq.spec_index(i) == seq@[i]);
                }
            }
            (scanned_seq, acc)
        }

        fn inject(a: &ArraySeqS<T>, updates: &Vec<(usize, T)>) -> (injected: ArraySeqS<T>)
            where T: Clone + Eq
        {
            let ghost s = a.seq@;
            let ghost u = updates@;
            let len = a.seq.len();
            let ulen = updates.len();

            // Build an element-wise copy of a.
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

            // Apply updates in reverse so the first update to each position wins.
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
                    // Help the solver unfold spec_inject one step.
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
            let injected = ArraySeqS { seq: result_vec };
            proof {
                // Bridge result to the abstract ensures form.
                assert(Seq::new(injected.spec_len(), |i: int| injected.spec_index(i)) =~= result_vec@);
                assert forall|j: int| 0 <= j < a.spec_len() implies #[trigger] a.spec_index(j) == s[j]
                by { a.lemma_spec_index(j); }
                assert(Seq::new(a.spec_len(), |i: int| a.spec_index(i)) =~= s);
            }
            injected
        }

        fn scan_inclusive<F: Fn(&T, &T) -> T>(a: &ArraySeqS<T>, f: &F, Ghost(spec_f): Ghost<spec_fn(T, T) -> T>, id: T) -> (result: ArraySeqS<T>)
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
            let result = ArraySeqS { seq };
            proof {
                assert forall|i: int| #![trigger result.spec_index(i)] 0 <= i < a.spec_len() implies
                    result.spec_index(i) == s.take(i + 1).fold_left(id, spec_f)
                by {
                    assert(result.spec_index(i) == seq@[i]);
                }
            }
            result
        }

        fn subseq_copy(&self, start: usize, length: usize) -> (subseq: ArraySeqS<T>)
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
                    crate::vstdplus::feq::feq::axiom_cloned_implies_eq_owned(self.seq[i as int], last);
                }
                i += 1;
            }
            ArraySeqS { seq }
        }

        fn remove(&mut self, index: usize) -> (element: T) {
            let ghost old_seq = self.seq@;
            let element = self.seq.remove(index);
            proof {
                old_seq.remove_ensures(index as int);
            }
            element
        }

        fn insert(&mut self, index: usize, element: T) {
            let ghost old_seq = self.seq@;
            self.seq.insert(index, element);
            proof {
                old_seq.insert_ensures(index as int, element);
            }
        }

        fn from_vec(elts: Vec<T>) -> (seq: ArraySeqS<T>) {
            ArraySeqS { seq: elts }
        }

        /// Linear scan for the first group whose key matches `needle`.
        fn find_key<K: View + Eq + PartialEq, V: View>(
            groups: &ArraySeqS<(K, ArraySeqS<V>)>,
            needle: &K,
        ) -> (found: Option<usize>)
        {
            proof { reveal(obeys_concrete_eq); }
            let len = groups.seq.len();
            let mut j: usize = 0;
            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            while j < len
                invariant
                j <= len,
            len == groups.seq@.len(),
            forall|m: int| #![trigger groups.seq@[m]] 0 <= m < j ==> groups.seq@[m].0 != *needle,
            decreases len - j,
            {
                if groups.seq[j].0 == *needle {
                    return Some(j);
                }
                j += 1;
            }
            None
        }


        /// Definition 18.18 (collect). Group key-value pairs by key, preserving value order.
        /// This is not Rust style iter().collect(), this is a SQL style collect with group_by.
        /// The Verus limitation of "index for &mut not supported" prevents
        /// groups[idx].1.push(v). So we remove the entry, mutate it, and re-insert.
        fn collect<K: DeepView<V = K> + View + Clone + Eq + PartialEq, V: DeepView<V = V> + View + Clone + Eq>(
            pairs: &ArraySeqS<(K, V)>,
        ) -> (collected: ArraySeqS<(K, ArraySeqS<V>)>)
        {
            let plen = pairs.seq.len();
            let mut collected: ArraySeqS<(K, ArraySeqS<V>)> = ArraySeqS { seq: Vec::new() };
            let mut i: usize = 0;
            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            while i < plen
                invariant
                i <= plen,
                plen == pairs.seq@.len(),
                collected.seq.deep_view() =~= spec_collect(pairs.seq@.take(i as int)),
            decreases plen - i,
            {
                proof {
                    assert(pairs.seq@.take(i as int + 1) =~= pairs.seq@.take(i as int).push(pairs.seq@[i as int]));
                    let ghost t = pairs.seq@.take(i as int + 1);
                    assert(t.drop_last() =~= pairs.seq@.take(i as int));
                    assert(t.last() == pairs.seq@[i as int]);
                    reveal(spec_collect);
                }
                let ghost old_collected_dv = collected.seq.deep_view();
                let k = pairs.seq[i].0.clone();
                let v = pairs.seq[i].1.clone();
                proof {
                    axiom_cloned_implies_eq_owned::<K>(pairs.seq@[i as int].0, k);
                    axiom_cloned_implies_eq_owned::<V>(pairs.seq@[i as int].1, v);
                }
                match Self::find_key(&collected, &k) {
                    Some(idx) => {
                        proof {
                            lemma_find_key_some(&collected.seq, k, idx);
                            assert(old_collected_dv =~= collected.seq.deep_view());
                            assert(k.deep_view() == k);
                            assert(spec_find_key_index(old_collected_dv, k) == Some(idx as int));
                            lemma_spec_collect_step_some(old_collected_dv, pairs.seq@.take(i as int), k, v, idx as int);
                        }
                        let ghost new_collected_dv = old_collected_dv.remove(idx as int).insert(idx as int, (k, old_collected_dv[idx as int].1.push(v)));
                        let mut entry = collected.seq.remove(idx);
                        entry.1.seq.push(v);
                        collected.seq.insert(idx, entry);
                        proof {
                            lemma_deep_view_len(&collected.seq);
                            assert(k.deep_view() == k);
                            assert(v.deep_view() == v);
                            assert forall|j: int| 0 <= j < collected.seq.deep_view().len()
                                implies #[trigger] collected.seq.deep_view()[j] =~= new_collected_dv[j]
                            by {
                                lemma_deep_view_key::<K, V>(&collected.seq, j);
                            };
                        }
                    }
                    None => {
                        proof {
                            lemma_find_key_none(&collected.seq, k);
                            assert(old_collected_dv =~= collected.seq.deep_view());
                            assert(k.deep_view() == k);
                            assert(spec_find_key_index(old_collected_dv, k) == None::<int>);
                            lemma_spec_collect_step_none(old_collected_dv, pairs.seq@.take(i as int), k, v);
                        }
                        let ghost new_collected_dv = old_collected_dv.push((k, seq![v]));
                        collected.seq.push((k, ArraySeqS { seq: vec![v] }));
                        proof {
                            lemma_deep_view_len(&collected.seq);
                            assert(v.deep_view() == v);
                            assert(k.deep_view() == k);
                            assert forall|j: int| 0 <= j < collected.seq.deep_view().len()
                                implies #[trigger] collected.seq.deep_view()[j] =~= new_collected_dv[j]
                            by {
                                lemma_deep_view_key::<K, V>(&collected.seq, j);
                            };
                        }
                    }
                }
                i += 1;
            }
            proof {
                assert(pairs.seq@.take(plen as int) =~= pairs.seq@);
            }
            collected
        }
    }
        
    /// Algorithm 18.4 (map). Transform each element via `f`.
    /// Module-level function because map returns ArraySeqS<U> (different element type),
    /// which creates a Verus cycle error when spec_index/spec_len on the return value
    /// resolve through a concrete impl of the same trait.
    pub fn map<T: View, U: Clone + View, F: Fn(&T) -> U>(a: &ArraySeqS<T>, f: &F) -> (mapped: ArraySeqS<U>)
        requires forall|i: int| 0 <= i < a.spec_len() ==> #[trigger] f.requires((&a.spec_index(i),))
        ensures
            mapped.spec_len() == a.spec_len(),
            forall|i: int| #![trigger mapped.spec_index(i)] 0 <= i < a.spec_len() ==> f.ensures((&a.spec_index(i),), mapped.spec_index(i)),
    {
        let len = a.seq.len();
        let mut seq: Vec<U> = Vec::with_capacity(len);
        let mut i: usize = 0;
        while i < len
            invariant
                i <= len,
                len == a.seq@.len(),
                seq@.len() == i as int,
                forall|j: int| 0 <= j < a.spec_len() ==> #[trigger] f.requires((&a.spec_index(j),)),
                forall|j: int| #![trigger seq@[j]] 0 <= j < i ==> f.ensures((&a.spec_index(j),), seq@[j]),
            decreases len - i,
        {
            proof { a.lemma_spec_index(i as int); }
            seq.push(f(&a.seq[i]));
            i += 1;
        }
        ArraySeqS { seq }
    }

    /// Algorithm 18.3 (tabulate). Build a sequence by applying `f` to each index.
    /// Module-level function for the same reason as map: the return type ArraySeqS<T>
    /// is concrete, and its spec_len/spec_index in ensures creates a Verus cycle
    /// when tabulate is declared inside a trait that also defines spec_len/spec_index.
    pub fn tabulate<T: View, F: Fn(usize) -> T>(f: &F, length: usize) -> (tab_seq: ArraySeqS<T>)
        requires
            length <= usize::MAX,
            forall|i: usize| i < length ==> #[trigger] f.requires((i,)),
        ensures
            tab_seq.spec_len() == length as int,
            forall|i: int| #![trigger tab_seq.spec_index(i)] 0 <= i < length ==> f.ensures((i as usize,), tab_seq.spec_index(i)),
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
        ArraySeqS { seq }
    }

    /// Definition 18.15 (flatten). Concatenate a sequence of sequences.
    /// Module-level function because flatten takes ArraySeqS<ArraySeqS<T>>
    /// (nested concrete types), which creates Verus cycle issues in traits.
    pub fn flatten<T: View + Clone + Eq>(a: &ArraySeqS<ArraySeqS<T>>) -> (flattened: ArraySeqS<T>)
        requires
            obeys_feq_clone::<T>(),
        ensures
            flattened.seq@ =~= a.seq@.map_values(|inner: ArraySeqS<T>| inner.seq@).flatten(),
    {
        let outer_len = a.seq.len();
        let mut seq: Vec<T> = Vec::new();
        let mut i: usize = 0;
        while i < outer_len
            invariant
                i <= outer_len,
                outer_len == a.seq@.len(),
                obeys_feq_clone::<T>(),
                seq@ =~= a.seq@.take(i as int).map_values(|inner: ArraySeqS<T>| inner.seq@).flatten(),
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
                    seq@ =~= a.seq@.take(i as int).map_values(|inner: ArraySeqS<T>| inner.seq@).flatten()
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
                let ghost prefix = a.seq@.take(i as int).map_values(|inner: ArraySeqS<T>| inner.seq@);
                assert(a.seq@.take(i as int + 1).map_values(|inner: ArraySeqS<T>| inner.seq@)
                    =~= prefix.push(a.seq@[i as int].seq@));
                prefix.lemma_flatten_push(a.seq@[i as int].seq@);
            }
            i += 1;
        }
        proof {
            assert(a.seq@.take(outer_len as int) =~= a.seq@);
        }
        ArraySeqS { seq }
    }

    /// Definition 18.19 (iteratePrefixes). Return all intermediate accumulator values
    /// (exclusive prefixes) and the final result.  Module-level because the return type
    /// ArraySeqS<A> differs from Self when A differs from T.
    pub fn iterate_prefixes<T: View, A: View + Clone + Eq, F: Fn(&A, &T) -> A>(
        a: &ArraySeqS<T>, f: &F,
        Ghost(spec_f): Ghost<spec_fn(A, T) -> A>,
        start_x: A,
    ) -> (prefixes: (ArraySeqS<A>, A))
        requires
            obeys_feq_clone::<A>(),
            forall|x: &A, y: &T| #[trigger] f.requires((x, y)),
            forall|a: A, t: T, ret: A| f.ensures((&a, &t), ret) <==> ret == spec_f(a, t),
        ensures
            prefixes.0.spec_len() == a.spec_len(),
            forall|i: int| #![trigger prefixes.0.spec_index(i)] 0 <= i < a.spec_len() ==>
                prefixes.0.spec_index(i) == spec_iterate_prefixes(
                    Seq::new(a.spec_len(), |j: int| a.spec_index(j)), spec_f, start_x).0[i],
            prefixes.1 == spec_iterate(
                Seq::new(a.spec_len(), |j: int| a.spec_index(j)), spec_f, start_x),
    {
        let ghost s = Seq::new(a.spec_len(), |i: int| a.spec_index(i));
        let len = a.seq.len();
        let mut acc = start_x;
        let mut seq: Vec<A> = Vec::with_capacity(len);
        let mut i: usize = 0;
        while i < len
            invariant
                i <= len,
                len == a.seq@.len(),
                seq@.len() == i as int,
                obeys_feq_clone::<A>(),
                forall|x: &A, y: &T| #[trigger] f.requires((x, y)),
                forall|a: A, t: T, ret: A| f.ensures((&a, &t), ret) ==> ret == spec_f(a, t),
                s == Seq::new(a.spec_len(), |j: int| a.spec_index(j)),
                // The accumulator equals the fold of the first i elements.
                acc == s.take(i as int).fold_left(start_x, spec_f),
                // Each element written so far is the exclusive prefix fold.
                forall|k: int| #![trigger seq@[k]] 0 <= k < seq@.len() ==>
                    seq@[k] == s.take(k).fold_left(start_x, spec_f),
            decreases len - i,
        {
            // Push the current accumulator (exclusive: before processing element i).
            let cloned = acc.clone();
            proof {
                axiom_cloned_implies_eq_owned(acc, cloned);
            }
            seq.push(cloned);
            proof {
                a.lemma_spec_index(i as int);
                assert(s.take(i as int + 1) =~= s.take(i as int).push(s[i as int]));
            }
            acc = f(&acc, &a.seq[i]);
            proof {
                // Help the solver unfold fold_left on take(i+1).
                let ghost t = s.take(i as int + 1);
                assert(t.len() > 0);
                assert(t.drop_last() =~= s.take(i as int));
                assert(t.last() == s[i as int]);
                reveal(Seq::fold_left);
            }
            i += 1;
        }
        proof {
            // At loop exit, take(len) == s.
            assert(s.take(len as int) =~= s);
        }
        let result = ArraySeqS { seq };
        proof {
            // Bridge seq@ indices to spec_index for the ensures.
            assert forall|i: int| #![trigger result.spec_index(i)] 0 <= i < a.spec_len() implies
                result.spec_index(i) == spec_iterate_prefixes(s, spec_f, start_x).0[i]
            by {
                assert(result.spec_index(i) == seq@[i]);
            }
        }
        (result, acc)
    }

    // deep_view is the identity function for this type.
    pub open spec fn obeys_generic_deep_eq<T: DeepView<V = T>>() -> bool {
        forall|x: T| x.deep_view() == x
    }

    // Bridge: deep_view preserves length.
    proof fn lemma_deep_view_len<T: DeepView>(v: &Vec<T>)
        ensures
            v.deep_view().len() == v@.len(),
    {
    }

    // Bridge: deep_view preserves .0 at every index
    proof fn lemma_deep_view_key<K: DeepView, V: DeepView>(s: &Vec<(K, ArraySeqS<V>)>, i: int)
        requires
            0 <= i < s@.len(),
        ensures
            s.deep_view()[i].0 == s@[i].0.deep_view(),
            s.deep_view().len() == s@.len(),
    {
    }

    // spec_find_key_index returning Some(idx) implies idx is in bounds.
    proof fn lemma_find_key_index_bounds<K, V>(groups: Seq<(K, Seq<V>)>, k: K, idx: int)
        requires
            spec_find_key_index(groups, k) == Some(idx),
        ensures
            0 <= idx < groups.len(),
        decreases groups.len(),
    {
        reveal(spec_find_key_index);
        if groups.len() > 0 && groups[0].0 != k {
            lemma_find_key_index_bounds(groups.skip(1), k, idx - 1);
        }
    }

    // Pure spec: spec_find_key_index returns Some(idx) when element idx matches
    // and no earlier element does.
    proof fn lemma_find_key_index_found<K, V>(
        groups: Seq<(K, Seq<V>)>,
        k: K,
        idx: int,
    )
        requires
            0 <= idx < groups.len(),
            groups[idx].0 == k,
            forall|m: int| #![trigger groups[m]] 0 <= m < idx ==> groups[m].0 != k,
        ensures
            spec_find_key_index(groups, k) == Some(idx),
        decreases groups.len(),
    {
        reveal(spec_find_key_index);
        if groups.len() > 0 && groups[0].0 != k {
            lemma_find_key_index_found(groups.skip(1), k, idx - 1);
        }
    }

    // Pure spec: spec_find_key_index returns None when no element matches.
    proof fn lemma_find_key_index_not_found<K, V>(
        groups: Seq<(K, Seq<V>)>,
        k: K,
    )
        requires
            forall|m: int| #![trigger groups[m]] 0 <= m < groups.len() ==> groups[m].0 != k,
        ensures
            spec_find_key_index(groups, k) == None::<int>,
        decreases groups.len(),
    {
        reveal(spec_find_key_index);
        if groups.len() > 0 {
            lemma_find_key_index_not_found(groups.skip(1), k);
        }
    }

    // Unfolding spec_collect one step when the key is found.
    proof fn lemma_spec_collect_step_some<K, V>(
        old_dv: Seq<(K, Seq<V>)>,
        pairs_prefix: Seq<(K, V)>,
        k: K,
        v: V,
        idx: int,
    )
        requires
            old_dv =~= spec_collect(pairs_prefix),
            spec_find_key_index(old_dv, k) == Some(idx),
        ensures
            spec_collect(pairs_prefix.push((k, v)))
                =~= old_dv.remove(idx).insert(idx, (k, old_dv[idx].1.push(v))),
    {
        lemma_find_key_index_bounds(old_dv, k, idx);
        let extended = pairs_prefix.push((k, v));
        assert(extended.len() > 0);
        assert(extended.drop_last() =~= pairs_prefix);
        assert(extended.last() == (k, v));
        reveal(spec_collect);
    }

    // Unfolding spec_collect one step when the key is new.
    proof fn lemma_spec_collect_step_none<K, V>(
        old_dv: Seq<(K, Seq<V>)>,
        pairs_prefix: Seq<(K, V)>,
        k: K,
        v: V,
    )
        requires
            old_dv =~= spec_collect(pairs_prefix),
            spec_find_key_index(old_dv, k) == None::<int>,
        ensures
            spec_collect(pairs_prefix.push((k, v)))
                =~= old_dv.push((k, seq![v])),
    {
        let extended = pairs_prefix.push((k, v));
        assert(extended.len() > 0);
        assert(extended.drop_last() =~= pairs_prefix);
        assert(extended.last() == (k, v));
        reveal(spec_collect);
    }

    // When find_key returns Some(idx), spec_find_key_index on deep_view agrees.
    proof fn lemma_find_key_some<K: DeepView<V = K>, V: DeepView>(s: &Vec<(K, ArraySeqS<V>)>, k: K, idx: usize)
        requires
            obeys_generic_deep_eq::<K>(),
            idx < s@.len(),
            s@[idx as int].0 == k,
            forall|m: int| #![trigger s@[m]] 0 <= m < idx as int ==> s@[m].0 != k,
        ensures
            spec_find_key_index(s.deep_view(), k.deep_view()) == Some(idx as int),
    {
        assert forall|j: int| #![trigger s.deep_view()[j]]
            0 <= j < s.deep_view().len() implies s.deep_view()[j].0 == s@[j].0
        by {
            lemma_deep_view_key::<K, V>(s, j);
        };
        lemma_find_key_index_found(s.deep_view(), k.deep_view(), idx as int);
    }

    // When find_key returns None, spec_find_key_index on deep_view is None.
    proof fn lemma_find_key_none<K: DeepView<V = K>, V: DeepView>(s: &Vec<(K, ArraySeqS<V>)>, k: K)
        requires
            obeys_generic_deep_eq::<K>(),
            forall|m: int| #![trigger s@[m]] 0 <= m < s@.len() ==> s@[m].0 != k,
        ensures
            spec_find_key_index(s.deep_view(), k.deep_view()) == None::<int>,
    {
        assert forall|j: int| #![trigger s.deep_view()[j]]
            0 <= j < s.deep_view().len() implies s.deep_view()[j].0 == s@[j].0
        by {
            lemma_deep_view_key::<K, V>(s, j);
        };
        lemma_find_key_index_not_found(s.deep_view(), k.deep_view());
    }

    impl<T: View> ArraySeqS<T> {
        // Equate our spec_index on this type with vector indexing.
        broadcast proof fn lemma_spec_index(&self, i: int)
            requires 0 <= i < self.spec_len()
            ensures #[trigger] self.seq@[i] == self.spec_index(i)
        {}

        /// Returns custom ArraySeqIter following Chap05 pattern.
        pub fn iter(&self) -> (it: ArraySeqIter<'_, T>)
            ensures
                it@.0 == 0,
                it@.1 == self.seq@,
                iter_invariant(&it),
        {
            ArraySeqIter { inner: self.seq.iter() }
        }
    }

    #[verifier::external]
    impl<T: View> ArraySeqS<T> {
        pub fn iter_mut(&mut self) -> IterMut<'_, T> { self.seq.iter_mut() }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: View + PartialEq> PartialEqSpecImpl for ArraySeqS<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }


    //		10. iterators

    /// Iterator wrapper with closed spec view for encapsulation.
    /// Inner is private; closed view() can access it but external code cannot see it.
    #[verifier::reject_recursive_types(T)]
    pub struct ArraySeqIter<'a, T> {
        inner: std::slice::Iter<'a, T>,  // PRIVATE
    }

    /// Ghost iterator for ForLoopGhostIterator support (for-iter, for-borrow patterns).
    #[verifier::reject_recursive_types(T)]
    pub struct ArraySeqGhostIterator<'a, T> {
        pub pos: int,
        pub elements: Seq<T>,
        pub phantom: core::marker::PhantomData<&'a T>,
    }

    impl<'a, T> View for ArraySeqIter<'a, T> {
        type V = (int, Seq<T>);
        closed spec fn view(&self) -> (int, Seq<T>) {
            self.inner@
        }
    }

    impl<'a, T> View for ArraySeqGhostIterator<'a, T> {
        type V = Seq<T>;

        open spec fn view(&self) -> Seq<T> {
            self.elements.take(self.pos)
        }
    }


    pub open spec fn iter_invariant<'a, T>(it: &ArraySeqIter<'a, T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<'a, T> std::iter::Iterator for ArraySeqIter<'a, T> {
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

    impl<'a, T> vstd::pervasive::ForLoopGhostIteratorNew for ArraySeqIter<'a, T> {
        type GhostIter = ArraySeqGhostIterator<'a, T>;

        open spec fn ghost_iter(&self) -> ArraySeqGhostIterator<'a, T> {
            ArraySeqGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    impl<'a, T> vstd::pervasive::ForLoopGhostIterator for ArraySeqGhostIterator<'a, T> {
        type ExecIter = ArraySeqIter<'a, T>;
        type Item = T;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &ArraySeqIter<'a, T>) -> bool {
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

        open spec fn ghost_advance(&self, _exec_iter: &ArraySeqIter<'a, T>) -> ArraySeqGhostIterator<'a, T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, T> std::iter::IntoIterator for &'a ArraySeqS<T> {
        type Item = &'a T;
        type IntoIter = ArraySeqIter<'a, T>;
        fn into_iter(self) -> Self::IntoIter { ArraySeqIter { inner: self.seq.iter() } }
    }

    impl<T> std::iter::IntoIterator for ArraySeqS<T> {
        type Item = T;
        type IntoIter = IntoIter<T>;
        fn into_iter(self) -> Self::IntoIter { self.seq.into_iter() }
    }

    #[verifier::external]
    impl<'a, T> std::iter::IntoIterator for &'a mut ArraySeqS<T> {
        type Item = &'a mut T;
        type IntoIter = IterMut<'a, T>;
        fn into_iter(self) -> Self::IntoIter { self.seq.iter_mut() }
    }


    //		11. derive impls in verus!

    impl<T: Clone> Clone for ArraySeqS<T> {
        fn clone(&self) -> Self {
            ArraySeqS { seq: self.seq.clone() }
        }
    }

    impl<T: Eq + View> Eq for ArraySeqS<T> {}

    impl<T: PartialEq + View> PartialEq for ArraySeqS<T> {
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

    impl<T: Debug> Debug for ArraySeqS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_list().entries(self.seq.iter()).finish()
        }
    }

    impl<T: Display> Display for ArraySeqS<T> {
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
