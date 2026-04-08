//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Parallel scan using contraction technique (Chapter 27, Algorithm 27.3).
//! Uses the help-first scheduler for fork-join parallelism.
//! Verusified.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 7. proof fns/broadcast groups
//	Section 8. traits
//	Section 9. impls

//		Section 1. module

pub mod ScanContractMtEph {


    //		Section 2. imports

    use std::sync::Arc;
    use vstd::prelude::*;

    verus! 
{


    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Chap27::ReduceContractMtEph::ReduceContractMtEph::contract_parallel;
    use crate::vstdplus::smart_ptrs::smart_ptrs::call_f;
    use crate::Concurrency::Concurrency::StTInMtT;
    use crate::vstdplus::monoid::monoid::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;

    //		Section 3. broadcast use


    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
        crate::vstdplus::feq::feq::group_feq_axioms,
        vstd::seq_lib::group_seq_properties,
        vstd::seq_lib::group_to_multiset_ensures,
    };

    //		Section 7. proof fns/broadcast groups


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
    }

    /// Helper: fold_left of a 1-element sequence equals f(id, a) = a.
    proof fn lemma_fold_left_singleton<T>(a: T, f: spec_fn(T, T) -> T, id: T)
        requires spec_monoid(f, id),
        ensures seq![a].fold_left(id, f) == a,
    {
        reveal_with_fuel(Seq::fold_left, 2);
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
            lemma_fold_left_pair::<T>(s[0], s[1], f, id);
            // Veracity: NEEDED assert
            assert(b =~= seq![f(s[0], s[1])]);
            lemma_fold_left_singleton::<T>(f(s[0], s[1]), f, id);
        } else {
            let s_tail = s.subrange(2, n as int);
            let b_tail = b.subrange(1, b.len() as int);

            s.lemma_fold_left_split(id, f, 2);
            let s_head = s.subrange(0, 2);
            // Veracity: NEEDED assert
            assert(s_head =~= seq![s[0], s[1]]);

            lemma_fold_left_pair::<T>(s[0], s[1], f, id);

            lemma_fold_left_monoid::<T>(s_tail, b[0], f, id);
            let s_tail_result = s_tail.fold_left(id, f);

            // Veracity: NEEDED assert
            assert(b_tail =~= Seq::new(
                (s_tail.len() / 2) as nat,
                |i: int| f(s_tail[2 * i], s_tail[2 * i + 1]),
            ));

            lemma_contraction_even::<T>(s_tail, f, id);
            let b_tail_result = b_tail.fold_left(id, f);

            lemma_fold_left_monoid::<T>(b_tail, b[0], f, id);

            b.lemma_fold_left_split(id, f, 1);
            lemma_fold_left_singleton::<T>(b[0], f, id);
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
        // Veracity: NEEDED assert
        assert(contracted =~= b.take(k));
    }

    /// Expand even step: b_seq.take(j).fold_left(id, f) == s.take(2j).fold_left(id, f).
    proof fn lemma_expand_even<T>(s: Seq<T>, b_seq: Seq<T>, f: spec_fn(T, T) -> T, id: T, j: int)
        requires
            spec_monoid(f, id),
            j >= 0,
            2 * j <= s.len(),
            b_seq.len() >= j,
            forall|i: int| #![trigger b_seq[i]] 0 <= i < b_seq.len() ==> b_seq[i] == f(s[2 * i], s[2 * i + 1]),
        ensures
            b_seq.take(j).fold_left(id, f) == s.take(2 * j).fold_left(id, f),
    {
        if j > 0 {
            lemma_prefix_contraction::<T>(s, b_seq, f, id, j);
        } else {
            reveal(Seq::fold_left);
        }
    }

    /// Expand odd step: f(s.take(2j).fold_left(id, f), s[2j]) == s.take(2j+1).fold_left(id, f).
    proof fn lemma_expand_odd<T>(s: Seq<T>, f: spec_fn(T, T) -> T, id: T, j: int)
        requires
            spec_monoid(f, id),
            j >= 0,
            2 * j + 1 <= s.len(),
        ensures
            f(s.take(2 * j).fold_left(id, f), s[2 * j]) == s.take(2 * j + 1).fold_left(id, f),
    {
        let take_2j1 = s.take(2 * j + 1);
        take_2j1.lemma_fold_left_split(id, f, 2 * j);
        // Veracity: NEEDED assert
        assert(take_2j1.subrange(0, 2 * j) =~= s.take(2 * j));
        reveal(Seq::fold_left);
    }

    /// Expand odd-length tail: last element when n is odd.
    proof fn lemma_expand_odd_tail<T>(
        s: Seq<T>, b_seq: Seq<T>, f: spec_fn(T, T) -> T, id: T, half: int,
    )
        requires
            spec_monoid(f, id),
            half >= 1,
            s.len() == 2 * half + 1,
            b_seq.len() == half,
            forall|i: int| #![trigger b_seq[i]] 0 <= i < b_seq.len() ==> b_seq[i] == f(s[2 * i], s[2 * i + 1]),
        ensures
            f(b_seq.take(half - 1).fold_left(id, f), b_seq[half - 1])
                == s.take(2 * half).fold_left(id, f),
    {
        b_seq.lemma_fold_left_split(id, f, half - 1);
        reveal(Seq::fold_left);

        lemma_prefix_contraction::<T>(s, b_seq, f, id, half);
    }

    //		Section 8. traits


    pub trait ScanContractMtEphTrait<T: StTInMtT> {
        /// Exclusive scan using parallel contraction: contract→solve→expand.
        /// Returns prefixes where result[i] = fold_left(input[0..i], id, spec_f).
        /// - Alg Analysis: APAS (Ch27 Alg 27.3): Work O(n), Span O(lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — contraction parallel via one-level join; expansion sequential.
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

        /// Expand phase: interleave contracted scan results into full scan output.
        /// - Alg Analysis: APAS (Ch27 Alg 27.3): Work O(n), Span O(1) — parallel tabulate (expansion step).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — sequential loop, no parallelism.
        fn expand_scan_parallel<F: Fn(&T, &T) -> T + Send + Sync + 'static>(
            a: &ArraySeqMtEphS<T>,
            b: &ArraySeqMtEphS<T>,
            c: &ArraySeqMtEphS<T>,
            f: &Arc<F>,
            Ghost(spec_f): Ghost<spec_fn(T, T) -> T>,
            id: &T,
            n: usize,
            half: usize,
        ) -> (prefixes: Vec<T>)
            requires
                n == a.spec_len(),
                n >= 2,
                half == n / 2,
                half <= n,
                b.spec_len() == half as nat,
                c.spec_len() == half as nat,
                spec_monoid(spec_f, *id),
                forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) ==> ret == spec_f(x, y),
                forall|k: int| #![trigger b.spec_index(k)] 0 <= k < half as int ==> {
                    &&& 2 * k + 1 < a.spec_len()
                    &&& b.spec_index(k) == spec_f(a.spec_index(2 * k), a.spec_index(2 * k + 1))
                },
                forall|k: int| #![trigger c.spec_index(k)] 0 <= k < half as int ==>
                    c.spec_index(k) == Seq::new(b.spec_len(), |j: int| b.spec_index(j)).take(k).fold_left(*id, spec_f),
            ensures ({
                let s = Seq::new(a.spec_len(), |i: int| a.spec_index(i));
                &&& prefixes@.len() == n
                &&& forall|k: int| #![trigger prefixes@[k]] 0 <= k < n as int ==>
                        prefixes@[k] == s.take(k).fold_left(*id, spec_f)
            });
    }

    //		Section 9. impls


    impl<T: StTInMtT + Clone + 'static> ScanContractMtEphTrait<T> for ArraySeqMtEphS<T> {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(lg^2 n) — recursive contract/expand; Mt parallel contract via join + parallel expand via tabulate.
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
                // Veracity: NEEDED proof block
                proof {
                }
                return ArraySeqMtEphS { seq: Vec::new() };
            }

            // Base case: single element — result is [id]
            if n == 1 {
                let mut v: Vec<T> = Vec::with_capacity(1);
                v.push(id);
                // Veracity: NEEDED proof block
                proof {
                }
                return ArraySeqMtEphS { seq: v };
            }

            // f(id, id) == id by left identity, producing an owned T without clone.
            let id_for_recurse = call_f(&f, &id, &id);
            // Veracity: NEEDED proof block
            proof {
            }

            // Contract: b[i] = f(a[2i], a[2i+1]) — parallel via join
            let half = n / 2;
            let b = contract_parallel(a, &f, Ghost(spec_f), half);

            let ghost b_seq = Seq::new(b.spec_len(), |i: int| b.spec_index(i));
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                assert forall|j: int| #![trigger b_seq[j]] 0 <= j < half as int implies {
                    &&& 2 * j + 1 < s.len()
                    &&& b_seq[j] == spec_f(s[2 * j], s[2 * j + 1])
                } by {
                }
            }

            // Solve: recursively scan contracted sequence
            let c = Self::scan_contract_parallel(&b, Arc::clone(&f), Ghost(spec_f), id_for_recurse);

            // Veracity: NEEDED proof block
            proof {
                let ghost b_view = Seq::new(b.spec_len(), |j: int| b.spec_index(j));
                // Veracity: NEEDED assert
                assert forall|k: int| #![trigger c.spec_index(k)] 0 <= k < half as int implies
                    c.spec_index(k) == b_seq.take(k).fold_left(id, spec_f)
                by {
                }
            }

            // Expand
            let result_vec = Self::expand_scan_parallel(a, &b, &c, &f, Ghost(spec_f), &id, n, half);
            let scanned = ArraySeqMtEphS { seq: result_vec };
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                assert forall|k: int| #![trigger scanned.spec_index(k)]
                    0 <= k < n as int implies
                    scanned.spec_index(k) == s.take(k).fold_left(id, spec_f)
                by {
                }
            }
            scanned
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(lg n) — parallel expand via tabulate over n elements.
        fn expand_scan_parallel<F: Fn(&T, &T) -> T + Send + Sync + 'static>(
            a: &ArraySeqMtEphS<T>,
            b: &ArraySeqMtEphS<T>,
            c: &ArraySeqMtEphS<T>,
            f: &Arc<F>,
            Ghost(spec_f): Ghost<spec_fn(T, T) -> T>,
            id: &T,
            n: usize,
            half: usize,
        ) -> (prefixes: Vec<T>)
        {
            let ghost s = Seq::new(a.spec_len(), |i: int| a.spec_index(i));
            let ghost b_seq = Seq::new(b.spec_len(), |i: int| b.spec_index(i));

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
                    b_seq == Seq::new(b.spec_len(), |i: int| b.spec_index(i)),
                    b_seq.len() == half as nat,
                    c.spec_len() == half as nat,
                    forall|k: int| #![trigger b_seq[k]] 0 <= k < b_seq.len() ==>
                        b_seq[k] == spec_f(s[2 * k], s[2 * k + 1]),
                    forall|k: int| #![trigger c.spec_index(k)] 0 <= k < half as int ==>
                        c.spec_index(k) == b_seq.take(k).fold_left(*id, spec_f),
                    result_vec@.len() == 2 * j as int,
                    forall|k: int| #![trigger result_vec@[k]] 0 <= k < 2 * j as int ==>
                        result_vec@[k] == s.take(k).fold_left(*id, spec_f),
                    spec_monoid(spec_f, *id),
                    forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                    forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) ==> ret == spec_f(x, y),
                decreases half - j,
            {
                let even_val = call_f(f, id, c.nth(j));
                // Veracity: NEEDED proof block
                proof {
                    lemma_expand_even::<T>(s, b_seq, spec_f, *id, j as int);
                }
                result_vec.push(even_val);

                let odd_val = call_f(f, c.nth(j), a.nth(2 * j));
                // Veracity: NEEDED proof block
                proof {
                    lemma_expand_odd::<T>(s, spec_f, *id, j as int);
                }
                result_vec.push(odd_val);

                j += 1;
            }

            if n % 2 == 1 {
                let last_val = call_f(f, c.nth(half - 1), b.nth(half - 1));
                // Veracity: NEEDED proof block
                proof {
                    lemma_expand_odd_tail::<T>(s, b_seq, spec_f, *id, half as int);
                }
                result_vec.push(last_val);
            }

            result_vec
        }
    }

    } // verus!
} // mod
