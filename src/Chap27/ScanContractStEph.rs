//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Sequential scan using contraction technique (Chapter 27, Algorithm 27.3).
//! Verusified.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	7. proof fns
//	8. traits
//	9. impls

//		1. module

pub mod ScanContractStEph {

    use vstd::prelude::*;

    verus! {

    //		2. imports

    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap27::ReduceContractStEph::ReduceContractStEph::lemma_contraction_even;
    use crate::vstdplus::monoid::monoid::*;
    use crate::Types::Types::*;

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

    /// Prefix contraction lemma: fold_left of an even-length prefix s.take(2k)
    /// equals fold_left of the first k elements of the contracted sequence b.
    /// - APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: N/A — proof function, no runtime cost.
    pub proof fn lemma_prefix_contraction<T>(s: Seq<T>, b: Seq<T>, f: spec_fn(T, T) -> T, id: T, k: int)
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

    pub trait ScanContractStEphTrait<T: StT> {
        /// Exclusive scan using contraction: contract→solve→expand.
        /// Returns prefixes where result[i] = fold_left(input[0..i], id, spec_f).
        /// - APAS: Work Θ(n), Span Θ(log n) — Algorithm 27.3.
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — sequential implementation, no parallelism.
        fn scan_contract<F: Fn(&T, &T) -> T>(
            a: &ArraySeqStEphS<T>,
            f: &F,
            Ghost(spec_f): Ghost<spec_fn(T, T) -> T>,
            id: T,
        ) -> (scanned: ArraySeqStEphS<T>)
            requires
                a.spec_len() <= usize::MAX,
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

    impl<T: StT + Clone> ScanContractStEphTrait<T> for ArraySeqStEphS<T> {
        fn scan_contract<F: Fn(&T, &T) -> T>(
            a: &ArraySeqStEphS<T>,
            f: &F,
            Ghost(spec_f): Ghost<spec_fn(T, T) -> T>,
            id: T,
        ) -> (scanned: ArraySeqStEphS<T>)
            decreases a.spec_len(),
        {
            let n = a.length();
            let ghost s = Seq::new(a.spec_len(), |i: int| a.spec_index(i));

            // Base case: empty
            if n == 0 {
                proof {
                    assert(s =~= Seq::<T>::empty());
                }
                return ArraySeqStEphS { seq: Vec::new() };
            }

            // Base case: single element — result is [id]
            // result[0] = fold_left(s.take(0), id, spec_f) = fold_left(empty, id, spec_f) = id
            if n == 1 {
                let mut v: Vec<T> = Vec::with_capacity(1);
                v.push(id);
                proof {
                    reveal(Seq::fold_left);
                    assert(s.take(0) =~= Seq::<T>::empty());
                }
                return ArraySeqStEphS { seq: v };
            }

            // Create a spec-equal copy of id via f for the recursive call.
            // f(id, id) == id by left identity, producing an owned T without clone.
            let id_for_recurse = f(&id, &id);
            proof {
                assert(id_for_recurse == id);
            }

            // ---- Contract: b[i] = f(a[2i], a[2i+1]) ----
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
                assert forall|j: int| #![trigger b_seq[j]] 0 <= j < half as int implies {
                    &&& 2 * j + 1 < s.len()
                    &&& b_seq[j] == spec_f(s[2 * j], s[2 * j + 1])
                } by {
                    assert(b_seq[j] == b_vec@[j]);
                }
            }

            // ---- Solve: recursively scan contracted sequence ----
            let c = Self::scan_contract(&b, f, Ghost(spec_f), id_for_recurse);

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
                let even_val = f(&id, c.nth(j));
                proof {
                    // even_val == spec_f(id, c[j]) == c[j] by left identity
                    // c[j] == b_seq.take(j).fold_left(id, spec_f)
                    // For j == 0: both s.take(0) and b_seq.take(0) are empty, fold_left = id
                    // For j >= 1: by prefix contraction
                    if j > 0 {
                        lemma_prefix_contraction::<T>(s, b_seq, spec_f, id, j as int);
                    } else {
                        reveal(Seq::fold_left);
                        assert(s.take(0) =~= Seq::<T>::empty());
                        assert(b_seq.take(0) =~= Seq::<T>::empty());
                    }
                    assert(even_val == s.take(2 * j as int).fold_left(id, spec_f));
                }
                result_vec.push(even_val);

                // Odd position: result[2j+1] = f(c[j], a[2j])
                let odd_val = f(c.nth(j), a.nth(2 * j));
                proof {
                    // odd_val == spec_f(c[j], s[2j])
                    //         == spec_f(s.take(2j).fold_left(id, spec_f), s[2j])
                    // Split s.take(2j+1) at position 2j:
                    let take_2j1 = s.take(2 * j as int + 1);
                    take_2j1.lemma_fold_left_split(id, spec_f, 2 * j as int);
                    assert(take_2j1.subrange(0, 2 * j as int) =~= s.take(2 * j as int));
                    assert(take_2j1.subrange(2 * j as int, 2 * j as int + 1) =~= seq![s[2 * j as int]]);
                    reveal(Seq::fold_left);
                    assert(odd_val == s.take(2 * j as int + 1).fold_left(id, spec_f));
                }
                result_vec.push(odd_val);

                j += 1;
            }

            // Handle odd-length: one more element
            if n % 2 == 1 {
                // result[n-1] = fold_left(s.take(n-1), id, f) = fold_left(s.take(2*half), id, f)
                // = fold_left(b_seq, id, f) = f(c[half-1], b[half-1])
                let last_val = f(c.nth(half - 1), b.nth(half - 1));
                proof {
                    // Step 1: b_seq.fold_left(id, f) == f(c[half-1], b[half-1])
                    b_seq.lemma_fold_left_split(id, spec_f, (half - 1) as int);
                    assert(b_seq.subrange(0, (half - 1) as int) =~= b_seq.take((half - 1) as int));
                    assert(b_seq.subrange((half - 1) as int, half as int) =~= seq![b_seq[(half - 1) as int]]);
                    reveal(Seq::fold_left);

                    // Step 2: s.take(2*half).fold_left(id, f) == b_seq.fold_left(id, f)
                    lemma_prefix_contraction::<T>(s, b_seq, spec_f, id, half as int);
                    assert(b_seq.take(half as int) =~= b_seq);

                    // Step 3: s.take(n-1) =~= s.take(2*half)
                    assert(s.take((n - 1) as int) =~= s.take(2 * half as int));

                    assert(last_val == s.take((n - 1) as int).fold_left(id, spec_f));
                }
                result_vec.push(last_val);
            }

            // Build scanned
            let scanned = ArraySeqStEphS { seq: result_vec };
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
