//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Sequential scan using contraction technique (Chapter 27, Algorithm 27.3).
//! Verusified.

//  Table of Contents
//	1. module
//	2. imports
//	3. broadcast use
//	8. traits
//	9. impls

//		1. module

pub mod ScanContractStEph {

    use vstd::prelude::*;

    verus! {

    //		2. imports

    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::vstdplus::monoid::monoid::*;
    use crate::Types::Types::*;

    //		3. broadcast use

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
    };

    //		8. traits

    pub trait ScanContractStEphTrait<T: StT> {
        /// Exclusive scan using contraction: contract→solve→expand.
        /// APAS Algorithm 27.3: Work Θ(n), Span Θ(n) (sequential).
        /// Returns prefixes where result[i] = fold_left(input[0..i], id, spec_f).
        fn scan_contract<F: Fn(&T, &T) -> T>(
            a: &ArraySeqStEphS<T>,
            f: &F,
            Ghost(spec_f): Ghost<spec_fn(T, T) -> T>,
            id: T,
        ) -> (result: ArraySeqStEphS<T>)
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

    impl<T: StT + Clone> ScanContractStEphTrait<T> for ArraySeqStEphS<T> {
        #[verifier::external_body]
        fn scan_contract<F: Fn(&T, &T) -> T>(
            a: &ArraySeqStEphS<T>,
            f: &F,
            Ghost(spec_f): Ghost<spec_fn(T, T) -> T>,
            id: T,
        ) -> (result: ArraySeqStEphS<T>) {
            scan_contract_inner(a, f, id)
        }
    }

    } // verus!

    //		Implementation (outside verus! — recursive contraction algorithm).

    /// Recursive contraction-based exclusive scan.
    /// - APAS Algorithm 27.3: pairs consecutive elements, recurses on half-size, expands interleaved.
    /// - Work Θ(n), Span Θ(n) (sequential).
    fn scan_contract_inner<T: StT + Clone, F: Fn(&T, &T) -> T>(
        a: &ArraySeqStEphS<T>,
        f: &F,
        id: T,
    ) -> ArraySeqStEphS<T> {
        let n = a.length();

        // Base case: empty sequence
        if n == 0 {
            return ArraySeqStEphS::empty();
        }

        // Base case: single element
        if n == 1 {
            return ArraySeqStEphS::singleton(id.clone());
        }

        // Contract: pair up consecutive elements
        // b[i] = f(a[2i], a[2i+1])
        let half = n / 2;
        let b = ArraySeqStEphS::tabulate(
            &|i: usize| f(a.nth(2 * i), a.nth(2 * i + 1)),
            half,
        );

        // Solve: recursively scan contracted sequence
        let c = scan_contract_inner(&b, f, id.clone());

        // Expand: reconstruct result
        // For even indices: result[2i] = c[i]
        // For odd indices: result[2i+1] = f(c[i], a[2i])
        let main_len = if n % 2 == 0 { n } else { n - 1 };
        let main_result = ArraySeqStEphS::tabulate(
            &|i: usize| {
                if i % 2 == 0 {
                    c.nth(i / 2).clone()
                } else {
                    f(c.nth(i / 2), a.nth(i - 1))
                }
            },
            main_len,
        );

        // Handle last element if odd length
        if n % 2 == 1 {
            let second_to_last = main_result.nth(main_len - 1).clone();
            let last_scan_val = f(&second_to_last, a.nth(n - 2));
            let last_part = ArraySeqStEphS::singleton(last_scan_val);
            ArraySeqStEphS::append(&main_result, &last_part)
        } else {
            main_result
        }
    }

} // mod
