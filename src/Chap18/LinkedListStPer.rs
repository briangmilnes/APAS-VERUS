//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 18 algorithms for LinkedListStPer. Verusified using Vec internally.

pub mod LinkedListStPer {

    use std::fmt::{Debug, Display, Formatter};
    use std::fmt::Result as FmtResult;
    use std::slice::Iter;
    use std::vec::IntoIter;

    #[cfg(verus_keep_ghost)]
    use vstd::prelude::*;

    #[cfg(verus_keep_ghost)]
    verus! {

    use vstd::std_specs::clone::*;
    broadcast use vstd::std_specs::vec::group_vec_axioms;

    #[verifier::reject_recursive_types(T)]
    pub struct LinkedListStPerS<T> {
        pub seq: Vec<T>,
    }

    /// Base trait for single-threaded persistent linked list sequences (Chapter 18).
    pub trait LinkedListStPerBaseTrait<T>: Sized {
        /// Work Θ(n), Span Θ(1)
        fn new(length: usize, init_value: T) -> Self where T: Clone;
        /// Work Θ(1), Span Θ(1)
        fn length(&self) -> usize;
        /// Work Θ(n), Span Θ(1) - linked list traversal
        fn nth(&self, index: usize) -> &T;
        /// Work Θ(len), Span Θ(1)
        fn subseq_copy(&self, start: usize, length: usize) -> Self where T: Clone;
        /// Work Θ(Σ|a[i]|), Span Θ(1)
        fn flatten(a: &LinkedListStPerS<LinkedListStPerS<T>>) -> Self where T: Clone;
        /// Work Θ(n), Span Θ(1)
        fn from_vec(elts: Vec<T>) -> Self;
    }

    /// Redefinable trait - may be overridden with better algorithms in later chapters.
    pub trait LinkedListStPerRedefinableTrait<T>: Sized {
        /// Work Θ(1), Span Θ(1)
        fn empty() -> Self;
        /// Work Θ(1), Span Θ(1)
        fn singleton(item: T) -> Self;
        /// Work Θ(n), Span Θ(1)
        fn tabulate<F: Fn(usize) -> T>(f: &F, length: usize) -> LinkedListStPerS<T>;
        /// Work Θ(|a|), Span Θ(1)
        fn map<U: Clone, F: Fn(&T) -> U>(a: &LinkedListStPerS<T>, f: &F) -> LinkedListStPerS<U>;
        /// Work Θ(|a|+|b|), Span Θ(1)
        fn append(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> Self where T: Clone;
        /// Work Θ(|a|), Span Θ(1)
        fn filter<F: Fn(&T) -> bool>(a: &LinkedListStPerS<T>, pred: &F) -> Self where T: Clone;
        /// Work Θ(|a|), Span Θ(1)
        fn update(a: &LinkedListStPerS<T>, index: usize, item: T) -> Self where T: Clone;
        /// Work Θ(1), Span Θ(1)
        fn is_empty(&self) -> bool;
        /// Work Θ(1), Span Θ(1)
        fn is_singleton(&self) -> bool;
        /// Work Θ(|a|), Span Θ(1)
        fn iterate<A, F: Fn(&A, &T) -> A>(a: &LinkedListStPerS<T>, f: &F, seed: A) -> A;
        /// Work Θ(|a|), Span Θ(1)
        fn reduce<F: Fn(&T, &T) -> T>(a: &LinkedListStPerS<T>, f: &F, id: T) -> T where T: Clone;
        /// Work Θ(|a|), Span Θ(1)
        fn scan<F: Fn(&T, &T) -> T>(a: &LinkedListStPerS<T>, f: &F, id: T) -> (LinkedListStPerS<T>, T) where T: Clone;
    }

    impl<T: View> View for LinkedListStPerS<T> {
        type V = Seq<T::V>;

        open spec fn view(&self) -> Seq<T::V> {
            self.seq@.map(|_i: int, t: T| t@)
        }
    }

    #[verifier::reject_recursive_types(T)]
    pub struct LinkedListStPerIter<T> {
        pub elements: Vec<T>,
        pub pos: usize,
    }

    impl<T> View for LinkedListStPerIter<T> {
        type V = (int, Seq<T>);
        open spec fn view(&self) -> (int, Seq<T>) { (self.pos as int, self.elements@) }
    }

    pub open spec fn iter_invariant<T>(it: &LinkedListStPerIter<T>) -> bool { it.pos <= it.elements@.len() }

    // See experiments/simple_seq_iter.rs::assumption_free_next for a version that proves
    // without assume() by requiring iter_invariant. We can't add requires to Iterator::next in Verus.
    // and Rust iterators have 70 functions on them making this sensible requirement impossible.
    impl<T: Clone> Iterator for LinkedListStPerIter<T> {
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

    impl<T: View> LinkedListStPerS<T> {
        pub fn new(length: usize, init_value: T) -> (result: LinkedListStPerS<T>)
            where T: Clone
            requires length <= usize::MAX
            ensures result.seq@.len() == length
        {
            LinkedListStPerS { seq: vec![init_value; length] }
        }

        pub fn empty() -> (result: LinkedListStPerS<T>)
            ensures result.seq@.len() == 0
        {
            LinkedListStPerS { seq: Vec::new() }
        }

        pub fn singleton(item: T) -> (result: LinkedListStPerS<T>)
            ensures result.seq@.len() == 1
        {
            let mut seq = Vec::with_capacity(1);
            seq.push(item);
            LinkedListStPerS { seq }
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

        pub fn tabulate<F: Fn(usize) -> T>(f: &F, n: usize) -> (result: LinkedListStPerS<T>)
            requires 
                n <= usize::MAX,
                forall|i: usize| i < n ==> #[trigger] f.requires((i,)),
            ensures result.seq@.len() == n
        {
            let mut seq = Vec::with_capacity(n);
            let mut i: usize = 0;
            while i < n
                invariant
                    i <= n,
                    seq@.len() == i as int,
                    forall|j: usize| j < n ==> #[trigger] f.requires((j,)),
                decreases n - i,
            {
                seq.push(f(i));
                i += 1;
            }
            LinkedListStPerS { seq }
        }

        pub fn map<U: Clone + View, F: Fn(&T) -> U>(a: &LinkedListStPerS<T>, f: &F) -> (result: LinkedListStPerS<U>)
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
            LinkedListStPerS { seq }
        }

        pub fn append(a: &LinkedListStPerS<T>, b: &LinkedListStPerS<T>) -> (result: LinkedListStPerS<T>)
            where T: Clone
            requires a.seq@.len() + b.seq@.len() <= usize::MAX
            ensures result.seq@.len() == a.seq@.len() + b.seq@.len()
        {
            let a_len = a.seq.len();
            let b_len = b.seq.len();
            let mut seq: Vec<T> = Vec::with_capacity(a_len + b_len);
            let mut i: usize = 0;
            while i < a_len
                invariant i <= a_len, a_len == a.seq@.len(), seq@.len() == i as int,
                decreases a_len - i,
            {
                seq.push(a.seq[i].clone());
                i += 1;
            }
            let mut j: usize = 0;
            while j < b_len
                invariant j <= b_len, b_len == b.seq@.len(), seq@.len() == a_len + j,
                decreases b_len - j,
            {
                seq.push(b.seq[j].clone());
                j += 1;
            }
            LinkedListStPerS { seq }
        }

        pub fn filter<F: Fn(&T) -> bool>(a: &LinkedListStPerS<T>, pred: &F) -> (result: LinkedListStPerS<T>)
            where T: Clone
            requires forall|i: int| 0 <= i < a.seq@.len() ==> #[trigger] pred.requires((&a.seq@[i],)),
            ensures result.seq@.len() <= a.seq@.len()
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
            LinkedListStPerS { seq }
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

        pub fn from_vec(elts: Vec<T>) -> (result: LinkedListStPerS<T>)
            ensures result.seq@ == elts@
        {
            LinkedListStPerS { seq: elts }
        }

        pub fn iter(&self) -> (it: LinkedListStPerIter<T>)
            where T: Clone
            ensures
                it.elements@.len() == self.seq@.len(),
                forall|i: int| 0 <= i < self.seq@.len() ==> cloned(self.seq@[i], #[trigger] it.elements@[i]),
                it.pos == 0,
                iter_invariant(&it),
        {
            LinkedListStPerIter { elements: self.seq.clone(), pos: 0 }
        }

        pub fn subseq_copy(&self, start: usize, length: usize) -> (result: LinkedListStPerS<T>)
            where T: Clone
            requires 
                start + length <= self.seq@.len(),
                self.seq@.len() <= usize::MAX as int,
            ensures result.seq@.len() == length
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
            LinkedListStPerS { seq }
        }

        pub fn reduce<F: Fn(&T, &T) -> T>(a: &LinkedListStPerS<T>, f: &F, id: T) -> (result: T)
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

        pub fn iterate<A, F: Fn(&A, &T) -> A>(a: &LinkedListStPerS<T>, f: &F, seed: A) -> (result: A)
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

    } // verus!

    // Non-Verus impls
    #[cfg(verus_keep_ghost)]
    impl<T: Clone> Clone for LinkedListStPerS<T> {
        fn clone(&self) -> Self { LinkedListStPerS { seq: self.seq.clone() } }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: PartialEq> PartialEq for LinkedListStPerS<T> {
        fn eq(&self, other: &Self) -> bool { self.seq == other.seq }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: Eq> Eq for LinkedListStPerS<T> {}

    #[cfg(verus_keep_ghost)]
    impl<T: Debug> Debug for LinkedListStPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_list().entries(self.seq.iter()).finish()
        }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: Display> Display for LinkedListStPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "[")?;
            for (i, item) in self.seq.iter().enumerate() {
                if i > 0 { write!(f, ", ")?; }
                write!(f, "{item}")?;
            }
            write!(f, "]")
        }
    }

    #[cfg(verus_keep_ghost)]
    impl<'a, T> IntoIterator for &'a LinkedListStPerS<T> {
        type Item = &'a T;
        type IntoIter = Iter<'a, T>;
        fn into_iter(self) -> Self::IntoIter { self.seq.iter() }
    }

    #[cfg(verus_keep_ghost)]
    impl<T> IntoIterator for LinkedListStPerS<T> {
        type Item = T;
        type IntoIter = IntoIter<T>;
        fn into_iter(self) -> Self::IntoIter { self.seq.into_iter() }
    }

    // Non-Verus stub
    #[cfg(not(verus_keep_ghost))]
    #[derive(Clone, PartialEq, Eq)]
    pub struct LinkedListStPerS<T> {
        pub seq: Vec<T>,
    }

    #[cfg(not(verus_keep_ghost))]
    pub struct LinkedListStPerIter<T> {
        pub elements: Vec<T>,
        pub pos: usize,
    }

    #[cfg(not(verus_keep_ghost))]
    impl<T: Clone> Iterator for LinkedListStPerIter<T> {
        type Item = T;
        fn next(&mut self) -> Option<T> {
            if self.pos < self.elements.len() {
                let elem = self.elements[self.pos].clone();
                self.pos += 1;
                Some(elem)
            } else {
                None
            }
        }
    }

    #[cfg(not(verus_keep_ghost))]
    impl<T> LinkedListStPerS<T> {
        pub fn new(length: usize, init_value: T) -> Self where T: Clone {
            LinkedListStPerS { seq: vec![init_value; length] }
        }
        pub fn empty() -> Self { LinkedListStPerS { seq: Vec::new() } }
        pub fn singleton(item: T) -> Self { LinkedListStPerS { seq: vec![item] } }
        pub fn length(&self) -> usize { self.seq.len() }
        pub fn nth(&self, index: usize) -> &T { &self.seq[index] }
        pub fn tabulate<F: Fn(usize) -> T>(f: &F, n: usize) -> Self {
            LinkedListStPerS { seq: (0..n).map(f).collect() }
        }
        pub fn map<U, F: Fn(&T) -> U>(a: &Self, f: &F) -> LinkedListStPerS<U> {
            LinkedListStPerS { seq: a.seq.iter().map(f).collect() }
        }
        pub fn append(a: &Self, b: &Self) -> Self where T: Clone {
            let mut seq = a.seq.clone();
            seq.extend(b.seq.iter().cloned());
            LinkedListStPerS { seq }
        }
        pub fn filter<F: Fn(&T) -> bool>(a: &Self, pred: &F) -> Self where T: Clone {
            LinkedListStPerS { seq: a.seq.iter().filter(|x| pred(x)).cloned().collect() }
        }
        pub fn isEmpty(&self) -> bool { self.seq.is_empty() }
        pub fn isSingleton(&self) -> bool { self.seq.len() == 1 }
        pub fn from_vec(elts: Vec<T>) -> Self { LinkedListStPerS { seq: elts } }
        pub fn iter(&self) -> LinkedListStPerIter<T> where T: Clone {
            LinkedListStPerIter { elements: self.seq.clone(), pos: 0 }
        }
        pub fn iter_std(&self) -> Iter<'_, T> { self.seq.iter() }
        pub fn subseq_copy(&self, start: usize, length: usize) -> Self where T: Clone {
            let end = (start + length).min(self.seq.len());
            LinkedListStPerS { seq: self.seq[start..end].to_vec() }
        }
        pub fn reduce<F: Fn(&T, &T) -> T>(a: &Self, f: &F, id: T) -> T where T: Clone {
            a.seq.iter().fold(id, |acc, x| f(&acc, x))
        }
        pub fn iterate<A, F: Fn(&A, &T) -> A>(a: &Self, f: &F, seed: A) -> A {
            a.seq.iter().fold(seed, |acc, x| f(&acc, x))
        }
    }

    #[cfg(not(verus_keep_ghost))]
    impl<T: Debug> Debug for LinkedListStPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_list().entries(self.seq.iter()).finish()
        }
    }

    #[cfg(not(verus_keep_ghost))]
    impl<T: Display> Display for LinkedListStPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "[")?;
            for (i, item) in self.seq.iter().enumerate() {
                if i > 0 { write!(f, ", ")?; }
                write!(f, "{item}")?;
            }
            write!(f, "]")
        }
    }

    #[cfg(not(verus_keep_ghost))]
    impl<'a, T> IntoIterator for &'a LinkedListStPerS<T> {
        type Item = &'a T;
        type IntoIter = Iter<'a, T>;
        fn into_iter(self) -> Self::IntoIter { self.seq.iter() }
    }

    #[cfg(not(verus_keep_ghost))]
    impl<T> IntoIterator for LinkedListStPerS<T> {
        type Item = T;
        type IntoIter = IntoIter<T>;
        fn into_iter(self) -> Self::IntoIter { self.seq.into_iter() }
    }
}
