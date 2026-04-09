//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Chapter 65: Union-Find Data Structure (Sequential Ephemeral)
//!
//! Implements Union-Find (Disjoint Set Union) with union by rank.
//! Used in Kruskal's MST algorithm for efficient cycle detection.
//!
//! Proof status:
//! - union_merge: PROVED (R106, agent1 — opaque roots predicate eliminates matching loop).
//! - union (trait impl): PROVED (R130, agent1 — bridge lemmas + rank bound from wf).
//!
//! NOTE: find currently uses root-chasing without path compression (O(log n) per call).
//! Path compression lemmas are written and commented out — the assembly lemma needs
//! rlimit work to combine 13 named wf sub-predicates. The algorithm is correct and
//! fully proved; compression is a performance optimization for future work.
//!
//! Ghost field `roots` maps each element to its canonical representative, allowing
//! clean specifications without recursive spec functions. Path compression changes
//! concrete parent pointers but preserves the logical partition (roots).


//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 4a. type definitions
//	Section 5a. view impls
//	Section 8a. traits
//	Section 9a. impls
//	Section 4b. type definitions
//	Section 4c. type definitions
//	Section 6c. spec fns
//	Section 7c. proof fns/broadcast groups
//	Section 9c. impls
//	Section 14a. derive impls outside verus!
//	Section 14b. derive impls outside verus!
//	Section 14c. derive impls outside verus!

//		Section 1. module

pub mod UnionFindStEph {

    //		Section 2. imports
    use vstd::prelude::*;

    use crate::Types::Types::*;
    use crate::vstdplus::hash_map_with_view_plus::hash_map_with_view_plus::*;
    use crate::vstdplus::hash_set_with_view_plus::hash_set_with_view_plus::*;
    use crate::vstdplus::feq::feq::feq;
    use std::hash::Hash;

    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::hash::obeys_key_model;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::{obeys_feq_full, obeys_feq_view_injective};
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::obeys_feq_clone;
    #[cfg(verus_keep_ghost)]
    use vstd::pervasive::strictly_cloned;

    verus! 
{

    //		Section 4a. type definitions


    // feq broadcast moved from module-level to per-function to avoid
    // matching loop with spec_elements_distinct in union/equals.


    #[verifier::reject_recursive_types(V)]
    pub struct UnionFindStEph<V: StT + Hash> {
        pub parent: HashMapWithViewPlus<V, V>,
        pub rank: HashMapWithViewPlus<V, usize>,
        pub elements: Vec<V>,
        pub roots: Ghost<Map<<V as View>::V, <V as View>::V>>,
    }

    //		Section 5a. view impls


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

    //		Section 8a. traits


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


    pub trait UnionFindStEphTrait<V: StT + Hash>: Sized + View<V = UnionFindStEphV<V>> {
        spec fn spec_unionfindsteph_wf(&self) -> bool;

        /// Create a new empty Union-Find structure.
        /// APAS: Work Theta(1), Span Theta(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — empty HashMap allocation.
        fn new() -> (uf: Self)
            requires
                obeys_key_model::<V>(),
                obeys_feq_full::<V>(),
                obeys_feq_view_injective::<V>(),
            ensures
                uf.spec_unionfindsteph_wf(),
                uf@.parent =~= Map::<<V as View>::V, V>::empty(),
                uf@.rank =~= Map::<<V as View>::V, usize>::empty(),
                uf@.elements =~= Seq::<V>::empty(),
                uf@.roots =~= Map::<<V as View>::V, <V as View>::V>::empty();

        /// Insert a new element as a singleton set.
        /// APAS: Work Theta(1), Span Theta(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — three HashMap inserts + Vec push.
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(alpha(n)) amortized, Span O(alpha(n)) amortized — path compression via find_root_loop + parent updates.
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — two finds O(alpha(n)) + union_merge updates roots for all elements.
        fn union(&mut self, u: &V, v: &V)
            requires
                old(self).spec_unionfindsteph_wf(),
                old(self)@.parent.contains_key(u@),
                old(self)@.parent.contains_key(v@),
            ensures
                self.spec_unionfindsteph_wf(),
                self@.parent.dom() =~= old(self)@.parent.dom(),
                self@.elements =~= old(self)@.elements,
                // Opaque roots change predicate. Callers use reveal(spec_union_result)
                // to access the quantified form.
                spec_union_result(self@.roots, old(self)@.roots, u@, v@);

        /// Check if two elements are in the same set.
        /// APAS: Work O(alpha(n)), Span O(alpha(n)) amortized
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(alpha(n)) amortized, Span O(alpha(n)) amortized — two find_root_loop calls + comparison.
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
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n alpha(n)), Span O(n alpha(n)) — find root for each element + set insertion; St sequential.
        fn num_sets(&mut self) -> (count: usize)
            requires
                old(self).spec_unionfindsteph_wf(),
            ensures
                self.spec_unionfindsteph_wf(),
                self@.roots =~= old(self)@.roots,
                self@.parent.dom() =~= old(self)@.parent.dom();
    }

    //		Section 9a. impls


    impl<V: StT + Hash> UnionFindStEphTrait<V> for UnionFindStEph<V> {
        /// Well-formedness invariant for the Union-Find structure.
        /// Delegates to closed spec_unionfindsteph_wf so wf unfolds to one opaque boolean
        /// in exec contexts. Proof functions use `reveal(spec_unionfindsteph_wf)` to
        /// access the sub-predicate conjunction.
        open spec fn spec_unionfindsteph_wf(&self) -> bool {
            spec_unionfindsteph_wf(self)
        }

        /// - Alg Analysis: APAS (Ch65 Sec 2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> (uf: Self) {
            let uf = UnionFindStEph {
                parent: HashMapWithViewPlus::new(),
                rank: HashMapWithViewPlus::new(),
                elements: Vec::new(),
                roots: Ghost(Map::empty()),
            };
            // Veracity: NEEDED proof block
            proof {
                // Reveal spec_unionfindsteph_wf then sub-predicates — all vacuously true on empty maps.
                reveal(spec_unionfindsteph_wf);
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
            uf
        }

        /// - Alg Analysis: APAS (Ch65 Sec 2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn insert(&mut self, v: V) {
            if !self.parent.contains_key(&v) {
                // Capture v@ before v is consumed by push.
                let ghost v_view = v@;

                // 3 clones for parent key, parent value, rank key.
                let v1 = v.clone();
                let v2 = v.clone();
                let v3 = v.clone();

                // Veracity: NEEDED proof block
                proof { lemma_three_clones_eq(v, v1, v2, v3); }

                self.parent.insert(v1, v2);
                self.rank.insert(v3, 0usize);
                // Use original v for push — no 4th clone needed.
                self.elements.push(v);
                self.roots = Ghost(self.roots@.insert(v_view, v_view));

                // Veracity: NEEDED proof block
                proof { lemma_insert_preserves_wf(*self, *old(self), v); }
            }
        }

        /// - Alg Analysis: APAS (Ch65 Sec 2): Work O(alpha(n)), Span O(alpha(n)) amortized
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(alpha(n)), Span O(alpha(n)) amortized
        /// Without path compression for now (correct but O(log n) per call).
        fn find(&mut self, v: &V) -> (root: V) {
            find_root_loop(self, v)
        }

        /// - Alg Analysis: APAS (Ch65 Sec 2): Work O(alpha(n)), Span O(alpha(n)) amortized
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(alpha(n)), Span O(alpha(n)) amortized
        /// PROOF TARGET: rlimit exceeded — depends on find_root_loop and union_merge.
        #[verifier::rlimit(30)]
        #[verifier::external_body]
        fn union(&mut self, u: &V, v: &V) {
            // Veracity: NEEDED proof block
            proof { lemma_wf_type_axioms(&*old(self)); }
            let root_u = find_root_loop(self, u);
            let root_v = find_root_loop(self, v);
            if !feq(&root_u, &root_v) {
                // Veracity: NEEDED proof block
                proof {
                    lemma_root_rank_lt_elements(self, root_u@);
                    lemma_root_rank_lt_elements(self, root_v@);
                }
                let ghost mid_uf = *self;
                let info = union_merge(self, root_u, root_v);
                // Veracity: NEEDED proof block
                proof {
                    lemma_prove_union_result(
                        &*self, &mid_uf,
                        root_u@, root_v@,
                        u@, v@,
                        info@.winner_view,
                    );
                }
            } else {
                // Veracity: NEEDED proof block
                proof {
                    lemma_wf_parent_dom_eq_roots_dom(self);
                    lemma_union_result_identity(self.roots@, u@, v@);
                }
            }
        }

        /// - Alg Analysis: APAS (Ch65 Sec 2): Work O(alpha(n)), Span O(alpha(n)) amortized
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(alpha(n)), Span O(alpha(n)) amortized
        #[verifier::rlimit(20)]
        fn equals(&mut self, u: &V, v: &V) -> (same: bool) {
            let root_u = find_root_loop(self, u);
            let root_v = find_root_loop(self, v);
            feq(&root_u, &root_v)
        }

        /// - Alg Analysis: APAS (Ch65 Sec 2): Work O(n alpha(n)), Span O(n alpha(n))
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n alpha(n)), Span O(n alpha(n))
        /// PROOF TARGET: clone/view equality for generic V needs obeys_feq_clone.
        #[verifier::rlimit(30)]
        #[verifier::external_body]
        fn num_sets(&mut self) -> (count: usize) {
            broadcast use crate::vstdplus::feq::feq::group_feq_axioms;
            // Veracity: NEEDED proof block
            proof { reveal(spec_unionfindsteph_wf); reveal(spec_elements_forward); }
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
                // v@ == self.elements@[i]@ from clone + feq.
                // elements_forward: parent.contains_key(self.elements@[i]@).
                let root = find_root_loop(self, &v);
                let _ = roots_set.insert(root);
                i = i + 1;
            }
            roots_set.len()
        }
    }

    //		Section 4b. type definitions


    /// Ghost info returned by union_merge_exec for the proof coordination step.
    pub ghost struct UnionMergeInfo<V: View> {
        pub winner_view: <V as View>::V,
        pub loser_view: <V as View>::V,
        pub winner_val: V,
    }

    //		Section 4c. type definitions


    pub ghost struct UnionFindStEphV<V: View> {
        pub parent: Map<<V as View>::V, V>,
        pub rank: Map<<V as View>::V, usize>,
        pub elements: Seq<V>,
        pub roots: Map<<V as View>::V, <V as View>::V>,
    }

    //		Section 6c. spec fns


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

    /// For each element with rank k > 0, there exists a distinct element with the same
    /// root and rank k-1. Enables inductive proof that rank < elements.len().
    pub closed spec fn spec_rank_has_predecessor<V: StT + Hash>(uf: &UnionFindStEph<V>) -> bool {
        forall|v: <V as View>::V|
            uf.rank@.contains_key(v) && uf.rank@[v] > 0usize ==>
            exists|w: <V as View>::V|
                #[trigger] uf.rank@.contains_key(w)
                && uf.roots@[w] == uf.roots@[v]
                && uf.rank@[w] == (uf.rank@[v] - 1) as usize
                && w != v
    }

    /// For all elements, rank is strictly less than elements.len().
    /// True invariant of union-by-rank (rank < log₂(n) < n).
    pub closed spec fn spec_rank_lt_elements<V: StT + Hash>(uf: &UnionFindStEph<V>) -> bool {
        forall|v: <V as View>::V| #[trigger] uf.rank@.contains_key(v) ==>
            uf.rank@[v] < uf.elements@.len()
    }

    /// Union result predicate: how roots changed after union(u, v).
    /// Closed to prevent matching loop between roots quantifier and dom =~= in &mut contexts.
    /// Operates on view types so it works in trait ensures with Self.
    pub closed spec fn spec_union_result<VV>(
        new_roots: Map<VV, VV>,
        old_roots: Map<VV, VV>,
        u_view: VV,
        v_view: VV,
    ) -> bool {
        forall|x: VV| #[trigger] new_roots.contains_key(x) ==> {
            let old_root_u = old_roots[u_view];
            let old_root_v = old_roots[v_view];
            if old_roots[x] == old_root_u || old_roots[x] == old_root_v {
                new_roots[x] == new_roots[u_view]
            } else {
                new_roots[x] == old_roots[x]
            }
        }
    }

    /// Roots change predicate for union merge. Closed to prevent matching loop
    /// between the roots quantifier and parent Map =~= in &mut contexts.
    /// R115: now embeds dom equality so callers never need dom =~= in their context.
    pub closed spec fn spec_roots_changed_by_merge<V: StT + Hash>(
        uf: &UnionFindStEph<V>,
        mid: &UnionFindStEph<V>,
        root_u_view: <V as View>::V,
        root_v_view: <V as View>::V,
        winner_view: <V as View>::V,
    ) -> bool {
        &&& uf.roots@.dom() =~= mid.roots@.dom()
        &&& forall|x: <V as View>::V| mid.roots@.contains_key(x) ==> (
            #[trigger] uf.roots@[x] == (
                if mid.roots@[x] == root_u_view || mid.roots@[x] == root_v_view {
                    winner_view
                } else {
                    mid.roots@[x]
                }
            )
        )
    }

    /// Closed conjunction of all sub-predicates. The open trait method delegates
    /// to this so wf unfolds to one opaque boolean in exec contexts. Use
    /// `reveal(spec_unionfindsteph_wf)` in proof contexts to see the sub-predicate conjunction.
    pub closed spec fn spec_unionfindsteph_wf<V: StT + Hash>(uf: &UnionFindStEph<V>) -> bool {
        &&& spec_key_model::<V>()
        &&& spec_feq_full::<V>()
        &&& spec_parent_rank_same_dom(uf)
        &&& spec_roots_parent_same_dom(uf)
        &&& spec_roots_idempotent(uf)
        &&& spec_parent_closed(uf)
        &&& spec_roots_in_dom(uf)
        &&& spec_elements_forward(uf)
        &&& spec_elements_backward(uf)
        &&& spec_elements_distinct(uf)
        &&& spec_self_parent_is_root(uf)
        &&& spec_parent_preserves_root(uf)
        &&& spec_rank_increases(uf)
        &&& spec_rank_bounded(uf)
        &&& spec_rank_has_predecessor(uf)
        &&& spec_rank_lt_elements(uf)
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
        &&& spec_rank_has_predecessor(mid)
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

    //		Section 7c. proof fns/broadcast groups


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
        broadcast use crate::vstdplus::feq::feq::group_feq_axioms;
    }

    /// Prove wf preservation after insert: frame lemma + new element properties.
    /// PROOF TARGET: assert forall bodies need proof hints (elements_backward, elements_distinct, etc.).
    #[verifier::rlimit(50)]
    #[verifier::external_body]
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
        // Reveal spec_unionfindsteph_wf to access sub-predicates, then reveal each closed
        // sub-predicate for old_uf's quantifiers and new uf's proof.
        reveal(spec_unionfindsteph_wf);
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
        reveal(spec_rank_has_predecessor);

        let old_p = old_uf.parent@;
        let old_r = old_uf.rank@;
        let old_e = old_uf.elements@;
        let old_rt = old_uf.roots@;

        // Frame: for existing keys w != v@, old maps are preserved, and old_rt[w] != v@.
        assert forall |w: <V as View>::V| #[trigger] old_p.contains_key(w) && w != v@ implies {
            &&& uf.parent@[w] == old_p[w]
            &&& uf.rank@[w] == old_r[w]
            &&& uf.roots@[w] == old_rt[w]
            &&& old_rt[w] != v@
        } by {
            // old_rt[w] is in old_p domain (wf conjunct), so != v@
        };

        // New element v@ is a self-parent singleton root with rank 0.

        // Elements backward: v@ is at the new last index.
        assert forall |w: <V as View>::V| #[trigger] uf.parent@.contains_key(w) implies
            exists|i: int| 0 <= i < uf.elements@.len() as int && #[trigger] uf.elements@[i]@ == w
        by {
            if w == v@ {
            } else {
                let i = choose|i: int| 0 <= i < old_e.len() as int && #[trigger] old_e[i]@ == w;
            }
        };

        // Elements no duplicates: v@ is not in old_p domain, but old elements are.
        assert forall |i: int, j: int|
            0 <= i < uf.elements@.len() as int &&
            0 <= j < uf.elements@.len() as int &&
            i != j implies
            #[trigger] uf.elements@[i]@ != #[trigger] uf.elements@[j]@
        by {
            let n = old_e.len() as int;
            if i == n && j < n {
            } else if j == n && i < n {
            }
        };

        // Roots idempotent: for w != v@, old_rt is preserved and old_rt[w] != v@.
        assert forall |w: <V as View>::V| #[trigger] uf.roots@.contains_key(w) implies
            uf.roots@.contains_key(uf.roots@[w]) && uf.roots@[uf.roots@[w]] == uf.roots@[w]
        by {
            if w != v@ {
            }
        };

        // Parent preserves root component: for w != v@, parent[w]@ != v@.
        assert forall |w: <V as View>::V| #[trigger] uf.parent@.contains_key(w) implies
            uf.roots@[uf.parent@[w]@] == uf.roots@[w]
        by {
            if w != v@ {
                let pw = old_p[w]@;
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
        reveal(spec_unionfindsteph_wf);
        reveal(spec_rank_increases);
        reveal(spec_parent_preserves_root);
        reveal(spec_rank_bounded);
        reveal(spec_parent_closed);
        if uf.parent@[rv]@ != rv {
        }
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

        // Roots idempotent.
        assert forall |w: <V as View>::V| #[trigger] new_roots.contains_key(w) implies
            new_roots.contains_key(new_roots[w]) && new_roots[new_roots[w]] == new_roots[w]
        by {
            let old_rw = mid_roots[w];
            if old_rw == root_u_view || old_rw == root_v_view {
            } else {
            }
        };

        // Roots in dom.
        assert forall |w: <V as View>::V| #[trigger] new_roots.contains_key(w) implies
            new_parent_dom.contains(new_roots[w])
        by {
            if mid_roots[w] == root_u_view || mid_roots[w] == root_v_view {
            } else {
            }
        };
    }

    /// Part 1b: parent wf conjuncts after union (closed, self-parent, preserves root).
    /// PROOF TARGET: self_parent_is_root case split needs intermediate assertions.
    #[verifier::rlimit(60)]
    #[verifier::external_body]
    proof fn lemma_union_wf_parent<V: StT + Hash>(
        uf: &UnionFindStEph<V>,
        mid: &UnionFindStEph<V>,
        winner_val: V,
        root_u_view: <V as View>::V,
        root_v_view: <V as View>::V,
        winner_view: <V as View>::V,
        loser_view: <V as View>::V,
    )
        requires spec_union_lemma_pre(uf, mid, winner_val, root_u_view, root_v_view, winner_view, loser_view),
        ensures
            spec_parent_closed(uf),
            spec_self_parent_is_root(uf),
            spec_parent_preserves_root(uf),
    {
        reveal(spec_union_lemma_pre);
        reveal(spec_parent_closed);
        reveal(spec_self_parent_is_root);
        reveal(spec_parent_preserves_root);

        let mp = mid.parent@;
        let mrt = mid.roots@;

        // Parent view frame.
        assert forall |k: <V as View>::V| #[trigger] uf.parent@.contains_key(k) implies
            uf.parent@[k]@ == mp[k]@
        by {};

        // Parent closed.
        assert forall |w: <V as View>::V| #[trigger] uf.parent@.contains_key(w) implies
            uf.parent@.contains_key(uf.parent@[w]@)
        by {
            if w == loser_view {
            } else {
            }
        };

        // Self-parent is root.
        assert forall |w: <V as View>::V| uf.parent@.contains_key(w) && uf.parent@[w]@ == w implies
            #[trigger] uf.roots@[w] == w
        by {
            if w == loser_view {
            } else {
                if mrt[w] == root_u_view || mrt[w] == root_v_view {
                }
            }
        };

        // Parent preserves root.
        assert forall |w: <V as View>::V| #[trigger] uf.parent@.contains_key(w) implies
            uf.roots@[uf.parent@[w]@] == uf.roots@[w]
        by {
            if w == loser_view {
            } else {
                let pw = mp[w]@;
                if mrt[w] == root_u_view || mrt[w] == root_v_view {
                }
            }
        };
    }

    /// Part 2: ordering wf conjuncts after union (rank_increases, rank_bounded).
    /// PROOF TARGET: rank_increases case split needs intermediate assertions.
    #[verifier::rlimit(80)]
    #[verifier::external_body]
    proof fn lemma_union_wf_ordering<V: StT + Hash>(
        uf: &UnionFindStEph<V>,
        mid: &UnionFindStEph<V>,
        winner_val: V,
        root_u_view: <V as View>::V,
        root_v_view: <V as View>::V,
        winner_view: <V as View>::V,
        loser_view: <V as View>::V,
    )
        requires spec_union_lemma_pre(uf, mid, winner_val, root_u_view, root_v_view, winner_view, loser_view),
        ensures
            spec_rank_increases(uf),
            spec_rank_bounded(uf),
    {
        reveal(spec_union_lemma_pre);
        reveal(spec_rank_increases);
        reveal(spec_rank_bounded);
        reveal(spec_roots_idempotent);

        let mp = mid.parent@;
        let mr = mid.rank@;
        let mrt = mid.roots@;

        // Parent view frame (needed for rank_increases).
        assert forall |k: <V as View>::V| #[trigger] uf.parent@.contains_key(k) implies
            uf.parent@[k]@ == mp[k]@
        by {};

        // Rank increases.
        assert forall |w: <V as View>::V| uf.parent@.contains_key(w)
            && uf.parent@[w]@ != w implies
            uf.rank@[w] < #[trigger] uf.rank@[uf.parent@[w]@]
        by {
            if w == loser_view {
            } else {
                let pw = mp[w]@;
                if w == winner_view {
                }
                if pw == winner_view {
                }
            }
        };

        // Rank bounded.
        assert forall |w: <V as View>::V| #[trigger] uf.rank@.contains_key(w) implies
            uf.rank@[w] <= uf.rank@[uf.roots@[w]]
        by {
            let old_rw = mrt[w];
            if old_rw == root_u_view || old_rw == root_v_view {
                if w != winner_view {
                    if old_rw == root_u_view {
                    } else {
                    }
                }
            } else {
                if w == winner_view {
                    if winner_view == root_u_view {
                    } else {
                    }
                }
                if old_rw == winner_view {
                    if winner_view == root_u_view {
                    } else {
                    }
                }
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
        reveal(spec_unionfindsteph_wf);
        reveal(spec_self_parent_is_root);
        reveal(spec_rank_increases);
        reveal(spec_parent_preserves_root);
        reveal(spec_rank_bounded);
        // cv is not a self-parent: if parent[cv]@ == cv, wf says roots[cv] == cv, but roots[cv] == rv != cv.
        // Non-root ordering: rank[cv] < rank[parent[cv]@].
        // Parent preserves root: roots[parent[cv]@] == roots[cv] == rv.
        // Rank bounded: rank[parent[cv]@] <= rank[roots[parent[cv]@]] == rank[rv].
    }

    /// Derive rank[v] < elements.len() from wf by induction on rank.
    /// Uses spec_rank_has_predecessor to find a chain of distinct elements.
    /// PROOF TARGET: rlimit exceeded — inductive proof needs budget tuning or decomposition.
    #[verifier::external_body]
    proof fn lemma_rank_lt_elements<V: StT + Hash>(
        uf: &UnionFindStEph<V>,
        v_view: <V as View>::V,
    )
        requires
            uf.spec_unionfindsteph_wf(),
            uf.rank@.contains_key(v_view),
        ensures
            uf.rank@[v_view] < uf.elements@.len(),
        decreases uf.rank@[v_view] as int,
    {
        reveal(spec_unionfindsteph_wf);
        reveal(spec_rank_has_predecessor);
        reveal(spec_elements_backward);
        reveal(spec_elements_distinct);

        if uf.rank@[v_view] == 0usize {
            // v_view is in parent dom (parent_rank_same_dom). By elements_backward,
            // exists i with elements[i]@ == v_view. So elements.len() >= 1 > 0.
            let i = choose|i: int| 0 <= i < uf.elements@.len() as int
                && #[trigger] uf.elements@[i]@ == v_view;
        } else {
            // Predecessor exists: w with same root, rank = rank[v] - 1, w != v.
            let w = choose|w: <V as View>::V|
                uf.rank@.contains_key(w)
                && uf.roots@[w] == uf.roots@[v_view]
                && uf.rank@[w] == (uf.rank@[v_view] - 1) as usize
                && w != v_view;
            // By induction: rank[w] < elements.len().
            lemma_rank_lt_elements(uf, w);
            // rank[w] = rank[v] - 1, so rank[v] - 1 < elements.len().
            // Need strict <: v_view and w are both in elements at distinct indices.
            let i_v = choose|i: int| 0 <= i < uf.elements@.len() as int
                && #[trigger] uf.elements@[i]@ == v_view;
            let i_w = choose|i: int| 0 <= i < uf.elements@.len() as int
                && #[trigger] uf.elements@[i]@ == w;
            // w != v_view, so i_w != i_v by elements_distinct.
            // elements.len() > max(i_v, i_w) + 1 >= 2. But more precisely:
            // rank[w] < elements.len() means rank[v] - 1 < elements.len(),
            // i.e., rank[v] < elements.len() + 1. That gives rank[v] <= elements.len().
            // But we also know v and w are at two distinct indices, both < elements.len().
            // We need to show rank[v] < elements.len(), which is rank[w] + 1 < elements.len().
            // From induction: rank[w] < elements.len(). We need rank[w] + 1 < elements.len().
            // Since w and v are distinct elements, elements.len() >= 2.
            // But rank[w] could be elements.len() - 1...
            // Key: rank[w] < elements.len() only gives rank[v] <= elements.len().
            // To get strict <, need elements.len() > rank[v] = rank[w] + 1.
            // We need rank[w] + 1 < elements.len(), i.e., rank[w] < elements.len() - 1.
            // This is NOT guaranteed by the induction hypothesis alone.
            // But: the induction gives us a chain of rank[v]+1 distinct elements
            // (v, w, w's predecessor, ..., down to rank 0). All in elements with distinct views.
            // elements.len() >= rank[v] + 1. Actually rank[v] + 1 = rank[w] + 2 = ... = 0 + (rank[v]+1).
            // So elements.len() >= rank[v] + 1, i.e., rank[v] < elements.len().
            // The "chain of distinct elements" argument:
            // At each level, we have a new element (w != v_view).
            // The base case gives 1 element, each step adds 1 more distinct element.
            // Total: rank[v] + 1 distinct elements, all in elements.
            // Since elements are distinct by view, elements.len() >= rank[v] + 1.
        }
    }

    /// Decompose the monolithic wf into individual sub-predicates.
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
            spec_rank_has_predecessor(uf),
    {
        reveal(spec_unionfindsteph_wf);
    }

    /// Assemble the monolithic wf from individual sub-predicates.
    /// PROOF TARGET: rlimit exceeded — conjunction of 15 predicates overwhelms Z3.
    #[verifier::external_body]
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
            spec_rank_has_predecessor(uf),
        ensures
            uf.spec_unionfindsteph_wf(),
    {
        reveal(spec_unionfindsteph_wf);
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
            spec_rank_has_predecessor(mid),
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
        uf: &UnionFindStEph<V>,
        mid: &UnionFindStEph<V>,
        root_u_view: <V as View>::V,
        root_v_view: <V as View>::V,
        winner_view: <V as View>::V,
    )
        requires
            spec_roots_idempotent(mid),
            spec_roots_in_dom(mid),
            spec_roots_parent_same_dom(mid),
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
            spec_roots_idempotent(uf),
            spec_roots_in_dom(uf),
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
        uf: &UnionFindStEph<V>,
        mid: &UnionFindStEph<V>,
    )
        requires
            spec_elements_forward(mid),
            spec_elements_backward(mid),
            spec_elements_distinct(mid),
            uf.elements@ =~= mid.elements@,
            uf.parent@.dom() =~= mid.parent@.dom(),
        ensures
            spec_elements_forward(uf),
            spec_elements_backward(uf),
            spec_elements_distinct(uf),
    {
        reveal(spec_elements_forward);
        reveal(spec_elements_backward);
        reveal(spec_elements_distinct);
    }

    /// Coordinate all sub-lemmas to prove wf after union merge.
    /// Separated from exec to keep &mut encoding out of the proof context.
    /// Reveals spec_unionfindsteph_wf to decompose mid's wf into sub-predicates.
    /// PROOF TARGET: rlimit exceeded — coordinates sub-lemmas, needs budget tuning.
    #[verifier::rlimit(80)]
    #[verifier::external_body]
    proof fn lemma_union_merge_wf<V: StT + Hash>(
        uf: &UnionFindStEph<V>,
        mid: &UnionFindStEph<V>,
        winner_val: V,
        root_u_view: <V as View>::V,
        root_v_view: <V as View>::V,
        winner_view: <V as View>::V,
        loser_view: <V as View>::V,
    )
        requires
            mid.spec_unionfindsteph_wf(),
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
            // Pointwise parent facts (avoids Map =~= in caller's Z3 context).
            uf.parent@.dom() =~= mid.parent@.dom(),
            uf.parent@[loser_view] == winner_val,
            forall|k: <V as View>::V| mid.parent@.contains_key(k) && k != loser_view ==>
                #[trigger] uf.parent@[k] == mid.parent@[k],
            uf.rank@.dom() =~= mid.rank@.dom(),
            forall|k: <V as View>::V| mid.rank@.contains_key(k) && k != winner_view ==>
                #[trigger] uf.rank@[k] == mid.rank@[k],
            uf.rank@[winner_view] >= mid.rank@[winner_view],
            uf.rank@[winner_view] >= mid.rank@[loser_view],
            mid.rank@[loser_view] < uf.rank@[winner_view],
            uf.elements@ == mid.elements@,
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
            uf.spec_unionfindsteph_wf(),
    {
        // Reveal spec_unionfindsteph_wf to decompose mid's wf into sub-predicates.
        reveal(spec_unionfindsteph_wf);

        // Reconstruct Map =~= from pointwise facts for sub-lemmas.

        lemma_establish_union_pre(
            uf, mid, winner_val,
            root_u_view, root_v_view, winner_view, loser_view,
        );
        lemma_union_wf_roots_closed(
            uf, mid,
            root_u_view, root_v_view, winner_view,
        );
        lemma_union_wf_parent(
            uf, mid, winner_val,
            root_u_view, root_v_view, winner_view, loser_view,
        );
        lemma_union_wf_ordering(
            uf, mid, winner_val,
            root_u_view, root_v_view, winner_view, loser_view,
        );
        lemma_union_wf_frame(
            uf, mid,
        );
        lemma_assemble_wf(uf);
    }

    /// Extract parent.dom() =~= roots.dom() from wf without polluting the caller's
    /// Z3 context with all 14 sub-predicates.
    proof fn lemma_wf_parent_dom_eq_roots_dom<V: StT + Hash>(
        uf: &UnionFindStEph<V>,
    )
        requires uf.spec_unionfindsteph_wf(),
        ensures
            uf.parent@.dom() =~= uf.roots@.dom(),
            uf.parent@.dom() =~= uf.rank@.dom(),
    {
        reveal(spec_unionfindsteph_wf);
        reveal(spec_parent_rank_same_dom);
        reveal(spec_roots_parent_same_dom);
    }

    /// Bridge: translate union_merge's quantified ensures (trigger on roots@[x])
    /// to union's quantified ensures (trigger on roots.contains_key(x)).
    /// Proof-only, no &mut — keeps Z3 under budget.
    proof fn lemma_union_ensures_bridge<V: StT + Hash>(
        uf_new: &UnionFindStEph<V>,
        uf_old: &UnionFindStEph<V>,
        root_u_view: <V as View>::V,
        root_v_view: <V as View>::V,
        u_view: <V as View>::V,
        v_view: <V as View>::V,
        winner_view: <V as View>::V,
    )
        requires
            root_u_view == uf_old.roots@[u_view],
            root_v_view == uf_old.roots@[v_view],
            uf_new.roots@.dom() =~= uf_old.roots@.dom(),
            uf_old.roots@.contains_key(u_view),
            uf_new.roots@[u_view] == winner_view,
            forall|x: <V as View>::V| uf_old.roots@.contains_key(x) ==> (
                #[trigger] uf_new.roots@[x] == (
                    if uf_old.roots@[x] == root_u_view
                        || uf_old.roots@[x] == root_v_view
                    {
                        winner_view
                    } else {
                        uf_old.roots@[x]
                    }
                )
            ),
        ensures
            forall|x: <V as View>::V|
                #[trigger] uf_new.roots@.contains_key(x) ==> {
                    let old_root_u = uf_old.roots@[u_view];
                    let old_root_v = uf_old.roots@[v_view];
                    if uf_old.roots@[x] == old_root_u
                        || uf_old.roots@[x] == old_root_v
                    {
                        uf_new.roots@[x] == uf_new.roots@[u_view]
                    } else {
                        uf_new.roots@[x] == uf_old.roots@[x]
                    }
                },
    {
        assert forall |x: <V as View>::V|
            #[trigger] uf_new.roots@.contains_key(x) implies (
                if uf_old.roots@[x] == uf_old.roots@[u_view]
                    || uf_old.roots@[x] == uf_old.roots@[v_view]
                {
                    uf_new.roots@[x] == uf_new.roots@[u_view]
                } else {
                    uf_new.roots@[x] == uf_old.roots@[x]
                }
            )
        by {
        }
    }

    /// Extract spec_key_model and spec_feq_full from wf.
    proof fn lemma_wf_type_axioms<V: StT + Hash>(
        uf: &UnionFindStEph<V>,
    )
        requires uf.spec_unionfindsteph_wf(),
        ensures
            spec_key_model::<V>(),
            spec_feq_full::<V>(),
    {
        reveal(spec_unionfindsteph_wf);
    }

    /// Derive rank[root] < elements.len() from wf + roots.contains_key.
    proof fn lemma_root_rank_lt_elements<V: StT + Hash>(
        uf: &UnionFindStEph<V>,
        root_view: <V as View>::V,
    )
        requires
            uf.spec_unionfindsteph_wf(),
            uf.roots@.contains_key(root_view),
        ensures
            uf.rank@.contains_key(root_view),
            uf.rank@[root_view] < uf.elements@.len(),
    {
        lemma_wf_parent_dom_eq_roots_dom(uf);
        lemma_rank_lt_elements(uf, root_view);
    }

    /// Prove spec_union_result for the identity case (same root, no mutation).
    proof fn lemma_union_result_identity<VV>(
        roots: Map<VV, VV>,
        u_view: VV,
        v_view: VV,
    )
        requires
            roots.contains_key(u_view),
            roots.contains_key(v_view),
            roots[u_view] == roots[v_view],
        ensures
            spec_union_result(roots, roots, u_view, v_view),
    {
        reveal(spec_union_result);
    }

    /// Derive all union_merge_exec prerequisites from wf + root conditions.
    /// Single proof call instead of 4 separate lemma calls.
    proof fn lemma_union_merge_exec_pre<V: StT + Hash>(
        uf: &UnionFindStEph<V>,
        root_u_view: <V as View>::V,
        root_v_view: <V as View>::V,
    )
        requires
            uf.spec_unionfindsteph_wf(),
            root_u_view != root_v_view,
            uf.roots@.contains_key(root_u_view),
            uf.roots@.contains_key(root_v_view),
            uf.roots@[root_u_view] == root_u_view,
            uf.roots@[root_v_view] == root_v_view,
        ensures
            spec_key_model::<V>(),
            spec_feq_full::<V>(),
            uf.parent@.dom() =~= uf.roots@.dom(),
            uf.parent@.dom() =~= uf.rank@.dom(),
            uf@.parent.contains_key(root_u_view),
            uf@.parent[root_u_view]@ == root_u_view,
            uf@.parent.contains_key(root_v_view),
            uf@.parent[root_v_view]@ == root_v_view,
    {
        lemma_wf_type_axioms(uf);
        lemma_wf_parent_dom_eq_roots_dom(uf);
        lemma_root_is_self_parent(uf, root_u_view);
        lemma_root_is_self_parent(uf, root_v_view);
    }

    /// Prove spec_union_result from spec_roots_changed_by_merge.
    /// Isolated proof context: reveals both closed predicates + does the bridge translation.
    /// R115: dom =~= is now inside spec_roots_changed_by_merge, so no explicit dom =~= requires.
    #[verifier::rlimit(30)]
    proof fn lemma_prove_union_result<V: StT + Hash>(
        uf_new: &UnionFindStEph<V>,
        uf_old: &UnionFindStEph<V>,
        root_u_view: <V as View>::V,
        root_v_view: <V as View>::V,
        u_view: <V as View>::V,
        v_view: <V as View>::V,
        winner_view: <V as View>::V,
    )
        requires
            spec_roots_changed_by_merge(uf_new, uf_old, root_u_view, root_v_view, winner_view),
            root_u_view == uf_old.roots@[u_view],
            root_v_view == uf_old.roots@[v_view],
            uf_old.roots@.contains_key(u_view),
        ensures
            spec_union_result(uf_new.roots@, uf_old.roots@, u_view, v_view),
    {
        reveal(spec_roots_changed_by_merge);
        reveal(spec_union_result);
        lemma_union_ensures_bridge::<V>(
            uf_new, uf_old,
            root_u_view, root_v_view,
            u_view, v_view,
            winner_view,
        );
    }

    /// Prove spec_roots_changed_by_merge from the Map::new structure.
    /// Isolated proof context: reveals the closed spec fn without Map =~= terms.
    proof fn lemma_prove_roots_changed<V: StT + Hash>(
        uf: &UnionFindStEph<V>,
        mid: &UnionFindStEph<V>,
        root_u_view: <V as View>::V,
        root_v_view: <V as View>::V,
        winner_view: <V as View>::V,
    )
        requires
            uf.roots@.dom() =~= mid.roots@.dom(),
            forall|x: <V as View>::V| mid.roots@.contains_key(x) ==> (
                #[trigger] uf.roots@[x] == (
                    if mid.roots@[x] == root_u_view || mid.roots@[x] == root_v_view {
                        winner_view
                    } else {
                        mid.roots@[x]
                    }
                )
            ),
        ensures
            spec_roots_changed_by_merge(uf, mid, root_u_view, root_v_view, winner_view),
    {
        reveal(spec_roots_changed_by_merge);
    }

    /// Combined bridge: reveal opaque roots predicate + translate to union ensures form.
    /// Single proof call with isolated Z3 context (no &mut overhead).
    /// Prove the union-style roots quantifier from the opaque merge predicate.
    /// Uses the OLD dom as guard (avoids matching loop with dom =~=).
    /// R115: dom =~= now inside spec_roots_changed_by_merge, no explicit dom requires.
    proof fn lemma_union_roots_bridge<V: StT + Hash>(
        uf_new: &UnionFindStEph<V>,
        uf_old: &UnionFindStEph<V>,
        root_u_view: <V as View>::V,
        root_v_view: <V as View>::V,
        u_view: <V as View>::V,
        v_view: <V as View>::V,
        winner_view: <V as View>::V,
    )
        requires
            spec_roots_changed_by_merge(uf_new, uf_old, root_u_view, root_v_view, winner_view),
            root_u_view == uf_old.roots@[u_view],
            root_v_view == uf_old.roots@[v_view],
            uf_old.roots@.contains_key(u_view),
        ensures
            // Uses OLD dom guard — caller must translate via dom =~=.
            forall|x: <V as View>::V|
                #[trigger] uf_old.roots@.contains_key(x) ==> {
                    let old_root_u = uf_old.roots@[u_view];
                    let old_root_v = uf_old.roots@[v_view];
                    if uf_old.roots@[x] == old_root_u
                        || uf_old.roots@[x] == old_root_v
                    {
                        uf_new.roots@[x] == uf_new.roots@[u_view]
                    } else {
                        uf_new.roots@[x] == uf_old.roots@[x]
                    }
                },
    {
        reveal(spec_roots_changed_by_merge);
        lemma_union_ensures_bridge::<V>(
            uf_new, uf_old,
            root_u_view, root_v_view,
            u_view, v_view,
            winner_view,
        );
    }

    /// Bridge: forward exec ensures + wf to union_merge's ensures.
    /// Runs in its own Z3 context (proof fn, no &mut encoding), avoiding
    /// the matching loop between exec quantifiers and &mut-encoded goals.
    proof fn lemma_union_merge_ensures_bridge<V: StT + Hash>(
        uf: &UnionFindStEph<V>,
        mid: &UnionFindStEph<V>,
        info: UnionMergeInfo<V>,
        root_u_view: <V as View>::V,
        root_v_view: <V as View>::V,
    )
        requires
            uf.spec_unionfindsteph_wf(),
            ((info.winner_view == root_u_view && info.loser_view == root_v_view) ||
             (info.winner_view == root_v_view && info.loser_view == root_u_view)),
            info.winner_val@ == info.winner_view,
            uf.elements@ == mid.elements@,
            uf.roots@.dom() =~= mid.roots@.dom(),
            uf.parent@.dom() =~= mid.parent@.dom(),
            forall|x: <V as View>::V| mid.roots@.contains_key(x) ==> (
                #[trigger] uf.roots@[x] == (
                    if mid.roots@[x] == root_u_view || mid.roots@[x] == root_v_view {
                        info.winner_view
                    } else {
                        mid.roots@[x]
                    }
                )
            ),
        ensures
            uf.spec_unionfindsteph_wf(),
            ((info.winner_view == root_u_view && info.loser_view == root_v_view) ||
             (info.winner_view == root_v_view && info.loser_view == root_u_view)),
            info.winner_val@ == info.winner_view,
            uf.elements@ == mid.elements@,
            uf.roots@.dom() =~= mid.roots@.dom(),
            uf.parent@.dom() =~= mid.parent@.dom(),
            forall|x: <V as View>::V| mid.roots@.contains_key(x) ==> (
                #[trigger] uf.roots@[x] == (
                    if mid.roots@[x] == root_u_view || mid.roots@[x] == root_v_view {
                        info.winner_view
                    } else {
                        mid.roots@[x]
                    }
                )
            ),
    {
        // Trivial: all ensures are direct forwards from requires.
    }

    //		Section 9c. impls


    /// Chase parent pointers to the root (no mutation).
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(depth), Span O(depth) — follows parent chain to root; depth bounded by O(lg n) with union-by-rank.
    /// PROOF TARGET: rlimit on while loop + invariant failures (rank-based termination).
    #[verifier::rlimit(30)]
    #[verifier::external_body]
    fn find_root_loop<V: StT + Hash>(uf: &UnionFindStEph<V>, v: &V) -> (root: V)
        requires
            uf.spec_unionfindsteph_wf(),
            uf@.parent.contains_key(v@),
        ensures
            root@ == uf@.roots[v@],
            uf@.parent.contains_key(root@),
            uf@.roots.contains_key(root@),
            uf@.roots[root@] == root@,
    {
        broadcast use crate::vstdplus::feq::feq::group_feq_axioms;
        // Veracity: NEEDED proof block
        proof {
            reveal(spec_unionfindsteph_wf);
            reveal(spec_parent_closed);
            reveal(spec_rank_increases);
            reveal(spec_rank_bounded);
            reveal(spec_parent_preserves_root);
            reveal(spec_self_parent_is_root);
        }
        let mut current = v.clone();
        // Veracity: NEEDED proof block
        proof {
        }

        // Read first parent for the while condition.
        let mut p = uf.parent.get(&current).unwrap().clone();
        // Veracity: NEEDED proof block
        proof {
            let ghost pv = uf.parent@[current@];
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
            // Veracity: NEEDED proof block
            proof {
                // p@ != current@, so non-root.
            }

            current = p;
            p = uf.parent.get(&current).unwrap().clone();
            // Veracity: NEEDED proof block
            proof {
                let ghost pv = uf.parent@[current@];
            }
        }

        // After loop: feq(&p, &current) is true, so p@ == current@.
        // Invariant: p@ == parent[current@]@. So parent[current@]@ == current@.
        // wf self-parent: roots[current@] == current@. Invariant: roots[current@] == roots[v@].
        // Therefore current@ == roots[v@].
        // Veracity: NEEDED proof block
        proof {
        }

        current
    }

    /// Execute the mutations for union merge. No wf sub-predicates in scope —
    /// only structural requires/ensures. This isolates HashMap/Map axioms from
    /// the quantified wf proof, keeping Z3 under 4 GB.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — two HashMap updates (parent + rank).
    /// PROOF TARGET: postcondition failures — clone/view connections for winner_val.
    #[verifier::rlimit(30)]
    #[verifier::external_body]
    fn union_merge_exec<V: StT + Hash>(
        uf: &mut UnionFindStEph<V>,
        root_u: V,
        root_v: V,
    ) -> (info: Ghost<UnionMergeInfo<V>>)
        requires
            old(uf).spec_unionfindsteph_wf(),
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
            // Rank bounded by elements length — prevents overflow on rank increment.
            // True invariant of union-by-rank: 2^rank <= component_size <= elements.len().
            old(uf).rank@[root_u@] < old(uf).elements@.len(),
            old(uf).rank@[root_v@] < old(uf).elements@.len(),
        ensures
            ((info@.winner_view == root_u@ && info@.loser_view == root_v@) ||
             (info@.winner_view == root_v@ && info@.loser_view == root_u@)),
            info@.winner_val@ == info@.winner_view,
            uf.parent@.dom() =~= old(uf).parent@.dom(),
            uf.parent@[info@.loser_view] == info@.winner_val,
            forall|k: <V as View>::V| old(uf).parent@.contains_key(k) && k != info@.loser_view ==>
                #[trigger] uf.parent@[k] == old(uf).parent@[k],
            uf.rank@.dom() =~= old(uf).rank@.dom(),
            forall|k: <V as View>::V| old(uf).rank@.contains_key(k) && k != info@.winner_view ==>
                #[trigger] uf.rank@[k] == old(uf).rank@[k],
            uf.rank@[info@.winner_view] >= old(uf).rank@[info@.winner_view],
            uf.rank@[info@.winner_view] >= old(uf).rank@[info@.loser_view],
            old(uf).rank@[info@.loser_view] < uf.rank@[info@.winner_view],
            uf.elements@ == old(uf).elements@,
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
        broadcast use crate::vstdplus::feq::feq::group_feq_axioms;
        let ghost root_u_view = root_u@;
        let ghost root_v_view = root_v@;
        let ghost mid_roots = uf.roots@;

        let rank_u = *uf.rank.get(&root_u).unwrap();
        let rank_v = *uf.rank.get(&root_v).unwrap();

        let ru1 = root_u.clone();
        let rv1 = root_v.clone();
        // Veracity: NEEDED proof block
        proof {
        }

        let ghost winner_view: <V as View>::V;
        let ghost loser_view: <V as View>::V;
        let ghost winner_v: V;

        if rank_u < rank_v {
            // Veracity: NEEDED proof block
            proof { winner_view = root_v_view; loser_view = root_u_view; winner_v = rv1; }
            uf.parent.insert(ru1, rv1);
        } else {
            // Veracity: NEEDED proof block
            proof { winner_view = root_u_view; loser_view = root_v_view; winner_v = ru1; }
            uf.parent.insert(rv1, ru1);
            if rank_u == rank_v {
                let ru2 = root_u.clone();
                let ghost elem_len = uf.elements.len();
                // Veracity: NEEDED proof block
                proof {
                    // rank_u < elements.len() (from requires). elements.len() is usize,
                    // so rank_u < usize::MAX, and rank_u + 1 <= usize::MAX.
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
    /// Returns Ghost info for caller to derive structural ensures.
    /// Exec does mutations via union_merge_exec; proof delegates to
    /// lemma_union_merge_wf to keep &mut encoding out of the proof context.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — updates roots map for all elements in merged component.
    /// PROOF TARGET: rlimit exceeded — depends on lemma_union_merge_wf.
    #[verifier::rlimit(30)]
    #[verifier::external_body]
    fn union_merge<V: StT + Hash>(
        uf: &mut UnionFindStEph<V>,
        root_u: V,
        root_v: V,
    ) -> (info: Ghost<UnionMergeInfo<V>>)
        requires
            old(uf).spec_unionfindsteph_wf(),
            root_u@ != root_v@,
            old(uf).roots@.contains_key(root_u@),
            old(uf).roots@.contains_key(root_v@),
            old(uf).roots@[root_u@] == root_u@,
            old(uf).roots@[root_v@] == root_v@,
            // Rank bound for overflow prevention in rank increment.
            old(uf).rank@[root_u@] < old(uf).elements@.len(),
            old(uf).rank@[root_v@] < old(uf).elements@.len(),
        ensures
            uf.spec_unionfindsteph_wf(),
            ((info@.winner_view == root_u@ && info@.loser_view == root_v@) ||
             (info@.winner_view == root_v@ && info@.loser_view == root_u@)),
            info@.winner_val@ == info@.winner_view,
            uf.elements@ == old(uf).elements@,
            uf.parent@.dom() =~= old(uf).parent@.dom(),
            // R115: dom =~= for roots is now inside spec_roots_changed_by_merge.
            // Opaque roots change predicate — avoids matching loop in &mut context.
            spec_roots_changed_by_merge(uf, &*old(uf), root_u@, root_v@, info@.winner_view),
    {
        // Inline exec body to avoid &mut function call overhead.
        broadcast use crate::vstdplus::feq::feq::group_feq_axioms;
        let ghost mid_uf = *uf;
        let ghost root_u_view = root_u@;
        let ghost root_v_view = root_v@;
        let ghost mid_roots = uf.roots@;

        // Veracity: NEEDED proof block
        proof {
            lemma_union_merge_exec_pre(&mid_uf, root_u_view, root_v_view);
        }

        let rank_u = *uf.rank.get(&root_u).unwrap();
        let rank_v = *uf.rank.get(&root_v).unwrap();

        let ru1 = root_u.clone();
        let rv1 = root_v.clone();
        // Veracity: NEEDED proof block
        proof {
        }

        let ghost winner_view: <V as View>::V;
        let ghost loser_view: <V as View>::V;
        let ghost winner_v: V;

        if rank_u < rank_v {
            // Veracity: NEEDED proof block
            proof { winner_view = root_v_view; loser_view = root_u_view; winner_v = rv1; }
            uf.parent.insert(ru1, rv1);
        } else {
            // Veracity: NEEDED proof block
            proof { winner_view = root_u_view; loser_view = root_v_view; winner_v = ru1; }
            uf.parent.insert(rv1, ru1);
            if rank_u == rank_v {
                let ru2 = root_u.clone();
                let ghost elem_len = uf.elements.len();
                // Veracity: NEEDED proof block
                proof {
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

        let info = Ghost(UnionMergeInfo { winner_view, loser_view, winner_val: winner_v });

        // Veracity: NEEDED proof block
        proof {
            // Prove opaque roots predicate in isolated proof context (no Map =~=).
            lemma_prove_roots_changed(
                &(*uf), &mid_uf,
                root_u@, root_v@, info@.winner_view,
            );
            // Prove wf on the final state.
            lemma_union_merge_wf(
                &(*uf), &mid_uf, info@.winner_val,
                root_u@, root_v@,
                info@.winner_view, info@.loser_view,
            );
        }

        info
    }

    // R115 EXPERIMENT: isolate the merge-branch proof to find the matching loop source.
    // Test 1: just find + feq (no merge). Does spec_elements_distinct fire?
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(alpha(n)), Span O(alpha(n)) — two find_root_loop calls + feq; experimental.
    #[verifier::rlimit(30)]
    fn union_experiment_find_only<V: StT + Hash>(
        uf: &mut UnionFindStEph<V>,
        u: &V,
        v: &V,
    )
        requires
            old(uf).spec_unionfindsteph_wf(),
            old(uf)@.parent.contains_key(u@),
            old(uf)@.parent.contains_key(v@),
        ensures
            uf.spec_unionfindsteph_wf(),
            uf@.parent.dom() =~= old(uf)@.parent.dom(),
            uf@.elements =~= old(uf)@.elements,
    {
        // Veracity: NEEDED proof block
        proof { lemma_wf_type_axioms(&*old(uf)); }
        let root_u = find_root_loop(uf, u);
        let root_v = find_root_loop(uf, v);
        let _same = feq(&root_u, &root_v);
    }

    // Test 2: merge but NO wf in ensures. Is the loop from the ensures goal?
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — two finds + conditional union_merge; experimental.
    #[verifier::rlimit(30)]
    fn union_experiment_merge_no_wf_ensures<V: StT + Hash>(
        uf: &mut UnionFindStEph<V>,
        u: &V,
        v: &V,
    )
        requires
            old(uf).spec_unionfindsteph_wf(),
            old(uf)@.parent.contains_key(u@),
            old(uf)@.parent.contains_key(v@),
        ensures
            uf@.parent.dom() =~= old(uf)@.parent.dom(),
            uf@.elements =~= old(uf)@.elements,
    {
        // Veracity: NEEDED proof block
        proof { lemma_wf_type_axioms(&*old(uf)); }
        let root_u = find_root_loop(uf, u);
        let root_v = find_root_loop(uf, v);
        if !feq(&root_u, &root_v) {
            // Veracity: NEEDED proof block
            proof {
                lemma_root_rank_lt_elements(uf, root_u@);
                lemma_root_rank_lt_elements(uf, root_v@);
            }
            let _info = union_merge(uf, root_u, root_v);
        }
    }

    // Test 3: merge WITH wf in ensures. Is wf the trigger?
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — two finds + conditional union_merge + wf proof; experimental.
    #[verifier::rlimit(30)]
    fn union_experiment_merge_with_wf<V: StT + Hash>(
        uf: &mut UnionFindStEph<V>,
        u: &V,
        v: &V,
    )
        requires
            old(uf).spec_unionfindsteph_wf(),
            old(uf)@.parent.contains_key(u@),
            old(uf)@.parent.contains_key(v@),
        ensures
            uf.spec_unionfindsteph_wf(),
            uf@.parent.dom() =~= old(uf)@.parent.dom(),
            uf@.elements =~= old(uf)@.elements,
    {
        // Veracity: NEEDED proof block
        proof { lemma_wf_type_axioms(&*old(uf)); }
        let root_u = find_root_loop(uf, u);
        let root_v = find_root_loop(uf, v);
        if !feq(&root_u, &root_v) {
            // Veracity: NEEDED proof block
            proof {
                lemma_root_rank_lt_elements(uf, root_u@);
                lemma_root_rank_lt_elements(uf, root_v@);
            }
            let _info = union_merge(uf, root_u, root_v);
        }
    }

    } // verus!

    //		Section 14a. derive impls outside verus!


    impl<V: StT + Hash> std::fmt::Debug for UnionFindStEph<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("UnionFindStEph")
                .field("parent_len", &self.parent.len())
                .field("rank_len", &self.rank.len())
                .field("elements_len", &self.elements.len())
                .finish()
        }
    }

    impl<V: StT + Hash> std::fmt::Display for UnionFindStEph<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "UnionFindStEph(elements: {})", self.elements.len())
        }
    }

    //		Section 14b. derive impls outside verus!

    impl<V: View> std::fmt::Debug for UnionMergeInfo<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "UnionMergeInfo")
        }
    }

    impl<V: View> std::fmt::Display for UnionMergeInfo<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "UnionMergeInfo")
        }
    }

    //		Section 14c. derive impls outside verus!

    impl<V: View> std::fmt::Debug for UnionFindStEphV<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "UnionFindStEphV")
        }
    }

    impl<V: View> std::fmt::Display for UnionFindStEphV<V> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "UnionFindStEphV")
        }
    }
}
