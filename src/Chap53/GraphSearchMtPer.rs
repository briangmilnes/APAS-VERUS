//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 53: Generic Graph Search (persistent, multi-threaded).
//!
//! Note: This is a SEQUENTIAL implementation using thread-safe types.
//! True parallelism would require more complex lifetime management for the graph closure.
//! The parallel algorithm concept is demonstrated via use of AVLTreeSetMtPer which has
//! true parallel operations (filter, union, intersection).

pub mod GraphSearchMtPer {

    use vstd::prelude::*;
    use crate::Chap37::AVLTreeSeqMtPer::AVLTreeSeqMtPer::AVLTreeSeqMtPerTrait;
    use crate::Chap41::AVLTreeSetMtPer::AVLTreeSetMtPer::*;
    use crate::Types::Types::*;

    verus! {

    // 4. type definitions
    #[derive(Clone)]
    pub struct SearchResult<V: StTInMtT + Ord + 'static> {
        pub visited: AVLTreeSetMtPer<V>,
        pub parent: Option<AVLTreeSetMtPer<Pair<V, V>>>,
    }

    pub struct SelectAll;
    pub struct SelectOne;

    // 8. traits
    pub trait SelectionStrategy<V: StTInMtT + Ord + 'static> {
        fn select(&self, frontier: &AVLTreeSetMtPer<V>) -> (AVLTreeSetMtPer<V>, B);
    }

    pub trait GraphSearchMtPerTrait<V: StTInMtT + Ord + 'static> {
        /// - APAS: (no explicit cost; Theorem 53.1: ≤ |V| rounds; BFS rounds parallelizable)
        /// - Claude-Opus-4.6: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — neighbor loop is sequential despite parallel set ops.
        fn graph_search<G, S>(graph: &G, source: V, strategy: &S)                         -> SearchResult<V>
        where
            G: Fn(&V) -> AVLTreeSetMtPer<V>,
            S: SelectionStrategy<V>;

        /// - APAS: (no explicit cost; Theorem 53.1: ≤ |V| rounds)
        /// - Claude-Opus-4.6: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — neighbor loop is sequential despite parallel set ops.
        fn graph_search_multi<G, S>(graph: &G, sources: AVLTreeSetMtPer<V>, strategy: &S) -> SearchResult<V>
        where
            G: Fn(&V) -> AVLTreeSetMtPer<V>,
            S: SelectionStrategy<V>;

        /// - APAS: (no explicit cost; Theorem 53.1: ≤ |V| rounds)
        /// - Claude-Opus-4.6: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — sequential control flow; uses SelectAll (BFS).
        fn reachable<G>(graph: &G, source: V)                                             -> AVLTreeSetMtPer<V>
        where
            G: Fn(&V) -> AVLTreeSetMtPer<V>;
    }

    // 9. impls
    impl<V: StTInMtT + Ord + 'static> SelectionStrategy<V> for SelectAll {
        #[verifier::external_body]
        fn select(&self, frontier: &AVLTreeSetMtPer<V>) -> (AVLTreeSetMtPer<V>, B) { (frontier.clone(), false) }
    }

    impl<V: StTInMtT + Ord + 'static> SelectionStrategy<V> for SelectOne {
        #[verifier::external_body]
        fn select(&self, frontier: &AVLTreeSetMtPer<V>) -> (AVLTreeSetMtPer<V>, B) {
            if frontier.size() == 0 {
                (AVLTreeSetMtPer::empty(), false)
            } else {
                let seq = frontier.to_seq();
                let first = seq.nth(0).clone();
                (AVLTreeSetMtPer::singleton(first), false)
            }
        }
    }

    /// Generic graph search starting from single source (Algorithm 53.4).
    /// - APAS: (no explicit cost; Theorem 53.1: ≤ |V| rounds; BFS rounds parallelizable)
    /// - Claude-Opus-4.6: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — delegates to graph_search_multi; neighbor loop sequential.
    #[verifier::external_body]
    pub fn graph_search<V: StTInMtT + Ord + 'static, G, S>(graph: &G, source: V, strategy: &S) -> SearchResult<V>
    where
        G: Fn(&V) -> AVLTreeSetMtPer<V>,
        S: SelectionStrategy<V>,
    {
        let sources = AVLTreeSetMtPer::singleton(source);
        graph_search_multi(graph, sources, strategy)
    }

    /// Generic graph search starting from multiple sources (Exercise 53.3).
    /// - APAS: (no explicit cost; Theorem 53.1: ≤ |V| rounds; BFS rounds parallelizable)
    /// - Claude-Opus-4.6: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — neighbor loop is sequential despite parallel set ops.
    #[verifier::external_body]
    pub fn graph_search_multi<V: StTInMtT + Ord + 'static, G, S>(
        graph: &G,
        sources: AVLTreeSetMtPer<V>,
        strategy: &S,
    ) -> SearchResult<V>
    where
        G: Fn(&V) -> AVLTreeSetMtPer<V>,
        S: SelectionStrategy<V>,
    {
        fn explore<V, G, S>(
            graph: &G,
            strategy: &S,
            visited: AVLTreeSetMtPer<V>,
            frontier: AVLTreeSetMtPer<V>,
        ) -> AVLTreeSetMtPer<V>
        where
            V: StTInMtT + Ord + 'static,
            G: Fn(&V) -> AVLTreeSetMtPer<V>,
            S: SelectionStrategy<V>,
        {
            if frontier.size() == 0 {
                return visited;
            }

            let (selected, _) = strategy.select(&frontier);
            // Parallel union via AVLTreeSetMtPer's parallel implementation
            let visited_new = visited.union(&selected);

            // Compute out-neighbors (sequential loop, but union is parallel)
            let mut new_neighbors = AVLTreeSetMtPer::empty();
            let selected_seq = selected.to_seq();
            for i in 0..selected_seq.length() {
                let v = selected_seq.nth(i);
                let neighbors = graph(v);
                // Parallel union operation
                new_neighbors = new_neighbors.union(&neighbors);
            }

            // Parallel difference operation
            let frontier_new = new_neighbors.difference(&visited_new);
            explore(graph, strategy, visited_new, frontier_new)
        }

        let visited = explore(graph, strategy, AVLTreeSetMtPer::empty(), sources);

        SearchResult { visited, parent: None }
    }

    /// Find all vertices reachable from source (Problem 53.2) using SelectAll (BFS).
    /// - APAS: (no explicit cost; Theorem 53.1: ≤ |V| rounds)
    /// - Claude-Opus-4.6: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — delegates to graph_search with SelectAll.
    #[verifier::external_body]
    pub fn reachable<V: StTInMtT + Ord + 'static, G>(graph: &G, source: V) -> AVLTreeSetMtPer<V>
    where
        G: Fn(&V) -> AVLTreeSetMtPer<V>,
    {
        let result = graph_search(graph, source, &SelectAll);
        result.visited
    }

    } // verus!

    // 13. derive impls outside verus!

    impl<V: StTInMtT + Ord + 'static> std::fmt::Debug for SearchResult<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("SearchResult")
                .field("visited", &self.visited)
                .field("parent", &self.parent)
                .finish()
        }
    }
}
