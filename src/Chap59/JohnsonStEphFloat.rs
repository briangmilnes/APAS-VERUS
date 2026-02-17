//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 59: Johnson's Algorithm - Single-threaded Ephemeral Float Weights
//!
//! Implements Algorithm 59.1 from the textbook.
//! All-Pairs Shortest Paths for graphs with negative weights (but no negative cycles).
//!
//! **Algorithmic Analysis:**
//! - Johnson APSP: Work O(mn log n), Span O(mn log n) where n = |V|, m = |E|
//! - Phase 1 (Bellman-Ford): Work O(nm), Span O(nm)
//! - Phase 2 (Reweighting): Work O(m), Span O(m)
//! - Phase 3 (n Dijkstras): Work O(n * m log n) = O(mn log n), Span O(mn log n) sequential

pub mod JohnsonStEphFloat {

    use ordered_float::OrderedFloat;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::*;
    use crate::Chap06::WeightedDirGraphStEphFloat::WeightedDirGraphStEphFloat::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap56::AllPairsResultStEphFloat::AllPairsResultStEphFloat::AllPairsResultStEphFloat;
    use crate::Chap57::DijkstraStEphFloat::DijkstraStEphFloat::dijkstra;
    use crate::Chap58::BellmanFordStEphFloat::BellmanFordStEphFloat::bellman_ford;
    use crate::Types::Types::*;
    pub type T = WeightedDirGraphStEphFloat<usize>;

    pub trait JohnsonStEphFloatTrait {
        /// Johnson's all-pairs shortest path algorithm
        /// APAS: Work O(mn log n), Span O(mn log n) where n = |V|, m = |E|
        fn johnson_apsp(graph: &WeightedDirGraphStEphFloat<usize>) -> AllPairsResultStEphFloat;
    }

    /// Algorithm 59.1: Johnson's All-Pairs Shortest Paths (Float version)
    ///
    /// Solves APSP problem with negative float weights allowed using:
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
    /// * `graph` - Weighted directed graph with float weights (can be negative, no negative cycles)
    ///
    /// # Returns
    /// `AllPairsResultStEphFloat` containing n×n distance matrix and predecessor matrix
    pub fn johnson_apsp(graph: &WeightedDirGraphStEphFloat<usize>) -> AllPairsResultStEphFloat {
        let n = graph.vertices().size();
        let (graph_with_dummy, dummy_idx) = add_dummy_source(graph, n);

        let bellman_ford_result = match bellman_ford(&graph_with_dummy, dummy_idx) {
            | Ok(result) => result,
            | Err(_) => return create_negative_cycle_result(n),
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
                    if d_prime.0.is_infinite() {
                        OrderedFloat(f64::INFINITY)
                    } else {
                        let p_v = *potentials.nth(v);
                        OrderedFloat(d_prime.0 - p_u.0 + p_v.0)
                    }
                },
                n,
            );

            all_distances = ArraySeqStEphS::append(&all_distances, &ArraySeqStEphS::singleton(adjusted_row));
            all_predecessors = ArraySeqStEphS::append(
                &all_predecessors,
                &ArraySeqStEphS::singleton(dijkstra_result.predecessors.clone()),
            );
        }

        AllPairsResultStEphFloat {
            distances: all_distances,
            predecessors: all_predecessors,
            n,
        }
    }

    fn add_dummy_source(
        graph: &WeightedDirGraphStEphFloat<usize>,
        n: usize,
    ) -> (WeightedDirGraphStEphFloat<usize>, usize) {
        let dummy_idx = n;
        let mut vertices = SetStEph::empty();
        for i in 0..=n {
            vertices.insert(i);
        }
        let mut edges = SetStEph::empty();
        for LabEdge(from, to, weight) in graph.labeled_arcs().iter() {
            edges.insert(Triple(*from, *to, *weight));
        }
        for i in 0..n {
            edges.insert(Triple(dummy_idx, i, OrderedFloat(0.0)));
        }
        (
            WeightedDirGraphStEphFloat::from_weighted_edges(vertices, edges),
            dummy_idx,
        )
    }

    fn reweight_graph(
        graph: &WeightedDirGraphStEphFloat<usize>,
        potentials: &ArraySeqStEphS<OrderedF64>,
        n: usize,
    ) -> WeightedDirGraphStEphFloat<usize> {
        let mut vertices = SetStEph::empty();
        for i in 0..n {
            vertices.insert(i);
        }
        let mut edges = SetStEph::empty();
        for LabEdge(from, to, weight) in graph.labeled_arcs().iter() {
            let p_from = *potentials.nth(*from);
            let p_to = *potentials.nth(*to);
            let new_weight = OrderedFloat(weight.0 + p_from.0 - p_to.0);
            edges.insert(Triple(*from, *to, new_weight));
        }
        WeightedDirGraphStEphFloat::from_weighted_edges(vertices, edges)
    }

    fn create_negative_cycle_result(n: usize) -> AllPairsResultStEphFloat {
        let distances = ArraySeqStEphS::tabulate(&|_| ArraySeqStEphS::tabulate(&|_| OrderedFloat(f64::INFINITY), n), n);
        let predecessors = ArraySeqStEphS::tabulate(&|_| ArraySeqStEphS::tabulate(&|_| 0, n), n);
        AllPairsResultStEphFloat {
            distances,
            predecessors,
            n,
        }
    }
}
