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

    // 8. traits

    pub trait AdjTableGraphStEphTrait<V: StT + Ord> {
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty()                                                     -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn from_table(table: OrderedTableStEph<V, AVLTreeSetStEph<V>>) -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn num_vertices(&self)                                         -> N;
        /// claude-4-sonet: Work Θ(|V| + |E|), Span Θ(|V| + |E|), Parallelism Θ(1)
        fn num_edges(&self)                                            -> N;
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
        /// - APAS: N/A — constructor not in cost table.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — creates empty table.
        #[verifier::external_body]
        fn empty() -> Self {
            AdjTableGraphStEph {
                adj: OrderedTableStEph::empty(),
            }
        }

        /// - APAS: N/A — constructor not in cost table.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — wraps existing table.
        #[verifier::external_body]
        fn from_table(table: OrderedTableStEph<V, AVLTreeSetStEph<V>>) -> Self { AdjTableGraphStEph { adj: table } }

        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — delegates to table size.
        #[verifier::external_body]
        fn num_vertices(&self) -> N { self.adj.size() }

        /// - APAS: (no cost stated, implied by map over edges)
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) — sequential iteration over domain + neighbor sizes.
        #[verifier::external_body]
        fn num_edges(&self) -> N {
            let domain = self.adj.domain();
            let mut count = 0;
            for i in 0..domain.size() {
                let seq = domain.to_seq();
                if i < seq.length() {
                    let v = seq.nth(i);
                    if let Some(neighbors) = self.adj.find(v) {
                        count += neighbors.size();
                    }
                }
            }
            count
        }

        /// - APAS: Work Θ(n), Span Θ(lg n) [Cost Spec 52.3, map over vertices]
        /// - Claude-Opus-4.6: Work Θ(n lg n), Span Θ(n lg n) — sequential iteration with AVL inserts.
        #[verifier::external_body]
        fn vertices(&self) -> AVLTreeSetStEph<V> {
            let domain = self.adj.domain();
            let seq = domain.to_seq();
            let mut result = AVLTreeSetStEph::empty();
            for i in 0..seq.length() {
                result.insert(seq.nth(i).clone());
            }
            result
        }

        /// - APAS: Work Θ(lg n), Span Θ(lg n) [Cost Spec 52.3]
        /// - Claude-Opus-4.6: Work Θ(lg n), Span Θ(lg n) — agrees with APAS.
        #[verifier::external_body]
        fn has_edge(&self, u: &V, v: &V) -> B {
            match self.adj.find(u) {
                | Some(neighbors) => neighbors.find(v),
                | None => false,
            }
        }

        /// - APAS: Work Θ(lg n + d(v)), Span Θ(lg n) [Cost Spec 52.3]
        /// - Claude-Opus-4.6: Work Θ(lg n), Span Θ(lg n) — agrees with APAS.
        #[verifier::external_body]
        fn out_neighbors(&self, u: &V) -> AVLTreeSetStEph<V> {
            match self.adj.find(u) {
                | Some(neighbors) => neighbors.clone(),
                | None => AVLTreeSetStEph::empty(),
            }
        }

        /// - APAS: Work Θ(lg n), Span Θ(lg n) [Cost Spec 52.3]
        /// - Claude-Opus-4.6: Work Θ(lg n), Span Θ(lg n) — agrees with APAS.
        #[verifier::external_body]
        fn out_degree(&self, u: &V) -> N { self.out_neighbors(u).size() }

        /// - APAS: Work Θ(lg n), Span Θ(lg n) [Cost Spec 52.3]
        /// - Claude-Opus-4.6: Work Θ(lg n), Span Θ(lg n) — agrees with APAS.
        #[verifier::external_body]
        fn insert_vertex(&mut self, v: V) { self.adj.insert(v, AVLTreeSetStEph::empty(), |_, new| new.clone()); }

        /// - APAS: Work Θ(lg n), Span Θ(lg n) [Cost Spec 52.3, isolated vertex]
        /// - Claude-Opus-4.6: Work Θ(n lg n), Span Θ(n lg n) — iterates all vertices to remove from neighbor sets; APAS assumes isolated.
        #[verifier::external_body]
        fn delete_vertex(&mut self, v: &V) {
            let v_clone = v.clone();
            let domain = self.adj.domain();
            let seq = domain.to_seq();
            let vertices = (0..seq.length()).map(|i| seq.nth(i).clone()).collect::<Vec<V>>();

            self.adj.delete(&v_clone);
            for u in vertices {
                if let Some(neighbors) = self.adj.find(&u) {
                    let mut neighbors = neighbors.clone();
                    neighbors.delete(&v_clone);
                    self.adj.insert(u, neighbors, |_, new| new.clone());
                }
            }
        }

        /// - APAS: Work Θ(lg n), Span Θ(lg n) [Cost Spec 52.3]
        /// - Claude-Opus-4.6: Work Θ(lg n), Span Θ(lg n) — agrees with APAS.
        #[verifier::external_body]
        fn insert_edge(&mut self, u: V, v: V) {
            let neighbors = match self.adj.find(&u) {
                | Some(ns) => {
                    let mut ns = ns.clone();
                    ns.insert(v.clone());
                    ns
                }
                | None => AVLTreeSetStEph::singleton(v.clone()),
            };
            self.adj.insert(u, neighbors, |_, new| new.clone());
            if self.adj.find(&v).is_none() {
                self.adj.insert(v, AVLTreeSetStEph::empty(), |_, new| new.clone());
            }
        }

        /// - APAS: Work Θ(lg n), Span Θ(lg n) [Cost Spec 52.3]
        /// - Claude-Opus-4.6: Work Θ(lg n), Span Θ(lg n) — agrees with APAS.
        #[verifier::external_body]
        fn delete_edge(&mut self, u: &V, v: &V) {
            if let Some(neighbors) = self.adj.find(u) {
                let mut neighbors = neighbors.clone();
                neighbors.delete(v);
                self.adj.insert(u.clone(), neighbors, |_, new| new.clone());
            }
        }
    }

    } // verus!
}
