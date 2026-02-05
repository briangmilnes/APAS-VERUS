//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 18 algorithms for ArraySeqMtPer multithreaded persistent. Verusified.
//! Uses global work-stealing pool for parallel operations (map_par, reduce_par).

pub mod ArraySeqMtPer {

    use std::fmt::{Debug, Display, Formatter};
    use std::fmt::Result as FmtResult;
    use std::slice::Iter;
    use std::vec::IntoIter;

    use vstd::prelude::*;
    use crate::Chap02::WSSchedulerMtEph::WSSchedulerMtEph::join;
    use crate::vstdplus::clone_plus::clone_plus::{clone_fn, clone_fn2, clone_pred};

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
        spec fn spec_len(&self) -> int;

        /// Work Θ(n), Span Θ(log n)
        fn new(length: usize, init_value: T) -> (result: Self)
            where T: Clone
            requires length <= usize::MAX
            ensures result.spec_len() == length as int;

        /// Work Θ(1), Span Θ(1)
        fn length(&self) -> (len: usize)
            ensures len as int == self.spec_len();

        /// Work Θ(1), Span Θ(1)
        fn nth(&self, index: usize) -> (result: &T)
            requires index < self.spec_len();

        /// Work Θ(len), Span Θ(log len)
        fn subseq_copy(&self, start: usize, length: usize) -> (result: Self)
            where T: Clone
            requires start + length <= self.spec_len()
            ensures result.spec_len() == length as int;

        /// Work Θ(n), Span Θ(1)
        fn from_vec(elts: Vec<T>) -> (result: Self)
            ensures result.spec_len() == elts@.len();
    }

    /// Redefinable trait - may be overridden with better algorithms in later chapters.
    pub trait ArraySeqMtPerRedefinableTrait<T>: Sized {
        spec fn spec_len(&self) -> int;

        /// Work Θ(1), Span Θ(1)
        fn empty() -> (result: Self)
            ensures result.spec_len() == 0;

        /// Work Θ(1), Span Θ(1)
        fn singleton(item: T) -> (result: Self)
            ensures result.spec_len() == 1;

        /// Work Θ(n), Span Θ(n)
        fn tabulate<F: Fn(usize) -> T>(f: &F, length: usize) -> (result: ArraySeqMtPerS<T>)
            requires
                length <= usize::MAX,
                forall|i: usize| i < length ==> #[trigger] f.requires((i,)),
            ensures
                result.seq@.len() == length,
                forall|i: int| #![auto] 0 <= i < length ==> f.ensures((i as usize,), result.seq@[i]);

        /// Work Θ(|a|), Span Θ(log|a|)
        fn map<U: Clone, F: Fn(&T) -> U>(a: &ArraySeqMtPerS<T>, f: &F) -> (result: ArraySeqMtPerS<U>)
            requires
                forall|i: int| 0 <= i < a.seq@.len() ==> #[trigger] f.requires((&a.seq@[i],)),
            ensures
                result.seq@.len() == a.seq@.len(),
                forall|i: int| #![auto] 0 <= i < a.seq@.len() ==> f.ensures((&a.seq@[i],), result.seq@[i]);

        /// Work Θ(|a|+|b|), Span Θ(log(|a|+|b|))
        fn append(a: &ArraySeqMtPerS<T>, b: &ArraySeqMtPerS<T>) -> (result: Self)
            where T: Clone
            requires a.seq@.len() + b.seq@.len() <= usize::MAX as int
            ensures result.spec_len() == a.seq@.len() + b.seq@.len();

        /// Work Θ(|a|), Span Θ(|a|)
        fn filter<F: Fn(&T) -> bool>(a: &ArraySeqMtPerS<T>, pred: &F) -> (result: Self)
            where T: Clone
            requires forall|i: int| 0 <= i < a.seq@.len() ==> #[trigger] pred.requires((&a.seq@[i],))
            ensures result.spec_len() <= a.seq@.len();

        /// Work Θ(Σ|a[i]|), Span Θ(Σ|a[i]|)
        fn flatten(a: &ArraySeqMtPerS<ArraySeqMtPerS<T>>) -> (result: Self) where T: Clone;

        /// Work Θ(n), Span Θ(log n)
        fn update(a: &ArraySeqMtPerS<T>, index: usize, item: T) -> (result: Self)
            where T: Clone
            requires index < a.seq@.len()
            ensures result.spec_len() == a.seq@.len();

        /// Work Θ(1), Span Θ(1)
        fn is_empty(&self) -> (empty: bool)
            ensures empty <==> self.spec_len() == 0;

        /// Work Θ(1), Span Θ(1)
        fn is_singleton(&self) -> (single: bool)
            ensures single <==> self.spec_len() == 1;

        /// Work Θ(|a|), Span Θ(|a|)
        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqMtPerS<T>, f: &F, seed: A) -> A
            requires forall|x: &A, y: &T| #[trigger] f.requires((x, y));

        /// Work Θ(|a|), Span Θ(log|a|)
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqMtPerS<T>, f: &F, id: T) -> T
            where T: Clone
            requires forall|x: &T, y: &T| #[trigger] f.requires((x, y));

        /// Work Θ(|a|), Span Θ(log|a|)
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqMtPerS<T>, f: &F, id: T) -> (result: (ArraySeqMtPerS<T>, T))
            where T: Clone
            requires forall|x: &T, y: &T| #[trigger] f.requires((x, y))
            ensures result.0.seq@.len() == a.seq@.len();
    }

    impl<T: View> View for ArraySeqMtPerS<T> {
        type V = Seq<T::V>;

        open spec fn view(&self) -> Seq<T::V> {
            self.seq@.map(|_i: int, t: T| t@)
        }
    }

    /// Iterator wrapper with closed spec view for encapsulation.
    #[verifier::reject_recursive_types(T)]
    pub struct ArraySeqMtPerIter<'a, T> {
        inner: std::slice::Iter<'a, T>,
    }

    impl<'a, T> View for ArraySeqMtPerIter<'a, T> {
        type V = (int, Seq<T>);
        closed spec fn view(&self) -> (int, Seq<T>) { self.inner@ }
    }

    pub open spec fn iter_invariant<'a, T>(it: &ArraySeqMtPerIter<'a, T>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<'a, T> std::iter::Iterator for ArraySeqMtPerIter<'a, T> {
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

    /// Ghost iterator for ForLoopGhostIterator support.
    #[verifier::reject_recursive_types(T)]
    pub struct ArraySeqMtPerGhostIterator<'a, T> {
        pub pos: int,
        pub elements: Seq<T>,
        pub phantom: core::marker::PhantomData<&'a T>,
    }

    impl<'a, T> vstd::pervasive::ForLoopGhostIteratorNew for ArraySeqMtPerIter<'a, T> {
        type GhostIter = ArraySeqMtPerGhostIterator<'a, T>;
        open spec fn ghost_iter(&self) -> ArraySeqMtPerGhostIterator<'a, T> {
            ArraySeqMtPerGhostIterator { pos: self@.0, elements: self@.1, phantom: core::marker::PhantomData }
        }
    }

    impl<'a, T> vstd::pervasive::ForLoopGhostIterator for ArraySeqMtPerGhostIterator<'a, T> {
        type ExecIter = ArraySeqMtPerIter<'a, T>;
        type Item = T;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &ArraySeqMtPerIter<'a, T>) -> bool {
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

        open spec fn ghost_advance(&self, _exec_iter: &ArraySeqMtPerIter<'a, T>) -> ArraySeqMtPerGhostIterator<'a, T> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, T> View for ArraySeqMtPerGhostIterator<'a, T> {
        type V = Seq<T>;
        open spec fn view(&self) -> Seq<T> { self.elements.take(self.pos) }
    }

    impl<T> ArraySeqMtPerS<T> {
        pub open spec fn spec_len(&self) -> int {
            self.seq@.len() as int
        }

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
            ensures
                result.seq@.len() == length,
                forall|i: int| #![auto] 0 <= i < length ==> f.ensures((i as usize,), result.seq@[i]),
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
            ArraySeqMtPerS { seq }
        }

        pub fn map<U: Clone + View, F: Fn(&T) -> U>(a: &ArraySeqMtPerS<T>, f: &F) -> (result: ArraySeqMtPerS<U>)
            requires forall|i: int| 0 <= i < a.seq@.len() ==> #[trigger] f.requires((&a.seq@[i],)),
            ensures
                result.seq@.len() == a.seq@.len(),
                forall|i: int| #![auto] 0 <= i < a.seq@.len() ==> f.ensures((&a.seq@[i],), result.seq@[i]),
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
            ArraySeqMtPerS { seq }
        }

        pub fn map_par<U: Clone + View + Send + Sync + 'static, F: Fn(&T) -> U + Send + Sync + Clone + 'static>(
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

                let fa = move || -> (r: ArraySeqMtPerS<U>)
                    requires
                        forall|i: int| 0 <= i < left_seq.seq@.len() ==> #[trigger] f1.requires((&left_seq.seq@[i],)),
                    ensures r.seq@.len() == left_seq.seq@.len(),
                {
                    Self::map_par(&left_seq, f1)
                };

                let fb = move || -> (r: ArraySeqMtPerS<U>)
                    requires
                        forall|i: int| 0 <= i < right_seq.seq@.len() ==> #[trigger] f2.requires((&right_seq.seq@[i],)),
                    ensures r.seq@.len() == right_seq.seq@.len(),
                {
                    Self::map_par(&right_seq, f2)
                };

                let (left, right) = join(fa, fb);
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

                let fa = move || -> (r: ArraySeqMtPerS<T>)
                    requires forall|i: int| 0 <= i < left_seq.seq@.len() ==> #[trigger] p1.requires((&left_seq.seq@[i],)),
                    ensures r.seq@.len() <= left_seq.seq@.len(),
                {
                    Self::filter_par(&left_seq, p1)
                };

                let fb = move || -> (r: ArraySeqMtPerS<T>)
                    requires forall|i: int| 0 <= i < right_seq.seq@.len() ==> #[trigger] p2.requires((&right_seq.seq@[i],)),
                    ensures r.seq@.len() <= right_seq.seq@.len(),
                {
                    Self::filter_par(&right_seq, p2)
                };

                let (left, right) = join(fa, fb);
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
                let id1 = id.clone();
                let id2 = id.clone();

                let fa = move || -> (r: T)
                    requires
                        left_seq.seq@.len() > 0,
                        forall|x: &T, y: &T| #[trigger] f1.requires((x, y)),
                {
                    Self::reduce_par(&left_seq, f1, id1)
                };

                let fb = move || -> (r: T)
                    requires
                        right_seq.seq@.len() > 0,
                        forall|x: &T, y: &T| #[trigger] f2.requires((x, y)),
                {
                    Self::reduce_par(&right_seq, f2, id2)
                };

                let (left, right) = join(fa, fb);
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

        /// Returns an iterator over the sequence elements.
        pub fn iter(&self) -> (it: ArraySeqMtPerIter<'_, T>)
            ensures
                it@.0 == 0,
                it@.1 == self.seq@,
                iter_invariant(&it),
        {
            ArraySeqMtPerIter { inner: self.seq.iter() }
        }
    }

    impl<'a, T> std::iter::IntoIterator for &'a ArraySeqMtPerS<T> {
        type Item = &'a T;
        type IntoIter = ArraySeqMtPerIter<'a, T>;
        fn into_iter(self) -> Self::IntoIter { ArraySeqMtPerIter { inner: self.seq.iter() } }
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
