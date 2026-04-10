//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Chapter 50: Matrix Chain Multiplication - persistent, multi-threaded.
//!
//! Memoized top-down DP with parallel min reduction.
//! Uses Arc<RwLock<HashMapWithViewPlus>> for the memo table.


//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4a. type definitions
//	Section 5a. view impls
//	Section 4b. type definitions
//	Section 4c. type definitions
//	Section 6c. spec fns
//	Section 8c. traits
//	Section 4d. type definitions
//	Section 5d. view impls
//	Section 9d. impls
//	Section 11c. top level coarse locking
//	Section 12d. derive impls in verus!
//	Section 13. macros
//	Section 14. derive impls outside verus!
//	Section 14a. derive impls outside verus!
//	Section 14b. derive impls outside verus!
//	Section 14c. derive impls outside verus!
//	Section 14d. derive impls outside verus!

//		Section 1. module

pub mod MatrixChainMtPer {


    //		Section 2. imports

    use std::fmt::{Debug, Display, Formatter, Result};
    use std::iter::Cloned;
    use std::slice::Iter;
    use std::sync::Arc;
    use std::vec::IntoIter;

    use vstd::prelude::*;
    use vstd::rwlock::*;

    use crate::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::join;
    use crate::Types::Types::*;
    use crate::vstdplus::arc_rwlock::arc_rwlock::*;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
    use crate::vstdplus::smart_ptrs::smart_ptrs::arc_deref;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

    verus! 
{

    //		Section 3. broadcast use


broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    crate::Types::Types::group_Pair_axioms,
    vstd::map::group_map_axioms,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
    vstd::seq_lib::group_to_multiset_ensures,
};

    //		Section 4a. type definitions


    #[verifier::reject_recursive_types]
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct MatrixDim {
        pub rows: usize,
        pub cols: usize,
    }

    //		Section 5a. view impls


    impl View for MatrixDim {
        type V = (nat, nat);
        open spec fn view(&self) -> (nat, nat) {
            (self.rows as nat, self.cols as nat)
        }
    }

    //		Section 4b. type definitions


    pub ghost struct MatrixChainMtPerV {
        pub dimensions: Seq<MatrixDim>,
    }

    //		Section 4c. type definitions


    pub struct MatrixChainMtPerMemoInv {
        pub ghost dims: Seq<MatrixDim>,
    }

    //		Section 6c. spec fns


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

    //		Section 8c. traits


    pub trait MatrixChainMtPerTrait: Sized + View<V = MatrixChainMtPerV> {
        spec fn spec_matrixchainmtper_wf(&self) -> bool;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — allocate empty Arc and Arc<RwLock>
        fn new() -> (mc: Self)
            ensures mc@.dimensions.len() == 0, mc.spec_matrixchainmtper_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — wrap dimensions in Arc, n = dimensions.len()
        fn from_dimensions(dimensions: Vec<MatrixDim>) -> (mc: Self)
            ensures mc@.dimensions =~= dimensions@, mc.spec_matrixchainmtper_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — convert pairs to MatrixDim vec, n = dim_pairs.len()
        fn from_dim_pairs(dim_pairs: Vec<Pair<usize, usize>>) -> (mc: Self)
            ensures mc@.dimensions.len() == dim_pairs@.len(), mc.spec_matrixchainmtper_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^3), Span O(n^3) — clear memo then run memoized DP
        fn optimal_cost(&self) -> (cost: usize)
            requires
                self.spec_matrixchainmtper_wf(),
                spec_dims_bounded(self@.dimensions),
                self@.dimensions.len() > 1 ==>
                    spec_costs_fit(self@.dimensions, 0, (self@.dimensions.len() - 1) as int),
            ensures
                cost as nat == if self@.dimensions.len() <= 1 { 0 }
                    else { spec_chain_cost(self@.dimensions, 0, (self@.dimensions.len() - 1) as int, 0) };

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — return reference to Arc field
        fn dimensions(&self) -> (dims: &Arc<Vec<MatrixDim>>)
            ensures dims@ =~= self@.dimensions;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — read Vec length through Arc deref
        fn num_matrices(&self) -> (n: usize)
            ensures n == self@.dimensions.len();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — read memo length under read lock
        fn memo_size(&self) -> (n: usize);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — three array lookups and two multiplications
        fn multiply_cost(&self, i: usize, k: usize, j: usize) -> (cost: usize)
            requires
                i < self@.dimensions.len(),
                k < self@.dimensions.len(),
                j < self@.dimensions.len(),
                (self@.dimensions[i as int].rows as nat) * (self@.dimensions[k as int].cols as nat) <= usize::MAX as nat,
                spec_multiply_cost(self@.dimensions, i as int, k as int, j as int) <= usize::MAX as nat,
            ensures
                cost as nat == spec_multiply_cost(self@.dimensions, i as int, k as int, j as int);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^3), Span O(n^3) — memoized DP, n^2 subproblems each O(n) split scan
        fn matrix_chain_rec(&self, i: usize, j: usize) -> (cost: usize)
            requires
                self.spec_matrixchainmtper_wf(),
                i <= j,
                j < self@.dimensions.len(),
                spec_dims_bounded(self@.dimensions),
                spec_costs_fit(self@.dimensions, i as int, j as int),
            ensures
                cost as nat == spec_chain_cost(self@.dimensions, i as int, j as int, i as int),
            decreases j - i;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — sequential linear scan for minimum
        fn parallel_min_reduction(&self, costs: Vec<usize>) -> (min: usize)
            requires costs@.len() > 0,
            ensures
                costs@.contains(min),
                forall|i: int| 0 <= i < costs@.len() ==> min <= costs@[i];
    }

    //		Section 4d. type definitions


    pub struct MatrixChainMtPerS {
        pub dimensions: Arc<Vec<MatrixDim>>,
        pub memo: Arc<RwLock<HashMapWithViewPlus<Pair<usize, usize>, usize>, MatrixChainMtPerMemoInv>>,
    }

    //		Section 5d. view impls


    impl View for MatrixChainMtPerS {
        type V = MatrixChainMtPerV;
        open spec fn view(&self) -> Self::V {
            MatrixChainMtPerV { dimensions: self.dimensions@ }
        }
    }

    //		Section 9d. impls


    impl MatrixChainMtPerTrait for MatrixChainMtPerS {
        open spec fn spec_matrixchainmtper_wf(&self) -> bool {
            self.memo.pred().dims =~= self@.dimensions
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — allocate empty Arc and Arc<RwLock>
        fn new() -> (mc: Self) {
            // Veracity: NEEDED proof block
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            Self {
                dimensions: Arc::new(Vec::new()),
                memo: new_arc_rwlock(HashMapWithViewPlus::new(), Ghost(MatrixChainMtPerMemoInv { dims: Seq::empty() })),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — wrap dimensions in Arc, n = dimensions.len()
        fn from_dimensions(dimensions: Vec<MatrixDim>) -> (mc: Self) {
            // Veracity: NEEDED proof block
            let ghost gd = dimensions@;
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            Self {
                dimensions: Arc::new(dimensions),
                memo: new_arc_rwlock(HashMapWithViewPlus::new(), Ghost(MatrixChainMtPerMemoInv { dims: gd })),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — convert pairs to MatrixDim vec, n = dim_pairs.len()
        fn from_dim_pairs(dim_pairs: Vec<Pair<usize, usize>>) -> (mc: Self) {
            let mut dimensions: Vec<MatrixDim> = Vec::new();
            let mut idx: usize = 0;
            while idx < dim_pairs.len()
                invariant
                    idx <= dim_pairs@.len(),
                    dimensions@.len() == idx as int,
                decreases dim_pairs@.len() - idx,
            {
                dimensions.push(MatrixDim {
                    rows: dim_pairs[idx].0,
                    cols: dim_pairs[idx].1,
                });
                idx = idx + 1;
            // Veracity: NEEDED proof block
            }
            let ghost gd = dimensions@;
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            Self {
                dimensions: Arc::new(dimensions),
                memo: new_arc_rwlock(HashMapWithViewPlus::new(), Ghost(MatrixChainMtPerMemoInv { dims: gd })),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — three array lookups and two multiplications via Arc deref
        fn multiply_cost(&self, i: usize, k: usize, j: usize) -> (cost: usize) {
            let dims = arc_deref(&self.dimensions);
            let left_rows = dims[i].rows;
            let split_cols = dims[k].cols;
            let right_cols = dims[j].cols;
            let intermediate = left_rows * split_cols;
            intermediate * right_cols
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — sequential linear scan for minimum
        fn parallel_min_reduction(&self, costs: Vec<usize>) -> (min: usize) {
            let mut best: usize = costs[0];
            let mut idx: usize = 1;
            while idx < costs.len()
                invariant
                    1 <= idx <= costs@.len(),
                    costs@.len() > 0,
                    costs@.contains(best),
                    forall|k: int| 0 <= k < idx as int ==> best <= costs@[k],
                decreases costs@.len() - idx,
            {
                if costs[idx] < best {
                    best = costs[idx];
                }
                idx = idx + 1;
            }
            best
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^3), Span O(n^3) — memoized DP, n^2 subproblems each O(n) split scan
        fn matrix_chain_rec(&self, i: usize, j: usize) -> (cost: usize)
            decreases j - i,
        {
            // Memo lookup.
            {
                let rwlock = arc_deref(&self.memo);
                let handle = rwlock.acquire_read();
// Veracity: UNNEEDED assert                 assert(rwlock.pred().dims =~= self@.dimensions);
                let found = match handle.borrow().get(&Pair(i, j)) {
                    Some(v) => Some(*v),
                    None => None,
                };
                handle.release_read();
                if let Some(cached_cost) = found {
                    return cached_cost;
                }
            }

            if i == j {
                let rwlock = arc_deref(&self.memo);
                let (mut memo, wh) = rwlock.acquire_write();
                // Veracity: NEEDED assert (speed hint)
                // Veracity: NEEDED proof block
                assert(rwlock.pred().dims =~= self@.dimensions);
                let ghost pre_insert = memo@;
                memo.insert(Pair(i, j), 0usize);
                proof {
                    // Veracity: NEEDED assert
                    assert forall|a: usize, b: usize| #[trigger] memo@.contains_key((a, b))
                    implies
                        memo@[(a, b)] as nat == spec_chain_cost(self@.dimensions, a as int, b as int, a as int)
                    by {
                        if a == i && b == j {
                        } else {
                            // Veracity: NEEDED assert (speed hint)
                            assert(pre_insert.contains_key((a, b)));
                        }
                    };
                }
                wh.release_write(memo);
                return 0;
            }

            let ghost gdims = self@.dimensions;
            let mut best: usize = usize::MAX;
            let mut k: usize = i;
            while k < j
                invariant
                    i < j,
                    i <= k <= j,
                    j < self@.dimensions.len(),
                    self@.dimensions =~= gdims,
                    spec_dims_bounded(self@.dimensions),
                    spec_costs_fit(self@.dimensions, i as int, j as int),
                    self.spec_matrixchainmtper_wf(),
                    spec_chain_cost(gdims, i as int, j as int, i as int) == (
                        if best as nat <= spec_chain_cost(gdims, i as int, j as int, k as int) {
                            best as nat
                        } else {
                            spec_chain_cost(gdims, i as int, j as int, k as int)
                        }),
                decreases j - k,
            {
                let left_cost = self.matrix_chain_rec(i, k);
                let right_cost = self.matrix_chain_rec(k + 1, j);
                let split_cost = self.multiply_cost(i, k, j);
// Veracity: UNNEEDED assert                 assert(left_cost as nat + right_cost as nat + split_cost as nat <= usize::MAX as nat);
                let total = left_cost + right_cost + split_cost;

                if total < best {
                    best = total;
                }
                k = k + 1;
            }

            // Store in memo.
            let rwlock = arc_deref(&self.memo);
            let (mut memo, wh) = rwlock.acquire_write();
            // Veracity: NEEDED proof block
            // Veracity: NEEDED assert (speed hint)
            assert(rwlock.pred().dims =~= self@.dimensions);
            let ghost pre_insert = memo@;
            memo.insert(Pair(i, j), best);
            proof {
                // Veracity: NEEDED assert
                assert forall|a: usize, b: usize| #[trigger] memo@.contains_key((a, b))
                implies
                    memo@[(a, b)] as nat == spec_chain_cost(gdims, a as int, b as int, a as int)
                by {
                    if a == i && b == j {
                    } else {
                        // Veracity: NEEDED assert (speed hint)
                        assert(pre_insert.contains_key((a, b)));
                    }
                };
            }
            wh.release_write(memo);
            best
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^3), Span O(n^3) — clear memo then run memoized DP
        fn optimal_cost(&self) -> (cost: usize) {
            let dims = arc_deref(&self.dimensions);
            if dims.len() <= 1 {
                return 0;
            }

            {
                let rwlock = arc_deref(&self.memo);
                let (mut memo, write_handle) = rwlock.acquire_write();
                memo.clear();
                write_handle.release_write(memo);
            }

            let n = dims.len();
            self.matrix_chain_rec(0, n - 1)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — return reference to Arc field
        fn dimensions(&self) -> (dims: &Arc<Vec<MatrixDim>>) { &self.dimensions }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — read Vec length through Arc deref
        fn num_matrices(&self) -> (n: usize) {
            let dims = arc_deref(&self.dimensions);
            dims.len()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — read memo length under read lock
        fn memo_size(&self) -> (n: usize) {
            let rwlock = arc_deref(&self.memo);
            let handle = rwlock.acquire_read();
            let n = handle.borrow().len();
            handle.release_read();
            n
        }
    }

    //		Section 11c. top level coarse locking


    impl RwLockPredicate<HashMapWithViewPlus<Pair<usize, usize>, usize>> for MatrixChainMtPerMemoInv {
        open spec fn inv(self, v: HashMapWithViewPlus<Pair<usize, usize>, usize>) -> bool {
            &&& v@.dom().finite()
            &&& spec_memo_correct(self.dims, v@)
        }
    }

    //		Section 12d. derive impls in verus!


    impl Clone for MatrixChainMtPerS {
        fn clone(&self) -> (mc: Self)
            ensures mc@ == self@
        // Veracity: NEEDED proof block
        {
            let mc = MatrixChainMtPerS {
                dimensions: self.dimensions.clone(),
                memo: self.memo.clone(),
            };
            proof { assume(mc@ == self@); }
            mc
        }
    }

    #[cfg(verus_keep_ghost)]
    impl PartialEqSpecImpl for MatrixChainMtPerS {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl PartialEq for MatrixChainMtPerS {
        // Veracity: NEEDED proof block
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            let self_dims = arc_deref(&self.dimensions);
            let other_dims = arc_deref(&other.dimensions);
            let equal = *self_dims == *other_dims;
            proof { assume(equal == (self@ == other@)); }
            equal
        }
    }

    impl Eq for MatrixChainMtPerS {}

    } // verus!

    //		Section 13. macros


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

    //		Section 14. derive impls outside verus!

    impl<'a> IntoIterator for &'a MatrixChainMtPerS {
        type Item = MatrixDim;
        type IntoIter = Cloned<Iter<'a, MatrixDim>>;

        /// - Alg Analysis: APAS (Ch50 ref): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — create cloned iterator adapter over Arc<Vec>
        fn into_iter(self) -> Self::IntoIter { self.dimensions.iter().cloned() }
    }

    //		Section 14a. derive impls outside verus!

    impl Display for MatrixDim {
        /// - Alg Analysis: APAS (Ch50 ref): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — format two integers
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "{}×{}", self.rows, self.cols) }
    }

    //		Section 14b. derive impls outside verus!

    impl Debug for MatrixChainMtPerV {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "MatrixChainMtPerV") }
    }

    impl Display for MatrixChainMtPerV {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "MatrixChainMtPerV") }
    }

    //		Section 14c. derive impls outside verus!

    impl Debug for MatrixChainMtPerMemoInv {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "MatrixChainMtPerMemoInv") }
    }

    impl Display for MatrixChainMtPerMemoInv {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "MatrixChainMtPerMemoInv") }
    }

    //		Section 14d. derive impls outside verus!

    impl Debug for MatrixChainMtPerS {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { Display::fmt(self, f) }
    }

    impl Display for MatrixChainMtPerS {
        /// - Alg Analysis: APAS (Ch50 ref): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — format two integers
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

        /// - Alg Analysis: APAS (Ch50 ref): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — unwrap or clone Vec from Arc
        fn into_iter(self) -> Self::IntoIter {
            match Arc::try_unwrap(self.dimensions) {
                | Ok(vec) => vec.into_iter(),
                | Err(arc) => (*arc).clone().into_iter(),
            }
        }
    }
}
