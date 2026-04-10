//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Chapter 53: Min-Priority Queue Search - persistent, single-threaded.
//!
//! Implements Algorithm 53.7 - Priority Queue Search framework.
//! Selects minimum priority vertices first (lower priority = higher urgency).


//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 6. spec fns
//	Section 8. traits
//	Section 9. impls
//	Section 12. derive impls in verus!
//	Section 14. derive impls outside verus!

//		Section 1. module

pub mod PQMinStPer {


    //		Section 2. imports

    use vstd::prelude::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::AVLTreeSeqStPerTrait;
    #[cfg(verus_keep_ghost)]
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::lemma_wf_implies_len_bound_stper;
    #[cfg(verus_keep_ghost)]
    use crate::Chap38::BSTParaStEph::BSTParaStEph::view_ord_consistent;
    use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
    use crate::Types::Types::*;
    use crate::vstdplus::clone_plus::clone_plus::ClonePlus;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;
    #[cfg(verus_keep_ghost)]
    use vstd::relations::injective_on;

    verus! 
{

    //		Section 3. broadcast use


    broadcast use crate::vstdplus::feq::feq::group_feq_axioms;

    //		Section 4. type definitions


    #[verifier::reject_recursive_types(V)]
    #[verifier::reject_recursive_types(P)]
    pub struct PQMinResult<V: StT + Ord + TotalOrder, P: StT + Ord + TotalOrder> {
        pub visited: AVLTreeSetStPer<V>,
        pub priorities: AVLTreeSetStPer<Pair<V, P>>,     // (vertex, priority)
        pub parent: Option<AVLTreeSetStPer<Pair<V, V>>>, // (child, parent)
    }

    //		Section 6. spec fns


    pub open spec fn spec_pqminstper_wf_generic<V: StT + Ord + TotalOrder, P: StT + Ord + TotalOrder>(
        s: &PQMinResult<V, P>,
    ) -> bool {
        s.visited@.finite() && s.priorities@.finite()
    }

    //		Section 8. traits


    pub trait PQMinStPerTrait<V: StT + Ord + TotalOrder, P: StT + Ord + TotalOrder> {
        spec fn spec_pqminstper_wf(&self) -> bool;

        /// - Alg Analysis: Code review (Claude Opus 4.6): no explicit PFS cost in APAS — N/A
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(|V|² + |E| log |V|), Span Θ(|V|² + |E| log |V|) — find_min uses to_seq O(|F|) per round.
        fn pq_min<G, PF>(graph: &G, source: V, priority_fn: &PF, Ghost(vertex_universe): Ghost<Set<<V as View>::V>>, Ghost(spec_priority): Ghost<spec_fn(<V as View>::V) -> <P as View>::V>) -> (search: PQMinResult<V, P>)
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
                forall|v: &V, p: P| #[trigger] priority_fn.ensures((v,), p) ==> p@ == spec_priority(v@),
                vstd::laws_cmp::obeys_cmp_spec::<V>(),
                view_ord_consistent::<V>(),
                vstd::laws_cmp::obeys_cmp_spec::<Pair<Pair<P, V>, V>>(),
                view_ord_consistent::<Pair<Pair<P, V>, V>>(),
                vstd::laws_cmp::obeys_cmp_spec::<Pair<V, P>>(),
                view_ord_consistent::<Pair<V, P>>(),
            ensures
                spec_pqminstper_wf_generic(&search),
                search.visited@.contains(source@);

        /// - Alg Analysis: Code review (Claude Opus 4.6): no explicit PFS cost in APAS — N/A
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work Θ(|V|² + |E| log |V|), Span Θ(|V|² + |E| log |V|) — find_min uses to_seq O(|F|) per round.
        fn pq_min_multi<G, PF>(graph: &G, sources: AVLTreeSetStPer<V>, priority_fn: &PF, Ghost(vertex_universe): Ghost<Set<<V as View>::V>>, Ghost(spec_priority): Ghost<spec_fn(<V as View>::V) -> <P as View>::V>) -> (search: PQMinResult<V, P>)
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
                forall|v: &V, p: P| #[trigger] priority_fn.ensures((v,), p) ==> p@ == spec_priority(v@),
                vstd::laws_cmp::obeys_cmp_spec::<V>(),
                view_ord_consistent::<V>(),
                vstd::laws_cmp::obeys_cmp_spec::<Pair<Pair<P, V>, V>>(),
                view_ord_consistent::<Pair<Pair<P, V>, V>>(),
                vstd::laws_cmp::obeys_cmp_spec::<Pair<V, P>>(),
                view_ord_consistent::<Pair<V, P>>(),
            ensures
                spec_pqminstper_wf_generic(&search),
                sources@.subset_of(search.visited@);
    }

    //		Section 9. impls


    impl<V: StT + Ord + TotalOrder, P: StT + Ord + TotalOrder> PQMinStPerTrait<V, P> for PQMinResult<V, P> {
        open spec fn spec_pqminstper_wf(&self) -> bool {
            spec_pqminstper_wf_generic(self)
            && obeys_feq_full::<V>()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|^2 + |E| log |V|), Span O(|V|^2 + |E| log |V|) — delegates to free fn; St sequential.
        fn pq_min<G, PF>(graph: &G, source: V, priority_fn: &PF, Ghost(vertex_universe): Ghost<Set<<V as View>::V>>, Ghost(spec_priority): Ghost<spec_fn(<V as View>::V) -> <P as View>::V>) -> (search: PQMinResult<V, P>)
        where G: Fn(&V) -> AVLTreeSetStPer<V>, PF: Fn(&V) -> P,
        {
         pq_min(graph, source, priority_fn, Ghost(vertex_universe), Ghost(spec_priority)) }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|^2 + |E| log |V|), Span O(|V|^2 + |E| log |V|) — delegates to free fn; St sequential.
        fn pq_min_multi<G, PF>(graph: &G, sources: AVLTreeSetStPer<V>, priority_fn: &PF, Ghost(vertex_universe): Ghost<Set<<V as View>::V>>, Ghost(spec_priority): Ghost<spec_fn(<V as View>::V) -> <P as View>::V>) -> (search: PQMinResult<V, P>)
        where G: Fn(&V) -> AVLTreeSetStPer<V>, PF: Fn(&V) -> P,
        {
         pq_min_multi(graph, sources, priority_fn, Ghost(vertex_universe), Ghost(spec_priority)) }
    }

    /// Priority-first search from single source (Section 53.4).
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|^2 + |E| log |V|), Span O(|V|^2 + |E| log |V|) — delegates to pq_min_multi; St sequential.
    pub fn pq_min<V: StT + Ord + TotalOrder, P: StT + Ord + TotalOrder, G, PF>(graph: &G, source: V, priority_fn: &PF, Ghost(vertex_universe): Ghost<Set<<V as View>::V>>, Ghost(spec_priority): Ghost<spec_fn(<V as View>::V) -> <P as View>::V>) -> (search: PQMinResult<V, P>)
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
            forall|v: &V, p: P| #[trigger] priority_fn.ensures((v,), p) ==> p@ == spec_priority(v@),
            vstd::laws_cmp::obeys_cmp_spec::<V>(),
            view_ord_consistent::<V>(),
            vstd::laws_cmp::obeys_cmp_spec::<Pair<Pair<P, V>, V>>(),
            view_ord_consistent::<Pair<Pair<P, V>, V>>(),
            vstd::laws_cmp::obeys_cmp_spec::<Pair<V, P>>(),
            view_ord_consistent::<Pair<V, P>>(),
        ensures
            spec_pqminstper_wf_generic(&search),
            search.visited@.contains(source@),
    {
        let sources = AVLTreeSetStPer::singleton(source);
// Veracity: UNNEEDED proof block         proof {
// Veracity: UNNEEDED proof block         }
        pq_min_multi(graph, sources, priority_fn, Ghost(vertex_universe), Ghost(spec_priority))
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log |frontier|), Span O(log |frontier|) — AVL to_seq + nth(0) for min; St sequential.
    fn pq_find_min_priority<V: StT + Ord + TotalOrder, P: StT + Ord + TotalOrder>(
        frontier: &AVLTreeSetStPer<Pair<Pair<P, V>, V>>,
    ) -> (min_vertex: Option<V>)
        requires
            frontier.spec_avltreesetstper_wf(),
            obeys_feq_clone::<V>(),
        ensures
            frontier@.len() == 0 ==> min_vertex.is_none(),
            frontier@.len() > 0 ==> min_vertex.is_some(),
    {
        if frontier.size() == 0 {
            None
        } else {
            let seq = frontier.to_seq();
            // Veracity: NEEDED proof block
            proof {
                if seq@.len() == 0 {
                    // Veracity: NEEDED assert
                    assert(seq@.to_set() =~= Set::<<Pair<Pair<P, V>, V> as View>::V>::empty());
                }
            }
            let entry_ref = seq.nth(0);
            let v = entry_ref.1.clone();
            Some(v)
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|^2 + |E| log |V|), Span O(|V|^2 + |E| log |V|) — |V| rounds × (find_min + neighbor scan × AVL insert); St sequential.
    #[verifier::exec_allows_no_decreases_clause]
    fn pq_explore<V: StT + Ord + TotalOrder, P: StT + Ord + TotalOrder, G: Fn(&V) -> AVLTreeSetStPer<V>, PF: Fn(&V) -> P>(
        graph: &G,
        priority_fn: &PF,
        visited_init: AVLTreeSetStPer<V>,
        frontier_init: AVLTreeSetStPer<Pair<Pair<P, V>, V>>,
        Ghost(vertex_universe): Ghost<Set<<V as View>::V>>,
        Ghost(spec_priority): Ghost<spec_fn(<V as View>::V) -> <P as View>::V>,
    ) -> (explored: (AVLTreeSetStPer<V>, AVLTreeSetStPer<Pair<V, P>>))
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
            // Priority function view-determinism and frontier entry structure.
            forall|v: &V, p: P| #[trigger] priority_fn.ensures((v,), p) ==> p@ == spec_priority(v@),
            forall|e: ((<P as View>::V, <V as View>::V), <V as View>::V)|
                #[trigger] frontier_init@.contains(e) ==> e.0 == (spec_priority(e.1), e.1),
            // Comparison axioms for AVLTreeSetStPer operations.
            vstd::laws_cmp::obeys_cmp_spec::<V>(),
            view_ord_consistent::<V>(),
            vstd::laws_cmp::obeys_cmp_spec::<Pair<Pair<P, V>, V>>(),
            view_ord_consistent::<Pair<Pair<P, V>, V>>(),
            vstd::laws_cmp::obeys_cmp_spec::<Pair<V, P>>(),
            view_ord_consistent::<Pair<V, P>>(),
        ensures
            explored.0.spec_avltreesetstper_wf(),
            explored.1.spec_avltreesetstper_wf(),
            visited_init@.subset_of(explored.0@),
    {
        let mut visited = visited_init;
        let mut frontier = frontier_init;

        while frontier.size() > 0
            invariant
                visited.spec_avltreesetstper_wf(),
                frontier.spec_avltreesetstper_wf(),
                visited_init@.subset_of(visited@),
                forall|v: &V| #[trigger] graph.requires((v,)),
                forall|v: &V| #[trigger] priority_fn.requires((v,)),
                forall|v: &V, p: P| #[trigger] priority_fn.ensures((v,), p) ==> p@ == spec_priority(v@),
                forall|v: &V, r: AVLTreeSetStPer<V>| #[trigger] graph.ensures((v,), r) ==> r.spec_avltreesetstper_wf(),
                // Vertex universe invariants.
                vertex_universe.finite(),
                vertex_universe.len() + 1 < usize::MAX as nat,
                visited@.subset_of(vertex_universe),
                forall|v: &V, r: AVLTreeSetStPer<V>| #[trigger] graph.ensures((v,), r) ==> r@.subset_of(vertex_universe),
                forall|e: ((<P as View>::V, <V as View>::V), <V as View>::V)|
                    #[trigger] frontier@.contains(e) ==> vertex_universe.contains(e.1),
                // Frontier entry structure: entries determined by vertex via spec_priority.
                forall|e: ((<P as View>::V, <V as View>::V), <V as View>::V)|
                    #[trigger] frontier@.contains(e) ==> e.0 == (spec_priority(e.1), e.1),
                // Comparison axioms.
                vstd::laws_cmp::obeys_cmp_spec::<V>(),
                view_ord_consistent::<V>(),
                vstd::laws_cmp::obeys_cmp_spec::<Pair<Pair<P, V>, V>>(),
                view_ord_consistent::<Pair<Pair<P, V>, V>>(),
                vstd::laws_cmp::obeys_cmp_spec::<Pair<V, P>>(),
                view_ord_consistent::<Pair<V, P>>(),
        {
            // Veracity: NEEDED proof block
            let frontier_seq = frontier.to_seq();
            proof {
                // Prove frontier_seq is non-empty from frontier@.len() > 0.
                if frontier_seq@.len() == 0 {
                    // Veracity: NEEDED assert
                    assert(frontier_seq@.to_set() =~= Set::<<Pair<Pair<P, V>, V> as View>::V>::empty());
                }
            }
            let entry_ref = frontier_seq.nth(0);
            let v = entry_ref.1.clone_plus();
            let p = priority_fn(&v);
            let v_clone1 = v.clone_plus();
            let v_clone2 = v.clone_plus();
            let entry = Pair(Pair(p, v_clone1), v_clone2);
            let frontier_new = frontier.difference(&AVLTreeSetStPer::singleton(entry));
// Veracity: NEEDED proof block

            // Capacity: visited ⊆ vertex_universe from invariant.
            proof {
                vstd::set_lib::lemma_len_subset(visited@, vertex_universe);
            }
            // Veracity: NEEDED proof block
            let v_for_visited = v.clone_plus();
            let visited_new = visited.union(&AVLTreeSetStPer::singleton(v_for_visited));
            // Maintain visited@.subset_of(vertex_universe): v@ ∈ vertex_universe from frontier entry.
            proof {
                // Veracity: NEEDED assert
                assert(frontier@.contains(frontier_seq@[0 as int]));
            }

            let neighbors = graph(&v);
            let neighbors_seq = neighbors.to_seq();
            // Veracity: NEEDED proof block
            let mut frontier_updated = frontier_new;
            let nlen = neighbors_seq.length();
            let mut i: usize = 0;
            // Establish neighbor vertices ∈ vertex_universe before inner loop.
            proof {
            }
            while i < nlen
                invariant
                    i <= nlen,
                    nlen as nat == neighbors_seq.spec_seq().len(),
                    neighbors_seq.spec_avltreeseqstper_wf(),
                    visited_new.spec_avltreesetstper_wf(),
                    frontier_updated.spec_avltreesetstper_wf(),
                    forall|v: &V| #[trigger] priority_fn.requires((v,)),
                    forall|v: &V, p: P| #[trigger] priority_fn.ensures((v,), p) ==> p@ == spec_priority(v@),
                    // Vertex universe invariants for frontier vertex tracking.
                    vertex_universe.finite(),
                    vertex_universe.len() + 1 < usize::MAX as nat,
                    forall|j: int| 0 <= j < neighbors_seq@.len()
                        ==> vertex_universe.contains(#[trigger] neighbors_seq@[j]),
                    forall|e: ((<P as View>::V, <V as View>::V), <V as View>::V)|
                        #[trigger] frontier_updated@.contains(e) ==> vertex_universe.contains(e.1),
                    // Frontier entry structure: entries determined by vertex via spec_priority.
                    forall|e: ((<P as View>::V, <V as View>::V), <V as View>::V)|
                        #[trigger] frontier_updated@.contains(e) ==> e.0 == (spec_priority(e.1), e.1),
                    // Comparison axioms.
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
                    // Veracity: NEEDED proof block
                    let neighbor_clone2 = neighbor.clone_plus();
                    let neighbor_entry = Pair(Pair(neighbor_p, neighbor_clone1), neighbor_clone2);
                    // Capacity via injection: frontier entries are uniquely determined by
                    // their vertex component, so |frontier| ≤ |vertex_universe|.
                    let ghost old_fu = frontier_updated@;
                    proof {
                        // Establish neighbor_entry structure.
                        // Define injection: entry ↦ entry.1 (vertex component).
                        let f = |e: ((<P as View>::V, <V as View>::V), <V as View>::V)| -> <V as View>::V { e.1 };
                        // Prove injection is injective on frontier_updated@.
                        // Image: frontier.map(f) ⊆ vertex_universe.
                        let projected = frontier_updated@.map(f);
                        frontier_updated@.lemma_map_finite(f);
                        // |frontier| == |projected| via injective map.
                        vstd::set_lib::lemma_map_size(frontier_updated@, projected, f);
// Veracity: UNNEEDED proof block                         // |projected| ≤ |vertex_universe| via subset.
// Veracity: UNNEEDED proof block                         vstd::set_lib::lemma_len_subset(projected, vertex_universe);
// Veracity: UNNEEDED proof block                         // Combine: frontier.len() ≤ vertex_universe.len() < usize::MAX - 1.
// Veracity: UNNEEDED proof block                     }
// Veracity: UNNEEDED proof block                     frontier_updated = frontier_updated.union(&AVLTreeSetStPer::singleton(neighbor_entry));
// Veracity: UNNEEDED proof block                     // Maintain frontier vertex and entry structure invariants after union.
// Veracity: UNNEEDED proof block                     proof {
// Veracity: UNNEEDED proof block                         // Veracity: NEEDED assert
// Veracity: UNNEEDED proof block                         assert(vertex_universe.contains(neighbors_seq@[i as int]));
// Veracity: UNNEEDED proof block                         // Veracity: NEEDED assert
// Veracity: UNNEEDED proof block                         assert forall|e: ((<P as View>::V, <V as View>::V), <V as View>::V)|
// Veracity: UNNEEDED proof block                             #[trigger] frontier_updated@.contains(e)
// Veracity: UNNEEDED proof block                             implies vertex_universe.contains(e.1) by {
// Veracity: UNNEEDED proof block                             if old_fu.contains(e) {}
// Veracity: UNNEEDED proof block                         }
// Veracity: UNNEEDED proof block                         // Veracity: NEEDED assert
                        assert forall|e: ((<P as View>::V, <V as View>::V), <V as View>::V)|
                            #[trigger] frontier_updated@.contains(e)
                            implies e.0 == (spec_priority(e.1), e.1) by {
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
        let visited_seq = visited.to_seq();
        let vlen = visited_seq.length();
        let mut j: usize = 0;
        while j < vlen
            invariant
                j <= vlen,
                vlen as nat == visited_seq.spec_seq().len(),
                visited_seq.spec_avltreeseqstper_wf(),
                visited.spec_avltreesetstper_wf(),
                visited_init@.subset_of(visited@),
                priorities.spec_avltreesetstper_wf(),
                forall|v: &V| #[trigger] priority_fn.requires((v,)),
                priorities@.len() <= j as nat,
                vstd::laws_cmp::obeys_cmp_spec::<Pair<V, P>>(),
                // Veracity: NEEDED proof block
                view_ord_consistent::<Pair<V, P>>(),
            decreases vlen - j,
        {
            let vref = visited_seq.nth(j);
            // Veracity: NEEDED proof block
            let p = priority_fn(vref);
            let vp = Pair(vref.clone(), p);
            proof {
                lemma_wf_implies_len_bound_stper(visited_seq);
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
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|^2 + |E| log |V|), Span O(|V|^2 + |E| log |V|) — delegates to pq_explore; St sequential.
    pub fn pq_min_multi<V: StT + Ord + TotalOrder, P: StT + Ord + TotalOrder, G, PF>(
        graph: &G,
        sources: AVLTreeSetStPer<V>,
        priority_fn: &PF,
        Ghost(vertex_universe): Ghost<Set<<V as View>::V>>,
        Ghost(spec_priority): Ghost<spec_fn(<V as View>::V) -> <P as View>::V>,
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
            forall|v: &V, p: P| #[trigger] priority_fn.ensures((v,), p) ==> p@ == spec_priority(v@),
            vstd::laws_cmp::obeys_cmp_spec::<V>(),
            view_ord_consistent::<V>(),
            vstd::laws_cmp::obeys_cmp_spec::<Pair<Pair<P, V>, V>>(),
            view_ord_consistent::<Pair<Pair<P, V>, V>>(),
            vstd::laws_cmp::obeys_cmp_spec::<Pair<V, P>>(),
            view_ord_consistent::<Pair<V, P>>(),
        ensures
            // Veracity: NEEDED proof block
            spec_pqminstper_wf_generic(&search),
            sources@.subset_of(search.visited@),
    {
        let mut initial_frontier = AVLTreeSetStPer::empty();
        let sources_seq = sources.to_seq();
        let slen = sources_seq.length();
        let mut i: usize = 0;
        // Establish sources_seq vertices ∈ vertex_universe.
        proof {
        }
        // Close initial_frontier+1 via loop counter: slen < usize::MAX from tree wf.
        // Maintain frontier vertex invariant: all entry vertices ∈ vertex_universe.
        while i < slen
            invariant
                i <= slen,
                slen as nat == sources_seq.spec_seq().len(),
                sources_seq.spec_avltreeseqstper_wf(),
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
                forall|j: int| 0 <= j < sources_seq@.len()
                    ==> vertex_universe.contains(#[trigger] sources_seq@[j]),
                forall|e: ((<P as View>::V, <V as View>::V), <V as View>::V)|
                    #[trigger] initial_frontier@.contains(e) ==> vertex_universe.contains(e.1),
                // Frontier entry structure for injection proof.
                forall|v: &V, p: P| #[trigger] priority_fn.ensures((v,), p) ==> p@ == spec_priority(v@),
                forall|e: ((<P as View>::V, <V as View>::V), <V as View>::V)|
                    #[trigger] initial_frontier@.contains(e) ==> e.0 == (spec_priority(e.1), e.1),
                // Comparison axioms.
                // Veracity: NEEDED proof block
                vstd::laws_cmp::obeys_cmp_spec::<Pair<Pair<P, V>, V>>(),
                view_ord_consistent::<Pair<Pair<P, V>, V>>(),
            decreases slen - i,
        {
            let v = sources_seq.nth(i);
            let p = priority_fn(v);
            // Veracity: NEEDED proof block
            let v_clone1 = v.clone_plus();
            let v_clone2 = v.clone_plus();
            let entry = Pair(Pair(p, v_clone1), v_clone2);
            proof {
                // slen < usize::MAX from tree wf lemma.
                lemma_wf_implies_len_bound_stper(sources_seq);
                // initial_frontier@.len() + 1 <= i + 1 <= slen < usize::MAX.
            }
            let ghost old_if = initial_frontier@;
            initial_frontier = initial_frontier.union(&AVLTreeSetStPer::singleton(entry));
            proof {
                vstd::set_lib::lemma_len_union(old_if, Set::empty().insert(entry@));
                // Maintain frontier vertex invariant: entry@.1 = v@ ∈ vertex_universe.
                // Veracity: NEEDED assert
                assert forall|e: ((<P as View>::V, <V as View>::V), <V as View>::V)|
                    #[trigger] initial_frontier@.contains(e)
                    implies vertex_universe.contains(e.1) by {
                    if old_if.contains(e) {}
                }
                // Maintain entry structure invariant.
                // Veracity: NEEDED assert
                assert forall|e: ((<P as View>::V, <V as View>::V), <V as View>::V)|
                    #[trigger] initial_frontier@.contains(e)
                    implies e.0 == (spec_priority(e.1), e.1) by {
                    if old_if.contains(e) {}
                }
            }
            i = i + 1;
        }

        let (visited, priorities) = pq_explore(graph, priority_fn, sources, initial_frontier, Ghost(vertex_universe), Ghost(spec_priority));

        PQMinResult {
            visited,
            priorities,
            parent: None,
        }
    }

    //		Section 12. derive impls in verus!


    impl<V: StT + Ord + TotalOrder, P: StT + Ord + TotalOrder> Clone for PQMinResult<V, P> {
        fn clone(&self) -> (out: Self) {
            PQMinResult {
                visited: self.visited.clone(),
                priorities: self.priorities.clone(),
                parent: self.parent.clone(),
            }
        }
    }

    } // verus!

    //		Section 14. derive impls outside verus!


    impl<V: StT + Ord + TotalOrder, P: StT + Ord + TotalOrder> std::fmt::Debug for PQMinResult<V, P> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("PQMinResult")
                .field("visited", &self.visited)
                .field("priorities", &self.priorities)
                .field("parent", &self.parent)
                .finish()
        }
    }

    impl<V: StT + Ord + TotalOrder, P: StT + Ord + TotalOrder> std::fmt::Display for PQMinResult<V, P> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "PQMinResult(visited={}, priorities={})", self.visited.size(), self.priorities.size())
        }
    }
}
