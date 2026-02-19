// Copyright (C) 2025 Brian G. Milnes
// SPDX-License-Identifier: MIT

//! Shared specifications for Maximum Contiguous Subsequence Sum (Chapter 28).
//!
//! Provides the mathematical definitions from APAS Chapter 28:
//! - `spec_range_sum`: sum of elements in a contiguous range (Definition 28.4)
//! - `spec_mcss`: the MCSS value as `Option<int>` (`None` for empty = -infinity)
//! - `is_mcss_of`: predicate asserting a value is the MCSS of a sequence

//  Table of Contents
//	1. module
//	6. spec fns
//	7. proof fns/broadcast groups

//		1. module


pub mod MCSSSpec {
    use vstd::prelude::*;

    verus! {

    //		6. spec fns

    // ─── 1. spec definitions ───

    /// Sum of elements s[lo], s[lo+1], ..., s[hi-1] as unbounded int.
    /// Empty range (lo >= hi) sums to 0.
    pub open spec fn spec_range_sum(s: Seq<i32>, lo: int, hi: int) -> int
        decreases (if hi > lo { hi - lo } else { 0 }),
    {
        if lo >= hi {
            0
        } else {
            s[lo] as int + spec_range_sum(s, lo + 1, hi)
        }
    }

    /// True when `m` is the maximum contiguous subsequence sum of `s`.
    /// For non-empty `s`, `m` must:
    ///   (a) equal the sum of some non-empty contiguous range, and
    ///   (b) be >= every such sum.
    pub open spec fn is_mcss_of(s: Seq<i32>, m: int) -> bool {
        // (a) m is achieved
        (exists|lo: int, hi: int|
            #![trigger spec_range_sum(s, lo, hi)]
            0 <= lo < hi <= s.len() && spec_range_sum(s, lo, hi) == m
        )
        &&
        // (b) m is maximal
        (forall|lo: int, hi: int|
            #![trigger spec_range_sum(s, lo, hi)]
            0 <= lo < hi <= s.len() ==> spec_range_sum(s, lo, hi) <= m
        )
    }

    /// Full MCSS specification matching Definition 28.4.
    /// Returns None for empty sequence (= -infinity), Some(m) otherwise.
    pub open spec fn spec_mcss(s: Seq<i32>) -> Option<int> {
        if s.len() == 0 {
            None
        } else {
            Some(choose|m: int| is_mcss_of(s, m))
        }
    }

    /// All partial sums in [lo..hi) fit in i32.  Precondition for exec code.
    pub open spec fn sums_fit_i32(s: Seq<i32>) -> bool {
        forall|lo: int, hi: int|
            #![trigger spec_range_sum(s, lo, hi)]
            0 <= lo <= hi <= s.len() ==>
                i32::MIN as int <= spec_range_sum(s, lo, hi) <= i32::MAX as int
    }

    /// Prefix sum: sum of s[0..k).
    pub open spec fn spec_prefix_sum(s: Seq<i32>, k: int) -> int {
        spec_range_sum(s, 0, k)
    }

    /// Minimum prefix sum over indices 0..=k.
    pub open spec fn spec_min_prefix_sum(s: Seq<i32>, k: int) -> int
        decreases (if k >= 0 { k + 1 } else { 0 }),
    {
        if k < 0 {
            i32::MAX as int
        } else if k == 0 {
            spec_prefix_sum(s, 0)
        } else {
            let prev = spec_min_prefix_sum(s, k - 1);
            let curr = spec_prefix_sum(s, k);
            if curr < prev { curr } else { prev }
        }
    }

    /// Spec: is_max_suffix_sum(s, m) means m is the maximum suffix sum.
    /// i.e., m = max over lo in 0..n of range_sum(s, lo, n).
    pub open spec fn is_max_suffix_sum(s: Seq<i32>, m: int) -> bool {
        let n = s.len() as int;
        // achieved
        (exists|lo: int|
            #![trigger spec_range_sum(s, lo, n)]
            0 <= lo < n && spec_range_sum(s, lo, n) == m
        )
        &&
        // maximal
        (forall|lo: int|
            #![trigger spec_range_sum(s, lo, n)]
            0 <= lo < n ==> spec_range_sum(s, lo, n) <= m
        )
    }

    /// Spec: is_max_prefix_sum(s, m) means m is the maximum prefix sum.
    /// i.e., m = max over hi in 1..=n of range_sum(s, 0, hi).
    pub open spec fn is_max_prefix_sum(s: Seq<i32>, m: int) -> bool {
        let n = s.len() as int;
        // achieved
        (exists|hi: int|
            #![trigger spec_range_sum(s, 0, hi)]
            1 <= hi <= n && spec_range_sum(s, 0, hi) == m
        )
        &&
        // maximal
        (forall|hi: int|
            #![trigger spec_range_sum(s, 0, hi)]
            1 <= hi <= n ==> spec_range_sum(s, 0, hi) <= m
        )
    }


    //		7. proof fns/broadcast groups

    // ─── 2. lemmas ───

    /// Extending a range sum by one element.
    pub proof fn lemma_range_sum_snoc(s: Seq<i32>, lo: int, hi: int)
        requires
            0 <= lo,
            lo < hi,
            hi <= s.len(),
        ensures
            spec_range_sum(s, lo, hi) == spec_range_sum(s, lo, hi - 1) + s[hi - 1] as int,
        decreases hi - lo,
    {
        reveal_with_fuel(spec_range_sum, 2);
        if hi - lo > 1 {
            lemma_range_sum_snoc(s, lo + 1, hi);
        }
    }

    /// Range sum of a single element.
    pub proof fn lemma_range_sum_single(s: Seq<i32>, i: int)
        requires
            0 <= i < s.len(),
        ensures
            spec_range_sum(s, i, i + 1) == s[i] as int,
    {
        reveal_with_fuel(spec_range_sum, 2);
    }

    /// Range sum of empty range is 0.
    pub proof fn lemma_range_sum_empty(s: Seq<i32>, lo: int, hi: int)
        requires lo >= hi,
        ensures spec_range_sum(s, lo, hi) == 0,
    {
    }

    /// Splitting a range sum at a midpoint.
    pub proof fn lemma_range_sum_split(s: Seq<i32>, lo: int, mid: int, hi: int)
        requires
            0 <= lo <= mid <= hi <= s.len(),
        ensures
            spec_range_sum(s, lo, hi) == spec_range_sum(s, lo, mid) + spec_range_sum(s, mid, hi),
        decreases mid - lo,
    {
        reveal_with_fuel(spec_range_sum, 2);
        if lo < mid {
            lemma_range_sum_split(s, lo + 1, mid, hi);
        }
    }

    /// Range sum equals difference of prefix sums.
    pub proof fn lemma_range_sum_via_prefix(s: Seq<i32>, lo: int, hi: int)
        requires
            0 <= lo <= hi <= s.len(),
        ensures
            spec_range_sum(s, lo, hi) == spec_prefix_sum(s, hi) - spec_prefix_sum(s, lo),
    {
        lemma_range_sum_split(s, 0, lo, hi);
    }

    /// The min prefix sum is a lower bound on all prefix sums in [0..=k].
    pub proof fn lemma_min_prefix_sum_is_min(s: Seq<i32>, k: int, j: int)
        requires
            0 <= j <= k,
        ensures
            spec_min_prefix_sum(s, k) <= spec_prefix_sum(s, j),
        decreases k,
    {
        reveal_with_fuel(spec_min_prefix_sum, 2);
        if k > 0 && j < k {
            lemma_min_prefix_sum_is_min(s, k - 1, j);
        }
    }

    /// The min prefix sum is achieved by some index in [0..=k].
    pub proof fn lemma_min_prefix_sum_achieved(s: Seq<i32>, k: int)
        requires k >= 0,
        ensures
            exists|j: int| #![trigger spec_prefix_sum(s, j)]
                0 <= j <= k && spec_prefix_sum(s, j) == spec_min_prefix_sum(s, k),
        decreases k,
    {
        reveal_with_fuel(spec_min_prefix_sum, 2);
        if k == 0 {
            assert(spec_prefix_sum(s, 0) == spec_min_prefix_sum(s, 0));
        } else {
            lemma_min_prefix_sum_achieved(s, k - 1);
            let prev = spec_min_prefix_sum(s, k - 1);
            let curr = spec_prefix_sum(s, k);
            if curr < prev {
                assert(spec_prefix_sum(s, k) == spec_min_prefix_sum(s, k));
            }
        }
    }

    /// When `sub` is a contiguous slice of `full` starting at `offset`,
    /// the range sum over `sub[lo..hi)` equals the range sum over `full[offset+lo..offset+hi)`.
    pub proof fn lemma_range_sum_subseq(full: Seq<i32>, sub: Seq<i32>, offset: int, lo: int, hi: int)
        requires
            0 <= offset,
            0 <= lo <= hi <= sub.len(),
            offset + hi <= full.len(),
            forall|i: int| #![trigger sub[i]] 0 <= i < sub.len() ==> sub[i] == full[offset + i],
        ensures
            spec_range_sum(sub, lo, hi) == spec_range_sum(full, offset + lo, offset + hi),
        decreases hi - lo,
    {
        reveal_with_fuel(spec_range_sum, 2);
        if lo < hi {
            lemma_range_sum_subseq(full, sub, offset, lo + 1, hi);
        }
    }

    /// MCSS of the whole is the max of: MCSS-left, MCSS-right, max-crossing.
    /// A crossing subsequence a[lo..hi) with lo < mid < hi decomposes as
    /// range_sum(a, lo, mid) + range_sum(a, mid, hi).
    pub proof fn lemma_crossing_decompose(s: Seq<i32>, lo: int, mid: int, hi: int)
        requires
            0 <= lo < mid < hi <= s.len(),
        ensures
            spec_range_sum(s, lo, hi) == spec_range_sum(s, lo, mid) + spec_range_sum(s, mid, hi),
    {
        lemma_range_sum_split(s, lo, mid, hi);
    }

    /// sums_fit_i32 is monotone for subsequences.
    pub proof fn lemma_sums_fit_subseq(full: Seq<i32>, sub: Seq<i32>, offset: int)
        requires
            sums_fit_i32(full),
            0 <= offset,
            offset + sub.len() <= full.len(),
            forall|i: int| #![trigger sub[i]] 0 <= i < sub.len() ==> sub[i] == full[offset + i],
        ensures
            sums_fit_i32(sub),
    {
        assert forall|lo: int, hi: int|
            #![trigger spec_range_sum(sub, lo, hi)]
            0 <= lo <= hi <= sub.len()
        implies
            i32::MIN as int <= spec_range_sum(sub, lo, hi) <= i32::MAX as int
        by {
            lemma_range_sum_subseq(full, sub, offset, lo, hi);
        };
    }

    } // verus!
}
