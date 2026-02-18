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
    use crate::Chap56::AllPairsResultStEphI64::AllPairsResultStEphI64::AllPairsResultStEphI64;
    use crate::Types::Types::*;

    #[cfg(not(verus_keep_ghost))]
    use std::thread;
    #[cfg(not(verus_keep_ghost))]
    use crate::Chap57::DijkstraStEphI64::DijkstraStEphI64::dijkstra;
    #[cfg(not(verus_keep_ghost))]
    use crate::Chap58::BellmanFordStEphI64::BellmanFordStEphI64::bellman_ford;

    verus! {
        pub trait JohnsonMtEphI64Trait {
            /// Parallel Johnson's all-pairs shortest path algorithm
            /// APAS: Work O(mn log n), Span O(m log n) where n = |V|, m = |E|
            fn johnson_apsp(graph: &WeightedDirGraphStEphI128<usize>) -> AllPairsResultStEphI64;
        }
    } // verus!

    #[cfg(not(verus_keep_ghost))]
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
    #[cfg(not(verus_keep_ghost))]
    pub fn johnson_apsp(graph: &WeightedDirGraphStEphI128<usize>) -> AllPairsResultStEphI64 {
        let n = graph.vertices().size();

        let (graph_with_dummy, dummy_idx) = add_dummy_source(&graph, n);

        let bellman_ford_result = match bellman_ford(&graph_with_dummy, dummy_idx) {
            | Ok(res) => res,
            | Err(_) => {
                return create_negative_cycle_result(n);
            }
        };

        let potentials = ArraySeqStEphS::tabulate(&|i| bellman_ford_result.get_distance(i), n);

        let reweighted_graph = reweight_graph(&graph, &potentials, n);

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
    #[cfg(not(verus_keep_ghost))]
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
    #[cfg(not(verus_keep_ghost))]
    fn add_dummy_source(graph: &WeightedDirGraphStEphI128<usize>, n: usize) -> (WeightedDirGraphStEphI128<usize>, usize) {
        let mut vertices = SetStEph::empty();
        for i in 0..n {
            vertices.insert(i);
        }

        vertices.insert(n);

        let mut edges = SetStEph::empty();

        for LabEdge(from, to, weight) in graph.labeled_arcs().iter() {
            edges.insert(WeightedEdge(*from, *to, *weight));
        }

        for v in 0..n {
            edges.insert(WeightedEdge(n, v, 0i128));
        }

        (WeightedDirGraphStEphI128::from_weighed_edges(vertices, edges), n)
    }

    /// Reweight edges: w'(u,v) = w(u,v) + p(u) - p(v)
    ///
    /// - APAS: Work O(m), Span O(m)
    /// - Claude-Opus-4.6: Work O(n + m), Span O(n + m) — rebuilds vertex set O(n) plus iterates edges O(m)
    #[cfg(not(verus_keep_ghost))]
    fn reweight_graph(
        graph: &WeightedDirGraphStEphI128<usize>,
        potentials: &ArraySeqStEphS<i64>,
        n: usize,
    ) -> WeightedDirGraphStEphI128<usize> {
        let mut vertices = SetStEph::empty();
        for i in 0..n {
            vertices.insert(i);
        }

        let mut reweighted_edges = SetStEph::empty();
        for LabEdge(from, to, weight) in graph.labeled_arcs().iter() {
            let p_from = *potentials.nth(*from) as i128;
            let p_to = *potentials.nth(*to) as i128;
            let w_prime: i128 = *weight + p_from - p_to;
            reweighted_edges.insert(WeightedEdge(*from, *to, w_prime));
        }

        WeightedDirGraphStEphI128::from_weighed_edges(vertices, reweighted_edges)
    }

    /// Create result for negative cycle case.
    ///
    /// - APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: Work O(n^2), Span O(n^2) — builds n×n distance and predecessor matrices
    #[cfg(not(verus_keep_ghost))]
    fn create_negative_cycle_result(n: usize) -> AllPairsResultStEphI64 {
        let distances = ArraySeqStEphS::tabulate(&|_| ArraySeqStEphS::tabulate(&|_| i64::MAX, n), n);
        let predecessors = ArraySeqStEphS::tabulate(&|_| ArraySeqStEphS::tabulate(&|_| 0, n), n);
        AllPairsResultStEphI64 {
            distances,
            predecessors,
            n,
        }
    }
}
