//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! The simplest possible version, ignoring parallelism. Verusified.

pub mod ArraySeq {

    use std::fmt::{Debug, Display, Formatter};
    use std::fmt::Result as FmtResult;
    use std::slice::{Iter, IterMut};
    use std::vec::IntoIter;

    use vstd::prelude::*;

    verus! {

    #[cfg(verus_keep_ghost)]
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::vec::*;
    #[cfg(verus_keep_ghost)]
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::clone::*;
    broadcast use vstd::std_specs::vec::group_vec_axioms;

    #[verifier::reject_recursive_types(T)]
    pub struct ArraySeqS<T> {
        pub seq: Vec<T>,
    }

    /// Data Type 18.1: Generic sequence trait for array-backed sequences.
    pub trait ArraySeqTrait<T>: Sized {
        /// Create a new sequence of length `length` with each element initialized to `init_value`.
        /// Work Θ(length), Span Θ(1).
        fn new(length: usize, init_value: T) -> Self where T: Clone;

        /// Set the element at `index` to `item` in place.
        /// Work Θ(1), Span Θ(1).
        fn set(&mut self, index: usize, item: T) -> Result<(), &'static str>;

        /// Definition 18.1 (length). Return the number of elements.
        /// Work Θ(1), Span Θ(1).
        fn length(&self) -> usize;

        /// Algorithm 19.11 (Function nth). Return a reference to the element at `index`.
        /// Work Θ(1), Span Θ(1).
        fn nth(&self, index: usize) -> &T;

        /// Definition 18.1 (empty). Construct the empty sequence.
        /// Work Θ(1), Span Θ(1).
        fn empty() -> Self;

        /// Definition 18.1 (singleton). Construct a singleton sequence containing `item`.
        /// Work Θ(1), Span Θ(1).
        fn singleton(item: T) -> Self;

        /// Algorithm 18.3 (tabulate). Build a sequence by applying `f` to each index.
        /// Work Θ(length), Span Θ(1).
        fn tabulate<F: Fn(usize) -> T>(f: &F, length: usize) -> ArraySeqS<T>;

        /// Algorithm 18.4 (map). Transform each element via `f`.
        /// Work Θ(|a|), Span Θ(1).
        fn map<U: Clone, F: Fn(&T) -> U>(a: &ArraySeqS<T>, f: &F) -> ArraySeqS<U>;

        /// Definition 18.12 (subseq). Extract a contiguous subsequence.
        /// Work Θ(length), Span Θ(1).
        fn subseq(a: &ArraySeqS<T>, start: usize, length: usize) -> Self where T: Clone;

        /// Definition 18.13 (append). Concatenate two sequences.
        /// Work Θ(|a| + |b|), Span Θ(1).
        fn append(a: &ArraySeqS<T>, b: &ArraySeqS<T>) -> Self where T: Clone;

        /// Definition 18.14 (filter). Keep elements satisfying `pred`.
        /// Work Θ(|a|), Span Θ(1).
        fn filter<F: Fn(&T) -> bool>(a: &ArraySeqS<T>, pred: &F) -> Self where T: Clone;

        /// Definition 18.15 (flatten). Concatenate a sequence of sequences.
        /// Work Θ(total length), Span Θ(1).
        fn flatten(a: &ArraySeqS<ArraySeqS<T>>) -> Self where T: Clone;

        /// Definition 18.16 (update). Return a copy with the index replaced by the new value.
        /// Work Θ(|a|), Span Θ(1).
        fn update(a: &ArraySeqS<T>, index: usize, item: T) -> Self where T: Clone;

        /// Definition 18.5 (isEmpty). true iff the sequence has length zero.
        /// Work Θ(1), Span Θ(1).
        fn is_empty(&self) -> bool;

        /// Definition 18.5 (isSingleton). true iff the sequence has length one.
        /// Work Θ(1), Span Θ(1).
        fn is_singleton(&self) -> bool;

        /// Definition 18.7 (iterate). Fold with accumulator `seed`.
        /// Work Θ(|a|), Span Θ(1).
        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqS<T>, f: &F, seed: A) -> A;

        /// Definition 18.18 (reduce). Combine elements using associative `f` and identity `id`.
        /// Work Θ(|a|), Span Θ(1).
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqS<T>, f: &F, id: T) -> T where T: Clone;

        /// Definition 18.19 (scan). Prefix-reduce returning partial sums and total.
        /// Work Θ(|a|), Span Θ(1).
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqS<T>, f: &F, id: T) -> (ArraySeqS<T>, T) where T: Clone;

        /// Definition 18.12 (subseq copy). Extract contiguous subsequence with allocation.
        /// Work Θ(length), Span Θ(1).
        fn subseq_copy(&self, start: usize, length: usize) -> ArraySeqS<T> where T: Clone;

        /// Create sequence from Vec.
        /// Work Θ(n) worst case, Θ(1) best case, Span Θ(1).
        fn from_vec(elts: Vec<T>) -> Self;
    }

    impl<T: View> View for ArraySeqS<T> {
        type V = Seq<T::V>;

        open spec fn view(&self) -> Seq<T::V> {
            self.seq@.map(|_i: int, t: T| t@)
        }
    }

    #[verifier::reject_recursive_types(T)]
    pub struct ArraySeqIter<T> {
        pub elements: Vec<T>,
        pub pos: usize,
    }

    impl<T> View for ArraySeqIter<T> {
        type V = (int, Seq<T>);
        open spec fn view(&self) -> (int, Seq<T>) { (self.pos as int, self.elements@) }
    }

    pub open spec fn iter_invariant<T>(it: &ArraySeqIter<T>) -> bool { it.pos <= it.elements@.len() }

    // See experiments/simple_seq_iter.rs::assumption_free_next for a version that proves
    // without assume() by requiring iter_invariant. We can't add requires to Iterator::next in Verus.
    // and Rust iterators have 70 functions on them making this sensible requirement impossible.
    impl<T: Clone> Iterator for ArraySeqIter<T> {
        type Item = T;

        fn next(&mut self) -> (result: Option<T>)
            ensures
                self.pos <= self.elements.len(),
                ({
                    let (old_index, old_seq) = old(self)@;
                    match result {
                        None => {
                            &&& self@ == old(self)@
                            &&& old_index == old_seq.len()
                            &&& self.pos == old_seq.len()
                        },
                        Some(element) => {
                            let (new_index, new_seq) = self@;
                            &&& 0 <= old_index < old_seq.len()
                            &&& new_seq == old_seq
                            &&& new_index == old_index + 1
                            &&& vstd::pervasive::cloned(old_seq[old_index as int], element)
                        },
                    }
                }),
        {
            if self.pos < self.elements.len() {
                let elem = self.elements[self.pos].clone();
                self.pos = self.pos + 1;
                Some(elem)
            } else {
                assume(self.pos <= self.elements.len());
                None
            }
        }
    }

    impl<T: View> ArraySeqS<T> {
        pub fn new(length: usize, init_value: T) -> (result: ArraySeqS<T>)
            where T: Clone
            requires length <= usize::MAX
            ensures result@.len() == length
        {
            let seq = vec![init_value; length];
            ArraySeqS { seq }
        }

        pub fn set(&mut self, index: usize, item: T) -> (result: Result<(), &'static str>)
            requires index < old(self).seq@.len()
            ensures result.is_ok() ==> self.seq@.len() == old(self).seq@.len()
        {
            if index < self.seq.len() {
                self.seq.set(index, item);
                Ok(())
            } else {
                Err("Index out of bounds")
            }
        }

        pub fn length(&self) -> (len: usize)
            ensures len == self.seq@.len()
        {
            self.seq.len()
        }

        pub fn nth(&self, index: usize) -> (result: &T)
            requires index < self.seq@.len()
        {
            &self.seq[index]
        }

        pub fn empty() -> (result: ArraySeqS<T>)
            ensures result.seq@.len() == 0
        {
            ArraySeqS { seq: Vec::new() }
        }

        pub fn singleton(item: T) -> (result: ArraySeqS<T>)
            ensures result.seq@.len() == 1
        {
            let mut seq = Vec::with_capacity(1);
            seq.push(item);
            ArraySeqS { seq }
        }

        pub fn tabulate<F: Fn(usize) -> T>(f: &F, length: usize) -> (result: ArraySeqS<T>)
            requires 
                length <= usize::MAX,
                forall|i: usize| i < length ==> #[trigger] f.requires((i,)),
            ensures result.seq@.len() == length
        {
            let mut seq = Vec::with_capacity(length);
            let mut i: usize = 0;
            while i < length
                invariant
                    i <= length,
                    seq@.len() == i as int,
                    forall|j: usize| j < length ==> #[trigger] f.requires((j,)),
                decreases length - i,
            {
                seq.push(f(i));
                i += 1;
            }
            ArraySeqS { seq }
        }

        pub fn map<U: Clone + View, F: Fn(&T) -> U>(a: &ArraySeqS<T>, f: &F) -> (result: ArraySeqS<U>)
            requires forall|i: int| 0 <= i < a.seq@.len() ==> #[trigger] f.requires((&a.seq@[i],)),
            ensures result.seq@.len() == a.seq@.len()
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
                decreases len - i,
            {
                seq.push(f(&a.seq[i]));
                i += 1;
            }
            ArraySeqS { seq }
        }

        pub fn isEmpty(&self) -> (empty: bool)
            ensures empty <==> self.seq@.len() == 0
        {
            self.seq.len() == 0
        }

        pub fn isSingleton(&self) -> (single: bool)
            ensures single <==> self.seq@.len() == 1
        {
            self.seq.len() == 1
        }

        pub fn from_vec(elts: Vec<T>) -> (result: ArraySeqS<T>)
            ensures result.seq@ == elts@
        {
            ArraySeqS { seq: elts }
        }

        pub fn iter(&self) -> (it: ArraySeqIter<T>)
            where T: Clone
            ensures
                it.elements@.len() == self.seq@.len(),
                forall|i: int| 0 <= i < self.seq@.len() ==> cloned(self.seq@[i], #[trigger] it.elements@[i]),
                it.pos == 0,
                iter_invariant(&it),
        {
            ArraySeqIter { elements: self.seq.clone(), pos: 0 }
        }
    }

    } // verus!

    // Helper methods outside verus! (use std types Verus doesn't support)
    impl<T> ArraySeqS<T> {
        pub fn iter_std(&self) -> Iter<'_, T> { self.seq.iter() }
        pub fn iter_mut(&mut self) -> IterMut<'_, T> { self.seq.iter_mut() }
    }

    // Trait impls outside verus! - work in both modes
    impl<T: Clone> Clone for ArraySeqS<T> {
        fn clone(&self) -> Self {
            ArraySeqS { seq: self.seq.clone() }
        }
    }

    impl<T: PartialEq> PartialEq for ArraySeqS<T> {
        fn eq(&self, other: &Self) -> bool { self.seq == other.seq }
    }

    impl<T: Eq> Eq for ArraySeqS<T> {}

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

    impl<'a, T> IntoIterator for &'a ArraySeqS<T> {
        type Item = &'a T;
        type IntoIter = Iter<'a, T>;
        fn into_iter(self) -> Self::IntoIter { self.seq.iter() }
    }

    impl<'a, T> IntoIterator for &'a mut ArraySeqS<T> {
        type Item = &'a mut T;
        type IntoIter = IterMut<'a, T>;
        fn into_iter(self) -> Self::IntoIter { self.seq.iter_mut() }
    }

    impl<T> IntoIterator for ArraySeqS<T> {
        type Item = T;
        type IntoIter = IntoIter<T>;
        fn into_iter(self) -> Self::IntoIter { self.seq.into_iter() }
    }
}
