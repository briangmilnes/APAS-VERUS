//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Depth-First Search - Sequential Persistent (Chapter 55, Algorithm 55.2).
//! Recursive DFS for finding reachable vertices from a source vertex.
//! Work: O(|V| + |E|), Span: O(|V| + |E|).

pub mod DFSStPer {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
    use crate::Chap55::TopoSortStEph::TopoSortStEph::{spec_num_false, lemma_set_true_decreases_num_false, lemma_set_true_num_false_eq};
    use crate::Chap55::TopoSortStPer::TopoSortStPer::{spec_toposortstper_wf, spec_reachable_per};
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

    pub type T<N> = ArraySeqStPerS<ArraySeqStPerS<N>>;
    pub struct DFSStPer;

    // 6. spec fns

    /// Bridge: for ArraySeqStPerS<usize>, view index equals spec_index.
    proof fn lemma_usize_per_view_eq_spec_index(a: &ArraySeqStPerS<N>)
        ensures forall|j: int| 0 <= j < a@.len() ==> #[trigger] a@[j] == a.spec_index(j),
    {
        assert forall|j: int| 0 <= j < a@.len() implies #[trigger] a@[j] == a.spec_index(j) by {}
    }

    /// Bridge: for persistent graph adjacency list, the view at vertex equals the spec_index view.
    proof fn lemma_graph_per_view_bridge(
        graph: &ArraySeqStPerS<ArraySeqStPerS<N>>,
        neighbors: &ArraySeqStPerS<N>,
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

    pub trait DFSStPerTrait {
        /// Performs DFS from source vertex s on adjacency list graph G
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        fn dfs(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>, source: N) -> (reachable: AVLTreeSetStPer<N>)
            requires
                source < graph@.len(),
                spec_toposortstper_wf(graph),
            ensures
                reachable@.contains(source),
                forall|v: usize| reachable@.contains(v) ==> (v as int) < graph@.len(),
                forall|v: int| 0 <= v < graph@.len()
                    ==> (reachable@.contains(v as usize) <==> #[trigger] spec_reachable_per(graph, source as int, v)),
            ;
    }

    // 9. impls

    /// Recursive DFS helper using a bool vector for termination tracking and
    /// an AVLTreeSetStPer for persistent result accumulation.
    fn dfs_recursive(
        graph: &ArraySeqStPerS<ArraySeqStPerS<N>>,
        visited_bool: &mut Vec<bool>,
        reachable: AVLTreeSetStPer<N>,
        vertex: N,
    ) -> (out: AVLTreeSetStPer<N>)
        requires
            vertex < old(visited_bool)@.len(),
            old(visited_bool)@.len() == graph@.len(),
            spec_toposortstper_wf(graph),
            reachable.spec_avltreesetstper_wf(),
            forall|v: usize| reachable@.contains(v) ==> (v as int) < graph@.len(),
            graph@.len() < usize::MAX,
            reachable@.len() + spec_num_false(old(visited_bool)@) <= graph@.len(),
        ensures
            visited_bool@.len() == old(visited_bool)@.len(),
            forall|j: int|
                0 <= j < visited_bool@.len() && #[trigger] old(visited_bool)@[j]
                ==> visited_bool@[j],
            spec_num_false(visited_bool@) <= spec_num_false(old(visited_bool)@),
            out.spec_avltreesetstper_wf(),
            forall|v: usize| out@.contains(v) ==> (v as int) < graph@.len(),
            out@.len() + spec_num_false(visited_bool@) <= graph@.len(),
        decreases spec_num_false(old(visited_bool)@),
    {
        if visited_bool[vertex] {
            return reachable;
        }
        assert(!old(visited_bool)@[vertex as int]);
        visited_bool.set(vertex, true);
        proof {
            lemma_set_true_decreases_num_false(old(visited_bool)@, vertex as int);
            lemma_set_true_num_false_eq(old(visited_bool)@, vertex as int);
        }

        // After Vec::set: visited_bool@ == old(visited_bool)@.update(vertex, true).
        assert(visited_bool@ =~= old(visited_bool)@.update(vertex as int, true));
        assert(spec_num_false(visited_bool@) < spec_num_false(old(visited_bool)@));
        assert(spec_num_false(visited_bool@) == spec_num_false(old(visited_bool)@) - 1);

        // Monotonicity.
        assert forall|j: int| 0 <= j < visited_bool@.len() && #[trigger] old(visited_bool)@[j]
            implies visited_bool@[j] by {};

        // Combined bound: reachable@.len() + 1 <= graph@.len().
        assert(reachable@.len() + spec_num_false(visited_bool@) <= graph@.len() - 1);
        assert(reachable@.len() + 1 <= graph@.len());
        assert(reachable@.len() + 1 < usize::MAX as nat);

        let reachable = reachable.insert(vertex);
        assert(reachable.spec_avltreesetstper_wf());
        assert(reachable@.len() + spec_num_false(visited_bool@) <= graph@.len());

        assert((vertex as int) < graph@.len());
        assert(vertex < graph.spec_len());
        let neighbors = graph.nth(vertex);
        let neighbors_len = neighbors.length();
        assert(neighbors_len as int == neighbors.spec_len());

        // Establish graph/neighbors bridge before the loop.
        assert(*neighbors == graph.spec_index(vertex as int));
        proof {
            lemma_graph_per_view_bridge(graph, neighbors, vertex as int);
        }
        assert(neighbors@ =~= graph@[vertex as int]);

        let mut i: usize = 0;
        let mut reachable = reachable;
        while i < neighbors_len
            invariant
                i <= neighbors_len,
                neighbors_len as int == neighbors.spec_len(),
                neighbors_len == graph@[vertex as int].len(),
                neighbors@ =~= graph@[vertex as int],
                *neighbors == graph.spec_index(vertex as int),
                (vertex as int) < graph@.len(),
                visited_bool@.len() == graph@.len(),
                spec_toposortstper_wf(graph),
                graph@.len() < usize::MAX,
                forall|j: int|
                    0 <= j < visited_bool@.len() && #[trigger] old(visited_bool)@[j]
                    ==> visited_bool@[j],
                spec_num_false(visited_bool@) < spec_num_false(old(visited_bool)@),
                reachable.spec_avltreesetstper_wf(),
                forall|v: usize| reachable@.contains(v) ==> (v as int) < graph@.len(),
                reachable@.len() + spec_num_false(visited_bool@) <= graph@.len(),
            decreases neighbors_len - i,
        {
            let neighbor = *neighbors.nth(i);
            proof { lemma_usize_per_view_eq_spec_index(neighbors); }
            assert(neighbor == neighbors@[i as int]);
            assert(neighbor == graph@[vertex as int][i as int]);
            assert(graph@[vertex as int][i as int] < graph@.len());
            assert(neighbor < graph@.len());
            reachable = dfs_recursive(graph, visited_bool, reachable, neighbor);
            i = i + 1;
        }
        reachable
    }

    impl DFSStPerTrait for DFSStPer {
        /// Performs DFS from source vertex s on adjacency list graph G.
        /// Returns the set of all vertices reachable from s.
        #[verifier::external_body]
        fn dfs(graph: &ArraySeqStPerS<ArraySeqStPerS<N>>, source: N) -> AVLTreeSetStPer<N>
        {
            let n = graph.length();
            let mut visited_bool: Vec<bool> = Vec::new();
            let mut j: usize = 0;
            while j < n
                invariant j <= n, visited_bool@.len() == j as int,
                decreases n - j,
            {
                visited_bool.push(false);
                j = j + 1;
            }
            let reachable = AVLTreeSetStPer::empty();
            dfs_recursive(graph, &mut visited_bool, reachable, source)
        }
    } // impl DFSStPerTrait

    } // verus!
}
