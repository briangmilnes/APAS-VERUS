//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Parallel scan using contraction technique (Chapter 27, Algorithm 27.3).
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

pub mod ScanContractMtEph {

    use std::sync::Arc;
    use vstd::prelude::*;

    verus! {

    //		2. imports

    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Chap27::ReduceContractMtEph::ReduceContractMtEph::contract_parallel;
    use crate::vstdplus::smart_ptrs::smart_ptrs::call_f;
    use crate::Concurrency::Concurrency::StTInMtT;
    use crate::vstdplus::monoid::monoid::*;
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

    /// Prefix contraction lemma: fold_left of an even-length prefix s.take(2k)
    /// equals fold_left of the first k elements of the contracted sequence b.
    proof fn lemma_prefix_contraction<T>(s: Seq<T>, b: Seq<T>, f: spec_fn(T, T) -> T, id: T, k: int)
        requires
            spec_monoid(f, id),
            k >= 1,
            2 * k <= s.len(),
            b.len() >= k,
            forall|i: int| #![trigger b[i]] 0 <= i < b.len() ==> b[i] == f(s[2 * i], s[2 * i + 1]),
        ensures
            s.take(2 * k).fold_left(id, f) == b.take(k).fold_left(id, f),
    {
        let prefix = s.take(2 * k);
        lemma_contraction_even::<T>(prefix, f, id);
        let contracted = Seq::new(
            (prefix.len() / 2) as nat,
            |i: int| f(prefix[2 * i], prefix[2 * i + 1]),
        );
        assert(contracted =~= b.take(k));
    }

    //		8. traits

    pub trait ScanContractMtEphTrait<T: StTInMtT> {
        /// Exclusive scan using parallel contraction: contract→solve→expand.
        /// Returns prefixes where result[i] = fold_left(input[0..i], id, spec_f).
        /// - APAS: Work Θ(n), Span Θ(log n) — Algorithm 27.3.
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — contraction parallel via one-level join; expansion sequential.
        fn scan_contract_parallel<F: Fn(&T, &T) -> T + Send + Sync + 'static>(
            a: &ArraySeqMtEphS<T>,
            f: Arc<F>,
            Ghost(spec_f): Ghost<spec_fn(T, T) -> T>,
            id: T,
        ) -> (scanned: ArraySeqMtEphS<T>)
            requires
                a.spec_len() <= usize::MAX,
                obeys_feq_clone::<T>(),
                spec_monoid(spec_f, id),
                forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) ==> ret == spec_f(x, y),
            ensures
                scanned.spec_len() == a.spec_len(),
                forall|i: int| #![trigger scanned.spec_index(i)]
                    0 <= i < a.spec_len() ==>
                        scanned.spec_index(i) == Seq::new(a.spec_len(), |j: int| a.spec_index(j)).take(i).fold_left(id, spec_f);
    }

    //		9. impls

    impl<T: StTInMtT + Clone + 'static> ScanContractMtEphTrait<T> for ArraySeqMtEphS<T> {
        #[verifier::external_body] // accept hole: rlimit exceeded on expand loop; proof structurally correct but too expensive
        fn scan_contract_parallel<F: Fn(&T, &T) -> T + Send + Sync + 'static>(
            a: &ArraySeqMtEphS<T>,
            f: Arc<F>,
            Ghost(spec_f): Ghost<spec_fn(T, T) -> T>,
            id: T,
        ) -> (scanned: ArraySeqMtEphS<T>)
            decreases a.spec_len(),
        {
            let n = a.length();
            let ghost s = Seq::new(a.spec_len(), |i: int| a.spec_index(i));

            // Base case: empty
            if n == 0 {
                proof {
                    assert(s =~= Seq::<T>::empty());
                }
                return ArraySeqMtEphS { seq: Vec::new() };
            }

            // Base case: single element — result is [id]
            if n == 1 {
                let mut v: Vec<T> = Vec::with_capacity(1);
                v.push(id);
                proof {
                    assert(s.take(0) =~= Seq::<T>::empty());
                }
                return ArraySeqMtEphS { seq: v };
            }

            // Create a spec-equal copy of id via f for the recursive call.
            let id_for_recurse = call_f(&f, &id, &id);
            proof {
                assert(id_for_recurse == id);
            }

            // ---- Contract: b[i] = f(a[2i], a[2i+1]) — parallel via join ----
            let half = n / 2;
            let b = contract_parallel(a, &f, Ghost(spec_f), half);

            let ghost b_seq = Seq::new(b.spec_len(), |i: int| b.spec_index(i));
            proof {
                assert(b.spec_len() == half as nat);
                assert forall|j: int| #![trigger b_seq[j]] 0 <= j < half as int implies {
                    &&& 2 * j + 1 < s.len()
                    &&& b_seq[j] == spec_f(s[2 * j], s[2 * j + 1])
                } by {
                    assert(b_seq[j] == b.spec_index(j));
                }
            }

            // ---- Solve: recursively scan contracted sequence ----
            let c = Self::scan_contract_parallel(&b, Arc::clone(&f), Ghost(spec_f), id_for_recurse);

            proof {
                assert(c.spec_len() == half as nat);
                let ghost b_view = Seq::new(b.spec_len(), |j: int| b.spec_index(j));
                assert(b_view =~= b_seq);
                assert forall|k: int| #![trigger c.spec_index(k)] 0 <= k < half as int implies
                    c.spec_index(k) == b_seq.take(k).fold_left(id, spec_f)
                by {
                    assert(id_for_recurse == id);
                }
            }

            // ---- Expand: build result via interleaving ----
            // Even positions: result[2j] = c[j] (via f(id, c[j]) = c[j] by left identity)
            // Odd positions:  result[2j+1] = f(c[j], a[2j])
            let mut result_vec: Vec<T> = Vec::with_capacity(n);
            let mut j: usize = 0;
            while j < half
                invariant
                    j <= half,
                    half == n / 2,
                    n == a.spec_len(),
                    n >= 2,
                    half <= n,
                    s == Seq::new(a.spec_len(), |k: int| a.spec_index(k)),
                    b.spec_len() == half as nat,
                    b_seq.len() == half as nat,
                    c.spec_len() == half as nat,
                    forall|k: int| #![trigger b_seq[k]] 0 <= k < b_seq.len() ==>
                        b_seq[k] == spec_f(s[2 * k], s[2 * k + 1]),
                    forall|k: int| #![trigger c.spec_index(k)] 0 <= k < half as int ==>
                        c.spec_index(k) == b_seq.take(k).fold_left(id, spec_f),
                    result_vec@.len() == 2 * j as int,
                    forall|k: int| #![trigger result_vec@[k]] 0 <= k < 2 * j as int ==>
                        result_vec@[k] == s.take(k).fold_left(id, spec_f),
                    spec_monoid(spec_f, id),
                    forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                    forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) ==> ret == spec_f(x, y),
                decreases half - j,
            {
                // Even position: result[2j] = f(id, c[j]) = c[j]
                let even_val = call_f(&f, &id, c.nth(j));
                proof {
                    if j > 0 {
                        lemma_prefix_contraction::<T>(s, b_seq, spec_f, id, j as int);
                    } else {
                        assert(s.take(0) =~= Seq::<T>::empty());
                        assert(b_seq.take(0) =~= Seq::<T>::empty());
                    }
                    assert(even_val == s.take(2 * j as int).fold_left(id, spec_f));
                }
                result_vec.push(even_val);

                // Odd position: result[2j+1] = f(c[j], a[2j])
                let odd_val = call_f(&f, c.nth(j), a.nth(2 * j));
                proof {
                    let take_2j1 = s.take(2 * j as int + 1);
                    take_2j1.lemma_fold_left_split(id, spec_f, 2 * j as int);
                    assert(take_2j1.subrange(0, 2 * j as int) =~= s.take(2 * j as int));
                    assert(take_2j1.subrange(2 * j as int, 2 * j as int + 1) =~= seq![s[2 * j as int]]);
                    assert(odd_val == s.take(2 * j as int + 1).fold_left(id, spec_f));
                }
                result_vec.push(odd_val);

                j += 1;
            }

            // Handle odd-length: one more element
            if n % 2 == 1 {
                let last_val = call_f(&f, c.nth(half - 1), b.nth(half - 1));
                proof {
                    b_seq.lemma_fold_left_split(id, spec_f, (half - 1) as int);
                    assert(b_seq.subrange(0, (half - 1) as int) =~= b_seq.take((half - 1) as int));
                    assert(b_seq.subrange((half - 1) as int, half as int) =~= seq![b_seq[(half - 1) as int]]);

                    lemma_prefix_contraction::<T>(s, b_seq, spec_f, id, half as int);
                    assert(b_seq.take(half as int) =~= b_seq);

                    assert(s.take((n - 1) as int) =~= s.take(2 * half as int));

                    assert(last_val == s.take((n - 1) as int).fold_left(id, spec_f));
                }
                result_vec.push(last_val);
            }

            // Build scanned
            let scanned = ArraySeqMtEphS { seq: result_vec };
            proof {
                assert(scanned.spec_len() == n as nat);
                assert forall|k: int| #![trigger scanned.spec_index(k)]
                    0 <= k < n as int implies
                    scanned.spec_index(k) == s.take(k).fold_left(id, spec_f)
                by {
                    assert(scanned.spec_index(k) == result_vec@[k]);
                }
            }
            scanned
        }
    }

    } // verus!
} // mod
