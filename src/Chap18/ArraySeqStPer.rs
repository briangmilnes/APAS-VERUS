//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Chapter 18 persistent sequence implementation for array-backed sequences. Verusified.

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


pub mod ArraySeqStPer {

    use std::fmt::{Debug, Display, Formatter};
    use std::fmt::Result as FmtResult;
    use std::slice::Iter;
    use std::vec::IntoIter;

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

    verus! {

    //		2. imports

    #[cfg(verus_keep_ghost)]
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::clone::*;


    //		3. broadcast use

    broadcast use vstd::std_specs::vec::group_vec_axioms;


    //		4. type definitions

    #[verifier::reject_recursive_types(T)]
    pub struct ArraySeqStPerS<T> {
        pub seq: Vec<T>,
    }

    /// Iterator wrapper with closed spec view for encapsulation.
    #[verifier::reject_recursive_types(T)]
    pub struct ArraySeqStPerIter<'a, T> {
        inner: std::slice::Iter<'a, T>,
    }

    /// Ghost iterator for ForLoopGhostIterator support.
    #[verifier::reject_recursive_types(T)]
    pub struct ArraySeqStPerGhostIterator<'a, T> {
        pub pos: int,
        pub elements: Seq<T>,
        pub phantom: core::marker::PhantomData<&'a T>,
    }


    //		5. view impls

    impl<T: View> View for ArraySeqStPerS<T> {
        type V = Seq<T::V>;

        open spec fn view(&self) -> Seq<T::V> {
            self.seq@.map(|_i: int, t: T| t@)
        }
    }

    impl<'a, T> View for ArraySeqStPerIter<'a, T> {
        type V = (int, Seq<T>);
        closed spec fn view(&self) -> (int, Seq<T>) { self.inner@ }
    }

    impl<'a, T> View for ArraySeqStPerGhostIterator<'a, T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.elements.take(self.pos) }
    }


    //		6. spec fns

    pub open spec fn iter_invariant<'a, T>(it: &ArraySeqStPerIter<'a, T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }


    //		8. traits

    /// - Base trait for single-threaded persistent array sequences (Chapter 18).
    /// - These methods are never redefined in later chapters.
    pub trait ArraySeqStPerBaseTrait<T>: Sized {
        spec fn spec_len(&self) -> int;

        /// Work Θ(n), Span Θ(1)
        fn new(length: usize, init_value: T) -> (new_seq: Self)
            where T: Clone
            requires length <= usize::MAX
            ensures new_seq.spec_len() == length as int;

        /// Work Θ(1), Span Θ(1)
        fn length(&self) -> (len: usize)
            ensures len as int == self.spec_len();

        /// Work Θ(1), Span Θ(1)
        fn nth(&self, index: usize) -> (nth_elem: &T)
            requires index < self.spec_len();

        /// Work Θ(len), Span Θ(1)
        fn subseq_copy(&self, start: usize, length: usize) -> (subseq: Self)
            where T: Clone
            requires start + length <= self.spec_len()
            ensures subseq.spec_len() == length as int;

        /// Work Θ(n), Span Θ(1)
        fn from_vec(elts: Vec<T>) -> (seq: Self)
            ensures seq.spec_len() == elts@.len();
    }

    /// Redefinable trait - may be overridden with better algorithms in later chapters.
    pub trait ArraySeqStPerRedefinableTrait<T>: Sized {
        spec fn spec_len(&self) -> int;

        /// Work Θ(1), Span Θ(1)
        fn empty() -> (empty_seq: Self)
            ensures empty_seq.spec_len() == 0;

        /// Work Θ(1), Span Θ(1)
        fn singleton(item: T) -> (singleton: Self)
            ensures singleton.spec_len() == 1;

        /// Work Θ(n), Span Θ(1)
        fn tabulate<F: Fn(usize) -> T>(f: &F, length: usize) -> (tab_seq: ArraySeqStPerS<T>)
            requires
                length <= usize::MAX,
                forall|i: usize| i < length ==> #[trigger] f.requires((i,)),
            ensures
                tab_seq.seq@.len() == length,
                forall|i: int| #![auto] 0 <= i < length ==> f.ensures((i as usize,), tab_seq.seq@[i]);

        /// Work Θ(|a|), Span Θ(1)
        fn map<U: Clone, F: Fn(&T) -> U>(a: &ArraySeqStPerS<T>, f: &F) -> (mapped: ArraySeqStPerS<U>)
            requires
                forall|i: int| 0 <= i < a.seq@.len() ==> #[trigger] f.requires((&a.seq@[i],)),
            ensures
                mapped.seq@.len() == a.seq@.len(),
                forall|i: int| #![auto] 0 <= i < a.seq@.len() ==> f.ensures((&a.seq@[i],), mapped.seq@[i]);

        /// Work Θ(|a|+|b|), Span Θ(1)
        fn append(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> (appended: Self)
            where T: Clone
            requires a.seq@.len() + b.seq@.len() <= usize::MAX as int
            ensures appended.spec_len() == a.seq@.len() + b.seq@.len();

        /// Work Θ(|a|), Span Θ(1)
        fn filter<F: Fn(&T) -> bool>(a: &ArraySeqStPerS<T>, pred: &F) -> (filtered: Self)
            where T: Clone
            requires forall|i: int| 0 <= i < a.seq@.len() ==> #[trigger] pred.requires((&a.seq@[i],))
            ensures filtered.spec_len() <= a.seq@.len();

        /// Work Θ(Σ|a[i]|), Span Θ(1)
        fn flatten(a: &ArraySeqStPerS<ArraySeqStPerS<T>>) -> (flattened: Self)
            where T: Clone
            ensures a.seq@.len() == 0 ==> flattened.spec_len() == 0;

        /// Work Θ(|a|), Span Θ(1)
        fn update(a: &ArraySeqStPerS<T>, index: usize, item: T) -> (updated: Self)
            where T: Clone
            requires index < a.seq@.len()
            ensures updated.spec_len() == a.seq@.len();

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


    //		9. impls

    impl<T> ArraySeqStPerS<T> {
        pub open spec fn spec_len(&self) -> int {
            self.seq@.len() as int
        }

        pub fn new(length: usize, init_value: T) -> (new_seq: ArraySeqStPerS<T>)
            where T: Clone
            requires length <= usize::MAX
            ensures new_seq.seq@.len() == length
        {
            ArraySeqStPerS { seq: vec![init_value; length] }
        }

        pub fn length(&self) -> (len: usize)
            ensures len == self.seq@.len()
        {
            self.seq.len()
        }

        pub fn nth(&self, index: usize) -> (nth_elem: &T)
            requires index < self.seq@.len()
            ensures *nth_elem == self.seq@[index as int]
        {
            &self.seq[index]
        }

        pub fn empty() -> (empty_seq: ArraySeqStPerS<T>)
            ensures empty_seq.seq@.len() == 0
        {
            ArraySeqStPerS { seq: Vec::new() }
        }

        pub fn singleton(item: T) -> (singleton: ArraySeqStPerS<T>)
            ensures singleton.seq@.len() == 1
        {
            let mut seq = Vec::with_capacity(1);
            seq.push(item);
            ArraySeqStPerS { seq }
        }

        pub fn tabulate<F: Fn(usize) -> T>(f: &F, length: usize) -> (tab_seq: ArraySeqStPerS<T>)
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

        pub fn map<U: Clone + View, F: Fn(&T) -> U>(a: &ArraySeqStPerS<T>, f: &F) -> (mapped: ArraySeqStPerS<U>)
            requires forall|i: int| 0 <= i < a.seq@.len() ==> #[trigger] f.requires((&a.seq@[i],)),
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

        pub fn append(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> (appended: ArraySeqStPerS<T>)
            where T: Clone
            requires a.seq@.len() + b.seq@.len() <= usize::MAX
            ensures appended.seq@.len() == a.seq@.len() + b.seq@.len()
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
                decreases a_len - i,
            {
                seq.push(a.seq[i].clone());
                i += 1;
            }
            let mut j: usize = 0;
            while j < b_len
                invariant
                    j <= b_len,
                    b_len == b.seq@.len(),
                    seq@.len() == a_len + j,
                decreases b_len - j,
            {
                seq.push(b.seq[j].clone());
                j += 1;
            }
            ArraySeqStPerS { seq }
        }

        pub fn filter<F: Fn(&T) -> bool>(a: &ArraySeqStPerS<T>, pred: &F) -> (filtered: ArraySeqStPerS<T>)
            where T: Clone
            requires forall|i: int| 0 <= i < a.seq@.len() ==> #[trigger] pred.requires((&a.seq@[i],)),
            ensures filtered.seq@.len() <= a.seq@.len()
        {
            let len = a.seq.len();
            let mut seq: Vec<T> = Vec::new();
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len == a.seq@.len(),
                    seq@.len() <= i,
                    forall|j: int| 0 <= j < a.seq@.len() ==> #[trigger] pred.requires((&a.seq@[j],)),
                decreases len - i,
            {
                if pred(&a.seq[i]) {
                    seq.push(a.seq[i].clone());
                }
                i += 1;
            }
            ArraySeqStPerS { seq }
        }

        pub fn is_empty(&self) -> (empty: bool)
            ensures empty <==> self.seq@.len() == 0
        {
            self.seq.len() == 0
        }

        pub fn is_singleton(&self) -> (single: bool)
            ensures single <==> self.seq@.len() == 1
        {
            self.seq.len() == 1
        }

        pub fn from_vec(elts: Vec<T>) -> (seq: ArraySeqStPerS<T>)
            ensures seq.seq@ == elts@
        {
            ArraySeqStPerS { seq: elts }
        }

        /// Returns an iterator over the sequence elements.
        pub fn iter(&self) -> (it: ArraySeqStPerIter<'_, T>)
            ensures
                it@.0 == 0,
                it@.1 == self.seq@,
                iter_invariant(&it),
        {
            ArraySeqStPerIter { inner: self.seq.iter() }
        }

        pub fn subseq_copy(&self, start: usize, length: usize) -> (subseq: ArraySeqStPerS<T>)
            where T: Clone
            requires 
                start + length <= self.seq@.len(),
                self.seq@.len() <= usize::MAX as int,
            ensures subseq.seq@.len() == length
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
                decreases end - i,
            {
                seq.push(self.seq[i].clone());
                i += 1;
            }
            ArraySeqStPerS { seq }
        }

        pub fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, id: T) -> (reduced: T)
            where T: Clone
            requires forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
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

        pub fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqStPerS<T>, f: &F, seed: A) -> (acc: A)
            requires forall|x: &A, y: &T| #[trigger] f.requires((x, y)),
        {
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
    }

    #[cfg(verus_keep_ghost)]
    impl<T: View + PartialEq> PartialEqSpecImpl for ArraySeqStPerS<T> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }


    //		10. iterators

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

    impl<'a, T> std::iter::IntoIterator for &'a ArraySeqStPerS<T> {
        type Item = &'a T;
        type IntoIter = ArraySeqStPerIter<'a, T>;
        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                it@.0 == 0,
                it@.1 == self.seq@,
                iter_invariant(&it),
        { ArraySeqStPerIter { inner: self.seq.iter() } }
    }

    impl<T> std::iter::IntoIterator for ArraySeqStPerS<T> {
        type Item = T;
        type IntoIter = IntoIter<T>;
        fn into_iter(self) -> (it: Self::IntoIter)
            ensures
                it@.0 == 0,
                it@.1 == self.seq@,
        { self.seq.into_iter() }
    }


    //		11. derive impls in verus!

    impl<T: Clone> Clone for ArraySeqStPerS<T> {
        fn clone(&self) -> Self { ArraySeqStPerS { seq: self.seq.clone() } }
    }

    impl<T: Eq + View> Eq for ArraySeqStPerS<T> {}

    impl<T: PartialEq + View> PartialEq for ArraySeqStPerS<T> {
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
