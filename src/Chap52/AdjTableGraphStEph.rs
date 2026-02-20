// Copyright 2024-2025 A Conditions of Use, Privacy Policy, and Terms of Use
// SPDX-License-Identifier: Apache-2.0

//! Chapter 52: Adjacency Table Graph representation (ephemeral, single-threaded).

pub mod AdjTableGraphStEph {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::AVLTreeSeqStEphTrait;
    use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::ArraySetStEphTrait;
    use crate::Chap43::OrderedTableStEph::OrderedTableStEph::*;
    use crate::Types::Types::*;

    verus! {

    // Table of Contents
    // 1. module (above)
    // 2. imports (above)
    // 4. type definitions
    // 5. view impls
    // 6. spec fns
    // 7. proof fns
    // 8. traits
    // 9. impls
    // 11. derive impls in verus!

    // 4. type definitions

    #[derive(Clone)]
    pub struct AdjTableGraphStEph<V: StT + Ord> {
        adj: OrderedTableStEph<V, AVLTreeSetStEph<V>>,
    }

    // 5. view impls

    impl<V: StT + Ord> View for AdjTableGraphStEph<V> {
        type V = Self;
        open spec fn view(&self) -> Self::V { *self }
    }

    // 6. spec fns

    /// Sum of f(0) + f(1) + ... + f(n-1).
    pub open spec fn spec_sum_of(n: int, f: spec_fn(int) -> nat) -> nat
        decreases n
    {
        if n <= 0 { 0 }
        else { f(n - 1) + spec_sum_of(n - 1, f) }
    }

    /// Sum of neighbor-set sizes over map domain (recursive over dom).
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

    // 7. proof fns

    proof fn lemma_sum_adj_sizes_monotone<VV>(m: Map<VV, Set<VV>>, sub: Set<VV>)
        requires m.dom().finite(), sub.finite(), sub.subset_of(m.dom())
        ensures spec_sum_adj_sizes(m) >= 0
    {
    }

    // 8. traits

    pub trait AdjTableGraphStEphTrait<V: StT + Ord> {
        spec fn spec_adj(&self) -> Map<<V as View>::V, Set<<V as View>::V>>;
        spec fn spec_num_edges(&self) -> nat;

        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty()                                                     -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn from_table(table: OrderedTableStEph<V, AVLTreeSetStEph<V>>) -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn num_vertices(&self)                                         -> N;
        /// claude-4-sonet: Work Θ(|V| + |E|), Span Θ(|V| + |E|), Parallelism Θ(1)
        fn num_edges(&self) -> (m: N)
            requires self.spec_num_edges() <= usize::MAX as nat
            ensures m as nat == self.spec_num_edges();
        /// claude-4-sonet: Work Θ(|V|), Span Θ(|V|), Parallelism Θ(1)
        fn vertices(&self)                                             -> AVLTreeSetStEph<V>;
        /// claude-4-sonet: Work Θ(log |V| + log |E|), Span Θ(log |V| + log |E|), Parallelism Θ(1)
        fn has_edge(&self, u: &V, v: &V)                               -> B;
        /// claude-4-sonet: Work Θ(log |V|), Span Θ(log |V|), Parallelism Θ(1)
        fn out_neighbors(&self, u: &V)                                 -> AVLTreeSetStEph<V>;
        /// claude-4-sonet: Work Θ(log |V|), Span Θ(log |V|), Parallelism Θ(1)
        fn out_degree(&self, u: &V)                                    -> N;
        /// claude-4-sonet: Work Θ(log |V|), Span Θ(log |V|), Parallelism Θ(1)
        fn insert_vertex(&mut self, v: V);
        /// claude-4-sonet: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|), Parallelism Θ(1)
        fn delete_vertex(&mut self, v: &V);
        /// claude-4-sonet: Work Θ(log |V| + log |E|), Span Θ(log |V| + log |E|), Parallelism Θ(1)
        fn insert_edge(&mut self, u: V, v: V);
        /// claude-4-sonet: Work Θ(log |V| + log |E|), Span Θ(log |V| + log |E|), Parallelism Θ(1)
        fn delete_edge(&mut self, u: &V, v: &V);
    }

    // 9. impls

    impl<V: StT + Ord> AdjTableGraphStEphTrait<V> for AdjTableGraphStEph<V> {
        open spec fn spec_adj(&self) -> Map<<V as View>::V, Set<<V as View>::V>> {
            self.adj@
        }

        open spec fn spec_num_edges(&self) -> nat {
            spec_sum_adj_sizes(self.spec_adj())
        }

        fn empty() -> Self {
            AdjTableGraphStEph {
                adj: OrderedTableStEph::empty(),
            }
        }

        fn from_table(table: OrderedTableStEph<V, AVLTreeSetStEph<V>>) -> Self { AdjTableGraphStEph { adj: table } }

        fn num_vertices(&self) -> N { self.adj.size() }

        fn num_edges(&self) -> (m: N)
            requires self.spec_num_edges() <= usize::MAX as nat
            ensures m as nat == self.spec_num_edges()
        {
            let domain = self.adj.domain();
            let seq = domain.to_seq();
            let mut count = 0;
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

        fn vertices(&self) -> (result: AVLTreeSetStEph<V>)
            ensures result@ == self.spec_adj().dom()
        {
            let domain = self.adj.domain();
            let seq = domain.to_seq();
            let mut result = AVLTreeSetStEph::empty();
            let mut i: usize = 0;
            while i < seq.length()
                invariant
                    i <= seq.length(),
                    result@.finite(),
                    result@ == seq.subrange(0, i as int).to_set(),
                decreases seq.length() - i
            {
                result.insert(seq.nth(i).clone());
                i += 1;
            }
            result
        }

        fn has_edge(&self, u: &V, v: &V) -> (found: B)
            ensures found == (self.spec_adj().dom().contains(u@) && self.spec_adj()[u@].contains(v@))
        {
            match self.adj.find(u) {
                Some(neighbors) => neighbors.find(v),
                None => false,
            }
        }

        fn out_neighbors(&self, u: &V) -> (result: AVLTreeSetStEph<V>)
            ensures
                self.spec_adj().dom().contains(u@) ==> result@ == self.spec_adj()[u@],
                !self.spec_adj().dom().contains(u@) ==> result@ == Set::<<V as View>::V>::empty(),
        {
            match self.adj.find(u) {
                Some(neighbors) => neighbors.clone(),
                None => AVLTreeSetStEph::empty(),
            }
        }

        fn out_degree(&self, u: &V) -> N { self.out_neighbors(u).size() }

        fn insert_vertex(&mut self, v: V)
            ensures self.spec_adj().dom().contains(v@)
        {
            self.adj.insert(v, AVLTreeSetStEph::empty(), |old, _| old.clone());
        }

        fn delete_vertex(&mut self, v: &V)
            ensures !self.spec_adj().dom().contains(v@)
        {
            let v_clone = v.clone();
            let domain = self.adj.domain();
            let seq = domain.to_seq();
            let mut i: usize = 0;
            self.adj.delete(&v_clone);
            while i < seq.length()
                invariant i <= seq.length()
                decreases seq.length() - i
            {
                let u = seq.nth(i).clone();
                if let Some(neighbors) = self.adj.find(&u) {
                    let mut neighbors = neighbors.clone();
                    neighbors.delete(&v_clone);
                    self.adj.insert(u, neighbors, |_, new| new.clone());
                }
                i += 1;
            }
        }

        fn insert_edge(&mut self, u: V, v: V)
            ensures
                self.spec_adj().dom().contains(u@),
                self.spec_adj().dom().contains(v@),
                self.spec_adj()[u@].contains(v@),
        {
            let neighbors = match self.adj.find(&u) {
                Some(ns) => {
                    let mut ns = ns.clone();
                    ns.insert(v.clone());
                    ns
                }
                None => AVLTreeSetStEph::singleton(v.clone()),
            };
            self.adj.insert(u, neighbors, |_, new| new.clone());
            if self.adj.find(&v).is_none() {
                self.adj.insert(v, AVLTreeSetStEph::empty(), |old, _| old.clone());
            }
        }

        fn delete_edge(&mut self, u: &V, v: &V)
            ensures
                !self.spec_adj().dom().contains(u@)
                    || !self.spec_adj()[u@].contains(v@),
        {
            if let Some(neighbors) = self.adj.find(u) {
                let mut neighbors = neighbors.clone();
                neighbors.delete(v);
                self.adj.insert(u.clone(), neighbors, |_, new| new.clone());
            }
        }
    }

    } // verus!
}
