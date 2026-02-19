//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Breadth-First Search - Parallel Persistent (Chapter 54, Algorithms 54.3 and 54.6).
//! Layer-by-layer parallel BFS using HF Scheduler join() for fork-join frontier processing.
//! Algorithm 54.3: distances. Algorithm 54.6: shortest-path tree + BFS-order iteration.
//! Work: O(|V| + |E|), Span: O(d·lg n) where d is diameter.

pub mod BFSMtPer {

    use vstd::prelude::*;
    use crate::Types::Types::*;
    use crate::Chap18::ArraySeqMtPer::ArraySeqMtPer::*;
    use crate::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::join;

    verus! {

    // Table of Contents
    // 4. type definitions
    // 6. spec fns
    // 7. proof fns
    // 8. traits
    // 9. impls

    // 4. type definitions
    pub type T<N> = ArraySeqMtPerS<ArraySeqMtPerS<N>>;

    pub const UNREACHABLE: N = N::MAX;
    pub const NO_PARENT: N = N::MAX;

    pub struct BFSTreeS {
        pub parents: ArraySeqMtPerS<N>,
        pub order: ArraySeqMtPerS<N>,
    }

    // 6. spec fns

    pub open spec fn spec_wf_graph(graph: &ArraySeqMtPerS<ArraySeqMtPerS<N>>) -> bool {
        forall|u: int, i: int| #![auto]
            0 <= u < graph.spec_len() && 0 <= i < graph.spec_index(u).spec_len()
            ==> graph.spec_index(u).spec_index(i) < graph.spec_len()
    }

    pub open spec fn spec_distances_bounded(distances: &ArraySeqMtPerS<N>, n: int) -> bool {
        forall|j: int| #![auto] 0 <= j < distances.spec_len() ==>
            distances.spec_index(j) == UNREACHABLE || distances.spec_index(j) < n
    }

    /// Every parent entry is either NO_PARENT or a valid vertex index.
    pub open spec fn spec_parents_bounded(parents: &ArraySeqMtPerS<N>, n: int) -> bool {
        forall|j: int| #![auto] 0 <= j < parents.spec_len() ==>
            parents.spec_index(j) == NO_PARENT || parents.spec_index(j) < n
    }

    // 7. proof fns

    proof fn lemma_tabulate_all_no_parent(parents: &ArraySeqMtPerS<N>, n: int)
        requires
            parents.spec_len() == n,
            forall|i: int| #![auto] 0 <= i < n ==> parents.spec_index(i) == NO_PARENT,
        ensures
            spec_parents_bounded(parents, n),
    {}

    proof fn lemma_update_preserves_parents_bounded(
        parents: &ArraySeqMtPerS<N>,
        old_parents: &ArraySeqMtPerS<N>,
        v: int,
        new_val: N,
        n: int,
    )
        requires
            parents.spec_len() == n,
            old_parents.spec_len() == n,
            0 <= v < n,
            new_val < n,
            parents.spec_index(v) == new_val,
            forall|j: int| #![auto] 0 <= j < n && j != v ==>
                parents.spec_index(j) == old_parents.spec_index(j),
            spec_parents_bounded(old_parents, n),
        ensures
            spec_parents_bounded(parents, n),
    {
        assert forall|j: int| 0 <= j < parents.spec_len()
        implies
            parents.spec_index(j) == NO_PARENT || parents.spec_index(j) < n
        by {
            if j == v {
                assert(parents.spec_index(j) == new_val);
            } else {
                assert(parents.spec_index(j) == old_parents.spec_index(j));
            }
        }
    }

    proof fn lemma_copy_preserves_parents_bounded(
        original: &ArraySeqMtPerS<N>,
        copy: &ArraySeqMtPerS<N>,
        n: int,
    )
        requires
            spec_parents_bounded(original, n),
            copy.spec_len() == original.spec_len(),
            forall|i: int| #![auto] 0 <= i < original.spec_len() ==>
                copy.spec_index(i) == original.spec_index(i),
        ensures
            spec_parents_bounded(copy, n),
    {
        assert forall|j: int| 0 <= j < copy.spec_len()
        implies
            copy.spec_index(j) == NO_PARENT || copy.spec_index(j) < n
        by {
            assert(copy.spec_index(j) == original.spec_index(j));
        }
    }

    proof fn lemma_tabulate_all_unreachable(distances: &ArraySeqMtPerS<N>, n: int)
        requires
            distances.spec_len() == n,
            forall|i: int| #![auto] 0 <= i < n ==>
                distances.spec_index(i) == UNREACHABLE,
        ensures
            spec_distances_bounded(distances, n),
    {
    }

    proof fn lemma_update_preserves_bounded(
        distances: &ArraySeqMtPerS<N>,
        old_distances: &ArraySeqMtPerS<N>,
        v: int,
        new_val: N,
        n: int,
    )
        requires
            distances.spec_len() == n,
            old_distances.spec_len() == n,
            0 <= v < n,
            new_val < n,
            distances.spec_index(v) == new_val,
            forall|j: int| #![auto] 0 <= j < n && j != v ==>
                distances.spec_index(j) == old_distances.spec_index(j),
            spec_distances_bounded(old_distances, n),
        ensures
            spec_distances_bounded(distances, n),
    {
        assert forall|j: int| 0 <= j < distances.spec_len()
        implies
            distances.spec_index(j) == UNREACHABLE || distances.spec_index(j) < n
        by {
            if j == v {
                assert(distances.spec_index(j) == new_val);
            } else {
                assert(distances.spec_index(j) == old_distances.spec_index(j));
            }
        }
    }

    // Builds an owned copy of the distances array with proven spec equality.
    fn copy_distances(distances: &ArraySeqMtPerS<N>) -> (result: ArraySeqMtPerS<N>)
        requires distances.spec_len() <= usize::MAX,
        ensures
            result.spec_len() == distances.spec_len(),
            forall|i: int| #![auto] 0 <= i < distances.spec_len() ==>
                result.spec_index(i) == distances.spec_index(i),
    {
        let n = distances.length();
        ArraySeqMtPerS::tabulate(
            &|idx: usize| -> (r: N)
                requires idx < n, n == distances.spec_len()
                ensures r == distances.spec_index(idx as int)
            { *distances.nth(idx) },
            n,
        )
    }

    // Builds an owned copy of the graph with proven spec equality.
    fn copy_graph(graph: &ArraySeqMtPerS<ArraySeqMtPerS<N>>) -> (result: ArraySeqMtPerS<ArraySeqMtPerS<N>>)
        requires graph.spec_len() <= usize::MAX,
        ensures
            result.spec_len() == graph.spec_len(),
            forall|u: int| #![auto] 0 <= u < graph.spec_len() ==>
                result.spec_index(u).spec_len() == graph.spec_index(u).spec_len(),
            forall|u: int, i: int| #![auto]
                0 <= u < graph.spec_len() && 0 <= i < graph.spec_index(u).spec_len() ==>
                result.spec_index(u).spec_index(i) == graph.spec_index(u).spec_index(i),
    {
        let n = graph.length();
        ArraySeqMtPerS::tabulate(
            &|u_idx: usize| -> (r: ArraySeqMtPerS<N>)
                requires u_idx < n, n == graph.spec_len()
                ensures
                    r.spec_len() == graph.spec_index(u_idx as int).spec_len(),
                    forall|k: int| #![auto] 0 <= k < r.spec_len() ==>
                        r.spec_index(k) == graph.spec_index(u_idx as int).spec_index(k),
            {
                let adj = graph.nth(u_idx);
                let adj_len = adj.length();
                ArraySeqMtPerS::tabulate(
                    &|k: usize| -> (r: N)
                        requires k < adj_len, adj_len == adj.spec_len()
                        ensures r == adj.spec_index(k as int)
                    { *adj.nth(k) },
                    adj_len,
                )
            },
            n,
        )
    }

    // Proves that spec_wf_graph holds for a graph copy with matching spec values.
    proof fn lemma_copy_preserves_wf(
        original: &ArraySeqMtPerS<ArraySeqMtPerS<N>>,
        copy: &ArraySeqMtPerS<ArraySeqMtPerS<N>>,
    )
        requires
            spec_wf_graph(original),
            copy.spec_len() == original.spec_len(),
            forall|u: int| #![auto] 0 <= u < original.spec_len() ==>
                copy.spec_index(u).spec_len() == original.spec_index(u).spec_len(),
            forall|u: int, i: int| #![auto]
                0 <= u < original.spec_len() && 0 <= i < original.spec_index(u).spec_len() ==>
                copy.spec_index(u).spec_index(i) == original.spec_index(u).spec_index(i),
        ensures
            spec_wf_graph(copy),
    {
        assert forall|u: int, i: int|
            0 <= u < copy.spec_len() && 0 <= i < copy.spec_index(u).spec_len()
        implies
            copy.spec_index(u).spec_index(i) < copy.spec_len()
        by {
            assert(copy.spec_index(u).spec_index(i) == original.spec_index(u).spec_index(i));
            assert(original.spec_index(u).spec_index(i) < original.spec_len());
        }
    }

    // Proves that spec_distances_bounded holds for a distances copy with matching spec values.
    proof fn lemma_copy_preserves_bounded(
        original: &ArraySeqMtPerS<N>,
        copy: &ArraySeqMtPerS<N>,
        n: int,
    )
        requires
            spec_distances_bounded(original, n),
            copy.spec_len() == original.spec_len(),
            forall|i: int| #![auto] 0 <= i < original.spec_len() ==>
                copy.spec_index(i) == original.spec_index(i),
        ensures
            spec_distances_bounded(copy, n),
    {
        assert forall|j: int| 0 <= j < copy.spec_len()
        implies
            copy.spec_index(j) == UNREACHABLE || copy.spec_index(j) < n
        by {
            assert(copy.spec_index(j) == original.spec_index(j));
        }
    }

    // 8. traits
    pub trait BFSMtPerTrait {
        fn bfs(graph: &ArraySeqMtPerS<ArraySeqMtPerS<N>>, source: N) -> (result: ArraySeqMtPerS<N>)
            requires
                source < graph.spec_len(),
                graph.spec_len() > 0,
                graph.spec_len() < usize::MAX,
                spec_wf_graph(graph),
            ensures
                result.spec_len() == graph.spec_len(),
                result.spec_index(source as int) == 0usize,
                spec_distances_bounded(&result, graph.spec_len() as int),
        ;

        /// Algorithm 54.6: BFS Tree. Returns parent array and BFS-order vertex sequence.
        /// - APAS: Work O(|V| + |E|), Span O(d·lg n)
        fn bfs_tree(graph: &ArraySeqMtPerS<ArraySeqMtPerS<N>>, source: N) -> (result: BFSTreeS)
            requires
                source < graph.spec_len(),
                graph.spec_len() > 0,
                graph.spec_len() < usize::MAX,
                spec_wf_graph(graph),
            ensures
                result.parents.spec_len() == graph.spec_len(),
                result.parents.spec_index(source as int) == source,
                result.order.spec_len() > 0,
                result.order.spec_index(0) == source,
                forall|i: int| #![auto] 0 <= i < result.order.spec_len()
                    ==> result.order.spec_index(i) < graph.spec_len(),
                spec_parents_bounded(&result.parents, graph.spec_len() as int),
        ;
    }

    // 9. impls

    // Parallel frontier processing via fork-join divide-and-conquer.
    // Splits the frontier in half, processes each half in parallel via join().
    fn process_frontier_parallel(
        graph: ArraySeqMtPerS<ArraySeqMtPerS<N>>,
        distances: ArraySeqMtPerS<N>,
        frontier: Vec<N>,
        next_dist: N,
    ) -> (result: (Vec<N>, Vec<Pair<N, N>>))
        requires
            graph.spec_len() > 0,
            graph.spec_len() < usize::MAX,
            spec_wf_graph(&graph),
            distances.spec_len() == graph.spec_len(),
            spec_distances_bounded(&distances, graph.spec_len() as int),
            next_dist < graph.spec_len(),
            forall|j: int| #![auto] 0 <= j < frontier@.len() ==>
                frontier@[j] < graph.spec_len(),
        ensures
            forall|j: int| #![auto] 0 <= j < result.0@.len() ==>
                result.0@[j] < graph.spec_len(),
            forall|j: int| #![auto] 0 <= j < result.1@.len() ==>
                result.1@[j].0 < graph.spec_len()
                && result.1@[j].1 == next_dist
                && distances.spec_index(result.1@[j].0 as int) == UNREACHABLE,
        decreases frontier@.len()
    {
        let n = graph.length();

        if frontier.len() == 0 {
            return (Vec::new(), Vec::new());
        }

        if frontier.len() == 1 {
            let u = frontier[0];
            let neighbors = graph.nth(u);
            let num_neighbors = neighbors.length();
            let mut next_verts: Vec<N> = Vec::new();
            let mut updates: Vec<Pair<N, N>> = Vec::new();
            let mut i: usize = 0;

            while i < num_neighbors
                invariant
                    0 <= i <= num_neighbors,
                    num_neighbors == neighbors.spec_len(),
                    n == graph.spec_len(),
                    u < n,
                    spec_wf_graph(&graph),
                    *neighbors == graph.spec_index(u as int),
                    next_dist < n,
                    distances.spec_len() == n,
                    spec_distances_bounded(&distances, n as int),
                    forall|j: int| #![auto] 0 <= j < next_verts@.len() ==>
                        next_verts@[j] < n,
                    forall|j: int| #![auto] 0 <= j < updates@.len() ==>
                        updates@[j].0 < n
                        && updates@[j].1 == next_dist
                        && distances.spec_index(updates@[j].0 as int) == UNREACHABLE,
                decreases num_neighbors - i
            {
                let v = *neighbors.nth(i);
                if *distances.nth(v) == UNREACHABLE {
                    next_verts.push(v);
                    updates.push(Pair(v, next_dist));
                }
                i = i + 1;
            }

            return (next_verts, updates);
        }

        // Parallel case: split frontier and process halves via join().
        let mut left_frontier = frontier;
        let mid = left_frontier.len() / 2;
        let right_frontier = left_frontier.split_off(mid);

        // Build owned copies of graph and distances for the spawned closure.
        let graph_copy = copy_graph(&graph);
        let distances_copy = copy_distances(&distances);

        proof {
            lemma_copy_preserves_wf(&graph, &graph_copy);
            lemma_copy_preserves_bounded(&distances, &distances_copy, n as int);
        }

        // Ghost views from the originals for connecting closure ensures back.
        let ghost n_spec: int = graph.spec_len() as int;
        let ghost dist_fn: spec_fn(int) -> N = |i: int| distances.spec_index(i);

        // f1 processes the left half using copies.
        let f1 = move || -> (r: (Vec<N>, Vec<Pair<N, N>>))
            ensures
                forall|j: int| #![auto] 0 <= j < r.0@.len() ==> (r.0@[j] as int) < n_spec,
                forall|j: int| #![auto] 0 <= j < r.1@.len() ==>
                    (r.1@[j].0 as int) < n_spec
                    && r.1@[j].1 == next_dist
                    && dist_fn(r.1@[j].0 as int) == UNREACHABLE,
        {
            let r = process_frontier_parallel(graph_copy, distances_copy, left_frontier, next_dist);
            proof {
                // process_frontier_parallel ensures facts about distances_copy.
                // We proved distances_copy.spec_index(i) == distances.spec_index(i),
                // and dist_fn(i) == distances.spec_index(i) by definition.
                assert forall|j: int| #![auto] 0 <= j < r.1@.len()
                implies dist_fn(r.1@[j].0 as int) == UNREACHABLE
                by {}
            }
            r
        };

        // f2 processes the right half using originals (moved).
        let f2 = move || -> (r: (Vec<N>, Vec<Pair<N, N>>))
            ensures
                forall|j: int| #![auto] 0 <= j < r.0@.len() ==> (r.0@[j] as int) < n_spec,
                forall|j: int| #![auto] 0 <= j < r.1@.len() ==>
                    (r.1@[j].0 as int) < n_spec
                    && r.1@[j].1 == next_dist
                    && dist_fn(r.1@[j].0 as int) == UNREACHABLE,
        {
            let r = process_frontier_parallel(graph, distances, right_frontier, next_dist);
            proof {
                assert forall|j: int| #![auto] 0 <= j < r.1@.len()
                implies dist_fn(r.1@[j].0 as int) == UNREACHABLE
                by {}
            }
            r
        };

        let ((left_verts, left_updates), (right_verts, right_updates)) = join(f1, f2);

        // Combine vertex results.
        let mut all_verts = left_verts;
        let mut k: usize = 0;
        while k < right_verts.len()
            invariant
                0 <= k <= right_verts@.len(),
                forall|j: int| #![auto] 0 <= j < all_verts@.len() ==>
                    (all_verts@[j] as int) < n_spec,
                forall|j: int| #![auto] 0 <= j < right_verts@.len() ==>
                    (right_verts@[j] as int) < n_spec,
            decreases right_verts@.len() - k
        {
            all_verts.push(right_verts[k]);
            k = k + 1;
        }

        // Combine update results.
        let mut all_updates = left_updates;
        let mut k2: usize = 0;
        while k2 < right_updates.len()
            invariant
                0 <= k2 <= right_updates@.len(),
                forall|j: int| #![auto] 0 <= j < all_updates@.len() ==>
                    (all_updates@[j].0 as int) < n_spec
                    && all_updates@[j].1 == next_dist
                    && dist_fn(all_updates@[j].0 as int) == UNREACHABLE,
                forall|j: int| #![auto] 0 <= j < right_updates@.len() ==>
                    (right_updates@[j].0 as int) < n_spec
                    && right_updates@[j].1 == next_dist
                    && dist_fn(right_updates@[j].0 as int) == UNREACHABLE,
            decreases right_updates@.len() - k2
        {
            all_updates.push(right_updates[k2]);
            k2 = k2 + 1;
        }

        (all_verts, all_updates)
    }

    #[verifier::exec_allows_no_decreases_clause]
    pub fn bfs(graph: &ArraySeqMtPerS<ArraySeqMtPerS<N>>, source: N) -> (result: ArraySeqMtPerS<N>)
        requires
            source < graph.spec_len(),
            graph.spec_len() > 0,
            graph.spec_len() < usize::MAX,
            spec_wf_graph(graph),
        ensures
            result.spec_len() == graph.spec_len(),
            result.spec_index(source as int) == 0usize,
            spec_distances_bounded(&result, graph.spec_len() as int),
    {
        let n = graph.length();

        let mut distances = ArraySeqMtPerS::tabulate(
            &|_idx: usize| -> (r: N) ensures r == UNREACHABLE { UNREACHABLE },
            n,
        );

        proof { lemma_tabulate_all_unreachable(&distances, n as int); }

        let old_d = distances;
        distances = ArraySeqMtPerS::update(&old_d, source, 0);

        proof { lemma_update_preserves_bounded(&distances, &old_d, source as int, 0, n as int); }

        let mut current_layer: Vec<N> = Vec::new();
        current_layer.push(source);
        let mut current_dist: N = 0;

        while current_layer.len() > 0
            invariant
                n as int == graph.spec_len(),
                distances.spec_len() == n as int,
                source < n,
                n > 0,
                n < usize::MAX,
                spec_wf_graph(graph),
                distances.spec_index(source as int) == 0usize,
                spec_distances_bounded(&distances, n as int),
                forall|j: int| #![auto] 0 <= j < current_layer@.len() ==>
                    current_layer@[j] < n,
                current_dist < n,
        {
            if current_dist + 1 < n {
                // Build owned copies for parallel frontier processing.
                let graph_owned = copy_graph(graph);
                let distances_snapshot = copy_distances(&distances);

                proof {
                    lemma_copy_preserves_wf(graph, &graph_owned);
                    lemma_copy_preserves_bounded(&distances, &distances_snapshot, n as int);
                }

                let (next_vertices, distance_updates) =
                    process_frontier_parallel(
                        graph_owned, distances_snapshot, current_layer, current_dist + 1
                    );

                // The ensures of process_frontier_parallel gives us facts about
                // distances_snapshot. Connect back to the actual distances.
                proof {
                    assert forall|j: int| #![auto] 0 <= j < distance_updates@.len()
                    implies
                        distances.spec_index(distance_updates@[j].0 as int) == UNREACHABLE
                        && distance_updates@[j].0 < n
                        && distance_updates@[j].1 == current_dist + 1
                    by {
                        // distances_snapshot.spec_index(v) == distances.spec_index(v)
                        // from copy_distances ensures
                    }
                }

                // All update vertices are distinct from source.
                proof {
                    assert forall|j: int| #![auto] 0 <= j < distance_updates@.len()
                    implies distance_updates@[j].0 != source
                    by {
                        assert(distances.spec_index(distance_updates@[j].0 as int) == UNREACHABLE);
                        assert(distances.spec_index(source as int) == 0usize);
                    }
                }

                // Apply distance updates sequentially.
                let mut k: usize = 0;
                while k < distance_updates.len()
                    invariant
                        0 <= k <= distance_updates@.len(),
                        distances.spec_len() == n as int,
                        n as int == graph.spec_len(),
                        source < n,
                        n > 0,
                        n < usize::MAX,
                        spec_wf_graph(graph),
                        distances.spec_index(source as int) == 0usize,
                        spec_distances_bounded(&distances, n as int),
                        current_dist + 1 < n,
                        forall|j: int| #![auto] 0 <= j < distance_updates@.len() ==>
                            distance_updates@[j].0 < graph.spec_len()
                            && distance_updates@[j].1 == current_dist + 1
                            && distance_updates@[j].0 != source,
                    decreases distance_updates@.len() - k
                {
                    let pair = &distance_updates[k];
                    let v = pair.0;
                    let d = pair.1;

                    let old_d_inner = distances;
                    distances = ArraySeqMtPerS::update(&old_d_inner, v, d);

                    proof {
                        lemma_update_preserves_bounded(
                            &distances, &old_d_inner,
                            v as int, d, n as int,
                        );
                        assert(distances.spec_index(source as int) == old_d_inner.spec_index(source as int));
                    }

                    k = k + 1;
                }

                current_layer = next_vertices;
                current_dist = current_dist + 1;
            } else {
                current_layer = Vec::new();
            }
        }

        distances
    }

    // Parallel frontier processing for BFS tree: collects (neighbor, parent) pairs.
    fn process_frontier_tree_parallel(
        graph: ArraySeqMtPerS<ArraySeqMtPerS<N>>,
        parents: ArraySeqMtPerS<N>,
        frontier: Vec<N>,
    ) -> (result: Vec<Pair<N, N>>)
        requires
            graph.spec_len() > 0,
            graph.spec_len() < usize::MAX,
            spec_wf_graph(&graph),
            parents.spec_len() == graph.spec_len(),
            spec_parents_bounded(&parents, graph.spec_len() as int),
            forall|j: int| #![auto] 0 <= j < frontier@.len() ==>
                frontier@[j] < graph.spec_len(),
        ensures
            forall|j: int| #![auto] 0 <= j < result@.len() ==>
                result@[j].0 < graph.spec_len()
                && result@[j].1 < graph.spec_len()
                && parents.spec_index(result@[j].0 as int) == NO_PARENT,
        decreases frontier@.len()
    {
        let n = graph.length();

        if frontier.len() == 0 {
            return Vec::new();
        }

        if frontier.len() == 1 {
            let u = frontier[0];
            let neighbors = graph.nth(u);
            let num_neighbors = neighbors.length();
            let mut updates: Vec<Pair<N, N>> = Vec::new();
            let mut i: usize = 0;

            while i < num_neighbors
                invariant
                    0 <= i <= num_neighbors,
                    num_neighbors == neighbors.spec_len(),
                    n == graph.spec_len(),
                    u < n,
                    spec_wf_graph(&graph),
                    *neighbors == graph.spec_index(u as int),
                    parents.spec_len() == n,
                    spec_parents_bounded(&parents, n as int),
                    forall|j: int| #![auto] 0 <= j < updates@.len() ==>
                        updates@[j].0 < n
                        && updates@[j].1 < n
                        && parents.spec_index(updates@[j].0 as int) == NO_PARENT,
                decreases num_neighbors - i
            {
                let v = *neighbors.nth(i);
                if *parents.nth(v) == NO_PARENT {
                    updates.push(Pair(v, u));
                }
                i = i + 1;
            }

            return updates;
        }

        let mut left_frontier = frontier;
        let mid = left_frontier.len() / 2;
        let right_frontier = left_frontier.split_off(mid);

        let graph_copy = copy_graph(&graph);
        let parents_copy = copy_distances(&parents);

        proof {
            lemma_copy_preserves_wf(&graph, &graph_copy);
            lemma_copy_preserves_parents_bounded(&parents, &parents_copy, n as int);
        }

        let ghost n_spec: int = graph.spec_len() as int;
        let ghost parents_fn: spec_fn(int) -> N = |i: int| parents.spec_index(i);

        let f1 = move || -> (r: Vec<Pair<N, N>>)
            ensures
                forall|j: int| #![auto] 0 <= j < r@.len() ==>
                    (r@[j].0 as int) < n_spec
                    && (r@[j].1 as int) < n_spec
                    && parents_fn(r@[j].0 as int) == NO_PARENT,
        {
            let r = process_frontier_tree_parallel(graph_copy, parents_copy, left_frontier);
            proof {
                assert forall|j: int| #![auto] 0 <= j < r@.len()
                implies parents_fn(r@[j].0 as int) == NO_PARENT
                by {}
            }
            r
        };

        let f2 = move || -> (r: Vec<Pair<N, N>>)
            ensures
                forall|j: int| #![auto] 0 <= j < r@.len() ==>
                    (r@[j].0 as int) < n_spec
                    && (r@[j].1 as int) < n_spec
                    && parents_fn(r@[j].0 as int) == NO_PARENT,
        {
            let r = process_frontier_tree_parallel(graph, parents, right_frontier);
            proof {
                assert forall|j: int| #![auto] 0 <= j < r@.len()
                implies parents_fn(r@[j].0 as int) == NO_PARENT
                by {}
            }
            r
        };

        let (left_updates, right_updates) = join(f1, f2);

        let mut all_updates = left_updates;
        let mut k: usize = 0;
        while k < right_updates.len()
            invariant
                0 <= k <= right_updates@.len(),
                forall|j: int| #![auto] 0 <= j < all_updates@.len() ==>
                    (all_updates@[j].0 as int) < n_spec
                    && (all_updates@[j].1 as int) < n_spec
                    && parents_fn(all_updates@[j].0 as int) == NO_PARENT,
                forall|j: int| #![auto] 0 <= j < right_updates@.len() ==>
                    (right_updates@[j].0 as int) < n_spec
                    && (right_updates@[j].1 as int) < n_spec
                    && parents_fn(right_updates@[j].0 as int) == NO_PARENT,
            decreases right_updates@.len() - k
        {
            all_updates.push(right_updates[k]);
            k = k + 1;
        }

        all_updates
    }

    /// Algorithm 54.6: BFS Tree with parallel frontier processing.
    #[verifier::exec_allows_no_decreases_clause]
    pub fn bfs_tree(graph: &ArraySeqMtPerS<ArraySeqMtPerS<N>>, source: N) -> (result: BFSTreeS)
        requires
            source < graph.spec_len(),
            graph.spec_len() > 0,
            graph.spec_len() < usize::MAX,
            spec_wf_graph(graph),
        ensures
            result.parents.spec_len() == graph.spec_len(),
            result.parents.spec_index(source as int) == source,
            result.order.spec_len() > 0,
            result.order.spec_index(0) == source,
            forall|i: int| #![auto] 0 <= i < result.order.spec_len()
                ==> result.order.spec_index(i) < graph.spec_len(),
            spec_parents_bounded(&result.parents, graph.spec_len() as int),
    {
        let n = graph.length();

        let mut parents = ArraySeqMtPerS::tabulate(
            &|_idx: usize| -> (r: N) ensures r == NO_PARENT { NO_PARENT },
            n,
        );

        proof { lemma_tabulate_all_no_parent(&parents, n as int); }

        let old_p = parents;
        parents = ArraySeqMtPerS::update(&old_p, source, source);

        proof { lemma_update_preserves_parents_bounded(&parents, &old_p, source as int, source, n as int); }

        let mut current_layer: Vec<N> = Vec::new();
        current_layer.push(source);

        let mut order: Vec<N> = Vec::new();
        order.push(source);

        while current_layer.len() > 0
            invariant
                n as int == graph.spec_len(),
                parents.spec_len() == n as int,
                source < n,
                n > 0,
                n < usize::MAX,
                spec_wf_graph(graph),
                parents.spec_index(source as int) == source,
                spec_parents_bounded(&parents, n as int),
                forall|j: int| #![auto] 0 <= j < current_layer@.len() ==>
                    current_layer@[j] < n,
                order@.len() > 0,
                order@[0] == source,
                forall|j: int| #![auto] 0 <= j < order@.len() ==> order@[j] < n,
        {
            let graph_owned = copy_graph(graph);
            let parents_snapshot = copy_distances(&parents);

            proof {
                lemma_copy_preserves_wf(graph, &graph_owned);
                lemma_copy_preserves_parents_bounded(&parents, &parents_snapshot, n as int);
            }

            let tree_updates =
                process_frontier_tree_parallel(
                    graph_owned, parents_snapshot, current_layer
                );

            proof {
                assert forall|j: int| #![auto] 0 <= j < tree_updates@.len()
                implies
                    parents.spec_index(tree_updates@[j].0 as int) == NO_PARENT
                    && tree_updates@[j].0 < n
                    && tree_updates@[j].1 < n
                by {}
            }

            // Apply updates with deduplication: first write to each vertex wins.
            let mut next_layer: Vec<N> = Vec::new();
            let mut k: usize = 0;
            while k < tree_updates.len()
                invariant
                    0 <= k <= tree_updates@.len(),
                    parents.spec_len() == n as int,
                    n as int == graph.spec_len(),
                    source < n,
                    n > 0,
                    n < usize::MAX,
                    spec_wf_graph(graph),
                    parents.spec_index(source as int) == source,
                    spec_parents_bounded(&parents, n as int),
                    forall|j: int| #![auto] 0 <= j < tree_updates@.len() ==>
                        tree_updates@[j].0 < n && tree_updates@[j].1 < n,
                    forall|j: int| #![auto] 0 <= j < next_layer@.len() ==>
                        next_layer@[j] < n,
                    order@.len() > 0,
                    order@[0] == source,
                    forall|j: int| #![auto] 0 <= j < order@.len() ==> order@[j] < n,
                decreases tree_updates@.len() - k
            {
                let pair = &tree_updates[k];
                let v = pair.0;
                let u = pair.1;

                if *parents.nth(v) == NO_PARENT {
                    let old_p_inner = parents;
                    parents = ArraySeqMtPerS::update(&old_p_inner, v, u);
                    next_layer.push(v);
                    order.push(v);

                    proof {
                        lemma_update_preserves_parents_bounded(
                            &parents, &old_p_inner,
                            v as int, u, n as int,
                        );
                        assert(parents.spec_index(source as int) == source) by {
                            if v as int == source as int {
                                assert(old_p_inner.spec_index(source as int) == source);
                                assert(source < n);
                                assert(n < usize::MAX);
                            }
                        };
                    }
                }

                k = k + 1;
            }

            current_layer = next_layer;
        }

        let order_seq = ArraySeqMtPerS::from_vec(order);
        BFSTreeS { parents, order: order_seq }
    }

    impl BFSTreeS {
        /// Vertices in BFS order (root first, then distance 1, 2, ...).
        pub fn top_down_order(&self) -> (result: &ArraySeqMtPerS<N>)
            ensures
                result.spec_len() == self.order.spec_len(),
                forall|i: int| #![auto] 0 <= i < result.spec_len() ==>
                    result.spec_index(i) == self.order.spec_index(i),
        {
            &self.order
        }

        /// Vertices in reverse BFS order (furthest from root first).
        pub fn bottom_up_order(&self) -> (result: ArraySeqMtPerS<N>)
            requires self.order.spec_len() <= usize::MAX,
            ensures
                result.spec_len() == self.order.spec_len(),
                forall|i: int| #![auto] 0 <= i < result.spec_len() ==>
                    result.spec_index(i) == self.order.spec_index(self.order.spec_len() - 1 - i),
        {
            let n = self.order.length();
            ArraySeqMtPerS::tabulate(
                &|i: usize| -> (r: N)
                    requires i < n, n == self.order.spec_len()
                    ensures r == self.order.spec_index((n - 1 - i) as int)
                { *self.order.nth(n - 1 - i) },
                n,
            )
        }
    }

    } // verus!
}
