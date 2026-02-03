//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 19 algorithms for ArraySeqStEph. Verusified.
//! Redefines Chap18 methods using tabulate as the core primitive.
//! Use the trait `ArraySeqStEphTrait` to access these implementations.

pub mod ArraySeqStEph {

    #[cfg(verus_keep_ghost)]
    use vstd::prelude::*;

    #[cfg(verus_keep_ghost)]
    pub use crate::Chap18::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS;

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

    // The chapter 19 trait provides alternative algorithmic implementations of seq a fixed set of primitives.
    pub trait ArraySeqStEphTrait<T: View + Clone>: Sized {
        spec fn spec_len(&self) -> nat;

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

        fn map<U: View + Clone, F: Fn(&T) -> U>(a: &Self, f: &F) -> ArraySeqStEphS<U>
            where Self: Sized
            requires
                a.spec_len() <= usize::MAX as int,
                forall|i: int| 0 <= i < a.spec_len() ==> #[trigger] f.requires((&a.nth_spec(i),));

        spec fn nth_spec(&self, i: int) -> T;

        fn nth(&self, i: usize) -> (result: &T)
            requires (i as int) < self.spec_len()
            ensures *result == self.nth_spec(i as int);

        fn length(&self) -> (result: usize)
            ensures result == self.spec_len();

        fn append(a: &Self, b: &Self) -> (result: Self)
            requires a.spec_len() + b.spec_len() <= usize::MAX as nat
            ensures result.spec_len() == a.spec_len() + b.spec_len();

        fn filter<F: Fn(&T) -> bool>(a: &Self, pred: &F) -> (result: Self)
            requires
                a.spec_len() <= usize::MAX as int,
                forall|i: int| 0 <= i < a.spec_len() ==> #[trigger] pred.requires((&a.nth_spec(i),)),
            ensures
                result.spec_len() <= a.spec_len();

        fn update(a: &Self, index: usize, item: T) -> (result: Self)
            requires (index as int) < a.spec_len()
            ensures result.spec_len() == a.spec_len();

        fn is_empty(a: &Self) -> (result: bool)
            ensures result == (a.spec_len() == 0);

        fn is_singleton(a: &Self) -> (result: bool)
            ensures result == (a.spec_len() == 1);

        fn iterate<A: Clone, F: Fn(&A, &T) -> A>(a: &Self, f: &F, x: A) -> A
            requires
                forall|acc: &A, elem: &T| #[trigger] f.requires((acc, elem));

        fn reduce<F: Fn(&T, &T) -> T>(a: &Self, f: &F, id: T) -> T
            requires
                forall|x: &T, y: &T| #[trigger] f.requires((x, y));

        fn scan<F: Fn(&T, &T) -> T>(a: &Self, f: &F, id: T) -> (ArraySeqStEphS<T>, T)
            requires
                forall|x: &T, y: &T| #[trigger] f.requires((x, y));

        fn select(a: &Self, b: &Self, index: usize) -> (result: Option<T>)
            ensures (index as int) < a.spec_len() + b.spec_len() ==> result.is_some();

        fn append_select(a: &Self, b: &Self) -> (result: Self)
            requires a.spec_len() + b.spec_len() <= usize::MAX as nat
            ensures result.spec_len() == a.spec_len() + b.spec_len();

        fn deflate<F: Fn(&T) -> bool>(f: &F, x: &T) -> (result: Self)
            requires f.requires((x,))
            ensures result.spec_len() <= 1;

        fn from_set(set: &SetStEph<T>) -> (result: Self)
            where T: StT + Hash
            requires valid_key_type::<T>()
            ensures result.spec_len() == set@.len();
    }

    impl<T: View + Clone> ArraySeqStEphTrait<T> for ArraySeqStEphS<T> {
        open spec fn spec_len(&self) -> nat {
            self.seq@.len() as nat
        }

        fn empty() -> (result: ArraySeqStEphS<T>) {
            ArraySeqStEphS { seq: Vec::new() }
        }

        fn singleton(item: T) -> (result: ArraySeqStEphS<T>) {
            let mut seq = Vec::with_capacity(1);
            seq.push(item);
            ArraySeqStEphS { seq }
        }

        fn tabulate<F: Fn(usize) -> T>(f: &F, length: usize) -> (result: ArraySeqStEphS<T>)
        {
            let mut seq = Vec::with_capacity(length);
            let mut i: usize = 0;
            while i < length
                invariant
                    i <= length,
                    seq@.len() == i as int,
                    forall|j: usize| j < length ==> #[trigger] f.requires((j,)),
                    forall|j: usize| #![auto] j < i ==> f.ensures((j,), seq@[j as int]),
                decreases length - i,
            {
                seq.push(f(i));
                i += 1;
            }
            let result = ArraySeqStEphS { seq };
            proof {
                // The invariant gives us: forall|j| j < length ==> f.ensures((j,), seq@[j])
                // We need to prove: forall|j| j < length ==> f.ensures((j,), result.nth_spec(j))
                // Since result.seq == seq and result.nth_spec(j) == result.seq@[j] == seq@[j]
                assert forall|j: usize| j < length implies #[trigger] f.ensures((j,), result.nth_spec(j as int)) by {
                    assert(result.seq@ == seq@);
                    assert(result.nth_spec(j as int) == seq@[j as int]);
                    assert(f.ensures((j,), seq@[j as int]));
                }
            }
            result
        }

        open spec fn nth_spec(&self, i: int) -> T {
            self.seq@[i]
        }

        fn nth(&self, i: usize) -> (result: &T) {
            &self.seq[i]
        }

        fn length(&self) -> (result: usize) {
            self.seq.len()
        }

        fn map<U: View + Clone, F: Fn(&T) -> U>(a: &ArraySeqStEphS<T>, f: &F) -> (result: ArraySeqStEphS<U>) {
            ArraySeqStEphS::<U>::tabulate(
                &(|i: usize| -> (r: U)
                    requires
                        (i as int) < a.spec_len(),
                        f.requires((&a.nth_spec(i as int),)),
                {
                    let elem = &a.seq[i];
                    f(elem)
                }),
                a.seq.len(),
            )
        }

        fn append(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> (result: ArraySeqStEphS<T>) {
            // Clone inner Vecs (vstd has clone spec for Vec)
            let a_clone = ArraySeqStEphS { seq: a.seq.clone() };
            let b_clone = ArraySeqStEphS { seq: b.seq.clone() };
            let mut pair_vec: Vec<ArraySeqStEphS<T>> = Vec::with_capacity(2);
            pair_vec.push(a_clone);
            pair_vec.push(b_clone);
            let pair = ArraySeqStEphS::<ArraySeqStEphS<T>> { seq: pair_vec };
            proof {
                assert(pair.spec_len() == 2);
                assert(pair.seq@[0].seq@.len() == a.seq@.len());
                assert(pair.seq@[1].seq@.len() == b.seq@.len());
                // Prove flatten precondition
                assert(sum_lens(pair.seq@, 0) == 0);
                assert(sum_lens(pair.seq@, 1) == a.seq@.len());
                assert(sum_lens(pair.seq@, 2) == a.seq@.len() + b.seq@.len());
            }
            flatten(&pair)
        }

        fn filter<F: Fn(&T) -> bool>(a: &ArraySeqStEphS<T>, pred: &F) -> (result: ArraySeqStEphS<T>) {
            // Build deflated array manually to avoid opaque f.ensures
            let n = a.seq.len();
            let mut deflated_vec: Vec<ArraySeqStEphS<T>> = Vec::with_capacity(n);
            proof {
                // Connect nth_spec to seq@ for invariant
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
                let elem = &a.seq[i];
                let deflated_elem = if pred(elem) {
                    Self::singleton(elem.clone())
                } else {
                    Self::empty()
                };
                proof {
                    assert(deflated_elem.seq@.len() <= 1);
                }
                deflated_vec.push(deflated_elem);
                i += 1;
            }
            let deflated = ArraySeqStEphS::<ArraySeqStEphS<T>> { seq: deflated_vec };
            proof {
                // Prove flatten precondition: sum_lens <= n <= usize::MAX
                lemma_sum_lens_bounded(deflated.seq@, n as int);
            }
            flatten(&deflated)
        }

        fn update(a: &ArraySeqStEphS<T>, index: usize, item: T) -> (result: ArraySeqStEphS<T>) {
            Self::tabulate(
                &(|j: usize| -> (r: T)
                    requires j < a.seq@.len()
                {
                    if j == index { item.clone() } else { a.seq[j].clone() }
                }),
                a.seq.len(),
            )
        }

        fn is_empty(a: &ArraySeqStEphS<T>) -> (result: bool) {
            a.seq.len() == 0
        }

        fn is_singleton(a: &ArraySeqStEphS<T>) -> (result: bool) {
            a.seq.len() == 1
        }

        fn iterate<A: Clone, F: Fn(&A, &T) -> A>(a: &ArraySeqStEphS<T>, f: &F, x: A) -> A {
            let mut acc = x;
            let mut i: usize = 0;
            while i < a.seq.len()
                invariant
                    i <= a.seq@.len(),
                    forall|acc: &A, elem: &T| #[trigger] f.requires((acc, elem)),
                decreases a.seq@.len() - i,
            {
                acc = f(&acc, &a.seq[i]);
                i += 1;
            }
            acc
        }

        fn reduce<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> T {
            if a.seq.len() == 0 {
                id
            } else if a.seq.len() == 1 {
                a.seq[0].clone()
            } else {
                let mid = a.seq.len() / 2;
                let left = subseq_copy(a, 0, mid);
                let right = subseq_copy(a, mid, a.seq.len() - mid);
                let left_result = Self::reduce(&left, f, id.clone());
                let right_result = Self::reduce(&right, f, id);
                f(&left_result, &right_result)
            }
        }

        fn scan<F: Fn(&T, &T) -> T>(a: &ArraySeqStEphS<T>, f: &F, id: T) -> (ArraySeqStEphS<T>, T) {
            if a.seq.len() == 0 {
                return (Self::empty(), id);
            }
            let mut acc = id.clone();
            let mut results: Vec<T> = Vec::with_capacity(a.seq.len());
            let mut i: usize = 0;
            while i < a.seq.len()
                invariant
                    i <= a.seq@.len(),
                    results@.len() == i as int,
                    forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                decreases a.seq@.len() - i,
            {
                acc = f(&acc, &a.seq[i]);
                results.push(acc.clone());
                i += 1;
            }
            (ArraySeqStEphS { seq: results }, acc)
        }

        fn select(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>, index: usize) -> (result: Option<T>) {
            if index < a.seq.len() {
                Some(a.seq[index].clone())
            } else {
                let offset = index - a.seq.len();
                if offset < b.seq.len() {
                    Some(b.seq[offset].clone())
                } else {
                    None
                }
            }
        }

        fn append_select(a: &ArraySeqStEphS<T>, b: &ArraySeqStEphS<T>) -> (result: ArraySeqStEphS<T>) {
            Self::tabulate(
                &(|i: usize| -> (r: T)
                    requires i < a.seq@.len() + b.seq@.len()
                {
                    Self::select(a, b, i).unwrap()
                }),
                a.seq.len() + b.seq.len(),
            )
        }

        fn deflate<F: Fn(&T) -> bool>(f: &F, x: &T) -> (result: ArraySeqStEphS<T>) {
            if f(x) {
                Self::singleton(x.clone())
            } else {
                Self::empty()
            }
        }

        fn from_set(set: &SetStEph<T>) -> (result: ArraySeqStEphS<T>)
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
            ArraySeqStEphS { seq }
        }
    }

    fn subseq_copy<T: View + Clone>(a: &ArraySeqStEphS<T>, start: usize, len: usize) -> (result: ArraySeqStEphS<T>)
        requires
            start as int + len as int <= a.seq@.len(),
        ensures
            result.seq@.len() == len,
    {
        let a_len = a.seq.len();
        let mut result: Vec<T> = Vec::with_capacity(len);
        let mut i: usize = 0;
        while i < len
            invariant
                i <= len,
                result@.len() == i as int,
                start as int + len as int <= a_len as int,
                a_len as int == a.seq@.len(),
            decreases len - i,
        {
            result.push(a.seq[start + i].clone());
            i += 1;
        }
        ArraySeqStEphS { seq: result }
    }

    // Spec function to sum lengths of first n inner sequences
    spec fn sum_lens<T>(ss: Seq<ArraySeqStEphS<T>>, n: int) -> int
        decreases n
    {
        if n <= 0 { 0 }
        else { sum_lens(ss, n - 1) + ss[n - 1].seq@.len() as int }
    }

    // Lemma: if all inner lengths <= 1, then sum_lens(n) <= n
    proof fn lemma_sum_lens_bounded<T>(ss: Seq<ArraySeqStEphS<T>>, n: int)
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

    // Lemma: sum_lens is monotonically increasing
    proof fn lemma_sum_lens_monotonic<T>(ss: Seq<ArraySeqStEphS<T>>, a: int, b: int)
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

    fn flatten<T: View + Clone>(ss: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> (result: ArraySeqStEphS<T>)
        requires
            sum_lens(ss.seq@, ss.seq@.len() as int) <= usize::MAX as int,
        ensures
            ss.seq@.len() == 2 ==> result.seq@.len() == ss.seq@[0].seq@.len() + ss.seq@[1].seq@.len(),
            (forall|i: int| #![auto] 0 <= i < ss.seq@.len() ==> ss.seq@[i].seq@.len() <= 1)
                ==> result.seq@.len() <= ss.seq@.len(),
    {
        // First pass: compute total length
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

        // Second pass: copy all elements
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

        ArraySeqStEphS { seq: result }
    }

    } // verus!

    // Non-verus impl for cargo compatibility
    #[cfg(not(verus_keep_ghost))]
    pub use crate::Chap18::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS;
}
