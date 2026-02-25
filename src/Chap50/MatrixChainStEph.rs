//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 50: Matrix Chain Multiplication - ephemeral, single-threaded.
//!
//! Memoized top-down DP for optimal matrix chain parenthesization.
//! Uses HashMapWithViewPlus for the memo table.

pub mod MatrixChainStEph {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::iter::Cloned;
    use std::slice::Iter;
    use std::vec::IntoIter;

    use vstd::prelude::*;

    use crate::Types::Types::*;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

    verus! {

// Table of Contents
// 1. module
// 2. imports
// 3. broadcast use
// 4. type definitions
// 5. view impls
// 8. traits
// 9. impls
// 11. derive impls in verus!

// 3. broadcast use
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::map::group_map_axioms,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
    vstd::seq_lib::group_to_multiset_ensures,
};

    // 4. type definitions
    #[verifier::reject_recursive_types]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct MatrixDim {
        pub rows: usize,
        pub cols: usize,
    }

    // 5. view impls
    impl View for MatrixDim {
        type V = (nat, nat);
        open spec fn view(&self) -> (nat, nat) {
            (self.rows as nat, self.cols as nat)
        }
    }

    pub struct MatrixChainStEphS {
        pub dimensions: Vec<MatrixDim>,
        pub memo: HashMapWithViewPlus<Pair<usize, usize>, usize>,
    }

    impl View for MatrixChainStEphS {
        type V = (Seq<MatrixDim>, Map<(usize, usize), usize>);
        open spec fn view(&self) -> Self::V {
            (self.dimensions@, self.memo@)
        }
    }

    // 8. traits
    pub trait MatrixChainStEphTrait: Sized + View<V = (Seq<MatrixDim>, Map<(usize, usize), usize>)> {
        fn new() -> (result: Self)
            ensures
                result@.0.len() == 0,
                result@.1 =~= Map::<(usize, usize), usize>::empty();

        fn from_dimensions(dimensions: Vec<MatrixDim>) -> (result: Self)
            ensures
                result@.0 =~= dimensions@,
                result@.1 =~= Map::<(usize, usize), usize>::empty();

        fn from_dim_pairs(dim_pairs: Vec<Pair<usize, usize>>) -> (result: Self)
            ensures
                result@.0.len() == dim_pairs@.len(),
                result@.1 =~= Map::<(usize, usize), usize>::empty();

        fn optimal_cost(&mut self) -> (result: usize);

        fn dimensions(&self) -> (result: &Vec<MatrixDim>)
            ensures result@ =~= self@.0;

        fn set_dimension(&mut self, index: usize, dim: MatrixDim);

        fn update_dimension(&mut self, index: usize, rows: usize, cols: usize);

        fn num_matrices(&self) -> (result: usize)
            ensures result == self@.0.len();

        fn clear_memo(&mut self)
            ensures self@.1 =~= Map::<(usize, usize), usize>::empty();

        fn memo_size(&self) -> (result: usize)
            ensures result == self@.1.len();
    }

    // 9. impls

    #[verifier::external_body]
    fn multiply_cost_st_eph(s: &MatrixChainStEphS, i: usize, k: usize, j: usize) -> (result: usize)
        requires
            i < s.dimensions@.len(),
            k < s.dimensions@.len(),
            j < s.dimensions@.len(),
    {
        let left_rows = s.dimensions[i].rows;
        let split_cols = s.dimensions[k].cols;
        let right_cols = s.dimensions[j].cols;
        left_rows * split_cols * right_cols
    }

    #[verifier::external_body]
    fn matrix_chain_rec_st_eph(s: &mut MatrixChainStEphS, i: usize, j: usize) -> (result: usize)
        requires
            i <= j,
            j < old(s).dimensions@.len(),
    {
        if let Some(&result) = s.memo.get(&Pair(i, j)) {
            return result;
        }

        let result = if i == j {
            0
        } else {
            (i..j)
                .map(|k| {
                    let left_cost = matrix_chain_rec_st_eph(s, i, k);
                    let right_cost = matrix_chain_rec_st_eph(s, k + 1, j);
                    let split_cost = multiply_cost_st_eph(s, i, k, j);
                    left_cost + right_cost + split_cost
                })
                .min()
                .unwrap_or(0)
        };

        s.memo.insert(Pair(i, j), result);
        result
    }

    impl MatrixChainStEphTrait for MatrixChainStEphS {
        #[verifier::external_body]
        fn new() -> (result: Self) {
            Self {
                dimensions: Vec::new(),
                memo: HashMapWithViewPlus::new(),
            }
        }

        #[verifier::external_body]
        fn from_dimensions(dimensions: Vec<MatrixDim>) -> (result: Self) {
            Self {
                dimensions,
                memo: HashMapWithViewPlus::new(),
            }
        }

        #[verifier::external_body]
        fn from_dim_pairs(dim_pairs: Vec<Pair<usize, usize>>) -> (result: Self) {
            let dimensions = dim_pairs
                .into_iter()
                .map(|pair| MatrixDim {
                    rows: pair.0,
                    cols: pair.1,
                })
                .collect();

            Self {
                dimensions,
                memo: HashMapWithViewPlus::new(),
            }
        }

        #[verifier::external_body]
        fn optimal_cost(&mut self) -> (result: usize) {
            if self.dimensions.len() <= 1 {
                return 0;
            }

            self.memo.clear();

            let n = self.dimensions.len();
            matrix_chain_rec_st_eph(self, 0, n - 1)
        }

        fn dimensions(&self) -> (result: &Vec<MatrixDim>)
        { &self.dimensions }

        #[verifier::external_body]
        fn set_dimension(&mut self, index: usize, dim: MatrixDim) {
            if index < self.dimensions.len() {
                self.dimensions[index] = dim;
            }
            self.memo.clear();
        }

        #[verifier::external_body]
        fn update_dimension(&mut self, index: usize, rows: usize, cols: usize) {
            let dim = MatrixDim { rows, cols };
            if index < self.dimensions.len() {
                self.dimensions[index] = dim;
            }
            self.memo.clear();
        }

        fn num_matrices(&self) -> (result: usize)
        { self.dimensions.len() }

        fn clear_memo(&mut self) { self.memo.clear(); }

        fn memo_size(&self) -> (result: usize)
        { self.memo.len() }
    }

    // 11. derive impls in verus!
    impl Clone for MatrixChainStEphS {
        #[verifier::external_body]
        fn clone(&self) -> (s: Self)
            ensures s@ == self@
        {
            MatrixChainStEphS {
                dimensions: self.dimensions.clone(),
                memo: self.memo.clone(),
            }
        }
    }

    #[cfg(verus_keep_ghost)]
    impl PartialEqSpecImpl for MatrixChainStEphS {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl PartialEq for MatrixChainStEphS {
        #[verifier::external_body]
        fn eq(&self, other: &Self) -> (r: bool)
            ensures r == (self@ == other@)
        {
            self.dimensions == other.dimensions && self.memo == other.memo
        }
    }

    impl Eq for MatrixChainStEphS {}

    } // verus!

    // 13. derive impls outside verus!
    impl Display for MatrixChainStEphS {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — format two integers
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "MatrixChainStEph(matrices: {}, memo_entries: {})",
                self.dimensions.len(),
                self.memo.len()
            )
        }
    }

    impl IntoIterator for MatrixChainStEphS {
        type Item = MatrixDim;
        type IntoIter = IntoIter<MatrixDim>;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — move Vec into iterator
        fn into_iter(self) -> Self::IntoIter { self.dimensions.into_iter() }
    }

    impl<'a> IntoIterator for &'a MatrixChainStEphS {
        type Item = MatrixDim;
        type IntoIter = Cloned<Iter<'a, MatrixDim>>;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — create cloned iterator adapter
        fn into_iter(self) -> Self::IntoIter { self.dimensions.iter().cloned() }
    }

    impl<'a> IntoIterator for &'a mut MatrixChainStEphS {
        type Item = MatrixDim;
        type IntoIter = Cloned<Iter<'a, MatrixDim>>;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — create cloned iterator adapter
        fn into_iter(self) -> Self::IntoIter { self.dimensions.iter().cloned() }
    }

    impl Display for MatrixDim {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — format two integers
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "{}×{}", self.rows, self.cols) }
    }

    // 12. macros
    #[macro_export]
    macro_rules! MatrixChainStEphLit {
        (dims: [$(($r:expr, $c:expr)),* $(,)?]) => {
            $crate::Chap50::MatrixChainStEph::MatrixChainStEph::MatrixChainStEphS::from_dim_pairs(
                vec![$($crate::Types::Types::Pair($r, $c)),*]
            )
        };
        () => {
            $crate::Chap50::MatrixChainStEph::MatrixChainStEph::MatrixChainStEphS::new()
        };
    }
}
