//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Chapter 62: Star Contraction - Sequential Ephemeral Implementation
//!
//! Implements Algorithm 62.5: Star Contraction (sequential version)
//! A higher-order function that recursively contracts a graph using star partitions.


//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 8. traits
//	Section 9. impls
//	Section 14. derive impls outside verus!

//		Section 1. module

pub mod StarContractionStEph {


    //		Section 2. imports

    use vstd::prelude::*;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::UnDirGraphStEph::UnDirGraphStEph::*;
    use crate::Types::Types::*;

    use std::hash::Hash;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
    use crate::vstdplus::clone_view::clone_view::ClonePreservesView;
    use crate::Chap62::StarPartitionStEph::StarPartitionStEph::sequential_star_partition;
    #[cfg(verus_keep_ghost)]
    use crate::Chap62::StarPartitionStEph::StarPartitionStEph::spec_valid_partition_map;
    use crate::SetLit;

    verus! 
{

    //		Section 3. broadcast use


    broadcast use crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::group_hash_set_with_view_plus_axioms;

    //		Section 4. type definitions


    /// Namespace struct for trait impl.
    pub struct StarContractionStEph;

    pub type T<V> = UnDirGraphStEph<V>;

    //		Section 8. traits


    pub trait StarContractionStEphTrait {
        /// Well-formedness for star contraction algorithm input.
        open spec fn spec_starcontractionsteph_wf<V: StT + Hash>(graph: &UnDirGraphStEph<V>) -> bool {
            spec_graphview_wf(graph@)
        }

        /// Sequential star contraction higher-order function.
        /// APAS: Work O((n + m) lg n), Span O((n + m) lg n)
        /// - Alg Analysis: APAS (Ch62 Thm 62.3): Work O((n + m) lg n), Span O(lg^2 n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((n + m) lg n), Span O((n + m) lg n) — DIFFERS: sequential recursive contraction, span = work
        fn star_contract<V, R, F, G>(
            graph: &UnDirGraphStEph<V>, base: &F, expand: &G,
            Ghost(r_inv): Ghost<spec_fn(R) -> bool>,
        ) -> (contracted: R)
        where
            V: HashOrd,
            F: Fn(&SetStEph<V>) -> R,
            G: Fn(&SetStEph<V>, &SetStEph<Edge<V>>, &SetStEph<V>, &HashMapWithViewPlus<V, V>, R) -> R
        requires
            Self::spec_starcontractionsteph_wf(graph),
            valid_key_type_Edge::<V>(),
            forall|s: &SetStEph<V>| s.spec_setsteph_wf() ==> #[trigger] base.requires((s,)),
            forall|s: &SetStEph<V>, r: R| s.spec_setsteph_wf() && base.ensures((s,), r) ==> r_inv(r),
            forall|v: &SetStEph<V>, e: &SetStEph<Edge<V>>, c: &SetStEph<V>, p: &HashMapWithViewPlus<V, V>, r: R|
                v.spec_setsteph_wf() && e.spec_setsteph_wf() && c.spec_setsteph_wf() && r_inv(r)
                ==> #[trigger] expand.requires((v, e, c, p, r)),
            forall|v: &SetStEph<V>, e: &SetStEph<Edge<V>>, c: &SetStEph<V>, p: &HashMapWithViewPlus<V, V>, r: R, out: R|
                #[trigger] expand.ensures((v, e, c, p, r), out) ==> r_inv(out),
        ensures r_inv(contracted);

        /// Contract graph to just vertices (no edges).
        /// APAS: Work O((n + m) lg n), Span O((n + m) lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((n + m) lg n), Span O((n + m) lg n) — recursive star contraction halving vertices; St sequential.
        fn contract_to_vertices<V: HashOrd>(graph: &UnDirGraphStEph<V>) -> SetStEph<V>
            requires
                Self::spec_starcontractionsteph_wf(graph),
                valid_key_type_Edge::<V>();
    }

    //		Section 9. impls


    /// Inner recursive star contraction with fuel for termination.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((n + m) lg n), Span O((n + m) lg n) — recursive: O(n + m) per level × O(lg n) levels; St sequential.
    fn star_contract_fuel<V, R, F, G>(
        graph: &UnDirGraphStEph<V>, base: &F, expand: &G, fuel: usize,
        Ghost(r_inv): Ghost<spec_fn(R) -> bool>,
    ) -> (contracted: R)
    where
        V: HashOrd,
        F: Fn(&SetStEph<V>) -> R,
        G: Fn(&SetStEph<V>, &SetStEph<Edge<V>>, &SetStEph<V>, &HashMapWithViewPlus<V, V>, R) -> R,
    requires
        spec_graphview_wf(graph@),
        valid_key_type_Edge::<V>(),
        forall|s: &SetStEph<V>| s.spec_setsteph_wf() ==> #[trigger] base.requires((s,)),
        forall|s: &SetStEph<V>, r: R| s.spec_setsteph_wf() && base.ensures((s,), r) ==> r_inv(r),
        forall|v: &SetStEph<V>, e: &SetStEph<Edge<V>>, c: &SetStEph<V>, p: &HashMapWithViewPlus<V, V>, r: R|
            v.spec_setsteph_wf() && e.spec_setsteph_wf() && c.spec_setsteph_wf() && r_inv(r)
            ==> #[trigger] expand.requires((v, e, c, p, r)),
        forall|v: &SetStEph<V>, e: &SetStEph<Edge<V>>, c: &SetStEph<V>, p: &HashMapWithViewPlus<V, V>, r: R, out: R|
            #[trigger] expand.ensures((v, e, c, p, r), out) ==> r_inv(out),
    ensures
        r_inv(contracted),
        (graph@.A.is_empty() || fuel == 0) ==>
            exists|s: &SetStEph<V>| s@ == graph@.V && #[trigger] s.spec_setsteph_wf() && base.ensures((s,), contracted),
    decreases fuel,
    {
        if graph.sizeE() == 0 || fuel == 0 {
            let verts = graph.vertices();
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                assert(verts.spec_setsteph_wf());
            }
            let result = base(verts);
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                assert(verts.spec_setsteph_wf() && base.ensures((verts,), result));
                // Veracity: NEEDED assert
                assert(verts@ == graph@.V && verts.spec_setsteph_wf() && base.ensures((verts,), result));
            }
            return result;
        }

        let (centers, partition_map) = sequential_star_partition(graph);

        // spec_valid_partition_map follows from sequential_star_partition's ensures:
        // graph.V@ == graph@.V, result.0@ == centers@, result.1@ == partition_map@.

        let quotient_graph = build_quotient_graph(graph, &centers, &partition_map);

        let r = star_contract_fuel(&quotient_graph, base, expand, fuel - 1, Ghost(r_inv));

        // Prove expand's guarded requires: v, e, c are wf; r_inv(r) from induction.
        let verts = graph.vertices();
        let eds = graph.edges();
        // Veracity: NEEDED proof block
        proof {
            // Veracity: NEEDED assert
            assert(verts.spec_setsteph_wf());
            // Veracity: NEEDED assert
            assert(eds.spec_setsteph_wf());
            // Veracity: NEEDED assert
            assert(centers.spec_setsteph_wf());
        }
        let result = expand(verts, eds, &centers, &partition_map, r);
        // Veracity: NEEDED proof block
        proof {
        }
        result
    }

    /// Algorithm 62.5: Star Contraction (Sequential)
    ///
    /// Higher-order recursive star contraction:
    /// - Base case: No edges, call base function on vertices
    /// - Recursive case: Partition graph, build quotient graph, recur, then expand
    ///
    /// - Alg Analysis: APAS (Ch62 Thm 62.3): Work O((n + m) lg n), Span O(lg^2 n)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((n + m) lg n), Span O((n + m) lg n) — DIFFERS: sequential recursive contraction, span = work
    ///
    /// Arguments:
    /// - graph: The undirected graph to contract
    /// - base: Function to call on the base case (isolated vertices)
    /// - expand: Function to expand result from quotient graph to original graph
    ///
    /// Returns:
    /// - Result of type R as computed by base and expand functions
    pub fn star_contract<V, R, F, G>(
        graph: &UnDirGraphStEph<V>, base: &F, expand: &G,
        Ghost(r_inv): Ghost<spec_fn(R) -> bool>,
    ) -> (contracted: R)
    where
        V: HashOrd,
        F: Fn(&SetStEph<V>) -> R,
        G: Fn(&SetStEph<V>, &SetStEph<Edge<V>>, &SetStEph<V>, &HashMapWithViewPlus<V, V>, R) -> R,
    requires
        spec_graphview_wf(graph@),
        valid_key_type_Edge::<V>(),
        forall|s: &SetStEph<V>| s.spec_setsteph_wf() ==> #[trigger] base.requires((s,)),
        forall|s: &SetStEph<V>, r: R| s.spec_setsteph_wf() && base.ensures((s,), r) ==> r_inv(r),
        forall|v: &SetStEph<V>, e: &SetStEph<Edge<V>>, c: &SetStEph<V>, p: &HashMapWithViewPlus<V, V>, r: R|
            v.spec_setsteph_wf() && e.spec_setsteph_wf() && c.spec_setsteph_wf() && r_inv(r)
            ==> #[trigger] expand.requires((v, e, c, p, r)),
        forall|v: &SetStEph<V>, e: &SetStEph<Edge<V>>, c: &SetStEph<V>, p: &HashMapWithViewPlus<V, V>, r: R, out: R|
            #[trigger] expand.ensures((v, e, c, p, r), out) ==> r_inv(out),
    ensures
        r_inv(contracted),
        graph@.A.is_empty() ==>
            exists|s: &SetStEph<V>| s@ == graph@.V && #[trigger] s.spec_setsteph_wf() && base.ensures((s,), contracted),
    {
        let fuel = graph.sizeV();
        let result = star_contract_fuel(graph, base, expand, fuel, Ghost(r_inv));
        // Veracity: NEEDED proof block
        proof {
            if graph@.A.is_empty() {
                // Callee's existential now holds; re-assert for Z3 stability.
            }
        }
        result
    }

    /// Build quotient graph from partition.
    ///
    /// Routes edges through partition map, removing self-loops.
    /// Uses ClonePreservesView for view-preserving vertex clones.
    ///
    /// - Alg Analysis: Code review (Claude Opus 4.6): matches APAS
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m), Span O(m) — sequential loop over all edges.
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
            // Veracity: NEEDED proof block
            proof {
                // edge_vec comes from graph.E.to_seq(), so element i's view is in graph.E@.
                // By Seq::map definition: map(f)[i] == f(i, seq[i]) == seq[i]@
                // Veracity: NEEDED assert
                assert(edge_vec@.map(|_j: int, t: Edge<V>| t@)[i as int] == edge_vec@[i as int]@);
                // Contains: the element at index i witnesses the existential.
                // to_seq postcondition: graph.E@.contains(x) <==> edge_vec@.map(f).contains(x)
                // graph@.A == graph.E@ (from UnDirGraphStEph::view)
                // Edge(u, v)@ == (u@, v@) from Edge<V>::view
                // Veracity: NEEDED assert
                assert(graph@.A.contains(((*u)@, (*v)@)));
                // spec_graphview_wf: arc endpoints are vertices
                // spec_valid_partition_map part 1: every graph vertex is in partition_map
            }

            // Resolve u's center using if-let so val stays in scope for the proof.
            let u_center = if let Some(val) = partition_map.get(u) {
                let c = val.clone_view();
                // Veracity: NEEDED proof block
                proof {
                    // get ensures: *val == partition_map@[u@] (Some branch)
                    // clone_view ensures: c@ == (*val)@
                    // Combined: c@ == partition_map@[u@]@
                    // spec_valid_partition_map part 2: mapped value's view is in centers@
                }
                c
            } else {
                // None contradicts partition_map@.contains_key(u@) proved above.
                u.clone_view()
            };

            // Resolve v's center using the same pattern.
            let v_center = if let Some(val) = partition_map.get(v) {
                let c = val.clone_view();
                // Veracity: NEEDED proof block
                proof {
                }
                c
            } else {
                v.clone_view()
            };

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
    /// - Alg Analysis: APAS (Ch62 Thm 62.3): Work O((n + m) lg n), Span O((n + m) lg n)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((n + m) lg n), Span O((n + m) lg n); delegates to star_contract
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((n + m) lg n), Span O((n + m) lg n) — agrees with APAS.
    pub fn contract_to_vertices<V: HashOrd>(graph: &UnDirGraphStEph<V>) -> (vertices: SetStEph<V>)
        requires
            spec_graphview_wf(graph@),
            valid_key_type_Edge::<V>(),
        ensures true,
    {
        star_contract(
            graph,
            &|vertices: &SetStEph<V>| -> (r: SetStEph<V>) { vertices.clone() },
            &|_v: &SetStEph<V>, _e: &SetStEph<Edge<V>>, _centers: &SetStEph<V>, _part: &HashMapWithViewPlus<V, V>, result: SetStEph<V>| -> (r: SetStEph<V>) { result },
            Ghost(|r: SetStEph<V>| true),
        )
    }

    } // verus!

    //		Section 14. derive impls outside verus!


    impl std::fmt::Debug for StarContractionStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "StarContractionStEph")
        }
    }

    impl std::fmt::Display for StarContractionStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "StarContractionStEph")
        }
    }
}
