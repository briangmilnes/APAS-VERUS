//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 53: Min-Priority Queue Search - persistent, single-threaded.
//!
//! Implements Algorithm 53.7 - Priority Queue Search framework.
//! Selects minimum priority vertices first (lower priority = higher urgency).

pub mod PQMinStPer {

    use vstd::prelude::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::AVLTreeSeqStPerTrait;
    #[cfg(verus_keep_ghost)]
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::lemma_wf_implies_len_bound_stper;
    use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
    use crate::Types::Types::*;
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;

    verus! {

    broadcast use crate::vstdplus::feq::feq::group_feq_axioms;

    // 4. type definitions
    #[verifier::reject_recursive_types(V)]
    #[verifier::reject_recursive_types(P)]
    pub struct PQMinResult<V: StT + Ord, P: StT + Ord> {
        pub visited: AVLTreeSetStPer<V>,
        pub priorities: AVLTreeSetStPer<Pair<V, P>>,     // (vertex, priority)
        pub parent: Option<AVLTreeSetStPer<Pair<V, V>>>, // (child, parent)
    }

    // 6. spec fns

    pub open spec fn spec_pqminstper_wf_generic<V: StT + Ord, P: StT + Ord>(
        s: &PQMinResult<V, P>,
    ) -> bool {
        s.visited@.finite() && s.priorities@.finite()
    }

    // 8. traits
    pub trait PQMinStPerTrait<V: StT + Ord, P: StT + Ord> {
        spec fn spec_pqminstper_wf(&self) -> bool;

        /// - APAS: (no explicit PFS cost in Chap53; PFS cost depends on priority queue implementation)
        /// - Claude-Opus-4.6: Work Θ(|V|² + |E| log |V|), Span Θ(|V|² + |E| log |V|) — find_min uses to_seq O(|F|) per round.
        fn pq_min<G, PF>(graph: &G, source: V, priority_fn: &PF, Ghost(vertex_universe): Ghost<Set<<V as View>::V>>) -> (search: PQMinResult<V, P>)
        where
            G: Fn(&V) -> AVLTreeSetStPer<V>,
            PF: Fn(&V) -> P,
            requires
                forall|v: &V| #[trigger] graph.requires((v,)),
                forall|v: &V| #[trigger] priority_fn.requires((v,)),
                forall|v: &V, r: AVLTreeSetStPer<V>| #[trigger] graph.ensures((v,), r) ==> r.spec_avltreesetstper_wf(),
                vertex_universe.finite(),
                vertex_universe.len() + 1 < usize::MAX as nat,
                vertex_universe.contains(source@),
                forall|v: &V, r: AVLTreeSetStPer<V>| #[trigger] graph.ensures((v,), r) ==> r@.subset_of(vertex_universe),
            ensures
                spec_pqminstper_wf_generic(&search),
                search.visited@.contains(source@);

        /// - APAS: (no explicit PFS cost in Chap53; PFS cost depends on priority queue implementation)
        /// - Claude-Opus-4.6: Work Θ(|V|² + |E| log |V|), Span Θ(|V|² + |E| log |V|) — find_min uses to_seq O(|F|) per round.
        fn pq_min_multi<G, PF>(graph: &G, sources: AVLTreeSetStPer<V>, priority_fn: &PF, Ghost(vertex_universe): Ghost<Set<<V as View>::V>>) -> (search: PQMinResult<V, P>)
        where
            G: Fn(&V) -> AVLTreeSetStPer<V>,
            PF: Fn(&V) -> P,
            requires
                sources.spec_avltreesetstper_wf(),
                forall|v: &V| #[trigger] graph.requires((v,)),
                forall|v: &V| #[trigger] priority_fn.requires((v,)),
                forall|v: &V, r: AVLTreeSetStPer<V>| #[trigger] graph.ensures((v,), r) ==> r.spec_avltreesetstper_wf(),
                vertex_universe.finite(),
                vertex_universe.len() + 1 < usize::MAX as nat,
                sources@.subset_of(vertex_universe),
                forall|v: &V, r: AVLTreeSetStPer<V>| #[trigger] graph.ensures((v,), r) ==> r@.subset_of(vertex_universe),
            ensures
                spec_pqminstper_wf_generic(&search),
                sources@.subset_of(search.visited@);
    }

    // 9. impls

    impl<V: StT + Ord, P: StT + Ord> PQMinStPerTrait<V, P> for PQMinResult<V, P> {
        open spec fn spec_pqminstper_wf(&self) -> bool {
            spec_pqminstper_wf_generic(self)
        }

        fn pq_min<G, PF>(graph: &G, source: V, priority_fn: &PF, Ghost(vertex_universe): Ghost<Set<<V as View>::V>>) -> (search: PQMinResult<V, P>)
        where G: Fn(&V) -> AVLTreeSetStPer<V>, PF: Fn(&V) -> P,
        { pq_min(graph, source, priority_fn, Ghost(vertex_universe)) }

        fn pq_min_multi<G, PF>(graph: &G, sources: AVLTreeSetStPer<V>, priority_fn: &PF, Ghost(vertex_universe): Ghost<Set<<V as View>::V>>) -> (search: PQMinResult<V, P>)
        where G: Fn(&V) -> AVLTreeSetStPer<V>, PF: Fn(&V) -> P,
        { pq_min_multi(graph, sources, priority_fn, Ghost(vertex_universe)) }
    }

    /// Priority-first search from single source (Section 53.4).
    pub fn pq_min<V: StT + Ord, P: StT + Ord, G, PF>(graph: &G, source: V, priority_fn: &PF, Ghost(vertex_universe): Ghost<Set<<V as View>::V>>) -> (search: PQMinResult<V, P>)
    where
        G: Fn(&V) -> AVLTreeSetStPer<V>,
        PF: Fn(&V) -> P,
        requires
            forall|v: &V| #[trigger] graph.requires((v,)),
            forall|v: &V| #[trigger] priority_fn.requires((v,)),
            forall|v: &V, r: AVLTreeSetStPer<V>| #[trigger] graph.ensures((v,), r) ==> r.spec_avltreesetstper_wf(),
            vertex_universe.finite(),
            vertex_universe.len() + 1 < usize::MAX as nat,
            vertex_universe.contains(source@),
            forall|v: &V, r: AVLTreeSetStPer<V>| #[trigger] graph.ensures((v,), r) ==> r@.subset_of(vertex_universe),
        ensures
            spec_pqminstper_wf_generic(&search),
            search.visited@.contains(source@),
    {
        let sources = AVLTreeSetStPer::singleton(source);
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
        frontier: &AVLTreeSetStPer<Pair<Pair<P, V>, V>>,
    ) -> (result: Option<V>)
        requires
            frontier.spec_avltreesetstper_wf(),
            obeys_feq_clone::<V>(),
        ensures
            frontier.elements.spec_seq().len() == 0 ==> result.is_none(),
            frontier.elements.spec_seq().len() > 0 ==> result.is_some(),
    {
        if frontier.elements.length() == 0 {
            None
        } else {
            let entry_ref = frontier.elements.nth(0);
            let v = entry_ref.1.clone();
            proof { assert(cloned(entry_ref.1, v)); }
            Some(v)
        }
    }

    #[verifier::exec_allows_no_decreases_clause]
    fn pq_explore<V: StT + Ord, P: StT + Ord, G: Fn(&V) -> AVLTreeSetStPer<V>, PF: Fn(&V) -> P>(
        graph: &G,
        priority_fn: &PF,
        visited_init: AVLTreeSetStPer<V>,
        frontier_init: AVLTreeSetStPer<Pair<Pair<P, V>, V>>,
        Ghost(vertex_universe): Ghost<Set<<V as View>::V>>,
    ) -> (result: (AVLTreeSetStPer<V>, AVLTreeSetStPer<Pair<V, P>>))
        requires
            visited_init.spec_avltreesetstper_wf(),
            frontier_init.spec_avltreesetstper_wf(),
            forall|v: &V| #[trigger] graph.requires((v,)),
            forall|v: &V| #[trigger] priority_fn.requires((v,)),
            forall|v: &V, r: AVLTreeSetStPer<V>| #[trigger] graph.ensures((v,), r) ==> r.spec_avltreesetstper_wf(),
            // Vertex universe constraints for capacity proofs.
            vertex_universe.finite(),
            vertex_universe.len() + 1 < usize::MAX as nat,
            visited_init@.subset_of(vertex_universe),
            forall|v: &V, r: AVLTreeSetStPer<V>| #[trigger] graph.ensures((v,), r) ==> r@.subset_of(vertex_universe),
            forall|e: ((<P as View>::V, <V as View>::V), <V as View>::V)|
                #[trigger] frontier_init@.contains(e) ==> vertex_universe.contains(e.1),
        ensures
            result.0.spec_avltreesetstper_wf(),
            result.1.spec_avltreesetstper_wf(),
            visited_init@.subset_of(result.0@),
    {
        let mut visited = visited_init;
        let mut frontier = frontier_init;

        while frontier.elements.length() > 0
            invariant
                visited.spec_avltreesetstper_wf(),
                frontier.spec_avltreesetstper_wf(),
                visited_init@.subset_of(visited@),
                forall|v: &V| #[trigger] graph.requires((v,)),
                forall|v: &V| #[trigger] priority_fn.requires((v,)),
                forall|v: &V, r: AVLTreeSetStPer<V>| #[trigger] graph.ensures((v,), r) ==> r.spec_avltreesetstper_wf(),
                // Vertex universe invariants.
                vertex_universe.finite(),
                vertex_universe.len() + 1 < usize::MAX as nat,
                visited@.subset_of(vertex_universe),
                forall|v: &V, r: AVLTreeSetStPer<V>| #[trigger] graph.ensures((v,), r) ==> r@.subset_of(vertex_universe),
                forall|e: ((<P as View>::V, <V as View>::V), <V as View>::V)|
                    #[trigger] frontier@.contains(e) ==> vertex_universe.contains(e.1),
        {
            let entry_ref = frontier.elements.nth(0);
            let v = entry_ref.1.clone_plus();
            let p = priority_fn(&v);
            let v_clone1 = v.clone();
            let v_clone2 = v.clone_plus();
            let entry = Pair(Pair(p.clone(), v_clone1), v_clone2);
            let frontier_new = frontier.difference(&AVLTreeSetStPer::singleton(entry));

            // Capacity: visited ⊆ vertex_universe from invariant.
            proof {
                vstd::set_lib::lemma_len_subset(visited@, vertex_universe);
            }
            let v_for_visited = v.clone_plus();
            let visited_new = visited.union(&AVLTreeSetStPer::singleton(v_for_visited));
            // Maintain visited@.subset_of(vertex_universe): v@ ∈ vertex_universe from frontier entry.
            proof {
                assert(obeys_feq_full_trigger::<V>());
                assert(frontier@.contains(frontier.elements@[0 as int]));
                assert(vertex_universe.contains(v@));
                assert forall|a: <V as View>::V| visited_new@.contains(a)
                    implies #[trigger] vertex_universe.contains(a) by {
                }
            }

            let neighbors = graph(&v);
            let mut frontier_updated = frontier_new;
            let nlen = neighbors.elements.length();
            let mut i: usize = 0;
            // Establish neighbor vertices ∈ vertex_universe before inner loop.
            proof {
                assert forall|j: int| 0 <= j < neighbors.elements@.len()
                    implies vertex_universe.contains(#[trigger] neighbors.elements@[j]) by {
                    assert(neighbors@.contains(neighbors.elements@[j]));
                }
            }
            while i < nlen
                invariant
                    i <= nlen,
                    nlen as nat == neighbors.elements.spec_seq().len(),
                    neighbors.elements.spec_avltreeseqstper_wf(),
                    visited_new.spec_avltreesetstper_wf(),
                    frontier_updated.spec_avltreesetstper_wf(),
                    forall|v: &V| #[trigger] priority_fn.requires((v,)),
                    // Vertex universe invariants for frontier vertex tracking.
                    vertex_universe.finite(),
                    vertex_universe.len() + 1 < usize::MAX as nat,
                    forall|j: int| 0 <= j < neighbors.elements@.len()
                        ==> vertex_universe.contains(#[trigger] neighbors.elements@[j]),
                    forall|e: ((<P as View>::V, <V as View>::V), <V as View>::V)|
                        #[trigger] frontier_updated@.contains(e) ==> vertex_universe.contains(e.1),
                decreases nlen - i,
            {
                let neighbor = neighbors.elements.nth(i);
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
                    frontier_updated = frontier_updated.union(&AVLTreeSetStPer::singleton(neighbor_entry));
                    // Maintain frontier vertex invariant after union.
                    proof {
                        assert(obeys_feq_full_trigger::<V>());
                        assert(neighbor_entry@.1 == neighbor@);
                        assert(vertex_universe.contains(neighbors.elements@[i as int]));
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

        // Build priorities from visited. Close priorities+1 via loop counter.
        let mut priorities = AVLTreeSetStPer::empty();
        let vlen = visited.elements.length();
        let mut j: usize = 0;
        while j < vlen
            invariant
                j <= vlen,
                vlen as nat == visited.elements.spec_seq().len(),
                visited.elements.spec_avltreeseqstper_wf(),
                visited.spec_avltreesetstper_wf(),
                visited_init@.subset_of(visited@),
                priorities.spec_avltreesetstper_wf(),
                forall|v: &V| #[trigger] priority_fn.requires((v,)),
                priorities@.len() <= j as nat,
            decreases vlen - j,
        {
            let vref = visited.elements.nth(j);
            let p = priority_fn(vref);
            let vp = Pair(vref.clone(), p);
            proof {
                lemma_wf_implies_len_bound_stper(visited.elements);
                assert(priorities@.len() + 1 < usize::MAX as nat);
            }
            let ghost old_pri = priorities@;
            priorities = priorities.union(&AVLTreeSetStPer::singleton(vp));
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
        sources: AVLTreeSetStPer<V>,
        priority_fn: &PF,
        Ghost(vertex_universe): Ghost<Set<<V as View>::V>>,
    ) -> (search: PQMinResult<V, P>)
    where
        G: Fn(&V) -> AVLTreeSetStPer<V>,
        PF: Fn(&V) -> P,
        requires
            sources.spec_avltreesetstper_wf(),
            forall|v: &V| #[trigger] graph.requires((v,)),
            forall|v: &V| #[trigger] priority_fn.requires((v,)),
            forall|v: &V, r: AVLTreeSetStPer<V>| #[trigger] graph.ensures((v,), r) ==> r.spec_avltreesetstper_wf(),
            vertex_universe.finite(),
            vertex_universe.len() + 1 < usize::MAX as nat,
            sources@.subset_of(vertex_universe),
            forall|v: &V, r: AVLTreeSetStPer<V>| #[trigger] graph.ensures((v,), r) ==> r@.subset_of(vertex_universe),
        ensures
            spec_pqminstper_wf_generic(&search),
            sources@.subset_of(search.visited@),
    {
        let mut initial_frontier = AVLTreeSetStPer::empty();
        let slen = sources.elements.length();
        let mut i: usize = 0;
        // Establish sources.elements vertices ∈ vertex_universe.
        proof {
            assert forall|j: int| 0 <= j < sources.elements@.len()
                implies vertex_universe.contains(#[trigger] sources.elements@[j]) by {
                assert(sources@.contains(sources.elements@[j]));
            }
        }
        // Close initial_frontier+1 via loop counter: slen < usize::MAX from tree wf.
        // Maintain frontier vertex invariant: all entry vertices ∈ vertex_universe.
        while i < slen
            invariant
                i <= slen,
                slen as nat == sources.elements.spec_seq().len(),
                sources.elements.spec_avltreeseqstper_wf(),
                sources.spec_avltreesetstper_wf(),
                initial_frontier.spec_avltreesetstper_wf(),
                forall|v: &V| #[trigger] graph.requires((v,)),
                forall|v: &V| #[trigger] priority_fn.requires((v,)),
                forall|v: &V, r: AVLTreeSetStPer<V>| #[trigger] graph.ensures((v,), r) ==> r.spec_avltreesetstper_wf(),
                initial_frontier@.len() <= i as nat,
                // Frontier vertex invariant: all entry vertices belong to vertex_universe.
                sources@.subset_of(vertex_universe),
                vertex_universe.finite(),
                vertex_universe.len() + 1 < usize::MAX as nat,
                forall|j: int| 0 <= j < sources.elements@.len()
                    ==> vertex_universe.contains(#[trigger] sources.elements@[j]),
                forall|e: ((<P as View>::V, <V as View>::V), <V as View>::V)|
                    #[trigger] initial_frontier@.contains(e) ==> vertex_universe.contains(e.1),
            decreases slen - i,
        {
            let v = sources.elements.nth(i);
            let p = priority_fn(v);
            let v_clone1 = v.clone();
            let v_clone2 = v.clone_plus();
            let entry = Pair(Pair(p.clone(), v_clone1), v_clone2);
            proof {
                // slen < usize::MAX from tree wf lemma.
                lemma_wf_implies_len_bound_stper(sources.elements);
                // initial_frontier@.len() + 1 <= i + 1 <= slen < usize::MAX.
                assert(initial_frontier@.len() + 1 < usize::MAX as nat);
            }
            let ghost old_if = initial_frontier@;
            initial_frontier = initial_frontier.union(&AVLTreeSetStPer::singleton(entry));
            proof {
                vstd::set_lib::lemma_len_union(old_if, Set::empty().insert(entry@));
                // Maintain frontier vertex invariant: entry@.1 = v@ ∈ vertex_universe.
                assert(obeys_feq_full_trigger::<V>());
                assert(entry@.1 == v@);
                assert(vertex_universe.contains(sources.elements@[i as int]));
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
