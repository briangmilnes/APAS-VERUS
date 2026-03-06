//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 53: Min-Priority Queue Search - ephemeral, single-threaded.

pub mod PQMinStEph {

    use vstd::prelude::*;
    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::AVLTreeSeqStEphTrait;
    use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
    use crate::Types::Types::*;

    verus! {

    // 4. type definitions
    pub struct PQMinResult<V: StT + Ord, P: StT + Ord> {
        pub visited: AVLTreeSetStEph<V>,
        pub priorities: AVLTreeSetStEph<Pair<V, P>>,
        pub parent: Option<AVLTreeSetStEph<Pair<V, V>>>,
    }

    // 8. traits
    pub trait PQMinStEphTrait<V: StT + Ord, P: StT + Ord> {
        /// - APAS: (no explicit PFS cost in Chap53; PFS cost depends on priority queue implementation)
        /// - Claude-Opus-4.6: Work Θ(|V|² + |E| log |V|), Span Θ(|V|² + |E| log |V|) — find_min uses to_seq O(|F|) per round.
        fn pq_min<G, PF>(graph: &G, source: V, priority_fn: &PF)                         -> (search: PQMinResult<V, P>)
        where
            G: Fn(&V) -> AVLTreeSetStEph<V>,
            PF: Fn(&V) -> P,
            ensures search.visited@.contains(source@);

        /// - APAS: (no explicit PFS cost in Chap53; PFS cost depends on priority queue implementation)
        /// - Claude-Opus-4.6: Work Θ(|V|² + |E| log |V|), Span Θ(|V|² + |E| log |V|) — find_min uses to_seq O(|F|) per round.
        fn pq_min_multi<G, PF>(graph: &G, sources: AVLTreeSetStEph<V>, priority_fn: &PF) -> (search: PQMinResult<V, P>)
        where
            G: Fn(&V) -> AVLTreeSetStEph<V>,
            PF: Fn(&V) -> P,
            ensures sources@.subset_of(search.visited@);
    }

    // 9. impls

    impl<V: StT + Ord, P: StT + Ord> PQMinStEphTrait<V, P> for PQMinResult<V, P> {
        fn pq_min<G, PF>(graph: &G, source: V, priority_fn: &PF) -> (search: PQMinResult<V, P>)
        where G: Fn(&V) -> AVLTreeSetStEph<V>, PF: Fn(&V) -> P,
        { pq_min(graph, source, priority_fn) }

        fn pq_min_multi<G, PF>(graph: &G, sources: AVLTreeSetStEph<V>, priority_fn: &PF) -> (search: PQMinResult<V, P>)
        where G: Fn(&V) -> AVLTreeSetStEph<V>, PF: Fn(&V) -> P,
        { pq_min_multi(graph, sources, priority_fn) }
    }

    /// Priority-first search from single source (Section 53.4).
    /// - APAS: (no explicit PFS cost in Chap53; PFS cost depends on priority queue implementation)
    /// - Claude-Opus-4.6: Work Θ(|V|² + |E| log |V|), Span Θ(|V|² + |E| log |V|) — delegates to pq_min_multi.
    pub fn pq_min<V: StT + Ord, P: StT + Ord, G, PF>(graph: &G, source: V, priority_fn: &PF) -> (search: PQMinResult<V, P>)
    where
        G: Fn(&V) -> AVLTreeSetStEph<V>,
        PF: Fn(&V) -> P,
        ensures search.visited@.contains(source@),
    {
        let sources = AVLTreeSetStEph::singleton(source);
        pq_min_multi(graph, sources, priority_fn)
    }

    #[verifier::external_body]
    fn pq_find_min_priority<V: StT + Ord, P: StT + Ord>(
        frontier: &AVLTreeSetStEph<Pair<Pair<P, V>, V>>,
    ) -> Option<V> {
        if frontier.size() == 0 {
            None
        } else {
            let seq = frontier.to_seq();
            Some(seq.nth(0).1.clone())
        }
    }

    #[verifier::external_body]
    fn pq_explore<V: StT + Ord, P: StT + Ord, G: Fn(&V) -> AVLTreeSetStEph<V>, PF: Fn(&V) -> P>(
        graph: &G,
        priority_fn: &PF,
        visited: AVLTreeSetStEph<V>,
        frontier: AVLTreeSetStEph<Pair<Pair<P, V>, V>>,
    ) -> (AVLTreeSetStEph<V>, AVLTreeSetStEph<Pair<V, P>>)
    {
        if let Some(v) = pq_find_min_priority(&frontier) {
            let p = priority_fn(&v);
            let entry = Pair(Pair(p.clone(), v.clone()), v.clone());
            let frontier_new = frontier.difference(&AVLTreeSetStEph::singleton(entry));

            let visited_new = visited.union(&AVLTreeSetStEph::singleton(v.clone()));

            let neighbors = graph(&v);
            let mut frontier_updated = frontier_new;
            let neighbors_seq = neighbors.to_seq();

            let mut i: usize = 0;
            while i < neighbors_seq.length()
            {
                let neighbor = neighbors_seq.nth(i);
                if !visited_new.find(neighbor) {
                    let neighbor_p = priority_fn(neighbor);
                    let neighbor_entry = Pair(Pair(neighbor_p.clone(), neighbor.clone()), neighbor.clone());
                    frontier_updated = frontier_updated.union(&AVLTreeSetStEph::singleton(neighbor_entry));
                }
                i = i + 1;
            }

            pq_explore(graph, priority_fn, visited_new, frontier_updated)
        } else {
            let mut priorities = AVLTreeSetStEph::empty();
            let visited_seq = visited.to_seq();
            let mut j: usize = 0;
            while j < visited_seq.length()
            {
                let v = visited_seq.nth(j);
                let p = priority_fn(v);
                priorities = priorities.union(&AVLTreeSetStEph::singleton(Pair(v.clone(), p)));
                j = j + 1;
            }
            (visited, priorities)
        }
    }

    /// Priority-first search from multiple sources (Section 53.4).
    #[verifier::external_body]
    pub fn pq_min_multi<V: StT + Ord, P: StT + Ord, G, PF>(
        graph: &G,
        sources: AVLTreeSetStEph<V>,
        priority_fn: &PF,
    ) -> (search: PQMinResult<V, P>)
    where
        G: Fn(&V) -> AVLTreeSetStEph<V>,
        PF: Fn(&V) -> P,
        ensures sources@.subset_of(search.visited@),
    {
        let mut initial_frontier = AVLTreeSetStEph::empty();
        let sources_seq = sources.to_seq();
        let mut i: usize = 0;
        while i < sources_seq.length()
        {
            let v = sources_seq.nth(i);
            let p = priority_fn(v);
            let entry = Pair(Pair(p.clone(), v.clone()), v.clone());
            initial_frontier = initial_frontier.union(&AVLTreeSetStEph::singleton(entry));
            i = i + 1;
        }

        let (visited, priorities) = pq_explore(graph, priority_fn, AVLTreeSetStEph::empty(), initial_frontier);

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
