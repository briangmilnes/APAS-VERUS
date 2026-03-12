//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 53: Generic Graph Search (ephemeral, single-threaded).

pub mod GraphSearchStEph {

    use vstd::prelude::*;
    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::AVLTreeSeqStEphTrait;
    use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::accept::accept;

    verus! {

    // 4. type definitions
    pub struct SearchResult<V: StT + Ord> {
        pub visited: AVLTreeSetStEph<V>,
        pub parent: Option<AVLTreeSetStEph<Pair<V, V>>>,
    }

    pub struct SelectAll;
    pub struct SelectOne;

    // 8. traits
    pub trait SelectionStrategy<V: StT + Ord> {
        fn select(&self, frontier: &AVLTreeSetStEph<V>) -> (selected: (AVLTreeSetStEph<V>, B))
            ensures selected.0@.subset_of(frontier@);
    }

    pub trait GraphSearchStEphTrait<V: StT + Ord> {
        /// - APAS: (no explicit cost; Theorem 53.1: ≤ |V| rounds)
        /// - Claude-Opus-4.6: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — sequential; AVL set ops add log factor.
        fn graph_search<G, S>(graph: &G, source: V, strategy: &S)                         -> (search: SearchResult<V>)
        where
            G: Fn(&V) -> AVLTreeSetStEph<V>,
            S: SelectionStrategy<V>,
            ensures search.visited@.contains(source@);

        /// - APAS: (no explicit cost; Theorem 53.1: ≤ |V| rounds)
        /// - Claude-Opus-4.6: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — sequential; AVL set ops add log factor.
        fn graph_search_multi<G, S>(graph: &G, sources: AVLTreeSetStEph<V>, strategy: &S) -> (search: SearchResult<V>)
        where
            G: Fn(&V) -> AVLTreeSetStEph<V>,
            S: SelectionStrategy<V>,
            ensures sources@.subset_of(search.visited@);

        /// - APAS: (no explicit cost; Theorem 53.1: ≤ |V| rounds)
        /// - Claude-Opus-4.6: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — sequential; uses SelectAll (BFS).
        fn reachable<G>(graph: &G, source: V)                                             -> (reachable_set: AVLTreeSetStEph<V>)
        where
            G: Fn(&V) -> AVLTreeSetStEph<V>,
            ensures reachable_set@.contains(source@);
    }

    // 9. impls
    impl<V: StT + Ord> SelectionStrategy<V> for SelectAll {
        fn select(&self, frontier: &AVLTreeSetStEph<V>) -> (selected: (AVLTreeSetStEph<V>, B)) { (frontier.clone(), false) }
    }

    impl<V: StT + Ord> SelectionStrategy<V> for SelectOne {
        fn select(&self, frontier: &AVLTreeSetStEph<V>) -> (selected: (AVLTreeSetStEph<V>, B)) {
            if frontier.size() == 0 {
                (AVLTreeSetStEph::empty(), false)
            } else {
                let seq = frontier.to_seq();
                assert(seq@.len() > 0) by {
                    if seq@.len() == 0 {
                        assert(seq@.to_set() =~= Set::empty());
                    }
                }
                let first_ref = seq.nth(0);
                let first = first_ref.clone();
                proof { assume(first@ == first_ref@); }  // accept hole: V::clone external_body
                assert(frontier@.contains(first@));
                let result = AVLTreeSetStEph::singleton(first);
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

    pub fn graph_search<V: StT + Ord, G, S>(graph: &G, source: V, strategy: &S) -> (search: SearchResult<V>)
    where
        G: Fn(&V) -> AVLTreeSetStEph<V>,
        S: SelectionStrategy<V>,
        ensures search.visited@.contains(source@),
    {
        let sources = AVLTreeSetStEph::singleton(source);
        graph_search_multi(graph, sources, strategy)
    }

    
    #[verifier::external_body]
    fn graph_search_explore<V: StT + Ord, G: Fn(&V) -> AVLTreeSetStEph<V>, S: SelectionStrategy<V>>(
        graph: &G,
        strategy: &S,
        visited: AVLTreeSetStEph<V>,
        frontier: AVLTreeSetStEph<V>,
    ) -> (visited_all: AVLTreeSetStEph<V>)
        ensures visited@.subset_of(visited_all@), frontier@.subset_of(visited_all@),
    {
        if frontier.size() == 0 {
            return visited;
        }

        let (selected, _) = strategy.select(&frontier);
        let visited_new = visited.union(&selected);

        let mut new_neighbors = AVLTreeSetStEph::empty();
        let selected_seq = selected.to_seq();
        let len = selected_seq.length();
        let mut i: usize = 0;
        while i < len
            invariant
                i <= len,
            decreases len - i,
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
    ) -> (search: SearchResult<V>)
    where
        G: Fn(&V) -> AVLTreeSetStEph<V>,
        S: SelectionStrategy<V>,
        ensures sources@.subset_of(search.visited@),
    {
        let visited = graph_search_explore(graph, strategy, AVLTreeSetStEph::empty(), sources);
        SearchResult { visited, parent: None }
    }

    impl<V: StT + Ord> GraphSearchStEphTrait<V> for SearchResult<V> {
        fn graph_search<G, S>(graph: &G, source: V, strategy: &S) -> (search: SearchResult<V>)
        where G: Fn(&V) -> AVLTreeSetStEph<V>, S: SelectionStrategy<V>,
        { graph_search(graph, source, strategy) }

        fn graph_search_multi<G, S>(graph: &G, sources: AVLTreeSetStEph<V>, strategy: &S) -> (search: SearchResult<V>)
        where G: Fn(&V) -> AVLTreeSetStEph<V>, S: SelectionStrategy<V>,
        { graph_search_multi(graph, sources, strategy) }

        fn reachable<G>(graph: &G, source: V) -> (reachable_set: AVLTreeSetStEph<V>)
        where G: Fn(&V) -> AVLTreeSetStEph<V>,
        { reachable(graph, source) }
    }

    /// Find all vertices reachable from source (Problem 53.2) using SelectAll (BFS).
    /// - APAS: (no explicit cost; Theorem 53.1: ≤ |V| rounds)
    /// - Claude-Opus-4.6: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — delegates to graph_search with SelectAll.
    pub fn reachable<V: StT + Ord, G>(graph: &G, source: V) -> (reachable_set: AVLTreeSetStEph<V>)
    where
        G: Fn(&V) -> AVLTreeSetStEph<V>,
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
