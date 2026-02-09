//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Chapter 18 persistent sequence implementation for array-backed sequences. Verusified.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	4. type definitions
//	5. view impls
//	8. traits
//	9. impls
//	10. iterators
//	13. derive impls outside verus!

//		1. module


pub mod ArraySeqStPer {

    use std::fmt::{Debug, Display, Formatter};
    use std::fmt::Result as FmtResult;
    use std::slice::Iter;
    use std::vec::IntoIter;

    use vstd::prelude::*;

    verus! {

    //		2. imports

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::clone::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;


    //		3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
        vstd::seq_lib::group_seq_properties,
        crate::vstdplus::feq::feq::group_feq_axioms,
    };


    //		4. type definitions

    #[verifier::reject_recursive_types(T)]
    pub struct ArraySeqStPerS<T> {
        pub seq: Vec<T>,
    }


    //		5. view impls

    impl<T: View> View for ArraySeqStPerS<T> {
        type V = Seq<T::V>;

        open spec fn view(&self) -> Seq<T::V> {
            self.seq@.map(|_i: int, t: T| t@)
        }
    }


    //		8. traits

    /// - Base trait for single-threaded persistent array sequences (Chapter 18).
    /// - These methods are never redefined in later chapters.
    pub trait ArraySeqStPerBaseTrait<T>: Sized {
        spec fn spec_len(&self) -> int;
        spec fn spec_index(&self, i: int) -> T
            recommends i < self.spec_len();

        /// Work Θ(n), Span Θ(1)
        fn new(length: usize, init_value: T) -> (new_seq: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                length <= usize::MAX,
            ensures
                new_seq.spec_len() == length as int,
                forall|i: int| #![auto] 0 <= i < length ==> new_seq.spec_index(i) == init_value;

        /// Work Θ(1), Span Θ(1)
        fn length(&self) -> (len: usize)
            ensures len as int == self.spec_len();

        /// Work Θ(1), Span Θ(1)
        fn nth(&self, index: usize) -> (nth_elem: &T)
            requires index < self.spec_len()
            ensures *nth_elem == self.spec_index(index as int);

        /// Work Θ(len), Span Θ(1)
        fn subseq_copy(&self, start: usize, length: usize) -> (subseq: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                start + length <= usize::MAX,
                start + length <= self.spec_len(),
            ensures
                subseq.spec_len() == length as int,
                forall|i: int| #![auto] 0 <= i < length ==> subseq.spec_index(i) == self.spec_index(start as int + i);

        /// Work Θ(n), Span Θ(1)
        fn from_vec(elts: Vec<T>) -> (seq: Self)
            ensures
                seq.spec_len() == elts@.len(),
                forall|i: int| #![auto] 0 <= i < elts@.len() ==> seq.spec_index(i) == elts@[i];
    }

    /// Redefinable trait - may be overridden with better algorithms in later chapters.
    /// map, tabulate, and flatten are module-level functions to avoid Verus cycle errors.
    pub trait ArraySeqStPerRedefinableTrait<T>: ArraySeqStPerBaseTrait<T> {

        /// Work Θ(1), Span Θ(1)
        fn empty() -> (empty_seq: Self)
            ensures empty_seq.spec_len() == 0;

        /// Work Θ(1), Span Θ(1)
        fn singleton(item: T) -> (singleton: Self)
            ensures
                singleton.spec_len() == 1,
                singleton.spec_index(0) == item;

        /// Work Θ(|a|+|b|), Span Θ(1)
        fn append(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> (appended: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                a.seq@.len() + b.seq@.len() <= usize::MAX as int,
            ensures
                appended.spec_len() == a.seq@.len() + b.seq@.len(),
                forall|i: int| #![auto] 0 <= i < a.seq@.len() ==> appended.spec_index(i) == a.seq@[i],
                forall|i: int| #![auto] 0 <= i < b.seq@.len() ==> appended.spec_index(a.seq@.len() as int + i) == b.seq@[i];

        /// Work Θ(|a|), Span Θ(1)
        fn filter<F: Fn(&T) -> bool>(a: &ArraySeqStPerS<T>, pred: &F) -> (filtered: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                forall|i: int| 0 <= i < a.seq@.len() ==> #[trigger] pred.requires((&a.seq@[i],)),
            ensures
                filtered.spec_len() <= a.seq@.len(),
                forall|i: int| #![auto] 0 <= i < filtered.spec_len() ==> pred.ensures((&filtered.spec_index(i),), true);

        /// Work Θ(|a|), Span Θ(1)
        fn update(a: &ArraySeqStPerS<T>, index: usize, item: T) -> (updated: Self)
            where T: Clone + Eq
            requires
                obeys_feq_clone::<T>(),
                index < a.seq@.len(),
            ensures
                updated.spec_len() == a.seq@.len(),
                updated.spec_index(index as int) == item,
                forall|i: int| #![auto] 0 <= i < a.seq@.len() && i != index as int ==> updated.spec_index(i) == a.seq@[i];

        /// Work Θ(1), Span Θ(1)
        fn is_empty(&self) -> (empty: bool)
            ensures empty <==> self.spec_len() == 0;

        /// Work Θ(1), Span Θ(1)
        fn is_singleton(&self) -> (single: bool)
            ensures single <==> self.spec_len() == 1;

        /// Work Θ(|a|), Span Θ(1)
        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqStPerS<T>, f: &F, seed: A) -> A
            requires forall|x: &A, y: &T| #[trigger] f.requires((x, y));

        /// Work Θ(|a|), Span Θ(1)
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, id: T) -> T
            where T: Clone
            requires forall|x: &T, y: &T| #[trigger] f.requires((x, y));

        /// Work Θ(|a|), Span Θ(1)
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, id: T) -> (scanned: (ArraySeqStPerS<T>, T))
            where T: Clone
            requires forall|x: &T, y: &T| #[trigger] f.requires((x, y))
            ensures scanned.0.seq@.len() == a.seq@.len();
    }


    //		9. impl BaseTrait for Struct

    impl<T> ArraySeqStPerBaseTrait<T> for ArraySeqStPerS<T> {
        open spec fn spec_len(&self) -> int {
            self.seq@.len() as int
        }

        open spec fn spec_index(&self, i: int) -> T {
            self.seq[i]
        }

        fn new(length: usize, init_value: T) -> (new_seq: ArraySeqStPerS<T>)
            where T: Clone + Eq
        {
            let seq = std::vec::from_elem(init_value, length);
            ArraySeqStPerS { seq }
        }

        fn length(&self) -> (len: usize) {
            self.seq.len()
        }

        fn nth(&self, index: usize) -> (nth_elem: &T) {
            &self.seq[index]
        }

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
                    forall|j: int| #![auto] 0 <= j < seq@.len() ==> seq@[j] == self.seq@[(start + j) as int],
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
            ArraySeqStPerS { seq }
        }

        fn from_vec(elts: Vec<T>) -> (seq: ArraySeqStPerS<T>) {
            ArraySeqStPerS { seq: elts }
        }
    }

    //		9. impl RedefinableTrait for Struct

    impl<T> ArraySeqStPerRedefinableTrait<T> for ArraySeqStPerS<T> {
        fn empty() -> (empty_seq: ArraySeqStPerS<T>) {
            ArraySeqStPerS { seq: Vec::new() }
        }

        fn singleton(item: T) -> (singleton: ArraySeqStPerS<T>) {
            let mut seq = Vec::with_capacity(1);
            seq.push(item);
            ArraySeqStPerS { seq }
        }

        fn append(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> (appended: ArraySeqStPerS<T>)
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
                    forall|k: int| #![auto] 0 <= k < a_len ==> seq@[k] == a.seq@[k],
                    forall|k: int| #![auto] 0 <= k < j ==> seq@[a_len as int + k] == b.seq@[k],
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
            ArraySeqStPerS { seq }
        }

        fn filter<F: Fn(&T) -> bool>(a: &ArraySeqStPerS<T>, pred: &F) -> (filtered: ArraySeqStPerS<T>)
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
                    forall|j: int| #![auto] 0 <= j < seq@.len() ==> pred.ensures((&seq@[j],), true),
                decreases len - i,
            {
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
            ArraySeqStPerS { seq }
        }

        fn update(a: &ArraySeqStPerS<T>, index: usize, item: T) -> (updated: ArraySeqStPerS<T>)
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
            ArraySeqStPerS { seq }
        }

        fn is_empty(&self) -> (empty: bool) {
            self.seq.len() == 0
        }

        fn is_singleton(&self) -> (single: bool) {
            self.seq.len() == 1
        }

        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqStPerS<T>, f: &F, seed: A) -> (acc: A) {
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

        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, id: T) -> (reduced: T)
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

        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, id: T) -> (scanned: (ArraySeqStPerS<T>, T))
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
            (ArraySeqStPerS { seq }, acc)
        }
    }

    //		9. bare impl (helpers not in any trait)

    impl<T> ArraySeqStPerS<T> {
        broadcast proof fn lemma_spec_index(&self, i: int)
            requires 0 <= i < self.spec_len()
            ensures #[trigger] self.seq@[i] == self.spec_index(i)
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

    //		9. module-level functions (avoid Verus cycle errors in traits)

    /// Algorithm 18.3 (tabulate). Build a sequence by applying `f` to each index.
    /// Module-level to avoid Verus cycle errors with spec_len/spec_index in traits.
    pub fn tabulate<T, F: Fn(usize) -> T>(f: &F, length: usize) -> (tab_seq: ArraySeqStPerS<T>)
        requires
            length <= usize::MAX,
            forall|i: usize| i < length ==> #[trigger] f.requires((i,)),
        ensures
            tab_seq.seq@.len() == length,
            forall|i: int| #![auto] 0 <= i < length ==> f.ensures((i as usize,), tab_seq.seq@[i]),
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
        ArraySeqStPerS { seq }
    }

    /// Algorithm 18.4 (map). Transform each element via `f`.
    /// Module-level because map returns ArraySeqStPerS<U> (different element type).
    pub fn map<T, U: Clone, F: Fn(&T) -> U>(a: &ArraySeqStPerS<T>, f: &F) -> (mapped: ArraySeqStPerS<U>)
        requires
            forall|i: int| 0 <= i < a.seq@.len() ==> #[trigger] f.requires((&a.seq@[i],)),
        ensures
            mapped.seq@.len() == a.seq@.len(),
            forall|i: int| #![auto] 0 <= i < a.seq@.len() ==> f.ensures((&a.seq@[i],), mapped.seq@[i]),
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
                forall|j: int| #![auto] 0 <= j < i ==> f.ensures((&a.seq@[j],), seq@[j]),
            decreases len - i,
        {
            seq.push(f(&a.seq[i]));
            i += 1;
        }
        ArraySeqStPerS { seq }
    }

    /// Definition 18.15 (flatten). Concatenate a sequence of sequences.
    /// Module-level because flatten takes nested concrete types.
    pub fn flatten<T: Clone + Eq>(a: &ArraySeqStPerS<ArraySeqStPerS<T>>) -> (flattened: ArraySeqStPerS<T>)
        requires
            obeys_feq_clone::<T>(),
        ensures
            flattened.seq@ =~= a.seq@.map_values(|inner: ArraySeqStPerS<T>| inner.seq@).flatten(),
    {
        let outer_len = a.seq.len();
        let mut seq: Vec<T> = Vec::new();
        let mut i: usize = 0;
        while i < outer_len
            invariant
                i <= outer_len,
                outer_len == a.seq@.len(),
                obeys_feq_clone::<T>(),
                seq@ =~= a.seq@.take(i as int).map_values(|inner: ArraySeqStPerS<T>| inner.seq@).flatten(),
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
                    seq@ =~= a.seq@.take(i as int).map_values(|inner: ArraySeqStPerS<T>| inner.seq@).flatten()
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
                let ghost prefix = a.seq@.take(i as int).map_values(|inner: ArraySeqStPerS<T>| inner.seq@);
                assert(a.seq@.take(i as int + 1).map_values(|inner: ArraySeqStPerS<T>| inner.seq@)
                    =~= prefix.push(a.seq@[i as int].seq@));
                prefix.lemma_flatten_push(a.seq@[i as int].seq@);
            }
            i += 1;
        }
        proof {
            assert(a.seq@.take(outer_len as int) =~= a.seq@);
        }
        ArraySeqStPerS { seq }
    }


    //		10. iterators

    /// Iterator wrapper with closed spec view for encapsulation.
    #[verifier::reject_recursive_types(T)]
    pub struct ArraySeqStPerIter<'a, T> {
        inner: std::slice::Iter<'a, T>,
    }

    impl<'a, T> View for ArraySeqStPerIter<'a, T> {
        type V = (int, Seq<T>);
        closed spec fn view(&self) -> (int, Seq<T>) { self.inner@ }
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

    /// Ghost iterator for ForLoopGhostIterator support.
    #[verifier::reject_recursive_types(T)]
    pub struct ArraySeqStPerGhostIterator<'a, T> {
        pub pos: int,
        pub elements: Seq<T>,
        pub phantom: core::marker::PhantomData<&'a T>,
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

        open spec fn ghost_ensures(&self) -> bool { self.pos == self.elements.len() }
        open spec fn ghost_decrease(&self) -> Option<int> { Some(self.elements.len() - self.pos) }

        open spec fn ghost_peek_next(&self) -> Option<T> {
            if 0 <= self.pos < self.elements.len() { Some(self.elements[self.pos]) } else { None }
        }

        open spec fn ghost_advance(&self, _exec_iter: &ArraySeqStPerIter<'a, T>) -> ArraySeqStPerGhostIterator<'a, T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, T> View for ArraySeqStPerGhostIterator<'a, T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.elements.take(self.pos) }
    }

    impl<'a, T> std::iter::IntoIterator for &'a ArraySeqStPerS<T> {
        type Item = &'a T;
        type IntoIter = ArraySeqStPerIter<'a, T>;
        fn into_iter(self) -> Self::IntoIter { ArraySeqStPerIter { inner: self.seq.iter() } }
    }

    impl<T> std::iter::IntoIterator for ArraySeqStPerS<T> {
        type Item = T;
        type IntoIter = IntoIter<T>;
        fn into_iter(self) -> Self::IntoIter { self.seq.into_iter() }
    }

    } // verus!


    //		13. derive impls outside verus!

    impl<T: Clone> Clone for ArraySeqStPerS<T> {
        fn clone(&self) -> Self { ArraySeqStPerS { seq: self.seq.clone() } }
    }

    impl<T: PartialEq> PartialEq for ArraySeqStPerS<T> {
        fn eq(&self, other: &Self) -> bool { self.seq == other.seq }
    }

    impl<T: Eq> Eq for ArraySeqStPerS<T> {}

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

}
