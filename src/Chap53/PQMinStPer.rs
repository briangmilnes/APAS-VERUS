//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 53: Min-Priority Queue Search - persistent, single-threaded.
//!
//! Implements Algorithm 53.7 - Priority Queue Search framework.
//! Selects minimum priority vertices first (lower priority = higher urgency).

pub mod PQMinStPer {

    use vstd::prelude::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::AVLTreeSeqStPerTrait;
    use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
    use crate::Types::Types::*;

    verus! {

    // 4. type definitions
    #[verifier::reject_recursive_types(V)]
    #[verifier::reject_recursive_types(P)]
    pub struct PQMinResult<V: StT + Ord, P: StT + Ord> {
        pub visited: AVLTreeSetStPer<V>,
        pub priorities: AVLTreeSetStPer<Pair<V, P>>,     // (vertex, priority)
        pub parent: Option<AVLTreeSetStPer<Pair<V, V>>>, // (child, parent)
    }

    // 8. traits
    pub trait PQMinStPerTrait<V: StT + Ord, P: StT + Ord> {
        /// - APAS: (no explicit PFS cost in Chap53; PFS cost depends on priority queue implementation)
        /// - Claude-Opus-4.6: Work Θ(|V|² + |E| log |V|), Span Θ(|V|² + |E| log |V|) — find_min uses to_seq O(|F|) per round.
        fn pq_min<G, PF>(graph: &G, source: V, priority_fn: &PF)                         -> PQMinResult<V, P>
        where
            G: Fn(&V) -> AVLTreeSetStPer<V>,
            PF: Fn(&V) -> P;

        /// - APAS: (no explicit PFS cost in Chap53; PFS cost depends on priority queue implementation)
        /// - Claude-Opus-4.6: Work Θ(|V|² + |E| log |V|), Span Θ(|V|² + |E| log |V|) — find_min uses to_seq O(|F|) per round.
        fn pq_min_multi<G, PF>(graph: &G, sources: AVLTreeSetStPer<V>, priority_fn: &PF) -> PQMinResult<V, P>
        where
            G: Fn(&V) -> AVLTreeSetStPer<V>,
            PF: Fn(&V) -> P;
    }

    // 9. impls

    /// Priority-first search from single source (Section 53.4).
    /// - APAS: (no explicit PFS cost in Chap53; PFS cost depends on priority queue implementation)
    /// - Claude-Opus-4.6: Work Θ(|V|² + |E| log |V|), Span Θ(|V|² + |E| log |V|) — delegates to pq_min_multi.
    pub fn pq_min<V: StT + Ord, P: StT + Ord, G, PF>(graph: &G, source: V, priority_fn: &PF) -> PQMinResult<V, P>
    where
        G: Fn(&V) -> AVLTreeSetStPer<V>,
        PF: Fn(&V) -> P,
    {
        let sources = AVLTreeSetStPer::singleton(source);
        pq_min_multi(graph, sources, priority_fn)
    }

    #[verifier::external_body]
    fn pq_find_min_priority<V: StT + Ord, P: StT + Ord>(
        frontier: &AVLTreeSetStPer<Pair<Pair<P, V>, V>>,
    ) -> Option<V> {
        if frontier.size() == 0 {
            None
        } else {
            let seq = frontier.to_seq();
            Some(seq.nth(0).1.clone())
        }
    }

    #[verifier::external_body]
    fn pq_explore<V: StT + Ord, P: StT + Ord, G: Fn(&V) -> AVLTreeSetStPer<V>, PF: Fn(&V) -> P>(
        graph: &G,
        priority_fn: &PF,
        visited: AVLTreeSetStPer<V>,
        frontier: AVLTreeSetStPer<Pair<Pair<P, V>, V>>,
    ) -> (AVLTreeSetStPer<V>, AVLTreeSetStPer<Pair<V, P>>)
    {
        if let Some(v) = pq_find_min_priority(&frontier) {
            let p = priority_fn(&v);
            let entry = Pair(Pair(p.clone(), v.clone()), v.clone());
            let frontier_new = frontier.difference(&AVLTreeSetStPer::singleton(entry));

            let visited_new = visited.union(&AVLTreeSetStPer::singleton(v.clone()));

            let neighbors = graph(&v);
            let mut frontier_updated = frontier_new;
            let neighbors_seq = neighbors.to_seq();
            let len = neighbors_seq.length();

            let mut i: usize = 0;
            while i < len {
                let neighbor = neighbors_seq.nth(i);
                if !visited_new.find(neighbor) {
                    let neighbor_p = priority_fn(neighbor);
                    let neighbor_entry = Pair(Pair(neighbor_p.clone(), neighbor.clone()), neighbor.clone());
                    frontier_updated = frontier_updated.union(&AVLTreeSetStPer::singleton(neighbor_entry));
                }
                i = i + 1;
            }

            pq_explore(graph, priority_fn, visited_new, frontier_updated)
        } else {
            let mut priorities = AVLTreeSetStPer::empty();
            let visited_seq = visited.to_seq();
            let len = visited_seq.length();
            let mut j: usize = 0;
            while j < len {
                let v = visited_seq.nth(j);
                let p = priority_fn(v);
                priorities = priorities.union(&AVLTreeSetStPer::singleton(Pair(v.clone(), p)));
                j = j + 1;
            }
            (visited, priorities)
        }
    }

    /// Priority-first search from multiple sources (Section 53.4).
    #[verifier::external_body]
    pub fn pq_min_multi<V: StT + Ord, P: StT + Ord, G, PF>(
        graph: &G,
        sources: AVLTreeSetStPer<V>,
        priority_fn: &PF,
    ) -> PQMinResult<V, P>
    where
        G: Fn(&V) -> AVLTreeSetStPer<V>,
        PF: Fn(&V) -> P,
    {
        let mut initial_frontier = AVLTreeSetStPer::empty();
        let sources_seq = sources.to_seq();
        let len = sources_seq.length();
        let mut i: usize = 0;
        while i < len {
            let v = sources_seq.nth(i);
            let p = priority_fn(v);
            let entry = Pair(Pair(p.clone(), v.clone()), v.clone());
            initial_frontier = initial_frontier.union(&AVLTreeSetStPer::singleton(entry));
            i = i + 1;
        }

        let (visited, priorities) = pq_explore(graph, priority_fn, AVLTreeSetStPer::empty(), initial_frontier);

        PQMinResult {
            visited,
            priorities,
            parent: None,
        }
    }

    // 11. derive impls in verus!

    impl<V: StT + Ord, P: StT + Ord> Clone for PQMinResult<V, P> {
        fn clone(&self) -> (out: Self) {
            PQMinResult {
                visited: self.visited.clone(),
                priorities: self.priorities.clone(),
                parent: self.parent.clone(),
            }
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    impl<V: StT + Ord, P: StT + Ord> std::fmt::Debug for PQMinResult<V, P> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("PQMinResult")
                .field("visited", &self.visited)
                .field("priorities", &self.priorities)
                .field("parent", &self.parent)
                .finish()
        }
    }

    impl<V: StT + Ord, P: StT + Ord> std::fmt::Display for PQMinResult<V, P> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "PQMinResult(visited={}, priorities={})", self.visited.size(), self.priorities.size())
        }
    }
}
