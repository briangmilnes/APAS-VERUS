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

    use vstd::std_specs::clone::*;
    broadcast use vstd::std_specs::vec::group_vec_axioms;

    #[verifier::reject_recursive_types(T)]
    pub struct ArraySeqS<T> {
        pub seq: Vec<T>,
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

    // Non-Verus impls (Clone, Display, Debug, Iterator)
    #[cfg(verus_keep_ghost)]
    impl<T: Clone> Clone for ArraySeqS<T> {
        fn clone(&self) -> Self {
            ArraySeqS { seq: self.seq.clone() }
        }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: PartialEq> PartialEq for ArraySeqS<T> {
        fn eq(&self, other: &Self) -> bool { self.seq == other.seq }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: Eq> Eq for ArraySeqS<T> {}

    #[cfg(verus_keep_ghost)]
    impl<T: Debug> Debug for ArraySeqS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_list().entries(self.seq.iter()).finish()
        }
    }

    #[cfg(verus_keep_ghost)]
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

    #[cfg(verus_keep_ghost)]
    impl<'a, T> IntoIterator for &'a ArraySeqS<T> {
        type Item = &'a T;
        type IntoIter = Iter<'a, T>;
        fn into_iter(self) -> Self::IntoIter { self.seq.iter() }
    }

    #[cfg(verus_keep_ghost)]
    impl<'a, T> IntoIterator for &'a mut ArraySeqS<T> {
        type Item = &'a mut T;
        type IntoIter = IterMut<'a, T>;
        fn into_iter(self) -> Self::IntoIter { self.seq.iter_mut() }
    }

    #[cfg(verus_keep_ghost)]
    impl<T> IntoIterator for ArraySeqS<T> {
        type Item = T;
        type IntoIter = IntoIter<T>;
        fn into_iter(self) -> Self::IntoIter { self.seq.into_iter() }
    }

    // Non-Verus stub for cargo compilation
    #[cfg(not(verus_keep_ghost))]
    #[derive(Clone, PartialEq, Eq)]
    pub struct ArraySeqS<T> {
        pub seq: Vec<T>,
    }

    #[cfg(not(verus_keep_ghost))]
    pub struct ArraySeqIter<T> {
        pub elements: Vec<T>,
        pub pos: usize,
    }

    #[cfg(not(verus_keep_ghost))]
    impl<T: Clone> Iterator for ArraySeqIter<T> {
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
    impl<T> ArraySeqS<T> {
        pub fn new(length: usize, init_value: T) -> ArraySeqS<T> where T: Clone {
            ArraySeqS { seq: vec![init_value; length] }
        }
        pub fn set(&mut self, index: usize, item: T) -> Result<(), &'static str> {
            if index < self.seq.len() { self.seq[index] = item; Ok(()) }
            else { Err("Index out of bounds") }
        }
        pub fn length(&self) -> usize { self.seq.len() }
        pub fn nth(&self, index: usize) -> &T { &self.seq[index] }
        pub fn empty() -> ArraySeqS<T> { ArraySeqS { seq: Vec::new() } }
        pub fn singleton(item: T) -> ArraySeqS<T> { ArraySeqS { seq: vec![item] } }
        pub fn tabulate<F: Fn(usize) -> T>(f: &F, length: usize) -> ArraySeqS<T> {
            ArraySeqS { seq: (0..length).map(f).collect() }
        }
        pub fn map<U: Clone, F: Fn(&T) -> U>(a: &ArraySeqS<T>, f: &F) -> ArraySeqS<U> {
            ArraySeqS { seq: a.seq.iter().map(f).collect() }
        }
        pub fn isEmpty(&self) -> bool { self.seq.is_empty() }
        pub fn isSingleton(&self) -> bool { self.seq.len() == 1 }
        pub fn from_vec(elts: Vec<T>) -> ArraySeqS<T> { ArraySeqS { seq: elts } }
        pub fn iter(&self) -> ArraySeqIter<T> where T: Clone {
            ArraySeqIter { elements: self.seq.clone(), pos: 0 }
        }
        pub fn iter_std(&self) -> Iter<'_, T> { self.seq.iter() }
        pub fn iter_mut(&mut self) -> IterMut<'_, T> { self.seq.iter_mut() }
    }

    #[cfg(not(verus_keep_ghost))]
    impl<T: Debug> Debug for ArraySeqS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_list().entries(self.seq.iter()).finish()
        }
    }

    #[cfg(not(verus_keep_ghost))]
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

    #[cfg(not(verus_keep_ghost))]
    impl<'a, T> IntoIterator for &'a ArraySeqS<T> {
        type Item = &'a T;
        type IntoIter = Iter<'a, T>;
        fn into_iter(self) -> Self::IntoIter { self.seq.iter() }
    }

    #[cfg(not(verus_keep_ghost))]
    impl<'a, T> IntoIterator for &'a mut ArraySeqS<T> {
        type Item = &'a mut T;
        type IntoIter = IterMut<'a, T>;
        fn into_iter(self) -> Self::IntoIter { self.seq.iter_mut() }
    }

    #[cfg(not(verus_keep_ghost))]
    impl<T> IntoIterator for ArraySeqS<T> {
        type Item = T;
        type IntoIter = IntoIter<T>;
        fn into_iter(self) -> Self::IntoIter { self.seq.into_iter() }
    }
}
