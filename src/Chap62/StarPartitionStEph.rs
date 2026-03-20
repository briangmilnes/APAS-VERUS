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
    use crate::SetLit;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;

    verus! {

    // 3. broadcast use

    broadcast use crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::group_hash_set_with_view_plus_axioms;

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

    pub type T<V> = UnDirGraphStEph<V>;

    /// Sequential Star Partition using greedy selection.
    ///
    /// - APAS: Work Θ(n + m), Span Θ(n + m)
    /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) — agrees with APAS.
    pub fn sequential_star_partition<V: HashOrd>(graph: &UnDirGraphStEph<V>) -> (result: (SetStEph<V>, HashMapWithViewPlus<V, V>))
        requires
            spec_graphview_wf(graph@),
            valid_key_type_Edge::<V>(),
        ensures
            result.0.spec_setsteph_wf(),
    {
        let mut partition_map = HashMapWithViewPlus::<V, V>::new();
        let mut centers: SetStEph<V> = SetLit![];
        let mut processed: SetStEph<V> = SetLit![];

        let vert_vec = graph.V.to_seq();
        let edge_vec = graph.E.to_seq();
        let nv = vert_vec.len();
        let ne = edge_vec.len();

        let mut vi: usize = 0;
        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        while vi < nv
            invariant
                valid_key_type_Edge::<V>(),
                centers.spec_setsteph_wf(),
                processed.spec_setsteph_wf(),
                vi <= nv,
                nv == vert_vec@.len(),
                ne == edge_vec@.len(),
            decreases nv - vi,
        {
            let vertex = &vert_vec[vi];

            if !processed.mem(vertex) {
                let _ = centers.insert(vertex.clone());
                partition_map.insert(vertex.clone(), vertex.clone());
                let _ = processed.insert(vertex.clone());

                let mut ei: usize = 0;
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                while ei < ne
                    invariant
                        valid_key_type_Edge::<V>(),
                        processed.spec_setsteph_wf(),
                        centers.spec_setsteph_wf(),
                        ei <= ne,
                        ne == edge_vec@.len(),
                    decreases ne - ei,
                {
                    let edge = &edge_vec[ei];
                    let Edge(a, b) = edge;
                    if a.clone() == vertex.clone() {
                        if !processed.mem(b) {
                            partition_map.insert(b.clone(), vertex.clone());
                            let _ = processed.insert(b.clone());
                        }
                    } else if b.clone() == vertex.clone() {
                        if !processed.mem(a) {
                            partition_map.insert(a.clone(), vertex.clone());
                            let _ = processed.insert(a.clone());
                        }
                    }
                    ei = ei + 1;
                }
            }

            vi = vi + 1;
        }

        (centers, partition_map)
    }

    } // verus!
}
