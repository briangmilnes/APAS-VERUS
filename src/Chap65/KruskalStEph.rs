//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 65: Kruskal's MST Algorithm (Sequential Ephemeral)
//!
//! Implements Algorithm 65.2: Kruskal's algorithm for computing Minimum Spanning Trees.
//! Uses Union-Find data structure for efficient cycle detection.

pub mod KruskalStEph {

    use vstd::prelude::*;
    use crate::vstdplus::float::float::{FloatTotalOrder, WrappedF64, zero_dist};
    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabUnDirGraphStEph::LabUnDirGraphStEph::*;
    use crate::Types::Types::*;
    use std::hash::Hash;
    use crate::Chap65::UnionFindStEph::UnionFindStEph::*;
    use crate::vstdplus::feq::feq::obeys_feq_full;
    use crate::SetLit;

    pub type T<V> = LabUnDirGraphStEph<V, WrappedF64>;

    verus! {

    // 4. type definitions

    /// Namespace struct for trait impl.
    pub struct KruskalStEph;

    // 8. traits

    pub trait KruskalStEphTrait {
        /// Well-formedness for sequential Kruskal MST algorithm input.
        open spec fn spec_kruskalsteph_wf<V: StT + Hash>(graph: &LabUnDirGraphStEph<V, WrappedF64>) -> bool {
            spec_labgraphview_wf(graph@)
        }

        /// Kruskal's MST algorithm.
        /// APAS: Work O(m log m), Span O(m log m) where m = |E|
        fn kruskal_mst<V: HashOrd>(
            graph: &LabUnDirGraphStEph<V, WrappedF64>,
        ) -> SetStEph<LabEdge<V, WrappedF64>>
            requires Self::spec_kruskalsteph_wf(graph);

        /// Compute total weight of MST.
        /// APAS: Work O(m), Span O(1)
        fn mst_weight<V: StT + Hash>(mst: &SetStEph<LabEdge<V, WrappedF64>>) -> WrappedF64
            requires mst.spec_setsteph_wf();

        /// Verify MST has correct size.
        /// APAS: Work O(1), Span O(1)
        fn verify_mst_size<V: HashOrd>(
            graph: &LabUnDirGraphStEph<V, WrappedF64>,
            mst: &SetStEph<LabEdge<V, WrappedF64>>,
        ) -> B
            requires Self::spec_kruskalsteph_wf(graph), mst.spec_setsteph_wf();
    }

    /// Sort edges by weight (external_body: closure-based sort not verifiable).
    #[verifier::external_body]
    fn sort_edges_by_weight<V: HashOrd>(edges: &mut Vec<LabEdge<V, WrappedF64>>)
        ensures
            edges@.len() == old(edges)@.len(),
            forall|i: int| 0 <= i < edges@.len() ==>
                old(edges)@.contains(#[trigger] edges@[i]),
            forall|i: int, j: int| #![trigger edges@[i], edges@[j]]
                0 <= i <= j < edges@.len() ==>
                edges@[i].2.val.le(edges@[j].2.val),
    {
        edges.sort_by(|e1, e2| {
            let LabEdge(_u1, _v1, w1) = e1;
            let LabEdge(_u2, _v2, w2) = e2;
            w1.cmp(w2)
        });
    }

    /// Algorithm 65.2: Kruskal's MST Algorithm.
    ///
    /// - APAS: Work O(m lg n), Span O(m lg n)
    /// - Claude-Opus-4.6: Work O(m lg m), Span O(m lg m) — sorting dominates.
    pub fn kruskal_mst<V: HashOrd>(
        graph: &LabUnDirGraphStEph<V, WrappedF64>,
    ) -> (mst_edges: SetStEph<LabEdge<V, WrappedF64>>)
        requires
            spec_labgraphview_wf(graph@),
            obeys_key_model::<V>(),
            obeys_feq_full::<V>(),
            forall|k1: V, k2: V| k1@ == k2@ ==> k1 == k2,
        ensures
            mst_edges.spec_setsteph_wf(),
    {
        let mut mst_edges: SetStEph<LabEdge<V, WrappedF64>> = SetStEph::empty();
        let mut uf = UnionFindStEph::new();

        // Insert all vertices into union-find.
        let vertices = graph.vertices();
        let mut vit = vertices.iter();
        let ghost vseq = vit@.1;

        loop
            invariant
                iter_invariant(&vit),
                vseq == vit@.1,
                uf.spec_unionfindsteph_wf(),
                forall|j: int| 0 <= j < vit@.0 ==>
                    #[trigger] uf@.parent.contains_key(vseq[j]@),
                vseq.map(|i: int, k: V| k@).to_set() == vertices@,
            decreases vseq.len() - vit@.0,
        {
            if let Some(v) = vit.next() {
                uf.insert(v.clone());
            } else {
                break;
            }
        }

        // All graph vertices now in UF.
        // vit@.0 == vseq.len(), so forall j < vseq.len(): uf has vseq[j]@.

        // Collect edges into Vec.
        let labeled = graph.labeled_edges();
        let mut edges_vec: Vec<LabEdge<V, WrappedF64>> = Vec::new();
        let mut eit = labeled.iter();
        let ghost eseq = eit@.1;

        loop
            invariant
                iter_invariant(&eit),
                eseq == eit@.1,
                edges_vec@.len() == eit@.0,
                forall|j: int| 0 <= j < edges_vec@.len() ==>
                    edges_vec@[j] == #[trigger] eseq[j],
            decreases eseq.len() - eit@.0,
        {
            if let Some(e) = eit.next() {
                edges_vec.push(*e);
            } else {
                break;
            }
        }

        // Sort edges by weight.
        let ghost pre_sort = edges_vec@;
        sort_edges_by_weight(&mut edges_vec);

        // Greedily add edges that don't form cycles.
        let mut i: usize = 0;
        while i < edges_vec.len()
            invariant
                0 <= i <= edges_vec@.len(),
                uf.spec_unionfindsteph_wf(),
                mst_edges.spec_setsteph_wf(),
                forall|v: <V as View>::V| #[trigger] graph@.V.contains(v) ==>
                    uf@.parent.contains_key(v),
                // Edge provenance: every element was in pre-sort, which came from graph.
                forall|k: int| 0 <= k < edges_vec@.len() ==>
                    pre_sort.contains(#[trigger] edges_vec@[k]),
                forall|k: int| 0 <= k < pre_sort.len() ==>
                    pre_sort[k] == #[trigger] eseq[k],
                eseq.map(|idx: int, e: LabEdge<V, WrappedF64>| e@).to_set() =~= graph@.A,
                spec_labgraphview_wf(graph@),
            decreases edges_vec@.len() - i,
        {
            let edge = edges_vec[i];
            let LabEdge(u, v, _w) = edge;

            // Prove edge endpoints are in UF via graph wf.
            // edge was in pre_sort, which equals eseq, whose views map to graph@.A.
            proof {
                // edges_vec@[i as int] is in pre_sort.
                assert(pre_sort.contains(edges_vec@[i as int]));
                // pre_sort elements equal eseq elements.
                let j = choose|j: int| 0 <= j < pre_sort.len() && pre_sort[j] == edges_vec@[i as int];
                assert(eseq[j] == edges_vec@[i as int]);
                // eseq[j]@ is in graph@.A.
                assert(eseq.map(|idx: int, e: LabEdge<V, WrappedF64>| e@).to_set().contains(eseq[j]@));
                assert(graph@.A.contains(eseq[j]@));
                // Graph wf: endpoints are vertices.
                assert(graph@.V.contains(u@));
                assert(graph@.V.contains(v@));
                // Invariant: vertices are in UF.
                assert(uf@.parent.contains_key(u@));
                assert(uf@.parent.contains_key(v@));
            }

            if !uf.equals(&u, &v) {
                let _ = mst_edges.insert(edge);
                uf.union(&u, &v);
            }
            i = i + 1;
        }

        mst_edges
    }

    /// Compute total MST weight.
    /// - APAS: (no cost stated) — utility function
    /// - Claude-Opus-4.6: Work O(|MST|), Span O(|MST|) — linear scan over MST edges
    pub fn mst_weight<V: StT + Hash>(mst_edges: &SetStEph<LabEdge<V, WrappedF64>>) -> (total: WrappedF64)
        requires mst_edges.spec_setsteph_wf(),
        ensures mst_edges@.len() == 0 ==> total@ == 0.0f64,
    {
        if mst_edges.size() == 0 {
            return WrappedF64 { val: 0.0 };
        }
        let mut total = WrappedF64 { val: 0.0 };
        let mut it = mst_edges.iter();
        let ghost le_seq = it@.1;
        loop
            invariant
                it@.0 <= le_seq.len(),
                it@.1 == le_seq,
            decreases le_seq.len() - it@.0,
        {
            match it.next() {
                None => return total,
                Some(edge) => {
                    total = WrappedF64 { val: total.val + edge.2.val };
                },
            }
        }
    }

    /// Verify MST has correct number of edges.
    /// A valid MST of n vertices should have n-1 edges.
    /// - APAS: (no cost stated) — validation utility
    /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1)
    pub fn verify_mst_size<V: HashOrd>(
        n_vertices: N,
        mst_edges: &SetStEph<LabEdge<V, WrappedF64>>,
    ) -> (result: B)
        requires mst_edges.spec_setsteph_wf(),
        ensures result == (mst_edges@.len() == (if n_vertices > 0 { (n_vertices - 1) as nat } else { 0nat })),
    {
        let expected_edges = if n_vertices > 0 { n_vertices - 1 } else { 0 };
        mst_edges.size() == expected_edges
    }

    } // verus!
}
