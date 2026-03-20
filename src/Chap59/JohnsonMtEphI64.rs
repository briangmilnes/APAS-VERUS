//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 59: Johnson's Algorithm - Multi-threaded Ephemeral Integer Weights
//!
//! Implements Algorithm 59.1 from the textbook with parallelism in Phase 3.
//! All-Pairs Shortest Paths for graphs with negative weights (but no negative cycles).
//!
//! **Algorithmic Analysis:**
//! - Johnson APSP: Work O(mn log n), Span O(m log n), Parallelism Θ(n) where n = |V|, m = |E|
//! - Phase 1 (Bellman-Ford): Work O(nm), Span O(nm) - sequential
//! - Phase 2 (Reweighting): Work O(m), Span O(m) - sequential
//! - Phase 3 (n Dijkstras in parallel): Work O(n * m log n) = O(mn log n), Span O(m log n)
//! - Parallelism in Phase 3: Θ(n) - n independent Dijkstra runs

pub mod JohnsonMtEphI64 {

    use vstd::prelude::*;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::*;
    use crate::Chap06::WeightedDirGraphStEphI128::WeightedDirGraphStEphI128::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap56::AllPairsResultStEphI64::AllPairsResultStEphI64::*;
    use crate::Chap56::SSSPResultStEphI64::SSSPResultStEphI64::*;
    use crate::Types::Types::*;

    use crate::Chap57::DijkstraStEphU64::DijkstraStEphU64::dijkstra;
    use crate::Chap58::BellmanFordStEphI64::BellmanFordStEphI64::bellman_ford;

    verus! {
        pub trait JohnsonMtEphI64Trait {
            /// Parallel Johnson's all-pairs shortest path algorithm
            /// APAS: Work O(mn log n), Span O(m log n) where n = |V|, m = |E|
            fn johnson_apsp(graph: &WeightedDirGraphStEphI128<usize>) -> AllPairsResultStEphI64;
        }
    pub type T = WeightedDirGraphStEphI128<usize>;

    /// Algorithm 59.1: Johnson's All-Pairs Shortest Paths (Parallel)
    ///
    /// Solves APSP problem with negative weights using:
    /// 1. Bellman-Ford to compute potentials (sequential)
    /// 2. Reweight edges (sequential)
    /// 3. Parallel Dijkstra from each vertex using ParaPair! divide-and-conquer
    ///
    /// - APAS: Work O(mn log n), Span O(m log n), Parallelism Θ(n)
    /// - Claude-Opus-4.6: Work O(mn log n), Span O(m log n) — agrees with APAS; ParaPair! recursion achieves Θ(n) parallelism in Phase 3
    pub fn johnson_apsp(graph: &WeightedDirGraphStEphI128<usize>) -> (result: AllPairsResultStEphI64)
        requires
            graph@.V.len() > 0,
            graph@.V.len() < usize::MAX as nat,
            spec_labgraphview_wf(graph@),
            valid_key_type_WeightedEdge::<usize, i128>(),
            forall|v: usize| graph@.V.contains(v) <==> v < graph@.V.len(),
        ensures
            result.spec_n() as nat == graph@.V.len(),
    {
        let n = graph.vertices().size();
        assert(n as nat == graph@.V.len());
        assert(n > 0);
        assert(n < usize::MAX);

        let (graph_with_dummy, dummy_idx) = add_dummy_source(graph, n);

        let bellman_ford_result = match bellman_ford(&graph_with_dummy, dummy_idx) {
            Ok(res) => res,
            Err(_) => {
                return create_negative_cycle_result(n);
            }
        };

        let get_dist = |i: usize| -> (d: i64) { bellman_ford_result.get_distance(i) };
        let potentials = ArraySeqStEphS::tabulate(&get_dist, n);

        let reweighted_graph = reweight_graph(graph, &potentials, n);

        let (all_distances, all_predecessors) = parallel_dijkstra_all(&reweighted_graph, &potentials, 0, n, n);

        AllPairsResultStEphI64 {
            distances: all_distances,
            predecessors: all_predecessors,
            n,
        }
    }

    /// Parallel Dijkstra execution using recursive divide-and-conquer with ParaPair!
    ///
    /// - APAS: N/A — internal helper, not named in prose.
    /// - Claude-Opus-4.6: Work O(k * m log n), Span O(m log n) where k = end - start — binary split with ParaPair! gives log k depth, each leaf runs Dijkstra O(m log n)
    #[verifier::external_body]
    fn parallel_dijkstra_all(
        graph: &WeightedDirGraphStEphI128<usize>,
        potentials: &ArraySeqStEphS<i64>,
        start: usize,
        end: usize,
        n: usize,
    ) -> (
        ArraySeqStEphS<ArraySeqStEphS<i64>>,
        ArraySeqStEphS<ArraySeqStEphS<usize>>,
    ) {
        let range_size = end - start;

        if range_size == 0 {
            return (ArraySeqStEphS::empty(), ArraySeqStEphS::empty());
        }

        if range_size == 1 {
            let u = start;
            let dijkstra_result = dijkstra(graph, u);

            let p_u = *potentials.nth(u);
            let adjusted_row = ArraySeqStEphS::tabulate(
                &|v| {
                    let d_prime = dijkstra_result.get_distance(v);
                    if d_prime == i64::MAX {
                        i64::MAX
                    } else {
                        let p_v = *potentials.nth(v);
                        d_prime - p_u + p_v
                    }
                },
                n,
            );

            let dist_seq = ArraySeqStEphS::singleton(adjusted_row);
            let pred_seq = ArraySeqStEphS::singleton(dijkstra_result.predecessors.clone());
            return (dist_seq, pred_seq);
        }

        let mid = start + range_size / 2;
        let graph_left = graph.clone();
        let graph_right = graph.clone();
        let potentials_left = potentials.clone();
        let potentials_right = potentials.clone();

        let Pair((left_dist, left_pred), (right_dist, right_pred)) = crate::ParaPair!(
            move || parallel_dijkstra_all(&graph_left, &potentials_left, start, mid, n),
            move || parallel_dijkstra_all(&graph_right, &potentials_right, mid, end, n)
        );

        let combined_dist = ArraySeqStEphS::append(&left_dist, &right_dist);
        let combined_pred = ArraySeqStEphS::append(&left_pred, &right_pred);

        (combined_dist, combined_pred)
    }

    /// Add dummy source vertex s with zero-weight edges to all vertices in G.
    ///
    /// - APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: Work O(n + m), Span O(n + m) — iterates over vertices and edges
    fn add_dummy_source(graph: &WeightedDirGraphStEphI128<usize>, n: usize) -> (augmented_and_idx: (WeightedDirGraphStEphI128<usize>, usize))
        requires
            n > 0,
            n < usize::MAX,
            n as nat == graph@.V.len(),
            spec_labgraphview_wf(graph@),
            valid_key_type_WeightedEdge::<usize, i128>(),
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
                valid_key_type_WeightedEdge::<usize, i128>(),
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
        let mut edges = SetStEph::<WeightedEdge<usize, i128>>::empty();
        let arcs = graph.labeled_arcs();
        let it = arcs.iter();
        let ghost arcs_seq = it@.1;

        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        for labeled_edge in iter: it
            invariant
                iter.elements == arcs_seq,
                arcs_seq.map(|i: int, e: LabEdge<usize, i128>| e@).to_set() =~= graph@.A,
                edges.spec_setsteph_wf(),
                vertices.spec_setsteph_wf(),
                forall|k: usize| vertices@.contains(k) <==> k <= n,
                spec_labgraphview_wf(graph@),
                valid_key_type_WeightedEdge::<usize, i128>(),
                forall|v: usize| graph@.V.contains(v) <==> v < n,
                forall|a: usize, b: usize, w: i128|
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

        // Add dummy edges from n to each vertex 0..n.
        let mut j: usize = 0;
        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        while j < n
            invariant
                j <= n,
                edges.spec_setsteph_wf(),
                vertices.spec_setsteph_wf(),
                forall|k: usize| vertices@.contains(k) <==> k <= n,
                valid_key_type_WeightedEdge::<usize, i128>(),
                forall|a: usize, b: usize, w: i128|
                    #[trigger] edges@.contains((a, b, w)) ==>
                    vertices@.contains(a) && vertices@.contains(b),
            decreases n - j,
        {
            let _ = edges.insert(WeightedEdge(n, j, 0i128));
            j = j + 1;
        }

        (WeightedDirGraphStEphI128::from_weighed_edges(vertices, edges), n)
    }

    /// Reweight edges: w'(u,v) = w(u,v) + p(u) - p(v)
    ///
    /// - APAS: Work O(m), Span O(m)
    /// - Claude-Opus-4.6: Work O(n + m), Span O(n + m) — rebuilds vertex set O(n) plus iterates edges O(m)
    fn reweight_graph(
        graph: &WeightedDirGraphStEphI128<usize>,
        potentials: &ArraySeqStEphS<i64>,
        n: usize,
    ) -> (reweighted: WeightedDirGraphStEphI128<usize>)
        requires
            n > 0,
            n < usize::MAX,
            n as nat == graph@.V.len(),
            potentials.seq@.len() == n as int,
            spec_labgraphview_wf(graph@),
            valid_key_type_WeightedEdge::<usize, i128>(),
            forall|v: usize| graph@.V.contains(v) <==> v < n,
        ensures
            spec_labgraphview_wf(reweighted@),
            reweighted@.V.len() == n as nat,
            forall|v: usize| v < n ==> reweighted@.V.contains(v),
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
                valid_key_type_WeightedEdge::<usize, i128>(),
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
        let mut reweighted_edges = SetStEph::<WeightedEdge<usize, i128>>::empty();
        let arcs = graph.labeled_arcs();
        let it = arcs.iter();
        let ghost arcs_seq = it@.1;

        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        for labeled_edge in iter: it
            invariant
                iter.elements == arcs_seq,
                arcs_seq.map(|i: int, e: LabEdge<usize, i128>| e@).to_set() =~= graph@.A,
                reweighted_edges.spec_setsteph_wf(),
                vertices.spec_setsteph_wf(),
                vertices@.len() == n as nat,
                forall|k: usize| vertices@.contains(k) <==> k < n,
                spec_labgraphview_wf(graph@),
                valid_key_type_WeightedEdge::<usize, i128>(),
                forall|v: usize| graph@.V.contains(v) <==> v < n,
                potentials.seq@.len() == n as int,
                forall|a: usize, b: usize, w: i128|
                    #[trigger] reweighted_edges@.contains((a, b, w)) ==>
                    vertices@.contains(a) && vertices@.contains(b),
        {
            let from = labeled_edge.0;
            let to = labeled_edge.1;
            let weight = labeled_edge.2;
            if from < n && to < n {
                let p_from = *potentials.nth(from) as i128;
                let p_to = *potentials.nth(to) as i128;
                let diff: i128 = p_from - p_to;
                let w_prime: i128 = if diff >= 0 {
                    if weight <= i128::MAX - diff { weight + diff } else { i128::MAX }
                } else {
                    if weight >= i128::MIN - diff { weight + diff } else { i128::MIN }
                };
                let _ = reweighted_edges.insert(WeightedEdge(from, to, w_prime));
            }
        }

        WeightedDirGraphStEphI128::from_weighed_edges(vertices, reweighted_edges)
    }

    /// Create result for negative cycle case.
    ///
    /// - APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: Work O(n^2), Span O(n^2) — builds n×n distance and predecessor matrices
    fn create_negative_cycle_result(n: usize) -> (result: AllPairsResultStEphI64)
        requires
            n < usize::MAX,
        ensures
            result.n == n,
            result.distances.seq@.len() == n as int,
            result.predecessors.seq@.len() == n as int,
    {
        let ghost n_int = n as int;
        let inner_dist = |_i: usize| -> (r: i64) ensures r == i64::MAX { i64::MAX };
        let dist_row = |_i: usize| -> (r: ArraySeqStEphS<i64>)
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
        AllPairsResultStEphI64 {
            distances,
            predecessors,
            n,
        }
    }

    } // verus!
}
