// Copyright (C) 2025 Brian G. Milnes
// SPDX-License-Identifier: MIT
//! Maximum Contiguous Subsequence Sum — Parallel Optimal (Chapter 28, Algorithm 28.16).
//!
//! Verified sequential impl under verus_keep_ghost; parallel impl at runtime.

pub mod MaxContigSubSumOptMtEph {
    use vstd::prelude::*;

    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Chap28::MCSSSpec::MCSSSpec::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap28::MaxContigSubSumOptStEph::MaxContigSubSumOptStEph::lemma_prefix_opt_is_mcss;

    pub type T = ArraySeqMtEphS<i32>;

    verus! {

    pub trait MaxContigSubSumOptMtTrait {
        /// Compute MCSS using parallel optimal scan-based algorithm (Algorithm 28.16).
        /// Returns None for empty sequence (representing -infinity).
        /// - APAS: Work Θ(n), Span Θ(log n)
        /// - Claude-Opus-4.6 (verified): Work Θ(n), Span Θ(n)
        fn max_contig_sub_sum_opt_mt(a: &ArraySeqMtEphS<i32>) -> (result: Option<i32>)
            requires
                sums_fit_i32(a.seq@),
                a.seq@.len() < usize::MAX,
            ensures
                a.seq@.len() == 0 ==> result.is_none(),
                a.seq@.len() > 0 ==> result.is_some(),
                result.is_some() ==> is_mcss_of(a.seq@, result.unwrap() as int);
    }

    #[cfg(verus_keep_ghost)]
    impl MaxContigSubSumOptMtTrait for ArraySeqMtEphS<i32> {
        fn max_contig_sub_sum_opt_mt(a: &ArraySeqMtEphS<i32>) -> (result: Option<i32>) {
            let n = a.length();
            if n == 0 { return None; }

            // Phase 1: prefix sums.
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
                        0 <= k <= i ==> pv@[k] as int == spec_prefix_sum(a.seq@, k),
                decreases n - i,
            {
                proof {
                    lemma_range_sum_snoc(a.seq@, 0, (i + 1) as int);
                    assert(spec_range_sum(a.seq@, 0, (i + 1) as int) ==
                           spec_range_sum(a.seq@, 0, i as int) + a.seq@[i as int] as int);
                }
                rsum = rsum + *a.nth(i);
                pv.push(rsum);
                i = i + 1;
            }
            let ap = ArraySeqMtEphS::from_vec(pv);

            // Phase 2: min prefix sums.
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
                        0 <= k <= n ==> ap.spec_index(k) as int == spec_prefix_sum(a.seq@, k),
                    mv@.len() == j as int,
                    rmin as int == spec_min_prefix_sum(a.seq@, (j - 1) as int),
                    forall|k: int| #![trigger mv@[k]]
                        0 <= k < j ==> mv@[k] as int == spec_min_prefix_sum(a.seq@, k),
                decreases n + 1 - j,
            {
                let cur = *ap.nth(j);
                if cur < rmin { rmin = cur; }
                proof {
                    reveal_with_fuel(spec_min_prefix_sum, 2);
                    assert(rmin as int == spec_min_prefix_sum(a.seq@, j as int));
                }
                mv.push(rmin);
                j = j + 1;
            }
            let mp = ArraySeqMtEphS::from_vec(mv);

            // Phase 3: max of (ap[i] - mp[i-1]) for i = 1..=n.
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
                        0 <= k <= n ==> ap.spec_index(k) as int == spec_prefix_sum(a.seq@, k),
                    forall|k: int| #![trigger mp.spec_index(k)]
                        0 <= k <= n ==> mp.spec_index(k) as int == spec_min_prefix_sum(a.seq@, k),
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
            proof { lemma_prefix_opt_is_mcss(a.seq@, max_sum.unwrap() as int, n as int); }
            max_sum
        }
    }

    } // verus!

    #[cfg(not(verus_keep_ghost))]
    impl MaxContigSubSumOptMtTrait for ArraySeqMtEphS<i32> {
        fn max_contig_sub_sum_opt_mt(a: &ArraySeqMtEphS<i32>) -> Option<i32> {
            let n = a.length();
            if n == 0 { return None; }

            let (inclusive_prefixes, _total) =
                <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::scan(a, &|x, y| x + y, 0);
            let zero_seq = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::singleton(0);
            let all_prefixes =
                <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::append(&zero_seq, &inclusive_prefixes);
            let (min_prefixes, _) =
                <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::scan(&all_prefixes, &|x, y| (*x).min(*y), i32::MAX);
            let differences = <ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::tabulate(
                &|i| *all_prefixes.nth(i + 1) - *min_prefixes.nth(i),
                n,
            );
            Some(<ArraySeqMtEphS<i32> as ArraySeqMtEphTrait<i32>>::reduce(
                &differences, &|x: &i32, y: &i32| (*x).max(*y), i32::MIN,
            ))
        }
    }
}
