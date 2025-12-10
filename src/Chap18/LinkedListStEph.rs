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

    #[verifier::reject_recursive_types(T)]
    pub struct LinkedListStEphS<T> {
        pub data: Vec<T>,
    }

    impl<T: View> View for LinkedListStEphS<T> {
        type V = Seq<T::V>;

        open spec fn view(&self) -> Seq<T::V> {
            self.data@.map(|_i: int, t: T| t@)
        }
    }

    impl<T: View> LinkedListStEphS<T> {
        pub fn new(length: usize, init_value: T) -> (result: LinkedListStEphS<T>)
            where T: Clone
            requires length <= usize::MAX
            ensures result.data@.len() == length
        {
            LinkedListStEphS { data: vec![init_value; length] }
        }

        pub fn set(&mut self, index: usize, item: T) -> (result: Result<(), &'static str>)
            requires index < old(self).data@.len()
            ensures result.is_ok() ==> self.data@.len() == old(self).data@.len()
        {
            if index < self.data.len() {
                self.data.set(index, item);
                Ok(())
            } else {
                Err("Index out of bounds")
            }
        }

        pub fn length(&self) -> (len: usize)
            ensures len == self.data@.len()
        {
            self.data.len()
        }

        pub fn nth(&self, index: usize) -> (result: &T)
            requires index < self.data@.len()
        {
            &self.data[index]
        }

        pub fn empty() -> (result: LinkedListStEphS<T>)
            ensures result.data@.len() == 0
        {
            LinkedListStEphS { data: Vec::new() }
        }

        pub fn singleton(item: T) -> (result: LinkedListStEphS<T>)
            ensures result.data@.len() == 1
        {
            let mut data = Vec::with_capacity(1);
            data.push(item);
            LinkedListStEphS { data }
        }

        pub fn tabulate<F: Fn(usize) -> T>(f: &F, n: usize) -> (result: LinkedListStEphS<T>)
            requires 
                n <= usize::MAX,
                forall|i: usize| i < n ==> #[trigger] f.requires((i,)),
            ensures result.data@.len() == n
        {
            let mut data = Vec::with_capacity(n);
            let mut i: usize = 0;
            while i < n
                invariant
                    i <= n,
                    data@.len() == i as int,
                    forall|j: usize| j < n ==> #[trigger] f.requires((j,)),
                decreases n - i,
            {
                data.push(f(i));
                i += 1;
            }
            LinkedListStEphS { data }
        }

        pub fn map<U: Clone + View, F: Fn(&T) -> U>(a: &LinkedListStEphS<T>, f: &F) -> (result: LinkedListStEphS<U>)
            requires forall|i: int| 0 <= i < a.data@.len() ==> #[trigger] f.requires((&a.data@[i],)),
            ensures result.data@.len() == a.data@.len()
        {
            let len = a.data.len();
            let mut data: Vec<U> = Vec::with_capacity(len);
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len == a.data@.len(),
                    data@.len() == i as int,
                    forall|j: int| 0 <= j < a.data@.len() ==> #[trigger] f.requires((&a.data@[j],)),
                decreases len - i,
            {
                data.push(f(&a.data[i]));
                i += 1;
            }
            LinkedListStEphS { data }
        }

        pub fn append(a: &LinkedListStEphS<T>, b: &LinkedListStEphS<T>) -> (result: LinkedListStEphS<T>)
            where T: Clone
            requires a.data@.len() + b.data@.len() <= usize::MAX
            ensures result.data@.len() == a.data@.len() + b.data@.len()
        {
            let a_len = a.data.len();
            let b_len = b.data.len();
            let mut data: Vec<T> = Vec::with_capacity(a_len + b_len);
            let mut i: usize = 0;
            while i < a_len
                invariant i <= a_len, a_len == a.data@.len(), data@.len() == i as int,
                decreases a_len - i,
            {
                data.push(a.data[i].clone());
                i += 1;
            }
            let mut j: usize = 0;
            while j < b_len
                invariant j <= b_len, b_len == b.data@.len(), data@.len() == a_len + j,
                decreases b_len - j,
            {
                data.push(b.data[j].clone());
                j += 1;
            }
            LinkedListStEphS { data }
        }

        pub fn filter<F: Fn(&T) -> bool>(a: &LinkedListStEphS<T>, pred: &F) -> (result: LinkedListStEphS<T>)
            where T: Clone
            requires forall|i: int| 0 <= i < a.data@.len() ==> #[trigger] pred.requires((&a.data@[i],)),
            ensures result.data@.len() <= a.data@.len()
        {
            let len = a.data.len();
            let mut data: Vec<T> = Vec::new();
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len == a.data@.len(),
                    data@.len() <= i,
                    forall|j: int| 0 <= j < a.data@.len() ==> #[trigger] pred.requires((&a.data@[j],)),
                decreases len - i,
            {
                if pred(&a.data[i]) {
                    data.push(a.data[i].clone());
                }
                i += 1;
            }
            LinkedListStEphS { data }
        }

        pub fn isEmpty(&self) -> (empty: bool)
            ensures empty <==> self.data@.len() == 0
        {
            self.data.len() == 0
        }

        pub fn isSingleton(&self) -> (single: bool)
            ensures single <==> self.data@.len() == 1
        {
            self.data.len() == 1
        }

        pub fn from_vec(elts: Vec<T>) -> (result: LinkedListStEphS<T>)
            ensures result.data@ == elts@
        {
            LinkedListStEphS { data: elts }
        }

        #[verifier::external_body]
        pub fn iter(&self) -> Iter<'_, T> {
            self.data.iter()
        }

        pub fn subseq_copy(&self, start: usize, length: usize) -> (result: LinkedListStEphS<T>)
            where T: Clone
            requires 
                start + length <= self.data@.len(),
                self.data@.len() <= usize::MAX as int,
            ensures result.data@.len() == length
        {
            let end = start + length;
            let mut data: Vec<T> = Vec::with_capacity(length);
            let mut i: usize = start;
            while i < end
                invariant
                    start <= i <= end,
                    end == start + length,
                    end <= self.data@.len(),
                    data@.len() == (i - start) as int,
                decreases end - i,
            {
                data.push(self.data[i].clone());
                i += 1;
            }
            LinkedListStEphS { data }
        }

        pub fn reduce<F: Fn(&T, &T) -> T>(a: &LinkedListStEphS<T>, f: &F, id: T) -> (result: T)
            where T: Clone
            requires forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
        {
            let len = a.data.len();
            let mut acc = id;
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len == a.data@.len(),
                    forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                decreases len - i,
            {
                acc = f(&acc, &a.data[i]);
                i += 1;
            }
            acc
        }

        pub fn iterate<A, F: Fn(&A, &T) -> A>(a: &LinkedListStEphS<T>, f: &F, seed: A) -> (result: A)
            requires forall|x: &A, y: &T| #[trigger] f.requires((x, y)),
        {
            let len = a.data.len();
            let mut acc = seed;
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len == a.data@.len(),
                    forall|x: &A, y: &T| #[trigger] f.requires((x, y)),
                decreases len - i,
            {
                acc = f(&acc, &a.data[i]);
                i += 1;
            }
            acc
        }
    }

    } // verus!

    // Non-Verus impls
    #[cfg(verus_keep_ghost)]
    impl<T: Clone> Clone for LinkedListStEphS<T> {
        fn clone(&self) -> Self { LinkedListStEphS { data: self.data.clone() } }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: PartialEq> PartialEq for LinkedListStEphS<T> {
        fn eq(&self, other: &Self) -> bool { self.data == other.data }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: Eq> Eq for LinkedListStEphS<T> {}

    #[cfg(verus_keep_ghost)]
    impl<T: Debug> Debug for LinkedListStEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_list().entries(self.data.iter()).finish()
        }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: Display> Display for LinkedListStEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "[")?;
            for (i, item) in self.data.iter().enumerate() {
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
        fn into_iter(self) -> Self::IntoIter { self.data.iter() }
    }

    #[cfg(verus_keep_ghost)]
    impl<T> IntoIterator for LinkedListStEphS<T> {
        type Item = T;
        type IntoIter = IntoIter<T>;
        fn into_iter(self) -> Self::IntoIter { self.data.into_iter() }
    }

    // Non-Verus stub
    #[cfg(not(verus_keep_ghost))]
    #[derive(Clone, PartialEq, Eq)]
    pub struct LinkedListStEphS<T> {
        pub data: Vec<T>,
    }

    #[cfg(not(verus_keep_ghost))]
    impl<T> LinkedListStEphS<T> {
        pub fn new(length: usize, init_value: T) -> Self where T: Clone {
            LinkedListStEphS { data: vec![init_value; length] }
        }
        pub fn set(&mut self, index: usize, item: T) -> Result<(), &'static str> {
            if index < self.data.len() { self.data[index] = item; Ok(()) }
            else { Err("Index out of bounds") }
        }
        pub fn length(&self) -> usize { self.data.len() }
        pub fn nth(&self, index: usize) -> &T { &self.data[index] }
        pub fn empty() -> Self { LinkedListStEphS { data: Vec::new() } }
        pub fn singleton(item: T) -> Self { LinkedListStEphS { data: vec![item] } }
        pub fn tabulate<F: Fn(usize) -> T>(f: &F, n: usize) -> Self {
            LinkedListStEphS { data: (0..n).map(f).collect() }
        }
        pub fn map<U, F: Fn(&T) -> U>(a: &Self, f: &F) -> LinkedListStEphS<U> {
            LinkedListStEphS { data: a.data.iter().map(f).collect() }
        }
        pub fn append(a: &Self, b: &Self) -> Self where T: Clone {
            let mut data = a.data.clone();
            data.extend(b.data.iter().cloned());
            LinkedListStEphS { data }
        }
        pub fn filter<F: Fn(&T) -> bool>(a: &Self, pred: &F) -> Self where T: Clone {
            LinkedListStEphS { data: a.data.iter().filter(|x| pred(x)).cloned().collect() }
        }
        pub fn isEmpty(&self) -> bool { self.data.is_empty() }
        pub fn isSingleton(&self) -> bool { self.data.len() == 1 }
        pub fn from_vec(elts: Vec<T>) -> Self { LinkedListStEphS { data: elts } }
        pub fn iter(&self) -> Iter<'_, T> { self.data.iter() }
        pub fn subseq_copy(&self, start: usize, length: usize) -> Self where T: Clone {
            let end = (start + length).min(self.data.len());
            LinkedListStEphS { data: self.data[start..end].to_vec() }
        }
        pub fn reduce<F: Fn(&T, &T) -> T>(a: &Self, f: &F, id: T) -> T where T: Clone {
            a.data.iter().fold(id, |acc, x| f(&acc, x))
        }
        pub fn iterate<A, F: Fn(&A, &T) -> A>(a: &Self, f: &F, seed: A) -> A {
            a.data.iter().fold(seed, |acc, x| f(&acc, x))
        }
    }

    #[cfg(not(verus_keep_ghost))]
    impl<T: Debug> Debug for LinkedListStEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_list().entries(self.data.iter()).finish()
        }
    }

    #[cfg(not(verus_keep_ghost))]
    impl<T: Display> Display for LinkedListStEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "[")?;
            for (i, item) in self.data.iter().enumerate() {
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
        fn into_iter(self) -> Self::IntoIter { self.data.iter() }
    }

    #[cfg(not(verus_keep_ghost))]
    impl<T> IntoIterator for LinkedListStEphS<T> {
        type Item = T;
        type IntoIter = IntoIter<T>;
        fn into_iter(self) -> Self::IntoIter { self.data.into_iter() }
    }
}
