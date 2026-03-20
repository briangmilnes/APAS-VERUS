//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 62: Star Partition - Multi-threaded Ephemeral Implementation
//!
//! Implements Algorithm 62.3: Parallel Star Partition using randomized coin flips.
//! Uses Seq.inject for efficient parallel updates.

pub mod StarPartitionMtEph {

    use vstd::prelude::*;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::UnDirGraphMtEph::UnDirGraphMtEph::*;
    use crate::Types::Types::*;

    use std::hash::Hash;
    #[cfg(not(verus_keep_ghost))]
    use std::vec::Vec;
    #[cfg(not(verus_keep_ghost))]
    use crate::SetLit;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
    use crate::vstdplus::rand::rand::{seeded_rng, random_bool_seeded};

    verus! {

    // 4. type definitions

    /// Namespace struct for trait impl.
    pub struct StarPartitionMtEph;

    // 8. traits

    pub trait StarPartitionMtEphTrait {
        /// Well-formedness for parallel star partition algorithm input.
        open spec fn spec_starpartitionmteph_wf<V: StT + MtT + Hash>(graph: &UnDirGraphMtEph<V>) -> bool {
            spec_graphview_wf(graph@)
        }

        /// Parallel star partition using randomized coin flips.
        /// APAS: Work O(|V| + |E|), Span O(lg |V|)
        fn parallel_star_partition<V: StT + MtT + Hash + Ord + 'static>(
            graph: &UnDirGraphMtEph<V>,
            seed: u64,
        ) -> (SetStEph<V>, HashMapWithViewPlus<V, V>)
            requires Self::spec_starpartitionmteph_wf(graph);
    }

    #[cfg(not(verus_keep_ghost))]
    pub type T<V> = UnDirGraphMtEph<V>;

    /// Algorithm 62.3: Parallel Star Partition.
    ///
    /// - APAS: Work O(n + m), Span O(lg n)
    /// - Claude-Opus-4.6: Work O(n + m), Span O(n + m) — all loops sequential.
    #[verifier::external_body]
    #[cfg(not(verus_keep_ghost))]
    pub fn parallel_star_partition<V: StT + MtT + Hash + Ord + 'static>(
        graph: &UnDirGraphMtEph<V>,
        seed: u64,
    ) -> (SetStEph<V>, HashMapWithViewPlus<V, V>) {
        let vertices_vec = graph.vertices().iter().cloned().collect::<Vec<V>>();
        let n = vertices_vec.len() as N;

        let mut vertex_to_index = HashMapWithViewPlus::<V, N>::new();
        for (i, v) in vertices_vec.iter().enumerate() {
            vertex_to_index.insert(v.clone(), i as N);
        }

        let mut rng = seeded_rng(seed);
        let mut coin_flips = HashMapWithViewPlus::<V, bool>::new();
        for vertex in vertices_vec.iter() {
            coin_flips.insert(vertex.clone(), random_bool_seeded(&mut rng));
        }

        let mut th_edges = Vec::<(N, V)>::new();
        for edge in graph.edges().iter() {
            let Edge(u, v) = edge;
            let u_heads = coin_flips.get(u).copied().unwrap_or(false);
            let v_heads = coin_flips.get(v).copied().unwrap_or(false);

            if !u_heads && v_heads {
                if let Some(&u_idx) = vertex_to_index.get(u) {
                    th_edges.push((u_idx, v.clone()));
                }
            }
            if !v_heads && u_heads {
                if let Some(&v_idx) = vertex_to_index.get(v) {
                    th_edges.push((v_idx, u.clone()));
                }
            }
        }

        let mut p_vec: Vec<V> = vertices_vec.clone();
        for (idx, vertex) in th_edges {
            p_vec[idx as usize] = vertex;
        }

        let mut centers: SetStEph<V> = SetLit![];
        let mut partition_map = HashMapWithViewPlus::<V, V>::new();

        for (i, vertex) in vertices_vec.iter().enumerate() {
            let center = p_vec[i].clone();
            partition_map.insert(vertex.clone(), center.clone());

            if *vertex == center {
                let _ = centers.insert(vertex.clone());
            }
        }

        (centers, partition_map)
    }

    } // verus!
}
