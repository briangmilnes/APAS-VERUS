//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 52: Adjacency Sequence Graph (ephemeral, multi-threaded).

pub mod AdjSeqGraphMtEph {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Types::Types::*;

    verus! {

    // Table of Contents
    // 4. type definitions
    // 5. view impls
    // 8. traits
    // 9. impls

    // 4. type definitions

    #[derive(Clone)]
    pub struct AdjSeqGraphMtEph {
        pub adj: ArraySeqMtEphS<ArraySeqMtEphS<N>>,
    }

    // 5. view impls

    impl View for AdjSeqGraphMtEph {
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

    pub trait AdjSeqGraphMtEphTrait: Sized {
        /// claude-4-sonet: Work Θ(n), Span Θ(1)
        fn new(n: N)                   -> Self;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn num_vertices(&self)         -> N;
        /// claude-4-sonet: Work Θ(Σ deg(v)), Span Θ(n), Parallelism Θ(|E|/n)
        fn num_edges(&self)            -> N;
        /// claude-4-sonet: Work Θ(deg(u)), Span Θ(deg(u)), Parallelism Θ(1)
        fn has_edge(&self, u: N, v: N) -> B;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn out_neighbors(&self, u: N)  -> ArraySeqMtEphS<N>;
        /// claude-4-sonet: Work Θ(1), Span Θ(1)
        fn out_degree(&self, u: N)     -> N;
        /// claude-4-sonet: Work Θ(deg(u)), Span Θ(deg(u)) with locking
        fn set_edge(&mut self, u: N, v: N, exists: B);
    }

    // 9. impls

    impl AdjSeqGraphMtEphTrait for AdjSeqGraphMtEph {
        /// - APAS: N/A — constructor not in cost table.
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) — sequential loop creating n empty neighbor lists.
        #[verifier::external_body]
        fn new(n: N) -> Self {
            let empty_list = ArraySeqMtEphS::empty();
            let mut adj_lists = Vec::with_capacity(n);
            for _ in 0..n {
                adj_lists.push(empty_list.clone());
            }
            AdjSeqGraphMtEph {
                adj: ArraySeqMtEphS::from_vec(adj_lists),
            }
        }

        /// - APAS: (no cost stated)
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — sequence length.
        #[verifier::external_body]
        fn num_vertices(&self) -> N { self.adj.length() }

        /// - APAS: Work Θ(n + m), Span Θ(1) [Cost Spec 52.5, map over edges]
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) — sequential loop; span not parallel despite Mt type.
        #[verifier::external_body]
        fn num_edges(&self) -> N {
            let n = self.adj.length();
            let mut count = 0;
            for i in 0..n {
                let adj_list = self.adj.nth(i);
                count += adj_list.length();
            }
            count
        }

        /// - APAS: Work Θ(d(u)), Span Θ(lg d(u)) [Cost Spec 52.5]
        /// - Claude-Opus-4.6: Work Θ(d(u)), Span Θ(d(u)) — sequential linear scan.
        #[verifier::external_body]
        fn has_edge(&self, u: N, v: N) -> B {
            if u >= self.adj.length() {
                return false;
            }
            let neighbors = self.adj.nth(u).clone();
            for i in 0..neighbors.length() {
                if neighbors.nth(i).clone() == v {
                    return true;
                }
            }
            false
        }

        /// - APAS: Work Θ(1), Span Θ(1) [Cost Spec 52.5]
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        #[verifier::external_body]
        fn out_neighbors(&self, u: N) -> ArraySeqMtEphS<N> { self.adj.nth(u).clone() }

        /// - APAS: Work Θ(1), Span Θ(1) [Cost Spec 52.5]
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) — agrees with APAS.
        #[verifier::external_body]
        fn out_degree(&self, u: N) -> N { self.adj.nth(u).length().clone() }

        /// - APAS: Work Θ(n), Span Θ(1) [Cost Spec 52.5, insert/delete edge]
        /// - Claude-Opus-4.6: Work Θ(d(u)), Span Θ(d(u)) — linear scan + rebuild of neighbor list; O(1) array set.
        #[verifier::external_body]
        fn set_edge(&mut self, u: N, v: N, exists: B) {
            if u >= self.adj.length() || v >= self.adj.length() {
                return;
            }
            let old_neighbors = self.adj.nth(u).clone();
            if exists {
                let mut found = false;
                for i in 0..old_neighbors.length() {
                    if old_neighbors.nth(i).clone() == v {
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
                    let _ = self.adj.set(u, ArraySeqMtEphS::from_vec(new_neighbors_vec));
                }
            } else {
                let mut new_neighbors_vec = Vec::<N>::new();
                for i in 0..old_neighbors.length() {
                    let neighbor = old_neighbors.nth(i).clone();
                    if neighbor != v {
                        new_neighbors_vec.push(neighbor);
                    }
                }
                let _ = self.adj.set(u, ArraySeqMtEphS::from_vec(new_neighbors_vec));
            }
        }
    }

    } // verus!
}
