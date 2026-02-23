//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 50: Matrix Chain Multiplication - persistent, single-threaded.
//!
//! This module is outside verus! because it uses std::collections::HashMap for
//! memoization, which Verus does not support. Full verification would require
//! replacing HashMap with a verified equivalent.

pub mod MatrixChainStPer {

    use std::collections::HashMap;
    use std::fmt::{Debug, Display, Formatter, Result};
    use std::iter::Cloned;
    use std::slice::Iter;
    use std::vec::IntoIter;

    use vstd::prelude::*;

    use crate::Types::Types::*;

    verus! {

// Veracity: added broadcast group
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

    impl View for MatrixDim {
        type V = (nat, nat);
        open spec fn view(&self) -> (nat, nat) {
            (self.rows as nat, self.cols as nat)
        }
    }

    /// Persistent single-threaded matrix chain multiplication solver using dynamic programming.
    pub struct MatrixChainStPerS {
        pub dimensions: Vec<MatrixDim>,
        pub memo: std::collections::HashMap<(usize, usize), usize>,
    }

    impl View for MatrixChainStPerS {
        type V = (Seq<MatrixDim>, Map<(usize, usize), usize>);
        open spec fn view(&self) -> Self::V {
            (self.dimensions@, self.memo@)
        }
    }

    impl Clone for MatrixChainStPerS {
        #[verifier::external_body]
        fn clone(&self) -> (s: Self)
            ensures s@ == self@
        {
            MatrixChainStPerS {
                dimensions: self.dimensions.clone(),
                memo: self.memo.clone(),
            }
        }
    }

    impl PartialEq for MatrixChainStPerS {
        #[verifier::external_body]
        fn eq(&self, other: &Self) -> (r: bool)
            ensures r == (self@ == other@)
        {
            self.dimensions == other.dimensions && self.memo == other.memo
        }
    }

    impl Eq for MatrixChainStPerS {}
    }

    // 8. traits
    /// Trait for matrix chain multiplication operations
    pub trait MatrixChainStPerTrait: Sized {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — allocate empty collections
        fn new()                                              -> Self;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — move ownership of Vec
        fn from_dimensions(dimensions: Vec<MatrixDim>)        -> Self;

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — map n pairs to MatrixDim
        fn from_dim_pairs(dim_pairs: Vec<Pair<usize, usize>>) -> Self;

        /// - APAS: Work Θ(n³), Span Θ(n³)
        /// - Claude-Opus-4.6: Work Θ(n³), Span Θ(n³) — clones self then invokes memoized DP, sequential
        fn optimal_cost(&self)                                -> usize;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — reference access
        fn dimensions(&self)                                  -> &Vec<MatrixDim>;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — Vec::len
        fn num_matrices(&self)                                -> usize;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — HashMap::len
        fn memo_size(&self)                                   -> usize;
    }

    // 9. impls

    fn multiply_cost_st_per(s: &MatrixChainStPerS, i: usize, k: usize, j: usize) -> usize {
        let left_rows = s.dimensions[i].rows;
        let split_cols = s.dimensions[k].cols;
        let right_cols = s.dimensions[j].cols;
        left_rows * split_cols * right_cols
    }

    fn matrix_chain_rec_st_per(s: &mut MatrixChainStPerS, i: usize, j: usize) -> usize {
        if let Some(&result) = s.memo.get(&(i, j)) {
            return result;
        }

        let result = if i == j {
            0
        } else {
            (i..j)
                .map(|k| {
                    let left_cost = matrix_chain_rec_st_per(s, i, k);
                    let right_cost = matrix_chain_rec_st_per(s, k + 1, j);
                    let split_cost = multiply_cost_st_per(s, i, k, j);
                    left_cost + right_cost + split_cost
                })
                .min()
                .unwrap_or(0)
        };

        s.memo.insert((i, j), result);
        result
    }

    impl MatrixChainStPerTrait for MatrixChainStPerS {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — allocate empty Vec and HashMap
        fn new() -> Self {
            Self {
                dimensions: Vec::new(),
                memo: HashMap::new(),
            }
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — move ownership of dimensions Vec
        fn from_dimensions(dimensions: Vec<MatrixDim>) -> Self {
            Self {
                dimensions,
                memo: HashMap::new(),
            }
        }

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — map n Pair values to MatrixDim structs
        fn from_dim_pairs(dim_pairs: Vec<Pair<usize, usize>>) -> Self {
            let dimensions = dim_pairs
                .into_iter()
                .map(|pair| MatrixDim {
                    rows: pair.0,
                    cols: pair.1,
                })
                .collect();

            Self {
                dimensions,
                memo: HashMap::new(),
            }
        }

        /// - APAS: Work Θ(n³), Span Θ(n³)
        /// - Claude-Opus-4.6: Work Θ(n³), Span Θ(n³) — clones self, clears memo, invokes matrix_chain_rec
        fn optimal_cost(&self) -> usize {
            if self.dimensions.len() <= 1 {
                return 0;
            }

            let mut solver = self.clone();
            solver.memo.clear();

            let n = solver.dimensions.len();
            matrix_chain_rec_st_per(&mut solver, 0, n - 1)
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — reference access
        fn dimensions(&self) -> &Vec<MatrixDim> { &self.dimensions }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — Vec::len
        fn num_matrices(&self) -> usize { self.dimensions.len() }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — HashMap::len
        fn memo_size(&self) -> usize { self.memo.len() }
    }

    // 13. derive impls outside verus!
    impl Display for MatrixChainStPerS {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — format two integers
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(
                f,
                "MatrixChainStPer(matrices: {}, memo_entries: {})",
                self.dimensions.len(),
                self.memo.len()
            )
        }
    }

    impl IntoIterator for MatrixChainStPerS {
        type Item = MatrixDim;
        type IntoIter = IntoIter<MatrixDim>;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — move Vec into iterator
        fn into_iter(self) -> Self::IntoIter { self.dimensions.into_iter() }
    }

    impl<'a> IntoIterator for &'a MatrixChainStPerS {
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
}

#[macro_export]
macro_rules! MatrixChainStPerLit {
    (dims: [$(($r:expr, $c:expr)),* $(,)?]) => {
        $crate::Chap50::MatrixChainStPer::MatrixChainStPer::MatrixChainStPerS::from_dim_pairs(
            vec![$($crate::Types::Types::Pair($r, $c)),*]
        )
    };
    () => {
        $crate::Chap50::MatrixChainStPer::MatrixChainStPer::MatrixChainStPerS::new()
    };
}
