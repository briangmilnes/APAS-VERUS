//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 53: Min-Priority Queue Search - ephemeral, single-threaded.

pub mod PQMinStEph {

    use vstd::prelude::*;
    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::AVLTreeSeqStEphTrait;
    use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::accept::accept;

    verus! {

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
        fn pq_min<G, PF>(graph: &G, source: V, priority_fn: &PF)                         -> (search: PQMinResult<V, P>)
        where
            G: Fn(&V) -> AVLTreeSetStEph<V>,
            PF: Fn(&V) -> P,
            ensures
                spec_pqminsteph_wf_generic(&search),
                search.visited@.contains(source@);

        /// - APAS: (no explicit PFS cost in Chap53; PFS cost depends on priority queue implementation)
        /// - Claude-Opus-4.6: Work Θ(|V|² + |E| log |V|), Span Θ(|V|² + |E| log |V|) — find_min uses to_seq O(|F|) per round.
        fn pq_min_multi<G, PF>(graph: &G, sources: AVLTreeSetStEph<V>, priority_fn: &PF) -> (search: PQMinResult<V, P>)
        where
            G: Fn(&V) -> AVLTreeSetStEph<V>,
            PF: Fn(&V) -> P,
            ensures
                spec_pqminsteph_wf_generic(&search),
                sources@.subset_of(search.visited@);
    }

    // 9. impls

    impl<V: StT + Ord, P: StT + Ord> PQMinStEphTrait<V, P> for PQMinResult<V, P> {
        open spec fn spec_pqminsteph_wf(&self) -> bool {
            spec_pqminsteph_wf_generic(self)
        }

        fn pq_min<G, PF>(graph: &G, source: V, priority_fn: &PF) -> (search: PQMinResult<V, P>)
        where G: Fn(&V) -> AVLTreeSetStEph<V>, PF: Fn(&V) -> P,
        { pq_min(graph, source, priority_fn) }

        fn pq_min_multi<G, PF>(graph: &G, sources: AVLTreeSetStEph<V>, priority_fn: &PF) -> (search: PQMinResult<V, P>)
        where G: Fn(&V) -> AVLTreeSetStEph<V>, PF: Fn(&V) -> P,
        { pq_min_multi(graph, sources, priority_fn) }
    }

    /// Priority-first search from single source (Section 53.4).
    /// - APAS: (no explicit PFS cost in Chap53; PFS cost depends on priority queue implementation)
    /// - Claude-Opus-4.6: Work Θ(|V|² + |E| log |V|), Span Θ(|V|² + |E| log |V|) — delegates to pq_min_multi.
    pub fn pq_min<V: StT + Ord, P: StT + Ord, G, PF>(graph: &G, source: V, priority_fn: &PF) -> (search: PQMinResult<V, P>)
    where
        G: Fn(&V) -> AVLTreeSetStEph<V>,
        PF: Fn(&V) -> P,
        ensures
            spec_pqminsteph_wf_generic(&search),
            search.visited@.contains(source@),
    {
        let sources = AVLTreeSetStEph::singleton(source);
        pq_min_multi(graph, sources, priority_fn)
    }

    fn pq_find_min_priority<V: StT + Ord, P: StT + Ord>(
        frontier: &AVLTreeSetStEph<Pair<Pair<P, V>, V>>,
    ) -> (result: Option<V>) {
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
            proof { accept(v@ == entry_ref.1@); }  // accept hole: V::clone external_body
            Some(v)
        }
    }

    #[verifier::exec_allows_no_decreases_clause]
    fn pq_explore<V: StT + Ord, P: StT + Ord, G: Fn(&V) -> AVLTreeSetStEph<V>, PF: Fn(&V) -> P>(
        graph: &G,
        priority_fn: &PF,
        visited_init: AVLTreeSetStEph<V>,
        frontier_init: AVLTreeSetStEph<Pair<Pair<P, V>, V>>,
    ) -> (result: (AVLTreeSetStEph<V>, AVLTreeSetStEph<Pair<V, P>>))
        requires
            forall|v: &V| #[trigger] graph.requires((v,)),
            forall|v: &V| #[trigger] priority_fn.requires((v,)),
    {
        let mut visited = visited_init;
        let mut frontier = frontier_init;

        while frontier.size() > 0
            invariant
                forall|v: &V| #[trigger] graph.requires((v,)),
                forall|v: &V| #[trigger] priority_fn.requires((v,)),
        {
            // Extract min-priority vertex (inline pq_find_min_priority).
            let seq = frontier.to_seq();
            assert(seq@.len() > 0) by {
                if seq@.len() == 0 {
                    assert(seq@.to_set() =~= Set::empty());
                }
            }
            let entry_ref = seq.nth(0);
            let v = entry_ref.1.clone();

            let p = priority_fn(&v);
            let entry = Pair(Pair(p.clone(), v.clone()), v.clone());
            let frontier_new = frontier.difference(&AVLTreeSetStEph::singleton(entry));

            let visited_new = visited.union(&AVLTreeSetStEph::singleton(v.clone()));

            let neighbors = graph(&v);
            let mut frontier_updated = frontier_new;
            let neighbors_seq = neighbors.to_seq();
            let nlen = neighbors_seq.length();
            let mut i: usize = 0;
            while i < nlen
                invariant
                    i <= nlen,
                    nlen == neighbors_seq@.len(),
                    neighbors_seq.spec_avltreeseqsteph_wf(),
                    forall|v: &V| #[trigger] priority_fn.requires((v,)),
                decreases nlen - i,
            {
                let neighbor = neighbors_seq.nth(i);
                if !visited_new.find(neighbor) {
                    let neighbor_p = priority_fn(neighbor);
                    let neighbor_entry = Pair(Pair(neighbor_p.clone(), neighbor.clone()), neighbor.clone());
                    frontier_updated = frontier_updated.union(&AVLTreeSetStEph::singleton(neighbor_entry));
                }
                i = i + 1;
            }

            visited = visited_new;
            frontier = frontier_updated;
        }

        // Build priorities from visited.
        let mut priorities = AVLTreeSetStEph::empty();
        let visited_seq = visited.to_seq();
        let vlen = visited_seq.length();
        let mut j: usize = 0;
        while j < vlen
            invariant
                j <= vlen,
                vlen == visited_seq@.len(),
                visited_seq.spec_avltreeseqsteph_wf(),
                forall|v: &V| #[trigger] priority_fn.requires((v,)),
            decreases vlen - j,
        {
            let vref = visited_seq.nth(j);
            let p = priority_fn(vref);
            priorities = priorities.union(&AVLTreeSetStEph::singleton(Pair(vref.clone(), p)));
            j = j + 1;
        }
        (visited, priorities)
    }

    /// Priority-first search from multiple sources (Section 53.4).
    #[verifier::external_body]
    pub fn pq_min_multi<V: StT + Ord, P: StT + Ord, G, PF>(
        graph: &G,
        sources: AVLTreeSetStEph<V>,
        priority_fn: &PF,
    ) -> (search: PQMinResult<V, P>)
    where
        G: Fn(&V) -> AVLTreeSetStEph<V>,
        PF: Fn(&V) -> P,
        ensures
            spec_pqminsteph_wf_generic(&search),
            sources@.subset_of(search.visited@),
    {
        let mut initial_frontier = AVLTreeSetStEph::empty();
        let sources_seq = sources.to_seq();
        let mut i: usize = 0;
        while i < sources_seq.length()
        {
            let v = sources_seq.nth(i);
            let p = priority_fn(v);
            let entry = Pair(Pair(p.clone(), v.clone()), v.clone());
            initial_frontier = initial_frontier.union(&AVLTreeSetStEph::singleton(entry));
            i = i + 1;
        }

        let (visited, priorities) = pq_explore(graph, priority_fn, AVLTreeSetStEph::empty(), initial_frontier);

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
