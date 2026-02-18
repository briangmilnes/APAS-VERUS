//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 52: Adjacency Sequence Graph (ephemeral, single-threaded).

pub mod AdjSeqGraphStEph {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Types::Types::*;

    verus! {

    // Table of Contents
    // 4. type definitions
    // 5. view impls
    // 8. traits
    // 9. impls

    // 4. type definitions

    #[derive(Clone)]
    pub struct AdjSeqGraphStEph {
        pub adj: ArraySeqStEphS<ArraySeqStEphS<N>>,
    }

    // 5. view impls

    impl View for AdjSeqGraphStEph {
        type V = Seq<Seq<int>>;
        open spec fn view(&self) -> Self::V {
            Seq::new(self.adj.spec_len(), |i: int|
                Seq::new(self.adj.spec_index(i).spec_len(), |j: int|
                    self.adj.spec_index(i).spec_index(j) as int
                )
            )
        }
    }

    // 8. traits

    pub trait AdjSeqGraphStEphTrait: Sized {
        /// claude-4-sonet: Work Θ(n), Span Θ(n), Parallelism Θ(1)
        fn new(n: N)                                        -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn from_seq(adj: ArraySeqStEphS<ArraySeqStEphS<N>>) -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn num_vertices(&self)                              -> N;
        /// claude-4-sonet: Work Θ(Σ deg(v)), Span Θ(Σ deg(v)), Parallelism Θ(1)
        fn num_edges(&self)                                 -> N;
        /// claude-4-sonet: Work Θ(deg(u)), Span Θ(deg(u)), Parallelism Θ(1)
        fn has_edge(&self, u: N, v: N)                      -> B;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn out_neighbors(&self, u: N)                       -> ArraySeqStEphS<N>;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn out_degree(&self, u: N)                          -> N;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn set_neighbors(&mut self, v: N, neighbors: ArraySeqStEphS<N>);
        /// claude-4-sonet: Work Θ(deg(u)), Span Θ(deg(u)), Parallelism Θ(1)
        fn set_edge(&mut self, u: N, v: N, exists: B);
    }

    // 9. impls

    impl AdjSeqGraphStEphTrait for AdjSeqGraphStEph {
        /// - APAS: N/A — constructor not in cost table.
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — sequential loop creating n empty neighbor lists.
        #[verifier::external_body]
        fn new(n: N) -> Self {
            let empty_list = ArraySeqStEphS::empty();
            let mut adj_lists = Vec::with_capacity(n);
            for _ in 0..n {
                adj_lists.push(empty_list.clone());
            }
            AdjSeqGraphStEph {
                adj: ArraySeqStEphS::from_vec(adj_lists),
            }
        }

        /// - APAS: N/A — constructor not in cost table.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — wraps existing sequence.
        #[verifier::external_body]
        fn from_seq(adj: ArraySeqStEphS<ArraySeqStEphS<N>>) -> Self { AdjSeqGraphStEph { adj } }

        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — sequence length.
        #[verifier::external_body]
        fn num_vertices(&self) -> N { self.adj.length() }

        /// - APAS: Work Θ(n + m), Span Θ(1) [Cost Spec 52.5, map over edges]
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) — sequential loop; span not parallel.
        #[verifier::external_body]
        fn num_edges(&self) -> N {
            let n = self.adj.length();
            let mut count = 0;
            for i in 0..n {
                count += self.adj.nth(i).length();
            }
            count
        }

        /// - APAS: Work Θ(d(u)), Span Θ(lg d(u)) [Cost Spec 52.5]
        /// - Claude-Opus-4.6: Work Θ(d(u)), Span Θ(d(u)) — sequential linear scan; span not logarithmic.
        #[verifier::external_body]
        fn has_edge(&self, u: N, v: N) -> B {
            if u >= self.adj.length() {
                return false;
            }
            let neighbors = self.adj.nth(u);
            for i in 0..neighbors.length() {
                if *neighbors.nth(i) == v {
                    return true;
                }
            }
            false
        }

        /// - APAS: Work Θ(1), Span Θ(1) [Cost Spec 52.5]
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS (clone is O(1) for persistent backing).
        #[verifier::external_body]
        fn out_neighbors(&self, u: N) -> ArraySeqStEphS<N> { self.adj.nth(u).clone() }

        /// - APAS: Work Θ(1), Span Θ(1) [Cost Spec 52.5]
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        #[verifier::external_body]
        fn out_degree(&self, u: N) -> N { self.adj.nth(u).length() }

        /// - APAS: N/A — Verus-specific scaffolding.
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — single array set.
        #[verifier::external_body]
        fn set_neighbors(&mut self, v: N, neighbors: ArraySeqStEphS<N>) {
            if v < self.adj.length() {
                let _ = self.adj.set(v, neighbors);
            }
        }

        /// - APAS: Work Θ(n), Span Θ(1) [Cost Spec 52.5, insert/delete edge]
        /// - Claude-Opus-4.6: Work Θ(d(u)), Span Θ(d(u)) — linear scan + rebuild of neighbor list; O(1) array set.
        #[verifier::external_body]
        fn set_edge(&mut self, u: N, v: N, exists: B) {
            if u >= self.adj.length() || v >= self.adj.length() {
                return;
            }
            let old_neighbors = self.adj.nth(u);
            if exists {
                let mut found = false;
                for i in 0..old_neighbors.length() {
                    if *old_neighbors.nth(i) == v {
                        found = true;
                        break;
                    }
                }
                if !found {
                    let mut new_neighbors_vec = Vec::<N>::with_capacity(old_neighbors.length() + 1);
                    for i in 0..old_neighbors.length() {
                        new_neighbors_vec.push(*old_neighbors.nth(i));
                    }
                    new_neighbors_vec.push(v);
                    let _ = self.adj.set(u, ArraySeqStEphS::from_vec(new_neighbors_vec));
                }
            } else {
                let mut new_neighbors_vec = Vec::<N>::new();
                for i in 0..old_neighbors.length() {
                    let neighbor = *old_neighbors.nth(i);
                    if neighbor != v {
                        new_neighbors_vec.push(neighbor);
                    }
                }
                let _ = self.adj.set(u, ArraySeqStEphS::from_vec(new_neighbors_vec));
            }
        }
    }

    } // verus!
}
