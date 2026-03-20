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

    use std::hash::Hash;
    #[cfg(not(verus_keep_ghost))]
    use crate::SetLit;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;

    verus! {

    // 4. type definitions

    /// Namespace struct for trait impl.
    pub struct StarPartitionStEph;

    // 8. traits

    pub trait StarPartitionStEphTrait {
        /// Well-formedness for star partition algorithm input.
        open spec fn spec_starpartitionsteph_wf<V: StT + Hash>(graph: &UnDirGraphStEph<V>) -> bool {
            spec_graphview_wf(graph@)
        }

        /// Sequential star partition using greedy selection.
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn sequential_star_partition<V: HashOrd>(graph: &UnDirGraphStEph<V>) -> (SetStEph<V>, HashMapWithViewPlus<V, V>)
            requires Self::spec_starpartitionsteph_wf(graph);
    }

    #[cfg(not(verus_keep_ghost))]
    pub type T<V> = UnDirGraphStEph<V>;

    /// Sequential Star Partition using greedy selection.
    ///
    /// - APAS: Work Θ(n + m), Span Θ(n + m)
    /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) — agrees with APAS.
    #[verifier::external_body]
    #[cfg(not(verus_keep_ghost))]
    pub fn sequential_star_partition<V: HashOrd>(graph: &UnDirGraphStEph<V>) -> (SetStEph<V>, HashMapWithViewPlus<V, V>) {
        let mut partition_map = HashMapWithViewPlus::<V, V>::new();
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

    } // verus!
}
