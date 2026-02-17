//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Breadth-First Search - Parallel Persistent (Chapter 54).
//! Layer-by-layer parallel BFS for finding distances and reachability.
//! Work: O(|V| + |E|), Span: O(d·lg n) where d is diameter.

pub mod BFSMtPer {

    use std::sync::{Arc, Mutex};
    use std::thread;

    use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
    use crate::ParaPair;
    use crate::Types::Types::*;
    pub type T<N> = ArraySeqMtPerS<ArraySeqMtPerS<N>>;

    pub trait BFSMtPerTrait {
        /// Performs parallel BFS from source vertex s on adjacency list graph G
        /// APAS: Work O(|V| + |E|), Span O(d·lg n) where d is diameter
        fn bfs(graph: &ArraySeqMtPerS<ArraySeqMtPerS<N>>, source: N) -> ArraySeqMtPerS<N>;
    }

    const UNREACHABLE: N = N::MAX;

    /// Performs parallel BFS from source vertex s on adjacency list graph G.
    /// Graph is represented as sequence of sequences (adjacency list).
    /// Returns array where result[v] = distance if reachable, UNREACHABLE otherwise.
    /// 
    /// Parallel version: Work O(|V| + |E|), Span O(d·lg n) where d is diameter
    /// - O(d) sequential layers (unavoidable in BFS)
    /// - O(lg n) parallel processing per layer using thread::spawn
    pub fn bfs(graph: &ArraySeqMtPerS<ArraySeqMtPerS<N>>, source: N) -> ArraySeqMtPerS<N> {
        let n = graph.length();
        if source >= n {
            return ArraySeqMtPerS::tabulate(&|_| UNREACHABLE, n);
        }

        let mut distances = ArraySeqMtPerS::tabulate(&|_| UNREACHABLE, n);
        distances = ArraySeqMtPerS::update(&distances, source, 0);

        let mut current_layer = vec![source];
        let mut current_dist = 0;

        while !current_layer.is_empty() {
            // Parallel processing of current layer
            // Each thread processes a chunk of vertices, collecting neighbors and updates
            let (next_vertices, distance_updates) = 
                process_layer_parallel(graph, &distances, &current_layer, current_dist + 1);

            // Apply distance updates (ArraySeqMtPer has no ninject; sequential updates)
            for Pair(v, d) in distance_updates {
                distances = ArraySeqMtPerS::update(&distances, v, d);
            }

            current_layer = next_vertices;
            current_dist += 1;
        }

        distances
    }

    /// Process a BFS layer in parallel
    /// Returns: (next_layer_vertices, distance_updates)
    fn process_layer_parallel(
        graph: &ArraySeqMtPerS<ArraySeqMtPerS<N>>,
        distances: &ArraySeqMtPerS<N>,
        current_layer: &[N],
        next_dist: N,
    ) -> (Vec<N>, Vec<Pair<N, N>>) {
        if current_layer.is_empty() {
            return (Vec::new(), Vec::new());
        }

        if current_layer.len() == 1 {
            // Base case: single vertex, process sequentially
            let u = current_layer[0];
            let neighbors = graph.nth(u);
            let mut next_verts = Vec::new();
            let mut updates = Vec::new();

            for i in 0..neighbors.length() {
                let v = *neighbors.nth(i);
                if *distances.nth(v) == UNREACHABLE {
                    next_verts.push(v);
                    updates.push(Pair(v, next_dist));
                }
            }

            return (next_verts, updates);
        }

        // Parallel case: split layer in half and process in parallel
        let mid = current_layer.len() / 2;
        let left_layer = current_layer[..mid].to_vec();
        let right_layer = current_layer[mid..].to_vec();

        let graph_clone = graph.clone();
        let distances_clone = distances.clone();

        let handle = thread::spawn(move || {
            process_layer_parallel(&graph_clone, &distances_clone, &left_layer, next_dist)
        });

        let (right_verts, right_updates) = 
            process_layer_parallel(graph, distances, &right_layer, next_dist);

        let (left_verts, left_updates) = handle.join().unwrap();

        // Merge results
        let mut all_verts = left_verts;
        all_verts.extend(right_verts);

        let mut all_updates = left_updates;
        all_updates.extend(right_updates);

        (all_verts, all_updates)
    }
}
