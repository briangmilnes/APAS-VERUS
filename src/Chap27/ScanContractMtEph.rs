//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Parallel scan using contraction technique (Chapter 27, Algorithm 27.3).
//! Uses the help-first scheduler for fork-join parallelism.
//! Verusified.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	8. traits
//	9. impls

//		1. module

pub mod ScanContractMtEph {

    use std::sync::Arc;
    use vstd::prelude::*;

    verus! {

    //		2. imports

    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Concurrency::Concurrency::StTInMtT;
    use crate::vstdplus::monoid::monoid::*;

    //		3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
    };

    //		8. traits

    pub trait ScanContractMtEphTrait<T: StTInMtT> {
        /// Exclusive scan using parallel contraction: contract→solve→expand.
        /// APAS Algorithm 27.3: Work Θ(n), Span Θ(log n).
        /// Returns prefixes where result[i] = fold_left(input[0..i], id, spec_f).
        fn scan_contract_parallel<F: Fn(&T, &T) -> T + Send + Sync + 'static>(
            a: &ArraySeqMtEphS<T>,
            f: Arc<F>,
            Ghost(spec_f): Ghost<spec_fn(T, T) -> T>,
            id: T,
        ) -> (result: ArraySeqMtEphS<T>)
            requires
                a.spec_len() <= usize::MAX,
                spec_monoid(spec_f, id),
                forall|x: &T, y: &T| #[trigger] f.requires((x, y)),
                forall|x: T, y: T, ret: T| f.ensures((&x, &y), ret) ==> ret == spec_f(x, y),
            ensures
                result.spec_len() == a.spec_len(),
                forall|i: int| #![trigger result.spec_index(i)]
                    0 <= i < a.spec_len() ==>
                        result.spec_index(i) == Seq::new(a.spec_len(), |j: int| a.spec_index(j)).take(i).fold_left(id, spec_f);
    }

    //		9. impls

    impl<T: StTInMtT + Clone + 'static> ScanContractMtEphTrait<T> for ArraySeqMtEphS<T> {
        #[verifier::external_body]
        fn scan_contract_parallel<F: Fn(&T, &T) -> T + Send + Sync + 'static>(
            a: &ArraySeqMtEphS<T>,
            f: Arc<F>,
            Ghost(spec_f): Ghost<spec_fn(T, T) -> T>,
            id: T,
        ) -> (result: ArraySeqMtEphS<T>) {
            scan_contract_parallel_inner(a, f, id)
        }
    }

    } // verus!

    //		Implementation (outside verus! — uses threads via parallel tabulate).

    /// Recursive parallel contraction-based exclusive scan.
    /// - APAS Algorithm 27.3: parallel tabulate for contraction + expansion, recursive scan.
    /// - Work Θ(n), Span Θ(log n).
    fn scan_contract_parallel_inner<T: StTInMtT + Clone + 'static, F: Fn(&T, &T) -> T + Send + Sync + 'static>(
        a: &ArraySeqMtEphS<T>,
        f: Arc<F>,
        id: T,
    ) -> ArraySeqMtEphS<T> {
        let n = a.length();

        // Base case: empty sequence
        if n == 0 {
            return ArraySeqMtEphS::empty();
        }

        // Base case: single element
        if n == 1 {
            return ArraySeqMtEphS::singleton(id.clone());
        }

        // Contract: pair up consecutive elements in parallel
        // b[i] = f(a[2i], a[2i+1])
        let half = n / 2;
        let a_arc = Arc::new(a.clone());
        let f_contract = Arc::clone(&f);

        let b = ArraySeqMtEphS::tabulate(
            &|i: usize| {
                let left = a_arc.nth(2 * i).clone();
                let right = a_arc.nth(2 * i + 1).clone();
                f_contract(&left, &right)
            },
            half,
        );

        // Solve: recursively scan contracted sequence
        let f_solve = Arc::clone(&f);
        let c = scan_contract_parallel_inner(&b, f_solve, id.clone());

        // Expand: reconstruct result using parallel tabulation
        // For even indices: result[2i] = c[i]
        // For odd indices: result[2i+1] = f(c[i], a[2i])
        let c_arc = Arc::new(c);
        let a_arc2 = Arc::new(a.clone());
        let f_expand = Arc::clone(&f);

        let main_len = if n % 2 == 0 { n } else { n - 1 };
        let main_result = ArraySeqMtEphS::tabulate(
            &|i: usize| {
                if i % 2 == 0 {
                    c_arc.nth(i / 2).clone()
                } else {
                    let scan_val = c_arc.nth(i / 2).clone();
                    let orig_val = a_arc2.nth(i - 1).clone();
                    f_expand(&scan_val, &orig_val)
                }
            },
            main_len,
        );

        // Handle last element if odd length
        if n % 2 == 1 {
            let second_to_last = main_result.nth(main_len - 1).clone();
            let last_scan_val = f(&second_to_last, a.nth(n - 2));
            let last_part = ArraySeqMtEphS::singleton(last_scan_val);
            ArraySeqMtEphS::append(&main_result, &last_part)
        } else {
            main_result
        }
    }

} // mod
