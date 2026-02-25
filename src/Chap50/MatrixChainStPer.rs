//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 50: Matrix Chain Multiplication - persistent, single-threaded.
//!
//! Memoized top-down DP for optimal matrix chain parenthesization.
//! Uses HashMapWithViewPlus for the memo table.

pub mod MatrixChainStPer {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::iter::Cloned;
    use std::slice::Iter;
    use std::vec::IntoIter;

    use vstd::prelude::*;

    use crate::Types::Types::*;
    use crate::vstdplus::accept::accept;
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
// 6. spec fns
// 8. traits
// 9. impls
// 11. derive impls in verus!

// 3. broadcast use
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    crate::Types::Types::group_Pair_axioms,
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

    pub ghost struct MatrixChainStPerV {
        pub dimensions: Seq<MatrixDim>,
        pub memo: Map<(usize, usize), usize>,
    }

    pub struct MatrixChainStPerS {
        pub dimensions: Vec<MatrixDim>,
        pub memo: HashMapWithViewPlus<Pair<usize, usize>, usize>,
    }

    impl View for MatrixChainStPerS {
        type V = MatrixChainStPerV;
        open spec fn view(&self) -> Self::V {
            MatrixChainStPerV {
                dimensions: self.dimensions@,
                memo: self.memo@,
            }
        }
    }

    // 6. spec fns
    pub open spec fn spec_multiply_cost(dims: Seq<MatrixDim>, i: int, k: int, j: int) -> nat {
        (dims[i].rows as nat) * (dims[k].cols as nat) * (dims[j].cols as nat)
    }

    /// True when all possible multiply_cost calls and their intermediates fit in usize.
    pub open spec fn spec_dims_bounded(dims: Seq<MatrixDim>) -> bool {
        forall|i: int, k: int, j: int|
            0 <= i < dims.len() && 0 <= k < dims.len() && 0 <= j < dims.len()
            ==> {
                &&& (dims[i].rows as nat) * (dims[k].cols as nat) <= usize::MAX as nat
                &&& spec_multiply_cost(dims, i, k, j) <= usize::MAX as nat
            }
    }

    /// All sub-problem costs and split-point totals in [lo, hi] fit in usize.
    pub open spec fn spec_costs_fit(dims: Seq<MatrixDim>, lo: int, hi: int) -> bool {
        forall|a: int, b: int, k: int|
            lo <= a && a <= k && k < b && b <= hi ==> {
                &&& spec_chain_cost(dims, a, b, a) <= usize::MAX as nat
                &&& spec_chain_cost(dims, a, k, a)
                    + spec_chain_cost(dims, k + 1, b, k + 1)
                    + spec_multiply_cost(dims, a, k, b) <= usize::MAX as nat
            }
    }

    /// Every memo entry holds the correct optimal cost.
    pub open spec fn spec_memo_correct(dims: Seq<MatrixDim>, memo: Map<(usize, usize), usize>) -> bool {
        forall|a: usize, b: usize| #[trigger] memo.contains_key((a, b)) ==>
            memo[(a, b)] as nat == spec_chain_cost(dims, a as int, b as int, a as int)
    }

    /// Optimal parenthesization cost for matrices i..=j, considering splits from k onward.
    /// Callers pass k == i to get the full optimum.
    pub open spec fn spec_chain_cost(dims: Seq<MatrixDim>, i: int, j: int, k: int) -> nat
        decreases j - i, j - k,
    {
        if i >= j { 0 }
        else if k < i || k >= j { usize::MAX as nat }
        else {
            let cost_at_k =
                spec_chain_cost(dims, i, k, i)
                + spec_chain_cost(dims, k + 1, j, k + 1)
                + spec_multiply_cost(dims, i, k, j);
            if k + 1 >= j {
                cost_at_k
            } else {
                let rest = spec_chain_cost(dims, i, j, k + 1);
                if cost_at_k <= rest { cost_at_k } else { rest }
            }
        }
    }

    // 8. traits
    pub trait MatrixChainStPerTrait: Sized + View<V = MatrixChainStPerV> {
        fn new() -> (mc: Self)
            ensures
                mc@.dimensions.len() == 0,
                mc@.memo =~= Map::<(usize, usize), usize>::empty();

        fn from_dimensions(dimensions: Vec<MatrixDim>) -> (mc: Self)
            ensures
                mc@.dimensions =~= dimensions@,
                mc@.memo =~= Map::<(usize, usize), usize>::empty();

        fn from_dim_pairs(dim_pairs: Vec<Pair<usize, usize>>) -> (mc: Self)
            ensures
                mc@.dimensions.len() == dim_pairs@.len(),
                mc@.memo =~= Map::<(usize, usize), usize>::empty();

        fn optimal_cost(&self) -> (cost: usize)
            requires
                spec_dims_bounded(self@.dimensions),
                self@.dimensions.len() > 1 ==>
                    spec_costs_fit(self@.dimensions, 0, (self@.dimensions.len() - 1) as int),
            ensures
                cost as nat == if self@.dimensions.len() <= 1 { 0 }
                    else { spec_chain_cost(self@.dimensions, 0, (self@.dimensions.len() - 1) as int, 0) };

        fn dimensions(&self) -> (dims: &Vec<MatrixDim>)
            ensures dims@ =~= self@.dimensions;

        fn num_matrices(&self) -> (n: usize)
            ensures n == self@.dimensions.len();

        fn memo_size(&self) -> (n: usize)
            ensures n == self@.memo.len();

        fn multiply_cost(&self, i: usize, k: usize, j: usize) -> (cost: usize)
            requires
                i < self@.dimensions.len(),
                k < self@.dimensions.len(),
                j < self@.dimensions.len(),
                (self@.dimensions[i as int].rows as nat) * (self@.dimensions[k as int].cols as nat) <= usize::MAX as nat,
                spec_multiply_cost(self@.dimensions, i as int, k as int, j as int) <= usize::MAX as nat,
            ensures
                cost as nat == spec_multiply_cost(self@.dimensions, i as int, k as int, j as int);

        fn matrix_chain_rec(&mut self, i: usize, j: usize) -> (cost: usize)
            requires
                i <= j,
                j < old(self)@.dimensions.len(),
                spec_dims_bounded(old(self)@.dimensions),
                spec_costs_fit(old(self)@.dimensions, i as int, j as int),
                spec_memo_correct(old(self)@.dimensions, old(self)@.memo),
            ensures
                self@.dimensions =~= old(self)@.dimensions,
                self@.memo.contains_key((i, j)),
                self@.memo[(i, j)] == cost,
                cost as nat == spec_chain_cost(old(self)@.dimensions, i as int, j as int, i as int),
                spec_memo_correct(self@.dimensions, self@.memo),
            decreases j - i;
    }

    // 9. impls

    impl MatrixChainStPerTrait for MatrixChainStPerS {
        fn new() -> (mc: Self)
            ensures
                mc@.dimensions.len() == 0,
                mc@.memo =~= Map::<(usize, usize), usize>::empty(),
        {
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            Self {
                dimensions: Vec::new(),
                memo: HashMapWithViewPlus::new(),
            }
        }

        fn from_dimensions(dimensions: Vec<MatrixDim>) -> (mc: Self)
            ensures
                mc@.dimensions =~= dimensions@,
                mc@.memo =~= Map::<(usize, usize), usize>::empty(),
        {
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            Self {
                dimensions,
                memo: HashMapWithViewPlus::new(),
            }
        }

        fn from_dim_pairs(dim_pairs: Vec<Pair<usize, usize>>) -> (mc: Self)
            ensures
                mc@.dimensions.len() == dim_pairs@.len(),
                mc@.memo =~= Map::<(usize, usize), usize>::empty(),
        {
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            let mut dimensions: Vec<MatrixDim> = Vec::new();
            let mut idx: usize = 0;
            while idx < dim_pairs.len()
                invariant
                    idx <= dim_pairs@.len(),
                    dimensions@.len() == idx as nat,
                decreases dim_pairs@.len() - idx,
            {
                dimensions.push(MatrixDim {
                    rows: dim_pairs[idx].0,
                    cols: dim_pairs[idx].1,
                });
                idx = idx + 1;
            }
            Self {
                dimensions,
                memo: HashMapWithViewPlus::new(),
            }
        }

        fn multiply_cost(&self, i: usize, k: usize, j: usize) -> (cost: usize)
        {
            let left_rows = self.dimensions[i].rows;
            let split_cols = self.dimensions[k].cols;
            let right_cols = self.dimensions[j].cols;
            let intermediate = left_rows * split_cols;
            intermediate * right_cols
        }

        fn matrix_chain_rec(&mut self, i: usize, j: usize) -> (cost: usize)
            decreases j - i,
        {
            if let Some(cached) = self.memo.get(&Pair(i, j)) {
                return *cached;
            }

            if i == j {
                self.memo.insert(Pair(i, j), 0);
                return 0;
            }

            let ghost old_dims = self.dimensions@;
            let mut best: usize = usize::MAX;
            let mut k: usize = i;
            while k < j
                invariant
                    i < j,
                    i <= k <= j,
                    j < self.dimensions@.len(),
                    self.dimensions@ =~= old_dims,
                    spec_dims_bounded(self.dimensions@),
                    spec_costs_fit(self.dimensions@, i as int, j as int),
                    spec_memo_correct(self.dimensions@, self.memo@),
                    spec_chain_cost(old_dims, i as int, j as int, i as int) == (
                        if best as nat <= spec_chain_cost(old_dims, i as int, j as int, k as int) {
                            best as nat
                        } else {
                            spec_chain_cost(old_dims, i as int, j as int, k as int)
                        }),
                decreases j - k,
            {
                let left_cost = self.matrix_chain_rec(i, k);
                let right_cost = self.matrix_chain_rec(k + 1, j);
                let split_cost = self.multiply_cost(i, k, j);

                assert(left_cost as nat + right_cost as nat + split_cost as nat <= usize::MAX as nat);
                let total = left_cost + right_cost + split_cost;

                if total < best {
                    best = total;
                }
                k = k + 1;
            }

            self.memo.insert(Pair(i, j), best);
            best
        }

        fn optimal_cost(&self) -> (cost: usize)
        {
            if self.dimensions.len() <= 1 {
                return 0;
            }

            let mut solver = Self::from_dimensions(self.dimensions.clone());
            let n = solver.num_matrices();
            solver.matrix_chain_rec(0, n - 1)
        }

        fn dimensions(&self) -> (dims: &Vec<MatrixDim>)
            ensures dims@ =~= self@.dimensions,
        { &self.dimensions }

        fn num_matrices(&self) -> (n: usize)
            ensures n == self@.dimensions.len(),
        { self.dimensions.len() }

        fn memo_size(&self) -> (n: usize)
            ensures n == self@.memo.len(),
        { self.memo.len() }
    }

    // 11. derive impls in verus!
    impl Clone for MatrixChainStPerS {
        fn clone(&self) -> (mc: Self)
            ensures mc@ == self@
        {
            let mc = MatrixChainStPerS {
                      dimensions: self.dimensions.clone(),
                      memo: self.memo.clone(),
                      };
            proof {accept(mc@ == self@);}
            mc
        }
    }

    #[cfg(verus_keep_ghost)]
    impl PartialEqSpecImpl for MatrixChainStPerS {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
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

    } // verus!

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
