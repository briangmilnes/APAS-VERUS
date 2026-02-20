//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 50: Matrix Chain Multiplication - ephemeral, single-threaded.
//!
//! Uses HashMap for memoization. Struct/trait in verus! with external_type_spec.

pub mod MatrixChainStEph {

    use std::collections::HashMap;
    use std::fmt::{Debug, Display, Formatter, Result};
    use std::iter::Cloned;
    use std::slice::Iter;
    use std::vec::IntoIter;

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

    use crate::Types::Types::*;

    verus! {
    // 4. type definitions
    #[verifier::reject_recursive_types]
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

    impl Clone for MatrixDim {
        fn clone(&self) -> (s: Self)
            ensures s@ == self@
        {
            MatrixDim { rows: self.rows, cols: self.cols }
        }
    }

    #[cfg(verus_keep_ghost)]
    impl PartialEqSpecImpl for MatrixDim {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl Eq for MatrixDim {}
    impl PartialEq for MatrixDim {
        fn eq(&self, other: &Self) -> (r: bool)
            ensures r == (self@ == other@)
        {
            let r = self.rows == other.rows && self.cols == other.cols;
            proof { assume(r == (self@ == other@)); }
            r
        }
    }

    #[verifier::external_type_specification]
    pub struct ExMatrixChainStEphS(MatrixChainStEphS);
    }

    // Struct contains HashMap for memoization — cannot be inside verus!.
    /// Ephemeral single-threaded matrix chain multiplication solver using dynamic programming
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct MatrixChainStEphS {
        pub dimensions: Vec<MatrixDim>,
        pub memo: HashMap<(usize, usize), usize>,
    }

    // 8. traits
    /// Trait for matrix chain multiplication operations
    pub trait MatrixChainStEphTrait: Sized {
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
        /// - Claude-Opus-4.6: Work Θ(n³), Span Θ(n³) — memoized DP, n² subproblems × O(n) each, sequential
        fn optimal_cost(&mut self)                            -> usize;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — reference access
        fn dimensions(&self)                                  -> &Vec<MatrixDim>;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — mutable reference access
        fn dimensions_mut(&mut self)                          -> &mut Vec<MatrixDim>;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — array write plus memo clear
        fn set_dimension(&mut self, index: usize, dim: MatrixDim);

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — array write plus memo clear
        fn update_dimension(&mut self, index: usize, rows: usize, cols: usize);

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — Vec::len
        fn num_matrices(&self)                                -> usize;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — HashMap::clear
        fn clear_memo(&mut self);

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — HashMap::len
        fn memo_size(&self)                                   -> usize;
    }

    // 9. impls

    fn multiply_cost_st_eph(s: &MatrixChainStEphS, i: usize, k: usize, j: usize) -> usize {
        let left_rows = s.dimensions[i].rows;
        let split_cols = s.dimensions[k].cols;
        let right_cols = s.dimensions[j].cols;
        left_rows * split_cols * right_cols
    }

    fn matrix_chain_rec_st_eph(s: &mut MatrixChainStEphS, i: usize, j: usize) -> usize {
        if let Some(&result) = s.memo.get(&(i, j)) {
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

        s.memo.insert((i, j), result);
        result
    }

    impl MatrixChainStEphTrait for MatrixChainStEphS {
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
        /// - Claude-Opus-4.6: Work Θ(n³), Span Θ(n³) — invokes matrix_chain_rec on full range
        fn optimal_cost(&mut self) -> usize {
            if self.dimensions.len() <= 1 {
                return 0;
            }

            self.memo.clear();

            let n = self.dimensions.len();
            matrix_chain_rec_st_eph(self, 0, n - 1)
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — reference access
        fn dimensions(&self) -> &Vec<MatrixDim> { &self.dimensions }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — mutable reference access
        fn dimensions_mut(&mut self) -> &mut Vec<MatrixDim> { &mut self.dimensions }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — array write plus memo clear
        fn set_dimension(&mut self, index: usize, dim: MatrixDim) {
            if index < self.dimensions.len() {
                self.dimensions[index] = dim;
            }
            self.memo.clear();
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — array write plus memo clear
        fn update_dimension(&mut self, index: usize, rows: usize, cols: usize) {
            let dim = MatrixDim { rows, cols };
            if index < self.dimensions.len() {
                self.dimensions[index] = dim;
            }
            self.memo.clear();
        }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — Vec::len
        fn num_matrices(&self) -> usize { self.dimensions.len() }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — HashMap::clear
        fn clear_memo(&mut self) { self.memo.clear(); }

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — HashMap::len
        fn memo_size(&self) -> usize { self.memo.len() }
    }

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

    impl Debug for MatrixDim {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "MatrixDim({}×{})", self.rows, self.cols) }
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
