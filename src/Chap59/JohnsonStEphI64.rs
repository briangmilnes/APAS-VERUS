//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 59: Johnson's Algorithm - Single-threaded Ephemeral Integer Weights
//!
//! Implements Algorithm 59.1 from the textbook.
//! All-Pairs Shortest Paths for graphs with negative weights (but no negative cycles).
//!
//! **Algorithmic Analysis:**
//! - Johnson APSP: Work O(mn log n), Span O(mn log n) where n = |V|, m = |E|
//! - Phase 1 (Bellman-Ford): Work O(nm), Span O(nm)
//! - Phase 2 (Reweighting): Work O(m), Span O(m)
//! - Phase 3 (n Dijkstras): Work O(n * m log n) = O(mn log n), Span O(mn log n) sequential

pub mod JohnsonStEphI64 {

    use vstd::prelude::*;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::LabDirGraphStEphTrait;
    use crate::Chap06::WeightedDirGraphStEphI128::WeightedDirGraphStEphI128::*;
    use crate::Chap56::AllPairsResultStEphI64::AllPairsResultStEphI64::{
        AllPairsResultStEphI64, AllPairsResultStEphI64Trait,
    };
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphTrait;
    use crate::Chap56::SSSPResultStEphI64::SSSPResultStEphI64::{
        SSSPResultStEphI64, SSSPResultStEphI64Trait, UNREACHABLE, NO_PREDECESSOR,
    };
    use crate::Chap57::DijkstraStEphU64::DijkstraStEphU64::dijkstra;
    use crate::Chap58::BellmanFordStEphI64::BellmanFordStEphI64::{bellman_ford, BellmanFordError};
    use crate::Types::Types::*;

    verus! {

    // Table of Contents
    // 1. module (JohnsonStEphI64)
    // 2. imports
    // 3. broadcast use
    // 8. traits
    // 9. impls

    // 3. broadcast use

    broadcast use {
        crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::group_hash_set_with_view_plus_axioms,
        crate::Types::Types::group_Pair_axioms,
        crate::Types::Types::group_WeightedEdge_axioms,
    };

    // 8. traits

    pub trait JohnsonStEphI64Trait {
        /// Johnson's all-pairs shortest path algorithm.
        /// - Alg Analysis: APAS (Ch59 Alg 59.1): Work O(mn lg n), Span O(m lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(mn lg n), Span O(mn lg n) — matches APAS work; sequential: 1x BF O(nm) + n x Dijkstra O(m lg n)
        fn johnson_apsp(graph: &WeightedDirGraphStEphI128<usize>)
            -> (apsp: AllPairsResultStEphI64)
            requires
                graph@.V.len() > 0,
                graph@.V.len() < usize::MAX as nat,
                spec_labgraphview_wf(graph@),
                valid_key_type_WeightedEdge::<usize, i128>(),
                forall|v: usize| graph@.V.contains(v) <==> v < graph@.V.len(),
                graph@.A.len() * 2 + 2 <= usize::MAX as int,
            ensures
                apsp.spec_n() as nat == graph@.V.len();
    }

    // 9. impls

    /// Adjust reweighted distance back to original weights.
    /// d(u,v) = d'(u,v) - h(u) + h(v), using i128 to avoid overflow.
    /// - Alg Analysis: APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: Work O(1), Span O(1).
    // veracity: no_requires
    fn adjust_distance(d_prime: i64, h_u: i64, h_v: i64) -> (adjusted: i64)
        ensures
            d_prime == UNREACHABLE ==> adjusted == UNREACHABLE,
    {
        if d_prime == UNREACHABLE { UNREACHABLE }
        else {
            let sum: i128 = (d_prime as i128) - (h_u as i128) + (h_v as i128);
            if sum >= UNREACHABLE as i128 { UNREACHABLE }
            else if sum < i64::MIN as i128 { i64::MIN }
            else { sum as i64 }
        }
    }

    /// Reweight edge: new_weight = weight + h(u) - h(v), clamped to i128 range.
    /// - Alg Analysis: APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: Work O(1), Span O(1).
    // veracity: no_requires
    fn reweight_edge(weight: i128, h_u: i64, h_v: i64) -> (reweighted: i128)
        ensures true,
    {
        let diff: i128 = (h_u as i128) - (h_v as i128);
        if diff >= 0 {
            if weight <= i128::MAX - diff { weight + diff } else { i128::MAX }
        } else {
            if weight >= i128::MIN - diff { weight + diff } else { i128::MIN }
        }
    }

    /// Build a vertex set {0, ..., max_val} and track its cardinality.
    /// - Alg Analysis: APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: Work O(max_val), Span O(max_val).
    fn build_vertex_set(max_val: usize) -> (vertices: SetStEph<usize>)
        requires
            max_val < usize::MAX,
            valid_key_type_WeightedEdge::<usize, i128>(),
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
            proof {
                assert(!vertices@.contains(i));
            }
            let _ = vertices.insert(i);
            proof {
                assert(vertices@.len() == (i + 1) as nat);
                assert(forall|k: usize| vertices@.contains(k) <==> k < i + 1);
            }
            i = i + 1;
        }
        proof {
            assert(i == max_val + 1);
            assert(forall|k: usize| vertices@.contains(k) <==> k < max_val + 1);
            assert(forall|k: usize| vertices@.contains(k) <==> k <= max_val);
        }
        vertices
    }

    /// Add dummy source vertex n with zero-weight edges to all vertices.
    /// Returns augmented graph with n+1 vertices.
    /// - Alg Analysis: APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: Work O(n + m), Span O(n + m).
    fn add_dummy_source(
        graph: &WeightedDirGraphStEphI128<usize>,
        n: usize,
    ) -> (augmented: WeightedDirGraphStEphI128<usize>)
        requires
            n > 0,
            n < usize::MAX,
            n as nat == graph@.V.len(),
            spec_labgraphview_wf(graph@),
            valid_key_type_WeightedEdge::<usize, i128>(),
            forall|v: usize| graph@.V.contains(v) <==> v < n,
        ensures
            spec_labgraphview_wf(augmented@),
            augmented@.V.len() == (n + 1) as nat,
            forall|v: usize| v <= n ==> augmented@.V.contains(v),
    {
        let vertices = build_vertex_set(n);

        let mut edges = SetStEph::<WeightedEdge<usize, i128>>::empty();

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
                valid_key_type_WeightedEdge::<usize, i128>(),
                forall|v: usize| graph@.V.contains(v) <==> v < n,
                forall|a: usize, b: usize, w: i128|
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
                    valid_key_type_WeightedEdge::<usize, i128>(),
                    u < n,
                    forall|v: usize| graph@.V.contains(v) <==> v < n,
                    it@.0 <= it@.1.len(),
                    forall|a: usize, b: usize, w: i128|
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

        // Add dummy edges from n to each vertex 0..n.
        let mut j: usize = 0;
        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        while j < n
            invariant
                j <= n,
                edges.spec_setsteph_wf(),
                vertices.spec_setsteph_wf(),
                forall|k: usize| vertices@.contains(k) <==> k <= n,
                valid_key_type_WeightedEdge::<usize, i128>(),
                forall|a: usize, b: usize, w: i128|
                    #[trigger] edges@.contains((a, b, w)) ==>
                    vertices@.contains(a) && vertices@.contains(b),
            decreases n - j,
        {
            let _ = edges.insert(WeightedEdge(n, j, 0i128));
            j = j + 1;
        }

        WeightedDirGraphStEphI128::from_weighed_edges(vertices, edges)
    }

    /// Reweight graph edges: w'(u,v) = w(u,v) + h(u) - h(v).
    /// Returns a new graph with the same vertices and reweighted edges.
    /// - APAS: Work O(m), Span O(m).
    /// - Claude-Opus-4.6: Work O(n + m), Span O(n + m).
    fn reweight_graph(
        graph: &WeightedDirGraphStEphI128<usize>,
        potentials: &Vec<i64>,
        n: usize,
    ) -> (reweighted: WeightedDirGraphStEphI128<usize>)
        requires
            n > 0,
            n < usize::MAX,
            n as nat == graph@.V.len(),
            potentials@.len() == n as int,
            spec_labgraphview_wf(graph@),
            valid_key_type_WeightedEdge::<usize, i128>(),
            forall|v: usize| graph@.V.contains(v) <==> v < n,
            graph@.A.len() * 2 + 2 <= usize::MAX as int,
        ensures
            spec_labgraphview_wf(reweighted@),
            reweighted@.V.len() == n as nat,
            forall|v: usize| v < n ==> reweighted@.V.contains(v),
            reweighted@.A.len() * 2 + 2 <= usize::MAX as int,
    {
        let vertices = build_vertex_set(n - 1);
        proof {
            assert(vertices@.len() == n as nat);
            assert(forall|k: usize| vertices@.contains(k) <==> k <= n - 1);
            assert(forall|k: usize| vertices@.contains(k) <==> k < n);
        }

        let mut edges = SetStEph::<WeightedEdge<usize, i128>>::empty();

        // Iterate over all arcs directly (avoids sum-of-degrees lemma).
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
                valid_key_type_WeightedEdge::<usize, i128>(),
                forall|v: usize| graph@.V.contains(v) <==> v < n,
                n > 0,
                n < usize::MAX,
                it@.0 <= it@.1.len(),
                it@.1 == arcs_seq,
                arcs_seq.map(|i: int, k: LabEdge<usize, i128>| k@).to_set() =~= graph@.A,
                arcs_seq.no_duplicates(),
                edges@.len() <= it@.0,
                forall|a: usize, b: usize, w: i128|
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

        let result = WeightedDirGraphStEphI128::from_weighed_edges(vertices, edges);
        proof {
            assert(result@.V.len() == n as nat);
            // Prove edges@.len() <= graph@.A.len():
            // At loop exit: edges@.len() <= it@.0 == arcs_seq.len().
            // The view function on LabEdge<usize, i128> is injective (identity).
            let view_fn = |k: LabEdge<usize, i128>| k@;
            assert forall|x: LabEdge<usize, i128>, y: LabEdge<usize, i128>|
                #[trigger] view_fn(x) == #[trigger] view_fn(y) implies x == y
            by {};
            // By lemma_no_duplicates_injective: mapped seq has no duplicates.
            arcs_seq.lemma_no_duplicates_injective(view_fn);
            // By unique_seq_to_set: no-dup seq length == to_set length.
            let mapped = arcs_seq.map_values(view_fn);
            mapped.unique_seq_to_set();
            // mapped.to_set() =~= arcs_seq.map(|i, k| k@).to_set() =~= graph@.A.
            assert(mapped =~= arcs_seq.map(|i: int, k: LabEdge<usize, i128>| k@));
            assert(mapped.to_set() =~= graph@.A);
            // Therefore arcs_seq.len() == graph@.A.len().
            assert(arcs_seq.len() == graph@.A.len());
            // And edges@.len() <= arcs_seq.len() from loop invariant.
            assert(result@.A.len() <= graph@.A.len());
        }
        result
    }

    /// Create all-UNREACHABLE result for negative cycle detection.
    /// - Alg Analysis: APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: Work O(n^2), Span O(n^2).
    fn create_negative_cycle_result(n: usize) -> (neg_cycle_apsp: AllPairsResultStEphI64)
        requires n < usize::MAX,
        ensures
            neg_cycle_apsp.spec_n() == n,
            neg_cycle_apsp.spec_distances_len() == n as nat,
            neg_cycle_apsp.spec_predecessors_len() == n as nat,
    {
        let mut result = AllPairsResultStEphI64::new(n);
        let mut u: usize = 0;
        while u < n
            invariant
                u <= n,
                result.spec_n() == n,
                result.spec_distances_len() == n as nat,
                result.spec_predecessors_len() == n as nat,
                forall|r: int| #![trigger result.spec_distances_row_len(r)]
                    0 <= r < n ==> result.spec_distances_row_len(r) == n as nat,
                forall|r: int| #![trigger result.spec_predecessors_row_len(r)]
                    0 <= r < n ==> result.spec_predecessors_row_len(r) == n as nat,
            decreases n - u,
        {
            result.set_distance(u, u, UNREACHABLE);
            u = u + 1;
        }
        result
    }

    /// Algorithm 59.1: Johnson's All-Pairs Shortest Paths.
    ///
    /// Computes all-pairs shortest paths for graphs with arbitrary (including negative) edge
    /// weights. Detects negative-weight cycles and returns all-UNREACHABLE if one exists.
    ///
    /// Phase 1: Bellman-Ford on augmented graph to get potentials h(v).
    /// Phase 2: Reweight edges w'(u,v) = w(u,v) + h(u) - h(v) (non-negative).
    /// Phase 3: Run Dijkstra from each vertex on reweighted graph, adjust distances back.
    ///
    /// - Alg Analysis: APAS (Ch59 Alg 59.1): Work O(mn lg n), Span O(m lg n)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(mn lg n), Span O(mn lg n) — matches APAS work; sequential: 1x BF + n x Dijkstra
    pub fn johnson_apsp(graph: &WeightedDirGraphStEphI128<usize>)
        -> (apsp: AllPairsResultStEphI64)
        requires
            graph@.V.len() > 0,
            graph@.V.len() < usize::MAX as nat,
            spec_labgraphview_wf(graph@),
            valid_key_type_WeightedEdge::<usize, i128>(),
            forall|v: usize| graph@.V.contains(v) <==> v < graph@.V.len(),
            graph@.A.len() * 2 + 2 <= usize::MAX as int,
        ensures
            apsp.spec_n() as nat == graph@.V.len(),
    {
        let n = graph.vertices().size();
        assert(n as nat == graph@.V.len());
        assert(n > 0);
        assert(n < usize::MAX);

        // Phase 1: Bellman-Ford on augmented graph to compute potentials.
        let augmented = add_dummy_source(graph, n);
        assert(augmented@.V.len() == (n + 1) as nat);
        assert(n < augmented@.V.len());

        let bf_result = match bellman_ford(&augmented, n) {
            Ok(sssp) => sssp,
            Err(_) => {
                return create_negative_cycle_result(n);
            }
        };

        // Extract potentials from BF result.
        let mut potentials: Vec<i64> = Vec::new();
        let mut i: usize = 0;
        while i < n
            invariant
                i <= n,
                potentials@.len() == i as int,
            decreases n - i,
        {
            potentials.push(bf_result.get_distance(i));
            i = i + 1;
        }

        // Phase 2: Reweight graph edges.
        let reweighted = reweight_graph(graph, &potentials, n);
        assert(reweighted@.V.len() == n as nat);
        assert(spec_labgraphview_wf(reweighted@));

        // Phase 3: Run Dijkstra from each vertex, adjust distances back.
        let mut result = AllPairsResultStEphI64::new(n);
        let mut u: usize = 0;
        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        while u < n
            invariant
                u <= n,
                n > 0,
                n as nat == graph@.V.len(),
                result.spec_n() == n,
                result.spec_distances_len() == n as nat,
                result.spec_predecessors_len() == n as nat,
                forall|r: int| #![trigger result.spec_distances_row_len(r)]
                    0 <= r < n ==> result.spec_distances_row_len(r) == n as nat,
                forall|r: int| #![trigger result.spec_predecessors_row_len(r)]
                    0 <= r < n ==> result.spec_predecessors_row_len(r) == n as nat,
                potentials@.len() == n as int,
                spec_labgraphview_wf(reweighted@),
                valid_key_type_WeightedEdge::<usize, i128>(),
                reweighted@.V.len() == n as nat,
                forall|v: usize| v < n ==> reweighted@.V.contains(v),
                reweighted@.A.len() * 2 + 2 <= usize::MAX as int,
            decreases n - u,
        {
            // Dijkstra requires source < graph.vertices().size().
            // reweighted.vertices().size() == reweighted@.V.len() == n, and u < n.
            let sssp = dijkstra(&reweighted, u);

            let h_u = potentials[u];
            let mut v: usize = 0;
            while v < n
                invariant
                    v <= n,
                    u < n,
                    potentials@.len() == n as int,
                    result.spec_n() == n,
                    result.spec_distances_len() == n as nat,
                    result.spec_predecessors_len() == n as nat,
                    forall|r: int| #![trigger result.spec_distances_row_len(r)]
                        0 <= r < n ==> result.spec_distances_row_len(r) == n as nat,
                    forall|r: int| #![trigger result.spec_predecessors_row_len(r)]
                        0 <= r < n ==> result.spec_predecessors_row_len(r) == n as nat,
                decreases n - v,
            {
                let d_prime = sssp.get_distance(v);
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
