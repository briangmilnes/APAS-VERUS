//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
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


//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 4. type definitions
//	Section 8. traits
//	Section 9. impls

//		Section 1. module

pub mod JohnsonMtEphI64 {


    //		Section 2. imports

    use vstd::prelude::*;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::*;
    use crate::Chap06::WeightedDirGraphStEphI128::WeightedDirGraphStEphI128::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap56::AllPairsResultStEphI64::AllPairsResultStEphI64::*;
    use crate::Chap56::SSSPResultStEphI64::SSSPResultStEphI64::*;
    use crate::Types::Types::*;

    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_clone;

    use crate::Chap57::DijkstraStEphU64::DijkstraStEphU64::dijkstra;
    use crate::Chap58::BellmanFordStEphI64::BellmanFordStEphI64::bellman_ford;

    verus! 
{

    //		Section 4. type definitions


    pub type T = WeightedDirGraphStEphI128<usize>;

    //		Section 8. traits


        pub trait JohnsonMtEphI64Trait {
            /// Parallel Johnson's all-pairs shortest path algorithm
            /// APAS: Work O(mn log n), Span O(m log n) where n = |V|, m = |E|
            /// - Alg Analysis: APAS (Ch59 Alg 59.1): Work O(mn lg n), Span O(m lg n)
            /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(mn lg n), Span O(m lg n); parallel: n Dijkstra calls via join
            fn johnson_apsp(graph: &WeightedDirGraphStEphI128<usize>) -> (apsp: AllPairsResultStEphI64)
                requires
                    graph@.V.len() > 0,
                    graph@.V.len() < usize::MAX as nat,
                    spec_labgraphview_wf(graph@),
                    valid_key_type_WeightedEdge::<usize, i128>(),
                    forall|v: usize| graph@.V.contains(v) <==> v < graph@.V.len(),
                    graph@.A.len() * 2 + 2 <= usize::MAX as int,
                    obeys_feq_clone::<ArraySeqStEphS<i64>>(),
                    obeys_feq_clone::<ArraySeqStEphS<usize>>(),
                ensures
                    apsp.spec_n() as nat == graph@.V.len();
        }

    //		Section 9. impls


    /// Adjust reweighted distance back to original weights.
    /// d(u,v) = d'(u,v) - h(u) + h(v), using i128 to avoid overflow.
    ///
    /// - Alg Analysis: APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: Work O(1), Span O(1).
    // veracity: no_requires
    fn adjust_distance(d_prime: i64, h_u: i64, h_v: i64) -> (adjusted: i64)
        ensures
            d_prime == i64::MAX ==> adjusted == i64::MAX,
    {
        if d_prime == i64::MAX { i64::MAX }
        else {
            let sum: i128 = (d_prime as i128) - (h_u as i128) + (h_v as i128);
            if sum >= i64::MAX as i128 { i64::MAX }
            else if sum < i64::MIN as i128 { i64::MIN }
            else { sum as i64 }
        }
    }

    /// Algorithm 59.1: Johnson's All-Pairs Shortest Paths (Parallel)
    ///
    /// Solves APSP problem with negative weights using:
    /// 1. Bellman-Ford to compute potentials (sequential)
    /// 2. Reweight edges (sequential)
    /// 3. Parallel Dijkstra from each vertex using ParaPair! divide-and-conquer
    ///
    /// - Alg Analysis: APAS (Ch59 Alg 59.1): Work O(mn lg n), Span O(m lg n)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(mn lg n), Span O(m lg n); parallel: BF O(nm) then n x Dijkstra in parallel
    pub fn johnson_apsp(graph: &WeightedDirGraphStEphI128<usize>) -> (apsp: AllPairsResultStEphI64)
        requires
            graph@.V.len() > 0,
            graph@.V.len() < usize::MAX as nat,
            spec_labgraphview_wf(graph@),
            valid_key_type_WeightedEdge::<usize, i128>(),
            forall|v: usize| graph@.V.contains(v) <==> v < graph@.V.len(),
            graph@.A.len() * 2 + 2 <= usize::MAX as int,
            obeys_feq_clone::<ArraySeqStEphS<i64>>(),
            obeys_feq_clone::<ArraySeqStEphS<usize>>(),
        ensures
            apsp.spec_n() as nat == graph@.V.len(),
    {
        let n = graph.vertices().size();

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
        // Veracity: NEEDED proof block
        proof {
        }

        let (all_distances, all_predecessors) = parallel_dijkstra_all(&reweighted_graph, &potentials, 0, n, n);

        AllPairsResultStEphI64 {
            distances: all_distances,
            predecessors: all_predecessors,
            n,
        }
    }

    /// Parallel Dijkstra execution using recursive divide-and-conquer with ParaPair!
    ///
    /// - Alg Analysis: APAS: N/A — internal helper, not named in prose.
    /// - Claude-Opus-4.6: Work O(k * m log n), Span O(m log n) where k = end - start — binary split with ParaPair! gives log k depth, each leaf runs Dijkstra O(m log n)
    fn parallel_dijkstra_all(
        graph: &WeightedDirGraphStEphI128<usize>,
        potentials: &ArraySeqStEphS<i64>,
        start: usize,
        end: usize,
        n: usize,
    ) -> (dist_pred: (
        ArraySeqStEphS<ArraySeqStEphS<i64>>,
        ArraySeqStEphS<ArraySeqStEphS<usize>>,
    ))
        requires
            start <= end,
            end <= n,
            n > 0,
            n < usize::MAX,
            n as nat == graph@.V.len(),
            potentials.seq@.len() == n as int,
            spec_labgraphview_wf(graph@),
            valid_key_type_WeightedEdge::<usize, i128>(),
            forall|v: usize| graph@.V.contains(v) <==> v < n,
            graph@.A.len() * 2 + 2 <= usize::MAX as int,
            obeys_feq_clone::<ArraySeqStEphS<i64>>(),
            obeys_feq_clone::<ArraySeqStEphS<usize>>(),
        ensures
            dist_pred.0.seq@.len() == (end - start) as int,
            dist_pred.1.seq@.len() == (end - start) as int,
        decreases end - start,
    {
        let range_size = end - start;

        if range_size == 0 {
            return (ArraySeqStEphS::empty(), ArraySeqStEphS::empty());
        }

        if range_size == 1 {
            let u = start;
            let dijkstra_result = dijkstra(graph, u);

            let p_u = *potentials.nth(u);
            let ghost pot_len = potentials.seq@.len();
            let adjusted_row = ArraySeqStEphS::tabulate(
                &(|v: usize| -> (r: i64)
                    requires
                        v < n,
                        potentials.seq@.len() == n as int,
                        n as nat == graph@.V.len(),
                    ensures true,
                {
                    let d_prime = dijkstra_result.get_distance(v);
                    adjust_distance(d_prime, p_u, *potentials.nth(v))
                }),
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

        let f1 = move || -> (r: (ArraySeqStEphS<ArraySeqStEphS<i64>>, ArraySeqStEphS<ArraySeqStEphS<usize>>))
            requires
                start <= mid,
                mid <= n,
                n > 0,
                n < usize::MAX,
                n as nat == graph_left@.V.len(),
                potentials_left.seq@.len() == n as int,
                spec_labgraphview_wf(graph_left@),
                valid_key_type_WeightedEdge::<usize, i128>(),
                forall|v: usize| graph_left@.V.contains(v) <==> v < n,
                graph_left@.A.len() * 2 + 2 <= usize::MAX as int,
                obeys_feq_clone::<ArraySeqStEphS<i64>>(),
                obeys_feq_clone::<ArraySeqStEphS<usize>>(),
            ensures
                r.0.seq@.len() == (mid - start) as int,
                r.1.seq@.len() == (mid - start) as int,
        {
            parallel_dijkstra_all(&graph_left, &potentials_left, start, mid, n)
        };

        let f2 = move || -> (r: (ArraySeqStEphS<ArraySeqStEphS<i64>>, ArraySeqStEphS<ArraySeqStEphS<usize>>))
            requires
                mid <= end,
                end <= n,
                n > 0,
                n < usize::MAX,
                n as nat == graph_right@.V.len(),
                potentials_right.seq@.len() == n as int,
                spec_labgraphview_wf(graph_right@),
                valid_key_type_WeightedEdge::<usize, i128>(),
                forall|v: usize| graph_right@.V.contains(v) <==> v < n,
                graph_right@.A.len() * 2 + 2 <= usize::MAX as int,
                obeys_feq_clone::<ArraySeqStEphS<i64>>(),
                obeys_feq_clone::<ArraySeqStEphS<usize>>(),
            ensures
                r.0.seq@.len() == (end - mid) as int,
                r.1.seq@.len() == (end - mid) as int,
        {
            parallel_dijkstra_all(&graph_right, &potentials_right, mid, end, n)
        };

        let Pair((left_dist, left_pred), (right_dist, right_pred)) = crate::ParaPair!(f1, f2);

        let combined_dist = ArraySeqStEphS::append(&left_dist, &right_dist);
        let combined_pred = ArraySeqStEphS::append(&left_pred, &right_pred);

        (combined_dist, combined_pred)
    }

    /// Add dummy source vertex s with zero-weight edges to all vertices in G.
    ///
    /// - Alg Analysis: APAS: N/A — Verus-specific scaffolding.
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
            let _ = vertices.insert(i);
            i = i + 1;
        }
        // Veracity: NEEDED proof block
        proof {
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
    /// - Alg Analysis: APAS (Ch59 Alg 59.1): Work O(m), Span O(m)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m), Span O(m)
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
            graph@.A.len() * 2 + 2 <= usize::MAX as int,
        ensures
            spec_labgraphview_wf(reweighted@),
            reweighted@.V.len() == n as nat,
            forall|v: usize| reweighted@.V.contains(v) <==> v < n,
            reweighted@.A.len() <= graph@.A.len(),
            reweighted@.A.len() * 2 + 2 <= usize::MAX as int,
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
            let _ = vertices.insert(i);
            i = i + 1;
        }
        // Veracity: NEEDED proof block
        proof {
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
                iter.pos <= arcs_seq.len(),
                arcs_seq.map(|i: int, e: LabEdge<usize, i128>| e@).to_set() =~= graph@.A,
                reweighted_edges.spec_setsteph_wf(),
                reweighted_edges@.len() <= iter.pos as nat,
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

        // Veracity: NEEDED proof block
        proof {
            // Prove reweighted@.A.len() <= graph@.A.len().
            // After the loop: reweighted_edges@.len() <= arcs_seq.len().
            // Prove arcs_seq.len() == graph@.A.len() via injective mapping.
            let view_fn = |k: LabEdge<usize, i128>| k@;
            // Veracity: NEEDED assert
            assert forall|x: LabEdge<usize, i128>, y: LabEdge<usize, i128>|
                #[trigger] view_fn(x) == #[trigger] view_fn(y) implies x == y
            by {};
            arcs_seq.lemma_no_duplicates_injective(view_fn);
            let mapped = arcs_seq.map_values(view_fn);
            mapped.unique_seq_to_set();
            // Veracity: NEEDED assert
            assert(mapped =~= arcs_seq.map(|i: int, k: LabEdge<usize, i128>| k@));
        }

        let result = WeightedDirGraphStEphI128::from_weighed_edges(vertices, reweighted_edges);
        // Veracity: NEEDED proof block
        proof {
        }
        result
    }

    /// Create result for negative cycle case.
    ///
    /// - Alg Analysis: APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: Work O(n^2), Span O(n^2) — builds n×n distance and predecessor matrices
    fn create_negative_cycle_result(n: usize) -> (neg_cycle_apsp: AllPairsResultStEphI64)
        requires
            n < usize::MAX,
        ensures
            neg_cycle_apsp.n == n,
            neg_cycle_apsp.distances.seq@.len() == n as int,
            neg_cycle_apsp.predecessors.seq@.len() == n as int,
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
