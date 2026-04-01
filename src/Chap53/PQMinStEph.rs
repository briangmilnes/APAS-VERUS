//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Chapter 53: Min-Priority Queue Search - ephemeral, single-threaded.

pub mod PQMinStEph {

    use vstd::prelude::*;
    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::AVLTreeSeqStEphTrait;
    #[cfg(verus_keep_ghost)]
    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::lemma_wf_implies_len_bound_steph;
    #[cfg(verus_keep_ghost)]
    use crate::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent;
    use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
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

        /// - Alg Analysis: APAS (Ch53 PFS): (no explicit PFS cost in Chap53; PFS cost depends on priority queue implementation)
        /// - Alg Analysis: Code review (Claude Opus 4.6): no explicit PFS cost in APAS — N/A
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
                // Priority function view-determinism: same vertex view implies same priority view.
                forall|v1: &V, v2: &V, r1: P, r2: P|
                    #![trigger priority_fn.ensures((v1,), r1), priority_fn.ensures((v2,), r2)]
                    v1@ == v2@ && priority_fn.ensures((v1,), r1) && priority_fn.ensures((v2,), r2)
                    ==> r1@ == r2@,
                // Type axioms for AVLTreeSetStEph operations.
                vstd::laws_cmp::obeys_cmp_spec::<V>(),
                view_ord_consistent::<V>(),
                vstd::laws_cmp::obeys_cmp_spec::<Pair<Pair<P, V>, V>>(),
                view_ord_consistent::<Pair<Pair<P, V>, V>>(),
                vstd::laws_cmp::obeys_cmp_spec::<Pair<V, P>>(),
                view_ord_consistent::<Pair<V, P>>(),
            ensures
                spec_pqminsteph_wf_generic(&search),
                search.visited@.contains(source@);

        /// - Alg Analysis: APAS (Ch53 PFS): (no explicit PFS cost in Chap53; PFS cost depends on priority queue implementation)
        /// - Alg Analysis: Code review (Claude Opus 4.6): no explicit PFS cost in APAS — N/A
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
                forall|v1: &V, v2: &V, r1: P, r2: P|
                    #![trigger priority_fn.ensures((v1,), r1), priority_fn.ensures((v2,), r2)]
                    v1@ == v2@ && priority_fn.ensures((v1,), r1) && priority_fn.ensures((v2,), r2)
                    ==> r1@ == r2@,
                // Type axioms for AVLTreeSetStEph operations.
                vstd::laws_cmp::obeys_cmp_spec::<V>(),
                view_ord_consistent::<V>(),
                vstd::laws_cmp::obeys_cmp_spec::<Pair<Pair<P, V>, V>>(),
                view_ord_consistent::<Pair<Pair<P, V>, V>>(),
                vstd::laws_cmp::obeys_cmp_spec::<Pair<V, P>>(),
                view_ord_consistent::<Pair<V, P>>(),
            ensures
                spec_pqminsteph_wf_generic(&search),
                sources@.subset_of(search.visited@);
    }

    // 9. impls

    impl<V: StT + Ord, P: StT + Ord> PQMinStEphTrait<V, P> for PQMinResult<V, P> {
        open spec fn spec_pqminsteph_wf(&self) -> bool {
            spec_pqminsteph_wf_generic(self)
            && obeys_feq_fulls::<P, V>()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|^2 + |E| log |V|), Span O(|V|^2 + |E| log |V|) — delegates to pq_min_multi; St sequential.
        fn pq_min<G, PF>(graph: &G, source: V, priority_fn: &PF, Ghost(vertex_universe): Ghost<Set<<V as View>::V>>) -> (search: PQMinResult<V, P>)
        where G: Fn(&V) -> AVLTreeSetStEph<V>, PF: Fn(&V) -> P,
        {
        assert(obeys_feq_full_trigger::<P>());
        assert(obeys_feq_full_trigger::<V>());
         pq_min(graph, source, priority_fn, Ghost(vertex_universe)) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|^2 + |E| log |V|), Span O(|V|^2 + |E| log |V|) — delegates to free fn pq_min_multi; St sequential.
        fn pq_min_multi<G, PF>(graph: &G, sources: AVLTreeSetStEph<V>, priority_fn: &PF, Ghost(vertex_universe): Ghost<Set<<V as View>::V>>) -> (search: PQMinResult<V, P>)
        where G: Fn(&V) -> AVLTreeSetStEph<V>, PF: Fn(&V) -> P,
        {
        assert(obeys_feq_full_trigger::<P>());
        assert(obeys_feq_full_trigger::<V>());
         pq_min_multi(graph, sources, priority_fn, Ghost(vertex_universe)) }
    }

    /// Priority-first search from single source (Section 53.4).
    /// - Alg Analysis: APAS (Ch53 PFS): (no explicit PFS cost in Chap53; PFS cost depends on priority queue implementation)
    /// - Alg Analysis: Code review (Claude Opus 4.6): no explicit PFS cost in APAS — N/A
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
            forall|v1: &V, v2: &V, r1: P, r2: P|
                #![trigger priority_fn.ensures((v1,), r1), priority_fn.ensures((v2,), r2)]
                v1@ == v2@ && priority_fn.ensures((v1,), r1) && priority_fn.ensures((v2,), r2)
                ==> r1@ == r2@,
            vstd::laws_cmp::obeys_cmp_spec::<V>(),
            view_ord_consistent::<V>(),
            vstd::laws_cmp::obeys_cmp_spec::<Pair<Pair<P, V>, V>>(),
            view_ord_consistent::<Pair<Pair<P, V>, V>>(),
            vstd::laws_cmp::obeys_cmp_spec::<Pair<V, P>>(),
            view_ord_consistent::<Pair<V, P>>(),
        ensures
            spec_pqminsteph_wf_generic(&search),
            search.visited@.contains(source@),
    {
              assert(obeys_feq_full_trigger::<P>());
       assert(obeys_feq_full_trigger::<V>());
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

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log |frontier|), Span O(log |frontier|) — AVL to_seq + nth(0) for min; St sequential.
    fn pq_find_min_priority<V: StT + Ord, P: StT + Ord>(
        frontier: &AVLTreeSetStEph<Pair<Pair<P, V>, V>>,
    ) -> (min_vertex: Option<V>)
        requires
            frontier.spec_avltreesetsteph_wf(),
            obeys_feq_clone::<V>(),
        ensures
            frontier@.len() == 0 ==> min_vertex.is_none(),
            frontier@.len() > 0 ==> min_vertex.is_some(),
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

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|^2 + |E| log |V|), Span O(|V|^2 + |E| log |V|) — |V| rounds × (find_min O(log V) + neighbor scan O(deg) × AVL insert O(log V)); St sequential.
    #[verifier::exec_allows_no_decreases_clause]
    fn pq_explore<V: StT + Ord, P: StT + Ord, G: Fn(&V) -> AVLTreeSetStEph<V>, PF: Fn(&V) -> P>(
        graph: &G,
        priority_fn: &PF,
        visited_init: AVLTreeSetStEph<V>,
        frontier_init: AVLTreeSetStEph<Pair<Pair<P, V>, V>>,
        Ghost(vertex_universe): Ghost<Set<<V as View>::V>>,
    ) -> (explored: (AVLTreeSetStEph<V>, AVLTreeSetStEph<Pair<V, P>>))
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
            // Priority function view-determinism.
            forall|v1: &V, v2: &V, r1: P, r2: P|
                #![trigger priority_fn.ensures((v1,), r1), priority_fn.ensures((v2,), r2)]
                v1@ == v2@ && priority_fn.ensures((v1,), r1) && priority_fn.ensures((v2,), r2)
                ==> r1@ == r2@,
            // Frontier entries are canonical: inner vertex matches outer vertex.
            forall|e: ((<P as View>::V, <V as View>::V), <V as View>::V)|
                #[trigger] frontier_init@.contains(e) ==> e.0.1 == e.1,
            // Frontier entry priorities are from priority_fn.
            forall|e: ((<P as View>::V, <V as View>::V), <V as View>::V)|
                frontier_init@.contains(e) ==> (exists|v_ref: &V, p_val: P|
                    v_ref@ == e.1 && #[trigger] priority_fn.ensures((v_ref,), p_val) && p_val@ == e.0.0),
            // Type axioms for AVLTreeSetStEph operations.
            vstd::laws_cmp::obeys_cmp_spec::<V>(),
            view_ord_consistent::<V>(),
            vstd::laws_cmp::obeys_cmp_spec::<Pair<Pair<P, V>, V>>(),
            view_ord_consistent::<Pair<Pair<P, V>, V>>(),
            vstd::laws_cmp::obeys_cmp_spec::<Pair<V, P>>(),
            view_ord_consistent::<Pair<V, P>>(),
        ensures
            explored.0.spec_avltreesetsteph_wf(),
            explored.1.spec_avltreesetsteph_wf(),
            visited_init@.subset_of(explored.0@),
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
                // Frontier canonical form: inner vertex matches outer, priority from priority_fn.
                forall|e: ((<P as View>::V, <V as View>::V), <V as View>::V)|
                    #[trigger] frontier@.contains(e) ==> e.0.1 == e.1,
                forall|e: ((<P as View>::V, <V as View>::V), <V as View>::V)|
                    frontier@.contains(e) ==> (exists|v_ref: &V, p_val: P|
                        v_ref@ == e.1 && priority_fn.ensures((v_ref,), p_val) && p_val@ == e.0.0),
                // View-determinism (from requires).
                forall|v1: &V, v2: &V, r1: P, r2: P|
                    #![trigger priority_fn.ensures((v1,), r1), priority_fn.ensures((v2,), r2)]
                    v1@ == v2@ && priority_fn.ensures((v1,), r1) && priority_fn.ensures((v2,), r2)
                    ==> r1@ == r2@,
                // Type axioms for AVLTreeSetStEph operations.
                vstd::laws_cmp::obeys_cmp_spec::<V>(),
                view_ord_consistent::<V>(),
                vstd::laws_cmp::obeys_cmp_spec::<Pair<Pair<P, V>, V>>(),
                view_ord_consistent::<Pair<Pair<P, V>, V>>(),
                vstd::laws_cmp::obeys_cmp_spec::<Pair<V, P>>(),
                view_ord_consistent::<Pair<V, P>>(),
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
                    // Frontier canonical form.
                    forall|e: ((<P as View>::V, <V as View>::V), <V as View>::V)|
                        #[trigger] frontier_updated@.contains(e) ==> e.0.1 == e.1,
                    forall|e: ((<P as View>::V, <V as View>::V), <V as View>::V)|
                        frontier_updated@.contains(e) ==> (exists|v_ref: &V, p_val: P|
                            v_ref@ == e.1 && priority_fn.ensures((v_ref,), p_val) && p_val@ == e.0.0),
                    // View-determinism (from requires).
                    forall|v1: &V, v2: &V, r1: P, r2: P|
                        #![trigger priority_fn.ensures((v1,), r1), priority_fn.ensures((v2,), r2)]
                        v1@ == v2@ && priority_fn.ensures((v1,), r1) && priority_fn.ensures((v2,), r2)
                        ==> r1@ == r2@,
                    // Type axioms for AVLTreeSetStEph operations.
                    vstd::laws_cmp::obeys_cmp_spec::<V>(),
                    view_ord_consistent::<V>(),
                    vstd::laws_cmp::obeys_cmp_spec::<Pair<Pair<P, V>, V>>(),
                    view_ord_consistent::<Pair<Pair<P, V>, V>>(),
                decreases nlen - i,
            {
                let neighbor = neighbors_seq.nth(i);
                if !visited_new.find(neighbor) {
                    let neighbor_p = priority_fn(neighbor);
                    let neighbor_clone1 = neighbor.clone_plus();
                    let neighbor_clone2 = neighbor.clone_plus();
                    let neighbor_p_clone = neighbor_p.clone_plus();
                    let neighbor_entry = Pair(Pair(neighbor_p_clone, neighbor_clone1), neighbor_clone2);
                    // Capacity: frontier_updated@.len() + 1 < usize::MAX via injection bound.
                    // View-determinism ensures each vertex has at most one frontier entry,
                    // so |frontier| <= |vertex_universe| < usize::MAX - 1.
                    let ghost old_fu = frontier_updated@;
                    proof {
                        let proj = |e: ((<P as View>::V, <V as View>::V), <V as View>::V)| -> <V as View>::V { e.1 };
                        // Prove injective_on(proj, frontier_updated@).
                        // Canonical form + priority witness invariants give existential
                        // ensures terms; view-det multi-trigger fires on both, yielding
                        // e1.0.0 == e2.0.0, combined with e1.0.1 == e2.0.1 gives e1 == e2.
                        assert(vstd::relations::injective_on(proj, frontier_updated@)) by {
                            assert forall|e1: ((<P as View>::V, <V as View>::V), <V as View>::V),
                                           e2: ((<P as View>::V, <V as View>::V), <V as View>::V)|
                                frontier_updated@.contains(e1) && frontier_updated@.contains(e2)
                                && #[trigger] proj(e1) == #[trigger] proj(e2)
                                implies e1 == e2 by {
                            }
                        };
                        // Image of projection is subset of vertex_universe.
                        let image = frontier_updated@.map(proj);
                        assert(image.subset_of(vertex_universe)) by {
                            assert forall|v: <V as View>::V| image.contains(v)
                                implies #[trigger] vertex_universe.contains(v) by {
                                let wit: ((<P as View>::V, <V as View>::V), <V as View>::V) =
                                    choose|e: ((<P as View>::V, <V as View>::V), <V as View>::V)|
                                        #[trigger] frontier_updated@.contains(e) && e.1 == v;
                                assert(frontier_updated@.contains(wit));
                            }
                        };
                        vstd::set_lib::lemma_map_size(frontier_updated@, image, proj);
                        vstd::set_lib::lemma_len_subset(image, vertex_universe);
                    }
                    frontier_updated = frontier_updated.union(&AVLTreeSetStEph::singleton(neighbor_entry));
                    // Maintain all frontier invariants after union.
                    proof {
                        assert(obeys_feq_full_trigger::<V>());
                        assert(obeys_feq_full_trigger::<P>());
                        // clone_plus gives view equality.
                        assert(neighbor_clone2@ == neighbor@);
                        assert(neighbor_clone1@ == neighbor@);
                        assert(neighbor_p_clone@ == neighbor_p@);
                        assert(neighbor_entry@.1 == neighbor@);
                        assert(neighbor_entry@.0.1 == neighbor@);
                        assert(neighbor_entry@.0.0 == neighbor_p@);
                        assert(vertex_universe.contains(neighbors_seq@[i as int]));
                        assert(vertex_universe.contains(neighbor_entry@.1));
                        // Vertex universe membership.
                        assert forall|e: ((<P as View>::V, <V as View>::V), <V as View>::V)|
                            #[trigger] frontier_updated@.contains(e)
                            implies vertex_universe.contains(e.1) by {
                            if old_fu.contains(e) {}
                        }
                        // Canonical form: inner vertex == outer vertex.
                        assert forall|e: ((<P as View>::V, <V as View>::V), <V as View>::V)|
                            #[trigger] frontier_updated@.contains(e)
                            implies e.0.1 == e.1 by {
                            if old_fu.contains(e) {}
                        }
                        // Priority witness: priority from priority_fn.
                        assert forall|e: ((<P as View>::V, <V as View>::V), <V as View>::V)|
                            frontier_updated@.contains(e)
                            implies (exists|v_ref: &V, p_val: P|
                                v_ref@ == e.1 && priority_fn.ensures((v_ref,), p_val) && p_val@ == e.0.0) by {
                            if old_fu.contains(e) {
                            } else {
                                assert(priority_fn.ensures((neighbor,), neighbor_p));
                            }
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
                vstd::laws_cmp::obeys_cmp_spec::<Pair<V, P>>(),
                view_ord_consistent::<Pair<V, P>>(),
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
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|^2 + |E| log |V|), Span O(|V|^2 + |E| log |V|) — delegates to pq_explore; St sequential.
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
            forall|v1: &V, v2: &V, r1: P, r2: P|
                #![trigger priority_fn.ensures((v1,), r1), priority_fn.ensures((v2,), r2)]
                v1@ == v2@ && priority_fn.ensures((v1,), r1) && priority_fn.ensures((v2,), r2)
                ==> r1@ == r2@,
            vstd::laws_cmp::obeys_cmp_spec::<V>(),
            view_ord_consistent::<V>(),
            vstd::laws_cmp::obeys_cmp_spec::<Pair<Pair<P, V>, V>>(),
            view_ord_consistent::<Pair<Pair<P, V>, V>>(),
            vstd::laws_cmp::obeys_cmp_spec::<Pair<V, P>>(),
            view_ord_consistent::<Pair<V, P>>(),
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
                // Frontier canonical form for pq_explore requires.
                forall|e: ((<P as View>::V, <V as View>::V), <V as View>::V)|
                    #[trigger] initial_frontier@.contains(e) ==> e.0.1 == e.1,
                forall|e: ((<P as View>::V, <V as View>::V), <V as View>::V)|
                    initial_frontier@.contains(e) ==> (exists|v_ref: &V, p_val: P|
                        v_ref@ == e.1 && priority_fn.ensures((v_ref,), p_val) && p_val@ == e.0.0),
                // Type axioms for AVLTreeSetStEph operations.
                vstd::laws_cmp::obeys_cmp_spec::<Pair<Pair<P, V>, V>>(),
                view_ord_consistent::<Pair<Pair<P, V>, V>>(),
            decreases slen - i,
        {
            let v = sources_seq.nth(i);
            let p = priority_fn(v);
            let v_clone1 = v.clone_plus();
            let v_clone2 = v.clone_plus();
            let p_clone = p.clone_plus();
            let entry = Pair(Pair(p_clone, v_clone1), v_clone2);
            proof {
                lemma_wf_implies_len_bound_steph(sources_seq);
                assert(initial_frontier@.len() + 1 < usize::MAX as nat);
            }
            let ghost old_if = initial_frontier@;
            initial_frontier = initial_frontier.union(&AVLTreeSetStEph::singleton(entry));
            proof {
                vstd::set_lib::lemma_len_union(old_if, Set::empty().insert(entry@));
                // clone_plus gives view equality.
                assert(obeys_feq_full_trigger::<V>());
                assert(obeys_feq_full_trigger::<P>());
                assert(v_clone1@ == v@);
                assert(v_clone2@ == v@);
                assert(p_clone@ == p@);
                assert(entry@.1 == v@);
                assert(entry@.0.1 == v@);
                assert(entry@.0.0 == p@);
                assert(vertex_universe.contains(sources_seq@[i as int]));
                assert(vertex_universe.contains(entry@.1));
                // Vertex universe membership.
                assert forall|e: ((<P as View>::V, <V as View>::V), <V as View>::V)|
                    #[trigger] initial_frontier@.contains(e)
                    implies vertex_universe.contains(e.1) by {
                    if old_if.contains(e) {}
                }
                // Canonical form: inner vertex == outer vertex.
                assert forall|e: ((<P as View>::V, <V as View>::V), <V as View>::V)|
                    #[trigger] initial_frontier@.contains(e)
                    implies e.0.1 == e.1 by {
                    if old_if.contains(e) {}
                }
                // Priority witness.
                assert(priority_fn.ensures((v,), p));
                assert forall|e: ((<P as View>::V, <V as View>::V), <V as View>::V)|
                    initial_frontier@.contains(e)
                    implies (exists|v_ref: &V, p_val: P|
                        v_ref@ == e.1 && priority_fn.ensures((v_ref,), p_val) && p_val@ == e.0.0) by {
                    if old_if.contains(e) {
                    } else {
                        assert(priority_fn.ensures((v,), p));
                    }
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
