//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
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
    #[cfg(verus_keep_ghost)]
    use crate::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent;
    use crate::Types::Types::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;

    verus! {

    broadcast use crate::vstdplus::feq::feq::group_feq_axioms;

    // 4. type definitions
    #[verifier::reject_recursive_types(V)]
    pub struct SearchResult<V: StTInMtT + Ord + 'static> {
        pub visited: AVLTreeSetMtPer<V>,
        pub parent: Option<AVLTreeSetMtPer<Pair<V, V>>>,
    }

    pub struct SelectAll;
    pub struct SelectOne;

    // 8. traits
    pub trait SelectionStrategy<V: StTInMtT + Ord + 'static> {
        fn select(&self, frontier: &AVLTreeSetMtPer<V>) -> (selected: (AVLTreeSetMtPer<V>, bool))
            requires obeys_feq_clone::<V>(), frontier.spec_avltreesetmtper_wf(),
            ensures selected.0@.subset_of(frontier@);
    }

    pub trait GraphSearchMtPerTrait<V: StTInMtT + Ord + 'static> {
        /// - Alg Analysis: APAS (Ch53 Thm 53.1): (no explicit cost; ≤ |V| rounds)
        /// - Alg Analysis: Code review (Claude Opus 4.6): no explicit cost in APAS — N/A
        /// - Claude-Opus-4.6: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — neighbor loop is sequential despite parallel set ops.
        fn graph_search<G, S>(graph: &G, source: V, strategy: &S, Ghost(vertex_universe): Ghost<Set<<V as View>::V>>) -> (search: SearchResult<V>)
        where
            G: Fn(&V) -> AVLTreeSetMtPer<V>,
            S: SelectionStrategy<V>,
            requires
                forall|v: &V| #[trigger] graph.requires((v,)),
                forall|v: &V, r: AVLTreeSetMtPer<V>| #[trigger] graph.ensures((v,), r) ==> r.spec_avltreesetmtper_wf(),
                vertex_universe.finite(),
                vertex_universe.len() + vertex_universe.len() < usize::MAX as nat,
                vertex_universe.contains(source@),
                forall|v: &V, r: AVLTreeSetMtPer<V>| #[trigger] graph.ensures((v,), r) ==> r@.subset_of(vertex_universe),
                vstd::laws_cmp::obeys_cmp_spec::<V>(),
                view_ord_consistent::<V>(),
            ensures search.visited@.contains(source@);

        /// - Alg Analysis: APAS (Ch53 Thm 53.1): (no explicit cost; ≤ |V| rounds)
        /// - Alg Analysis: Code review (Claude Opus 4.6): no explicit cost in APAS — N/A
        /// - Claude-Opus-4.6: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — neighbor loop is sequential despite parallel set ops.
        fn graph_search_multi<G, S>(graph: &G, sources: AVLTreeSetMtPer<V>, strategy: &S, Ghost(vertex_universe): Ghost<Set<<V as View>::V>>) -> (search: SearchResult<V>)
        where
            G: Fn(&V) -> AVLTreeSetMtPer<V>,
            S: SelectionStrategy<V>,
            requires
                sources.spec_avltreesetmtper_wf(),
                forall|v: &V| #[trigger] graph.requires((v,)),
                forall|v: &V, r: AVLTreeSetMtPer<V>| #[trigger] graph.ensures((v,), r) ==> r.spec_avltreesetmtper_wf(),
                vertex_universe.finite(),
                vertex_universe.len() + vertex_universe.len() < usize::MAX as nat,
                sources@.subset_of(vertex_universe),
                forall|v: &V, r: AVLTreeSetMtPer<V>| #[trigger] graph.ensures((v,), r) ==> r@.subset_of(vertex_universe),
                vstd::laws_cmp::obeys_cmp_spec::<V>(),
                view_ord_consistent::<V>(),
            ensures sources@.subset_of(search.visited@);

        /// - Alg Analysis: APAS (Ch53 Thm 53.1): (no explicit cost; ≤ |V| rounds)
        /// - Alg Analysis: Code review (Claude Opus 4.6): no explicit cost in APAS — N/A
        /// - Claude-Opus-4.6: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — sequential control flow; uses SelectAll (BFS).
        fn reachable<G>(graph: &G, source: V, Ghost(vertex_universe): Ghost<Set<<V as View>::V>>) -> (reachable_set: AVLTreeSetMtPer<V>)
        where
            G: Fn(&V) -> AVLTreeSetMtPer<V>,
            requires
                forall|v: &V| #[trigger] graph.requires((v,)),
                forall|v: &V, r: AVLTreeSetMtPer<V>| #[trigger] graph.ensures((v,), r) ==> r.spec_avltreesetmtper_wf(),
                vertex_universe.finite(),
                vertex_universe.len() + vertex_universe.len() < usize::MAX as nat,
                vertex_universe.contains(source@),
                forall|v: &V, r: AVLTreeSetMtPer<V>| #[trigger] graph.ensures((v,), r) ==> r@.subset_of(vertex_universe),
                vstd::laws_cmp::obeys_cmp_spec::<V>(),
                view_ord_consistent::<V>(),
            ensures reachable_set@.contains(source@);
    }

    // 9. impls
    impl<V: StTInMtT + Ord + 'static> SelectionStrategy<V> for SelectAll {
        fn select(&self, frontier: &AVLTreeSetMtPer<V>) -> (selected: (AVLTreeSetMtPer<V>, bool)) { (frontier.clone(), false) }
    }

    impl<V: StTInMtT + Ord + 'static> SelectionStrategy<V> for SelectOne {
        fn select(&self, frontier: &AVLTreeSetMtPer<V>) -> (selected: (AVLTreeSetMtPer<V>, bool)) {
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

    pub fn graph_search<V: StTInMtT + Ord + 'static, G, S>(graph: &G, source: V, strategy: &S, Ghost(vertex_universe): Ghost<Set<<V as View>::V>>) -> (search: SearchResult<V>)
    where
        G: Fn(&V) -> AVLTreeSetMtPer<V>,
        S: SelectionStrategy<V>,
        requires
            forall|v: &V| #[trigger] graph.requires((v,)),
            forall|v: &V, r: AVLTreeSetMtPer<V>| #[trigger] graph.ensures((v,), r) ==> r.spec_avltreesetmtper_wf(),
            vertex_universe.finite(),
            vertex_universe.len() + vertex_universe.len() < usize::MAX as nat,
            vertex_universe.contains(source@),
            forall|v: &V, r: AVLTreeSetMtPer<V>| #[trigger] graph.ensures((v,), r) ==> r@.subset_of(vertex_universe),
            vstd::laws_cmp::obeys_cmp_spec::<V>(),
            view_ord_consistent::<V>(),
        ensures search.visited@.contains(source@),
    {
        let sources = AVLTreeSetMtPer::singleton(source);
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
    fn graph_search_explore<V: StTInMtT + Ord + 'static, G: Fn(&V) -> AVLTreeSetMtPer<V>, S: SelectionStrategy<V>>(
        graph: &G,
        strategy: &S,
        visited_init: AVLTreeSetMtPer<V>,
        frontier_init: AVLTreeSetMtPer<V>,
        Ghost(vertex_universe): Ghost<Set<<V as View>::V>>,
    ) -> (visited_all: AVLTreeSetMtPer<V>)
        requires
            visited_init.spec_avltreesetmtper_wf(),
            frontier_init.spec_avltreesetmtper_wf(),
            forall|v: &V| #[trigger] graph.requires((v,)),
            forall|v: &V, r: AVLTreeSetMtPer<V>| #[trigger] graph.ensures((v,), r) ==> r.spec_avltreesetmtper_wf(),
            vertex_universe.finite(),
            vertex_universe.len() + vertex_universe.len() < usize::MAX as nat,
            visited_init@.subset_of(vertex_universe),
            frontier_init@.subset_of(vertex_universe),
            forall|v: &V, r: AVLTreeSetMtPer<V>| #[trigger] graph.ensures((v,), r) ==> r@.subset_of(vertex_universe),
            vstd::laws_cmp::obeys_cmp_spec::<V>(),
            view_ord_consistent::<V>(),
        ensures
            visited_init@.subset_of(visited_all@),
            frontier_init@.subset_of(visited_all@),
            visited_all.spec_avltreesetmtper_wf(),
    {
        let mut visited = visited_init;
        let mut frontier = frontier_init;

        while frontier.size() > 0
            invariant
                visited.spec_avltreesetmtper_wf(),
                frontier.spec_avltreesetmtper_wf(),
                visited_init@.subset_of(visited@),
                frontier_init@.subset_of(visited@.union(frontier@)),
                forall|v: &V| #[trigger] graph.requires((v,)),
                forall|v: &V, r: AVLTreeSetMtPer<V>| #[trigger] graph.ensures((v,), r) ==> r.spec_avltreesetmtper_wf(),
                vertex_universe.finite(),
                vertex_universe.len() + vertex_universe.len() < usize::MAX as nat,
                visited@.subset_of(vertex_universe),
                frontier@.subset_of(vertex_universe),
                forall|v: &V, r: AVLTreeSetMtPer<V>| #[trigger] graph.ensures((v,), r) ==> r@.subset_of(vertex_universe),
                vstd::laws_cmp::obeys_cmp_spec::<V>(),
                view_ord_consistent::<V>(),
        {
            proof {
                vstd::set_lib::lemma_len_subset(visited@, vertex_universe);
                vstd::set_lib::lemma_len_subset(frontier@, vertex_universe);
            }
            let visited_new = visited.union(&frontier);

            let mut new_neighbors = AVLTreeSetMtPer::empty();
            let frontier_seq = frontier.to_seq();
            let nlen = frontier_seq.length();
            let mut i: usize = 0;
            while i < nlen
                invariant
                    i <= nlen,
                    nlen as nat == frontier_seq@.len(),
                    frontier_seq.spec_avltreeseqmtper_wf(),
                    frontier.spec_avltreesetmtper_wf(),
                    new_neighbors.spec_avltreesetmtper_wf(),
                    forall|v: &V| #[trigger] graph.requires((v,)),
                    forall|v: &V, r: AVLTreeSetMtPer<V>| #[trigger] graph.ensures((v,), r) ==> r.spec_avltreesetmtper_wf(),
                    new_neighbors@.subset_of(vertex_universe),
                    vertex_universe.finite(),
                    vertex_universe.len() + vertex_universe.len() < usize::MAX as nat,
                    forall|v: &V, r: AVLTreeSetMtPer<V>| #[trigger] graph.ensures((v,), r) ==> r@.subset_of(vertex_universe),
                    vstd::laws_cmp::obeys_cmp_spec::<V>(),
                    view_ord_consistent::<V>(),
                decreases nlen - i,
            {
                let v = frontier_seq.nth(i);
                let neighbors = graph(v);
                proof {
                    // Graph closure ensures wf: invariant says graph.ensures((v,), r) ==> r.wf().
                    // After graph(v), Verus knows graph.ensures((v,), neighbors), triggering the forall.
                    assert(neighbors.spec_avltreesetmtper_wf());
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
    pub fn graph_search_multi<V: StTInMtT + Ord + 'static, G, S>(
        graph: &G,
        sources: AVLTreeSetMtPer<V>,
        strategy: &S,
        Ghost(vertex_universe): Ghost<Set<<V as View>::V>>,
    ) -> (search: SearchResult<V>)
    where
        G: Fn(&V) -> AVLTreeSetMtPer<V>,
        S: SelectionStrategy<V>,
        requires
            sources.spec_avltreesetmtper_wf(),
            forall|v: &V| #[trigger] graph.requires((v,)),
            forall|v: &V, r: AVLTreeSetMtPer<V>| #[trigger] graph.ensures((v,), r) ==> r.spec_avltreesetmtper_wf(),
            vertex_universe.finite(),
            vertex_universe.len() + vertex_universe.len() < usize::MAX as nat,
            sources@.subset_of(vertex_universe),
            forall|v: &V, r: AVLTreeSetMtPer<V>| #[trigger] graph.ensures((v,), r) ==> r@.subset_of(vertex_universe),
            vstd::laws_cmp::obeys_cmp_spec::<V>(),
            view_ord_consistent::<V>(),
        ensures sources@.subset_of(search.visited@),
    {
        let visited = graph_search_explore(graph, strategy, AVLTreeSetMtPer::empty(), sources, Ghost(vertex_universe));
        SearchResult { visited, parent: None }
    }

    impl<V: StTInMtT + Ord + 'static> GraphSearchMtPerTrait<V> for SearchResult<V> {
        fn graph_search<G, S>(graph: &G, source: V, strategy: &S, Ghost(vertex_universe): Ghost<Set<<V as View>::V>>) -> (search: SearchResult<V>)
        where G: Fn(&V) -> AVLTreeSetMtPer<V>, S: SelectionStrategy<V>,
        { crate::Chap53::GraphSearchMtPer::GraphSearchMtPer::graph_search(graph, source, strategy, Ghost(vertex_universe)) }

        fn graph_search_multi<G, S>(graph: &G, sources: AVLTreeSetMtPer<V>, strategy: &S, Ghost(vertex_universe): Ghost<Set<<V as View>::V>>) -> (search: SearchResult<V>)
        where G: Fn(&V) -> AVLTreeSetMtPer<V>, S: SelectionStrategy<V>,
        {
            crate::Chap53::GraphSearchMtPer::GraphSearchMtPer::graph_search_multi(graph, sources, strategy, Ghost(vertex_universe))
        }

        fn reachable<G>(graph: &G, source: V, Ghost(vertex_universe): Ghost<Set<<V as View>::V>>) -> (reachable_set: AVLTreeSetMtPer<V>)
        where G: Fn(&V) -> AVLTreeSetMtPer<V>,
        { crate::Chap53::GraphSearchMtPer::GraphSearchMtPer::reachable(graph, source, Ghost(vertex_universe)) }
    }

    /// Find all vertices reachable from source (Problem 53.2) using SelectAll (BFS).
    /// - Alg Analysis: APAS (Ch53 Thm 53.1): (no explicit cost; ≤ |V| rounds)
    /// - Alg Analysis: Code review (Claude Opus 4.6): no explicit cost in APAS — N/A
    /// - Claude-Opus-4.6: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — delegates to graph_search with SelectAll.
    pub fn reachable<V: StTInMtT + Ord + 'static, G>(graph: &G, source: V, Ghost(vertex_universe): Ghost<Set<<V as View>::V>>) -> (reachable_set: AVLTreeSetMtPer<V>)
    where
        G: Fn(&V) -> AVLTreeSetMtPer<V>,
        requires
            forall|v: &V| #[trigger] graph.requires((v,)),
            forall|v: &V, r: AVLTreeSetMtPer<V>| #[trigger] graph.ensures((v,), r) ==> r.spec_avltreesetmtper_wf(),
            vertex_universe.finite(),
            vertex_universe.len() + vertex_universe.len() < usize::MAX as nat,
            vertex_universe.contains(source@),
            forall|v: &V, r: AVLTreeSetMtPer<V>| #[trigger] graph.ensures((v,), r) ==> r@.subset_of(vertex_universe),
            vstd::laws_cmp::obeys_cmp_spec::<V>(),
            view_ord_consistent::<V>(),
        ensures reachable_set@.contains(source@),
    {
        let result = graph_search(graph, source, &SelectAll, Ghost(vertex_universe));
        result.visited
    }

    // 12. derive impls in verus!

    impl<V: StTInMtT + Ord + 'static> Clone for SearchResult<V> {
        fn clone(&self) -> (cloned: Self) {
            SearchResult {
                visited: self.visited.clone(),
                parent: self.parent.clone(),
            }
        }
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
