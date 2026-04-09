//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Breadth-First Search - Sequential Ephemeral (Chapter 54, Algorithms 54.5 and 54.6).
//! Queue-based BFS for distances (54.5) and shortest-path tree (54.6).
//! Work: O(|V| + |E|), Span: O(|V| + |E|).


//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 4. type definitions
//	Section 4a. type definitions
//	Section 8a. traits
//	Section 9a. impls
//	Section 4b. type definitions
//	Section 6b. spec fns
//	Section 7b. proof fns/broadcast groups
//	Section 8b. traits
//	Section 9b. impls
//	Section 14a. derive impls outside verus!
//	Section 14b. derive impls outside verus!

//		Section 1. module

pub mod BFSStEph {


    //		Section 2. imports

    use std::collections::VecDeque;

    use vstd::prelude::*;
    use crate::Types::Types::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap54::BFSSpecsAndLemmas::BFSSpecsAndLemmas::*;

    verus! 
{

    //		Section 4. type definitions


    pub type T<N> = ArraySeqStEphS<ArraySeqStEphS<N>>;

    // UNREACHABLE and NO_PARENT imported from BFSSpecsAndLemmas.

    //		Section 4a. type definitions


    pub struct BFSTreeS {
        pub parents: ArraySeqStEphS<usize>,
        pub order: ArraySeqStEphS<usize>,
    }

    //		Section 8a. traits


    pub trait BFSTreeStEphTrait {
        spec fn spec_order(&self) -> ArraySeqStEphS<usize>;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — returns reference.
        fn top_down_order(&self) -> (order: &ArraySeqStEphS<usize>)
            ensures
                order.spec_len() == self.spec_order().spec_len(),
                forall|i: int| 0 <= i < order.spec_len() ==>
                    #[trigger] order.spec_index(i) == self.spec_order().spec_index(i);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — reverse via tabulate.
        fn bottom_up_order(&self) -> (order: ArraySeqStEphS<usize>)
            requires self.spec_order().spec_len() <= usize::MAX,
            ensures
                order.spec_len() == self.spec_order().spec_len(),
                forall|i: int| 0 <= i < order.spec_len() ==>
                    #[trigger] order.spec_index(i) == self.spec_order().spec_index(self.spec_order().spec_len() - 1 - i);
    }

    //		Section 9a. impls


    impl BFSTreeStEphTrait for BFSTreeS {
        open spec fn spec_order(&self) -> ArraySeqStEphS<usize> {
            self.order
        }

        /// Vertices in BFS order (root first, then distance 1, 2, ...).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — returns reference.
        fn top_down_order(&self) -> (order: &ArraySeqStEphS<usize>) {
            &self.order
        }

        /// Vertices in reverse BFS order (furthest from root first).
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — reverse via tabulate.
        fn bottom_up_order(&self) -> (order: ArraySeqStEphS<usize>) {
            let n = self.order.length();
            ArraySeqStEphS::tabulate(
                &|i: usize| -> (r: usize)
                    requires i < n, n == self.order.spec_len()
                    ensures r == self.order.spec_index((n - 1 - i) as int)
                { *self.order.nth(n - 1 - i) },
                n,
            )
        }
    }

    //		Section 4b. type definitions


    pub struct BFSStEph;

    //		Section 6b. spec fns


    /// All neighbor indices in the adjacency list are valid vertex indices.
    pub open spec fn spec_bfssteph_wf(graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>) -> bool {
        forall|u: int, i: int|
            0 <= u < graph.spec_len() && 0 <= i < graph.spec_index(u).spec_len()
            ==> #[trigger] graph.spec_index(u).spec_index(i) < graph.spec_len()
    }

    /// Every distance entry is either UNREACHABLE or bounded by n.
    pub open spec fn spec_distances_bounded(distances: &ArraySeqStEphS<usize>, n: int) -> bool {
        forall|j: int| 0 <= j < distances.spec_len() ==>
            #[trigger] distances.spec_index(j) == UNREACHABLE || distances.spec_index(j) < n
    }

    /// Every parent entry is either NO_PARENT or a valid vertex index.
    pub open spec fn spec_parents_bounded(parents: &ArraySeqStEphS<usize>, n: int) -> bool {
        forall|j: int| 0 <= j < parents.spec_len() ==>
            #[trigger] parents.spec_index(j) == NO_PARENT || parents.spec_index(j) < n
    }

    //		Section 7b. proof fns/broadcast groups


    proof fn lemma_tabulate_all_no_parent(parents: &ArraySeqStEphS<usize>, n: int)
        requires
            parents.spec_len() == n,
            forall|i: int| 0 <= i < n ==> #[trigger] parents.spec_index(i) == NO_PARENT,
        ensures
            spec_parents_bounded(parents, n),
    {
        // Veracity: NEEDED assert
        assert forall|i: int| 0 <= i < parents@.len() implies #[trigger] parents@[i] == parents.spec_index(i) by {};
        lemma_bfs_all_no_parent(parents@, n);
    }

    proof fn lemma_set_preserves_parents_bounded(
        parents: &ArraySeqStEphS<usize>,
        old_parents: &ArraySeqStEphS<usize>,
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
        // Veracity: NEEDED assert
        assert forall|i: int| 0 <= i < parents@.len() implies #[trigger] parents@[i] == parents.spec_index(i) by {};
        // Veracity: NEEDED assert
        assert forall|i: int| 0 <= i < old_parents@.len() implies #[trigger] old_parents@[i] == old_parents.spec_index(i) by {};
        lemma_bfs_update_preserves_parents_bounded(parents@, old_parents@, v, new_val, n);
    }

    proof fn lemma_tabulate_all_unreachable(distances: &ArraySeqStEphS<usize>, n: int)
        requires
            distances.spec_len() == n,
            forall|i: int| 0 <= i < n ==>
                #[trigger] distances.spec_index(i) == UNREACHABLE,
        ensures
            spec_distances_bounded(distances, n),
    {
        // Veracity: NEEDED assert
        assert forall|i: int| 0 <= i < distances@.len() implies #[trigger] distances@[i] == distances.spec_index(i) by {};
        lemma_bfs_all_unreachable(distances@, n);
    }

    /// After a point update, the bounded-distances property is preserved.
    proof fn lemma_set_preserves_bounded(
        distances: &ArraySeqStEphS<usize>,
        old_distances: &ArraySeqStEphS<usize>,
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
        // Veracity: NEEDED assert
        assert forall|i: int| 0 <= i < distances@.len() implies #[trigger] distances@[i] == distances.spec_index(i) by {};
        // Veracity: NEEDED assert
        assert forall|i: int| 0 <= i < old_distances@.len() implies #[trigger] old_distances@[i] == old_distances.spec_index(i) by {};
        lemma_bfs_update_preserves_bounded(distances@, old_distances@, v, new_val, n);
    }

    //		Section 8b. traits


    pub trait BFSStEphTrait {
        /// Algorithm 54.5: BFSDistance. Returns distance from source for every vertex.
        /// - Alg Analysis: APAS (Ch54 Alg 54.4): Work O(m lg n), Span O(d lg^2 n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n+m), Span O(n+m) — ACCEPTED DIFFERENCE: impl uses sequential queue-based BFS (Alg 54.3), not parallel set-based (Alg 54.4)
        fn bfs(graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>, source: usize) -> (traversal: ArraySeqStEphS<usize>)
            requires
                source < graph.spec_len(),
                graph.spec_len() > 0,
                graph.spec_len() < usize::MAX,
                spec_bfssteph_wf(graph),
            ensures
                traversal.spec_len() == graph.spec_len(),
                traversal.spec_index(source as int) == 0usize,
                spec_distances_bounded(&traversal, graph.spec_len() as int),
                forall|v: int| 0 <= v < traversal.spec_len()
                    && #[trigger] traversal.spec_index(v) != UNREACHABLE && v != source as int
                    ==> traversal.spec_index(v) > 0usize,
        ;

        /// Algorithm 54.6: BFS Tree. Returns parent array and BFS-order vertex sequence.
        /// - Alg Analysis: APAS (Ch54 Alg 54.6): Work O(n + m), Span O(d lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n+m), Span O(n+m) work; sequential BFS tree with array seqs
        fn bfs_tree(graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>, source: usize) -> (traversal: BFSTreeS)
            requires
                source < graph.spec_len(),
                graph.spec_len() > 0,
                graph.spec_len() < usize::MAX,
                spec_bfssteph_wf(graph),
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

    //		Section 9b. impls


    impl BFSStEphTrait for BFSStEph {

    /// - Alg Analysis: APAS (Ch54 Alg 54.6): Work O(|V| + |E|), Span O(|V| + |E|)
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V| + |E|), Span O(|V| + |E|)
    #[verifier::exec_allows_no_decreases_clause]
    fn bfs(graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>, source: usize) -> (traversal: ArraySeqStEphS<usize>)
    {
        broadcast use vstd::std_specs::vecdeque::group_vec_dequeue_axioms;

        let n = graph.length();

        let mut distances = ArraySeqStEphS::tabulate(
            &|_idx: usize| -> (r: usize) ensures r == UNREACHABLE { UNREACHABLE },
            n,
        );

        // After tabulate, all entries are UNREACHABLE → bounded.

        let ghost pre_set = *&distances;
        let _ = distances.set(source, 0);

        // After setting source to 0: 0 < n, so still bounded.
        // Veracity: NEEDED proof block
        proof {
            lemma_set_preserves_bounded(&distances, &pre_set, source as int, 0, n as int);
            // Veracity: NEEDED assert
            assert forall|v: int| 0 <= v < distances.spec_len()
                && distances.spec_index(v) != UNREACHABLE && v != source as int
            implies distances.spec_index(v) > 0usize
            by {
                // Veracity: NEEDED assert
                assert(distances.spec_index(v) == pre_set.spec_index(v));
            }
        }

        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(source);

        while queue.len() > 0
            invariant
                n as int == graph.spec_len(),
                distances.spec_len() == n as int,
                source < n,
                n > 0,
                n < usize::MAX,
                spec_bfssteph_wf(graph),
                distances.spec_index(source as int) == 0usize,
                forall|j: int| 0 <= j < queue@.len() ==>
                    (#[trigger] queue@[j]) < n,
                forall|j: int| 0 <= j < queue@.len() ==>
                    distances.spec_index((#[trigger] queue@[j]) as int) != UNREACHABLE,
                spec_distances_bounded(&distances, n as int),
                forall|v: int| 0 <= v < distances.spec_len()
                    && #[trigger] distances.spec_index(v) != UNREACHABLE && v != source as int
                    ==> distances.spec_index(v) > 0usize,
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
                            spec_bfssteph_wf(graph),
                            distances.spec_index(source as int) == 0usize,
                            u < n,
                            dist < n,
                            *neighbors == graph.spec_index(u as int),
                            forall|j: int| 0 <= j < queue@.len() ==>
                                (#[trigger] queue@[j]) < n,
                            forall|j: int| 0 <= j < queue@.len() ==>
                                distances.spec_index((#[trigger] queue@[j]) as int) != UNREACHABLE,
                            spec_distances_bounded(&distances, n as int),
                            forall|w: int| 0 <= w < distances.spec_len()
                                && #[trigger] distances.spec_index(w) != UNREACHABLE && w != source as int
                                ==> distances.spec_index(w) > 0usize,
                        decreases num_neighbors - i
                    {
                        let v = *neighbors.nth(i);

                        if *distances.nth(v) == UNREACHABLE {
                            if dist + 1 < n {
                                let ghost pre_inner_set = *&distances;
                                let _ = distances.set(v, dist + 1);
                                queue.push_back(v);
// Veracity: NEEDED proof block

                                proof {
                                    lemma_set_preserves_bounded(
                                        &distances, &pre_inner_set,
                                        v as int, (dist + 1) as usize, n as int,
                                    );
                                    // Veracity: NEEDED assert
                                    assert forall|w: int| 0 <= w < distances.spec_len()
                                        && distances.spec_index(w) != UNREACHABLE
                                        && w != source as int
                                    implies distances.spec_index(w) > 0usize
                                    by {
                                        if w == v as int {
                                        } else {
                                            // Veracity: NEEDED assert
                                            assert(distances.spec_index(w)
                                                == pre_inner_set.spec_index(w));
                                        }
                                    }
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

    /// Algorithm 54.6: BFS Tree. Returns parent array and BFS-order vertex sequence.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|+|E|), Span O(|V|+|E|) — standard BFS with VecDeque frontier; St sequential.
    #[verifier::exec_allows_no_decreases_clause]
    fn bfs_tree(graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>, source: usize) -> (traversal: BFSTreeS)
    {
        broadcast use vstd::std_specs::vecdeque::group_vec_dequeue_axioms;

        let n = graph.length();

        let mut parents = ArraySeqStEphS::tabulate(
            &|_idx: usize| -> (r: usize) ensures r == NO_PARENT { NO_PARENT },
            n,
        // Veracity: NEEDED proof block
        );

        proof { lemma_tabulate_all_no_parent(&parents, n as int); }

        // Veracity: NEEDED proof block
        let ghost pre_set = *&parents;
        let _ = parents.set(source, source);

        proof { lemma_set_preserves_parents_bounded(&parents, &pre_set, source as int, source, n as int); }

        let mut queue: VecDeque<usize> = VecDeque::new();
        queue.push_back(source);

        let mut order: Vec<usize> = Vec::new();
        order.push(source);

        while queue.len() > 0
            invariant
                n as int == graph.spec_len(),
                parents.spec_len() == n as int,
                source < n,
                n > 0,
                n < usize::MAX,
                spec_bfssteph_wf(graph),
                parents.spec_index(source as int) == source,
                forall|j: int| 0 <= j < queue@.len() ==> (#[trigger] queue@[j]) < n,
                forall|j: int| 0 <= j < queue@.len() ==>
                    parents.spec_index((#[trigger] queue@[j]) as int) != NO_PARENT,
                spec_parents_bounded(&parents, n as int),
                order@.len() > 0,
                order@.len() <= n as int,
                order@[0] == source,
                forall|j: int| 0 <= j < order@.len() ==> (#[trigger] order@[j]) < n,
                forall|j: int| #![trigger order@[j]] 0 <= j < order@.len() ==>
                    parents.spec_index(order@[j] as int) != NO_PARENT,
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
                            spec_bfssteph_wf(graph),
                            parents.spec_index(source as int) == source,
                            u < n,
                            *neighbors == graph.spec_index(u as int),
                            forall|j: int| 0 <= j < queue@.len() ==> (#[trigger] queue@[j]) < n,
                            forall|j: int| 0 <= j < queue@.len() ==>
                                parents.spec_index((#[trigger] queue@[j]) as int) != NO_PARENT,
                            spec_parents_bounded(&parents, n as int),
                            order@.len() > 0,
                            order@.len() <= n as int,
                            order@[0] == source,
                            forall|j: int| 0 <= j < order@.len() ==> (#[trigger] order@[j]) < n,
                            forall|j: int| #![trigger order@[j]] 0 <= j < order@.len() ==>
                                parents.spec_index(order@[j] as int) != NO_PARENT,
                        decreases num_neighbors - i
                    {
                        let v = *neighbors.nth(i);

                        if *parents.nth(v) == NO_PARENT && order.len() < n {
                            let ghost pre_inner = *&parents;
                            // Veracity: NEEDED proof block
                            let _ = parents.set(v, u);
                            queue.push_back(v);
                            order.push(v);

                            proof {
                                lemma_set_preserves_parents_bounded(
                                    &parents, &pre_inner,
                                    v as int, u, n as int,
                                );
                                // v != source: parents[source] == source != NO_PARENT
                            }
                        }
                        i = i + 1;
                    }
                }
            }
        }

        let order_seq = ArraySeqStEphS::from_vec(order);
        BFSTreeS { parents, order: order_seq }
    }

    } // impl BFSStEphTrait
    } // verus!

    //		Section 14a. derive impls outside verus!


    impl std::fmt::Debug for BFSTreeS {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("BFSTreeS")
                .field("parents", &self.parents)
                .field("order", &self.order)
                .finish()
        }
    }

    impl std::fmt::Display for BFSTreeS {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BFSTreeS(parents: {}, order: {})", self.parents.length(), self.order.length())
        }
    }

    //		Section 14b. derive impls outside verus!

    impl std::fmt::Debug for BFSStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BFSStEph")
        }
    }

    impl std::fmt::Display for BFSStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BFSStEph")
        }
    }
}
