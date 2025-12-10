//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! The simplest possible version, ignoring parallelism. Verusified.

pub mod ArraySeq {

    use std::fmt::{Debug, Display, Formatter};
    use std::fmt::Result as FmtResult;
    use std::slice::{Iter, IterMut};
    use std::vec::IntoIter;

    #[cfg(verus_keep_ghost)]
    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::vec::*;

    #[cfg(verus_keep_ghost)]
    verus! {

    #[verifier::reject_recursive_types(T)]
    pub struct ArraySeqS<T> {
        pub data: Vec<T>,
    }

    impl<T: View> View for ArraySeqS<T> {
        type V = Seq<T::V>;

        open spec fn view(&self) -> Seq<T::V> {
            self.data@.map(|_i: int, t: T| t@)
        }
    }

    impl<T: View> ArraySeqS<T> {
        pub fn new(length: usize, init_value: T) -> (result: ArraySeqS<T>)
            where T: Clone
            requires length <= usize::MAX
            ensures result@.len() == length
        {
            let data = vec![init_value; length];
            ArraySeqS { data }
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

        pub fn empty() -> (result: ArraySeqS<T>)
            ensures result.data@.len() == 0
        {
            ArraySeqS { data: Vec::new() }
        }

        pub fn singleton(item: T) -> (result: ArraySeqS<T>)
            ensures result.data@.len() == 1
        {
            let mut data = Vec::with_capacity(1);
            data.push(item);
            ArraySeqS { data }
        }

        pub fn tabulate<F: Fn(usize) -> T>(f: &F, length: usize) -> (result: ArraySeqS<T>)
            requires 
                length <= usize::MAX,
                forall|i: usize| i < length ==> #[trigger] f.requires((i,)),
            ensures result.data@.len() == length
        {
            let mut data = Vec::with_capacity(length);
            let mut i: usize = 0;
            while i < length
                invariant
                    i <= length,
                    data@.len() == i as int,
                    forall|j: usize| j < length ==> #[trigger] f.requires((j,)),
                decreases length - i,
            {
                data.push(f(i));
                i += 1;
            }
            ArraySeqS { data }
        }

        pub fn map<U: Clone + View, F: Fn(&T) -> U>(a: &ArraySeqS<T>, f: &F) -> (result: ArraySeqS<U>)
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
            ArraySeqS { data }
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

        pub fn from_vec(elts: Vec<T>) -> (result: ArraySeqS<T>)
            ensures result.data@ == elts@
        {
            ArraySeqS { data: elts }
        }

        #[verifier::external_body]
        pub fn iter(&self) -> Iter<'_, T> {
            self.data.iter()
        }

    }

    } // verus!

    // Non-Verus impls (Clone, Display, Debug, Iterator)
    #[cfg(verus_keep_ghost)]
    impl<T: Clone> Clone for ArraySeqS<T> {
        fn clone(&self) -> Self {
            ArraySeqS { data: self.data.clone() }
        }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: PartialEq> PartialEq for ArraySeqS<T> {
        fn eq(&self, other: &Self) -> bool { self.data == other.data }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: Eq> Eq for ArraySeqS<T> {}

    #[cfg(verus_keep_ghost)]
    impl<T: Debug> Debug for ArraySeqS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_list().entries(self.data.iter()).finish()
        }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: Display> Display for ArraySeqS<T> {
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
    impl<'a, T> IntoIterator for &'a ArraySeqS<T> {
        type Item = &'a T;
        type IntoIter = Iter<'a, T>;
        fn into_iter(self) -> Self::IntoIter { self.data.iter() }
    }

    #[cfg(verus_keep_ghost)]
    impl<'a, T> IntoIterator for &'a mut ArraySeqS<T> {
        type Item = &'a mut T;
        type IntoIter = IterMut<'a, T>;
        fn into_iter(self) -> Self::IntoIter { self.data.iter_mut() }
    }

    #[cfg(verus_keep_ghost)]
    impl<T> IntoIterator for ArraySeqS<T> {
        type Item = T;
        type IntoIter = IntoIter<T>;
        fn into_iter(self) -> Self::IntoIter { self.data.into_iter() }
    }

    // Non-Verus stub for cargo compilation
    #[cfg(not(verus_keep_ghost))]
    #[derive(Clone, PartialEq, Eq)]
    pub struct ArraySeqS<T> {
        pub data: Vec<T>,
    }

    #[cfg(not(verus_keep_ghost))]
    impl<T> ArraySeqS<T> {
        pub fn new(length: usize, init_value: T) -> ArraySeqS<T> where T: Clone {
            ArraySeqS { data: vec![init_value; length] }
        }
        pub fn set(&mut self, index: usize, item: T) -> Result<(), &'static str> {
            if index < self.data.len() { self.data[index] = item; Ok(()) }
            else { Err("Index out of bounds") }
        }
        pub fn length(&self) -> usize { self.data.len() }
        pub fn nth(&self, index: usize) -> &T { &self.data[index] }
        pub fn empty() -> ArraySeqS<T> { ArraySeqS { data: Vec::new() } }
        pub fn singleton(item: T) -> ArraySeqS<T> { ArraySeqS { data: vec![item] } }
        pub fn tabulate<F: Fn(usize) -> T>(f: &F, length: usize) -> ArraySeqS<T> {
            ArraySeqS { data: (0..length).map(f).collect() }
        }
        pub fn map<U: Clone, F: Fn(&T) -> U>(a: &ArraySeqS<T>, f: &F) -> ArraySeqS<U> {
            ArraySeqS { data: a.data.iter().map(f).collect() }
        }
        pub fn isEmpty(&self) -> bool { self.data.is_empty() }
        pub fn isSingleton(&self) -> bool { self.data.len() == 1 }
        pub fn from_vec(elts: Vec<T>) -> ArraySeqS<T> { ArraySeqS { data: elts } }
        pub fn iter(&self) -> Iter<'_, T> { self.data.iter() }
        pub fn iter_mut(&mut self) -> IterMut<'_, T> { self.data.iter_mut() }
    }

    #[cfg(not(verus_keep_ghost))]
    impl<T: Debug> Debug for ArraySeqS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_list().entries(self.data.iter()).finish()
        }
    }

    #[cfg(not(verus_keep_ghost))]
    impl<T: Display> Display for ArraySeqS<T> {
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
    impl<'a, T> IntoIterator for &'a ArraySeqS<T> {
        type Item = &'a T;
        type IntoIter = Iter<'a, T>;
        fn into_iter(self) -> Self::IntoIter { self.data.iter() }
    }

    #[cfg(not(verus_keep_ghost))]
    impl<'a, T> IntoIterator for &'a mut ArraySeqS<T> {
        type Item = &'a mut T;
        type IntoIter = IterMut<'a, T>;
        fn into_iter(self) -> Self::IntoIter { self.data.iter_mut() }
    }

    #[cfg(not(verus_keep_ghost))]
    impl<T> IntoIterator for ArraySeqS<T> {
        type Item = T;
        type IntoIter = IntoIter<T>;
        fn into_iter(self) -> Self::IntoIter { self.data.into_iter() }
    }
}
