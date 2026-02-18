//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//!
//! Bellman-Ford's Algorithm - Single Source Shortest Path with arbitrary edge weights (integer)
//! Implements Algorithm 58.2 from the textbook.

pub mod BellmanFordStEphI64 {

    use vstd::prelude::*;

    #[cfg(not(verus_keep_ghost))]
    use std::collections::HashMap;

    use crate::Chap06::WeightedDirGraphStEphI128::WeightedDirGraphStEphI128::*;
    use crate::Chap56::SSSPResultStEphI64::SSSPResultStEphI64::SSSPResultStEphI64;

    #[cfg(not(verus_keep_ghost))]
    use crate::Chap05::SetStEph::SetStEph::*;
    #[cfg(not(verus_keep_ghost))]
    use crate::Chap06::LabDirGraphStEph::LabDirGraphStEph::LabDirGraphStEphTrait;
    #[cfg(not(verus_keep_ghost))]
    use crate::Types::Types::*;

    verus! {

    pub trait BellmanFordStEphI64Trait {
        fn bellman_ford(graph: &WeightedDirGraphStEphI128<usize>, source: usize)
            -> (result: Result<SSSPResultStEphI64, String>);
    }

    } // verus!

    #[cfg(not(verus_keep_ghost))]
    pub fn bellman_ford(graph: &WeightedDirGraphStEphI128<usize>, source: usize) -> Result<SSSPResultStEphI64, String> {
        let n = graph.vertices().size();
        let mut distances = HashMap::<usize, i64>::new();
        for v in 0..n {
            distances.insert(v, if v == source { 0 } else { i64::MAX });
        }
        for round in 0..n {
            let mut changed = false;
            let mut new_distances = HashMap::<usize, i64>::new();
            for v in 0..n {
                let current_dist = *distances.get(&v).unwrap_or(&i64::MAX);
                let mut min_dist = current_dist;
                let in_neighbors = graph.in_neighbors_weighed(&v);
                for Pair(u, weight) in in_neighbors.iter() {
                    let u_dist = *distances.get(u).unwrap_or(&i64::MAX);
                    if u_dist != i64::MAX {
                        let new_dist = u_dist.saturating_add(*weight as i64);
                        if new_dist < min_dist { min_dist = new_dist; }
                    }
                }
                new_distances.insert(v, min_dist);
                if min_dist != current_dist { changed = true; }
            }
            distances = new_distances;
            if !changed {
                let mut result = SSSPResultStEphI64::new(n, source);
                for v in 0..n {
                    let dist = *distances.get(&v).unwrap_or(&i64::MAX);
                    result.set_distance(v, dist);
                }
                reconstruct_predecessors(graph, &distances, &mut result, source);
                return Ok(result);
            }
            if round == n - 1 { return Err("Negative-weight cycle detected".to_string()); }
        }
        Err("Algorithm error: max rounds exceeded".to_string())
    }

    #[cfg(not(verus_keep_ghost))]
    fn reconstruct_predecessors(
        graph: &WeightedDirGraphStEphI128<usize>,
        distances: &HashMap<usize, i64>,
        result: &mut SSSPResultStEphI64,
        source: usize,
    ) {
        let n = graph.vertices().size();
        for v in 0..n {
            if v == source { continue; }
            let v_dist = *distances.get(&v).unwrap_or(&i64::MAX);
            if v_dist == i64::MAX { continue; }
            let in_neighbors = graph.in_neighbors_weighed(&v);
            for Pair(u, weight) in in_neighbors.iter() {
                let u_dist = *distances.get(u).unwrap_or(&i64::MAX);
                if u_dist != i64::MAX {
                    let path_dist = u_dist.saturating_add(*weight as i64);
                    if path_dist == v_dist { result.set_predecessor(v, *u); break; }
                }
            }
        }
    }
}
