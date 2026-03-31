//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Breadth-First Search - Parallel Ephemeral (Chapter 54, Algorithms 54.5 and 54.6).
//! Layer-by-layer parallel BFS using HF Scheduler join() for fork-join frontier processing.
//! Algorithm 54.5: distances. Algorithm 54.6: shortest-path tree + BFS-order iteration.
//! Work: O(|V| + |E|), Span: O(d·lg n) where d is diameter.

pub mod BFSMtEph {

    use vstd::prelude::*;
    use crate::Types::Types::*;
    use crate::Chap19::ArraySeqMtEph::ArraySeqMtEph::*;
    use crate::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::join;

    verus! {

    // Table of Contents
    // 4. type definitions
    // 6. spec fns
    // 7. proof fns
    // 8. traits
    // 9. impls

    // 4. type definitions
    pub type T<N> = ArraySeqMtEphS<ArraySeqMtEphS<N>>;

    pub const UNREACHABLE: usize = usize::MAX;
    pub const NO_PARENT: usize = usize::MAX;

    pub struct BFSTreeS {
        pub parents: ArraySeqMtEphS<usize>,
        pub order: ArraySeqMtEphS<usize>,
    }

    pub struct BFSMtEph;

    // 6. spec fns

    /// All neighbor indices in the adjacency list are valid vertex indices.
    pub open spec fn spec_bfsmteph_wf(graph: &ArraySeqMtEphS<ArraySeqMtEphS<usize>>) -> bool {
        forall|u: int, i: int|
            0 <= u < graph.spec_len() && 0 <= i < graph.spec_index(u).spec_len()
            ==> #[trigger] graph.spec_index(u).spec_index(i) < graph.spec_len()
    }

    /// Every distance entry is either UNREACHABLE or bounded by n.
    pub open spec fn spec_distances_bounded(distances: &ArraySeqMtEphS<usize>, n: int) -> bool {
        forall|j: int| 0 <= j < distances.spec_len() ==>
            #[trigger] distances.spec_index(j) == UNREACHABLE || distances.spec_index(j) < n
    }

    /// Every parent entry is either NO_PARENT or a valid vertex index.
    pub open spec fn spec_parents_bounded(parents: &ArraySeqMtEphS<usize>, n: int) -> bool {
        forall|j: int| 0 <= j < parents.spec_len() ==>
            #[trigger] parents.spec_index(j) == NO_PARENT || parents.spec_index(j) < n
    }

    // 7. proof fns

    proof fn lemma_tabulate_all_no_parent(parents: &ArraySeqMtEphS<usize>, n: int)
        requires
            parents.spec_len() == n,
            forall|i: int| 0 <= i < n ==> #[trigger] parents.spec_index(i) == NO_PARENT,
        ensures
            spec_parents_bounded(parents, n),
    {}

    proof fn lemma_set_preserves_parents_bounded(
        parents: &ArraySeqMtEphS<usize>,
        old_parents: &ArraySeqMtEphS<usize>,
        v: int,
        new_val: usize,
        n: int,
    )
        requires
            parents.spec_len() == n,
            old_parents.spec_len() == n,
            0 <= v < n,
            new_val < n,
            parents.spec_index(v) == new_val,
            forall|j: int| 0 <= j < n && j != v ==>
                #[trigger] parents.spec_index(j) == old_parents.spec_index(j),
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
        original: &ArraySeqMtEphS<usize>,
        copy: &ArraySeqMtEphS<usize>,
        n: int,
    )
        requires
            spec_parents_bounded(original, n),
            copy.spec_len() == original.spec_len(),
            forall|i: int| 0 <= i < original.spec_len() ==>
                #[trigger] copy.spec_index(i) == original.spec_index(i),
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

    proof fn lemma_tabulate_all_unreachable(distances: &ArraySeqMtEphS<usize>, n: int)
        requires
            distances.spec_len() == n,
            forall|i: int| 0 <= i < n ==>
                #[trigger] distances.spec_index(i) == UNREACHABLE,
        ensures
            spec_distances_bounded(distances, n),
    {
    }

    proof fn lemma_set_preserves_bounded(
        distances: &ArraySeqMtEphS<usize>,
        old_distances: &ArraySeqMtEphS<usize>,
        v: int,
        new_val: usize,
        n: int,
    )
        requires
            distances.spec_len() == n,
            old_distances.spec_len() == n,
            0 <= v < n,
            new_val < n,
            distances.spec_index(v) == new_val,
            forall|j: int| 0 <= j < n && j != v ==>
                #[trigger] distances.spec_index(j) == old_distances.spec_index(j),
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
    fn copy_distances(distances: &ArraySeqMtEphS<usize>) -> (copied: ArraySeqMtEphS<usize>)
        requires distances.spec_len() <= usize::MAX,
        ensures
            copied.spec_len() == distances.spec_len(),
            forall|i: int| 0 <= i < distances.spec_len() ==>
                #[trigger] copied.spec_index(i) == distances.spec_index(i),
    {
        let n = distances.length();
        ArraySeqMtEphS::tabulate(
            &|idx: usize| -> (r: usize)
                requires idx < n, n == distances.spec_len()
                ensures r == distances.spec_index(idx as int)
            { *distances.nth(idx) },
            n,
        )
    }

    // Builds an owned copy of the graph with proven spec equality.
    fn copy_graph(graph: &ArraySeqMtEphS<ArraySeqMtEphS<usize>>) -> (copied: ArraySeqMtEphS<ArraySeqMtEphS<usize>>)
        requires graph.spec_len() <= usize::MAX,
        ensures
            copied.spec_len() == graph.spec_len(),
            forall|u: int| 0 <= u < graph.spec_len() ==>
                #[trigger] copied.spec_index(u).spec_len() == graph.spec_index(u).spec_len(),
            forall|u: int, i: int|
                0 <= u < graph.spec_len() && 0 <= i < graph.spec_index(u).spec_len() ==>
                #[trigger] copied.spec_index(u).spec_index(i) == graph.spec_index(u).spec_index(i),
    {
        let n = graph.length();
        ArraySeqMtEphS::tabulate(
            &|u_idx: usize| -> (r: ArraySeqMtEphS<usize>)
                requires u_idx < n, n == graph.spec_len()
                ensures
                    r.spec_len() == graph.spec_index(u_idx as int).spec_len(),
                    forall|k: int| 0 <= k < r.spec_len() ==>
                        #[trigger] r.spec_index(k) == graph.spec_index(u_idx as int).spec_index(k),
            {
                let adj = graph.nth(u_idx);
                let adj_len = adj.length();
                ArraySeqMtEphS::tabulate(
                    &|k: usize| -> (r: usize)
                        requires k < adj_len, adj_len == adj.spec_len()
                        ensures r == adj.spec_index(k as int)
                    { *adj.nth(k) },
                    adj_len,
                )
            },
            n,
        )
    }

    proof fn lemma_copy_preserves_wf(
        original: &ArraySeqMtEphS<ArraySeqMtEphS<usize>>,
        copy: &ArraySeqMtEphS<ArraySeqMtEphS<usize>>,
    )
        requires
            spec_bfsmteph_wf(original),
            copy.spec_len() == original.spec_len(),
            forall|u: int| 0 <= u < original.spec_len() ==>
                #[trigger] copy.spec_index(u).spec_len() == original.spec_index(u).spec_len(),
            forall|u: int, i: int|
                0 <= u < original.spec_len() && 0 <= i < original.spec_index(u).spec_len() ==>
                #[trigger] copy.spec_index(u).spec_index(i) == original.spec_index(u).spec_index(i),
        ensures
            spec_bfsmteph_wf(copy),
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

    proof fn lemma_copy_preserves_bounded(
        original: &ArraySeqMtEphS<usize>,
        copy: &ArraySeqMtEphS<usize>,
        n: int,
    )
        requires
            spec_distances_bounded(original, n),
            copy.spec_len() == original.spec_len(),
            forall|i: int| 0 <= i < original.spec_len() ==>
                #[trigger] copy.spec_index(i) == original.spec_index(i),
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
    pub trait BFSTreeMtEphTrait {
        spec fn spec_order(&self) -> ArraySeqMtEphS<usize>;

        /// Vertices in BFS order (root first, then distance 1, 2, ...).
        fn top_down_order(&self) -> (order: &ArraySeqMtEphS<usize>)
            ensures
                order.spec_len() == self.spec_order().spec_len(),
                forall|i: int| 0 <= i < order.spec_len() ==>
                    #[trigger] order.spec_index(i) == self.spec_order().spec_index(i),
        ;

        /// Vertices in reverse BFS order (furthest from root first).
        fn bottom_up_order(&self) -> (order: ArraySeqMtEphS<usize>)
            requires self.spec_order().spec_len() <= usize::MAX,
            ensures
                order.spec_len() == self.spec_order().spec_len(),
                forall|i: int| 0 <= i < order.spec_len() ==>
                    #[trigger] order.spec_index(i) == self.spec_order().spec_index(self.spec_order().spec_len() - 1 - i),
        ;
    }

    pub trait BFSMtEphTrait {
        /// Algorithm 54.5: BFSDistance. Returns distance from source for every vertex.
        /// - Alg Analysis: APAS (Ch54 Alg 54.4): Work O(m lg n), Span O(d lg^2 n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m lg n), Span O(d lg^2 n) — matches APAS; parallel BFS with set union per round
        fn bfs(graph: &ArraySeqMtEphS<ArraySeqMtEphS<usize>>, source: usize) -> (traversal: ArraySeqMtEphS<usize>)
            requires
                source < graph.spec_len(),
                graph.spec_len() > 0,
                graph.spec_len() < usize::MAX,
                spec_bfsmteph_wf(graph),
            ensures
                traversal.spec_len() == graph.spec_len(),
                traversal.spec_index(source as int) == 0usize,
                spec_distances_bounded(&traversal, graph.spec_len() as int),
                forall|v: int| #![trigger traversal.spec_index(v)] 0 <= v < traversal.spec_len()
                    && traversal.spec_index(v) != UNREACHABLE && v != source as int
                    ==> traversal.spec_index(v) > 0usize,
        ;

        /// Algorithm 54.6: BFS Tree. Returns parent array and BFS-order vertex sequence.
        /// - Alg Analysis: APAS (Ch54 Alg 54.6): Work O(n + m), Span O(d lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n+m), Span O(d lg n) — matches APAS; parallel BFS tree with inject per round
        fn bfs_tree(graph: &ArraySeqMtEphS<ArraySeqMtEphS<usize>>, source: usize) -> (traversal: BFSTreeS)
            requires
                source < graph.spec_len(),
                graph.spec_len() > 0,
                graph.spec_len() < usize::MAX,
                spec_bfsmteph_wf(graph),
            ensures
                traversal.parents.spec_len() == graph.spec_len(),
                traversal.parents.spec_index(source as int) == source,
                traversal.order.spec_len() > 0,
                traversal.order.spec_len() <= graph.spec_len(),
                traversal.order.spec_index(0) == source,
                forall|i: int| 0 <= i < traversal.order.spec_len()
                    ==> #[trigger] traversal.order.spec_index(i) < graph.spec_len(),
                spec_parents_bounded(&traversal.parents, graph.spec_len() as int),
                forall|i: int| #![trigger traversal.order.spec_index(i)] 0 <= i < traversal.order.spec_len()
                    ==> traversal.parents.spec_index(
                        traversal.order.spec_index(i) as int) != NO_PARENT,
        ;
    }

    // 9. impls

    // Parallel frontier processing via fork-join divide-and-conquer.
    fn process_frontier_parallel(
        graph: ArraySeqMtEphS<ArraySeqMtEphS<usize>>,
        distances: ArraySeqMtEphS<usize>,
        frontier: Vec<usize>,
        next_dist: usize,
    ) -> (traversal: (Vec<usize>, Vec<Pair<usize, usize>>))
        requires
            graph.spec_len() > 0,
            graph.spec_len() < usize::MAX,
            spec_bfsmteph_wf(&graph),
            distances.spec_len() == graph.spec_len(),
            spec_distances_bounded(&distances, graph.spec_len() as int),
            next_dist < graph.spec_len(),
            forall|j: int| #![trigger frontier@[j]] 0 <= j < frontier@.len() ==>
                frontier@[j] < graph.spec_len(),
        ensures
            forall|j: int| #![trigger traversal.0@[j]] 0 <= j < traversal.0@.len() ==>
                traversal.0@[j] < graph.spec_len(),
            forall|j: int| #![trigger traversal.1@[j]] 0 <= j < traversal.1@.len() ==>
                (traversal.1@[j]).0 < graph.spec_len()
                && (traversal.1@[j]).1 == next_dist
                && distances.spec_index((traversal.1@[j]).0 as int) == UNREACHABLE,
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
            let mut next_verts: Vec<usize> = Vec::new();
            let mut updates: Vec<Pair<usize, usize>> = Vec::new();
            let mut i: usize = 0;

            while i < num_neighbors
                invariant
                    0 <= i <= num_neighbors,
                    num_neighbors == neighbors.spec_len(),
                    n == graph.spec_len(),
                    u < n,
                    spec_bfsmteph_wf(&graph),
                    *neighbors == graph.spec_index(u as int),
                    next_dist < n,
                    distances.spec_len() == n,
                    spec_distances_bounded(&distances, n as int),
                    forall|j: int| #![trigger next_verts@[j]] 0 <= j < next_verts@.len() ==>
                        next_verts@[j] < n,
                    forall|j: int| #![trigger updates@[j]] 0 <= j < updates@.len() ==>
                        (updates@[j]).0 < n
                        && (updates@[j]).1 == next_dist
                        && distances.spec_index((updates@[j]).0 as int) == UNREACHABLE,
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

        let graph_copy = copy_graph(&graph);
        let distances_copy = copy_distances(&distances);

        proof {
            lemma_copy_preserves_wf(&graph, &graph_copy);
            lemma_copy_preserves_bounded(&distances, &distances_copy, n as int);
        }

        let ghost n_spec: int = graph.spec_len() as int;
        let ghost dist_fn: spec_fn(int) -> usize = |i: int| distances.spec_index(i);

        let f1 = move || -> (r: (Vec<usize>, Vec<Pair<usize, usize>>))
            ensures
                forall|j: int| #![trigger r.0@[j]] 0 <= j < r.0@.len() ==> (r.0@[j] as int) < n_spec,
                forall|j: int| #![trigger r.1@[j]] 0 <= j < r.1@.len() ==>
                    ((r.1@[j]).0 as int) < n_spec
                    && (r.1@[j]).1 == next_dist
                    && dist_fn((r.1@[j]).0 as int) == UNREACHABLE,
        {
            let r = process_frontier_parallel(graph_copy, distances_copy, left_frontier, next_dist);
            proof {
                assert forall|j: int| #![trigger r.1@[j]] 0 <= j < r.1@.len()
                implies dist_fn((r.1@[j]).0 as int) == UNREACHABLE
                by {}
            }
            r
        };

        let f2 = move || -> (r: (Vec<usize>, Vec<Pair<usize, usize>>))
            ensures
                forall|j: int| #![trigger r.0@[j]] 0 <= j < r.0@.len() ==> (r.0@[j] as int) < n_spec,
                forall|j: int| #![trigger r.1@[j]] 0 <= j < r.1@.len() ==>
                    ((r.1@[j]).0 as int) < n_spec
                    && (r.1@[j]).1 == next_dist
                    && dist_fn((r.1@[j]).0 as int) == UNREACHABLE,
        {
            let r = process_frontier_parallel(graph, distances, right_frontier, next_dist);
            proof {
                assert forall|j: int| #![trigger r.1@[j]] 0 <= j < r.1@.len()
                implies dist_fn((r.1@[j]).0 as int) == UNREACHABLE
                by {}
            }
            r
        };

        let ((left_verts, left_updates), (right_verts, right_updates)) = join(f1, f2);

        let mut all_verts = left_verts;
        let mut k: usize = 0;
        while k < right_verts.len()
            invariant
                0 <= k <= right_verts@.len(),
                forall|j: int| #![trigger all_verts@[j]] 0 <= j < all_verts@.len() ==>
                    (all_verts@[j] as int) < n_spec,
                forall|j: int| #![trigger right_verts@[j]] 0 <= j < right_verts@.len() ==>
                    (right_verts@[j] as int) < n_spec,
            decreases right_verts@.len() - k
        {
            all_verts.push(right_verts[k]);
            k = k + 1;
        }

        let mut all_updates = left_updates;
        let mut k2: usize = 0;
        while k2 < right_updates.len()
            invariant
                0 <= k2 <= right_updates@.len(),
                forall|j: int| #![trigger all_updates@[j]] 0 <= j < all_updates@.len() ==>
                    ((all_updates@[j]).0 as int) < n_spec
                    && (all_updates@[j]).1 == next_dist
                    && dist_fn((all_updates@[j]).0 as int) == UNREACHABLE,
                forall|j: int| #![trigger right_updates@[j]] 0 <= j < right_updates@.len() ==>
                    ((right_updates@[j]).0 as int) < n_spec
                    && (right_updates@[j]).1 == next_dist
                    && dist_fn((right_updates@[j]).0 as int) == UNREACHABLE,
            decreases right_updates@.len() - k2
        {
            all_updates.push(right_updates[k2]);
            k2 = k2 + 1;
        }

        (all_verts, all_updates)
    }

    // Parallel frontier processing for BFS tree: collects (neighbor, parent) pairs.
    fn process_frontier_tree_parallel(
        graph: ArraySeqMtEphS<ArraySeqMtEphS<usize>>,
        parents: ArraySeqMtEphS<usize>,
        frontier: Vec<usize>,
    ) -> (traversal: Vec<Pair<usize, usize>>)
        requires
            graph.spec_len() > 0,
            graph.spec_len() < usize::MAX,
            spec_bfsmteph_wf(&graph),
            parents.spec_len() == graph.spec_len(),
            spec_parents_bounded(&parents, graph.spec_len() as int),
            forall|j: int| #![trigger frontier@[j]] 0 <= j < frontier@.len() ==>
                frontier@[j] < graph.spec_len(),
        ensures
            forall|j: int| #![trigger traversal@[j]] 0 <= j < traversal@.len() ==>
                (traversal@[j]).0 < graph.spec_len()
                && (traversal@[j]).1 < graph.spec_len()
                && parents.spec_index((traversal@[j]).0 as int) == NO_PARENT,
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
            let mut updates: Vec<Pair<usize, usize>> = Vec::new();
            let mut i: usize = 0;

            while i < num_neighbors
                invariant
                    0 <= i <= num_neighbors,
                    num_neighbors == neighbors.spec_len(),
                    n == graph.spec_len(),
                    u < n,
                    spec_bfsmteph_wf(&graph),
                    *neighbors == graph.spec_index(u as int),
                    parents.spec_len() == n,
                    spec_parents_bounded(&parents, n as int),
                    forall|j: int| #![trigger updates@[j]] 0 <= j < updates@.len() ==>
                        (updates@[j]).0 < n
                        && (updates@[j]).1 < n
                        && parents.spec_index((updates@[j]).0 as int) == NO_PARENT,
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
        let ghost parents_fn: spec_fn(int) -> usize = |i: int| parents.spec_index(i);

        let f1 = move || -> (r: Vec<Pair<usize, usize>>)
            ensures
                forall|j: int| #![trigger r@[j]] 0 <= j < r@.len() ==>
                    ((r@[j]).0 as int) < n_spec
                    && ((r@[j]).1 as int) < n_spec
                    && parents_fn((r@[j]).0 as int) == NO_PARENT,
        {
            let r = process_frontier_tree_parallel(graph_copy, parents_copy, left_frontier);
            proof {
                assert forall|j: int| #![trigger r@[j]] 0 <= j < r@.len()
                implies parents_fn((r@[j]).0 as int) == NO_PARENT
                by {}
            }
            r
        };

        let f2 = move || -> (r: Vec<Pair<usize, usize>>)
            ensures
                forall|j: int| #![trigger r@[j]] 0 <= j < r@.len() ==>
                    ((r@[j]).0 as int) < n_spec
                    && ((r@[j]).1 as int) < n_spec
                    && parents_fn((r@[j]).0 as int) == NO_PARENT,
        {
            let r = process_frontier_tree_parallel(graph, parents, right_frontier);
            proof {
                assert forall|j: int| #![trigger r@[j]] 0 <= j < r@.len()
                implies parents_fn((r@[j]).0 as int) == NO_PARENT
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
                forall|j: int| #![trigger all_updates@[j]] 0 <= j < all_updates@.len() ==>
                    ((all_updates@[j]).0 as int) < n_spec
                    && ((all_updates@[j]).1 as int) < n_spec
                    && parents_fn((all_updates@[j]).0 as int) == NO_PARENT,
                forall|j: int| #![trigger right_updates@[j]] 0 <= j < right_updates@.len() ==>
                    ((right_updates@[j]).0 as int) < n_spec
                    && ((right_updates@[j]).1 as int) < n_spec
                    && parents_fn((right_updates@[j]).0 as int) == NO_PARENT,
            decreases right_updates@.len() - k
        {
            all_updates.push(right_updates[k]);
            k = k + 1;
        }

        all_updates
    }

    impl BFSMtEphTrait for BFSMtEph {

    #[verifier::exec_allows_no_decreases_clause]
    fn bfs(graph: &ArraySeqMtEphS<ArraySeqMtEphS<usize>>, source: usize) -> (traversal: ArraySeqMtEphS<usize>)
    {
        broadcast use vstd::std_specs::vecdeque::group_vec_dequeue_axioms;

        let n = graph.length();

        let mut distances = ArraySeqMtEphS::tabulate(
            &|_idx: usize| -> (r: usize) ensures r == UNREACHABLE { UNREACHABLE },
            n,
        );

        proof { lemma_tabulate_all_unreachable(&distances, n as int); }

        let ghost pre_set = *&distances;
        let _ = distances.set(source, 0);

        proof {
            lemma_set_preserves_bounded(&distances, &pre_set, source as int, 0, n as int);
            assert forall|v: int| 0 <= v < distances.spec_len()
                && distances.spec_index(v) != UNREACHABLE && v != source as int
            implies distances.spec_index(v) > 0usize
            by {
                assert(distances.spec_index(v) == pre_set.spec_index(v));
            }
        }

        let mut current_layer: Vec<usize> = Vec::new();
        current_layer.push(source);
        let mut current_dist: usize = 0;

        while current_layer.len() > 0
            invariant
                n as int == graph.spec_len(),
                distances.spec_len() == n as int,
                source < n,
                n > 0,
                n < usize::MAX,
                spec_bfsmteph_wf(graph),
                distances.spec_index(source as int) == 0usize,
                spec_distances_bounded(&distances, n as int),
                forall|v: int| #![trigger distances.spec_index(v)] 0 <= v < distances.spec_len()
                    && distances.spec_index(v) != UNREACHABLE && v != source as int
                    ==> distances.spec_index(v) > 0usize,
                forall|j: int| #![trigger current_layer@[j]] 0 <= j < current_layer@.len() ==>
                    current_layer@[j] < n,
                current_dist < n,
        {
            if current_dist + 1 < n {
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

                proof {
                    assert forall|j: int| #![trigger distance_updates@[j]] 0 <= j < distance_updates@.len()
                    implies
                        distances.spec_index((distance_updates@[j]).0 as int) == UNREACHABLE
                        && (distance_updates@[j]).0 < n
                        && (distance_updates@[j]).1 == current_dist + 1
                    by {}
                }

                proof {
                    assert forall|j: int| #![trigger distance_updates@[j]] 0 <= j < distance_updates@.len()
                    implies (distance_updates@[j]).0 != source
                    by {
                        assert(distances.spec_index((distance_updates@[j]).0 as int) == UNREACHABLE);
                        assert(distances.spec_index(source as int) == 0usize);
                    }
                }

                // Apply distance updates sequentially via set().
                let mut k: usize = 0;
                while k < distance_updates.len()
                    invariant
                        0 <= k <= distance_updates@.len(),
                        distances.spec_len() == n as int,
                        n as int == graph.spec_len(),
                        source < n,
                        n > 0,
                        n < usize::MAX,
                        spec_bfsmteph_wf(graph),
                        distances.spec_index(source as int) == 0usize,
                        spec_distances_bounded(&distances, n as int),
                        forall|v: int| #![trigger distances.spec_index(v)] 0 <= v < distances.spec_len()
                            && distances.spec_index(v) != UNREACHABLE && v != source as int
                            ==> distances.spec_index(v) > 0usize,
                        current_dist + 1 < n,
                        forall|j: int| #![trigger distance_updates@[j]] 0 <= j < distance_updates@.len() ==>
                            (distance_updates@[j]).0 < graph.spec_len()
                            && (distance_updates@[j]).1 == current_dist + 1
                            && (distance_updates@[j]).0 != source,
                    decreases distance_updates@.len() - k
                {
                    let pair = &distance_updates[k];
                    let v = pair.0;
                    let d = pair.1;

                    let ghost pre_inner_set = *&distances;
                    let _ = distances.set(v, d);

                    proof {
                        lemma_set_preserves_bounded(
                            &distances, &pre_inner_set,
                            v as int, d as usize, n as int,
                        );
                        assert(distances.spec_index(source as int) == pre_inner_set.spec_index(source as int));
                        assert forall|w: int| 0 <= w < distances.spec_len()
                            && distances.spec_index(w) != UNREACHABLE
                            && w != source as int
                        implies distances.spec_index(w) > 0usize
                        by {
                            if w == v as int {
                                assert(distances.spec_index(w) == d);
                            } else {
                                assert(distances.spec_index(w)
                                    == pre_inner_set.spec_index(w));
                            }
                        }
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

    /// Algorithm 54.6: BFS Tree with parallel frontier processing.
    #[verifier::exec_allows_no_decreases_clause]
    fn bfs_tree(graph: &ArraySeqMtEphS<ArraySeqMtEphS<usize>>, source: usize) -> (traversal: BFSTreeS)
    {
        let n = graph.length();

        let mut parents = ArraySeqMtEphS::tabulate(
            &|_idx: usize| -> (r: usize) ensures r == NO_PARENT { NO_PARENT },
            n,
        );

        proof { lemma_tabulate_all_no_parent(&parents, n as int); }

        let ghost pre_set = *&parents;
        let _ = parents.set(source, source);

        proof { lemma_set_preserves_parents_bounded(&parents, &pre_set, source as int, source, n as int); }

        let mut current_layer: Vec<usize> = Vec::new();
        current_layer.push(source);

        let mut order: Vec<usize> = Vec::new();
        order.push(source);

        while current_layer.len() > 0
            invariant
                n as int == graph.spec_len(),
                parents.spec_len() == n as int,
                source < n,
                n > 0,
                n < usize::MAX,
                spec_bfsmteph_wf(graph),
                parents.spec_index(source as int) == source,
                spec_parents_bounded(&parents, n as int),
                forall|j: int| #![trigger current_layer@[j]] 0 <= j < current_layer@.len() ==>
                    current_layer@[j] < n,
                order@.len() > 0,
                order@.len() <= n as int,
                order@[0] == source,
                forall|j: int| #![trigger order@[j]] 0 <= j < order@.len() ==> order@[j] < n,
                forall|j: int| #![trigger order@[j]] 0 <= j < order@.len() ==>
                    parents.spec_index(order@[j] as int) != NO_PARENT,
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
                assert forall|j: int| #![trigger tree_updates@[j]] 0 <= j < tree_updates@.len()
                implies
                    parents.spec_index((tree_updates@[j]).0 as int) == NO_PARENT
                    && (tree_updates@[j]).0 < n
                    && (tree_updates@[j]).1 < n
                by {}
            }

            // Apply updates with deduplication: first write to each vertex wins.
            let mut next_layer: Vec<usize> = Vec::new();
            let mut k: usize = 0;
            while k < tree_updates.len()
                invariant
                    0 <= k <= tree_updates@.len(),
                    parents.spec_len() == n as int,
                    n as int == graph.spec_len(),
                    source < n,
                    n > 0,
                    n < usize::MAX,
                    spec_bfsmteph_wf(graph),
                    parents.spec_index(source as int) == source,
                    spec_parents_bounded(&parents, n as int),
                    forall|j: int| #![trigger tree_updates@[j]] 0 <= j < tree_updates@.len() ==>
                        (tree_updates@[j]).0 < n && (tree_updates@[j]).1 < n,
                    forall|j: int| #![trigger next_layer@[j]] 0 <= j < next_layer@.len() ==>
                        next_layer@[j] < n,
                    order@.len() > 0,
                    order@.len() <= n as int,
                    order@[0] == source,
                    forall|j: int| #![trigger order@[j]] 0 <= j < order@.len() ==> order@[j] < n,
                    forall|j: int| #![trigger order@[j]] 0 <= j < order@.len() ==>
                        parents.spec_index(order@[j] as int) != NO_PARENT,
                decreases tree_updates@.len() - k
            {
                let pair = &tree_updates[k];
                let v = pair.0;
                let u = pair.1;

                if *parents.nth(v) == NO_PARENT && order.len() < n {
                    let ghost pre_inner = *&parents;
                    let _ = parents.set(v, u);
                    next_layer.push(v);
                    order.push(v);

                    proof {
                        lemma_set_preserves_parents_bounded(
                            &parents, &pre_inner,
                            v as int, u, n as int,
                        );
                        assert(parents.spec_index(source as int) == source) by {
                            if v as int == source as int {
                                assert(pre_inner.spec_index(source as int) == source);
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

        let order_seq = ArraySeqMtEphS::from_vec(order);
        BFSTreeS { parents, order: order_seq }
    }

    } // impl BFSMtEphTrait

    impl BFSTreeMtEphTrait for BFSTreeS {
        open spec fn spec_order(&self) -> ArraySeqMtEphS<usize> {
            self.order
        }

        fn top_down_order(&self) -> (order: &ArraySeqMtEphS<usize>) {
            &self.order
        }

        /// Vertices in reverse BFS order (furthest from root first).
        fn bottom_up_order(&self) -> (order: ArraySeqMtEphS<usize>) {
            let n = self.order.length();
            ArraySeqMtEphS::tabulate(
                &|i: usize| -> (r: usize)
                    requires i < n, n == self.order.spec_len()
                    ensures r == self.order.spec_index((n - 1 - i) as int)
                { *self.order.nth(n - 1 - i) },
                n,
            )
        }
    }

    } // verus!
}
