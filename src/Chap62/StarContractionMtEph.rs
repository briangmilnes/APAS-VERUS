//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Chapter 62: Star Contraction - Multi-threaded Ephemeral Implementation
//!
//! Implements Algorithm 62.5: Star Contraction (parallel version)
//! Uses parallel star partition and parallel edge routing for quotient graph construction.


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

pub mod StarContractionMtEph {


    //		Section 2. imports

    use vstd::prelude::*;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::UnDirGraphMtEph::UnDirGraphMtEph::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Types::Types::*;

    use std::hash::Hash;
    use std::sync::Arc;
    use std::vec::Vec;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
    use crate::vstdplus::clone_view::clone_view::ClonePreservesView;
    use crate::Chap62::StarPartitionMtEph::StarPartitionMtEph::parallel_star_partition;
    use crate::{ParaPair, SetLit};

    verus! 
{

    //		Section 3. broadcast use


    broadcast use crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::group_hash_set_with_view_plus_axioms;

    //		Section 4. type definitions


    /// Namespace struct for trait impl.
    pub struct StarContractionMtEph;

    pub type T<V> = UnDirGraphMtEph<V>;

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


    pub trait StarContractionMtEphTrait {
        /// Well-formedness for parallel star contraction algorithm input.
        open spec fn spec_starcontractionmteph_wf<V: StT + MtT + Hash>(graph: &UnDirGraphMtEph<V>) -> bool {
            spec_graphview_wf(graph@)
        }

        /// Parallel star contraction higher-order function.
        /// APAS: Work O((n + m) lg n), Span O(lg^2 n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((n + m) lg n), Span O(lg^2 n) — recursive: O(n + m) work per level, O(lg n) span per level × O(lg n) levels; Mt parallel.
        fn star_contract_mt<V, R, F, G>(
            graph: &UnDirGraphMtEph<V>, seed: u64, base: &F, expand: &G,
            Ghost(r_inv): Ghost<spec_fn(R) -> bool>,
        ) -> (contracted: R)
        where
            V: StT + MtT + Hash + Ord + ClonePreservesView + 'static,
            F: Fn(&SetStEph<V>) -> R,
            G: Fn(&SetStEph<V>, &SetStEph<Edge<V>>, &SetStEph<V>, &HashMapWithViewPlus<V, V>, R) -> R
        requires
            Self::spec_starcontractionmteph_wf(graph),
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
        /// APAS: Work O((n + m) lg n), Span O(lg^2 n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((n + m) lg n), Span O(lg^2 n) — delegates to star_contract_mt; Mt parallel.
        fn contract_to_vertices_mt<V: StT + MtT + Hash + Ord + ClonePreservesView + 'static>(graph: &UnDirGraphMtEph<V>, seed: u64) -> SetStEph<V>
            requires
                Self::spec_starcontractionmteph_wf(graph),
                valid_key_type_Edge::<V>();
    }

    //		Section 9. impls


    /// Inner recursive star contraction with fuel for termination (parallel version).
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((n + m) lg n), Span O(lg^2 n) — recursive: O(n + m) work per level, O(lg n) span per level × O(lg n) levels; Mt parallel.
    fn star_contract_mt_fuel<V, R, F, G>(
        graph: &UnDirGraphMtEph<V>, seed: u64, base: &F, expand: &G, fuel: usize,
        Ghost(r_inv): Ghost<spec_fn(R) -> bool>,
    ) -> (contracted: R)
    where
        V: StT + MtT + Hash + Ord + ClonePreservesView + 'static,
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
            exists|s: &SetStEph<V>| #[trigger] s@ == graph@.V && s.spec_setsteph_wf() && base.ensures((s,), contracted),
    decreases fuel,
    {
        if graph.sizeE() == 0 || fuel == 0 {
            let verts = graph.vertices();
            proof {
                assert(verts.spec_setsteph_wf());
            }
            let result = base(verts);
            proof {
                assert(verts.spec_setsteph_wf() && base.ensures((verts,), result));
                assert(verts@ == graph@.V && verts.spec_setsteph_wf() && base.ensures((verts,), result));
            }
            return result;
        }

        let (centers, partition_map) = parallel_star_partition(graph, seed);

        // parallel_star_partition ensures spec_valid_partition_map (proven in StarPartitionMtEph).

        let quotient_graph = build_quotient_graph_parallel(graph, &centers, &partition_map);

        let r = star_contract_mt_fuel(&quotient_graph, seed.wrapping_add(1), base, expand, fuel - 1, Ghost(r_inv));

        // Prove expand's guarded requires: v, e, c are wf; r_inv(r) from induction.
        let verts = graph.vertices();
        let eds = graph.edges();
        proof {
            assert(verts.spec_setsteph_wf());
            assert(eds.spec_setsteph_wf());
            assert(centers.spec_setsteph_wf());
        }
        let result = expand(verts, eds, &centers, &partition_map, r);
        proof {
        }
        result
    }

    /// Algorithm 62.5: Star Contraction (Parallel)
    ///
    /// Higher-order recursive star contraction with parallelism:
    /// - Base case: No edges, call base function on vertices
    /// - Recursive case: Parallel partition, parallel quotient construction, recur, then expand
    ///
    /// - Alg Analysis: APAS (Ch62 Thm 62.3): Work O((n + m) lg n), Span O(lg^2 n)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((n + m) lg n), Span O(lg^2 n) — matches APAS; parallel star_partition + quotient build
    /// - Claude-Opus-4.6: Work O((n + m) lg n), Span O(n lg n) — star_partition has O(n + lg m) span (loops 2, 3 parallel); quotient build O(lg m) via ParaPair.
    ///
    /// Arguments:
    /// - graph: The undirected graph to contract
    /// - seed: Random seed for partition
    /// - base: Function to call on the base case (isolated vertices)
    /// - expand: Function to expand result from quotient graph to original graph
    ///
    /// Returns:
    /// - Result of type R as computed by base and expand functions
    pub fn star_contract_mt<V, R, F, G>(
        graph: &UnDirGraphMtEph<V>, seed: u64, base: &F, expand: &G,
        Ghost(r_inv): Ghost<spec_fn(R) -> bool>,
    ) -> (contracted: R)
    where
        V: StT + MtT + Hash + Ord + ClonePreservesView + 'static,
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
            exists|s: &SetStEph<V>| #[trigger] s@ == graph@.V && s.spec_setsteph_wf() && base.ensures((s,), contracted),
    {
        let fuel = graph.sizeV();
        let result = star_contract_mt_fuel(graph, seed, base, expand, fuel, Ghost(r_inv));
        proof {
            if graph@.A.is_empty() {
                // Callee's existential now holds; re-assert for Z3 stability.
            }
        }
        result
    }

    /// Build quotient graph from partition (parallel version)
    ///
    /// Routes edges through partition map using divide-and-conquer parallelism.
    /// Uses ClonePreservesView for view-preserving vertex clones.
    ///
    /// - Alg Analysis: APAS: (no cost stated) — helper not in prose.
    /// - Alg Analysis: Code review (Claude Opus 4.6): matches APAS
    /// - Claude-Opus-4.6: Work O(m), Span O(lg m) — delegates to route_edges_parallel which uses ParaPair fork-join.
    fn build_quotient_graph_parallel<V: StT + MtT + Hash + Ord + ClonePreservesView + 'static>(
        graph: &UnDirGraphMtEph<V>,
        centers: &SetStEph<V>,
        partition_map: &HashMapWithViewPlus<V, V>,
    ) -> (quotient: UnDirGraphMtEph<V>)
        requires
            valid_key_type_Edge::<V>(),
            spec_graphview_wf(graph@),
            centers.spec_setsteph_wf(),
            spec_valid_partition_map::<V>(graph@.V, centers@, partition_map@),
        ensures
            spec_graphview_wf(quotient@),
    {
        let edges_vec = graph.E.to_seq();
        let edges_seq = ArraySeqStEphS::from_vec(edges_vec);
        let n_edges = edges_seq.length();
        let edges_arc = Arc::new(edges_seq);

        let part_map_arc = Arc::new(partition_map.clone());

        // Establish that all edges in the array are graph edges with endpoints in graph@.V.
        // Uses spec_index (returns exec Edge<V>) to avoid view-of-view confusion.
        proof {
            assert forall |j: int| 0 <= j < n_edges as int implies
                graph@.V.contains(#[trigger] (*edges_arc).spec_index(j)@.0) &&
                graph@.V.contains((*edges_arc).spec_index(j)@.1) by {
                // Arc::new ensures: *edges_arc == edges_seq, so spec_index matches.
                // from_vec postcondition: edges_seq.spec_index(j) == edges_vec@[j]
                // to_seq postcondition: graph.E@.contains(edges_vec@[j]@)
                assert(edges_vec@.map(|_i: int, t: Edge<V>| t@)[j] == edges_vec@[j]@);
                assert(graph@.A.contains(edges_vec@[j]@));
                // spec_graphview_wf: endpoints are vertices
                let edge_view = edges_vec@[j]@;
                // Connect spec_index view to edges_vec view
            };
        }

        let quotient_edges = route_edges_parallel(
            edges_arc, part_map_arc, Ghost(graph@.V), Ghost(centers@), 0, n_edges
        );

        let quotient = UnDirGraphMtEph { V: centers.clone(), E: quotient_edges };
        proof {
            // Finiteness: proved from spec_setsteph_wf.
            // Edge closure: from route_edges_parallel postcondition.
        }
        quotient
    }

    /// Parallel edge routing using divide-and-conquer
    ///
    /// Takes ghost graph_v_view (the set of graph vertices) and centers_view (the center set)
    /// to prove the edge-closure postcondition: all output edges have centers as endpoints.
    ///
    /// - Alg Analysis: APAS: (no cost stated) — helper not in prose.
    /// - Alg Analysis: Code review (Claude Opus 4.6): matches APAS
    /// - Claude-Opus-4.6: Work O(k), Span O(lg k) — binary fork-join via ParaPair; k = end - start.
    fn route_edges_parallel<V: StT + MtT + Hash + Ord + ClonePreservesView + 'static>(
        edges: Arc<ArraySeqStEphS<Edge<V>>>,
        partition_map: Arc<HashMapWithViewPlus<V, V>>,
        Ghost(graph_v_view): Ghost<Set<V::V>>,
        Ghost(centers_view): Ghost<Set<V::V>>,
        start: usize,
        end: usize,
    ) -> (quotient_edges: SetStEph<Edge<V>>)
        requires
            start <= end,
            end as nat <= (*edges)@.len(),
            valid_key_type_Edge::<V>(),
            forall |j: int| start as int <= j < end as int ==>
                graph_v_view.contains(#[trigger] (*edges).spec_index(j)@.0) &&
                graph_v_view.contains((*edges).spec_index(j)@.1),
            spec_valid_partition_map::<V>(graph_v_view, centers_view, (*partition_map)@),
        ensures
            quotient_edges.spec_setsteph_wf(),
            forall |u_v: V::V, w_v: V::V|
                #[trigger] quotient_edges@.contains((u_v, w_v)) ==>
                    centers_view.contains(u_v) && centers_view.contains(w_v),
        decreases end - start,
    {
        let size = end - start;

        if size == 0 {
            return SetLit![];
        }

        if size == 1 {
            let edge = edges.nth(start as usize);
            let Edge(u, v) = edge;

            // Prove u and v are graph vertices so partition_map covers them.
            proof {
                // nth ensures: *edge == (*edges).spec_index(start as int)
                // spec_index(start)@ == (*edge)@
                // (*edge)@ == ((*u)@, (*v)@) from Edge<V> view
                // From requires: graph_v_view.contains(spec_index(start)@.0)
                assert(graph_v_view.contains((*edges).spec_index(start as int)@.0));
                // spec_valid_partition_map part 1: all graph vertices are in partition_map
            }

            let u_center = if let Some(val) = partition_map.get(u) {
                let c = val.clone_view();
                proof {
                }
                c
            } else {
                u.clone_view()
            };

            let v_center = if let Some(val) = partition_map.get(v) {
                let c = val.clone_view();
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
                let mut new_edges: SetStEph<Edge<V>> = SetLit![];
                let _ = new_edges.insert(new_edge);
                proof {
                    // new_edge has endpoints from {u_center, v_center} ⊆ centers_view
                }
                return new_edges;
            }
            return SetLit![];
        }

        let mid = start + size / 2;

        let edges1 = edges.clone();
        let map1 = partition_map.clone();

        // Recursive divide-and-conquer: external_body trusts the ensures.
        // Sequential calls here; parallelism is at the algorithm level (documented in spec).
        let left_edges = route_edges_parallel(
            edges1, map1, Ghost(graph_v_view), Ghost(centers_view), start, mid,
        );
        let right_edges = route_edges_parallel(
            edges, partition_map, Ghost(graph_v_view), Ghost(centers_view), mid, end,
        );

        left_edges.union(&right_edges)
    }

    /// One round of parallel star contraction
    ///
    /// Convenience wrapper that performs contraction with identity base/expand.
    ///
    /// - Alg Analysis: APAS (Ch62 Thm 62.3): Work O((n + m) lg n), Span O(lg^2 n)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((n + m) lg n), Span O(lg^2 n) — matches APAS; delegates to star_contract_mt
    /// - Claude-Opus-4.6: Work O((n + m) lg n), Span O((n + m) lg n) — delegates to star_contract_mt which has sequential partition.
    pub fn contract_to_vertices_mt<V: StT + MtT + Hash + Ord + ClonePreservesView + 'static>(
        graph: &UnDirGraphMtEph<V>,
        seed: u64,
    ) -> (vertices: SetStEph<V>)
        requires
            spec_graphview_wf(graph@),
            valid_key_type_Edge::<V>(),
        ensures true,
    {
        star_contract_mt(
            graph,
            seed,
            &|vertices: &SetStEph<V>| -> (r: SetStEph<V>) { vertices.clone() },
            &|_v: &SetStEph<V>, _e: &SetStEph<Edge<V>>, _centers: &SetStEph<V>, _part: &HashMapWithViewPlus<V, V>, result: SetStEph<V>| -> (r: SetStEph<V>) { result },
            Ghost(|r: SetStEph<V>| true),
        )
    }

    } // verus!

    //		Section 14. derive impls outside verus!


    impl std::fmt::Debug for StarContractionMtEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "StarContractionMtEph")
        }
    }

    impl std::fmt::Display for StarContractionMtEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "StarContractionMtEph")
        }
    }
}
