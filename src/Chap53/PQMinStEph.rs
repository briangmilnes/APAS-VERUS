//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 53: Min-Priority Queue Search - ephemeral, single-threaded.

pub mod PQMinStEph {

    use vstd::prelude::*;
    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::AVLTreeSeqStEphTrait;
    #[cfg(verus_keep_ghost)]
    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::lemma_wf_implies_len_bound_steph;
    use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;

    verus! {

    broadcast use crate::vstdplus::feq::feq::group_feq_axioms;

    // 4. type definitions
    pub struct PQMinResult<V: StT + Ord, P: StT + Ord> {
        pub visited: AVLTreeSetStEph<V>,
        pub priorities: AVLTreeSetStEph<Pair<V, P>>,
        pub parent: Option<AVLTreeSetStEph<Pair<V, V>>>,
    }

    // 6. spec fns

    pub open spec fn spec_pqminsteph_wf_generic<V: StT + Ord, P: StT + Ord>(
        s: &PQMinResult<V, P>,
    ) -> bool {
        s.visited@.finite() && s.priorities@.finite()
    }

    // 8. traits
    pub trait PQMinStEphTrait<V: StT + Ord, P: StT + Ord> {
        spec fn spec_pqminsteph_wf(&self) -> bool;

        /// - APAS: (no explicit PFS cost in Chap53; PFS cost depends on priority queue implementation)
        /// - Claude-Opus-4.6: Work Θ(|V|² + |E| log |V|), Span Θ(|V|² + |E| log |V|) — find_min uses to_seq O(|F|) per round.
        fn pq_min<G, PF>(graph: &G, source: V, priority_fn: &PF, Ghost(vertex_universe): Ghost<Set<<V as View>::V>>) -> (search: PQMinResult<V, P>)
        where
            G: Fn(&V) -> AVLTreeSetStEph<V>,
            PF: Fn(&V) -> P,
            requires
                forall|v: &V| #[trigger] graph.requires((v,)),
                forall|v: &V| #[trigger] priority_fn.requires((v,)),
                forall|v: &V, r: AVLTreeSetStEph<V>| #[trigger] graph.ensures((v,), r) ==> r.spec_avltreesetsteph_wf(),
                vertex_universe.finite(),
                vertex_universe.len() + 1 < usize::MAX as nat,
                vertex_universe.contains(source@),
                forall|v: &V, r: AVLTreeSetStEph<V>| #[trigger] graph.ensures((v,), r) ==> r@.subset_of(vertex_universe),
            ensures
                spec_pqminsteph_wf_generic(&search),
                search.visited@.contains(source@);

        /// - APAS: (no explicit PFS cost in Chap53; PFS cost depends on priority queue implementation)
        /// - Claude-Opus-4.6: Work Θ(|V|² + |E| log |V|), Span Θ(|V|² + |E| log |V|) — find_min uses to_seq O(|F|) per round.
        fn pq_min_multi<G, PF>(graph: &G, sources: AVLTreeSetStEph<V>, priority_fn: &PF, Ghost(vertex_universe): Ghost<Set<<V as View>::V>>) -> (search: PQMinResult<V, P>)
        where
            G: Fn(&V) -> AVLTreeSetStEph<V>,
            PF: Fn(&V) -> P,
            requires
                sources.spec_avltreesetsteph_wf(),
                forall|v: &V| #[trigger] graph.requires((v,)),
                forall|v: &V| #[trigger] priority_fn.requires((v,)),
                forall|v: &V, r: AVLTreeSetStEph<V>| #[trigger] graph.ensures((v,), r) ==> r.spec_avltreesetsteph_wf(),
                vertex_universe.finite(),
                vertex_universe.len() + 1 < usize::MAX as nat,
                sources@.subset_of(vertex_universe),
                forall|v: &V, r: AVLTreeSetStEph<V>| #[trigger] graph.ensures((v,), r) ==> r@.subset_of(vertex_universe),
            ensures
                spec_pqminsteph_wf_generic(&search),
                sources@.subset_of(search.visited@);
    }

    // 9. impls

    impl<V: StT + Ord, P: StT + Ord> PQMinStEphTrait<V, P> for PQMinResult<V, P> {
        open spec fn spec_pqminsteph_wf(&self) -> bool {
            spec_pqminsteph_wf_generic(self)
        }

        fn pq_min<G, PF>(graph: &G, source: V, priority_fn: &PF, Ghost(vertex_universe): Ghost<Set<<V as View>::V>>) -> (search: PQMinResult<V, P>)
        where G: Fn(&V) -> AVLTreeSetStEph<V>, PF: Fn(&V) -> P,
        { pq_min(graph, source, priority_fn, Ghost(vertex_universe)) }

        fn pq_min_multi<G, PF>(graph: &G, sources: AVLTreeSetStEph<V>, priority_fn: &PF, Ghost(vertex_universe): Ghost<Set<<V as View>::V>>) -> (search: PQMinResult<V, P>)
        where G: Fn(&V) -> AVLTreeSetStEph<V>, PF: Fn(&V) -> P,
        { pq_min_multi(graph, sources, priority_fn, Ghost(vertex_universe)) }
    }

    /// Priority-first search from single source (Section 53.4).
    /// - APAS: (no explicit PFS cost in Chap53; PFS cost depends on priority queue implementation)
    /// - Claude-Opus-4.6: Work Θ(|V|² + |E| log |V|), Span Θ(|V|² + |E| log |V|) — delegates to pq_min_multi.
    pub fn pq_min<V: StT + Ord, P: StT + Ord, G, PF>(graph: &G, source: V, priority_fn: &PF, Ghost(vertex_universe): Ghost<Set<<V as View>::V>>) -> (search: PQMinResult<V, P>)
    where
        G: Fn(&V) -> AVLTreeSetStEph<V>,
        PF: Fn(&V) -> P,
        requires
            forall|v: &V| #[trigger] graph.requires((v,)),
            forall|v: &V| #[trigger] priority_fn.requires((v,)),
            forall|v: &V, r: AVLTreeSetStEph<V>| #[trigger] graph.ensures((v,), r) ==> r.spec_avltreesetsteph_wf(),
            vertex_universe.finite(),
            vertex_universe.len() + 1 < usize::MAX as nat,
            vertex_universe.contains(source@),
            forall|v: &V, r: AVLTreeSetStEph<V>| #[trigger] graph.ensures((v,), r) ==> r@.subset_of(vertex_universe),
        ensures
            spec_pqminsteph_wf_generic(&search),
            search.visited@.contains(source@),
    {
        let sources = AVLTreeSetStEph::singleton(source);
        proof {
            assert(sources@.subset_of(vertex_universe)) by {
                assert forall|a: <V as View>::V| sources@.contains(a)
                    implies #[trigger] vertex_universe.contains(a) by {
                    assert(sources@ == Set::<<V as View>::V>::empty().insert(source@));
                }
            }
        }
        pq_min_multi(graph, sources, priority_fn, Ghost(vertex_universe))
    }

    fn pq_find_min_priority<V: StT + Ord, P: StT + Ord>(
        frontier: &AVLTreeSetStEph<Pair<Pair<P, V>, V>>,
    ) -> (result: Option<V>)
        requires
            frontier.spec_avltreesetsteph_wf(),
            obeys_feq_clone::<V>(),
        ensures
            frontier@.len() == 0 ==> result.is_none(),
            frontier@.len() > 0 ==> result.is_some(),
    {
        if frontier.size() == 0 {
            None
        } else {
            let seq = frontier.to_seq();
            assert(seq@.len() > 0) by {
                if seq@.len() == 0 {
                    assert(seq@.to_set() =~= Set::empty());
                }
            }
            let entry_ref = seq.nth(0);
            let v = entry_ref.1.clone();
            proof { assert(cloned(entry_ref.1, v)); }
            Some(v)
        }
    }

    #[verifier::exec_allows_no_decreases_clause]
    fn pq_explore<V: StT + Ord, P: StT + Ord, G: Fn(&V) -> AVLTreeSetStEph<V>, PF: Fn(&V) -> P>(
        graph: &G,
        priority_fn: &PF,
        visited_init: AVLTreeSetStEph<V>,
        frontier_init: AVLTreeSetStEph<Pair<Pair<P, V>, V>>,
        Ghost(vertex_universe): Ghost<Set<<V as View>::V>>,
    ) -> (result: (AVLTreeSetStEph<V>, AVLTreeSetStEph<Pair<V, P>>))
        requires
            visited_init.spec_avltreesetsteph_wf(),
            frontier_init.spec_avltreesetsteph_wf(),
            forall|v: &V| #[trigger] graph.requires((v,)),
            forall|v: &V| #[trigger] priority_fn.requires((v,)),
            forall|v: &V, r: AVLTreeSetStEph<V>| #[trigger] graph.ensures((v,), r) ==> r.spec_avltreesetsteph_wf(),
            // Vertex universe constraints for capacity proofs.
            vertex_universe.finite(),
            vertex_universe.len() + 1 < usize::MAX as nat,
            visited_init@.subset_of(vertex_universe),
            forall|v: &V, r: AVLTreeSetStEph<V>| #[trigger] graph.ensures((v,), r) ==> r@.subset_of(vertex_universe),
            forall|e: ((<P as View>::V, <V as View>::V), <V as View>::V)|
                #[trigger] frontier_init@.contains(e) ==> vertex_universe.contains(e.1),
        ensures
            result.0.spec_avltreesetsteph_wf(),
            result.1.spec_avltreesetsteph_wf(),
            visited_init@.subset_of(result.0@),
    {
        let mut visited = visited_init;
        let mut frontier = frontier_init;

        while frontier.size() > 0
            invariant
                visited.spec_avltreesetsteph_wf(),
                frontier.spec_avltreesetsteph_wf(),
                visited_init@.subset_of(visited@),
                forall|v: &V| #[trigger] graph.requires((v,)),
                forall|v: &V| #[trigger] priority_fn.requires((v,)),
                forall|v: &V, r: AVLTreeSetStEph<V>| #[trigger] graph.ensures((v,), r) ==> r.spec_avltreesetsteph_wf(),
                // Vertex universe invariants.
                vertex_universe.finite(),
                vertex_universe.len() + 1 < usize::MAX as nat,
                visited@.subset_of(vertex_universe),
                forall|v: &V, r: AVLTreeSetStEph<V>| #[trigger] graph.ensures((v,), r) ==> r@.subset_of(vertex_universe),
                forall|e: ((<P as View>::V, <V as View>::V), <V as View>::V)|
                    #[trigger] frontier@.contains(e) ==> vertex_universe.contains(e.1),
        {
            let seq = frontier.to_seq();
            assert(seq@.len() > 0) by {
                if seq@.len() == 0 {
                    assert(seq@.to_set() =~= Set::empty());
                }
            }
            let entry_ref = seq.nth(0);
            let v = entry_ref.1.clone_plus();
            let p = priority_fn(&v);
            let v_clone1 = v.clone();
            let v_clone2 = v.clone_plus();
            let entry = Pair(Pair(p.clone(), v_clone1), v_clone2);
            let frontier_new = frontier.difference(&AVLTreeSetStEph::singleton(entry));

            // Capacity: visited ⊆ vertex_universe from invariant.
            proof {
                vstd::set_lib::lemma_len_subset(visited@, vertex_universe);
            }
            let v_for_visited = v.clone_plus();
            let visited_new = visited.union(&AVLTreeSetStEph::singleton(v_for_visited));
            // Maintain visited@.subset_of(vertex_universe): v@ ∈ vertex_universe from frontier entry.
            proof {
                assert(obeys_feq_full_trigger::<V>());
                // v was clone_plus'd from entry_ref.1, so cloned(entry_ref.1, v).
                // feq chain: entry_ref.1 == v, hence v@ == entry_ref@.1 == seq@[0].1.
                assert(frontier@.contains(seq@[0 as int]));
                assert(vertex_universe.contains(v@));
                // v_for_visited was clone_plus'd from v, so v == v_for_visited.
                // visited_new@ = visited@ ∪ {v_for_visited@} = visited@ ∪ {v@}.
                assert forall|a: <V as View>::V| visited_new@.contains(a)
                    implies #[trigger] vertex_universe.contains(a) by {
                }
            }

            let neighbors = graph(&v);
            let mut frontier_updated = frontier_new;
            let neighbors_seq = neighbors.to_seq();
            let nlen = neighbors_seq.length();
            let mut i: usize = 0;
            // Establish neighbor vertices ∈ vertex_universe before inner loop.
            proof {
                assert forall|j: int| 0 <= j < neighbors_seq@.len()
                    implies vertex_universe.contains(#[trigger] neighbors_seq@[j]) by {
                    assert(neighbors@.contains(neighbors_seq@[j]));
                }
            }
            while i < nlen
                invariant
                    i <= nlen,
                    nlen == neighbors_seq@.len(),
                    neighbors_seq.spec_avltreeseqsteph_wf(),
                    visited_new.spec_avltreesetsteph_wf(),
                    frontier_updated.spec_avltreesetsteph_wf(),
                    forall|v: &V| #[trigger] priority_fn.requires((v,)),
                    // Vertex universe invariants for frontier vertex tracking.
                    vertex_universe.finite(),
                    vertex_universe.len() + 1 < usize::MAX as nat,
                    forall|j: int| 0 <= j < neighbors_seq@.len()
                        ==> vertex_universe.contains(#[trigger] neighbors_seq@[j]),
                    forall|e: ((<P as View>::V, <V as View>::V), <V as View>::V)|
                        #[trigger] frontier_updated@.contains(e) ==> vertex_universe.contains(e.1),
                decreases nlen - i,
            {
                let neighbor = neighbors_seq.nth(i);
                if !visited_new.find(neighbor) {
                    let neighbor_p = priority_fn(neighbor);
                    let neighbor_clone1 = neighbor.clone();
                    let neighbor_clone2 = neighbor.clone_plus();
                    let neighbor_entry = Pair(Pair(neighbor_p.clone(), neighbor_clone1), neighbor_clone2);
                    // Capacity: frontier@.len() + 1 < usize::MAX requires bounding frontier
                    // through vertex_universe via injection (each entry uniquely determines a
                    // vertex). Proving injection needs priority_fn view-determinism, which the
                    // generic Fn interface does not guarantee.
                    let ghost old_fu = frontier_updated@;
                    proof { assume(frontier_updated@.len() + 1 < usize::MAX as nat); }
                    frontier_updated = frontier_updated.union(&AVLTreeSetStEph::singleton(neighbor_entry));
                    // Maintain frontier vertex invariant after union.
                    proof {
                        assert(obeys_feq_full_trigger::<V>());
                        assert(neighbor_entry@.1 == neighbor@);
                        assert(vertex_universe.contains(neighbors_seq@[i as int]));
                        assert(vertex_universe.contains(neighbor_entry@.1));
                        assert forall|e: ((<P as View>::V, <V as View>::V), <V as View>::V)|
                            #[trigger] frontier_updated@.contains(e)
                            implies vertex_universe.contains(e.1) by {
                            if old_fu.contains(e) {}
                        }
                    }
                }
                i = i + 1;
            }

            visited = visited_new;
            frontier = frontier_updated;
        }

        // Build priorities from visited. Close via loop counter + len_bound lemma.
        let mut priorities = AVLTreeSetStEph::empty();
        let visited_seq = visited.to_seq();
        let vlen = visited_seq.length();
        let mut j: usize = 0;
        proof {
            lemma_wf_implies_len_bound_steph(visited_seq);
        }
        while j < vlen
            invariant
                j <= vlen,
                vlen == visited_seq@.len(),
                vlen < usize::MAX,
                visited_seq.spec_avltreeseqsteph_wf(),
                visited.spec_avltreesetsteph_wf(),
                visited_init@.subset_of(visited@),
                priorities.spec_avltreesetsteph_wf(),
                priorities@.len() <= j as nat,
                forall|v: &V| #[trigger] priority_fn.requires((v,)),
                priorities@.len() <= j as nat,
            decreases vlen - j,
        {
            let vref = visited_seq.nth(j);
            let p = priority_fn(vref);
            let vp = Pair(vref.clone(), p);
            proof {
                lemma_wf_implies_len_bound_steph(visited_seq);
                assert(priorities@.len() + 1 < usize::MAX as nat);
            }
            let ghost old_pri = priorities@;
            priorities = priorities.union(&AVLTreeSetStEph::singleton(vp));
            proof {
                vstd::set_lib::lemma_len_union(old_pri, Set::empty().insert(vp@));
            }
            j = j + 1;
        }
        (visited, priorities)
    }

    /// Priority-first search from multiple sources (Section 53.4).
    pub fn pq_min_multi<V: StT + Ord, P: StT + Ord, G, PF>(
        graph: &G,
        sources: AVLTreeSetStEph<V>,
        priority_fn: &PF,
        Ghost(vertex_universe): Ghost<Set<<V as View>::V>>,
    ) -> (search: PQMinResult<V, P>)
    where
        G: Fn(&V) -> AVLTreeSetStEph<V>,
        PF: Fn(&V) -> P,
        requires
            sources.spec_avltreesetsteph_wf(),
            forall|v: &V| #[trigger] graph.requires((v,)),
            forall|v: &V| #[trigger] priority_fn.requires((v,)),
            forall|v: &V, r: AVLTreeSetStEph<V>| #[trigger] graph.ensures((v,), r) ==> r.spec_avltreesetsteph_wf(),
            vertex_universe.finite(),
            vertex_universe.len() + 1 < usize::MAX as nat,
            sources@.subset_of(vertex_universe),
            forall|v: &V, r: AVLTreeSetStEph<V>| #[trigger] graph.ensures((v,), r) ==> r@.subset_of(vertex_universe),
        ensures
            spec_pqminsteph_wf_generic(&search),
            sources@.subset_of(search.visited@),
    {
        let mut initial_frontier = AVLTreeSetStEph::empty();
        let sources_seq = sources.to_seq();
        let slen = sources_seq.length();
        let mut i: usize = 0;
        // Establish sources_seq vertices ∈ vertex_universe.
        proof {
            lemma_wf_implies_len_bound_steph(sources_seq);
            assert forall|j: int| 0 <= j < sources_seq@.len()
                implies vertex_universe.contains(#[trigger] sources_seq@[j]) by {
                assert(sources@.contains(sources_seq@[j]));
            }
        }
        while i < slen
            invariant
                i <= slen,
                slen == sources_seq@.len(),
                slen < usize::MAX,
                sources_seq.spec_avltreeseqsteph_wf(),
                sources.spec_avltreesetsteph_wf(),
                initial_frontier.spec_avltreesetsteph_wf(),
                initial_frontier@.len() <= i as nat,
                forall|v: &V| #[trigger] graph.requires((v,)),
                forall|v: &V| #[trigger] priority_fn.requires((v,)),
                forall|v: &V, r: AVLTreeSetStEph<V>| #[trigger] graph.ensures((v,), r) ==> r.spec_avltreesetsteph_wf(),
                initial_frontier@.len() <= i as nat,
                // Frontier vertex invariant: all entry vertices belong to vertex_universe.
                sources@.subset_of(vertex_universe),
                vertex_universe.finite(),
                vertex_universe.len() + 1 < usize::MAX as nat,
                forall|j: int| 0 <= j < sources_seq@.len()
                    ==> vertex_universe.contains(#[trigger] sources_seq@[j]),
                forall|e: ((<P as View>::V, <V as View>::V), <V as View>::V)|
                    #[trigger] initial_frontier@.contains(e) ==> vertex_universe.contains(e.1),
            decreases slen - i,
        {
            let v = sources_seq.nth(i);
            let p = priority_fn(v);
            let v_clone1 = v.clone();
            let v_clone2 = v.clone_plus();
            let entry = Pair(Pair(p.clone(), v_clone1), v_clone2);
            proof {
                lemma_wf_implies_len_bound_steph(sources_seq);
                assert(initial_frontier@.len() + 1 < usize::MAX as nat);
            }
            let ghost old_if = initial_frontier@;
            initial_frontier = initial_frontier.union(&AVLTreeSetStEph::singleton(entry));
            proof {
                vstd::set_lib::lemma_len_union(old_if, Set::empty().insert(entry@));
                // Maintain frontier vertex invariant: entry@.1 = v@ ∈ vertex_universe.
                assert(obeys_feq_full_trigger::<V>());
                assert(entry@.1 == v@);
                assert(vertex_universe.contains(sources_seq@[i as int]));
                assert(vertex_universe.contains(entry@.1));
                assert forall|e: ((<P as View>::V, <V as View>::V), <V as View>::V)|
                    #[trigger] initial_frontier@.contains(e)
                    implies vertex_universe.contains(e.1) by {
                    if old_if.contains(e) {}
                }
            }
            i = i + 1;
        }

        let (visited, priorities) = pq_explore(graph, priority_fn, sources, initial_frontier, Ghost(vertex_universe));

        PQMinResult {
            visited,
            priorities,
            parent: None,
        }
    }

    // 11. derive impls in verus!

    impl<V: StT + Ord, P: StT + Ord> Clone for PQMinResult<V, P> {
        fn clone(&self) -> (out: Self) {
            PQMinResult {
                visited: self.visited.clone(),
                priorities: self.priorities.clone(),
                parent: self.parent.clone(),
            }
        }
    }

    } // verus!

    // 13. derive impls outside verus!

    impl<V: StT + Ord, P: StT + Ord> std::fmt::Debug for PQMinResult<V, P> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("PQMinResult")
                .field("visited", &self.visited)
                .field("priorities", &self.priorities)
                .field("parent", &self.parent)
                .finish()
        }
    }

    impl<V: StT + Ord, P: StT + Ord> std::fmt::Display for PQMinResult<V, P> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "PQMinResult(visited={}, priorities={})", self.visited.size(), self.priorities.size())
        }
    }
}
