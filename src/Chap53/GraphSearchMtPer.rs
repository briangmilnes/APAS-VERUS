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
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;

    verus! {

    broadcast use crate::vstdplus::feq::feq::group_feq_axioms;

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
            requires obeys_feq_clone::<V>(),
            ensures selected.0@.subset_of(frontier@);
    }

    pub trait GraphSearchMtPerTrait<V: StTInMtT + Ord + 'static> {
        /// - APAS: (no explicit cost; Theorem 53.1: ≤ |V| rounds; BFS rounds parallelizable)
        /// - Claude-Opus-4.6: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — neighbor loop is sequential despite parallel set ops.
        fn graph_search<G, S>(graph: &G, source: V, strategy: &S)                         -> (search: SearchResult<V>)
        where
            G: Fn(&V) -> AVLTreeSetMtPer<V>,
            S: SelectionStrategy<V>,
            requires forall|v: &V| #[trigger] graph.requires((v,)),
            ensures search.visited@.contains(source@);

        /// - APAS: (no explicit cost; Theorem 53.1: ≤ |V| rounds)
        /// - Claude-Opus-4.6: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — neighbor loop is sequential despite parallel set ops.
        fn graph_search_multi<G, S>(graph: &G, sources: AVLTreeSetMtPer<V>, strategy: &S) -> (search: SearchResult<V>)
        where
            G: Fn(&V) -> AVLTreeSetMtPer<V>,
            S: SelectionStrategy<V>,
            requires
                forall|v: &V| #[trigger] graph.requires((v,)),
                sources.spec_avltreesetmtper_wf(),
            ensures sources@.subset_of(search.visited@);

        /// - APAS: (no explicit cost; Theorem 53.1: ≤ |V| rounds)
        /// - Claude-Opus-4.6: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — sequential control flow; uses SelectAll (BFS).
        fn reachable<G>(graph: &G, source: V)                                             -> (reachable_set: AVLTreeSetMtPer<V>)
        where
            G: Fn(&V) -> AVLTreeSetMtPer<V>,
            requires forall|v: &V| #[trigger] graph.requires((v,)),
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
                proof { assert(cloned(*first_ref, first)); }
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
        requires forall|v: &V| #[trigger] graph.requires((v,)),
        ensures search.visited@.contains(source@),
    {
        let sources = AVLTreeSetMtPer::singleton(source);
        graph_search_multi(graph, sources, strategy)
    }

    /// Graph exploration loop (Algorithm 53.4).
    #[verifier::exec_allows_no_decreases_clause]
    fn graph_search_explore<V: StTInMtT + Ord + 'static, G: Fn(&V) -> AVLTreeSetMtPer<V>, S: SelectionStrategy<V>>(
        graph: &G,
        strategy: &S,
        visited_init: AVLTreeSetMtPer<V>,
        frontier_init: AVLTreeSetMtPer<V>,
    ) -> (visited_all: AVLTreeSetMtPer<V>)
        requires
            forall|v: &V| #[trigger] graph.requires((v,)),
            visited_init.spec_avltreesetmtper_wf(),
            frontier_init.spec_avltreesetmtper_wf(),
        ensures
            visited_init@.subset_of(visited_all@),
            frontier_init@.subset_of(visited_all@),
            visited_all.spec_avltreesetmtper_wf(),
    {
        let mut visited = visited_init;
        let mut frontier = frontier_init;

        while frontier.size() > 0
            invariant
                visited_init@.subset_of(visited@),
                frontier_init@.subset_of(visited@.union(frontier@)),
                forall|v: &V| #[trigger] graph.requires((v,)),
                visited.spec_avltreesetmtper_wf(),
                frontier.spec_avltreesetmtper_wf(),
        {
            let visited_new = visited.union(&frontier);

            let mut new_neighbors = AVLTreeSetMtPer::empty();
            let nlen = frontier.elements.length();
            let mut i: usize = 0;
            while i < nlen
                invariant
                    i <= nlen,
                    nlen as nat == frontier.elements.spec_seq().len(),
                    frontier.spec_avltreesetmtper_wf(),
                    forall|v: &V| #[trigger] graph.requires((v,)),
                decreases nlen - i,
            {
                let v = frontier.elements.nth(i);
                let neighbors = graph(v);
                new_neighbors = new_neighbors.union(&neighbors);
                i = i + 1;
            }

            let frontier_new = new_neighbors.difference(&visited_new);

            visited = visited_new;
            frontier = frontier_new;
        }

        visited
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
        requires
            forall|v: &V| #[trigger] graph.requires((v,)),
            sources.spec_avltreesetmtper_wf(),
        ensures sources@.subset_of(search.visited@),
    {
        let visited = graph_search_explore(graph, strategy, AVLTreeSetMtPer::empty(), sources);
        SearchResult { visited, parent: None }
    }

    impl<V: StTInMtT + Ord + 'static> GraphSearchMtPerTrait<V> for SearchResult<V> {
        fn graph_search<G, S>(graph: &G, source: V, strategy: &S) -> (search: SearchResult<V>)
        where G: Fn(&V) -> AVLTreeSetMtPer<V>, S: SelectionStrategy<V>,
        { crate::Chap53::GraphSearchMtPer::GraphSearchMtPer::graph_search(graph, source, strategy) }

        fn graph_search_multi<G, S>(graph: &G, sources: AVLTreeSetMtPer<V>, strategy: &S) -> (search: SearchResult<V>)
        where G: Fn(&V) -> AVLTreeSetMtPer<V>, S: SelectionStrategy<V>,
        {
            crate::Chap53::GraphSearchMtPer::GraphSearchMtPer::graph_search_multi(graph, sources, strategy)
        }

        fn reachable<G>(graph: &G, source: V) -> (reachable_set: AVLTreeSetMtPer<V>)
        where G: Fn(&V) -> AVLTreeSetMtPer<V>,
        { crate::Chap53::GraphSearchMtPer::GraphSearchMtPer::reachable(graph, source) }
    }

    /// Find all vertices reachable from source (Problem 53.2) using SelectAll (BFS).
    /// - APAS: (no explicit cost; Theorem 53.1: ≤ |V| rounds)
    /// - Claude-Opus-4.6: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — delegates to graph_search with SelectAll.
    pub fn reachable<V: StTInMtT + Ord + 'static, G>(graph: &G, source: V) -> (reachable_set: AVLTreeSetMtPer<V>)
    where
        G: Fn(&V) -> AVLTreeSetMtPer<V>,
        requires forall|v: &V| #[trigger] graph.requires((v,)),
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
