// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Umut Acar, Guy Blelloch and Brian Milnes
//! REVIEWED: NO
//! Strongly Connected Components - Sequential Ephemeral (Chapter 55, Algorithm 55.18).
//! Finds all strongly connected components using ephemeral structures.
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

pub mod SCCStEph {


    //		Section 2. imports

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::{AVLTreeSeqStEphS, AVLTreeSeqStEphTrait};
    use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
    use crate::Chap55::TopoSortStEph::TopoSortStEph::dfs_finish_order;
    #[cfg(verus_keep_ghost)]
    use crate::Chap55::DFSSpecsAndLemmas::DFSSpecsAndLemmas::{
        spec_num_false, lemma_set_true_decreases_num_false, lemma_set_true_num_false_eq,
        lemma_all_true_num_false_zero, lemma_all_false_num_false_eq_len,
        lemma_bool_view_eq_spec_index, lemma_usize_view_eq_spec_index, lemma_graph_view_bridge,
    };
    #[cfg(verus_keep_ghost)]
    use crate::Chap55::TopoSortStEph::TopoSortStEph::spec_toposortsteph_wf;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_full_trigger;
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


    pub type T<N> = ArraySeqStEphS<ArraySeqStEphS<N>>;


    pub struct SCCStEph;

    //		Section 8. traits


    pub trait SCCStEphTrait {
        /// Finds strongly connected components in a directed graph (Algorithm 55.18).
        /// - Alg Analysis: APAS (Ch55 CS 55.8): Work O(|V| + |E|), Span O(|V| + |E|) 
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V| + |E|), Span O(|V| + |E|)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V| + |E|), Span O(|V| + |E|) — agrees with APAS.
        fn scc(graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>) -> (components: AVLTreeSeqStEphS<AVLTreeSetStEph<usize>>)
            requires
                spec_toposortsteph_wf(graph),
                graph@.len() < usize::MAX,
            ensures
                components@.len() >= 1 || graph@.len() == 0,
            ;
    }

    //		Section 9. impls


    /// Computes the finish order for SCC (decreasing finish times).
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|+|E|), Span O(|V|+|E|) — full DFS producing finish-time ordering; St sequential.
    fn compute_finish_order(graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>) -> (finish_order: AVLTreeSeqStEphS<usize>)
        requires
            spec_toposortsteph_wf(graph),
            graph@.len() < usize::MAX,
        ensures
            finish_order.spec_avltreeseqsteph_wf(),
            finish_order@.len() == graph@.len(),
            forall|i: int| 0 <= i < finish_order@.len()
                ==> (#[trigger] finish_order@[i] as int) < graph@.len(),
    {
        let n = graph.length();
        let init_false = |_x: usize| -> (r: bool)
            ensures !r
        { false };
        let mut visited = ArraySeqStEphS::tabulate(&init_false, n);
        let mut finish_order: Vec<usize> = Vec::new();

        // Veracity: NEEDED proof block
        proof {
// Veracity: UNNEEDED assert             assert forall|j: int| 0 <= j < visited@.len() implies !visited@[j] by {
// Veracity: UNNEEDED assert                 // tabulate ensures: init_false.ensures((j as usize,), visited.seq@[j])
// Veracity: UNNEEDED assert                 // which means !visited.seq@[j].
// Veracity: UNNEEDED assert                 lemma_bool_view_eq_spec_index(&visited);
// Veracity: UNNEEDED assert                 // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED assert                 assert(!visited.spec_index(j));
// Veracity: UNNEEDED assert             }
            lemma_all_false_num_false_eq_len(visited@);
        }

        let mut start: usize = 0;
        while start < n
            invariant
                start <= n,
                n == graph@.len(),
                n < usize::MAX,
                visited@.len() == n,
                visited.spec_len() == n,
                spec_toposortsteph_wf(graph),
                forall|k: int| 0 <= k < finish_order@.len()
                    ==> (#[trigger] finish_order@[k] as int) < graph@.len(),
                forall|j: int| 0 <= j < start as int ==> #[trigger] visited@[j],
                finish_order@.len() + spec_num_false(visited@) == n,
            decreases n - start,
        // Veracity: NEEDED proof block
        {
            proof { lemma_bool_view_eq_spec_index(&visited); }
// Veracity: UNNEEDED assert             assert(start < visited.spec_len());
            if !*visited.nth(start) {
                let ghost pre_vis = visited@;
                // Veracity: NEEDED proof block
                dfs_finish_order(graph, &mut visited, &mut finish_order, start);
                // dfs_finish_order ensures visited@[start as int] and monotonicity.
                proof {
                    lemma_bool_view_eq_spec_index(&visited);
                    // Veracity: NEEDED assert
                    assert forall|j: int| 0 <= j < start as int + 1
                        implies #[trigger] visited@[j] by {
                        if j < start as int {
                            // pre_vis[j] was true (from invariant), monotonicity preserves it.
                            // Veracity: NEEDED assert
                            assert(pre_vis[j]);
                        }
                        // j == start: ensured by dfs_finish_order.
                    // Veracity: NEEDED proof block
                    };
                }
            } else {
                proof { lemma_bool_view_eq_spec_index(&visited); }
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
            // Veracity: NEEDED proof block
            reversed.push(finish_order[k]);
        }
        // Veracity: NEEDED assert (speed hint)
        assert(reversed@.len() < usize::MAX);
        // Veracity: NEEDED assert
        proof { assert(obeys_feq_full_trigger::<usize>()); }
        AVLTreeSeqStEphS::from_vec(reversed)
    }

    /// Transposes a directed graph (reverses all edges).
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|+|E|), Span O(|V|+|E|) — iterates all edges once to build transposed adj lists; St sequential.
    fn transpose_graph(graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>) -> (transposed: ArraySeqStEphS<ArraySeqStEphS<usize>>)
        requires spec_toposortsteph_wf(graph),
        ensures
            transposed@.len() == graph@.len(),
            spec_toposortsteph_wf(&transposed),
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
                spec_toposortsteph_wf(graph),
                forall|w: int, j: int|
                    0 <= w < n as int && 0 <= j < adj_vecs@[w].len()
                    ==> (#[trigger] adj_vecs@[w][j] as int) < n,
            decreases n - u,
        {
            // Veracity: NEEDED assert (speed hint)
            assert((u as int) < graph@.len());
            let neighbors = graph.nth(u);
            let neighbors_len = neighbors.length();
// Veracity: UNNEEDED proof block             // Veracity: NEEDED assert (speed hint)
            assert(neighbors_len as int == neighbors.spec_len());
// Veracity: UNNEEDED assert             assert(neighbors_len == graph@[u as int].len());
            // Bridge neighbors to graph view.
            // Veracity: NEEDED assert (speed hint)
            assert(*neighbors == graph.spec_index(u as int));
            proof { lemma_graph_view_bridge(graph, neighbors, u as int); }
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
                    spec_toposortsteph_wf(graph),
                    // Veracity: NEEDED proof block
                    forall|w: int, j: int|
                        0 <= w < n as int && 0 <= j < adj_vecs@[w].len()
                        ==> (#[trigger] adj_vecs@[w][j] as int) < n,
                decreases neighbors_len - i,
            {
                let v = *neighbors.nth(i);
                proof { lemma_usize_view_eq_spec_index(neighbors); }
// Veracity: UNNEEDED assert                 assert(v == neighbors@[i as int]);
// Veracity: UNNEEDED assert                 assert(v == graph@[u as int][i as int]);
                // Veracity: NEEDED assert (speed hint)
                assert(graph@[u as int][i as int] < graph@.len());
// Veracity: UNNEEDED assert                 assert(v < n);
                let mut temp = adj_vecs.remove(v);
                temp.push(u);
                adj_vecs.insert(v, temp);
// Veracity: UNNEEDED assert                 assert(forall|w: int, j: int|
// Veracity: UNNEEDED assert                     0 <= w < n as int && 0 <= j < adj_vecs@[w].len()
// Veracity: UNNEEDED assert                     ==> (#[trigger] adj_vecs@[w][j] as int) < n);
                i = i + 1;
            }
            u = u + 1;
        }

        let mut result_vecs: Vec<ArraySeqStEphS<usize>> = Vec::new();
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
                    // Veracity: NEEDED proof block
                    0 <= r < m as int && 0 <= j < result_vecs@[r]@.len()
                    ==> (#[trigger] result_vecs@[r]@[j]) < graph@.len(),
            decreases n - m,
        {
            let cloned_vec = adj_vecs[m].clone();
            let ghost cv_view = cloned_vec@;
            let new_arr = ArraySeqStEphS::from_vec(cloned_vec);
            proof {
                lemma_usize_view_eq_spec_index(&new_arr);
// Veracity: UNNEEDED assert                 assert(cv_view =~= adj_vecs@[m as int]@);
                // Veracity: NEEDED assert (speed hint)
                assert forall|j: int| 0 <= j < new_arr@.len()
                    implies (#[trigger] new_arr@[j]) < graph@.len() by {
                    // Veracity: NEEDED assert (speed hint)
                    assert(new_arr@[j] == new_arr.spec_index(j));
                    // Veracity: NEEDED assert (speed hint)
                    assert(new_arr.spec_index(j) == cv_view[j]);
                    // Veracity: NEEDED proof block
                    // Veracity: NEEDED assert (speed hint)
                    assert((adj_vecs@[m as int][j] as int) < (n as int));
                };
            }
            result_vecs.push(new_arr);
            m = m + 1;
        }
        let transposed = ArraySeqStEphS::from_vec(result_vecs);
        proof {
            // Veracity: NEEDED assert (speed hint)
            assert(transposed@.len() == n as nat);
            // Bridge: transposed@[v] == result_vecs@[v]@ for each vertex.
            // Veracity: NEEDED assert
            assert forall|v: int, i: int|
                0 <= v < transposed@.len() && 0 <= i < transposed@[v].len()
                implies (#[trigger] transposed@[v][i]) < transposed@.len() by {
                // transposed.spec_index(v) == result_vecs@[v]
                // Veracity: NEEDED assert
                assert(transposed.spec_index(v) == result_vecs@[v]);
                // transposed@[v] == transposed.spec_index(v)@
                // result_vecs@[v]@[i] < graph@.len() from invariant
// Veracity: UNNEEDED assert                 assert(result_vecs@[v]@[i] < graph@.len());
            };
        }
        assert(spec_toposortsteph_wf(&transposed));
        transposed
    }

    /// Runtime check that all neighbor indices are valid vertex indices.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|+|E|), Span O(|V|+|E|) — checks all edges for valid indices; St sequential.
    // veracity: no_requires
    fn check_wf_adj_list_eph(graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>) -> (valid: bool)
        ensures valid ==> spec_toposortsteph_wf(graph),
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
            // Veracity: NEEDED proof block
            decreases n - u,
        {
            let neighbors = graph.nth(u);
            let neighbors_len = neighbors.length();
// Veracity: UNNEEDED assert             assert(neighbors_len as int == neighbors.spec_len());
            // Veracity: NEEDED assert (speed hint)
            assert(neighbors_len == graph@[u as int].len());
            // Bridge neighbors to graph view.
// Veracity: UNNEEDED assert             assert(*neighbors == graph.spec_index(u as int));
            proof { lemma_graph_view_bridge(graph, neighbors, u as int); }
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
// Veracity: UNNEEDED proof block                     *neighbors == graph.spec_index(u as int),
                    forall|v: int, j: int|
                        0 <= v < u as int && 0 <= j < graph@[v].len()
                        ==> (#[trigger] graph@[v][j]) < graph@.len(),
                    forall|j: int|
                        0 <= j < i as int
                        ==> (#[trigger] graph@[u as int][j]) < graph@.len(),
                decreases neighbors_len - i,
            {
                let neighbor = *neighbors.nth(i);
                proof { lemma_usize_view_eq_spec_index(neighbors); }
// Veracity: UNNEEDED assert                 assert(neighbor == graph@[u as int][i as int]);
                if neighbor >= n {
                    return false;
                }
                i = i + 1;
            }
            u = u + 1;
        }
        true
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|+|E|), Span O(|V|+|E|) — DFS collecting reachable component; St sequential.
    fn dfs_reach(
        graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>,
        visited: &mut ArraySeqStEphS<bool>,
        component: &mut AVLTreeSetStEph<usize>,
        vertex: usize,
    )
        requires
            vertex < old(visited)@.len(),
            old(visited)@.len() == graph@.len(),
            spec_toposortsteph_wf(graph),
            old(component).spec_avltreesetsteph_wf(),
            old(component)@.len() + spec_num_false(old(visited)@) < usize::MAX as nat,
        ensures
            // Veracity: NEEDED proof block
            visited@.len() == graph@.len(),
            forall|j: int|
                0 <= j < visited@.len() && #[trigger] old(visited)@[j]
                ==> visited@[j],
            spec_num_false(visited@) <= spec_num_false(old(visited)@),
            component.spec_avltreesetsteph_wf(),
            component@.len() + spec_num_false(visited@) <= old(component)@.len() + spec_num_false(old(visited)@),
            visited@[vertex as int],
        decreases spec_num_false(old(visited)@),
    {
        proof { lemma_bool_view_eq_spec_index(visited); }
        // Veracity: NEEDED assert (speed hint)
        assert(visited.spec_len() == visited@.len());
        // Veracity: NEEDED assert (speed hint)
        assert(vertex < visited.spec_len());
        // Veracity: NEEDED proof block
        if *visited.nth(vertex) {
// Veracity: UNNEEDED assert             assert(visited@[vertex as int]);
            return;
        }
        // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED proof block         assert(!old(visited)@[vertex as int]);
        // Veracity: NEEDED assert (speed hint)
        assert(vertex < visited.spec_len());
        let set_ok = visited.set(vertex, true);
        // Veracity: NEEDED assert (speed hint)
        assert(set_ok.is_ok());
        proof {
            lemma_set_true_decreases_num_false(old(visited)@, vertex as int);
            lemma_set_true_num_false_eq(old(visited)@, vertex as int);
        }

        // Establish visited@ == old(visited)@.update(vertex, true).
        proof { lemma_bool_view_eq_spec_index(visited); }
// Veracity: UNNEEDED assert         assert forall|j: int| 0 <= j < visited@.len()
// Veracity: UNNEEDED assert             implies #[trigger] visited@[j] == old(visited)@.update(vertex as int, true)[j] by {
// Veracity: UNNEEDED assert             // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED assert             assert(visited@[j] == visited.spec_index(j));
// Veracity: UNNEEDED assert             if j == vertex as int {
// Veracity: UNNEEDED assert                 // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED assert                 assert(visited.spec_index(j) == true);
// Veracity: UNNEEDED assert             } else {
// Veracity: UNNEEDED assert                 // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED assert                 assert(visited.spec_index(j) == old(visited).spec_index(j));
// Veracity: UNNEEDED assert             }
// Veracity: UNNEEDED assert         };
        // Veracity: NEEDED assert
        assert(visited@ =~= old(visited)@.update(vertex as int, true));
        // Veracity: NEEDED assert (speed hint)
        assert(visited@[vertex as int]);
        // Veracity: NEEDED assert (speed hint)
        assert(spec_num_false(visited@) < spec_num_false(old(visited)@));
        // Veracity: NEEDED assert (speed hint)
        assert(visited@.len() == graph@.len());

        // Monotonicity.
        // Veracity: NEEDED assert
        assert forall|j: int| 0 <= j < visited@.len() && #[trigger] old(visited)@[j]
            implies visited@[j] by {};

        // Combined bound: spec_num_false decreased by 1, so component can grow by 1.
// Veracity: UNNEEDED assert         assert(spec_num_false(visited@) == spec_num_false(old(visited)@) - 1);
        // Veracity: NEEDED assert (speed hint)
        assert(old(component)@.len() + 1 < usize::MAX as nat);
        component.insert(vertex);
        // After insert: component@.len() increased by at most 1.
        // component@.len() + spec_num_false(visited@) <= old(component)@.len() + 1 + spec_num_false(old(visited)@) - 1
        // Veracity: NEEDED assert (speed hint)
        assert(component@.len() + spec_num_false(visited@) <= old(component)@.len() + spec_num_false(old(visited)@));

        // Veracity: NEEDED assert (speed hint)
        assert((vertex as int) < graph@.len());
// Veracity: UNNEEDED proof block         // Veracity: NEEDED assert (speed hint)
        assert(vertex < graph.spec_len());
        let neighbors = graph.nth(vertex);
        let neighbors_len = neighbors.length();
        // Veracity: NEEDED assert (speed hint)
        assert(neighbors_len as int == neighbors.spec_len());
        // Veracity: NEEDED assert (speed hint)
        assert(neighbors_len == graph@[vertex as int].len());

        // Bridge neighbors to graph view.
        // Veracity: NEEDED assert (speed hint)
        assert(*neighbors == graph.spec_index(vertex as int));
        proof { lemma_graph_view_bridge(graph, neighbors, vertex as int); }
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
                visited.spec_len() == graph@.len(),
                // Veracity: NEEDED proof block
                spec_toposortsteph_wf(graph),
                component.spec_avltreesetsteph_wf(),
                component@.len() + spec_num_false(visited@) < usize::MAX as nat,
                component@.len() + spec_num_false(visited@) <= old(component)@.len() + spec_num_false(old(visited)@),
                forall|j: int|
                    0 <= j < visited@.len() && #[trigger] old(visited)@[j]
                    ==> visited@[j],
                spec_num_false(visited@) < spec_num_false(old(visited)@),
                visited@[vertex as int],
            decreases neighbors_len - i,
        {
            let neighbor = *neighbors.nth(i);
            proof { lemma_usize_view_eq_spec_index(neighbors); }
            // Veracity: NEEDED assert (speed hint)
            assert(neighbor == neighbors@[i as int]);
// Veracity: UNNEEDED assert             assert(neighbor == graph@[vertex as int][i as int]);
            // Veracity: NEEDED assert (speed hint)
            assert(graph@[vertex as int][i as int] < graph@.len());
            // Veracity: NEEDED assert (speed hint)
            assert(neighbor < graph@.len());
            let ghost pre_vis = visited@;
            dfs_reach(graph, visited, component, neighbor);
            // visited@[vertex] maintained: pre_vis[vertex] was true, monotonicity preserves it.
// Veracity: UNNEEDED assert             assert(visited@[vertex as int]) by {
// Veracity: UNNEEDED assert                 // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED assert                 assert(pre_vis[vertex as int]);
// Veracity: UNNEEDED assert             };
            i = i + 1;
        }
    }

    impl SCCStEphTrait for SCCStEph {
        /// Finds strongly connected components in a directed graph.
        // Veracity: NEEDED proof block
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|+|E|), Span O(|V|+|E|) — Kosaraju's: two DFS passes + transpose; St sequential.
        fn scc(graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>) -> AVLTreeSeqStEphS<AVLTreeSetStEph<usize>>
        {
            let finish_order = compute_finish_order(graph);
            let transposed = transpose_graph(graph);

            let n = transposed.length();
            let init_false = |_x: usize| -> (r: bool)
                ensures !r
            { false };
            let mut visited = ArraySeqStEphS::tabulate(&init_false, n);
            let mut components_vec: Vec<AVLTreeSetStEph<usize>> = Vec::new();

            proof {
                // Veracity: NEEDED assert (speed hint)
                assert forall|j: int| 0 <= j < visited@.len() implies !visited@[j] by {
                    lemma_bool_view_eq_spec_index(&visited);
                    // Veracity: NEEDED assert (speed hint)
                    assert(!visited.spec_index(j));
                }
                lemma_all_false_num_false_eq_len(visited@);
            }

            let finish_len = finish_order.length();

            if finish_len > 0 {
                // Handle first vertex to guarantee at least one component.
                let vertex = *finish_order.nth(0usize);
                // Veracity: NEEDED assert (speed hint)
                assert((vertex as int) < n);
                // Veracity: NEEDED assert (speed hint)
                assert(vertex < visited.spec_len());
                let mut component = AVLTreeSetStEph::empty();
                // Veracity: NEEDED assert (speed hint)
                assert(component@.len() + spec_num_false(visited@) < usize::MAX as nat) by {
// Veracity: UNNEEDED assert                     assert(component@.len() == 0nat);
// Veracity: UNNEEDED assert                     assert(spec_num_false(visited@) <= n as nat);
                };
                dfs_reach(&transposed, &mut visited, &mut component, vertex);
                components_vec.push(component);

                let mut i: usize = 1;
                while i < finish_len
                    invariant
                        1 <= i <= finish_len,
                        finish_len as int == finish_order@.len(),
                        finish_len == n,
                        finish_order.spec_avltreeseqsteph_wf(),
                        forall|k: int| 0 <= k < finish_order@.len()
                            ==> (#[trigger] finish_order@[k] as int) < n,
                        visited@.len() == n,
                        visited.spec_len() == n,
                        n == transposed@.len(),
// Veracity: UNNEEDED proof block                         n == graph@.len(),
                        n < usize::MAX,
                        spec_toposortsteph_wf(&transposed),
                        spec_num_false(visited@) <= n,
                        forall|k: int| 0 <= k < components_vec@.len()
                            ==> (#[trigger] components_vec@[k]).spec_avltreesetsteph_wf(),
                        components_vec@.len() >= 1,
                        components_vec@.len() <= i,
                    decreases finish_len - i,
                {
// Veracity: UNNEEDED assert                     assert((i as int) < finish_order@.len());
                    let vertex = *finish_order.nth(i);
// Veracity: UNNEEDED assert                     assert((vertex as int) < n);
// Veracity: UNNEEDED assert                     assert(vertex < visited.spec_len());
                    proof { lemma_bool_view_eq_spec_index(&visited); }
                    if !*visited.nth(vertex) {
                        let mut component = AVLTreeSetStEph::empty();
                        // Veracity: NEEDED assert (speed hint)
                        assert(component@.len() + spec_num_false(visited@) < usize::MAX as nat) by {
                            // Veracity: NEEDED assert (speed hint)
                            assert(component@.len() == 0nat);
                            // Veracity: NEEDED assert (speed hint)
// Veracity: UNNEEDED proof block                             assert(spec_num_false(visited@) <= n as nat);
                        };
                        dfs_reach(&transposed, &mut visited, &mut component, vertex);
                        if component.size() > 0 {
                            components_vec.push(component);
                        }
                    }
                    i = i + 1;
                }
            }
// Veracity: UNNEEDED assert             assert(components_vec@.len() >= 1 || graph@.len() == 0);
            // Veracity: NEEDED assert (speed hint)
            assert(components_vec@.len() < usize::MAX);
            // Veracity: NEEDED assert
            proof { assert(obeys_feq_full_trigger::<AVLTreeSetStEph<usize>>()); }
            AVLTreeSeqStEphS::from_vec(components_vec)
        }
    } // impl SCCStEphTrait

    } // verus!

    //		Section 14. derive impls outside verus!


    impl std::fmt::Debug for SCCStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SCCStEph")
        }
    }

    impl std::fmt::Display for SCCStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SCCStEph")
        }
    }
}
