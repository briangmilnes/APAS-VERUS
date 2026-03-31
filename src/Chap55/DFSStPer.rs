//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Depth-First Search - Sequential Persistent (Chapter 55, Algorithm 55.2).
//! Recursive DFS for finding reachable vertices from a source vertex.
//! Work: O(|V| + |E|), Span: O(|V| + |E|).

pub mod DFSStPer {

    use vstd::prelude::*;
    use crate::Chap19::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap41::AVLTreeSetStPer::AVLTreeSetStPer::*;
    #[cfg(verus_keep_ghost)]
    use crate::Chap55::TopoSortStEph::TopoSortStEph::{spec_num_false, lemma_set_true_decreases_num_false, lemma_set_true_num_false_eq, lemma_all_false_num_false_eq_len};
    #[cfg(verus_keep_ghost)]
    use crate::Chap55::TopoSortStPer::TopoSortStPer::{spec_toposortstper_wf, spec_reachable_per, spec_has_edge_per, spec_is_path_per};
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

    pub type T<N> = ArraySeqStPerS<ArraySeqStPerS<N>>;
    pub struct DFSStPer;

    // 6. spec fns

    /// Bridge: for ArraySeqStPerS<usize>, view index equals spec_index.
    proof fn lemma_usize_per_view_eq_spec_index(a: &ArraySeqStPerS<usize>)
        ensures forall|j: int| 0 <= j < a@.len() ==> #[trigger] a@[j] == a.spec_index(j),
    {
        assert forall|j: int| 0 <= j < a@.len() implies #[trigger] a@[j] == a.spec_index(j) by {}
    }

    /// Bridge: for persistent graph adjacency list, the view at vertex equals the spec_index view.
    proof fn lemma_graph_per_view_bridge(
        graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>,
        neighbors: &ArraySeqStPerS<usize>,
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

    /// A vertex is trivially reachable from itself (persistent graph variant).
    proof fn lemma_reachable_self_per(graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>, v: int)
        requires 0 <= v < graph@.len(),
        ensures spec_reachable_per(graph, v, v),
    {
        let path = seq![v];
        assert(path.len() == 1);
        assert(path[0] == v);
        assert(path.last() == v);
        assert forall|k: int| 0 <= k < path.len() implies 0 <= #[trigger] path[k] < graph@.len() by {};
        assert(spec_is_path_per(graph, path));
    }

    /// If there is an edge u→v and v can reach w, then u can reach w (persistent variant).
    proof fn lemma_reachable_step_per(
        graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>,
        u: int, v: int, w: int,
    )
        requires
            0 <= u < graph@.len(),
            spec_has_edge_per(graph, u, v),
            spec_reachable_per(graph, v, w),
        ensures
            spec_reachable_per(graph, u, w),
    {
        let path_vw = choose|path: Seq<int>|
            spec_is_path_per(graph, path) && path[0] == v && #[trigger] path.last() == w;
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
            implies spec_has_edge_per(graph, #[trigger] path_uw[k], path_uw[k + 1]) by {
            if k == 0 {
                assert(path_uw[0] == u);
                assert(path_uw[1] == path_vw[0]);
                assert(path_vw[0] == v);
            } else {
                assert(path_uw[k] == path_vw[k - 1]);
                assert(path_uw[k + 1] == path_vw[k]);
            }
        };
        assert(spec_is_path_per(graph, path_uw));
        assert(path_uw.last() == path_vw.last());
        assert(path_uw.last() == w);
    }

    /// If visited is neighbor-closed and path[0] is visited, all vertices on the path are visited
    /// (persistent variant).
    proof fn lemma_neighbor_closed_path_per(
        graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>,
        visited: Seq<bool>,
        path: Seq<int>,
    )
        requires
            spec_toposortstper_wf(graph),
            visited.len() == graph@.len(),
            spec_is_path_per(graph, path),
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
            assert(spec_has_edge_per(graph, path[0], path[1]));
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
                implies spec_has_edge_per(graph, #[trigger] path_tail[k], path_tail[k + 1]) by {
                assert(path_tail[k] == path[k + 1]);
                assert(path_tail[k + 1] == path[k + 2]);
            };
            assert(spec_is_path_per(graph, path_tail));
            lemma_neighbor_closed_path_per(graph, visited, path_tail);
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
    /// is visited (persistent variant).
    proof fn lemma_neighbor_closed_implies_reachable_per(
        graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>,
        visited: Seq<bool>,
        source: int,
        target: int,
    )
        requires
            spec_toposortstper_wf(graph),
            visited.len() == graph@.len(),
            0 <= source < graph@.len(),
            0 <= target < graph@.len(),
            visited[source],
            spec_reachable_per(graph, source, target),
            forall|u: int, j: int| #![trigger graph@[u][j]]
                0 <= u < graph@.len() && 0 <= j < graph@[u].len()
                && visited[u] ==> visited[graph@[u][j] as int],
        ensures
            visited[target],
    {
        let path = choose|path: Seq<int>|
            spec_is_path_per(graph, path) && path[0] == source && #[trigger] path.last() == target;
        lemma_neighbor_closed_path_per(graph, visited, path);
    }

    // 8. traits

    pub trait DFSStPerTrait {
        /// Performs DFS from source vertex s on adjacency list graph G
        /// APAS: Work O(|V| + |E|), Span O(|V| + |E|)
        /// - Alg Analysis: APAS (Ch55 CS 55.8): Work O((m+n) lg n), Span O((m+n) lg n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((m+n) — matches APAS
        /// - Alg Analysis: APAS (Ch55 CS 55.8): Work O(m + n), Span O(m + n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O((m+n) lg n), Span O((m+n) lg n) — matches APAS tree-based; uses AVL set for visited, adj seq for graph
        fn dfs(graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>, source: usize) -> (reachable: AVLTreeSetStPer<usize>)
            requires
                source < graph@.len(),
                spec_toposortstper_wf(graph),
                graph@.len() < usize::MAX,
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
    /// Ghost parameter `gray` tracks vertices on the recursion stack.
    fn dfs_recursive(
        graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>,
        visited_bool: &mut Vec<bool>,
        reachable: AVLTreeSetStPer<usize>,
        vertex: usize,
        Ghost(gray): Ghost<Set<int>>,
    ) -> (out: AVLTreeSetStPer<usize>)
        requires
            vertex < old(visited_bool)@.len(),
            old(visited_bool)@.len() == graph@.len(),
            spec_toposortstper_wf(graph),
            reachable.spec_avltreesetstper_wf(),
            forall|v: usize| reachable@.contains(v) ==> (v as int) < graph@.len(),
            graph@.len() < usize::MAX,
            reachable@.len() + spec_num_false(old(visited_bool)@) <= graph@.len(),
            // Gray set: all gray vertices are valid and visited.
            !gray.contains(vertex as int),
            forall|u: int| (#[trigger] gray.contains(u)) ==> 0 <= u < graph@.len() && old(visited_bool)@[u],
            // Neighbor-closure except gray.
            forall|u: int, j: int| #![trigger graph@[u][j]]
                0 <= u < graph@.len() && 0 <= j < graph@[u].len()
                && old(visited_bool)@[u] && !gray.contains(u)
                ==> old(visited_bool)@[graph@[u][j] as int],
            // Visited-reachable correspondence except gray.
            forall|v: int| 0 <= v < old(visited_bool)@.len() && (#[trigger] old(visited_bool)@[v]) && !gray.contains(v)
                ==> reachable@.contains(v as usize),
        ensures
            visited_bool@.len() == old(visited_bool)@.len(),
            forall|j: int|
                0 <= j < visited_bool@.len() && #[trigger] old(visited_bool)@[j]
                ==> visited_bool@[j],
            spec_num_false(visited_bool@) <= spec_num_false(old(visited_bool)@),
            out.spec_avltreesetstper_wf(),
            forall|v: usize| reachable@.contains(v) ==> (#[trigger] out@.contains(v)),
            forall|v: usize| out@.contains(v) ==> (v as int) < graph@.len(),
            out@.len() + spec_num_false(visited_bool@) <= graph@.len(),
            // vertex is visited and in out.
            visited_bool@[vertex as int],
            out@.contains(vertex),
            // Neighbor-closure except gray.
            forall|u: int, j: int| #![trigger graph@[u][j]]
                0 <= u < graph@.len() && 0 <= j < graph@[u].len()
                && visited_bool@[u] && !gray.contains(u)
                ==> visited_bool@[graph@[u][j] as int],
            // Visited-reachable correspondence except gray.
            forall|v: int| 0 <= v < visited_bool@.len() && (#[trigger] visited_bool@[v]) && !gray.contains(v)
                ==> out@.contains(v as usize),
            // Soundness: new entries are graph-reachable from vertex.
            forall|v: usize| (#[trigger] out@.contains(v)) ==>
                reachable@.contains(v) || spec_reachable_per(graph, vertex as int, v as int),
        decreases spec_num_false(old(visited_bool)@),
    {
        if visited_bool[vertex] {
            proof {
                assert(old(visited_bool)@[vertex as int]);
                assert(!gray.contains(vertex as int));
                assert(reachable@.contains(vertex as usize));
            }
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

        // Soundness: vertex is reachable from vertex.
        proof { lemma_reachable_self_per(graph, vertex as int); }

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

        let ghost gray_inner = gray.insert(vertex as int);
        let ghost old_reachable_snap = reachable@;

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
                forall|v: usize| old_reachable_snap.contains(v) ==> (#[trigger] reachable@.contains(v)),
                forall|v: usize| reachable@.contains(v) ==> (v as int) < graph@.len(),
                reachable@.len() + spec_num_false(visited_bool@) <= graph@.len(),
                // vertex is visited and in reachable.
                visited_bool@[vertex as int],
                reachable@.contains(vertex),
                // Gray invariant.
                gray_inner == gray.insert(vertex as int),
                !gray.contains(vertex as int),
                forall|u: int| (#[trigger] gray.contains(u)) ==> 0 <= u < graph@.len() && visited_bool@[u],
                // Neighbor-closure except gray_inner.
                forall|u: int, j: int| #![trigger graph@[u][j]]
                    0 <= u < graph@.len() && 0 <= j < graph@[u].len()
                    && visited_bool@[u] && !gray_inner.contains(u)
                    ==> visited_bool@[graph@[u][j] as int],
                // Visited-reachable except gray_inner.
                forall|v: int| 0 <= v < visited_bool@.len() && (#[trigger] visited_bool@[v]) && !gray_inner.contains(v)
                    ==> reachable@.contains(v as usize),
                // Soundness.
                forall|v: usize| (#[trigger] reachable@.contains(v)) ==>
                    old_reachable_snap.contains(v) || spec_reachable_per(graph, vertex as int, v as int),
                // All neighbors 0..i are visited.
                forall|j: int| 0 <= j < i ==> visited_bool@[(#[trigger] graph@[vertex as int][j]) as int],
            decreases neighbors_len - i,
        {
            let neighbor = *neighbors.nth(i);
            proof { lemma_usize_per_view_eq_spec_index(neighbors); }
            assert(neighbor == neighbors@[i as int]);
            assert(neighbor == graph@[vertex as int][i as int]);
            assert(graph@[vertex as int][i as int] < graph@.len());
            assert(neighbor < graph@.len());

            if !visited_bool[neighbor] {
                // neighbor is unvisited => not in gray (gray ⊆ old(visited_bool) ⊆ visited_bool).
                assert(!gray_inner.contains(neighbor as int));

                let ghost reachable_before = reachable@;
                reachable = dfs_recursive(graph, visited_bool, reachable, neighbor, Ghost(gray_inner));

                // Chain soundness: new entries reachable from neighbor, hence from vertex.
                proof {
                    assert(spec_has_edge_per(graph, vertex as int, neighbor as int));
                    assert forall|v: usize| (#[trigger] reachable@.contains(v)) implies
                        old_reachable_snap.contains(v) || spec_reachable_per(graph, vertex as int, v as int) by {
                        if !reachable_before.contains(v) {
                            assert(spec_reachable_per(graph, neighbor as int, v as int));
                            lemma_reachable_step_per(graph, vertex as int, neighbor as int, v as int);
                        }
                    };
                }
            }
            i = i + 1;
        }
        // After loop: all of vertex's neighbors are visited.
        proof {
            assert forall|u: int, j: int| #![trigger graph@[u][j]]
                0 <= u < graph@.len() && 0 <= j < graph@[u].len()
                && visited_bool@[u] && !gray.contains(u)
                implies visited_bool@[graph@[u][j] as int] by {
                if u == vertex as int {
                    assert(visited_bool@[graph@[vertex as int][j] as int]);
                } else {
                    assert(!gray_inner.contains(u));
                }
            };
            assert forall|v: int| 0 <= v < visited_bool@.len() && (#[trigger] visited_bool@[v]) && !gray.contains(v)
                implies reachable@.contains(v as usize) by {
                if v == vertex as int {
                    assert(reachable@.contains(vertex));
                } else {
                    assert(!gray_inner.contains(v));
                }
            };
        }
        reachable
    }

    impl DFSStPerTrait for DFSStPer {
        /// Performs DFS from source vertex s on adjacency list graph G.
        /// Returns the set of all vertices reachable from s.
        fn dfs(graph: &ArraySeqStPerS<ArraySeqStPerS<usize>>, source: usize) -> (reachable: AVLTreeSetStPer<usize>)
        {
            let n = graph.length();
            let mut visited_bool: Vec<bool> = Vec::new();
            let mut j: usize = 0;
            while j < n
                invariant
                    j <= n,
                    visited_bool@.len() == j as int,
                    forall|k: int| 0 <= k < j as int ==> !(#[trigger] visited_bool@[k]),
                decreases n - j,
            {
                visited_bool.push(false);
                j = j + 1;
            }

            proof {
                lemma_all_false_num_false_eq_len(visited_bool@);
            }

            let reachable = AVLTreeSetStPer::empty();
            let reachable = dfs_recursive(graph, &mut visited_bool, reachable, source, Ghost(Set::empty()));

            proof {
                assert forall|v: int| 0 <= v < graph@.len()
                    && spec_reachable_per(graph, source as int, v)
                    implies reachable@.contains(v as usize) by {
                    lemma_neighbor_closed_implies_reachable_per(graph, visited_bool@, source as int, v);
                    assert(visited_bool@[v]);
                    assert(!Set::<int>::empty().contains(v));
                };
                assert forall|v: int| 0 <= v < graph@.len()
                    implies (reachable@.contains(v as usize) <==>
                        #[trigger] spec_reachable_per(graph, source as int, v)) by {
                    if reachable@.contains(v as usize) {
                        assert(spec_reachable_per(graph, source as int, v as int));
                    }
                    if spec_reachable_per(graph, source as int, v) {
                        assert(reachable@.contains(v as usize));
                    }
                };
                lemma_reachable_self_per(graph, source as int);
            }
            reachable
        }
    } // impl DFSStPerTrait

    } // verus!
}
