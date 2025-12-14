//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 18 algorithms for LinkedListStEph (ephemeral). Verusified using Vec internally.

pub mod LinkedListStEph {

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
    pub struct LinkedListStEphS<T> {
        pub seq: Vec<T>,
    }

    /// Trait for single-threaded ephemeral linked list sequences (Chapter 18).
    pub trait LinkedListStEphTrait<T>: Sized {
        fn new(length: usize, init_value: T) -> Self where T: Clone;
        fn set(&mut self, index: usize, item: T) -> Result<(), &'static str>;
        fn length(&self) -> usize;
        fn nth(&self, index: usize) -> &T;
        fn empty() -> Self;
        fn singleton(item: T) -> Self;
        fn tabulate<F: Fn(usize) -> T>(f: &F, length: usize) -> LinkedListStEphS<T>;
        fn map<U: Clone, F: Fn(&T) -> U>(a: &LinkedListStEphS<T>, f: &F) -> LinkedListStEphS<U>;
        fn subseq_copy(&self, start: usize, length: usize) -> Self where T: Clone;
        fn append(a: &LinkedListStEphS<T>, b: &LinkedListStEphS<T>) -> Self where T: Clone;
        fn filter<F: Fn(&T) -> bool>(a: &LinkedListStEphS<T>, pred: &F) -> Self where T: Clone;
        fn flatten(a: &LinkedListStEphS<LinkedListStEphS<T>>) -> Self where T: Clone;
        fn update(a: &LinkedListStEphS<T>, index: usize, item: T) -> Self where T: Clone;
        fn is_empty(&self) -> bool;
        fn is_singleton(&self) -> bool;
        fn iterate<A, F: Fn(&A, &T) -> A>(a: &LinkedListStEphS<T>, f: &F, seed: A) -> A;
        fn reduce<F: Fn(&T, &T) -> T>(a: &LinkedListStEphS<T>, f: &F, id: T) -> T where T: Clone;
        fn scan<F: Fn(&T, &T) -> T>(a: &LinkedListStEphS<T>, f: &F, id: T) -> (LinkedListStEphS<T>, T) where T: Clone;
        fn from_vec(elts: Vec<T>) -> Self;
    }

    impl<T: View> View for LinkedListStEphS<T> {
        type V = Seq<T::V>;

        open spec fn view(&self) -> Seq<T::V> {
            self.seq@.map(|_i: int, t: T| t@)
        }
    }

    #[verifier::reject_recursive_types(T)]
    pub struct LinkedListStEphIter<T> {
        pub elements: Vec<T>,
        pub pos: usize,
    }

    impl<T> View for LinkedListStEphIter<T> {
        type V = (int, Seq<T>);
        open spec fn view(&self) -> (int, Seq<T>) { (self.pos as int, self.elements@) }
    }

    pub open spec fn iter_invariant<T>(it: &LinkedListStEphIter<T>) -> bool { it.pos <= it.elements@.len() }

    // See experiments/simple_seq_iter.rs::assumption_free_next for a version that proves
    // without assume() by requiring iter_invariant. We can't add requires to Iterator::next in Verus.
    // and Rust iterators have 70 functions on them making this sensible requirement impossible.
    impl<T: Clone> Iterator for LinkedListStEphIter<T> {
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

    impl<T: View> LinkedListStEphS<T> {
        pub fn new(length: usize, init_value: T) -> (result: LinkedListStEphS<T>)
            where T: Clone
            requires length <= usize::MAX
            ensures result.seq@.len() == length
        {
            LinkedListStEphS { seq: vec![init_value; length] }
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

        pub fn empty() -> (result: LinkedListStEphS<T>)
            ensures result.seq@.len() == 0
        {
            LinkedListStEphS { seq: Vec::new() }
        }

        pub fn singleton(item: T) -> (result: LinkedListStEphS<T>)
            ensures result.seq@.len() == 1
        {
            let mut seq = Vec::with_capacity(1);
            seq.push(item);
            LinkedListStEphS { seq }
        }

        pub fn tabulate<F: Fn(usize) -> T>(f: &F, n: usize) -> (result: LinkedListStEphS<T>)
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
            LinkedListStEphS { seq }
        }

        pub fn map<U: Clone + View, F: Fn(&T) -> U>(a: &LinkedListStEphS<T>, f: &F) -> (result: LinkedListStEphS<U>)
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
            LinkedListStEphS { seq }
        }

        pub fn append(a: &LinkedListStEphS<T>, b: &LinkedListStEphS<T>) -> (result: LinkedListStEphS<T>)
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
            LinkedListStEphS { seq }
        }

        pub fn filter<F: Fn(&T) -> bool>(a: &LinkedListStEphS<T>, pred: &F) -> (result: LinkedListStEphS<T>)
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
            LinkedListStEphS { seq }
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

        pub fn from_vec(elts: Vec<T>) -> (result: LinkedListStEphS<T>)
            ensures result.seq@ == elts@
        {
            LinkedListStEphS { seq: elts }
        }

        pub fn iter(&self) -> (it: LinkedListStEphIter<T>)
            where T: Clone
            ensures
                it.elements@.len() == self.seq@.len(),
                forall|i: int| 0 <= i < self.seq@.len() ==> cloned(self.seq@[i], #[trigger] it.elements@[i]),
                it.pos == 0,
                iter_invariant(&it),
        {
            LinkedListStEphIter { elements: self.seq.clone(), pos: 0 }
        }

        pub fn subseq_copy(&self, start: usize, length: usize) -> (result: LinkedListStEphS<T>)
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
            LinkedListStEphS { seq }
        }

        pub fn reduce<F: Fn(&T, &T) -> T>(a: &LinkedListStEphS<T>, f: &F, id: T) -> (result: T)
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

        pub fn iterate<A, F: Fn(&A, &T) -> A>(a: &LinkedListStEphS<T>, f: &F, seed: A) -> (result: A)
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
    impl<T: Clone> Clone for LinkedListStEphS<T> {
        fn clone(&self) -> Self { LinkedListStEphS { seq: self.seq.clone() } }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: PartialEq> PartialEq for LinkedListStEphS<T> {
        fn eq(&self, other: &Self) -> bool { self.seq == other.seq }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: Eq> Eq for LinkedListStEphS<T> {}

    #[cfg(verus_keep_ghost)]
    impl<T: Debug> Debug for LinkedListStEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_list().entries(self.seq.iter()).finish()
        }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: Display> Display for LinkedListStEphS<T> {
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
    impl<'a, T> IntoIterator for &'a LinkedListStEphS<T> {
        type Item = &'a T;
        type IntoIter = Iter<'a, T>;
        fn into_iter(self) -> Self::IntoIter { self.seq.iter() }
    }

    #[cfg(verus_keep_ghost)]
    impl<T> IntoIterator for LinkedListStEphS<T> {
        type Item = T;
        type IntoIter = IntoIter<T>;
        fn into_iter(self) -> Self::IntoIter { self.seq.into_iter() }
    }

    // Non-Verus stub
    #[cfg(not(verus_keep_ghost))]
    #[derive(Clone, PartialEq, Eq)]
    pub struct LinkedListStEphS<T> {
        pub seq: Vec<T>,
    }

    #[cfg(not(verus_keep_ghost))]
    pub struct LinkedListStEphIter<T> {
        pub elements: Vec<T>,
        pub pos: usize,
    }

    #[cfg(not(verus_keep_ghost))]
    impl<T: Clone> Iterator for LinkedListStEphIter<T> {
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
    impl<T> LinkedListStEphS<T> {
        pub fn new(length: usize, init_value: T) -> Self where T: Clone {
            LinkedListStEphS { seq: vec![init_value; length] }
        }
        pub fn set(&mut self, index: usize, item: T) -> Result<(), &'static str> {
            if index < self.seq.len() { self.seq[index] = item; Ok(()) }
            else { Err("Index out of bounds") }
        }
        pub fn length(&self) -> usize { self.seq.len() }
        pub fn nth(&self, index: usize) -> &T { &self.seq[index] }
        pub fn empty() -> Self { LinkedListStEphS { seq: Vec::new() } }
        pub fn singleton(item: T) -> Self { LinkedListStEphS { seq: vec![item] } }
        pub fn tabulate<F: Fn(usize) -> T>(f: &F, n: usize) -> Self {
            LinkedListStEphS { seq: (0..n).map(f).collect() }
        }
        pub fn map<U, F: Fn(&T) -> U>(a: &Self, f: &F) -> LinkedListStEphS<U> {
            LinkedListStEphS { seq: a.seq.iter().map(f).collect() }
        }
        pub fn append(a: &Self, b: &Self) -> Self where T: Clone {
            let mut seq = a.seq.clone();
            seq.extend(b.seq.iter().cloned());
            LinkedListStEphS { seq }
        }
        pub fn filter<F: Fn(&T) -> bool>(a: &Self, pred: &F) -> Self where T: Clone {
            LinkedListStEphS { seq: a.seq.iter().filter(|x| pred(x)).cloned().collect() }
        }
        pub fn isEmpty(&self) -> bool { self.seq.is_empty() }
        pub fn isSingleton(&self) -> bool { self.seq.len() == 1 }
        pub fn from_vec(elts: Vec<T>) -> Self { LinkedListStEphS { seq: elts } }
        pub fn iter(&self) -> LinkedListStEphIter<T> where T: Clone {
            LinkedListStEphIter { elements: self.seq.clone(), pos: 0 }
        }
        pub fn iter_std(&self) -> Iter<'_, T> { self.seq.iter() }
        pub fn subseq_copy(&self, start: usize, length: usize) -> Self where T: Clone {
            let end = (start + length).min(self.seq.len());
            LinkedListStEphS { seq: self.seq[start..end].to_vec() }
        }
        pub fn reduce<F: Fn(&T, &T) -> T>(a: &Self, f: &F, id: T) -> T where T: Clone {
            a.seq.iter().fold(id, |acc, x| f(&acc, x))
        }
        pub fn iterate<A, F: Fn(&A, &T) -> A>(a: &Self, f: &F, seed: A) -> A {
            a.seq.iter().fold(seed, |acc, x| f(&acc, x))
        }
    }

    #[cfg(not(verus_keep_ghost))]
    impl<T: Debug> Debug for LinkedListStEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_list().entries(self.seq.iter()).finish()
        }
    }

    #[cfg(not(verus_keep_ghost))]
    impl<T: Display> Display for LinkedListStEphS<T> {
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
    impl<'a, T> IntoIterator for &'a LinkedListStEphS<T> {
        type Item = &'a T;
        type IntoIter = Iter<'a, T>;
        fn into_iter(self) -> Self::IntoIter { self.seq.iter() }
    }

    #[cfg(not(verus_keep_ghost))]
    impl<T> IntoIterator for LinkedListStEphS<T> {
        type Item = T;
        type IntoIter = IntoIter<T>;
        fn into_iter(self) -> Self::IntoIter { self.seq.into_iter() }
    }
}
