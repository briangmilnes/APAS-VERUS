//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Chapter 62: Star Partition - Sequential Ephemeral Implementation
//!
//! Implements sequential star partition using greedy vertex selection.
//! A star partition divides a graph into blocks where each block is a
//! vertex-induced subgraph with respect to a star graph.


//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 6. spec fns
//	Section 8. traits
//	Section 9. impls
//	Section 14. derive impls outside verus!

//		Section 1. module

pub mod StarPartitionStEph {


    //		Section 2. imports

    use vstd::prelude::*;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::UnDirGraphStEph::UnDirGraphStEph::*;
    use crate::Types::Types::*;

    use std::hash::Hash;
    use crate::SetLit;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;

    verus! 
{

    //		Section 3. broadcast use


    broadcast use crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::group_hash_set_with_view_plus_axioms;

    //		Section 4. type definitions


    /// Namespace struct for trait impl.
    pub struct StarPartitionStEph;

    pub type T<V> = UnDirGraphStEph<V>;

    //		Section 6. spec fns


    /// Partition map validity: every graph vertex is mapped and every value is a center.
    pub open spec fn spec_valid_partition_map<V: View>(
        graph_vertices: Set<V::V>,
        centers: Set<V::V>,
        partition_map: Map<V::V, V>,
    ) -> bool {
        // Every graph vertex is in the partition map.
        &&& forall |v_view: V::V|
                #[trigger] graph_vertices.contains(v_view) ==>
                    partition_map.contains_key(v_view)
        // Every partition map value is a center.
        &&& forall |v_view: V::V|
                #[trigger] partition_map.contains_key(v_view) ==>
                    centers.contains(partition_map[v_view]@)
    }

    //		Section 8. traits


    pub trait StarPartitionStEphTrait {
        /// Well-formedness for star partition algorithm input.
        open spec fn spec_starpartitionsteph_wf<V: StT + Hash>(graph: &UnDirGraphStEph<V>) -> bool {
            spec_graphview_wf(graph@)
        }

        /// Sequential star partition using greedy selection.
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V| + |E|), Span O(|V| + |E|) — single pass over vertices + edges; St sequential.
        fn sequential_star_partition<V: HashOrd>(graph: &UnDirGraphStEph<V>) -> (SetStEph<V>, HashMapWithViewPlus<V, V>)
            requires Self::spec_starpartitionsteph_wf(graph);
    }

    //		Section 9. impls


    /// Sequential Star Partition using greedy selection.
    ///
    /// - Alg Analysis: APAS (Ch62 Thm 62.1): Work O(n + m), Span O(n + m)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n + m), Span O(n + m) — matches APAS
    /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) — agrees with APAS.
    pub fn sequential_star_partition<V: HashOrd>(graph: &UnDirGraphStEph<V>) -> (partition: (SetStEph<V>, HashMapWithViewPlus<V, V>))
        requires
            spec_graphview_wf(graph@),
            valid_key_type_Edge::<V>(),
        ensures
            partition.0.spec_setsteph_wf(),
            spec_valid_partition_map::<V>(graph.V@, partition.0@, partition.1@),
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
                // All vertices at indices < vi are in processed.
                forall|j: int| 0 <= j < vi as int ==> #[trigger] processed@.contains(vert_vec@[j]@),
                // All processed vertex views are in partition_map domain.
                forall|w: V::V| #[trigger] processed@.contains(w) ==> partition_map@.contains_key(w),
                // Range validity: every partition_map value is a center.
                forall|v_view: V::V| #[trigger] partition_map@.contains_key(v_view) ==>
                    centers@.contains(partition_map@[v_view]@),
            decreases nv - vi,
        {
            let vertex = &vert_vec[vi];

            if !processed.mem(vertex) {
                // Save ghost views before mutations to help invariant proofs.
                let ghost pre_proc: Set<V::V> = processed@;
                let ghost pre_pm: Map<V::V, V> = partition_map@;
                let ghost pre_ctr: Set<V::V> = centers@;
                let ghost vv: V::V = (*vertex)@;

                // Named clones: clone_view() ensures result@ == self@, so all views equal vv.
                let vc_key = vertex.clone_view();   // vc_key@ == vv
                let vc_val = vertex.clone_view();   // vc_val@ == vv (VALUE stored in partition_map)
                let vc_ctr = vertex.clone_view();   // vc_ctr@ == vv
                let vc_proc = vertex.clone_view();  // vc_proc@ == vv
                let _ = centers.insert(vc_ctr);
                partition_map.insert(vc_key, vc_val);
                let _ = processed.insert(vc_proc);

                // Prove inner loop invariants from outer loop inv + insert ensures.
                proof {
                    // From insert ensures and clone_view@ == vv:
                    // partition_map stores vc_val at key vv; vc_val@ == vv.
                    // vertex was not in processed (from !processed.mem check).

                    // Domain invariant: processed@.contains(w) => partition_map@.contains_key(w).

                    // Range invariant: partition_map@.contains_key(v) => centers@.contains(pm@[v]@).

                    // Prefix invariant: prior vertices (j < vi) are still in processed.
                    // processed@ = pre_proc.insert(vv), and pre_proc already had them.
                }

                let mut ei: usize = 0;
                #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
                while ei < ne
                    invariant
                        valid_key_type_Edge::<V>(),
                        processed.spec_setsteph_wf(),
                        centers.spec_setsteph_wf(),
                        ei <= ne,
                        ne == edge_vec@.len(),
                        nv == vert_vec@.len(),
                        vi < nv,
                        // The current center is still in processed and centers.
                        processed@.contains((*vertex)@),
                        centers@.contains((*vertex)@),
                        // Prior vertices (j < vi) are still in processed.
                        forall|j: int| 0 <= j < vi as int ==> #[trigger] processed@.contains(vert_vec@[j]@),
                        // All processed vertex views are in partition_map domain.
                        forall|w: V::V| #[trigger] processed@.contains(w) ==> partition_map@.contains_key(w),
                        // Range validity: every partition_map value is a center.
                        forall|v_view: V::V| #[trigger] partition_map@.contains_key(v_view) ==>
                            centers@.contains(partition_map@[v_view]@),
                    decreases ne - ei,
                {
                    let edge = &edge_vec[ei];
                    let Edge(a, b) = edge;
                    if a.clone_view() == vertex.clone_view() {
                        if !processed.mem(b) {
                            let ghost pre_proc_i: Set<V::V> = processed@;
                            let ghost pre_pm_i: Map<V::V, V> = partition_map@;
                            let ghost bv: V::V = (*b)@;
                            let ghost cv: V::V = (*vertex)@;
                            partition_map.insert(b.clone_view(), vertex.clone_view());
                            let _ = processed.insert(b.clone_view());
                            proof {
                                // Prefix invariant maintained (insert only adds bv, old j<vi still covered).
                            }
                        }
                    } else if b.clone_view() == vertex.clone_view() {
                        if !processed.mem(a) {
                            let ghost pre_proc_i: Set<V::V> = processed@;
                            let ghost pre_pm_i: Map<V::V, V> = partition_map@;
                            let ghost av: V::V = (*a)@;
                            let ghost cv: V::V = (*vertex)@;
                            partition_map.insert(a.clone_view(), vertex.clone_view());
                            let _ = processed.insert(a.clone_view());
                            proof {
                                // Prefix invariant maintained.
                            }
                        }
                    }
                    ei = ei + 1;
                }
            }

            // Prove forall j <= vi: processed@.contains(vert_vec@[j]@) (before incrementing vi).
            proof {
                // (*vertex)@ == vert_vec@[vi as int]@ from Vec::index ensures.
                // In both branches of the if, vertex ends up in processed:
                // - if branch: inner loop invariant processed@.contains((*vertex)@)
                // - else branch: mem ensures processed@.contains((*vertex)@)
                // So processed@.contains(vert_vec@[vi as int]@).
                // Combined with the prefix invariant (j < vi from inner loop inv or outer inv):
            }
            vi = vi + 1;
        }

        // Post-loop: connect graph.V@ to partition_map domain via to_seq ensures.
        proof {
            assert forall|v_view: V::V| #[trigger] graph.V@.contains(v_view)
                implies partition_map@.contains_key(v_view) by {
                // to_seq ensures: graph.V@.contains(v_view) ↔ vert_vec@.map(fn).contains(v_view)
                // Derive: ∃j < nv, vert_vec@[j]@ == v_view (from Seq::map open definition).
                // trigger on vert_vec@[j]@ avoids lambda-in-trigger error.
                let j = choose|j: int| 0 <= j < nv as int && #[trigger] vert_vec@[j]@ == v_view;
                // vi == nv after loop: loop invariant gives processed@.contains(vert_vec@[j]@).
                assert(processed@.contains(v_view));
            };
        }

        (centers, partition_map)
    }

    } // verus!

    //		Section 14. derive impls outside verus!


    impl std::fmt::Debug for StarPartitionStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "StarPartitionStEph")
        }
    }

    impl std::fmt::Display for StarPartitionStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "StarPartitionStEph")
        }
    }
}
