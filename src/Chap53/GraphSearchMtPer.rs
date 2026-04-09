//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Chapter 53: Generic Graph Search (persistent, multi-threaded).
//!
//! Note: This is a SEQUENTIAL implementation using thread-safe types.
//! True parallelism would require more complex lifetime management for the graph closure.
//! The parallel algorithm concept is demonstrated via use of AVLTreeSetMtPer which has
//! true parallel operations (filter, union, intersection).


//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4a. type definitions
//	Section 9a. impls
//	Section 4b. type definitions
//	Section 9b. impls
//	Section 4c. type definitions
//	Section 8c. traits
//	Section 9c. impls
//	Section 12a. derive impls in verus!
//	Section 14a. derive impls outside verus!
//	Section 14b. derive impls outside verus!
//	Section 14c. derive impls outside verus!

//		Section 1. module

pub mod GraphSearchMtPer {


    //		Section 2. imports

    use vstd::prelude::*;
    use crate::Chap37::AVLTreeSeqMtPer::AVLTreeSeqMtPer::AVLTreeSeqMtPerTrait;
    use crate::Chap41::AVLTreeSetMtPer::AVLTreeSetMtPer::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent;
    use crate::Types::Types::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;

    verus! 
{

    //		Section 3. broadcast use


    broadcast use crate::vstdplus::feq::feq::group_feq_axioms;

    //		Section 4a. type definitions


    #[verifier::reject_recursive_types(V)]
    pub struct SearchResult<V: StTInMtT + Ord + TotalOrder + 'static> {
        pub visited: AVLTreeSetMtPer<V>,
        pub parent: Option<AVLTreeSetMtPer<Pair<V, V>>>,
    }

    //		Section 9a. impls


    impl<V: StTInMtT + Ord + TotalOrder + 'static> GraphSearchMtPerTrait<V> for SearchResult<V> {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((|V|+|E|) log |V|), Span O((|V|+|E|) log |V|) — delegates to free fn.
        fn graph_search<G, S>(graph: &G, source: V, strategy: &S, Ghost(vertex_universe): Ghost<Set<<V as View>::V>>) -> (search: SearchResult<V>)
        where G: Fn(&V) -> AVLTreeSetMtPer<V>, S: SelectionStrategy<V>,
        { crate::Chap53::GraphSearchMtPer::GraphSearchMtPer::graph_search(graph, source, strategy, Ghost(vertex_universe)) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((|V|+|E|) log |V|), Span O((|V|+|E|) log |V|) — delegates to free fn.
        fn graph_search_multi<G, S>(graph: &G, sources: AVLTreeSetMtPer<V>, strategy: &S, Ghost(vertex_universe): Ghost<Set<<V as View>::V>>) -> (search: SearchResult<V>)
        where G: Fn(&V) -> AVLTreeSetMtPer<V>, S: SelectionStrategy<V>,
        {
            crate::Chap53::GraphSearchMtPer::GraphSearchMtPer::graph_search_multi(graph, sources, strategy, Ghost(vertex_universe))
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((|V|+|E|) log |V|), Span O((|V|+|E|) log |V|) — delegates to free fn.
        fn reachable<G>(graph: &G, source: V, Ghost(vertex_universe): Ghost<Set<<V as View>::V>>) -> (reachable_set: AVLTreeSetMtPer<V>)
        where G: Fn(&V) -> AVLTreeSetMtPer<V>,
        { crate::Chap53::GraphSearchMtPer::GraphSearchMtPer::reachable(graph, source, Ghost(vertex_universe)) }
    }

    //		Section 4b. type definitions


    pub struct SelectAll;

    //		Section 9b. impls


    impl<V: StTInMtT + Ord + TotalOrder + 'static> SelectionStrategy<V> for SelectAll {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|frontier|), Span O(|frontier|) — clones entire frontier.
        fn select(&self, frontier: &AVLTreeSetMtPer<V>) -> (selected: (AVLTreeSetMtPer<V>, bool)) { (frontier.clone(), false) }
    }

    //		Section 4c. type definitions


    pub struct SelectOne;

    //		Section 8c. traits


    pub trait SelectionStrategy<V: StTInMtT + Ord + TotalOrder + 'static> {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work varies by strategy, Span varies by strategy — abstract selection from frontier.
        fn select(&self, frontier: &AVLTreeSetMtPer<V>) -> (selected: (AVLTreeSetMtPer<V>, bool))
            requires
                obeys_feq_clone::<V>(),
                frontier.spec_avltreesetmtper_wf(),
                frontier@.len() < usize::MAX as nat,
            ensures selected.0@.subset_of(frontier@);
    }

    pub trait GraphSearchMtPerTrait<V: StTInMtT + Ord + TotalOrder + 'static> {
        /// - Alg Analysis: Code review (Claude Opus 4.6): no explicit cost in APAS — N/A
        /// - Alg Analysis: Code review (Claude Opus 4.6): ACCEPTED DIFFERENCE: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — neighbor loop is sequential despite parallel set ops.
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): no explicit cost in APAS — N/A
        /// - Alg Analysis: Code review (Claude Opus 4.6): ACCEPTED DIFFERENCE: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — neighbor loop is sequential despite parallel set ops.
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): no explicit cost in APAS — N/A
        /// - Alg Analysis: Code review (Claude Opus 4.6): ACCEPTED DIFFERENCE: Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — sequential control flow; uses SelectAll (BFS).
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

    //		Section 9c. impls


    impl<V: StTInMtT + Ord + TotalOrder + 'static> SelectionStrategy<V> for SelectOne {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log |frontier|), Span O(log |frontier|) — picks first element.
        fn select(&self, frontier: &AVLTreeSetMtPer<V>) -> (selected: (AVLTreeSetMtPer<V>, bool)) {
            if frontier.size() == 0 {
                (AVLTreeSetMtPer::empty(), false)
            } else {
                let seq = frontier.to_seq();
                // Veracity: NEEDED assert
                assert(seq@.len() > 0) by {
                    if seq@.len() == 0 {
                        // Veracity: NEEDED assert
                        assert(seq@.to_set() =~= Set::empty());
                    }
                }
                let first_ref = seq.nth(0);
                let first = first_ref.clone();
                // Veracity: NEEDED proof block
                proof { assert(cloned(*first_ref, first)); }
                let result = AVLTreeSetMtPer::singleton(first);
                (result, false)
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): ACCEPTED DIFFERENCE: Work O((|V|+|E|) log |V|), Span O((|V|+|E|) log |V|) — delegates to graph_search_multi; sequential despite Mt module (no join).
    pub fn graph_search<V: StTInMtT + Ord + TotalOrder + 'static, G, S>(graph: &G, source: V, strategy: &S, Ghost(vertex_universe): Ghost<Set<<V as View>::V>>) -> (search: SearchResult<V>)
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
        // Veracity: NEEDED proof block
        proof {
        }
        graph_search_multi(graph, sources, strategy, Ghost(vertex_universe))
    }

    /// Graph exploration loop (Algorithm 53.4).
    /// - Alg Analysis: Code review (Claude Opus 4.6): ACCEPTED DIFFERENCE: Work O((|V|+|E|) log |V|), Span O((|V|+|E|) log |V|) — ≤|V| rounds with AVL set ops; sequential despite Mt module (no join).
    #[verifier::exec_allows_no_decreases_clause]
    fn graph_search_explore<V: StTInMtT + Ord + TotalOrder + 'static, G: Fn(&V) -> AVLTreeSetMtPer<V>, S: SelectionStrategy<V>>(
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
            // Veracity: NEEDED proof block
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
                // Veracity: NEEDED proof block
                proof {
                    // Graph closure ensures wf: invariant says graph.ensures((v,), r) ==> r.wf().
                    // After graph(v), Verus knows graph.ensures((v,), neighbors), triggering the forall.
                    // Veracity: NEEDED assert
                    assert(neighbors.spec_avltreesetmtper_wf());
                    vstd::set_lib::lemma_len_subset(new_neighbors@, vertex_universe);
                    vstd::set_lib::lemma_len_subset(neighbors@, vertex_universe);
                }
                new_neighbors = new_neighbors.union(&neighbors);
                i = i + 1;
            }

            let frontier_new = new_neighbors.difference(&visited_new);

            // Veracity: NEEDED proof block
            proof {
            }

            visited = visited_new;
            frontier = frontier_new;
        }

        visited
    }

    /// Generic graph search starting from multiple sources (Exercise 53.3).
    /// - Alg Analysis: Code review (Claude Opus 4.6): ACCEPTED DIFFERENCE: Work O((|V|+|E|) log |V|), Span O((|V|+|E|) log |V|) — delegates to graph_search_explore; sequential despite Mt module.
    pub fn graph_search_multi<V: StTInMtT + Ord + TotalOrder + 'static, G, S>(
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

    /// Find all vertices reachable from source (Problem 53.2) using SelectAll (BFS).
    /// - Alg Analysis: Code review (Claude Opus 4.6): no explicit cost in APAS — N/A
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — delegates to graph_search with SelectAll.
    pub fn reachable<V: StTInMtT + Ord + TotalOrder + 'static, G>(graph: &G, source: V, Ghost(vertex_universe): Ghost<Set<<V as View>::V>>) -> (reachable_set: AVLTreeSetMtPer<V>)
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

    //		Section 12a. derive impls in verus!


    impl<V: StTInMtT + Ord + TotalOrder + 'static> Clone for SearchResult<V> {
        fn clone(&self) -> (cloned: Self) {
            SearchResult {
                visited: self.visited.clone(),
                parent: self.parent.clone(),
            }
        }
    }

    } // verus!

    //		Section 14a. derive impls outside verus!


    impl<V: StTInMtT + Ord + TotalOrder + 'static> std::fmt::Debug for SearchResult<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("SearchResult")
                .field("visited", &self.visited)
                .field("parent", &self.parent)
                .finish()
        }
    }

    impl<V: StTInMtT + Ord + TotalOrder + 'static> std::fmt::Display for SearchResult<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SearchResult(visited={})", self.visited.size())
        }
    }

    //		Section 14b. derive impls outside verus!

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

    //		Section 14c. derive impls outside verus!

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
