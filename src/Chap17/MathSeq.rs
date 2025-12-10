//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! Mathematical sequence backed by a growable vector. Dense domain 0..len-1.
//!
//! Abstract: Definition 17.1 (Sequence) — runtime-sized, dense-domain sequence (0..n-1),
//! using rust vector which is dense.

pub mod MathSeq {
    use std::fmt::{Debug, Display, Formatter};
    use std::hash::Hash;
    use std::slice::{Iter, IterMut};
    use std::vec::IntoIter;

    use vstd::prelude::*;

    use crate::Types::Types::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::obeys_key_model;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_full;
    use crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::HashSetWithViewPlus;
    use crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::HashSetWithViewPlusTrait;

    verus! {

broadcast use vstd::std_specs::vec::group_vec_axioms;

pub open spec fn valid_key_type<T: View + Clone + Eq>() -> bool {
    &&& obeys_key_model::<T>()
    &&& obeys_feq_full::<T>()
}

#[verifier::reject_recursive_types(T)]
pub struct MathSeqS<T: StT> {
    pub data: Vec<T>,
}

impl<T: StT> View for MathSeqS<T> {
    type V = Seq<T::V>;

    open spec fn view(&self) -> Seq<T::V> {
        self.data@.map_values(|t: T| t@)
    }
}

impl<T: StT + Hash> MathSeqS<T> {

    pub open spec fn spec_len(&self) -> nat {
        self@.len()
    }

    pub open spec fn spec_nth(&self, i: int) -> T::V
        recommends 0 <= i < self.spec_len(),
    {
        self@[i]
    }

    pub open spec fn spec_is_empty(&self) -> bool {
        self.spec_len() == 0
    }

    pub open spec fn spec_is_singleton(&self) -> bool {
        self.spec_len() == 1
    }

    pub fn new(length: N, init_value: T) -> (result: Self)
        ensures
            result.spec_len() == length,
            forall|i: int| #![auto] 0 <= i < length ==> cloned(init_value, result.data@[i]),
    {
        let v = vec![init_value; length];
        MathSeqS { data: v }
    }

    pub fn set(&mut self, index: N, value: T) -> (success: bool)
        ensures
            success ==> index < old(self).spec_len()
                && self.spec_len() == old(self).spec_len()
                && self@[index as int] == value@
                && forall|i: int| 0 <= i < self.spec_len() && i != index as int ==> self@[i] == old(self)@[i],
            !success ==> index >= old(self).spec_len() && self@ == old(self)@,
    {
        if index < self.data.len() {
            self.data.set(index, value);
            true
        } else {
            false
        }
    }

    pub fn length(&self) -> (len: N)
        ensures len == self.spec_len(),
    {
        self.data.len()
    }

    pub fn nth(&self, index: N) -> (elem: &T)
        requires index < self.spec_len(),
        ensures elem@ == self@[index as int],
    {
        &self.data[index]
    }

    pub fn empty() -> (result: Self)
        ensures result.spec_len() == 0,
    {
        MathSeqS { data: Vec::new() }
    }

    pub fn singleton(item: T) -> (result: Self)
        ensures
            result.spec_len() == 1,
            result@[0] == item@,
    {
        MathSeqS { data: vec![item] }
    }

    pub fn add_last(&mut self, value: T)
        ensures
            self.spec_len() == old(self).spec_len() + 1,
            self@[self.spec_len() - 1] == value@,
            forall|i: int| 0 <= i < old(self).spec_len() ==> self@[i] == old(self)@[i],
    {
        self.data.push(value);
    }

    pub fn delete_last(&mut self) -> (result: Option<T>)
        ensures
            old(self).spec_len() == 0 ==> result is None && self@ == old(self)@,
            old(self).spec_len() > 0 ==>
                result is Some
                && result->Some_0@ == old(self)@[old(self).spec_len() - 1]
                && self.spec_len() == old(self).spec_len() - 1
                && forall|i: int| 0 <= i < self.spec_len() ==> self@[i] == old(self)@[i],
    {
        self.data.pop()
    }

    pub fn is_empty(&self) -> (result: bool)
        ensures result == self.spec_is_empty(),
    {
        self.data.len() == 0
    }

    pub fn is_singleton(&self) -> (result: bool)
        ensures result == self.spec_is_singleton(),
    {
        self.data.len() == 1
    }

    pub fn from_vec(data: Vec<T>) -> (result: Self)
        ensures result.data@ == data@,
    {
        MathSeqS { data }
    }

    pub fn with_len(length: N, init_value: T) -> (result: Self)
        ensures
            result.spec_len() == length,
            forall|i: int| #![auto] 0 <= i < length ==> cloned(init_value, result.data@[i]),
    {
        Self::new(length, init_value)
    }

    #[verifier::external_body]
    pub fn subseq(&self, start: N, length: N) -> (result: &[T])
        ensures result@.len() <= length,
    {
        let n = self.data.len();
        let s = start.min(n);
        let e = start.saturating_add(length).min(n);
        &self.data[s..e]
    }

    #[verifier::external_body]
    pub fn subseq_copy(&self, start: N, length: N) -> (result: Self)
        ensures result.spec_len() <= length,
    {
        let n = self.data.len();
        let s = start.min(n);
        let e = start.saturating_add(length).min(n);
        if e <= s {
            return MathSeqS { data: Vec::new() };
        }
        MathSeqS { data: self.data[s..e].to_vec() }
    }

    pub fn domain(&self) -> (result: Vec<N>)
        ensures result@.len() == self.spec_len(),
    {
        let mut v = Vec::new();
        let len = self.data.len();
        let mut i: usize = 0;
        while i < len
            invariant
                i <= len,
                v@.len() == i as int,
            decreases len - i,
        {
            v.push(i);
            i = i + 1;
        }
        v
    }

    #[verifier::external_body]
    pub fn range(&self) -> (result: Vec<T>)
        requires valid_key_type::<T>(),
        ensures
            result@.len() <= self.data@.len(),
            result@.no_duplicates(),
    {
        let mut seen: HashSetWithViewPlus<T> = HashSetWithViewPlus::new();
        let mut out: Vec<T> = Vec::new();
        let mut i: usize = 0;
        while i < self.data.len() {
            let x = self.data[i].clone();
            if !seen.contains(&x) {
                seen.insert(x.clone());
                out.push(x);
            }
            i = i + 1;
        }
        out
    }

    #[verifier::external_body]
    pub fn multiset_range(&self) -> (result: Vec<(N, T)>)
        requires valid_key_type::<T>(),
        ensures result@.len() <= self.data@.len(),
    {
        use std::collections::hash_map::Entry;
        use std::collections::HashMap;
        let mut counts = HashMap::<T, N>::with_capacity(self.data.len());
        let mut order = Vec::<T>::new();
        for x in self.data.iter() {
            match counts.entry(x.clone()) {
                Entry::Vacant(e) => {
                    e.insert(1);
                    order.push(x.clone());
                }
                Entry::Occupied(mut e) => {
                    *e.get_mut() += 1;
                }
            }
        }
        order.into_iter().map(|x| (*counts.get(&x).unwrap(), x)).collect()
    }
}

    } // verus!

    // Clone implementation outside verus! block
    impl<T: StT> Clone for MathSeqS<T> {
        fn clone(&self) -> Self {
            MathSeqS { data: self.data.clone() }
        }
    }

    // Iterator methods outside verus! block
    impl<T: StT + Hash> MathSeqS<T> {
        pub fn iter(&self) -> Iter<'_, T> {
            self.data.iter()
        }

        pub fn iter_mut(&mut self) -> IterMut<'_, T> {
            self.data.iter_mut()
        }
    }

    impl<'a, T: StT> IntoIterator for &'a MathSeqS<T> {
        type Item = &'a T;
        type IntoIter = Iter<'a, T>;
        fn into_iter(self) -> Self::IntoIter {
            self.data.iter()
        }
    }

    impl<'a, T: StT> IntoIterator for &'a mut MathSeqS<T> {
        type Item = &'a mut T;
        type IntoIter = IterMut<'a, T>;
        fn into_iter(self) -> Self::IntoIter {
            self.data.iter_mut()
        }
    }

    impl<T: StT> IntoIterator for MathSeqS<T> {
        type Item = T;
        type IntoIter = IntoIter<T>;
        fn into_iter(self) -> Self::IntoIter {
            self.data.into_iter()
        }
    }

    impl<T: StT> PartialEq for MathSeqS<T> {
        fn eq(&self, other: &Self) -> bool {
            self.data == other.data
        }
    }

    impl<T: StT> Eq for MathSeqS<T> {}

    impl<T: StT> Debug for MathSeqS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.debug_list().entries(self.data.iter()).finish()
        }
    }

    impl<T: StT> Display for MathSeqS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "[")?;
            let mut first = true;
            for x in &self.data {
                if !first {
                    write!(f, ", ")?;
                } else {
                    first = false;
                }
                write!(f, "{x}")?;
            }
            write!(f, "]")
        }
    }

    #[macro_export]
    macro_rules! MathSeqSLit {
        () => {
            $crate::Chap17::MathSeq::MathSeq::MathSeqS::empty()
        };
        ($x:expr; $n:expr) => {
            $crate::Chap17::MathSeq::MathSeq::MathSeqS::with_len($n, $x)
        };
        ($($x:expr),* $(,)?) => {
            $crate::Chap17::MathSeq::MathSeq::MathSeqS::from_vec(vec![$($x),*])
        };
    }
}

