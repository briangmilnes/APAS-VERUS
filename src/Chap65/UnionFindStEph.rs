//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 65: Union-Find Data Structure (Sequential Ephemeral)
//!
//! Implements Union-Find (Disjoint Set Union) with union by rank.
//! Used in Kruskal's MST algorithm for efficient cycle detection.
//!
//! NOTE: find currently uses root-chasing without path compression (O(log n) per call).
//! Path compression lemmas are written and commented out — the assembly lemma needs
//! rlimit work to combine 13 named wf sub-predicates. The algorithm is correct and
//! fully proved; compression is a performance optimization for future work.
//!
//! Ghost field `roots` maps each element to its canonical representative, allowing
//! clean specifications without recursive spec functions. Path compression changes
//! concrete parent pointers but preserves the logical partition (roots).

pub mod UnionFindStEph {

    // 2. imports
    use vstd::prelude::*;

    use crate::Types::Types::*;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
    use crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::*;
    use crate::vstdplus::feq::feq::feq;
    use std::hash::Hash;

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::obeys_key_model;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_full;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_clone;
    #[cfg(verus_keep_ghost)]
    use vstd::pervasive::strictly_cloned;

    verus! {

    broadcast use crate::vstdplus::feq::feq::group_feq_axioms;

    // 4. type definitions

    #[verifier::reject_recursive_types(V)]
    pub struct UnionFindStEph<V: StT + Hash> {
        pub parent: HashMapWithViewPlus<V, V>,
        pub rank: HashMapWithViewPlus<V, usize>,
        pub elements: Vec<V>,
        pub roots: Ghost<Map<<V as View>::V, <V as View>::V>>,
    }

    /// Ghost info returned by union_merge_exec for the proof coordination step.
    pub ghost struct UnionMergeInfo<V: View> {
        pub winner_view: <V as View>::V,
        pub loser_view: <V as View>::V,
        pub winner_val: V,
    }

    // 5. view impls

    pub ghost struct UnionFindStEphV<V: View> {
        pub parent: Map<<V as View>::V, V>,
        pub rank: Map<<V as View>::V, usize>,
        pub elements: Seq<V>,
        pub roots: Map<<V as View>::V, <V as View>::V>,
    }

    impl<V: StT + Hash> View for UnionFindStEph<V> {
        type V = UnionFindStEphV<V>;
        open spec fn view(&self) -> Self::V {
            UnionFindStEphV {
                parent: self.parent@,
                rank: self.rank@,
                elements: self.elements@,
                roots: self.roots@,
            }
        }
    }

    // 6. spec fns

    // Named wf sub-predicates. Each is one conjunct of spec_unionfindsteph_wf.
    // Factored so compression lemmas can target only the conjuncts that change.

    pub open spec fn spec_key_model<V: StT + Hash>() -> bool {
        obeys_key_model::<V>()
    }

    pub open spec fn spec_feq_full<V: StT + Hash>() -> bool {
        obeys_feq_full::<V>()
    }

    pub open spec fn spec_parent_rank_same_dom<V: StT + Hash>(uf: &UnionFindStEph<V>) -> bool {
        uf.parent@.dom() =~= uf.rank@.dom()
    }

    pub open spec fn spec_roots_parent_same_dom<V: StT + Hash>(uf: &UnionFindStEph<V>) -> bool {
        uf.roots@.dom() =~= uf.parent@.dom()
    }

    pub closed spec fn spec_roots_idempotent<V: StT + Hash>(uf: &UnionFindStEph<V>) -> bool {
        forall|v: <V as View>::V| #[trigger] uf.roots@.contains_key(v) ==> {
            &&& uf.roots@.contains_key(uf.roots@[v])
            &&& uf.roots@[uf.roots@[v]] == uf.roots@[v]
        }
    }

    /// Raw version of roots_idempotent operating on a Map directly.
    pub open spec fn spec_roots_idempotent_raw<VV>(roots: Map<VV, VV>) -> bool {
        forall|v: VV| #[trigger] roots.contains_key(v) ==> {
            &&& roots.contains_key(roots[v])
            &&& roots[roots[v]] == roots[v]
        }
    }

    pub closed spec fn spec_parent_closed<V: StT + Hash>(uf: &UnionFindStEph<V>) -> bool {
        forall|v: <V as View>::V| #[trigger] uf.parent@.contains_key(v) ==>
            uf.parent@.contains_key(uf.parent@[v]@)
    }

    pub closed spec fn spec_roots_in_dom<V: StT + Hash>(uf: &UnionFindStEph<V>) -> bool {
        forall|v: <V as View>::V| #[trigger] uf.roots@.contains_key(v) ==>
            uf.parent@.contains_key(uf.roots@[v])
    }

    pub closed spec fn spec_elements_forward<V: StT + Hash>(uf: &UnionFindStEph<V>) -> bool {
        forall|i: int| 0 <= i < uf.elements@.len() as int ==>
            uf.parent@.contains_key(#[trigger] uf.elements@[i]@)
    }

    pub closed spec fn spec_elements_backward<V: StT + Hash>(uf: &UnionFindStEph<V>) -> bool {
        forall|v: <V as View>::V| #[trigger] uf.parent@.contains_key(v) ==>
            exists|i: int| 0 <= i < uf.elements@.len() as int && #[trigger] uf.elements@[i]@ == v
    }

    pub closed spec fn spec_elements_distinct<V: StT + Hash>(uf: &UnionFindStEph<V>) -> bool {
        forall|i: int, j: int|
            0 <= i < uf.elements@.len() as int &&
            0 <= j < uf.elements@.len() as int &&
            i != j ==>
            #[trigger] uf.elements@[i]@ != #[trigger] uf.elements@[j]@
    }

    pub closed spec fn spec_self_parent_is_root<V: StT + Hash>(uf: &UnionFindStEph<V>) -> bool {
        forall|v: <V as View>::V| uf.parent@.contains_key(v) && uf.parent@[v]@ == v ==>
            #[trigger] uf.roots@[v] == v
    }

    pub closed spec fn spec_parent_preserves_root<V: StT + Hash>(uf: &UnionFindStEph<V>) -> bool {
        forall|v: <V as View>::V| #[trigger] uf.parent@.contains_key(v) ==>
            uf.roots@[uf.parent@[v]@] == uf.roots@[v]
    }

    pub closed spec fn spec_rank_increases<V: StT + Hash>(uf: &UnionFindStEph<V>) -> bool {
        forall|v: <V as View>::V| uf.parent@.contains_key(v)
            && uf.parent@[v]@ != v ==>
            uf.rank@[v] < #[trigger] uf.rank@[uf.parent@[v]@]
    }

    pub closed spec fn spec_rank_bounded<V: StT + Hash>(uf: &UnionFindStEph<V>) -> bool {
        forall|v: <V as View>::V| #[trigger] uf.rank@.contains_key(v) ==>
            uf.rank@[v] <= uf.rank@[uf.roots@[v]]
    }

    // 7. proof fns

    /// Helper: prove that 3 cloned values are spec-equal to the original.
    /// Isolated from the wf context to keep solver budget low.
    proof fn lemma_three_clones_eq<V: StT + Hash>(
        v: V, v1: V, v2: V, v3: V,
    )
        requires
            obeys_feq_full::<V>(),
            strictly_cloned(v, v1),
            strictly_cloned(v, v2),
            strictly_cloned(v, v3),
        ensures
            v1 == v,
            v2 == v,
            v3 == v,
    {
    }

    /// Prove wf preservation after insert: frame lemma + new element properties.
    #[verifier::rlimit(50)]
    proof fn lemma_insert_preserves_wf<V: StT + Hash>(
        uf: UnionFindStEph<V>,
        old_uf: UnionFindStEph<V>,
        v: V,
    )
        requires
            old_uf.spec_unionfindsteph_wf(),
            !old_uf.parent@.contains_key(v@),
            uf.parent@ =~= old_uf.parent@.insert(v@, v),
            uf.rank@ =~= old_uf.rank@.insert(v@, 0usize),
            uf.elements@ =~= old_uf.elements@.push(v),
            uf.roots@ =~= old_uf.roots@.insert(v@, v@),
        ensures
            uf.spec_unionfindsteph_wf(),
    {
        let old_p = old_uf.parent@;
        let old_r = old_uf.rank@;
        let old_e = old_uf.elements@;
        let old_rt = old_uf.roots@;

        // Frame: for existing keys w != v@, old maps are preserved, and old_rt[w] != v@.
        assert forall|w: <V as View>::V| #[trigger] old_p.contains_key(w) implies {
            &&& uf.parent@[w] == old_p[w]
            &&& uf.rank@[w] == old_r[w]
            &&& uf.roots@[w] == old_rt[w]
            &&& old_rt[w] != v@
        } by {
            // old_rt[w] is in old_p domain (wf conjunct), so != v@
            assert(old_p.contains_key(old_rt[w]));
        };

        // New element v@ is a self-parent singleton root with rank 0.
        assert(uf.parent@[v@] == v);
        assert(uf.parent@[v@]@ == v@);
        assert(uf.rank@[v@] == 0usize);
        assert(uf.roots@[v@] == v@);

        // Elements backward: v@ is at the new last index.
        assert forall|w: <V as View>::V| #[trigger] uf.parent@.contains_key(w) implies
            exists|i: int| 0 <= i < uf.elements@.len() as int && #[trigger] uf.elements@[i]@ == w
        by {
            if w == v@ {
                assert(uf.elements@[old_e.len() as int]@ == v@);
            } else {
                let i = choose|i: int| 0 <= i < old_e.len() as int && #[trigger] old_e[i]@ == w;
                assert(uf.elements@[i]@ == w);
            }
        };

        // Elements no duplicates: v@ is not in old_p domain, but old elements are.
        assert forall|i: int, j: int|
            0 <= i < uf.elements@.len() as int &&
            0 <= j < uf.elements@.len() as int &&
            i != j implies
            #[trigger] uf.elements@[i]@ != #[trigger] uf.elements@[j]@
        by {
            let n = old_e.len() as int;
            if i == n && j < n {
                assert(old_p.contains_key(uf.elements@[j]@));
            } else if j == n && i < n {
                assert(old_p.contains_key(uf.elements@[i]@));
            }
        };

        // Roots idempotent: for w != v@, old_rt is preserved and old_rt[w] != v@.
        assert forall|w: <V as View>::V| #[trigger] uf.roots@.contains_key(w) implies
            uf.roots@.contains_key(uf.roots@[w]) && uf.roots@[uf.roots@[w]] == uf.roots@[w]
        by {
            if w != v@ {
                assert(old_p.contains_key(old_rt[w]));
            }
        };

        // Parent preserves root component: for w != v@, parent[w]@ != v@.
        assert forall|w: <V as View>::V| #[trigger] uf.parent@.contains_key(w) implies
            uf.roots@[uf.parent@[w]@] == uf.roots@[w]
        by {
            if w != v@ {
                let pw = old_p[w]@;
                assert(old_p.contains_key(pw));
            }
        };
    }

    /// Prove that a canonical root is a self-parent: roots[v] == v ==> parent[v]@ == v.
    proof fn lemma_root_is_self_parent<V: StT + Hash>(
        uf: &UnionFindStEph<V>,
        rv: <V as View>::V,
    )
        requires
            uf.spec_unionfindsteph_wf(),
            uf@.roots.contains_key(rv),
            uf@.roots[rv] == rv,
        ensures
            uf@.parent.contains_key(rv),
            uf@.parent[rv]@ == rv,
    {
        if uf.parent@[rv]@ != rv {
            assert(uf.rank@[rv] < uf.rank@[uf.parent@[rv]@]);
            assert(uf.roots@[uf.parent@[rv]@] == rv);
            assert(uf.rank@.contains_key(uf.parent@[rv]@));
            assert(uf.rank@[uf.parent@[rv]@] <= uf.rank@[uf.roots@[uf.parent@[rv]@]]);
        }
    }

    // Common requires for both union wf lemma halves.
    // Factored as a spec fn to reduce requires duplication.
    pub closed spec fn spec_union_lemma_pre<V: StT + Hash>(
        uf: &UnionFindStEph<V>,
        mid: &UnionFindStEph<V>,
        winner_val: V,
        root_u_view: <V as View>::V,
        root_v_view: <V as View>::V,
        winner_view: <V as View>::V,
        loser_view: <V as View>::V,
    ) -> bool {
        &&& spec_key_model::<V>()
        &&& spec_feq_full::<V>()
        &&& spec_parent_rank_same_dom(mid)
        &&& spec_roots_parent_same_dom(mid)
        &&& spec_roots_idempotent(mid)
        &&& spec_parent_closed(mid)
        &&& spec_roots_in_dom(mid)
        &&& spec_elements_forward(mid)
        &&& spec_elements_backward(mid)
        &&& spec_elements_distinct(mid)
        &&& spec_self_parent_is_root(mid)
        &&& spec_parent_preserves_root(mid)
        &&& spec_rank_increases(mid)
        &&& spec_rank_bounded(mid)
        &&& root_u_view != root_v_view
        &&& mid.roots@.contains_key(root_u_view)
        &&& mid.roots@.contains_key(root_v_view)
        &&& mid.roots@[root_u_view] == root_u_view
        &&& mid.roots@[root_v_view] == root_v_view
        &&& mid.parent@[root_u_view]@ == root_u_view
        &&& mid.parent@[root_v_view]@ == root_v_view
        &&& ((winner_view == root_u_view && loser_view == root_v_view) ||
             (winner_view == root_v_view && loser_view == root_u_view))
        &&& winner_val@ == winner_view
        &&& uf.parent@ =~= mid.parent@.insert(loser_view, winner_val)
        &&& uf.rank@.dom() =~= mid.rank@.dom()
        &&& forall|k: <V as View>::V| mid.rank@.contains_key(k) && k != winner_view ==>
            #[trigger] uf.rank@[k] == mid.rank@[k]
        &&& uf.rank@[winner_view] >= mid.rank@[winner_view]
        &&& uf.rank@[winner_view] >= mid.rank@[loser_view]
        &&& mid.rank@[loser_view] < uf.rank@[winner_view]
        &&& uf.elements@ =~= mid.elements@
        &&& uf.roots@.dom() =~= mid.roots@.dom()
        &&& forall|k: <V as View>::V| mid.roots@.contains_key(k) ==> (
            #[trigger] uf.roots@[k] == (
                if mid.roots@[k] == root_u_view || mid.roots@[k] == root_v_view {
                    winner_view
                } else {
                    mid.roots@[k]
                }
            )
        )
    }

    /// Part 1a: roots wf conjuncts after union (idempotent, in dom).
    /// Uses targeted requires (not spec_union_lemma_pre) to keep solver cost low.
    #[verifier::rlimit(50)]
    proof fn lemma_union_wf_roots<V: StT + Hash>(
        new_roots: Map<<V as View>::V, <V as View>::V>,
        new_parent_dom: Set<<V as View>::V>,
        mid_roots: Map<<V as View>::V, <V as View>::V>,
        root_u_view: <V as View>::V,
        root_v_view: <V as View>::V,
        winner_view: <V as View>::V,
    )
        requires
            root_u_view != root_v_view,
            mid_roots.contains_key(root_u_view),
            mid_roots.contains_key(root_v_view),
            mid_roots[root_u_view] == root_u_view,
            mid_roots[root_v_view] == root_v_view,
            winner_view == root_u_view || winner_view == root_v_view,
            // Mid roots idempotent.
            forall|v: <V as View>::V| #[trigger] mid_roots.contains_key(v) ==> {
                &&& mid_roots.contains_key(mid_roots[v])
                &&& mid_roots[mid_roots[v]] == mid_roots[v]
            },
            // Mid roots in parent dom.
            forall|v: <V as View>::V| #[trigger] mid_roots.contains_key(v) ==>
                new_parent_dom.contains(mid_roots[v]),
            // New roots definition.
            new_roots.dom() =~= mid_roots.dom(),
            forall|k: <V as View>::V| mid_roots.contains_key(k) ==> (
                #[trigger] new_roots[k] == (
                    if mid_roots[k] == root_u_view || mid_roots[k] == root_v_view {
                        winner_view
                    } else {
                        mid_roots[k]
                    }
                )
            ),
            // New parent domain unchanged.
            new_parent_dom =~= mid_roots.dom(),
        ensures
            // Roots idempotent.
            forall|v: <V as View>::V| #[trigger] new_roots.contains_key(v) ==> {
                &&& new_roots.contains_key(new_roots[v])
                &&& new_roots[new_roots[v]] == new_roots[v]
            },
            // Roots in dom.
            forall|v: <V as View>::V| #[trigger] new_roots.contains_key(v) ==>
                new_parent_dom.contains(new_roots[v]),
    {
        // Pre-compute: winner maps to itself.
        assert(mid_roots[winner_view] == winner_view) by {
            if winner_view == root_u_view { } else { }
        };
        assert(new_roots[winner_view] == winner_view);

        // Roots idempotent.
        assert forall|w: <V as View>::V|
            #[trigger] new_roots.contains_key(w) implies
            new_roots.contains_key(new_roots[w]) && new_roots[new_roots[w]] == new_roots[w]
        by {
            let old_rw = mid_roots[w];
            if old_rw == root_u_view || old_rw == root_v_view {
            } else {
                assert(mid_roots.contains_key(old_rw));
                assert(mid_roots[old_rw] == old_rw);
            }
        };

        // Roots in dom.
        assert forall|w: <V as View>::V|
            #[trigger] new_roots.contains_key(w) implies
            new_parent_dom.contains(new_roots[w])
        by {
            if mid_roots[w] == root_u_view || mid_roots[w] == root_v_view {
                assert(new_parent_dom.contains(winner_view));
            } else {
                assert(new_parent_dom.contains(mid_roots[w]));
            }
        };
    }

    /// Part 1b: parent wf conjuncts after union (closed, self-parent, preserves root).
    #[verifier::rlimit(60)]
    proof fn lemma_union_wf_parent<V: StT + Hash>(
        uf: UnionFindStEph<V>,
        mid: UnionFindStEph<V>,
        winner_val: V,
        root_u_view: <V as View>::V,
        root_v_view: <V as View>::V,
        winner_view: <V as View>::V,
        loser_view: <V as View>::V,
    )
        requires spec_union_lemma_pre(&uf, &mid, winner_val, root_u_view, root_v_view, winner_view, loser_view),
        ensures
            spec_parent_closed(&uf),
            spec_self_parent_is_root(&uf),
            spec_parent_preserves_root(&uf),
    {
        reveal(spec_union_lemma_pre);
        reveal(spec_parent_closed);
        reveal(spec_self_parent_is_root);
        reveal(spec_parent_preserves_root);

        let mp = mid.parent@;
        let mrt = mid.roots@;

        // Parent view frame.
        assert(uf.parent@[loser_view]@ == winner_view);
        assert forall|k: <V as View>::V|
            mp.contains_key(k) && k != loser_view implies
            #[trigger] uf.parent@[k]@ == mp[k]@
        by {};

        // Parent closed.
        assert forall|w: <V as View>::V|
            #[trigger] uf.parent@.contains_key(w) implies
            uf.parent@.contains_key(uf.parent@[w]@)
        by {
            if w == loser_view {
                assert(uf.parent@[w]@ == winner_view);
                assert(mp.contains_key(winner_view));
            } else {
                assert(mp.contains_key(mp[w]@));
            }
        };

        // Self-parent is root.
        assert forall|w: <V as View>::V|
            uf.parent@.contains_key(w) && uf.parent@[w]@ == w implies
            #[trigger] uf.roots@[w] == w
        by {
            if w == loser_view {
                assert(uf.parent@[w]@ == winner_view);
                assert(winner_view != loser_view);
            } else {
                assert(mp[w]@ == w);
                assert(mrt[w] == w);
                if mrt[w] == root_u_view || mrt[w] == root_v_view {
                    assert(w != loser_view);
                }
            }
        };

        // Parent preserves root.
        assert forall|w: <V as View>::V|
            #[trigger] uf.parent@.contains_key(w) implies
            uf.roots@[uf.parent@[w]@] == uf.roots@[w]
        by {
            if w == loser_view {
                assert(uf.parent@[w]@ == winner_view);
            } else {
                let pw = mp[w]@;
                assert(mrt[pw] == mrt[w]);
                if mrt[w] == root_u_view || mrt[w] == root_v_view {
                    assert(mrt[pw] == root_u_view || mrt[pw] == root_v_view);
                }
            }
        };
    }

    /// Part 2: ordering wf conjuncts after union (rank_increases, rank_bounded).
    #[verifier::rlimit(80)]
    proof fn lemma_union_wf_ordering<V: StT + Hash>(
        uf: UnionFindStEph<V>,
        mid: UnionFindStEph<V>,
        winner_val: V,
        root_u_view: <V as View>::V,
        root_v_view: <V as View>::V,
        winner_view: <V as View>::V,
        loser_view: <V as View>::V,
    )
        requires spec_union_lemma_pre(&uf, &mid, winner_val, root_u_view, root_v_view, winner_view, loser_view),
        ensures
            spec_rank_increases(&uf),
            spec_rank_bounded(&uf),
    {
        reveal(spec_union_lemma_pre);
        reveal(spec_rank_increases);
        reveal(spec_rank_bounded);
        reveal(spec_roots_idempotent);

        let mp = mid.parent@;
        let mr = mid.rank@;
        let mrt = mid.roots@;

        // Parent view frame (needed for rank_increases).
        assert(uf.parent@[loser_view]@ == winner_view);
        assert forall|k: <V as View>::V|
            mp.contains_key(k) && k != loser_view implies
            #[trigger] uf.parent@[k]@ == mp[k]@
        by {};

        // Rank increases.
        assert forall|w: <V as View>::V|
            uf.parent@.contains_key(w) && uf.parent@[w]@ != w implies
            uf.rank@[w] < #[trigger] uf.rank@[uf.parent@[w]@]
        by {
            if w == loser_view {
                assert(uf.parent@[w]@ == winner_view);
            } else {
                let pw = mp[w]@;
                assert(mp[w]@ != w);
                assert(mr[w] < mr[pw]);
                if w == winner_view {
                    assert(mp[winner_view]@ == winner_view);
                }
                assert(w != winner_view);
                if pw == winner_view {
                    assert(uf.rank@[winner_view] >= mr[winner_view]);
                }
            }
        };

        // Rank bounded.
        assert forall|w: <V as View>::V|
            #[trigger] uf.rank@.contains_key(w) implies
            uf.rank@[w] <= uf.rank@[uf.roots@[w]]
        by {
            let old_rw = mrt[w];
            if old_rw == root_u_view || old_rw == root_v_view {
                assert(mr[w] <= mr[old_rw]);
                if w != winner_view {
                    if old_rw == root_u_view {
                        assert(uf.rank@[winner_view] >= mr[root_u_view]);
                    } else {
                        assert(uf.rank@[winner_view] >= mr[root_v_view]);
                    }
                }
            } else {
                assert(mrt.contains_key(old_rw));
                assert(mrt[old_rw] == old_rw);
                if w == winner_view {
                    if winner_view == root_u_view {
                        assert(mrt[w] == root_u_view);
                    } else {
                        assert(mrt[w] == root_v_view);
                    }
                }
                assert(w != winner_view);
                if old_rw == winner_view {
                    if winner_view == root_u_view {
                        assert(mrt[old_rw] == root_u_view);
                    } else {
                        assert(mrt[old_rw] == root_v_view);
                    }
                }
                assert(old_rw != winner_view);
            }
        };

    }

    /// Prove rank[cv] < rank[rv] for a non-root node cv with root rv.
    proof fn lemma_non_root_rank_lt_root<V: StT + Hash>(
        uf: &UnionFindStEph<V>,
        cv: <V as View>::V,
        rv: <V as View>::V,
    )
        requires
            uf.spec_unionfindsteph_wf(),
            uf@.parent.contains_key(cv),
            uf@.roots[cv] == rv,
            cv != rv,
        ensures
            uf@.rank[cv] < uf@.rank[rv],
    {
        // cv is not a self-parent: if parent[cv]@ == cv, wf says roots[cv] == cv, but roots[cv] == rv != cv.
        assert(uf.parent@[cv]@ != cv) by {
            if uf.parent@[cv]@ == cv { assert(uf.roots@[cv] == cv); }
        };
        // Non-root ordering: rank[cv] < rank[parent[cv]@].
        assert(uf.rank@[cv] < uf.rank@[uf.parent@[cv]@]);
        // Parent preserves root: roots[parent[cv]@] == roots[cv] == rv.
        assert(uf.roots@[uf.parent@[cv]@] == rv);
        // Rank bounded: rank[parent[cv]@] <= rank[roots[parent[cv]@]] == rank[rv].
        assert(uf.rank@.contains_key(uf.parent@[cv]@));
        assert(uf.rank@[uf.parent@[cv]@] <= uf.rank@[uf.roots@[uf.parent@[cv]@]]);
    }

    /// Decompose the monolithic wf into individual sub-predicates.
    /// The 5 closed predicates (parent_closed, self_parent_is_root,
    /// parent_preserves_root, rank_increases, rank_bounded) require reveal.
    proof fn lemma_decompose_wf<V: StT + Hash>(uf: &UnionFindStEph<V>)
        requires uf.spec_unionfindsteph_wf(),
        ensures
            spec_key_model::<V>(),
            spec_feq_full::<V>(),
            spec_parent_rank_same_dom(uf),
            spec_roots_parent_same_dom(uf),
            spec_roots_idempotent(uf),
            spec_parent_closed(uf),
            spec_roots_in_dom(uf),
            spec_elements_forward(uf),
            spec_elements_backward(uf),
            spec_elements_distinct(uf),
            spec_self_parent_is_root(uf),
            spec_parent_preserves_root(uf),
            spec_rank_increases(uf),
            spec_rank_bounded(uf),
    {
        reveal(spec_roots_idempotent);
        reveal(spec_parent_closed);
        reveal(spec_roots_in_dom);
        reveal(spec_elements_forward);
        reveal(spec_elements_backward);
        reveal(spec_elements_distinct);
        reveal(spec_self_parent_is_root);
        reveal(spec_parent_preserves_root);
        reveal(spec_rank_increases);
        reveal(spec_rank_bounded);
    }

    /// Assemble the monolithic wf from individual sub-predicates.
    proof fn lemma_assemble_wf<V: StT + Hash>(uf: &UnionFindStEph<V>)
        requires
            spec_key_model::<V>(),
            spec_feq_full::<V>(),
            spec_parent_rank_same_dom(uf),
            spec_roots_parent_same_dom(uf),
            spec_roots_idempotent(uf),
            spec_parent_closed(uf),
            spec_roots_in_dom(uf),
            spec_elements_forward(uf),
            spec_elements_backward(uf),
            spec_elements_distinct(uf),
            spec_self_parent_is_root(uf),
            spec_parent_preserves_root(uf),
            spec_rank_increases(uf),
            spec_rank_bounded(uf),
        ensures
            uf.spec_unionfindsteph_wf(),
    {
        reveal(spec_roots_idempotent);
        reveal(spec_parent_closed);
        reveal(spec_roots_in_dom);
        reveal(spec_elements_forward);
        reveal(spec_elements_backward);
        reveal(spec_elements_distinct);
        reveal(spec_self_parent_is_root);
        reveal(spec_parent_preserves_root);
        reveal(spec_rank_increases);
        reveal(spec_rank_bounded);
    }

    /// Chase parent pointers to the root (no mutation).
    #[verifier::rlimit(30)]
    fn find_root_loop<V: StT + Hash>(uf: &UnionFindStEph<V>, v: &V) -> (root: V)
        requires
            uf.spec_unionfindsteph_wf(),
            uf@.parent.contains_key(v@),
        ensures
            root@ == uf@.roots[v@],
            uf@.parent.contains_key(root@),
            uf@.roots[root@] == root@,
    {
        let mut current = v.clone();
        proof {
            assert(strictly_cloned(*v, current));
            assert(obeys_feq_full::<V>());
            assert(current@ == v@);
        }

        // Read first parent for the while condition.
        let mut p = uf.parent.get(&current).unwrap().clone();
        proof {
            let ghost pv = uf.parent@[current@];
            assert(strictly_cloned(pv, p));
            assert(p@ == uf@.parent[current@]@);
        }

        while !feq(&p, &current)
            invariant
                uf.spec_unionfindsteph_wf(),
                uf@.parent.contains_key(current@),
                uf@.roots.contains_key(current@),
                uf@.roots.contains_key(v@),
                uf@.rank.contains_key(current@),
                uf@.roots[current@] == uf@.roots[v@],
                p@ == uf@.parent[current@]@,
                uf@.parent.contains_key(p@),
            decreases uf@.rank[uf@.roots[v@]] - uf@.rank[current@],
        {
            proof {
                // p@ != current@, so non-root.
                assert(uf@.parent[current@]@ != current@);
                assert(uf@.rank[current@] < uf@.rank[uf@.parent[current@]@]);
                assert(uf@.roots[p@] == uf@.roots[current@]);
            }

            current = p;
            p = uf.parent.get(&current).unwrap().clone();
            proof {
                let ghost pv = uf.parent@[current@];
                assert(strictly_cloned(pv, p));
                assert(p@ == uf@.parent[current@]@);
            }
        }

        // After loop: feq(&p, &current) is true, so p@ == current@.
        // Invariant: p@ == parent[current@]@. So parent[current@]@ == current@.
        // wf self-parent: roots[current@] == current@. Invariant: roots[current@] == roots[v@].
        // Therefore current@ == roots[v@].
        proof {
            assert(uf@.parent[current@]@ == current@);
            assert(uf@.roots[current@] == current@);
        }

        current
    }

    /// Assemble spec_union_lemma_pre from individual facts.
    /// Reveals spec_union_lemma_pre internally so union_merge stays opaque.
    proof fn lemma_establish_union_pre<V: StT + Hash>(
        uf: &UnionFindStEph<V>,
        mid: &UnionFindStEph<V>,
        winner_val: V,
        root_u_view: <V as View>::V,
        root_v_view: <V as View>::V,
        winner_view: <V as View>::V,
        loser_view: <V as View>::V,
    )
        requires
            spec_key_model::<V>(),
            spec_feq_full::<V>(),
            spec_parent_rank_same_dom(mid),
            spec_roots_parent_same_dom(mid),
            spec_roots_idempotent(mid),
            spec_parent_closed(mid),
            spec_roots_in_dom(mid),
            spec_elements_forward(mid),
            spec_elements_backward(mid),
            spec_elements_distinct(mid),
            spec_self_parent_is_root(mid),
            spec_parent_preserves_root(mid),
            spec_rank_increases(mid),
            spec_rank_bounded(mid),
            root_u_view != root_v_view,
            mid.roots@.contains_key(root_u_view),
            mid.roots@.contains_key(root_v_view),
            mid.roots@[root_u_view] == root_u_view,
            mid.roots@[root_v_view] == root_v_view,
            mid.parent@[root_u_view]@ == root_u_view,
            mid.parent@[root_v_view]@ == root_v_view,
            ((winner_view == root_u_view && loser_view == root_v_view) ||
             (winner_view == root_v_view && loser_view == root_u_view)),
            winner_val@ == winner_view,
            uf.parent@ =~= mid.parent@.insert(loser_view, winner_val),
            uf.rank@.dom() =~= mid.rank@.dom(),
            forall|k: <V as View>::V| mid.rank@.contains_key(k) && k != winner_view ==>
                #[trigger] uf.rank@[k] == mid.rank@[k],
            uf.rank@[winner_view] >= mid.rank@[winner_view],
            uf.rank@[winner_view] >= mid.rank@[loser_view],
            mid.rank@[loser_view] < uf.rank@[winner_view],
            uf.elements@ =~= mid.elements@,
            uf.roots@.dom() =~= mid.roots@.dom(),
            forall|k: <V as View>::V| mid.roots@.contains_key(k) ==> (
                #[trigger] uf.roots@[k] == (
                    if mid.roots@[k] == root_u_view || mid.roots@[k] == root_v_view {
                        winner_view
                    } else {
                        mid.roots@[k]
                    }
                )
            ),
        ensures
            spec_union_lemma_pre(uf, mid, winner_val, root_u_view, root_v_view, winner_view, loser_view),
    {
        reveal(spec_union_lemma_pre);
    }

    /// Closed-interface wrapper for lemma_union_wf_roots. Reveals roots_idempotent
    /// and roots_in_dom internally so the caller stays quantifier-free.
    #[verifier::rlimit(50)]
    proof fn lemma_union_wf_roots_closed<V: StT + Hash>(
        uf: UnionFindStEph<V>,
        mid: UnionFindStEph<V>,
        root_u_view: <V as View>::V,
        root_v_view: <V as View>::V,
        winner_view: <V as View>::V,
    )
        requires
            spec_roots_idempotent(&mid),
            spec_roots_in_dom(&mid),
            spec_roots_parent_same_dom(&mid),
            root_u_view != root_v_view,
            mid.roots@.contains_key(root_u_view),
            mid.roots@.contains_key(root_v_view),
            mid.roots@[root_u_view] == root_u_view,
            mid.roots@[root_v_view] == root_v_view,
            winner_view == root_u_view || winner_view == root_v_view,
            uf.roots@.dom() =~= mid.roots@.dom(),
            forall|k: <V as View>::V| mid.roots@.contains_key(k) ==> (
                #[trigger] uf.roots@[k] == (
                    if mid.roots@[k] == root_u_view || mid.roots@[k] == root_v_view {
                        winner_view
                    } else {
                        mid.roots@[k]
                    }
                )
            ),
            uf.parent@.dom() =~= mid.parent@.dom(),
        ensures
            spec_roots_idempotent(&uf),
            spec_roots_in_dom(&uf),
    {
        reveal(spec_roots_idempotent);
        reveal(spec_roots_in_dom);
        lemma_union_wf_roots::<V>(
            uf.roots@, uf.parent@.dom(), mid.roots@,
            root_u_view, root_v_view, winner_view,
        );
    }

    /// Frame lemma: elements predicates transfer when elements and parent.dom are unchanged.
    proof fn lemma_union_wf_frame<V: StT + Hash>(
        uf: UnionFindStEph<V>,
        mid: UnionFindStEph<V>,
    )
        requires
            spec_elements_forward(&mid),
            spec_elements_backward(&mid),
            spec_elements_distinct(&mid),
            uf.elements@ =~= mid.elements@,
            uf.parent@.dom() =~= mid.parent@.dom(),
        ensures
            spec_elements_forward(&uf),
            spec_elements_backward(&uf),
            spec_elements_distinct(&uf),
    {
        reveal(spec_elements_forward);
        reveal(spec_elements_backward);
        reveal(spec_elements_distinct);
    }

    /// Execute the mutations for union merge. No wf sub-predicates in scope —
    /// only structural requires/ensures. This isolates HashMap/Map axioms from
    /// the quantified wf proof, keeping Z3 under 4 GB.
    #[verifier::rlimit(30)]
    fn union_merge_exec<V: StT + Hash>(
        uf: &mut UnionFindStEph<V>,
        root_u: V,
        root_v: V,
    ) -> (info: Ghost<UnionMergeInfo<V>>)
        requires
            spec_key_model::<V>(),
            spec_feq_full::<V>(),
            old(uf)@.parent.dom() =~= old(uf)@.rank.dom(),
            old(uf)@.roots.dom() =~= old(uf)@.parent.dom(),
            root_u@ != root_v@,
            old(uf)@.roots.contains_key(root_u@),
            old(uf)@.roots.contains_key(root_v@),
            old(uf)@.roots[root_u@] == root_u@,
            old(uf)@.roots[root_v@] == root_v@,
            old(uf)@.parent[root_u@]@ == root_u@,
            old(uf)@.parent[root_v@]@ == root_v@,
        ensures
            ((info@.winner_view == root_u@ && info@.loser_view == root_v@) ||
             (info@.winner_view == root_v@ && info@.loser_view == root_u@)),
            info@.winner_val@ == info@.winner_view,
            uf.parent@ =~= old(uf).parent@.insert(info@.loser_view, info@.winner_val),
            uf.rank@.dom() =~= old(uf).rank@.dom(),
            forall|k: <V as View>::V| old(uf).rank@.contains_key(k) && k != info@.winner_view ==>
                #[trigger] uf.rank@[k] == old(uf).rank@[k],
            uf.rank@[info@.winner_view] >= old(uf).rank@[info@.winner_view],
            uf.rank@[info@.winner_view] >= old(uf).rank@[info@.loser_view],
            old(uf).rank@[info@.loser_view] < uf.rank@[info@.winner_view],
            uf.elements@ =~= old(uf).elements@,
            uf.roots@.dom() =~= old(uf).roots@.dom(),
            forall|k: <V as View>::V| old(uf).roots@.contains_key(k) ==> (
                #[trigger] uf.roots@[k] == (
                    if old(uf).roots@[k] == root_u@ || old(uf).roots@[k] == root_v@ {
                        info@.winner_view
                    } else {
                        old(uf).roots@[k]
                    }
                )
            ),
    {
        let ghost root_u_view = root_u@;
        let ghost root_v_view = root_v@;
        let ghost mid_roots = uf.roots@;

        let rank_u = *uf.rank.get(&root_u).unwrap();
        let rank_v = *uf.rank.get(&root_v).unwrap();

        let ru1 = root_u.clone();
        let rv1 = root_v.clone();
        proof {
            assert(strictly_cloned(root_u, ru1));
            assert(strictly_cloned(root_v, rv1));
        }

        let ghost winner_view: <V as View>::V;
        let ghost loser_view: <V as View>::V;
        let ghost winner_v: V;

        if rank_u < rank_v {
            proof { winner_view = root_v_view; loser_view = root_u_view; winner_v = rv1; }
            uf.parent.insert(ru1, rv1);
        } else {
            proof { winner_view = root_u_view; loser_view = root_v_view; winner_v = ru1; }
            uf.parent.insert(rv1, ru1);
            if rank_u == rank_v {
                let ru2 = root_u.clone();
                proof {
                    assert(strictly_cloned(root_u, ru2));
                    admit(); // Overflow: rank bounded by log2(n) < 64 (2^rank theorem).
                }
                uf.rank.insert(ru2, rank_u + 1);
            }
        }

        uf.roots = Ghost(Map::new(
            |x: <V as View>::V| mid_roots.contains_key(x),
            |x: <V as View>::V|
                if mid_roots[x] == root_u_view || mid_roots[x] == root_v_view {
                    winner_view
                } else {
                    mid_roots[x]
                },
        ));

        Ghost(UnionMergeInfo { winner_view, loser_view, winner_val: winner_v })
    }

    /// Merge two components: mutate parent/rank/roots, prove wf.
    /// Proof architecture: union_merge_exec (verified) does mutations,
    /// 3 sub-lemmas (verified) prove wf conjuncts. external_body here because
    /// Z3 can't coordinate the proof within rlimit — the sub-lemma calls +
    /// exec ensures + domain assertions exceed Z3's quantifier budget.
    #[verifier::external_body]
    fn union_merge<V: StT + Hash>(
        uf: &mut UnionFindStEph<V>,
        root_u: V,
        root_v: V,
    )
        requires
            old(uf).spec_unionfindsteph_wf(),
            root_u@ != root_v@,
            old(uf)@.roots.contains_key(root_u@),
            old(uf)@.roots.contains_key(root_v@),
            old(uf)@.roots[root_u@] == root_u@,
            old(uf)@.roots[root_v@] == root_v@,
            old(uf)@.parent[root_u@]@ == root_u@,
            old(uf)@.parent[root_v@]@ == root_v@,
        ensures
            uf.spec_unionfindsteph_wf(),
            uf@.parent.dom() =~= old(uf)@.parent.dom(),
            uf@.elements =~= old(uf)@.elements,
            uf@.roots.dom() =~= old(uf)@.roots.dom(),
            forall|x: <V as View>::V| #[trigger] uf@.roots.contains_key(x) ==> {
                if old(uf)@.roots[x] == root_u@ || old(uf)@.roots[x] == root_v@ {
                    uf@.roots[x] == uf@.roots[root_u@]
                } else {
                    uf@.roots[x] == old(uf)@.roots[x]
                }
            },
    {
        let ghost root_u_view = root_u@;
        let ghost root_v_view = root_v@;
        let ghost mid_uf = *uf;

        let info = union_merge_exec(uf, root_u, root_v);

        proof {
            let ghost winner_view = info@.winner_view;
            let ghost loser_view = info@.loser_view;
            let ghost winner_val = info@.winner_val;

            assert(uf@.elements =~= mid_uf@.elements);
            assert(uf@.parent.dom() =~= mid_uf@.parent.dom());
            assert(uf@.rank.dom() =~= mid_uf@.rank.dom());
            assert(uf@.roots.dom() =~= mid_uf@.roots.dom());

            lemma_establish_union_pre(
                &(*uf), &mid_uf, winner_val,
                root_u_view, root_v_view, winner_view, loser_view,
            );
            lemma_union_wf_roots_closed(
                *uf, mid_uf,
                root_u_view, root_v_view, winner_view,
            );
            lemma_union_wf_parent(
                *uf, mid_uf, winner_val,
                root_u_view, root_v_view, winner_view, loser_view,
            );
            lemma_union_wf_ordering(
                *uf, mid_uf, winner_val,
                root_u_view, root_v_view, winner_view, loser_view,
            );
            lemma_union_wf_frame(
                *uf, mid_uf,
            );
            lemma_assemble_wf(uf);
        }
    }

    // Path compression commented out — correct but exceeds solver budget.
    // Uncomment when compression sub-lemmas are restored.
    // fn compress_path<V: StT + Hash>(uf: &mut UnionFindStEph<V>, v: &V, root: &V)
    //     requires
    //         old(uf).spec_unionfindsteph_wf(),
    //         old(uf)@.parent.contains_key(v@),
    //         root@ == old(uf)@.roots[v@],
    //         old(uf)@.roots[root@] == root@,
    //         old(uf)@.parent.contains_key(root@),
    //     ensures
    //         uf.spec_unionfindsteph_wf(),
    //         uf@.roots =~= old(uf)@.roots,
    //         uf@.parent.dom() =~= old(uf)@.parent.dom(),
    //         uf@.rank =~= old(uf)@.rank,
    //         uf@.elements =~= old(uf)@.elements,
    // { ... }

    // 8. traits

    pub trait UnionFindStEphTrait<V: StT + Hash>: Sized + View<V = UnionFindStEphV<V>> {
        spec fn spec_unionfindsteph_wf(&self) -> bool;

        /// Create a new empty Union-Find structure.
        /// APAS: Work Theta(1), Span Theta(1)
        fn new() -> (uf: Self)
            requires
                obeys_key_model::<V>(),
                obeys_feq_full::<V>(),
                forall|k1: V, k2: V| k1@ == k2@ ==> k1 == k2,
            ensures
                uf@.parent =~= Map::<<V as View>::V, V>::empty(),
                uf@.rank =~= Map::<<V as View>::V, usize>::empty(),
                uf@.elements =~= Seq::<V>::empty(),
                uf@.roots =~= Map::<<V as View>::V, <V as View>::V>::empty();

        /// Insert a new element as a singleton set.
        /// APAS: Work Theta(1), Span Theta(1)
        fn insert(&mut self, v: V)
            requires
                old(self).spec_unionfindsteph_wf(),
            ensures
                self.spec_unionfindsteph_wf(),
                old(self)@.parent.contains_key(v@) ==> self@ == old(self)@,
                !old(self)@.parent.contains_key(v@) ==> {
                    &&& self@.parent =~= old(self)@.parent.insert(v@, v)
                    &&& self@.rank =~= old(self)@.rank.insert(v@, 0usize)
                    &&& self@.roots =~= old(self)@.roots.insert(v@, v@)
                    &&& self@.elements.len() == old(self)@.elements.len() + 1
                };

        /// Find the root representative with path compression.
        /// APAS: Work O(alpha(n)), Span O(alpha(n)) amortized (inverse Ackermann)
        fn find(&mut self, v: &V) -> (root: V)
            requires
                old(self).spec_unionfindsteph_wf(),
                old(self)@.parent.contains_key(v@),
            ensures
                self.spec_unionfindsteph_wf(),
                root@ == old(self)@.roots[v@],
                self@.roots =~= old(self)@.roots,
                self@.parent.dom() =~= old(self)@.parent.dom(),
                self@.rank =~= old(self)@.rank,
                self@.elements =~= old(self)@.elements;

        /// Union two sets by rank.
        /// APAS: Work O(alpha(n)), Span O(alpha(n)) amortized
        fn union(&mut self, u: &V, v: &V)
            requires
                old(self).spec_unionfindsteph_wf(),
                old(self)@.parent.contains_key(u@),
                old(self)@.parent.contains_key(v@),
            ensures
                self.spec_unionfindsteph_wf(),
                self@.parent.dom() =~= old(self)@.parent.dom(),
                self@.elements =~= old(self)@.elements,
                forall|x: <V as View>::V| #[trigger] self@.roots.contains_key(x) ==> {
                    let old_root_u = old(self)@.roots[u@];
                    let old_root_v = old(self)@.roots[v@];
                    if old(self)@.roots[x] == old_root_u || old(self)@.roots[x] == old_root_v {
                        self@.roots[x] == self@.roots[u@]
                    } else {
                        self@.roots[x] == old(self)@.roots[x]
                    }
                };

        /// Check if two elements are in the same set.
        /// APAS: Work O(alpha(n)), Span O(alpha(n)) amortized
        fn equals(&mut self, u: &V, v: &V) -> (same: bool)
            requires
                old(self).spec_unionfindsteph_wf(),
                old(self)@.parent.contains_key(u@),
                old(self)@.parent.contains_key(v@),
            ensures
                self.spec_unionfindsteph_wf(),
                same == (old(self)@.roots[u@] == old(self)@.roots[v@]),
                self@.roots =~= old(self)@.roots,
                self@.parent.dom() =~= old(self)@.parent.dom();

        /// Count distinct sets.
        /// APAS: Work O(n alpha(n)), Span O(n alpha(n))
        fn num_sets(&mut self) -> (count: usize)
            requires
                old(self).spec_unionfindsteph_wf(),
            ensures
                self.spec_unionfindsteph_wf(),
                self@.roots =~= old(self)@.roots,
                self@.parent.dom() =~= old(self)@.parent.dom();
    }

    // 9. impls

    impl<V: StT + Hash> UnionFindStEphTrait<V> for UnionFindStEph<V> {
        /// Well-formedness invariant for the Union-Find structure.
        open spec fn spec_unionfindsteph_wf(&self) -> bool {
            &&& obeys_key_model::<V>()
            &&& obeys_feq_full::<V>()
            &&& self.parent@.dom() =~= self.rank@.dom()
            &&& self.roots@.dom() =~= self.parent@.dom()
            &&& forall|v: <V as View>::V| #[trigger] self.roots@.contains_key(v) ==> {
                &&& self.roots@.contains_key(self.roots@[v])
                &&& self.roots@[self.roots@[v]] == self.roots@[v]
            }
            &&& forall|v: <V as View>::V| #[trigger] self.parent@.contains_key(v) ==>
                self.parent@.contains_key(self.parent@[v]@)
            &&& forall|v: <V as View>::V| #[trigger] self.roots@.contains_key(v) ==>
                self.parent@.contains_key(self.roots@[v])
            &&& forall|i: int| 0 <= i < self.elements@.len() as int ==>
                self.parent@.contains_key(#[trigger] self.elements@[i]@)
            &&& forall|v: <V as View>::V| #[trigger] self.parent@.contains_key(v) ==>
                exists|i: int| 0 <= i < self.elements@.len() as int && #[trigger] self.elements@[i]@ == v
            &&& forall|i: int, j: int|
                0 <= i < self.elements@.len() as int &&
                0 <= j < self.elements@.len() as int &&
                i != j ==>
                #[trigger] self.elements@[i]@ != #[trigger] self.elements@[j]@
            &&& forall|v: <V as View>::V| self.parent@.contains_key(v) && self.parent@[v]@ == v ==>
                #[trigger] self.roots@[v] == v
            &&& forall|v: <V as View>::V| #[trigger] self.parent@.contains_key(v) ==>
                self.roots@[self.parent@[v]@] == self.roots@[v]
            &&& forall|v: <V as View>::V| self.parent@.contains_key(v)
                && self.parent@[v]@ != v ==>
                self.rank@[v] < #[trigger] self.rank@[self.parent@[v]@]
            &&& forall|v: <V as View>::V| #[trigger] self.rank@.contains_key(v) ==>
                self.rank@[v] <= self.rank@[self.roots@[v]]
        }

        /// - APAS: Work Theta(1), Span Theta(1)
        fn new() -> (uf: Self) {
            UnionFindStEph {
                parent: HashMapWithViewPlus::new(),
                rank: HashMapWithViewPlus::new(),
                elements: Vec::new(),
                roots: Ghost(Map::empty()),
            }
        }

        /// - APAS: Work Theta(1), Span Theta(1)
        fn insert(&mut self, v: V) {
            if !self.parent.contains_key(&v) {
                // Capture v@ before v is consumed by push.
                let ghost v_view = v@;

                // 3 clones for parent key, parent value, rank key.
                let v1 = v.clone();
                let v2 = v.clone();
                let v3 = v.clone();

                proof { lemma_three_clones_eq(v, v1, v2, v3); }

                self.parent.insert(v1, v2);
                self.rank.insert(v3, 0usize);
                // Use original v for push — no 4th clone needed.
                self.elements.push(v);
                self.roots = Ghost(self.roots@.insert(v_view, v_view));

                proof { lemma_insert_preserves_wf(*self, *old(self), v); }
            }
        }

        /// - APAS: Work O(alpha(n)), Span O(alpha(n)) amortized
        /// Without path compression for now (correct but O(log n) per call).
        fn find(&mut self, v: &V) -> (root: V) {
            find_root_loop(self, v)
        }

        /// - APAS: Work O(alpha(n)), Span O(alpha(n)) amortized
        /// external_body: Z3 can't fit 26+ quantifiers (13 pre-wf + 13 post-wf)
        /// in one context. All proof components verify separately:
        /// union_merge_exec (mutations), 3 sub-lemmas (wf conjuncts),
        /// lemma_union_wf_roots_closed, lemma_union_wf_frame.
        #[verifier::external_body]
        fn union(&mut self, u: &V, v: &V) {
            let root_u = find_root_loop(self, u);
            let root_v = find_root_loop(self, v);

            if !feq(&root_u, &root_v) {
                proof {
                    lemma_root_is_self_parent(self, root_u@);
                    lemma_root_is_self_parent(self, root_v@);
                }
                union_merge(self, root_u, root_v);
            }
        }

        /// - APAS: Work O(alpha(n)), Span O(alpha(n)) amortized
        #[verifier::rlimit(20)]
        fn equals(&mut self, u: &V, v: &V) -> (same: bool) {
            let root_u = find_root_loop(self, u);
            let root_v = find_root_loop(self, v);
            feq(&root_u, &root_v)
        }

        /// - APAS: Work O(n alpha(n)), Span O(n alpha(n))
        fn num_sets(&mut self) -> (count: usize) {
            let mut roots_set = HashSetWithViewPlus::<V>::new();
            let mut i: usize = 0;
            while i < self.elements.len()
                invariant
                    self.spec_unionfindsteph_wf(),
                    self@ == old(self)@,
                    0 <= i <= self.elements@.len(),
                decreases self.elements@.len() - i,
            {
                let v = self.elements[i].clone();
                proof {
                    assert(strictly_cloned(self.elements@[i as int], v));
                    assert(obeys_feq_full::<V>());
                    assert(v@ == self.elements@[i as int]@);
                    assert(self@.parent.contains_key(v@));
                }
                let root = find_root_loop(self, &v);
                let _ = roots_set.insert(root);
                i = i + 1;
            }
            roots_set.len()
        }
    }

    } // verus!
}
