// Copyright 2024-2025 A Conditions of Use, Privacy Policy, and Terms of Use
// SPDX-License-Identifier: Apache-2.0

//! Chapter 52: Adjacency Table Graph representation (persistent, multi-threaded with TRUE parallelism).
//! G = (V, A:) where the graph is represented as a table mapping vertices to their out-neighbor sets.

pub mod AdjTableGraphMtPer {

    use vstd::prelude::*;
    use crate::Chap41::AVLTreeSetMtPer::AVLTreeSetMtPer::*;
    use crate::Chap43::OrderedSetMtEph::OrderedSetMtEph::OrderedSetMtEphTrait;
    use crate::Chap43::OrderedTableMtPer::OrderedTableMtPer::*;
    use crate::Types::Types::*;

    verus! {

// Veracity: added broadcast group
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::map::group_map_axioms,
    vstd::set::group_set_axioms,
    vstd::set_lib::group_set_lib_default,
};

    // Table of Contents
    // 1. module (above)
    // 2. imports (above)
    // 4. type definitions
    // 5. view impls
    // 6. spec fns
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

    // 6. spec fns

    /// Sum of all neighbor set sizes across all vertices in the adjacency map.
    /// Local copy — standalone rule forbids importing from StEph.
    pub open spec fn spec_sum_adj_sizes<VV>(m: Map<VV, Set<VV>>) -> nat
        decreases m.dom().len()
    {
        if m.dom().is_empty() {
            0
        } else {
            let k = m.dom().choose();
            m[k].len() + spec_sum_adj_sizes(m.remove(k))
        }
    }

    // 8. traits

    pub trait AdjTableGraphMtPerTrait<V: StTInMtT + Ord + 'static> {
        spec fn spec_adj(&self) -> Map<<V as View>::V, Set<<V as View>::V>>;
        spec fn spec_num_edges(&self) -> nat;

        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty()                          -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn num_vertices(&self)              -> N;
        /// claude-4-sonet: Work Θ(|V| + |E|), Span Θ(log |V| × log |E|), Parallelism Θ(|E|/log |V|)
        fn num_edges(&self) -> (m: N)
            requires self.spec_num_edges() <= usize::MAX as nat
            ensures m as nat == self.spec_num_edges();
        /// claude-4-sonet: Work Θ(log |V| + log |E|), Span Θ(log |V| + log |E|), Parallelism Θ(1)
        fn has_edge(&self, u: &V, v: &V) -> (found: B)
            ensures found == (self.spec_adj().dom().contains(u@) && self.spec_adj()[u@].contains(v@));
        /// claude-4-sonet: Work Θ(log |V|), Span Θ(log |V|), Parallelism Θ(1)
        fn out_neighbors(&self, u: &V) -> (neighbors: AVLTreeSetMtPer<V>)
            ensures
                self.spec_adj().dom().contains(u@) ==> neighbors@ == self.spec_adj()[u@],
                !self.spec_adj().dom().contains(u@) ==> neighbors@ == Set::<<V as View>::V>::empty();
        /// claude-4-sonet: Work Θ(log |V|), Span Θ(log |V|), Parallelism Θ(1)
        fn out_degree(&self, u: &V)         -> N;
        /// claude-4-sonet: Work Θ(log |V|), Span Θ(log |V|), Parallelism Θ(1)
        fn insert_vertex(&self, v: V) -> (updated: Self)
            ensures updated.spec_adj().dom().contains(v@);
        /// claude-4-sonet: Work Θ((|V| + |E|) log |V|), Span Θ(log² |V| + log |E|), Parallelism Θ(|E|/log |V|)
        fn delete_vertex(&self, v: &V) -> (updated: Self)
            ensures !updated.spec_adj().dom().contains(v@);
        /// claude-4-sonet: Work Θ(log |V|), Span Θ(log |V|), Parallelism Θ(1)
        fn insert_edge(&self, u: V, v: V) -> (updated: Self)
            ensures
                updated.spec_adj().dom().contains(u@),
                updated.spec_adj().dom().contains(v@),
                updated.spec_adj()[u@].contains(v@);
        /// claude-4-sonet: Work Θ(log |V| + log |E|), Span Θ(log |V| + log |E|), Parallelism Θ(1)
        fn delete_edge(&self, u: &V, v: &V) -> (updated: Self)
            ensures
                !updated.spec_adj().dom().contains(u@)
                    || !updated.spec_adj()[u@].contains(v@);
    }

    // 9. impls

    impl<V: StTInMtT + Ord + 'static> AdjTableGraphMtPerTrait<V> for AdjTableGraphMtPer<V> {
        open spec fn spec_adj(&self) -> Map<<V as View>::V, Set<<V as View>::V>> {
            self.adj@
        }

        open spec fn spec_num_edges(&self) -> nat {
            spec_sum_adj_sizes(self.spec_adj())
        }

        fn empty() -> Self {
            AdjTableGraphMtPer {
                adj: OrderedTableMtPer::empty(),
            }
        }

        fn num_vertices(&self) -> N { self.adj.size() }

        fn num_edges(&self) -> (m: N)
            requires self.spec_num_edges() <= usize::MAX as nat
            ensures m as nat == self.spec_num_edges()
        {
            let domain = self.adj.domain();
            let domain_seq = domain.to_seq();
            let mut count: usize = 0;
            let mut i: usize = 0;
            while i < domain.size()
                invariant i <= domain.size()
                decreases domain.size() - i
            {
                let v = domain_seq.nth(i).clone();
                if let Some(neighbors) = self.adj.find(&v) {
                    count += neighbors.size();
                }
                i += 1;
            }
            count
        }

        fn has_edge(&self, u: &V, v: &V) -> (found: B)
            ensures found == (self.spec_adj().dom().contains(u@) && self.spec_adj()[u@].contains(v@))
        {
            match self.adj.find(u) {
                Some(neighbors) => neighbors.find(v),
                None => false,
            }
        }

        fn out_neighbors(&self, u: &V) -> (neighbors: AVLTreeSetMtPer<V>)
            ensures
                self.spec_adj().dom().contains(u@) ==> neighbors@ == self.spec_adj()[u@],
                !self.spec_adj().dom().contains(u@) ==> neighbors@ == Set::<<V as View>::V>::empty(),
        {
            match self.adj.find(u) {
                Some(neighbors) => neighbors.clone(),
                None => AVLTreeSetMtPer::empty(),
            }
        }

        fn out_degree(&self, u: &V) -> N { self.out_neighbors(u).size() }

        fn insert_vertex(&self, v: V) -> (updated: Self)
            ensures updated.spec_adj().dom().contains(v@)
        {
            if self.adj.find(&v).is_some() {
                self.clone()
            } else {
                AdjTableGraphMtPer {
                    adj: self.adj.insert(v, AVLTreeSetMtPer::empty()),
                }
            }
        }

        /// - APAS: Work Θ(lg n), Span Θ(lg n) [Cost Spec 52.3, isolated vertex]
        /// - Work Θ(|V| lg |V|), Span Θ(lg² |V|) — map is parallel via treap.
        fn delete_vertex(&self, v: &V) -> (updated: Self)
            ensures !updated.spec_adj().dom().contains(v@)
        {
            let without_v = self.adj.delete(v);
            let v_clone = v.clone();
            let cleaned = without_v.map(move |_k: &V, neighbors: &AVLTreeSetMtPer<V>| {
                neighbors.delete(&v_clone)
            });
            AdjTableGraphMtPer { adj: cleaned }
        }

        fn insert_edge(&self, u: V, v: V) -> (updated: Self)
            ensures
                updated.spec_adj().dom().contains(u@),
                updated.spec_adj().dom().contains(v@),
                updated.spec_adj()[u@].contains(v@),
        {
            let mut new_adj = self.adj.clone();
            if new_adj.find(&u).is_none() {
                new_adj = new_adj.insert(u.clone(), AVLTreeSetMtPer::empty());
            }
            if new_adj.find(&v).is_none() {
                new_adj = new_adj.insert(v.clone(), AVLTreeSetMtPer::empty());
            }
            let u_neighbors = match new_adj.find(&u) {
                Some(ns) => ns,
                None => AVLTreeSetMtPer::empty(),
            };
            let new_u_neighbors = u_neighbors.insert(v);
            AdjTableGraphMtPer {
                adj: new_adj.insert(u, new_u_neighbors),
            }
        }

        fn delete_edge(&self, u: &V, v: &V) -> (updated: Self)
            ensures
                !updated.spec_adj().dom().contains(u@)
                    || !updated.spec_adj()[u@].contains(v@),
        {
            match self.adj.find(u) {
                Some(u_neighbors) => {
                    let new_u_neighbors = u_neighbors.delete(v);
                    AdjTableGraphMtPer {
                        adj: self.adj.insert(u.clone(), new_u_neighbors),
                    }
                }
                None => self.clone(),
            }
        }
    }

    // 11. derive impls in verus!

    impl<V: StTInMtT + Ord + 'static> Default for AdjTableGraphMtPer<V> {
        fn default() -> Self { Self::empty() }
    }

    } // verus!
}
