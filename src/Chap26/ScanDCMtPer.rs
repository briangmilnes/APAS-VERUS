//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Divide-and-conquer scan - parallel implementation (Chapter 26, Section 3).
//! Note: Unconditionally parallel - no thresholding per APAS rules.
//! Verusified.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	8. traits
//	9. impls

//		1. module

pub mod ScanDCMtPer {

    use std::sync::Arc;
    use std::thread;
    use vstd::prelude::*;

    verus! {

    //		2. imports

    use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap26::ScanDCStPer::ScanDCStPer::*;
    use crate::vstdplus::monoid::monoid::*;
    use crate::Types::Types::*;

    //		3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
    };

    //		8. traits

    pub trait ScanDCMtTrait {
        /// Exclusive prefix sums via parallel divide-and-conquer scan.
        /// Returns (prefixes, total) where prefixes[i] = sum(a[0], ..., a[i-1]).
        /// - APAS: Work Θ(n lg n), Span Θ(lg n) — Algorithm 26.5 with parallel recursive calls.
        /// - Claude-Opus-4.6: Work Θ(n lg n), Span depends on tabulate/append — Θ(lg n) if parallel O(1), Θ(n) if sequential.
        fn prefix_sums_dc_parallel(a: &ArraySeqMtPerS<N>) -> (result: (ArraySeqMtPerS<N>, N))
            requires a.spec_len() <= usize::MAX,
            ensures
                spec_scan_post(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)),
                    |x: N, y: N| (x + y) as N, 0,
                    Seq::new(result.0.spec_len(), |i: int| result.0.spec_index(i)),
                    result.1);
    }

    //		9. impls

    impl ScanDCMtTrait for ArraySeqMtPerS<N> {
        #[verifier::external_body]
        fn prefix_sums_dc_parallel(a: &ArraySeqMtPerS<N>) -> (result: (ArraySeqMtPerS<N>, N)) {
            prefix_sums_dc_inner(a)
        }
    }

    } // verus!

    /// Inner recursive function for parallel prefix sums DC.
    /// Outside verus! because it uses thread::scope directly.
    /// - APAS: N/A — internal recursive helper for Algorithm 26.5 (parallel).
    /// - Claude-Opus-4.6: Work Θ(n lg n), Span depends on tabulate/append — see prefix_sums_dc_parallel.
    fn prefix_sums_dc_inner(a: &ArraySeqMtPerS<N>) -> (ArraySeqMtPerS<N>, N) {
        let n = a.length();
        if n == 0 {
            return (ArraySeqMtPerS::empty(), 0);
        }
        if n == 1 {
            return (ArraySeqMtPerS::singleton(0), *a.nth(0));
        }
        let mid = n / 2;
        let left = a.subseq_copy(0, mid);
        let right = a.subseq_copy(mid, n - mid);

        let left_arc = Arc::new(left);
        let right_arc = Arc::new(right);

        let ((l_prefixes, l_total), (r_prefixes, r_total)) = thread::scope(|s| {
            let la = left_arc.clone();
            let handle = s.spawn(move || prefix_sums_dc_inner(&*la));

            let ra = right_arc.clone();
            let right_result = prefix_sums_dc_inner(&*ra);

            let left_result = handle.join().unwrap();
            (left_result, right_result)
        });

        // Combine: r'[i] = l_total + r[i]
        let captured_l_total = l_total;
        let r_adjusted = ArraySeqMtPerS::tabulate(
            &|i: usize| -> N { captured_l_total + *r_prefixes.nth(i) },
            r_prefixes.length(),
        );

        let result_prefixes = ArraySeqMtPerS::append(&l_prefixes, &r_adjusted);
        let total = l_total + r_total;

        (result_prefixes, total)
    }

    use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
    use crate::Types::Types::*;

} // mod
