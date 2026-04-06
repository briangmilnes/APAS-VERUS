//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Chapter 50: Matrix Chain Multiplication - ephemeral, multi-threaded.
//!
//! Memoized top-down DP with parallel min reduction.
//! Uses Arc<RwLock<HashMapWithViewPlus>> for the memo table.

pub mod MatrixChainMtEph {

    use std::fmt::{Debug, Display, Formatter, Result};
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

    pub struct MatrixChainMtEphDimInv {
        pub ghost expected_dims: Seq<MatrixDim>,
    }
    impl RwLockPredicate<Vec<MatrixDim>> for MatrixChainMtEphDimInv {
        open spec fn inv(self, v: Vec<MatrixDim>) -> bool {
            v@ =~= self.expected_dims
        }
    }

    pub struct MatrixChainMtEphMemoInv {
        pub ghost dims: Seq<MatrixDim>,
    }
    impl RwLockPredicate<HashMapWithViewPlus<Pair<usize, usize>, usize>> for MatrixChainMtEphMemoInv {
        open spec fn inv(self, v: HashMapWithViewPlus<Pair<usize, usize>, usize>) -> bool {
            &&& v@.dom().finite()
            &&& spec_memo_correct(self.dims, v@)
        }
    }


    pub struct MatrixChainMtEphS {
        pub dimensions: Arc<RwLock<Vec<MatrixDim>, MatrixChainMtEphDimInv>>,
        pub memo: Arc<RwLock<HashMapWithViewPlus<Pair<usize, usize>, usize>, MatrixChainMtEphMemoInv>>,
        pub ghost_dimensions: Ghost<Seq<MatrixDim>>,
    }

    pub ghost struct MatrixChainMtEphV {
        pub dimensions: Seq<MatrixDim>,
    }

    impl View for MatrixChainMtEphS {
        type V = MatrixChainMtEphV;
        open spec fn view(&self) -> Self::V {
            MatrixChainMtEphV { dimensions: self.ghost_dimensions@ }
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
    pub trait MatrixChainMtEphTrait: Sized + View<V = MatrixChainMtEphV> {
        spec fn spec_matrixchainmteph_wf(&self) -> bool;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — allocate empty Arc<RwLock> wrappers
        fn new() -> (mc: Self)
            ensures mc@.dimensions.len() == 0, mc.spec_matrixchainmteph_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — wrap dimensions in Arc<RwLock>, n = dimensions.len()
        fn from_dimensions(dimensions: Vec<MatrixDim>) -> (mc: Self)
            ensures mc@.dimensions =~= dimensions@, mc.spec_matrixchainmteph_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — convert pairs to MatrixDim vec, n = dim_pairs.len()
        fn from_dim_pairs(dim_pairs: Vec<Pair<usize, usize>>) -> (mc: Self)
            ensures mc@.dimensions.len() == dim_pairs@.len(), mc.spec_matrixchainmteph_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^3), Span O(n^3) — memoized DP over n matrices, sequential loop
        fn optimal_cost(&mut self) -> (cost: usize)
            requires
                old(self).spec_matrixchainmteph_wf(),
                spec_dims_bounded(old(self)@.dimensions),
                old(self)@.dimensions.len() > 1 ==>
                    spec_costs_fit(old(self)@.dimensions, 0, (old(self)@.dimensions.len() - 1) as int),
            ensures
                self@.dimensions =~= old(self)@.dimensions,
                self.spec_matrixchainmteph_wf(),
                cost as nat == if old(self)@.dimensions.len() <= 1 { 0 }
                    else { spec_chain_cost(old(self)@.dimensions, 0, (old(self)@.dimensions.len() - 1) as int, 0) };

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — clone Vec under read lock
        fn dimensions(&self) -> (dims: Vec<MatrixDim>)
            requires self.spec_matrixchainmteph_wf(),
            ensures dims@ =~= self@.dimensions;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — clone dims, rebuild struct with new Arc<RwLock>
        fn set_dimension(&mut self, index: usize, dim: MatrixDim)
            requires index < old(self)@.dimensions.len(), old(self).spec_matrixchainmteph_wf(),
            ensures
                self@.dimensions =~= old(self)@.dimensions.update(index as int, dim),
                self.spec_matrixchainmteph_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — clone dims, rebuild struct with new Arc<RwLock>
        fn update_dimension(&mut self, index: usize, rows: usize, cols: usize)
            requires index < old(self)@.dimensions.len(), old(self).spec_matrixchainmteph_wf(),
            ensures
                self@.dimensions =~= old(self)@.dimensions.update(
                    index as int, MatrixDim { rows, cols }),
                self.spec_matrixchainmteph_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — read Vec length under read lock
        fn num_matrices(&self) -> (n: usize)
            requires self.spec_matrixchainmteph_wf(),
            ensures n == self@.dimensions.len();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — allocate new empty RwLock for memo
        fn clear_memo(&mut self)
            requires old(self).spec_matrixchainmteph_wf(),
            ensures self@.dimensions =~= old(self)@.dimensions, self.spec_matrixchainmteph_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — read memo length under read lock
        fn memo_size(&self) -> (n: usize);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — three array lookups and two multiplications
        fn multiply_cost(&self, i: usize, k: usize, j: usize) -> (cost: usize)
            requires
                self.spec_matrixchainmteph_wf(),
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
                self.spec_matrixchainmteph_wf(),
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

    // 9. impls

    impl MatrixChainMtEphTrait for MatrixChainMtEphS {
        open spec fn spec_matrixchainmteph_wf(&self) -> bool {
            &&& self.dimensions.pred().expected_dims =~= self.ghost_dimensions@
            &&& self.memo.pred().dims =~= self.ghost_dimensions@
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — allocate empty Arc<RwLock> wrappers
        fn new() -> (mc: Self) {
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            Self {
                dimensions: new_arc_rwlock(Vec::new(), Ghost(MatrixChainMtEphDimInv { expected_dims: Seq::empty() })),
                memo: new_arc_rwlock(HashMapWithViewPlus::new(), Ghost(MatrixChainMtEphMemoInv { dims: Seq::empty() })),
                ghost_dimensions: Ghost(Seq::empty()),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — wrap dimensions in Arc<RwLock>, n = dimensions.len()
        fn from_dimensions(dimensions: Vec<MatrixDim>) -> (mc: Self) {
            let ghost gd = dimensions@;
            let _len = dimensions.len();
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            Self {
                dimensions: new_arc_rwlock(dimensions, Ghost(MatrixChainMtEphDimInv { expected_dims: gd })),
                memo: new_arc_rwlock(HashMapWithViewPlus::new(), Ghost(MatrixChainMtEphMemoInv { dims: gd })),
                ghost_dimensions: Ghost(gd),
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
            }
            let ghost gd = dimensions@;
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            Self {
                dimensions: new_arc_rwlock(dimensions, Ghost(MatrixChainMtEphDimInv { expected_dims: gd })),
                memo: new_arc_rwlock(HashMapWithViewPlus::new(), Ghost(MatrixChainMtEphMemoInv { dims: gd })),
                ghost_dimensions: Ghost(gd),
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — three array lookups and two multiplications under read lock
        fn multiply_cost(&self, i: usize, k: usize, j: usize) -> (cost: usize) {
            let rwlock = arc_deref(&self.dimensions);
            let handle = rwlock.acquire_read();
            let dims = handle.borrow();
            assert(dims@ =~= self.ghost_dimensions@);
            let left_rows = dims[i].rows;
            let split_cols = dims[k].cols;
            let right_cols = dims[j].cols;
            assert(dims@ =~= self@.dimensions);
            assert(left_rows == self@.dimensions[i as int].rows);
            assert(split_cols == self@.dimensions[k as int].cols);
            assert(right_cols == self@.dimensions[j as int].cols);
            handle.release_read();
            left_rows * split_cols * right_cols
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
                assert(rwlock.pred().dims =~= self@.dimensions);
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
                assert(rwlock.pred().dims =~= self@.dimensions);
                let ghost pre_insert = memo@;
                memo.insert(Pair(i, j), 0usize);
                proof {
                    assert forall|a: usize, b: usize| #[trigger] memo@.contains_key((a, b))
                    implies
                        memo@[(a, b)] as nat == spec_chain_cost(self@.dimensions, a as int, b as int, a as int)
                    by {
                        if a == i && b == j {
                        } else {
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
                    self.spec_matrixchainmteph_wf(),
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
                assert(left_cost as nat + right_cost as nat + split_cost as nat <= usize::MAX as nat);
                let total = left_cost + right_cost + split_cost;

                if total < best {
                    best = total;
                }
                k = k + 1;
            }

            // Store in memo.
            let rwlock = arc_deref(&self.memo);
            let (mut memo, wh) = rwlock.acquire_write();
            assert(rwlock.pred().dims =~= self@.dimensions);
            let ghost pre_insert = memo@;
            memo.insert(Pair(i, j), best);
            proof {
                assert forall|a: usize, b: usize| #[trigger] memo@.contains_key((a, b))
                implies
                    memo@[(a, b)] as nat == spec_chain_cost(gdims, a as int, b as int, a as int)
                by {
                    if a == i && b == j {
                    } else {
                        assert(pre_insert.contains_key((a, b)));
                    }
                };
            }
            wh.release_write(memo);
            best
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n^3), Span O(n^3) — clear memo then run memoized DP
        fn optimal_cost(&mut self) -> (cost: usize) {
            let n = self.num_matrices();
            if n <= 1 {
                return 0;
            }

            // Rebuild memo with correct dims reference.
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            self.memo = new_arc_rwlock(
                HashMapWithViewPlus::new(),
                Ghost(MatrixChainMtEphMemoInv { dims: self.ghost_dimensions@ }),
            );

            self.matrix_chain_rec(0, n - 1)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — clone Vec under read lock
        fn dimensions(&self) -> (dims: Vec<MatrixDim>) {
            let rwlock = arc_deref(&self.dimensions);
            let handle = rwlock.acquire_read();
            let borrowed = handle.borrow();
            assert(borrowed@ =~= self.ghost_dimensions@);
            let dims = borrowed.clone();
            handle.release_read();
            dims
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — clone dims, rebuild struct with new Arc<RwLock>
        fn set_dimension(&mut self, index: usize, dim: MatrixDim) {
            let rwlock = arc_deref(&self.dimensions);
            let handle = rwlock.acquire_read();
            let mut dims = handle.borrow().clone();
            handle.release_read();

            dims.set(index, dim);
            let ghost new_ghost = dims@;
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            *self = MatrixChainMtEphS {
                dimensions: new_arc_rwlock(dims, Ghost(MatrixChainMtEphDimInv { expected_dims: new_ghost })),
                memo: new_arc_rwlock(HashMapWithViewPlus::new(), Ghost(MatrixChainMtEphMemoInv { dims: new_ghost })),
                ghost_dimensions: Ghost(new_ghost),
            };
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — clone dims, rebuild struct with new Arc<RwLock>
        fn update_dimension(&mut self, index: usize, rows: usize, cols: usize) {
            let dim = MatrixDim { rows, cols };
            let rwlock = arc_deref(&self.dimensions);
            let handle = rwlock.acquire_read();
            let mut dims = handle.borrow().clone();
            handle.release_read();

            dims.set(index, dim);
            let ghost new_ghost = dims@;
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            *self = MatrixChainMtEphS {
                dimensions: new_arc_rwlock(dims, Ghost(MatrixChainMtEphDimInv { expected_dims: new_ghost })),
                memo: new_arc_rwlock(HashMapWithViewPlus::new(), Ghost(MatrixChainMtEphMemoInv { dims: new_ghost })),
                ghost_dimensions: Ghost(new_ghost),
            };
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — read Vec length under read lock
        fn num_matrices(&self) -> (n: usize) {
            let rwlock = arc_deref(&self.dimensions);
            let handle = rwlock.acquire_read();
            let dims = handle.borrow();
            assert(dims@ =~= self.ghost_dimensions@);
            let n = dims.len();
            handle.release_read();
            n
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — allocate new empty RwLock for memo
        fn clear_memo(&mut self) {
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            self.memo = new_arc_rwlock(
                HashMapWithViewPlus::new(),
                Ghost(MatrixChainMtEphMemoInv { dims: self.ghost_dimensions@ }),
            );
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

    // 11. derive impls in verus!
    impl Clone for MatrixChainMtEphS {
        fn clone(&self) -> (mc: Self)
            ensures mc@ == self@
        {
            let mc = MatrixChainMtEphS {
                dimensions: self.dimensions.clone(),
                memo: self.memo.clone(),
                ghost_dimensions: Ghost(self.ghost_dimensions@),
            };
            proof { assume(mc@ == self@); }
            mc
        }
    }

    #[cfg(verus_keep_ghost)]
    impl PartialEqSpecImpl for MatrixChainMtEphS {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl PartialEq for MatrixChainMtEphS {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            let self_rwlock = arc_deref(&self.dimensions);
            let other_rwlock = arc_deref(&other.dimensions);
            let self_handle = self_rwlock.acquire_read();
            let other_handle = other_rwlock.acquire_read();
            let equal = *self_handle.borrow() == *other_handle.borrow();
            other_handle.release_read();
            self_handle.release_read();
            proof { assume(equal == (self@ == other@)); }
            equal
        }
    }

    impl Eq for MatrixChainMtEphS {}

    } // verus!

    // 13. derive impls outside verus!
    impl Debug for MatrixChainMtEphS {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { Display::fmt(self, f) }
    }

    impl Display for MatrixChainMtEphS {
        /// - Alg Analysis: APAS (Ch50 ref): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — format two integers under read locks
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            let memo_handle = self.memo.acquire_read();
            let memo_size = memo_handle.borrow().len();
            memo_handle.release_read();
            let dim_handle = self.dimensions.acquire_read();
            let dimensions_len = dim_handle.borrow().len();
            dim_handle.release_read();
            write!(
                f,
                "MatrixChainMtEph(matrices: {dimensions_len}, memo_entries: {memo_size})"
            )
        }
    }

    impl IntoIterator for MatrixChainMtEphS {
        type Item = MatrixDim;
        type IntoIter = IntoIter<MatrixDim>;

        /// - Alg Analysis: APAS (Ch50 ref): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — clone Vec from Arc<RwLock>
        fn into_iter(self) -> Self::IntoIter {
            let handle = self.dimensions.acquire_read();
            let dims = handle.borrow().clone();
            handle.release_read();
            dims.into_iter()
        }
    }

    impl IntoIterator for &MatrixChainMtEphS {
        type Item = MatrixDim;
        type IntoIter = IntoIter<MatrixDim>;

        /// - Alg Analysis: APAS (Ch50 ref): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — clone Vec under read lock
        fn into_iter(self) -> Self::IntoIter {
            let handle = self.dimensions.acquire_read();
            let dims = handle.borrow().clone();
            handle.release_read();
            dims.into_iter()
        }
    }

    impl IntoIterator for &mut MatrixChainMtEphS {
        type Item = MatrixDim;
        type IntoIter = IntoIter<MatrixDim>;

        /// - Alg Analysis: APAS (Ch50 ref): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — clone Vec under read lock
        fn into_iter(self) -> Self::IntoIter {
            let handle = self.dimensions.acquire_read();
            let dims = handle.borrow().clone();
            handle.release_read();
            dims.into_iter()
        }
    }

    impl Display for MatrixDim {
        /// - Alg Analysis: APAS (Ch50 ref): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — format two integers
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { write!(f, "{}×{}", self.rows, self.cols) }
    }

    // 12. macros
    #[macro_export]
    macro_rules! MatrixChainMtEphLit {
        (dims: [$(($r:expr, $c:expr)),* $(,)?]) => {
            $crate::Chap50::MatrixChainMtEph::MatrixChainMtEph::MatrixChainMtEphS::from_dim_pairs(
                vec![$($crate::Types::Types::Pair($r, $c)),*]
            )
        };
        () => {
            $crate::Chap50::MatrixChainMtEph::MatrixChainMtEph::MatrixChainMtEphS::new()
        };
    }
}
