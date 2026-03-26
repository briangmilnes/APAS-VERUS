//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Strongly Connected Components - Sequential Ephemeral (Chapter 55, Algorithm 55.18).
//! Finds all strongly connected components using ephemeral structures.
//! Work: O(|V| + |E|), Span: O(|V| + |E|).

pub mod SCCStEph {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::{AVLTreeSeqStEphS, AVLTreeSeqStEphTrait};
    use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
    use crate::Chap55::TopoSortStEph::TopoSortStEph::{
        spec_toposortsteph_wf, spec_num_false, lemma_set_true_decreases_num_false,
        dfs_finish_order, lemma_all_true_num_false_zero, lemma_all_false_num_false_eq_len,
    };
    use crate::Types::Types::*;

    verus! {

    broadcast use {
        vstd::seq::group_seq_axioms,
        vstd::seq_lib::group_seq_properties,
        crate::vstdplus::feq::feq::group_feq_axioms,
    };

    // Table of Contents
    // 1. module
    // 2. imports
    // 4. type definitions
    // 8. traits
    // 9. impls

    // 4. type definitions

    pub type T<N> = ArraySeqStEphS<ArraySeqStEphS<N>>;
    pub struct SCCStEph;

    // 8. traits

    pub trait SCCStEphTrait {
        /// Finds strongly connected components in a directed graph (Algorithm 55.18).
        /// - APAS: Work O(|V| + |E|), Span O(|V| + |E|) [inherits from DFS cost]
        /// - Claude-Opus-4.6: Work O(|V| + |E|), Span O(|V| + |E|) — agrees with APAS.
        fn scc(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> (components: AVLTreeSeqStEphS<AVLTreeSetStEph<N>>)
            requires
                spec_toposortsteph_wf(graph),
            ensures
                components@.len() >= 1 || graph@.len() == 0,
            ;
    }

    // 9. impls

    /// Computes the finish order for SCC (decreasing finish times).
    #[verifier::external_body]
    fn compute_finish_order(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> (result: AVLTreeSeqStEphS<N>)
        requires spec_toposortsteph_wf(graph),
        ensures
            result.spec_avltreeseqsteph_wf(),
            result@.len() == graph@.len(),
            forall|i: int| 0 <= i < result@.len()
                ==> (#[trigger] result@[i] as int) < graph@.len(),
    {
        let n = graph.length();
        let mut visited = ArraySeqStEphS::tabulate(&|_x| false, n);
        let mut finish_order: Vec<N> = Vec::new();

        proof {
            assert forall|j: int| 0 <= j < visited@.len() implies !visited@[j] by {
                assert(visited@[j] == visited.seq@[j]);
            }
            lemma_all_false_num_false_eq_len(visited@);
        }

        let mut start: usize = 0;
        while start < n
            invariant
                start <= n,
                n == graph@.len(),
                visited@.len() == n,
                visited.spec_len() == n,
                spec_toposortsteph_wf(graph),
                forall|k: int| 0 <= k < finish_order@.len()
                    ==> (#[trigger] finish_order@[k] as int) < graph@.len(),
                forall|j: int| 0 <= j < start as int ==> #[trigger] visited@[j],
                finish_order@.len() + spec_num_false(visited@) == n,
            decreases n - start,
        {
            assert(start < visited.spec_len());
            if !*visited.nth(start) {
                dfs_finish_order(graph, &mut visited, &mut finish_order, start);
            }
            start = start + 1;
        }
        proof {
            lemma_all_true_num_false_zero(visited@);
        }
        let result_len = finish_order.len();
        let mut reversed: Vec<N> = Vec::new();
        let mut k: usize = result_len;
        while k > 0
            invariant
                k <= result_len,
                result_len == finish_order@.len(),
                result_len == n,
                n == graph@.len(),
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
        assert(reversed@.len() == result_len as nat);
        assert(reversed@.len() == n as nat);
        // n is a usize, so n as nat <= usize::MAX as nat. Since result_len is a usize,
        // result_len < usize::MAX (from the invariant result_len == n and n is usize).
        assert(reversed@.len() < usize::MAX) by {
            assert(result_len <= usize::MAX);
            assert(reversed@.len() == result_len as nat);
        };
        AVLTreeSeqStEphS::from_vec(reversed)
    }

    /// Transposes a directed graph (reverses all edges).
    #[verifier::external_body]
    fn transpose_graph(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> (transposed: ArraySeqStEphS<ArraySeqStEphS<N>>)
        requires spec_toposortsteph_wf(graph),
        ensures
            transposed@.len() == graph@.len(),
            spec_toposortsteph_wf(&transposed),
    {
        let n = graph.length();
        let mut adj_vecs: Vec<Vec<N>> = Vec::new();
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
            assert((u as int) < graph@.len());
            let neighbors = graph.nth(u);
            let neighbors_len = neighbors.length();
            assert(neighbors_len as int == neighbors.spec_len());
            assert(neighbors_len == graph@[u as int].len());
            let mut i: usize = 0;
            while i < neighbors_len
                invariant
                    i <= neighbors_len,
                    neighbors_len as int == neighbors.spec_len(),
                    neighbors_len == graph@[u as int].len(),
                    (u as int) < graph@.len(),
                    u < n,
                    adj_vecs@.len() == n,
                    n == graph@.len(),
                    spec_toposortsteph_wf(graph),
                    forall|w: int, j: int|
                        0 <= w < n as int && 0 <= j < adj_vecs@[w].len()
                        ==> (#[trigger] adj_vecs@[w][j] as int) < n,
                decreases neighbors_len - i,
            {
                let v = *neighbors.nth(i);
                assert(graph@[u as int][i as int] < graph@.len());
                assert(v < n);
                let mut temp = adj_vecs.remove(v);
                temp.push(u);
                adj_vecs.insert(v, temp);
                assert(forall|w: int, j: int|
                    0 <= w < n as int && 0 <= j < adj_vecs@[w].len()
                    ==> (#[trigger] adj_vecs@[w][j] as int) < n);
                i = i + 1;
            }
            u = u + 1;
        }

        let mut result_vecs: Vec<ArraySeqStEphS<N>> = Vec::new();
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
                    ==> (#[trigger] result_vecs@[r]@[j]) < graph@.len(),
            decreases n - m,
        {
            result_vecs.push(ArraySeqStEphS::from_vec(adj_vecs[m].clone()));
            m = m + 1;
        }
        let transposed = ArraySeqStEphS::from_vec(result_vecs);
        assert(spec_toposortsteph_wf(&transposed)) by {
            assert(transposed@.len() == n as nat);
            assert forall|v: int, i: int|
                0 <= v < transposed@.len() && 0 <= i < transposed@[v].len()
                implies (#[trigger] transposed@[v][i]) < transposed@.len() by {
                assert(transposed@[v][i] < graph@.len());
            };
        };
        transposed
    }

    /// Runtime check that all neighbor indices are valid vertex indices.
    // veracity: no_requires
    #[verifier::external_body]
    fn check_wf_adj_list_eph(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> (valid: bool)
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
            decreases n - u,
        {
            let neighbors = graph.nth(u);
            let neighbors_len = neighbors.length();
            assert(neighbors_len as int == neighbors.spec_len());
            assert(neighbors_len == graph@[u as int].len());
            let mut i: usize = 0;
            while i < neighbors_len
                invariant
                    i <= neighbors_len,
                    neighbors_len as int == neighbors.spec_len(),
                    u < n,
                    n == graph@.len(),
                    neighbors_len == graph@[u as int].len(),
                    forall|v: int, j: int|
                        0 <= v < u as int && 0 <= j < graph@[v].len()
                        ==> (#[trigger] graph@[v][j]) < graph@.len(),
                    forall|j: int|
                        0 <= j < i as int
                        ==> (#[trigger] graph@[u as int][j]) < graph@.len(),
                decreases neighbors_len - i,
            {
                let neighbor = *neighbors.nth(i);
                if neighbor >= n {
                    return false;
                }
                i = i + 1;
            }
            u = u + 1;
        }
        true
    }

    #[verifier::external_body]
    fn dfs_reach(
        graph: &ArraySeqStEphS<ArraySeqStEphS<N>>,
        visited: &mut ArraySeqStEphS<B>,
        component: &mut AVLTreeSetStEph<N>,
        vertex: N,
    )
        requires
            vertex < old(visited)@.len(),
            old(visited)@.len() == graph@.len(),
            spec_toposortsteph_wf(graph),
            old(component).spec_avltreesetsteph_wf(),
            old(component)@.len() + spec_num_false(old(visited)@) < usize::MAX as nat,
        ensures
            visited@.len() == old(visited)@.len(),
            forall|j: int|
                0 <= j < visited@.len() && #[trigger] old(visited)@[j]
                ==> visited@[j],
            spec_num_false(visited@) <= spec_num_false(old(visited)@),
            component.spec_avltreesetsteph_wf(),
            component@.len() <= old(component)@.len() + spec_num_false(old(visited)@),
        decreases spec_num_false(old(visited)@),
    {
        assert(visited.spec_len() == visited@.len());
        assert(vertex < visited.spec_len());
        if *visited.nth(vertex) {
            return;
        }
        assert(!old(visited)@[vertex as int]);
        assert(vertex < visited.spec_len());
        let set_ok = visited.set(vertex, true);
        assert(set_ok.is_ok());
        proof {
            lemma_set_true_decreases_num_false(old(visited)@, vertex as int);
        }
        // After set: spec_num_false(visited@) == spec_num_false(old(visited)@) - 1.
        // old(component)@.len() + spec_num_false(old(visited)@) < usize::MAX
        // => old(component)@.len() < usize::MAX - spec_num_false(old(visited)@) + 1
        // => old(component)@.len() + 1 <= old(component)@.len() + spec_num_false(old(visited)@) < usize::MAX
        assert(old(component)@.len() + 1 < usize::MAX as nat);
        component.insert(vertex);

        let neighbors = graph.nth(vertex);
        let neighbors_len = neighbors.length();
        assert(neighbors_len as int == neighbors.spec_len());
        assert((vertex as int) < graph@.len());
        assert(neighbors_len == graph@[vertex as int].len());
        let mut i: usize = 0;
        while i < neighbors_len
            invariant
                i <= neighbors_len,
                neighbors_len as int == neighbors.spec_len(),
                neighbors_len == graph@[vertex as int].len(),
                (vertex as int) < graph@.len(),
                visited@.len() == graph@.len(),
                visited.spec_len() == graph@.len(),
                spec_toposortsteph_wf(graph),
                component.spec_avltreesetsteph_wf(),
                component@.len() + spec_num_false(visited@) < usize::MAX as nat,
                forall|j: int|
                    0 <= j < visited@.len() && #[trigger] old(visited)@[j]
                    ==> visited@[j],
                spec_num_false(visited@) < spec_num_false(old(visited)@),
            decreases neighbors_len - i,
        {
            let neighbor = *neighbors.nth(i);
            assert(graph@[vertex as int][i as int] < graph@.len());
            dfs_reach(graph, visited, component, neighbor);
            i = i + 1;
        }
    }

    impl SCCStEphTrait for SCCStEph {
        /// Finds strongly connected components in a directed graph.
        #[verifier::external_body]
        fn scc(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> AVLTreeSeqStEphS<AVLTreeSetStEph<N>>
        {
            let finish_order = compute_finish_order(graph);
            // transpose_graph ensures spec_toposortsteph_wf(&transposed).
            let transposed = transpose_graph(graph);

            let n = transposed.length();
            let mut visited = ArraySeqStEphS::tabulate(&|_x| false, n);
            let mut components_vec: Vec<AVLTreeSetStEph<N>> = Vec::new();

            proof {
                assert forall|j: int| 0 <= j < visited@.len() implies !visited@[j] by {
                    assert(visited@[j] == visited.seq@[j]);
                }
                lemma_all_false_num_false_eq_len(visited@);
            }

            let finish_len = finish_order.length();
            let mut i: usize = 0;
            while i < finish_len
                invariant
                    i <= finish_len,
                    finish_len as int == finish_order@.len(),
                    finish_order.spec_avltreeseqsteph_wf(),
                    forall|k: int| 0 <= k < finish_order@.len()
                        ==> (#[trigger] finish_order@[k] as int) < n,
                    visited@.len() == n,
                    visited.spec_len() == n,
                    n == transposed@.len(),
                    n == graph@.len(),
                    spec_toposortsteph_wf(&transposed),
                    spec_num_false(visited@) + i as nat == n,
                    forall|k: int| 0 <= k < components_vec@.len()
                        ==> (#[trigger] components_vec@[k]).spec_avltreesetsteph_wf(),
                    components_vec@.len() < usize::MAX,
                decreases finish_len - i,
            {
                assert((i as int) < finish_order@.len());
                assert(finish_order.spec_avltreeseqsteph_wf());
                let vertex = *finish_order.nth(i);
                assert((vertex as int) < n);
                assert(vertex < visited.spec_len());
                if !*visited.nth(vertex) {
                    let mut component = AVLTreeSetStEph::empty();
                    // empty@.len() == 0, spec_num_false(visited@) <= n, so 0 + spec_num_false < usize::MAX.
                    assert(component@.len() + spec_num_false(visited@) < usize::MAX as nat) by {
                        assert(component@.len() == 0nat);
                        assert(spec_num_false(visited@) <= n as nat);
                        assert(n as nat <= usize::MAX as nat);
                    };
                    dfs_reach(&transposed, &mut visited, &mut component, vertex);
                    assert(visited.spec_len() == n);
                    if component.size() > 0 {
                        components_vec.push(component);
                    }
                }
                i = i + 1;
            }
            assert(components_vec@.len() < usize::MAX);
            AVLTreeSeqStEphS::from_vec(components_vec)
        }
    } // impl SCCStEphTrait

    } // verus!
}
