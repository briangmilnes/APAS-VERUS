//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! Bellman-Ford's Algorithm - Single Source Shortest Path with arbitrary edge weights (float)
//!
//! Implements Algorithm 58.2 from the textbook.
//! Handles negative edge weights and detects negative-weight cycles.
//!
//! **Algorithmic Analysis:**
//! - Bellman-Ford: Work O(nm), Span O(n lg n) where n = |V|, m = |E|
//! - Parallelizable: Lines 5-6 parallelize over vertices

pub mod BellmanFordStEphFloat {

    use std::collections::HashMap;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::LabDirGraphStEphTrait;
    use crate::Chap06::WeightedDirGraphStEphFloat::WeightedDirGraphStEphFloat::*;
    use crate::Chap56::SSSPResultStEphFloat::SSSPResultStEphFloat::SSSPResultStEphFloat;
    use crate::Types::Types::*;
    pub type T = WeightedDirGraphStEphFloat<usize>;

    pub trait BellmanFordStEphFloatTrait {
        /// Bellman-Ford single source shortest path algorithm
        /// - APAS: Work O(nm), Span O(n lg n) where n = |V|, m = |E|
        /// - Claude-Opus-4.6: Work O(nm), Span O(nm) — sequential implementation; inner vertex loop not parallelized
        fn bellman_ford(graph: &WeightedDirGraphStEphFloat<usize>, source: usize) -> SSSPResultStEphFloat;
    }

    /// Runs Bellman-Ford algorithm on a weighted directed graph
    /// Computes single-source shortest paths for arbitrary edge weights (including negative)
    ///
    /// **Algorithm 58.2**: Iterative k-hop distance computation
    ///
    /// - APAS: Work O(nm), Span O(n lg n) where n = |V|, m = |E|
    /// - Claude-Opus-4.6: Work O(nm), Span O(nm) — sequential implementation; inner vertex loop not parallelized
    ///
    /// # Arguments
    /// * `graph` - Weighted directed graph with float weights (can be negative)
    /// * `source` - Source vertex (0-indexed)
    ///
    /// # Returns
    /// * `Ok(SSSPResultStEphFloat)` - Shortest paths if no negative-weight cycles
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
    pub fn bellman_ford(
        graph: &WeightedDirGraphStEphFloat<usize>,
        source: usize,
    ) -> Result<SSSPResultStEphFloat, String> {
        let n = graph.vertices().size();

        // Initialize distances: source = 0, others = infinity
        let mut distances = HashMap::<usize, OrderedF64>::new();
        for v in 0..n {
            distances.insert(
                v,
                if v == source {
                    OrderedF64::from(0.0)
                } else {
                    OrderedF64::from(f64::INFINITY)
                },
            );
        }

        // Iterate up to |V| rounds
        for round in 0..n {
            let mut changed = false;
            let mut new_distances = HashMap::<usize, OrderedF64>::new();

            // For each vertex, compute minimum distance through in-neighbors (Line 5)
            for v in 0..n {
                let current_dist = *distances.get(&v).unwrap_or(&OrderedF64::from(f64::INFINITY));
                let mut min_dist = current_dist;

                // Compute Din(v) = min over u in N⁻(v) of (d[u] + w(u,v))
                let in_neighbors = graph.in_neighbors_weighted(&v);
                for Pair(u, weight) in in_neighbors.iter() {
                    let u_dist = *distances.get(u).unwrap_or(&OrderedF64::from(f64::INFINITY));
                    if u_dist.0 != f64::INFINITY {
                        let new_dist = OrderedF64::from(u_dist.0 + weight.0);
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
                let mut result = SSSPResultStEphFloat::new(n, source);
                for v in 0..n {
                    let dist = *distances.get(&v).unwrap_or(&OrderedF64::from(f64::INFINITY));
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
    /// - APAS: N/A — Verus-specific scaffolding
    /// - Claude-Opus-4.6: Work O(n + m), Span O(n + m) — sequential scan over vertices and in-neighbors
    fn reconstruct_predecessors(
        graph: &WeightedDirGraphStEphFloat<usize>,
        distances: &HashMap<usize, OrderedF64>,
        result: &mut SSSPResultStEphFloat,
        source: usize,
    ) {
        let n = graph.vertices().size();
        for v in 0..n {
            if v == source {
                continue;
            }

            let v_dist = *distances.get(&v).unwrap_or(&OrderedF64::from(f64::INFINITY));
            if v_dist.0 == f64::INFINITY {
                continue; // Unreachable
            }

            // Find which in-neighbor u gave us the shortest path
            let in_neighbors = graph.in_neighbors_weighted(&v);
            for Pair(u, weight) in in_neighbors.iter() {
                let u_dist = *distances.get(u).unwrap_or(&OrderedF64::from(f64::INFINITY));
                if u_dist.0 != f64::INFINITY {
                    let path_dist = OrderedF64::from(u_dist.0 + weight.0);
                    if path_dist == v_dist {
                        result.set_predecessor(v, *u);
                        break; // Found the predecessor
                    }
                }
            }
        }
    }
}
