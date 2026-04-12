//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 65: Kruskal's MST Algorithm (Sequential Ephemeral)
//!
//! Implements Algorithm 65.2: Kruskal's algorithm for computing Minimum Spanning Trees.
//! Uses Union-Find with path compression for efficient cycle detection.
//!
//! Proof status:
//! - R196: rewired from old UnionFindStEph to UnionFindPCStEph (path compression).
//!   The old uf_opaque_wrappers nested module was deleted: PC's spec_wf is already
//!   bundled around opaque spec_light_wf (R195), so an outer wrapper is redundant.
//
//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions — struct KruskalStEph
//	Section 7. proof fns/broadcast groups — struct KruskalStEph
//	Section 8. traits — struct KruskalStEph
//	Section 9. impls — struct KruskalStEph

pub mod KruskalStEph {

	//		Section 2. imports

    use vstd::prelude::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap05::SetStEph::SetStEph::iter_invariant;
    use crate::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::*;
    use crate::Types::Types::*;
    use std::hash::Hash;
    use crate::Chap65::UnionFindPCStEph::UnionFindPCStEph::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{obeys_feq_full, obeys_feq_view_injective};
    use crate::vstdplus::pervasives_plus::pervasives_plus::vec_swap;
    use crate::vstdplus::clone_view::clone_view::ClonePreservesView;
    use crate::SetLit;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::obeys_key_model;

    pub type T<V> = LabUnDirGraphStEph<V, u64>;

    verus! {

	//		Section 3. broadcast use

    broadcast use {
        crate::Types::Types::group_LabEdge_axioms,
    };

	//		Section 4. type definitions — struct KruskalStEph

    /// Namespace struct for trait impl.
    pub struct KruskalStEph;

	//		Section 7. proof fns/broadcast groups — struct KruskalStEph

    /// Prove that a sorted edge's endpoints are in the UF domain.
    /// Chains: sort provenance -> pre_sort view -> edge_seq view -> mapped_es -> labeled -> graph@.A -> graph wf -> UF.
    /// Factored out to reduce rlimit pressure on the greedy loop.
    proof fn lemma_sorted_edge_in_graph_v<V: HashOrd>(
        edges_vec_i: LabEdge<V, u64>,
        pre_sort: Seq<LabEdge<V, u64>>,
        edge_seq: Seq<LabEdge<V, u64>>,
        mapped_es: Seq<(<V as View>::V, <V as View>::V, u64)>,
        labeled_view: Set<(<V as View>::V, <V as View>::V, u64)>,
        graph_V: Set<<V as View>::V>,
        graph_A: Set<(<V as View>::V, <V as View>::V, u64)>,
    )
        requires
            pre_sort.contains(edges_vec_i),
            pre_sort.len() == edge_seq.len(),
            forall|k: int| 0 <= k < pre_sort.len() ==> #[trigger] pre_sort[k]@ == edge_seq[k]@,
            mapped_es == edge_seq.map(|_i: int, e: LabEdge<V, u64>| e@),
            forall|x: (<V as View>::V, <V as View>::V, u64)|
                labeled_view.contains(x) <==> mapped_es.contains(x),
            labeled_view =~= graph_A,
            spec_labgraphview_wf(LabGraphView { V: graph_V, A: graph_A }),
        ensures
            graph_V.contains(edges_vec_i@.0),
            graph_V.contains(edges_vec_i@.1),
    {
        let j = choose|j: int| 0 <= j < pre_sort.len() && pre_sort[j] == edges_vec_i;
        assert(j < edge_seq.len());
        assert(pre_sort[j]@ == edge_seq[j]@);
        assert(edges_vec_i@ == edge_seq[j]@);
        assert(mapped_es[j] == edge_seq[j]@);
        assert(mapped_es.contains(edge_seq[j]@));
        assert(labeled_view.contains(edge_seq[j]@));
        assert(graph_A.contains(edge_seq[j]@));
        assert(graph_A.contains((edge_seq[j]@.0, edge_seq[j]@.1, edge_seq[j]@.2)));
        assert(graph_V.contains(edge_seq[j]@.0));
        assert(graph_V.contains(edge_seq[j]@.1));
    }

	//		Section 8. traits — struct KruskalStEph

    pub trait KruskalStEphTrait {
        /// Well-formedness for sequential Kruskal MST algorithm input.
        open spec fn spec_kruskalsteph_wf<V: HashOrd>(graph: &LabUnDirGraphStEph<V, u64>) -> bool {
            spec_labgraphview_wf(graph@)
        }

        /// Kruskal's MST algorithm.
        /// APAS: Work O(m log m), Span O(m log m) where m = |E|
        fn kruskal_mst<V: HashOrd>(
            graph: &LabUnDirGraphStEph<V, u64>,
        ) -> SetStEph<LabEdge<V, u64>>
            requires Self::spec_kruskalsteph_wf(graph);

        /// Compute total weight of MST.
        /// APAS: Work O(m), Span O(1)
        fn mst_weight<V: StT + Hash>(mst: &SetStEph<LabEdge<V, u64>>) -> (total: u64)
            requires mst.spec_setsteph_wf(),
            ensures mst@.len() == 0 ==> total == 0;

        /// Verify MST has correct size.
        /// APAS: Work O(1), Span O(1)
        fn verify_mst_size<V: HashOrd>(
            graph: &LabUnDirGraphStEph<V, u64>,
            mst: &SetStEph<LabEdge<V, u64>>,
        ) -> bool
            requires Self::spec_kruskalsteph_wf(graph), mst.spec_setsteph_wf();
    }

	//		Section 9. impls — struct KruskalStEph

    /// Process one edge: if endpoints are in different components, add to MST and union.
    pub fn kruskal_process_edge<V: HashOrd>(
        uf: &mut UnionFindPC<V>,
        mst_edges: &mut SetStEph<LabEdge<V, u64>>,
        edge: LabEdge<V, u64>,
    )
        requires
            old(uf).spec_wf(),
            old(mst_edges).spec_setsteph_wf(),
            old(uf).spec_contains(edge@.0),
            old(uf).spec_contains(edge@.1),
        ensures
            uf.spec_wf(),
            mst_edges.spec_setsteph_wf(),
            forall|z: <V as View>::V| old(uf).spec_contains(z) <==> uf.spec_contains(z),
    {
        let u = edge.0.clone_view();
        let v = edge.1.clone_view();
        if !uf.equals(&u, &v) {
            let _ = mst_edges.insert(edge);
            uf.union(&u, &v);
        }
    }

    /// Greedy edge-adding phase of Kruskal's algorithm.
    fn kruskal_greedy_phase<V: HashOrd>(
        uf: &mut UnionFindPC<V>,
        mst_edges: &mut SetStEph<LabEdge<V, u64>>,
        edges_vec: &Vec<LabEdge<V, u64>>,
        Ghost(pre_sort): Ghost<Seq<LabEdge<V, u64>>>,
        Ghost(edge_seq): Ghost<Seq<LabEdge<V, u64>>>,
        Ghost(mapped_es): Ghost<Seq<(<V as View>::V, <V as View>::V, u64)>>,
        Ghost(labeled_view): Ghost<Set<(<V as View>::V, <V as View>::V, u64)>>,
        Ghost(graph_V): Ghost<Set<<V as View>::V>>,
        Ghost(graph_A): Ghost<Set<(<V as View>::V, <V as View>::V, u64)>>,
    )
        requires
            old(uf).spec_wf(),
            old(mst_edges).spec_setsteph_wf(),
            forall|v: <V as View>::V| #[trigger] graph_V.contains(v) ==>
                old(uf).spec_contains(v),
            forall|k: int| 0 <= k < edges_vec@.len() ==>
                pre_sort.contains(#[trigger] edges_vec@[k]),
            pre_sort.len() == edge_seq.len(),
            forall|k: int| 0 <= k < pre_sort.len() ==>
                #[trigger] pre_sort[k]@ == edge_seq[k]@,
            mapped_es == edge_seq.map(|_i: int, e: LabEdge<V, u64>| e@),
            forall|x: (<V as View>::V, <V as View>::V, u64)|
                labeled_view.contains(x) <==> mapped_es.contains(x),
            labeled_view =~= graph_A,
            spec_labgraphview_wf(LabGraphView { V: graph_V, A: graph_A }),
        ensures
            mst_edges.spec_setsteph_wf(),
    {
        let mut i: usize = 0;
        while i < edges_vec.len()
            invariant
                0 <= i <= edges_vec@.len(),
                uf.spec_wf(),
                mst_edges.spec_setsteph_wf(),
                forall|v: <V as View>::V| #[trigger] graph_V.contains(v) ==>
                    uf.spec_contains(v),
                pre_sort.len() == edge_seq.len(),
                forall|k: int| 0 <= k < edges_vec@.len() ==>
                    pre_sort.contains(#[trigger] edges_vec@[k]),
                forall|k: int| 0 <= k < pre_sort.len() ==>
                    #[trigger] pre_sort[k]@ == edge_seq[k]@,
                mapped_es =~= edge_seq.map(|_i: int, e: LabEdge<V, u64>| e@),
                forall|x: (<V as View>::V, <V as View>::V, u64)|
                    labeled_view.contains(x) <==> mapped_es.contains(x),
                labeled_view =~= graph_A,
                spec_labgraphview_wf(LabGraphView { V: graph_V, A: graph_A }),
            decreases edges_vec@.len() - i,
        {
            let edge = edges_vec[i].clone_view();

            // Prove endpoints are in graph_V → loop invariant gives uf.spec_contains.
            proof {
                lemma_sorted_edge_in_graph_v::<V>(
                    edges_vec@[i as int], pre_sort, edge_seq, mapped_es,
                    labeled_view, graph_V, graph_A,
                );
            }

            kruskal_process_edge(uf, mst_edges, edge);
            i = i + 1;
        }
    }

    /// Sort edges by weight — selection sort.
    // veracity: no_requires
    fn sort_edges_by_weight<V: HashOrd>(edges: &mut Vec<LabEdge<V, u64>>)
        ensures
            edges@.len() == old(edges)@.len(),
            forall|i: int| 0 <= i < edges@.len() ==>
                old(edges)@.contains(#[trigger] edges@[i]),
            forall|i: int, j: int| #![trigger edges@[i], edges@[j]]
                0 <= i <= j < edges@.len() ==>
                edges@[i].2 <= edges@[j].2,
    {
        let n = edges.len();
        let mut i: usize = 0;
        while i < n
            invariant
                n == edges@.len(),
                i <= n,
                forall|k: int| 0 <= k < n ==>
                    old(edges)@.contains(#[trigger] edges@[k]),
                forall|a: int, b: int| #![trigger edges@[a], edges@[b]]
                    0 <= a <= b < i as int ==>
                    edges@[a].2 <= edges@[b].2,
                forall|a: int, b: int| #![trigger edges@[a], edges@[b]]
                    0 <= a < i as int && i as int <= b < n ==>
                    edges@[a].2 <= edges@[b].2,
            decreases n - i,
        {
            // Find index of minimum weight edge in [i..n).
            let mut min_idx: usize = i;
            let mut j: usize = i + 1;
            while j < n
                invariant
                    n == edges@.len(),
                    i <= min_idx < j <= n,
                    forall|k: int| i as int <= k < j as int ==>
                        edges@[min_idx as int].2 <= #[trigger] edges@[k].2,
                decreases n - j,
            {
                if edges[j].2 < edges[min_idx].2 {
                    min_idx = j;
                }
                j += 1;
            }

            // Swap minimum into position i.
            if min_idx != i {
                vec_swap(edges, i, min_idx);
            }

            // Prove outer invariant for i+1.
            proof {
                let new_i = i as int + 1;
                // Prefix [0..new_i) sorted.
                assert forall|a: int, b: int| #![trigger edges@[a], edges@[b]] 0 <= a <= b < new_i
                    implies edges@[a].2 <= edges@[b].2
                by {
                    if b == i as int && a < i as int {
                        assert(edges@[a].2 <= edges@[i as int].2);
                    }
                };
                // Prefix [0..new_i) ≤ suffix [new_i..n).
                assert forall|a: int, b: int| #![trigger edges@[a], edges@[b]] 0 <= a < new_i && new_i <= b < n
                    implies edges@[a].2 <= edges@[b].2
                by {
                    assert(edges@[i as int].2 <= edges@[b].2);
                    if a < i as int {
                        // Transitivity: a < i, so edges[a].2 <= edges[i].2 <= edges[b].2.
                    }
                };
            }

            i += 1;
        }
    }

    /// Algorithm 65.2: Kruskal's MST Algorithm.
    ///
    /// - Alg Analysis: APAS (Ch65 Alg 65.2): Work O(m lg n), Span O(m lg n)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m lg n), Span O(m lg n) — matches APAS
    /// - Claude-Opus-4.6: Work O(m lg m), Span O(m lg m) — sorting dominates.
    #[verifier::rlimit(50)]
    pub fn kruskal_mst<V: HashOrd>(
        graph: &LabUnDirGraphStEph<V, u64>,
    ) -> (mst_edges: SetStEph<LabEdge<V, u64>>)
        requires
            spec_labgraphview_wf(graph@),
            obeys_key_model::<V>(),
            obeys_feq_full::<V>(),
            obeys_feq_view_injective::<V>(),
        ensures
            mst_edges.spec_setsteph_wf(),
    {
        // Trigger LabEdge broadcast axioms for SetStEph::empty precondition.
        proof { assert(LabEdge_feq_trigger::<V, u64>()); }

        let mut mst_edges: SetStEph<LabEdge<V, u64>> = SetStEph::empty();
        let mut uf: UnionFindPC<V> = UnionFindPC::new();

        // Convert sets to Vecs for index-based iteration (avoids loop-break info loss).
        let vertex_seq = graph.vertices().to_seq();
        let labeled = graph.labeled_edges();
        let ghost labeled_view = labeled@;
        let edge_seq = labeled.to_seq();

        // Phase 1: Insert all vertices into union-find. PC's insert requires the
        // vertex not already be present — we discharge that from vertex_seq's
        // no_duplicates and view-injectivity.
        // Establish the empty-uf base case: spec_n == 0 (from new()) implies parent@.dom() is empty.
        proof {
            crate::vstdplus::feq::feq::lemma_reveal_view_injective::<V>();
            reveal(crate::Chap65::UnionFindPCStEph::UnionFindPCStEph::spec_light_wf);
            assert(uf.parent@.dom().finite());
            assert(uf.parent@.dom().len() == 0);
            uf.parent@.dom().lemma_len0_is_empty();
        }
        let mut vi: usize = 0;
        while vi < vertex_seq.len()
            invariant
                0 <= vi <= vertex_seq@.len(),
                uf.spec_wf(),
                vertex_seq@.no_duplicates(),
                obeys_feq_view_injective::<V>(),
                forall|j: int| 0 <= j < vi ==>
                    uf.spec_contains(#[trigger] vertex_seq@[j]@),
                forall|k: <V as View>::V| uf.spec_contains(k) ==>
                    exists|j: int| 0 <= j < vi && #[trigger] vertex_seq@[j]@ == k,
            decreases vertex_seq@.len() - vi,
        {
            // Prove !uf.spec_contains(vertex_seq[vi]@) — distinct views imply distinct V values.
            proof {
                crate::vstdplus::feq::feq::lemma_reveal_view_injective::<V>();
                if uf.spec_contains(vertex_seq@[vi as int]@) {
                    let j = choose|j: int| 0 <= j < vi
                        && #[trigger] vertex_seq@[j]@ == vertex_seq@[vi as int]@;
                    // View injectivity: equal views ==> equal V values.
                    assert(vertex_seq@[j] == vertex_seq@[vi as int]);
                    // no_duplicates says distinct indices have distinct values.
                    assert(j != vi as int);
                    assert(vertex_seq@.no_duplicates());
                    assert(vertex_seq@[j] != vertex_seq@[vi as int]);
                    assert(false);
                }
            }
            uf.insert(vertex_seq[vi].clone_view());
            vi += 1;
        }

        // Bridge: all graph vertices are now in UF.
        proof {
            let mapped_vs = vertex_seq@.map(|_i: int, t: V| t@);
            assert forall|v: <V as View>::V| #[trigger] graph@.V.contains(v) implies
                uf.spec_contains(v)
            by {
                assert(mapped_vs.contains(v));
                let j = choose|j: int| 0 <= j < mapped_vs.len() && mapped_vs[j] == v;
                assert(mapped_vs.len() == vertex_seq@.len());
                assert(mapped_vs[j] == vertex_seq@[j]@);
                assert(0 <= j && j < vertex_seq@.len());
                assert(uf.spec_contains(vertex_seq@[j]@));
            };
        }

        // Phase 2: Collect edges into Vec.
        let mut edges_vec: Vec<LabEdge<V, u64>> = Vec::new();
        let ghost mapped_es = edge_seq@.map(|_i: int, e: LabEdge<V, u64>| e@);
        let mut ei: usize = 0;
        while ei < edge_seq.len()
            invariant
                0 <= ei <= edge_seq@.len(),
                edges_vec@.len() == ei,
                forall|j: int| 0 <= j < ei ==>
                    #[trigger] edges_vec@[j]@ == edge_seq@[j]@,
                mapped_es == edge_seq@.map(|_i: int, e: LabEdge<V, u64>| e@),
                forall|x: (<V as View>::V, <V as View>::V, u64)|
                    labeled_view.contains(x) <==> mapped_es.contains(x),
                labeled_view =~= graph@.A,
            decreases edge_seq@.len() - ei,
        {
            edges_vec.push(edge_seq[ei].clone_view());
            ei += 1;
        }

        // Phase 3: Sort edges by weight.
        // After while: ei >= edge_seq.len(), so edges_vec@.len() == edge_seq@.len().
        let ghost pre_sort = edges_vec@;
        sort_edges_by_weight(&mut edges_vec);

        // Phase 4: Greedily add edges that don't form cycles.
        kruskal_greedy_phase(
            &mut uf, &mut mst_edges, &edges_vec,
            Ghost(pre_sort), Ghost(edge_seq@), Ghost(mapped_es),
            Ghost(labeled_view), Ghost(graph@.V), Ghost(graph@.A),
        );

        mst_edges
    }

    /// Compute total MST weight.
    /// - Alg Analysis: APAS: (no cost stated) — utility function
    /// - Alg Analysis: Code review (Claude Opus 4.6): matches APAS
    /// - Claude-Opus-4.6: Work O(|MST|), Span O(|MST|) — linear scan over MST edges
    /// Overflow-safe: skips edges that would cause u64 overflow (never triggers for MST weights).
    pub fn mst_weight<V: StT + Hash>(mst_edges: &SetStEph<LabEdge<V, u64>>) -> (total: u64)
        requires mst_edges.spec_setsteph_wf(),
        ensures mst_edges@.len() == 0 ==> total == 0,
    {
        if mst_edges.size() == 0 {
            return 0u64;
        }
        let mut total: u64 = 0;
        let mut it = mst_edges.iter();
        let ghost le_seq = it@.1;
        loop
            invariant
                it@.0 <= le_seq.len(),
                it@.1 == le_seq,
                mst_edges@.len() > 0,
            decreases le_seq.len() - it@.0,
        {
            match it.next() {
                None => return total,
                Some(edge) => {
                    if edge.2 <= u64::MAX - total {
                        total = total + edge.2;
                    }
                },
            }
        }
    }

    /// Verify MST has correct number of edges.
    /// A valid MST of n vertices should have n-1 edges.
    /// - Alg Analysis: APAS: (no cost stated) — validation utility
    /// - Alg Analysis: Code review (Claude Opus 4.6): matches APAS
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    pub fn verify_mst_size<V: HashOrd>(
        n_vertices: usize,
        mst_edges: &SetStEph<LabEdge<V, u64>>,
    ) -> (valid: bool)
        requires mst_edges.spec_setsteph_wf(),
        ensures valid == (mst_edges@.len() == (if n_vertices > 0 { (n_vertices - 1) as nat } else { 0nat })),
    {
        let expected_edges = if n_vertices > 0 { n_vertices - 1 } else { 0 };
        mst_edges.size() == expected_edges
    }

    } // verus!
}
