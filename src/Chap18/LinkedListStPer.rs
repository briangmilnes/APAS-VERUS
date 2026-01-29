//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 18 algorithms for LinkedListStPer. Verusified using Vec internally.

pub mod LinkedListStPer {

    use std::fmt::{Debug, Display, Formatter};
    use std::fmt::Result as FmtResult;
    use std::slice::Iter;
    use std::vec::IntoIter;

    use vstd::prelude::*;

    verus! {

    #[cfg(verus_keep_ghost)]
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

    /// Iterator wrapper with closed spec view for encapsulation.
    #[verifier::reject_recursive_types(T)]
    pub struct LinkedListStPerIter<'a, T> {
        inner: std::slice::Iter<'a, T>,
    }

    impl<'a, T> View for LinkedListStPerIter<'a, T> {
        type V = (int, Seq<T>);
        closed spec fn view(&self) -> (int, Seq<T>) { self.inner@ }
    }

    pub open spec fn iter_invariant<'a, T>(it: &LinkedListStPerIter<'a, T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<'a, T> std::iter::Iterator for LinkedListStPerIter<'a, T> {
        type Item = &'a T;

        #[verifier::external_body]
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
    pub struct LinkedListStPerGhostIterator<'a, T> {
        pub pos: int,
        pub elements: Seq<T>,
        pub phantom: core::marker::PhantomData<&'a T>,
    }

    impl<'a, T> vstd::pervasive::ForLoopGhostIteratorNew for LinkedListStPerIter<'a, T> {
        type GhostIter = LinkedListStPerGhostIterator<'a, T>;
        open spec fn ghost_iter(&self) -> LinkedListStPerGhostIterator<'a, T> {
            LinkedListStPerGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    impl<'a, T> vstd::pervasive::ForLoopGhostIterator for LinkedListStPerGhostIterator<'a, T> {
        type ExecIter = LinkedListStPerIter<'a, T>;
        type Item = T;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &LinkedListStPerIter<'a, T>) -> bool {
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

        open spec fn ghost_advance(&self, _exec_iter: &LinkedListStPerIter<'a, T>) -> LinkedListStPerGhostIterator<'a, T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, T> View for LinkedListStPerGhostIterator<'a, T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.elements.take(self.pos) }
    }

    impl<T> LinkedListStPerS<T> {
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

        /// Returns an iterator over the list elements.
        pub fn iter(&self) -> (it: LinkedListStPerIter<'_, T>)
            ensures
                it@.0 == 0,
                it@.1 == self.seq@,
                iter_invariant(&it),
        {
            LinkedListStPerIter { inner: self.seq.iter() }
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

    impl<'a, T> std::iter::IntoIterator for &'a LinkedListStPerS<T> {
        type Item = &'a T;
        type IntoIter = LinkedListStPerIter<'a, T>;
        fn into_iter(self) -> Self::IntoIter { LinkedListStPerIter { inner: self.seq.iter() } }
    }

    impl<T> std::iter::IntoIterator for LinkedListStPerS<T> {
        type Item = T;
        type IntoIter = IntoIter<T>;
        fn into_iter(self) -> Self::IntoIter { self.seq.into_iter() }
    }

    } // verus!

    // Non-Verus impls
    impl<T: Clone> Clone for LinkedListStPerS<T> {
        fn clone(&self) -> Self { LinkedListStPerS { seq: self.seq.clone() } }
    }

    impl<T: PartialEq> PartialEq for LinkedListStPerS<T> {
        fn eq(&self, other: &Self) -> bool { self.seq == other.seq }
    }

    impl<T: Eq> Eq for LinkedListStPerS<T> {}

    impl<T: Debug> Debug for LinkedListStPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_list().entries(self.seq.iter()).finish()
        }
    }

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
}
