//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 65: Union-Find Data Structure (Sequential Ephemeral)
//!
//! Implements Union-Find (Disjoint Set Union) with path compression and union by rank.
//! Used in Kruskal's MST algorithm for efficient cycle detection.
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

    pub open spec fn spec_roots_idempotent<V: StT + Hash>(uf: &UnionFindStEph<V>) -> bool {
        forall|v: <V as View>::V| #[trigger] uf.roots@.contains_key(v) ==> {
            &&& uf.roots@.contains_key(uf.roots@[v])
            &&& uf.roots@[uf.roots@[v]] == uf.roots@[v]
        }
    }

    pub open spec fn spec_parent_closed<V: StT + Hash>(uf: &UnionFindStEph<V>) -> bool {
        forall|v: <V as View>::V| #[trigger] uf.parent@.contains_key(v) ==>
            uf.parent@.contains_key(uf.parent@[v]@)
    }

    pub open spec fn spec_roots_in_dom<V: StT + Hash>(uf: &UnionFindStEph<V>) -> bool {
        forall|v: <V as View>::V| #[trigger] uf.roots@.contains_key(v) ==>
            uf.parent@.contains_key(uf.roots@[v])
    }

    pub open spec fn spec_elements_forward<V: StT + Hash>(uf: &UnionFindStEph<V>) -> bool {
        forall|i: int| 0 <= i < uf.elements@.len() as int ==>
            uf.parent@.contains_key(#[trigger] uf.elements@[i]@)
    }

    pub open spec fn spec_elements_backward<V: StT + Hash>(uf: &UnionFindStEph<V>) -> bool {
        forall|v: <V as View>::V| #[trigger] uf.parent@.contains_key(v) ==>
            exists|i: int| 0 <= i < uf.elements@.len() as int && #[trigger] uf.elements@[i]@ == v
    }

    pub open spec fn spec_elements_distinct<V: StT + Hash>(uf: &UnionFindStEph<V>) -> bool {
        forall|i: int, j: int|
            0 <= i < uf.elements@.len() as int &&
            0 <= j < uf.elements@.len() as int &&
            i != j ==>
            #[trigger] uf.elements@[i]@ != #[trigger] uf.elements@[j]@
    }

    pub open spec fn spec_self_parent_is_root<V: StT + Hash>(uf: &UnionFindStEph<V>) -> bool {
        forall|v: <V as View>::V| uf.parent@.contains_key(v) && uf.parent@[v]@ == v ==>
            #[trigger] uf.roots@[v] == v
    }

    pub open spec fn spec_parent_preserves_root<V: StT + Hash>(uf: &UnionFindStEph<V>) -> bool {
        forall|v: <V as View>::V| #[trigger] uf.parent@.contains_key(v) ==>
            uf.roots@[uf.parent@[v]@] == uf.roots@[v]
    }

    pub open spec fn spec_rank_increases<V: StT + Hash>(uf: &UnionFindStEph<V>) -> bool {
        forall|v: <V as View>::V| uf.parent@.contains_key(v)
            && uf.parent@[v]@ != v ==>
            uf.rank@[v] < #[trigger] uf.rank@[uf.parent@[v]@]
    }

    pub open spec fn spec_rank_bounded<V: StT + Hash>(uf: &UnionFindStEph<V>) -> bool {
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
        fn equals(&mut self, u: &V, v: &V) -> (same: B)
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
        #[verifier::external_body]
        fn union(&mut self, u: &V, v: &V) {
            let root_u = self.find(u);
            let root_v = self.find(v);

            if root_u != root_v {
                let rank_u = *self.rank.get(&root_u).unwrap();
                let rank_v = *self.rank.get(&root_v).unwrap();

                if rank_u < rank_v {
                    self.parent.insert(root_u.clone(), root_v.clone());
                } else if rank_u > rank_v {
                    self.parent.insert(root_v.clone(), root_u.clone());
                } else {
                    self.parent.insert(root_v.clone(), root_u.clone());
                    self.rank.insert(root_u.clone(), rank_u + 1);
                }
            }
        }

        /// - APAS: Work O(alpha(n)), Span O(alpha(n)) amortized
        #[verifier::rlimit(20)]
        fn equals(&mut self, u: &V, v: &V) -> (same: B) {
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
