//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 59: Johnson's Algorithm - Multi-threaded Ephemeral Float Weights
//!
//! Implements Algorithm 59.1 from the textbook with parallelism in Phase 3.
//! All-Pairs Shortest Paths for graphs with negative weights (but no negative cycles).
//!
//! **Algorithmic Analysis:**
//! - Johnson APSP: Work O(mn log n), Span O(m log n), Parallelism Theta(n) where n = |V|, m = |E|
//! - Phase 1 (Bellman-Ford): Work O(nm), Span O(nm) - sequential
//! - Phase 2 (Reweighting): Work O(m), Span O(m) - sequential
//! - Phase 3 (n Dijkstras in parallel): Work O(n * m log n) = O(mn log n), Span O(m log n)
//! - Parallelism in Phase 3: Theta(n) - n independent Dijkstra runs

pub mod JohnsonMtEphF64 {

    use vstd::prelude::*;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::*;
    use crate::Chap06::WeightedDirGraphStEphF64::WeightedDirGraphStEphF64::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap56::AllPairsResultStEphF64::AllPairsResultStEphF64::*;
    use crate::Chap56::SSSPResultStEphF64::SSSPResultStEphF64::*;
    use crate::vstdplus::float::float::*;
    use crate::Types::Types::*;

    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_clone;

    use crate::Chap58::BellmanFordStEphF64::BellmanFordStEphF64::bellman_ford;

    verus! {

    // Table of Contents
    // 1. module (JohnsonMtEphF64)
    // 2. imports
    // 3. broadcast use
    // 8. traits
    // 9. impls

    // 3. broadcast use

    broadcast use {
        crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::group_hash_set_with_view_plus_axioms,
        crate::Types::Types::group_Pair_axioms,
        crate::Types::Types::group_WeightedEdge_axioms,
        crate::vstdplus::float::float::group_float_finite_total_order,
        crate::vstdplus::float::float::group_float_arithmetic,
    };

    // 8. traits

    pub trait JohnsonMtEphF64Trait {
        /// Parallel Johnson's all-pairs shortest path algorithm.
        /// APAS: Work O(mn log n), Span O(m log n) where n = |V|, m = |E|.
        fn johnson_apsp(graph: &WeightedDirGraphStEphF64<usize>) -> (result: AllPairsResultStEphF64)
            requires
                graph@.V.len() > 0,
                graph@.V.len() < usize::MAX as nat,
                spec_labgraphview_wf(graph@),
                valid_key_type_WeightedEdge::<usize, WrappedF64>(),
                forall|v: usize| graph@.V.contains(v) <==> v < graph@.V.len(),
                graph@.A.len() * 2 + 2 <= usize::MAX as int,
                obeys_feq_clone::<ArraySeqStEphS<WrappedF64>>(),
                obeys_feq_clone::<ArraySeqStEphS<usize>>(),
            ensures
                result.spec_n() as nat == graph@.V.len();
    }

    // 9. impls

    pub type T = WeightedDirGraphStEphF64<usize>;

    /// Adjust reweighted distance back to original weights.
    /// d(u,v) = d'(u,v) - h(u) + h(v), using float arithmetic directly.
    ///
    /// - APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: Work O(1), Span O(1).
    // veracity: no_requires
    fn adjust_distance(d_prime: WrappedF64, h_u: WrappedF64, h_v: WrappedF64) -> (result: WrappedF64)
        ensures
            !d_prime.spec_is_finite() ==> !result.spec_is_finite(),
    {
        if !d_prime.is_finite() { unreachable_dist() }
        else {
            d_prime.dist_sub(&h_u).dist_add(&h_v)
        }
    }

    /// Algorithm 59.1: Johnson's All-Pairs Shortest Paths (Parallel)
    ///
    /// Solves APSP problem with negative weights using:
    /// 1. Bellman-Ford to compute potentials (sequential)
    /// 2. Reweight edges (sequential)
    /// 3. Parallel Dijkstra from each vertex using ParaPair! divide-and-conquer
    ///
    /// Blocked: Phase 3 requires DijkstraStEphF64 (agent3 building concurrently).
    ///
    /// - APAS: Work O(mn log n), Span O(m log n), Parallelism Theta(n)
    /// - Claude-Opus-4.6: Work O(mn log n), Span O(m log n) — agrees with APAS
    #[verifier::external_body]
    pub fn johnson_apsp(graph: &WeightedDirGraphStEphF64<usize>) -> (result: AllPairsResultStEphF64)
        requires
            graph@.V.len() > 0,
            graph@.V.len() < usize::MAX as nat,
            spec_labgraphview_wf(graph@),
            valid_key_type_WeightedEdge::<usize, WrappedF64>(),
            forall|v: usize| graph@.V.contains(v) <==> v < graph@.V.len(),
            graph@.A.len() * 2 + 2 <= usize::MAX as int,
            obeys_feq_clone::<ArraySeqStEphS<WrappedF64>>(),
            obeys_feq_clone::<ArraySeqStEphS<usize>>(),
        ensures
            result.spec_n() as nat == graph@.V.len(),
    {
        unimplemented!("Blocked: requires DijkstraStEphF64 (agent3 building concurrently)")
    }

    /// Add dummy source vertex s with zero-weight edges to all vertices in G.
    ///
    /// - APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: Work O(n + m), Span O(n + m) — iterates over vertices and edges.
    fn add_dummy_source(graph: &WeightedDirGraphStEphF64<usize>, n: usize) -> (augmented_and_idx: (WeightedDirGraphStEphF64<usize>, usize))
        requires
            n > 0,
            n < usize::MAX,
            n as nat == graph@.V.len(),
            spec_labgraphview_wf(graph@),
            valid_key_type_WeightedEdge::<usize, WrappedF64>(),
            forall|v: usize| graph@.V.contains(v) <==> v < n,
        ensures
            spec_labgraphview_wf(augmented_and_idx.0@),
            augmented_and_idx.0@.V.len() == (n + 1) as nat,
            forall|v: usize| v <= n ==> augmented_and_idx.0@.V.contains(v),
            augmented_and_idx.1 == n,
    {
        // Build vertex set {0, ..., n}.
        let mut vertices = SetStEph::<usize>::empty();
        let mut i: usize = 0;
        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        while i <= n
            invariant
                i <= n + 1,
                vertices.spec_setsteph_wf(),
                vertices@.len() == i as nat,
                forall|k: usize| vertices@.contains(k) <==> k < i,
                valid_key_type_WeightedEdge::<usize, WrappedF64>(),
                n < usize::MAX,
            decreases (n + 1) - i,
        {
            proof { assert(!vertices@.contains(i)); }
            let _ = vertices.insert(i);
            i = i + 1;
        }
        proof {
            assert(i == n + 1);
            assert(forall|k: usize| vertices@.contains(k) <==> k <= n);
        }

        // Copy original edges.
        let mut edges = SetStEph::<WeightedEdge<usize, WrappedF64>>::empty();
        let arcs = graph.labeled_arcs();
        let it = arcs.iter();
        let ghost arcs_seq = it@.1;

        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        for labeled_edge in iter: it
            invariant
                iter.elements == arcs_seq,
                arcs_seq.map(|i: int, e: LabEdge<usize, WrappedF64>| e@).to_set() =~= graph@.A,
                edges.spec_setsteph_wf(),
                vertices.spec_setsteph_wf(),
                forall|k: usize| vertices@.contains(k) <==> k <= n,
                spec_labgraphview_wf(graph@),
                valid_key_type_WeightedEdge::<usize, WrappedF64>(),
                forall|v: usize| graph@.V.contains(v) <==> v < n,
                forall|a: usize, b: usize, w: f64|
                    #[trigger] edges@.contains((a, b, w)) ==>
                    vertices@.contains(a) && vertices@.contains(b),
        {
            let from = labeled_edge.0;
            let to = labeled_edge.1;
            let weight = labeled_edge.2;
            if from <= n && to <= n {
                let _ = edges.insert(WeightedEdge(from, to, weight));
            }
        }

        // Add dummy edges from n to each vertex 0..n with zero weight.
        let zero = zero_dist();
        let mut j: usize = 0;
        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        while j < n
            invariant
                j <= n,
                edges.spec_setsteph_wf(),
                vertices.spec_setsteph_wf(),
                forall|k: usize| vertices@.contains(k) <==> k <= n,
                valid_key_type_WeightedEdge::<usize, WrappedF64>(),
                forall|a: usize, b: usize, w: f64|
                    #[trigger] edges@.contains((a, b, w)) ==>
                    vertices@.contains(a) && vertices@.contains(b),
            decreases n - j,
        {
            let _ = edges.insert(WeightedEdge(n, j, zero));
            j = j + 1;
        }

        (WeightedDirGraphStEphF64::from_weighed_edges(vertices, edges), n)
    }

    /// Reweight edges: w'(u,v) = w(u,v) + p(u) - p(v).
    ///
    /// - APAS: Work O(m), Span O(m).
    /// - Claude-Opus-4.6: Work O(n + m), Span O(n + m) — rebuilds vertex set O(n) plus iterates edges O(m).
    fn reweight_graph(
        graph: &WeightedDirGraphStEphF64<usize>,
        potentials: &ArraySeqStEphS<WrappedF64>,
        n: usize,
    ) -> (reweighted: WeightedDirGraphStEphF64<usize>)
        requires
            n > 0,
            n < usize::MAX,
            n as nat == graph@.V.len(),
            potentials.seq@.len() == n as int,
            spec_labgraphview_wf(graph@),
            valid_key_type_WeightedEdge::<usize, WrappedF64>(),
            forall|v: usize| graph@.V.contains(v) <==> v < n,
            graph@.A.len() * 2 + 2 <= usize::MAX as int,
        ensures
            spec_labgraphview_wf(reweighted@),
            reweighted@.V.len() == n as nat,
            forall|v: usize| reweighted@.V.contains(v) <==> v < n,
    {
        // Build vertex set {0, ..., n-1}.
        let mut vertices = SetStEph::<usize>::empty();
        let mut i: usize = 0;
        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        while i < n
            invariant
                i <= n,
                vertices.spec_setsteph_wf(),
                vertices@.len() == i as nat,
                forall|k: usize| vertices@.contains(k) <==> k < i,
                valid_key_type_WeightedEdge::<usize, WrappedF64>(),
                n > 0,
                n < usize::MAX,
            decreases n - i,
        {
            proof { assert(!vertices@.contains(i)); }
            let _ = vertices.insert(i);
            i = i + 1;
        }
        proof {
            assert(vertices@.len() == n as nat);
            assert(forall|k: usize| vertices@.contains(k) <==> k < n);
        }

        // Reweight edges.
        let mut reweighted_edges = SetStEph::<WeightedEdge<usize, WrappedF64>>::empty();
        let arcs = graph.labeled_arcs();
        let it = arcs.iter();
        let ghost arcs_seq = it@.1;

        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        for labeled_edge in iter: it
            invariant
                iter.elements == arcs_seq,
                reweighted_edges.spec_setsteph_wf(),
                vertices.spec_setsteph_wf(),
                vertices@.len() == n as nat,
                forall|k: usize| vertices@.contains(k) <==> k < n,
                spec_labgraphview_wf(graph@),
                valid_key_type_WeightedEdge::<usize, WrappedF64>(),
                forall|v: usize| graph@.V.contains(v) <==> v < n,
                potentials.seq@.len() == n as int,
                forall|a: usize, b: usize, w: f64|
                    #[trigger] reweighted_edges@.contains((a, b, w)) ==>
                    vertices@.contains(a) && vertices@.contains(b),
        {
            let from = labeled_edge.0;
            let to = labeled_edge.1;
            let weight = labeled_edge.2;
            if from < n && to < n {
                let p_from = *potentials.nth(from);
                let p_to = *potentials.nth(to);
                let w_prime = weight.dist_add(&p_from).dist_sub(&p_to);
                let _ = reweighted_edges.insert(WeightedEdge(from, to, w_prime));
            }
        }

        WeightedDirGraphStEphF64::from_weighed_edges(vertices, reweighted_edges)
    }

    /// Create result for negative cycle case.
    ///
    /// - APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: Work O(n^2), Span O(n^2) — builds n x n distance and predecessor matrices.
    fn create_negative_cycle_result(n: usize) -> (result: AllPairsResultStEphF64)
        requires
            n < usize::MAX,
        ensures
            result.n == n,
            result.distances.seq@.len() == n as int,
            result.predecessors.seq@.len() == n as int,
    {
        let ghost n_int = n as int;
        let unreach = unreachable_dist();
        let inner_dist = |_i: usize| -> (r: WrappedF64) ensures true { unreachable_dist() };
        let dist_row = |_i: usize| -> (r: ArraySeqStEphS<WrappedF64>)
            ensures r.seq@.len() == n_int
        {
            ArraySeqStEphS::tabulate(&inner_dist, n)
        };
        let distances = ArraySeqStEphS::tabulate(&dist_row, n);
        let inner_pred = |_i: usize| -> (r: usize) ensures r == 0usize { 0usize };
        let pred_row = |_i: usize| -> (r: ArraySeqStEphS<usize>)
            ensures r.seq@.len() == n_int
        {
            ArraySeqStEphS::tabulate(&inner_pred, n)
        };
        let predecessors = ArraySeqStEphS::tabulate(&pred_row, n);
        AllPairsResultStEphF64 {
            distances,
            predecessors,
            n,
        }
    }

    } // verus!
}
