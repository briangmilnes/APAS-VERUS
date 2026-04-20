// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO
//! Chapter 53: Generic Graph Search (persistent, single-threaded).
//!
//! Implements Algorithm 53.4 - Generic Graph Search with pluggable frontier selection.


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

pub mod GraphSearchStPer {


    //		Section 2. imports

    use vstd::prelude::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::AVLTreeSeqStPerTrait;
    #[cfg(verus_keep_ghost)]
    use crate::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent;
    use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
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
    pub struct SearchResult<V: StT + Ord + TotalOrder> {
        pub visited: AVLTreeSetStPer<V>,
        pub parent: Option<AVLTreeSetStPer<Pair<V, V>>>, // (child, parent) edges
    }

    //		Section 9a. impls


    impl<V: StT + Ord + TotalOrder> GraphSearchStPerTrait<V> for SearchResult<V> {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((|V|+|E|) log |V|), Span O((|V|+|E|) log |V|) — delegates to free fn; St sequential.
        fn graph_search<G, S>(graph: &G, source: V, strategy: &S, Ghost(vertex_universe): Ghost<Set<<V as View>::V>>) -> (search: SearchResult<V>)
        where G: Fn(&V) -> AVLTreeSetStPer<V>, S: SelectionStrategy<V>,
        { graph_search(graph, source, strategy, Ghost(vertex_universe)) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((|V|+|E|) log |V|), Span O((|V|+|E|) log |V|) — delegates to free fn; St sequential.
        fn graph_search_multi<G, S>(graph: &G, sources: AVLTreeSetStPer<V>, strategy: &S, Ghost(vertex_universe): Ghost<Set<<V as View>::V>>) -> (search: SearchResult<V>)
        where G: Fn(&V) -> AVLTreeSetStPer<V>, S: SelectionStrategy<V>,
        { graph_search_multi(graph, sources, strategy, Ghost(vertex_universe)) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((|V|+|E|) log |V|), Span O((|V|+|E|) log |V|) — delegates to free fn; St sequential.
        fn reachable<G>(graph: &G, source: V, Ghost(vertex_universe): Ghost<Set<<V as View>::V>>) -> (reachable_set: AVLTreeSetStPer<V>)
        where G: Fn(&V) -> AVLTreeSetStPer<V>,
        { reachable(graph, source, Ghost(vertex_universe)) }
    }

    //		Section 4b. type definitions


    /// Select all vertices in frontier (breadth-first style).
    pub struct SelectAll;

    //		Section 9b. impls


    impl<V: StT + Ord + TotalOrder> SelectionStrategy<V> for SelectAll {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|frontier|), Span O(|frontier|) — clones entire frontier.
        fn select(&self, frontier: &AVLTreeSetStPer<V>) -> (selected: (AVLTreeSetStPer<V>, bool)) { (frontier.clone(), false) }
    }

    //		Section 4c. type definitions


    /// Select single arbitrary vertex (depth-first style).
    pub struct SelectOne;

    //		Section 8c. traits


    /// Strategy for selecting which frontier vertices to visit next.
    pub trait SelectionStrategy<V: StT + Ord + TotalOrder> {
        /// Select subset U ⊆ F where |U| ≥ 1.
        /// Returns (selected vertices, should_track_parents).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work varies by strategy, Span varies by strategy — abstract selection from frontier.
        fn select(&self, frontier: &AVLTreeSetStPer<V>) -> (selected: (AVLTreeSetStPer<V>, bool))
            requires
                frontier.spec_avltreesetstper_wf(),
                obeys_feq_clone::<V>(),
            ensures selected.0@.subset_of(frontier@);
    }

    pub trait GraphSearchStPerTrait<V: StT + Ord + TotalOrder> {
        /// - Alg Analysis: Code review (Claude Opus 4.6): no explicit cost in APAS — N/A
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — sequential; AVL set ops add log factor.
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
                vstd::laws_cmp::obeys_cmp::<V>(),
                view_ord_consistent::<V>(),
            ensures search.visited@.contains(source@);

        /// - Alg Analysis: Code review (Claude Opus 4.6): no explicit cost in APAS — N/A
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — sequential; AVL set ops add log factor.
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
                vstd::laws_cmp::obeys_cmp::<V>(),
                view_ord_consistent::<V>(),
            ensures sources@.subset_of(search.visited@);

        /// - Alg Analysis: Code review (Claude Opus 4.6): no explicit cost in APAS — N/A
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — sequential; uses SelectAll (BFS).
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
                vstd::laws_cmp::obeys_cmp::<V>(),
                view_ord_consistent::<V>(),
            ensures reachable_set@.contains(source@);
    }

    //		Section 9c. impls


    impl<V: StT + Ord + TotalOrder> SelectionStrategy<V> for SelectOne {
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log |frontier|), Span O(log |frontier|) — picks first element via to_seq + nth.
        fn select(&self, frontier: &AVLTreeSetStPer<V>) -> (selected: (AVLTreeSetStPer<V>, bool)) {
            let n = frontier.size();
            if n == 0 {
                (AVLTreeSetStPer::empty(), false)
            } else {
                let seq = frontier.to_seq();
                // Veracity: NEEDED proof block
                proof {
                    // Prove seq is non-empty from frontier being non-empty.
                    if seq@.len() == 0 {
                        // Veracity: NEEDED assert
                        assert(seq@.to_set() =~= Set::<<V as View>::V>::empty());
                    }
                }
                let first_ref = seq.nth(0);
                let first = first_ref.clone();
// Veracity: UNNEEDED proof block                 // Veracity: NEEDED assert
                proof { assert(cloned(*first_ref, first)); }
                let result = AVLTreeSetStPer::singleton(first);
                (result, false)
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((|V|+|E|) log |V|), Span O((|V|+|E|) log |V|) — delegates to graph_search_multi; St sequential.
    pub fn graph_search<V: StT + Ord + TotalOrder, G, S>(
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
            vstd::laws_cmp::obeys_cmp::<V>(),
            view_ord_consistent::<V>(),
        ensures search.visited@.contains(source@),
    {
        // Veracity: NEEDED proof block
        let sources = AVLTreeSetStPer::singleton(source);
        proof {
        }
        graph_search_multi(graph, sources, strategy, Ghost(vertex_universe))
    }

    /// Graph exploration loop (Algorithm 53.4).
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((|V|+|E|) log |V|), Span O((|V|+|E|) log |V|) — ≤|V| rounds with AVL set ops; St sequential.
    #[verifier::exec_allows_no_decreases_clause]
    fn graph_search_explore<V: StT + Ord + TotalOrder, G: Fn(&V) -> AVLTreeSetStPer<V>, S: SelectionStrategy<V>>(
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
            vstd::laws_cmp::obeys_cmp::<V>(),
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
                vstd::laws_cmp::obeys_cmp::<V>(),
                // Veracity: NEEDED proof block
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
                    vstd::laws_cmp::obeys_cmp::<V>(),
                    view_ord_consistent::<V>(),
                decreases nlen - i,
            // Veracity: NEEDED proof block
            {
                let v = frontier_seq.nth(i);
                let neighbors = graph(v);
                proof {
                    vstd::set_lib::lemma_len_subset(new_neighbors@, vertex_universe);
                    vstd::set_lib::lemma_len_subset(neighbors@, vertex_universe);
                }
                new_neighbors = new_neighbors.union(&neighbors);
                i = i + 1;
            // Veracity: NEEDED proof block
            }

            let frontier_new = new_neighbors.difference(&visited_new);

            proof {
            }

            visited = visited_new;
            frontier = frontier_new;
        }

        visited
    }

    /// Generic graph search starting from multiple sources (Exercise 53.3).
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((|V|+|E|) log |V|), Span O((|V|+|E|) log |V|) — delegates to graph_search_explore; St sequential.
    pub fn graph_search_multi<V: StT + Ord + TotalOrder, G, S>(
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
            vstd::laws_cmp::obeys_cmp::<V>(),
            view_ord_consistent::<V>(),
        ensures sources@.subset_of(search.visited@),
    {
        let visited = graph_search_explore(graph, strategy, AVLTreeSetStPer::empty(), sources, Ghost(vertex_universe));
        SearchResult { visited, parent: None }
    }

    /// Find all vertices reachable from source (Problem 53.2) using SelectAll (BFS).
    /// - Alg Analysis: Code review (Claude Opus 4.6): no explicit cost in APAS — N/A
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ((|V| + |E|) log |V|), Span Θ((|V| + |E|) log |V|) — delegates to graph_search with SelectAll.
    pub fn reachable<V: StT + Ord + TotalOrder, G>(
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
            vstd::laws_cmp::obeys_cmp::<V>(),
            view_ord_consistent::<V>(),
        ensures reachable_set@.contains(source@),
    {
        let result = graph_search(graph, source, &SelectAll, Ghost(vertex_universe));
        result.visited
    }

    //		Section 12a. derive impls in verus!


    impl<V: StT + Ord + TotalOrder> Clone for SearchResult<V> {
        fn clone(&self) -> (out: Self) {
            SearchResult {
                visited: self.visited.clone(),
                parent: self.parent.clone(),
            }
        }
    }

    } // verus!

    //		Section 14a. derive impls outside verus!


    impl<V: StT + Ord + TotalOrder> std::fmt::Debug for SearchResult<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("SearchResult")
                .field("visited", &self.visited)
                .field("parent", &self.parent)
                .finish()
        }
    }

    impl<V: StT + Ord + TotalOrder> std::fmt::Display for SearchResult<V> {
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
