//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 53: Generic Graph Search (persistent, single-threaded).
//!
//! Implements Algorithm 53.4 - Generic Graph Search with pluggable frontier selection.

pub mod GraphSearchStPer {

    use vstd::prelude::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::AVLTreeSeqStPerTrait;
    use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
    use crate::Types::Types::*;

    verus! {

    // 4. type definitions
    #[derive(Clone)]
    pub struct SearchResult<V: StT + Ord> {
        pub visited: AVLTreeSetStPer<V>,
        pub parent: Option<AVLTreeSetStPer<Pair<V, V>>>, // (child, parent) edges
    }

    /// Select all vertices in frontier (breadth-first style).
    pub struct SelectAll;

    /// Select single arbitrary vertex (depth-first style).
    pub struct SelectOne;

    // 8. traits
    /// Strategy for selecting which frontier vertices to visit next.
    pub trait SelectionStrategy<V: StT + Ord> {
        /// Select subset U ⊆ F where |U| ≥ 1.
        /// Returns (selected vertices, should_track_parents).
        fn select(&self, frontier: &AVLTreeSetStPer<V>) -> (AVLTreeSetStPer<V>, B);
    }

    pub trait GraphSearchStPerTrait<V: StT + Ord> {
        /// - APAS: (no explicit cost; Theorem 53.1: ≤ |V| rounds)
        /// - Claude-Opus-4.6: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — sequential; AVL set ops add log factor.
        fn graph_search<G, S>(graph: &G, source: V, strategy: &S)                         -> SearchResult<V>
        where
            G: Fn(&V) -> AVLTreeSetStPer<V>,
            S: SelectionStrategy<V>;

        /// - APAS: (no explicit cost; Theorem 53.1: ≤ |V| rounds)
        /// - Claude-Opus-4.6: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — sequential; AVL set ops add log factor.
        fn graph_search_multi<G, S>(graph: &G, sources: AVLTreeSetStPer<V>, strategy: &S) -> SearchResult<V>
        where
            G: Fn(&V) -> AVLTreeSetStPer<V>,
            S: SelectionStrategy<V>;

        /// - APAS: (no explicit cost; Theorem 53.1: ≤ |V| rounds)
        /// - Claude-Opus-4.6: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — sequential; uses SelectAll (BFS).
        fn reachable<G>(graph: &G, source: V)                                             -> AVLTreeSetStPer<V>
        where
            G: Fn(&V) -> AVLTreeSetStPer<V>;
    }

    // 9. impls
    impl<V: StT + Ord> SelectionStrategy<V> for SelectAll {
        fn select(&self, frontier: &AVLTreeSetStPer<V>) -> (AVLTreeSetStPer<V>, B) { (frontier.clone(), false) }
    }

    impl<V: StT + Ord> SelectionStrategy<V> for SelectOne {
        #[verifier::external_body]
        fn select(&self, frontier: &AVLTreeSetStPer<V>) -> (AVLTreeSetStPer<V>, B) {
            if frontier.size() == 0 {
                (AVLTreeSetStPer::empty(), false)
            } else {
                let seq = frontier.to_seq();
                let first = seq.nth(0).clone();
                (AVLTreeSetStPer::singleton(first), false)
            }
        }
    }

    /// Generic graph search starting from single source (Algorithm 53.4).
    /// - APAS: (no explicit cost; Theorem 53.1: ≤ |V| rounds)
    /// - Claude-Opus-4.6: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — delegates to graph_search_multi.
    #[verifier::external_body]
    pub fn graph_search<V: StT + Ord, G, S>(graph: &G, source: V, strategy: &S) -> SearchResult<V>
    where
        G: Fn(&V) -> AVLTreeSetStPer<V>,
        S: SelectionStrategy<V>,
    {
        let sources = AVLTreeSetStPer::singleton(source);
        graph_search_multi(graph, sources, strategy)
    }

    /// Generic graph search starting from multiple sources (Exercise 53.3).
    /// - APAS: (no explicit cost; Theorem 53.1: ≤ |V| rounds)
    /// - Claude-Opus-4.6: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — sequential; AVL set ops add log factor.
    #[verifier::external_body]
    pub fn graph_search_multi<V: StT + Ord, G, S>(
        graph: &G,
        sources: AVLTreeSetStPer<V>,
        strategy: &S,
    ) -> SearchResult<V>
    where
        G: Fn(&V) -> AVLTreeSetStPer<V>,
        S: SelectionStrategy<V>,
    {
        // Algorithm 53.4: Generic Graph Search
        fn explore<V, G, S>(
            graph: &G,
            strategy: &S,
            visited: AVLTreeSetStPer<V>,
            frontier: AVLTreeSetStPer<V>,
        ) -> AVLTreeSetStPer<V>
        where
            V: StT + Ord,
            G: Fn(&V) -> AVLTreeSetStPer<V>,
            S: SelectionStrategy<V>,
        {
            // Line 4: if |F| = 0 then X
            if frontier.size() == 0 {
                return visited;
            }

            // Line 7: choose U ⊆ F such that |U| ≥ 1
            let (selected, _) = strategy.select(&frontier);

            // Line 9: X' = X ∪ U
            let visited_new = visited.union(&selected);

            // Line 10: F' = N+(X') \ X'
            // Compute out-neighbors of all newly visited vertices
            let mut new_neighbors = AVLTreeSetStPer::empty();
            let selected_seq = selected.to_seq();
            for i in 0..selected_seq.length() {
                let v = selected_seq.nth(i);
                let neighbors = graph(v);
                new_neighbors = new_neighbors.union(&neighbors);
            }

            // Remove already visited vertices
            let frontier_new = new_neighbors.difference(&visited_new);

            // Line 11: explore X' F'
            explore(graph, strategy, visited_new, frontier_new)
        }

        // Line 13: explore {} {s}
        let visited = explore(graph, strategy, AVLTreeSetStPer::empty(), sources);

        SearchResult { visited, parent: None }
    }

    /// Find all vertices reachable from source (Problem 53.2) using SelectAll (BFS).
    /// - APAS: (no explicit cost; Theorem 53.1: ≤ |V| rounds)
    /// - Claude-Opus-4.6: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — delegates to graph_search with SelectAll.
    #[verifier::external_body]
    pub fn reachable<V: StT + Ord, G>(graph: &G, source: V) -> AVLTreeSetStPer<V>
    where
        G: Fn(&V) -> AVLTreeSetStPer<V>,
    {
        let result = graph_search(graph, source, &SelectAll);
        result.visited
    }

    } // verus!

    // 13. derive impls outside verus!

    impl<V: StT + Ord> std::fmt::Debug for SearchResult<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("SearchResult")
                .field("visited", &self.visited)
                .field("parent", &self.parent)
                .finish()
        }
    }
}
