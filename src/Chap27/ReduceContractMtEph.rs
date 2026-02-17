//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Parallel reduce using contraction technique (Chapter 27, Algorithm 27.2).
//! Uses the help-first scheduler for fork-join parallelism.
//! Verusified.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
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

    //		3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
    };

    //		8. traits

    pub trait ReduceContractMtEphTrait<T: StTInMtT> {
        /// Reduce a sequence using parallel contraction: contract→solve→expand.
        /// APAS Algorithm 27.2: Work Θ(n), Span Θ(log n).
        fn reduce_contract_parallel<F: Fn(&T, &T) -> T + Send + Sync + 'static>(
            a: &ArraySeqMtEphS<T>,
            f: Arc<F>,
            id: T,
        ) -> (result: T);
    }

    //		9. impls

    impl<T: StTInMtT + Clone + 'static> ReduceContractMtEphTrait<T> for ArraySeqMtEphS<T> {
        #[verifier::external_body]
        fn reduce_contract_parallel<F: Fn(&T, &T) -> T + Send + Sync + 'static>(
            a: &ArraySeqMtEphS<T>,
            f: Arc<F>,
            id: T,
        ) -> (result: T) {
            reduce_contract_parallel_inner(a, f, id)
        }
    }

    } // verus!

    //		Implementation (outside verus! — uses threads via ParaPair!).

    /// Recursive parallel contraction-based reduce.
    /// - APAS Algorithm 27.2: parallel tabulate for contraction, recursive solve.
    /// - Work Θ(n), Span Θ(log n).
    fn reduce_contract_parallel_inner<T: StTInMtT + Clone + 'static, F: Fn(&T, &T) -> T + Send + Sync + 'static>(
        a: &ArraySeqMtEphS<T>,
        f: Arc<F>,
        id: T,
    ) -> T {
        let n = a.length();

        // Base case: empty sequence
        if n == 0 {
            return id;
        }
        // Base case: single element
        if n == 1 {
            return a.nth(0).clone();
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

        // Solve: recursively reduce contracted sequence
        let f_solve = Arc::clone(&f);
        let result = reduce_contract_parallel_inner(&b, f_solve, id);

        // Expand: handle odd-length sequences
        if n % 2 == 1 {
            f(&result, a.nth(n - 1))
        } else {
            result
        }
    }

} // mod
