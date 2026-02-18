//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 53: Min-Priority Queue Search - ephemeral, single-threaded.

pub mod PQMinStEph {

    use std::marker::PhantomData;

    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::AVLTreeSeqStEphTrait;
    use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
    use crate::Types::Types::*;

    #[derive(Clone, Debug)]
    pub struct PQMinResult<V: StT + Ord, P: StT + Ord> {
        pub visited: AVLTreeSetStEph<V>,
        pub priorities: AVLTreeSetStEph<Pair<V, P>>,
        pub parent: Option<AVLTreeSetStEph<Pair<V, V>>>,
    }

    pub trait PriorityFn<V: StT + Ord, P: StT + Ord> {
        fn priority(&self, v: &V) -> P;
    }

    pub struct ClosurePriority<V: StT + Ord, P: StT + Ord, F: Fn(&V) -> P> {
        f: F,
        _phantom: PhantomData<(V, P)>,
    }

    pub trait PQMinStEphTrait<V: StT + Ord, P: StT + Ord> {
        /// - APAS: (no explicit PFS cost in Chap53; PFS cost depends on priority queue implementation)
        /// - Claude-Opus-4.6: Work Θ(|V|² + |E| log |V|), Span Θ(|V|² + |E| log |V|) — find_min uses to_seq O(|F|) per round.
        fn pq_min<G, PF>(graph: &G, source: V, priority_fn: &PF)                         -> PQMinResult<V, P>
        where
            G: Fn(&V) -> AVLTreeSetStEph<V>,
            PF: PriorityFn<V, P>;

        /// - APAS: (no explicit PFS cost in Chap53; PFS cost depends on priority queue implementation)
        /// - Claude-Opus-4.6: Work Θ(|V|² + |E| log |V|), Span Θ(|V|² + |E| log |V|) — find_min uses to_seq O(|F|) per round.
        fn pq_min_multi<G, PF>(graph: &G, sources: AVLTreeSetStEph<V>, priority_fn: &PF) -> PQMinResult<V, P>
        where
            G: Fn(&V) -> AVLTreeSetStEph<V>,
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
        G: Fn(&V) -> AVLTreeSetStEph<V>,
        PF: PriorityFn<V, P>,
    {
        let sources = AVLTreeSetStEph::singleton(source);
        pq_min_multi(graph, sources, priority_fn)
    }

    /// Priority-first search from multiple sources (Section 53.4).
    /// - APAS: (no explicit PFS cost in Chap53; PFS cost depends on priority queue implementation)
    /// - Claude-Opus-4.6: Work Θ(|V|² + |E| log |V|), Span Θ(|V|² + |E| log |V|) — find_min uses to_seq O(|F|) per round; sequential.
    pub fn pq_min_multi<V: StT + Ord, P: StT + Ord, G, PF>(
        graph: &G,
        sources: AVLTreeSetStEph<V>,
        priority_fn: &PF,
    ) -> PQMinResult<V, P>
    where
        G: Fn(&V) -> AVLTreeSetStEph<V>,
        PF: PriorityFn<V, P>,
    {
        fn find_min_priority<V: StT + Ord, P: StT + Ord>(frontier: &AVLTreeSetStEph<Pair<Pair<P, V>, V>>) -> Option<V> {
            if frontier.size() == 0 {
                None
            } else {
                let seq = frontier.to_seq();
                Some(seq.nth(0).1.clone())
            }
        }

        fn explore<V, P, G, PF>(
            graph: &G,
            priority_fn: &PF,
            visited: AVLTreeSetStEph<V>,
            frontier: AVLTreeSetStEph<Pair<Pair<P, V>, V>>,
        ) -> (AVLTreeSetStEph<V>, AVLTreeSetStEph<Pair<V, P>>)
        where
            V: StT + Ord,
            P: StT + Ord,
            G: Fn(&V) -> AVLTreeSetStEph<V>,
            PF: PriorityFn<V, P>,
        {
            if let Some(v) = find_min_priority(&frontier) {
                let p = priority_fn.priority(&v);
                let entry = Pair(Pair(p.clone(), v.clone()), v.clone());
                let frontier_new = frontier.difference(&AVLTreeSetStEph::singleton(entry));

                let visited_new = visited.union(&AVLTreeSetStEph::singleton(v.clone()));

                let neighbors = graph(&v);
                let mut frontier_updated = frontier_new;
                let neighbors_seq = neighbors.to_seq();

                for i in 0..neighbors_seq.length() {
                    let neighbor = neighbors_seq.nth(i);
                    if !visited_new.find(neighbor) {
                        let neighbor_p = priority_fn.priority(neighbor);
                        let neighbor_entry = Pair(Pair(neighbor_p.clone(), neighbor.clone()), neighbor.clone());
                        frontier_updated = frontier_updated.union(&AVLTreeSetStEph::singleton(neighbor_entry));
                    }
                }

                explore(graph, priority_fn, visited_new, frontier_updated)
            } else {
                let mut priorities = AVLTreeSetStEph::empty();
                let visited_seq = visited.to_seq();
                for i in 0..visited_seq.length() {
                    let v = visited_seq.nth(i);
                    let p = priority_fn.priority(v);
                    priorities = priorities.union(&AVLTreeSetStEph::singleton(Pair(v.clone(), p)));
                }
                (visited, priorities)
            }
        }

        let mut initial_frontier = AVLTreeSetStEph::empty();
        let sources_seq = sources.to_seq();
        for i in 0..sources_seq.length() {
            let v = sources_seq.nth(i);
            let p = priority_fn.priority(v);
            let entry = Pair(Pair(p.clone(), v.clone()), v.clone());
            initial_frontier = initial_frontier.union(&AVLTreeSetStEph::singleton(entry));
        }

        let (visited, priorities) = explore(graph, priority_fn, AVLTreeSetStEph::empty(), initial_frontier);

        PQMinResult {
            visited,
            priorities,
            parent: None,
        }
    }
}
