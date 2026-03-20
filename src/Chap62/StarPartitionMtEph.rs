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
    use std::vec::Vec;
    use crate::SetLit;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
    use crate::vstdplus::rand::rand::{seeded_rng, random_bool_seeded};

    verus! {

    // 3. broadcast use

    broadcast use crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::group_hash_set_with_view_plus_axioms;

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

    pub type T<V> = UnDirGraphMtEph<V>;

    /// Algorithm 62.3: Parallel Star Partition.
    ///
    /// - APAS: Work O(n + m), Span O(lg n)
    /// - Claude-Opus-4.6: Work O(n + m), Span O(n + m) — all loops sequential.
    pub fn parallel_star_partition<V: StT + MtT + Hash + Ord + 'static>(
        graph: &UnDirGraphMtEph<V>,
        seed: u64,
    ) -> (result: (SetStEph<V>, HashMapWithViewPlus<V, V>))
        requires valid_key_type_Edge::<V>(),
        ensures true,
    {
        let vertices_vec = graph.V.to_seq();
        let nv = vertices_vec.len();

        // Build vertex-to-index map.
        let mut vertex_to_index = HashMapWithViewPlus::<V, N>::new();
        let mut i: usize = 0;
        while i < nv
            invariant i <= nv, nv == vertices_vec@.len(),
            decreases nv - i,
        {
            vertex_to_index.insert(vertices_vec[i].clone(), i as N);
            i = i + 1;
        }

        // Flip coins for each vertex.
        let mut rng = seeded_rng(seed);
        let mut coin_flips = HashMapWithViewPlus::<V, bool>::new();
        let mut j: usize = 0;
        while j < nv
            invariant j <= nv, nv == vertices_vec@.len(),
            decreases nv - j,
        {
            coin_flips.insert(vertices_vec[j].clone(), random_bool_seeded(&mut rng));
            j = j + 1;
        }

        // Build tail-heads edges: for each edge (u,v), if u is tails and v is heads,
        // record that u should map to v.
        let edge_vec = graph.E.to_seq();
        let ne = edge_vec.len();
        let mut th_edges: Vec<(N, V)> = Vec::new();
        let mut k: usize = 0;
        while k < ne
            invariant k <= ne, ne == edge_vec@.len(),
            decreases ne - k,
        {
            let edge = &edge_vec[k];
            let Edge(u, v) = edge;
            let u_heads = match coin_flips.get(u) {
                Some(val) => *val,
                None => false,
            };
            let v_heads = match coin_flips.get(v) {
                Some(val) => *val,
                None => false,
            };

            if !u_heads && v_heads {
                match vertex_to_index.get(u) {
                    Some(u_idx) => { th_edges.push((*u_idx, v.clone())); },
                    None => {},
                }
            }
            if !v_heads && u_heads {
                match vertex_to_index.get(v) {
                    Some(v_idx) => { th_edges.push((*v_idx, u.clone())); },
                    None => {},
                }
            }
            k = k + 1;
        }

        // Build partition: start with each vertex mapping to itself,
        // then apply tail-heads edges.
        let mut p_vec: Vec<V> = Vec::new();
        let mut m: usize = 0;
        while m < nv
            invariant m <= nv, nv == vertices_vec@.len(),
            decreases nv - m,
        {
            p_vec.push(vertices_vec[m].clone());
            m = m + 1;
        }

        let nth = th_edges.len();
        let mut t: usize = 0;
        while t < nth
            invariant
                t <= nth,
                nth == th_edges@.len(),
                p_vec@.len() == nv,
            decreases nth - t,
        {
            let (idx, ref vertex) = th_edges[t];
            if (idx as usize) < nv {
                p_vec.set(idx as usize, vertex.clone());
            }
            t = t + 1;
        }

        // Build centers and partition map.
        let mut centers: SetStEph<V> = SetLit![];
        let mut partition_map = HashMapWithViewPlus::<V, V>::new();
        let mut q: usize = 0;
        while q < nv
            invariant
                valid_key_type_Edge::<V>(),
                centers.spec_setsteph_wf(),
                q <= nv,
                nv == vertices_vec@.len(),
                p_vec@.len() == nv,
            decreases nv - q,
        {
            let vertex = &vertices_vec[q];
            let center = p_vec[q].clone();
            partition_map.insert(vertex.clone(), center.clone());

            if *vertex == center {
                let _ = centers.insert(vertex.clone());
            }
            q = q + 1;
        }

        (centers, partition_map)
    }

    } // verus!
}
