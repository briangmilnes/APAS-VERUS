//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Divide-and-conquer scan - sequential implementation (Chapter 26, Section 3).
//! Verusified.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	4. spec functions
//	8. traits
//	9. impls

//		1. module

pub mod ScanDCStPer {

    use vstd::prelude::*;

    verus! {

    //		2. imports

    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap18::ArraySeq::ArraySeq::{spec_iterate, spec_monoid};
    use crate::Types::Types::*;

    //		3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
    };

    //		4. spec functions

    /// Spec function: exclusive prefix scan result at position i is the fold of elements [0..i).
    pub open spec fn spec_scan_at(s: Seq<N>, spec_f: spec_fn(N, N) -> N, id: N, i: int) -> N
        recommends 0 <= i <= s.len(),
    {
        s.take(i).fold_left(id, spec_f)
    }

    /// Spec function: full exclusive scan postcondition.
    pub open spec fn spec_scan_post(
        input: Seq<N>, spec_f: spec_fn(N, N) -> N, id: N,
        prefixes: Seq<N>, total: N,
    ) -> bool {
        &&& prefixes.len() == input.len()
        &&& forall|i: int| #![trigger prefixes[i]]
                0 <= i < input.len() ==> prefixes[i] == spec_scan_at(input, spec_f, id, i)
        &&& total == spec_iterate(input, spec_f, id)
    }

    //		8. traits

    pub trait ScanDCStTrait {
        /// Algorithm 26.5: Exclusive prefix scan via divide and conquer.
        /// Returns (prefixes, total) where prefixes[i] = f(id, a[0], ..., a[i-1]).
        /// Work Θ(n log n), Span Θ(log n)
        fn scan_dc<F: Fn(&N, &N) -> N>(a: &ArraySeqStPerS<N>, f: &F, Ghost(spec_f): Ghost<spec_fn(N, N) -> N>, id: N) -> (result: (ArraySeqStPerS<N>, N))
            requires
                a.spec_len() <= usize::MAX,
                spec_monoid(spec_f, id),
                forall|x: &N, y: &N| #[trigger] f.requires((x, y)),
                forall|x: N, y: N, ret: N| f.ensures((&x, &y), ret) <==> ret == spec_f(x, y),
            ensures
                spec_scan_post(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)),
                    spec_f, id,
                    Seq::new(result.0.spec_len(), |i: int| result.0.spec_index(i)),
                    result.1);

        /// Exclusive prefix sums via divide-and-conquer scan.
        /// Convenience: scan_dc with (+, 0).
        fn prefix_sums_dc(a: &ArraySeqStPerS<N>) -> (result: (ArraySeqStPerS<N>, N))
            requires a.spec_len() <= usize::MAX,
            ensures
                result.0.spec_len() == a.spec_len(),
                result.1 == spec_iterate(
                    Seq::new(a.spec_len(), |i: int| a.spec_index(i)),
                    |x: N, y: N| (x + y) as N, 0);
    }

    //		9. impls

    impl ScanDCStTrait for ArraySeqStPerS<N> {
        #[verifier::external_body]
        fn scan_dc<F: Fn(&N, &N) -> N>(a: &ArraySeqStPerS<N>, f: &F, Ghost(spec_f): Ghost<spec_fn(N, N) -> N>, id: N) -> (result: (ArraySeqStPerS<N>, N)) {
            let n = a.length();
            if n == 0 {
                return (ArraySeqStPerS::empty(), id);
            }
            if n == 1 {
                return (ArraySeqStPerS::singleton(id), *a.nth(0));
            }
            // Divide: split at midpoint
            let mid = n / 2;
            let left = a.subseq_copy(0, mid);
            let right = a.subseq_copy(mid, n - mid);

            // Recur sequentially
            let (l_prefixes, l_total) = Self::scan_dc(&left, f, Ghost(spec_f), id);
            let (r_prefixes, r_total) = Self::scan_dc(&right, f, Ghost(spec_f), id);

            // Combine: r'[i] = f(l_total, r[i])
            let r_adjusted = ArraySeqStPerS::tabulate(
                &|i: usize| -> N { f(&l_total, r_prefixes.nth(i)) },
                r_prefixes.length(),
            );

            let result_prefixes = ArraySeqStPerS::append(&l_prefixes, &r_adjusted);
            let total = f(&l_total, &r_total);

            (result_prefixes, total)
        }

        #[verifier::external_body]
        fn prefix_sums_dc(a: &ArraySeqStPerS<N>) -> (result: (ArraySeqStPerS<N>, N)) {
            Self::scan_dc(a, &|x: &N, y: &N| x + y, Ghost(|x: N, y: N| (x + y) as N), 0)
        }
    }

    } // verus!
} // mod
