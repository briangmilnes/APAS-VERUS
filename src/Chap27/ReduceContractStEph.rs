//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Sequential reduce using contraction technique (Chapter 27, Algorithm 27.2).
//! Verusified.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	8. traits
//	9. impls

//		1. module

pub mod ReduceContractStEph {

    use vstd::prelude::*;

    verus! {

    //		2. imports

    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Types::Types::*;

    //		3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
    };

    //		8. traits

    pub trait ReduceContractStEphTrait<T: StT> {
        /// Reduce a sequence using contraction: contract→solve→expand.
        /// APAS Algorithm 27.2: Work Θ(n), Span Θ(n) (sequential).
        fn reduce_contract<F: Fn(&T, &T) -> T>(
            a: &ArraySeqStEphS<T>,
            f: &F,
            id: T,
        ) -> (result: T)
            requires
                forall|x: &T, y: &T| #[trigger] f.requires((x, y));
    }

    //		9. impls

    impl<T: StT + Clone> ReduceContractStEphTrait<T> for ArraySeqStEphS<T> {
        #[verifier::external_body]
        fn reduce_contract<F: Fn(&T, &T) -> T>(
            a: &ArraySeqStEphS<T>,
            f: &F,
            id: T,
        ) -> (result: T) {
            reduce_contract_inner(a, f, id)
        }
    }

    } // verus!

    //		Implementation (outside verus! — recursive contraction algorithm).

    /// Recursive contraction-based reduce.
    /// - APAS Algorithm 27.2: pairs consecutive elements, recurses on half-size problem.
    /// - Work Θ(n), Span Θ(n) (sequential).
    fn reduce_contract_inner<T: StT + Clone, F: Fn(&T, &T) -> T>(
        a: &ArraySeqStEphS<T>,
        f: &F,
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

        // Contract: pair up consecutive elements
        // b[i] = f(a[2i], a[2i+1])
        let half = n / 2;
        let b = ArraySeqStEphS::tabulate(
            &|i: usize| {
                let left = a.nth(2 * i);
                let right = a.nth(2 * i + 1);
                f(left, right)
            },
            half,
        );

        // Solve: recursively reduce contracted sequence
        let result = reduce_contract_inner(&b, f, id);

        // Expand: handle odd-length sequences
        if n % 2 == 1 {
            f(&result, a.nth(n - 1))
        } else {
            result
        }
    }

} // mod
