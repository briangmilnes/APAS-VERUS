//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 53: Generic Graph Search (persistent, single-threaded).
//!
//! Implements Algorithm 53.4 - Generic Graph Search with pluggable frontier selection.

pub mod GraphSearchStPer {

    use vstd::prelude::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::AVLTreeSeqStPerTrait;
    #[cfg(verus_keep_ghost)]
    use crate::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent;
    use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
    use crate::Types::Types::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;

    verus! {

    broadcast use crate::vstdplus::feq::feq::group_feq_axioms;

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
        fn select(&self, frontier: &AVLTreeSetStPer<V>) -> (selected: (AVLTreeSetStPer<V>, bool))
            requires
                frontier.spec_avltreesetstper_wf(),
                obeys_feq_clone::<V>(),
            ensures selected.0@.subset_of(frontier@);
    }

    pub trait GraphSearchStPerTrait<V: StT + Ord> {
        /// - APAS: (no explicit cost; Theorem 53.1: ≤ |V| rounds)
        /// - Claude-Opus-4.6: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — sequential; AVL set ops add log factor.
        fn graph_search<G, S>(graph: &G, source: V, strategy: &S, Ghost(vertex_universe): Ghost<Set<<V as View>::V>>) -> (search: SearchResult<V>)
        where
            G: Fn(&V) -> AVLTreeSetStPer<V>,
            S: SelectionStrategy<V>,
            requires
                forall|v: &V| #[trigger] graph.requires((v,)),
                forall|v: &V, r: AVLTreeSetStPer<V>| #[trigger] graph.ensures((v,), r) ==> r.spec_avltreesetstper_wf(),
                vertex_universe.finite(),
                vertex_universe.len() + vertex_universe.len() < usize::MAX as nat,
                vertex_universe.contains(source@),
                forall|v: &V, r: AVLTreeSetStPer<V>| #[trigger] graph.ensures((v,), r) ==> r@.subset_of(vertex_universe),
                vstd::laws_cmp::obeys_cmp_spec::<V>(),
                view_ord_consistent::<V>(),
            ensures search.visited@.contains(source@);

        /// - APAS: (no explicit cost; Theorem 53.1: ≤ |V| rounds)
        /// - Claude-Opus-4.6: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — sequential; AVL set ops add log factor.
        fn graph_search_multi<G, S>(graph: &G, sources: AVLTreeSetStPer<V>, strategy: &S, Ghost(vertex_universe): Ghost<Set<<V as View>::V>>) -> (search: SearchResult<V>)
        where
            G: Fn(&V) -> AVLTreeSetStPer<V>,
            S: SelectionStrategy<V>,
            requires
                sources.spec_avltreesetstper_wf(),
                forall|v: &V| #[trigger] graph.requires((v,)),
                forall|v: &V, r: AVLTreeSetStPer<V>| #[trigger] graph.ensures((v,), r) ==> r.spec_avltreesetstper_wf(),
                vertex_universe.finite(),
                vertex_universe.len() + vertex_universe.len() < usize::MAX as nat,
                sources@.subset_of(vertex_universe),
                forall|v: &V, r: AVLTreeSetStPer<V>| #[trigger] graph.ensures((v,), r) ==> r@.subset_of(vertex_universe),
                vstd::laws_cmp::obeys_cmp_spec::<V>(),
                view_ord_consistent::<V>(),
            ensures sources@.subset_of(search.visited@);

        /// - APAS: (no explicit cost; Theorem 53.1: ≤ |V| rounds)
        /// - Claude-Opus-4.6: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — sequential; uses SelectAll (BFS).
        fn reachable<G>(graph: &G, source: V, Ghost(vertex_universe): Ghost<Set<<V as View>::V>>) -> (reachable_set: AVLTreeSetStPer<V>)
        where
            G: Fn(&V) -> AVLTreeSetStPer<V>,
            requires
                forall|v: &V| #[trigger] graph.requires((v,)),
                forall|v: &V, r: AVLTreeSetStPer<V>| #[trigger] graph.ensures((v,), r) ==> r.spec_avltreesetstper_wf(),
                vertex_universe.finite(),
                vertex_universe.len() + vertex_universe.len() < usize::MAX as nat,
                vertex_universe.contains(source@),
                forall|v: &V, r: AVLTreeSetStPer<V>| #[trigger] graph.ensures((v,), r) ==> r@.subset_of(vertex_universe),
                vstd::laws_cmp::obeys_cmp_spec::<V>(),
                view_ord_consistent::<V>(),
            ensures reachable_set@.contains(source@);
    }

    // 9. impls
    impl<V: StT + Ord> SelectionStrategy<V> for SelectAll {
        fn select(&self, frontier: &AVLTreeSetStPer<V>) -> (selected: (AVLTreeSetStPer<V>, bool)) { (frontier.clone(), false) }
    }

    impl<V: StT + Ord> SelectionStrategy<V> for SelectOne {
        fn select(&self, frontier: &AVLTreeSetStPer<V>) -> (selected: (AVLTreeSetStPer<V>, bool)) {
            let n = frontier.size();
            if n == 0 {
                (AVLTreeSetStPer::empty(), false)
            } else {
                let seq = frontier.to_seq();
                proof {
                    // Prove seq is non-empty from frontier being non-empty.
                    assert(seq@.to_set() =~= frontier@);
                    assert(frontier@.len() > 0);
                    if seq@.len() == 0 {
                        assert(seq@.to_set() =~= Set::<<V as View>::V>::empty());
                    }
                }
                let first_ref = seq.nth(0);
                let first = first_ref.clone();
                proof { assert(cloned(*first_ref, first)); }
                assert(seq@.to_set() =~= frontier@);
                assert(frontier@.contains(first@));
                let result = AVLTreeSetStPer::singleton(first);
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

    pub fn graph_search<V: StT + Ord, G, S>(
        graph: &G, source: V, strategy: &S,
        Ghost(vertex_universe): Ghost<Set<<V as View>::V>>,
    ) -> (search: SearchResult<V>)
    where
        G: Fn(&V) -> AVLTreeSetStPer<V>,
        S: SelectionStrategy<V>,
        requires
            forall|v: &V| #[trigger] graph.requires((v,)),
            forall|v: &V, r: AVLTreeSetStPer<V>| #[trigger] graph.ensures((v,), r) ==> r.spec_avltreesetstper_wf(),
            vertex_universe.finite(),
            vertex_universe.len() + vertex_universe.len() < usize::MAX as nat,
            vertex_universe.contains(source@),
            forall|v: &V, r: AVLTreeSetStPer<V>| #[trigger] graph.ensures((v,), r) ==> r@.subset_of(vertex_universe),
            vstd::laws_cmp::obeys_cmp_spec::<V>(),
            view_ord_consistent::<V>(),
        ensures search.visited@.contains(source@),
    {
        let sources = AVLTreeSetStPer::singleton(source);
        proof {
            assert(sources@.subset_of(vertex_universe)) by {
                assert forall|a: <V as View>::V| sources@.contains(a) implies #[trigger] vertex_universe.contains(a) by {
                    assert(sources@ == Set::<<V as View>::V>::empty().insert(source@));
                }
            }
        }
        graph_search_multi(graph, sources, strategy, Ghost(vertex_universe))
    }

    /// Graph exploration loop (Algorithm 53.4).
    #[verifier::exec_allows_no_decreases_clause]
    fn graph_search_explore<V: StT + Ord, G: Fn(&V) -> AVLTreeSetStPer<V>, S: SelectionStrategy<V>>(
        graph: &G,
        strategy: &S,
        visited_init: AVLTreeSetStPer<V>,
        frontier_init: AVLTreeSetStPer<V>,
        Ghost(vertex_universe): Ghost<Set<<V as View>::V>>,
    ) -> (visited_all: AVLTreeSetStPer<V>)
        requires
            visited_init.spec_avltreesetstper_wf(),
            frontier_init.spec_avltreesetstper_wf(),
            forall|v: &V| #[trigger] graph.requires((v,)),
            forall|v: &V, r: AVLTreeSetStPer<V>| #[trigger] graph.ensures((v,), r) ==> r.spec_avltreesetstper_wf(),
            vertex_universe.finite(),
            vertex_universe.len() + vertex_universe.len() < usize::MAX as nat,
            visited_init@.subset_of(vertex_universe),
            frontier_init@.subset_of(vertex_universe),
            forall|v: &V, r: AVLTreeSetStPer<V>| #[trigger] graph.ensures((v,), r) ==> r@.subset_of(vertex_universe),
            vstd::laws_cmp::obeys_cmp_spec::<V>(),
            view_ord_consistent::<V>(),
        ensures
            visited_init@.subset_of(visited_all@),
            frontier_init@.subset_of(visited_all@),
    {
        let mut visited = visited_init;
        let mut frontier = frontier_init;

        while frontier.size() > 0
            invariant
                visited.spec_avltreesetstper_wf(),
                frontier.spec_avltreesetstper_wf(),
                visited_init@.subset_of(visited@),
                frontier_init@.subset_of(visited@.union(frontier@)),
                forall|v: &V| #[trigger] graph.requires((v,)),
                forall|v: &V, r: AVLTreeSetStPer<V>| #[trigger] graph.ensures((v,), r) ==> r.spec_avltreesetstper_wf(),
                vertex_universe.finite(),
                vertex_universe.len() + vertex_universe.len() < usize::MAX as nat,
                visited@.subset_of(vertex_universe),
                frontier@.subset_of(vertex_universe),
                forall|v: &V, r: AVLTreeSetStPer<V>| #[trigger] graph.ensures((v,), r) ==> r@.subset_of(vertex_universe),
                vstd::laws_cmp::obeys_cmp_spec::<V>(),
                view_ord_consistent::<V>(),
        {
            proof {
                vstd::set_lib::lemma_len_subset(visited@, vertex_universe);
                vstd::set_lib::lemma_len_subset(frontier@, vertex_universe);
            }
            let visited_new = visited.union(&frontier);

            let mut new_neighbors = AVLTreeSetStPer::empty();
            let frontier_seq = frontier.to_seq();
            let nlen = frontier_seq.length();
            let mut i: usize = 0;
            while i < nlen
                invariant
                    i <= nlen,
                    nlen as nat == frontier_seq.spec_seq().len(),
                    frontier_seq.spec_avltreeseqstper_wf(),
                    new_neighbors.spec_avltreesetstper_wf(),
                    forall|v: &V| #[trigger] graph.requires((v,)),
                    forall|v: &V, r: AVLTreeSetStPer<V>| #[trigger] graph.ensures((v,), r) ==> r.spec_avltreesetstper_wf(),
                    new_neighbors@.subset_of(vertex_universe),
                    vertex_universe.finite(),
                    vertex_universe.len() + vertex_universe.len() < usize::MAX as nat,
                    forall|v: &V, r: AVLTreeSetStPer<V>| #[trigger] graph.ensures((v,), r) ==> r@.subset_of(vertex_universe),
                    vstd::laws_cmp::obeys_cmp_spec::<V>(),
                    view_ord_consistent::<V>(),
                decreases nlen - i,
            {
                let v = frontier_seq.nth(i);
                let neighbors = graph(v);
                proof {
                    vstd::set_lib::lemma_len_subset(new_neighbors@, vertex_universe);
                    vstd::set_lib::lemma_len_subset(neighbors@, vertex_universe);
                }
                new_neighbors = new_neighbors.union(&neighbors);
                i = i + 1;
            }

            let frontier_new = new_neighbors.difference(&visited_new);

            proof {
                assert(visited_new@.subset_of(vertex_universe)) by {
                    assert forall|a: <V as View>::V| visited_new@.contains(a)
                        implies #[trigger] vertex_universe.contains(a) by {}
                }
                assert(frontier_new@.subset_of(vertex_universe)) by {
                    assert forall|a: <V as View>::V| frontier_new@.contains(a)
                        implies #[trigger] vertex_universe.contains(a) by {
                        assert(new_neighbors@.contains(a));
                    }
                }
            }

            visited = visited_new;
            frontier = frontier_new;
        }

        visited
    }

    /// Generic graph search starting from multiple sources (Exercise 53.3).
    pub fn graph_search_multi<V: StT + Ord, G, S>(
        graph: &G,
        sources: AVLTreeSetStPer<V>,
        strategy: &S,
        Ghost(vertex_universe): Ghost<Set<<V as View>::V>>,
    ) -> (search: SearchResult<V>)
    where
        G: Fn(&V) -> AVLTreeSetStPer<V>,
        S: SelectionStrategy<V>,
        requires
            sources.spec_avltreesetstper_wf(),
            forall|v: &V| #[trigger] graph.requires((v,)),
            forall|v: &V, r: AVLTreeSetStPer<V>| #[trigger] graph.ensures((v,), r) ==> r.spec_avltreesetstper_wf(),
            vertex_universe.finite(),
            vertex_universe.len() + vertex_universe.len() < usize::MAX as nat,
            sources@.subset_of(vertex_universe),
            forall|v: &V, r: AVLTreeSetStPer<V>| #[trigger] graph.ensures((v,), r) ==> r@.subset_of(vertex_universe),
            vstd::laws_cmp::obeys_cmp_spec::<V>(),
            view_ord_consistent::<V>(),
        ensures sources@.subset_of(search.visited@),
    {
        let visited = graph_search_explore(graph, strategy, AVLTreeSetStPer::empty(), sources, Ghost(vertex_universe));
        SearchResult { visited, parent: None }
    }

    impl<V: StT + Ord> GraphSearchStPerTrait<V> for SearchResult<V> {
        fn graph_search<G, S>(graph: &G, source: V, strategy: &S, Ghost(vertex_universe): Ghost<Set<<V as View>::V>>) -> (search: SearchResult<V>)
        where G: Fn(&V) -> AVLTreeSetStPer<V>, S: SelectionStrategy<V>,
        { graph_search(graph, source, strategy, Ghost(vertex_universe)) }

        fn graph_search_multi<G, S>(graph: &G, sources: AVLTreeSetStPer<V>, strategy: &S, Ghost(vertex_universe): Ghost<Set<<V as View>::V>>) -> (search: SearchResult<V>)
        where G: Fn(&V) -> AVLTreeSetStPer<V>, S: SelectionStrategy<V>,
        { graph_search_multi(graph, sources, strategy, Ghost(vertex_universe)) }

        fn reachable<G>(graph: &G, source: V, Ghost(vertex_universe): Ghost<Set<<V as View>::V>>) -> (reachable_set: AVLTreeSetStPer<V>)
        where G: Fn(&V) -> AVLTreeSetStPer<V>,
        { reachable(graph, source, Ghost(vertex_universe)) }
    }

    /// Find all vertices reachable from source (Problem 53.2) using SelectAll (BFS).
    /// - APAS: (no explicit cost; Theorem 53.1: ≤ |V| rounds)
    /// - Claude-Opus-4.6: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — delegates to graph_search with SelectAll.
    pub fn reachable<V: StT + Ord, G>(
        graph: &G, source: V,
        Ghost(vertex_universe): Ghost<Set<<V as View>::V>>,
    ) -> (reachable_set: AVLTreeSetStPer<V>)
    where
        G: Fn(&V) -> AVLTreeSetStPer<V>,
        requires
            forall|v: &V| #[trigger] graph.requires((v,)),
            forall|v: &V, r: AVLTreeSetStPer<V>| #[trigger] graph.ensures((v,), r) ==> r.spec_avltreesetstper_wf(),
            vertex_universe.finite(),
            vertex_universe.len() + vertex_universe.len() < usize::MAX as nat,
            vertex_universe.contains(source@),
            forall|v: &V, r: AVLTreeSetStPer<V>| #[trigger] graph.ensures((v,), r) ==> r@.subset_of(vertex_universe),
            vstd::laws_cmp::obeys_cmp_spec::<V>(),
            view_ord_consistent::<V>(),
        ensures reachable_set@.contains(source@),
    {
        let result = graph_search(graph, source, &SelectAll, Ghost(vertex_universe));
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
