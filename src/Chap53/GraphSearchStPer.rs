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
    #[verifier::reject_recursive_types(V)]
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
        fn select(&self, frontier: &AVLTreeSetStPer<V>) -> (selected: (AVLTreeSetStPer<V>, B))
            ensures selected.0@.subset_of(frontier@);
    }

    pub trait GraphSearchStPerTrait<V: StT + Ord> {
        /// - APAS: (no explicit cost; Theorem 53.1: ≤ |V| rounds)
        /// - Claude-Opus-4.6: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — sequential; AVL set ops add log factor.
        fn graph_search<G, S>(graph: &G, source: V, strategy: &S)                         -> (search: SearchResult<V>)
        where
            G: Fn(&V) -> AVLTreeSetStPer<V>,
            S: SelectionStrategy<V>,
            ensures search.visited@.contains(source@);

        /// - APAS: (no explicit cost; Theorem 53.1: ≤ |V| rounds)
        /// - Claude-Opus-4.6: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — sequential; AVL set ops add log factor.
        fn graph_search_multi<G, S>(graph: &G, sources: AVLTreeSetStPer<V>, strategy: &S) -> (search: SearchResult<V>)
        where
            G: Fn(&V) -> AVLTreeSetStPer<V>,
            S: SelectionStrategy<V>,
            ensures sources@.subset_of(search.visited@);

        /// - APAS: (no explicit cost; Theorem 53.1: ≤ |V| rounds)
        /// - Claude-Opus-4.6: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — sequential; uses SelectAll (BFS).
        fn reachable<G>(graph: &G, source: V)                                             -> (reachable_set: AVLTreeSetStPer<V>)
        where
            G: Fn(&V) -> AVLTreeSetStPer<V>,
            ensures reachable_set@.contains(source@);
    }

    // 9. impls
    impl<V: StT + Ord> SelectionStrategy<V> for SelectAll {
        fn select(&self, frontier: &AVLTreeSetStPer<V>) -> (selected: (AVLTreeSetStPer<V>, B)) { (frontier.clone(), false) }
    }

    impl<V: StT + Ord> SelectionStrategy<V> for SelectOne {
        #[verifier::external_body]
        fn select(&self, frontier: &AVLTreeSetStPer<V>) -> (selected: (AVLTreeSetStPer<V>, B)) {
            if frontier.size() == 0 {
                (AVLTreeSetStPer::empty(), false)
            } else {
                let seq = frontier.to_seq();
                let first = seq.nth(0).clone();
                (AVLTreeSetStPer::singleton(first), false)
            }
        }
    }

    pub fn graph_search<V: StT + Ord, G, S>(graph: &G, source: V, strategy: &S) -> (search: SearchResult<V>)
    where
        G: Fn(&V) -> AVLTreeSetStPer<V>,
        S: SelectionStrategy<V>,
        ensures search.visited@.contains(source@),
    {
        let sources = AVLTreeSetStPer::singleton(source);
        graph_search_multi(graph, sources, strategy)
    }

    /// Recursive graph exploration helper (Algorithm 53.4 loop body).
    #[verifier::external_body]
    fn graph_search_explore<V: StT + Ord, G: Fn(&V) -> AVLTreeSetStPer<V>, S: SelectionStrategy<V>>(
        graph: &G,
        strategy: &S,
        visited: AVLTreeSetStPer<V>,
        frontier: AVLTreeSetStPer<V>,
    ) -> (visited_all: AVLTreeSetStPer<V>)
        ensures visited@.subset_of(visited_all@), frontier@.subset_of(visited_all@),
    {
        if frontier.size() == 0 {
            return visited;
        }

        let (selected, _) = strategy.select(&frontier);
        let visited_new = visited.union(&selected);

        let mut new_neighbors = AVLTreeSetStPer::empty();
        let selected_seq = selected.to_seq();
        let len = selected_seq.length();
        let mut i: usize = 0;
        while i < len {
            let v = selected_seq.nth(i);
            let neighbors = graph(v);
            new_neighbors = new_neighbors.union(&neighbors);
            i = i + 1;
        }

        let frontier_new = new_neighbors.difference(&visited_new);
        graph_search_explore(graph, strategy, visited_new, frontier_new)
    }

    /// Generic graph search starting from multiple sources (Exercise 53.3).
    pub fn graph_search_multi<V: StT + Ord, G, S>(
        graph: &G,
        sources: AVLTreeSetStPer<V>,
        strategy: &S,
    ) -> (search: SearchResult<V>)
    where
        G: Fn(&V) -> AVLTreeSetStPer<V>,
        S: SelectionStrategy<V>,
        ensures sources@.subset_of(search.visited@),
    {
        let visited = graph_search_explore(graph, strategy, AVLTreeSetStPer::empty(), sources);
        SearchResult { visited, parent: None }
    }

    impl<V: StT + Ord> GraphSearchStPerTrait<V> for SearchResult<V> {
        fn graph_search<G, S>(graph: &G, source: V, strategy: &S) -> (search: SearchResult<V>)
        where G: Fn(&V) -> AVLTreeSetStPer<V>, S: SelectionStrategy<V>,
        { graph_search(graph, source, strategy) }

        fn graph_search_multi<G, S>(graph: &G, sources: AVLTreeSetStPer<V>, strategy: &S) -> (search: SearchResult<V>)
        where G: Fn(&V) -> AVLTreeSetStPer<V>, S: SelectionStrategy<V>,
        { graph_search_multi(graph, sources, strategy) }

        fn reachable<G>(graph: &G, source: V) -> (reachable_set: AVLTreeSetStPer<V>)
        where G: Fn(&V) -> AVLTreeSetStPer<V>,
        { reachable(graph, source) }
    }

    /// Find all vertices reachable from source (Problem 53.2) using SelectAll (BFS).
    /// - APAS: (no explicit cost; Theorem 53.1: ≤ |V| rounds)
    /// - Claude-Opus-4.6: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — delegates to graph_search with SelectAll.
    pub fn reachable<V: StT + Ord, G>(graph: &G, source: V) -> (reachable_set: AVLTreeSetStPer<V>)
    where
        G: Fn(&V) -> AVLTreeSetStPer<V>,
        ensures reachable_set@.contains(source@),
    {
        let result = graph_search(graph, source, &SelectAll);
        result.visited
    }

    // 11. derive impls in verus!

    impl<V: StT + Ord> Clone for SearchResult<V> {
        fn clone(&self) -> (out: Self) {
            SearchResult {
                visited: self.visited.clone(),
                parent: self.parent.clone(),
            }
        }
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

    impl<V: StT + Ord> std::fmt::Display for SearchResult<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SearchResult(visited={})", self.visited.size())
        }
    }

    impl std::fmt::Debug for SelectAll {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SelectAll")
        }
    }

    impl std::fmt::Display for SelectAll {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SelectAll")
        }
    }

    impl std::fmt::Debug for SelectOne {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SelectOne")
        }
    }

    impl std::fmt::Display for SelectOne {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SelectOne")
        }
    }
}
