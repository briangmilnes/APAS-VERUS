// Copyright (C) 2025 Brian G. Milnes
// SPDX-License-Identifier: MIT
//! Maximum Contiguous Subsequence Sum — Work Optimal (Chapter 28, Algorithm 28.16).
//!
//! ## Table of Contents
//! 1. imports
//! 2. spec definitions
//! 3. exec functions

pub mod MaxContigSubSumOptStEph {
    use vstd::prelude::*;

    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap28::MCSSSpec::MCSSSpec::*;

    verus! {

// Veracity: added broadcast group
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
    vstd::seq_lib::group_to_multiset_ensures,
};

    // ─── 3. exec functions ───

    /// Helper: given that m = max over hi of (prefix[hi] - min_prefix[hi-1]),
    /// prove is_mcss_of(s, m).
    pub proof fn lemma_prefix_opt_is_mcss(s: Seq<i32>, m: int, n: int)
        requires
            n == s.len(),
            n > 0,
            sums_fit_i32(s),
            // m is achieved by some hi
            exists|hi: int|
                #![trigger spec_prefix_sum(s, hi)]
                1 <= hi <= n &&
                m == spec_prefix_sum(s, hi) - spec_min_prefix_sum(s, hi - 1),
            // m is maximal over all hi
            forall|hi: int|
                #![trigger spec_prefix_sum(s, hi)]
                1 <= hi <= n ==>
                m >= spec_prefix_sum(s, hi) - spec_min_prefix_sum(s, hi - 1),
        ensures
            is_mcss_of(s, m),
    {
        // Part (a): achieved.
        let hi_w: int = choose|hi: int|
            #![trigger spec_prefix_sum(s, hi)]
            1 <= hi <= n &&
            m == spec_prefix_sum(s, hi) - spec_min_prefix_sum(s, hi - 1);

        lemma_min_prefix_sum_achieved(s, hi_w - 1);
        let lo_w: int = choose|jj: int|
            #![trigger spec_prefix_sum(s, jj)]
            0 <= jj <= hi_w - 1 && spec_prefix_sum(s, jj) == spec_min_prefix_sum(s, hi_w - 1);

        lemma_range_sum_via_prefix(s, lo_w, hi_w);
        assert(spec_range_sum(s, lo_w, hi_w) == m);

        // Part (b): maximal.
        assert forall|lo: int, hi: int|
            #![trigger spec_range_sum(s, lo, hi)]
            0 <= lo < hi <= s.len()
        implies
            spec_range_sum(s, lo, hi) <= m
        by {
            lemma_range_sum_via_prefix(s, lo, hi);
            lemma_min_prefix_sum_is_min(s, hi - 1, lo);
        };
    }

    pub trait MaxContigSubSumOptTrait {
        /// Compute MCSS using optimal prefix-sum algorithm (Algorithm 28.16).
        /// Returns None for empty sequence (representing -infinity).
        /// - APAS: Work Θ(n), Span Θ(log n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — sequential
        fn max_contig_sub_sum_opt(a: &ArraySeqStEphS<i32>) -> (mcss: Option<i32>)
            requires
                sums_fit_i32(a.seq@),
                a.seq@.len() < usize::MAX,
            ensures
                a.seq@.len() == 0 ==> mcss.is_none(),
                a.seq@.len() > 0 ==> mcss.is_some(),
                mcss.is_some() ==> is_mcss_of(a.seq@, mcss.unwrap() as int);
    }

    impl MaxContigSubSumOptTrait for ArraySeqStEphS<i32> {
        fn max_contig_sub_sum_opt(a: &ArraySeqStEphS<i32>) -> (mcss: Option<i32>) {
            let n = a.length();

            if n == 0 {
                return None;
            }

            // Phase 1: Build prefix sums.
            // all_prefixes[k] = sum(a[0..k)) for k = 0..=n.
            let mut pv: Vec<i32> = Vec::with_capacity(n + 1);
            pv.push(0);
            let mut rsum: i32 = 0;
            let mut i: usize = 0;
            while i < n
                invariant
                    0 <= i <= n,
                    n as int == a.seq@.len(),
                    sums_fit_i32(a.seq@),
                    pv@.len() == i as int + 1,
                    rsum as int == spec_prefix_sum(a.seq@, i as int),
                    forall|k: int| #![trigger pv@[k]]
                        0 <= k <= i ==>
                            pv@[k] as int == spec_prefix_sum(a.seq@, k),
                decreases n - i,
            {
                proof {
                    lemma_range_sum_snoc(a.seq@, 0, (i + 1) as int);
                    // spec_prefix_sum(a.seq@, i+1) = spec_prefix_sum(a.seq@, i) + a.seq@[i]
                    // So rsum + a.seq@[i] fits in i32 (from sums_fit_i32).
                    assert(spec_range_sum(a.seq@, 0, (i + 1) as int) ==
                           spec_range_sum(a.seq@, 0, i as int) + a.seq@[i as int] as int);
                }
                rsum = rsum + *a.nth(i);
                pv.push(rsum);
                i = i + 1;
            }
            let ap = ArraySeqStEphS::from_vec(pv);

            // Phase 2: Build min prefix sums.
            // mp[k] = min(ap[0..=k]) for k = 0..=n.
            let mut mv: Vec<i32> = Vec::with_capacity(n + 1);
            let mut rmin: i32 = *ap.nth(0);
            mv.push(rmin);
            let mut j: usize = 1;
            while j <= n
                invariant
                    1 <= j <= n + 1,
                    n as int == a.seq@.len(),
                    n < usize::MAX,
                    sums_fit_i32(a.seq@),
                    ap.spec_len() == n as int + 1,
                    forall|k: int| #![trigger ap.spec_index(k)]
                        0 <= k <= n ==>
                            ap.spec_index(k) as int == spec_prefix_sum(a.seq@, k),
                    mv@.len() == j as int,
                    rmin as int == spec_min_prefix_sum(a.seq@, (j - 1) as int),
                    forall|k: int| #![trigger mv@[k]]
                        0 <= k < j ==>
                            mv@[k] as int == spec_min_prefix_sum(a.seq@, k),
                decreases n + 1 - j,
            {
                let cur = *ap.nth(j);
                if cur < rmin {
                    rmin = cur;
                }
                proof {
                    reveal_with_fuel(spec_min_prefix_sum, 2);
                    assert(rmin as int == spec_min_prefix_sum(a.seq@, j as int));
                }
                mv.push(rmin);
                j = j + 1;
            }
            let mp = ArraySeqStEphS::from_vec(mv);

            // Phase 3: Find max of (ap[i] - mp[i-1]) for i = 1..=n.
            let mut max_sum: Option<i32> = None;
            let mut idx: usize = 1;
            while idx <= n
                invariant
                    1 <= idx <= n + 1,
                    n as int == a.seq@.len(),
                    n > 0,
                    n < usize::MAX,
                    sums_fit_i32(a.seq@),
                    ap.spec_len() == n as int + 1,
                    mp.spec_len() == n as int + 1,
                    forall|k: int| #![trigger ap.spec_index(k)]
                        0 <= k <= n ==>
                            ap.spec_index(k) as int == spec_prefix_sum(a.seq@, k),
                    forall|k: int| #![trigger mp.spec_index(k)]
                        0 <= k <= n ==>
                            mp.spec_index(k) as int == spec_min_prefix_sum(a.seq@, k),
                    idx == 1 ==> max_sum.is_none(),
                    idx > 1 ==> max_sum.is_some(),
                    idx > 1 ==> (
                        (exists|hi: int|
                            #![trigger spec_prefix_sum(a.seq@, hi)]
                            1 <= hi < idx &&
                            max_sum.unwrap() as int == spec_prefix_sum(a.seq@, hi) - spec_min_prefix_sum(a.seq@, hi - 1)
                        ) &&
                        (forall|hi: int|
                            #![trigger spec_prefix_sum(a.seq@, hi)]
                            1 <= hi < idx ==>
                            max_sum.unwrap() as int >= spec_prefix_sum(a.seq@, hi) - spec_min_prefix_sum(a.seq@, hi - 1)
                        )
                    ),
                decreases n + 1 - idx,
            {
                let p_hi = *ap.nth(idx);
                let m_lo = *mp.nth(idx - 1);
                proof {
                    // Show the subtraction fits in i32.
                    // p_hi = prefix(idx), m_lo = min_prefix(idx-1).
                    // min_prefix is achieved by some lo_j, so m_lo = prefix(lo_j).
                    // p_hi - m_lo = prefix(idx) - prefix(lo_j) = range_sum(lo_j, idx).
                    // By sums_fit_i32, this fits.
                    lemma_min_prefix_sum_achieved(a.seq@, (idx - 1) as int);
                    let lo_j: int = choose|jj: int|
                        #![trigger spec_prefix_sum(a.seq@, jj)]
                        0 <= jj <= idx - 1 && spec_prefix_sum(a.seq@, jj) == spec_min_prefix_sum(a.seq@, (idx - 1) as int);
                    lemma_range_sum_via_prefix(a.seq@, lo_j, idx as int);
                }
                let ending_max: i32 = p_hi - m_lo;
                max_sum = match max_sum {
                    None => Some(ending_max),
                    Some(cur) => Some(if cur >= ending_max { cur } else { ending_max }),
                };

                idx = idx + 1;
            }

            assert(max_sum.is_some());

            proof {
                lemma_prefix_opt_is_mcss(a.seq@, max_sum.unwrap() as int, n as int);
            }

            max_sum
        }
    }

    } // verus!
}
