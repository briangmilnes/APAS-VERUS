//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 52: Adjacency Table Graph representation (persistent, single-threaded).
//! G = (V × V set) table - maps vertices to sets of their out-neighbors.

pub mod AdjTableGraphStPer {

    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    use crate::Chap43::OrderedTableStPer::OrderedTableStPer::*;
    use crate::Types::Types::*;

    #[derive(Clone)]
    pub struct AdjTableGraphStPer<V: StT + Ord> {
        adj: OrderedTableStPer<V, AVLTreeSetStPer<V>>,
    }

    pub trait AdjTableGraphStPerTrait<V: StT + Ord> {
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn empty()                                                     -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn from_table(table: OrderedTableStPer<V, AVLTreeSetStPer<V>>) -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn num_vertices(&self)                                         -> N;
        /// claude-4-sonet: Work Θ(|V| + |E|), Span Θ(|V| + |E|), Parallelism Θ(1)
        fn num_edges(&self)                                            -> N;
        /// claude-4-sonet: Work Θ(|V|), Span Θ(|V|), Parallelism Θ(1)
        fn vertices(&self)                                             -> AVLTreeSetStPer<V>;
        /// claude-4-sonet: Work Θ(log |V| + log |E|), Span Θ(log |V| + log |E|), Parallelism Θ(1)
        fn has_edge(&self, u: &V, v: &V)                               -> B;
        /// claude-4-sonet: Work Θ(log |V|), Span Θ(log |V|), Parallelism Θ(1)
        fn out_neighbors(&self, u: &V)                                 -> AVLTreeSetStPer<V>;
        /// claude-4-sonet: Work Θ(log |V|), Span Θ(log |V|), Parallelism Θ(1)
        fn out_degree(&self, u: &V)                                    -> N;
        /// claude-4-sonet: Work Θ(log |V|), Span Θ(log |V|), Parallelism Θ(1)
        fn insert_vertex(&self, v: V)                                  -> Self;
        /// claude-4-sonet: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|), Parallelism Θ(1)
        fn delete_vertex(&self, v: &V)                                 -> Self;
        /// claude-4-sonet: Work Θ(log |V| + log |E|), Span Θ(log |V| + log |E|), Parallelism Θ(1)
        fn insert_edge(&self, u: V, v: V)                              -> Self;
        /// claude-4-sonet: Work Θ(log |V| + log |E|), Span Θ(log |V| + log |E|), Parallelism Θ(1)
        fn delete_edge(&self, u: &V, v: &V)                            -> Self;
    }

    impl<V: StT + Ord> AdjTableGraphStPerTrait<V> for AdjTableGraphStPer<V> {
        /// - APAS: N/A — constructor not in cost table.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — creates empty table.
        fn empty() -> Self {
            AdjTableGraphStPer {
                adj: OrderedTableStPer::empty(),
            }
        }

        /// - APAS: N/A — constructor not in cost table.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — wraps existing table.
        fn from_table(table: OrderedTableStPer<V, AVLTreeSetStPer<V>>) -> Self { AdjTableGraphStPer { adj: table } }

        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — delegates to table size.
        fn num_vertices(&self) -> N { self.adj.size() }

        /// - APAS: (no cost stated, implied by map over edges: Work Θ(m), Span Θ(lg n))
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) — sequential iteration over domain + neighbor sizes.
        fn num_edges(&self) -> N {
            let domain = self.adj.domain();
            let seq = domain.to_seq();
            let mut count = 0;
            for i in 0..seq.length() {
                let v = seq.nth(i);
                if let Some(neighbors) = self.adj.find(v) {
                    count += neighbors.size();
                }
            }
            count
        }

        /// - APAS: Work Θ(n), Span Θ(lg n) [Cost Spec 52.3, map over vertices]
        /// - Claude-Opus-4.6: Work Θ(n lg n), Span Θ(n lg n) — sequential iteration with AVL inserts.
        fn vertices(&self) -> AVLTreeSetStPer<V> {
            let domain_set = self.adj.domain();
            let seq = domain_set.to_seq();
            let mut vertices = AVLTreeSetStPer::empty();
            for i in 0..seq.length() {
                vertices = vertices.insert(seq.nth(i).clone());
            }
            vertices
        }

        /// - APAS: Work Θ(lg n), Span Θ(lg n) [Cost Spec 52.3]
        /// - Claude-Opus-4.6: Work Θ(lg n), Span Θ(lg n) — agrees with APAS.
        fn has_edge(&self, u: &V, v: &V) -> B {
            match self.adj.find(u) {
                | Some(neighbors) => neighbors.find(v),
                | None => false,
            }
        }

        /// - APAS: Work Θ(lg n + d(v)), Span Θ(lg n) [Cost Spec 52.3]
        /// - Claude-Opus-4.6: Work Θ(lg n), Span Θ(lg n) — table lookup + clone; agrees with APAS.
        fn out_neighbors(&self, u: &V) -> AVLTreeSetStPer<V> {
            match self.adj.find(u) {
                | Some(neighbors) => neighbors.clone(),
                | None => AVLTreeSetStPer::empty(),
            }
        }

        /// - APAS: Work Θ(lg n), Span Θ(lg n) [Cost Spec 52.3]
        /// - Claude-Opus-4.6: Work Θ(lg n), Span Θ(lg n) — agrees with APAS.
        fn out_degree(&self, u: &V) -> N { self.out_neighbors(u).size() }

        /// - APAS: Work Θ(lg n), Span Θ(lg n) [Cost Spec 52.3]
        /// - Claude-Opus-4.6: Work Θ(lg n), Span Θ(lg n) — agrees with APAS.
        fn insert_vertex(&self, v: V) -> Self {
            let new_adj = self.adj.insert(v, AVLTreeSetStPer::empty());
            AdjTableGraphStPer { adj: new_adj }
        }

        /// - APAS: Work Θ(lg n), Span Θ(lg n) [Cost Spec 52.3, isolated vertex]
        /// - Claude-Opus-4.6: Work Θ(n lg n), Span Θ(n lg n) — iterates all vertices to remove from neighbor sets; APAS assumes isolated.
        fn delete_vertex(&self, v: &V) -> Self {
            let v_clone = v.clone();
            let new_adj = self.adj.delete(&v_clone);
            // Remove v from all neighbor sets
            let domain = new_adj.domain();
            let seq = domain.to_seq();
            let mut result_adj = new_adj;
            for i in 0..seq.length() {
                let u = seq.nth(i);
                if let Some(neighbors) = result_adj.find(u) {
                    let new_neighbors = neighbors.delete(&v_clone);
                    result_adj = result_adj.insert(u.clone(), new_neighbors);
                }
            }
            AdjTableGraphStPer { adj: result_adj }
        }

        /// - APAS: Work Θ(lg n), Span Θ(lg n) [Cost Spec 52.3]
        /// - Claude-Opus-4.6: Work Θ(lg n), Span Θ(lg n) — agrees with APAS.
        fn insert_edge(&self, u: V, v: V) -> Self {
            let neighbors = match self.adj.find(&u) {
                | Some(ns) => ns.insert(v.clone()),
                | None => AVLTreeSetStPer::singleton(v.clone()),
            };
            let new_adj = self.adj.insert(u, neighbors);
            // Ensure v is in vertex set
            let final_adj = if new_adj.find(&v).is_none() {
                new_adj.insert(v, AVLTreeSetStPer::empty())
            } else {
                new_adj
            };
            AdjTableGraphStPer { adj: final_adj }
        }

        /// - APAS: Work Θ(lg n), Span Θ(lg n) [Cost Spec 52.3]
        /// - Claude-Opus-4.6: Work Θ(lg n), Span Θ(lg n) — agrees with APAS.
        fn delete_edge(&self, u: &V, v: &V) -> Self {
            match self.adj.find(u) {
                | Some(neighbors) => {
                    let new_neighbors = neighbors.delete(v);
                    let new_adj = self.adj.insert(u.clone(), new_neighbors);
                    AdjTableGraphStPer { adj: new_adj }
                }
                | None => self.clone(),
            }
        }
    }
}
