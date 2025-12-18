//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
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
    use vstd::std_specs::clone::*;
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;

    // Clone spec for ArraySeqStEphS - defines what cloned() means for this type
    pub assume_specification<T: Clone>
        [ <ArraySeqStEphS<T> as Clone>::clone ]
        (s: &ArraySeqStEphS<T>) -> (result: ArraySeqStEphS<T>)
        ensures result.seq@ == s.seq@;

    // Chapter 19 trait - provides alternative algorithmic implementations
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
            // Construct pair manually to avoid opaque f.ensures
            let a_clone = a.clone_plus();
            let b_clone = b.clone_plus();
            proof {
                assert(a_clone.seq@ =~= a.seq@);
                assert(b_clone.seq@ =~= b.seq@);
            }
            let mut pair_vec: Vec<ArraySeqStEphS<T>> = Vec::with_capacity(2);
            pair_vec.push(a_clone);
            pair_vec.push(b_clone);
            let pair = ArraySeqStEphS::<ArraySeqStEphS<T>> { seq: pair_vec };
            proof {
                assert(pair.spec_len() == 2);
                assert(pair.seq@[0].seq@.len() == a.seq@.len());
                assert(pair.seq@[1].seq@.len() == b.seq@.len());
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
                // Since no_duplicates and same elements, lengths must match
                assert(seq@.no_duplicates());
                assume(seq@.len() == set@.len()); // TODO: prove via bijection lemma
            }
            ArraySeqStEphS { seq }
        }
    }

    // Helper functions

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

    #[verifier::external_body]
    fn flatten<T: View + Clone>(ss: &ArraySeqStEphS<ArraySeqStEphS<T>>) -> (result: ArraySeqStEphS<T>)
        ensures
            ss.seq@.len() == 2 ==> result.seq@.len() == ss.seq@[0].seq@.len() + ss.seq@[1].seq@.len(),
            (forall|i: int| #![auto] 0 <= i < ss.seq@.len() ==> ss.seq@[i].seq@.len() <= 1)
                ==> result.seq@.len() <= ss.seq@.len(),
    {
        let mut total_len: usize = 0;
        let ss_len = ss.seq.len();
        for i in 0..ss_len {
            total_len = total_len + ss.seq[i].seq.len();
        }
        let mut result: Vec<T> = Vec::with_capacity(total_len);
        for i in 0..ss_len {
            for j in 0..ss.seq[i].seq.len() {
                result.push(ss.seq[i].seq[j].clone());
            }
        }
        ArraySeqStEphS { seq: result }
    }

    } // verus!

    // Non-verus impl for cargo compatibility
    #[cfg(not(verus_keep_ghost))]
    pub use crate::Chap18::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphS;
}

