//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Breadth-First Search - Sequential Persistent (Chapter 54, Algorithms 54.3 and 54.6).
//! Queue-based BFS for distances (54.3) and shortest-path tree (54.6).
//! Work: O(|V| + |E|), Span: O(|V| + |E|).

pub mod BFSStPer {

    use std::collections::VecDeque;

    use vstd::prelude::*;
    use crate::Types::Types::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;

    verus! {

    // Table of Contents
    // 4. type definitions
    // 6. spec fns
    // 7. proof fns
    // 8. traits
    // 9. impls

    // 4. type definitions
    pub type T<N> = ArraySeqStPerS<ArraySeqStPerS<N>>;

    pub const UNREACHABLE: N = N::MAX;
    pub const NO_PARENT: N = N::MAX;

    pub struct BFSTreeS {
        pub parents: ArraySeqStPerS<N>,
        pub order: ArraySeqStPerS<N>,
    }

    // 6. spec fns

    /// All neighbor indices in the adjacency list are valid vertex indices.
    pub open spec fn spec_wf_graph(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>) -> bool {
        forall|u: int, i: int| #![auto]
            0 <= u < graph.spec_len() && 0 <= i < graph.spec_index(u).spec_len()
            ==> graph.spec_index(u).spec_index(i) < graph.spec_len()
    }

    /// Every distance entry is either UNREACHABLE or bounded by n.
    pub open spec fn spec_distances_bounded(distances: &ArraySeqStPerS<N>, n: int) -> bool {
        forall|j: int| #![auto] 0 <= j < distances.spec_len() ==>
            distances.spec_index(j) == UNREACHABLE || distances.spec_index(j) < n
    }

    /// Every parent entry is either NO_PARENT or a valid vertex index.
    pub open spec fn spec_parents_bounded(parents: &ArraySeqStPerS<N>, n: int) -> bool {
        forall|j: int| #![auto] 0 <= j < parents.spec_len() ==>
            parents.spec_index(j) == NO_PARENT || parents.spec_index(j) < n
    }

    // 7. proof fns

    proof fn lemma_tabulate_all_no_parent(parents: &ArraySeqStPerS<N>, n: int)
        requires
            parents.spec_len() == n,
            forall|i: int| #![auto] 0 <= i < n ==> parents.spec_index(i) == NO_PARENT,
        ensures
            spec_parents_bounded(parents, n),
    {}

    proof fn lemma_update_preserves_parents_bounded(
        parents: &ArraySeqStPerS<N>,
        old_parents: &ArraySeqStPerS<N>,
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

    proof fn lemma_tabulate_all_unreachable(distances: &ArraySeqStPerS<N>, n: int)
        requires
            distances.spec_len() == n,
            forall|i: int| #![auto] 0 <= i < n ==>
                distances.spec_index(i) == UNREACHABLE,
        ensures
            spec_distances_bounded(distances, n),
    {
    }

    proof fn lemma_update_preserves_bounded(
        distances: &ArraySeqStPerS<N>,
        old_distances: &ArraySeqStPerS<N>,
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

    // 8. traits
    pub trait BFSStPerTrait {
        /// - APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn bfs(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>, source: N) -> (result: ArraySeqStPerS<N>)
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
        /// - APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn bfs_tree(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>, source: N) -> (result: BFSTreeS)
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

    /// - APAS: Work O(|V| + |E|), Span O(|V| + |E|)
    #[verifier::exec_allows_no_decreases_clause]
    pub fn bfs(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>, source: N) -> (result: ArraySeqStPerS<N>)
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
        broadcast use vstd::std_specs::vecdeque::group_vec_dequeue_axioms;

        let n = graph.length();

        let mut distances = ArraySeqStPerS::tabulate(
            &|_idx: usize| -> (r: N) ensures r == UNREACHABLE { UNREACHABLE },
            n,
        );

        proof { lemma_tabulate_all_unreachable(&distances, n as int); }

        let old_d = distances;
        distances = ArraySeqStPerS::update(&old_d, source, 0);

        proof { lemma_update_preserves_bounded(&distances, &old_d, source as int, 0, n as int); }

        let mut queue: VecDeque<N> = VecDeque::new();
        queue.push_back(source);

        while queue.len() > 0
            invariant
                n as int == graph.spec_len(),
                distances.spec_len() == n as int,
                source < n,
                n > 0,
                n < usize::MAX,
                spec_wf_graph(graph),
                distances.spec_index(source as int) == 0usize,
                forall|j: int| #![auto] 0 <= j < queue@.len() ==>
                    queue@[j] < n,
                forall|j: int| #![auto] 0 <= j < queue@.len() ==>
                    distances.spec_index(queue@[j] as int) != UNREACHABLE,
                spec_distances_bounded(&distances, n as int),
        {
            let u_opt = queue.pop_front();
            match u_opt {
                None => {}
                Some(u) => {
                    let dist = *distances.nth(u);
                    let neighbors = graph.nth(u);
                    let num_neighbors = neighbors.length();
                    let mut i: usize = 0;

                    while i < num_neighbors
                        invariant
                            0 <= i <= num_neighbors,
                            num_neighbors as int == neighbors.spec_len(),
                            n as int == graph.spec_len(),
                            distances.spec_len() == n as int,
                            source < n,
                            n > 0,
                            n < usize::MAX,
                            spec_wf_graph(graph),
                            distances.spec_index(source as int) == 0usize,
                            u < n,
                            dist < n,
                            *neighbors == graph.spec_index(u as int),
                            forall|j: int| #![auto] 0 <= j < queue@.len() ==>
                                queue@[j] < n,
                            forall|j: int| #![auto] 0 <= j < queue@.len() ==>
                                distances.spec_index(queue@[j] as int) != UNREACHABLE,
                            spec_distances_bounded(&distances, n as int),
                        decreases num_neighbors - i
                    {
                        let v = *neighbors.nth(i);

                        if *distances.nth(v) == UNREACHABLE {
                            if dist + 1 < n {
                                let old_d_inner = distances;
                                distances = ArraySeqStPerS::update(&old_d_inner, v, dist + 1);
                                queue.push_back(v);

                                proof {
                                    lemma_update_preserves_bounded(
                                        &distances, &old_d_inner,
                                        v as int, (dist + 1) as N, n as int,
                                    );
                                }
                            }
                        }
                        i = i + 1;
                    }
                }
            }
        }

        distances
    }

    /// Algorithm 54.6: BFS Tree with VecDeque. Returns parent array and BFS-order vertex sequence.
    #[verifier::exec_allows_no_decreases_clause]
    pub fn bfs_tree(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>, source: N) -> (result: BFSTreeS)
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
        broadcast use vstd::std_specs::vecdeque::group_vec_dequeue_axioms;

        let n = graph.length();

        let mut parents = ArraySeqStPerS::tabulate(
            &|_idx: usize| -> (r: N) ensures r == NO_PARENT { NO_PARENT },
            n,
        );

        proof { lemma_tabulate_all_no_parent(&parents, n as int); }

        let old_p = parents;
        parents = ArraySeqStPerS::update(&old_p, source, source);

        proof { lemma_update_preserves_parents_bounded(&parents, &old_p, source as int, source, n as int); }

        let mut queue: VecDeque<N> = VecDeque::new();
        queue.push_back(source);

        let mut order: Vec<N> = Vec::new();
        order.push(source);

        while queue.len() > 0
            invariant
                n as int == graph.spec_len(),
                parents.spec_len() == n as int,
                source < n,
                n > 0,
                n < usize::MAX,
                spec_wf_graph(graph),
                parents.spec_index(source as int) == source,
                forall|j: int| #![auto] 0 <= j < queue@.len() ==> queue@[j] < n,
                forall|j: int| #![auto] 0 <= j < queue@.len() ==>
                    parents.spec_index(queue@[j] as int) != NO_PARENT,
                spec_parents_bounded(&parents, n as int),
                order@.len() > 0,
                order@[0] == source,
                forall|j: int| #![auto] 0 <= j < order@.len() ==> order@[j] < n,
        {
            let u_opt = queue.pop_front();
            match u_opt {
                None => {}
                Some(u) => {
                    let neighbors = graph.nth(u);
                    let num_neighbors = neighbors.length();
                    let mut i: usize = 0;

                    while i < num_neighbors
                        invariant
                            0 <= i <= num_neighbors,
                            num_neighbors as int == neighbors.spec_len(),
                            n as int == graph.spec_len(),
                            parents.spec_len() == n as int,
                            source < n,
                            n > 0,
                            n < usize::MAX,
                            spec_wf_graph(graph),
                            parents.spec_index(source as int) == source,
                            u < n,
                            *neighbors == graph.spec_index(u as int),
                            forall|j: int| #![auto] 0 <= j < queue@.len() ==> queue@[j] < n,
                            forall|j: int| #![auto] 0 <= j < queue@.len() ==>
                                parents.spec_index(queue@[j] as int) != NO_PARENT,
                            spec_parents_bounded(&parents, n as int),
                            order@.len() > 0,
                            order@[0] == source,
                            forall|j: int| #![auto] 0 <= j < order@.len() ==> order@[j] < n,
                        decreases num_neighbors - i
                    {
                        let v = *neighbors.nth(i);

                        if *parents.nth(v) == NO_PARENT {
                            let old_p_inner = parents;
                            parents = ArraySeqStPerS::update(&old_p_inner, v, u);
                            queue.push_back(v);
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
                        i = i + 1;
                    }
                }
            }
        }

        let order_seq = ArraySeqStPerS::from_vec(order);
        BFSTreeS { parents, order: order_seq }
    }

    impl BFSTreeS {
        /// Vertices in BFS order (root first, then distance 1, 2, ...).
        pub fn top_down_order(&self) -> (result: &ArraySeqStPerS<N>)
            ensures
                result.spec_len() == self.order.spec_len(),
                forall|i: int| #![auto] 0 <= i < result.spec_len() ==>
                    result.spec_index(i) == self.order.spec_index(i),
        {
            &self.order
        }

        /// Vertices in reverse BFS order (furthest from root first).
        pub fn bottom_up_order(&self) -> (result: ArraySeqStPerS<N>)
            requires self.order.spec_len() <= usize::MAX,
            ensures
                result.spec_len() == self.order.spec_len(),
                forall|i: int| #![auto] 0 <= i < result.spec_len() ==>
                    result.spec_index(i) == self.order.spec_index(self.order.spec_len() - 1 - i),
        {
            let n = self.order.length();
            ArraySeqStPerS::tabulate(
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
