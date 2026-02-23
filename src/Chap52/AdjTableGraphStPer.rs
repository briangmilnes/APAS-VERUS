// Copyright 2024-2025 A Conditions of Use, Privacy Policy, and Terms of Use
// SPDX-License-Identifier: Apache-2.0

//! Chapter 52: Adjacency Table Graph representation (persistent, single-threaded).
//! G = (V × V set) table - maps vertices to sets of their out-neighbors.

pub mod AdjTableGraphStPer {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    use crate::Chap43::OrderedTableStPer::OrderedTableStPer::*;
    use crate::Chap52::AdjTableGraphStEph::AdjTableGraphStEph::spec_sum_adj_sizes;
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
    // 8. traits
    // 9. impls
    // 11. derive impls in verus!

    // 4. type definitions

    #[derive(Clone)]
    pub struct AdjTableGraphStPer<V: StT + Ord> {
        adj: OrderedTableStPer<V, AVLTreeSetStPer<V>>,
    }

    // 5. view impls

    impl<V: StT + Ord> View for AdjTableGraphStPer<V> {
        type V = Self;
        open spec fn view(&self) -> Self::V { *self }
    }

    // 8. traits

    pub trait AdjTableGraphStPerTrait<V: StT + Ord> {
        spec fn spec_adj(&self) -> Map<<V as View>::V, Set<<V as View>::V>>;
        spec fn spec_num_edges(&self) -> nat;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty()                                                     -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn from_table(table: OrderedTableStPer<V, AVLTreeSetStPer<V>>) -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn num_vertices(&self)                                         -> N;
        /// claude-4-sonet: Work Θ(|V| + |E|), Span Θ(|V| + |E|), Parallelism Θ(1)
        fn num_edges(&self) -> (m: N)
            requires self.spec_num_edges() <= usize::MAX as nat
            ensures m as nat == self.spec_num_edges();
        /// claude-4-sonet: Work Θ(|V|), Span Θ(|V|), Parallelism Θ(1)
        fn vertices(&self) -> (result: AVLTreeSetStPer<V>)
            ensures result@ == self.spec_adj().dom();
        /// claude-4-sonet: Work Θ(log |V| + log |E|), Span Θ(log |V| + log |E|), Parallelism Θ(1)
        fn has_edge(&self, u: &V, v: &V) -> (found: B)
            ensures found == (self.spec_adj().dom().contains(u@) && self.spec_adj()[u@].contains(v@));
        /// claude-4-sonet: Work Θ(log |V|), Span Θ(log |V|), Parallelism Θ(1)
        fn out_neighbors(&self, u: &V) -> (result: AVLTreeSetStPer<V>)
            ensures
                self.spec_adj().dom().contains(u@) ==> result@ == self.spec_adj()[u@],
                !self.spec_adj().dom().contains(u@) ==> result@ == Set::<<V as View>::V>::empty();
        /// claude-4-sonet: Work Θ(log |V|), Span Θ(log |V|), Parallelism Θ(1)
        fn out_degree(&self, u: &V)                                    -> N;
        /// claude-4-sonet: Work Θ(log |V|), Span Θ(log |V|), Parallelism Θ(1)
        fn insert_vertex(&self, v: V) -> (result: Self)
            ensures result.spec_adj().dom().contains(v@);
        /// claude-4-sonet: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|), Parallelism Θ(1)
        fn delete_vertex(&self, v: &V) -> (result: Self)
            ensures !result.spec_adj().dom().contains(v@);
        /// claude-4-sonet: Work Θ(log |V| + log |E|), Span Θ(log |V| + log |E|), Parallelism Θ(1)
        fn insert_edge(&self, u: V, v: V) -> (result: Self)
            ensures
                result.spec_adj().dom().contains(u@),
                result.spec_adj().dom().contains(v@),
                result.spec_adj()[u@].contains(v@);
        /// claude-4-sonet: Work Θ(log |V| + log |E|), Span Θ(log |V| + log |E|), Parallelism Θ(1)
        fn delete_edge(&self, u: &V, v: &V) -> (result: Self)
            ensures
                !result.spec_adj().dom().contains(u@)
                    || !result.spec_adj()[u@].contains(v@);
    }

    // 9. impls

    impl<V: StT + Ord> AdjTableGraphStPerTrait<V> for AdjTableGraphStPer<V> {
        open spec fn spec_adj(&self) -> Map<<V as View>::V, Set<<V as View>::V>> {
            self.adj@
        }

        open spec fn spec_num_edges(&self) -> nat {
            spec_sum_adj_sizes(self.spec_adj())
        }

        fn empty() -> Self {
            AdjTableGraphStPer {
                adj: OrderedTableStPer::empty(),
            }
        }

        fn from_table(table: OrderedTableStPer<V, AVLTreeSetStPer<V>>) -> Self { AdjTableGraphStPer { adj: table } }

        fn num_vertices(&self) -> N { self.adj.size() }

        fn num_edges(&self) -> (m: N)
            requires self.spec_num_edges() <= usize::MAX as nat
            ensures m as nat == self.spec_num_edges()
        {
            let domain = self.adj.domain();
            let seq = domain.to_seq();
            let mut count: usize = 0;
            let mut i: usize = 0;
            while i < seq.length()
                invariant i <= seq.length()
                decreases seq.length() - i
            {
                let v = seq.nth(i).clone();
                if let Some(neighbors) = self.adj.find(&v) {
                    count += neighbors.size();
                }
                i += 1;
            }
            count
        }

        fn vertices(&self) -> (result: AVLTreeSetStPer<V>)
            ensures result@ == self.spec_adj().dom()
        {
            let domain_set = self.adj.domain();
            let seq = domain_set.to_seq();
            let mut vertices = AVLTreeSetStPer::empty();
            let mut i: usize = 0;
            while i < seq.length()
                invariant
                    i <= seq.length(),
                    vertices@.finite(),
                    vertices@ == seq.subrange(0, i as int).to_set(),
                decreases seq.length() - i
            {
                vertices = vertices.insert(seq.nth(i).clone());
                i += 1;
            }
            vertices
        }

        fn has_edge(&self, u: &V, v: &V) -> (found: B)
            ensures found == (self.spec_adj().dom().contains(u@) && self.spec_adj()[u@].contains(v@))
        {
            match self.adj.find(u) {
                Some(neighbors) => neighbors.find(v),
                None => false,
            }
        }

        fn out_neighbors(&self, u: &V) -> (result: AVLTreeSetStPer<V>)
            ensures
                self.spec_adj().dom().contains(u@) ==> result@ == self.spec_adj()[u@],
                !self.spec_adj().dom().contains(u@) ==> result@ == Set::<<V as View>::V>::empty(),
        {
            match self.adj.find(u) {
                Some(neighbors) => neighbors.clone(),
                None => AVLTreeSetStPer::empty(),
            }
        }

        fn out_degree(&self, u: &V) -> N { self.out_neighbors(u).size() }

        fn insert_vertex(&self, v: V) -> (result: Self)
            ensures result.spec_adj().dom().contains(v@)
        {
            let new_adj = self.adj.insert(v, AVLTreeSetStPer::empty());
            AdjTableGraphStPer { adj: new_adj }
        }

        fn delete_vertex(&self, v: &V) -> (result: Self)
            ensures !result.spec_adj().dom().contains(v@)
        {
            let v_clone = v.clone();
            let new_adj = self.adj.delete(&v_clone);
            let domain = new_adj.domain();
            let seq = domain.to_seq();
            let mut result_adj = new_adj;
            let mut i: usize = 0;
            while i < seq.length()
                invariant i <= seq.length()
                decreases seq.length() - i
            {
                let u = seq.nth(i).clone();
                if let Some(neighbors) = result_adj.find(&u) {
                    let new_neighbors = neighbors.delete(&v_clone);
                    result_adj = result_adj.insert(u, new_neighbors);
                }
                i += 1;
            }
            AdjTableGraphStPer { adj: result_adj }
        }

        fn insert_edge(&self, u: V, v: V) -> (result: Self)
            ensures
                result.spec_adj().dom().contains(u@),
                result.spec_adj().dom().contains(v@),
                result.spec_adj()[u@].contains(v@),
        {
            let neighbors = match self.adj.find(&u) {
                Some(ns) => ns.insert(v.clone()),
                None => AVLTreeSetStPer::singleton(v.clone()),
            };
            let new_adj = self.adj.insert(u, neighbors);
            let final_adj = if new_adj.find(&v).is_none() {
                new_adj.insert(v, AVLTreeSetStPer::empty())
            } else {
                new_adj
            };
            AdjTableGraphStPer { adj: final_adj }
        }

        fn delete_edge(&self, u: &V, v: &V) -> (result: Self)
            ensures
                !result.spec_adj().dom().contains(u@)
                    || !result.spec_adj()[u@].contains(v@),
        {
            match self.adj.find(u) {
                Some(neighbors) => {
                    let new_neighbors = neighbors.delete(v);
                    let new_adj = self.adj.insert(u.clone(), new_neighbors);
                    AdjTableGraphStPer { adj: new_adj }
                }
                None => self.clone(),
            }
        }
    }

    } // verus!
}
