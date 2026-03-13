//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 53: Min-Priority Queue Search - persistent, single-threaded.
//!
//! Implements Algorithm 53.7 - Priority Queue Search framework.
//! Selects minimum priority vertices first (lower priority = higher urgency).

pub mod PQMinStPer {

    use vstd::prelude::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::AVLTreeSeqStPerTrait;
    use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
    use crate::Types::Types::*;
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
        fn pq_min<G, PF>(graph: &G, source: V, priority_fn: &PF)                         -> (search: PQMinResult<V, P>)
        where
            G: Fn(&V) -> AVLTreeSetStPer<V>,
            PF: Fn(&V) -> P,
            requires
                forall|v: &V| #[trigger] graph.requires((v,)),
                forall|v: &V| #[trigger] priority_fn.requires((v,)),
                forall|v: &V, r: AVLTreeSetStPer<V>| #[trigger] graph.ensures((v,), r) ==> r.spec_avltreesetstper_wf(),
            ensures
                spec_pqminstper_wf_generic(&search),
                search.visited@.contains(source@);

        /// - APAS: (no explicit PFS cost in Chap53; PFS cost depends on priority queue implementation)
        /// - Claude-Opus-4.6: Work Θ(|V|² + |E| log |V|), Span Θ(|V|² + |E| log |V|) — find_min uses to_seq O(|F|) per round.
        fn pq_min_multi<G, PF>(graph: &G, sources: AVLTreeSetStPer<V>, priority_fn: &PF) -> (search: PQMinResult<V, P>)
        where
            G: Fn(&V) -> AVLTreeSetStPer<V>,
            PF: Fn(&V) -> P,
            requires
                sources.spec_avltreesetstper_wf(),
                forall|v: &V| #[trigger] graph.requires((v,)),
                forall|v: &V| #[trigger] priority_fn.requires((v,)),
                forall|v: &V, r: AVLTreeSetStPer<V>| #[trigger] graph.ensures((v,), r) ==> r.spec_avltreesetstper_wf(),
            ensures
                spec_pqminstper_wf_generic(&search),
                sources@.subset_of(search.visited@);
    }

    // 9. impls

    impl<V: StT + Ord, P: StT + Ord> PQMinStPerTrait<V, P> for PQMinResult<V, P> {
        open spec fn spec_pqminstper_wf(&self) -> bool {
            spec_pqminstper_wf_generic(self)
        }

        fn pq_min<G, PF>(graph: &G, source: V, priority_fn: &PF) -> (search: PQMinResult<V, P>)
        where G: Fn(&V) -> AVLTreeSetStPer<V>, PF: Fn(&V) -> P,
        { pq_min(graph, source, priority_fn) }

        fn pq_min_multi<G, PF>(graph: &G, sources: AVLTreeSetStPer<V>, priority_fn: &PF) -> (search: PQMinResult<V, P>)
        where G: Fn(&V) -> AVLTreeSetStPer<V>, PF: Fn(&V) -> P,
        { pq_min_multi(graph, sources, priority_fn) }
    }

    /// Priority-first search from single source (Section 53.4).
    pub fn pq_min<V: StT + Ord, P: StT + Ord, G, PF>(graph: &G, source: V, priority_fn: &PF) -> (search: PQMinResult<V, P>)
    where
        G: Fn(&V) -> AVLTreeSetStPer<V>,
        PF: Fn(&V) -> P,
        requires
            forall|v: &V| #[trigger] graph.requires((v,)),
            forall|v: &V| #[trigger] priority_fn.requires((v,)),
            forall|v: &V, r: AVLTreeSetStPer<V>| #[trigger] graph.ensures((v,), r) ==> r.spec_avltreesetstper_wf(),
        ensures
            spec_pqminstper_wf_generic(&search),
            search.visited@.contains(source@),
    {
        let sources = AVLTreeSetStPer::singleton(source);
        pq_min_multi(graph, sources, priority_fn)
    }

    fn pq_find_min_priority<V: StT + Ord, P: StT + Ord>(
        frontier: &AVLTreeSetStPer<Pair<Pair<P, V>, V>>,
    ) -> (result: Option<V>)
        requires
            frontier.spec_avltreesetstper_wf(),
            obeys_feq_clone::<V>(),
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
    ) -> (result: (AVLTreeSetStPer<V>, AVLTreeSetStPer<Pair<V, P>>))
        requires
            visited_init.spec_avltreesetstper_wf(),
            frontier_init.spec_avltreesetstper_wf(),
            forall|v: &V| #[trigger] graph.requires((v,)),
            forall|v: &V| #[trigger] priority_fn.requires((v,)),
            forall|v: &V, r: AVLTreeSetStPer<V>| #[trigger] graph.ensures((v,), r) ==> r.spec_avltreesetstper_wf(),
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
        {
            // Extract min-priority vertex.
            let entry_ref = frontier.elements.nth(0);
            let v = entry_ref.1.clone();

            let p = priority_fn(&v);
            let entry = Pair(Pair(p.clone(), v.clone()), v.clone());
            let frontier_new = frontier.difference(&AVLTreeSetStPer::singleton(entry));

            let visited_new = visited.union(&AVLTreeSetStPer::singleton(v.clone()));

            let neighbors = graph(&v);
            let mut frontier_updated = frontier_new;
            let nlen = neighbors.elements.length();
            let mut i: usize = 0;
            while i < nlen
                invariant
                    i <= nlen,
                    nlen as nat == neighbors.elements.spec_seq().len(),
                    neighbors.elements.spec_avltreeseqstper_wf(),
                    visited_new.spec_avltreesetstper_wf(),
                    frontier_updated.spec_avltreesetstper_wf(),
                    forall|v: &V| #[trigger] priority_fn.requires((v,)),
                decreases nlen - i,
            {
                let neighbor = neighbors.elements.nth(i);
                if !visited_new.find(neighbor) {
                    let neighbor_p = priority_fn(neighbor);
                    let neighbor_entry = Pair(Pair(neighbor_p.clone(), neighbor.clone()), neighbor.clone());
                    frontier_updated = frontier_updated.union(&AVLTreeSetStPer::singleton(neighbor_entry));
                }
                i = i + 1;
            }

            visited = visited_new;
            frontier = frontier_updated;
        }

        // Build priorities from visited.
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
            decreases vlen - j,
        {
            let vref = visited.elements.nth(j);
            let p = priority_fn(vref);
            priorities = priorities.union(&AVLTreeSetStPer::singleton(Pair(vref.clone(), p)));
            j = j + 1;
        }
        (visited, priorities)
    }

    /// Priority-first search from multiple sources (Section 53.4).
    pub fn pq_min_multi<V: StT + Ord, P: StT + Ord, G, PF>(
        graph: &G,
        sources: AVLTreeSetStPer<V>,
        priority_fn: &PF,
    ) -> (search: PQMinResult<V, P>)
    where
        G: Fn(&V) -> AVLTreeSetStPer<V>,
        PF: Fn(&V) -> P,
        requires
            sources.spec_avltreesetstper_wf(),
            forall|v: &V| #[trigger] graph.requires((v,)),
            forall|v: &V| #[trigger] priority_fn.requires((v,)),
            forall|v: &V, r: AVLTreeSetStPer<V>| #[trigger] graph.ensures((v,), r) ==> r.spec_avltreesetstper_wf(),
        ensures
            spec_pqminstper_wf_generic(&search),
            sources@.subset_of(search.visited@),
    {
        let mut initial_frontier = AVLTreeSetStPer::empty();
        let slen = sources.elements.length();
        let mut i: usize = 0;
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
            decreases slen - i,
        {
            let v = sources.elements.nth(i);
            let p = priority_fn(v);
            let entry = Pair(Pair(p.clone(), v.clone()), v.clone());
            initial_frontier = initial_frontier.union(&AVLTreeSetStPer::singleton(entry));
            i = i + 1;
        }

        let (visited, priorities) = pq_explore(graph, priority_fn, sources, initial_frontier);

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
