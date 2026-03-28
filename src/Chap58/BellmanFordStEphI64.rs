//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! Bellman-Ford's Algorithm - Single Source Shortest Path with arbitrary edge weights (integer)
//! Implements Algorithm 58.2 from the textbook.

pub mod BellmanFordStEphI64 {

    use vstd::prelude::*;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::LabDirGraphStEphTrait;
    use crate::Chap06::WeightedDirGraphStEphI128::WeightedDirGraphStEphI128::*;
    use crate::Chap56::SSSPResultStEphI64::SSSPResultStEphI64::*;
    use crate::Types::Types::*;

    verus! {

    // Table of Contents
    // 1. module (BellmanFordStEphI64)
    // 2. imports
    // 3. broadcast use
    // 4. type definitions
    // 8. traits
    // 9. impls
    // 14. derive impls outside verus!

    // 3. broadcast use

    broadcast use {
        crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::group_hash_set_with_view_plus_axioms,
        crate::Types::Types::group_Pair_axioms,
    };

    // 4. type definitions

    pub enum BellmanFordError {
        NegativeCycleDetected,
        AlgorithmError,
    }

    // 8. traits

    pub trait BellmanFordStEphI64Trait {
        /// Bellman-Ford single-source shortest path algorithm.
        /// - APAS: Work O(nm), Span O(n lg n) where n = |V|, m = |E| (with sequences).
        /// - Claude-Opus-4.6: Work O(nm), Span O(nm) — sequential implementation.
        fn bellman_ford(graph: &WeightedDirGraphStEphI128<usize>, source: usize)
            -> (sssp: Result<SSSPResultStEphI64, BellmanFordError>)
            requires
                source < graph@.V.len(),
                spec_labgraphview_wf(graph@),
                valid_key_type_WeightedEdge::<usize, i128>(),
            ensures
                sssp is Ok ==> sssp->Ok_0.spec_distances().len() == graph@.V.len(),
                sssp is Ok ==> sssp->Ok_0.spec_source() == source;
    }

    // 9. impls

    /// Clamp an i128 weight to i64 range.
    /// - APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: Work O(1), Span O(1).
    // veracity: no_requires
    fn clamp_weight(w: i128) -> (clamped: i64)
        ensures
            w >= i64::MIN as i128 && w <= i64::MAX as i128 ==> clamped == w as i64,
            w < i64::MIN as i128 ==> clamped == i64::MIN,
            w > i64::MAX as i128 ==> clamped == i64::MAX,
    {
        if w > i64::MAX as i128 { i64::MAX }
        else if w < i64::MIN as i128 { i64::MIN }
        else { w as i64 }
    }

    /// Safe distance addition: compute d + w via i128 to avoid overflow.
    /// Returns UNREACHABLE on positive overflow, i64::MIN on negative overflow.
    /// - APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: Work O(1), Span O(1).
    fn add_distance(d: i64, w: i64) -> (sum: i64)
        requires d != UNREACHABLE,
        ensures
            (d as i128) + (w as i128) >= UNREACHABLE as i128 ==> sum == UNREACHABLE,
            (d as i128) + (w as i128) < i64::MIN as i128 ==> sum == i64::MIN,
            (d as i128) + (w as i128) >= i64::MIN as i128 && (d as i128) + (w as i128) < UNREACHABLE as i128
                ==> sum == ((d as i128) + (w as i128)) as i64,
    {
        let sum: i128 = (d as i128) + (w as i128);
        if sum >= UNREACHABLE as i128 { UNREACHABLE }
        else if sum < (i64::MIN as i128) { i64::MIN }
        else { sum as i64 }
    }

    /// Reconstruct predecessor array from converged distances.
    /// - APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: Work O(nm), Span O(nm).
    fn reconstruct_predecessors(
        graph: &WeightedDirGraphStEphI128<usize>,
        distances: &Vec<i64>,
        sssp: &mut SSSPResultStEphI64,
        n: usize,
        source: usize,
    )
        requires
            n > 0,
            source < n,
            distances@.len() == n as int,
            old(sssp).spec_distances().len() == n as int,
            old(sssp).spec_predecessors().len() == n as int,
            old(sssp).spec_source() == source,
            spec_labgraphview_wf(graph@),
            valid_key_type_WeightedEdge::<usize, i128>(),
        ensures
            sssp.spec_distances().len() == n as int,
            sssp.spec_predecessors().len() == n as int,
            sssp.spec_source() == source,
            sssp.spec_distances() == old(sssp).spec_distances(),
    {
        let ghost original_distances = sssp.spec_distances();
        let mut v: usize = 0;
        #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
        while v < n
            invariant
                v <= n,
                n > 0,
                source < n,
                distances@.len() == n as int,
                sssp.spec_distances().len() == n as int,
                sssp.spec_predecessors().len() == n as int,
                sssp.spec_source() == source,
                sssp.spec_distances() == original_distances,
                spec_labgraphview_wf(graph@),
                valid_key_type_WeightedEdge::<usize, i128>(),
            decreases n - v,
        {
            if v != source && distances[v] != UNREACHABLE {
                let in_neighbors = graph.in_neighbors_weighed(&v);
                let mut it = in_neighbors.iter();
                loop
                    invariant
                        v < n,
                        distances@.len() == n as int,
                        sssp.spec_distances().len() == n as int,
                        sssp.spec_predecessors().len() == n as int,
                        sssp.spec_source() == source,
                        sssp.spec_distances() == original_distances,
                        it@.0 <= it@.1.len(),
                    decreases it@.1.len() - it@.0,
                {
                    match it.next() {
                        None => break,
                        Some(pair) => {
                            let Pair(u, weight) = pair;
                            if *u < n {
                                let u_dist = distances[*u];
                                if u_dist != UNREACHABLE {
                                    let w64 = clamp_weight(*weight);
                                    let path_dist = add_distance(u_dist, w64);
                                    if path_dist == distances[v] {
                                        sssp.set_predecessor(v, *u);
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            v = v + 1;
        }
    }

    /// Bellman-Ford single-source shortest path algorithm.
    ///
    /// Computes single-source shortest paths for graphs with arbitrary (including negative) edge
    /// weights. Detects negative-weight cycles and returns an error if one exists.
    ///
    /// - APAS: Work O(nm), Span O(n lg n) where n = |V|, m = |E| (with sequences).
    /// - Claude-Opus-4.6: Work O(nm), Span O(nm) — sequential implementation, no parallelism.
    pub fn bellman_ford(graph: &WeightedDirGraphStEphI128<usize>, source: usize)
        -> (sssp: Result<SSSPResultStEphI64, BellmanFordError>)
        requires
            source < graph@.V.len(),
            spec_labgraphview_wf(graph@),
            valid_key_type_WeightedEdge::<usize, i128>(),
        ensures
            sssp is Ok ==> sssp->Ok_0.spec_distances().len() == graph@.V.len(),
            sssp is Ok ==> sssp->Ok_0.spec_source() == source,
    {
        let n = graph.vertices().size();
        assert(n == graph@.V.len());

        // Initialize distances: source = 0, all others = UNREACHABLE.
        let mut distances: Vec<i64> = Vec::new();
        let mut i: usize = 0;
        while i < n
            invariant
                i <= n,
                distances@.len() == i as int,
                forall|k: int| #![trigger distances@[k]] 0 <= k < i ==>
                    distances@[k] == (if k == source as int { 0i64 } else { UNREACHABLE }),
            decreases n - i,
        {
            if i == source { distances.push(0i64); } else { distances.push(UNREACHABLE); }
            i = i + 1;
        }

        // Main relaxation rounds.
        let mut round: usize = 0;
        while round < n
            invariant
                round <= n,
                n == graph@.V.len(),
                distances@.len() == n as int,
                spec_labgraphview_wf(graph@),
                valid_key_type_WeightedEdge::<usize, i128>(),
                source < n,
            decreases n - round,
        {
            let mut changed = false;
            let mut new_distances: Vec<i64> = Vec::new();
            let mut v: usize = 0;
            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            while v < n
                invariant
                    v <= n,
                    new_distances@.len() == v as int,
                    distances@.len() == n as int,
                    spec_labgraphview_wf(graph@),
                    valid_key_type_WeightedEdge::<usize, i128>(),
                decreases n - v,
            {
                let current_dist = distances[v];
                let mut min_dist = current_dist;
                let in_neighbors = graph.in_neighbors_weighed(&v);
                let mut it = in_neighbors.iter();

                loop
                    invariant
                        distances@.len() == n as int,
                        it@.0 <= it@.1.len(),
                    decreases it@.1.len() - it@.0,
                {
                    match it.next() {
                        None => break,
                        Some(pair) => {
                            let Pair(u, weight) = pair;
                            if *u < n {
                                let u_dist = distances[*u];
                                if u_dist != UNREACHABLE {
                                    let w64 = clamp_weight(*weight);
                                    let new_dist = add_distance(u_dist, w64);
                                    if new_dist < min_dist {
                                        min_dist = new_dist;
                                    }
                                }
                            }
                        }
                    }
                }

                new_distances.push(min_dist);
                if min_dist != current_dist { changed = true; }
                v = v + 1;
            }

            distances = new_distances;

            if !changed {
                // Converged. Build SSSPResult.
                let mut sssp = SSSPResultStEphI64::new(n, source);
                let mut vi: usize = 0;
                while vi < n
                    invariant
                        vi <= n,
                        n == graph@.V.len(),
                        distances@.len() == n as int,
                        sssp.spec_distances().len() == n as int,
                        sssp.spec_predecessors().len() == n as int,
                        sssp.spec_source() == source,
                        source < n,
                        spec_labgraphview_wf(graph@),
                        valid_key_type_WeightedEdge::<usize, i128>(),
                    decreases n - vi,
                {
                    sssp.set_distance(vi, distances[vi]);
                    vi = vi + 1;
                }
                reconstruct_predecessors(graph, &distances, &mut sssp, n, source);
                return Ok(sssp);
            }

            if round == n - 1 {
                return Err(BellmanFordError::NegativeCycleDetected);
            }

            round = round + 1;
        }

        Err(BellmanFordError::AlgorithmError)
    }

    } // verus!

    // 14. derive impls outside verus!

    impl std::fmt::Debug for BellmanFordError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                BellmanFordError::NegativeCycleDetected => write!(f, "NegativeCycleDetected"),
                BellmanFordError::AlgorithmError => write!(f, "AlgorithmError"),
            }
        }
    }

    impl std::fmt::Display for BellmanFordError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                BellmanFordError::NegativeCycleDetected => {
                    write!(f, "Negative-weight cycle detected")
                }
                BellmanFordError::AlgorithmError => {
                    write!(f, "Algorithm error: max rounds exceeded")
                }
            }
        }
    }
}
