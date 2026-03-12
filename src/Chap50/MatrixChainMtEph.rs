//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
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
    use crate::vstdplus::accept::accept;
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

    pub struct MatrixChainMtEphDimInv;
    impl RwLockPredicate<Vec<MatrixDim>> for MatrixChainMtEphDimInv {
        open spec fn inv(self, v: Vec<MatrixDim>) -> bool {
            v@.len() <= usize::MAX as nat
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
        fn new() -> (mc: Self)
            ensures mc@.dimensions.len() == 0;

        fn from_dimensions(dimensions: Vec<MatrixDim>) -> (mc: Self)
            ensures mc@.dimensions.len() == dimensions@.len();

        fn from_dim_pairs(dim_pairs: Vec<Pair<usize, usize>>) -> (mc: Self)
            ensures mc@.dimensions.len() == dim_pairs@.len();

        fn optimal_cost(&mut self) -> (cost: usize);

        fn dimensions(&self) -> (dims: Vec<MatrixDim>);

        fn set_dimension(&mut self, index: usize, dim: MatrixDim)
            requires index < old(self)@.dimensions.len();

        fn update_dimension(&mut self, index: usize, rows: usize, cols: usize)
            requires index < old(self)@.dimensions.len();

        fn num_matrices(&self) -> (n: usize)
            ensures n == self@.dimensions.len();

        fn clear_memo(&mut self)
            ensures self@.dimensions =~= old(self)@.dimensions;

        fn memo_size(&self) -> (n: usize);

        fn multiply_cost(&self, i: usize, k: usize, j: usize) -> (cost: usize);

        fn matrix_chain_rec(&self, i: usize, j: usize) -> (cost: usize);

        fn parallel_min_reduction(&self, costs: Vec<usize>) -> (min: usize)
            requires costs@.len() > 0,
            ensures
                costs@.contains(min),
                forall|i: int| 0 <= i < costs@.len() ==> min <= costs@[i];
    }

    // 9. impls

    impl MatrixChainMtEphTrait for MatrixChainMtEphS {
        fn new() -> (mc: Self) {
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            Self {
                dimensions: new_arc_rwlock(Vec::new(), Ghost(MatrixChainMtEphDimInv)),
                memo: new_arc_rwlock(HashMapWithViewPlus::new(), Ghost(MatrixChainMtEphMemoInv { dims: Seq::empty() })),
                ghost_dimensions: Ghost(Seq::empty()),
            }
        }

        fn from_dimensions(dimensions: Vec<MatrixDim>) -> (mc: Self) {
            let ghost gd = dimensions@;
            let _len = dimensions.len();
            proof { let _ = Pair_feq_trigger::<usize, usize>(); }
            Self {
                dimensions: new_arc_rwlock(dimensions, Ghost(MatrixChainMtEphDimInv)),
                memo: new_arc_rwlock(HashMapWithViewPlus::new(), Ghost(MatrixChainMtEphMemoInv { dims: Seq::empty() })),
                ghost_dimensions: Ghost(gd),
            }
        }

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
                dimensions: new_arc_rwlock(dimensions, Ghost(MatrixChainMtEphDimInv)),
                memo: new_arc_rwlock(HashMapWithViewPlus::new(), Ghost(MatrixChainMtEphMemoInv { dims: Seq::empty() })),
                ghost_dimensions: Ghost(gd),
            }
        }

        fn multiply_cost(&self, i: usize, k: usize, j: usize) -> (cost: usize) {
            let rwlock = arc_deref(&self.dimensions);
            let handle = rwlock.acquire_read();
            let dims = handle.borrow();
            proof { accept(i < dims@.len() && k < dims@.len() && j < dims@.len()); }
            let left_rows = dims[i].rows;
            let split_cols = dims[k].cols;
            let right_cols = dims[j].cols;
            proof { accept((left_rows as nat) * (split_cols as nat) <= usize::MAX as nat
                && (left_rows as nat) * (split_cols as nat) * (right_cols as nat) <= usize::MAX as nat); }
            handle.release_read();
            left_rows * split_cols * right_cols
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

            let f1 = move || s1.parallel_min_reduction(left_costs);
            let f2 = move || s2.parallel_min_reduction(right_costs);
            let (left_min, right_min) = join(f1, f2);

            left_min.min(right_min)
        }

        #[verifier::external_body]
        fn matrix_chain_rec(&self, i: usize, j: usize) -> (cost: usize) {
            {
                let handle = self.memo.acquire_read();
                let cached = handle.borrow().get(&Pair(i, j)).copied();
                handle.release_read();
                if let Some(cached_cost) = cached {
                    return cached_cost;
                }
            }

            let cost = if i == j {
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
                memo.insert(Pair(i, j), cost);
                write_handle.release_write(memo);
            }

            cost
        }

        fn optimal_cost(&mut self) -> (cost: usize) {
            let rwlock = arc_deref(&self.dimensions);
            let handle = rwlock.acquire_read();
            let dimensions_len = handle.borrow().len();
            handle.release_read();

            if dimensions_len <= 1 {
                return 0;
            }

            {
                let memo_arc = self.memo.clone();
                let rwlock = arc_deref(&memo_arc);
                let (mut memo, write_handle) = rwlock.acquire_write();
                memo.clear();
                write_handle.release_write(memo);
            }

            self.matrix_chain_rec(0, dimensions_len - 1)
        }

        fn dimensions(&self) -> (dims: Vec<MatrixDim>) {
            let rwlock = arc_deref(&self.dimensions);
            let handle = rwlock.acquire_read();
            let dims = handle.borrow().clone();
            handle.release_read();
            dims
        }

        fn set_dimension(&mut self, index: usize, dim: MatrixDim) {
            {
                let dims_arc = self.dimensions.clone();
                let rwlock = arc_deref(&dims_arc);
                let (mut dims, write_handle) = rwlock.acquire_write();
                proof { accept(index < dims@.len()); }
                dims.set(index, dim);
                write_handle.release_write(dims);
            }
            let memo_arc = self.memo.clone();
            let rwlock = arc_deref(&memo_arc);
            let (mut memo, write_handle) = rwlock.acquire_write();
            memo.clear();
            write_handle.release_write(memo);
        }

        fn update_dimension(&mut self, index: usize, rows: usize, cols: usize) {
            let dim = MatrixDim { rows, cols };
            {
                let dims_arc = self.dimensions.clone();
                let rwlock = arc_deref(&dims_arc);
                let (mut dims, write_handle) = rwlock.acquire_write();
                proof { accept(index < dims@.len()); }
                dims.set(index, dim);
                write_handle.release_write(dims);
            }
            let memo_arc = self.memo.clone();
            let rwlock = arc_deref(&memo_arc);
            let (mut memo, write_handle) = rwlock.acquire_write();
            memo.clear();
            write_handle.release_write(memo);
        }

        fn num_matrices(&self) -> (n: usize) {
            let rwlock = arc_deref(&self.dimensions);
            let handle = rwlock.acquire_read();
            let n = handle.borrow().len();
            handle.release_read();
            proof { accept(n == self@.dimensions.len()); }
            n
        }

        fn clear_memo(&mut self) {
            let memo_arc = self.memo.clone();
            let rwlock = arc_deref(&memo_arc);
            let (mut memo, write_handle) = rwlock.acquire_write();
            memo.clear();
            write_handle.release_write(memo);
        }

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
            proof { accept(mc@ == self@); }
            mc
        }
    }

    #[cfg(verus_keep_ghost)]
    impl PartialEqSpecImpl for MatrixChainMtEphS {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl PartialEq for MatrixChainMtEphS {
        fn eq(&self, other: &Self) -> (r: bool)
            ensures r == (self@ == other@)
        {
            let self_rwlock = arc_deref(&self.dimensions);
            let other_rwlock = arc_deref(&other.dimensions);
            let self_handle = self_rwlock.acquire_read();
            let other_handle = other_rwlock.acquire_read();
            let r = *self_handle.borrow() == *other_handle.borrow();
            other_handle.release_read();
            self_handle.release_read();
            proof { accept(r == (self@ == other@)); }
            r
        }
    }

    impl Eq for MatrixChainMtEphS {}

    } // verus!

    // 13. derive impls outside verus!
    impl Debug for MatrixChainMtEphS {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result { Display::fmt(self, f) }
    }

    impl Display for MatrixChainMtEphS {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — format two integers under read locks
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

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — clone Vec from Arc<RwLock>
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

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — clone Vec under read lock
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

        /// - APAS: Work Θ(n), Span Θ(n)
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — clone Vec under read lock
        fn into_iter(self) -> Self::IntoIter {
            let handle = self.dimensions.acquire_read();
            let dims = handle.borrow().clone();
            handle.release_read();
            dims.into_iter()
        }
    }

    impl Display for MatrixDim {
        /// - APAS: Work Θ(1), Span Θ(1)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — format two integers
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
