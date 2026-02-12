//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Chapter 19 algorithms for ArraySeqStPer. Verusified.
//! Redefines Chap18 methods using tabulate as the core primitive.
//! Use the trait `ArraySeqStPerTrait` to access these implementations.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	8. traits
//	9. impls

//		1. module


pub mod ArraySeqStPer {

    use std::hash::Hash;

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    verus! {

    //		2. imports

    use crate::Types::Types::*;
    use crate::Chap05::SetStEph::SetStEph::{SetStEph, SetStEphTrait, valid_key_type};
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    use crate::vstdplus::feq::feq::*;
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;


    //		3. broadcast use

    broadcast use vstd::std_specs::vec::group_vec_axioms;


    //		8. traits

    // Chapter 19 trait - provides alternative algorithmic implementations
    // Import and use this trait to get Chapter 19's algorithms
    pub trait ArraySeqStPerTrait<T: View + Clone + Eq>: ArraySeqStPerBaseTrait<T> {
        fn empty() -> (result: Self)
            ensures result.spec_len() == 0;

        fn singleton(item: T) -> (result: Self)
            ensures result.spec_len() == 1;

        fn map<U: Clone + View, F: Fn(&T) -> U>(a: &Self, f: &F) -> ArraySeqStPerS<U>
            requires
                a.spec_len() <= usize::MAX as int,
                forall|i: int| 0 <= i < a.spec_len() ==> #[trigger] f.requires((&a.spec_index(i),));

        fn append(a: &Self, b: &Self) -> (result: Self)
            requires a.spec_len() + b.spec_len() <= usize::MAX as int
            ensures result.spec_len() == a.spec_len() + b.spec_len();

        fn filter<F: Fn(&T) -> bool>(a: &Self, pred: &F) -> (result: Self)
            requires
                a.spec_len() <= usize::MAX as int,
                forall|i: int| 0 <= i < a.spec_len() ==> #[trigger] pred.requires((&a.spec_index(i),))
            ensures result.spec_len() <= a.spec_len();

        fn update(a: &Self, index: usize, item: T) -> (result: Self)
            requires
                index < a.spec_len(),
                a.spec_len() <= usize::MAX as int,
            ensures result.spec_len() == a.spec_len();

        fn is_empty(a: &Self) -> (empty: bool)
            ensures empty <==> a.spec_len() == 0;

        fn is_singleton(a: &Self) -> (single: bool)
            ensures single <==> a.spec_len() == 1;

        fn iterate<A, F: Fn(&A, &T) -> A>(a: &Self, f: &F, seed: A) -> A
            requires forall|x: &A, y: &T| #[trigger] f.requires((x, y));

        fn reduce<F: Fn(&T, &T) -> T>(a: &Self, f: &F, id: T) -> T
            requires
                a.spec_len() <= usize::MAX as int,
                forall|x: &T, y: &T| #[trigger] f.requires((x, y));

        fn scan<F: Fn(&T, &T) -> T>(a: &Self, f: &F, id: T) -> (result: (Self, T))
            requires forall|x: &T, y: &T| #[trigger] f.requires((x, y))
            ensures result.0.spec_len() == a.spec_len();

        // Recursive versions matching textbook algorithms
        fn iterate_rec<A, F: Fn(&A, &T) -> A>(a: &Self, f: &F, seed: A) -> A
            requires
                obeys_feq_clone::<T>(),
                a.spec_len() <= usize::MAX as int,
                forall|x: &A, y: &T| #[trigger] f.requires((x, y));

        fn reduce_rec<F: Fn(&T, &T) -> T>(a: &Self, f: &F, id: T) -> T
            requires
                obeys_feq_clone::<T>(),
                a.spec_len() <= usize::MAX as int,
                forall|x: &T, y: &T| #[trigger] f.requires((x, y));

        fn scan_rec<F: Fn(&T, &T) -> T>(a: &Self, f: &F, id: T) -> (result: (Self, T))
            requires
                obeys_feq_clone::<T>(),
                a.spec_len() <= usize::MAX as int,
                forall|x: &T, y: &T| #[trigger] f.requires((x, y))
            ensures result.0.spec_len() == a.spec_len();

        fn select<'a>(a: &'a Self, b: &'a Self, i: usize) -> (result: Option<&'a T>)
            ensures
                i < a.spec_len() ==> result.is_some(),
                a.spec_len() <= i < a.spec_len() + b.spec_len() ==> result.is_some();

        fn append_select(a: &Self, b: &Self) -> (result: Self)
            requires a.spec_len() + b.spec_len() <= usize::MAX as int
            ensures result.spec_len() == a.spec_len() + b.spec_len();

        fn from_set(set: &SetStEph<T>) -> (result: Self)
            where T: StT + Hash
            requires valid_key_type::<T>()
            ensures result.spec_len() == set@.len();
    }


    //		9. impls

    impl<T: View + Clone + Eq> ArraySeqStPerTrait<T> for ArraySeqStPerS<T> {

        // Algorithm 19.1: empty = tabulate(lambda i._, 0)
        fn empty() -> ArraySeqStPerS<T> {
            <ArraySeqStPerS<T> as ArraySeqStPerRedefinableTrait<T>>::empty()
        }

        // Algorithm 19.2: singleton x = tabulate(lambda i.x, 1)
        fn singleton(item: T) -> ArraySeqStPerS<T> {
            ArraySeqStPerS::tabulate(&|_i: usize| item.clone(), 1)
        }

        // Algorithm 19.3: map f a = tabulate(lambda i.f(a[i]), |a|)
        fn map<U: Clone + View, F: Fn(&T) -> U>(a: &ArraySeqStPerS<T>, f: &F) -> ArraySeqStPerS<U> {
            let n = a.length();
            proof {
                // Connect outer requires to closure requires
                assert forall|i: usize| i < n implies (i as int) < a.seq@.len() && #[trigger] f.requires((&a.seq@[i as int],)) by {
                    assert(a.spec_index(i as int) == a.seq@[i as int]);
                }
            }
            ArraySeqStPerS::tabulate(
                &(|i: usize| -> (r: U)
                    requires
                        (i as int) < a.seq@.len(),
                        f.requires((&a.seq@[i as int],)),
                {
                    let elem = a.nth(i);
                    f(elem)
                }),
                n,
            )
        }

        // Algorithm 19.4: append a b = flatten([a, b])
        fn append(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T> {
            // Clone inner Vecs (vstd has clone spec for Vec)
            let a_clone = ArraySeqStPerS { seq: a.seq.clone() };
            let b_clone = ArraySeqStPerS { seq: b.seq.clone() };
            let mut pair_vec: Vec<ArraySeqStPerS<T>> = Vec::with_capacity(2);
            pair_vec.push(a_clone);
            pair_vec.push(b_clone);
            let pair = ArraySeqStPerS::<ArraySeqStPerS<T>> { seq: pair_vec };
            proof {
                assert(pair.seq@.len() == 2);
                assert(pair.seq@[0].seq@.len() == a.seq@.len());
                assert(pair.seq@[1].seq@.len() == b.seq@.len());
            }
            flatten(&pair)
        }

        // Algorithm 19.5: filter f a = flatten(map(deflate f, a))
        fn filter<F: Fn(&T) -> bool>(a: &ArraySeqStPerS<T>, pred: &F) -> ArraySeqStPerS<T> {
            // Build deflated array manually to avoid opaque f.ensures
            let n = a.length();
            let mut deflated_vec: Vec<ArraySeqStPerS<T>> = Vec::with_capacity(n);
            proof {
                // Connect outer requires (spec_index) to seq@
                assert forall|j: usize| j < n implies #[trigger] pred.requires((&a.seq@[j as int],)) by {
                    assert(a.spec_index(j as int) == a.seq@[j as int]);
                }
            }
            let mut i: usize = 0;
            while i < n
                invariant
                    i <= n,
                    n == a.seq@.len(),
                    deflated_vec@.len() == i as int,
                    forall|j: int| 0 <= j < i as int ==> #[trigger] deflated_vec@[j].seq@.len() <= 1,
                    forall|j: usize| j < n ==> #[trigger] pred.requires((&a.seq@[j as int],)),
                decreases n - i,
            {
                let elem = a.nth(i);
                let deflated_elem = deflate(pred, elem);
                proof {
                    assert(deflated_elem.seq@.len() <= 1);
                }
                deflated_vec.push(deflated_elem);
                i += 1;
            }
            let deflated = ArraySeqStPerS::<ArraySeqStPerS<T>> { seq: deflated_vec };
            flatten(&deflated)
        }

        // Algorithm 19.6: update a (i, x) = tabulate(lambda j. if i = j then x else a[j], |a|)
        fn update(a: &ArraySeqStPerS<T>, index: usize, item: T) -> ArraySeqStPerS<T> {
            ArraySeqStPerS::tabulate(
                &(|j: usize| -> (r: T)
                    requires j < a.seq@.len()
                {
                    if j == index { item.clone() } else { a.nth(j).clone() }
                }),
                a.length(),
            )
        }

        // Algorithm 19.7: is_empty a = |a| = 0
        fn is_empty(a: &ArraySeqStPerS<T>) -> bool {
            a.length() == 0
        }

        // Algorithm 19.7: is_singleton a = |a| = 1
        fn is_singleton(a: &ArraySeqStPerS<T>) -> bool {
            a.length() == 1
        }

        // Algorithm 19.8: iterate f x a = left-to-right traversal
        fn iterate<A, F: Fn(&A, &T) -> A>(a: &ArraySeqStPerS<T>, f: &F, seed: A) -> A {
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

        // Algorithm 19.9 (iterative): reduce using left fold
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, id: T) -> T {
            let len = a.length();
            let mut acc = id;
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len == a.seq@.len(),
                    forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                decreases len - i,
            {
                acc = f(&acc, a.nth(i));
                i += 1;
            }
            acc
        }

        // Algorithm 19.10 (iterative): scan using sequential left-to-right accumulation
        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, id: T) -> (ArraySeqStPerS<T>, T) {
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

        // Algorithm 19.8 (recursive): iterate f x a = if |a|=0 then x else iterate f (f(x,a[0])) a[1..|a|-1]
        fn iterate_rec<A, F: Fn(&A, &T) -> A>(a: &ArraySeqStPerS<T>, f: &F, seed: A) -> A
            decreases a.seq@.len()
        {
            let len = a.length();
            if len == 0 {
                seed
            } else if len == 1 {
                f(&seed, a.nth(0))
            } else {
                let new_seed = f(&seed, a.nth(0));
                let rest = a.subseq_copy(1, len - 1);
                <ArraySeqStPerS<T> as ArraySeqStPerTrait<T>>::iterate_rec(&rest, f, new_seed)
            }
        }

        // Algorithm 19.9 (recursive): reduce using divide-and-conquer
        fn reduce_rec<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, id: T) -> T
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
                let left_result = <ArraySeqStPerS<T> as ArraySeqStPerTrait<T>>::reduce_rec(&left, f, id.clone());
                let right_result = <ArraySeqStPerS<T> as ArraySeqStPerTrait<T>>::reduce_rec(&right, f, id);
                f(&left_result, &right_result)
            }
        }

        // Algorithm 19.10 (recursive): scan using divide-and-conquer
        fn scan_rec<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, id: T) -> (ArraySeqStPerS<T>, T)
            decreases a.seq@.len()
        {
            let len = a.length();
            if len == 0 {
                (<ArraySeqStPerS<T> as ArraySeqStPerTrait<T>>::empty(), id)
            } else if len == 1 {
                let total = f(&id, a.nth(0));
                let singleton = ArraySeqStPerS::tabulate(&|_i: usize| total.clone(), 1);
                (singleton, total)
            } else {
                let mid = len / 2;
                let left_seq = a.subseq_copy(0, mid);
                let right_seq = a.subseq_copy(mid, len - mid);
                let (left_scan, left_total) = <ArraySeqStPerS<T> as ArraySeqStPerTrait<T>>::scan_rec(&left_seq, f, id);
                let (right_scan, right_total) = <ArraySeqStPerS<T> as ArraySeqStPerTrait<T>>::scan_rec(&right_seq, f, left_total);
                let combined = <ArraySeqStPerS<T> as ArraySeqStPerTrait<T>>::append(&left_scan, &right_scan);
                (combined, right_total)
            }
        }

        // select helper for append_select
        fn select<'a>(a: &'a ArraySeqStPerS<T>, b: &'a ArraySeqStPerS<T>, i: usize) -> Option<&'a T> {
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
        fn append_select(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T> {
            let total = a.length() + b.length();
            ArraySeqStPerS::tabulate(
                &(|i: usize| -> (r: T)
                    requires i < a.seq@.len() + b.seq@.len()
                {
                    let opt = <ArraySeqStPerS<T> as ArraySeqStPerTrait<T>>::select(a, b, i);
                    proof { assert(opt.is_some()); }
                    opt.unwrap().clone()
                }),
                total,
            )
        }

        fn from_set(set: &SetStEph<T>) -> (result: ArraySeqStPerS<T>)
            where T: StT + Hash
        {
            let seq = set.to_seq();
            proof {
                // to_seq ensures: seq@.no_duplicates() && bijection with set@
                // Use unique_seq_to_set: no_duplicates ==> seq.len() == seq.to_set().len()
                let mapped = seq@.map(|_i: int, t: T| t@);
                mapped.unique_seq_to_set();
                // mapped.to_set() == set@ from the bijection ensures
                assert(mapped.to_set() =~= set@);
                assert(seq@.len() == set@.len());
            }
            ArraySeqStPerS { seq }
        }
    }

    // Helper: flatten - sums lengths of inner sequences
    #[verifier::external_body]
    fn flatten<T: View + Clone>(ss: &ArraySeqStPerS<ArraySeqStPerS<T>>) -> (result: ArraySeqStPerS<T>)
        ensures
            // For 2-element case (append)
            ss.seq@.len() == 2 ==> result.seq@.len() == ss.seq@[0].seq@.len() + ss.seq@[1].seq@.len(),
            // For filter case: result <= sum of inner lengths <= outer length (each inner <= 1)
            (forall|i: int| #![auto] 0 <= i < ss.seq@.len() ==> ss.seq@[i].seq@.len() <= 1)
                ==> result.seq@.len() <= ss.seq@.len(),
    {
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

    fn deflate<T: View + Clone + Eq, F: Fn(&T) -> bool>(pred: &F, x: &T) -> (result: ArraySeqStPerS<T>)
        requires pred.requires((x,))
        ensures result.seq@.len() <= 1
    {
        if pred(x) {
            <ArraySeqStPerS<T> as ArraySeqStPerRedefinableTrait<T>>::singleton(x.clone())
        } else {
            <ArraySeqStPerS<T> as ArraySeqStPerRedefinableTrait<T>>::empty()
        }
    }

    } // verus!

}
