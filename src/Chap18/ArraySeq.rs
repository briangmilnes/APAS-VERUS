//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! The simplest possible version, ignoring parallelism. Verusified.

pub mod ArraySeq {

    use std::fmt::{Debug, Display, Formatter};
    use std::fmt::Result as FmtResult;
    use std::slice::{Iter, IterMut};
    use std::vec::IntoIter;

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

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
    pub trait ArraySeqTrait<T: View>: Sized {
        spec fn spec_len(&self) -> int;

        /// - Create a new sequence of length `length` with each element initialized to `init_value`.
        /// - Work Θ(length), Span Θ(1).
        fn new(length: usize, init_value: T) -> (new_seq: Self)
            where T: Clone
            requires length <= usize::MAX
            ensures new_seq.spec_len() == length as int;

        /// - Set the element at `index` to `item` in place.
        /// - Work Θ(1), Span Θ(1).
        fn set(&mut self, index: usize, item: T) -> (success: Result<(), &'static str>)
            requires index < old(self).spec_len()
            ensures success.is_ok() ==> self.spec_len() == old(self).spec_len();

        /// - Definition 18.1 (length). Return the number of elements.
        /// - Work Θ(1), Span Θ(1).
        fn length(&self) -> (len: usize)
            ensures len as int == self.spec_len();

        /// - Algorithm 19.11 (Function nth). Return a reference to the element at `index`.
        /// - Work Θ(1), Span Θ(1).
        fn nth(&self, index: usize) -> (nth_elem: &T)
            requires index < self.spec_len();

        /// - Definition 18.1 (empty). Construct the empty sequence.
        /// - Work Θ(1), Span Θ(1).
        fn empty() -> (empty_seq: Self)
            ensures empty_seq.spec_len() == 0;

        /// - Definition 18.1 (singleton). Construct a singleton sequence containing `item`.
        /// - Work Θ(1), Span Θ(1).
        fn singleton(item: T) -> (singleton: Self)
            ensures singleton.spec_len() == 1;

        /// - Algorithm 18.3 (tabulate). Build a sequence by applying `f` to each index.
        /// - Work Θ(length), Span Θ(1).
        fn tabulate<F: Fn(usize) -> T>(f: &F, length: usize) -> (tab_seq: ArraySeqS<T>)
            requires
                length <= usize::MAX,
                forall|i: usize| i < length ==> #[trigger] f.requires((i,)),
            ensures
                tab_seq.spec_len() == length as int,
                forall|i: int| #![auto] 0 <= i < length ==> f.ensures((i as usize,), tab_seq.seq@[i]);

        /// - Algorithm 18.4 (map). Transform each element via `f`.
        /// - Work Θ(|a|), Span Θ(1).
        fn map<U: Clone + View, F: Fn(&T) -> U>(a: &ArraySeqS<T>, f: &F) -> (mapped: ArraySeqS<U>)
            requires forall|i: int| 0 <= i < a.spec_len() ==> #[trigger] f.requires((&a.seq@[i],))
            ensures
                mapped.spec_len() == a.spec_len(),
                forall|i: int| #![auto] 0 <= i < a.spec_len() ==> f.ensures((&a.seq@[i],), mapped.seq@[i]);

        /// - Definition 18.12 (subseq). Extract a contiguous subsequence.
        /// - Work Θ(length), Span Θ(1).
        fn subseq(a: &ArraySeqS<T>, start: usize, length: usize) -> (subseq: Self)
            where T: Clone
            requires start + length <= a.spec_len()
            ensures subseq.spec_len() == length as int;

        /// - Definition 18.13 (append). Concatenate two sequences.
        /// - Work Θ(|a| + |b|), Span Θ(1).
        fn append(a: &ArraySeqS<T>, b: &ArraySeqS<T>) -> (appended: Self)
            where T: Clone
            requires a.spec_len() + b.spec_len() <= usize::MAX as int
            ensures appended.spec_len() == a.spec_len() + b.spec_len();

        /// - Definition 18.14 (filter). Keep elements satisfying `pred`.
        /// - Work Θ(|a|), Span Θ(1).
        fn filter<F: Fn(&T) -> bool>(a: &ArraySeqS<T>, pred: &F) -> (filtered: Self)
            where T: Clone
            requires forall|i: int| 0 <= i < a.spec_len() ==> #[trigger] pred.requires((&a.seq@[i],))
            ensures filtered.spec_len() <= a.spec_len();

        /// - Definition 18.15 (flatten). Concatenate a sequence of sequences.
        /// - Work Θ(total length), Span Θ(1).
        fn flatten(a: &ArraySeqS<ArraySeqS<T>>) -> (flattened: Self) where T: Clone;

        /// - Definition 18.16 (update). Return a copy with the index replaced by the new value.
        /// - Work Θ(|a|), Span Θ(1).
        fn update(a: &ArraySeqS<T>, index: usize, item: T) -> (updated: Self)
            where T: Clone
            requires index < a.spec_len()
            ensures updated.spec_len() == a.spec_len();

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
        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqS<T>, f: &F, seed: A) -> A
            requires forall|x: &A, y: &T| #[trigger] f.requires((x, y));

        /// - Definition 18.18 (reduce). Combine elements using associative `f` and identity `id`.
        /// - Work Θ(|a|), Span Θ(1).
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqS<T>, f: &F, id: T) -> T
            where T: Clone
            requires forall|x: &T, y: &T| #[trigger] f.requires((x, y));

        /// - Definition 18.19 (scan). Prefix-reduce returning partial sums and total.
        /// - Work Θ(|a|), Span Θ(1).
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqS<T>, f: &F, id: T) -> (scanned: (ArraySeqS<T>, T))
            where T: Clone
            requires forall|x: &T, y: &T| #[trigger] f.requires((x, y))
            ensures scanned.0.spec_len() == a.spec_len();

        /// - Definition 18.12 (subseq copy). Extract contiguous subsequence with allocation.
        /// - Work Θ(length), Span Θ(1).
        fn subseq_copy(&self, start: usize, length: usize) -> (subseq: ArraySeqS<T>)
            where T: Clone
            requires start + length <= self.spec_len()
            ensures subseq.spec_len() == length as int;

        /// - Create sequence from Vec.
        /// - Work Θ(n) worst case, Θ(1) best case, Span Θ(1).
        fn from_vec(elts: Vec<T>) -> (seq: Self)
            ensures seq.spec_len() == elts@.len();
    }

    impl<T: View> View for ArraySeqS<T> {
        type V = Seq<T::V>;

        open spec fn view(&self) -> Seq<T::V> {
            self.seq@.map(|_i: int, t: T| t@)
        }
    }

    /// Iterator wrapper with closed spec view for encapsulation.
    /// Inner is private; closed view() can access it but external code cannot see it.
    #[verifier::reject_recursive_types(T)]
    pub struct ArraySeqIter<'a, T> {
        inner: std::slice::Iter<'a, T>,  // PRIVATE
    }

    impl<'a, T> View for ArraySeqIter<'a, T> {
        type V = (int, Seq<T>);
        closed spec fn view(&self) -> (int, Seq<T>) {
            self.inner@
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

    /// Ghost iterator for ForLoopGhostIterator support (for-iter, for-borrow patterns).
    #[verifier::reject_recursive_types(T)]
    pub struct ArraySeqGhostIterator<'a, T> {
        pub pos: int,
        pub elements: Seq<T>,
        pub phantom: core::marker::PhantomData<&'a T>,
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

    impl<'a, T> View for ArraySeqGhostIterator<'a, T> {
        type V = Seq<T>;

        open spec fn view(&self) -> Seq<T> {
            self.elements.take(self.pos)
        }
    }

    impl<T: View> ArraySeqS<T> {
        pub open spec fn spec_len(&self) -> int {
            self.seq@.len() as int
        }

        pub fn new(length: usize, init_value: T) -> (new_seq: ArraySeqS<T>)
            where T: Clone
            requires length <= usize::MAX
            ensures new_seq.spec_len() == length as int
        {
            let seq = vec![init_value; length];
            ArraySeqS { seq }
        }

        pub fn set(&mut self, index: usize, item: T) -> (success: Result<(), &'static str>)
            requires index < old(self).spec_len()
            ensures success.is_ok() ==> self.spec_len() == old(self).spec_len()
        {
            if index < self.seq.len() {
                self.seq.set(index, item);
                Ok(())
            } else {
                Err("Index out of bounds")
            }
        }

        pub fn length(&self) -> (len: usize)
            ensures len as int == self.spec_len()
        {
            self.seq.len()
        }

        pub fn nth(&self, index: usize) -> (nth_elem: &T)
            requires index < self.spec_len()
        {
            &self.seq[index]
        }

        pub fn empty() -> (empty_seq: ArraySeqS<T>)
            ensures empty_seq.spec_len() == 0
        {
            ArraySeqS { seq: Vec::new() }
        }

        pub fn singleton(item: T) -> (singleton: ArraySeqS<T>)
            ensures singleton.spec_len() == 1
        {
            let mut seq = Vec::with_capacity(1);
            seq.push(item);
            ArraySeqS { seq }
        }

        pub fn tabulate<F: Fn(usize) -> T>(f: &F, length: usize) -> (tab_seq: ArraySeqS<T>)
            requires 
                length <= usize::MAX,
                forall|i: usize| i < length ==> #[trigger] f.requires((i,)),
            ensures
                tab_seq.spec_len() == length as int,
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
            ArraySeqS { seq }
        }

        pub fn map<U: Clone + View, F: Fn(&T) -> U>(a: &ArraySeqS<T>, f: &F) -> (mapped: ArraySeqS<U>)
            requires forall|i: int| 0 <= i < a.spec_len() ==> #[trigger] f.requires((&a.seq@[i],))
            ensures
                mapped.spec_len() == a.spec_len(),
                forall|i: int| #![auto] 0 <= i < a.spec_len() ==> f.ensures((&a.seq@[i],), mapped.seq@[i]),
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
            ArraySeqS { seq }
        }

        pub fn is_empty(&self) -> (empty: bool)
            ensures empty <==> self.spec_len() == 0
        {
            self.seq.len() == 0
        }

        pub fn is_singleton(&self) -> (single: bool)
            ensures single <==> self.spec_len() == 1
        {
            self.seq.len() == 1
        }

        pub fn from_vec(elts: Vec<T>) -> (seq: ArraySeqS<T>)
            ensures seq.spec_len() == elts@.len()
        {
            ArraySeqS { seq: elts }
        }

        /// Returns an iterator over the sequence elements.
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

    #[verifier::external]
    impl<T: View> ArraySeqS<T> {
        pub fn iter_mut(&mut self) -> IterMut<'_, T> { self.seq.iter_mut() }
    }

    impl<T: Clone> Clone for ArraySeqS<T> {
        fn clone(&self) -> Self {
            ArraySeqS { seq: self.seq.clone() }
        }
    }

    impl<T: View + PartialEq> PartialEqSpecImpl for ArraySeqS<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<T: Eq + View> Eq for ArraySeqS<T> {}

    impl<T: PartialEq + View> PartialEq for ArraySeqS<T> {
        fn eq(&self, other: &Self) -> (r: bool)
            ensures r == (self@ == other@)
        {
            let r = self.seq == other.seq;
            proof { assume(r == (self@ == other@)); }
            r
        }
    }

    } // verus!

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
