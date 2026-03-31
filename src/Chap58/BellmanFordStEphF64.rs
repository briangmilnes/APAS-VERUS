//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! Bellman-Ford's Algorithm - Single Source Shortest Path with arbitrary edge weights (float)
//! Implements Algorithm 58.2 from the textbook.

pub mod BellmanFordStEphF64 {

    use vstd::prelude::*;

    use crate::Chap05::SetStEph::SetStEph::*;
    use crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::LabDirGraphStEphTrait;
    use crate::Chap06::WeightedDirGraphStEphF64::WeightedDirGraphStEphF64::*;
    use crate::Chap56::SSSPResultStEphF64::SSSPResultStEphF64::*;
    use crate::vstdplus::float::float::*;
    use crate::Types::Types::*;

    verus! {

    // Table of Contents
    // 1. module (BellmanFordStEphF64)
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
        crate::vstdplus::float::float::group_float_finite_total_order,
        crate::vstdplus::float::float::group_float_arithmetic,
    };

    // 4. type definitions

    pub enum BellmanFordError {
        NegativeCycleDetected,
        AlgorithmError,
    }

    // 8. traits

    pub trait BellmanFordStEphF64Trait {
        /// Bellman-Ford single-source shortest path algorithm.
        /// - Alg Analysis: APAS (Ch58 Alg 58.2): Work O(nm lg n), Span O(n lg n)
        /// - Alg Analysis: APAS (Ch58 Alg 58.2): Work O(nm), Span O(n lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(nm), Span O(nm) — matches APAS seq-based; n rounds, O(m) per round with array seqs
        fn bellman_ford(graph: &WeightedDirGraphStEphF64<usize>, source: usize)
            -> (sssp: Result<SSSPResultStEphF64, BellmanFordError>)
            requires
                source < graph@.V.len(),
                spec_labgraphview_wf(graph@),
                valid_key_type_WeightedEdge::<usize, WrappedF64>(),
            ensures
                sssp is Ok ==> sssp->Ok_0.spec_distances().len() == graph@.V.len(),
                sssp is Ok ==> sssp->Ok_0.spec_source() == source;
    }

    // 9. impls

    /// Reconstruct predecessor array from converged distances.
    /// - APAS: N/A — Verus-specific scaffolding.
    /// - Claude-Opus-4.6: Work O(nm), Span O(nm).
    fn reconstruct_predecessors(
        graph: &WeightedDirGraphStEphF64<usize>,
        distances: &Vec<WrappedF64>,
        sssp: &mut SSSPResultStEphF64,
        n: usize,
        source: usize,
    )
        requires
            n > 0,
            source < n,
            distances@.len() == n as int,
            old(sssp).spec_ssspresultstephf64_wf(),
            old(sssp).spec_distances().len() == n as int,
            old(sssp).spec_predecessors().len() == n as int,
            old(sssp).spec_source() == source,
            spec_labgraphview_wf(graph@),
            valid_key_type_WeightedEdge::<usize, WrappedF64>(),
        ensures
            sssp.spec_ssspresultstephf64_wf(),
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
                sssp.spec_ssspresultstephf64_wf(),
                sssp.spec_distances().len() == n as int,
                sssp.spec_predecessors().len() == n as int,
                sssp.spec_source() == source,
                sssp.spec_distances() == original_distances,
                spec_labgraphview_wf(graph@),
                valid_key_type_WeightedEdge::<usize, WrappedF64>(),
            decreases n - v,
        {
            if v != source && distances[v].is_finite() {
                let in_neighbors = graph.in_neighbors_weighed(&v);
                let mut it = in_neighbors.iter();
                loop
                    invariant
                        v < n,
                        distances@.len() == n as int,
                        sssp.spec_ssspresultstephf64_wf(),
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
                                if u_dist.is_finite() {
                                    let path_dist = u_dist.dist_add(weight);
                                    if path_dist.is_finite() && path_dist.eq(&distances[v]) {
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
    /// - Alg Analysis: APAS (Ch58 Alg 58.2): Work O(nm lg n), Span O(n lg n)
    /// - Alg Analysis: APAS (Ch58 Alg 58.2): Work O(nm), Span O(n lg n)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(nm), Span O(nm) — matches APAS seq-based; n rounds, O(m) per round with array seqs
    pub fn bellman_ford(graph: &WeightedDirGraphStEphF64<usize>, source: usize)
        -> (sssp: Result<SSSPResultStEphF64, BellmanFordError>)
        requires
            source < graph@.V.len(),
            spec_labgraphview_wf(graph@),
            valid_key_type_WeightedEdge::<usize, WrappedF64>(),
        ensures
            sssp is Ok ==> sssp->Ok_0.spec_distances().len() == graph@.V.len(),
            sssp is Ok ==> sssp->Ok_0.spec_source() == source,
    {
        let n = graph.vertices().size();
        assert(n == graph@.V.len());

        // Initialize distances: source = 0, all others = unreachable (infinity).
        let unreach = unreachable_dist();
        let zero = zero_dist();
        let mut distances: Vec<WrappedF64> = Vec::new();
        let mut i: usize = 0;
        while i < n
            invariant
                i <= n,
                distances@.len() == i as int,
            decreases n - i,
        {
            if i == source { distances.push(zero); } else { distances.push(unreach); }
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
                valid_key_type_WeightedEdge::<usize, WrappedF64>(),
                source < n,
            decreases n - round,
        {
            let mut changed = false;
            let mut new_distances: Vec<WrappedF64> = Vec::new();
            let mut v: usize = 0;
            #[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
            while v < n
                invariant
                    v <= n,
                    new_distances@.len() == v as int,
                    distances@.len() == n as int,
                    spec_labgraphview_wf(graph@),
                    valid_key_type_WeightedEdge::<usize, WrappedF64>(),
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
                                if u_dist.is_finite() {
                                    let new_dist = u_dist.dist_add(weight);
                                    if new_dist.is_finite() {
                                        if !min_dist.is_finite() || new_dist.dist_lt(&min_dist) {
                                            min_dist = new_dist;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                new_distances.push(min_dist);
                if !min_dist.eq(&current_dist) { changed = true; }
                v = v + 1;
            }

            distances = new_distances;

            if !changed {
                // Converged. Build SSSPResult.
                let mut sssp = SSSPResultStEphF64::new(n, source);
                let mut vi: usize = 0;
                while vi < n
                    invariant
                        vi <= n,
                        n == graph@.V.len(),
                        distances@.len() == n as int,
                        sssp.spec_ssspresultstephf64_wf(),
                        sssp.spec_distances().len() == n as int,
                        sssp.spec_predecessors().len() == n as int,
                        sssp.spec_source() == source,
                        source < n,
                        spec_labgraphview_wf(graph@),
                        valid_key_type_WeightedEdge::<usize, WrappedF64>(),
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
