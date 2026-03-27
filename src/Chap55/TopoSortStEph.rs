//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Topological Sort - Sequential Ephemeral (Chapter 55, Algorithm 55.13).
//! Sorts DAG vertices in topological order using ephemeral structures.
//! Work: O(|V| + |E|), Span: O(|V| + |E|).

pub mod TopoSortStEph {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap37::AVLTreeSeqStEph::AVLTreeSeqStEph::*;
    use crate::Types::Types::*;

    verus! {

// Veracity: added broadcast group
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
    vstd::seq_lib::group_to_multiset_ensures,
};

    // Table of Contents
    // 1. module
    // 2. imports
    // 4. type definitions
    // 6. spec fns
    // 7. proof fns
    // 8. traits
    // 9. impls

    // 4. type definitions

    pub type T<N> = ArraySeqStEphS<ArraySeqStEphS<N>>;
    pub struct TopoSortStEph;

    // 6. spec fns

    /// Counts false entries in a boolean sequence (termination measure for DFS).
    pub open spec fn spec_num_false(s: Seq<bool>) -> nat
        decreases s.len()
    {
        if s.len() == 0 { 0 }
        else if !s.last() { 1 + spec_num_false(s.drop_last()) }
        else { spec_num_false(s.drop_last()) }
    }

    /// Well-formed adjacency list: all neighbor indices are valid vertex indices.
    pub open spec fn spec_toposortsteph_wf(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> bool {
        forall|v: int, i: int|
            0 <= v < graph@.len() && 0 <= i < graph@[v].len()
            ==> (#[trigger] graph@[v][i]) < graph@.len()
    }

    /// Whether there is a directed edge from u to v in the graph.
    pub open spec fn spec_has_edge(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>, u: int, v: int) -> bool {
        0 <= u < graph@.len()
        && exists|i: int| 0 <= i < graph@[u].len() && (#[trigger] graph@[u][i]) == v
    }

    /// Whether a sequence of vertex indices forms a valid path in the graph.
    pub open spec fn spec_is_path(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>, path: Seq<int>) -> bool {
        path.len() >= 1
        && (forall|k: int| 0 <= k < path.len() ==> 0 <= #[trigger] path[k] < graph@.len())
        && (forall|k: int| #![trigger path[k]] 0 <= k < path.len() - 1 ==> spec_has_edge(graph, path[k], path[k + 1]))
    }

    /// Whether vertex v is reachable from vertex u (Definition 55.3, reachability).
    pub open spec fn spec_reachable(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>, u: int, v: int) -> bool {
        exists|path: Seq<int>| spec_is_path(graph, path) && path[0] == u && #[trigger] path.last() == v
    }

    /// Whether the graph is a directed acyclic graph (Definition 55.11).
    pub open spec fn spec_is_dag(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> bool {
        !exists|path: Seq<int>| spec_is_path(graph, path) && path.len() >= 2 && path[0] == #[trigger] path.last()
    }

    /// Whether a sequence is a valid topological ordering (Definition 55.12).
    pub open spec fn spec_is_topo_order(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>, order: Seq<usize>) -> bool {
        order.len() == graph@.len()
        && order.no_duplicates()
        && (forall|k: int| 0 <= k < order.len() ==> (#[trigger] order[k] as int) < graph@.len())
        && (forall|i: int, j: int| #![trigger order[i], order[j]]
            0 <= i < order.len() && 0 <= j < order.len()
            && spec_has_edge(graph, order[i] as int, order[j] as int)
            ==> i < j)
    }

    /// Whether a set of vertices is strongly connected (Definition 55.14).
    pub open spec fn spec_strongly_connected(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>, vertices: Set<int>) -> bool {
        forall|u: int, v: int| #![trigger vertices.contains(u), vertices.contains(v)]
            vertices.contains(u) && vertices.contains(v)
            ==> spec_reachable(graph, u, v)
    }

    /// Whether vertex v belongs to at least one component.
    pub open spec fn spec_vertex_covered(components: Seq<Set<int>>, v: int) -> bool {
        exists|c: int| 0 <= c < components.len() && (#[trigger] components[c]).contains(v)
    }

    /// Whether components form a valid SCC decomposition in topological order (Definition 55.17).
    pub open spec fn spec_is_scc(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>, components: Seq<Set<int>>) -> bool {
        // Each component is strongly connected.
        (forall|c: int| 0 <= c < components.len()
            ==> #[trigger] spec_strongly_connected(graph, components[c]))
        // Components partition the vertex set.
        && (forall|v: int| 0 <= v < graph@.len() ==>
            #[trigger] spec_vertex_covered(components, v))
        // Components are disjoint.
        && (forall|c1: int, c2: int| #![trigger components[c1], components[c2]]
            0 <= c1 < components.len() && 0 <= c2 < components.len() && c1 != c2
            ==> components[c1].disjoint(components[c2]))
        // Inter-component edges go forward (topological order).
        && (forall|c1: int, c2: int, u: int, v: int| #![trigger components[c1].contains(u), components[c2].contains(v)]
            0 <= c1 < components.len() && 0 <= c2 < components.len()
            && components[c1].contains(u) && components[c2].contains(v)
            && spec_has_edge(graph, u, v) && c1 != c2
            ==> c1 < c2)
    }

    /// Bridge: for ArraySeqStEphS<bool>, view index equals spec_index.
    proof fn lemma_bool_view_eq_spec_index(a: &ArraySeqStEphS<bool>)
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

    /// Bridge: graph adjacency list view at vertex equals spec_index view.
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

    // 7. proof fns

    /// Setting a false entry to true strictly decreases the count of false entries.
    pub proof fn lemma_set_true_decreases_num_false(s: Seq<bool>, idx: int)
        requires
            0 <= idx < s.len(),
            !s[idx],
        ensures
            spec_num_false(s.update(idx, true)) < spec_num_false(s),
        decreases s.len(),
    {
        if s.len() == 1 {
            // s = [false], s.update(0, true) = [true].
            assert(s.drop_last() =~= Seq::<bool>::empty());
            assert(s.update(0, true).drop_last() =~= Seq::<bool>::empty());
            assert(!s.last());
            assert(s.update(0, true).last());
        } else if idx == s.len() - 1 {
            assert(!s.last());
            assert(s.update(idx, true).last());
            assert(s.update(idx, true).drop_last() =~= s.drop_last());
        } else {
            assert(s.update(idx, true).drop_last() =~= s.drop_last().update(idx, true));
            lemma_set_true_decreases_num_false(s.drop_last(), idx);
            assert(!s.last() <==> !s.update(idx, true).last());
        }
    }

    /// Setting a false entry to true decreases the count by exactly one.
    pub proof fn lemma_set_true_num_false_eq(s: Seq<bool>, idx: int)
        requires
            0 <= idx < s.len(),
            !s[idx],
        ensures
            spec_num_false(s.update(idx, true)) == spec_num_false(s) - 1,
        decreases s.len(),
    {
        if s.len() == 1 {
            assert(s.drop_last() =~= Seq::<bool>::empty());
            assert(s.update(0, true).drop_last() =~= Seq::<bool>::empty());
            assert(!s.last());
            assert(s.update(0, true).last());
        } else if idx == s.len() - 1 {
            assert(!s.last());
            assert(s.update(idx, true).last());
            assert(s.update(idx, true).drop_last() =~= s.drop_last());
        } else {
            assert(s.update(idx, true).drop_last() =~= s.drop_last().update(idx, true));
            lemma_set_true_num_false_eq(s.drop_last(), idx);
            assert(!s.last() <==> !s.update(idx, true).last());
        }
    }

    /// An all-true sequence has zero false entries.
    pub proof fn lemma_all_true_num_false_zero(s: Seq<bool>)
        requires forall|j: int| 0 <= j < s.len() ==> #[trigger] s[j],
        ensures spec_num_false(s) == 0,
        decreases s.len(),
    {
        if s.len() > 0 {
            lemma_all_true_num_false_zero(s.drop_last());
        }
    }

    /// An all-false sequence has num_false equal to its length.
    pub proof fn lemma_all_false_num_false_eq_len(s: Seq<bool>)
        requires forall|j: int| #![trigger s[j]] 0 <= j < s.len() ==> !s[j],
        ensures spec_num_false(s) == s.len(),
        decreases s.len(),
    {
        if s.len() > 0 {
            lemma_all_false_num_false_eq_len(s.drop_last());
        }
    }

    // 8. traits

    pub trait TopoSortStEphTrait {
        /// Computes topological sort of a DAG (Algorithm 55.13).
        /// - APAS: Work O(|V| + |E|), Span O(|V| + |E|) [Exercise 55.6]
        /// - Claude-Opus-4.6: Work O(|V| + |E|), Span O(|V| + |E|) — agrees with APAS.
        fn topo_sort(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> (order: AVLTreeSeqStEphS<N>)
            requires
                spec_toposortsteph_wf(graph),
                graph@.len() < usize::MAX,
            ensures
                order@.len() == graph@.len(),
                spec_is_dag(graph) ==> spec_is_topo_order(graph, order@),
            ;
    }

    // 9. impls

    /// Recursive DFS that appends vertices in finish order.
    /// Also used by SCCStEph::compute_finish_order.
    pub fn dfs_finish_order(
        graph: &ArraySeqStEphS<ArraySeqStEphS<N>>,
        visited: &mut ArraySeqStEphS<bool>,
        finish_order: &mut Vec<N>,
        vertex: N,
    )
        requires
            vertex < old(visited)@.len(),
            old(visited)@.len() == graph@.len(),
            spec_toposortsteph_wf(graph),
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
        proof { lemma_bool_view_eq_spec_index(visited); }
        assert(visited.spec_len() == visited@.len());
        if *visited.nth(vertex) {
            return;
        }
        assert(!old(visited)@[vertex as int]);
        assert(vertex < visited.spec_len());
        let set_ok = visited.set(vertex, true);
        assert(set_ok.is_ok());
        proof {
            lemma_set_true_decreases_num_false(old(visited)@, vertex as int);
            lemma_set_true_num_false_eq(old(visited)@, vertex as int);
        }

        // Establish visited@ == old(visited)@.update(vertex, true).
        proof { lemma_bool_view_eq_spec_index(visited); }
        assert forall|j: int| 0 <= j < visited@.len()
            implies #[trigger] visited@[j] == old(visited)@.update(vertex as int, true)[j] by {
            assert(visited@[j] == visited.spec_index(j));
            if j == vertex as int {
                assert(visited.spec_index(j) == true);
            } else {
                assert(visited.spec_index(j) == old(visited).spec_index(j));
            }
        };
        assert(visited@ =~= old(visited)@.update(vertex as int, true));
        assert(spec_num_false(visited@) < spec_num_false(old(visited)@));
        assert(spec_num_false(visited@) == spec_num_false(old(visited)@) - 1);
        assert(visited@.len() == graph@.len());

        // Monotonicity.
        assert forall|j: int| 0 <= j < visited@.len() && #[trigger] old(visited)@[j]
            implies visited@[j] by {};

        // visited@[vertex as int] is true.
        assert(visited@[vertex as int]);

        assert((vertex as int) < graph@.len());
        assert(vertex < graph.spec_len());
        let neighbors = graph.nth(vertex);
        let neighbors_len = neighbors.length();
        assert(neighbors_len as int == neighbors.spec_len());

        // Bridge neighbors to graph view.
        assert(*neighbors == graph.spec_index(vertex as int));
        proof { lemma_graph_view_bridge(graph, neighbors, vertex as int); }
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
            let neighbor = *neighbors.nth(i);
            proof { lemma_usize_view_eq_spec_index(neighbors); }
            assert(neighbor == neighbors@[i as int]);
            assert(neighbor == graph@[vertex as int][i as int]);
            assert(graph@[vertex as int][i as int] < graph@.len());
            assert(neighbor < graph@.len());
            dfs_finish_order(graph, visited, finish_order, neighbor);
            i = i + 1;
        }
        finish_order.push(vertex);
    }

    /// Recursive DFS with cycle detection via rec_stack.
    /// Returns true if no cycle found, false if cycle detected.
    fn dfs_finish_order_cycle_detect(
        graph: &ArraySeqStEphS<ArraySeqStEphS<N>>,
        visited: &mut ArraySeqStEphS<bool>,
        rec_stack: &mut ArraySeqStEphS<bool>,
        finish_order: &mut Vec<N>,
        vertex: N,
    ) -> (cycle_free: bool)
        requires
            vertex < old(visited)@.len(),
            old(visited)@.len() == graph@.len(),
            old(rec_stack)@.len() == graph@.len(),
            spec_toposortsteph_wf(graph),
        ensures
            visited@.len() == graph@.len(),
            rec_stack@.len() == graph@.len(),
            forall|j: int|
                0 <= j < visited@.len() && #[trigger] old(visited)@[j]
                ==> visited@[j],
            spec_num_false(visited@) <= spec_num_false(old(visited)@),
            finish_order@.len() + spec_num_false(visited@) <=
                old(finish_order)@.len() + spec_num_false(old(visited)@),
        decreases spec_num_false(old(visited)@),
    {
        proof { lemma_bool_view_eq_spec_index(visited); }
        assert(visited.spec_len() == visited@.len());
        assert(rec_stack.spec_len() == rec_stack@.len());
        if *rec_stack.nth(vertex) {
            return false;
        }
        if *visited.nth(vertex) {
            return true;
        }

        assert(!old(visited)@[vertex as int]);
        assert(vertex < visited.spec_len());
        assert(vertex < rec_stack.spec_len());
        let ok1 = visited.set(vertex, true);
        assert(ok1.is_ok());
        let ok2 = rec_stack.set(vertex, true);
        assert(ok2.is_ok());
        proof {
            lemma_set_true_decreases_num_false(old(visited)@, vertex as int);
            lemma_set_true_num_false_eq(old(visited)@, vertex as int);
        }

        // Establish visited@ after both sets.
        proof { lemma_bool_view_eq_spec_index(visited); }
        assert(visited.spec_len() == old(visited).spec_len());
        assert(rec_stack.spec_len() == old(rec_stack).spec_len());
        assert forall|j: int| 0 <= j < visited@.len()
            implies #[trigger] visited@[j] == old(visited)@.update(vertex as int, true)[j] by {
            assert(visited@[j] == visited.spec_index(j));
            if j == vertex as int {
                assert(visited.spec_index(j) == true);
            } else {
                assert(visited.spec_index(j) == old(visited).spec_index(j));
            }
        };
        assert(visited@ =~= old(visited)@.update(vertex as int, true));
        assert(spec_num_false(visited@) < spec_num_false(old(visited)@));
        assert(visited@.len() == graph@.len());
        assert(rec_stack@.len() == graph@.len());

        // Monotonicity.
        assert forall|j: int| 0 <= j < visited@.len() && #[trigger] old(visited)@[j]
            implies visited@[j] by {};

        assert((vertex as int) < graph@.len());
        assert(vertex < graph.spec_len());
        let neighbors = graph.nth(vertex);
        let neighbors_len = neighbors.length();
        assert(neighbors_len as int == neighbors.spec_len());

        // Bridge neighbors to graph view.
        assert(*neighbors == graph.spec_index(vertex as int));
        proof { lemma_graph_view_bridge(graph, neighbors, vertex as int); }
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
                vertex < rec_stack.spec_len(),
                visited@.len() == graph@.len(),
                visited.spec_len() == visited@.len(),
                rec_stack@.len() == graph@.len(),
                rec_stack.spec_len() == rec_stack@.len(),
                spec_toposortsteph_wf(graph),
                forall|j: int|
                    0 <= j < visited@.len() && #[trigger] old(visited)@[j]
                    ==> visited@[j],
                spec_num_false(visited@) < spec_num_false(old(visited)@),
                finish_order@.len() + spec_num_false(visited@) <
                    old(finish_order)@.len() + spec_num_false(old(visited)@),
            decreases neighbors_len - i,
        {
            let neighbor = *neighbors.nth(i);
            proof { lemma_usize_view_eq_spec_index(neighbors); }
            assert(neighbor == neighbors@[i as int]);
            assert(neighbor == graph@[vertex as int][i as int]);
            assert(graph@[vertex as int][i as int] < graph@.len());
            assert(neighbor < graph@.len());
            if !dfs_finish_order_cycle_detect(graph, visited, rec_stack, finish_order, neighbor) {
                return false;
            }
            i = i + 1;
        }

        assert(vertex < rec_stack.spec_len());
        let ok3 = rec_stack.set(vertex, false);
        assert(ok3.is_ok());
        assert(rec_stack@.len() == rec_stack.spec_len());
        assert(rec_stack@.len() == graph@.len());
        finish_order.push(vertex);
        true
    }

    /// Returns Some(sequence) if graph is acyclic, None if contains a cycle.
    #[verifier::external_body]
    pub fn topological_sort_opt(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> (topo_order: Option<AVLTreeSeqStEphS<N>>)
        requires
            spec_toposortsteph_wf(graph),
            graph@.len() < usize::MAX,
        ensures
            topo_order.is_some() <==> spec_is_dag(graph),
            topo_order.is_some() ==> spec_is_topo_order(graph, topo_order.unwrap()@),
    {
        let n = graph.length();
        let f_false = |_x: usize| -> (r: bool) ensures !r { false };
        let mut visited = ArraySeqStEphS::tabulate(&f_false, n);
        let mut rec_stack = ArraySeqStEphS::tabulate(&f_false, n);
        let mut finish_order: Vec<N> = Vec::new();

        proof {
            assert forall|j: int| 0 <= j < visited@.len() implies !visited@[j] by {
                // tabulate ensures: f_false.ensures((j as usize,), visited.seq@[j])
                // f_false.ensures(_, r) <==> !r, so visited.seq@[j] == false.
                assert(!visited.seq@[j]);
                assert(visited@[j] == visited.seq@[j]@);
            }
            lemma_all_false_num_false_eq_len(visited@);
        }

        let mut start: usize = 0;
        while start < n
            invariant
                start <= n,
                n == graph@.len(),
                n < usize::MAX,
                visited@.len() == n,
                visited.spec_len() == visited@.len(),
                rec_stack@.len() == n,
                rec_stack.spec_len() == rec_stack@.len(),
                spec_toposortsteph_wf(graph),
                finish_order@.len() + spec_num_false(visited@) <= n,
            decreases n - start,
        {
            assert(start < visited.spec_len());
            if !*visited.nth(start) {
                if !dfs_finish_order_cycle_detect(graph, &mut visited, &mut rec_stack, &mut finish_order, start) {
                    return None;
                }
            }
            start = start + 1;
        }
        assert(finish_order@.len() <= n);
        assert(finish_order@.len() < usize::MAX);
        let result_len = finish_order.len();
        let mut reversed: Vec<N> = Vec::new();
        let mut k: usize = result_len;
        while k > 0
            invariant
                k <= result_len,
                result_len == finish_order@.len(),
                result_len < usize::MAX,
                reversed@.len() == (result_len - k) as nat,
                reversed@.len() < usize::MAX,
            decreases k,
        {
            k = k - 1;
            reversed.push(finish_order[k]);
        }
        assert(reversed@.len() < usize::MAX);
        Some(AVLTreeSeqStEphS::from_vec(reversed))
    }

    impl TopoSortStEphTrait for TopoSortStEph {
        /// Returns sequence of vertices in topological order.
        #[verifier::external_body]
        fn topo_sort(graph: &ArraySeqStEphS<ArraySeqStEphS<N>>) -> (order: AVLTreeSeqStEphS<N>)
        {
            let n = graph.length();
            let f_false = |_x: usize| -> (r: bool) ensures !r { false };
            let mut visited = ArraySeqStEphS::tabulate(&f_false, n);
            let mut finish_order: Vec<N> = Vec::new();

            proof {
                assert forall|j: int| 0 <= j < visited@.len() implies !visited@[j] by {
                    assert(!visited.seq@[j]);
                    assert(visited@[j] == visited.seq@[j]@);
                }
                lemma_all_false_num_false_eq_len(visited@);
            }

            let mut start: usize = 0;
            while start < n
                invariant
                    start <= n,
                    n == graph@.len(),
                    n < usize::MAX,
                    visited@.len() == n,
                    visited.spec_len() == visited@.len(),
                    spec_toposortsteph_wf(graph),
                    forall|k: int| 0 <= k < finish_order@.len()
                        ==> (#[trigger] finish_order@[k] as int) < graph@.len(),
                    finish_order@.len() + spec_num_false(visited@) == n,
                    forall|j: int| #![trigger visited@[j]] 0 <= j < start ==> visited@[j],
                decreases n - start,
            {
                assert(start < visited.spec_len());
                proof { lemma_bool_view_eq_spec_index(&visited); }
                if !*visited.nth(start) {
                    let ghost old_visited = visited@;
                    dfs_finish_order(graph, &mut visited, &mut finish_order, start);
                    // Monotonicity preserves prior visited entries.
                    proof {
                        assert forall|j: int| #![trigger visited@[j]] 0 <= j < start + 1 implies visited@[j] by {
                            if j < start as int {
                                assert(old_visited[j]);
                            }
                        };
                    }
                } else {
                    assert(visited@[start as int]);
                }
                start = start + 1;
            }
            proof {
                lemma_all_true_num_false_zero(visited@);
            }
            assert(finish_order@.len() == n);
            assert(finish_order@.len() < usize::MAX);
            let result_len = finish_order.len();
            let mut reversed: Vec<N> = Vec::new();
            let mut k: usize = result_len;
            while k > 0
                invariant
                    k <= result_len,
                    result_len == finish_order@.len(),
                    result_len < usize::MAX,
                    reversed@.len() == (result_len - k) as nat,
                    reversed@.len() < usize::MAX,
                decreases k,
            {
                k = k - 1;
                reversed.push(finish_order[k]);
            }
            assert(reversed@.len() < usize::MAX);
            AVLTreeSeqStEphS::from_vec(reversed)
        }
    } // impl TopoSortStEphTrait

    } // verus!
}
