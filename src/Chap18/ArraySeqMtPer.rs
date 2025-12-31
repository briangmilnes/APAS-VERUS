//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 18 algorithms for ArraySeqMtPer multithreaded persistent. Verusified.
//! Uses work-stealing Pool for parallel operations (map_par, reduce_par).

pub mod ArraySeqMtPer {

    use std::fmt::{Debug, Display, Formatter};
    use std::fmt::Result as FmtResult;
    use std::slice::Iter;
    use std::vec::IntoIter;

    use vstd::prelude::*;
    use crate::Chap02::WSSchedulerMtEph::WSSchedulerMtEph::{Pool, PoolTrait};
    use crate::vstdplus::clone_plus::clone_plus::{ClonePlus, clone_fn, clone_fn2, clone_pred};

    verus! {

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::clone::*;
    use crate::vstdplus::feq::feq::obeys_feq_clone;
    broadcast use {vstd::std_specs::vec::group_vec_axioms, crate::vstdplus::feq::feq::group_feq_axioms};

    #[verifier::reject_recursive_types(T)]
    pub struct ArraySeqMtPerS<T> {
        pub seq: Vec<T>,
    }

    /// Base trait for multi-threaded persistent array sequences (Chapter 18).
    pub trait ArraySeqMtPerBaseTrait<T>: Sized {
        /// Work Θ(n), Span Θ(log n)
        fn new(length: usize, init_value: T) -> Self where T: Clone;
        /// Work Θ(1), Span Θ(1)
        fn length(&self) -> usize;
        /// Work Θ(1), Span Θ(1)
        fn nth(&self, index: usize) -> &T;
        /// Work Θ(len), Span Θ(log len)
        fn subseq_copy(&self, start: usize, length: usize) -> Self where T: Clone;
        /// Work Θ(Σ|a[i]|), Span Θ(Σ|a[i]|)
        fn flatten(a: &ArraySeqMtPerS<ArraySeqMtPerS<T>>) -> Self where T: Clone;
        /// Work Θ(n), Span Θ(1)
        fn from_vec(elts: Vec<T>) -> Self;
    }

    /// Redefinable trait - may be overridden with better algorithms in later chapters.
    pub trait ArraySeqMtPerRedefinableTrait<T>: Sized {
        /// Work Θ(1), Span Θ(1)
        fn empty() -> Self;
        /// Work Θ(1), Span Θ(1)
        fn singleton(item: T) -> Self;
        /// Work Θ(n), Span Θ(n)
        fn tabulate<F: Fn(usize) -> T>(f: &F, length: usize) -> ArraySeqMtPerS<T>;
        /// Work Θ(|a|), Span Θ(log|a|)
        fn map<U: Clone, F: Fn(&T) -> U>(a: &ArraySeqMtPerS<T>, f: &F) -> ArraySeqMtPerS<U>;
        /// Work Θ(|a|+|b|), Span Θ(log(|a|+|b|))
        fn append(a: &ArraySeqMtPerS<T>, b: &ArraySeqMtPerS<T>) -> Self where T: Clone;
        /// Work Θ(|a|), Span Θ(|a|)
        fn filter<F: Fn(&T) -> bool>(a: &ArraySeqMtPerS<T>, pred: &F) -> Self where T: Clone;
        /// Work Θ(n), Span Θ(log n)
        fn update(a: &ArraySeqMtPerS<T>, index: usize, item: T) -> Self where T: Clone;
        /// Work Θ(1), Span Θ(1)
        fn is_empty(&self) -> bool;
        /// Work Θ(1), Span Θ(1)
        fn is_singleton(&self) -> bool;
        /// Work Θ(|a|), Span Θ(|a|)
        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqMtPerS<T>, f: &F, seed: A) -> A;
        /// Work Θ(|a|), Span Θ(log|a|)
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqMtPerS<T>, f: &F, id: T) -> T where T: Clone;
        /// Work Θ(|a|), Span Θ(log|a|)
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqMtPerS<T>, f: &F, id: T) -> (ArraySeqMtPerS<T>, T) where T: Clone;
    }

    impl<T: View> View for ArraySeqMtPerS<T> {
        type V = Seq<T::V>;

        open spec fn view(&self) -> Seq<T::V> {
            self.seq@.map(|_i: int, t: T| t@)
        }
    }

    #[verifier::reject_recursive_types(T)]
    pub struct ArraySeqMtPerIter<T> {
        pub elements: Vec<T>,
        pub pos: usize,
    }

    impl<T> View for ArraySeqMtPerIter<T> {
        type V = (int, Seq<T>);
        open spec fn view(&self) -> (int, Seq<T>) { (self.pos as int, self.elements@) }
    }

    pub open spec fn iter_invariant<T>(it: &ArraySeqMtPerIter<T>) -> bool { it.pos <= it.elements@.len() }

    // See experiments/simple_seq_iter.rs::assumption_free_next for a version that proves
    // without assume() by requiring iter_invariant. We can't add requires to Iterator::next in Verus
    // and Rust iterators have 70 functions on them making this sensible requirement impossible.
    impl<T: Clone> Iterator for ArraySeqMtPerIter<T> {
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

    impl<T> ArraySeqMtPerS<T> {
        pub fn new(length: usize, init_value: T) -> (result: ArraySeqMtPerS<T>)
            where T: Clone
            requires length <= usize::MAX
            ensures result.seq@.len() == length
        {
            ArraySeqMtPerS { seq: vec![init_value; length] }
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

        pub fn empty() -> (result: ArraySeqMtPerS<T>)
            ensures result.seq@.len() == 0
        {
            ArraySeqMtPerS { seq: Vec::new() }
        }

        pub fn singleton(item: T) -> (result: ArraySeqMtPerS<T>)
            ensures result.seq@.len() == 1
        {
            let mut seq = Vec::with_capacity(1);
            seq.push(item);
            ArraySeqMtPerS { seq }
        }

        pub fn tabulate<F: Fn(usize) -> T>(f: &F, length: usize) -> (result: ArraySeqMtPerS<T>)
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
            ArraySeqMtPerS { seq }
        }

        pub fn map<U: Clone + View, F: Fn(&T) -> U>(a: &ArraySeqMtPerS<T>, f: &F) -> (result: ArraySeqMtPerS<U>)
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
            ArraySeqMtPerS { seq }
        }

        pub fn map_par<U: Clone + View + Send + Sync + 'static, F: Fn(&T) -> U + Send + Sync + Clone + 'static>(
            pool: &Pool,
            a: &ArraySeqMtPerS<T>,
            f: F,
        ) -> (result: ArraySeqMtPerS<U>)
            where T: Clone + Send + Sync + Eq + 'static
            requires
                obeys_feq_clone::<T>(),
                forall|i: int| 0 <= i < a.seq@.len() ==> #[trigger] f.requires((&a.seq@[i],)),
            ensures result.seq@.len() == a.seq@.len()
            decreases a.seq@.len()
        {
            let len = a.seq.len();
            if len == 0 {
                ArraySeqMtPerS { seq: Vec::new() }
            } else if len == 1 {
                ArraySeqMtPerS { seq: vec![f(&a.seq[0])] }
            } else {
                let mid = len / 2;
                let left_seq = a.subseq_copy(0, mid);
                let right_seq = a.subseq_copy(mid, len - mid);
                let f1 = clone_fn(&f);
                let f2 = f;
                proof {
                    // Prove f1.requires on left_seq elements
                    assert forall|i: int| 0 <= i < left_seq.seq@.len() implies #[trigger] f1.requires((&left_seq.seq@[i],)) by {
                        assert(cloned(a.seq@[0 + i], left_seq.seq@[i]));
                        assert(a.seq@[i] == left_seq.seq@[i]);  // from obeys_feq_clone
                    }
                    // Prove f2.requires on right_seq elements
                    assert forall|i: int| 0 <= i < right_seq.seq@.len() implies #[trigger] f2.requires((&right_seq.seq@[i],)) by {
                        let orig_i = mid as int + i;
                        assert(cloned(a.seq@[mid as int + i], right_seq.seq@[i]));
                        assert(a.seq@[orig_i] == right_seq.seq@[i]);  // from obeys_feq_clone
                    }
                }
                let pool1 = pool.clone_plus();
                let pool2 = pool.clone_plus();

                let fa = move || -> (r: ArraySeqMtPerS<U>)
                    requires
                        forall|i: int| 0 <= i < left_seq.seq@.len() ==> #[trigger] f1.requires((&left_seq.seq@[i],)),
                    ensures r.seq@.len() == left_seq.seq@.len(),
                {
                    Self::map_par(&pool1, &left_seq, f1)
                };

                let fb = move || -> (r: ArraySeqMtPerS<U>)
                    requires
                        forall|i: int| 0 <= i < right_seq.seq@.len() ==> #[trigger] f2.requires((&right_seq.seq@[i],)),
                    ensures r.seq@.len() == right_seq.seq@.len(),
                {
                    Self::map_par(&pool2, &right_seq, f2)
                };

                let (left, right) = pool.join(fa, fb);
                ArraySeqMtPerS::<U>::append(&left, &right)
            }
        }

        pub fn append(a: &ArraySeqMtPerS<T>, b: &ArraySeqMtPerS<T>) -> (result: ArraySeqMtPerS<T>)
            where T: Clone
            requires a.seq@.len() + b.seq@.len() <= usize::MAX
            ensures result.seq@.len() == a.seq@.len() + b.seq@.len()
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
            ArraySeqMtPerS { seq }
        }

        pub fn filter<F: Fn(&T) -> bool>(a: &ArraySeqMtPerS<T>, pred: &F) -> (result: ArraySeqMtPerS<T>)
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
            ArraySeqMtPerS { seq }
        }

        pub fn filter_par<F: Fn(&T) -> bool + Send + Sync + Clone + 'static>(
            pool: &Pool,
            a: &ArraySeqMtPerS<T>,
            pred: F,
        ) -> (result: ArraySeqMtPerS<T>)
            where T: Clone + Send + Sync + Eq + 'static
            requires
                obeys_feq_clone::<T>(),
                forall|i: int| 0 <= i < a.seq@.len() ==> #[trigger] pred.requires((&a.seq@[i],)),
            ensures result.seq@.len() <= a.seq@.len()
            decreases a.seq@.len()
        {
            let len = a.seq.len();
            if len == 0 {
                ArraySeqMtPerS { seq: Vec::new() }
            } else if len == 1 {
                if pred(&a.seq[0]) {
                    ArraySeqMtPerS { seq: vec![a.seq[0].clone()] }
                } else {
                    ArraySeqMtPerS { seq: Vec::new() }
                }
            } else {
                let mid = len / 2;
                let left_seq = a.subseq_copy(0, mid);
                let right_seq = a.subseq_copy(mid, len - mid);
                let p1 = clone_pred(&pred);
                let p2 = pred;
                proof {
                    assert forall|i: int| 0 <= i < left_seq.seq@.len() implies #[trigger] p1.requires((&left_seq.seq@[i],)) by {
                        assert(cloned(a.seq@[0 + i], left_seq.seq@[i]));
                        assert(a.seq@[i] == left_seq.seq@[i]);
                    }
                    assert forall|i: int| 0 <= i < right_seq.seq@.len() implies #[trigger] p2.requires((&right_seq.seq@[i],)) by {
                        let orig_i = mid as int + i;
                        assert(cloned(a.seq@[mid as int + i], right_seq.seq@[i]));
                        assert(a.seq@[orig_i] == right_seq.seq@[i]);
                    }
                }
                let pool1 = pool.clone_plus();
                let pool2 = pool.clone_plus();

                let fa = move || -> (r: ArraySeqMtPerS<T>)
                    requires forall|i: int| 0 <= i < left_seq.seq@.len() ==> #[trigger] p1.requires((&left_seq.seq@[i],)),
                    ensures r.seq@.len() <= left_seq.seq@.len(),
                {
                    Self::filter_par(&pool1, &left_seq, p1)
                };

                let fb = move || -> (r: ArraySeqMtPerS<T>)
                    requires forall|i: int| 0 <= i < right_seq.seq@.len() ==> #[trigger] p2.requires((&right_seq.seq@[i],)),
                    ensures r.seq@.len() <= right_seq.seq@.len(),
                {
                    Self::filter_par(&pool2, &right_seq, p2)
                };

                let (left, right) = pool.join(fa, fb);
                Self::append(&left, &right)
            }
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

        pub fn from_vec(elts: Vec<T>) -> (result: ArraySeqMtPerS<T>)
            ensures result.seq@ == elts@
        {
            ArraySeqMtPerS { seq: elts }
        }

        pub fn subseq_copy(&self, start: usize, length: usize) -> (result: ArraySeqMtPerS<T>)
            where T: Clone
            requires 
                start + length <= self.seq@.len(),
                self.seq@.len() <= usize::MAX as int,
            ensures 
                result.seq@.len() == length,
                forall|j: int| 0 <= j < length ==> cloned(#[trigger] self.seq@[start as int + j], result.seq@[j]),
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
                    forall|j: int| 0 <= j < (i - start) as int ==> cloned(#[trigger] self.seq@[start as int + j], seq@[j]),
                decreases end - i,
            {
                seq.push(self.seq[i].clone());
                i += 1;
            }
            ArraySeqMtPerS { seq }
        }

        pub fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqMtPerS<T>, f: &F, id: T) -> (result: T)
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

        pub fn reduce_par<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(
            pool: &Pool,
            a: &ArraySeqMtPerS<T>,
            f: F,
            id: T,
        ) -> (result: T)
            where T: Clone + Send + Sync + 'static
            requires
                a.seq@.len() > 0,
                forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
            decreases a.seq@.len()
        {
            let len = a.seq.len();
            if len == 1 {
                a.seq[0].clone()
            } else {
                let mid = len / 2;
                let left_seq = a.subseq_copy(0, mid);
                let right_seq = a.subseq_copy(mid, len - mid);
                let f1 = clone_fn2(&f);
                let f2 = clone_fn2(&f);
                // clone_fn2 preserves requires: forall|x, y| f.requires((x,y)) == f1.requires((x,y))
                let id1 = id.clone();
                let id2 = id.clone();
                let pool1 = pool.clone_plus();
                let pool2 = pool.clone_plus();

                let fa = move || -> (r: T)
                    requires
                        left_seq.seq@.len() > 0,
                        forall|x: &T, y: &T| #[trigger] f1.requires((x, y)),
                {
                    Self::reduce_par(&pool1, &left_seq, f1, id1)
                };

                let fb = move || -> (r: T)
                    requires
                        right_seq.seq@.len() > 0,
                        forall|x: &T, y: &T| #[trigger] f2.requires((x, y)),
                {
                    Self::reduce_par(&pool2, &right_seq, f2, id2)
                };

                let (left, right) = pool.join(fa, fb);
                f(&left, &right)
            }
        }

        pub fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqMtPerS<T>, f: &F, seed: A) -> (result: A)
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

        pub fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqMtPerS<T>, f: &F, id: T) -> (result: (ArraySeqMtPerS<T>, T))
            where T: Clone
            requires forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
            ensures result.0.seq@.len() == a.seq@.len()
        {
            let len = a.seq.len();
            let mut acc = id;
            let mut seq: Vec<T> = Vec::with_capacity(len);
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len == a.seq@.len(),
                    seq@.len() == i as int,
                    forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                decreases len - i,
            {
                acc = f(&acc, &a.seq[i]);
                seq.push(acc.clone());
                i += 1;
            }
            (ArraySeqMtPerS { seq }, acc)
        }

        pub fn iter(&self) -> (it: ArraySeqMtPerIter<T>)
            where T: Clone
            ensures
                it.elements@.len() == self.seq@.len(),
                forall|i: int| 0 <= i < self.seq@.len() ==> cloned(self.seq@[i], #[trigger] it.elements@[i]),
                it.pos == 0,
                iter_invariant(&it),
        {
            ArraySeqMtPerIter { elements: self.seq.clone(), pos: 0 }
        }
    }

    impl<'a, T> std::iter::IntoIterator for &'a ArraySeqMtPerS<T> {
        type Item = &'a T;
        type IntoIter = Iter<'a, T>;
        fn into_iter(self) -> Self::IntoIter { self.seq.iter() }
    }

    impl<T> std::iter::IntoIterator for ArraySeqMtPerS<T> {
        type Item = T;
        type IntoIter = IntoIter<T>;
        fn into_iter(self) -> Self::IntoIter { self.seq.into_iter() }
    }

    } // verus!

    // Non-Verus impls
    impl<T: Clone> Clone for ArraySeqMtPerS<T> {
        fn clone(&self) -> Self { ArraySeqMtPerS { seq: self.seq.clone() } }
    }

    impl<T: PartialEq> PartialEq for ArraySeqMtPerS<T> {
        fn eq(&self, other: &Self) -> bool { self.seq == other.seq }
    }

    impl<T: Eq> Eq for ArraySeqMtPerS<T> {}

    impl<T: Debug> Debug for ArraySeqMtPerS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_list().entries(self.seq.iter()).finish()
        }
    }

    impl<T: Display> Display for ArraySeqMtPerS<T> {
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
