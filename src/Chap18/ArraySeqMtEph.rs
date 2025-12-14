//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 18 algorithms for ArraySeqMtEph multithreaded ephemeral. Verusified.
//! Uses work-stealing Pool for parallel operations (map_par, reduce_par, filter_par).

pub mod ArraySeqMtEph {

    use std::fmt::{Debug, Display, Formatter};
    use std::fmt::Result as FmtResult;
    use std::slice::Iter;
    use std::vec::IntoIter;

    #[cfg(verus_keep_ghost)]
    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap02::WSSchedulerMtEph::WSSchedulerMtEph::Pool;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::clone_plus::clone_plus::{ClonePlus, clone_fn, clone_fn2, clone_pred};

    #[cfg(verus_keep_ghost)]
    verus! {

    use vstd::std_specs::clone::*;
    use crate::vstdplus::feq::feq::obeys_feq_clone;
    broadcast use {vstd::std_specs::vec::group_vec_axioms, crate::vstdplus::feq::feq::group_feq_axioms};

    #[verifier::reject_recursive_types(T)]
    pub struct ArraySeqMtEphS<T> {
        pub seq: Vec<T>,
    }

    /// Base trait for multi-threaded ephemeral array sequences (Chapter 18).
    pub trait ArraySeqMtEphBaseTrait<T>: Sized {
        /// Work Θ(n), Span Θ(log n)
        fn new(length: usize, init_value: T) -> Self where T: Clone;
        /// Work Θ(1), Span Θ(1)
        fn set(&mut self, index: usize, item: T) -> Result<(), &'static str>;
        /// Work Θ(1), Span Θ(1)
        fn length(&self) -> usize;
        /// Work Θ(1), Span Θ(1)
        fn nth(&self, index: usize) -> &T;
        /// Work Θ(len), Span Θ(log len)
        fn subseq_copy(&self, start: usize, length: usize) -> Self where T: Clone;
        /// Work Θ(Σ|a[i]|), Span Θ(Σ|a[i]|)
        fn flatten(a: &ArraySeqMtEphS<ArraySeqMtEphS<T>>) -> Self where T: Clone;
        /// Work Θ(n), Span Θ(1)
        fn from_vec(elts: Vec<T>) -> Self;
    }

    /// Redefinable trait - may be overridden with better algorithms in later chapters.
    pub trait ArraySeqMtEphRedefinableTrait<T>: Sized {
        /// Work Θ(1), Span Θ(1)
        fn empty() -> Self;
        /// Work Θ(1), Span Θ(1)
        fn singleton(item: T) -> Self;
        /// Work Θ(n), Span Θ(n)
        fn tabulate<F: Fn(usize) -> T>(f: &F, length: usize) -> ArraySeqMtEphS<T>;
        /// Work Θ(|a|), Span Θ(log|a|)
        fn map<U: Clone, F: Fn(&T) -> U>(a: &ArraySeqMtEphS<T>, f: &F) -> ArraySeqMtEphS<U>;
        /// Work Θ(|a|+|b|), Span Θ(log(|a|+|b|))
        fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> Self where T: Clone;
        /// Work Θ(|a|), Span Θ(|a|)
        fn filter<F: Fn(&T) -> bool>(a: &ArraySeqMtEphS<T>, pred: &F) -> Self where T: Clone;
        /// Work Θ(n), Span Θ(log n)
        fn update(a: &ArraySeqMtEphS<T>, index: usize, item: T) -> Self where T: Clone;
        /// Work Θ(1), Span Θ(1)
        fn is_empty(&self) -> bool;
        /// Work Θ(1), Span Θ(1)
        fn is_singleton(&self) -> bool;
        /// Work Θ(|a|), Span Θ(|a|)
        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqMtEphS<T>, f: &F, seed: A) -> A;
        /// Work Θ(|a|), Span Θ(log|a|)
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqMtEphS<T>, f: &F, id: T) -> T where T: Clone;
        /// Work Θ(|a|), Span Θ(log|a|)
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqMtEphS<T>, f: &F, id: T) -> (ArraySeqMtEphS<T>, T) where T: Clone;
    }

    impl<T: View> View for ArraySeqMtEphS<T> {
        type V = Seq<T::V>;

        open spec fn view(&self) -> Seq<T::V> {
            self.seq@.map(|_i: int, t: T| t@)
        }
    }

    #[verifier::reject_recursive_types(T)]
    pub struct ArraySeqMtEphIter<T> {
        pub elements: Vec<T>,
        pub pos: usize,
    }

    impl<T> View for ArraySeqMtEphIter<T> {
        type V = (int, Seq<T>);
        open spec fn view(&self) -> (int, Seq<T>) { (self.pos as int, self.elements@) }
    }

    pub open spec fn iter_invariant<T>(it: &ArraySeqMtEphIter<T>) -> bool { it.pos <= it.elements@.len() }

    // See experiments/simple_seq_iter.rs::assumption_free_next for a version that proves
    // without assume() by requiring iter_invariant. We can't add requires to Iterator::next in Verus
    // and Rust iterators have 70 functions on them making this sensible requirement impossible.
    impl<T: Clone> Iterator for ArraySeqMtEphIter<T> {
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

    impl<T: View> ArraySeqMtEphS<T> {
        pub fn new(length: usize, init_value: T) -> (result: ArraySeqMtEphS<T>)
            where T: Clone
            requires length <= usize::MAX
            ensures result.seq@.len() == length
        {
            ArraySeqMtEphS { seq: vec![init_value; length] }
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

        pub fn empty() -> (result: ArraySeqMtEphS<T>)
            ensures result.seq@.len() == 0
        {
            ArraySeqMtEphS { seq: Vec::new() }
        }

        pub fn singleton(item: T) -> (result: ArraySeqMtEphS<T>)
            ensures result.seq@.len() == 1
        {
            let mut seq = Vec::with_capacity(1);
            seq.push(item);
            ArraySeqMtEphS { seq }
        }

        pub fn tabulate<F: Fn(usize) -> T>(f: &F, length: usize) -> (result: ArraySeqMtEphS<T>)
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
            ArraySeqMtEphS { seq }
        }

        pub fn map<U: Clone + View, F: Fn(&T) -> U>(a: &ArraySeqMtEphS<T>, f: &F) -> (result: ArraySeqMtEphS<U>)
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
            ArraySeqMtEphS { seq }
        }

        pub fn append(a: &ArraySeqMtEphS<T>, b: &ArraySeqMtEphS<T>) -> (result: ArraySeqMtEphS<T>)
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
            ArraySeqMtEphS { seq }
        }

        pub fn filter<F: Fn(&T) -> bool>(a: &ArraySeqMtEphS<T>, pred: &F) -> (result: ArraySeqMtEphS<T>)
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
            ArraySeqMtEphS { seq }
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

        pub fn from_vec(elts: Vec<T>) -> (result: ArraySeqMtEphS<T>)
            ensures result.seq@ == elts@
        {
            ArraySeqMtEphS { seq: elts }
        }

        pub fn subseq_copy(&self, start: usize, length: usize) -> (result: ArraySeqMtEphS<T>)
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
            ArraySeqMtEphS { seq }
        }

        pub fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqMtEphS<T>, f: &F, id: T) -> (result: T)
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

        pub fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqMtEphS<T>, f: &F, seed: A) -> (result: A)
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

        pub fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqMtEphS<T>, f: &F, id: T) -> (result: (ArraySeqMtEphS<T>, T))
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
            (ArraySeqMtEphS { seq }, acc)
        }

        pub fn iter(&self) -> (it: ArraySeqMtEphIter<T>)
            where T: Clone
            ensures
                it.elements@.len() == self.seq@.len(),
                forall|i: int| 0 <= i < self.seq@.len() ==> cloned(self.seq@[i], #[trigger] it.elements@[i]),
                it.pos == 0,
                iter_invariant(&it),
        {
            ArraySeqMtEphIter { elements: self.seq.clone(), pos: 0 }
        }

        pub fn map_par<U: Clone + View + Send + Sync + 'static, F: Fn(&T) -> U + Send + Sync + Clone + 'static>(
            pool: &Pool,
            a: &ArraySeqMtEphS<T>,
            f: F,
        ) -> (result: ArraySeqMtEphS<U>)
            where T: Clone + Send + Sync + Eq + 'static
            requires
                obeys_feq_clone::<T>(),
                a.seq@.len() <= usize::MAX as int,
                forall|i: int| 0 <= i < a.seq@.len() ==> #[trigger] f.requires((&a.seq@[i],)),
            ensures result.seq@.len() == a.seq@.len()
            decreases a.seq@.len()
        {
            let len = a.seq.len();
            if len == 0 {
                ArraySeqMtEphS { seq: Vec::new() }
            } else if len == 1 {
                let mut seq = Vec::with_capacity(1);
                seq.push(f(&a.seq[0]));
                ArraySeqMtEphS { seq }
            } else {
                let mid = len / 2;
                let left_seq = a.subseq_copy(0, mid);
                let right_seq = a.subseq_copy(mid, len - mid);
                let pool1 = pool.clone_plus();
                let pool2 = pool.clone_plus();
                let f1 = clone_fn(&f);
                let f2 = f;
                proof {
                    assert forall|i: int| 0 <= i < left_seq.seq@.len() implies #[trigger] f1.requires((&left_seq.seq@[i],)) by {
                        assert(cloned(a.seq@[0 + i], left_seq.seq@[i]));
                        assert(a.seq@[i] == left_seq.seq@[i]);
                    }
                    assert forall|i: int| 0 <= i < right_seq.seq@.len() implies #[trigger] f2.requires((&right_seq.seq@[i],)) by {
                        let orig_i = mid as int + i;
                        assert(cloned(a.seq@[mid as int + i], right_seq.seq@[i]));
                        assert(a.seq@[orig_i] == right_seq.seq@[i]);
                    }
                }
                let left_len = Ghost(left_seq.seq@.len());
                let right_len = Ghost(right_seq.seq@.len());
                let (left, right) = pool.join(
                    (move || -> (r: ArraySeqMtEphS<U>)
                        ensures r.seq@.len() == left_len@
                    { Self::map_par(&pool1, &left_seq, f1) }),
                    (move || -> (r: ArraySeqMtEphS<U>)
                        ensures r.seq@.len() == right_len@
                    { Self::map_par(&pool2, &right_seq, f2) }),
                );
                ArraySeqMtEphS::<U>::append(&left, &right)
            }
        }

        pub fn filter_par<F: Fn(&T) -> bool + Send + Sync + Clone + 'static>(
            pool: &Pool,
            a: &ArraySeqMtEphS<T>,
            pred: F,
        ) -> (result: ArraySeqMtEphS<T>)
            where T: Clone + Send + Sync + Eq + 'static
            requires
                obeys_feq_clone::<T>(),
                a.seq@.len() <= usize::MAX as int,
                forall|i: int| 0 <= i < a.seq@.len() ==> #[trigger] pred.requires((&a.seq@[i],)),
            ensures result.seq@.len() <= a.seq@.len()
            decreases a.seq@.len()
        {
            let len = a.seq.len();
            if len == 0 {
                ArraySeqMtEphS { seq: Vec::new() }
            } else if len == 1 {
                if pred(&a.seq[0]) {
                    let mut seq = Vec::with_capacity(1);
                    seq.push(a.seq[0].clone());
                    ArraySeqMtEphS { seq }
                } else {
                    ArraySeqMtEphS { seq: Vec::new() }
                }
            } else {
                let mid = len / 2;
                let left_seq = a.subseq_copy(0, mid);
                let right_seq = a.subseq_copy(mid, len - mid);
                let pool1 = pool.clone_plus();
                let pool2 = pool.clone_plus();
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
                let left_len = Ghost(left_seq.seq@.len());
                let right_len = Ghost(right_seq.seq@.len());
                let (left, right) = pool.join(
                    (move || -> (r: ArraySeqMtEphS<T>)
                        ensures r.seq@.len() <= left_len@
                    { Self::filter_par(&pool1, &left_seq, p1) }),
                    (move || -> (r: ArraySeqMtEphS<T>)
                        ensures r.seq@.len() <= right_len@
                    { Self::filter_par(&pool2, &right_seq, p2) }),
                );
                Self::append(&left, &right)
            }
        }

        pub fn reduce_par<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(
            pool: &Pool,
            a: &ArraySeqMtEphS<T>,
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
                let pool1 = pool.clone_plus();
                let pool2 = pool.clone_plus();
                let f1 = clone_fn2(&f);
                let f2 = clone_fn2(&f);
                // clone_fn2 preserves requires: forall|x, y| f.requires((x,y)) == f1.requires((x,y))
                let id1 = id.clone();
                let id2 = id.clone();
                let (left, right) = pool.join(
                    move || Self::reduce_par(&pool1, &left_seq, f1, id1),
                    move || Self::reduce_par(&pool2, &right_seq, f2, id2),
                );
                f(&left, &right)
            }
        }
    }

    } // verus!

    // Non-Verus impls
    #[cfg(verus_keep_ghost)]
    impl<T: Clone> Clone for ArraySeqMtEphS<T> {
        fn clone(&self) -> Self { ArraySeqMtEphS { seq: self.seq.clone() } }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: PartialEq> PartialEq for ArraySeqMtEphS<T> {
        fn eq(&self, other: &Self) -> bool { self.seq == other.seq }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: Eq> Eq for ArraySeqMtEphS<T> {}

    #[cfg(verus_keep_ghost)]
    impl<T: Debug> Debug for ArraySeqMtEphS<T> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            f.debug_list().entries(self.seq.iter()).finish()
        }
    }

    #[cfg(verus_keep_ghost)]
    impl<T: Display> Display for ArraySeqMtEphS<T> {
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
    impl<'a, T> IntoIterator for &'a ArraySeqMtEphS<T> {
        type Item = &'a T;
        type IntoIter = Iter<'a, T>;
        fn into_iter(self) -> Self::IntoIter { self.seq.iter() }
    }

    #[cfg(verus_keep_ghost)]
    impl<T> IntoIterator for ArraySeqMtEphS<T> {
        type Item = T;
        type IntoIter = IntoIter<T>;
        fn into_iter(self) -> Self::IntoIter { self.seq.into_iter() }
    }

    // Non-Verus stub
    #[cfg(not(verus_keep_ghost))]
    use crate::Chap02::WSSchedulerMtEph::WSSchedulerMtEph::Pool;

    #[cfg(not(verus_keep_ghost))]
    #[derive(Clone, PartialEq, Eq, Debug)]
    pub struct ArraySeqMtEphS<T> {
        pub seq: Vec<T>,
    }

    #[cfg(not(verus_keep_ghost))]
    pub struct ArraySeqMtEphIter<T> {
        pub elements: Vec<T>,
        pub pos: usize,
    }

    #[cfg(not(verus_keep_ghost))]
    impl<T: Clone> Iterator for ArraySeqMtEphIter<T> {
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
    impl<T> ArraySeqMtEphS<T> {
        pub fn new(length: usize, init_value: T) -> Self where T: Clone {
            ArraySeqMtEphS { seq: vec![init_value; length] }
        }
        pub fn set(&mut self, index: usize, item: T) -> Result<(), &'static str> {
            if index < self.seq.len() { self.seq[index] = item; Ok(()) }
            else { Err("Index out of bounds") }
        }
        pub fn length(&self) -> usize { self.seq.len() }
        pub fn nth(&self, index: usize) -> &T { &self.seq[index] }
        pub fn empty() -> Self { ArraySeqMtEphS { seq: Vec::new() } }
        pub fn singleton(item: T) -> Self { ArraySeqMtEphS { seq: vec![item] } }
        pub fn tabulate<F: Fn(usize) -> T>(f: &F, length: usize) -> Self {
            ArraySeqMtEphS { seq: (0..length).map(f).collect() }
        }
        pub fn map<U, F: Fn(&T) -> U>(a: &Self, f: &F) -> ArraySeqMtEphS<U> {
            ArraySeqMtEphS { seq: a.seq.iter().map(f).collect() }
        }
        pub fn append(a: &Self, b: &Self) -> Self where T: Clone {
            let mut seq = a.seq.clone();
            seq.extend(b.seq.iter().cloned());
            ArraySeqMtEphS { seq }
        }
        pub fn filter<F: Fn(&T) -> bool>(a: &Self, pred: &F) -> Self where T: Clone {
            ArraySeqMtEphS { seq: a.seq.iter().filter(|x| pred(x)).cloned().collect() }
        }
        pub fn isEmpty(&self) -> bool { self.seq.is_empty() }
        pub fn isSingleton(&self) -> bool { self.seq.len() == 1 }
        pub fn from_vec(elts: Vec<T>) -> Self { ArraySeqMtEphS { seq: elts } }
        pub fn subseq_copy(&self, start: usize, length: usize) -> Self where T: Clone {
            let end = (start + length).min(self.seq.len());
            ArraySeqMtEphS { seq: self.seq[start..end].to_vec() }
        }
        pub fn reduce<F: Fn(&T, &T) -> T>(a: &Self, f: &F, id: T) -> T where T: Clone {
            a.seq.iter().fold(id, |acc, x| f(&acc, x))
        }
        pub fn iterate<A, F: Fn(&A, &T) -> A>(a: &Self, f: &F, seed: A) -> A {
            a.seq.iter().fold(seed, |acc, x| f(&acc, x))
        }
        pub fn scan<F: Fn(&T, &T) -> T>(a: &Self, f: &F, id: T) -> (Self, T) where T: Clone {
            let mut acc = id;
            let seq: Vec<T> = a.seq.iter().map(|x| { acc = f(&acc, x); acc.clone() }).collect();
            (ArraySeqMtEphS { seq }, acc)
        }
        pub fn iter(&self) -> ArraySeqMtEphIter<T> where T: Clone {
            ArraySeqMtEphIter { elements: self.seq.clone(), pos: 0 }
        }
        pub fn iter_std(&self) -> Iter<'_, T> { self.seq.iter() }
        pub fn map_par<U: Clone + Send + Sync + 'static, F: Fn(&T) -> U + Send + Sync + Clone + 'static>(
            pool: &Pool, a: &Self, f: F,
        ) -> ArraySeqMtEphS<U> where T: Clone + Send + Sync + 'static {
            let len = a.seq.len();
            if len == 0 { ArraySeqMtEphS { seq: Vec::new() } }
            else if len == 1 { ArraySeqMtEphS { seq: vec![f(&a.seq[0])] } }
            else {
                let mid = len / 2;
                let left_seq = a.subseq_copy(0, mid);
                let right_seq = a.subseq_copy(mid, len - mid);
                let (f1, f2) = (f.clone(), f.clone());
                let (pool1, pool2) = (pool.clone(), pool.clone());
                let (left, right) = pool.join(
                    move || Self::map_par(&pool1, &left_seq, f1),
                    move || Self::map_par(&pool2, &right_seq, f2),
                );
                ArraySeqMtEphS::<U>::append(&left, &right)
            }
        }
        pub fn filter_par<F: Fn(&T) -> bool + Send + Sync + Clone + 'static>(
            pool: &Pool, a: &Self, pred: F,
        ) -> Self where T: Clone + Send + Sync + 'static {
            let len = a.seq.len();
            if len == 0 { ArraySeqMtEphS { seq: Vec::new() } }
            else if len == 1 {
                if pred(&a.seq[0]) { ArraySeqMtEphS { seq: vec![a.seq[0].clone()] } }
                else { ArraySeqMtEphS { seq: Vec::new() } }
            } else {
                let mid = len / 2;
                let left_seq = a.subseq_copy(0, mid);
                let right_seq = a.subseq_copy(mid, len - mid);
                let (p1, p2) = (pred.clone(), pred.clone());
                let (pool1, pool2) = (pool.clone(), pool.clone());
                let (left, right) = pool.join(
                    move || Self::filter_par(&pool1, &left_seq, p1),
                    move || Self::filter_par(&pool2, &right_seq, p2),
                );
                Self::append(&left, &right)
            }
        }
        pub fn reduce_par<F: Fn(&T, &T) -> T + Send + Sync + Clone + 'static>(
            pool: &Pool, a: &Self, f: F, id: T,
        ) -> T where T: Clone + Send + Sync + 'static {
            let len = a.seq.len();
            if len == 0 { id }
            else if len == 1 { a.seq[0].clone() }
            else {
                let mid = len / 2;
                let left_seq = a.subseq_copy(0, mid);
                let right_seq = a.subseq_copy(mid, len - mid);
                let (f1, f2) = (f.clone(), f.clone());
                let (id1, id2) = (id.clone(), id.clone());
                let (pool1, pool2) = (pool.clone(), pool.clone());
                let (left, right) = pool.join(
                    move || Self::reduce_par(&pool1, &left_seq, f1, id1),
                    move || Self::reduce_par(&pool2, &right_seq, f2, id2),
                );
                f(&left, &right)
            }
        }
    }

    #[cfg(not(verus_keep_ghost))]
    impl<T: Display> Display for ArraySeqMtEphS<T> {
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
    impl<'a, T> IntoIterator for &'a ArraySeqMtEphS<T> {
        type Item = &'a T;
        type IntoIter = Iter<'a, T>;
        fn into_iter(self) -> Self::IntoIter { self.seq.iter() }
    }

    #[cfg(not(verus_keep_ghost))]
    impl<T> IntoIterator for ArraySeqMtEphS<T> {
        type Item = T;
        type IntoIter = IntoIter<T>;
        fn into_iter(self) -> Self::IntoIter { self.seq.into_iter() }
    }
}
