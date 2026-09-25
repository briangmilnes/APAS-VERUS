// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
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

    use std::collections::HashMap;
    use std::hash::Hash;
    use crate::vstdplus::clone_view::clone_view::ClonePreservesView;
    use crate::vstdplus::feq::feq::feq;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::hash_specs_plus::hash_specs_plus::key_view;
    use crate::Chap62::StarPartitionStEph::StarPartitionStEph::sequential_star_partition;
    #[cfg(verus_keep_ghost)]
    use crate::Chap62::StarPartitionStEph::StarPartitionStEph::spec_valid_partition_map;
    use crate::SetLit;

    verus!
{

    //		Section 3. broadcast use


    broadcast use {
        vstd::std_specs::hash::group_hash_axioms,
        crate::vstdplus::hash_specs_plus::hash_specs_plus::group_key_view_lemmas,
    };

    //		Section 4. type definitions


    /// Namespace struct for trait impl.
    #[derive(Debug)]
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((n + m) lg n), Span O((n + m) lg n) — ACCEPTED DIFFERENCE: sequential recursive contraction, span = work
        fn star_contract<V, R, F, G>(
            graph: &UnDirGraphStEph<V>, base: &F, expand: &G,
            Ghost(r_inv): Ghost<spec_fn(R) -> bool>,
        ) -> (contracted: R)
        where
            V: HashOrd,
            F: Fn(&SetStEph<V>) -> R,
            G: Fn(&SetStEph<V>, &SetStEph<Edge<V>>, &SetStEph<V>, &HashMap<V, V>, R) -> R
        requires
            Self::spec_starcontractionsteph_wf(graph),
            valid_key_type_Edge::<V>(),
            forall|s: &SetStEph<V>| s.spec_setsteph_wf() ==> #[trigger] base.requires((s,)),
            forall|s: &SetStEph<V>, r: R| s.spec_setsteph_wf() && base.ensures((s,), r) ==> r_inv(r),
            forall|v: &SetStEph<V>, e: &SetStEph<Edge<V>>, c: &SetStEph<V>, p: &HashMap<V, V>, r: R|
                v.spec_setsteph_wf() && e.spec_setsteph_wf() && c.spec_setsteph_wf() && r_inv(r)
                ==> #[trigger] expand.requires((v, e, c, p, r)),
            forall|v: &SetStEph<V>, e: &SetStEph<Edge<V>>, c: &SetStEph<V>, p: &HashMap<V, V>, r: R, out: R|
                #[trigger] expand.ensures((v, e, c, p, r), out) ==> r_inv(out),
        ensures
            r_inv(contracted),
            graph@.A.is_empty() ==>
                exists|s: &SetStEph<V>| s@ == graph@.V && #[trigger] s.spec_setsteph_wf() && base.ensures((s,), contracted),
            (exists|s: &SetStEph<V>| s@ == graph@.V && #[trigger] s.spec_setsteph_wf() && base.ensures((s,), contracted))
            || (exists|v: &SetStEph<V>, e: &SetStEph<Edge<V>>, c: &SetStEph<V>, p: &HashMap<V, V>, r: R|
                    #[trigger] expand.ensures((v, e, c, p, r), contracted)
                    && v@ == graph@.V && e@ == graph@.A
                    && v.spec_setsteph_wf() && e.spec_setsteph_wf() && c.spec_setsteph_wf()
                    && spec_valid_partition_map::<V>(graph@.V, c@, key_view(p@)));

        /// Contract graph to just vertices (no edges).
        /// APAS: Work O((n + m) lg n), Span O((n + m) lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((n + m) lg n), Span O((n + m) lg n) — recursive star contraction halving vertices; St sequential.
        fn contract_to_vertices<V: HashOrd>(graph: &UnDirGraphStEph<V>) -> SetStEph<V>
            requires
                Self::spec_starcontractionsteph_wf(graph),
                valid_key_type_Edge::<V>();
    }

    //		Section 9. impls


    /// Inner recursive star contraction.
    ///
    /// Every round removes at least one vertex, so the recursion terminates on
    /// |V|. The greedy partition removes a vertex whenever a non-loop edge
    /// exists, but its ensures do not state this, so the round checks it; a
    /// round without progress contracts one non-loop edge
    /// (`single_edge_partition`). A graph whose edges are all self-loops is a
    /// base case.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((n + m) lg n), Span O((n + m) lg n) — recursive: O(n + m) per level × O(lg n) levels; St sequential.
    fn star_contract_rec<V, R, F, G>(
        graph: &UnDirGraphStEph<V>, base: &F, expand: &G,
        Ghost(r_inv): Ghost<spec_fn(R) -> bool>,
    ) -> (contracted: R)
    where
        V: HashOrd,
        F: Fn(&SetStEph<V>) -> R,
        G: Fn(&SetStEph<V>, &SetStEph<Edge<V>>, &SetStEph<V>, &HashMap<V, V>, R) -> R,
    requires
        spec_graphview_wf(graph@),
        valid_key_type_Edge::<V>(),
        forall|s: &SetStEph<V>| s.spec_setsteph_wf() ==> #[trigger] base.requires((s,)),
        forall|s: &SetStEph<V>, r: R| s.spec_setsteph_wf() && base.ensures((s,), r) ==> r_inv(r),
        forall|v: &SetStEph<V>, e: &SetStEph<Edge<V>>, c: &SetStEph<V>, p: &HashMap<V, V>, r: R|
            v.spec_setsteph_wf() && e.spec_setsteph_wf() && c.spec_setsteph_wf() && r_inv(r)
            ==> #[trigger] expand.requires((v, e, c, p, r)),
        forall|v: &SetStEph<V>, e: &SetStEph<Edge<V>>, c: &SetStEph<V>, p: &HashMap<V, V>, r: R, out: R|
            #[trigger] expand.ensures((v, e, c, p, r), out) ==> r_inv(out),
    ensures
        r_inv(contracted),
        graph@.A.is_empty() ==>
            exists|s: &SetStEph<V>| s@ == graph@.V && #[trigger] s.spec_setsteph_wf() && base.ensures((s,), contracted),
        (exists|s: &SetStEph<V>| s@ == graph@.V && #[trigger] s.spec_setsteph_wf() && base.ensures((s,), contracted))
        || (exists|v: &SetStEph<V>, e: &SetStEph<Edge<V>>, c: &SetStEph<V>, p: &HashMap<V, V>, r: R|
                #[trigger] expand.ensures((v, e, c, p, r), contracted)
                && v@ == graph@.V && e@ == graph@.A
                && v.spec_setsteph_wf() && e.spec_setsteph_wf() && c.spec_setsteph_wf()
                && spec_valid_partition_map::<V>(graph@.V, c@, key_view(p@))),
    decreases graph@.V.len(),
    {
        if graph.sizeE() == 0 {
            let verts = graph.vertices();
            // Veracity: NEEDED proof block
            // Veracity: NEEDED proof block (speed hint)
            proof {
                // Veracity: NEEDED assert
                assert(verts.spec_setsteph_wf());
            }
            let result = base(verts);
            // Veracity: NEEDED proof block
            // Veracity: NEEDED proof block
            proof {
                // Veracity: NEEDED assert
                assert(verts.spec_setsteph_wf() && base.ensures((verts,), result));
                // Veracity: NEEDED assert
                assert(verts@ == graph@.V && verts.spec_setsteph_wf() && base.ensures((verts,), result));
            }
            return result;
        }

        let (greedy_centers, greedy_map) = sequential_star_partition(graph);

        // spec_valid_partition_map follows from sequential_star_partition's ensures:
        // graph.V@ == graph@.V, result.0@ == centers@, result.1@ == partition_map@.
        // Its ensures do not state progress, so check it; without progress, contract one edge.
        let (centers, partition_map) = if greedy_centers.size() < graph.sizeV() {
            (greedy_centers, greedy_map)
        } else {
            match find_non_loop_edge(graph) {
                Some((u, v)) => single_edge_partition(graph, &u, &v),
                None => {
                    // Every edge is a self-loop: no edge joins two vertices.
                    let verts = graph.vertices();
                    proof { assert(verts.spec_setsteph_wf()); }
                    let result = base(verts);
                    proof {
                        assert(verts@ == graph@.V && verts.spec_setsteph_wf() && base.ensures((verts,), result));
                    }
                    return result;
                },
            }
        };

        let quotient_graph = build_quotient_graph(graph, &centers, &partition_map);

        let r = star_contract_rec(&quotient_graph, base, expand, Ghost(r_inv));

        // Prove expand's guarded requires: v, e, c are wf; r_inv(r) from induction.
        let verts = graph.vertices();
        // Veracity: NEEDED proof block
        let eds = graph.edges();
        // Veracity: NEEDED proof block
        proof {
            // Veracity: NEEDED assert
            assert(verts.spec_setsteph_wf());
            // Veracity: NEEDED assert
            assert(eds.spec_setsteph_wf());
            // Veracity: NEEDED assert
            assert(centers.spec_setsteph_wf());
        // Veracity: NEEDED proof block
        }
        let ghost quotient_result = r;
        let result = expand(verts, eds, &centers, &partition_map, r);
        // Veracity: NEEDED proof block
        proof {
            assert(expand.ensures((verts, eds, &centers, &partition_map, quotient_result), result)
                && verts@ == graph@.V && eds@ == graph@.A
                && verts.spec_setsteph_wf() && eds.spec_setsteph_wf() && centers.spec_setsteph_wf()
                && spec_valid_partition_map::<V>(graph@.V, centers@, key_view(partition_map@)));
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
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((n + m) lg n), Span O((n + m) lg n) — ACCEPTED DIFFERENCE: sequential recursive contraction, span = work
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
        G: Fn(&SetStEph<V>, &SetStEph<Edge<V>>, &SetStEph<V>, &HashMap<V, V>, R) -> R,
    requires
        spec_graphview_wf(graph@),
        valid_key_type_Edge::<V>(),
        forall|s: &SetStEph<V>| s.spec_setsteph_wf() ==> #[trigger] base.requires((s,)),
        forall|s: &SetStEph<V>, r: R| s.spec_setsteph_wf() && base.ensures((s,), r) ==> r_inv(r),
        forall|v: &SetStEph<V>, e: &SetStEph<Edge<V>>, c: &SetStEph<V>, p: &HashMap<V, V>, r: R|
            v.spec_setsteph_wf() && e.spec_setsteph_wf() && c.spec_setsteph_wf() && r_inv(r)
            ==> #[trigger] expand.requires((v, e, c, p, r)),
        forall|v: &SetStEph<V>, e: &SetStEph<Edge<V>>, c: &SetStEph<V>, p: &HashMap<V, V>, r: R, out: R|
            #[trigger] expand.ensures((v, e, c, p, r), out) ==> r_inv(out),
    ensures
        r_inv(contracted),
        graph@.A.is_empty() ==>
            exists|s: &SetStEph<V>| s@ == graph@.V && #[trigger] s.spec_setsteph_wf() && base.ensures((s,), contracted),
        (exists|s: &SetStEph<V>| s@ == graph@.V && #[trigger] s.spec_setsteph_wf() && base.ensures((s,), contracted))
        || (exists|v: &SetStEph<V>, e: &SetStEph<Edge<V>>, c: &SetStEph<V>, p: &HashMap<V, V>, r: R|
                #[trigger] expand.ensures((v, e, c, p, r), contracted)
                && v@ == graph@.V && e@ == graph@.A
                && v.spec_setsteph_wf() && e.spec_setsteph_wf() && c.spec_setsteph_wf()
                && spec_valid_partition_map::<V>(graph@.V, c@, key_view(p@))),
    // Veracity: NEEDED proof block
    {
        let result = star_contract_rec(graph, base, expand, Ghost(r_inv));
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
        partition_map: &HashMap<V, V>,
    ) -> (quotient: UnDirGraphStEph<V>)
        requires
            valid_key_type_Edge::<V>(),
            spec_graphview_wf(graph@),
            centers.spec_setsteph_wf(),
            spec_valid_partition_map::<V>(graph@.V, centers@, key_view(partition_map@)),
        ensures
            spec_graphview_wf(quotient@),
            quotient@.V == centers@,
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
                spec_valid_partition_map::<V>(graph@.V, centers@, key_view(partition_map@)),
                spec_graphview_wf(graph@),
            decreases n - i,
        {
            // Veracity: NEEDED proof block
            let edge = &edge_vec[i];
            let Edge(u, v) = edge;

            // Prove u and v are in the graph's vertex set.
            // Veracity: NEEDED proof block
            proof {
                // edge_vec comes from graph.E.to_seq(), so element i's view is in graph.E@.
                // By Seq::map definition: map(f)[i] == f(i, seq[i]) == seq[i]@
                // Veracity: NEEDED assert
                // Veracity: NEEDED assert
                assert(edge_vec@.map(|_j: int, t: Edge<V>| t@)[i as int] == edge_vec@[i as int]@);
                // Contains: the element at index i witnesses the existential.
                // to_seq postcondition: graph.E@.contains(x) <==> edge_vec@.map(f).contains(x)
                // graph@.A == graph.E@ (from UnDirGraphStEph::view)
                // Edge(u, v)@ == (u@, v@) from Edge<V>::view
                // Veracity: NEEDED assert
                // Veracity: NEEDED assert
                assert(graph@.A.contains(((*u)@, (*v)@)));
                // spec_graphview_wf: arc endpoints are vertices
                // spec_valid_partition_map part 1: every graph vertex is in partition_map
            // Veracity: NEEDED proof block
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
                // Veracity: NEEDED proof block
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

    /// Find an edge whose endpoints differ, or report that every edge is a self-loop.
    ///
    /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(m), Span O(m) — sequential scan of the edge set.
    pub(crate) fn find_non_loop_edge<V: HashOrd>(
        graph: &UnDirGraphStEph<V>,
    ) -> (found: Option<(V, V)>)
        requires
            valid_key_type_Edge::<V>(),
            spec_graphview_wf(graph@),
        ensures
            found matches Some((u, v)) ==> graph@.A.contains((u@, v@)) && u@ != v@,
    {
        proof { assert(graph.E.spec_setsteph_wf()); }
        let edge_vec = graph.E.to_seq();
        let ne = edge_vec.len();
        let ghost mapped_edges = edge_vec@.map(|_i: int, t: Edge<V>| t@);
        let mut i: usize = 0;
        while i < ne
            invariant
                valid_key_type_Edge::<V>(),
                i <= ne,
                ne == edge_vec@.len(),
                mapped_edges == edge_vec@.map(|_i: int, t: Edge<V>| t@),
                forall|x: (V::V, V::V)| graph@.A.contains(x) <==> #[trigger] mapped_edges.contains(x),
            decreases ne - i,
        {
            let Edge(a, b) = &edge_vec[i];
            if !feq(a, b) {
                proof {
                    assert(mapped_edges[i as int] == edge_vec@[i as int]@);
                    assert(mapped_edges.contains(edge_vec@[i as int]@));
                }
                return Some((a.clone_view(), b.clone_view()));
            }
            i = i + 1;
        }
        None
    }

    /// Partition that contracts the single edge (u, v): v joins the star of
    /// u, and every other vertex is its own center. Removes exactly one vertex.
    ///
    /// - Alg Analysis: Code review (Claude Opus 5.5): Work O(n), Span O(n) — one sequential pass over the vertices.
    pub(crate) fn single_edge_partition<V: HashOrd>(
        graph: &UnDirGraphStEph<V>,
        u: &V,
        v: &V,
    ) -> (partition: (SetStEph<V>, HashMap<V, V>))
        requires
            valid_key_type_Edge::<V>(),
            spec_graphview_wf(graph@),
            graph@.V.contains(u@),
            graph@.V.contains(v@),
            u@ != v@,
        ensures
            partition.0.spec_setsteph_wf(),
            spec_valid_partition_map::<V>(graph@.V, partition.0@, key_view(partition.1@)),
            partition.0@.len() < graph@.V.len(),
    {
        proof { assert(graph.V.spec_setsteph_wf()); }
        let vert_vec = graph.V.to_seq();
        let nv = vert_vec.len();
        let ghost mapped = vert_vec@.map(|_i: int, t: V| t@);
        let mut centers: SetStEph<V> = SetLit![];
        let mut partition_map: HashMap<V, V> = HashMap::new();
        let mut i: usize = 0;
        while i < nv
            invariant
                valid_key_type_Edge::<V>(),
                centers.spec_setsteph_wf(),
                i <= nv,
                nv == vert_vec@.len(),
                mapped == vert_vec@.map(|_i: int, t: V| t@),
                forall|x: V::V| graph@.V.contains(x) <==> #[trigger] mapped.contains(x),
                // Centers are graph vertices other than v.
                forall|x: V::V| #[trigger] centers@.contains(x) ==> graph@.V.contains(x) && x != v@,
                // Every processed vertex is a key.
                forall|j: int| 0 <= j < i ==> #[trigger] key_view(partition_map@).contains_key(vert_vec@[j]@),
                // Every processed vertex other than v is a center.
                forall|j: int| 0 <= j < i && vert_vec@[j]@ != v@ ==> #[trigger] centers@.contains(vert_vec@[j]@),
                // Every value is a center or u.
                forall|x: V::V| #[trigger] key_view(partition_map@).contains_key(x) ==>
                    centers@.contains(key_view(partition_map@)[x]@) || key_view(partition_map@)[x]@ == u@,
            decreases nv - i,
        {
            let w = &vert_vec[i];
            proof {
                assert(mapped[i as int] == vert_vec@[i as int]@);
                assert(mapped.contains(vert_vec@[i as int]@));
            }
            if feq(w, v) {
                partition_map.insert(w.clone_view(), u.clone_view());
            } else {
                let _ = centers.insert(w.clone_view());
                partition_map.insert(w.clone_view(), w.clone_view());
            }
            i = i + 1;
        }
        proof {
            // u is a graph vertex, so it was processed; u != v, so it is a center.
            assert(mapped.contains(u@));
            let ju = choose|j: int| 0 <= j < mapped.len() && mapped[j] == u@;
            assert(vert_vec@[ju]@ == u@);
            assert(centers@.contains(vert_vec@[ju]@));
            // Part A: every graph vertex is a key.
            assert forall|x: V::V| #[trigger] graph@.V.contains(x) implies
                key_view(partition_map@).contains_key(x) by {
                assert(mapped.contains(x));
                let j = choose|j: int| 0 <= j < mapped.len() && mapped[j] == x;
                assert(vert_vec@[j]@ == x);
            };
            // Centers lie in V minus v, which has |V| - 1 elements.
            assert(centers@.subset_of(graph@.V.remove(v@)));
            vstd::set_lib::lemma_len_subset(centers@, graph@.V.remove(v@));
        }
        (centers, partition_map)
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
            &|_v: &SetStEph<V>, _e: &SetStEph<Edge<V>>, _centers: &SetStEph<V>, _part: &HashMap<V, V>, result: SetStEph<V>| -> (r: SetStEph<V>) { result },
            Ghost(|r: SetStEph<V>| true),
        )
    }

    } // verus!

    //		Section 14. derive impls outside verus!



    impl std::fmt::Display for StarContractionStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "StarContractionStEph")
        }
    }
}
