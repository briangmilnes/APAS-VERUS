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
    use crate::vstdplus::accept::accept;

    verus! {

    // 4. type definitions
    #[derive(Clone)]
    #[verifier::reject_recursive_types(V)]
    pub struct SearchResult<V: StTInMtT + Ord + 'static> {
        pub visited: AVLTreeSetMtPer<V>,
        pub parent: Option<AVLTreeSetMtPer<Pair<V, V>>>,
    }

    pub struct SelectAll;
    pub struct SelectOne;

    // 8. traits
    pub trait SelectionStrategy<V: StTInMtT + Ord + 'static> {
        fn select(&self, frontier: &AVLTreeSetMtPer<V>) -> (selected: (AVLTreeSetMtPer<V>, B))
            ensures selected.0@.subset_of(frontier@);
    }

    pub trait GraphSearchMtPerTrait<V: StTInMtT + Ord + 'static> {
        /// - APAS: (no explicit cost; Theorem 53.1: ≤ |V| rounds; BFS rounds parallelizable)
        /// - Claude-Opus-4.6: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — neighbor loop is sequential despite parallel set ops.
        fn graph_search<G, S>(graph: &G, source: V, strategy: &S)                         -> (search: SearchResult<V>)
        where
            G: Fn(&V) -> AVLTreeSetMtPer<V>,
            S: SelectionStrategy<V>,
            ensures search.visited@.contains(source@);

        /// - APAS: (no explicit cost; Theorem 53.1: ≤ |V| rounds)
        /// - Claude-Opus-4.6: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — neighbor loop is sequential despite parallel set ops.
        fn graph_search_multi<G, S>(graph: &G, sources: AVLTreeSetMtPer<V>, strategy: &S) -> (search: SearchResult<V>)
        where
            G: Fn(&V) -> AVLTreeSetMtPer<V>,
            S: SelectionStrategy<V>,
            ensures sources@.subset_of(search.visited@);

        /// - APAS: (no explicit cost; Theorem 53.1: ≤ |V| rounds)
        /// - Claude-Opus-4.6: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — sequential control flow; uses SelectAll (BFS).
        fn reachable<G>(graph: &G, source: V)                                             -> (reachable_set: AVLTreeSetMtPer<V>)
        where
            G: Fn(&V) -> AVLTreeSetMtPer<V>,
            ensures reachable_set@.contains(source@);
    }

    // 9. impls
    impl<V: StTInMtT + Ord + 'static> SelectionStrategy<V> for SelectAll {
        fn select(&self, frontier: &AVLTreeSetMtPer<V>) -> (selected: (AVLTreeSetMtPer<V>, B)) { (frontier.clone(), false) }
    }

    impl<V: StTInMtT + Ord + 'static> SelectionStrategy<V> for SelectOne {
        fn select(&self, frontier: &AVLTreeSetMtPer<V>) -> (selected: (AVLTreeSetMtPer<V>, B)) {
            if frontier.size() == 0 {
                (AVLTreeSetMtPer::empty(), false)
            } else {
                let seq = frontier.to_seq();
                assert(seq@.len() > 0) by {
                    if seq@.len() == 0 {
                        assert(seq@.to_set() =~= Set::empty());
                    }
                }
                let first_ref = seq.nth(0);
                let first = first_ref.clone();
                proof { accept(first@ == first_ref@); }  // accept hole: V::clone external_body
                assert(frontier@.contains(first@));
                let result = AVLTreeSetMtPer::singleton(first);
                assert(result@.subset_of(frontier@)) by {
                    assert forall|a: <V as View>::V| result@.contains(a)
                        implies frontier@.contains(a) by {
                        assert(result@ == Set::<<V as View>::V>::empty().insert(first@));
                    }
                }
                (result, false)
            }
        }
    }

    pub fn graph_search<V: StTInMtT + Ord + 'static, G, S>(graph: &G, source: V, strategy: &S) -> (search: SearchResult<V>)
    where
        G: Fn(&V) -> AVLTreeSetMtPer<V>,
        S: SelectionStrategy<V>,
        ensures search.visited@.contains(source@),
    {
        let sources = AVLTreeSetMtPer::singleton(source);
        graph_search_multi(graph, sources, strategy)
    }

    /// Recursive graph exploration helper (Algorithm 53.4 loop body).
    #[verifier::external_body]
    fn graph_search_explore<V: StTInMtT + Ord + 'static, G: Fn(&V) -> AVLTreeSetMtPer<V>, S: SelectionStrategy<V>>(
        graph: &G,
        strategy: &S,
        visited: AVLTreeSetMtPer<V>,
        frontier: AVLTreeSetMtPer<V>,
    ) -> (visited_all: AVLTreeSetMtPer<V>)
        ensures visited@.subset_of(visited_all@), frontier@.subset_of(visited_all@),
    {
        if frontier.size() == 0 {
            return visited;
        }

        let (selected, _) = strategy.select(&frontier);
        let visited_new = visited.union(&selected);

        let mut new_neighbors = AVLTreeSetMtPer::empty();
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
    pub fn graph_search_multi<V: StTInMtT + Ord + 'static, G, S>(
        graph: &G,
        sources: AVLTreeSetMtPer<V>,
        strategy: &S,
    ) -> (search: SearchResult<V>)
    where
        G: Fn(&V) -> AVLTreeSetMtPer<V>,
        S: SelectionStrategy<V>,
        ensures sources@.subset_of(search.visited@),
    {
        let visited = graph_search_explore(graph, strategy, AVLTreeSetMtPer::empty(), sources);
        SearchResult { visited, parent: None }
    }

    impl<V: StTInMtT + Ord + 'static> GraphSearchMtPerTrait<V> for SearchResult<V> {
        fn graph_search<G, S>(graph: &G, source: V, strategy: &S) -> (search: SearchResult<V>)
        where G: Fn(&V) -> AVLTreeSetMtPer<V>, S: SelectionStrategy<V>,
        { graph_search(graph, source, strategy) }

        fn graph_search_multi<G, S>(graph: &G, sources: AVLTreeSetMtPer<V>, strategy: &S) -> (search: SearchResult<V>)
        where G: Fn(&V) -> AVLTreeSetMtPer<V>, S: SelectionStrategy<V>,
        { graph_search_multi(graph, sources, strategy) }

        fn reachable<G>(graph: &G, source: V) -> (reachable_set: AVLTreeSetMtPer<V>)
        where G: Fn(&V) -> AVLTreeSetMtPer<V>,
        { reachable(graph, source) }
    }

    /// Find all vertices reachable from source (Problem 53.2) using SelectAll (BFS).
    /// - APAS: (no explicit cost; Theorem 53.1: ≤ |V| rounds)
    /// - Claude-Opus-4.6: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — delegates to graph_search with SelectAll.
    pub fn reachable<V: StTInMtT + Ord + 'static, G>(graph: &G, source: V) -> (reachable_set: AVLTreeSetMtPer<V>)
    where
        G: Fn(&V) -> AVLTreeSetMtPer<V>,
        ensures reachable_set@.contains(source@),
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

    impl<V: StTInMtT + Ord + 'static> std::fmt::Display for SearchResult<V> {
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
