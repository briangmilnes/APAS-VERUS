//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Chapter 59: Johnson's Algorithm - Single-threaded Ephemeral Float Weights
//!
//! Implements Algorithm 59.1 from the textbook.
//! All-Pairs Shortest Paths for graphs with negative weights (but no negative cycles).
//!
//! **Algorithmic Analysis:**
//! - Johnson APSP: Work O(mn log n), Span O(mn log n) where n = |V|, m = |E|
//! - Phase 1 (Bellman-Ford): Work O(nm), Span O(nm)
//! - Phase 2 (Reweighting): Work O(m), Span O(m)
//! - Phase 3 (n Dijkstras): Work O(n * m log n) = O(mn log n), Span O(mn log n) sequential


//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 8. traits
//	Section 9. impls

//		Section 1. module

pub mod JohnsonStEphF64 {


    //		Section 2. imports

    use vstd::prelude::*;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::LabDirGraphStEphTrait;
    use crate::Chap06::WeightedDirGraphStEphF64::WeightedDirGraphStEphF64::*;
    use crate::Chap56::AllPairsResultStEphF64::AllPairsResultStEphF64::{
        AllPairsResultStEphF64, AllPairsResultStEphF64Trait,
    };
    use crate::Chap56::SSSPResultStEphF64::SSSPResultStEphF64::{
        SSSPResultStEphF64, SSSPResultStEphF64Trait, NO_PREDECESSOR,
    };
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphTrait;
    use crate::Chap57::DijkstraStEphF64::DijkstraStEphF64::dijkstra;
    use crate::Chap58::BellmanFordStEphF64::BellmanFordStEphF64::{bellman_ford, BellmanFordError};
    use crate::vstdplus::float::float::*;
    use crate::Types::Types::*;

    verus! 
{

    //		Section 3. broadcast use


    broadcast use {
        crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::group_hash_set_with_view_plus_axioms,
        crate::Types::Types::group_Pair_axioms,
        crate::Types::Types::group_WeightedEdge_axioms,
        crate::vstdplus::float::float::group_float_finite_total_order,
        crate::vstdplus::float::float::group_float_arithmetic,
    };

    //		Section 8. traits


    pub trait JohnsonStEphF64Trait {
        /// Johnson's all-pairs shortest path algorithm.
        /// - Alg Analysis: APAS (Ch59 Alg 59.1): Work O(mn lg n), Span O(m lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(mn lg n), Span O(mn lg n) — matches APAS work; sequential: 1x BF O(nm) + n x Dijkstra O(m lg n)
        fn johnson_apsp(graph: &WeightedDirGraphStEphF64<usize>)
            -> (apsp: AllPairsResultStEphF64)
            requires
                graph@.V.len() > 0,
                graph@.V.len() < usize::MAX as nat,
                spec_labgraphview_wf(graph@),
                valid_key_type_WeightedEdge::<usize, WrappedF64>(),
                forall|v: usize| graph@.V.contains(v) <==> v < graph@.V.len(),
                graph@.A.len() * 2 + 2 <= usize::MAX as int,
            ensures
                apsp.spec_n() as nat == graph@.V.len();
    }

    //		Section 9. impls


    /// Adjust reweighted distance back to original weights.
    /// d(u,v) = d'(u,v) - h(u) + h(v), using float arithmetic directly.
    /// - Alg Analysis: APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: Work O(1), Span O(1).
    // veracity: no_requires
    fn adjust_distance(d_prime: WrappedF64, h_u: WrappedF64, h_v: WrappedF64) -> (adjusted: WrappedF64)
        ensures
            !d_prime.spec_is_finite() ==> !adjusted.spec_is_finite(),
    {
        if !d_prime.is_finite() { unreachable_dist() }
        else {
            d_prime.dist_sub(&h_u).dist_add(&h_v)
        }
    }

    /// Reweight edge: new_weight = weight + h(u) - h(v).
    /// - Alg Analysis: APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: Work O(1), Span O(1).
    // veracity: no_requires
    fn reweight_edge(weight: WrappedF64, h_u: WrappedF64, h_v: WrappedF64) -> (reweighted: WrappedF64)
        ensures true,
    {
        weight.dist_add(&h_u).dist_sub(&h_v)
    }

    /// Build a vertex set {0, ..., max_val} and track its cardinality.
    /// - Alg Analysis: APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: Work O(max_val), Span O(max_val).
    fn build_vertex_set(max_val: usize) -> (vertices: SetStEph<usize>)
        requires
            max_val < usize::MAX,
            valid_key_type_WeightedEdge::<usize, WrappedF64>(),
        ensures
            vertices.spec_setsteph_wf(),
            vertices@.len() == (max_val + 1) as nat,
            forall|k: usize| vertices@.contains(k) <==> k <= max_val,
    {
        let mut vertices = SetStEph::<usize>::empty();
        let mut i: usize = 0;
        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        while i <= max_val
            invariant
                i <= max_val + 1,
                vertices.spec_setsteph_wf(),
                vertices@.len() == i as nat,
                forall|k: usize| vertices@.contains(k) <==> k < i,
            decreases (max_val + 1) - i,
        {
            // Veracity: NEEDED proof block
            proof {
            }
            let _ = vertices.insert(i);
            // Veracity: NEEDED proof block
            proof {
            }
            i = i + 1;
        }
        // Veracity: NEEDED proof block
        proof {
        }
        vertices
    }

    /// Add dummy source vertex n with zero-weight edges to all vertices.
    /// Returns augmented graph with n+1 vertices.
    /// - Alg Analysis: APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: Work O(n + m), Span O(n + m).
    fn add_dummy_source(
        graph: &WeightedDirGraphStEphF64<usize>,
        n: usize,
    ) -> (augmented: WeightedDirGraphStEphF64<usize>)
        requires
            n > 0,
            n < usize::MAX,
            n as nat == graph@.V.len(),
            spec_labgraphview_wf(graph@),
            valid_key_type_WeightedEdge::<usize, WrappedF64>(),
            forall|v: usize| graph@.V.contains(v) <==> v < n,
        ensures
            spec_labgraphview_wf(augmented@),
            augmented@.V.len() == (n + 1) as nat,
            forall|v: usize| v <= n ==> augmented@.V.contains(v),
    {
        let vertices = build_vertex_set(n);

        let mut edges = SetStEph::<WeightedEdge<usize, WrappedF64>>::empty();

        // Copy original edges via out-neighbors.
        let mut u: usize = 0;
        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        while u < n
            invariant
                u <= n,
                n < usize::MAX,
                edges.spec_setsteph_wf(),
                vertices.spec_setsteph_wf(),
                vertices@.len() == (n + 1) as nat,
                forall|k: usize| vertices@.contains(k) <==> k <= n,
                spec_labgraphview_wf(graph@),
                valid_key_type_WeightedEdge::<usize, WrappedF64>(),
                forall|v: usize| graph@.V.contains(v) <==> v < n,
                forall|a: usize, b: usize, w: f64|
                    #[trigger] edges@.contains((a, b, w)) ==>
                    vertices@.contains(a) && vertices@.contains(b),
            decreases n - u,
        {
            let neighbors = graph.out_neighbors_weighed(&u);
            let mut it = neighbors.iter();
            loop
                invariant
                    edges.spec_setsteph_wf(),
                    vertices.spec_setsteph_wf(),
                    forall|k: usize| vertices@.contains(k) <==> k <= n,
                    spec_labgraphview_wf(graph@),
                    valid_key_type_WeightedEdge::<usize, WrappedF64>(),
                    u < n,
                    forall|v: usize| graph@.V.contains(v) <==> v < n,
                    it@.0 <= it@.1.len(),
                    forall|a: usize, b: usize, w: f64|
                        #[trigger] edges@.contains((a, b, w)) ==>
                        vertices@.contains(a) && vertices@.contains(b),
                decreases it@.1.len() - it@.0,
            {
                match it.next() {
                    None => break,
                    Some(pair) => {
                        let Pair(v, weight) = pair;
                        if *v <= n {
                            let _ = edges.insert(WeightedEdge(u, *v, *weight));
                        }
                    }
                }
            }
            u = u + 1;
        }

        // Add dummy edges from n to each vertex 0..n with zero weight.
        let zero = zero_dist();
        let mut j: usize = 0;
        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        while j < n
            invariant
                j <= n,
                edges.spec_setsteph_wf(),
                vertices.spec_setsteph_wf(),
                forall|k: usize| vertices@.contains(k) <==> k <= n,
                valid_key_type_WeightedEdge::<usize, WrappedF64>(),
                forall|a: usize, b: usize, w: f64|
                    #[trigger] edges@.contains((a, b, w)) ==>
                    vertices@.contains(a) && vertices@.contains(b),
            decreases n - j,
        {
            let _ = edges.insert(WeightedEdge(n, j, zero));
            j = j + 1;
        }

        WeightedDirGraphStEphF64::from_weighed_edges(vertices, edges)
    }

    /// Reweight graph edges: w'(u,v) = w(u,v) + h(u) - h(v).
    /// Returns a new graph with the same vertices and reweighted edges.
    /// - Alg Analysis: APAS (Ch59 Alg 59.1): Work O(m), Span O(m).
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m), Span O(m) — matches APAS
    /// - Claude-Opus-4.6: Work O(n + m), Span O(n + m).
    fn reweight_graph(
        graph: &WeightedDirGraphStEphF64<usize>,
        potentials: &Vec<WrappedF64>,
        n: usize,
    ) -> (reweighted: WeightedDirGraphStEphF64<usize>)
        requires
            n > 0,
            n < usize::MAX,
            n as nat == graph@.V.len(),
            potentials@.len() == n as int,
            spec_labgraphview_wf(graph@),
            valid_key_type_WeightedEdge::<usize, WrappedF64>(),
            forall|v: usize| graph@.V.contains(v) <==> v < n,
            graph@.A.len() * 2 + 2 <= usize::MAX as int,
        ensures
            spec_labgraphview_wf(reweighted@),
            reweighted@.V.len() == n as nat,
            forall|v: usize| v < n ==> reweighted@.V.contains(v),
            reweighted@.A.len() <= graph@.A.len(),
            reweighted@.A.len() * 2 + 2 <= usize::MAX as int,
    {
        let vertices = build_vertex_set(n - 1);
        // Veracity: NEEDED proof block
        proof {
        }

        let mut edges = SetStEph::<WeightedEdge<usize, WrappedF64>>::empty();

        // Iterate over all arcs directly.
        let arcs = graph.labeled_arcs();
        let mut it = arcs.iter();
        let ghost arcs_seq = it@.1;

        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        loop
            invariant
                edges.spec_setsteph_wf(),
                vertices.spec_setsteph_wf(),
                vertices@.len() == n as nat,
                forall|k: usize| vertices@.contains(k) <==> k < n,
                potentials@.len() == n as int,
                spec_labgraphview_wf(graph@),
                valid_key_type_WeightedEdge::<usize, WrappedF64>(),
                forall|v: usize| graph@.V.contains(v) <==> v < n,
                n > 0,
                n < usize::MAX,
                it@.0 <= it@.1.len(),
                it@.1 == arcs_seq,
                arcs_seq.map(|i: int, k: LabEdge<usize, WrappedF64>| k@).to_set() =~= graph@.A,
                arcs_seq.no_duplicates(),
                edges@.len() <= it@.0,
                forall|a: usize, b: usize, w: f64|
                    #[trigger] edges@.contains((a, b, w)) ==>
                    vertices@.contains(a) && vertices@.contains(b),
            decreases arcs_seq.len() - it@.0,
        {
            match it.next() {
                None => break,
                Some(arc) => {
                    let from = arc.0;
                    let to = arc.1;
                    let weight = arc.2;
                    if from < n && to < n {
                        let new_weight = reweight_edge(weight, potentials[from], potentials[to]);
                        let _ = edges.insert(WeightedEdge(from, to, new_weight));
                    }
                }
            }
        }

        let result = WeightedDirGraphStEphF64::from_weighed_edges(vertices, edges);
        // Veracity: NEEDED proof block
        proof {
            let view_fn = |k: LabEdge<usize, WrappedF64>| k@;
            // Veracity: NEEDED assert
            assert forall|x: LabEdge<usize, WrappedF64>, y: LabEdge<usize, WrappedF64>|
                #[trigger] view_fn(x) == #[trigger] view_fn(y) implies x == y
            by {};
            arcs_seq.lemma_no_duplicates_injective(view_fn);
            let mapped = arcs_seq.map_values(view_fn);
            mapped.unique_seq_to_set();
            // Veracity: NEEDED assert
            assert(mapped =~= arcs_seq.map(|i: int, k: LabEdge<usize, WrappedF64>| k@));
        }
        result
    }

    /// Create all-unreachable result for negative cycle detection.
    /// - Alg Analysis: APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: Work O(n^2), Span O(n^2).
    fn create_negative_cycle_result(n: usize) -> (neg_cycle_apsp: AllPairsResultStEphF64)
        requires n < usize::MAX,
        ensures
            neg_cycle_apsp.spec_n() == n,
    {
        AllPairsResultStEphF64::new(n)
    }

    /// Algorithm 59.1: Johnson's All-Pairs Shortest Paths.
    ///
    /// Computes all-pairs shortest paths for graphs with arbitrary (including negative) edge
    /// weights. Detects negative-weight cycles and returns all-unreachable if one exists.
    ///
    /// Phase 1: Bellman-Ford on augmented graph to get potentials h(v).
    /// Phase 2: Reweight edges w'(u,v) = w(u,v) + h(u) - h(v) (non-negative).
    /// Phase 3: Run Dijkstra from each vertex on reweighted graph, adjust distances back.
    ///
    /// - Alg Analysis: APAS (Ch59 Alg 59.1): Work O(mn lg n), Span O(m lg n)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(mn lg n), Span O(mn lg n) — matches APAS work; sequential: 1x BF + n x Dijkstra
    pub fn johnson_apsp(graph: &WeightedDirGraphStEphF64<usize>)
        -> (apsp: AllPairsResultStEphF64)
        requires
            graph@.V.len() > 0,
            graph@.V.len() < usize::MAX as nat,
            spec_labgraphview_wf(graph@),
            valid_key_type_WeightedEdge::<usize, WrappedF64>(),
            forall|v: usize| graph@.V.contains(v) <==> v < graph@.V.len(),
            graph@.A.len() * 2 + 2 <= usize::MAX as int,
        ensures
            apsp.spec_n() as nat == graph@.V.len(),
    {
        let n = graph.vertices().size();

        // Phase 1: Bellman-Ford on augmented graph to compute potentials.
        let augmented = add_dummy_source(graph, n);

        let bf_result = match bellman_ford(&augmented, n) {
            Ok(sssp) => sssp,
            Err(_) => {
                return create_negative_cycle_result(n);
            }
        };

        // Extract potentials from BF result.
        let mut potentials: Vec<WrappedF64> = Vec::new();
        let mut i: usize = 0;
        while i < n
            invariant
                i <= n,
                potentials@.len() == i as int,
                bf_result.spec_distances().len() == (n + 1) as nat,
            decreases n - i,
        {
            potentials.push(*bf_result.distances.nth(i));
            i = i + 1;
        }

        // Phase 2: Reweight graph edges.
        let reweighted = reweight_graph(graph, &potentials, n);
        // Veracity: NEEDED assert
        assert(spec_labgraphview_wf(reweighted@));

        // Phase 3: Run Dijkstra from each vertex, adjust distances back.
        let mut result = AllPairsResultStEphF64::new(n);
        let mut u: usize = 0;
        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        while u < n
            invariant
                u <= n,
                n > 0,
                n as nat == graph@.V.len(),
                result.spec_allpairsresultstephf64_wf(),
                result.spec_n() == n,
                potentials@.len() == n as int,
                spec_labgraphview_wf(reweighted@),
                valid_key_type_WeightedEdge::<usize, WrappedF64>(),
                reweighted@.V.len() == n as nat,
                forall|v: usize| v < n ==> reweighted@.V.contains(v),
                reweighted@.A.len() * 2 + 2 <= usize::MAX as int,
            decreases n - u,
        {
            let sssp = dijkstra(&reweighted, u);

            let h_u = potentials[u];
            let mut v: usize = 0;
            while v < n
                invariant
                    v <= n,
                    u < n,
                    potentials@.len() == n as int,
                    result.spec_allpairsresultstephf64_wf(),
                    result.spec_n() == n,
                    sssp.spec_distances().len() == n as nat,
                decreases n - v,
            {
                let d_prime = *sssp.distances.nth(v);
                let h_v = potentials[v];
                let adjusted = adjust_distance(d_prime, h_u, h_v);
                result.set_distance(u, v, adjusted);

                // Copy predecessor from Dijkstra result.
                if v < sssp.predecessors.length() {
                    let pred = *sssp.predecessors.nth(v);
                    if pred != NO_PREDECESSOR {
                        result.set_predecessor(u, v, pred);
                    }
                }
                v = v + 1;
            }
            u = u + 1;
        }

        result
    }

    } // verus!
}
