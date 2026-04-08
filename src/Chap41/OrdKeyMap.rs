//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Ordered key-value map backed by ParamBST<Pair<K,V>> with View = Map<K::V, V::V>.
//! Bridge layer between ParamBST's Set<(K::V,V::V)> view and Map<K::V,V::V>.

//  Table of Contents
//	Section 1. module
//	Section 2. imports
//	Section 3. broadcast use
//	Section 4. type definitions
//	Section 5. view impls
//	Section 6. spec fns
//	Section 7. proof fns/broadcast groups
//	Section 8. traits
//	Section 9. impls
//	Section 12. derive impls in verus!
//	Section 14. derive impls outside verus!

//		Section 1. module

pub mod OrdKeyMap {


    //		Section 2. imports

    use std::cmp::Ordering::{Equal, Greater, Less};
    use std::fmt::{Debug, Display, Formatter};

    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::OrdSpec;
    #[cfg(verus_keep_ghost)]
    use vstd::pervasive::cloned;
    #[cfg(verus_keep_ghost)]
    use vstd::laws_eq::obeys_view_eq;

    use crate::Chap38::BSTParaStEph::BSTParaStEph::*;
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::*;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::clone_plus::clone_plus::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;

    verus!
{

    //		Section 3. broadcast use


    broadcast use {
        crate::vstdplus::feq::feq::group_feq_axioms,
        vstd::map::group_map_axioms,
        vstd::set::group_set_axioms,
    };

    //		Section 4. type definitions


    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct OrdKeyMap<K: StT + Ord, V: StT + Ord> {
        pub inner: ParamBST<Pair<K, V>>,
    }

    //		Section 5. view impls


    impl<K: StT + Ord, V: StT + Ord> View for OrdKeyMap<K, V> {
        type V = Map<K::V, V::V>;

        open spec fn view(&self) -> Self::V { spec_pair_set_to_map(self.inner@) }
    }

    //		Section 6. spec fns


    /// Convert a set of (key, value) pairs to a map.
    /// With key uniqueness, each key maps to a unique value via `choose`.
    pub open spec fn spec_pair_set_to_map<KV, VV>(s: Set<(KV, VV)>) -> Map<KV, VV> {
        Map::new(
            |k: KV| exists|v: VV| s.contains((k, v)),
            |k: KV| choose|v: VV| s.contains((k, v)),
        )
    }

    /// Every element in the set has a Pair preimage under View.
    /// Sound: BSTs built from Pair operations only insert p@ for concrete Pair values.
    /// Subsets of View-generated sets are View-generated.
    pub open spec fn spec_set_pair_view_generated<K: View, V: View>(s: Set<(K::V, V::V)>) -> bool {
        forall|elem: (K::V, V::V)| s.contains(elem) ==> exists|p: Pair<K, V>| (#[trigger] p@) == elem
    }

    /// Key uniqueness for a set of pairs: no two pairs share the same first component.
    /// Nested quantifiers break the symmetric trigger loop that the flat form causes.
    pub open spec fn spec_key_unique_pairs_set<KV, VV>(s: Set<(KV, VV)>) -> bool {
        forall|k: KV, v: VV| #[trigger] s.contains((k, v)) ==>
            forall|v2: VV| s.contains((k, v2)) ==> v == v2
    }

    /// Pair ordering is determined by key ordering when keys differ.
    /// This is an axiom about Pair's derived lexicographic Ord.
    pub open spec fn spec_pair_key_determines_order<K: StT + Ord, V: StT + Ord>() -> bool {
        forall|p1: Pair<K, V>, p2: Pair<K, V>|
            p1.0.cmp_spec(&p2.0) != Equal ==>
            (#[trigger] p1.cmp_spec(&p2)) == p1.0.cmp_spec(&p2.0)
    }

    //		Section 7. proof fns/broadcast groups


    /// Subset of a View-generated set is View-generated.
    proof fn lemma_view_gen_subset<K: View, V: View>(
        sub: Set<(K::V, V::V)>,
        sup: Set<(K::V, V::V)>,
    )
        requires
            sub.subset_of(sup),
            spec_set_pair_view_generated::<K, V>(sup),
        ensures
            spec_set_pair_view_generated::<K, V>(sub),
    {
    }

    /// Inserting a Pair view into a View-generated set preserves View-generation.
    proof fn lemma_view_gen_insert<K: View, V: View>(
        s: Set<(K::V, V::V)>,
        pair: Pair<K, V>,
    )
        requires
            spec_set_pair_view_generated::<K, V>(s),
        ensures
            spec_set_pair_view_generated::<K, V>(s.insert(pair@)),
    {
    }

    /// Union of two View-generated sets is View-generated.
    proof fn lemma_view_gen_union<K: View, V: View>(
        a: Set<(K::V, V::V)>,
        b: Set<(K::V, V::V)>,
    )
        requires
            spec_set_pair_view_generated::<K, V>(a),
            spec_set_pair_view_generated::<K, V>(b),
        ensures
            spec_set_pair_view_generated::<K, V>(a.union(b)),
    {
    }

    /// The domain of spec_pair_set_to_map is finite when the source set is finite.
    pub proof fn lemma_pair_set_to_map_dom_finite<KV, VV>(s: Set<(KV, VV)>)
        requires s.finite()
        ensures spec_pair_set_to_map(s).dom().finite()
    {
        let dom_set = spec_pair_set_to_map(s).dom();
        let proj = |p: (KV, VV)| -> KV { p.0 };
        let proj_set = s.map(proj);
        // dom_set ⊆ proj_set.
        assert forall|k: KV| dom_set.contains(k)
            implies #[trigger] proj_set.contains(k)
        by {
            let v: VV = choose|v: VV| s.contains((k, v));
        };
        s.lemma_map_finite(proj);
        vstd::set_lib::lemma_len_subset(dom_set, proj_set);
    }

    /// The domain length equals the set length when keys are unique.
    proof fn lemma_pair_set_to_map_len<KV, VV>(s: Set<(KV, VV)>)
        requires s.finite(), spec_key_unique_pairs_set(s)
        ensures spec_pair_set_to_map(s).dom().len() == s.len()
    {
        lemma_pair_set_to_map_dom_finite(s);
        let dom_set = spec_pair_set_to_map(s).dom();
        let proj = |p: (KV, VV)| -> KV { p.0 };
        let proj_set = s.map(proj);
        // dom_set =~= proj_set.
        assert(dom_set =~= proj_set) by {
            assert forall|k: KV| dom_set.contains(k)
                implies #[trigger] proj_set.contains(k)
            by {
                let v: VV = choose|v: VV| s.contains((k, v));
            };
            assert forall|k: KV| proj_set.contains(k)
                implies #[trigger] dom_set.contains(k)
            by {
                let p: (KV, VV) = choose|p: (KV, VV)| #[trigger] s.contains(p) && p.0 == k;
                assert(s.contains((k, p.1)));
            };
        };
        // proj is injective on s when keys are unique: distinct pairs have distinct keys.
        assert(vstd::relations::injective_on(proj, s)) by {
            assert forall|x1: (KV, VV), x2: (KV, VV)|
                s.contains(x1) && s.contains(x2) && #[trigger] proj(x1) == #[trigger] proj(x2)
                implies x1 == x2
            by {
                // x1.0 == x2.0 and key_unique means x1.1 == x2.1.
            };
        };
        vstd::set_lib::lemma_map_size(s, proj_set, proj);
    }

    /// If a pair is in the set, the map contains that key with that value.
    proof fn lemma_pair_in_set_map_contains<KV, VV>(s: Set<(KV, VV)>, k: KV, v: VV)
        requires
            s.contains((k, v)),
            spec_key_unique_pairs_set(s),
        ensures
            spec_pair_set_to_map(s).contains_key(k),
            spec_pair_set_to_map(s)[k] == v,
    {
        let m = spec_pair_set_to_map(s);
        // m[k] == choose|v_| s.contains((k, v_)).
        // By key uniqueness, that chosen value must equal v.
        let v2 = choose|v2: VV| s.contains((k, v2));
    }

    /// If the map contains a key, a pair with that key exists in the set.
    proof fn lemma_map_contains_pair_in_set<KV, VV>(s: Set<(KV, VV)>, k: KV)
        requires spec_pair_set_to_map(s).contains_key(k)
        ensures exists|v: VV| s.contains((k, v))
    {
        // Follows directly from the domain definition.
    }

    /// Key uniqueness is preserved by set insert when the key is fresh.
    proof fn lemma_key_unique_insert<KV, VV>(s: Set<(KV, VV)>, k: KV, v: VV)
        requires
            spec_key_unique_pairs_set(s),
            !spec_pair_set_to_map(s).dom().contains(k),
        ensures
            spec_key_unique_pairs_set(s.insert((k, v)))
    {
        assert forall|k2: KV, v1: VV, v2: VV|
            s.insert((k, v)).contains((k2, v1)) && s.insert((k, v)).contains((k2, v2))
            implies v1 == v2
        by {
            if k2 == k {
                if s.contains((k2, v1)) {
                }
                if s.contains((k2, v2)) {
                }
                // Both must be (k, v) from the insert.
            } else {
                // Both in s (not the new element), so key uniqueness applies.
            }
        };
    }

    /// Equal-substitution for cmp_spec: Equal(a,b) implies a compares the same way as b to c.
    proof fn lemma_cmp_equal_congruent<T: StT + Ord>(a: T, b: T, c: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            a.cmp_spec(&b) == Equal,
        ensures
            a.cmp_spec(&c) == b.cmp_spec(&c),
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
    }

    /// In-order traversal keys are pairwise distinct: distinct indices have distinct keys.
    /// Follows from set-key-uniqueness and the bijection between sorted seq and tree set.
    proof fn lemma_sorted_keys_pairwise_distinct<KV, VV>(
        tree: Set<(KV, VV)>,
        sorted: Seq<(KV, VV)>,
    )
        requires
            tree.finite(),
            spec_key_unique_pairs_set(tree),
            forall|v: (KV, VV)| tree.contains(v) <==> sorted.contains(v),
            sorted.len() == tree.len(),
        ensures
            sorted.no_duplicates(),
            forall|i: int, j: int|
                0 <= i < sorted.len() && 0 <= j < sorted.len() && i != j
                ==> (#[trigger] sorted[i]).0 != (#[trigger] sorted[j]).0,
    {
        // sorted.to_set() =~= tree.
        assert(sorted.to_set() =~= tree) by {
        };
        // sorted.to_set().len() == tree.len() == sorted.len(), so no duplicates.
        sorted.lemma_no_dup_set_cardinality();
        // Pairwise distinct keys: same key + key uniqueness -> same pair -> contradicts no_duplicates.
        assert forall|i: int, j: int|
            0 <= i < sorted.len() && 0 <= j < sorted.len() && i != j
            implies (#[trigger] sorted[i]).0 != (#[trigger] sorted[j]).0
        by {
            if sorted[i].0 == sorted[j].0 {
                assert(tree.contains(sorted[i]));
                assert(tree.contains(sorted[j]));
                // Key uniqueness: same key in tree -> same value -> same pair.
                assert(sorted[i] == sorted[j]);
                // Contradicts no_duplicates.
            }
        };
    }

    /// Key uniqueness is preserved by set remove.
    proof fn lemma_key_unique_remove<KV, VV>(s: Set<(KV, VV)>, pair: (KV, VV))
        requires spec_key_unique_pairs_set(s)
        ensures spec_key_unique_pairs_set(s.remove(pair))
    {
    }

    /// Key uniqueness is preserved by subset.
    proof fn lemma_key_unique_subset<KV, VV>(s: Set<(KV, VV)>, sub: Set<(KV, VV)>)
        requires
            spec_key_unique_pairs_set(s),
            sub.subset_of(s),
        ensures
            spec_key_unique_pairs_set(sub)
    {
    }

    /// Key uniqueness holds trivially for the empty set.
    proof fn lemma_key_unique_empty<KV, VV>()
        ensures spec_key_unique_pairs_set(Set::<(KV, VV)>::empty())
    {
    }

    /// Key uniqueness for union of disjoint sets (by key) that are individually key-unique.
    proof fn lemma_key_unique_disjoint_union<KV, VV>(
        s1: Set<(KV, VV)>, s2: Set<(KV, VV)>, root: (KV, VV),
    )
        requires
            spec_key_unique_pairs_set(s1),
            spec_key_unique_pairs_set(s2),
            s1.disjoint(s2),
            !s1.contains(root),
            !s2.contains(root),
            // No key in s1 equals a key in s2 or the root key.
            forall|p1: (KV, VV), p2: (KV, VV)|
                #[trigger] s1.contains(p1) && #[trigger] s2.contains(p2) ==> p1.0 != p2.0,
            forall|p: (KV, VV)| #[trigger] s1.contains(p) ==> p.0 != root.0,
            forall|p: (KV, VV)| #[trigger] s2.contains(p) ==> p.0 != root.0,
        ensures
            spec_key_unique_pairs_set(s1.union(s2).insert(root))
    {
        let combined = s1.union(s2).insert(root);
        assert forall|k: KV, v1: VV, v2: VV|
            combined.contains((k, v1)) && combined.contains((k, v2))
            implies v1 == v2
        by {
            // Case analysis on which sets (k, v1) and (k, v2) come from.
            if k == root.0 {
                // Both must be root (by key separation from s1 and s2).
                if s1.contains((k, v1)) {
                }
                if s2.contains((k, v1)) {
                }
                if s1.contains((k, v2)) {
                }
                if s2.contains((k, v2)) {
                }
            } else {
                // (k, v1) and (k, v2) are in s1 union s2.
                // By key separation, both must be in the same set.
                if s1.contains((k, v1)) && s2.contains((k, v2)) {
                }
                if s2.contains((k, v1)) && s1.contains((k, v2)) {
                }
                // Both in s1 or both in s2 -> key uniqueness.
            }
        };
    }

    /// Map over the set after insert: extends the map with the new key-value pair.
    proof fn lemma_set_to_map_insert<KV, VV>(s: Set<(KV, VV)>, k: KV, v: VV)
        requires
            spec_key_unique_pairs_set(s),
            !spec_pair_set_to_map(s).dom().contains(k),
        ensures
            spec_pair_set_to_map(s.insert((k, v)))
                =~= spec_pair_set_to_map(s).insert(k, v),
    {
        let old_m = spec_pair_set_to_map(s);
        let new_s = s.insert((k, v));
        let new_m = spec_pair_set_to_map(new_s);
        assert forall|key: KV| #[trigger] new_m.dom().contains(key)
            implies old_m.insert(k, v).dom().contains(key)
        by {
            if key == k {
            } else {
                let vv: VV = choose|vv: VV| new_s.contains((key, vv));
            }
        };
        assert forall|key: KV| old_m.insert(k, v).dom().contains(key)
            implies #[trigger] new_m.dom().contains(key)
        by {
            if key == k {
                assert(new_s.contains((k, v)));
            } else {
                let vv: VV = choose|vv: VV| s.contains((key, vv));
                assert(new_s.contains((key, vv)));
            }
        };
        // Values agree.
        assert forall|key: KV| new_m.dom().contains(key)
            implies #[trigger] new_m[key] == old_m.insert(k, v)[key]
        by {
            if key == k {
                let cv: VV = choose|cv: VV| new_s.contains((k, cv));
                lemma_key_unique_insert(s, k, v);
            } else {
                let cv: VV = choose|cv: VV| new_s.contains((key, cv));
                let cv2: VV = choose|cv2: VV| s.contains((key, cv2));
            }
        };
    }

    /// Map over the set after remove: removes the key from the map.
    proof fn lemma_set_to_map_remove_pair<KV, VV>(s: Set<(KV, VV)>, k: KV, v: VV)
        requires
            spec_key_unique_pairs_set(s),
            s.contains((k, v)),
        ensures
            spec_pair_set_to_map(s.remove((k, v)))
                =~= spec_pair_set_to_map(s).remove(k),
    {
        let old_m = spec_pair_set_to_map(s);
        let new_s = s.remove((k, v));
        let new_m = spec_pair_set_to_map(new_s);
        // Forward: every key in new_m is in old_m.remove(k).
        assert forall|key: KV| new_m.dom().contains(key)
            implies old_m.remove(k).dom().contains(key) && #[trigger] new_m[key] == #[trigger] old_m[key]
        by {
            let vv: VV = choose|vv: VV| new_s.contains((key, vv));
            if key == k {
                // Key uniqueness: vv == v. Contradiction.
            }
            let cv: VV = choose|cv: VV| s.contains((key, cv));
        };
        // Backward: every key in old_m.remove(k) is in new_m.
        assert forall|key: KV| old_m.remove(k).dom().contains(key)
            implies #[trigger] new_m.dom().contains(key)
        by {
            let vv: VV = choose|vv: VV| s.contains((key, vv));
        };
    }

    /// The map over a union-insert equals the map over left union right union {root}.
    /// Used after expose/join_mid to relate tree view to subtree views.
    proof fn lemma_set_to_map_union_root<KV, VV>(
        left: Set<(KV, VV)>, right: Set<(KV, VV)>, root_k: KV, root_v: VV,
    )
        requires
            spec_key_unique_pairs_set(left),
            spec_key_unique_pairs_set(right),
            left.disjoint(right),
            !left.contains((root_k, root_v)),
            !right.contains((root_k, root_v)),
            forall|p: (KV, VV)| #[trigger] left.contains(p) ==> p.0 != root_k,
            forall|p: (KV, VV)| #[trigger] right.contains(p) ==> p.0 != root_k,
            forall|p1: (KV, VV), p2: (KV, VV)|
                #[trigger] left.contains(p1) && #[trigger] right.contains(p2) ==> p1.0 != p2.0,
        ensures
            ({
                let combined = left.union(right).insert((root_k, root_v));
                let combined_map = spec_pair_set_to_map(combined);
                let left_map = spec_pair_set_to_map(left);
                let right_map = spec_pair_set_to_map(right);
                &&& combined_map.dom() =~= left_map.dom().union(right_map.dom()).insert(root_k)
                &&& forall|k: KV| left_map.dom().contains(k) ==>
                    #[trigger] combined_map[k] == left_map[k]
                &&& forall|k: KV| right_map.dom().contains(k) ==>
                    #[trigger] combined_map[k] == right_map[k]
                &&& combined_map.contains_key(root_k) && combined_map[root_k] == root_v
            })
    {
        let combined = left.union(right).insert((root_k, root_v));
        let cm = spec_pair_set_to_map(combined);
        let lm = spec_pair_set_to_map(left);
        let rm = spec_pair_set_to_map(right);
        // Domain equality.
        assert(cm.dom() =~= lm.dom().union(rm.dom()).insert(root_k)) by {
            assert forall|k: KV| cm.dom().contains(k) implies
                #[trigger] lm.dom().union(rm.dom()).insert(root_k).contains(k)
            by {
                let v: VV = choose|v: VV| combined.contains((k, v));
                if k == root_k {
                } else if left.contains((k, v)) {
                } else {
                }
            };
            assert forall|k: KV| lm.dom().union(rm.dom()).insert(root_k).contains(k) implies
                #[trigger] cm.dom().contains(k)
            by {
                if k == root_k {
                    assert(combined.contains((root_k, root_v)));
                } else if lm.dom().contains(k) {
                    let v: VV = choose|v: VV| left.contains((k, v));
                    assert(combined.contains((k, v)));
                } else {
                    let v: VV = choose|v: VV| right.contains((k, v));
                    assert(combined.contains((k, v)));
                }
            };
        };
        // Root value.
        assert(cm.contains_key(root_k)) by {
        };
        let cv: VV = choose|cv: VV| combined.contains((root_k, cv));
        // Left values.
        // Right values.
    }

    /// The map over an empty set is the empty map.
    proof fn lemma_set_to_map_empty<KV, VV>()
        ensures spec_pair_set_to_map(Set::<(KV, VV)>::empty()) =~= Map::<KV, VV>::empty()
    {
    }

    /// cmp_spec antisymmetry: Less(a,b) implies Greater(b,a).
    proof fn lemma_cmp_antisymmetry<T: StT + Ord>(a: T, b: T)
        requires
            vstd::laws_cmp::obeys_cmp_spec::<T>(),
            view_ord_consistent::<T>(),
            a.cmp_spec(&b) == Less,
        ensures
            b.cmp_spec(&a) == Greater,
    {
        reveal(vstd::laws_cmp::obeys_cmp_ord);
        reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
    }

    //		Section 8. traits


    /// Trait defining ordered key-value map operations backed by ParamBST<Pair<K,V>>.
    pub trait OrdKeyMapTrait<K: StT + Ord, V: StT + Ord>: Sized + View<V = Map<K::V, V::V>> {
        spec fn spec_ordkeymap_wf(&self) -> bool;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> (empty: Self)
            requires
                obeys_feq_fulls::<K, V>(),
                obeys_feq_full::<Pair<K, V>>(),
                vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                view_ord_consistent::<Pair<K, V>>(),
                spec_pair_key_determines_order::<K, V>(),
                vstd::laws_cmp::obeys_cmp_spec::<K>(),
                view_ord_consistent::<K>(),
            ensures
                empty.spec_ordkeymap_wf(),
                empty@ == Map::<K::V, V::V>::empty();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size(&self) -> (count: usize)
            requires self.spec_ordkeymap_wf(),
            ensures count == self@.dom().len(), self@.dom().finite();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_empty(&self) -> (is_empty: bool)
            requires self.spec_ordkeymap_wf(),
            ensures is_empty == self@.dom().is_empty();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn find(&self, k: &K) -> (found: Option<V>)
            requires self.spec_ordkeymap_wf(), obeys_view_eq::<K>()
            ensures
                match found {
                    Some(v) => self@.contains_key(k@) && v@ == self@[k@],
                    None => !self@.contains_key(k@),
                };

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn insert(&mut self, k: K, v: V)
            requires
                old(self).spec_ordkeymap_wf(),
                obeys_view_eq::<K>(),
                !old(self)@.contains_key(k@) ==> old(self)@.dom().len() + 1 < usize::MAX as nat,
            ensures
                self@.contains_key(k@),
                self@[k@] == v@,
                self@.dom() =~= old(self)@.dom().insert(k@),
                forall|key: K::V| key != k@ && #[trigger] old(self)@.contains_key(key) ==> self@[key] == old(self)@[key],
                self@.dom().finite(),
                self.spec_ordkeymap_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn delete(&mut self, k: &K)
            requires
                old(self).spec_ordkeymap_wf(),
                obeys_view_eq::<K>(),
                obeys_feq_clone::<Pair<K, V>>(),
            ensures
                self@ == old(self)@.remove(k@),
                self@.dom().finite(),
                self.spec_ordkeymap_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn split(&self, k: &K) -> (parts: (Self, Option<V>, Self))
            requires
                self.spec_ordkeymap_wf(),
            ensures
                parts.0.spec_ordkeymap_wf(),
                parts.2.spec_ordkeymap_wf(),
                parts.1 matches Some(v) ==> self@.contains_key(k@) && v@ == self@[k@],
                parts.1 matches None ==> !self@.contains_key(k@),
                parts.0@.dom().finite(),
                parts.2@.dom().finite(),
                // Left has all keys < k in the pair-view sense.
                forall|key: K::V| #[trigger] parts.0@.contains_key(key) ==> self@.contains_key(key) && parts.0@[key] == self@[key],
                forall|key: K::V| #[trigger] parts.2@.contains_key(key) ==> self@.contains_key(key) && parts.2@[key] == self@[key],
                // Completeness: every key in self is in left, right, or equals k.
                forall|key: K::V| self@.contains_key(key) ==>
                    #[trigger] parts.0@.contains_key(key) || parts.2@.contains_key(key) || key == k@,
                !parts.0@.contains_key(k@),
                !parts.2@.contains_key(k@),
                parts.0@.dom().disjoint(parts.2@.dom());

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m)
        fn union(&self, other: &Self) -> (combined: Self)
            requires
                self.spec_ordkeymap_wf(),
                other.spec_ordkeymap_wf(),
                self@.dom().len() + other@.dom().len() < usize::MAX as nat,
            ensures
                combined.spec_ordkeymap_wf(),
                combined@.dom() =~= self@.dom().union(other@.dom()),
                forall|k: K::V| self@.contains_key(k) && !other@.contains_key(k)
                    ==> #[trigger] combined@[k] == self@[k],
                forall|k: K::V| other@.contains_key(k)
                    ==> #[trigger] combined@[k] == other@[k];

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m)
        fn intersect(&self, other: &Self) -> (common: Self)
            requires
                self.spec_ordkeymap_wf(),
                other.spec_ordkeymap_wf(),
            ensures
                common.spec_ordkeymap_wf(),
                common@.dom() =~= self@.dom().intersect(other@.dom()),
                forall|k: K::V| #[trigger] common@.contains_key(k) ==> common@[k] == self@[k];

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m)
        fn union_with<F: Fn(&V, &V) -> V>(&self, other: &Self, combine: &F) -> (combined: Self)
            requires
                self.spec_ordkeymap_wf(),
                other.spec_ordkeymap_wf(),
                forall|v1: &V, v2: &V| #[trigger] combine.requires((v1, v2)),
                self@.dom().len() + other@.dom().len() < usize::MAX as nat,
            ensures
                combined.spec_ordkeymap_wf(),
                combined@.dom() =~= self@.dom().union(other@.dom()),
                forall|k: K::V| self@.contains_key(k) && !other@.contains_key(k)
                    ==> #[trigger] combined@[k] == self@[k],
                forall|k: K::V| !self@.contains_key(k) && other@.contains_key(k)
                    ==> #[trigger] combined@[k] == other@[k],
                forall|k: K::V| self@.contains_key(k) && other@.contains_key(k)
                    ==> (exists|v1: V, v2: V, r: V|
                        v1@ == self@[k] && v2@ == other@[k]
                        && combine.ensures((&v1, &v2), r)
                        && #[trigger] combined@[k] == r@);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m)
        fn intersect_with<F: Fn(&V, &V) -> V>(&self, other: &Self, combine: &F) -> (common: Self)
            requires
                self.spec_ordkeymap_wf(),
                other.spec_ordkeymap_wf(),
                forall|v1: &V, v2: &V| #[trigger] combine.requires((v1, v2)),
            ensures
                common.spec_ordkeymap_wf(),
                common@.dom() =~= self@.dom().intersect(other@.dom()),
                forall|k: K::V| #[trigger] common@.contains_key(k) ==>
                    (exists|v1: V, v2: V, r: V|
                        v1@ == self@[k] && v2@ == other@[k]
                        && combine.ensures((&v1, &v2), r)
                        && common@[k] == r@);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m)
        fn difference(&self, other: &Self) -> (remaining: Self)
            requires
                self.spec_ordkeymap_wf(),
                other.spec_ordkeymap_wf(),
            ensures
                remaining.spec_ordkeymap_wf(),
                remaining@.dom() =~= self@.dom().difference(other@.dom()),
                forall|k: K::V| #[trigger] remaining@.contains_key(k) ==> remaining@[k] == self@[k];

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn next_key(&self, k: &K) -> (successor: Option<K>)
            where K: TotalOrder
            requires self.spec_ordkeymap_wf()
            ensures
                self@.dom().finite(),
                successor matches Some(nk) ==> self@.dom().contains(nk@),
                successor matches Some(v) ==> TotalOrder::le(*k, v) && v@ != k@,
                successor matches Some(v) ==> forall|t: K| #![trigger t@]
                    self@.dom().contains(t@) && TotalOrder::le(*k, t) && t@ != k@
                    ==> TotalOrder::le(v, t),
                successor is None ==> forall|t: K| #![trigger t@]
                    self@.dom().contains(t@)
                    ==> !(TotalOrder::le(*k, t) && t@ != k@);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn prev_key(&self, k: &K) -> (predecessor: Option<K>)
            where K: TotalOrder
            requires self.spec_ordkeymap_wf()
            ensures
                self@.dom().finite(),
                predecessor matches Some(pk) ==> self@.dom().contains(pk@),
                predecessor matches Some(v) ==> TotalOrder::le(v, *k) && v@ != k@,
                predecessor matches Some(v) ==> forall|t: K| #![trigger t@]
                    self@.dom().contains(t@) && TotalOrder::le(t, *k) && t@ != k@
                    ==> TotalOrder::le(t, v);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn rank_key(&self, k: &K) -> (rank: usize)
            where K: TotalOrder
            requires
                self.spec_ordkeymap_wf(),
                obeys_view_eq::<K>(),
            ensures
                self@.dom().finite(),
                rank <= self@.dom().len(),
                rank as int == self@.dom().filter(
                    |x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, *k) && t@ != k@
                ).len();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn select_key(&self, i: usize) -> (selected: Option<K>)
            where K: TotalOrder
            requires
                self.spec_ordkeymap_wf(),
                obeys_view_eq::<K>(),
            ensures
                self@.dom().finite(),
                i >= self@.dom().len() ==> selected matches None,
                selected matches Some(k) ==> self@.dom().contains(k@),
                selected matches Some(v) ==> self@.dom().filter(
                    |x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, v) && t@ != v@
                ).len() == i as int;

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn first_key(&self) -> (first: Option<K>)
            where K: TotalOrder
            requires self.spec_ordkeymap_wf()
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> first matches None,
                first matches Some(k) ==> self@.dom().contains(k@),
                first matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(v, t);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn last_key(&self) -> (last: Option<K>)
            where K: TotalOrder
            requires self.spec_ordkeymap_wf()
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> last matches None,
                last matches Some(k) ==> self@.dom().contains(k@),
                last matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(t, v);

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n)
        fn get_key_range(&self, k1: &K, k2: &K) -> (range: Self)
            requires self.spec_ordkeymap_wf()
            ensures
                range@.dom().finite(),
                range@.dom().subset_of(self@.dom()),
                forall|key| #[trigger] range@.dom().contains(key) ==> range@[key] == self@[key],
                range.spec_ordkeymap_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn split_rank_key(&mut self, i: usize) -> (split: (Self, Self))
            requires
                old(self).spec_ordkeymap_wf(),
            ensures
                old(self)@.dom().finite(),
                split.0@.dom().finite(),
                split.1@.dom().finite(),
                split.0@.dom().subset_of(old(self)@.dom()),
                split.1@.dom().subset_of(old(self)@.dom()),
                split.0@.dom().disjoint(split.1@.dom()),
                forall|key| #[trigger] old(self)@.dom().contains(key) ==> split.0@.dom().contains(key) || split.1@.dom().contains(key),
                split.0.spec_ordkeymap_wf(),
                split.1.spec_ordkeymap_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn collect(&self) -> (entries: Vec<Pair<K, V>>)
            requires self.spec_ordkeymap_wf(),
            ensures
                entries@.len() == self@.dom().len(),
                self@.dom().finite();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n)
        fn filter<F: Fn(&K, &V) -> bool>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(K::V, V::V) -> bool>,
        ) -> (filtered: Self)
            requires
                self.spec_ordkeymap_wf(),
                forall|k: &K, v: &V| #[trigger] f.requires((k, v)),
                forall|k: K, v: V, keep: bool|
                    f.ensures((&k, &v), keep) ==> keep == spec_pred(k@, v@),
            ensures
                filtered@.dom().subset_of(self@.dom()),
                forall|k: K::V| #[trigger] filtered@.contains_key(k) ==> filtered@[k] == self@[k],
                forall|k: K::V| self@.dom().contains(k) && spec_pred(k, self@[k])
                    ==> #[trigger] filtered@.dom().contains(k),
                filtered@.dom().finite(),
                filtered.spec_ordkeymap_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n)
        fn map_values<F: Fn(&K, &V) -> V>(&self, f: F) -> (mapped: Self)
            requires
                self.spec_ordkeymap_wf(),
                forall|k: &K, v: &V| #[trigger] f.requires((k, v)),
                obeys_feq_clone::<Pair<K, V>>(),
            ensures
                mapped@.dom() =~= self@.dom(),
                mapped@.dom().finite(),
                mapped.spec_ordkeymap_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn reduce<F: Fn(&V, &V) -> V>(&self, f: F, id: &V) -> (reduced: V)
            requires
                self.spec_ordkeymap_wf(),
                forall|v1: &V, v2: &V| #[trigger] f.requires((v1, v2)),
            ensures self@.dom().finite();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n)
        fn domain(&self) -> (keys: ArraySetStEph<K>)
            requires self.spec_ordkeymap_wf()
            ensures keys@ =~= self@.dom(), keys.spec_arraysetsteph_wf();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n)
        fn tabulate<F: Fn(&K) -> V>(keys: &ArraySetStEph<K>, f: &F) -> (table: Self)
            requires
                keys.spec_arraysetsteph_wf(),
                forall|k: &K| #[trigger] f.requires((k,)),
                obeys_feq_fulls::<K, V>(),
                obeys_feq_full::<Pair<K, V>>(),
                keys@.len() < usize::MAX as nat,
                vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                view_ord_consistent::<Pair<K, V>>(),
                spec_pair_key_determines_order::<K, V>(),
                vstd::laws_cmp::obeys_cmp_spec::<K>(),
                view_ord_consistent::<K>(),
            ensures
                table.spec_ordkeymap_wf(),
                table@.dom() =~= keys@,
                forall|k: K::V| #[trigger] table@.contains_key(k) ==>
                    (exists|key_arg: K, result: V|
                        key_arg@ == k && f.ensures((&key_arg,), result)
                        && table@[k] == result@),
                table@.dom().finite();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m)
        fn restrict(&self, keys: &ArraySetStEph<K>) -> (restricted: Self)
            requires self.spec_ordkeymap_wf(), keys.spec_arraysetsteph_wf()
            ensures
                restricted.spec_ordkeymap_wf(),
                restricted@.dom() =~= self@.dom().intersect(keys@),
                forall|k: K::V| #[trigger] restricted@.contains_key(k) ==> restricted@[k] == self@[k],
                restricted@.dom().finite();

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m)
        fn subtract(&self, keys: &ArraySetStEph<K>) -> (remaining: Self)
            requires self.spec_ordkeymap_wf(), keys.spec_arraysetsteph_wf()
            ensures
                remaining.spec_ordkeymap_wf(),
                remaining@.dom() =~= self@.dom().difference(keys@),
                forall|k: K::V| #[trigger] remaining@.contains_key(k) ==> remaining@[k] == self@[k],
                remaining@.dom().finite();
    }

    //		Section 9. impls


    /// Find a value by key via BST descent. O(lg n).
    fn ordkeymap_find<K: StT + Ord, V: StT + Ord>(
        tree: &ParamBST<Pair<K, V>>,
        k: &K,
    ) -> (found: Option<V>)
        requires
            tree.spec_bstparasteph_wf(),
            spec_key_unique_pairs_set(tree@),
            spec_set_pair_view_generated::<K, V>(tree@),
            view_ord_consistent::<K>(),
            obeys_feq_fulls::<K, V>(),
            vstd::laws_cmp::obeys_cmp_spec::<K>(),
            spec_pair_key_determines_order::<K, V>(),
            view_ord_consistent::<Pair<K, V>>(),
            vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
        ensures
            match found {
                Some(v) => spec_pair_set_to_map(tree@).contains_key(k@)
                    && v@ == spec_pair_set_to_map(tree@)[k@],
                None => !spec_pair_set_to_map(tree@).contains_key(k@),
            }
        decreases tree@.len(),
    {
        match tree.expose() {
            Exposed::Leaf => {
                proof {
                    if spec_pair_set_to_map(tree@).contains_key(k@) {
                        lemma_map_contains_pair_in_set(tree@, k@);
                    }
                }
                None
            },
            Exposed::Node(left, root_pair, right) => {
                proof {
                    reveal(vstd::laws_cmp::obeys_cmp_ord);
                    vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                    // Propagate key uniqueness and View generation to subtrees.
                    assert(spec_key_unique_pairs_set(left@)) by {
                        assert forall|kv: K::V, vv: V::V| #[trigger] left@.contains((kv, vv))
                            implies forall|vv2: V::V| left@.contains((kv, vv2)) ==> vv == vv2 by {
                            assert(tree@.contains((kv, vv)));
                            assert forall|vv2: V::V| left@.contains((kv, vv2)) implies vv == vv2 by {
                                assert(tree@.contains((kv, vv2)));
                            };
                        };
                    };
                    assert(spec_key_unique_pairs_set(right@)) by {
                        assert forall|kv: K::V, vv: V::V| #[trigger] right@.contains((kv, vv))
                            implies forall|vv2: V::V| right@.contains((kv, vv2)) ==> vv == vv2 by {
                            assert(tree@.contains((kv, vv)));
                            assert forall|vv2: V::V| right@.contains((kv, vv2)) implies vv == vv2 by {
                                assert(tree@.contains((kv, vv2)));
                            };
                        };
                    };
                    assert(spec_set_pair_view_generated::<K, V>(left@)) by {
                        assert forall|elem: (K::V, V::V)| left@.contains(elem)
                            implies exists|p: Pair<K, V>| (#[trigger] p@) == elem by {
                            assert(tree@.contains(elem));
                        };
                    };
                    assert(spec_set_pair_view_generated::<K, V>(right@)) by {
                        assert forall|elem: (K::V, V::V)| right@.contains(elem)
                            implies exists|p: Pair<K, V>| (#[trigger] p@) == elem by {
                            assert(tree@.contains(elem));
                        };
                    };
                }
                let c = k.cmp(&root_pair.0);
                proof { reveal(vstd::laws_cmp::obeys_cmp_ord); }
                match c {
                    Equal => {
                        let v_clone = root_pair.1.clone_plus();
                        proof {
                            lemma_cloned_view_eq(root_pair.1, v_clone);
                            lemma_pair_in_set_map_contains(tree@, k@, root_pair.1@);
                        }
                        Some(v_clone)
                    },
                    Less => {
                        let result = ordkeymap_find(&left, k);
                        proof {
                            if result is Some {
                                lemma_map_contains_pair_in_set(left@, k@);
                                let vv: V::V = choose|vv: V::V| left@.contains((k@, vv));
                                lemma_pair_in_set_map_contains(tree@, k@, vv);
                            } else {
                                if spec_pair_set_to_map(tree@).contains_key(k@) {
                                    lemma_map_contains_pair_in_set(tree@, k@);
                                    let vv: V::V = choose|vv: V::V| tree@.contains((k@, vv));
                                    let ghost p_wit: Pair<K, V> = choose|p: Pair<K, V>| p@ == (k@, vv);
                                    lemma_cmp_equal_congruent(p_wit.0, *k, root_pair.0);
                                }
                            }
                        }
                        result
                    },
                    Greater => {
                        let result = ordkeymap_find(&right, k);
                        proof {
                            if result is Some {
                                lemma_map_contains_pair_in_set(right@, k@);
                                let vv: V::V = choose|vv: V::V| right@.contains((k@, vv));
                                lemma_pair_in_set_map_contains(tree@, k@, vv);
                            } else {
                                if spec_pair_set_to_map(tree@).contains_key(k@) {
                                    lemma_map_contains_pair_in_set(tree@, k@);
                                    let vv: V::V = choose|vv: V::V| tree@.contains((k@, vv));
                                    let ghost p_wit: Pair<K, V> = choose|p: Pair<K, V>| p@ == (k@, vv);
                                    lemma_cmp_equal_congruent(p_wit.0, *k, root_pair.0);
                                }
                            }
                        }
                        result
                    },
                }
            }
        }
    }

    /// Split a ParamBST<Pair<K,V>> by key. O(lg n).
    fn ordkeymap_split<K: StT + Ord, V: StT + Ord>(
        tree: &ParamBST<Pair<K, V>>,
        k: &K,
    ) -> (parts: (ParamBST<Pair<K, V>>, Option<V>, ParamBST<Pair<K, V>>))
        requires
            tree.spec_bstparasteph_wf(),
            spec_key_unique_pairs_set(tree@),
            spec_set_pair_view_generated::<K, V>(tree@),
            view_ord_consistent::<K>(),
            obeys_feq_fulls::<K, V>(),
            vstd::laws_cmp::obeys_cmp_spec::<K>(),
            spec_pair_key_determines_order::<K, V>(),
            view_ord_consistent::<Pair<K, V>>(),
            vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
            obeys_feq_full::<Pair<K, V>>(),
        ensures
            parts.1 matches Some(v) ==> spec_pair_set_to_map(tree@).contains_key(k@) && v@ == spec_pair_set_to_map(tree@)[k@],
            parts.1 matches None ==> !spec_pair_set_to_map(tree@).contains_key(k@),
            parts.0@.finite(),
            parts.2@.finite(),
            forall|p: Pair<K, V>| (#[trigger] parts.0@.contains(p@)) ==> p.0.cmp_spec(k) == Less,
            forall|p: Pair<K, V>| (#[trigger] parts.2@.contains(p@)) ==> p.0.cmp_spec(k) == Greater,
            parts.0@.subset_of(tree@),
            parts.2@.subset_of(tree@),
            parts.0@.disjoint(parts.2@),
            !spec_pair_set_to_map(parts.0@).dom().contains(k@),
            !spec_pair_set_to_map(parts.2@).dom().contains(k@),
            forall|kv: <K as View>::V, vv: <V as View>::V| #[trigger] tree@.contains((kv, vv))
                ==> parts.0@.contains((kv, vv)) || parts.2@.contains((kv, vv)) || kv == k@,
            spec_key_unique_pairs_set(parts.0@),
            spec_key_unique_pairs_set(parts.2@),
            spec_set_pair_view_generated::<K, V>(parts.0@),
            spec_set_pair_view_generated::<K, V>(parts.2@),
            parts.0.spec_bstparasteph_wf(),
            parts.2.spec_bstparasteph_wf(),
            parts.0@.len() + parts.2@.len() <= tree@.len(),
        decreases tree@.len(),
    {
        match tree.expose() {
            Exposed::Leaf => {
                proof {
                    if spec_pair_set_to_map(tree@).contains_key(k@) {
                        lemma_map_contains_pair_in_set(tree@, k@);
                    }
                    lemma_key_unique_empty::<K::V, V::V>();
                }
                let empty1 = ParamBST::<Pair<K, V>>::new();
                let empty2 = ParamBST::<Pair<K, V>>::new();
                (empty1, None, empty2)
            },
            Exposed::Node(left, root_pair, right) => {
                proof {
                    reveal(vstd::laws_cmp::obeys_cmp_ord);
                    vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                    lemma_key_unique_subset(tree@, left@);
                    lemma_key_unique_subset(tree@, right@);
                    lemma_view_gen_subset::<K, V>(left@, tree@);
                    lemma_view_gen_subset::<K, V>(right@, tree@);
                    lemma_reveal_view_injective::<K>();
                }
                let c = k.cmp(&root_pair.0);
                proof { reveal(vstd::laws_cmp::obeys_cmp_ord); }
                match c {
                    Equal => {
                        let v = root_pair.1.clone_plus();
                        proof {
                            lemma_cloned_view_eq(root_pair.1, v);
                            lemma_pair_in_set_map_contains(tree@, root_pair.0@, root_pair.1@);
                            assert forall|p: Pair<K, V>| (#[trigger] left@.contains(p@))
                                implies p.0.cmp_spec(k) == Less by {
                                assert(p.0@ != root_pair.0@) by {
                                    if p.0@ == root_pair.0@ {
                                        assert(tree@.contains(p@));
                                    }
                                };
                                lemma_cmp_equal_congruent(root_pair.0, *k, p.0);
                            };
                            assert forall|p: Pair<K, V>| (#[trigger] right@.contains(p@))
                                implies p.0.cmp_spec(k) == Greater by {
                                assert(p.0@ != root_pair.0@) by {
                                    if p.0@ == root_pair.0@ {
                                        assert(tree@.contains(p@));
                                    }
                                };
                            };
                        }
                        (left, Some(v), right)
                    },
                    Less => {
                        let (ll, found, lr) = ordkeymap_split(&left, k);
                        proof {
                            vstd::set_lib::lemma_len_subset(lr@, left@);
                        }
                        let new_right = ParamBST::join_m(lr, root_pair, right);
                        proof {
                            assert forall|p: Pair<K, V>| (#[trigger] new_right@.contains(p@))
                                implies p.0.cmp_spec(k) == Greater by {
                                if lr@.contains(p@) {}
                                else if right@.contains(p@) {
                                    lemma_cmp_antisymmetry(*k, root_pair.0);
                                    reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                                } else {
                                    lemma_cmp_antisymmetry(*k, root_pair.0);
                                }
                            };
                            lemma_key_unique_subset(tree@, new_right@);
                            lemma_view_gen_subset::<K, V>(new_right@, tree@);
                            vstd::set_lib::lemma_set_disjoint_lens(lr@, right@);
                            if found is Some {
                                lemma_map_contains_pair_in_set(left@, k@);
                                let lv: V::V = choose|lv: V::V| left@.contains((k@, lv));
                                lemma_pair_in_set_map_contains(tree@, k@, lv);
                                lemma_pair_in_set_map_contains(left@, k@, lv);
                            }
                        }
                        (ll, found, new_right)
                    },
                    Greater => {
                        let (rl, found, rr) = ordkeymap_split(&right, k);
                        proof {
                            vstd::set_lib::lemma_len_subset(rl@, right@);
                        }
                        let new_left = ParamBST::join_m(left, root_pair, rl);
                        proof {
                            reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                            lemma_key_unique_subset(tree@, new_left@);
                            lemma_view_gen_subset::<K, V>(new_left@, tree@);
                            vstd::set_lib::lemma_set_disjoint_lens(left@, rl@);
                            if found is Some {
                                lemma_map_contains_pair_in_set(right@, k@);
                                let rv: V::V = choose|rv: V::V| right@.contains((k@, rv));
                                lemma_pair_in_set_map_contains(tree@, k@, rv);
                                lemma_pair_in_set_map_contains(right@, k@, rv);
                            }
                        }
                        (new_left, found, rr)
                    },
                }
            }
        }
    }

    /// Find the next (successor) key strictly greater than k via BST descent. O(lg n).
    #[verifier::rlimit(20)]
    fn ordkeymap_next<K: StT + Ord + TotalOrder, V: StT + Ord>(
        tree: &ParamBST<Pair<K, V>>,
        k: &K,
    ) -> (successor: Option<K>)
        requires
            tree.spec_bstparasteph_wf(),
            spec_key_unique_pairs_set(tree@),
            spec_set_pair_view_generated::<K, V>(tree@),
            view_ord_consistent::<K>(),
            obeys_feq_fulls::<K, V>(),
            vstd::laws_cmp::obeys_cmp_spec::<K>(),
            spec_pair_key_determines_order::<K, V>(),
            view_ord_consistent::<Pair<K, V>>(),
            vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
        ensures
            spec_pair_set_to_map(tree@).dom().finite(),
            successor matches Some(nk) ==> spec_pair_set_to_map(tree@).dom().contains(nk@),
            successor matches Some(v) ==> TotalOrder::le(*k, v) && v@ != k@,
            successor matches Some(v) ==> forall|t: K| #![trigger t@]
                spec_pair_set_to_map(tree@).dom().contains(t@) && TotalOrder::le(*k, t) && t@ != k@
                ==> TotalOrder::le(v, t),
            successor is None ==> forall|t: K| #![trigger t@]
                spec_pair_set_to_map(tree@).dom().contains(t@)
                ==> !(TotalOrder::le(*k, t) && t@ != k@),
        decreases tree@.len(),
    {
        proof { lemma_pair_set_to_map_dom_finite(tree@); }
        match tree.expose() {
            Exposed::Leaf => None,
            Exposed::Node(left, root_pair, right) => {
                reveal_param_bst_backings(&left);
                reveal_param_bst_backings(&right);
                proof {
                    reveal(vstd::laws_cmp::obeys_cmp_ord);
                    vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                    lemma_key_unique_subset(tree@, left@);
                    lemma_key_unique_subset(tree@, right@);
                    lemma_view_gen_subset::<K, V>(left@, tree@);
                    lemma_view_gen_subset::<K, V>(right@, tree@);
                    lemma_reveal_view_injective::<K>();
                }
                let c = Ord::cmp(k, &root_pair.0);
                proof { reveal(vstd::laws_cmp::obeys_cmp_ord); }
                match c {
                    Less => {
                        let left_result = ordkeymap_next(&left, k);
                        match left_result {
                            Some(lk) => {
                                proof {
                                    lemma_map_contains_pair_in_set(left@, lk@);
                                    let vv: V::V = choose|vv: V::V| left@.contains((lk@, vv));
                                    lemma_pair_in_set_map_contains(tree@, lk@, vv);
                                    assert forall|t: K| #![trigger t@]
                                        spec_pair_set_to_map(tree@).dom().contains(t@)
                                        && TotalOrder::le(*k, t) && t@ != k@
                                        implies TotalOrder::le(lk, t) by {
                                        lemma_map_contains_pair_in_set(tree@, t@);
                                        let tv: V::V = choose|tv: V::V| tree@.contains((t@, tv));
                                        if left@.contains((t@, tv)) {
                                            lemma_pair_in_set_map_contains(left@, t@, tv);
                                        } else if (t@, tv) == root_pair@ {
                                            let lp: Pair<K, V> = choose|lp: Pair<K, V>| #[trigger] left@.contains(lp@) && lp@ == (lk@, vv);
                                            K::cmp_spec_less_implies_le(lk, t);
                                        } else {
                                            let lp: Pair<K, V> = choose|lp: Pair<K, V>| #[trigger] left@.contains(lp@) && lp@ == (lk@, vv);
                                            let tp: Pair<K, V> = choose|tp: Pair<K, V>| #[trigger] right@.contains(tp@) && tp@ == (t@, tv);
                                            assert(lp.0@ != root_pair.0@) by {
                                                if lp.0@ == root_pair.0@ {
                                                    assert(tree@.contains(root_pair@));
                                                }
                                            };
                                            K::cmp_spec_less_implies_le(lk, root_pair.0);
                                            K::cmp_spec_greater_implies_le(tp.0, root_pair.0);
                                            K::transitive(lk, root_pair.0, t);
                                        }
                                    };
                                }
                                Some(lk)
                            },
                            None => {
                                let key = root_pair.0.clone_plus();
                                proof {
                                    lemma_cloned_view_eq(root_pair.0, key);
                                    lemma_pair_in_set_map_contains(tree@, root_pair.0@, root_pair.1@);
                                    K::cmp_spec_less_implies_le(*k, root_pair.0);
                                    assert forall|t: K| #![trigger t@]
                                        spec_pair_set_to_map(tree@).dom().contains(t@)
                                        && TotalOrder::le(*k, t) && t@ != k@
                                        implies TotalOrder::le(key, t) by {
                                        lemma_map_contains_pair_in_set(tree@, t@);
                                        let tv: V::V = choose|tv: V::V| tree@.contains((t@, tv));
                                        if left@.contains((t@, tv)) {
                                            lemma_pair_in_set_map_contains(left@, t@, tv);
                                        } else if (t@, tv) == root_pair@ {
                                            K::reflexive(key);
                                        } else {
                                            let tp: Pair<K, V> = choose|tp: Pair<K, V>| #[trigger] right@.contains(tp@) && tp@ == (t@, tv);
                                            K::cmp_spec_greater_implies_le(t, root_pair.0);
                                        }
                                    };
                                }
                                Some(key)
                            },
                        }
                    },
                    Equal => {
                        let right_min = right.min_key();
                        match right_min {
                            None => {
                                proof {
                                    assert forall|t: K| #![trigger t@]
                                        spec_pair_set_to_map(tree@).dom().contains(t@)
                                        && TotalOrder::le(*k, t) && t@ != k@
                                        implies false by {
                                        lemma_map_contains_pair_in_set(tree@, t@);
                                        let tv: V::V = choose|tv: V::V| tree@.contains((t@, tv));
                                        if left@.contains((t@, tv)) {
                                            let tp: Pair<K, V> = choose|tp: Pair<K, V>| #[trigger] left@.contains(tp@) && tp@ == (t@, tv);
                                            K::cmp_spec_less_implies_le(t, root_pair.0);
                                            K::antisymmetric(t, *k);
                                        } else if (t@, tv) == root_pair@ {
                                        } else {
                                        }
                                    };
                                }
                                None
                            },
                            Some(min_pair) => {
                                let key = min_pair.0.clone_plus();
                                proof {
                                    lemma_cloned_view_eq(min_pair.0, key);
                                    lemma_pair_in_set_map_contains(tree@, min_pair.0@, min_pair.1@);
                                    assert(min_pair.0@ != root_pair.0@) by {
                                        if min_pair.0@ == root_pair.0@ {
                                            assert(tree@.contains(root_pair@));
                                        }
                                    };
                                    K::cmp_spec_greater_implies_le(min_pair.0, root_pair.0);
                                    assert forall|t: K| #![trigger t@]
                                        spec_pair_set_to_map(tree@).dom().contains(t@)
                                        && TotalOrder::le(*k, t) && t@ != k@
                                        implies TotalOrder::le(key, t) by {
                                        lemma_map_contains_pair_in_set(tree@, t@);
                                        let tv: V::V = choose|tv: V::V| tree@.contains((t@, tv));
                                        if left@.contains((t@, tv)) {
                                            let tp: Pair<K, V> = choose|tp: Pair<K, V>| #[trigger] left@.contains(tp@) && tp@ == (t@, tv);
                                            K::cmp_spec_less_implies_le(t, root_pair.0);
                                            K::antisymmetric(t, *k);
                                        } else if (t@, tv) == root_pair@ {
                                        } else {
                                            let tp: Pair<K, V> = choose|tp: Pair<K, V>| #[trigger] right@.contains(tp@) && tp@ == (t@, tv);
                                            if min_pair@ == tp@ {
                                                K::reflexive(key);
                                            } else {
                                                K::cmp_spec_less_implies_le(key, t);
                                            }
                                        }
                                    };
                                }
                                Some(key)
                            },
                        }
                    },
                    Greater => {
                        let result = ordkeymap_next(&right, k);
                        proof {
                            if result is Some {
                                let rk = result->Some_0;
                                lemma_map_contains_pair_in_set(right@, rk@);
                                let rv: V::V = choose|rv: V::V| right@.contains((rk@, rv));
                                lemma_pair_in_set_map_contains(tree@, rk@, rv);
                                assert forall|t: K| #![trigger t@]
                                    spec_pair_set_to_map(tree@).dom().contains(t@)
                                    && TotalOrder::le(*k, t) && t@ != k@
                                    implies TotalOrder::le(rk, t) by {
                                    lemma_map_contains_pair_in_set(tree@, t@);
                                    let tv: V::V = choose|tv: V::V| tree@.contains((t@, tv));
                                    if left@.contains((t@, tv)) {
                                        let tp: Pair<K, V> = choose|tp: Pair<K, V>| #[trigger] left@.contains(tp@) && tp@ == (t@, tv);
                                        assert(tp.0@ != root_pair.0@) by {
                                            if tp.0@ == root_pair.0@ {
                                                assert(tree@.contains(root_pair@));
                                            }
                                        };
                                        K::cmp_spec_less_implies_le(t, root_pair.0);
                                        K::cmp_spec_greater_implies_le(*k, root_pair.0);
                                        K::transitive(t, root_pair.0, *k);
                                        K::antisymmetric(t, *k);
                                    } else if (t@, tv) == root_pair@ {
                                        K::cmp_spec_greater_implies_le(*k, root_pair.0);
                                        K::antisymmetric(t, *k);
                                    } else {
                                        lemma_pair_in_set_map_contains(right@, t@, tv);
                                    }
                                };
                            } else {
                                assert forall|t: K| #![trigger t@]
                                    spec_pair_set_to_map(tree@).dom().contains(t@)
                                    implies !(TotalOrder::le(*k, t) && t@ != k@) by {
                                    lemma_map_contains_pair_in_set(tree@, t@);
                                    let tv: V::V = choose|tv: V::V| tree@.contains((t@, tv));
                                    if left@.contains((t@, tv)) {
                                        let tp: Pair<K, V> = choose|tp: Pair<K, V>| #[trigger] left@.contains(tp@) && tp@ == (t@, tv);
                                        assert(tp.0@ != root_pair.0@) by {
                                            if tp.0@ == root_pair.0@ {
                                                assert(tree@.contains(root_pair@));
                                            }
                                        };
                                        K::cmp_spec_less_implies_le(t, root_pair.0);
                                        K::cmp_spec_greater_implies_le(*k, root_pair.0);
                                        K::transitive(t, root_pair.0, *k);
                                        if TotalOrder::le(*k, t) && t@ != k@ {
                                            K::antisymmetric(t, *k);
                                        }
                                    } else if (t@, tv) == root_pair@ {
                                        K::cmp_spec_greater_implies_le(*k, root_pair.0);
                                        if TotalOrder::le(*k, t) && t@ != k@ {
                                            K::antisymmetric(t, *k);
                                        }
                                    } else {
                                        lemma_pair_in_set_map_contains(right@, t@, tv);
                                    }
                                };
                            }
                        }
                        result
                    },
                }
            }
        }
    }

    /// Find the previous (predecessor) key strictly less than k via BST descent. O(lg n).
    fn ordkeymap_prev<K: StT + Ord + TotalOrder, V: StT + Ord>(
        tree: &ParamBST<Pair<K, V>>,
        k: &K,
    ) -> (predecessor: Option<K>)
        requires
            tree.spec_bstparasteph_wf(),
            spec_key_unique_pairs_set(tree@),
            spec_set_pair_view_generated::<K, V>(tree@),
            view_ord_consistent::<K>(),
            obeys_feq_fulls::<K, V>(),
            vstd::laws_cmp::obeys_cmp_spec::<K>(),
            spec_pair_key_determines_order::<K, V>(),
            view_ord_consistent::<Pair<K, V>>(),
            vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
        ensures
            spec_pair_set_to_map(tree@).dom().finite(),
            predecessor matches Some(pk) ==> spec_pair_set_to_map(tree@).dom().contains(pk@),
            predecessor matches Some(v) ==> TotalOrder::le(v, *k) && v@ != k@,
            predecessor matches Some(v) ==> forall|t: K| #![trigger t@]
                spec_pair_set_to_map(tree@).dom().contains(t@) && TotalOrder::le(t, *k) && t@ != k@
                ==> TotalOrder::le(t, v),
            predecessor is None ==> forall|t: K| #![trigger t@]
                spec_pair_set_to_map(tree@).dom().contains(t@)
                ==> !(TotalOrder::le(t, *k) && t@ != k@),
        decreases tree@.len(),
    {
        proof { lemma_pair_set_to_map_dom_finite(tree@); }
        match tree.expose() {
            Exposed::Leaf => None,
            Exposed::Node(left, root_pair, right) => {
                reveal_param_bst_backings(&left);
                reveal_param_bst_backings(&right);
                proof {
                    reveal(vstd::laws_cmp::obeys_cmp_ord);
                    vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                    lemma_key_unique_subset(tree@, left@);
                    lemma_key_unique_subset(tree@, right@);
                    lemma_view_gen_subset::<K, V>(left@, tree@);
                    lemma_view_gen_subset::<K, V>(right@, tree@);
                    lemma_reveal_view_injective::<K>();
                }
                let c = Ord::cmp(k, &root_pair.0);
                proof { reveal(vstd::laws_cmp::obeys_cmp_ord); }
                match c {
                    Greater => {
                        let right_result = ordkeymap_prev(&right, k);
                        match right_result {
                            Some(rk) => {
                                proof {
                                    lemma_map_contains_pair_in_set(right@, rk@);
                                    let rv: V::V = choose|rv: V::V| right@.contains((rk@, rv));
                                    lemma_pair_in_set_map_contains(tree@, rk@, rv);
                                    assert forall|t: K| #![trigger t@]
                                        spec_pair_set_to_map(tree@).dom().contains(t@)
                                        && TotalOrder::le(t, *k) && t@ != k@
                                        implies TotalOrder::le(t, rk) by {
                                        lemma_map_contains_pair_in_set(tree@, t@);
                                        let tv: V::V = choose|tv: V::V| tree@.contains((t@, tv));
                                        if right@.contains((t@, tv)) {
                                            lemma_pair_in_set_map_contains(right@, t@, tv);
                                        } else if (t@, tv) == root_pair@ {
                                            let rp: Pair<K, V> = choose|rp: Pair<K, V>| #[trigger] right@.contains(rp@) && rp@ == (rk@, rv);
                                            K::cmp_spec_greater_implies_le(rk, root_pair.0);
                                        } else {
                                            let tp: Pair<K, V> = choose|tp: Pair<K, V>| #[trigger] left@.contains(tp@) && tp@ == (t@, tv);
                                            let rp: Pair<K, V> = choose|rp: Pair<K, V>| #[trigger] right@.contains(rp@) && rp@ == (rk@, rv);
                                            assert(tp.0@ != root_pair.0@) by {
                                                if tp.0@ == root_pair.0@ {
                                                    assert(tree@.contains(root_pair@));
                                                }
                                            };
                                            K::cmp_spec_less_implies_le(t, root_pair.0);
                                            K::cmp_spec_greater_implies_le(rk, root_pair.0);
                                            K::transitive(t, root_pair.0, rk);
                                        }
                                    };
                                }
                                Some(rk)
                            },
                            None => {
                                let key = root_pair.0.clone_plus();
                                proof {
                                    lemma_cloned_view_eq(root_pair.0, key);
                                    lemma_pair_in_set_map_contains(tree@, root_pair.0@, root_pair.1@);
                                    K::cmp_spec_greater_implies_le(*k, root_pair.0);
                                    assert forall|t: K| #![trigger t@]
                                        spec_pair_set_to_map(tree@).dom().contains(t@)
                                        && TotalOrder::le(t, *k) && t@ != k@
                                        implies TotalOrder::le(t, key) by {
                                        lemma_map_contains_pair_in_set(tree@, t@);
                                        let tv: V::V = choose|tv: V::V| tree@.contains((t@, tv));
                                        if right@.contains((t@, tv)) {
                                            lemma_pair_in_set_map_contains(right@, t@, tv);
                                        } else if (t@, tv) == root_pair@ {
                                            K::reflexive(key);
                                        } else {
                                            let tp: Pair<K, V> = choose|tp: Pair<K, V>| #[trigger] left@.contains(tp@) && tp@ == (t@, tv);
                                            K::cmp_spec_less_implies_le(t, root_pair.0);
                                        }
                                    };
                                }
                                Some(key)
                            },
                        }
                    },
                    Equal => {
                        let left_max = left.max_key();
                        match left_max {
                            None => {
                                proof {
                                    assert forall|t: K| #![trigger t@]
                                        spec_pair_set_to_map(tree@).dom().contains(t@)
                                        && TotalOrder::le(t, *k) && t@ != k@
                                        implies false by {
                                        lemma_map_contains_pair_in_set(tree@, t@);
                                        let tv: V::V = choose|tv: V::V| tree@.contains((t@, tv));
                                        if right@.contains((t@, tv)) {
                                            let tp: Pair<K, V> = choose|tp: Pair<K, V>| #[trigger] right@.contains(tp@) && tp@ == (t@, tv);
                                            K::cmp_spec_greater_implies_le(t, root_pair.0);
                                            K::antisymmetric(*k, t);
                                        } else if (t@, tv) == root_pair@ {
                                        } else {
                                        }
                                    };
                                }
                                None
                            },
                            Some(max_pair) => {
                                let key = max_pair.0.clone_plus();
                                proof {
                                    lemma_cloned_view_eq(max_pair.0, key);
                                    lemma_pair_in_set_map_contains(tree@, max_pair.0@, max_pair.1@);
                                    assert(max_pair.0@ != root_pair.0@) by {
                                        if max_pair.0@ == root_pair.0@ {
                                            assert(tree@.contains(root_pair@));
                                        }
                                    };
                                    K::cmp_spec_less_implies_le(max_pair.0, root_pair.0);
                                    assert forall|t: K| #![trigger t@]
                                        spec_pair_set_to_map(tree@).dom().contains(t@)
                                        && TotalOrder::le(t, *k) && t@ != k@
                                        implies TotalOrder::le(t, key) by {
                                        lemma_map_contains_pair_in_set(tree@, t@);
                                        let tv: V::V = choose|tv: V::V| tree@.contains((t@, tv));
                                        if right@.contains((t@, tv)) {
                                            let tp: Pair<K, V> = choose|tp: Pair<K, V>| #[trigger] right@.contains(tp@) && tp@ == (t@, tv);
                                            K::cmp_spec_greater_implies_le(t, root_pair.0);
                                            K::antisymmetric(*k, t);
                                        } else if (t@, tv) == root_pair@ {
                                        } else {
                                            let tp: Pair<K, V> = choose|tp: Pair<K, V>| #[trigger] left@.contains(tp@) && tp@ == (t@, tv);
                                            if max_pair@ == tp@ {
                                                K::reflexive(key);
                                            } else {
                                                K::cmp_spec_less_implies_le(t, key);
                                            }
                                        }
                                    };
                                }
                                Some(key)
                            },
                        }
                    },
                    Less => {
                        let result = ordkeymap_prev(&left, k);
                        proof {
                            if result is Some {
                                let lk = result->Some_0;
                                lemma_map_contains_pair_in_set(left@, lk@);
                                let lv: V::V = choose|lv: V::V| left@.contains((lk@, lv));
                                lemma_pair_in_set_map_contains(tree@, lk@, lv);
                                assert forall|t: K| #![trigger t@]
                                    spec_pair_set_to_map(tree@).dom().contains(t@)
                                    && TotalOrder::le(t, *k) && t@ != k@
                                    implies TotalOrder::le(t, lk) by {
                                    lemma_map_contains_pair_in_set(tree@, t@);
                                    let tv: V::V = choose|tv: V::V| tree@.contains((t@, tv));
                                    if left@.contains((t@, tv)) {
                                        lemma_pair_in_set_map_contains(left@, t@, tv);
                                    } else if (t@, tv) == root_pair@ {
                                        K::cmp_spec_less_implies_le(*k, root_pair.0);
                                        K::antisymmetric(t, *k);
                                    } else {
                                        let tp: Pair<K, V> = choose|tp: Pair<K, V>| #[trigger] right@.contains(tp@) && tp@ == (t@, tv);
                                        assert(tp.0@ != root_pair.0@) by {
                                            if tp.0@ == root_pair.0@ {
                                                assert(tree@.contains(tp@));
                                                assert(tree@.contains(root_pair@));
                                            }
                                        };
                                        K::cmp_spec_greater_implies_le(t, root_pair.0);
                                        K::cmp_spec_less_implies_le(*k, root_pair.0);
                                        K::transitive(*k, root_pair.0, t);
                                        K::antisymmetric(t, *k);
                                    }
                                };
                            } else {
                                assert forall|t: K| #![trigger t@]
                                    spec_pair_set_to_map(tree@).dom().contains(t@)
                                    implies !(TotalOrder::le(t, *k) && t@ != k@) by {
                                    lemma_map_contains_pair_in_set(tree@, t@);
                                    let tv: V::V = choose|tv: V::V| tree@.contains((t@, tv));
                                    if left@.contains((t@, tv)) {
                                        lemma_pair_in_set_map_contains(left@, t@, tv);
                                    } else if (t@, tv) == root_pair@ {
                                        K::cmp_spec_less_implies_le(*k, root_pair.0);
                                        if TotalOrder::le(t, *k) && t@ != k@ {
                                            K::antisymmetric(t, *k);
                                        }
                                    } else {
                                        let tp: Pair<K, V> = choose|tp: Pair<K, V>| #[trigger] right@.contains(tp@) && tp@ == (t@, tv);
                                        assert(tp.0@ != root_pair.0@) by {
                                            if tp.0@ == root_pair.0@ {
                                                assert(tree@.contains(tp@));
                                                assert(tree@.contains(root_pair@));
                                            }
                                        };
                                        K::cmp_spec_greater_implies_le(t, root_pair.0);
                                        K::cmp_spec_less_implies_le(*k, root_pair.0);
                                        K::transitive(*k, root_pair.0, t);
                                        if TotalOrder::le(t, *k) && t@ != k@ {
                                            K::antisymmetric(t, *k);
                                        }
                                    }
                                };
                            }
                        }
                        result
                    },
                }
            }
        }
    }

    /// Rank: count of keys strictly less than k via BST descent. O(lg n).
    fn ordkeymap_rank<K: StT + Ord + TotalOrder, V: StT + Ord>(
        tree: &ParamBST<Pair<K, V>>,
        k: &K,
    ) -> (rank: usize)
        requires
            tree.spec_bstparasteph_wf(),
            spec_key_unique_pairs_set(tree@),
            spec_set_pair_view_generated::<K, V>(tree@),
            view_ord_consistent::<K>(),
            obeys_feq_fulls::<K, V>(),
            obeys_view_eq::<K>(),
            vstd::laws_cmp::obeys_cmp_spec::<K>(),
            spec_pair_key_determines_order::<K, V>(),
            view_ord_consistent::<Pair<K, V>>(),
            vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
        ensures
            spec_pair_set_to_map(tree@).dom().finite(),
            rank <= spec_pair_set_to_map(tree@).dom().len(),
            rank as int == spec_pair_set_to_map(tree@).dom().filter(
                |x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, *k) && t@ != k@
            ).len(),
        decreases tree@.len(),
    {
        proof {
            lemma_pair_set_to_map_dom_finite(tree@);
            lemma_pair_set_to_map_len(tree@);
            lemma_reveal_view_injective::<K>();
        }
        let ghost rank_pred = |x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, *k) && t@ != k@;
        match tree.expose() {
            Exposed::Leaf => {
                proof {
                    assert(spec_pair_set_to_map(tree@).dom().filter(rank_pred) =~= Set::empty());
                }
                0
            },
            Exposed::Node(left, root_pair, right) => {
                reveal_param_bst_backings(&left);
                reveal_param_bst_backings(&right);
                proof {
                    reveal(vstd::laws_cmp::obeys_cmp_ord);
                    vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                    lemma_key_unique_subset(tree@, left@);
                    lemma_key_unique_subset(tree@, right@);
                    lemma_view_gen_subset::<K, V>(left@, tree@);
                    lemma_view_gen_subset::<K, V>(right@, tree@);
                }
                let left_size = left.size();
                let c = Ord::cmp(k, &root_pair.0);
                proof { reveal(vstd::laws_cmp::obeys_cmp_ord); }
                match c {
                    Less => {
                        let rank = ordkeymap_rank(&left, k);
                        proof {
                            let tree_dom = spec_pair_set_to_map(tree@).dom();
                            let left_dom = spec_pair_set_to_map(left@).dom();
                            assert(tree_dom.filter(rank_pred) =~= left_dom.filter(rank_pred)) by {
                                assert forall|x: K::V| #[trigger] tree_dom.filter(rank_pred).contains(x)
                                    implies left_dom.filter(rank_pred).contains(x) by {
                                    lemma_map_contains_pair_in_set(tree@, x);
                                    let xv: V::V = choose|xv: V::V| tree@.contains((x, xv));
                                    let t: K = choose|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, *k) && t@ != k@;
                                    if left@.contains((x, xv)) {
                                        lemma_pair_in_set_map_contains(left@, x, xv);
                                    } else if (x, xv) == root_pair@ {
                                        K::cmp_spec_less_implies_le(*k, root_pair.0);
                                        K::antisymmetric(t, *k);
                                    } else {
                                        let xp: Pair<K, V> = choose|xp: Pair<K, V>| #[trigger] right@.contains(xp@) && xp@ == (x, xv);
                                        assert(xp.0@ != root_pair.0@) by {
                                            if xp.0@ == root_pair.0@ {
                                                assert(tree@.contains(root_pair@));
                                            }
                                        };
                                        K::cmp_spec_greater_implies_le(xp.0, root_pair.0);
                                        K::cmp_spec_less_implies_le(*k, root_pair.0);
                                        K::transitive(*k, root_pair.0, t);
                                        K::antisymmetric(t, *k);
                                    }
                                };
                                assert forall|x: K::V| #[trigger] left_dom.filter(rank_pred).contains(x)
                                    implies tree_dom.filter(rank_pred).contains(x) by {
                                    lemma_map_contains_pair_in_set(left@, x);
                                    let xv: V::V = choose|xv: V::V| left@.contains((x, xv));
                                    lemma_pair_in_set_map_contains(tree@, x, xv);
                                };
                            };
                            lemma_pair_set_to_map_len(tree@);
                            tree_dom.lemma_len_filter(rank_pred);
                        }
                        rank
                    },
                    Equal => {
                        proof {
                            let tree_dom = spec_pair_set_to_map(tree@).dom();
                            let left_dom = spec_pair_set_to_map(left@).dom();
                            lemma_pair_set_to_map_dom_finite(left@);
                            lemma_pair_set_to_map_len(left@);
                            assert(tree_dom.filter(rank_pred) =~= left_dom) by {
                                assert forall|x: K::V| #[trigger] tree_dom.filter(rank_pred).contains(x)
                                    implies left_dom.contains(x) by {
                                    lemma_map_contains_pair_in_set(tree@, x);
                                    let xv: V::V = choose|xv: V::V| tree@.contains((x, xv));
                                    let t: K = choose|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, *k) && t@ != k@;
                                    if left@.contains((x, xv)) {
                                        lemma_pair_in_set_map_contains(left@, x, xv);
                                    } else if (x, xv) == root_pair@ {
                                    } else {
                                        let xp: Pair<K, V> = choose|xp: Pair<K, V>| #[trigger] right@.contains(xp@) && xp@ == (x, xv);
                                        K::cmp_spec_greater_implies_le(xp.0, root_pair.0);
                                        K::antisymmetric(t, *k);
                                    }
                                };
                                assert forall|x: K::V| #[trigger] left_dom.contains(x)
                                    implies tree_dom.filter(rank_pred).contains(x) by {
                                    lemma_map_contains_pair_in_set(left@, x);
                                    let xv: V::V = choose|xv: V::V| left@.contains((x, xv));
                                    lemma_pair_in_set_map_contains(tree@, x, xv);
                                    let xp: Pair<K, V> = choose|xp: Pair<K, V>| #[trigger] left@.contains(xp@) && xp@ == (x, xv);
                                    assert(xp.0@ != root_pair.0@) by {
                                        if xp.0@ == root_pair.0@ {
                                            assert(tree@.contains(root_pair@));
                                        }
                                    };
                                    K::cmp_spec_less_implies_le(xp.0, root_pair.0);
                                };
                            };
                            lemma_pair_set_to_map_len(tree@);
                            tree_dom.lemma_len_filter(rank_pred);
                        }
                        left_size
                    },
                    Greater => {
                        let right_rank = ordkeymap_rank(&right, k);
                        proof {
                            let tree_dom = spec_pair_set_to_map(tree@).dom();
                            let left_dom = spec_pair_set_to_map(left@).dom();
                            let right_dom = spec_pair_set_to_map(right@).dom();
                            lemma_pair_set_to_map_dom_finite(left@);
                            lemma_pair_set_to_map_dom_finite(right@);
                            lemma_pair_set_to_map_len(left@);
                            lemma_pair_set_to_map_len(right@);
                            let root_key_set = Set::empty().insert(root_pair.0@);
                            assert(tree_dom.filter(rank_pred) =~= left_dom.union(root_key_set).union(right_dom.filter(rank_pred))) by {
                                assert forall|x: K::V| #[trigger] left_dom.union(root_key_set).union(right_dom.filter(rank_pred)).contains(x)
                                    implies tree_dom.filter(rank_pred).contains(x) by {
                                    if left_dom.contains(x) {
                                        lemma_map_contains_pair_in_set(left@, x);
                                        let xv: V::V = choose|xv: V::V| left@.contains((x, xv));
                                        lemma_pair_in_set_map_contains(tree@, x, xv);
                                        let xp: Pair<K, V> = choose|xp: Pair<K, V>| #[trigger] left@.contains(xp@) && xp@ == (x, xv);
                                        assert(xp.0@ != root_pair.0@) by {
                                            if xp.0@ == root_pair.0@ {
                                                assert(tree@.contains(root_pair@));
                                            }
                                        };
                                        K::cmp_spec_less_implies_le(xp.0, root_pair.0);
                                        K::cmp_spec_greater_implies_le(*k, root_pair.0);
                                        K::transitive(xp.0, root_pair.0, *k);
                                    } else if root_key_set.contains(x) {
                                        lemma_pair_in_set_map_contains(tree@, root_pair.0@, root_pair.1@);
                                        K::cmp_spec_greater_implies_le(*k, root_pair.0);
                                    } else {
                                        lemma_map_contains_pair_in_set(right@, x);
                                        let xv: V::V = choose|xv: V::V| right@.contains((x, xv));
                                        lemma_pair_in_set_map_contains(tree@, x, xv);
                                    }
                                };
                            };
                            assert(!left_dom.contains(root_pair.0@)) by {
                                if left_dom.contains(root_pair.0@) {
                                    lemma_map_contains_pair_in_set(left@, root_pair.0@);
                                    let lv: V::V = choose|lv: V::V| left@.contains((root_pair.0@, lv));
                                    assert(tree@.contains((root_pair.0@, lv)));
                                    assert(tree@.contains(root_pair@));
                                }
                            };
                            assert(root_key_set.disjoint(right_dom.filter(rank_pred))) by {
                                assert forall|x: K::V| !(root_key_set.contains(x) && #[trigger] right_dom.filter(rank_pred).contains(x)) by {
                                    if root_key_set.contains(x) && right_dom.contains(x) {
                                        lemma_map_contains_pair_in_set(right@, x);
                                        let rv: V::V = choose|rv: V::V| right@.contains((x, rv));
                                        assert(tree@.contains((x, rv)));
                                        assert(tree@.contains(root_pair@));
                                    }
                                };
                            };
                            let lu = left_dom.union(root_key_set);
                            right_dom.lemma_len_filter(rank_pred);
                            vstd::set_lib::lemma_len_union(left_dom, root_key_set);
                            assert(lu.len() == left_dom.len() + 1) by {
                                vstd::set_lib::lemma_set_disjoint_lens(left_dom, root_key_set);
                            };
                            vstd::set_lib::lemma_set_disjoint_lens(lu, right_dom.filter(rank_pred));
                            tree_dom.lemma_len_filter(rank_pred);
                            lemma_pair_set_to_map_len(tree@);
                        }
                        left_size + 1 + right_rank
                    },
                }
            }
        }
    }

    /// Select by rank: find the key with rank i (i-th smallest) via BST descent. O(lg n).
    fn ordkeymap_select<K: StT + Ord + TotalOrder, V: StT + Ord>(
        tree: &ParamBST<Pair<K, V>>,
        i: usize,
    ) -> (selected: Option<K>)
        requires
            tree.spec_bstparasteph_wf(),
            spec_key_unique_pairs_set(tree@),
            spec_set_pair_view_generated::<K, V>(tree@),
            view_ord_consistent::<K>(),
            obeys_feq_fulls::<K, V>(),
            obeys_view_eq::<K>(),
            vstd::laws_cmp::obeys_cmp_spec::<K>(),
            spec_pair_key_determines_order::<K, V>(),
            view_ord_consistent::<Pair<K, V>>(),
            vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
        ensures
            spec_pair_set_to_map(tree@).dom().finite(),
            i >= spec_pair_set_to_map(tree@).dom().len() ==> selected matches None,
            selected matches Some(k) ==> spec_pair_set_to_map(tree@).dom().contains(k@),
            selected matches Some(v) ==> spec_pair_set_to_map(tree@).dom().filter(
                |x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, v) && t@ != v@
            ).len() == i as int,
        decreases tree@.len(),
    {
        proof {
            lemma_pair_set_to_map_dom_finite(tree@);
            lemma_pair_set_to_map_len(tree@);
            lemma_reveal_view_injective::<K>();
        }
        match tree.expose() {
            Exposed::Leaf => None,
            Exposed::Node(left, root_pair, right) => {
                reveal_param_bst_backings(&left);
                reveal_param_bst_backings(&right);
                proof {
                    reveal(vstd::laws_cmp::obeys_cmp_ord);
                    vstd::set_lib::lemma_set_disjoint_lens(left@, right@);
                    lemma_key_unique_subset(tree@, left@);
                    lemma_key_unique_subset(tree@, right@);
                    lemma_view_gen_subset::<K, V>(left@, tree@);
                    lemma_view_gen_subset::<K, V>(right@, tree@);
                    lemma_pair_set_to_map_dom_finite(left@);
                    lemma_pair_set_to_map_len(left@);
                }
                let left_size = left.size();
                if i < left_size {
                    let result = ordkeymap_select(&left, i);
                    proof {
                        if result is Some {
                            let sel_key = result->Some_0;
                            {
                                lemma_map_contains_pair_in_set(left@, sel_key@);
                                let sv: V::V = choose|sv: V::V| left@.contains((sel_key@, sv));
                                lemma_pair_in_set_map_contains(tree@, sel_key@, sv);
                                let rank_pred_sel = |x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, sel_key) && t@ != sel_key@;
                                let tree_dom = spec_pair_set_to_map(tree@).dom();
                                let left_dom = spec_pair_set_to_map(left@).dom();
                                assert(tree_dom.filter(rank_pred_sel) =~= left_dom.filter(rank_pred_sel)) by {
                                    assert forall|x: K::V| #[trigger] tree_dom.filter(rank_pred_sel).contains(x)
                                        implies left_dom.filter(rank_pred_sel).contains(x) by {
                                        lemma_map_contains_pair_in_set(tree@, x);
                                        let xv: V::V = choose|xv: V::V| tree@.contains((x, xv));
                                        let t: K = choose|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, sel_key) && t@ != sel_key@;
                                        if left@.contains((x, xv)) {
                                            lemma_pair_in_set_map_contains(left@, x, xv);
                                        } else if (x, xv) == root_pair@ {
                                            let sp: Pair<K, V> = choose|sp: Pair<K, V>| #[trigger] left@.contains(sp@) && sp@ == (sel_key@, sv);
                                            K::cmp_spec_less_implies_le(sp.0, root_pair.0);
                                            K::antisymmetric(t, sel_key);
                                        } else {
                                            let sp: Pair<K, V> = choose|sp: Pair<K, V>| #[trigger] left@.contains(sp@) && sp@ == (sel_key@, sv);
                                            let xp: Pair<K, V> = choose|xp: Pair<K, V>| #[trigger] right@.contains(xp@) && xp@ == (x, xv);
                                            assert(sp.0@ != root_pair.0@) by {
                                                if sp.0@ == root_pair.0@ {
                                                    assert(tree@.contains(root_pair@));
                                                }
                                            };
                                            assert(xp.0@ != root_pair.0@) by {
                                                if xp.0@ == root_pair.0@ {
                                                    assert(tree@.contains(root_pair@));
                                                }
                                            };
                                            K::cmp_spec_less_implies_le(sp.0, root_pair.0);
                                            K::cmp_spec_greater_implies_le(xp.0, root_pair.0);
                                            K::transitive(sel_key, root_pair.0, t);
                                            K::antisymmetric(t, sel_key);
                                        }
                                    };
                                    assert forall|x: K::V| #[trigger] left_dom.filter(rank_pred_sel).contains(x)
                                        implies tree_dom.filter(rank_pred_sel).contains(x) by {
                                        lemma_map_contains_pair_in_set(left@, x);
                                        let xv: V::V = choose|xv: V::V| left@.contains((x, xv));
                                        lemma_pair_in_set_map_contains(tree@, x, xv);
                                    };
                                };
                            }
                        }
                    }
                    result
                } else if i == left_size {
                    let key = root_pair.0.clone_plus();
                    proof {
                        lemma_cloned_view_eq(root_pair.0, key);
                        lemma_pair_in_set_map_contains(tree@, root_pair.0@, root_pair.1@);
                        let rank_pred_root = |x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, key) && t@ != key@;
                        let tree_dom = spec_pair_set_to_map(tree@).dom();
                        let left_dom = spec_pair_set_to_map(left@).dom();
                        assert(tree_dom.filter(rank_pred_root) =~= left_dom) by {
                            assert forall|x: K::V| #[trigger] tree_dom.filter(rank_pred_root).contains(x)
                                implies left_dom.contains(x) by {
                                lemma_map_contains_pair_in_set(tree@, x);
                                let xv: V::V = choose|xv: V::V| tree@.contains((x, xv));
                                let t: K = choose|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, key) && t@ != key@;
                                if left@.contains((x, xv)) {
                                    lemma_pair_in_set_map_contains(left@, x, xv);
                                } else if (x, xv) == root_pair@ {
                                } else {
                                    let xp: Pair<K, V> = choose|xp: Pair<K, V>| #[trigger] right@.contains(xp@) && xp@ == (x, xv);
// Veracity: TESTING assert                                     assert(xp.0@ != root_pair.0@) by {
// Veracity: TESTING assert                                         if xp.0@ == root_pair.0@ {
// Veracity: TESTING assert                                             assert(tree@.contains(xp@));
// Veracity: TESTING assert                                             assert(tree@.contains(root_pair@));
// Veracity: TESTING assert                                         }
// Veracity: TESTING assert                                     };
                                    K::cmp_spec_greater_implies_le(xp.0, root_pair.0);
                                    K::antisymmetric(t, key);
                                }
                            };
                            assert forall|x: K::V| #[trigger] left_dom.contains(x)
                                implies tree_dom.filter(rank_pred_root).contains(x) by {
                                lemma_map_contains_pair_in_set(left@, x);
                                let xv: V::V = choose|xv: V::V| left@.contains((x, xv));
                                lemma_pair_in_set_map_contains(tree@, x, xv);
                                let xp: Pair<K, V> = choose|xp: Pair<K, V>| #[trigger] left@.contains(xp@) && xp@ == (x, xv);
                                assert(xp.0@ != root_pair.0@) by {
                                    if xp.0@ == root_pair.0@ {
                                        assert(tree@.contains(xp@));
                                        assert(tree@.contains(root_pair@));
                                    }
                                };
                                K::cmp_spec_less_implies_le(xp.0, root_pair.0);
                            };
                        };
                    }
                    Some(key)
                } else {
                    let result = ordkeymap_select(&right, i - left_size - 1);
                    proof {
                        lemma_pair_set_to_map_dom_finite(right@);
                        lemma_pair_set_to_map_len(right@);
                        if result is Some {
                            let sel_key = result->Some_0;
                            {
                                lemma_map_contains_pair_in_set(right@, sel_key@);
                                let sv: V::V = choose|sv: V::V| right@.contains((sel_key@, sv));
                                lemma_pair_in_set_map_contains(tree@, sel_key@, sv);
                                let rank_pred_sel = |x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, sel_key) && t@ != sel_key@;
                                let tree_dom = spec_pair_set_to_map(tree@).dom();
                                let left_dom = spec_pair_set_to_map(left@).dom();
                                let right_dom = spec_pair_set_to_map(right@).dom();
                                let root_key_set = Set::empty().insert(root_pair.0@);
                                assert(tree_dom.filter(rank_pred_sel) =~= left_dom.union(root_key_set).union(right_dom.filter(rank_pred_sel))) by {
                                    assert forall|x: K::V| #[trigger] tree_dom.filter(rank_pred_sel).contains(x)
                                        implies left_dom.union(root_key_set).union(right_dom.filter(rank_pred_sel)).contains(x) by {
                                        lemma_map_contains_pair_in_set(tree@, x);
                                        let xv: V::V = choose|xv: V::V| tree@.contains((x, xv));
                                        if left@.contains((x, xv)) {
                                            lemma_pair_in_set_map_contains(left@, x, xv);
                                        } else if (x, xv) == root_pair@ {
                                        } else {
                                            lemma_pair_in_set_map_contains(right@, x, xv);
                                        }
                                    };
                                    assert forall|x: K::V| #[trigger] left_dom.union(root_key_set).union(right_dom.filter(rank_pred_sel)).contains(x)
                                        implies tree_dom.filter(rank_pred_sel).contains(x) by {
                                        if left_dom.contains(x) {
                                            lemma_map_contains_pair_in_set(left@, x);
                                            let xv: V::V = choose|xv: V::V| left@.contains((x, xv));
                                            lemma_pair_in_set_map_contains(tree@, x, xv);
                                            let xp: Pair<K, V> = choose|xp: Pair<K, V>| #[trigger] left@.contains(xp@) && xp@ == (x, xv);
                                            let sp: Pair<K, V> = choose|sp: Pair<K, V>| #[trigger] right@.contains(sp@) && sp@ == (sel_key@, sv);
                                            assert(xp.0@ != root_pair.0@) by {
                                                if xp.0@ == root_pair.0@ {
                                                    assert(tree@.contains(xp@));
                                                    assert(tree@.contains(root_pair@));
                                                }
                                            };
                                            assert(sp.0@ != root_pair.0@) by {
                                                if sp.0@ == root_pair.0@ {
                                                    assert(tree@.contains(sp@));
                                                    assert(tree@.contains(root_pair@));
                                                }
                                            };
                                            K::cmp_spec_less_implies_le(xp.0, root_pair.0);
                                            K::cmp_spec_greater_implies_le(sp.0, root_pair.0);
                                            K::transitive(xp.0, root_pair.0, sel_key);
                                        } else if root_key_set.contains(x) {
                                            lemma_pair_in_set_map_contains(tree@, root_pair.0@, root_pair.1@);
                                            let sp: Pair<K, V> = choose|sp: Pair<K, V>| #[trigger] right@.contains(sp@) && sp@ == (sel_key@, sv);
                                            assert(sp.0@ != root_pair.0@) by {
                                                if sp.0@ == root_pair.0@ {
                                                    assert(tree@.contains(sp@));
                                                    assert(tree@.contains(root_pair@));
                                                }
                                            };
                                            K::cmp_spec_greater_implies_le(sp.0, root_pair.0);
                                        } else {
                                            lemma_map_contains_pair_in_set(right@, x);
                                            let xv: V::V = choose|xv: V::V| right@.contains((x, xv));
                                            lemma_pair_in_set_map_contains(tree@, x, xv);
                                        }
                                    };
                                };
                                assert(!left_dom.contains(root_pair.0@)) by {
                                    if left_dom.contains(root_pair.0@) {
                                        lemma_map_contains_pair_in_set(left@, root_pair.0@);
                                        let lv: V::V = choose|lv: V::V| left@.contains((root_pair.0@, lv));
                                        assert(tree@.contains((root_pair.0@, lv)));
                                        assert(tree@.contains(root_pair@));
                                    }
                                };
                                assert(left_dom.disjoint(root_key_set)) by {
                                    assert forall|x: K::V| !(left_dom.contains(x) && #[trigger] root_key_set.contains(x)) by {};
                                };
                                assert(left_dom.disjoint(right_dom.filter(rank_pred_sel))) by {
                                    assert forall|x: K::V| !(left_dom.contains(x) && #[trigger] right_dom.filter(rank_pred_sel).contains(x)) by {
                                        if left_dom.contains(x) && right_dom.contains(x) {
                                            lemma_map_contains_pair_in_set(left@, x);
                                            lemma_map_contains_pair_in_set(right@, x);
                                            let lv: V::V = choose|lv: V::V| left@.contains((x, lv));
                                            let rv: V::V = choose|rv: V::V| right@.contains((x, rv));
                                            assert(tree@.contains((x, lv)));
                                            assert(tree@.contains((x, rv)));
                                        }
                                    };
                                };
                                assert(root_key_set.disjoint(right_dom.filter(rank_pred_sel))) by {
                                    assert forall|x: K::V| !(root_key_set.contains(x) && #[trigger] right_dom.filter(rank_pred_sel).contains(x)) by {
                                        if root_key_set.contains(x) && right_dom.contains(x) {
                                            lemma_map_contains_pair_in_set(right@, x);
                                            let rv: V::V = choose|rv: V::V| right@.contains((x, rv));
                                            assert(tree@.contains((x, rv)));
                                            assert(tree@.contains(root_pair@));
                                        }
                                    };
                                };
                                let lu = left_dom.union(root_key_set);
                                assert(lu.finite()) by {
                                    vstd::set_lib::lemma_len_union(left_dom, root_key_set);
                                };
                                right_dom.lemma_len_filter(rank_pred_sel);
                                vstd::set_lib::lemma_len_union(left_dom, root_key_set);
                                assert(lu.len() == left_dom.len() + 1) by {
                                    vstd::set_lib::lemma_set_disjoint_lens(left_dom, root_key_set);
                                };
                                vstd::set_lib::lemma_set_disjoint_lens(lu, right_dom.filter(rank_pred_sel));
                                tree_dom.lemma_len_filter(rank_pred_sel);
                            }
                        }
                    }
                    result
                }
            }
        }
    }

    impl<K: StT + Ord, V: StT + Ord> OrdKeyMapTrait<K, V> for OrdKeyMap<K, V> {
        open spec fn spec_ordkeymap_wf(&self) -> bool {
            self.inner.spec_bstparasteph_wf()
            && spec_key_unique_pairs_set(self.inner@)
            && self.inner@.len() < usize::MAX as nat
            && obeys_feq_fulls::<K, V>()
            && obeys_feq_full::<Pair<K, V>>()
            && vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>()
            && view_ord_consistent::<Pair<K, V>>()
            && spec_pair_key_determines_order::<K, V>()
            && vstd::laws_cmp::obeys_cmp_spec::<K>()
            && view_ord_consistent::<K>()
            && spec_set_pair_view_generated::<K, V>(self.inner@)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn new() -> (empty: Self)
        {
            let inner = ParamBST::<Pair<K, V>>::new();
            proof {
                lemma_set_to_map_empty::<K::V, V::V>();
                assert(spec_key_unique_pairs_set::<K::V, V::V>(inner@));
                assert(spec_set_pair_view_generated::<K, V>(inner@));
                lemma_pair_set_to_map_dom_finite(inner@);
            }
            OrdKeyMap { inner }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size(&self) -> (count: usize)
            ensures count == self@.dom().len(), self@.dom().finite()
        {
            let r = self.inner.size();
            proof {
                lemma_pair_set_to_map_len(self.inner@);
                lemma_pair_set_to_map_dom_finite(self.inner@);
            }
            r
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_empty(&self) -> (is_empty: bool)
            ensures is_empty == self@.dom().is_empty()
        {
            let r = self.inner.is_empty();
            proof {
                lemma_pair_set_to_map_dom_finite(self.inner@);
                lemma_pair_set_to_map_len(self.inner@);
            }
            r
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- delegates to ordkeymap_find
        fn find(&self, k: &K) -> (found: Option<V>)
        {
            ordkeymap_find(&self.inner, k)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- find + BST delete/insert
        fn insert(&mut self, k: K, v: V)
        {
            let existing = self.find(&k);
            let ghost old_inner_view = self.inner@;
            let ghost old_map = self@;
            match existing {
                Some(old_v) => {
                    // Key exists: replace with new value.
                    proof {
                        lemma_map_contains_pair_in_set(old_inner_view, k@);
                        let ghost v_chosen: V::V = choose|vv: V::V| old_inner_view.contains((k@, vv));
                        lemma_pair_in_set_map_contains(old_inner_view, k@, v_chosen);
                        assert(old_inner_view.contains((k@, old_v@)));
                    }
                    // Delete old pair, insert new pair.
                    let k_del = k.clone_plus();
                    let ov_del = old_v.clone_plus();
                    proof {
                        lemma_cloned_view_eq(k, k_del);
                        lemma_cloned_view_eq(old_v, ov_del);
                    }
                    self.inner.delete(&Pair(k_del, ov_del));
                    let ghost mid_inner_view = self.inner@;
                    proof {
                        lemma_set_to_map_remove_pair(old_inner_view, k@, old_v@);
                        lemma_key_unique_remove(old_inner_view, (k@, old_v@));
                        assert(!spec_pair_set_to_map(mid_inner_view).dom().contains(k@));
                    }
                    let k_clone = k.clone_plus();
                    proof { lemma_cloned_view_eq(k, k_clone); }
                    self.inner.insert(Pair(k_clone, v));
                    proof {
                        lemma_set_to_map_insert(mid_inner_view, k@, v@);
                        lemma_key_unique_insert(mid_inner_view, k@, v@);
                        lemma_pair_set_to_map_dom_finite(self.inner@);
                        let ghost new_map = spec_pair_set_to_map(self.inner@);
                        assert(new_map =~= old_map.remove(k@).insert(k@, v@));
                        assert(new_map.contains_key(k@));
                        assert(new_map[k@] == v@);
                        assert(new_map.dom() =~= old_map.dom().insert(k@)) by {
                            assert(old_map.dom().contains(k@));
                            assert(old_map.remove(k@).insert(k@, v@).dom()
                                =~= old_map.dom().remove(k@).insert(k@));
                            assert(old_map.dom().remove(k@).insert(k@)
                                =~= old_map.dom());
                        };
                        assert forall|key: K::V| key != k@ && #[trigger] old_map.contains_key(key)
                            implies new_map[key] == old_map[key]
                        by {
                            assert(new_map[key] == old_map.remove(k@).insert(k@, v@)[key]);
                        };
                        assert(self.inner@.len() <= old_inner_view.len());
                        assert(self.inner@.len() < usize::MAX as nat);
                        // View generation maintained.
                        assert(spec_set_pair_view_generated::<K, V>(self.inner@)) by {
                            assert forall|elem: (K::V, V::V)| self.inner@.contains(elem)
                                implies exists|p: Pair<K, V>| (#[trigger] p@) == elem by {
                                if elem == (k@, v@) {
                                    assert(Pair(k_clone, v)@ == elem);
                                } else {
                                    assert(mid_inner_view.contains(elem));
                                    // mid_inner_view subset_of old_inner_view.
                                    assert(old_inner_view.contains(elem));
                                }
                            };
                        };
                    }
                },
                None => {
                    // Key does not exist: insert new pair.
                    self.inner.insert(Pair(k, v));
                    proof {
                        lemma_set_to_map_insert(old_inner_view, k@, v@);
                        lemma_pair_set_to_map_dom_finite(self.inner@);
                        lemma_key_unique_insert(old_inner_view, k@, v@);
                        lemma_pair_set_to_map_len(old_inner_view);
                        assert(self.inner@.len() < usize::MAX as nat);
                        // View generation maintained.
                        lemma_view_gen_insert::<K, V>(old_inner_view, Pair(k, v));
                    }
                },
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- find + BST delete
        fn delete(&mut self, k: &K)
        {
            let existing = self.find(k);
            let ghost old_inner_view = self.inner@;
            let ghost old_map = self@;
            match existing {
                Some(v) => {
                    let v_clone = v.clone_plus();
                    let k_clone = k.clone_plus();
                    proof {
                        lemma_cloned_view_eq(*k, k_clone);
                        lemma_cloned_view_eq(v, v_clone);
                    }
                    self.inner.delete(&Pair(k_clone, v_clone));
                    proof {
                        lemma_set_to_map_remove_pair(old_inner_view, k@, v@);
                        lemma_pair_set_to_map_dom_finite(self.inner@);
                        lemma_key_unique_remove(old_inner_view, (k@, v@));
                        // View generation: self.inner@ subset_of old_inner_view.
                        lemma_view_gen_subset::<K, V>(self.inner@, old_inner_view);
                    }
                },
                None => {
                    proof {
                        assert(self@ =~= old_map.remove(k@));
                        lemma_pair_set_to_map_dom_finite(self.inner@);
                    }
                },
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- delegates to ordkeymap_split
        fn split(&self, k: &K) -> (parts: (Self, Option<V>, Self))
        {
            let (left_bst, found, right_bst) = ordkeymap_split(&self.inner, k);
            let left = OrdKeyMap { inner: left_bst };
            let right = OrdKeyMap { inner: right_bst };
            proof {
                lemma_pair_set_to_map_dom_finite(left_bst@);
                lemma_pair_set_to_map_dom_finite(right_bst@);
                // Map-level ensures from BST-level ensures.
                // Left: every key in left map is in self map with same value.
                assert forall|key: K::V| #[trigger] left@.contains_key(key)
                    implies self@.contains_key(key) && left@[key] == self@[key]
                by {
                    lemma_map_contains_pair_in_set(left_bst@, key);
                    let vv: V::V = choose|vv: V::V| left_bst@.contains((key, vv));
                    assert(self.inner@.contains((key, vv)));
                    lemma_pair_in_set_map_contains(self.inner@, key, vv);
                    lemma_pair_in_set_map_contains(left_bst@, key, vv);
                };
                // Right: every key in right map is in self map with same value.
                assert forall|key: K::V| #[trigger] right@.contains_key(key)
                    implies self@.contains_key(key) && right@[key] == self@[key]
                by {
                    lemma_map_contains_pair_in_set(right_bst@, key);
                    let vv: V::V = choose|vv: V::V| right_bst@.contains((key, vv));
                    assert(self.inner@.contains((key, vv)));
                    lemma_pair_in_set_map_contains(self.inner@, key, vv);
                    lemma_pair_in_set_map_contains(right_bst@, key, vv);
                };
                // Completeness: every key in self is in left, right, or equals k.
                assert forall|key: K::V| self@.contains_key(key)
                    implies #[trigger] left@.contains_key(key) || right@.contains_key(key) || key == k@
                by {
                    lemma_map_contains_pair_in_set(self.inner@, key);
                    let vv: V::V = choose|vv: V::V| self.inner@.contains((key, vv));
                    // From ordkeymap_split completeness: in left_bst or right_bst or key == k@.
                    if left_bst@.contains((key, vv)) {
                        lemma_pair_in_set_map_contains(left_bst@, key, vv);
                    } else if right_bst@.contains((key, vv)) {
                        lemma_pair_in_set_map_contains(right_bst@, key, vv);
                    }
                };
                // wf propagation.
                assert(left.inner@.len() < usize::MAX as nat) by {
                    vstd::set_lib::lemma_len_subset(left_bst@, self.inner@);
                };
                assert(right.inner@.len() < usize::MAX as nat) by {
                    vstd::set_lib::lemma_len_subset(right_bst@, self.inner@);
                };
                // Disjointness: BST sets are disjoint, so map domains are disjoint.
                assert(left@.dom().disjoint(right@.dom())) by {
                    assert forall|key: K::V|
                        left@.dom().contains(key) && right@.dom().contains(key)
                        implies false
                    by {
                        lemma_map_contains_pair_in_set(left_bst@, key);
                        lemma_map_contains_pair_in_set(right_bst@, key);
                        let lv: V::V = choose|lv: V::V| left_bst@.contains((key, lv));
                        let rv: V::V = choose|rv: V::V| right_bst@.contains((key, rv));
                        // left_bst and right_bst are disjoint sets.
                        assert(left_bst@.disjoint(right_bst@));
                        assert(left_bst@.contains((key, lv)));
                        assert(right_bst@.contains((key, rv)));
                        // But they share key, so both contain pairs with that key.
                        // BST split ensures left keys < k and right keys > k, so no overlap.
                    };
                };
            }
            (left, found, right)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m)
        fn union(&self, other: &Self) -> (combined: Self)
        {
            let ghost self_tree = self.inner@;
            let ghost self_map = self@;
            let ghost other_map = other@;
            proof {
                lemma_pair_set_to_map_len(self_tree);
                lemma_pair_set_to_map_len(other.inner@);
            }
            let self_sorted = self.inner.in_order();
            let self_len = self_sorted.length();
            let mut new_tree = ParamBST::<Pair<K, V>>::new();
            let mut i: usize = 0;
            proof {
                lemma_sorted_keys_pairwise_distinct(self_tree, self_sorted@);
                lemma_key_unique_empty::<K::V, V::V>();
                assert(spec_set_pair_view_generated::<K, V>(new_tree@)) by {
                    assert forall|elem: (K::V, V::V)| new_tree@.contains(elem)
                        implies exists|p: Pair<K, V>| (#[trigger] p@) == elem by {
                        assert(false);
                    };
                };
                assert(obeys_view_eq_trigger::<K>());
            }
            while i < self_len
                invariant
                    self.inner@ == self_tree,
                    self.spec_ordkeymap_wf(),
                    other.spec_ordkeymap_wf(),
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<V>(),
                    obeys_feq_full::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
                    self_len as nat == self_sorted@.len(),
                    self_sorted@.len() == self_tree.len(),
                    forall|v: <Pair<K, V> as View>::V| self_tree.contains(v) <==> #[trigger] self_sorted@.contains(v),
                    forall|ii: int, jj: int|
                        0 <= ii < self_sorted@.len() && 0 <= jj < self_sorted@.len() && ii != jj
                        ==> (#[trigger] self_sorted@[ii]).0 != (#[trigger] self_sorted@[jj]).0,
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        exists|j: int| 0 <= j < i as int && p.0 == (#[trigger] self_sorted@[j]).0,
                    0 <= i <= self_len,
                    new_tree.spec_bstparasteph_wf(),
                    vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                    view_ord_consistent::<Pair<K, V>>(),
                    new_tree@.len() == i as nat,
                    new_tree@.len() < usize::MAX as nat,
                    spec_key_unique_pairs_set(new_tree@),
                    spec_key_unique_pairs_set(self_tree),
                    self_map == spec_pair_set_to_map(self_tree),
                    other_map == other@,
                    // Phase 1 completeness.
                    forall|j2: int| 0 <= j2 < i as int
                        ==> #[trigger] spec_pair_set_to_map(new_tree@).dom().contains(self_sorted@[j2].0),
                    // Phase 1 value tracking: map-level.
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        self_map.dom().contains(p.0) &&
                        ((!other_map.dom().contains(p.0) && p.1 == self_map[p.0])
                        || (other_map.dom().contains(p.0) && p.1 == other_map[p.0])),
                    spec_set_pair_view_generated::<K, V>(new_tree@),
                decreases self_len - i,
            {
                let pair = self_sorted.nth(i);
                proof { reveal(obeys_view_eq); }
                let other_find = other.find(&pair.0);
                let ghost old_new_tree_view = new_tree@;
                proof {
                    assert(self_sorted@.contains(self_sorted@[i as int])) by { assert(self_sorted@[i as int] == self_sorted@[i as int]); };
                    lemma_pair_in_set_map_contains(self_tree, self_sorted@[i as int].0, self_sorted@[i as int].1);
                }
                match other_find {
                    Some(ov) => {
                        let key_clone = pair.0.clone_plus();
                        proof { lemma_cloned_view_eq(pair.0, key_clone); }
                        new_tree.insert(Pair(key_clone, ov));
                        proof {
                            lemma_view_gen_insert::<K, V>(old_new_tree_view, Pair(key_clone, ov));
                            lemma_key_unique_insert(old_new_tree_view, self_sorted@[i as int].0, ov@);
                            lemma_pair_in_set_map_contains(new_tree@, self_sorted@[i as int].0, ov@);
                            assert forall|j2: int| 0 <= j2 < i as int
                                implies #[trigger] spec_pair_set_to_map(new_tree@).dom().contains(self_sorted@[j2].0)
                            by {
                                lemma_map_contains_pair_in_set(old_new_tree_view, self_sorted@[j2].0);
                                let w: V::V = choose|w: V::V| old_new_tree_view.contains((self_sorted@[j2].0, w));
                                lemma_pair_in_set_map_contains(new_tree@, self_sorted@[j2].0, w);
                            };
                            assert forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) implies
                                self_map.dom().contains(p.0) &&
                                ((!other_map.dom().contains(p.0) && p.1 == self_map[p.0])
                                || (other_map.dom().contains(p.0) && p.1 == other_map[p.0]))
                            by {
                                if old_new_tree_view.contains(p) {
                                } else {
                                }
                            };
                        }
                    },
                    None => {
                        let cloned = pair.clone_plus();
                        proof { lemma_cloned_view_eq(*pair, cloned); }
                        new_tree.insert(cloned);
                        proof {
                            lemma_view_gen_insert::<K, V>(old_new_tree_view, cloned);
                            lemma_key_unique_insert(old_new_tree_view, self_sorted@[i as int].0, self_sorted@[i as int].1);
                            lemma_pair_in_set_map_contains(new_tree@, self_sorted@[i as int].0, self_sorted@[i as int].1);
                            assert forall|j2: int| 0 <= j2 < i as int
                                implies #[trigger] spec_pair_set_to_map(new_tree@).dom().contains(self_sorted@[j2].0)
                            by {
                                lemma_map_contains_pair_in_set(old_new_tree_view, self_sorted@[j2].0);
                                let w: V::V = choose|w: V::V| old_new_tree_view.contains((self_sorted@[j2].0, w));
                                lemma_pair_in_set_map_contains(new_tree@, self_sorted@[j2].0, w);
                            };
                            assert forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) implies
                                self_map.dom().contains(p.0) &&
                                ((!other_map.dom().contains(p.0) && p.1 == self_map[p.0])
                                || (other_map.dom().contains(p.0) && p.1 == other_map[p.0]))
                            by {
                                if old_new_tree_view.contains(p) {
                                } else {
                                }
                            };
                        }
                    },
                }
                i += 1;
            }
            // Phase 2: iterate other entries, add those not in self.
            let other_sorted = other.inner.in_order();
            let other_len = other_sorted.length();
            let mut j: usize = 0;
            proof {
                lemma_sorted_keys_pairwise_distinct(other.inner@, other_sorted@);
                assert forall|kv: K::V| #[trigger] self_map.dom().contains(kv)
                    implies spec_pair_set_to_map(new_tree@).dom().contains(kv)
                by {
                    lemma_map_contains_pair_in_set(self_tree, kv);
                    let vv: V::V = choose|vv: V::V| self_tree.contains((kv, vv));
                    assert(self_sorted@.contains((kv, vv)));
                    let jx: int = choose|jx: int| 0 <= jx < self_sorted@.len() as int && self_sorted@[jx] == (kv, vv);
                    assert(spec_pair_set_to_map(new_tree@).dom().contains(self_sorted@[jx].0));
                };
                lemma_pair_set_to_map_len(self_tree);
                lemma_pair_set_to_map_len(other.inner@);
                assert(obeys_view_eq_trigger::<K>());
            }
            while j < other_len
                invariant
                    self.inner@ == self_tree,
                    self.spec_ordkeymap_wf(),
                    other.spec_ordkeymap_wf(),
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
                    obeys_feq_full::<V>(),
                    obeys_feq_full::<K>(),
                    other_map == other@,
                    self_map == spec_pair_set_to_map(self_tree),
                    other_len as nat == other_sorted@.len(),
                    other_sorted@.len() == other.inner@.len(),
                    self_sorted@.len() == self_tree.len(),
                    forall|v: <Pair<K, V> as View>::V| other.inner@.contains(v) <==> #[trigger] other_sorted@.contains(v),
                    forall|v: <Pair<K, V> as View>::V| self_tree.contains(v) <==> #[trigger] self_sorted@.contains(v),
                    forall|ii: int, jj: int|
                        0 <= ii < other_sorted@.len() && 0 <= jj < other_sorted@.len() && ii != jj
                        ==> (#[trigger] other_sorted@[ii]).0 != (#[trigger] other_sorted@[jj]).0,
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        self_map.dom().contains(p.0) ||
                        (exists|j2: int| 0 <= j2 < j as int && p.0 == (#[trigger] other_sorted@[j2]).0),
                    0 <= j <= other_len,
                    new_tree.spec_bstparasteph_wf(),
                    vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                    view_ord_consistent::<Pair<K, V>>(),
                    new_tree@.len() <= self_sorted@.len() + j as nat,
                    self_sorted@.len() + other_sorted@.len() < usize::MAX as nat,
                    spec_key_unique_pairs_set(new_tree@),
                    spec_key_unique_pairs_set(self_tree),
                    // Old keys preserved.
                    forall|kv: K::V| #[trigger] self_map.dom().contains(kv)
                        ==> spec_pair_set_to_map(new_tree@).dom().contains(kv),
                    // Other completeness.
                    forall|j2: int| 0 <= j2 < j as int && !self_map.dom().contains(other_sorted@[j2].0)
                        ==> #[trigger] spec_pair_set_to_map(new_tree@).dom().contains(other_sorted@[j2].0),
                    // Phase 2 value tracking: map-level.
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        (self_map.dom().contains(p.0) &&
                            ((!other_map.dom().contains(p.0) && p.1 == self_map[p.0])
                            || (other_map.dom().contains(p.0) && p.1 == other_map[p.0])))
                        || (!self_map.dom().contains(p.0) && other_map.dom().contains(p.0) && p.1 == other_map[p.0]),
                    spec_set_pair_view_generated::<K, V>(new_tree@),
                decreases other_len - j,
            {
                let pair = other_sorted.nth(j);
                proof { reveal(obeys_view_eq); }
                let in_self = self.find(&pair.0);
                match in_self {
                    None => {
                        let cloned = pair.clone_plus();
                        let ghost old_new_tree_view = new_tree@;
                        proof {
                            lemma_cloned_view_eq(*pair, cloned);
                            assert(other_sorted@.contains(other_sorted@[j as int])) by {
                                assert(other_sorted@[j as int] == other_sorted@[j as int]);
                            };
                            lemma_pair_in_set_map_contains(other.inner@, other_sorted@[j as int].0, other_sorted@[j as int].1);
                        }
                        new_tree.insert(cloned);
                        proof {
                            lemma_view_gen_insert::<K, V>(old_new_tree_view, cloned);
                            lemma_key_unique_insert(old_new_tree_view, other_sorted@[j as int].0, other_sorted@[j as int].1);
                            assert forall|kv: K::V| #[trigger] self_map.dom().contains(kv)
                                implies spec_pair_set_to_map(new_tree@).dom().contains(kv)
                            by {
                                lemma_map_contains_pair_in_set(old_new_tree_view, kv);
                                let w: V::V = choose|w: V::V| old_new_tree_view.contains((kv, w));
                                lemma_pair_in_set_map_contains(new_tree@, kv, w);
                            };
                            lemma_pair_in_set_map_contains(new_tree@, other_sorted@[j as int].0, other_sorted@[j as int].1);
                            assert forall|j2: int| 0 <= j2 < j as int + 1
                                && !self_map.dom().contains(other_sorted@[j2].0)
                                implies #[trigger] spec_pair_set_to_map(new_tree@).dom().contains(other_sorted@[j2].0)
                            by {
                                if j2 == j as int {
                                } else {
                                    lemma_map_contains_pair_in_set(old_new_tree_view, other_sorted@[j2].0);
                                    let w: V::V = choose|w: V::V| old_new_tree_view.contains((other_sorted@[j2].0, w));
                                    lemma_pair_in_set_map_contains(new_tree@, other_sorted@[j2].0, w);
                                }
                            };
                            assert forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) implies
                                (self_map.dom().contains(p.0) &&
                                    ((!other_map.dom().contains(p.0) && p.1 == self_map[p.0])
                                    || (other_map.dom().contains(p.0) && p.1 == other_map[p.0])))
                                || (!self_map.dom().contains(p.0) && other_map.dom().contains(p.0) && p.1 == other_map[p.0])
                            by {
                                if old_new_tree_view.contains(p) {
                                } else {
                                }
                            };
                        }
                    },
                    Some(_) => {},
                }
                j += 1;
            }
            let combined = OrdKeyMap { inner: new_tree };
            proof {
                lemma_pair_set_to_map_dom_finite(new_tree@);
                lemma_pair_set_to_map_dom_finite(self_tree);
                lemma_pair_set_to_map_dom_finite(other.inner@);
                assert(combined@.dom() =~= self_map.dom().union(other_map.dom())) by {
                    assert forall|kv: K::V| combined@.dom().contains(kv)
                        implies #[trigger] self_map.dom().union(other_map.dom()).contains(kv)
                    by {
                        lemma_map_contains_pair_in_set(new_tree@, kv);
                    };
                    assert forall|kv: K::V| #[trigger] self_map.dom().union(other_map.dom()).contains(kv)
                        implies combined@.dom().contains(kv)
                    by {
                        if self_map.dom().contains(kv) {
                        } else {
                            lemma_map_contains_pair_in_set(other.inner@, kv);
                            let vv: V::V = choose|vv: V::V| other.inner@.contains((kv, vv));
                            assert(other_sorted@.contains((kv, vv)));
                            let jx: int = choose|jx: int| 0 <= jx < other_sorted@.len() as int && other_sorted@[jx] == (kv, vv);
                            assert(!self_map.dom().contains(other_sorted@[jx].0));
                        }
                    };
                };
                assert forall|k: K::V| #[trigger] self_map.contains_key(k) && !other_map.contains_key(k)
                    implies combined@[k] == self_map[k]
                by {
                    lemma_map_contains_pair_in_set(new_tree@, k);
                    let vv: V::V = choose|vv: V::V| new_tree@.contains((k, vv));
                    lemma_pair_in_set_map_contains(new_tree@, k, vv);
                };
                assert forall|k: K::V| #[trigger] other_map.contains_key(k)
                    implies combined@[k] == other_map[k]
                by {
                    lemma_map_contains_pair_in_set(new_tree@, k);
                    let vv: V::V = choose|vv: V::V| new_tree@.contains((k, vv));
                    lemma_pair_in_set_map_contains(new_tree@, k, vv);
                };
            }
            combined
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m)
        fn intersect(&self, other: &Self) -> (common: Self)
        {
            let ghost self_tree = self.inner@;
            let ghost self_map = self@;
            let ghost other_map = other@;
            let sorted = self.inner.in_order();
            let len = sorted.length();
            let mut new_tree = ParamBST::<Pair<K, V>>::new();
            let mut i: usize = 0;
            proof {
                assert(obeys_feq_full_trigger::<V>());
                assert(obeys_feq_full_trigger::<K>());
                lemma_pair_set_to_map_dom_finite(self_tree);
                lemma_sorted_keys_pairwise_distinct(self_tree, sorted@);
                lemma_key_unique_empty::<K::V, V::V>();
                assert(spec_set_pair_view_generated::<K, V>(new_tree@)) by {
                    assert forall|elem: (K::V, V::V)| new_tree@.contains(elem)
                        implies exists|p: Pair<K, V>| (#[trigger] p@) == elem by {
                        assert(false);
                    };
                };
                assert(obeys_view_eq_trigger::<K>());
            }
            while i < len
                invariant
                    self.inner@ == self_tree,
                    self.spec_ordkeymap_wf(),
                    other.spec_ordkeymap_wf(),
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<V>(),
                    obeys_feq_full::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
                    self_map == spec_pair_set_to_map(self_tree),
                    other_map == other@,
                    len as nat == sorted@.len(),
                    sorted@.len() == self_tree.len(),
                    0 <= i <= len,
                    new_tree.spec_bstparasteph_wf(),
                    vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                    view_ord_consistent::<Pair<K, V>>(),
                    new_tree@.len() <= i as nat,
                    new_tree@.len() < usize::MAX as nat,
                    spec_key_unique_pairs_set(new_tree@),
                    spec_key_unique_pairs_set(self_tree),
                    forall|v: <Pair<K, V> as View>::V| self_tree.contains(v) <==> #[trigger] sorted@.contains(v),
                    forall|ii: int, jj: int|
                        0 <= ii < sorted@.len() && 0 <= jj < sorted@.len() && ii != jj
                        ==> (#[trigger] sorted@[ii]).0 != (#[trigger] sorted@[jj]).0,
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        exists|j: int| 0 <= j < i as int && p.0 == (#[trigger] sorted@[j]).0,
                    // All entries in new_tree are from self_tree (values from self).
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==> self_tree.contains(p),
                    // Entries in new_tree have keys in other's domain.
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        other_map.dom().contains(p.0),
                    // Completeness: processed keys in other are in new_tree.
                    forall|j2: int| 0 <= j2 < i as int && other_map.dom().contains(sorted@[j2].0)
                        ==> #[trigger] spec_pair_set_to_map(new_tree@).dom().contains(sorted@[j2].0),
                    spec_set_pair_view_generated::<K, V>(new_tree@),
                decreases len - i,
            {
                let pair = sorted.nth(i);
                proof { reveal(obeys_view_eq); }
                let other_find = other.find(&pair.0);
                match other_find {
                    Some(_) => {
                        let cloned = pair.clone_plus();
                        let ghost old_new_tree_view = new_tree@;
                        proof {
                            lemma_cloned_view_eq(*pair, cloned);
                            // Freshness proof.
                            assert(!spec_pair_set_to_map(old_new_tree_view).dom().contains(sorted@[i as int].0)) by {
                                if spec_pair_set_to_map(old_new_tree_view).dom().contains(sorted@[i as int].0) {
                                    lemma_map_contains_pair_in_set(old_new_tree_view, sorted@[i as int].0);
                                    let vv: V::V = choose|vv: V::V| old_new_tree_view.contains((sorted@[i as int].0, vv));
                                    let jj = choose|jj: int| 0 <= jj < i as int && (sorted@[i as int].0, vv).0 == (#[trigger] sorted@[jj]).0;
                                    assert(false);
                                }
                            };
                            assert(sorted@.contains(sorted@[i as int])) by { assert(sorted@[i as int] == sorted@[i as int]); };
                            assert(self_tree.contains(sorted@[i as int]));
                        }
                        new_tree.insert(cloned);
                        proof {
                            lemma_view_gen_insert::<K, V>(old_new_tree_view, cloned);
                            assert(new_tree@.len() <= i as nat + 1);
                            assert(new_tree@.len() < usize::MAX as nat);
                            lemma_key_unique_insert(old_new_tree_view, sorted@[i as int].0, sorted@[i as int].1);
                            // Completeness maintenance.
                            lemma_pair_in_set_map_contains(new_tree@, sorted@[i as int].0, sorted@[i as int].1);
                            assert forall|j2: int| 0 <= j2 < i as int && other_map.dom().contains(sorted@[j2].0)
                                implies #[trigger] spec_pair_set_to_map(new_tree@).dom().contains(sorted@[j2].0)
                            by {
                                lemma_map_contains_pair_in_set(old_new_tree_view, sorted@[j2].0);
                                let w: V::V = choose|w: V::V| old_new_tree_view.contains((sorted@[j2].0, w));
                                assert(new_tree@.contains((sorted@[j2].0, w)));
                                lemma_pair_in_set_map_contains(new_tree@, sorted@[j2].0, w);
                            };
                            assert(new_tree@.contains(sorted@[i as int]));
                        }
                    },
                    None => {},
                }
                i += 1;
            }
            let common = OrdKeyMap { inner: new_tree };
            proof {
                lemma_pair_set_to_map_dom_finite(new_tree@);
                lemma_pair_set_to_map_dom_finite(self_tree);
                // Domain proof: common@.dom() =~= self_map.dom().intersect(other_map.dom()).
                assert(common@.dom() =~= self_map.dom().intersect(other_map.dom())) by {
                    // Forward: key in common → key in self AND other.
                    assert forall|k: K::V| #[trigger] common@.dom().contains(k)
                        implies self_map.dom().contains(k) && other_map.dom().contains(k)
                    by {
                        lemma_map_contains_pair_in_set(new_tree@, k);
                        let v: V::V = choose|v: V::V| new_tree@.contains((k, v));
                        assert(self_tree.contains((k, v)));
                        lemma_pair_in_set_map_contains(self_tree, k, v);
                    };
                    // Backward: key in self AND other → key in common.
                    assert forall|k: K::V|
                        self_map.dom().contains(k) && other_map.dom().contains(k)
                        implies #[trigger] common@.dom().contains(k)
                    by {
                        lemma_map_contains_pair_in_set(self_tree, k);
                        let v: V::V = choose|v: V::V| self_tree.contains((k, v));
                        assert(sorted@.contains((k, v)));
                        let j = choose|j: int| 0 <= j < sorted@.len() && sorted@[j] == (k, v);
                        assert(spec_pair_set_to_map(new_tree@).dom().contains(sorted@[j].0));
                        assert(sorted@[j].0 == k);
                    };
                };
                // Value proof: values come from self.
                assert forall|k: K::V| #[trigger] common@.contains_key(k)
                    implies common@[k] == self_map[k]
                by {
                    lemma_map_contains_pair_in_set(new_tree@, k);
                    let v: V::V = choose|v: V::V| new_tree@.contains((k, v));
                    assert(self_tree.contains((k, v)));
                    lemma_pair_in_set_map_contains(new_tree@, k, v);
                    lemma_pair_in_set_map_contains(self_tree, k, v);
                };
                // WF.
                assert(new_tree@.len() < usize::MAX as nat);
                assert(spec_pair_key_determines_order::<K, V>());
                assert(vstd::laws_cmp::obeys_cmp_spec::<K>());
                assert(view_ord_consistent::<K>());
                assert(obeys_feq_fulls::<K, V>());
            }
            common
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m)
        fn union_with<F: Fn(&V, &V) -> V>(&self, other: &Self, combine: &F) -> (combined: Self)
        {
            let ghost self_tree = self.inner@;
            let ghost self_map = self@;
            let ghost other_map = other@;
            proof {
                lemma_pair_set_to_map_len(self_tree);
                lemma_pair_set_to_map_len(other.inner@);
            }
            // Phase 1: iterate self entries, merge with other where overlapping.
            let self_sorted = self.inner.in_order();
            let self_len = self_sorted.length();
            let mut new_tree = ParamBST::<Pair<K, V>>::new();
            let mut i: usize = 0;
            proof {
                lemma_sorted_keys_pairwise_distinct(self_tree, self_sorted@);
                lemma_key_unique_empty::<K::V, V::V>();
                assert(obeys_view_eq_trigger::<K>());
            }
            while i < self_len
                invariant
                    self.inner@ == self_tree,
                    self.spec_ordkeymap_wf(),
                    other.spec_ordkeymap_wf(),
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<V>(),
                    obeys_feq_full::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
                    forall|v1: &V, v2: &V| #[trigger] combine.requires((v1, v2)),
                    self_len as nat == self_sorted@.len(),
                    self_sorted@.len() == self_tree.len(),
                    forall|v: <Pair<K, V> as View>::V| self_tree.contains(v) <==> #[trigger] self_sorted@.contains(v),
                    forall|ii: int, jj: int|
                        0 <= ii < self_sorted@.len() && 0 <= jj < self_sorted@.len() && ii != jj
                        ==> (#[trigger] self_sorted@[ii]).0 != (#[trigger] self_sorted@[jj]).0,
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        exists|j: int| 0 <= j < i as int && p.0 == (#[trigger] self_sorted@[j]).0,
                    0 <= i <= self_len,
                    new_tree.spec_bstparasteph_wf(),
                    vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                    view_ord_consistent::<Pair<K, V>>(),
                    new_tree@.len() == i as nat,
                    new_tree@.len() < usize::MAX as nat,
                    spec_key_unique_pairs_set(new_tree@),
                    spec_key_unique_pairs_set(self_tree),
                    self_map == spec_pair_set_to_map(self_tree),
                    other_map == other@,
                    // Phase 1 completeness.
                    forall|j2: int| 0 <= j2 < i as int
                        ==> #[trigger] spec_pair_set_to_map(new_tree@).dom().contains(self_sorted@[j2].0),
                    // Phase 1 value tracking (unified per-pair).
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        self_map.dom().contains(p.0) &&
                        ((!other_map.dom().contains(p.0) && self_tree.contains(p))
                        || (other_map.dom().contains(p.0) && exists|v1: V, v2: V, r: V|
                            v1@ == self_map[p.0] && v2@ == other_map[p.0]
                            && combine.ensures((&v1, &v2), r) && p.1 == r@)),
                    spec_set_pair_view_generated::<K, V>(new_tree@),
                decreases self_len - i,
            {
                let pair = self_sorted.nth(i);
                proof { reveal(obeys_view_eq); }
                let other_find = other.find(&pair.0);
                let ghost old_new_tree_view = new_tree@;
                proof {
                    // Freshness: self_sorted@[i].0 not in new_tree.
                    // Link sorted entry to self_tree.
                    assert(self_sorted@.contains(self_sorted@[i as int])) by { assert(self_sorted@[i as int] == self_sorted@[i as int]); };
                    lemma_pair_in_set_map_contains(self_tree, self_sorted@[i as int].0, self_sorted@[i as int].1);
                }
                match other_find {
                    Some(ov) => {
                        // Both maps have this key — use combine.
                        let combined_v = combine(&pair.1, &ov);
                        let key_clone = pair.0.clone_plus();
                        proof { lemma_cloned_view_eq(pair.0, key_clone); }
                        new_tree.insert(Pair(key_clone, combined_v));
                        proof {
                            lemma_view_gen_insert::<K, V>(old_new_tree_view, Pair(key_clone, combined_v));
                            lemma_key_unique_insert(old_new_tree_view, self_sorted@[i as int].0, combined_v@);
                            // Completeness maintenance.
                            lemma_pair_in_set_map_contains(new_tree@, self_sorted@[i as int].0, combined_v@);
                            assert forall|j2: int| 0 <= j2 < i as int
                                implies #[trigger] spec_pair_set_to_map(new_tree@).dom().contains(self_sorted@[j2].0)
                            by {
                                lemma_map_contains_pair_in_set(old_new_tree_view, self_sorted@[j2].0);
                                let w: V::V = choose|w: V::V| old_new_tree_view.contains((self_sorted@[j2].0, w));
                                lemma_pair_in_set_map_contains(new_tree@, self_sorted@[j2].0, w);
                            };
                            // Value tracking maintenance.
                            assert forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) implies
                                self_map.dom().contains(p.0) &&
                                ((!other_map.dom().contains(p.0) && self_tree.contains(p))
                                || (other_map.dom().contains(p.0) && exists|v1: V, v2: V, r: V|
                                    v1@ == self_map[p.0] && v2@ == other_map[p.0]
                                    && combine.ensures((&v1, &v2), r) && p.1 == r@))
                            by {
                                if old_new_tree_view.contains(p) {
                                } else {
                                }
                            };
                        }
                    },
                    None => {
                        // Self-only — keep self's value.
                        let cloned = pair.clone_plus();
                        proof { lemma_cloned_view_eq(*pair, cloned); }
                        new_tree.insert(cloned);
                        proof {
                            lemma_view_gen_insert::<K, V>(old_new_tree_view, cloned);
                            lemma_key_unique_insert(old_new_tree_view, self_sorted@[i as int].0, self_sorted@[i as int].1);
                            // Completeness maintenance.
                            lemma_pair_in_set_map_contains(new_tree@, self_sorted@[i as int].0, self_sorted@[i as int].1);
                            assert forall|j2: int| 0 <= j2 < i as int
                                implies #[trigger] spec_pair_set_to_map(new_tree@).dom().contains(self_sorted@[j2].0)
                            by {
                                lemma_map_contains_pair_in_set(old_new_tree_view, self_sorted@[j2].0);
                                let w: V::V = choose|w: V::V| old_new_tree_view.contains((self_sorted@[j2].0, w));
                                lemma_pair_in_set_map_contains(new_tree@, self_sorted@[j2].0, w);
                            };
                            // Value tracking maintenance: self-only entry.
                            assert forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) implies
                                self_map.dom().contains(p.0) &&
                                ((!other_map.dom().contains(p.0) && self_tree.contains(p))
                                || (other_map.dom().contains(p.0) && exists|v1: V, v2: V, r: V|
                                    v1@ == self_map[p.0] && v2@ == other_map[p.0]
                                    && combine.ensures((&v1, &v2), r) && p.1 == r@))
                            by {
                                if old_new_tree_view.contains(p) {
                                } else {
                                }
                            };
                        }
                    },
                }
                i += 1;
            }
            // Phase 2: iterate other entries, add those not in self.
            let other_sorted = other.inner.in_order();
            let other_len = other_sorted.length();
            let mut j: usize = 0;
            proof {
                lemma_sorted_keys_pairwise_distinct(other.inner@, other_sorted@);
                // Bridge: old keys preserved (Phase 1 completeness → per-key form).
                assert forall|kv: K::V| #[trigger] self_map.dom().contains(kv)
                    implies spec_pair_set_to_map(new_tree@).dom().contains(kv)
                by {
                    lemma_map_contains_pair_in_set(self_tree, kv);
                    let vv: V::V = choose|vv: V::V| self_tree.contains((kv, vv));
                    let jx: int = choose|jx: int| 0 <= jx < self_sorted@.len() as int && self_sorted@[jx] == (kv, vv);
                    assert(spec_pair_set_to_map(new_tree@).dom().contains(self_sorted@[jx].0));
                };
                lemma_pair_set_to_map_len(self_tree);
                lemma_pair_set_to_map_len(other.inner@);
            }
            while j < other_len
                invariant
                    self.inner@ == self_tree,
                    self.spec_ordkeymap_wf(),
                    other.spec_ordkeymap_wf(),
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
                    obeys_feq_full::<V>(),
                    obeys_feq_full::<K>(),
                    other_map == other@,
                    self_map == spec_pair_set_to_map(self_tree),
                    other_len as nat == other_sorted@.len(),
                    other_sorted@.len() == other.inner@.len(),
                    self_sorted@.len() == self_tree.len(),
                    forall|v: <Pair<K, V> as View>::V| other.inner@.contains(v) <==> #[trigger] other_sorted@.contains(v),
                    forall|v: <Pair<K, V> as View>::V| self_tree.contains(v) <==> #[trigger] self_sorted@.contains(v),
                    forall|ii: int, jj: int|
                        0 <= ii < other_sorted@.len() && 0 <= jj < other_sorted@.len() && ii != jj
                        ==> (#[trigger] other_sorted@[ii]).0 != (#[trigger] other_sorted@[jj]).0,
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        self_map.dom().contains(p.0) ||
                        (exists|j2: int| 0 <= j2 < j as int && p.0 == (#[trigger] other_sorted@[j2]).0),
                    0 <= j <= other_len,
                    new_tree.spec_bstparasteph_wf(),
                    vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                    view_ord_consistent::<Pair<K, V>>(),
                    new_tree@.len() <= self_sorted@.len() + j as nat,
                    self_sorted@.len() + other_sorted@.len() < usize::MAX as nat,
                    spec_key_unique_pairs_set(new_tree@),
                    spec_key_unique_pairs_set(self_tree),
                    // Old keys preserved.
                    forall|kv: K::V| #[trigger] self_map.dom().contains(kv)
                        ==> spec_pair_set_to_map(new_tree@).dom().contains(kv),
                    // Other completeness.
                    forall|j2: int| 0 <= j2 < j as int && !self_map.dom().contains(other_sorted@[j2].0)
                        ==> #[trigger] spec_pair_set_to_map(new_tree@).dom().contains(other_sorted@[j2].0),
                    // Phase 2 value tracking (3-way).
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        (self_map.dom().contains(p.0) &&
                            ((!other_map.dom().contains(p.0) && self_tree.contains(p))
                            || (other_map.dom().contains(p.0) && exists|v1: V, v2: V, r: V|
                                v1@ == self_map[p.0] && v2@ == other_map[p.0]
                                && combine.ensures((&v1, &v2), r) && p.1 == r@)))
                        || (!self_map.dom().contains(p.0) && other.inner@.contains(p)),
                    spec_set_pair_view_generated::<K, V>(new_tree@),
                decreases other_len - j,
            {
                let pair = other_sorted.nth(j);
                proof { reveal(obeys_view_eq); }
                let in_self = self.find(&pair.0);
                match in_self {
                    None => {
                        let cloned = pair.clone_plus();
                        let ghost old_new_tree_view = new_tree@;
                        proof {
                            lemma_cloned_view_eq(*pair, cloned);
                            // Freshness: other_sorted@[j].0 not already in new_tree.
                            // Link sorted entry to other.inner@.
                            assert(other_sorted@.contains(other_sorted@[j as int])) by {
                            };
                            lemma_pair_in_set_map_contains(other.inner@, other_sorted@[j as int].0, other_sorted@[j as int].1);
                        }
                        new_tree.insert(cloned);
                        proof {
                            lemma_view_gen_insert::<K, V>(old_new_tree_view, cloned);
                            lemma_key_unique_insert(old_new_tree_view, other_sorted@[j as int].0, other_sorted@[j as int].1);
                            // Old keys preserved maintenance.
                            assert forall|kv: K::V| #[trigger] self_map.dom().contains(kv)
                                implies spec_pair_set_to_map(new_tree@).dom().contains(kv)
                            by {
                                lemma_map_contains_pair_in_set(old_new_tree_view, kv);
                                let w: V::V = choose|w: V::V| old_new_tree_view.contains((kv, w));
                                lemma_pair_in_set_map_contains(new_tree@, kv, w);
                            };
                            // Other completeness maintenance.
                            assert(other_sorted@.contains(other_sorted@[j as int])) by {
                                assert(other_sorted@[j as int] == other_sorted@[j as int]);
                            };
                            assert(other.inner@.contains(other_sorted@[j as int]));
                            lemma_pair_in_set_map_contains(new_tree@, other_sorted@[j as int].0, other_sorted@[j as int].1);
                            assert forall|j2: int| 0 <= j2 < j as int + 1
                                && !self_map.dom().contains(other_sorted@[j2].0)
                                implies #[trigger] spec_pair_set_to_map(new_tree@).dom().contains(other_sorted@[j2].0)
                            by {
                                if j2 == j as int {
                                } else {
                                    lemma_map_contains_pair_in_set(old_new_tree_view, other_sorted@[j2].0);
                                    let w: V::V = choose|w: V::V| old_new_tree_view.contains((other_sorted@[j2].0, w));
                                    assert(new_tree@.contains((other_sorted@[j2].0, w)));
                                    lemma_pair_in_set_map_contains(new_tree@, other_sorted@[j2].0, w);
                                }
                            };
                            // Value tracking maintenance.
                            assert forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) implies
                                (self_map.dom().contains(p.0) &&
                                    ((!other_map.dom().contains(p.0) && self_tree.contains(p))
                                    || (other_map.dom().contains(p.0) && exists|v1: V, v2: V, r: V|
                                        v1@ == self_map[p.0] && v2@ == other_map[p.0]
                                        && combine.ensures((&v1, &v2), r) && p.1 == r@)))
                                || (!self_map.dom().contains(p.0) && other.inner@.contains(p))
                            by {
                                if old_new_tree_view.contains(p) {
                                } else {
                                    assert(p.0 == other_sorted@[j as int].0);
                                    assert(p.1 == other_sorted@[j as int].1);
                                    assert(!self_map.dom().contains(p.0));
                                    assert(other.inner@.contains(p));
                                }
                            };
                        }
                    },
                    Some(_) => {},
                }
                j += 1;
            }
            let combined = OrdKeyMap { inner: new_tree };
            proof {
                lemma_pair_set_to_map_dom_finite(new_tree@);
                lemma_pair_set_to_map_dom_finite(self_tree);
                lemma_pair_set_to_map_dom_finite(other.inner@);
                // 1. Domain: combined@.dom() =~= self_map.dom().union(other_map.dom()).
                assert(combined@.dom() =~= self_map.dom().union(other_map.dom())) by {
                    assert forall|kv: K::V| combined@.dom().contains(kv)
                        implies #[trigger] self_map.dom().union(other_map.dom()).contains(kv)
                    by {
                        lemma_map_contains_pair_in_set(new_tree@, kv);
                        let vv: V::V = choose|vv: V::V| new_tree@.contains((kv, vv));
                        if !self_map.dom().contains(kv) {
                            assert(other.inner@.contains((kv, vv)));
                            lemma_pair_in_set_map_contains(other.inner@, kv, vv);
                        }
                    };
                    assert forall|kv: K::V| #[trigger] self_map.dom().union(other_map.dom()).contains(kv)
                        implies combined@.dom().contains(kv)
                    by {
                        if self_map.dom().contains(kv) {
                        } else {
                            assert(other_map.dom().contains(kv));
                            lemma_map_contains_pair_in_set(other.inner@, kv);
                            let vv: V::V = choose|vv: V::V| other.inner@.contains((kv, vv));
                            assert(other_sorted@.contains((kv, vv)));
                            let jx: int = choose|jx: int| 0 <= jx < other_sorted@.len() as int && other_sorted@[jx] == (kv, vv);
                            assert(!self_map.dom().contains(other_sorted@[jx].0));
                            assert(spec_pair_set_to_map(new_tree@).dom().contains(other_sorted@[jx].0));
                        }
                    };
                };
                // 2. Self-only values.
                assert forall|k: K::V| #[trigger] self_map.contains_key(k) && !other_map.contains_key(k)
                    implies combined@[k] == self_map[k]
                by {
                    lemma_map_contains_pair_in_set(new_tree@, k);
                    let vv: V::V = choose|vv: V::V| new_tree@.contains((k, vv));
                    assert(self_tree.contains((k, vv)));
                    lemma_pair_in_set_map_contains(new_tree@, k, vv);
                    lemma_pair_in_set_map_contains(self_tree, k, vv);
                };
                // 3. Other-only values.
                assert forall|k: K::V| !self_map.contains_key(k) && #[trigger] other_map.contains_key(k)
                    implies combined@[k] == other_map[k]
                by {
                    lemma_map_contains_pair_in_set(new_tree@, k);
                    let vv: V::V = choose|vv: V::V| new_tree@.contains((k, vv));
                    assert(other.inner@.contains((k, vv)));
                    lemma_pair_in_set_map_contains(new_tree@, k, vv);
                    lemma_pair_in_set_map_contains(other.inner@, k, vv);
                };
                // 4. Both values — combined via combine.
                assert forall|k: K::V| #[trigger] self_map.contains_key(k) && other_map.contains_key(k) implies
                    (exists|v1: V, v2: V, r: V|
                        v1@ == self_map[k] && v2@ == other_map[k]
                        && combine.ensures((&v1, &v2), r)
                        && combined@[k] == r@)
                by {
                    lemma_map_contains_pair_in_set(new_tree@, k);
                    let vv: V::V = choose|vv: V::V| new_tree@.contains((k, vv));
                    lemma_pair_in_set_map_contains(new_tree@, k, vv);
                };
                // 5. wf.
                assert(new_tree@.len() < usize::MAX as nat);
            }
            combined
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m)
        fn intersect_with<F: Fn(&V, &V) -> V>(&self, other: &Self, combine: &F) -> (common: Self)
        {
            let ghost self_tree = self.inner@;
            let ghost self_map = self@;
            let ghost other_map = other@;
            let sorted = self.inner.in_order();
            let len = sorted.length();
            let mut new_tree = ParamBST::<Pair<K, V>>::new();
            let mut i: usize = 0;
            proof {
                assert(obeys_feq_full_trigger::<V>());
                assert(obeys_feq_full_trigger::<K>());
                lemma_pair_set_to_map_dom_finite(self_tree);
                lemma_sorted_keys_pairwise_distinct(self_tree, sorted@);
                lemma_key_unique_empty::<K::V, V::V>();
                assert(spec_set_pair_view_generated::<K, V>(new_tree@)) by {
                    assert forall|elem: (K::V, V::V)| new_tree@.contains(elem)
                        implies exists|p: Pair<K, V>| (#[trigger] p@) == elem by {
                        assert(false);
                    };
                };
                assert(obeys_view_eq_trigger::<K>());
            }
            while i < len
                invariant
                    self.inner@ == self_tree,
                    self.spec_ordkeymap_wf(),
                    other.spec_ordkeymap_wf(),
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<V>(),
                    obeys_feq_full::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
                    forall|v1: &V, v2: &V| #[trigger] combine.requires((v1, v2)),
                    self_map == spec_pair_set_to_map(self_tree),
                    other_map == other@,
                    len as nat == sorted@.len(),
                    sorted@.len() == self_tree.len(),
                    0 <= i <= len,
                    new_tree.spec_bstparasteph_wf(),
                    vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                    view_ord_consistent::<Pair<K, V>>(),
                    new_tree@.len() <= i as nat,
                    new_tree@.len() < usize::MAX as nat,
                    spec_key_unique_pairs_set(new_tree@),
                    spec_key_unique_pairs_set(self_tree),
                    forall|v: <Pair<K, V> as View>::V| self_tree.contains(v) <==> #[trigger] sorted@.contains(v),
                    forall|ii: int, jj: int|
                        0 <= ii < sorted@.len() && 0 <= jj < sorted@.len() && ii != jj
                        ==> (#[trigger] sorted@[ii]).0 != (#[trigger] sorted@[jj]).0,
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        exists|j: int| 0 <= j < i as int && p.0 == (#[trigger] sorted@[j]).0,
                    // Entries in new_tree have keys in other's domain.
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        other_map.dom().contains(p.0),
                    // Completeness: processed keys in other are in new_tree.
                    forall|j2: int| 0 <= j2 < i as int && other_map.dom().contains(sorted@[j2].0)
                        ==> #[trigger] spec_pair_set_to_map(new_tree@).dom().contains(sorted@[j2].0),
                    // Value tracking: each entry's value is combine(self_val, other_val).
                    forall|kv: K::V| #[trigger] spec_pair_set_to_map(new_tree@).dom().contains(kv) ==>
                        (exists|v1: V, v2: V, r: V|
                            v1@ == self_map[kv] && v2@ == other_map[kv]
                            && combine.ensures((&v1, &v2), r)
                            && spec_pair_set_to_map(new_tree@)[kv] == r@),
                    spec_set_pair_view_generated::<K, V>(new_tree@),
                decreases len - i,
            {
                let pair = sorted.nth(i);
                proof { reveal(obeys_view_eq); }
                let other_find = other.find(&pair.0);
                match other_find {
                    Some(other_v) => {
                        let combined_v = combine(&pair.1, &other_v);
                        let key_clone = pair.0.clone_plus();
                        proof {
                            assert(obeys_feq_full_trigger::<K>());
                            assert(key_clone@ == pair.0@);
                        }
                        let ghost old_new_tree_view = new_tree@;
                        proof {
                            assert(!spec_pair_set_to_map(old_new_tree_view).dom().contains(sorted@[i as int].0)) by {
                                if spec_pair_set_to_map(old_new_tree_view).dom().contains(sorted@[i as int].0) {
                                    lemma_map_contains_pair_in_set(old_new_tree_view, sorted@[i as int].0);
                                    let vv: V::V = choose|vv: V::V| old_new_tree_view.contains((sorted@[i as int].0, vv));
                                    let jj = choose|jj: int| 0 <= jj < i as int && (sorted@[i as int].0, vv).0 == (#[trigger] sorted@[jj]).0;
                                    assert(false);
                                }
                            };
                            // Link pair values.
                            assert(sorted@.contains(sorted@[i as int])) by { assert(sorted@[i as int] == sorted@[i as int]); };
                            assert(self_tree.contains(sorted@[i as int]));
                            lemma_pair_in_set_map_contains(self_tree, sorted@[i as int].0, sorted@[i as int].1);
                        }
                        new_tree.insert(Pair(key_clone, combined_v));
                        proof {
                            lemma_view_gen_insert::<K, V>(old_new_tree_view, Pair(key_clone, combined_v));
                            assert(new_tree@.len() <= i as nat + 1);
                            assert(new_tree@.len() < usize::MAX as nat);
                            lemma_key_unique_insert(old_new_tree_view, sorted@[i as int].0, combined_v@);
                            // Completeness maintenance for new key.
                            lemma_pair_in_set_map_contains(new_tree@, sorted@[i as int].0, combined_v@);
                            // Completeness for old keys: old entries preserved.
                            assert forall|j2: int| 0 <= j2 < i as int && other_map.dom().contains(sorted@[j2].0)
                                implies #[trigger] spec_pair_set_to_map(new_tree@).dom().contains(sorted@[j2].0)
                            by {
                                lemma_map_contains_pair_in_set(old_new_tree_view, sorted@[j2].0);
                                let w: V::V = choose|w: V::V| old_new_tree_view.contains((sorted@[j2].0, w));
                                assert(new_tree@.contains((sorted@[j2].0, w)));
                                lemma_pair_in_set_map_contains(new_tree@, sorted@[j2].0, w);
                            };
                            // Value tracking for new key.
                            assert(spec_pair_set_to_map(new_tree@)[sorted@[i as int].0] == combined_v@) by {
                                lemma_pair_in_set_map_contains(new_tree@, sorted@[i as int].0, combined_v@);
                            };
                            assert(pair.1@ == self_map[sorted@[i as int].0]);
                            assert(other_v@ == other_map[sorted@[i as int].0]);
                            assert(combine.ensures((&pair.1, &other_v), combined_v));
                            // Value tracking for old keys.
                            assert forall|kv: K::V| #[trigger] spec_pair_set_to_map(new_tree@).dom().contains(kv)
                                implies (exists|v1: V, v2: V, r: V|
                                    v1@ == self_map[kv] && v2@ == other_map[kv]
                                    && combine.ensures((&v1, &v2), r)
                                    && spec_pair_set_to_map(new_tree@)[kv] == r@)
                            by {
                                if kv == sorted@[i as int].0 {
                                    assert(pair.1@ == self_map[kv]);
                                    assert(other_v@ == other_map[kv]);
                                    assert(combine.ensures((&pair.1, &other_v), combined_v));
                                    assert(spec_pair_set_to_map(new_tree@)[kv] == combined_v@);
                                } else {
                                    // Old entry — value unchanged.
                                    lemma_map_contains_pair_in_set(new_tree@, kv);
                                    let w: V::V = choose|w: V::V| new_tree@.contains((kv, w));
                                    assert(old_new_tree_view.contains((kv, w)));
                                    lemma_pair_in_set_map_contains(old_new_tree_view, kv, w);
                                    lemma_pair_in_set_map_contains(new_tree@, kv, w);
                                    assert(spec_pair_set_to_map(new_tree@)[kv] == spec_pair_set_to_map(old_new_tree_view)[kv]);
                                }
                            };
                        }
                    },
                    None => {},
                }
                i += 1;
            }
            let common = OrdKeyMap { inner: new_tree };
            proof {
                lemma_pair_set_to_map_dom_finite(new_tree@);
                lemma_pair_set_to_map_dom_finite(self_tree);
                // Domain proof: common@.dom() =~= self_map.dom().intersect(other_map.dom()).
                assert(common@.dom() =~= self_map.dom().intersect(other_map.dom())) by {
                    // Forward: key in common → key in self AND other.
                    assert forall|k: K::V| #[trigger] common@.dom().contains(k)
                        implies self_map.dom().contains(k) && other_map.dom().contains(k)
                    by {
                        lemma_map_contains_pair_in_set(new_tree@, k);
                        let v: V::V = choose|v: V::V| new_tree@.contains((k, v));
                        let j = choose|j: int| 0 <= j < i as int && (k, v).0 == (#[trigger] sorted@[j]).0;
                        assert(sorted@.contains(sorted@[j])) by { assert(sorted@[j] == sorted@[j]); };
                        assert(self_tree.contains(sorted@[j]));
                        lemma_pair_in_set_map_contains(self_tree, sorted@[j].0, sorted@[j].1);
                    };
                    // Backward: key in self AND other → key in common.
                    assert forall|k: K::V|
                        self_map.dom().contains(k) && other_map.dom().contains(k)
                        implies #[trigger] common@.dom().contains(k)
                    by {
                        lemma_map_contains_pair_in_set(self_tree, k);
                        let v: V::V = choose|v: V::V| self_tree.contains((k, v));
                        assert(sorted@.contains((k, v)));
                        let j = choose|j: int| 0 <= j < sorted@.len() && sorted@[j] == (k, v);
                        assert(spec_pair_set_to_map(new_tree@).dom().contains(sorted@[j].0));
                        assert(sorted@[j].0 == k);
                    };
                };
                // Value proof.
                assert forall|k: K::V| #[trigger] common@.contains_key(k) implies
                    (exists|v1: V, v2: V, r: V|
                        v1@ == self_map[k] && v2@ == other_map[k]
                        && combine.ensures((&v1, &v2), r)
                        && common@[k] == r@)
                by {
                    assert(spec_pair_set_to_map(new_tree@).dom().contains(k));
                };
                // WF.
                assert(new_tree@.len() < usize::MAX as nat);
                assert(spec_pair_key_determines_order::<K, V>());
                assert(vstd::laws_cmp::obeys_cmp_spec::<K>());
                assert(view_ord_consistent::<K>());
                assert(obeys_feq_fulls::<K, V>());
            }
            common
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m)
        fn difference(&self, other: &Self) -> (remaining: Self)
        {
            let ghost self_tree = self.inner@;
            let ghost self_map = self@;
            let ghost other_map = other@;
            let sorted = self.inner.in_order();
            let len = sorted.length();
            let mut new_tree = ParamBST::<Pair<K, V>>::new();
            let mut i: usize = 0;
            proof {
                lemma_sorted_keys_pairwise_distinct(self_tree, sorted@);
                lemma_key_unique_empty::<K::V, V::V>();
                assert(obeys_view_eq_trigger::<K>());
            }
            while i < len
                invariant
                    self.inner@ == self_tree,
                    self.spec_ordkeymap_wf(),
                    other.spec_ordkeymap_wf(),
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
                    self_map == spec_pair_set_to_map(self_tree),
                    other_map == other@,
                    len as nat == sorted@.len(),
                    sorted@.len() == self_tree.len(),
                    forall|v: <Pair<K, V> as View>::V| self_tree.contains(v) <==> #[trigger] sorted@.contains(v),
                    forall|ii: int, jj: int|
                        0 <= ii < sorted@.len() && 0 <= jj < sorted@.len() && ii != jj
                        ==> (#[trigger] sorted@[ii]).0 != (#[trigger] sorted@[jj]).0,
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        exists|j: int| 0 <= j < i as int && p.0 == (#[trigger] sorted@[j]).0,
                    // All entries in new_tree are from self_tree (values preserved).
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==> self_tree.contains(p),
                    // Entries in new_tree are not in other's domain.
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        !other_map.dom().contains(p.0),
                    // Completeness: processed entries not in other are in new_tree.
                    forall|j: int| 0 <= j < i as int && !other_map.dom().contains(sorted@[j].0)
                        ==> #[trigger] new_tree@.contains(sorted@[j]),
                    0 <= i <= len,
                    new_tree.spec_bstparasteph_wf(),
                    vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                    view_ord_consistent::<Pair<K, V>>(),
                    new_tree@.len() <= i as nat,
                    new_tree@.len() < usize::MAX as nat,
                    spec_key_unique_pairs_set(new_tree@),
                    spec_key_unique_pairs_set(self_tree),
                decreases len - i,
            {
                let pair = sorted.nth(i);
                proof { reveal(obeys_view_eq); }
                let in_other = other.find(&pair.0);
                match in_other {
                    None => {
                        let cloned = pair.clone_plus();
                        let ghost old_new_tree_view = new_tree@;
                        proof {
                            lemma_cloned_view_eq(*pair, cloned);
                            // Freshness proof.
                            assert(!spec_pair_set_to_map(old_new_tree_view).dom().contains(sorted@[i as int].0)) by {
                                if spec_pair_set_to_map(old_new_tree_view).dom().contains(sorted@[i as int].0) {
                                    lemma_map_contains_pair_in_set(old_new_tree_view, sorted@[i as int].0);
                                    let vv: V::V = choose|vv: V::V| old_new_tree_view.contains((sorted@[i as int].0, vv));
                                    let jj = choose|jj: int| 0 <= jj < i as int && (sorted@[i as int].0, vv).0 == (#[trigger] sorted@[jj]).0;
                                    assert(false);
                                }
                            };
                        }
                        new_tree.insert(cloned);
                        proof {
                            assert(new_tree@.len() <= i as nat + 1);
                            assert(new_tree@.len() < usize::MAX as nat);
                            lemma_key_unique_insert(old_new_tree_view, sorted@[i as int].0, sorted@[i as int].1);
                            // Prove sorted@[i] is in self_tree for the subset invariant.
                            assert(sorted@.contains(sorted@[i as int])) by { assert(sorted@[i as int] == sorted@[i as int]); };
                            assert(self_tree.contains(sorted@[i as int]));
                            // Maintain completeness: new_tree now includes sorted@[i].
                            assert(new_tree@.contains(sorted@[i as int]));
                        }
                    },
                    Some(_) => {},
                }
                i += 1;
            }
            let remaining = OrdKeyMap { inner: new_tree };
            proof {
                lemma_pair_set_to_map_dom_finite(new_tree@);
                lemma_pair_set_to_map_dom_finite(self_tree);
                // Prove: remaining@.dom() =~= self_map.dom().difference(other_map.dom())
                assert(remaining@.dom() =~= self_map.dom().difference(other_map.dom())) by {
                    // Forward: k in remaining dom ==> k in self dom and k not in other dom.
                    assert forall|k: K::V| #[trigger] remaining@.dom().contains(k)
                        implies self_map.dom().contains(k) && !other_map.dom().contains(k)
                    by {
                        lemma_map_contains_pair_in_set(new_tree@, k);
                        let v: V::V = choose|v: V::V| new_tree@.contains((k, v));
                        assert(self_tree.contains((k, v)));
                        lemma_pair_in_set_map_contains(self_tree, k, v);
                        assert(!other_map.dom().contains(k));
                    };
                    // Backward: k in self dom and k not in other dom ==> k in remaining dom.
                    assert forall|k: K::V|
                        self_map.dom().contains(k) && !other_map.dom().contains(k)
                        implies #[trigger] remaining@.dom().contains(k)
                    by {
                        lemma_map_contains_pair_in_set(self_tree, k);
                        let v: V::V = choose|v: V::V| self_tree.contains((k, v));
                        assert(sorted@.contains((k, v)));
                        let j = choose|j: int| 0 <= j < sorted@.len() && sorted@[j] == (k, v);
                        assert(new_tree@.contains(sorted@[j]));
                        lemma_pair_in_set_map_contains(new_tree@, k, v);
                    };
                };
                // Prove: values preserved from self.
                assert forall|k: K::V| #[trigger] remaining@.contains_key(k)
                    implies remaining@[k] == self_map[k]
                by {
                    lemma_map_contains_pair_in_set(new_tree@, k);
                    let v: V::V = choose|v: V::V| new_tree@.contains((k, v));
                    assert(self_tree.contains((k, v)));
                    lemma_pair_in_set_map_contains(new_tree@, k, v);
                    lemma_pair_in_set_map_contains(self_tree, k, v);
                };
                // Type axioms flow from self.spec_ordkeymap_wf().
                assert(spec_pair_key_determines_order::<K, V>());
                assert(vstd::laws_cmp::obeys_cmp_spec::<K>());
                assert(view_ord_consistent::<K>());
                assert(obeys_feq_fulls::<K, V>());
            }
            remaining
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- delegates to ordkeymap_next
        fn next_key(&self, k: &K) -> (successor: Option<K>)
            where K: TotalOrder
        {
            let r = ordkeymap_next(&self.inner, k);
            proof { lemma_pair_set_to_map_dom_finite(self.inner@); }
            r
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- delegates to ordkeymap_prev
        fn prev_key(&self, k: &K) -> (predecessor: Option<K>)
            where K: TotalOrder
        {
            let r = ordkeymap_prev(&self.inner, k);
            proof { lemma_pair_set_to_map_dom_finite(self.inner@); }
            r
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- delegates to ordkeymap_rank
        fn rank_key(&self, k: &K) -> (rank: usize)
            where K: TotalOrder
        {
            let r = ordkeymap_rank(&self.inner, k);
            proof { lemma_pair_set_to_map_dom_finite(self.inner@); }
            r
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- delegates to ordkeymap_select
        fn select_key(&self, i: usize) -> (selected: Option<K>)
            where K: TotalOrder
        {
            let r = ordkeymap_select(&self.inner, i);
            proof { lemma_pair_set_to_map_dom_finite(self.inner@); }
            r
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- BST min_key + key extraction
        fn first_key(&self) -> (first: Option<K>)
            where K: TotalOrder
        {
            proof {
                lemma_pair_set_to_map_dom_finite(self.inner@);
                lemma_pair_set_to_map_len(self.inner@);
            }
            let min_pair = self.inner.min_key();
            match min_pair {
                None => None,
                Some(pair) => {
                    let key = pair.0.clone_plus();
                    reveal_param_bst_backings(&self.inner);
                    proof {
                        lemma_reveal_view_injective::<K>();
                        lemma_cloned_view_eq(pair.0, key);
                        lemma_pair_in_set_map_contains(self.inner@, pair.0@, pair.1@);
                        assert(key == pair.0);
                        assert forall|t: K| #[trigger] self@.dom().contains(t@)
                            implies TotalOrder::le(key, t) by {
                            lemma_map_contains_pair_in_set(self.inner@, t@);
                            let vv: V::V = choose|vv: V::V| self.inner@.contains((t@, vv));
                            if pair.0@ == t@ {
                                assert(key@ == t@);
                                assert(key == t);
                                K::reflexive(key);
                            } else {
                                assert(pair@ != (t@, vv));
                                let tp: Pair<K, V> = choose|tp: Pair<K, V>| #[trigger] self.inner@.contains(tp@) && tp@ == (t@, vv);
                                assert(pair.cmp_spec(&tp) == Less);
                                assert(pair.0@ != tp.0@);
                                assert(pair.0.cmp_spec(&tp.0) == Less);
                                assert(tp.0 == t);
                                assert(key.cmp_spec(&t) == Less);
                                K::cmp_spec_less_implies_le(key, t);
                            }
                        };
                    }
                    Some(key)
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- BST max_key + key extraction
        fn last_key(&self) -> (last: Option<K>)
            where K: TotalOrder
        {
            proof {
                lemma_pair_set_to_map_dom_finite(self.inner@);
                lemma_pair_set_to_map_len(self.inner@);
            }
            let max_pair = self.inner.max_key();
            match max_pair {
                None => None,
                Some(pair) => {
                    let key = pair.0.clone_plus();
                    reveal_param_bst_backings(&self.inner);
                    proof {
                        lemma_reveal_view_injective::<K>();
                        lemma_cloned_view_eq(pair.0, key);
                        lemma_pair_in_set_map_contains(self.inner@, pair.0@, pair.1@);
                        assert(key == pair.0);
                        assert forall|t: K| #[trigger] self@.dom().contains(t@)
                            implies TotalOrder::le(t, key) by {
                            lemma_map_contains_pair_in_set(self.inner@, t@);
                            let vv: V::V = choose|vv: V::V| self.inner@.contains((t@, vv));
                            if pair.0@ == t@ {
                                assert(key@ == t@);
                                assert(key == t);
                                K::reflexive(key);
                            } else {
                                assert(pair@ != (t@, vv));
                                let tp: Pair<K, V> = choose|tp: Pair<K, V>| #[trigger] self.inner@.contains(tp@) && tp@ == (t@, vv);
                                assert(tp.cmp_spec(&pair) == Less);
                                assert(tp.0@ != pair.0@);
                                assert(tp.0.cmp_spec(&pair.0) == Less);
                                assert(tp.0 == t);
                                assert(t.cmp_spec(&key) == Less);
                                K::cmp_spec_less_implies_le(t, key);
                            }
                        };
                    }
                    Some(key)
                }
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- two BST splits + two finds
        fn get_key_range(&self, k1: &K, k2: &K) -> (range: Self)
        {
            // Split at k1: right_of_k1 has keys > k1.
            let (_, _, right_of_k1) = ordkeymap_split(&self.inner, k1);
            // Split right_of_k1 at k2: middle has keys between k1 and k2 (exclusive).
            let (middle, _, _) = ordkeymap_split(&right_of_k1, k2);
            // Check original tree for k1 and k2.
            let found_k1 = ordkeymap_find(&self.inner, k1);
            let found_k2 = ordkeymap_find(&self.inner, k2);
            let mut result_tree = middle;
            proof {
                assert forall|v: <Pair<K,V> as View>::V| result_tree@.contains(v)
                    implies #[trigger] self.inner@.contains(v) by {
                    assert(right_of_k1@.contains(v));
                };
            }
            match found_k1 {
                Some(v1) => {
                    let k1_clone = k1.clone_plus();
                    proof {
                        lemma_cloned_view_eq(*k1, k1_clone);
                        assert(result_tree@.len() < usize::MAX as nat) by {
                            vstd::set_lib::lemma_len_subset(result_tree@, self.inner@);
                        };
                        // The inserted pair (k1@, v1@) is in self.inner@.
                        lemma_map_contains_pair_in_set(self.inner@, k1@);
                        let ghost cv: V::V = choose|cv: V::V| self.inner@.contains((k1@, cv));
                        assert(cv == v1@);
                    }
                    result_tree.insert(Pair(k1_clone, v1));
                    proof {
                        assert forall|v: <Pair<K,V> as View>::V| result_tree@.contains(v)
                            implies #[trigger] self.inner@.contains(v) by {
                        };
                    }
                },
                None => {},
            }
            match found_k2 {
                Some(v2) => {
                    let k2_clone = k2.clone_plus();
                    proof {
                        lemma_cloned_view_eq(*k2, k2_clone);
                        assert(result_tree@.len() < usize::MAX as nat) by {
                            vstd::set_lib::lemma_len_subset(result_tree@, self.inner@);
                        };
                        lemma_map_contains_pair_in_set(self.inner@, k2@);
                        let ghost cv: V::V = choose|cv: V::V| self.inner@.contains((k2@, cv));
                        assert(cv == v2@);
                    }
                    result_tree.insert(Pair(k2_clone, v2));
                    proof {
                        assert forall|v: <Pair<K,V> as View>::V| result_tree@.contains(v)
                            implies #[trigger] self.inner@.contains(v) by {
                        };
                    }
                },
                None => {},
            }
            let range = OrdKeyMap { inner: result_tree };
            proof {
                lemma_pair_set_to_map_dom_finite(result_tree@);
                lemma_pair_set_to_map_dom_finite(self.inner@);
                assert(range@.dom().subset_of(self@.dom())) by {
                    assert forall|key: K::V| range@.dom().contains(key)
                        implies #[trigger] self@.dom().contains(key) by {
                        lemma_map_contains_pair_in_set(result_tree@, key);
                        let v: V::V = choose|v: V::V| result_tree@.contains((key, v));
                        assert(self.inner@.contains((key, v)));
                        lemma_pair_in_set_map_contains(self.inner@, key, v);
                    };
                };
                assert forall|key: K::V| #[trigger] range@.dom().contains(key)
                    implies range@[key] == self@[key] by {
                    lemma_map_contains_pair_in_set(result_tree@, key);
                    let v: V::V = choose|v: V::V| result_tree@.contains((key, v));
                    assert(self.inner@.contains((key, v)));
                    lemma_pair_in_set_map_contains(result_tree@, key, v);
                    lemma_pair_in_set_map_contains(self.inner@, key, v);
                };
                vstd::set_lib::lemma_len_subset(result_tree@, self.inner@);
                lemma_key_unique_subset(self.inner@, result_tree@);
                lemma_view_gen_subset::<K, V>(result_tree@, self.inner@);
            }
            range
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- in_order traversal + BST split
        fn split_rank_key(&mut self, i: usize) -> (split: (Self, Self))
        {
            let ghost old_tree = self.inner@;
            let ghost old_map = self@;
            proof {
                lemma_pair_set_to_map_dom_finite(old_tree);
                lemma_pair_set_to_map_len(old_tree);
            }
            let size = self.size();
            if i >= size {
                // Everything goes left, right is empty.
                let left_tree = self.inner.clone();
                let right_tree = ParamBST::<Pair<K, V>>::new();
                *self = OrdKeyMap::new();
                let left = OrdKeyMap { inner: left_tree };
                let right = OrdKeyMap { inner: right_tree };
                proof {
                    lemma_pair_set_to_map_dom_finite(left_tree@);
                    lemma_pair_set_to_map_dom_finite(right_tree@);
                    lemma_set_to_map_empty::<K::V, V::V>();
                    assert(left@.dom() =~= old_map.dom());
                    assert(right@.dom() =~= Set::empty());
                    assert(left@.dom().disjoint(right@.dom()));
                    assert(left.inner@ =~= old_tree);
                    lemma_key_unique_empty::<K::V, V::V>();
                    assert(spec_set_pair_view_generated::<K, V>(right.inner@)) by {
                        assert forall|elem: (K::V, V::V)| right.inner@.contains(elem)
                            implies exists|p: Pair<K, V>| (#[trigger] p@) == elem by {};
                    };
                }
                (left, right)
            } else {
                // Get sorted pairs, find the key at rank i, split by that key.
                let sorted = self.inner.in_order();
                let pair_at_i = sorted.nth(i);
                let split_key = pair_at_i.0.clone_plus();
                proof { lemma_cloned_view_eq(pair_at_i.0, split_key); }
                let (left_tree, found, right_tree) = ordkeymap_split(&self.inner, &split_key);
                // Re-insert split_key+value into right.
                let found_val = ordkeymap_find(&self.inner, &split_key);
                let mut right_tree_final = right_tree;
                match found_val {
                    Some(v) => {
                        let p = Pair(split_key.clone_plus(), v);
                        proof {
                            lemma_cloned_view_eq(split_key, p.0);
                            vstd::set_lib::lemma_len_subset(right_tree_final@, old_tree);
                        }
                        right_tree_final.insert(p);
                    },
                    None => {},
                }
                *self = OrdKeyMap::new();
                let left = OrdKeyMap { inner: left_tree };
                let right = OrdKeyMap { inner: right_tree_final };
                proof {
                    lemma_pair_set_to_map_dom_finite(left_tree@);
                    lemma_pair_set_to_map_dom_finite(right_tree_final@);
                    // left ⊆ old.
                    assert(left@.dom().subset_of(old_map.dom())) by {
                        assert forall|key: K::V| left@.dom().contains(key)
                            implies #[trigger] old_map.dom().contains(key) by {
                            lemma_map_contains_pair_in_set(left_tree@, key);
                            let v: V::V = choose|v: V::V| left_tree@.contains((key, v));
                            assert(old_tree.contains((key, v)));
                            lemma_pair_in_set_map_contains(old_tree, key, v);
                        };
                    };
                    // right ⊆ old.
                    assert forall|v: <Pair<K,V> as View>::V| right_tree_final@.contains(v)
                        implies #[trigger] old_tree.contains(v) by {
                        // Either in original right_tree (subset of old) or the re-inserted pair.
                    };
                    assert(right@.dom().subset_of(old_map.dom())) by {
                        assert forall|key: K::V| right@.dom().contains(key)
                            implies #[trigger] old_map.dom().contains(key) by {
                            lemma_map_contains_pair_in_set(right_tree_final@, key);
                            let v: V::V = choose|v: V::V| right_tree_final@.contains((key, v));
                            assert(old_tree.contains((key, v)));
                            lemma_pair_in_set_map_contains(old_tree, key, v);
                        };
                    };
                    // Disjoint.
                    assert(left@.dom().disjoint(right@.dom())) by {
                        assert forall|key: K::V|
                            !(left@.dom().contains(key) && #[trigger] right@.dom().contains(key))
                        by {
                            if left@.dom().contains(key) && right@.dom().contains(key) {
                                lemma_map_contains_pair_in_set(left_tree@, key);
                                lemma_map_contains_pair_in_set(right_tree_final@, key);
                                let lv: V::V = choose|v: V::V| left_tree@.contains((key, v));
                                let rv: V::V = choose|v: V::V| right_tree_final@.contains((key, v));
                                assert(old_tree.contains((key, lv)));
                                assert(old_tree.contains((key, rv)));
                                assert(lv == rv);
                                assert(left_tree@.contains((key, lv)));
                                assert(right_tree_final@.contains((key, lv)));
                                // left_tree is from split, all keys < split_key.
                                // right_tree_final has keys > split_key (plus split_key itself).
                                // !left_tree contains split_key@, so key != split_key@.
                                // left has key < split_key, right has key > split_key → contradiction.
                                assert(!left_tree@.disjoint(right_tree_final@));
                            }
                        };
                    };
                    // Completeness.
                    assert forall|key: K::V| #[trigger] old_map.dom().contains(key)
                        implies left@.dom().contains(key) || right@.dom().contains(key)
                    by {
                        lemma_map_contains_pair_in_set(old_tree, key);
                        let v: V::V = choose|v: V::V| old_tree.contains((key, v));
                        if key != split_key@ {
                            if left_tree@.contains((key, v)) {
                                lemma_pair_in_set_map_contains(left_tree@, key, v);
                            } else {
                                assert(right_tree@.contains((key, v)));
                                assert(right_tree_final@.contains((key, v)));
                                lemma_pair_in_set_map_contains(right_tree_final@, key, v);
                            }
                        } else {
                            // key == split_key@. The pair was re-inserted into right.
                            assert(right_tree_final@.contains((key, v)));
                            lemma_pair_in_set_map_contains(right_tree_final@, key, v);
                        }
                    };
                    // wf for left and right.
                    vstd::set_lib::lemma_len_subset(left_tree@, old_tree);
                    vstd::set_lib::lemma_len_subset(right_tree_final@, old_tree);
                    lemma_key_unique_subset(old_tree, right_tree_final@);
                    lemma_view_gen_subset::<K, V>(right_tree_final@, old_tree);
                }
                (left, right)
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to in_order
        fn collect(&self) -> (entries: Vec<Pair<K, V>>)
        {
            let sorted = self.inner.in_order();
            let len = sorted.length();
            let mut out: Vec<Pair<K, V>> = Vec::with_capacity(len);
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len as nat == sorted@.len(),
                    obeys_feq_full::<Pair<K, V>>(),
                    out@.len() == i as nat,
                    forall|j: int| 0 <= j < i as int ==>
                        (#[trigger] out@[j])@ == sorted@[j],
                decreases len - i,
            {
                let elem = sorted.nth(i);
                let cloned = elem.clone_plus();
                proof { lemma_cloned_view_eq(*elem, cloned); }
                out.push(cloned);
                i = i + 1;
            }
            proof {
                lemma_pair_set_to_map_len(self.inner@);
                lemma_pair_set_to_map_dom_finite(self.inner@);
            }
            out
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- in_order + conditional BST inserts
        fn filter<F: Fn(&K, &V) -> bool>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(K::V, V::V) -> bool>,
        ) -> (filtered: Self)
        {
            // Use ParamBST::filter with an adapter predicate.
            let pair_pred = |p: &Pair<K, V>| -> (keep: bool)
                ensures keep == spec_pred(p.0@, p.1@)
            {
                f(&p.0, &p.1)
            };
            let ghost pair_spec_pred = |pv: (K::V, V::V)| -> bool { spec_pred(pv.0, pv.1) };
            let filtered_tree = self.inner.filter(pair_pred, Ghost(pair_spec_pred));
            let filtered = OrdKeyMap { inner: filtered_tree };
            proof {
                lemma_pair_set_to_map_dom_finite(filtered_tree@);
                lemma_pair_set_to_map_dom_finite(self.inner@);
                lemma_key_unique_subset(self.inner@, filtered_tree@);
                // filtered_tree@ ⊆ self.inner@.
                assert(filtered@.dom().subset_of(self@.dom())) by {
                    assert forall|k: K::V| filtered@.dom().contains(k)
                        implies #[trigger] self@.dom().contains(k)
                    by {
                        lemma_map_contains_pair_in_set(filtered_tree@, k);
                        let v: V::V = choose|v: V::V| filtered_tree@.contains((k, v));
                        assert(self.inner@.contains((k, v)));
                        lemma_pair_in_set_map_contains(self.inner@, k, v);
                    };
                };
                assert forall|k: K::V| #[trigger] filtered@.contains_key(k)
                    implies filtered@[k] == self@[k]
                by {
                    lemma_map_contains_pair_in_set(filtered_tree@, k);
                    let v: V::V = choose|v: V::V| filtered_tree@.contains((k, v));
                    assert(self.inner@.contains((k, v)));
                    lemma_pair_in_set_map_contains(self.inner@, k, v);
                    lemma_pair_in_set_map_contains(filtered_tree@, k, v);
                };
                assert forall|k: K::V| self@.dom().contains(k) && spec_pred(k, self@[k])
                    implies #[trigger] filtered@.dom().contains(k)
                by {
                    lemma_map_contains_pair_in_set(self.inner@, k);
                    let v: V::V = choose|v: V::V| self.inner@.contains((k, v));
                    lemma_pair_in_set_map_contains(self.inner@, k, v);
                    // v == self@[k], so spec_pred(k, v) holds.
                    assert(pair_spec_pred((k, v)));
                    assert(filtered_tree@.contains((k, v)));
                    lemma_pair_in_set_map_contains(filtered_tree@, k, v);
                };
                vstd::set_lib::lemma_len_subset(filtered_tree@, self.inner@);
                assert(spec_pair_key_determines_order::<K, V>());
                assert(vstd::laws_cmp::obeys_cmp_spec::<K>());
                assert(view_ord_consistent::<K>());
                assert(obeys_feq_fulls::<K, V>());
                lemma_view_gen_subset::<K, V>(filtered_tree@, self.inner@);
            }
            filtered
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- in_order + BST inserts
        fn map_values<F: Fn(&K, &V) -> V>(&self, f: F) -> (mapped: Self)
        {
            let sorted = self.inner.in_order();
            let len = sorted.length();
            let mut new_tree = ParamBST::<Pair<K, V>>::new();
            let mut i: usize = 0;
            proof {
                lemma_sorted_keys_pairwise_distinct(self.inner@, sorted@);
                lemma_key_unique_empty::<K::V, V::V>();
                assert(spec_set_pair_view_generated::<K, V>(new_tree@)) by {
                    assert forall|elem: (K::V, V::V)| new_tree@.contains(elem)
                        implies exists|p: Pair<K, V>| (#[trigger] p@) == elem by {
                        assert(false);
                    };
                };
            }
            while i < len
                invariant
                    0 <= i <= len,
                    len as nat == sorted@.len(),
                    self.spec_ordkeymap_wf(),
                    forall|k: &K, v: &V| #[trigger] f.requires((k, v)),
                    obeys_feq_clone::<Pair<K, V>>(),
                    new_tree.spec_bstparasteph_wf(),
                    vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                    view_ord_consistent::<Pair<K, V>>(),
                    sorted@.len() == self.inner@.len(),
                    forall|v: (K::V, V::V)| self.inner@.contains(v) <==> #[trigger] sorted@.contains(v),
                    forall|ii: int, jj: int|
                        0 <= ii < sorted@.len() && 0 <= jj < sorted@.len() && ii != jj
                        ==> (#[trigger] sorted@[ii]).0 != (#[trigger] sorted@[jj]).0,
                    new_tree@.len() == i as nat,
                    new_tree@.len() < usize::MAX as nat,
                    spec_key_unique_pairs_set(new_tree@),
                    forall|p: (K::V, V::V)| new_tree@.contains(p) ==>
                        exists|j: int| 0 <= j < i as int && p.0 == (#[trigger] sorted@[j]).0,
                    forall|j: int| 0 <= j < i as int
                        ==> #[trigger] spec_pair_set_to_map(new_tree@).dom().contains(sorted@[j].0),
                    spec_set_pair_view_generated::<K, V>(new_tree@),
                decreases len - i,
            {
                let pair = sorted.nth(i);
                let new_val = f(&pair.0, &pair.1);
                let k_clone = pair.0.clone_plus();
                let ghost old_new_tree_view = new_tree@;
                proof {
                    lemma_cloned_view_eq(pair.0, k_clone);
                    assert(!spec_pair_set_to_map(old_new_tree_view).dom().contains(sorted@[i as int].0)) by {
                        if spec_pair_set_to_map(old_new_tree_view).dom().contains(sorted@[i as int].0) {
                            lemma_map_contains_pair_in_set(old_new_tree_view, sorted@[i as int].0);
                            let vv: V::V = choose|vv: V::V| old_new_tree_view.contains((sorted@[i as int].0, vv));
                            let jj = choose|jj: int| 0 <= jj < i as int && (sorted@[i as int].0, vv).0 == (#[trigger] sorted@[jj]).0;
                            assert(false);
                        }
                    };
                }
                new_tree.insert(Pair(k_clone, new_val));
                proof {
                    lemma_view_gen_insert::<K, V>(old_new_tree_view, Pair(k_clone, new_val));
                    assert(new_tree@.len() == i as nat + 1);
                    assert(new_tree@.len() < usize::MAX as nat);
                    lemma_key_unique_insert(old_new_tree_view, sorted@[i as int].0, new_val@);
                    assert forall|j: int| 0 <= j < i as int + 1
                        implies #[trigger] spec_pair_set_to_map(new_tree@).dom().contains(sorted@[j].0)
                    by {
                        if j == i as int {
                            assert(new_tree@.contains((sorted@[i as int].0, new_val@)));
                            lemma_pair_in_set_map_contains(new_tree@, sorted@[i as int].0, new_val@);
                        } else {
                            lemma_map_contains_pair_in_set(old_new_tree_view, sorted@[j].0);
                            let w: V::V = choose|w: V::V| old_new_tree_view.contains((sorted@[j].0, w));
                            assert(new_tree@.contains((sorted@[j].0, w)));
                            lemma_pair_in_set_map_contains(new_tree@, sorted@[j].0, w);
                        }
                    };
                }
                i = i + 1;
            }
            let mapped = OrdKeyMap { inner: new_tree };
            proof {
                lemma_pair_set_to_map_dom_finite(new_tree@);
                lemma_pair_set_to_map_dom_finite(self.inner@);
                assert(mapped@.dom() =~= self@.dom()) by {
                    assert forall|key: K::V| #[trigger] mapped@.dom().contains(key)
                        implies self@.dom().contains(key)
                    by {
                        lemma_map_contains_pair_in_set(new_tree@, key);
                        let v: V::V = choose|v: V::V| new_tree@.contains((key, v));
                        let j = choose|j: int| 0 <= j < i as int && (key, v).0 == (#[trigger] sorted@[j]).0;
                        assert(self.inner@.contains(sorted@[j]));
                        lemma_pair_in_set_map_contains(self.inner@, sorted@[j].0, sorted@[j].1);
                    };
                    assert forall|key: K::V| self@.dom().contains(key)
                        implies #[trigger] mapped@.dom().contains(key)
                    by {
                        lemma_map_contains_pair_in_set(self.inner@, key);
                        let v: V::V = choose|v: V::V| self.inner@.contains((key, v));
                        assert(sorted@.contains((key, v)));
                        let j = choose|j: int| 0 <= j < sorted@.len() && sorted@[j] == (key, v);
                        assert(spec_pair_set_to_map(new_tree@).dom().contains(sorted@[j].0));
                        assert(sorted@[j].0 == key);
                    };
                };
                assert(spec_pair_key_determines_order::<K, V>());
                assert(vstd::laws_cmp::obeys_cmp_spec::<K>());
                assert(view_ord_consistent::<K>());
                assert(obeys_feq_fulls::<K, V>());
            }
            mapped
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- in_order traversal + fold
        fn reduce<F: Fn(&V, &V) -> V>(&self, f: F, id: &V) -> (reduced: V)
            ensures self@.dom().finite()
        {
            let sorted = self.inner.in_order();
            let len = sorted.length();
            let mut reduced = id.clone_plus();
            proof { lemma_cloned_view_eq(*id, reduced); }
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len as nat == sorted@.len(),
                    forall|v1: &V, v2: &V| #[trigger] f.requires((v1, v2)),
                decreases len - i,
            {
                let pair = sorted.nth(i);
                reduced = f(&reduced, &pair.1);
                i = i + 1;
            }
            proof { lemma_pair_set_to_map_dom_finite(self.inner@); }
            reduced
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- in_order traversal + key clones
        fn domain(&self) -> (keys: ArraySetStEph<K>)
        {
            let sorted = self.inner.in_order();
            let len = sorted.length();
            let mut domain = ArraySetStEph::empty();
            let mut i: usize = 0;
            proof {
                lemma_pair_set_to_map_dom_finite(self.inner@);
            }
            while i < len
                invariant
                    obeys_feq_full::<K>(),
                    len as nat == sorted@.len(),
                    sorted@.len() == self.inner@.len(),
                    forall|v: <Pair<K, V> as View>::V| self.inner@.contains(v) <==> #[trigger] sorted@.contains(v),
                    0 <= i <= len,
                    domain.spec_arraysetsteph_wf(),
                    domain@.finite(),
                    forall|kv: K::V| domain@.contains(kv) ==>
                        #[trigger] self@.dom().contains(kv),
                    forall|j: int| 0 <= j < i ==>
                        domain@.contains(#[trigger] sorted@[j].0),
                    self.spec_ordkeymap_wf(),
                decreases len - i,
            {
                let pair = sorted.nth(i);
                let key_clone = pair.0.clone_plus();
                proof {
                    assert(cloned(pair.0, key_clone));
                    assert(key_clone@ == sorted@[i as int].0);
                    let ghost elem = sorted@[i as int];
                    assert(sorted@.contains(elem)) by {
                        assert(sorted@[i as int] == elem);
                    };
                    assert(self.inner@.contains(elem));
                    lemma_pair_in_set_map_contains(self.inner@, elem.0, elem.1);
                }
                domain.insert(key_clone);
                i += 1;
            }
            proof {
                assert(domain@ =~= self@.dom()) by {
                    assert forall|kv: K::V| self@.dom().contains(kv)
                        implies #[trigger] domain@.contains(kv)
                    by {
                        lemma_map_contains_pair_in_set(self.inner@, kv);
                        let v: V::V = choose|v: V::V| self.inner@.contains((kv, v));
                        assert(sorted@.contains((kv, v)));
                        let j = choose|j: int| 0 <= j < sorted@.len()
                            && (#[trigger] sorted@[j]) == (kv, v);
                        assert(domain@.contains(sorted@[j].0));
                    };
                };
            }
            domain
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- n BST inserts
        fn tabulate<F: Fn(&K) -> V>(keys: &ArraySetStEph<K>, f: &F) -> (table: Self)
        {
            proof {
                assert(obeys_feq_full_trigger::<K>());
                assert(obeys_feq_full_trigger::<V>());
                assert(obeys_feq_full_trigger::<Pair<K, V>>());
            }
            let seq = keys.to_seq();
            let len = seq.length();
            let ghost seq_view = seq@;
            let mut tree = ParamBST::<Pair<K, V>>::new();
            let ghost mut key_args: Seq<K> = Seq::empty();
            let ghost mut results: Seq<V> = Seq::empty();
            let mut i: usize = 0;
            proof {
                seq_view.unique_seq_to_set();
                assert(seq_view.len() == keys@.len());
                lemma_key_unique_empty::<K::V, V::V>();
                assert(spec_set_pair_view_generated::<K, V>(tree@)) by {
                    assert forall|elem: (K::V, V::V)| tree@.contains(elem)
                        implies exists|p: Pair<K, V>| (#[trigger] p@) == elem by {
                        assert(false);
                    };
                };
            }
            while i < len
                invariant
                    0 <= i <= len,
                    len as int == seq_view.len(),
                    seq_view == seq@,
                    seq_view.no_duplicates(),
                    seq_view.to_set() =~= keys@,
                    forall|k: &K| #[trigger] f.requires((k,)),
                    obeys_feq_full::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
                    tree.spec_bstparasteph_wf(),
                    vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                    view_ord_consistent::<Pair<K, V>>(),
                    tree@.len() == i as nat,
                    seq_view.len() == keys@.len(),
                    keys@.len() < usize::MAX as nat,
                    tree@.len() < usize::MAX as nat,
                    spec_key_unique_pairs_set(tree@),
                    key_args.len() == i as int,
                    results.len() == i as int,
                    forall|j: int| 0 <= j < i as int ==> {
                        &&& tree@.contains((seq_view[j], (#[trigger] results[j])@))
                        &&& key_args[j]@ == seq_view[j]
                        &&& f.ensures((&key_args[j],), results[j])
                    },
                    forall|p: (K::V, V::V)| tree@.contains(p) ==>
                        exists|j: int| 0 <= j < i as int && p.0 == seq_view[j],
                    keys@.len() < usize::MAX as nat,
                    spec_set_pair_view_generated::<K, V>(tree@),
                decreases len - i,
            {
                let k = seq.nth(i);
                let val = f(k);
                let k_clone = k.clone_plus();
                let ghost old_tree = tree@;
                proof {
                    assert(cloned(*k, k_clone));
                    key_args = key_args.push(*k);
                    results = results.push(val);
                    assert(!tree@.contains((seq_view[i as int], val@))) by {
                        if tree@.contains((seq_view[i as int], val@)) {
                            let j = choose|j: int| 0 <= j < i as int
                                && (seq_view[i as int], val@).0 == seq_view[j];
                            assert(seq_view[i as int] == seq_view[j]);
                            assert(false);
                        }
                    };
                }
                tree.insert(Pair(k_clone, val));
                proof {
                    lemma_view_gen_insert::<K, V>(old_tree, Pair(k_clone, val));
                    assert(old_tree.finite());
                    assert(!old_tree.contains((seq_view[i as int], val@)));
                    assert(tree@ =~= old_tree.insert((seq_view[i as int], val@)));
                    assert(tree@.len() == i as nat + 1);
                    assert((i as nat + 1) <= len as nat);
                    assert(tree@.len() < usize::MAX as nat);
                    assert(!spec_pair_set_to_map(old_tree).dom().contains(seq_view[i as int])) by {
                        if spec_pair_set_to_map(old_tree).dom().contains(seq_view[i as int]) {
                            lemma_map_contains_pair_in_set(old_tree, seq_view[i as int]);
                            let vv: V::V = choose|vv: V::V| old_tree.contains((seq_view[i as int], vv));
                            let j = choose|j: int| 0 <= j < i as int
                                && (seq_view[i as int], vv).0 == seq_view[j];
                            assert(false);
                        }
                    };
                    lemma_key_unique_insert(old_tree, seq_view[i as int], val@);
                }
                i += 1;
            }
            let table = OrdKeyMap { inner: tree };
            proof {
                lemma_pair_set_to_map_dom_finite(tree@);
                assert(table@.dom() =~= keys@) by {
                    assert forall|key: K::V| #[trigger] table@.dom().contains(key)
                        implies keys@.contains(key)
                    by {
                        lemma_map_contains_pair_in_set(tree@, key);
                        let v: V::V = choose|v: V::V| tree@.contains((key, v));
                        let j = choose|j: int| 0 <= j < i as int && (key, v).0 == seq_view[j];
                        assert(seq_view.to_set().contains(seq_view[j]));
                    };
                    assert forall|key: K::V| keys@.contains(key)
                        implies #[trigger] table@.dom().contains(key)
                    by {
                        assert(seq_view.to_set().contains(key));
                        let j = choose|j: int| 0 <= j < seq_view.len()
                            && (#[trigger] seq_view[j]) == key;
                        assert(tree@.contains((seq_view[j], results[j]@)));
                        lemma_pair_in_set_map_contains(tree@, key, results[j]@);
                    };
                };
                assert forall|key: K::V| #[trigger] table@.contains_key(key)
                    implies (exists|key_arg: K, result: V|
                        key_arg@ == key && f.ensures((&key_arg,), result)
                        && table@[key] == result@)
                by {
                    lemma_map_contains_pair_in_set(tree@, key);
                    let v: V::V = choose|v: V::V| tree@.contains((key, v));
                    let j = choose|j: int| 0 <= j < i as int && (key, v).0 == seq_view[j];
                    let ka = key_args[j];
                    let rv = results[j];
                    assert(ka@ == key);
                    assert(f.ensures((&ka,), rv));
                    lemma_pair_in_set_map_contains(tree@, key, rv@);
                };
                assert(obeys_feq_fulls::<K, V>());
            }
            table
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- in_order + conditional BST inserts
        fn restrict(&self, keys: &ArraySetStEph<K>) -> (restricted: Self)
        {
            let ghost old_tree = self.inner@;
            let ghost old_map = self@;
            let ghost keys_set = keys@;
            let sorted = self.inner.in_order();
            let len = sorted.length();
            let mut new_tree = ParamBST::<Pair<K, V>>::new();
            let mut i: usize = 0;
            proof {
                lemma_sorted_keys_pairwise_distinct(old_tree, sorted@);
                lemma_key_unique_empty::<K::V, V::V>();
            }
            while i < len
                invariant
                    self.inner@ == old_tree,
                    self.spec_ordkeymap_wf(),
                    obeys_feq_full::<Pair<K, V>>(),
                    keys@ == keys_set,
                    keys@.finite(),
                    old_map == spec_pair_set_to_map(old_tree),
                    len as nat == sorted@.len(),
                    sorted@.len() == old_tree.len(),
                    forall|v: <Pair<K, V> as View>::V| old_tree.contains(v) <==> #[trigger] sorted@.contains(v),
                    forall|ii: int, jj: int|
                        0 <= ii < sorted@.len() && 0 <= jj < sorted@.len() && ii != jj
                        ==> (#[trigger] sorted@[ii]).0 != (#[trigger] sorted@[jj]).0,
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        exists|j: int| 0 <= j < i as int && p.0 == (#[trigger] sorted@[j]).0,
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==> old_tree.contains(p),
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        keys_set.contains(p.0),
                    forall|j: int| 0 <= j < i as int && keys_set.contains(sorted@[j].0)
                        ==> #[trigger] new_tree@.contains(sorted@[j]),
                    0 <= i <= len,
                    new_tree.spec_bstparasteph_wf(),
                    vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                    view_ord_consistent::<Pair<K, V>>(),
                    new_tree@.len() <= i as nat,
                    new_tree@.len() < usize::MAX as nat,
                    spec_key_unique_pairs_set(new_tree@),
                    spec_key_unique_pairs_set(old_tree),
                decreases len - i,
            {
                let pair = sorted.nth(i);
                let in_keys = keys.find(&pair.0);
                if in_keys {
                    let cloned = pair.clone_plus();
                    let ghost old_new_tree_view = new_tree@;
                    proof {
                        lemma_cloned_view_eq(*pair, cloned);
                        assert(!spec_pair_set_to_map(old_new_tree_view).dom().contains(sorted@[i as int].0)) by {
                            if spec_pair_set_to_map(old_new_tree_view).dom().contains(sorted@[i as int].0) {
                                lemma_map_contains_pair_in_set(old_new_tree_view, sorted@[i as int].0);
                                let vv: V::V = choose|vv: V::V| old_new_tree_view.contains((sorted@[i as int].0, vv));
                                let jj = choose|jj: int| 0 <= jj < i as int && (sorted@[i as int].0, vv).0 == (#[trigger] sorted@[jj]).0;
                                assert(false);
                            }
                        };
                    }
                    new_tree.insert(cloned);
                    proof {
                        assert(new_tree@.len() <= i as nat + 1);
                        assert(new_tree@.len() < usize::MAX as nat);
                        lemma_key_unique_insert(old_new_tree_view, sorted@[i as int].0, sorted@[i as int].1);
                        assert(sorted@.contains(sorted@[i as int])) by { assert(sorted@[i as int] == sorted@[i as int]); };
                        assert(old_tree.contains(sorted@[i as int]));
                        assert(new_tree@.contains(sorted@[i as int]));
                    }
                }
                i = i + 1;
            }
            let restricted = OrdKeyMap { inner: new_tree };
            proof {
                lemma_pair_set_to_map_dom_finite(new_tree@);
                lemma_pair_set_to_map_dom_finite(old_tree);
                assert(restricted@.dom() =~= old_map.dom().intersect(keys_set)) by {
                    assert forall|k: K::V| #[trigger] restricted@.dom().contains(k)
                        implies old_map.dom().contains(k) && keys_set.contains(k)
                    by {
                        lemma_map_contains_pair_in_set(new_tree@, k);
                        let v: V::V = choose|v: V::V| new_tree@.contains((k, v));
                        assert(old_tree.contains((k, v)));
                        lemma_pair_in_set_map_contains(old_tree, k, v);
                        assert(keys_set.contains(k));
                    };
                    assert forall|k: K::V|
                        old_map.dom().contains(k) && keys_set.contains(k)
                        implies #[trigger] restricted@.dom().contains(k)
                    by {
                        lemma_map_contains_pair_in_set(old_tree, k);
                        let v: V::V = choose|v: V::V| old_tree.contains((k, v));
                        assert(sorted@.contains((k, v)));
                        let j = choose|j: int| 0 <= j < sorted@.len() && sorted@[j] == (k, v);
                        assert(new_tree@.contains(sorted@[j]));
                        lemma_pair_in_set_map_contains(new_tree@, k, v);
                    };
                };
                assert forall|k: K::V| #[trigger] restricted@.contains_key(k)
                    implies restricted@[k] == old_map[k]
                by {
                    lemma_map_contains_pair_in_set(new_tree@, k);
                    let v: V::V = choose|v: V::V| new_tree@.contains((k, v));
                    assert(old_tree.contains((k, v)));
                    lemma_pair_in_set_map_contains(new_tree@, k, v);
                    lemma_pair_in_set_map_contains(old_tree, k, v);
                };
                assert(spec_pair_key_determines_order::<K, V>());
                assert(vstd::laws_cmp::obeys_cmp_spec::<K>());
                assert(view_ord_consistent::<K>());
                assert(obeys_feq_fulls::<K, V>());
                lemma_view_gen_subset::<K, V>(new_tree@, old_tree);
            }
            restricted
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- in_order + conditional BST inserts
        fn subtract(&self, keys: &ArraySetStEph<K>) -> (remaining: Self)
        {
            let ghost old_tree = self.inner@;
            let ghost old_map = self@;
            let ghost keys_set = keys@;
            let sorted = self.inner.in_order();
            let len = sorted.length();
            let mut new_tree = ParamBST::<Pair<K, V>>::new();
            let mut i: usize = 0;
            proof {
                lemma_sorted_keys_pairwise_distinct(old_tree, sorted@);
                lemma_key_unique_empty::<K::V, V::V>();
            }
            while i < len
                invariant
                    self.inner@ == old_tree,
                    self.spec_ordkeymap_wf(),
                    obeys_feq_full::<Pair<K, V>>(),
                    keys@ == keys_set,
                    keys@.finite(),
                    old_map == spec_pair_set_to_map(old_tree),
                    len as nat == sorted@.len(),
                    sorted@.len() == old_tree.len(),
                    forall|v: <Pair<K, V> as View>::V| old_tree.contains(v) <==> #[trigger] sorted@.contains(v),
                    forall|ii: int, jj: int|
                        0 <= ii < sorted@.len() && 0 <= jj < sorted@.len() && ii != jj
                        ==> (#[trigger] sorted@[ii]).0 != (#[trigger] sorted@[jj]).0,
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        exists|j: int| 0 <= j < i as int && p.0 == (#[trigger] sorted@[j]).0,
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==> old_tree.contains(p),
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        !keys_set.contains(p.0),
                    forall|j: int| 0 <= j < i as int && !keys_set.contains(sorted@[j].0)
                        ==> #[trigger] new_tree@.contains(sorted@[j]),
                    0 <= i <= len,
                    new_tree.spec_bstparasteph_wf(),
                    vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                    view_ord_consistent::<Pair<K, V>>(),
                    new_tree@.len() <= i as nat,
                    new_tree@.len() < usize::MAX as nat,
                    spec_key_unique_pairs_set(new_tree@),
                    spec_key_unique_pairs_set(old_tree),
                decreases len - i,
            {
                let pair = sorted.nth(i);
                let in_keys = keys.find(&pair.0);
                if !in_keys {
                    let cloned = pair.clone_plus();
                    let ghost old_new_tree_view = new_tree@;
                    proof {
                        lemma_cloned_view_eq(*pair, cloned);
                        assert(!spec_pair_set_to_map(old_new_tree_view).dom().contains(sorted@[i as int].0)) by {
                            if spec_pair_set_to_map(old_new_tree_view).dom().contains(sorted@[i as int].0) {
                                lemma_map_contains_pair_in_set(old_new_tree_view, sorted@[i as int].0);
                                let vv: V::V = choose|vv: V::V| old_new_tree_view.contains((sorted@[i as int].0, vv));
                                let jj = choose|jj: int| 0 <= jj < i as int && (sorted@[i as int].0, vv).0 == (#[trigger] sorted@[jj]).0;
                                assert(false);
                            }
                        };
                    }
                    new_tree.insert(cloned);
                    proof {
                        assert(new_tree@.len() <= i as nat + 1);
                        assert(new_tree@.len() < usize::MAX as nat);
                        lemma_key_unique_insert(old_new_tree_view, sorted@[i as int].0, sorted@[i as int].1);
                        assert(sorted@.contains(sorted@[i as int])) by { assert(sorted@[i as int] == sorted@[i as int]); };
                        assert(old_tree.contains(sorted@[i as int]));
                        assert(new_tree@.contains(sorted@[i as int]));
                    }
                }
                i = i + 1;
            }
            let remaining = OrdKeyMap { inner: new_tree };
            proof {
                lemma_pair_set_to_map_dom_finite(new_tree@);
                lemma_pair_set_to_map_dom_finite(old_tree);
                assert(remaining@.dom() =~= old_map.dom().difference(keys_set)) by {
                    assert forall|k: K::V| #[trigger] remaining@.dom().contains(k)
                        implies old_map.dom().contains(k) && !keys_set.contains(k)
                    by {
                        lemma_map_contains_pair_in_set(new_tree@, k);
                        let v: V::V = choose|v: V::V| new_tree@.contains((k, v));
                        assert(old_tree.contains((k, v)));
                        lemma_pair_in_set_map_contains(old_tree, k, v);
                        assert(!keys_set.contains(k));
                    };
                    assert forall|k: K::V|
                        old_map.dom().contains(k) && !keys_set.contains(k)
                        implies #[trigger] remaining@.dom().contains(k)
                    by {
                        lemma_map_contains_pair_in_set(old_tree, k);
                        let v: V::V = choose|v: V::V| old_tree.contains((k, v));
                        assert(sorted@.contains((k, v)));
                        let j = choose|j: int| 0 <= j < sorted@.len() && sorted@[j] == (k, v);
                        assert(new_tree@.contains(sorted@[j]));
                        lemma_pair_in_set_map_contains(new_tree@, k, v);
                    };
                };
                assert forall|k: K::V| #[trigger] remaining@.contains_key(k)
                    implies remaining@[k] == old_map[k]
                by {
                    lemma_map_contains_pair_in_set(new_tree@, k);
                    let v: V::V = choose|v: V::V| new_tree@.contains((k, v));
                    assert(old_tree.contains((k, v)));
                    lemma_pair_in_set_map_contains(new_tree@, k, v);
                    lemma_pair_in_set_map_contains(old_tree, k, v);
                };
                assert(spec_pair_key_determines_order::<K, V>());
                assert(vstd::laws_cmp::obeys_cmp_spec::<K>());
                assert(view_ord_consistent::<K>());
                assert(obeys_feq_fulls::<K, V>());
                lemma_view_gen_subset::<K, V>(new_tree@, old_tree);
            }
            remaining
        }
    }

    //		Section 12. derive impls in verus!


    impl<K: StT + Ord, V: StT + Ord> Clone for OrdKeyMap<K, V> {
        fn clone(&self) -> (cloned: Self)
            ensures cloned@ == self@
        {
            OrdKeyMap { inner: self.inner.clone() }
        }
    }

} // verus!

    //		Section 14. derive impls outside verus!


    impl<K: StT + Ord + std::fmt::Debug, V: StT + Ord + std::fmt::Debug> Debug for OrdKeyMap<K, V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "OrdKeyMap({:?})", self.inner)
        }
    }

    impl<K: StT + Ord + std::fmt::Display, V: StT + Ord + std::fmt::Display> Display for OrdKeyMap<K, V> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "OrdKeyMap({})", self.inner)
        }
    }

} // pub mod OrdKeyMap
