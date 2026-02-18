//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 59: Johnson's Algorithm - Single-threaded Ephemeral Integer Weights
//!
//! Implements Algorithm 59.1 from the textbook.
//! All-Pairs Shortest Paths for graphs with negative weights (but no negative cycles).
//!
//! **Algorithmic Analysis:**
//! - Johnson APSP: Work O(mn log n), Span O(mn log n) where n = |V|, m = |E|
//! - Phase 1 (Bellman-Ford): Work O(nm), Span O(nm)
//! - Phase 2 (Reweighting): Work O(m), Span O(m)
//! - Phase 3 (n Dijkstras): Work O(n * m log n) = O(mn log n), Span O(mn log n) sequential

pub mod JohnsonStEphInt {

    use vstd::prelude::*;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::*;
    use crate::Chap06::WeightedDirGraphStEphI128::WeightedDirGraphStEphI128::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap56::AllPairsResultStEphInt::AllPairsResultStEphInt::*;
    use crate::Chap56::SSSPResultStEphInt::SSSPResultStEphInt::*;
    use crate::Types::Types::*;

    #[cfg(not(verus_keep_ghost))]
    use crate::Chap57::DijkstraStEphInt::DijkstraStEphInt::dijkstra;
    #[cfg(not(verus_keep_ghost))]
    use crate::Chap58::BellmanFordStEphInt::BellmanFordStEphInt::bellman_ford;

    verus! {
        pub trait JohnsonStEphIntTrait {
            /// Johnson's all-pairs shortest path algorithm
            /// APAS: Work O(mn log n), Span O(mn log n) where n = |V|, m = |E|
            fn johnson_apsp(graph: &WeightedDirGraphStEphI128<usize>) -> AllPairsResultStEphInt;
        }
    } // verus!

    #[cfg(not(verus_keep_ghost))]
    pub type T = WeightedDirGraphStEphI128<usize>;

    /// Algorithm 59.1: Johnson's All-Pairs Shortest Paths
    ///
    /// Solves APSP problem with negative weights allowed using:
    /// 1. Bellman-Ford to compute potentials and eliminate negative weights
    /// 2. Dijkstra from each vertex on reweighted graph
    ///
    /// - APAS: Work O(mn log n), Span O(m log n)
    /// - Claude-Opus-4.6: Work O(mn log n), Span O(mn log n) — sequential loop over n Dijkstra runs makes Span = Work for Phase 3
    #[cfg(not(verus_keep_ghost))]
    pub fn johnson_apsp(graph: &WeightedDirGraphStEphI128<usize>) -> AllPairsResultStEphInt {
        let n = graph.vertices().size();

        let (graph_with_dummy, dummy_idx) = add_dummy_source(graph, n);

        let bellman_ford_result = match bellman_ford(&graph_with_dummy, dummy_idx) {
            | Ok(result) => result,
            | Err(_) => {
                return create_negative_cycle_result(n);
            }
        };

        let potentials = ArraySeqStEphS::tabulate(&|i| bellman_ford_result.get_distance(i), n);

        let reweighted_graph = reweight_graph(graph, &potentials, n);

        let mut all_distances = ArraySeqStEphS::empty();
        let mut all_predecessors = ArraySeqStEphS::empty();

        for u in 0..n {
            let dijkstra_result = dijkstra(&reweighted_graph, u);

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

            let singleton_dist = ArraySeqStEphS::singleton(adjusted_row);
            let singleton_pred = ArraySeqStEphS::singleton(dijkstra_result.predecessors.clone());

            all_distances = ArraySeqStEphS::append(&all_distances, &singleton_dist);
            all_predecessors = ArraySeqStEphS::append(&all_predecessors, &singleton_pred);
        }

        AllPairsResultStEphInt {
            distances: all_distances,
            predecessors: all_predecessors,
            n,
        }
    }

    /// Add dummy source vertex s with zero-weight edges to all vertices in G.
    ///
    /// - APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: Work O(n + m), Span O(n + m) — iterates over vertices and edges
    #[cfg(not(verus_keep_ghost))]
    fn add_dummy_source(graph: &WeightedDirGraphStEphI128<usize>, n: usize) -> (WeightedDirGraphStEphI128<usize>, usize) {
        let dummy_idx = n;

        let mut vertices = SetStEph::empty();
        for i in 0..=n {
            vertices.insert(i);
        }

        let mut edges = SetStEph::empty();
        for LabEdge(from, to, weight) in graph.labeled_arcs().iter() {
            edges.insert(WeightedEdge(*from, *to, *weight));
        }

        for i in 0..n {
            edges.insert(WeightedEdge(dummy_idx, i, 0i128));
        }

        (
            WeightedDirGraphStEphI128::from_weighed_edges(vertices, edges),
            dummy_idx,
        )
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

        let mut edges = SetStEph::empty();
        for LabEdge(from, to, weight) in graph.labeled_arcs().iter() {
            let p_from = *potentials.nth(*from) as i128;
            let p_to = *potentials.nth(*to) as i128;
            let new_weight: i128 = *weight + p_from - p_to;
            edges.insert(WeightedEdge(*from, *to, new_weight));
        }

        WeightedDirGraphStEphI128::from_weighed_edges(vertices, edges)
    }

    /// Create result for negative cycle case.
    ///
    /// - APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: Work O(n^2), Span O(n^2) — builds n×n distance and predecessor matrices
    #[cfg(not(verus_keep_ghost))]
    fn create_negative_cycle_result(n: usize) -> AllPairsResultStEphInt {
        let distances = ArraySeqStEphS::tabulate(&|_| ArraySeqStEphS::tabulate(&|_| i64::MAX, n), n);
        let predecessors = ArraySeqStEphS::tabulate(&|_| ArraySeqStEphS::tabulate(&|_| 0, n), n);
        AllPairsResultStEphInt {
            distances,
            predecessors,
            n,
        }
    }
}
