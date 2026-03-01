//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 53: Generic Graph Search (ephemeral, single-threaded).

pub mod GraphSearchStEph {

    use vstd::prelude::*;
    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::AVLTreeSeqStEphTrait;
    use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
    use crate::Types::Types::*;

    verus! {

    // 4. type definitions
    #[derive(Clone)]
    pub struct SearchResult<V: StT + Ord> {
        pub visited: AVLTreeSetStEph<V>,
        pub parent: Option<AVLTreeSetStEph<Pair<V, V>>>,
    }

    pub struct SelectAll;
    pub struct SelectOne;

    // 8. traits
    pub trait SelectionStrategy<V: StT + Ord> {
        fn select(&self, frontier: &AVLTreeSetStEph<V>) -> (AVLTreeSetStEph<V>, B);
    }

    pub trait GraphSearchStEphTrait<V: StT + Ord> {
        /// - APAS: (no explicit cost; Theorem 53.1: ≤ |V| rounds)
        /// - Claude-Opus-4.6: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — sequential; AVL set ops add log factor.
        fn graph_search<G, S>(graph: &G, source: V, strategy: &S)                         -> SearchResult<V>
        where
            G: Fn(&V) -> AVLTreeSetStEph<V>,
            S: SelectionStrategy<V>;

        /// - APAS: (no explicit cost; Theorem 53.1: ≤ |V| rounds)
        /// - Claude-Opus-4.6: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — sequential; AVL set ops add log factor.
        fn graph_search_multi<G, S>(graph: &G, sources: AVLTreeSetStEph<V>, strategy: &S) -> SearchResult<V>
        where
            G: Fn(&V) -> AVLTreeSetStEph<V>,
            S: SelectionStrategy<V>;

        /// - APAS: (no explicit cost; Theorem 53.1: ≤ |V| rounds)
        /// - Claude-Opus-4.6: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — sequential; uses SelectAll (BFS).
        fn reachable<G>(graph: &G, source: V)                                             -> AVLTreeSetStEph<V>
        where
            G: Fn(&V) -> AVLTreeSetStEph<V>;
    }

    // 9. impls
    impl<V: StT + Ord> SelectionStrategy<V> for SelectAll {
        fn select(&self, frontier: &AVLTreeSetStEph<V>) -> (AVLTreeSetStEph<V>, B) { (frontier.clone(), false) }
    }

    impl<V: StT + Ord> SelectionStrategy<V> for SelectOne {
        fn select(&self, frontier: &AVLTreeSetStEph<V>) -> (AVLTreeSetStEph<V>, B) {
            if frontier.size() == 0 {
                (AVLTreeSetStEph::empty(), false)
            } else {
                let seq = frontier.to_seq();
                let first = seq.nth(0).clone();
                (AVLTreeSetStEph::singleton(first), false)
            }
        }
    }

    pub fn graph_search<V: StT + Ord, G, S>(graph: &G, source: V, strategy: &S) -> SearchResult<V>
    where
        G: Fn(&V) -> AVLTreeSetStEph<V>,
        S: SelectionStrategy<V>,
    {
        let sources = AVLTreeSetStEph::singleton(source);
        graph_search_multi(graph, sources, strategy)
    }

    /// Recursive graph exploration helper (Algorithm 53.1 loop body).
    fn graph_search_explore<V: StT + Ord, G: Fn(&V) -> AVLTreeSetStEph<V>, S: SelectionStrategy<V>>(
        graph: &G,
        strategy: &S,
        visited: AVLTreeSetStEph<V>,
        frontier: AVLTreeSetStEph<V>,
    ) -> AVLTreeSetStEph<V>
    {
        if frontier.size() == 0 {
            return visited;
        }

        let (selected, _) = strategy.select(&frontier);
        let visited_new = visited.union(&selected);

        let mut new_neighbors = AVLTreeSetStEph::empty();
        let selected_seq = selected.to_seq();
        let mut i: usize = 0;
        while i < selected_seq.length()
        {
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
        sources: AVLTreeSetStEph<V>,
        strategy: &S,
    ) -> SearchResult<V>
    where
        G: Fn(&V) -> AVLTreeSetStEph<V>,
        S: SelectionStrategy<V>,
    {
        let visited = graph_search_explore(graph, strategy, AVLTreeSetStEph::empty(), sources);
        SearchResult { visited, parent: None }
    }

    /// Find all vertices reachable from source (Problem 53.2) using SelectAll (BFS).
    /// - APAS: (no explicit cost; Theorem 53.1: ≤ |V| rounds)
    /// - Claude-Opus-4.6: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — delegates to graph_search with SelectAll.
    pub fn reachable<V: StT + Ord, G>(graph: &G, source: V) -> AVLTreeSetStEph<V>
    where
        G: Fn(&V) -> AVLTreeSetStEph<V>,
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
