//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! Parallel reduce using contraction technique (Chapter 27, Algorithm 27.2).
//! Uses the help-first scheduler for fork-join parallelism.
//! Verusified.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	7. proof fns
//	8. traits
//	9. impls

//		1. module

pub mod ReduceContractMtEph {

    use std::sync::Arc;
    use vstd::prelude::*;

    verus! {

    //		2. imports

    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Concurrency::Concurrency::StTInMtT;
    use crate::vstdplus::monoid::monoid::*;
    use crate::vstdplus::smart_ptrs::smart_ptrs::call_f;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;

    //		3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
        // Veracity: added broadcast groups
        crate::vstdplus::feq::feq::group_feq_axioms,
        vstd::seq_lib::group_seq_properties,
        vstd::seq_lib::group_to_multiset_ensures,
    };


    //		7. proof fns

    /// Monoid fold_left lemma: fold_left(s, x, f) == f(x, fold_left(s, id, f))
    /// when (f, id) is a monoid.
    proof fn lemma_fold_left_monoid<T>(s: Seq<T>, x: T, f: spec_fn(T, T) -> T, id: T)
        requires spec_monoid(f, id),
        ensures s.fold_left(x, f) == f(x, s.fold_left(id, f)),
        decreases s.len(),
    {
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

            s.lemma_fold_left_split(id, f, 2);
            let s_head = s.subrange(0, 2);
            assert(s_head =~= seq![s[0], s[1]]);

            lemma_fold_left_pair::<T>(s[0], s[1], f, id);
            assert(s.fold_left(id, f) == s_tail.fold_left(b[0], f));

            lemma_fold_left_monoid::<T>(s_tail, b[0], f, id);
            let s_tail_result = s_tail.fold_left(id, f);
            assert(s.fold_left(id, f) == f(b[0], s_tail_result));

            assert(s_tail.len() >= 2 && s_tail.len() % 2 == 0) by {
                assert(s_tail.len() == n - 2);
            }
            assert(b_tail =~= Seq::new(
                (s_tail.len() / 2) as nat,
                |i: int| f(s_tail[2 * i], s_tail[2 * i + 1]),
            ));

            lemma_contraction_even::<T>(s_tail, f, id);
            let b_tail_result = b_tail.fold_left(id, f);
            assert(s_tail_result == b_tail_result);
            assert(s.fold_left(id, f) == f(b[0], b_tail_result));

            lemma_fold_left_monoid::<T>(b_tail, b[0], f, id);
            assert(b_tail.fold_left(b[0], f) == f(b[0], b_tail_result));
            assert(s.fold_left(id, f) == b_tail.fold_left(b[0], f));

            b.lemma_fold_left_split(id, f, 1);
            assert(b.subrange(0, 1) =~= seq![b[0]]);
            assert(b.subrange(1, b.len() as int) =~= b_tail);
            lemma_fold_left_singleton::<T>(b[0], f, id);
            assert(b.fold_left(id, f) == b_tail.fold_left(b[0], f));
            assert(s.fold_left(id, f) == b.fold_left(id, f));
        }
    }

    pub trait ReduceContractMtEphTrait<T: StTInMtT> {
        /// Reduce a sequence using parallel contraction: contract→solve→expand.
        /// Subsumes Example 27.1 (Maximal Element): call with max and 0 identity.
        /// - APAS: Work Θ(n), Span Θ(log n) — Algorithm 27.2.
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — contraction via one-level join, each half sequential.
        fn reduce_contract_parallel<F: Fn(&T, &T) -> T + Send + Sync + 'static>(
            a: &ArraySeqMtEphS<T>,
            f: Arc<F>,
            Ghost(spec_f): Ghost<spec_fn(T, T) -> T>,
            id: T,
        ) -> (reduced: T)
            requires
                a.spec_len() <= usize::MAX,
                obeys_feq_clone::<T>(),
                spec_monoid(spec_f, id),
                forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) ==> ret == spec_f(x, y),
            ensures
                reduced == Seq::new(a.spec_len(), |i: int| a.spec_index(i)).fold_left(id, spec_f);
    }


    //		9. impls

    /// Parallel contraction: build b[j] = f(a[2j], a[2j+1]) using fork-join.
    /// Parallelism via the help-first scheduler's join.
    /// - APAS: N/A — Verus-specific helper (contraction step factored out for sharing).
    /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n/2) — two parallel halves via join, each Θ(n/4).
    pub fn contract_parallel<T: StTInMtT + Clone + 'static, F: Fn(&T, &T) -> T + Send + Sync + 'static>(
        a: &ArraySeqMtEphS<T>,
        f: &Arc<F>,
        Ghost(spec_f): Ghost<spec_fn(T, T) -> T>,
        half: usize,
    ) -> (b: ArraySeqMtEphS<T>)
        requires
            half == a.spec_len() / 2,
            a.spec_len() >= 2,
            half <= usize::MAX,
            obeys_feq_clone::<T>(),
            forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
            forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) ==> ret == spec_f(x, y),
        ensures
            b.spec_len() == half as nat,
            forall|j: int| #![trigger b.spec_index(j)] 0 <= j < half as int ==> {
                &&& 2 * j + 1 < a.spec_len()
                &&& b.spec_index(j) == spec_f(a.spec_index(2 * j), a.spec_index(2 * j + 1))
            },
    {
        use crate::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::join;

        let n = a.length();
        let mid_half = half / 2;

        // Clone array into Arc for sharing between closures
        let a_cloned = a.clone();
        proof {
            assert(a_cloned.seq@.len() == a.seq@.len());
            assert forall|i: int| 0 <= i < a.spec_len() implies
                a_cloned.spec_index(i) == a.spec_index(i)
            by {
                assert(cloned::<T>(a.seq@[i], a_cloned.seq@[i]));
                axiom_cloned_implies_eq_owned(a.seq@[i], a_cloned.seq@[i]);
            }
        }
        let a_arc = Arc::new(a_cloned);
        let f_left = Arc::clone(f);
        let f_right = Arc::clone(f);
        let a_left = Arc::clone(&a_arc);
        let a_right = Arc::clone(&a_arc);

        let ghost a_spec_len = a.spec_len();

        let fa = move || -> (r: Vec<T>)
            requires
                mid_half <= a_left.spec_len() / 2,
                a_left.spec_len() >= 2,
                a_left.spec_len() <= usize::MAX,
                a_left.spec_len() == a_spec_len,
                forall|x: &T, y: &T| #[trigger] f_left.requires((x, y)),
                forall|x: T, y: T, ret: T| f_left.ensures((&x, &y), ret) ==> ret == spec_f(x, y),
            ensures
                r@.len() == mid_half as int,
                forall|j: int| #![trigger r@[j]] 0 <= j < mid_half as int ==> {
                    &&& 2 * j + 1 < a_left.spec_len()
                    &&& r@[j] == spec_f(a_left.spec_index(2 * j), a_left.spec_index(2 * j + 1))
                },
        {
            let mut v: Vec<T> = Vec::with_capacity(mid_half);
            let mut i: usize = 0;
            while i < mid_half
                invariant
                    i <= mid_half,
                    mid_half <= a_left.spec_len() / 2,
                    a_left.spec_len() >= 2,
                    a_left.spec_len() <= usize::MAX,
                    v@.len() == i as int,
                    forall|x: &T, y: &T| #[trigger] f_left.requires((x, y)),
                    forall|x: T, y: T, ret: T| f_left.ensures((&x, &y), ret) ==> ret == spec_f(x, y),
                    forall|j: int| #![trigger v@[j]] 0 <= j < i as int ==> {
                        &&& 2 * j + 1 < a_left.spec_len()
                        &&& v@[j] == spec_f(a_left.spec_index(2 * j), a_left.spec_index(2 * j + 1))
                    },
                decreases mid_half - i,
            {
                assert(2 * (i as int) + 1 < a_left.spec_len()) by {
                    assert(i < mid_half);
                }
                let combined = call_f(&f_left, a_left.nth(2 * i), a_left.nth(2 * i + 1));
                v.push(combined);
                i += 1;
            }
            v
        };

        let fb = move || -> (r: Vec<T>)
            requires
                mid_half <= half,
                half == a_right.spec_len() / 2,
                a_right.spec_len() >= 2,
                a_right.spec_len() <= usize::MAX,
                a_right.spec_len() == a_spec_len,
                forall|x: &T, y: &T| #[trigger] f_right.requires((x, y)),
                forall|x: T, y: T, ret: T| f_right.ensures((&x, &y), ret) ==> ret == spec_f(x, y),
            ensures
                r@.len() == (half - mid_half) as int,
                forall|j: int| #![trigger r@[j]] 0 <= j < (half - mid_half) as int ==> {
                    &&& 2 * (mid_half as int + j) + 1 < a_right.spec_len()
                    &&& r@[j] == spec_f(a_right.spec_index(2 * (mid_half as int + j)), a_right.spec_index(2 * (mid_half as int + j) + 1))
                },
        {
            let mut v: Vec<T> = Vec::with_capacity(half - mid_half);
            let mut i: usize = 0;
            let count = half - mid_half;
            while i < count
                invariant
                    i <= count,
                    count == half - mid_half,
                    mid_half <= half,
                    half == a_right.spec_len() / 2,
                    a_right.spec_len() >= 2,
                    a_right.spec_len() <= usize::MAX,
                    v@.len() == i as int,
                    forall|x: &T, y: &T| #[trigger] f_right.requires((x, y)),
                    forall|x: T, y: T, ret: T| f_right.ensures((&x, &y), ret) ==> ret == spec_f(x, y),
                    forall|j: int| #![trigger v@[j]] 0 <= j < i as int ==> {
                        &&& 2 * (mid_half as int + j) + 1 < a_right.spec_len()
                        &&& v@[j] == spec_f(a_right.spec_index(2 * (mid_half as int + j)), a_right.spec_index(2 * (mid_half as int + j) + 1))
                    },
                decreases count - i,
            {
                let idx = mid_half + i;
                assert(2 * (idx as int) + 1 < a_right.spec_len()) by {
                    assert(idx < half);
                }
                let combined = call_f(&f_right, a_right.nth(2 * idx), a_right.nth(2 * idx + 1));
                v.push(combined);
                i += 1;
            }
            v
        };

        let (b_left, b_right) = join(fa, fb);

        // Concatenate left and right halves (vstd spec: vec@ == old(vec)@ + old(other)@)
        let mut b_vec = b_left;
        let ghost left_snap = b_vec@;
        let ghost right_snap = b_right@;
        let mut b_right_mut = b_right;
        b_vec.append(&mut b_right_mut);

        let b = ArraySeqMtEphS { seq: b_vec };

        proof {
            // b_vec@ == left_snap + right_snap (from vstd Vec::append spec)
            assert(b_vec@ =~= left_snap + right_snap);
            assert(b.spec_len() == half as nat);
            assert forall|j: int| #![trigger b.spec_index(j)] 0 <= j < half as int implies {
                &&& 2 * j + 1 < a.spec_len()
                &&& b.spec_index(j) == spec_f(a.spec_index(2 * j), a.spec_index(2 * j + 1))
            } by {
                if j < mid_half as int {
                    assert(b.spec_index(j) == left_snap[j]);
                } else {
                    let j2 = j - mid_half as int;
                    assert(b.spec_index(j) == right_snap[j2]);
                }
            }
        }

        b
    }

    impl<T: StTInMtT + Clone + 'static> ReduceContractMtEphTrait<T> for ArraySeqMtEphS<T> {
        fn reduce_contract_parallel<F: Fn(&T, &T) -> T + Send + Sync + 'static>(
            a: &ArraySeqMtEphS<T>,
            f: Arc<F>,
            Ghost(spec_f): Ghost<spec_fn(T, T) -> T>,
            id: T,
        ) -> (reduced: T)
            decreases a.spec_len(),
        {
            let n = a.length();
            let ghost s = Seq::new(a.spec_len(), |i: int| a.spec_index(i));

            // Base case: empty
            if n == 0 {
                proof {
                    assert(s =~= Seq::<T>::empty());
                }
                return id;
            }

            // Base case: single element — use f(id, a[0]) to avoid unspecified clone
            if n == 1 {
                let reduced = call_f(&f, &id, a.nth(0));
                proof {
                    reveal_with_fuel(Seq::fold_left, 2);
                    assert(s.drop_last() =~= Seq::<T>::empty());
                }
                return reduced;
            }

            // Contract: b[i] = f(a[2i], a[2i+1]) — parallel via join
            let half = n / 2;
            let b = contract_parallel(a, &f, Ghost(spec_f), half);

            let ghost b_seq = Seq::new(b.spec_len(), |i: int| b.spec_index(i));
            proof {
                assert(b.spec_len() == half as nat);
                assert forall|j: int| 0 <= j < half as int implies {
                    &&& 2 * j + 1 < s.len()
                    &&& b_seq[j] == spec_f(s[2 * j], s[2 * j + 1])
                } by {
                    assert(b_seq[j] == b.spec_index(j));
                }
            }

            // Recurse on contracted sequence
            let id_copy = call_f(&f, &id, &id);
            proof { assert(id_copy == id); }
            let contracted_result = Self::reduce_contract_parallel(&b, Arc::clone(&f), Ghost(spec_f), id_copy);

            // Expand: handle odd-length sequences
            if n % 2 == 1 {
                let last = a.nth(n - 1);
                let reduced = call_f(&f, &contracted_result, last);
                proof {
                    let s_even = s.subrange(0, (n - 1) as int);
                    let s_last_part = s.subrange((n - 1) as int, n as int);

                    s.lemma_fold_left_split(id, spec_f, (n - 1) as int);
                    assert(s_last_part =~= seq![s[(n - 1) as int]]);
                    assert(s_even.len() >= 2 && s_even.len() % 2 == 0) by {
                        assert(s_even.len() == n - 1);
                    }
                    assert(b_seq =~= Seq::new(
                        (s_even.len() / 2) as nat,
                        |i: int| spec_f(s_even[2 * i], s_even[2 * i + 1]),
                    ));
                    lemma_contraction_even::<T>(s_even, spec_f, id);
                }
                reduced
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
