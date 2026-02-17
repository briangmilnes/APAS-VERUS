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

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::*;
    use crate::Chap06::WeightedDirGraphStEphInt::WeightedDirGraphStEphInt::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap56::AllPairsResultStEphInt::AllPairsResultStEphInt::AllPairsResultStEphInt;
    use crate::Chap57::DijkstraStEphInt::DijkstraStEphInt::dijkstra;
    use crate::Chap58::BellmanFordStEphInt::BellmanFordStEphInt::bellman_ford;
    use crate::Types::Types::*;
    pub type T = WeightedDirGraphStEphInt<usize>;

    pub trait JohnsonStEphIntTrait {
        /// Johnson's all-pairs shortest path algorithm
        /// APAS: Work O(mn log n), Span O(mn log n) where n = |V|, m = |E|
        fn johnson_apsp(graph: &WeightedDirGraphStEphInt<usize>) -> AllPairsResultStEphInt;
    }

    /// Algorithm 59.1: Johnson's All-Pairs Shortest Paths
    ///
    /// Solves APSP problem with negative weights allowed using:
    /// 1. Bellman-Ford to compute potentials and eliminate negative weights
    /// 2. Dijkstra from each vertex on reweighted graph
    ///
    /// **APAS Analysis:** Work O(mn log n), Span O(m log n)
    /// **Claude Analysis:**
    /// - Phase 1: Bellman-Ford on G' (n+1 vertices, m+n edges): Work O((n+1)(m+n)) = O(nm), Span O(nm)
    /// - Phase 2: Reweight m edges: Work O(m), Span O(m)
    /// - Phase 3: n sequential Dijkstra runs: Work O(n * m log n) = O(mn log n), Span O(mn log n)
    /// - Total: Work O(mn log n), Span O(mn log n)
    ///
    /// # Arguments
    /// * `graph` - Weighted directed graph with integer weights (can be negative, no negative cycles)
    ///
    /// # Returns
    /// `AllPairsResultStEphInt` containing n×n distance matrix and predecessor matrix
    pub fn johnson_apsp(graph: &WeightedDirGraphStEphInt<usize>) -> AllPairsResultStEphInt {
        let n = graph.vertices().size();

        // Phase 1: Add dummy source and run Bellman-Ford
        let (graph_with_dummy, dummy_idx) = add_dummy_source(graph, n);

        let bellman_ford_result = match bellman_ford(&graph_with_dummy, dummy_idx) {
            | Ok(result) => result,
            | Err(_) => {
                // Negative cycle detected - return infinity matrix
                return create_negative_cycle_result(n);
            }
        };

        // Extract potentials
        let potentials = ArraySeqStEphS::tabulate(&|i| bellman_ford_result.get_distance(i), n);

        // Phase 2: Reweight edges
        let reweighted_graph = reweight_graph(graph, &potentials, n);

        // Phase 3: Run Dijkstra from each vertex and adjust distances
        let mut all_distances = ArraySeqStEphS::empty();
        let mut all_predecessors = ArraySeqStEphS::empty();

        for u in 0..n {
            let dijkstra_result = dijkstra(&reweighted_graph, u);

            // Adjust distances: δG(u,v) = δG'(u,v) - p(u) + p(v)
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

    /// Add dummy source with zero-weight edges to all vertices
    fn add_dummy_source(graph: &WeightedDirGraphStEphInt<usize>, n: usize) -> (WeightedDirGraphStEphInt<usize>, usize) {
        let dummy_idx = n;

        // Create vertices including dummy
        let mut vertices = SetStEph::empty();
        for i in 0..=n {
            vertices.insert(i);
        }

        // Copy all original edges
        let mut edges = SetStEph::empty();
        for LabEdge(from, to, weight) in graph.labeled_arcs().iter() {
            edges.insert(Triple(*from, *to, *weight));
        }

        // Add zero-weight edges from dummy to all original vertices
        for i in 0..n {
            edges.insert(Triple(dummy_idx, i, 0));
        }

        (
            WeightedDirGraphStEphInt::from_weighted_edges(vertices, edges),
            dummy_idx,
        )
    }

    /// Reweight edges: w'(u,v) = w(u,v) + p(u) - p(v)
    fn reweight_graph(
        graph: &WeightedDirGraphStEphInt<usize>,
        potentials: &ArraySeqStEphS<i64>,
        n: usize,
    ) -> WeightedDirGraphStEphInt<usize> {
        let mut vertices = SetStEph::empty();
        for i in 0..n {
            vertices.insert(i);
        }

        let mut edges = SetStEph::empty();
        for LabEdge(from, to, weight) in graph.labeled_arcs().iter() {
            let p_from = *potentials.nth(*from);
            let p_to = *potentials.nth(*to);
            let new_weight = (*weight as i64 + p_from - p_to) as i32;
            edges.insert(Triple(*from, *to, new_weight));
        }

        WeightedDirGraphStEphInt::from_weighted_edges(vertices, edges)
    }

    /// Create result for negative cycle case
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
