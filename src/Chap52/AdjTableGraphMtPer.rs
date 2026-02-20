// Copyright 2024-2025 A Conditions of Use, Privacy Policy, and Terms of Use
// SPDX-License-Identifier: Apache-2.0

//! Chapter 52: Adjacency Table Graph representation (persistent, multi-threaded with TRUE parallelism).
//! G = (V, A:) where the graph is represented as a table mapping vertices to their out-neighbor sets.

pub mod AdjTableGraphMtPer {

    use std::thread;

    use vstd::prelude::*;
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::ArraySeqStPerBaseTrait;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap41::AVLTreeSetMtPer::AVLTreeSetMtPer::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::ArraySetStEphTrait;
    use crate::Chap43::OrderedSetMtEph::OrderedSetMtEph::OrderedSetMtEphTrait;
    use crate::Chap43::OrderedTableMtPer::OrderedTableMtPer::*;
    use crate::Types::Types::*;

    verus! {

    // Table of Contents
    // 1. module (above)
    // 2. imports (above)
    // 4. type definitions
    // 5. view impls
    // 8. traits
    // 9. impls
    // 11. derive impls in verus!

    // 4. type definitions

    // This implementation requires V: Ord for BOTH keys and values because:
    // - OrderedTableMtPer is backed by BSTParaTreapMtEph<Pair<K,V>>
    // - BSTParaTreapMtEph requires elements to be MtKey (which includes Ord)
    // - This allows the table to use parallel tree operations (split, join, union)
    // - Sets (AVLTreeSetMtPer<V>) implement Ord via lexicographic ordering of elements
    // - This constraint enables efficient parallel operations on the adjacency structure
    #[derive(Clone)]
    pub struct AdjTableGraphMtPer<V: StTInMtT + Ord + 'static> {
        adj: OrderedTableMtPer<V, AVLTreeSetMtPer<V>>,
    }

    // 5. view impls

    impl<V: StTInMtT + Ord + 'static> View for AdjTableGraphMtPer<V> {
        type V = Self;
        open spec fn view(&self) -> Self::V { *self }
    }

    // 8. traits

    pub trait AdjTableGraphMtPerTrait<V: StTInMtT + Ord + 'static> {
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty()                          -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn num_vertices(&self)              -> N;
        /// claude-4-sonet: Work Θ(|V| + |E|), Span Θ(log |V| × log |E|), Parallelism Θ(|E|/log |V|)
        fn num_edges(&self)                 -> N;
        /// claude-4-sonet: Work Θ(log |V| + log |E|), Span Θ(log |V| + log |E|), Parallelism Θ(1)
        fn has_edge(&self, u: &V, v: &V)    -> B;
        /// claude-4-sonet: Work Θ(log |V|), Span Θ(log |V|), Parallelism Θ(1)
        fn out_neighbors(&self, u: &V)      -> AVLTreeSetMtPer<V>;
        /// claude-4-sonet: Work Θ(log |V|), Span Θ(log |V|), Parallelism Θ(1)
        fn out_degree(&self, u: &V)         -> N;
        /// claude-4-sonet: Work Θ(log |V|), Span Θ(log |V|), Parallelism Θ(1)
        fn insert_vertex(&self, v: V)       -> Self;
        /// claude-4-sonet: Work Θ((|V| + |E|) log |V|), Span Θ(log² |V| + log |E|), Parallelism Θ(|E|/log |V|)
        fn delete_vertex(&self, v: &V)      -> Self;
        /// claude-4-sonet: Work Θ(log |V|), Span Θ(log |V|), Parallelism Θ(1)
        fn insert_edge(&self, u: V, v: V)   -> Self;
        /// claude-4-sonet: Work Θ(log |V| + log |E|), Span Θ(log |V| + log |E|), Parallelism Θ(1)
        fn delete_edge(&self, u: &V, v: &V) -> Self;
    }

    // 9. impls

    impl<V: StTInMtT + Ord + 'static> AdjTableGraphMtPerTrait<V> for AdjTableGraphMtPer<V> {
        fn empty() -> Self {
            AdjTableGraphMtPer {
                adj: OrderedTableMtPer::empty(),
            }
        }

        fn num_vertices(&self) -> N { self.adj.size() }

        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) — sequential iteration over domain; not parallel despite Mt type.
        #[verifier::external_body]
        fn num_edges(&self) -> N {
            let domain = self.adj.domain();
            let domain_seq = domain.to_seq();
            let mut count = 0;
            for i in 0..domain.size() {
                let v = domain_seq.nth(i);
                if let Some(neighbors) = self.adj.find(v) {
                    count += neighbors.size();
                }
            }
            count
        }

        /// - APAS: Work Θ(lg n), Span Θ(lg n) [Cost Spec 52.3]
        /// - Claude-Opus-4.6: Work Θ(lg n), Span Θ(lg n) — agrees with APAS.
        #[verifier::external_body]
        fn has_edge(&self, u: &V, v: &V) -> B { self.adj.find(u).is_some_and(|neighbors| neighbors.find(v)) }

        /// - APAS: Work Θ(lg n + d(v)), Span Θ(lg n) [Cost Spec 52.3]
        /// - Claude-Opus-4.6: Work Θ(lg n), Span Θ(lg n) — agrees with APAS.
        #[verifier::external_body]
        fn out_neighbors(&self, u: &V) -> AVLTreeSetMtPer<V> {
            self.adj.find(u).unwrap_or_else(|| AVLTreeSetMtPer::empty())
        }

        fn out_degree(&self, u: &V) -> N { self.out_neighbors(u).size() }

        /// - APAS: Work Θ(lg n), Span Θ(lg n) [Cost Spec 52.3]
        /// - Claude-Opus-4.6: Work Θ(lg n), Span Θ(lg n) — agrees with APAS.
        #[verifier::external_body]
        fn insert_vertex(&self, v: V) -> Self {
            if self.adj.find(&v).is_some() {
                return self.clone();
            }
            AdjTableGraphMtPer {
                adj: self.adj.insert(v, AVLTreeSetMtPer::empty()),
            }
        }

        /// - APAS: Work Θ(lg n), Span Θ(lg n) [Cost Spec 52.3, isolated vertex]
        /// - Claude-Opus-4.6: Work Θ(n lg n), Span Θ(lg² n) — parallel computation of new neighbor sets, sequential table rebuild.
        #[verifier::external_body]
        fn delete_vertex(&self, v: &V) -> Self {
            const SEQUENTIAL_CUTOFF: usize = 64;

            let v_clone = v.clone();
            let new_adj = self.adj.delete(&v_clone);
            let domain = new_adj.domain();
            let seq = domain.to_seq();
            let len = seq.length();

            if len <= SEQUENTIAL_CUTOFF {
                let mut result_adj = new_adj;
                for i in 0..len {
                    let u = seq.nth(i);
                    if let Some(neighbors) = result_adj.find(u) {
                        let new_neighbors = neighbors.delete(&v_clone);
                        result_adj = result_adj.insert(u.clone(), new_neighbors);
                    }
                }
                return AdjTableGraphMtPer { adj: result_adj };
            }

            let mid = len / 2;
            let new_adj_left = new_adj.clone();
            let new_adj_right = new_adj.clone();
            let seq_clone = seq.clone();
            let v_clone_left = v_clone.clone();

            let left_handle = thread::spawn(move || {
                let mut updates = Vec::with_capacity(mid);
                for i in 0..mid {
                    let u = seq_clone.nth(i);
                    if let Some(neighbors) = new_adj_left.find(u) {
                        updates.push((u.clone(), neighbors.delete(&v_clone_left)));
                    }
                }
                updates
            });

            let mut right_updates = Vec::with_capacity(len - mid);
            for i in mid..len {
                let u = seq.nth(i);
                if let Some(neighbors) = new_adj_right.find(u) {
                    right_updates.push((u.clone(), neighbors.delete(&v_clone)));
                }
            }

            let left_updates = left_handle.join().unwrap();
            let mut result_adj = new_adj;
            for (u, new_neighbors) in left_updates.into_iter().chain(right_updates.into_iter()) {
                result_adj = result_adj.insert(u, new_neighbors);
            }
            AdjTableGraphMtPer { adj: result_adj }
        }

        /// - APAS: Work Θ(lg n), Span Θ(lg n) [Cost Spec 52.3]
        /// - Claude-Opus-4.6: Work Θ(lg n), Span Θ(lg n) — agrees with APAS.
        #[verifier::external_body]
        fn insert_edge(&self, u: V, v: V) -> Self {
            let mut new_adj = self.adj.clone();
            if new_adj.find(&u).is_none() {
                new_adj = new_adj.insert(u.clone(), AVLTreeSetMtPer::empty());
            }
            if new_adj.find(&v).is_none() {
                new_adj = new_adj.insert(v.clone(), AVLTreeSetMtPer::empty());
            }
            let u_neighbors = new_adj.find(&u).unwrap_or_else(|| AVLTreeSetMtPer::empty());
            let new_u_neighbors = u_neighbors.insert(v);
            AdjTableGraphMtPer {
                adj: new_adj.insert(u, new_u_neighbors),
            }
        }

        /// - APAS: Work Θ(lg n), Span Θ(lg n) [Cost Spec 52.3]
        /// - Claude-Opus-4.6: Work Θ(lg n), Span Θ(lg n) — agrees with APAS.
        #[verifier::external_body]
        fn delete_edge(&self, u: &V, v: &V) -> Self {
            if let Some(u_neighbors) = self.adj.find(u) {
                let new_u_neighbors = u_neighbors.delete(v);
                AdjTableGraphMtPer {
                    adj: self.adj.insert(u.clone(), new_u_neighbors),
                }
            } else {
                self.clone()
            }
        }
    }

    // 11. derive impls in verus!

    impl<V: StTInMtT + Ord + 'static> Default for AdjTableGraphMtPer<V> {
        fn default() -> Self { Self::empty() }
    }

    } // verus!
}
