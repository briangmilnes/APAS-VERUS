//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 19 algorithms for ArraySeqStPer. Verusified.
//! Redefines Chap18 methods using tabulate as the core primitive.

pub mod ArraySeqStPer {

    #[cfg(verus_keep_ghost)]
    use vstd::prelude::*;

    #[cfg(verus_keep_ghost)]
    pub use crate::Chap18::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS;

    #[cfg(verus_keep_ghost)]
    verus! {

    broadcast use vstd::std_specs::vec::group_vec_axioms;
    use vstd::std_specs::clone::*;

    // Chapter 19 implementations - alternative algorithms using tabulate as primitive
    impl<T: View + Clone> ArraySeqStPerS<T> {

        // Algorithm 19.1: empty = tabulate(lambda i._, 0)
        pub fn empty_c19() -> (result: ArraySeqStPerS<T>)
            ensures result.seq@.len() == 0
        {
            Self::empty()
        }

        // Algorithm 19.2: singleton x = tabulate(lambda i.x, 1)
        pub fn singleton_c19(item: T) -> (result: ArraySeqStPerS<T>)
            ensures result.seq@.len() == 1
        {
            Self::tabulate(&|_i: usize| item.clone(), 1)
        }

        // Algorithm 19.3: map f a = tabulate(lambda i.f(a[i]), |a|)
        #[verifier::external_body]
        pub fn map_c19<U: Clone + View, F: Fn(&T) -> U>(a: &ArraySeqStPerS<T>, f: &F) -> (result: ArraySeqStPerS<U>)
            requires
                a.seq@.len() <= usize::MAX as int,
                forall|i: int| 0 <= i < a.seq@.len() ==> #[trigger] f.requires((&a.seq@[i],)),
            ensures result.seq@.len() == a.seq@.len()
        {
            ArraySeqStPerS::<U>::tabulate(&|i: usize| f(a.nth(i)), a.length())
        }

        // Algorithm 19.4: append a b = flatten([a, b])
        #[verifier::external_body]
        pub fn append_c19(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> (result: ArraySeqStPerS<T>)
            requires a.seq@.len() + b.seq@.len() <= usize::MAX as int
            ensures result.seq@.len() == a.seq@.len() + b.seq@.len()
        {
            Self::flatten_c19(&ArraySeqStPerS::<ArraySeqStPerS<T>>::tabulate(
                &|i: usize| if i == 0 { a.clone() } else { b.clone() },
                2,
            ))
        }

        // Helper: flatten
        #[verifier::external_body]
        fn flatten_c19(ss: &ArraySeqStPerS<ArraySeqStPerS<T>>) -> (result: ArraySeqStPerS<T>) {
            let mut total_len: usize = 0;
            let ss_len = ss.seq.len();
            for i in 0..ss_len {
                total_len = total_len + ss.seq[i].seq.len();
            }
            let mut result: Vec<T> = Vec::with_capacity(total_len);
            for j in 0..ss_len {
                let inner = &ss.seq[j];
                for k in 0..inner.seq.len() {
                    result.push(inner.seq[k].clone());
                }
            }
            ArraySeqStPerS { seq: result }
        }

        // Algorithm 19.5: filter f a = flatten(map(deflate f, a))
        #[verifier::external_body]
        pub fn filter_c19<F: Fn(&T) -> bool>(a: &ArraySeqStPerS<T>, pred: &F) -> (result: ArraySeqStPerS<T>)
            requires
                a.seq@.len() <= usize::MAX as int,
                forall|i: int| 0 <= i < a.seq@.len() ==> #[trigger] pred.requires((&a.seq@[i],)),
            ensures result.seq@.len() <= a.seq@.len()
        {
            let deflated = ArraySeqStPerS::<ArraySeqStPerS<T>>::tabulate(
                &|i: usize| Self::deflate_c19(pred, a.nth(i)),
                a.length(),
            );
            Self::flatten_c19(&deflated)
        }

        // Helper: deflate for filter
        fn deflate_c19<F: Fn(&T) -> bool>(pred: &F, x: &T) -> (result: ArraySeqStPerS<T>)
            requires pred.requires((x,))
            ensures result.seq@.len() <= 1
        {
            if pred(x) {
                Self::singleton(x.clone())
            } else {
                Self::empty()
            }
        }

        // Algorithm 19.6: update a (i, x) = tabulate(lambda j. if i = j then x else a[j], |a|)
        #[verifier::external_body]
        pub fn update_c19(a: &ArraySeqStPerS<T>, index: usize, item: T) -> (result: ArraySeqStPerS<T>)
            requires
                index < a.seq@.len(),
                a.seq@.len() <= usize::MAX as int,
            ensures result.seq@.len() == a.seq@.len()
        {
            Self::tabulate(
                &|j: usize| if j == index { item.clone() } else { a.nth(j).clone() },
                a.length(),
            )
        }

        // Algorithm 19.7: isEmpty a = |a| = 0
        pub fn isEmpty_c19(a: &ArraySeqStPerS<T>) -> (empty: bool)
            ensures empty <==> a.seq@.len() == 0
        {
            a.length() == 0
        }

        // Algorithm 19.7: isSingleton a = |a| = 1
        pub fn isSingleton_c19(a: &ArraySeqStPerS<T>) -> (single: bool)
            ensures single <==> a.seq@.len() == 1
        {
            a.length() == 1
        }

        // Algorithm 19.8: iterate f x a = left-to-right traversal
        pub fn iterate_c19<A, F: Fn(&A, &T) -> A>(a: &ArraySeqStPerS<T>, f: &F, seed: A) -> (result: A)
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
                acc = f(&acc, a.nth(i));
                i += 1;
            }
            acc
        }

        // Algorithm 19.9: reduce using divide-and-conquer
        pub fn reduce_c19<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, id: T) -> (result: T)
            requires
                a.seq@.len() <= usize::MAX as int,
                forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
            decreases a.seq@.len()
        {
            let len = a.length();
            if len == 0 {
                id
            } else if len == 1 {
                a.nth(0).clone()
            } else {
                let mid = len / 2;
                let left = a.subseq_copy(0, mid);
                let right = a.subseq_copy(mid, len - mid);
                let left_result = Self::reduce_c19(&left, f, id.clone());
                let right_result = Self::reduce_c19(&right, f, id);
                f(&left_result, &right_result)
            }
        }

        // Algorithm 19.10: scan using contraction (simplified sequential version)
        pub fn scan_c19<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, id: T) -> (result: (ArraySeqStPerS<T>, T))
            requires forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
            ensures result.0.seq@.len() == a.seq@.len()
        {
            let len = a.seq.len();
            let mut acc = id.clone();
            let mut results: Vec<T> = Vec::with_capacity(len);
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len == a.seq@.len(),
                    results@.len() == i as int,
                    forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                decreases len - i,
            {
                acc = f(&acc, a.nth(i));
                results.push(acc.clone());
                i += 1;
            }
            (ArraySeqStPerS { seq: results }, acc)
        }

        // select helper for append_select
        pub fn select_c19<'a>(a: &'a ArraySeqStPerS<T>, b: &'a ArraySeqStPerS<T>, i: usize) -> (result: Option<&'a T>)
            ensures
                i < a.seq@.len() ==> result.is_some(),
                i >= a.seq@.len() && i < a.seq@.len() + b.seq@.len() ==> result.is_some(),
                i >= a.seq@.len() + b.seq@.len() ==> result.is_none(),
        {
            let len_a = a.length();
            if i < len_a {
                Some(a.nth(i))
            } else {
                let offset = i - len_a;
                let len_b = b.length();
                if offset < len_b {
                    Some(b.nth(offset))
                } else {
                    None
                }
            }
        }

        // Algorithm 19.4 alternative: append a b = tabulate(select(a,b), |a|+|b|)
        #[verifier::external_body]
        pub fn append_select_c19(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> (result: ArraySeqStPerS<T>)
            requires a.seq@.len() + b.seq@.len() <= usize::MAX as int
            ensures result.seq@.len() == a.seq@.len() + b.seq@.len()
        {
            let total = a.length() + b.length();
            Self::tabulate(&|i: usize| Self::select_c19(a, b, i).unwrap().clone(), total)
        }
    }

    } // verus!

    // Non-Verus stub
    #[cfg(not(verus_keep_ghost))]
    pub use crate::Chap18::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS;

    #[cfg(not(verus_keep_ghost))]
    impl<T: Clone> ArraySeqStPerS<T> {
        pub fn empty_c19() -> Self { Self::from_vec(Vec::new()) }
        pub fn singleton_c19(item: T) -> Self { Self::from_vec(vec![item]) }
        pub fn map_c19<U: Clone, F: Fn(&T) -> U>(a: &Self, f: &F) -> ArraySeqStPerS<U> {
            ArraySeqStPerS::<U>::tabulate(&|i| f(a.nth(i)), a.length())
        }
        pub fn append_c19(a: &Self, b: &Self) -> Self {
            let mut seq = a.seq.clone();
            seq.extend(b.seq.iter().cloned());
            Self { seq }
        }
        fn flatten_c19(ss: &ArraySeqStPerS<ArraySeqStPerS<T>>) -> Self {
            let mut result = Vec::new();
            for inner in ss.seq.iter() {
                result.extend(inner.seq.iter().cloned());
            }
            Self { seq: result }
        }
        pub fn filter_c19<F: Fn(&T) -> bool>(a: &Self, pred: &F) -> Self {
            Self { seq: a.seq.iter().filter(|x| pred(x)).cloned().collect() }
        }
        fn deflate_c19<F: Fn(&T) -> bool>(pred: &F, x: &T) -> Self {
            if pred(x) { Self::singleton(x.clone()) } else { Self::empty() }
        }
        pub fn update_c19(a: &Self, index: usize, item: T) -> Self {
            Self::tabulate(&|j| if j == index { item.clone() } else { a.nth(j).clone() }, a.length())
        }
        pub fn isEmpty_c19(a: &Self) -> bool { a.length() == 0 }
        pub fn isSingleton_c19(a: &Self) -> bool { a.length() == 1 }
        pub fn iterate_c19<A, F: Fn(&A, &T) -> A>(a: &Self, f: &F, seed: A) -> A {
            a.seq.iter().fold(seed, |acc, x| f(&acc, x))
        }
        pub fn reduce_c19<F: Fn(&T, &T) -> T>(a: &Self, f: &F, id: T) -> T {
            if a.length() == 0 { id }
            else if a.length() == 1 { a.nth(0).clone() }
            else {
                let mid = a.length() / 2;
                let left = a.subseq_copy(0, mid);
                let right = a.subseq_copy(mid, a.length() - mid);
                f(&Self::reduce_c19(&left, f, id.clone()), &Self::reduce_c19(&right, f, id))
            }
        }
        pub fn scan_c19<F: Fn(&T, &T) -> T>(a: &Self, f: &F, id: T) -> (Self, T) {
            let mut acc = id;
            let seq: Vec<T> = a.seq.iter().map(|x| { acc = f(&acc, x); acc.clone() }).collect();
            (Self { seq }, acc)
        }
        pub fn select_c19<'a>(a: &'a Self, b: &'a Self, i: usize) -> Option<&'a T> {
            if i < a.length() { Some(a.nth(i)) }
            else if i - a.length() < b.length() { Some(b.nth(i - a.length())) }
            else { None }
        }
        pub fn append_select_c19(a: &Self, b: &Self) -> Self {
            Self::tabulate(&|i| Self::select_c19(a, b, i).unwrap().clone(), a.length() + b.length())
        }
    }
}
