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

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

    verus! {

    //		2. imports

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::vec::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::clone::*;

    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;

    //		3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms,
    };


    //		4. type definitions

    #[verifier::reject_recursive_types(T)]
    pub struct ArraySeqS<T> {
        pub seq: Vec<T>,
    }

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


    //		5. view impls

    impl<T: View> View for ArraySeqS<T> {
        type V = Seq<T::V>;

        open spec fn view(&self) -> Seq<T::V> {
            self.seq@.map(|_i: int, t: T| t@)
        }
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


    //		6. spec fns

    pub open spec fn iter_invariant<'a, T>(it: &ArraySeqIter<'a, T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }


    //		8. traits

    /// Data Type 18.1: Generic sequence trait for array-backed sequences.
    pub trait ArraySeqTrait<T: View>: Sized {
        spec fn spec_len(&self) -> int;

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
              forall|i: int| #![auto] 0 <= i < length ==> new_seq.spec_index(i) == init_value; 

        /// - Set the element at `index` to `item` in place.
        /// - Work Θ(1), Span Θ(1).
        fn set(&mut self, index: usize, item: T) -> (success: Result<(), &'static str>)
            requires index < old(self).spec_len()
            ensures
                success.is_ok() ==> self.spec_len() == old(self).spec_len(),
                success.is_ok() ==> self.spec_index(index as int) == item,
                success.is_ok() ==> forall|i: int| #![auto] 0 <= i < old(self).spec_len() && i != index ==> self.spec_index(i) == old(self).spec_index(i);

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
                forall|i: int| #![auto] 0 <= i < length ==> subseq.spec_index(i) == a.spec_index(start as int + i);

        /// - Definition 18.13 (append). Concatenate two sequences.
        /// - Work Θ(|a| + |b|), Span Θ(1).
        fn append(a: &Self, b: &Self) -> (appended: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                a.spec_len() + b.spec_len() <= usize::MAX as int,
            ensures
                appended.spec_len() == a.spec_len() + b.spec_len(),
                forall|i: int| #![auto] 0 <= i < a.spec_len() ==> appended.spec_index(i) == a.spec_index(i),
                forall|i: int| #![auto] 0 <= i < b.spec_len() ==> appended.spec_index(a.spec_len() + i) == b.spec_index(i);

        /// - Definition 18.14 (filter). Keep elements satisfying `pred`.
        /// - Work Θ(|a|), Span Θ(1).
        fn filter<F: Fn(&T) -> bool>(a: &Self, pred: &F) -> (filtered: Self)
            where T: Clone + Eq
            requires 
              obeys_feq_clone::<T>(),
              forall|i: int| 0 <= i < a.spec_len() ==> #[trigger] pred.requires((&a.spec_index(i),))
            ensures
                filtered.spec_len() <= a.spec_len(),
                forall|i: int| #![auto] 0 <= i < filtered.spec_len() ==> pred.ensures((&filtered.spec_index(i),), true);

/* CORPSE: flatten needs concrete type access, can't use &Self yet
        /// - Definition 18.15 (flatten). Concatenate a sequence of sequences.
        /// - Work Θ(total length), Span Θ(1).
        fn flatten(a: &ArraySeqS<ArraySeqS<T>>) -> (flattened: Self)
            where T: Clone
            ensures a.spec_len() == 0 ==> flattened.spec_len() == 0;
*/

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
                forall|i: int| #![auto] 0 <= i < a.spec_len() && i != index as int ==> updated.spec_index(i) == a.spec_index(i);

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
        fn iterate<A, F: Fn(&A, &T) -> A>(a: &Self, f: &F, seed: A) -> A
            requires forall|x: &A, y: &T| #[trigger] f.requires((x, y));

        /// - Definition 18.18 (reduce). Combine elements using associative `f` and identity `id`.
        /// - Work Θ(|a|), Span Θ(1).
        fn reduce<F: Fn(&T, &T) -> T>(a: &Self, f: &F, id: T) -> T
            where T: Clone
            requires forall|x: &T, y: &T| #[trigger] f.requires((x, y));

        /// - Definition 18.19 (scan). Prefix-reduce returning partial sums and total.
        /// - Work Θ(|a|), Span Θ(1).
        fn scan<F: Fn(&T, &T) -> T>(a: &Self, f: &F, id: T) -> (scanned: (Self, T))
            where T: Clone
            requires forall|x: &T, y: &T| #[trigger] f.requires((x, y))
            ensures scanned.0.spec_len() == a.spec_len();

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
                forall|i: int| #![auto] 0 <= i < length ==> subseq.spec_index(i) == self.spec_index(start as int + i);

        /// - Create sequence from Vec.
        /// - Work Θ(n) worst case, Θ(1) best case, Span Θ(1).
        fn from_vec(elts: Vec<T>) -> (seq: Self)
            ensures
                seq.spec_len() == elts@.len(),
                forall|i: int| #![auto] 0 <= i < elts@.len() ==> seq.spec_index(i) == elts@[i];

    }


    //		9. impls


    impl<T: View> ArraySeqTrait<T> for ArraySeqS<T> {
        open spec fn spec_len(&self) -> int {
            self.seq@.len() as int
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

/* CORPSE: old duplicates of methods now in trait impl
        fn set_old(&mut self, index: usize, item: T) -> (success: Result<(), &'static str>) {
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
*/

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
                    forall|j: int| #![auto] 0 <= j < seq@.len() ==> seq@[j] == a.seq@[(start + j) as int],
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
                    forall|k: int| #![auto] 0 <= k < i ==> seq@[k] == a.seq@[k],
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
                    forall|k: int| #![auto] 0 <= k < a_len ==> seq@[k] == a.seq@[k],
                    forall|k: int| #![auto] 0 <= k < j ==> seq@[a_len as int + k] == b.seq@[k],
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

        fn filter<F: Fn(&T) -> bool>(a: &ArraySeqS<T>, pred: &F) -> (filtered: ArraySeqS<T>)
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
                    forall|j: int| 0 <= j < a.spec_len() ==> #[trigger] pred.requires((&a.spec_index(j),)),
                    forall|j: int| #![auto] 0 <= j < seq@.len() ==> pred.ensures((&seq@[j],), true),
                decreases len - i,
            {
                proof { a.lemma_spec_index(i as int); }
                if pred(&a.seq[i]) {
                    seq.push(a.seq[i].clone());
                    proof {
                        let ghost last = seq@[seq@.len() - 1 as int];
                        assert(cloned(a.seq[i as int], last));
                        axiom_cloned_implies_eq_owned(a.seq[i as int], last);
                    }
                }
                i += 1;
            }
            ArraySeqS { seq }
        }

/* CORPSE: flatten impl body (not in trait yet)
        fn flatten(a: &ArraySeqS<ArraySeqS<T>>) -> (flattened: ArraySeqS<T>)
            where T: Clone
        {
            let outer_len = a.seq.len();
            let mut seq: Vec<T> = Vec::new();
            let mut i: usize = 0;
            while i < outer_len
                invariant
                    i <= outer_len,
                    outer_len == a.seq@.len(),
                decreases outer_len - i,
            {
                let inner = &a.seq[i];
                let inner_len = inner.seq.len();
                let mut j: usize = 0;
                while j < inner_len
                    invariant
                        j <= inner_len,
                        inner_len == inner.seq@.len(),
                    decreases inner_len - j,
                {
                    seq.push(inner.seq[j].clone());
                    j += 1;
                }
                i += 1;
            }
            ArraySeqS { seq }
        }
*/

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
                    forall|k: int| #![auto] 0 <= k < i && k != index as int ==> seq@[k] == a.seq@[k],
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

        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqS<T>, f: &F, seed: A) -> (acc: A) {
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

        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqS<T>, f: &F, id: T) -> (reduced: T)
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

        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqS<T>, f: &F, id: T) -> (scanned: (ArraySeqS<T>, T))
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
            (ArraySeqS { seq }, acc)
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
                    forall|j: int| #![auto] 0 <= j < seq@.len() ==> seq@[j] == self.seq@[(start + j) as int],
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

        fn from_vec(elts: Vec<T>) -> (seq: ArraySeqS<T>) {
            ArraySeqS { seq: elts }
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
            forall|i: int| #![auto] 0 <= i < a.spec_len() ==> f.ensures((&a.spec_index(i),), mapped.spec_index(i)),
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
                forall|j: int| #![auto] 0 <= j < i ==> f.ensures((&a.spec_index(j),), seq@[j]),
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
            forall|i: int| #![auto] 0 <= i < length ==> f.ensures((i as usize,), tab_seq.spec_index(i)),
    {
        let mut seq = Vec::with_capacity(length);
        let mut i: usize = 0;
        while i < length
            invariant
                i <= length,
                seq@.len() == i as int,
                forall|j: usize| j < length ==> #[trigger] f.requires((j,)),
                forall|j: int| #![auto] 0 <= j < i ==> f.ensures((j as usize,), seq@[j]),
            decreases length - i,
        {
            seq.push(f(i));
            i += 1;
        }
        ArraySeqS { seq }
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
