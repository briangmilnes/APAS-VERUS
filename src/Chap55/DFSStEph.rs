//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Depth-First Search - Sequential Ephemeral (Chapter 55, Algorithm 55.7).
//! Recursive DFS using ephemeral arrays for efficient visited tracking.
//! Work: O(|V| + |E|), Span: O(|V| + |E|).

pub mod DFSStEph {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
    use crate::Chap55::TopoSortStEph::TopoSortStEph::{spec_num_false, spec_toposortsteph_wf, spec_reachable, lemma_set_true_decreases_num_false, lemma_set_true_num_false_eq, lemma_all_false_num_false_eq_len};
    use crate::Types::Types::*;

    verus! {

broadcast use vstd::seq::group_seq_axioms;

    // Table of Contents
    // 1. module
    // 2. imports
    // 4. type definitions
    // 6. spec fns
    // 8. traits
    // 9. impls

    // 4. type definitions

    pub type T<N> = ArraySeqStEphS<ArraySeqStEphS<N>>;
    pub struct DFSStEph;

    // 6. spec fns

    /// Bridge: for ArraySeqStEphS<bool>, view index equals spec_index.
    proof fn lemma_bool_view_eq_spec_index(a: &ArraySeqStEphS<B>)
        ensures forall|j: int| 0 <= j < a@.len() ==> #[trigger] a@[j] == a.spec_index(j),
    {
        assert forall|j: int| 0 <= j < a@.len() implies #[trigger] a@[j] == a.spec_index(j) by {}
    }

    /// Bridge: for ArraySeqStEphS<usize>, view index equals spec_index.
    proof fn lemma_usize_view_eq_spec_index(a: &ArraySeqStEphS<N>)
        ensures forall|j: int| 0 <= j < a@.len() ==> #[trigger] a@[j] == a.spec_index(j),
    {
        assert forall|j: int| 0 <= j < a@.len() implies #[trigger] a@[j] == a.spec_index(j) by {}
    }

    /// Bridge: for graph adjacency list, the view at vertex equals the spec_index view.
    proof fn lemma_graph_view_bridge(
        graph: &ArraySeqStEphS<ArraySeqStEphS<N>>,
        neighbors: &ArraySeqStEphS<N>,
        vertex: int,
    )
        requires
            0 <= vertex < graph@.len(),
            *neighbors == graph.spec_index(vertex),
        ensures
            neighbors@ =~= graph@[vertex],
    {
    }

    // 8. traits

    pub trait DFSStEphTrait {
        /// Performs DFS from source vertex s on adjacency list graph G.
        /// - APAS: Work O(|V| + |E|), Span O(|V| + |E|) [Cost Spec 55.8, array sequences]
        /// - Claude-Opus-4.6: Work O(|V| + |E|), Span O(|V| + |E|) — agrees with APAS.
        fn dfs(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>, source: N) -> (reachable: AVLTreeSetStEph<N>)
            requires
                source < graph@.len(),
                spec_toposortsteph_wf(graph),
            ensures
                reachable@.contains(source),
                forall|v: usize| reachable@.contains(v) ==> (v as int) < graph@.len(),
                forall|v: int| 0 <= v < graph@.len()
                    ==> (reachable@.contains(v as usize) <==> #[trigger] spec_reachable(graph, source as int, v)),
            ;
    }

    // 9. impls

    /// Recursive DFS helper that marks visited vertices and inserts them into the result set.
    fn dfs_recursive(
        graph: &ArraySeqStEphS<ArraySeqStEphS<N>>,
        visited: &mut ArraySeqStEphS<B>,
        reachable: &mut AVLTreeSetStEph<N>,
        vertex: N,
    )
        requires
            vertex < old(visited)@.len(),
            old(visited)@.len() == graph@.len(),
            spec_toposortsteph_wf(graph),
            old(reachable).spec_avltreesetsteph_wf(),
            forall|v: usize| old(reachable)@.contains(v) ==> (v as int) < graph@.len(),
            graph@.len() < usize::MAX,
            old(reachable)@.len() + spec_num_false(old(visited)@) <= graph@.len(),
        ensures
            visited@.len() == old(visited)@.len(),
            forall|j: int|
                0 <= j < visited@.len() && #[trigger] old(visited)@[j]
                ==> visited@[j],
            spec_num_false(visited@) <= spec_num_false(old(visited)@),
            reachable.spec_avltreesetsteph_wf(),
            forall|v: usize| reachable@.contains(v) ==> (v as int) < graph@.len(),
            reachable@.len() + spec_num_false(visited@) <= graph@.len(),
        decreases spec_num_false(old(visited)@),
    {
        // Bridge: visited@[j] == visited.spec_index(j) for bool arrays.
        proof { lemma_bool_view_eq_spec_index(visited); }
        assert(visited.spec_len() == visited@.len());

        if *visited.nth(vertex) {
            return;
        }
        // vertex was not visited — old(visited)@[vertex as int] is false.
        assert(!old(visited)@[vertex as int]);
        assert(vertex < visited.spec_len());

        let set_ok = visited.set(vertex, true);
        assert(set_ok.is_ok());

        proof {
            lemma_set_true_decreases_num_false(old(visited)@, vertex as int);
            lemma_set_true_num_false_eq(old(visited)@, vertex as int);
        }

        // After set: visited.spec_index(vertex) == true, others unchanged.
        // Re-establish bridge for the new visited state.
        proof { lemma_bool_view_eq_spec_index(visited); }
        assert(visited@.len() == old(visited)@.len());

        // Establish visited@ == old(visited)@.update(vertex, true).
        proof { lemma_bool_view_eq_spec_index(visited); }
        assert forall|j: int| 0 <= j < visited@.len()
            implies #[trigger] visited@[j] == old(visited)@.update(vertex as int, true)[j] by {
            assert(visited@[j] == visited.spec_index(j));
            assert(old(visited)@[j] == old(visited).spec_index(j));
            if j == vertex as int {
                assert(visited.spec_index(j) == true);
            } else {
                assert(visited.spec_index(j) == old(visited).spec_index(j));
            }
        };
        assert(visited@ =~= old(visited)@.update(vertex as int, true));

        // Now the lemma results apply directly.
        assert(spec_num_false(visited@) < spec_num_false(old(visited)@));
        assert(spec_num_false(visited@) == spec_num_false(old(visited)@) - 1);

        // Monotonicity: old(visited)@[j] ==> visited@[j].
        assert forall|j: int| 0 <= j < visited@.len() && #[trigger] old(visited)@[j]
            implies visited@[j] by {};

        // Combined bound: spec_num_false decreased by 1 and reachable unchanged.
        assert(reachable@.len() + spec_num_false(visited@)
            == reachable@.len() + spec_num_false(old(visited)@) - 1);
        assert(reachable@.len() + spec_num_false(visited@) <= graph@.len() - 1);
        assert(reachable@.len() + 1 <= graph@.len());
        assert(graph@.len() < usize::MAX);
        assert(reachable@.len() + 1 < usize::MAX as nat);

        reachable.insert(vertex);
        assert(reachable.spec_avltreesetsteph_wf());
        // After insert: reachable@.len() increased by at most 1.
        assert(reachable@.len() + spec_num_false(visited@) <= graph@.len());

        assert((vertex as int) < graph@.len());
        assert(vertex < graph.spec_len());
        let neighbors = graph.nth(vertex);
        let neighbors_len = neighbors.length();
        assert(neighbors_len as int == neighbors.spec_len());

        // Establish graph/neighbors bridge before the loop.
        assert(*neighbors == graph.spec_index(vertex as int));
        proof {
            lemma_graph_view_bridge(graph, neighbors, vertex as int);
        }
        assert(neighbors@ =~= graph@[vertex as int]);

        let mut i: usize = 0;
        while i < neighbors_len
            invariant
                i <= neighbors_len,
                neighbors_len as int == neighbors.spec_len(),
                neighbors_len == graph@[vertex as int].len(),
                neighbors@ =~= graph@[vertex as int],
                *neighbors == graph.spec_index(vertex as int),
                (vertex as int) < graph@.len(),
                visited@.len() == graph@.len(),
                spec_toposortsteph_wf(graph),
                graph@.len() < usize::MAX,
                forall|j: int|
                    0 <= j < visited@.len() && #[trigger] old(visited)@[j]
                    ==> visited@[j],
                spec_num_false(visited@) < spec_num_false(old(visited)@),
                reachable.spec_avltreesetsteph_wf(),
                forall|v: usize| reachable@.contains(v) ==> (v as int) < graph@.len(),
                reachable@.len() + spec_num_false(visited@) <= graph@.len(),
            decreases neighbors_len - i,
        {
            let neighbor = *neighbors.nth(i);
            proof { lemma_usize_view_eq_spec_index(neighbors); }
            assert(neighbor == neighbors@[i as int]);
            assert(neighbor == graph@[vertex as int][i as int]);
            assert(graph@[vertex as int][i as int] < graph@.len());
            assert(neighbor < graph@.len());
            dfs_recursive(graph, visited, reachable, neighbor);
            i = i + 1;
        }
    }

    impl DFSStEphTrait for DFSStEph {
        /// Performs DFS from source vertex s on adjacency list graph G.
        /// Returns the set of all vertices reachable from s.
        #[verifier::external_body]
        fn dfs(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>, source: N) -> AVLTreeSetStEph<N>
        {
            let n = graph.length();
            let mut visited = ArraySeqStEphS::tabulate(&|_x| false, n);
            let mut reachable = AVLTreeSetStEph::empty();
            dfs_recursive(graph, &mut visited, &mut reachable, source);
            reachable
        }
    } // impl DFSStEphTrait

    } // verus!
}
