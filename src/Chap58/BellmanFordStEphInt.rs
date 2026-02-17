//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! Bellman-Ford's Algorithm - Single Source Shortest Path with arbitrary edge weights (integer)
//!
//! Implements Algorithm 58.2 from the textbook.
//! Handles negative edge weights and detects negative-weight cycles.
//!
//! **Algorithmic Analysis:**
//! - Bellman-Ford: Work O(nm), Span O(n lg n) where n = |V|, m = |E|
//! - Parallelizable: Lines 5-6 parallelize over vertices

pub mod BellmanFordStEphInt {

    use std::collections::HashMap;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::LabDirGraphStEphTrait;
    use crate::Chap06::WeightedDirGraphStEphI128::WeightedDirGraphStEphI128::*;
    use crate::Chap56::SSSPResultStEphInt::SSSPResultStEphInt::SSSPResultStEphInt;
    use crate::Types::Types::*;
    pub type T = WeightedDirGraphStEphI128<usize>;

    pub trait BellmanFordStEphIntTrait {
        /// Bellman-Ford single source shortest path algorithm
        /// APAS: Work O(nm), Span O(n lg n) where n = |V|, m = |E|
        fn bellman_ford(graph: &WeightedDirGraphStEphI128<usize>, source: usize) -> Result<SSSPResultStEphInt, String>;
    }

    /// Runs Bellman-Ford algorithm on a weighted directed graph
    /// Computes single-source shortest paths for arbitrary edge weights (including negative)
    ///
    /// **Algorithm 58.2**: Iterative k-hop distance computation
    ///
    /// Work: O(nm), Span: O(n lg n) where n = |V|, m = |E|
    ///
    /// # Arguments
    /// * `graph` - Weighted directed graph with integer weights (can be negative)
    /// * `source` - Source vertex (0-indexed)
    ///
    /// # Returns
    /// * `Ok(SSSPResultStEphInt)` - Shortest paths if no negative-weight cycles
    /// * `Err(String)` - Error message if negative-weight cycle detected
    ///
    /// # Algorithm
    /// 1. Initialize distances: d(s) = 0, d(v) = ∞ for v ≠ s
    /// 2. For each round k = 0 to |V|-1:
    ///    - For each vertex v in parallel:
    ///      - Compute Din(v) = min over u ∈ N⁻(v) of (d[u] + w(u,v))
    ///      - Update d'[v] = min(d[v], Din(v))
    ///    - If no distances changed, return (converged)
    /// 3. If |V| rounds completed without convergence, negative cycle exists
    pub fn bellman_ford(graph: &WeightedDirGraphStEphI128<usize>, source: usize) -> Result<SSSPResultStEphInt, String> {
        let n = graph.vertices().size();

        // Initialize distances: source = 0, others = infinity
        let mut distances = HashMap::<usize, i64>::new();
        for v in 0..n {
            distances.insert(v, if v == source { 0 } else { i64::MAX });
        }

        // Iterate up to |V| rounds
        for round in 0..n {
            let mut changed = false;
            let mut new_distances = HashMap::<usize, i64>::new();

            // For each vertex, compute minimum distance through in-neighbors (Line 5)
            for v in 0..n {
                let current_dist = *distances.get(&v).unwrap_or(&i64::MAX);
                let mut min_dist = current_dist;

                // Compute Din(v) = min over u in N⁻(v) of (d[u] + w(u,v))
                let in_neighbors = graph.in_neighbors_weighed(&v);
                for Pair(u, weight) in in_neighbors.iter() {
                    let u_dist = *distances.get(u).unwrap_or(&i64::MAX);
                    if u_dist != i64::MAX {
                        // Check for overflow before adding
                        let new_dist = u_dist.saturating_add(*weight as i64);
                        if new_dist < min_dist {
                            min_dist = new_dist;
                        }
                    }
                }

                // Update distance (Line 6)
                new_distances.insert(v, min_dist);
                if min_dist != current_dist {
                    changed = true;
                }
            }

            // Update distances for next round
            distances = new_distances;

            // Check for convergence (Line 9-10)
            if !changed {
                // No changes, converged - build result
                let mut result = SSSPResultStEphInt::new(n, source);
                for v in 0..n {
                    let dist = *distances.get(&v).unwrap_or(&i64::MAX);
                    result.set_distance(v, dist);
                }

                // Reconstruct predecessors
                reconstruct_predecessors(graph, &distances, &mut result, source);

                return Ok(result);
            }

            // Check if we've reached |V| rounds without convergence (Line 8)
            if round == n - 1 {
                return Err("Negative-weight cycle detected".to_string());
            }
        }

        // Should not reach here, but handle it
        Err("Algorithm error: max rounds exceeded".to_string())
    }

    /// Reconstruct predecessor tree from final distances
    /// For each vertex v, find the in-neighbor u that achieves the shortest path
    fn reconstruct_predecessors(
        graph: &WeightedDirGraphStEphI128<usize>,
        distances: &HashMap<usize, i64>,
        result: &mut SSSPResultStEphInt,
        source: usize,
    ) {
        let n = graph.vertices().size();
        for v in 0..n {
            if v == source {
                continue;
            }

            let v_dist = *distances.get(&v).unwrap_or(&i64::MAX);
            if v_dist == i64::MAX {
                continue; // Unreachable
            }

            // Find which in-neighbor u gave us the shortest path
            let in_neighbors = graph.in_neighbors_weighed(&v);
            for Pair(u, weight) in in_neighbors.iter() {
                let u_dist = *distances.get(u).unwrap_or(&i64::MAX);
                if u_dist != i64::MAX {
                    let path_dist = u_dist.saturating_add(*weight as i64);
                    if path_dist == v_dist {
                        result.set_predecessor(v, *u);
                        break; // Found the predecessor
                    }
                }
            }
        }
    }
}
