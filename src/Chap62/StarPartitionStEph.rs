//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 62: Star Partition - Sequential Ephemeral Implementation
//!
//! Implements sequential star partition using greedy vertex selection.
//! A star partition divides a graph into blocks where each block is a
//! vertex-induced subgraph with respect to a star graph.

pub mod StarPartitionStEph {

    use vstd::prelude::*;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::UnDirGraphStEph::UnDirGraphStEph::*;
    use crate::Types::Types::*;

    #[cfg(not(verus_keep_ghost))]
    use std::collections::HashMap;
    use std::hash::Hash;
    #[cfg(not(verus_keep_ghost))]
    use crate::SetLit;

    verus! {
        pub trait StarPartitionStEphTrait {
            /// Sequential star partition using greedy selection
            /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
            fn sequential_star_partition<V: StT + Hash + Ord>(graph: &UnDirGraphStEph<V>) -> SetStEph<SetStEph<V>>;
        }
    } // verus!

    #[cfg(not(verus_keep_ghost))]
    pub type T<V> = UnDirGraphStEph<V>;

    /// Sequential Star Partition using greedy selection
    ///
    /// Constructs a star partition by iteratively selecting vertices:
    /// 1. Pick an arbitrary unprocessed vertex v as a star center
    /// 2. Add all neighbors of v as satellites
    /// 3. Remove v and its satellites from consideration
    /// 4. Repeat until all vertices are processed
    ///
    /// - APAS: Work Θ(n + m), Span Θ(n + m)
    /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) — agrees with APAS.
    ///
    /// Arguments:
    /// - graph: The undirected graph to partition
    ///
    /// Returns:
    /// - (centers, partition_map): Set of center vertices and mapping from each vertex to its center
    #[cfg(not(verus_keep_ghost))]
    pub fn sequential_star_partition<V: StT + Hash + Ord>(graph: &UnDirGraphStEph<V>) -> (SetStEph<V>, HashMap<V, V>) {
        let mut partition_map = HashMap::<V, V>::new();
        let mut centers: SetStEph<V> = SetLit![];
        let mut processed: SetStEph<V> = SetLit![];

        for vertex in graph.vertices().iter() {
            if processed.mem(vertex) {
                continue;
            }

            let _ = centers.insert(vertex.clone());
            let _ = partition_map.insert(vertex.clone(), vertex.clone());
            let _ = processed.insert(vertex.clone());

            let neighbors = graph.ng(vertex);
            for neighbor in neighbors.iter() {
                if !processed.mem(neighbor) {
                    let _ = partition_map.insert(neighbor.clone(), vertex.clone());
                    let _ = processed.insert(neighbor.clone());
                }
            }
        }

        (centers, partition_map)
    }
}
