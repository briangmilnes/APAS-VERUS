//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 53: Min-Priority Queue Search - persistent, single-threaded.
//!
//! Implements Algorithm 53.7 - Priority Queue Search framework.
//! Selects minimum priority vertices first (lower priority = higher urgency).

pub mod PQMinStPer {

    use std::marker::PhantomData;

    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::AVLTreeSeqStPerTrait;
    use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
    use crate::Types::Types::*;

    #[derive(Clone, Debug)]
    pub struct PQMinResult<V: StT + Ord, P: StT + Ord> {
        pub visited: AVLTreeSetStPer<V>,
        pub priorities: AVLTreeSetStPer<Pair<V, P>>,     // (vertex, priority)
        pub parent: Option<AVLTreeSetStPer<Pair<V, V>>>, // (child, parent)
    }

    /// Priority function: maps vertices to their priorities.
    /// Lower priority values = higher priority (visited first).
    pub trait PriorityFn<V: StT + Ord, P: StT + Ord> {
        fn priority(&self, v: &V) -> P;
    }

    /// Simple wrapper for closure-based priority functions.
    pub struct ClosurePriority<V: StT + Ord, P: StT + Ord, F: Fn(&V) -> P> {
        f: F,
        _phantom: PhantomData<(V, P)>,
    }

    pub trait PQMinStPerTrait<V: StT + Ord, P: StT + Ord> {
        /// - APAS: (no explicit PFS cost in Chap53; PFS cost depends on priority queue implementation)
        /// - Claude-Opus-4.6: Work Θ(|V|² + |E| log |V|), Span Θ(|V|² + |E| log |V|) — find_min uses to_seq O(|F|) per round.
        fn pq_min<G, PF>(graph: &G, source: V, priority_fn: &PF)                         -> PQMinResult<V, P>
        where
            G: Fn(&V) -> AVLTreeSetStPer<V>,
            PF: PriorityFn<V, P>;

        /// - APAS: (no explicit PFS cost in Chap53; PFS cost depends on priority queue implementation)
        /// - Claude-Opus-4.6: Work Θ(|V|² + |E| log |V|), Span Θ(|V|² + |E| log |V|) — find_min uses to_seq O(|F|) per round.
        fn pq_min_multi<G, PF>(graph: &G, sources: AVLTreeSetStPer<V>, priority_fn: &PF) -> PQMinResult<V, P>
        where
            G: Fn(&V) -> AVLTreeSetStPer<V>,
            PF: PriorityFn<V, P>;
    }

    pub trait ClosurePriorityTrait<V: StT + Ord, P: StT + Ord, F: Fn(&V) -> P> {
        fn new(f: F) -> Self;
    }

    impl<V: StT + Ord, P: StT + Ord, F: Fn(&V) -> P> ClosurePriorityTrait<V, P, F> for ClosurePriority<V, P, F> {
        fn new(f: F) -> Self {
            Self {
                f,
                _phantom: PhantomData,
            }
        }
    }

    impl<V: StT + Ord, P: StT + Ord, F: Fn(&V) -> P> PriorityFn<V, P> for ClosurePriority<V, P, F> {
        fn priority(&self, v: &V) -> P { (self.f)(v) }
    }

    /// Priority-first search from single source (Section 53.4).
    /// - APAS: (no explicit PFS cost in Chap53; PFS cost depends on priority queue implementation)
    /// - Claude-Opus-4.6: Work Θ(|V|² + |E| log |V|), Span Θ(|V|² + |E| log |V|) — delegates to pq_min_multi.
    pub fn pq_min<V: StT + Ord, P: StT + Ord, G, PF>(graph: &G, source: V, priority_fn: &PF) -> PQMinResult<V, P>
    where
        G: Fn(&V) -> AVLTreeSetStPer<V>,
        PF: PriorityFn<V, P>,
    {
        let sources = AVLTreeSetStPer::singleton(source);
        pq_min_multi(graph, sources, priority_fn)
    }

    /// Priority-first search from multiple sources (Section 53.4).
    /// - APAS: (no explicit PFS cost in Chap53; PFS cost depends on priority queue implementation)
    /// - Claude-Opus-4.6: Work Θ(|V|² + |E| log |V|), Span Θ(|V|² + |E| log |V|) — find_min uses to_seq O(|F|) per round; sequential.
    pub fn pq_min_multi<V: StT + Ord, P: StT + Ord, G, PF>(
        graph: &G,
        sources: AVLTreeSetStPer<V>,
        priority_fn: &PF,
    ) -> PQMinResult<V, P>
    where
        G: Fn(&V) -> AVLTreeSetStPer<V>,
        PF: PriorityFn<V, P>,
    {
        fn find_min_priority<V: StT + Ord, P: StT + Ord>(frontier: &AVLTreeSetStPer<Pair<Pair<P, V>, V>>) -> Option<V> {
            if frontier.size() == 0 {
                None
            } else {
                let seq = frontier.to_seq();
                // First element has minimum priority (sorted by (P, V) pair)
                let min_entry = seq.nth(0);
                Some(min_entry.1.clone())
            }
        }

        fn explore<V, P, G, PF>(
            graph: &G,
            priority_fn: &PF,
            visited: AVLTreeSetStPer<V>,
            frontier: AVLTreeSetStPer<Pair<Pair<P, V>, V>>, // ((priority, vertex), vertex)
        ) -> (AVLTreeSetStPer<V>, AVLTreeSetStPer<Pair<V, P>>)
        where
            V: StT + Ord,
            P: StT + Ord,
            G: Fn(&V) -> AVLTreeSetStPer<V>,
            PF: PriorityFn<V, P>,
        {
            // Select vertex with minimum priority
            if let Some(v) = find_min_priority(&frontier) {
                // Remove selected vertex from frontier
                let p = priority_fn.priority(&v);
                let entry = Pair(Pair(p.clone(), v.clone()), v.clone());
                let frontier_new = frontier.difference(&AVLTreeSetStPer::singleton(entry));

                // Add to visited
                let visited_new = visited.union(&AVLTreeSetStPer::singleton(v.clone()));

                // Get neighbors and add unvisited ones to frontier
                let neighbors = graph(&v);
                let mut frontier_updated = frontier_new;
                let neighbors_seq = neighbors.to_seq();

                for i in 0..neighbors_seq.length() {
                    let neighbor = neighbors_seq.nth(i);
                    if !visited_new.find(neighbor) {
                        let neighbor_p = priority_fn.priority(neighbor);
                        let neighbor_entry = Pair(Pair(neighbor_p.clone(), neighbor.clone()), neighbor.clone());
                        frontier_updated = frontier_updated.union(&AVLTreeSetStPer::singleton(neighbor_entry));
                    }
                }

                explore(graph, priority_fn, visited_new, frontier_updated)
            } else {
                // Build priority set from visited vertices
                let mut priorities = AVLTreeSetStPer::empty();
                let visited_seq = visited.to_seq();
                for i in 0..visited_seq.length() {
                    let v = visited_seq.nth(i);
                    let p = priority_fn.priority(v);
                    priorities = priorities.union(&AVLTreeSetStPer::singleton(Pair(v.clone(), p)));
                }
                (visited, priorities)
            }
        }

        // Initialize frontier with sources
        let mut initial_frontier = AVLTreeSetStPer::empty();
        let sources_seq = sources.to_seq();
        for i in 0..sources_seq.length() {
            let v = sources_seq.nth(i);
            let p = priority_fn.priority(v);
            let entry = Pair(Pair(p.clone(), v.clone()), v.clone());
            initial_frontier = initial_frontier.union(&AVLTreeSetStPer::singleton(entry));
        }

        let (visited, priorities) = explore(graph, priority_fn, AVLTreeSetStPer::empty(), initial_frontier);

        PQMinResult {
            visited,
            priorities,
            parent: None,
        }
    }
}
