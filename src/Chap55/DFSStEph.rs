//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Depth-First Search - Sequential Ephemeral (Chapter 55, Algorithm 55.7).
//! Recursive DFS using ephemeral arrays for efficient visited tracking.
//! Work: O(|V| + |E|), Span: O(|V| + |E|).

pub mod DFSStEph {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap41::AVLTreeSetStEph::AVLTreeSetStEph::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap55::TopoSortStEph::TopoSortStEph::{spec_num_false, spec_toposortsteph_wf, spec_reachable, spec_has_edge, spec_is_path, lemma_set_true_decreases_num_false, lemma_set_true_num_false_eq, lemma_all_false_num_false_eq_len};
    use crate::Types::Types::*;

    verus! {

broadcast use vstd::seq::group_seq_axioms;

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
    pub struct DFSStEph;

    // 6. spec fns

    /// Bridge: for ArraySeqStEphS<bool>, view index equals spec_index.
    proof fn lemma_bool_view_eq_spec_index(a: &ArraySeqStEphS<bool>)
        ensures forall|j: int| 0 <= j < a@.len() ==> #[trigger] a@[j] == a.spec_index(j),
    {
        assert forall|j: int| 0 <= j < a@.len() implies #[trigger] a@[j] == a.spec_index(j) by {}
    }

    /// Bridge: for ArraySeqStEphS<usize>, view index equals spec_index.
    proof fn lemma_usize_view_eq_spec_index(a: &ArraySeqStEphS<usize>)
        ensures forall|j: int| 0 <= j < a@.len() ==> #[trigger] a@[j] == a.spec_index(j),
    {
        assert forall|j: int| 0 <= j < a@.len() implies #[trigger] a@[j] == a.spec_index(j) by {}
    }

    /// Bridge: for graph adjacency list, the view at vertex equals the spec_index view.
    proof fn lemma_graph_view_bridge(
        graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>,
        neighbors: &ArraySeqStEphS<usize>,
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

    /// A vertex is trivially reachable from itself.
    proof fn lemma_reachable_self(graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>, v: int)
        requires 0 <= v < graph@.len(),
        ensures spec_reachable(graph, v, v),
    {
        let path = seq![v];
        assert(path.len() == 1);
        assert(path[0] == v);
        assert(path.last() == v);
        assert forall|k: int| 0 <= k < path.len() implies 0 <= #[trigger] path[k] < graph@.len() by {};
        assert(spec_is_path(graph, path));
    }

    /// If there is an edge u→v and v can reach w, then u can reach w.
    proof fn lemma_reachable_step(
        graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>,
        u: int, v: int, w: int,
    )
        requires
            0 <= u < graph@.len(),
            spec_has_edge(graph, u, v),
            spec_reachable(graph, v, w),
        ensures
            spec_reachable(graph, u, w),
    {
        let path_vw = choose|path: Seq<int>|
            spec_is_path(graph, path) && path[0] == v && #[trigger] path.last() == w;
        let path_uw = seq![u] + path_vw;
        assert(path_uw.len() >= 2);
        assert(path_uw[0] == u);
        assert forall|k: int| 0 <= k < path_uw.len()
            implies 0 <= #[trigger] path_uw[k] < graph@.len() by {
            if k == 0 {
            } else {
                assert(path_uw[k] == path_vw[k - 1]);
            }
        };
        assert forall|k: int| 0 <= k < path_uw.len() - 1
            implies #[trigger] spec_has_edge(graph, path_uw[k], path_uw[k + 1]) by {
            if k == 0 {
                assert(path_uw[0] == u);
                assert(path_uw[1] == path_vw[0]);
                assert(path_vw[0] == v);
            } else {
                assert(path_uw[k] == path_vw[k - 1]);
                assert(path_uw[k + 1] == path_vw[k]);
            }
        };
        assert(spec_is_path(graph, path_uw));
        assert(path_uw.last() == path_vw.last());
        assert(path_uw.last() == w);
    }

    /// If visited is neighbor-closed and path[0] is visited, all vertices on the path are visited.
    proof fn lemma_neighbor_closed_path(
        graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>,
        visited: Seq<bool>,
        path: Seq<int>,
    )
        requires
            spec_toposortsteph_wf(graph),
            visited.len() == graph@.len(),
            spec_is_path(graph, path),
            visited[path[0]],
            forall|u: int, j: int| #![trigger graph@[u][j]]
                0 <= u < graph@.len() && 0 <= j < graph@[u].len()
                && visited[u]
                ==> visited[graph@[u][j] as int],
        ensures
            forall|k: int| 0 <= k < path.len() ==> visited[#[trigger] path[k]],
        decreases path.len(),
    {
        if path.len() > 1 {
            assert(spec_has_edge(graph, path[0], path[1]));
            let edge_idx = choose|i: int|
                0 <= i < graph@[path[0]].len() && (#[trigger] graph@[path[0]][i]) == path[1];
            assert(visited[graph@[path[0]][edge_idx] as int]);
            assert(visited[path[1]]);

            let path_tail = path.subrange(1, path.len() as int);
            assert(path_tail.len() >= 1);
            assert(path_tail[0] == path[1]);
            assert forall|k: int| 0 <= k < path_tail.len()
                implies 0 <= #[trigger] path_tail[k] < graph@.len() by {
                assert(path_tail[k] == path[k + 1]);
            };
            assert forall|k: int| 0 <= k < path_tail.len() - 1
                implies #[trigger] spec_has_edge(graph, path_tail[k], path_tail[k + 1]) by {
                assert(path_tail[k] == path[k + 1]);
                assert(path_tail[k + 1] == path[k + 2]);
            };
            assert(spec_is_path(graph, path_tail));
            lemma_neighbor_closed_path(graph, visited, path_tail);
            assert forall|k: int| 0 <= k < path.len()
                implies visited[#[trigger] path[k]] by {
                if k == 0 {
                } else {
                    assert(path[k] == path_tail[k - 1]);
                }
            };
        }
    }

    /// If visited is neighbor-closed and source is visited, every vertex reachable from source
    /// is visited.
    proof fn lemma_neighbor_closed_implies_reachable(
        graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>,
        visited: Seq<bool>,
        source: int,
        target: int,
    )
        requires
            spec_toposortsteph_wf(graph),
            visited.len() == graph@.len(),
            0 <= source < graph@.len(),
            0 <= target < graph@.len(),
            visited[source],
            spec_reachable(graph, source, target),
            forall|u: int, j: int| #![trigger graph@[u][j]]
                0 <= u < graph@.len() && 0 <= j < graph@[u].len()
                && visited[u] ==> visited[graph@[u][j] as int],
        ensures
            visited[target],
    {
        let path = choose|path: Seq<int>|
            spec_is_path(graph, path) && path[0] == source && #[trigger] path.last() == target;
        lemma_neighbor_closed_path(graph, visited, path);
    }

    // 8. traits

    pub trait DFSStEphTrait {
        /// Performs DFS from source vertex s on adjacency list graph G.
        /// - Alg Analysis: APAS (Ch55 CS 55.8): Work O((m+n) lg n), Span O((m+n) lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((m+n) lg n), Span O((m+n) lg n) — matches APAS tree-based cost
        /// - Alg Analysis: APAS (Ch55 CS 55.8): Work O(m + n), Span O(m + n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((m+n) lg n), Span O((m+n) lg n) — matches APAS tree-based; uses AVL set for visited, adj seq for graph
        fn dfs(graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>, source: usize) -> (reachable: AVLTreeSetStEph<usize>)
            requires
                source < graph@.len(),
                spec_toposortsteph_wf(graph),
                graph@.len() < usize::MAX,
            ensures
                reachable@.contains(source),
                forall|v: usize| reachable@.contains(v) ==> (v as int) < graph@.len(),
                forall|v: int| 0 <= v < graph@.len()
                    ==> (reachable@.contains(v as usize) <==> #[trigger] spec_reachable(graph, source as int, v)),
            ;
    }

    // 9. impls

    /// Recursive DFS helper that marks visited vertices and inserts them into the result set.
    /// Ghost parameter `gray` tracks vertices on the recursion stack (visited but not yet
    /// fully processed). Neighbor-closure and visited-reachable are guaranteed for all
    /// visited vertices NOT in gray.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|+|E|), Span O(|V|+|E|) — visits each vertex and edge once; St sequential.
    fn dfs_recursive(
        graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>,
        visited: &mut ArraySeqStEphS<bool>,
        reachable: &mut AVLTreeSetStEph<usize>,
        vertex: usize,
        Ghost(gray): Ghost<Set<int>>,
    )
        requires
            vertex < old(visited)@.len(),
            old(visited)@.len() == graph@.len(),
            spec_toposortsteph_wf(graph),
            old(reachable).spec_avltreesetsteph_wf(),
            forall|v: usize| old(reachable)@.contains(v) ==> (v as int) < graph@.len(),
            graph@.len() < usize::MAX,
            old(reachable)@.len() + spec_num_false(old(visited)@) <= graph@.len(),
            // Gray set: all gray vertices are valid and visited.
            !gray.contains(vertex as int),
            forall|u: int| (#[trigger] gray.contains(u)) ==> 0 <= u < graph@.len() && old(visited)@[u],
            // Neighbor-closure except gray.
            forall|u: int, j: int| #![trigger graph@[u][j]]
                0 <= u < graph@.len() && 0 <= j < graph@[u].len()
                && old(visited)@[u] && !gray.contains(u)
                ==> old(visited)@[graph@[u][j] as int],
            // Visited-reachable correspondence except gray.
            forall|v: int| 0 <= v < old(visited)@.len() && (#[trigger] old(visited)@[v]) && !gray.contains(v)
                ==> old(reachable)@.contains(v as usize),
        ensures
            visited@.len() == old(visited)@.len(),
            forall|j: int|
                0 <= j < visited@.len() && #[trigger] old(visited)@[j]
                ==> visited@[j],
            spec_num_false(visited@) <= spec_num_false(old(visited)@),
            reachable.spec_avltreesetsteph_wf(),
            forall|v: usize| old(reachable)@.contains(v) ==> (#[trigger] reachable@.contains(v)),
            forall|v: usize| reachable@.contains(v) ==> (v as int) < graph@.len(),
            reachable@.len() + spec_num_false(visited@) <= graph@.len(),
            // vertex is visited and in reachable.
            visited@[vertex as int],
            reachable@.contains(vertex),
            // Neighbor-closure except gray (vertex now included since it is not in gray).
            forall|u: int, j: int| #![trigger graph@[u][j]]
                0 <= u < graph@.len() && 0 <= j < graph@[u].len()
                && visited@[u] && !gray.contains(u)
                ==> visited@[graph@[u][j] as int],
            // Visited-reachable correspondence except gray.
            forall|v: int| 0 <= v < visited@.len() && (#[trigger] visited@[v]) && !gray.contains(v)
                ==> reachable@.contains(v as usize),
            // Soundness: new reachable entries are graph-reachable from vertex.
            forall|v: usize| (#[trigger] reachable@.contains(v)) ==>
                old(reachable)@.contains(v) || spec_reachable(graph, vertex as int, v as int),
        decreases spec_num_false(old(visited)@),
    {
        // Bridge: visited@[j] == visited.spec_index(j) for bool arrays.
        proof { lemma_bool_view_eq_spec_index(visited); }
        assert(visited.spec_len() == visited@.len());

        if *visited.nth(vertex) {
            // Early return: vertex already visited. Nothing changes.
            proof {
                // vertex is visited, not in gray → reachable@.contains(vertex).
                assert(old(visited)@[vertex as int]);
                assert(!gray.contains(vertex as int));
                assert(old(reachable)@.contains(vertex as usize));
                // Establish each postcondition explicitly to avoid conjunction flakiness.
                assert(visited@[vertex as int]);
                assert(reachable@.contains(vertex));
            }
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

        // Re-establish bridge for the new visited state.
        proof { lemma_bool_view_eq_spec_index(visited); }
        assert(visited@.len() == old(visited)@.len());

        // Establish visited@ == old(visited)@.update(vertex, true).
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

        // Soundness: vertex is reachable from vertex.
        proof { lemma_reachable_self(graph, vertex as int); }

        let ghost old_reachable_snap = reachable@;
        reachable.insert(vertex);
        assert(reachable.spec_avltreesetsteph_wf());
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

        let ghost gray_inner = gray.insert(vertex as int);

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
                forall|v: usize| old(reachable)@.contains(v) ==> (#[trigger] reachable@.contains(v)),
                forall|v: usize| reachable@.contains(v) ==> (v as int) < graph@.len(),
                reachable@.len() + spec_num_false(visited@) <= graph@.len(),
                // vertex is visited and in reachable.
                visited@[vertex as int],
                reachable@.contains(vertex),
                // Gray invariant.
                gray_inner == gray.insert(vertex as int),
                !gray.contains(vertex as int),
                forall|u: int| (#[trigger] gray.contains(u)) ==> 0 <= u < graph@.len() && visited@[u],
                // Neighbor-closure except gray_inner.
                forall|u: int, j: int| #![trigger graph@[u][j]]
                    0 <= u < graph@.len() && 0 <= j < graph@[u].len()
                    && visited@[u] && !gray_inner.contains(u)
                    ==> visited@[graph@[u][j] as int],
                // Visited-reachable except gray_inner.
                forall|v: int| 0 <= v < visited@.len() && (#[trigger] visited@[v]) && !gray_inner.contains(v)
                    ==> reachable@.contains(v as usize),
                // Soundness.
                forall|v: usize| (#[trigger] reachable@.contains(v)) ==>
                    old(reachable)@.contains(v) || spec_reachable(graph, vertex as int, v as int),
                // All neighbors 0..i are visited.
                forall|j: int| 0 <= j < i ==> visited@[(#[trigger] graph@[vertex as int][j]) as int],
            decreases neighbors_len - i,
        {
            let neighbor = *neighbors.nth(i);
            proof { lemma_usize_view_eq_spec_index(neighbors); }
            assert(neighbor == neighbors@[i as int]);
            assert(neighbor == graph@[vertex as int][i as int]);
            assert(graph@[vertex as int][i as int] < graph@.len());
            assert(neighbor < graph@.len());

            proof { lemma_bool_view_eq_spec_index(visited); }
            if !*visited.nth(neighbor) {
                // neighbor is unvisited => not in gray (gray ⊆ old(visited) ⊆ visited).
                // Also neighbor != vertex (vertex is visited, neighbor is not).
                assert(!gray_inner.contains(neighbor as int));

                let ghost reachable_before = reachable@;
                dfs_recursive(graph, visited, reachable, neighbor, Ghost(gray_inner));

                // Chain soundness: new entries reachable from neighbor, hence from vertex.
                proof {
                    assert(spec_has_edge(graph, vertex as int, neighbor as int));
                    assert forall|v: usize| (#[trigger] reachable@.contains(v)) implies
                        old(reachable)@.contains(v) || spec_reachable(graph, vertex as int, v as int) by {
                        if !reachable_before.contains(v) {
                            // Added during this recursive call: reachable from neighbor.
                            assert(spec_reachable(graph, neighbor as int, v as int));
                            lemma_reachable_step(graph, vertex as int, neighbor as int, v as int);
                        }
                    };
                }
            }
            i = i + 1;
        }
        // After loop: all of vertex's neighbors are visited.
        // vertex is not in gray, so vertex is now neighbor-closed.
        // The postcondition neighbor-closure with gray (not gray_inner) follows.
        proof {
            assert forall|u: int, j: int| #![trigger graph@[u][j]]
                0 <= u < graph@.len() && 0 <= j < graph@[u].len()
                && visited@[u] && !gray.contains(u)
                implies visited@[graph@[u][j] as int] by {
                if u == vertex as int {
                    // vertex: all neighbors visited (from loop).
                    assert(visited@[graph@[vertex as int][j] as int]);
                } else {
                    // non-vertex, non-gray: already covered by loop invariant with gray_inner.
                    assert(!gray_inner.contains(u));
                }
            };
            assert forall|v: int| 0 <= v < visited@.len() && (#[trigger] visited@[v]) && !gray.contains(v)
                implies reachable@.contains(v as usize) by {
                if v == vertex as int {
                    assert(reachable@.contains(vertex));
                } else {
                    assert(!gray_inner.contains(v));
                }
            };
        }
    }

    impl DFSStEphTrait for DFSStEph {
        /// Performs DFS from source vertex s on adjacency list graph G.
        /// Returns the set of all vertices reachable from s.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|V|+|E|), Span O(|V|+|E|) — initializes visited + delegates to dfs_recursive; St sequential.
        fn dfs(graph: &ArraySeqStEphS<ArraySeqStEphS<usize>>, source: usize) -> (reachable: AVLTreeSetStEph<usize>)
        {
            let n = graph.length();
            let init_false = |_x: usize| -> (r: bool)
                ensures !r
            { false };
            let mut visited = ArraySeqStEphS::tabulate(&init_false, n);
            let mut reachable = AVLTreeSetStEph::empty();

            proof {
                // Tabulate ensures: init_false.ensures((j as usize,), visited.seq@[j]) for j in range.
                // The closure ensures !r, so !visited.seq@[j] for each j.
                assert forall|j: int| 0 <= j < visited@.len() implies !(#[trigger] visited@[j]) by {
                    assert(init_false.ensures((j as usize,), visited.seq@[j]));
                    assert(!visited.seq@[j]);
                    assert(visited@[j] == visited.seq@[j]@);
                };
                lemma_all_false_num_false_eq_len(visited@);
            }

            dfs_recursive(graph, &mut visited, &mut reachable, source, Ghost(Set::empty()));

            proof {
                // After the call with gray = empty:
                // Neighbor-closure holds for ALL visited vertices.
                // visited[source] is true.
                // Soundness: everything in reachable is reachable from source
                //   (old(reachable) was empty).
                // Completeness: everything reachable from source is visited (by lemma),
                //   and all visited vertices are in reachable (visited-reachable with empty gray).
                assert forall|v: int| 0 <= v < graph@.len()
                    && spec_reachable(graph, source as int, v)
                    implies reachable@.contains(v as usize) by {
                    lemma_neighbor_closed_implies_reachable(graph, visited@, source as int, v);
                    assert(visited@[v]);
                    assert(!Set::<int>::empty().contains(v));
                };
                assert forall|v: int| 0 <= v < graph@.len()
                    implies (reachable@.contains(v as usize) <==>
                        #[trigger] spec_reachable(graph, source as int, v)) by {
                    if reachable@.contains(v as usize) {
                        assert(spec_reachable(graph, source as int, v as int));
                    }
                    if spec_reachable(graph, source as int, v) {
                        assert(reachable@.contains(v as usize));
                    }
                };
                // source is reachable from source.
                lemma_reachable_self(graph, source as int);
            }
            reachable
        }
    } // impl DFSStEphTrait

    } // verus!

    // 14. derive impls outside verus!

    impl std::fmt::Debug for DFSStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "DFSStEph")
        }
    }

    impl std::fmt::Display for DFSStEph {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "DFSStEph")
        }
    }
}
