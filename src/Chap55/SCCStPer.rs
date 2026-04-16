// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO
//! Strongly Connected Components - Sequential Persistent (Chapter 55, Algorithm 55.18).
//! Finds all strongly connected components using transpose and DFS.
//! Work: O(|V| + |E|), Span: O(|V| + |E|).


//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 7. proof fns/broadcast groups
//	Section 8. traits
//	Section 9. impls
//	Section 14. derive impls outside verus!

//		Section 1. module

pub mod SCCStPer {


    //		Section 2. imports

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::{AVLTreeSeqStPerS, AVLTreeSeqStPerTrait};
    use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap55::DFSSpecsAndLemmas::DFSSpecsAndLemmas::{
        spec_num_false, lemma_set_true_decreases_num_false,
        lemma_set_true_num_false_eq, lemma_all_true_num_false_zero,
        lemma_all_false_num_false_eq_len, lemma_usize_per_view_eq_spec_index,
        lemma_graph_per_view_bridge,
    };
    #[cfg(verus_keep_ghost)]
    use crate::Chap55::TopoSortStPer::TopoSortStPer::spec_toposortstper_wf;
    use crate::Types::Types::*;

    verus! 
{

    //		Section 3. broadcast use


    broadcast use {
        vstd::seq::group_seq_axioms,
        vstd::seq_lib::group_seq_properties,
        crate::vstdplus::feq::feq::group_feq_axioms,
    };

    //		Section 4. type definitions


    pub type T<N> = ArraySeqStPerS<ArraySeqStPerS<N>>;


    pub struct SCCStPer;

    //		Section 8. traits


    pub trait SCCStPerTrait {
        /// Finds strongly connected components in a directed graph (Algorithm 55.18)
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|+|E|), Span O(|V|+|E|) — Kosaraju's algorithm; St sequential.
        fn scc(graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>) -> (components: AVLTreeSeqStPerS<AVLTreeSetStPer<usize>>)
            requires
                spec_toposortstper_wf(graph),
                graph@.len() < usize::MAX,
            ensures
                components@.len() >= 1 || graph@.len() == 0,
            ;
    }

    //		Section 9. impls


    /// Recursive DFS that appends vertices in finish order.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|+|E|), Span O(|V|+|E|) — DFS appending vertices at finish time; St sequential.
    fn dfs_finish_order(
        graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>,
        visited: &mut Vec<bool>,
        finish_order: &mut Vec<usize>,
        vertex: usize,
    )
        requires
            vertex < old(visited)@.len(),
            old(visited)@.len() == graph@.len(),
            spec_toposortstper_wf(graph),
            forall|k: int| 0 <= k < old(finish_order)@.len()
                ==> (#[trigger] old(finish_order)@[k] as int) < graph@.len(),
        ensures
            visited@.len() == graph@.len(),
            forall|j: int|
                0 <= j < visited@.len() && #[trigger] old(visited)@[j]
                ==> visited@[j],
            spec_num_false(visited@) <= spec_num_false(old(visited)@),
            finish_order@.len() >= old(finish_order)@.len(),
            forall|k: int| 0 <= k < finish_order@.len()
                ==> (#[trigger] finish_order@[k] as int) < graph@.len(),
            visited@[vertex as int],
            finish_order@.len() + spec_num_false(visited@)
                == old(finish_order)@.len() + spec_num_false(old(visited)@),
        decreases spec_num_false(old(visited)@),
    {
        if visited[vertex] {
            return;
        }
        // Veracity: NEEDED assert (speed hint)
        assert(!old(visited)@[vertex as int]);
        visited.set(vertex, true);
        // Veracity: NEEDED proof block
        proof {
            lemma_set_true_decreases_num_false(old(visited)@, vertex as int);
            lemma_set_true_num_false_eq(old(visited)@, vertex as int);
        }
// Veracity: UNNEEDED assert         assert(visited@ =~= old(visited)@.update(vertex as int, true));
// Veracity: UNNEEDED assert         assert(spec_num_false(visited@) < spec_num_false(old(visited)@));
// Veracity: UNNEEDED assert         assert(spec_num_false(visited@) == spec_num_false(old(visited)@) - 1);
        // Veracity: NEEDED assert (speed hint)
        assert(visited@.len() == graph@.len());
// Veracity: UNNEEDED assert         assert(visited@[vertex as int]);

        // Monotonicity.
        // Veracity: NEEDED assert
        assert forall|j: int| 0 <= j < visited@.len() && #[trigger] old(visited)@[j]
            implies visited@[j] by {};

// Veracity: UNNEEDED assert         assert((vertex as int) < graph@.len());
        // Veracity: NEEDED assert (speed hint)
        assert(vertex < graph.spec_len());
        let neighbors = graph.nth(vertex);
        let neighbors_len = neighbors.length();
        // Veracity: NEEDED assert (speed hint)
        assert(neighbors_len as int == neighbors.spec_len());
// Veracity: UNNEEDED assert         assert(neighbors_len == graph@[vertex as int].len());

        // Bridge neighbors to graph view.
// Veracity: UNNEEDED proof block // Veracity: UNNEEDED assert         assert(*neighbors == graph.spec_index(vertex as int));
        proof { lemma_graph_per_view_bridge(graph, neighbors, vertex as int); }
        // Veracity: NEEDED assert (speed hint)
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
                spec_toposortstper_wf(graph),
                forall|j: int|
                    0 <= j < visited@.len() && #[trigger] old(visited)@[j]
                    ==> visited@[j],
                spec_num_false(visited@) < spec_num_false(old(visited)@),
                finish_order@.len() >= old(finish_order)@.len(),
                forall|k: int| 0 <= k < finish_order@.len()
                    ==> (#[trigger] finish_order@[k] as int) < graph@.len(),
                visited@[vertex as int],
                finish_order@.len() + spec_num_false(visited@) + 1
                    == old(finish_order)@.len() + spec_num_false(old(visited)@),
            decreases neighbors_len - i,
        {
            // Veracity: NEEDED proof block
            let neighbor = *neighbors.nth(i);
            proof { lemma_usize_per_view_eq_spec_index(neighbors); }
            // Veracity: NEEDED assert (speed hint)
            assert(neighbor == neighbors@[i as int]);
            // Veracity: NEEDED assert (speed hint)
            assert(neighbor == graph@[vertex as int][i as int]);
// Veracity: UNNEEDED assert             assert(graph@[vertex as int][i as int] < graph@.len());
            // Veracity: NEEDED assert (speed hint)
            assert(neighbor < graph@.len());
            dfs_finish_order(graph, visited, finish_order, neighbor);
            i = i + 1;
        }
        finish_order.push(vertex);
    }

    /// Computes the finish order for SCC (decreasing finish times).
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|+|E|), Span O(|V|+|E|) — full DFS + reverse; St sequential.
    fn compute_finish_order(graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>) -> (finish_order: AVLTreeSeqStPerS<usize>)
        requires
            spec_toposortstper_wf(graph),
            graph@.len() < usize::MAX,
        ensures
            finish_order.spec_avltreeseqstper_wf(),
            finish_order@.len() == graph@.len(),
            forall|i: int| 0 <= i < finish_order@.len()
                ==> (#[trigger] finish_order@[i] as int) < graph@.len(),
    {
        let n = graph.length();
        let mut visited: Vec<bool> = Vec::new();
        let mut finish_order: Vec<usize> = Vec::new();
        let mut j: usize = 0;
        while j < n
            invariant
                j <= n,
                visited@.len() == j as int,
                forall|k: int| #![trigger visited@[k]] 0 <= k < j as int ==> !visited@[k],
            decreases n - j,
        {
            visited.push(false);
            j = j + 1;
        // Veracity: NEEDED proof block
        }

        proof {
            lemma_all_false_num_false_eq_len(visited@);
        }

        let mut start: usize = 0;
        while start < n
            invariant
                start <= n,
                n == graph@.len(),
                n < usize::MAX,
                visited@.len() == n,
                spec_toposortstper_wf(graph),
                forall|k: int| 0 <= k < finish_order@.len()
                    ==> (#[trigger] finish_order@[k] as int) < graph@.len(),
                forall|j: int| 0 <= j < start as int ==> #[trigger] visited@[j],
                finish_order@.len() + spec_num_false(visited@) == n,
            decreases n - start,
        {
            if !visited[start] {
                // Veracity: NEEDED proof block
                let ghost pre_vis = visited@;
                dfs_finish_order(graph, &mut visited, &mut finish_order, start);
                // dfs_finish_order ensures visited@[start as int] and monotonicity.
                proof {
                    // Veracity: NEEDED assert
                    assert forall|j: int| 0 <= j < start as int + 1
                        implies #[trigger] visited@[j] by {
                        if j < start as int {
                            // Veracity: NEEDED assert
                            assert(pre_vis[j]);
                        }
                    };
                }
            } else {
// Veracity: NEEDED proof block
// Veracity: UNNEEDED assert                 assert(visited@[start as int]);
            }
            start = start + 1;
        }
        proof {
            lemma_all_true_num_false_zero(visited@);
        }
        let result_len = finish_order.len();
        let mut reversed: Vec<usize> = Vec::new();
        let mut k: usize = result_len;
        while k > 0
            invariant
                k <= result_len,
                result_len == finish_order@.len(),
                result_len == n,
                n == graph@.len(),
                n < usize::MAX,
                forall|j: int| 0 <= j < finish_order@.len()
                    ==> (#[trigger] finish_order@[j] as int) < graph@.len(),
                forall|j: int| 0 <= j < reversed@.len()
                    ==> (#[trigger] reversed@[j] as int) < graph@.len(),
                reversed@.len() == (result_len - k) as nat,
            decreases k,
        {
            k = k - 1;
            reversed.push(finish_order[k]);
        }
        // Veracity: NEEDED assert (speed hint)
        assert(reversed@.len() < usize::MAX);
        AVLTreeSeqStPerS::from_vec(reversed)
    }

    /// Transposes a directed graph (reverses all edges).
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|+|E|), Span O(|V|+|E|) — iterates all edges; St sequential.
    fn transpose_graph(graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>) -> (transposed: ArraySeqStPerS<ArraySeqStPerS<usize>>)
        requires spec_toposortstper_wf(graph),
        ensures
            transposed@.len() == graph@.len(),
            spec_toposortstper_wf(&transposed),
    {
        let n = graph.length();
        let mut adj_vecs: Vec<Vec<usize>> = Vec::new();
        let mut k: usize = 0;
        while k < n
            invariant
                k <= n,
                adj_vecs@.len() == k as int,
                forall|w: int, j: int|
                    0 <= w < k as int && 0 <= j < adj_vecs@[w].len()
                    ==> (#[trigger] adj_vecs@[w][j] as int) < n,
            decreases n - k,
        {
            adj_vecs.push(Vec::new());
            k = k + 1;
        }

        let mut u: usize = 0;
        while u < n
            invariant
                u <= n,
                n == graph@.len(),
                adj_vecs@.len() == n,
                spec_toposortstper_wf(graph),
                forall|w: int, j: int|
                    0 <= w < n as int && 0 <= j < adj_vecs@[w].len()
                    ==> (#[trigger] adj_vecs@[w][j] as int) < n,
            decreases n - u,
        {
            // Veracity: NEEDED assert (speed hint)
            assert((u as int) < graph@.len());
            let neighbors = graph.nth(u);
            let neighbors_len = neighbors.length();
            // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED proof block             assert(neighbors_len as int == neighbors.spec_len());
// Veracity: UNNEEDED assert             assert(neighbors_len == graph@[u as int].len());
            // Bridge neighbors to graph view.
            // Veracity: NEEDED assert (speed hint)
            assert(*neighbors == graph.spec_index(u as int));
            proof { lemma_graph_per_view_bridge(graph, neighbors, u as int); }
            // Veracity: NEEDED assert (speed hint)
            assert(neighbors@ =~= graph@[u as int]);
            let mut i: usize = 0;
            while i < neighbors_len
                invariant
                    i <= neighbors_len,
                    neighbors_len as int == neighbors.spec_len(),
                    neighbors_len == graph@[u as int].len(),
                    neighbors@ =~= graph@[u as int],
                    *neighbors == graph.spec_index(u as int),
                    (u as int) < graph@.len(),
                    u < n,
                    adj_vecs@.len() == n,
                    n == graph@.len(),
                    spec_toposortstper_wf(graph),
                    forall|w: int, j: int|
                        // Veracity: NEEDED proof block
                        0 <= w < n as int && 0 <= j < adj_vecs@[w].len()
                        ==> (#[trigger] adj_vecs@[w][j] as int) < n,
                decreases neighbors_len - i,
            {
                let v = *neighbors.nth(i);
                proof { lemma_usize_per_view_eq_spec_index(neighbors); }
                // Veracity: NEEDED assert (speed hint)
                assert(v == neighbors@[i as int]);
// Veracity: UNNEEDED assert                 assert(v == graph@[u as int][i as int]);
                // Veracity: NEEDED assert (speed hint)
                assert(graph@[u as int][i as int] < graph@.len());
                // Veracity: NEEDED assert (speed hint)
                assert(v < n);
                let mut temp = adj_vecs.remove(v);
                temp.push(u);
                adj_vecs.insert(v, temp);
                // Veracity: NEEDED assert (speed hint)
                assert(forall|w: int, j: int|
                    0 <= w < n as int && 0 <= j < adj_vecs@[w].len()
                    ==> (#[trigger] adj_vecs@[w][j] as int) < n);
                i = i + 1;
            }
            u = u + 1;
        }

        let mut result_vecs: Vec<ArraySeqStPerS<usize>> = Vec::new();
        let mut m: usize = 0;
        while m < n
            invariant
                m <= n,
                n == graph@.len(),
                adj_vecs@.len() == n,
                result_vecs@.len() == m as int,
                forall|w: int, j: int|
                    0 <= w < n as int && 0 <= j < adj_vecs@[w].len()
                    ==> (#[trigger] adj_vecs@[w][j] as int) < n,
                forall|r: int, j: int|
                    0 <= r < m as int && 0 <= j < result_vecs@[r]@.len()
                    // Veracity: NEEDED proof block
                    ==> (#[trigger] result_vecs@[r]@[j]) < graph@.len(),
            decreases n - m,
        {
            let cloned_vec = adj_vecs[m].clone();
            let ghost cv_view = cloned_vec@;
            let new_arr = ArraySeqStPerS::from_vec(cloned_vec);
            proof {
                lemma_usize_per_view_eq_spec_index(&new_arr);
// Veracity: UNNEEDED assert                 assert(cv_view =~= adj_vecs@[m as int]@);
                // Veracity: NEEDED assert (speed hint)
                assert forall|j: int| 0 <= j < new_arr@.len()
                    implies (#[trigger] new_arr@[j]) < graph@.len() by {
                    // Veracity: NEEDED assert (speed hint)
                    assert(new_arr@[j] == new_arr.spec_index(j));
// Veracity: UNNEEDED assert                     assert(new_arr.spec_index(j) == cv_view[j]);
                    // Veracity: NEEDED assert (speed hint)
                    // Veracity: NEEDED proof block
                    assert((adj_vecs@[m as int][j] as int) < (n as int));
                };
            }
            result_vecs.push(new_arr);
            m = m + 1;
        }
        let transposed = ArraySeqStPerS::from_vec(result_vecs);
        proof {
            // Veracity: NEEDED assert (speed hint)
            assert(transposed@.len() == n as nat);
            // Veracity: NEEDED assert
            assert forall|v: int, i: int|
                0 <= v < transposed@.len() && 0 <= i < transposed@[v].len()
                implies (#[trigger] transposed@[v][i]) < transposed@.len() by {
                // Veracity: NEEDED assert
                assert(transposed.spec_index(v) == result_vecs@[v]);
                // Veracity: NEEDED assert (speed hint)
                assert(result_vecs@[v]@[i] < graph@.len());
            };
        }
        assert(spec_toposortstper_wf(&transposed));
        transposed
    }

    /// Runtime check that all neighbor indices are valid vertex indices.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|+|E|), Span O(|V|+|E|) — checks all edges; St sequential.
    // veracity: no_requires
    fn check_wf_adj_list_per(graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>) -> (valid: bool)
        ensures valid ==> spec_toposortstper_wf(graph),
    {
        let n = graph.length();
        let mut u: usize = 0;
        while u < n
            invariant
                u <= n,
                n == graph@.len(),
                forall|v: int, i: int|
                    0 <= v < u as int && 0 <= i < graph@[v].len()
                    ==> (#[trigger] graph@[v][i]) < graph@.len(),
            decreases n - u,
        // Veracity: NEEDED proof block
        {
            let neighbors = graph.nth(u);
            let neighbors_len = neighbors.length();
// Veracity: UNNEEDED assert             assert(neighbors_len as int == neighbors.spec_len());
// Veracity: UNNEEDED assert             assert(neighbors_len == graph@[u as int].len());
            // Bridge neighbors to graph view.
            // Veracity: NEEDED assert (speed hint)
            assert(*neighbors == graph.spec_index(u as int));
            proof { lemma_graph_per_view_bridge(graph, neighbors, u as int); }
            // Veracity: NEEDED assert (speed hint)
            assert(neighbors@ =~= graph@[u as int]);
            let mut i: usize = 0;
            while i < neighbors_len
                invariant
                    i <= neighbors_len,
                    neighbors_len as int == neighbors.spec_len(),
                    u < n,
                    n == graph@.len(),
                    neighbors_len == graph@[u as int].len(),
                    neighbors@ =~= graph@[u as int],
                    *neighbors == graph.spec_index(u as int),
                    // Veracity: NEEDED proof block
                    forall|v: int, j: int|
                        0 <= v < u as int && 0 <= j < graph@[v].len()
                        ==> (#[trigger] graph@[v][j]) < graph@.len(),
                    forall|j: int|
                        0 <= j < i as int
                        ==> (#[trigger] graph@[u as int][j]) < graph@.len(),
                decreases neighbors_len - i,
            {
                let neighbor = *neighbors.nth(i);
                proof { lemma_usize_per_view_eq_spec_index(neighbors); }
                // Veracity: NEEDED assert (speed hint)
                assert(neighbor == graph@[u as int][i as int]);
                if neighbor >= n {
                    return false;
                }
                i = i + 1;
            }
            u = u + 1;
        }
        true
    }

    /// DFS reachability using Vec<bool> for termination and persistent set
    /// for component accumulation (same pattern as DFSStPer::dfs_recursive).
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|+|E|), Span O(|V|+|E|) — DFS collecting component; St sequential.
    fn dfs_reach(
        graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>,
        visited_bool: &mut Vec<bool>,
        component: AVLTreeSetStPer<usize>,
        vertex: usize,
    ) -> (out: AVLTreeSetStPer<usize>)
        requires
            vertex < old(visited_bool)@.len(),
            old(visited_bool)@.len() == graph@.len(),
            spec_toposortstper_wf(graph),
            component.spec_avltreesetstper_wf(),
            component@.len() + spec_num_false(old(visited_bool)@) < usize::MAX as nat,
        ensures
            visited_bool@.len() == graph@.len(),
            forall|j: int|
                0 <= j < visited_bool@.len() && old(visited_bool)@[j]
                ==> #[trigger] visited_bool@[j],
            spec_num_false(visited_bool@) <= spec_num_false(old(visited_bool)@),
            out.spec_avltreesetstper_wf(),
            out@.len() + spec_num_false(visited_bool@) <= component@.len() + spec_num_false(old(visited_bool)@),
            visited_bool@[vertex as int],
        decreases spec_num_false(old(visited_bool)@),
    {
        // Veracity: NEEDED proof block
        let ghost init_comp_len = component@.len();

        if visited_bool[vertex] {
            // Veracity: NEEDED assert (speed hint)
            assert(visited_bool@[vertex as int]);
            return component;
        }
        // Veracity: NEEDED assert (speed hint)
        assert(!old(visited_bool)@[vertex as int]);
        visited_bool.set(vertex, true);
        proof {
            lemma_set_true_decreases_num_false(old(visited_bool)@, vertex as int);
            lemma_set_true_num_false_eq(old(visited_bool)@, vertex as int);
        }
// Veracity: UNNEEDED assert         assert(visited_bool@ =~= old(visited_bool)@.update(vertex as int, true));
        // Veracity: NEEDED assert (speed hint)
        assert(visited_bool@[vertex as int]);
// Veracity: UNNEEDED assert         assert(spec_num_false(visited_bool@) < spec_num_false(old(visited_bool)@));
        // Veracity: NEEDED assert (speed hint)
        assert(spec_num_false(visited_bool@) == spec_num_false(old(visited_bool)@) - 1);
        // Veracity: NEEDED assert (speed hint)
        assert(visited_bool@.len() == graph@.len());

        // Monotonicity.
        // Veracity: NEEDED assert
        assert forall|j: int| 0 <= j < visited_bool@.len() && old(visited_bool)@[j]
            implies #[trigger] visited_bool@[j] by {};

        // Veracity: NEEDED assert (speed hint)
        assert(component@.len() + 1 < usize::MAX as nat);
        let mut component = component.insert(vertex);
        // After insert: combined bound maintained.
        // Veracity: NEEDED assert (speed hint)
        assert(component@.len() + spec_num_false(visited_bool@) <= init_comp_len + spec_num_false(old(visited_bool)@));

// Veracity: UNNEEDED assert         assert((vertex as int) < graph@.len());
// Veracity: UNNEEDED proof block         // Veracity: NEEDED assert (speed hint)
        assert(vertex < graph.spec_len());
        let neighbors = graph.nth(vertex);
        let neighbors_len = neighbors.length();
        // Veracity: NEEDED assert (speed hint)
        assert(neighbors_len as int == neighbors.spec_len());
// Veracity: UNNEEDED assert         assert(neighbors_len == graph@[vertex as int].len());

        // Bridge neighbors to graph view.
        // Veracity: NEEDED assert (speed hint)
        assert(*neighbors == graph.spec_index(vertex as int));
        proof { lemma_graph_per_view_bridge(graph, neighbors, vertex as int); }
        // Veracity: NEEDED assert (speed hint)
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
                visited_bool@.len() == graph@.len(),
                spec_toposortstper_wf(graph),
                // Veracity: NEEDED proof block
                forall|j: int|
                    0 <= j < visited_bool@.len() && old(visited_bool)@[j]
                    ==> #[trigger] visited_bool@[j],
                spec_num_false(visited_bool@) < spec_num_false(old(visited_bool)@),
                component.spec_avltreesetstper_wf(),
                component@.len() + spec_num_false(visited_bool@) < usize::MAX as nat,
                component@.len() + spec_num_false(visited_bool@) <= init_comp_len + spec_num_false(old(visited_bool)@),
                visited_bool@[vertex as int],
            decreases neighbors_len - i,
        {
            let neighbor = *neighbors.nth(i);
            proof { lemma_usize_per_view_eq_spec_index(neighbors); }
            // Veracity: NEEDED assert (speed hint)
            assert(neighbor == neighbors@[i as int]);
            // Veracity: NEEDED assert (speed hint)
            assert(neighbor == graph@[vertex as int][i as int]);
// Veracity: UNNEEDED assert             assert(graph@[vertex as int][i as int] < graph@.len());
// Veracity: UNNEEDED assert             assert(neighbor < graph@.len());
            let ghost pre_vis = visited_bool@;
            component = dfs_reach(graph, visited_bool, component, neighbor);
            // visited_bool@[vertex] maintained via monotonicity.
            // Veracity: NEEDED assert (speed hint)
            assert(visited_bool@[vertex as int]) by {
// Veracity: UNNEEDED assert                 assert(pre_vis[vertex as int]);
            };
            i = i + 1;
        }
        component
    }

    impl SCCStPerTrait for SCCStPer {
        /// Finds strongly connected components in a directed graph.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|+|E|), Span O(|V|+|E|) — Kosaraju's: finish-order DFS + transpose + component DFS; St sequential.
        fn scc(graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>) -> AVLTreeSeqStPerS<AVLTreeSetStPer<usize>>
        {
            let finish_order = compute_finish_order(graph);
            let transposed = transpose_graph(graph);

            let n = transposed.length();
            let mut visited_bool: Vec<bool> = Vec::new();
            // Veracity: NEEDED proof block
            let mut j: usize = 0;
            while j < n
                invariant
                    j <= n,
                    visited_bool@.len() == j as int,
                    forall|k: int| #![trigger visited_bool@[k]] 0 <= k < j as int ==> !visited_bool@[k],
                decreases n - j,
            {
                visited_bool.push(false);
                j = j + 1;
            }

            proof {
                lemma_all_false_num_false_eq_len(visited_bool@);
            }

            let finish_len = finish_order.length();
            let mut components_vec: Vec<AVLTreeSetStPer<usize>> = Vec::new();

            if finish_len > 0 {
                // Handle first vertex to guarantee at least one component.
                let vertex = *finish_order.nth(0usize);
                // Veracity: NEEDED assert (speed hint)
                assert((vertex as int) < n);
                let component = AVLTreeSetStPer::empty();
                // Veracity: NEEDED assert (speed hint)
                assert(component@.len() + spec_num_false(visited_bool@) < usize::MAX as nat) by {
                    // Veracity: NEEDED assert (speed hint)
                    assert(component@.len() == 0nat);
// Veracity: UNNEEDED assert                     assert(spec_num_false(visited_bool@) <= n as nat);
                };
                let component = dfs_reach(&transposed, &mut visited_bool, component, vertex);
                components_vec.push(component);

                let mut i: usize = 1;
                while i < finish_len
                    invariant
                        1 <= i <= finish_len,
                        finish_len as int == finish_order@.len(),
                        finish_len == n,
                        finish_order.spec_avltreeseqstper_wf(),
                        forall|k: int| 0 <= k < finish_order@.len()
                            ==> (#[trigger] finish_order@[k] as int) < n,
                        visited_bool@.len() == n,
                        n == transposed@.len(),
                        n == graph@.len(),
                        n < usize::MAX,
                        spec_toposortstper_wf(&transposed),
                        spec_num_false(visited_bool@) <= n,
                        forall|k: int| 0 <= k < components_vec@.len()
                            ==> (#[trigger] components_vec@[k]).spec_avltreesetstper_wf(),
                        components_vec@.len() >= 1,
                        components_vec@.len() <= i,
                    decreases finish_len - i,
                {
                    // Veracity: NEEDED assert (speed hint)
                    assert((i as int) < finish_order@.len());
                    let vertex = *finish_order.nth(i);
                    if vertex < n && !visited_bool[vertex] {
                        let component = AVLTreeSetStPer::empty();
                        // Veracity: NEEDED assert (speed hint)
                        assert(component@.len() + spec_num_false(visited_bool@) < usize::MAX as nat) by {
// Veracity: UNNEEDED assert                             assert(component@.len() == 0nat);
// Veracity: UNNEEDED assert                             assert(spec_num_false(visited_bool@) <= n as nat);
                        };
                        let component = dfs_reach(&transposed, &mut visited_bool, component, vertex);
                        if component.size() > 0 {
                            components_vec.push(component);
                        }
                    }
                    i = i + 1;
                }
            }
            // Veracity: NEEDED assert (speed hint)
            assert(components_vec@.len() >= 1 || graph@.len() == 0);
            // Veracity: NEEDED assert (speed hint)
            assert(components_vec@.len() < usize::MAX);
            AVLTreeSeqStPerS::from_vec(components_vec)
        }
    } // impl SCCStPerTrait

    } // verus!

    //		Section 14. derive impls outside verus!


    impl std::fmt::Debug for SCCStPer {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SCCStPer")
        }
    }

    impl std::fmt::Display for SCCStPer {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SCCStPer")
        }
    }
}
