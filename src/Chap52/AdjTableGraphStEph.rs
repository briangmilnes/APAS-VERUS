//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 52: Adjacency Table Graph representation (ephemeral, single-threaded).

pub mod AdjTableGraphStEph {

    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::AVLTreeSeqStEphTrait;
    use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::ArraySetStEphTrait;
    use crate::Chap43::OrderedTableStEph::OrderedTableStEph::*;
    use crate::Types::Types::*;

    #[derive(Clone)]
    pub struct AdjTableGraphStEph<V: StT + Ord> {
        adj: OrderedTableStEph<V, AVLTreeSetStEph<V>>,
    }

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

    impl<V: StT + Ord> AdjTableGraphStEphTrait<V> for AdjTableGraphStEph<V> {
        fn empty() -> Self {
            AdjTableGraphStEph {
                adj: OrderedTableStEph::empty(),
            }
        }

        fn from_table(table: OrderedTableStEph<V, AVLTreeSetStEph<V>>) -> Self { AdjTableGraphStEph { adj: table } }

        fn num_vertices(&self) -> N { self.adj.size() }

        fn num_edges(&self) -> N {
            let domain = self.adj.domain();
            let mut count = 0;
            for i in 0..domain.size() {
                // Get vertex from domain via to_seq
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

        fn vertices(&self) -> AVLTreeSetStEph<V> {
            let domain = self.adj.domain();
            let seq = domain.to_seq();
            let mut result = AVLTreeSetStEph::empty();
            for i in 0..seq.length() {
                result.insert(seq.nth(i).clone());
            }
            result
        }

        fn has_edge(&self, u: &V, v: &V) -> B {
            match self.adj.find(u) {
                | Some(neighbors) => neighbors.find(v),
                | None => false,
            }
        }

        fn out_neighbors(&self, u: &V) -> AVLTreeSetStEph<V> {
            match self.adj.find(u) {
                | Some(neighbors) => neighbors.clone(),
                | None => AVLTreeSetStEph::empty(),
            }
        }

        fn out_degree(&self, u: &V) -> N { self.out_neighbors(u).size() }

        fn insert_vertex(&mut self, v: V) { self.adj.insert(v, AVLTreeSetStEph::empty(), |_, new| new.clone()); }

        fn delete_vertex(&mut self, v: &V) {
            let v_clone = v.clone();
            // Get all vertices before deleting
            let domain = self.adj.domain();
            let seq = domain.to_seq();
            let vertices = (0..seq.length()).map(|i| seq.nth(i).clone()).collect::<Vec<V>>();

            self.adj.delete(&v_clone);
            // Remove v from all neighbor sets
            for u in vertices {
                if let Some(neighbors) = self.adj.find(&u) {
                    let mut neighbors = neighbors.clone();
                    neighbors.delete(&v_clone);
                    self.adj.insert(u, neighbors, |_, new| new.clone());
                }
            }
        }

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
            // Ensure v is in vertex set
            if self.adj.find(&v).is_none() {
                self.adj.insert(v, AVLTreeSetStEph::empty(), |_, new| new.clone());
            }
        }

        fn delete_edge(&mut self, u: &V, v: &V) {
            if let Some(neighbors) = self.adj.find(u) {
                let mut neighbors = neighbors.clone();
                neighbors.delete(v);
                self.adj.insert(u.clone(), neighbors, |_, new| new.clone());
            }
        }
    }
}
