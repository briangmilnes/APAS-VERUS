//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Sequential reduce using contraction technique (Chapter 27, Algorithm 27.2).
//! Verusified.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	6. proof functions
//	8. traits
//	9. impls

//		1. module

pub mod ReduceContractStEph {

    use vstd::prelude::*;

    verus! {

    //		2. imports

    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::vstdplus::monoid::monoid::*;
    use crate::Types::Types::*;

    //		3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
    };

    //		6. proof functions

    /// Monoid fold_left lemma: fold_left(s, x, f) == f(x, fold_left(s, id, f))
    /// when (f, id) is a monoid.
    proof fn lemma_fold_left_monoid<T>(s: Seq<T>, x: T, f: spec_fn(T, T) -> T, id: T)
        requires spec_monoid(f, id),
        ensures s.fold_left(x, f) == f(x, s.fold_left(id, f)),
        decreases s.len(),
    {
        reveal(Seq::fold_left);
        if s.len() == 0 {
        } else {
            lemma_fold_left_monoid::<T>(s.drop_last(), x, f, id);
            lemma_fold_left_monoid::<T>(s.drop_last(), id, f, id);
        }
    }

    /// Helper: fold_left of a 2-element sequence equals f(a, b) under a monoid.
    proof fn lemma_fold_left_pair<T>(a: T, b: T, f: spec_fn(T, T) -> T, id: T)
        requires spec_monoid(f, id),
        ensures seq![a, b].fold_left(id, f) == f(a, b),
    {
        let s = seq![a, b];
        reveal_with_fuel(Seq::fold_left, 3);
        assert(s.drop_last() =~= seq![a]);
        assert(seq![a].drop_last() =~= Seq::<T>::empty());
        // fold_left([a, b], id, f) = f(fold_left([a], id, f), b)
        //                          = f(f(fold_left([], id, f), a), b)
        //                          = f(f(id, a), b) = f(a, b)
    }

    /// Helper: fold_left of a 1-element sequence equals f(id, a) = a.
    proof fn lemma_fold_left_singleton<T>(a: T, f: spec_fn(T, T) -> T, id: T)
        requires spec_monoid(f, id),
        ensures seq![a].fold_left(id, f) == a,
    {
        reveal_with_fuel(Seq::fold_left, 2);
        assert(seq![a].drop_last() =~= Seq::<T>::empty());
    }

    /// Contraction lemma: for an even-length sequence, folding the original equals
    /// folding the contracted (pairwise-combined) sequence under a monoid.
    proof fn lemma_contraction_even<T>(s: Seq<T>, f: spec_fn(T, T) -> T, id: T)
        requires
            spec_monoid(f, id),
            s.len() >= 2,
            s.len() % 2 == 0,
        ensures
            s.fold_left(id, f) == Seq::new(
                (s.len() / 2) as nat,
                |i: int| f(s[2 * i], s[2 * i + 1]),
            ).fold_left(id, f),
        decreases s.len(),
    {
        let n = s.len();
        let half = (n / 2) as nat;
        let b = Seq::new(half, |i: int| f(s[2 * i], s[2 * i + 1]));

        if n == 2 {
            assert(s =~= seq![s[0], s[1]]);
            lemma_fold_left_pair::<T>(s[0], s[1], f, id);
            assert(s.fold_left(id, f) == f(s[0], s[1]));
            assert(b =~= seq![f(s[0], s[1])]);
            lemma_fold_left_singleton::<T>(f(s[0], s[1]), f, id);
            assert(b.fold_left(id, f) == f(s[0], s[1]));
        } else {
            let s_tail = s.subrange(2, n as int);
            let b_tail = b.subrange(1, b.len() as int);

            // (A) Split s at position 2
            s.lemma_fold_left_split(id, f, 2);
            let s_head = s.subrange(0, 2);
            assert(s_head =~= seq![s[0], s[1]]);

            // (B) s_head.fold_left(id, f) == f(s[0], s[1]) == b[0]
            lemma_fold_left_pair::<T>(s[0], s[1], f, id);
            assert(s.fold_left(id, f) == s_tail.fold_left(b[0], f));

            // (C) Factor out b[0]
            lemma_fold_left_monoid::<T>(s_tail, b[0], f, id);
            let s_tail_result = s_tail.fold_left(id, f);
            assert(s.fold_left(id, f) == f(b[0], s_tail_result));

            // (D) b_tail corresponds to contraction of s_tail
            assert(s_tail.len() >= 2 && s_tail.len() % 2 == 0) by {
                assert(s_tail.len() == n - 2);
            }
            assert(b_tail =~= Seq::new(
                (s_tail.len() / 2) as nat,
                |i: int| f(s_tail[2 * i], s_tail[2 * i + 1]),
            ));

            // (E) Induction: s_tail.fold_left(id, f) == b_tail.fold_left(id, f)
            lemma_contraction_even::<T>(s_tail, f, id);
            let b_tail_result = b_tail.fold_left(id, f);
            assert(s_tail_result == b_tail_result);
            assert(s.fold_left(id, f) == f(b[0], b_tail_result));

            // (F) Factor b[0] back in: f(b[0], b_tail_result) == b_tail.fold_left(b[0], f)
            lemma_fold_left_monoid::<T>(b_tail, b[0], f, id);
            assert(b_tail.fold_left(b[0], f) == f(b[0], b_tail_result));
            assert(s.fold_left(id, f) == b_tail.fold_left(b[0], f));

            // (G) Connect b_tail.fold_left(b[0], f) to b.fold_left(id, f) via split at 1
            b.lemma_fold_left_split(id, f, 1);
            assert(b.subrange(0, 1) =~= seq![b[0]]);
            assert(b.subrange(1, b.len() as int) =~= b_tail);
            lemma_fold_left_singleton::<T>(b[0], f, id);
            // b.fold_left(id, f) == b_tail.fold_left(seq![b[0]].fold_left(id, f), f)
            //                     == b_tail.fold_left(b[0], f)
            assert(b.fold_left(id, f) == b_tail.fold_left(b[0], f));
            assert(s.fold_left(id, f) == b.fold_left(id, f));
        }
    }

    //		8. traits

    pub trait ReduceContractStEphTrait<T: StT> {
        /// Reduce a sequence using contraction: contract→solve→expand.
        /// APAS Algorithm 27.2: Work Θ(n), Span Θ(n) (sequential).
        fn reduce_contract<F: Fn(&T, &T) -> T>(
            a: &ArraySeqStEphS<T>,
            f: &F,
            Ghost(spec_f): Ghost<spec_fn(T, T) -> T>,
            id: T,
        ) -> (result: T)
            requires
                a.spec_len() <= usize::MAX,
                spec_monoid(spec_f, id),
                forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) ==> ret == spec_f(x, y),
            ensures
                result == Seq::new(a.spec_len(), |i: int| a.spec_index(i)).fold_left(id, spec_f);
    }

    //		9. impls

    impl<T: StT + Clone> ReduceContractStEphTrait<T> for ArraySeqStEphS<T> {
        fn reduce_contract<F: Fn(&T, &T) -> T>(
            a: &ArraySeqStEphS<T>,
            f: &F,
            Ghost(spec_f): Ghost<spec_fn(T, T) -> T>,
            id: T,
        ) -> (result: T)
            decreases a.spec_len(),
        {
            let n = a.length();
            let ghost s = Seq::new(a.spec_len(), |i: int| a.spec_index(i));

            // Base case: empty
            if n == 0 {
                proof {
                    reveal(Seq::fold_left);
                    assert(s =~= Seq::<T>::empty());
                }
                return id;
            }

            // Base case: single element — use f(id, a[0]) to avoid unspecified clone
            if n == 1 {
                let result = f(&id, a.nth(0));
                proof {
                    reveal_with_fuel(Seq::fold_left, 2);
                    assert(s.drop_last() =~= Seq::<T>::empty());
                }
                return result;
            }

            // Contract: b[i] = f(a[2i], a[2i+1])
            let half = n / 2;
            let mut b_vec: Vec<T> = Vec::with_capacity(half);
            let mut i: usize = 0;
            while i < half
                invariant
                    i <= half,
                    half == n / 2,
                    n == a.spec_len(),
                    n >= 2,
                    half <= n,
                    s == Seq::new(a.spec_len(), |k: int| a.spec_index(k)),
                    b_vec@.len() == i as int,
                    forall|j: int| #![trigger b_vec@[j]] 0 <= j < i as int ==> {
                        &&& 2 * j + 1 < s.len()
                        &&& b_vec@[j] == spec_f(s[2 * j], s[2 * j + 1])
                    },
                    forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                    forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) ==> ret == spec_f(x, y),
                decreases half - i,
            {
                assert(2 * (i as int) + 1 < n as int) by {
                    assert(i < half);
                }
                let left = a.nth(2 * i);
                let right = a.nth(2 * i + 1);
                let combined = f(left, right);
                b_vec.push(combined);
                i += 1;
            }
            let b = ArraySeqStEphS { seq: b_vec };

            let ghost b_seq = Seq::new(b.spec_len(), |i: int| b.spec_index(i));
            proof {
                assert(b.spec_len() == half as nat);
                assert forall|j: int| 0 <= j < half as int implies {
                    &&& 2 * j + 1 < s.len()
                    &&& b_seq[j] == spec_f(s[2 * j], s[2 * j + 1])
                } by {
                    assert(b_seq[j] == b_vec@[j]);
                }
            }

            // Recurse on contracted sequence
            let contracted_result = Self::reduce_contract(&b, f, Ghost(spec_f), id);

            // Expand: handle odd-length sequences
            if n % 2 == 1 {
                let last = a.nth(n - 1);
                let result = f(&contracted_result, last);
                proof {
                    let s_even = s.subrange(0, (n - 1) as int);
                    let s_last_part = s.subrange((n - 1) as int, n as int);

                    // s.fold_left(id, f) == s_last_part.fold_left(s_even.fold_left(id, f), f)
                    s.lemma_fold_left_split(id, spec_f, (n - 1) as int);
                    assert(s_last_part =~= seq![s[(n - 1) as int]]);

                    // s_last_part has one element, fold equals f(acc, s[n-1])
                    reveal(Seq::fold_left);

                    // s_even.fold_left(id, f) == b_seq.fold_left(id, f) by contraction
                    assert(s_even.len() >= 2 && s_even.len() % 2 == 0) by {
                        assert(s_even.len() == n - 1);
                    }
                    assert(b_seq =~= Seq::new(
                        (s_even.len() / 2) as nat,
                        |i: int| spec_f(s_even[2 * i], s_even[2 * i + 1]),
                    ));
                    lemma_contraction_even::<T>(s_even, spec_f, id);
                }
                result
            } else {
                proof {
                    assert(b_seq =~= Seq::new(
                        (s.len() / 2) as nat,
                        |i: int| spec_f(s[2 * i], s[2 * i + 1]),
                    ));
                    lemma_contraction_even::<T>(s, spec_f, id);
                }
                contracted_result
            }
        }
    }

    } // verus!
} // mod
