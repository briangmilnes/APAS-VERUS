//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 50: Matrix Chain Multiplication - persistent, multi-threaded.
//!
//! Memoized top-down DP with parallel min reduction.
//! Uses Arc<RwLock<HashMapWithViewPlus>> for the memo table.

pub mod MatrixChainMtPer {

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::iter::Cloned;
    use std::slice::Iter;
    use std::sync::Arc;
    use std::thread;
    use std::vec::IntoIter;

    use vstd::prelude::*;
    use vstd::rwlock::*;

    use crate::Types::Types::*;
    use crate::vstdplus::accept::accept;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
    use crate::vstdplus::smart_ptrs::smart_ptrs::arc_deref;
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

    pub ghost struct MatrixChainMtPerV {
        pub dimensions: Seq<MatrixDim>,
    }

    pub struct spec_matrixchainmtper_memo_wf {
        pub ghost dims: Seq<MatrixDim>,
    }
    impl RwLockPredicate<HashMapWithViewPlus<Pair<usize, usize>, usize>> for spec_matrixchainmtper_memo_wf {
        open spec fn inv(self, v: HashMapWithViewPlus<Pair<usize, usize>, usize>) -> bool {
            &&& v@.dom().finite()
            &&& spec_memo_correct(self.dims, v@)
        }
    }

    #[verifier::external_body]
    fn new_mcper_memo_lock(
        val: HashMapWithViewPlus<Pair<usize, usize>, usize>,
        Ghost(dims): Ghost<Seq<MatrixDim>>,
    ) -> (lock: RwLock<HashMapWithViewPlus<Pair<usize, usize>, usize>, spec_matrixchainmtper_memo_wf>)
        requires
            val@.dom().finite(),
            spec_memo_correct(dims, val@),
    {
        RwLock::new(val, Ghost(spec_matrixchainmtper_memo_wf { dims }))
    }

    pub struct MatrixChainMtPerS {
        pub dimensions: Arc<Vec<MatrixDim>>,
        pub memo: Arc<RwLock<HashMapWithViewPlus<Pair<usize, usize>, usize>, spec_matrixchainmtper_memo_wf>>,
    }

    impl View for MatrixChainMtPerS {
        type V = MatrixChainMtPerV;
        open spec fn view(&self) -> Self::V {
            MatrixChainMtPerV { dimensions: self.dimensions@ }
        }
    }

    // 6. spec fns
    pub open spec fn spec_multiply_cost(dims: Seq<MatrixDim>, i: int, k: int, j: int) -> nat {
        (dims[i].rows as nat) * (dims[k].cols as nat) * (dims[j].cols as nat)
    }

    pub open spec fn spec_dims_bounded(dims: Seq<MatrixDim>) -> bool {
        forall|i: int, k: int, j: int|
            0 <= i < dims.len() && 0 <= k < dims.len() && 0 <= j < dims.len()
            ==> {
                &&& (dims[i].rows as nat) * (dims[k].cols as nat) <= usize::MAX as nat
                &&& spec_multiply_cost(dims, i, k, j) <= usize::MAX as nat
            }
    }

    pub open spec fn spec_costs_fit(dims: Seq<MatrixDim>, lo: int, hi: int) -> bool {
        forall|a: int, b: int, k: int|
            lo <= a && a <= k && k < b && b <= hi ==> {
                &&& spec_chain_cost(dims, a, b, a) <= usize::MAX as nat
                &&& spec_chain_cost(dims, a, k, a)
                    + spec_chain_cost(dims, k + 1, b, k + 1)
                    + spec_multiply_cost(dims, a, k, b) <= usize::MAX as nat
            }
    }

    pub open spec fn spec_memo_correct(dims: Seq<MatrixDim>, memo: Map<(usize, usize), usize>) -> bool {
        forall|a: usize, b: usize| #[trigger] memo.contains_key((a, b)) ==>
            memo[(a, b)] as nat == spec_chain_cost(dims, a as int, b as int, a as int)
    }

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
    pub trait MatrixChainMtPerTrait: Sized + View<V = MatrixChainMtPerV> {
        fn new() -> (mc: Self)
            ensures mc@.dimensions.len() == 0;

        fn from_dimensions(dimensions: Vec<MatrixDim>) -> (mc: Self)
            ensures mc@.dimensions =~= dimensions@;

        fn from_dim_pairs(dim_pairs: Vec<Pair<usize, usize>>) -> (mc: Self)
            ensures mc@.dimensions.len() == dim_pairs@.len();

        fn optimal_cost(&self) -> (cost: usize)
            requires
                spec_dims_bounded(self@.dimensions),
                self@.dimensions.len() > 1 ==>
                    spec_costs_fit(self@.dimensions, 0, (self@.dimensions.len() - 1) as int),
            ensures
                cost as nat == if self@.dimensions.len() <= 1 { 0 }
                    else { spec_chain_cost(self@.dimensions, 0, (self@.dimensions.len() - 1) as int, 0) };

        fn dimensions(&self) -> (dims: &Arc<Vec<MatrixDim>>);

        fn num_matrices(&self) -> (n: usize)
            ensures n == self@.dimensions.len();

        fn memo_size(&self) -> (n: usize);

        fn multiply_cost(&self, i: usize, k: usize, j: usize) -> (cost: usize)
            requires
                i < self@.dimensions.len(),
                k < self@.dimensions.len(),
                j < self@.dimensions.len(),
                (self@.dimensions[i as int].rows as nat) * (self@.dimensions[k as int].cols as nat) <= usize::MAX as nat,
                spec_multiply_cost(self@.dimensions, i as int, k as int, j as int) <= usize::MAX as nat,
            ensures
                cost as nat == spec_multiply_cost(self@.dimensions, i as int, k as int, j as int);

        fn matrix_chain_rec(&self, i: usize, j: usize) -> (cost: usize)
            requires
                i <= j,
                j < self@.dimensions.len(),
                spec_dims_bounded(self@.dimensions),
                spec_costs_fit(self@.dimensions, i as int, j as int),
            ensures
                cost as nat == spec_chain_cost(self@.dimensions, i as int, j as int, i as int);

        fn parallel_min_reduction(&self, costs: Vec<usize>) -> (min: usize)
            requires costs@.len() > 0,
            ensures
                costs@.contains(min),
                forall|i: int| 0 <= i < costs@.len() ==> min <= costs@[i];
    }

    // 9. impls

    impl MatrixChainMtPerTrait for MatrixChainMtPerS {
        #[verifier::external_body]
        fn new() -> (mc: Self) {
            Self {
                dimensions: Arc::new(Vec::new()),
                memo: Arc::new(new_mcper_memo_lock(HashMapWithViewPlus::new(), Ghost(Seq::empty()))),
            }
        }

        #[verifier::external_body]
        fn from_dimensions(dimensions: Vec<MatrixDim>) -> (mc: Self) {
            Self {
                dimensions: Arc::new(dimensions),
                memo: Arc::new(new_mcper_memo_lock(HashMapWithViewPlus::new(), Ghost(Seq::empty()))),
            }
        }

        #[verifier::external_body]
        fn from_dim_pairs(dim_pairs: Vec<Pair<usize, usize>>) -> (mc: Self) {
            let mut dimensions: Vec<MatrixDim> = Vec::new();
            let mut idx: usize = 0;
            while idx < dim_pairs.len() {
                dimensions.push(MatrixDim {
                    rows: dim_pairs[idx].0,
                    cols: dim_pairs[idx].1,
                });
                idx = idx + 1;
            }
            Self {
                dimensions: Arc::new(dimensions),
                memo: Arc::new(new_mcper_memo_lock(HashMapWithViewPlus::new(), Ghost(Seq::empty()))),
            }
        }

        fn multiply_cost(&self, i: usize, k: usize, j: usize) -> (cost: usize) {
            let dims = arc_deref(&self.dimensions);
            let left_rows = dims[i].rows;
            let split_cols = dims[k].cols;
            let right_cols = dims[j].cols;
            let intermediate = left_rows * split_cols;
            intermediate * right_cols
        }

        #[verifier::external_body]
        fn parallel_min_reduction(&self, costs: Vec<usize>) -> (min: usize) {
            if costs.is_empty() {
                return usize::MAX;
            }
            if costs.len() == 1 {
                return costs[0];
            }

            let mid = costs.len() / 2;
            let left_costs = costs[..mid].to_vec();
            let right_costs = costs[mid..].to_vec();

            let s1 = self.clone();
            let s2 = self.clone();

            let handle1 = thread::spawn(move || s1.parallel_min_reduction(left_costs));
            let handle2 = thread::spawn(move || s2.parallel_min_reduction(right_costs));

            let left_min = handle1.join().unwrap();
            let right_min = handle2.join().unwrap();

            left_min.min(right_min)
        }

        #[verifier::external_body]
        fn matrix_chain_rec(&self, i: usize, j: usize) -> (cost: usize) {
            {
                let handle = self.memo.acquire_read();
                let cached = handle.borrow().get(&Pair(i, j)).copied();
                handle.release_read();
                if let Some(result) = cached {
                    return result;
                }
            }

            let result = if i == j {
                0
            } else {
                let costs = (i..j)
                    .map(|k| {
                        let left_cost = self.matrix_chain_rec(i, k);
                        let right_cost = self.matrix_chain_rec(k + 1, j);
                        let split_cost = self.multiply_cost(i, k, j);
                        left_cost + right_cost + split_cost
                    })
                    .collect::<Vec<usize>>();

                self.parallel_min_reduction(costs)
            };

            {
                let (mut memo, write_handle) = self.memo.acquire_write();
                memo.insert(Pair(i, j), result);
                write_handle.release_write(memo);
            }

            result
        }

        #[verifier::external_body]
        fn optimal_cost(&self) -> (cost: usize) {
            if self.dimensions.len() <= 1 {
                return 0;
            }

            {
                let (mut memo, write_handle) = self.memo.acquire_write();
                memo.clear();
                write_handle.release_write(memo);
            }

            let n = self.dimensions.len();
            self.matrix_chain_rec(0, n - 1)
        }

        fn dimensions(&self) -> (dims: &Arc<Vec<MatrixDim>>) { &self.dimensions }

        fn num_matrices(&self) -> (n: usize) {
            let dims = arc_deref(&self.dimensions);
            dims.len()
        }

        #[verifier::external_body]
        fn memo_size(&self) -> (n: usize) {
            let handle = self.memo.acquire_read();
            let len = handle.borrow().len();
            handle.release_read();
            len
        }
    }

    // 11. derive impls in verus!
    impl Clone for MatrixChainMtPerS {
        fn clone(&self) -> (mc: Self)
            ensures mc@ == self@
        {
            let mc = MatrixChainMtPerS {
                dimensions: self.dimensions.clone(),
                memo: self.memo.clone(),
            };
            proof { accept(mc@ == self@); }
            mc
        }
    }

    #[cfg(verus_keep_ghost)]
    impl PartialEqSpecImpl for MatrixChainMtPerS {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl PartialEq for MatrixChainMtPerS {
        #[verifier::external_body]
        fn eq(&self, other: &Self) -> (r: bool)
            ensures r == (self@ == other@)
        {
            self.dimensions == other.dimensions
        }
    }

    impl Eq for MatrixChainMtPerS {}

    } // verus!

    // 13. derive impls outside verus!
    impl Debug for MatrixChainMtPerS {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { Display::fmt(self, f) }
    }

    impl Display for MatrixChainMtPerS {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — format two integers
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            let memo_handle = self.memo.acquire_read();
            let memo_size = memo_handle.borrow().len();
            memo_handle.release_read();
            write!(
                f,
                "MatrixChainMtPer(matrices: {}, memo_entries: {})",
                self.dimensions.len(),
                memo_size
            )
        }
    }

    impl IntoIterator for MatrixChainMtPerS {
        type Item = MatrixDim;
        type IntoIter = IntoIter<MatrixDim>;

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — unwrap or clone Vec from Arc
        fn into_iter(self) -> Self::IntoIter {
            match Arc::try_unwrap(self.dimensions) {
                | Ok(vec) => vec.into_iter(),
                | Err(arc) => (*arc).clone().into_iter(),
            }
        }
    }

    impl<'a> IntoIterator for &'a MatrixChainMtPerS {
        type Item = MatrixDim;
        type IntoIter = Cloned<Iter<'a, MatrixDim>>;

        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — create cloned iterator adapter over Arc<Vec>
        fn into_iter(self) -> Self::IntoIter { self.dimensions.iter().cloned() }
    }

    impl Display for MatrixDim {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — format two integers
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "{}×{}", self.rows, self.cols) }
    }

    // 12. macros
    #[macro_export]
    macro_rules! MatrixChainMtPerLit {
        (dims: [$(($r:expr, $c:expr)),* $(,)?]) => {
            $crate::Chap50::MatrixChainMtPer::MatrixChainMtPer::MatrixChainMtPerS::from_dim_pairs(
                vec![$($crate::Types::Types::Pair($r, $c)),*]
            )
        };
        () => {
            $crate::Chap50::MatrixChainMtPer::MatrixChainMtPer::MatrixChainMtPerS::new()
        };
    }
}
