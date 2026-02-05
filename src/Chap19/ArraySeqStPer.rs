//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 19 algorithms for ArraySeqStPer. Verusified.
//! Redefines Chap18 methods using tabulate as the core primitive.
//! Use the trait `ArraySeqStPerTrait` to access these implementations.

pub mod ArraySeqStPer {

    #[cfg(verus_keep_ghost)]
    use vstd::prelude::*;

    #[cfg(verus_keep_ghost)]
    pub use crate::Chap18::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerS;

    #[cfg(verus_keep_ghost)]
    use std::hash::Hash;

    #[cfg(verus_keep_ghost)]
    use crate::Chap05::SetStEph::SetStEph::{SetStEph, SetStEphTrait, valid_key_type};

    #[cfg(verus_keep_ghost)]
    use crate::Types::Types::StT;

    #[cfg(verus_keep_ghost)]
    verus! {

    broadcast use vstd::std_specs::vec::group_vec_axioms;
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;

    // Chapter 19 trait - provides alternative algorithmic implementations
    // Import and use this trait to get Chapter 19's algorithms
    pub trait ArraySeqStPerTrait<T: View + Clone>: Sized {
        spec fn spec_len(&self) -> nat;

        spec fn nth_spec(&self, i: int) -> T;

        fn empty() -> (result: Self)
            ensures result.spec_len() == 0;

        fn singleton(item: T) -> (result: Self)
            ensures result.spec_len() == 1;

        fn tabulate<F: Fn(usize) -> T>(f: &F, length: usize) -> (result: Self)
            requires
                length <= usize::MAX,
                forall|i: usize| i < length ==> #[trigger] f.requires((i,)),
            ensures
                result.spec_len() == length,
                forall|j: usize| j < length ==> f.ensures((j,), #[trigger] result.nth_spec(j as int));

        fn length(&self) -> (result: usize)
            ensures result == self.spec_len();

        fn nth(&self, i: usize) -> (result: &T)
            requires (i as int) < self.spec_len()
            ensures *result == self.nth_spec(i as int);

        fn map<U: Clone + View, F: Fn(&T) -> U>(a: &Self, f: &F) -> (result: ArraySeqStPerS<U>)
            requires
                a.spec_len() <= usize::MAX as int,
                forall|i: int| 0 <= i < a.spec_len() ==> #[trigger] f.requires((&a.nth_spec(i),))
            ensures
                result.spec_len() == a.spec_len(),
                forall|i: int| #![auto] 0 <= i < a.spec_len() ==> f.ensures((&a.nth_spec(i),), result.seq@[i]);

        fn append(a: &Self, b: &Self) -> (result: Self)
            requires a.spec_len() + b.spec_len() <= usize::MAX as int
            ensures result.spec_len() == a.spec_len() + b.spec_len();

        fn flatten(ss: &ArraySeqStPerS<ArraySeqStPerS<T>>) -> (result: Self)
            requires sum_lens(ss.seq@, ss.seq@.len() as int) <= usize::MAX as int
            ensures
                result.spec_len() == sum_lens(ss.seq@, ss.seq@.len() as int),
                ss.seq@.len() == 2 ==> result.spec_len() == ss.seq@[0].seq@.len() + ss.seq@[1].seq@.len(),
                (forall|i: int| #![auto] 0 <= i < ss.seq@.len() ==> ss.seq@[i].seq@.len() <= 1)
                    ==> result.spec_len() <= ss.seq@.len();

        fn filter<F: Fn(&T) -> bool>(a: &Self, pred: &F) -> (result: Self)
            requires
                a.spec_len() <= usize::MAX as int,
                forall|i: int| 0 <= i < a.spec_len() ==> #[trigger] pred.requires((&a.nth_spec(i),))
            ensures result.spec_len() <= a.spec_len();

        fn deflate<F: Fn(&T) -> bool>(f: &F, x: &T) -> (result: Self)
            requires f.requires((x,))
            ensures
                result.spec_len() <= 1,
                result.spec_len() == 1 ==> f.ensures((x,), true),
                result.spec_len() == 0 ==> f.ensures((x,), false);

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

    impl<T: View + Clone> ArraySeqStPerTrait<T> for ArraySeqStPerS<T> {

        open spec fn spec_len(&self) -> nat {
            self.seq@.len()
        }

        open spec fn nth_spec(&self, i: int) -> T {
            self.seq@[i]
        }

        fn tabulate<F: Fn(usize) -> T>(f: &F, length: usize) -> (result: ArraySeqStPerS<T>) {
            ArraySeqStPerS::<T>::tabulate(f, length)
        }

        fn length(&self) -> (result: usize) {
            self.seq.len()
        }

        fn nth(&self, i: usize) -> (result: &T) {
            &self.seq[i]
        }

        // Algorithm 19.1: empty = tabulate(lambda i._, 0)
        fn empty() -> ArraySeqStPerS<T> {
            ArraySeqStPerS::<T>::empty()
        }

        // Algorithm 19.2: singleton x = tabulate(lambda i.x, 1)
        fn singleton(item: T) -> ArraySeqStPerS<T> {
            ArraySeqStPerS::<T>::tabulate(&|_i: usize| item.clone(), 1)
        }

        // Algorithm 19.3: map f a = tabulate(lambda i.f(a[i]), |a|)
        fn map<U: Clone + View, F: Fn(&T) -> U>(a: &ArraySeqStPerS<T>, f: &F) -> (result: ArraySeqStPerS<U>) {
            let n = a.length();
            proof {
                assert forall|i: usize| i < n implies (i as int) < a.spec_len() && #[trigger] f.requires((&a.seq@[i as int],)) by {
                    assert(a.nth_spec(i as int) == a.seq@[i as int]);
                }
            }
            ArraySeqStPerS::<U>::tabulate(
                &(|i: usize| -> (r: U)
                    requires
                        (i as int) < a.spec_len(),
                        f.requires((&a.nth_spec(i as int),)),
                    ensures
                        f.ensures((&a.nth_spec(i as int),), r),
                {
                    f(a.nth(i))
                }),
                n,
            )
        }

        // Algorithm 19.4: append a b = flatten([a, b])
        fn append(a: &ArraySeqStPerS<T>, b: &ArraySeqStPerS<T>) -> ArraySeqStPerS<T> {
            let a_clone = ArraySeqStPerS { seq: a.seq.clone() };
            let b_clone = ArraySeqStPerS { seq: b.seq.clone() };
            let mut pair_vec: Vec<ArraySeqStPerS<T>> = Vec::with_capacity(2);
            pair_vec.push(a_clone);
            pair_vec.push(b_clone);
            let pair = ArraySeqStPerS::<ArraySeqStPerS<T>> { seq: pair_vec };
            proof {
                assert(pair.spec_len() == 2);
                assert(pair.seq@[0].seq@.len() == a.seq@.len());
                assert(pair.seq@[1].seq@.len() == b.seq@.len());
                // Prove flatten precondition.
                assert(sum_lens(pair.seq@, 0) == 0);
                assert(sum_lens(pair.seq@, 1) == a.seq@.len());
                assert(sum_lens(pair.seq@, 2) == a.seq@.len() + b.seq@.len());
            }
            Self::flatten(&pair)
        }

        // Concatenates all inner sequences into a single flat sequence.
        fn flatten(ss: &ArraySeqStPerS<ArraySeqStPerS<T>>) -> (result: ArraySeqStPerS<T>) {
            // First pass: compute total length.
            let ss_len = ss.seq.len();
            let mut total_len: usize = 0;
            let mut i: usize = 0;
            proof {
                lemma_sum_lens_monotonic(ss.seq@, 0, ss.seq@.len() as int);
            }
            while i < ss_len
                invariant
                    i <= ss_len,
                    ss_len == ss.seq@.len(),
                    total_len as int == sum_lens(ss.seq@, i as int),
                    sum_lens(ss.seq@, ss.seq@.len() as int) <= usize::MAX as int,
                    sum_lens(ss.seq@, i as int) <= sum_lens(ss.seq@, ss.seq@.len() as int),
                decreases ss_len - i,
            {
                proof {
                    lemma_sum_lens_monotonic(ss.seq@, (i + 1) as int, ss.seq@.len() as int);
                }
                total_len = total_len + ss.seq[i].seq.len();
                i = i + 1;
            }

            // Second pass: copy all elements.
            let mut result: Vec<T> = Vec::with_capacity(total_len);
            let mut j: usize = 0;
            while j < ss_len
                invariant
                    j <= ss_len,
                    ss_len == ss.seq@.len(),
                    result@.len() == sum_lens(ss.seq@, j as int),
                decreases ss_len - j,
            {
                let inner = &ss.seq[j];
                let inner_len = inner.seq.len();
                let mut k: usize = 0;
                while k < inner_len
                    invariant
                        k <= inner_len,
                        inner_len == inner.seq@.len(),
                        j < ss_len,
                        ss_len == ss.seq@.len(),
                        result@.len() == sum_lens(ss.seq@, j as int) + k as int,
                    decreases inner_len - k,
                {
                    result.push(inner.seq[k].clone());
                    k = k + 1;
                }
                j = j + 1;
            }

            proof {
                if ss.seq@.len() == 2 {
                    assert(sum_lens(ss.seq@, 2) == sum_lens(ss.seq@, 1) + ss.seq@[1].seq@.len() as int);
                    assert(sum_lens(ss.seq@, 1) == sum_lens(ss.seq@, 0) + ss.seq@[0].seq@.len() as int);
                    assert(sum_lens(ss.seq@, 0) == 0int);
                }
                if forall|i: int| #![auto] 0 <= i < ss.seq@.len() ==> ss.seq@[i].seq@.len() <= 1 {
                    lemma_sum_lens_bounded(ss.seq@, ss.seq@.len() as int);
                }
            }

            ArraySeqStPerS { seq: result }
        }

        // Algorithm 19.5: filter f a = flatten(map(deflate f, a))
        // where deflate f x = if f x then singleton x else empty (inlined below)
        fn filter<F: Fn(&T) -> bool>(a: &ArraySeqStPerS<T>, pred: &F) -> ArraySeqStPerS<T> {
            let n = a.length();
            let mut deflated_vec: Vec<ArraySeqStPerS<T>> = Vec::with_capacity(n);
            proof {
                // Connect outer requires (nth_spec) to seq@
                assert forall|j: usize| j < n implies #[trigger] pred.requires((&a.seq@[j as int],)) by {
                    assert(a.nth_spec(j as int) == a.seq@[j as int]);
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
                // Inline deflate: singleton if pred true, empty otherwise
                let deflated_elem = if pred(elem) {
                    ArraySeqStPerS::<T>::singleton(elem.clone())
                } else {
                    ArraySeqStPerS::<T>::empty()
                };
                proof {
                    assert(deflated_elem.seq@.len() <= 1);
                }
                deflated_vec.push(deflated_elem);
                i += 1;
            }
            let deflated = ArraySeqStPerS::<ArraySeqStPerS<T>> { seq: deflated_vec };
            proof {
                // Prove flatten precondition: sum_lens <= n <= usize::MAX.
                lemma_sum_lens_bounded(deflated.seq@, n as int);
            }
            Self::flatten(&deflated)
        }

        // deflate f x = if f x then singleton x else empty
        fn deflate<F: Fn(&T) -> bool>(f: &F, x: &T) -> (result: ArraySeqStPerS<T>) {
            if f(x) {
                Self::singleton(x.clone())
            } else {
                Self::empty()
            }
        }

        // Algorithm 19.6: update a (i, x) = tabulate(lambda j. if i = j then x else a[j], |a|)
        fn update(a: &ArraySeqStPerS<T>, index: usize, item: T) -> ArraySeqStPerS<T> {
            ArraySeqStPerS::<T>::tabulate(
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

        // Algorithm 19.9: reduce using divide-and-conquer
        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqStPerS<T>, f: &F, id: T) -> T
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
                let left_result = <ArraySeqStPerS<T> as ArraySeqStPerTrait<T>>::reduce(&left, f, id.clone());
                let right_result = <ArraySeqStPerS<T> as ArraySeqStPerTrait<T>>::reduce(&right, f, id);
                f(&left_result, &right_result)
            }
        }

        // Algorithm 19.10: scan using contraction (simplified sequential version)
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

        // Selects an element from concatenated sequences by index.
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
            ArraySeqStPerS::<T>::tabulate(
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

    // Spec function to sum lengths of first n inner sequences.
    pub open spec fn sum_lens<T>(ss: Seq<ArraySeqStPerS<T>>, n: int) -> int
        decreases n
    {
        if n <= 0 { 0 }
        else { sum_lens(ss, n - 1) + ss[n - 1].seq@.len() as int }
    }

    // If all inner lengths <= 1, then sum_lens(n) <= n.
    proof fn lemma_sum_lens_bounded<T>(ss: Seq<ArraySeqStPerS<T>>, n: int)
        requires
            0 <= n <= ss.len(),
            forall|i: int| #![auto] 0 <= i < ss.len() ==> ss[i].seq@.len() <= 1,
        ensures
            sum_lens(ss, n) <= n,
        decreases n,
    {
        if n <= 0 {
        } else {
            lemma_sum_lens_bounded(ss, n - 1);
        }
    }

    // sum_lens is monotonically increasing.
    proof fn lemma_sum_lens_monotonic<T>(ss: Seq<ArraySeqStPerS<T>>, a: int, b: int)
        requires
            0 <= a <= b <= ss.len(),
        ensures
            sum_lens(ss, a) <= sum_lens(ss, b),
        decreases b - a,
    {
        if a == b {
        } else {
            lemma_sum_lens_monotonic(ss, a, b - 1);
        }
    }

    } // verus!
}
