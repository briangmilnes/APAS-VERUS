//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 62: Star Contraction - Sequential Ephemeral Implementation
//!
//! Implements Algorithm 62.5: Star Contraction (sequential version)
//! A higher-order function that recursively contracts a graph using star partitions.

pub mod StarContractionStEph {

    use vstd::prelude::*;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::UnDirGraphStEph::UnDirGraphStEph::*;
    use crate::Types::Types::*;

    use std::hash::Hash;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
    use crate::vstdplus::clone_view::clone_view::ClonePreservesView;
    use crate::Chap62::StarPartitionStEph::StarPartitionStEph::sequential_star_partition;
    use crate::Chap62::StarPartitionStEph::StarPartitionStEph::spec_valid_partition_map;
    use crate::SetLit;

    verus! {

    // 3. broadcast use

    broadcast use crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::group_hash_set_with_view_plus_axioms;

    // 4. type definitions

    /// Namespace struct for trait impl.
    pub struct StarContractionStEph;

    // 8. traits

    pub trait StarContractionStEphTrait {
        /// Well-formedness for star contraction algorithm input.
        open spec fn spec_starcontractionsteph_wf<V: StT + Hash>(graph: &UnDirGraphStEph<V>) -> bool {
            spec_graphview_wf(graph@)
        }

        /// Sequential star contraction higher-order function.
        /// APAS: Work O((n + m) lg n), Span O((n + m) lg n)
        fn star_contract<V, R, F, G>(graph: &UnDirGraphStEph<V>, base: &F, expand: &G) -> R
        where
            V: HashOrd,
            F: Fn(&SetStEph<V>) -> R,
            G: Fn(&SetStEph<V>, &SetStEph<Edge<V>>, &SetStEph<V>, &HashMapWithViewPlus<V, V>, R) -> R
        requires
            Self::spec_starcontractionsteph_wf(graph),
            valid_key_type_Edge::<V>(),
            forall|s: &SetStEph<V>| s.spec_setsteph_wf() ==> #[trigger] base.requires((s,)),
            forall|v: &SetStEph<V>, e: &SetStEph<Edge<V>>, c: &SetStEph<V>, p: &HashMapWithViewPlus<V, V>, r: R|
                #[trigger] expand.requires((v, e, c, p, r));

        /// Contract graph to just vertices (no edges).
        /// APAS: Work O((n + m) lg n), Span O((n + m) lg n)
        fn contract_to_vertices<V: HashOrd>(graph: &UnDirGraphStEph<V>) -> SetStEph<V>
            requires
                Self::spec_starcontractionsteph_wf(graph),
                valid_key_type_Edge::<V>();
    }

    pub type T<V> = UnDirGraphStEph<V>;

    /// Inner recursive star contraction with fuel for termination.
    fn star_contract_fuel<V, R, F, G>(
        graph: &UnDirGraphStEph<V>, base: &F, expand: &G, fuel: usize,
    ) -> R
    where
        V: HashOrd,
        F: Fn(&SetStEph<V>) -> R,
        G: Fn(&SetStEph<V>, &SetStEph<Edge<V>>, &SetStEph<V>, &HashMapWithViewPlus<V, V>, R) -> R,
    requires
        spec_graphview_wf(graph@),
        valid_key_type_Edge::<V>(),
        forall|s: &SetStEph<V>| s.spec_setsteph_wf() ==> #[trigger] base.requires((s,)),
        forall|v: &SetStEph<V>, e: &SetStEph<Edge<V>>, c: &SetStEph<V>, p: &HashMapWithViewPlus<V, V>, r: R|
            #[trigger] expand.requires((v, e, c, p, r)),
    decreases fuel,
    {
        if graph.sizeE() == 0 || fuel == 0 {
            let verts = graph.vertices();
            proof {
                assert(verts@.finite());
                assert(verts.spec_setsteph_wf());
            }
            return base(verts);
        }

        let (centers, partition_map) = sequential_star_partition(graph);

        // spec_valid_partition_map follows from sequential_star_partition's ensures:
        // graph.V@ == graph@.V, result.0@ == centers@, result.1@ == partition_map@.
        proof { assert(spec_valid_partition_map::<V>(graph@.V, centers@, partition_map@)); }

        let quotient_graph = build_quotient_graph(graph, &centers, &partition_map);

        let r = star_contract_fuel(&quotient_graph, base, expand, fuel - 1);

        expand(graph.vertices(), graph.edges(), &centers, &partition_map, r)
    }

    /// Algorithm 62.5: Star Contraction (Sequential)
    ///
    /// Higher-order recursive star contraction:
    /// - Base case: No edges, call base function on vertices
    /// - Recursive case: Partition graph, build quotient graph, recur, then expand
    ///
    /// - APAS: Work O((n + m) lg n), Span O((n + m) lg n)
    /// - Claude-Opus-4.6: Work O((n + m) lg n), Span O((n + m) lg n) — agrees with APAS.
    ///
    /// Arguments:
    /// - graph: The undirected graph to contract
    /// - base: Function to call on the base case (isolated vertices)
    /// - expand: Function to expand result from quotient graph to original graph
    ///
    /// Returns:
    /// - Result of type R as computed by base and expand functions
    pub fn star_contract<V, R, F, G>(graph: &UnDirGraphStEph<V>, base: &F, expand: &G) -> R
    where
        V: HashOrd,
        F: Fn(&SetStEph<V>) -> R,
        G: Fn(&SetStEph<V>, &SetStEph<Edge<V>>, &SetStEph<V>, &HashMapWithViewPlus<V, V>, R) -> R,
    requires
        spec_graphview_wf(graph@),
        valid_key_type_Edge::<V>(),
        forall|s: &SetStEph<V>| s.spec_setsteph_wf() ==> #[trigger] base.requires((s,)),
        forall|v: &SetStEph<V>, e: &SetStEph<Edge<V>>, c: &SetStEph<V>, p: &HashMapWithViewPlus<V, V>, r: R|
            #[trigger] expand.requires((v, e, c, p, r)),
    {
        star_contract_fuel(graph, base, expand, graph.sizeV())
    }

    /// Build quotient graph from partition.
    ///
    /// Routes edges through partition map, removing self-loops.
    /// Uses ClonePreservesView for view-preserving vertex clones.
    ///
    /// - APAS: (no cost stated) — helper not in prose.
    /// - Claude-Opus-4.6: Work O(m), Span O(m) — sequential loop over all edges.
    fn build_quotient_graph<V: HashOrd>(
        graph: &UnDirGraphStEph<V>,
        centers: &SetStEph<V>,
        partition_map: &HashMapWithViewPlus<V, V>,
    ) -> (quotient: UnDirGraphStEph<V>)
        requires
            valid_key_type_Edge::<V>(),
            spec_graphview_wf(graph@),
            centers.spec_setsteph_wf(),
            spec_valid_partition_map::<V>(graph@.V, centers@, partition_map@),
        ensures
            spec_graphview_wf(quotient@),
    {
        let mut quotient_edges: SetStEph<Edge<V>> = SetLit![];
        let edge_vec = graph.E.to_seq();
        let n = edge_vec.len();
        let mut i: usize = 0;
        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        while i < n
            invariant
                valid_key_type_Edge::<V>(),
                quotient_edges.spec_setsteph_wf(),
                i <= n,
                n == edge_vec@.len(),
                // Every edge in quotient_edges has both endpoints in centers.
                forall |u_v: V::V, w_v: V::V|
                    #[trigger] quotient_edges@.contains((u_v, w_v)) ==>
                        centers@.contains(u_v) && centers@.contains(w_v),
                // Partition map properties flow from outer scope.
                spec_valid_partition_map::<V>(graph@.V, centers@, partition_map@),
                spec_graphview_wf(graph@),
            decreases n - i,
        {
            let edge = &edge_vec[i];
            let Edge(u, v) = edge;

            // Prove u and v are in the graph's vertex set.
            proof {
                // edge_vec comes from graph.E.to_seq(), so element i's view is in graph.E@.
                // By Seq::map definition: map(f)[i] == f(i, seq[i]) == seq[i]@
                assert(edge_vec@.map(|_j: int, t: Edge<V>| t@)[i as int] == edge_vec@[i as int]@);
                // Contains: the element at index i witnesses the existential.
                assert(edge_vec@.map(|_j: int, t: Edge<V>| t@).contains(edge_vec@[i as int]@));
                // to_seq postcondition: graph.E@.contains(x) <==> edge_vec@.map(f).contains(x)
                assert(graph.E@.contains(edge_vec@[i as int]@));
                // graph@.A == graph.E@ (from UnDirGraphStEph::view)
                // Edge(u, v)@ == (u@, v@) from Edge<V>::view
                assert(edge_vec@[i as int]@ == ((*u)@, (*v)@));
                assert(graph@.A.contains(((*u)@, (*v)@)));
                // spec_graphview_wf: arc endpoints are vertices
                assert(graph@.V.contains((*u)@));
                assert(graph@.V.contains((*v)@));
                // spec_valid_partition_map part 1: every graph vertex is in partition_map
                assert(partition_map@.contains_key((*u)@));
                assert(partition_map@.contains_key((*v)@));
            }

            // Resolve u's center using if-let so val stays in scope for the proof.
            let u_center = if let Some(val) = partition_map.get(u) {
                let c = val.clone_view();
                proof {
                    // get ensures: *val == partition_map@[u@] (Some branch)
                    assert(*val == partition_map@[(*u)@]);
                    // clone_view ensures: c@ == (*val)@
                    assert(c@ == (*val)@);
                    // Combined: c@ == partition_map@[u@]@
                    assert(c@ == partition_map@[(*u)@]@);
                    // spec_valid_partition_map part 2: mapped value's view is in centers@
                    assert(centers@.contains(c@));
                }
                c
            } else {
                // None contradicts partition_map@.contains_key(u@) proved above.
                proof { assert(false); }
                u.clone_view()
            };
            proof { assert(centers@.contains(u_center@)); }

            // Resolve v's center using the same pattern.
            let v_center = if let Some(val) = partition_map.get(v) {
                let c = val.clone_view();
                proof {
                    assert(*val == partition_map@[(*v)@]);
                    assert(c@ == (*val)@);
                    assert(c@ == partition_map@[(*v)@]@);
                    assert(centers@.contains(c@));
                }
                c
            } else {
                proof { assert(false); }
                v.clone_view()
            };
            proof { assert(centers@.contains(v_center@)); }

            if u_center != v_center {
                let new_edge = if u_center < v_center {
                    Edge(u_center, v_center)
                } else {
                    Edge(v_center, u_center)
                };
                let _ = quotient_edges.insert(new_edge);
            }
            i = i + 1;
        }

        let quotient = UnDirGraphStEph { V: centers.clone(), E: quotient_edges };
        quotient
    }

    /// One round of sequential star contraction
    ///
    /// Convenience wrapper that performs contraction with identity base/expand.
    ///
    /// - APAS: Work O((n + m) lg n), Span O((n + m) lg n)
    /// - Claude-Opus-4.6: Work O((n + m) lg n), Span O((n + m) lg n) — agrees with APAS.
    pub fn contract_to_vertices<V: HashOrd>(graph: &UnDirGraphStEph<V>) -> (result: SetStEph<V>)
        requires
            spec_graphview_wf(graph@),
            valid_key_type_Edge::<V>(),
        ensures true,
    {
        star_contract(
            graph,
            &|vertices: &SetStEph<V>| -> (r: SetStEph<V>) { vertices.clone() },
            &|_v: &SetStEph<V>, _e: &SetStEph<Edge<V>>, _centers: &SetStEph<V>, _part: &HashMapWithViewPlus<V, V>, result: SetStEph<V>| -> (r: SetStEph<V>) { result },
        )
    }

    } // verus!
}
