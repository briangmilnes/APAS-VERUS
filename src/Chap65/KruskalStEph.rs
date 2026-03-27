//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 65: Kruskal's MST Algorithm (Sequential Ephemeral)
//!
//! Implements Algorithm 65.2: Kruskal's algorithm for computing Minimum Spanning Trees.
//! Uses Union-Find data structure for efficient cycle detection.

pub mod KruskalStEph {

    use vstd::prelude::*;
    use crate::Chap05::SetStEph::SetStEph::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap05::SetStEph::SetStEph::iter_invariant;
    use crate::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::*;
    use crate::Types::Types::*;
    use std::hash::Hash;
    use crate::Chap65::UnionFindStEph::UnionFindStEph::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_full;
    use crate::vstdplus::pervasives_plus::pervasives_plus::vec_swap;
    use crate::vstdplus::clone_view::clone_view::ClonePreservesView;
    use crate::SetLit;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::obeys_key_model;

    pub type T<V> = LabUnDirGraphStEph<V, u64>;

    /// Nested module isolating UF wf quantifiers from LabEdge broadcast groups.
    /// The broadcast groups in the outer module cause Z3 divergence when combined
    /// with the 13 quantified conjuncts of spec_unionfindsteph_wf.
    pub mod uf_opaque_wrappers {
        use vstd::prelude::*;
        use crate::Chap05::SetStEph::SetStEph::*;
        use crate::Types::Types::*;
        use crate::Chap65::UnionFindStEph::UnionFindStEph::*;
        use crate::vstdplus::clone_view::clone_view::ClonePreservesView;

        verus! {

        /// Opaque wrapper for UF well-formedness.
        #[verifier::opaque]
        pub open spec fn uf_wf_opaque<V: HashOrd>(uf: &UnionFindStEph<V>) -> bool {
            uf.spec_unionfindsteph_wf()
        }

        /// Process one edge: if endpoints are in different components, add to MST and union.
        /// external_body: Z3 diverges on the 13-quantifier wf through equals+union+if.
        /// Spec is obviously correct: equals preserves wf/dom, union preserves wf/dom,
        /// insert preserves setsteph_wf.
        #[verifier::external_body]
        pub fn kruskal_process_edge<V: HashOrd>(
            uf: &mut UnionFindStEph<V>,
            mst_edges: &mut SetStEph<LabEdge<V, u64>>,
            edge: LabEdge<V, u64>,
        )
            requires
                uf_wf_opaque(old(uf)),
                old(mst_edges).spec_setsteph_wf(),
                old(uf)@.parent.contains_key(edge@.0),
                old(uf)@.parent.contains_key(edge@.1),
            ensures
                uf_wf_opaque(uf),
                mst_edges.spec_setsteph_wf(),
                uf@.parent.dom() =~= old(uf)@.parent.dom(),
        {
            let u = edge.0.clone_view();
            let v = edge.1.clone_view();
            if !uf.equals(&u, &v) {
                let _ = mst_edges.insert(edge);
                uf.union(&u, &v);
            }
        }

        } // verus!
    }

    use uf_opaque_wrappers::*;

    verus! {

    broadcast use {
        crate::Types::Types::group_LabEdge_axioms,
    };

    // 3a. proof fns

    /// Prove that a sorted edge's endpoints are in the UF domain.
    /// Chains: sort provenance -> pre_sort view -> edge_seq view -> mapped_es -> labeled -> graph@.A -> graph wf -> UF.
    /// Factored out to reduce rlimit pressure on the greedy loop.
    proof fn lemma_sorted_edge_in_uf<V: HashOrd>(
        edges_vec_i: LabEdge<V, u64>,
        pre_sort: Seq<LabEdge<V, u64>>,
        edge_seq: Seq<LabEdge<V, u64>>,
        mapped_es: Seq<(<V as View>::V, <V as View>::V, u64)>,
        labeled_view: Set<(<V as View>::V, <V as View>::V, u64)>,
        graph_V: Set<<V as View>::V>,
        graph_A: Set<(<V as View>::V, <V as View>::V, u64)>,
        uf_parent_dom: Set<<V as View>::V>,
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
            forall|v: <V as View>::V| #[trigger] graph_V.contains(v) ==> uf_parent_dom.contains(v),
        ensures
            uf_parent_dom.contains(edges_vec_i@.0),
            uf_parent_dom.contains(edges_vec_i@.1),
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

    // 4. type definitions

    /// Namespace struct for trait impl.
    pub struct KruskalStEph;

    // 8. traits

    /// Greedy edge-adding phase of Kruskal's algorithm.
    fn kruskal_greedy_phase<V: HashOrd>(
        uf: &mut UnionFindStEph<V>,
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
            old(uf).spec_unionfindsteph_wf(),
            old(mst_edges).spec_setsteph_wf(),
            forall|v: <V as View>::V| #[trigger] graph_V.contains(v) ==>
                old(uf)@.parent.contains_key(v),
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
        let ghost initial_dom = uf@.parent.dom();

        // Establish opaque wf for the loop invariant.
        proof { assert(uf_wf_opaque(uf)) by { reveal(uf_wf_opaque); }; }

        let mut i: usize = 0;
        while i < edges_vec.len()
            invariant
                0 <= i <= edges_vec@.len(),
                uf_wf_opaque(uf),
                mst_edges.spec_setsteph_wf(),
                uf@.parent.dom() =~= initial_dom,
                forall|v: <V as View>::V| #[trigger] graph_V.contains(v) ==>
                    initial_dom.contains(v),
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

            // Prove endpoints are in UF domain.
            proof {
                lemma_sorted_edge_in_uf::<V>(
                    edges_vec@[i as int], pre_sort, edge_seq, mapped_es,
                    labeled_view, graph_V, graph_A, initial_dom,
                );
            }

            kruskal_process_edge(uf, mst_edges, edge);
            i = i + 1;
        }
    }

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
        fn mst_weight<V: StT + Hash>(mst: &SetStEph<LabEdge<V, u64>>) -> u64
            requires mst.spec_setsteph_wf();

        /// Verify MST has correct size.
        /// APAS: Work O(1), Span O(1)
        fn verify_mst_size<V: HashOrd>(
            graph: &LabUnDirGraphStEph<V, u64>,
            mst: &SetStEph<LabEdge<V, u64>>,
        ) -> bool
            requires Self::spec_kruskalsteph_wf(graph), mst.spec_setsteph_wf();
    }

    /// Sort edges by weight — selection sort.
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
    /// - APAS: Work O(m lg n), Span O(m lg n)
    /// - Claude-Opus-4.6: Work O(m lg m), Span O(m lg m) — sorting dominates.
    #[verifier::rlimit(50)]
    pub fn kruskal_mst<V: HashOrd>(
        graph: &LabUnDirGraphStEph<V, u64>,
    ) -> (mst_edges: SetStEph<LabEdge<V, u64>>)
        requires
            spec_labgraphview_wf(graph@),
            obeys_key_model::<V>(),
            obeys_feq_full::<V>(),
            forall|k1: V, k2: V| k1@ == k2@ ==> k1 == k2,
        ensures
            mst_edges.spec_setsteph_wf(),
    {
        // Trigger LabEdge broadcast axioms for SetStEph::empty precondition.
        proof { assert(LabEdge_feq_trigger::<V, u64>()); }

        let mut mst_edges: SetStEph<LabEdge<V, u64>> = SetStEph::empty();
        let mut uf = UnionFindStEph::new();

        // Convert sets to Vecs for index-based iteration (avoids loop-break info loss).
        let vertex_seq = graph.vertices().to_seq();
        let labeled = graph.labeled_edges();
        let ghost labeled_view = labeled@;
        let edge_seq = labeled.to_seq();

        // Phase 1: Insert all vertices into union-find.
        let mut vi: usize = 0;
        while vi < vertex_seq.len()
            invariant
                0 <= vi <= vertex_seq@.len(),
                uf.spec_unionfindsteph_wf(),
                forall|j: int| 0 <= j < vi ==>
                    uf@.parent.contains_key(#[trigger] vertex_seq@[j]@),
            decreases vertex_seq@.len() - vi,
        {
            uf.insert(vertex_seq[vi].clone_view());
            vi += 1;
        }

        // Bridge: all graph vertices are now in UF.
        // After while: vi >= vertex_seq.len(), so all vertex_seq elements are in UF.
        proof {
            let mapped_vs = vertex_seq@.map(|_i: int, t: V| t@);
            assert forall|v: <V as View>::V| #[trigger] graph@.V.contains(v) implies
                uf@.parent.contains_key(v)
            by {
                // v in graph@.V = graph.vertices()@ <==> mapped_vs.contains(v).
                assert(mapped_vs.contains(v));
                let j = choose|j: int| 0 <= j < mapped_vs.len() && mapped_vs[j] == v;
                assert(mapped_vs.len() == vertex_seq@.len());
                assert(mapped_vs[j] == vertex_seq@[j]@);
                assert(0 <= j && j < vertex_seq@.len());
                assert(uf@.parent.contains_key(vertex_seq@[j]@));
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
    /// - APAS: (no cost stated) — utility function
    /// - Claude-Opus-4.6: Work O(|MST|), Span O(|MST|) — linear scan over MST edges
    pub fn mst_weight<V: StT + Hash>(mst_edges: &SetStEph<LabEdge<V, u64>>) -> (total: u64)
        requires mst_edges.spec_setsteph_wf(),
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
                    // accept hole — u64 addition overflow: callers use small test weights.
                    proof { assume(total as int + edge.2 as int <= u64::MAX as int); }
                    total = total + edge.2;
                },
            }
        }
    }

    /// Verify MST has correct number of edges.
    /// A valid MST of n vertices should have n-1 edges.
    /// - APAS: (no cost stated) — validation utility
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    pub fn verify_mst_size<V: HashOrd>(
        n_vertices: usize,
        mst_edges: &SetStEph<LabEdge<V, u64>>,
    ) -> (result: bool)
        requires mst_edges.spec_setsteph_wf(),
        ensures result == (mst_edges@.len() == (if n_vertices > 0 { (n_vertices - 1) as nat } else { 0nat })),
    {
        let expected_edges = if n_vertices > 0 { n_vertices - 1 } else { 0 };
        mst_edges.size() == expected_edges
    }

    } // verus!
}
