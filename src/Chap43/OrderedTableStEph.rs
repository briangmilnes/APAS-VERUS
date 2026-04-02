//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Single-threaded ephemeral ordered table backed by ParamBST<Pair<K,V>>.

pub mod OrderedTableStEph {

    use std::cmp::Ordering::{Equal, Greater, Less};
    use std::vec::IntoIter;

    use crate::Chap38::BSTParaStEph::BSTParaStEph::*;
    use crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::*;
    use crate::Chap18::ArraySeqStPer::ArraySeqStPer::*;
    use crate::Chap19::ArraySeqStEph::ArraySeqStEph::ArraySeqStEphTrait;
    use crate::Chap41::ArraySetStEph::ArraySetStEph::*;
    use crate::Types::Types::*;
    use crate::vstdplus::clone_plus::clone_plus::*;
    use crate::vstdplus::total_order::total_order::TotalOrder;
    use crate::vstdplus::total_order::total_order::TotalOrderBridge;
    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;
    #[cfg(verus_keep_ghost)]
    use vstd::laws_eq::obeys_view_eq;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::OrdSpec;

    verus! {

// 3. broadcast use
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::map::group_map_axioms,
    vstd::set::group_set_axioms,
};

    // Table of Contents
    // 1. module (above)
    // 2. imports (above)
    // 3. broadcast use (above)
    // 4. type definitions
    // 5. view impls
    // 6. spec fns
    // 7. proof fns
    // 8. traits
    // 9. impls
    // 10. iterators
    // 11. derive impls in verus!
    // 12. macros
    // 13. derive impls outside verus!

    // 4. type definitions

    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct OrderedTableStEph<K: StT + Ord, V: StT + Ord> {
        pub tree: ParamBST<Pair<K, V>>,
    }

    pub type OrderedTableEph<K, V> = OrderedTableStEph<K, V>;

    // 5. view impls

    impl<K: StT + Ord, V: StT + Ord> View for OrderedTableStEph<K, V> {
        type V = Map<K::V, V::V>;

        open spec fn view(&self) -> Self::V { spec_pair_set_to_map(self.tree@) }
    }

    // 6. spec fns

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

    /// Spec predicate for rank_key: x is strictly less than k in the total order.
    pub open spec fn spec_rank_pred<K: StT + Ord + TotalOrder>(x: K::V, k: K) -> bool {
        exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, k) && t@ != k@
    }

    /// Connection between K's Ord ordering (cmp_spec) and K's TotalOrder (le).
    /// Required by O(lg n) ordering operations that leverage BST structure.
    /// Trivially holds for all integer types and String.
    pub open spec fn spec_ord_agrees_total_order<K: StT + Ord + TotalOrder>() -> bool {
        &&& forall|a: K, b: K| a.cmp_spec(&b) == Less ==> TotalOrder::le(a, b)
        &&& forall|a: K, b: K| a.cmp_spec(&b) == Greater ==> TotalOrder::le(b, a)
    }

    // 7. proof fns

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
        assert forall|elem: (K::V, V::V)| sub.contains(elem)
            implies exists|p: Pair<K, V>| (#[trigger] p@) == elem by {
            assert(sup.contains(elem));
        };
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
        assert forall|elem: (K::V, V::V)| s.insert(pair@).contains(elem)
            implies exists|p: Pair<K, V>| (#[trigger] p@) == elem by {
            if elem == pair@ {
                assert(pair@ == elem);
            } else {
                assert(s.contains(elem));
            }
        };
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
        assert forall|elem: (K::V, V::V)| a.union(b).contains(elem)
            implies exists|p: Pair<K, V>| (#[trigger] p@) == elem by {
            if a.contains(elem) {} else { assert(b.contains(elem)); }
        };
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
            assert(s.contains((k, v)));
            assert(proj((k, v)) == k);
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
                assert(s.contains((k, v)));
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
        assert(m.dom().contains(k));
        // m[k] == choose|v_| s.contains((k, v_)).
        // By key uniqueness, that chosen value must equal v.
        let v2 = choose|v2: VV| s.contains((k, v2));
        assert(s.contains((k, v2)));
        assert(v2 == v);
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
                    assert(spec_pair_set_to_map(s).dom().contains(k));
                }
                if s.contains((k2, v2)) {
                    assert(spec_pair_set_to_map(s).dom().contains(k));
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
        assert(a@ == b@);
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
            assert forall|v: (KV, VV)| sorted.to_set().contains(v) <==> #[trigger] tree.contains(v) by {};
        };
        // sorted.to_set().len() == tree.len() == sorted.len(), so no duplicates.
        sorted.lemma_no_dup_set_cardinality();
        // Pairwise distinct keys: same key + key uniqueness → same pair → contradicts no_duplicates.
        assert forall|i: int, j: int|
            0 <= i < sorted.len() && 0 <= j < sorted.len() && i != j
            implies (#[trigger] sorted[i]).0 != (#[trigger] sorted[j]).0
        by {
            if sorted[i].0 == sorted[j].0 {
                assert(tree.contains(sorted[i]));
                assert(tree.contains(sorted[j]));
                // Key uniqueness: same key in tree → same value → same pair.
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
                    assert(false);
                }
                if s2.contains((k, v1)) {
                    assert(false);
                }
                if s1.contains((k, v2)) {
                    assert(false);
                }
                if s2.contains((k, v2)) {
                    assert(false);
                }
            } else {
                // (k, v1) and (k, v2) are in s1 ∪ s2.
                // By key separation, both must be in the same set.
                if s1.contains((k, v1)) && s2.contains((k, v2)) {
                    assert(false); // key separation
                }
                if s2.contains((k, v1)) && s1.contains((k, v2)) {
                    assert(false); // key separation
                }
                // Both in s1 or both in s2 → key uniqueness.
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
                assert(s.contains((key, vv)));
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
                assert(new_s.contains((k, cv)));
                assert(new_s.contains((k, v)));
                lemma_key_unique_insert(s, k, v);
                assert(cv == v);
            } else {
                let cv: VV = choose|cv: VV| new_s.contains((key, cv));
                assert(s.contains((key, cv)));
                let cv2: VV = choose|cv2: VV| s.contains((key, cv2));
                assert(cv == cv2);
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
            assert(s.contains((key, vv)));
            // key != k because (k, v) was removed and (key, vv) is still there.
            // If key == k, then vv == v by key uniqueness, but (k, v) was removed.
            if key == k {
                assert(new_s.contains((k, vv)));
                assert(!new_s.contains((k, v)));
                assert(vv != v);
                assert(s.contains((k, vv)));
                assert(s.contains((k, v)));
                // Key uniqueness: vv == v. Contradiction.
                assert(false);
            }
            let cv: VV = choose|cv: VV| s.contains((key, cv));
            assert(cv == vv);
        };
        // Backward: every key in old_m.remove(k) is in new_m.
        assert forall|key: KV| old_m.remove(k).dom().contains(key)
            implies #[trigger] new_m.dom().contains(key)
        by {
            assert(key != k);
            let vv: VV = choose|vv: VV| s.contains((key, vv));
            assert(new_s.contains((key, vv)));
        };
    }

    /// The map over a union-insert equals the map over left ∪ right ∪ {root}.
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
                    assert(right.contains((k, v)));
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
            assert(combined.contains((root_k, root_v)));
        };
        let cv: VV = choose|cv: VV| combined.contains((root_k, cv));
        assert(cv == root_v) by {
            if left.contains((root_k, cv)) {
                assert(false);
            }
            if right.contains((root_k, cv)) {
                assert(false);
            }
        };
        // Left values.
        assert forall|k: KV| lm.dom().contains(k) implies #[trigger] cm[k] == lm[k] by {
            let v_l: VV = choose|v: VV| left.contains((k, v));
            assert(combined.contains((k, v_l)));
            let v_c: VV = choose|v: VV| combined.contains((k, v));
            // v_c must equal v_l: if from right, contradiction on key separation.
            if right.contains((k, v_c)) {
                assert(left.contains((k, v_l)));
                assert(false);
            }
            if (k, v_c) == (root_k, root_v) {
                assert(false);
            }
            assert(left.contains((k, v_c)));
            assert(v_c == v_l);
        };
        // Right values.
        assert forall|k: KV| rm.dom().contains(k) implies #[trigger] cm[k] == rm[k] by {
            let v_r: VV = choose|v: VV| right.contains((k, v));
            assert(combined.contains((k, v_r)));
            let v_c: VV = choose|v: VV| combined.contains((k, v));
            if left.contains((k, v_c)) {
                assert(right.contains((k, v_r)));
                assert(false);
            }
            if (k, v_c) == (root_k, root_v) {
                assert(false);
            }
            assert(right.contains((k, v_c)));
            assert(v_c == v_r);
        };
    }

    /// The map over an empty set is the empty map.
    proof fn lemma_set_to_map_empty<KV, VV>()
        ensures spec_pair_set_to_map(Set::<(KV, VV)>::empty()) =~= Map::<KV, VV>::empty()
    {
    }

    // 8. traits

    /// Trait defining all ordered table operations (ADT 42.1 + ADT 43.1) with ephemeral semantics.
    pub trait OrderedTableStEphTrait<K: StT + Ord, V: StT + Ord>: Sized + View<V = Map<K::V, V::V>> {
        spec fn spec_orderedtablesteph_wf(&self) -> bool;

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) -- agrees with APAS
        fn size(&self) -> (count: usize)
            requires self.spec_orderedtablesteph_wf(),
            ensures count == self@.dom().len(), self@.dom().finite();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) -- agrees with APAS
        fn empty() -> (empty: Self)
            ensures empty@ == Map::<K::V, V::V>::empty();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) -- agrees with APAS
        fn singleton(k: K, v: V) -> (tree: Self)
            requires
                obeys_feq_clone::<Pair<K, V>>(),
                vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                view_ord_consistent::<Pair<K, V>>(),
                spec_pair_key_determines_order::<K, V>(),
                vstd::laws_cmp::obeys_cmp_spec::<K>(),
                view_ord_consistent::<K>(),
            ensures tree@ == Map::<K::V, V::V>::empty().insert(k@, v@), tree@.dom().finite(), tree.spec_orderedtablesteph_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n) -- recursive BST descent
        fn find(&self, k: &K) -> (found: Option<V>)
            requires self.spec_orderedtablesteph_wf(), obeys_view_eq::<K>()
            ensures
                match found {
                    Some(v) => self@.contains_key(k@) && v@ == self@[k@],
                    None => !self@.contains_key(k@),
                };
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n) -- delegates to find
        fn lookup(&self, k: &K) -> (value: Option<V>)
            requires self.spec_orderedtablesteph_wf(), obeys_view_eq::<K>()
            ensures
                match value {
                    Some(v) => self@.contains_key(k@) && v@ == self@[k@],
                    None => !self@.contains_key(k@),
                };
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(1), Span Θ(1) -- agrees with APAS
        fn is_empty(&self) -> (is_empty: bool)
            requires self.spec_orderedtablesteph_wf(),
            ensures is_empty == self@.dom().is_empty();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n) -- recursive BST insert
        fn insert<F: Fn(&V, &V) -> V>(&mut self, k: K, v: V, combine: F)
            requires
                old(self).spec_orderedtablesteph_wf(),
                forall|v1: &V, v2: &V| combine.requires((v1, v2)),
                obeys_view_eq::<K>(),
                !old(self)@.contains_key(k@) ==> old(self)@.dom().len() + 1 < usize::MAX as nat,
            ensures
                self@.contains_key(k@),
                self@.dom() =~= old(self)@.dom().insert(k@),
                forall|key: K::V| key != k@ && #[trigger] old(self)@.contains_key(key) ==> self@[key] == old(self)@[key],
                !old(self)@.contains_key(k@) ==> self@[k@] == v@,
                old(self)@.contains_key(k@) ==> (exists|old_v: V, r: V|
                    old_v@ == old(self)@[k@] && combine.ensures((&old_v, &v), r) && self@[k@] == r@),
                self@.dom().finite(),
                self.spec_orderedtablesteph_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n) -- recursive BST delete
        fn delete(&mut self, k: &K) -> (updated: Option<V>)
            requires
                old(self).spec_orderedtablesteph_wf(),
                obeys_feq_clone::<Pair<K, V>>(),
                obeys_view_eq::<K>(),
            ensures self@ == old(self)@.remove(k@), self@.dom().finite(), self.spec_orderedtablesteph_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- collects keys from in_order
        fn domain(&self) -> (domain: ArraySetStEph<K>)
            requires self.spec_orderedtablesteph_wf(), obeys_feq_clone::<K>()
            ensures domain@ =~= self@.dom(), self@.dom().finite();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n log n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- inserts keys one by one
        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (tabulated: Self)
            requires
                keys.spec_arraysetsteph_wf(),
                forall|k: &K| f.requires((k,)),
                obeys_feq_full::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
                keys@.len() < usize::MAX as nat,
                vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                view_ord_consistent::<Pair<K, V>>(),
                spec_pair_key_determines_order::<K, V>(),
                vstd::laws_cmp::obeys_cmp_spec::<K>(),
                view_ord_consistent::<K>(),
                obeys_feq_fulls::<K, V>(),
            ensures
                tabulated@.dom() =~= keys@,
                tabulated.spec_orderedtablesteph_wf(),
                forall|k: K::V| #[trigger] tabulated@.contains_key(k) ==>
                    (exists|key_arg: K, result: V|
                        key_arg@ == k && f.ensures((&key_arg,), result)
                        && tabulated@[k] == result@),
                tabulated@.dom().finite();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects, maps, rebuilds
        fn map<F: Fn(&K, &V) -> V>(&self, f: F) -> (mapped: Self)
            requires
                self.spec_orderedtablesteph_wf(),
                forall|k: &K, v: &V| f.requires((k, v)),
                obeys_feq_clone::<Pair<K, V>>(),
            ensures mapped@.dom() =~= self@.dom(), mapped@.dom().finite(), mapped.spec_orderedtablesteph_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(n log n), Span Θ(n log n) -- collects, filters, rebuilds
        fn filter<F: Fn(&K, &V) -> bool>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(K::V, V::V) -> bool>,
        ) -> (filtered: Self)
            requires
                self.spec_orderedtablesteph_wf(),
                forall|k: &K, v: &V| f.requires((k, v)),
                forall|k: K, v: V, keep: bool|
                    f.ensures((&k, &v), keep) ==> keep == spec_pred(k@, v@),
            ensures
                filtered@.dom().subset_of(self@.dom()),
                forall|k: K::V| #[trigger] filtered@.contains_key(k) ==> filtered@[k] == self@[k],
                forall|k: K::V| self@.dom().contains(k) && spec_pred(k, self@[k])
                    ==> #[trigger] filtered@.dom().contains(k),
                filtered@.dom().finite(),
                filtered.spec_orderedtablesteph_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- iterates all entries
        fn reduce<R, F: Fn(R, &K, &V) -> R>(&self, init: R, f: F) -> (reduced: R)
            requires self.spec_orderedtablesteph_wf(), forall|r: R, k: &K, v: &V| f.requires((r, k, v))
            ensures self@.dom().finite();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) -- iterative merge
        fn intersection<F: Fn(&V, &V) -> V>(&mut self, other: &Self, f: F)
            requires
                old(self).spec_orderedtablesteph_wf(),
                other.spec_orderedtablesteph_wf(),
                forall|v1: &V, v2: &V| f.requires((v1, v2)),
                obeys_feq_clone::<K>(),
                obeys_view_eq::<K>(),
            ensures
                self@.dom() =~= old(self)@.dom().intersect(other@.dom()),
                forall|k: K::V| #[trigger] self@.contains_key(k) ==>
                    (exists|v1: V, v2: V, r: V|
                        v1@ == old(self)@[k] && v2@ == other@[k]
                        && f.ensures((&v1, &v2), r)
                        && self@[k] == r@),
                self@.dom().finite(),
                self.spec_orderedtablesteph_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) -- iterative merge
        fn union<F: Fn(&V, &V) -> V>(&mut self, other: &Self, f: F)
            requires
                old(self).spec_orderedtablesteph_wf(),
                other.spec_orderedtablesteph_wf(),
                forall|v1: &V, v2: &V| f.requires((v1, v2)),
                obeys_feq_clone::<K>(),
                obeys_view_eq::<K>(),
                old(self)@.dom().len() + other@.dom().len() < usize::MAX,
            ensures
                self@.dom() =~= old(self)@.dom().union(other@.dom()),
                forall|k: K::V| #[trigger] old(self)@.contains_key(k) && !other@.contains_key(k)
                    ==> self@[k] == old(self)@[k],
                forall|k: K::V| #[trigger] other@.contains_key(k) && !old(self)@.contains_key(k)
                    ==> self@[k] == other@[k],
                forall|k: K::V| #[trigger] old(self)@.contains_key(k) && other@.contains_key(k) ==>
                    (exists|v1: V, v2: V, r: V|
                        v1@ == old(self)@[k] && v2@ == other@[k]
                        && f.ensures((&v1, &v2), r)
                        && self@[k] == r@),
                self@.dom().finite(),
                self.spec_orderedtablesteph_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) -- iterative difference
        fn difference(&mut self, other: &Self)
            requires old(self).spec_orderedtablesteph_wf(), other.spec_orderedtablesteph_wf(),obeys_view_eq::<K>()
            ensures
                self@.dom() =~= old(self)@.dom().difference(other@.dom()),
                forall|k: K::V| #[trigger] self@.contains_key(k) ==> self@[k] == old(self)@[k],
                self@.dom().finite(),
                self.spec_orderedtablesteph_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(n * m), Span Θ(n * m) -- iterative restrict
        fn restrict(&mut self, keys: &ArraySetStEph<K>)
            requires old(self).spec_orderedtablesteph_wf()
            ensures
                self@.dom() =~= old(self)@.dom().intersect(keys@),
                forall|k: K::V| #[trigger] self@.contains_key(k) ==> self@[k] == old(self)@[k],
                self@.dom().finite(),
                self.spec_orderedtablesteph_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(n * m), Span Θ(n * m) -- iterative subtract
        fn subtract(&mut self, keys: &ArraySetStEph<K>)
            requires old(self).spec_orderedtablesteph_wf()
            ensures
                self@.dom() =~= old(self)@.dom().difference(keys@),
                forall|k: K::V| #[trigger] self@.contains_key(k) ==> self@[k] == old(self)@[k],
                self@.dom().finite(),
                self.spec_orderedtablesteph_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n log n), Span O(n log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(n), Span Θ(n) -- collects from in_order
        fn collect(&self) -> (collected: AVLTreeSeqStPerS<Pair<K, V>>)
            requires self.spec_orderedtablesteph_wf()
            ensures self@.dom().finite(), collected.spec_avltreeseqstper_wf(), collected@.len() == self@.dom().len();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n) -- recursive BST min
        fn first_key(&self) -> (first: Option<K>)
            where K: TotalOrderBridge
            requires self.spec_orderedtablesteph_wf()
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> first matches None,
                first matches Some(k) ==> self@.dom().contains(k@),
                first matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(v, t);
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n) -- recursive BST max
        fn last_key(&self) -> (last: Option<K>)
            where K: TotalOrderBridge
            requires self.spec_orderedtablesteph_wf()            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> last matches None,
                last matches Some(k) ==> self@.dom().contains(k@),
                last matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(t, v);
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n) -- recursive BST predecessor
        fn previous_key(&self, k: &K) -> (predecessor: Option<K>)
            where K: TotalOrderBridge
            requires self.spec_orderedtablesteph_wf()            ensures
                self@.dom().finite(),
                predecessor matches Some(pk) ==> self@.dom().contains(pk@),
                predecessor matches Some(v) ==> TotalOrder::le(v, *k) && v@ != k@,
                predecessor matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(t, *k) && t@ != k@ ==> TotalOrder::le(t, v);
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n) -- recursive BST successor
        fn next_key(&self, k: &K) -> (successor: Option<K>)
            where K: TotalOrderBridge
            requires self.spec_orderedtablesteph_wf()            ensures
                self@.dom().finite(),
                successor matches Some(nk) ==> self@.dom().contains(nk@),
                successor matches Some(v) ==> TotalOrder::le(*k, v) && v@ != k@,
                successor matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(*k, t) && t@ != k@ ==> TotalOrder::le(v, t);
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n) -- recursive BST split
        fn split_key(&mut self, k: &K) -> (split: (Self, Option<V>, Self))
            where Self: Sized
            requires
                old(self).spec_orderedtablesteph_wf(),
                obeys_view_eq::<K>(),
            ensures
                self@.dom().finite(),
                old(self)@.dom().finite(),
                split.0@.dom().finite(),
                split.2@.dom().finite(),
                split.1 matches Some(v) ==> old(self)@.contains_key(k@) && v@ == old(self)@[k@],
                split.1 matches None ==> !old(self)@.contains_key(k@),
                !split.0@.dom().contains(k@),
                !split.2@.dom().contains(k@),
                split.0@.dom().subset_of(old(self)@.dom()),
                split.2@.dom().subset_of(old(self)@.dom()),
                split.0@.dom().disjoint(split.2@.dom()),
                forall|key| #[trigger] old(self)@.dom().contains(key) ==> split.0@.dom().contains(key) || split.2@.dom().contains(key) || key == k@,
                split.0.spec_orderedtablesteph_wf(),
                split.2.spec_orderedtablesteph_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(n + m), Span Θ(n + m) -- delegates to union
        fn join_key(&mut self, other: Self)
            requires
                old(self).spec_orderedtablesteph_wf(),
                other.spec_orderedtablesteph_wf(),
                obeys_feq_clone::<K>(),
                obeys_view_eq::<K>(),
                old(self)@.dom().len() + other@.dom().len() < usize::MAX,
            ensures
                self@.dom() =~= old(self)@.dom().union(other@.dom()),
                self@.dom().finite(),
                self.spec_orderedtablesteph_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n + m) where m = output size, Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n + m) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n) -- recursive BST range
        fn get_key_range(&self, k1: &K, k2: &K) -> (range: Self)
            requires
                self.spec_orderedtablesteph_wf(),
            ensures
                range@.dom().finite(),
                range@.dom().subset_of(self@.dom()),
                forall|key| #[trigger] range@.dom().contains(key) ==> range@[key] == self@[key],
                range.spec_orderedtablesteph_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n) -- recursive BST rank
        fn rank_key(&self, k: &K) -> (rank: usize)
            where K: TotalOrderBridge
            requires
                self.spec_orderedtablesteph_wf(),
                obeys_view_eq::<K>(),
            ensures
                self@.dom().finite(),
                rank <= self@.dom().len(),
                rank as int == self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, *k) && t@ != k@).len();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n) -- recursive BST select
        fn select_key(&self, i: usize) -> (selected: Option<K>)
            where K: TotalOrderBridge
            requires
                self.spec_orderedtablesteph_wf(),
                obeys_view_eq::<K>(),
            ensures
                self@.dom().finite(),
                i >= self@.dom().len() ==> selected matches None,
                selected matches Some(k) ==> self@.dom().contains(k@),
                selected matches Some(v) ==> self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, v) && t@ != v@).len() == i as int;
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS
        /// - Claude-Opus-4.6: Work Θ(log n), Span Θ(log n) -- recursive BST split by rank
        fn split_rank_key(&mut self, i: usize) -> (split: (Self, Self))
            where Self: Sized
            requires
                old(self).spec_orderedtablesteph_wf(),
            ensures
                self@.dom().finite(),
                old(self)@.dom().finite(),
                split.0@.dom().finite(),
                split.1@.dom().finite(),
                split.0@.dom().subset_of(old(self)@.dom()),
                split.1@.dom().subset_of(old(self)@.dom()),
                split.0@.dom().disjoint(split.1@.dom()),
                forall|key| #[trigger] old(self)@.dom().contains(key) ==> split.0@.dom().contains(key) || split.1@.dom().contains(key),
                split.0.spec_orderedtablesteph_wf(),
                split.1.spec_orderedtablesteph_wf();
        /// Iterative alternative to `find`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- delegates to find
        fn find_iter(&self, k: &K) -> (found: Option<V>)
            requires self.spec_orderedtablesteph_wf(), obeys_view_eq::<K>()
            ensures
                match found {
                    Some(v) => self@.contains_key(k@) && v@ == self@[k@],
                    None => !self@.contains_key(k@),
                };
        /// Iterative alternative to `insert`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to insert
        fn insert_iter<F: Fn(&V, &V) -> V>(&mut self, k: K, v: V, combine: F)
            requires
                old(self).spec_orderedtablesteph_wf(),
                forall|v1: &V, v2: &V| combine.requires((v1, v2)),
                obeys_view_eq::<K>(),
                !old(self)@.contains_key(k@) ==> old(self)@.dom().len() + 1 < usize::MAX as nat,
            ensures
                self@.contains_key(k@),
                self@.dom() =~= old(self)@.dom().insert(k@),
                forall|key: K::V| key != k@ && #[trigger] old(self)@.contains_key(key) ==> self@[key] == old(self)@[key],
                !old(self)@.contains_key(k@) ==> self@[k@] == v@,
                old(self)@.contains_key(k@) ==> (exists|old_v: V, r: V|
                    old_v@ == old(self)@[k@] && combine.ensures((&old_v, &v), r) && self@[k@] == r@),
                self@.dom().finite(),
                self.spec_orderedtablesteph_wf();
        /// Iterative alternative to `delete`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to delete
        fn delete_iter(&mut self, k: &K) -> (updated: Option<V>)
            requires
                old(self).spec_orderedtablesteph_wf(),
                obeys_feq_clone::<Pair<K, V>>(),
                obeys_view_eq::<K>(),
            ensures self@ == old(self)@.remove(k@), self@.dom().finite(), self.spec_orderedtablesteph_wf();
        /// Iterative alternative to `first_key`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- BST min_key
        fn first_key_iter(&self) -> (first: Option<K>)
            where K: TotalOrderBridge
            requires self.spec_orderedtablesteph_wf()            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> first matches None,
                first matches Some(k) ==> self@.dom().contains(k@),
                first matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(v, t);
        /// Iterative alternative to `last_key`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- BST max_key
        fn last_key_iter(&self) -> (last: Option<K>)
            where K: TotalOrderBridge
            requires self.spec_orderedtablesteph_wf()            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> last matches None,
                last matches Some(k) ==> self@.dom().contains(k@),
                last matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(t, v);
        /// Iterative alternative to `previous_key`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- BST predecessor
        fn previous_key_iter(&self, k: &K) -> (predecessor: Option<K>)
            where K: TotalOrderBridge
            requires self.spec_orderedtablesteph_wf()            ensures
                self@.dom().finite(),
                predecessor matches Some(pk) ==> self@.dom().contains(pk@),
                predecessor matches Some(v) ==> TotalOrder::le(v, *k) && v@ != k@,
                predecessor matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(t, *k) && t@ != k@ ==> TotalOrder::le(t, v);
        /// Iterative alternative to `next_key`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- BST successor
        fn next_key_iter(&self, k: &K) -> (successor: Option<K>)
            where K: TotalOrderBridge
            requires self.spec_orderedtablesteph_wf()            ensures
                self@.dom().finite(),
                successor matches Some(nk) ==> self@.dom().contains(nk@),
                successor matches Some(v) ==> TotalOrder::le(*k, v) && v@ != k@,
                successor matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(*k, t) && t@ != k@ ==> TotalOrder::le(v, t);
        /// Iterative alternative to `split_key`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- BST key-only split
        fn split_key_iter(&mut self, k: &K) -> (split: (Self, Option<V>, Self))
            where Self: Sized
            requires
                old(self).spec_orderedtablesteph_wf(),
                obeys_view_eq::<K>(),
            ensures
                self@.dom().finite(),
                old(self)@.dom().finite(),
                split.0@.dom().finite(),
                split.2@.dom().finite(),
                split.1 matches Some(v) ==> old(self)@.contains_key(k@) && v@ == old(self)@[k@],
                split.1 matches None ==> !old(self)@.contains_key(k@),
                !split.0@.dom().contains(k@),
                !split.2@.dom().contains(k@),
                split.0@.dom().subset_of(old(self)@.dom()),
                split.2@.dom().subset_of(old(self)@.dom()),
                split.0@.dom().disjoint(split.2@.dom()),
                forall|key| #[trigger] old(self)@.dom().contains(key) ==> split.0@.dom().contains(key) || split.2@.dom().contains(key) || key == k@,
                split.0.spec_orderedtablesteph_wf(),
                split.2.spec_orderedtablesteph_wf();
        /// Iterative alternative to `get_key_range`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- two BST key-only splits
        fn get_key_range_iter(&self, k1: &K, k2: &K) -> (range: Self)
            requires
                self.spec_orderedtablesteph_wf(),
            ensures
                range@.dom().finite(),
                range@.dom().subset_of(self@.dom()),
                forall|key| #[trigger] range@.dom().contains(key) ==> range@[key] == self@[key],
                range.spec_orderedtablesteph_wf();
        /// Iterative alternative to `rank_key`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- BST rank
        fn rank_key_iter(&self, k: &K) -> (rank: usize)
            where K: TotalOrderBridge
            requires
                self.spec_orderedtablesteph_wf(),
                obeys_view_eq::<K>(),
            ensures
                self@.dom().finite(),
                rank <= self@.dom().len(),
                rank as int == self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, *k) && t@ != k@).len();
        /// Iterative alternative to `split_rank_key`.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- BST size-based select + split
        fn split_rank_key_iter(&mut self, i: usize) -> (split: (Self, Self))
            where Self: Sized
            requires
                old(self).spec_orderedtablesteph_wf(),
            ensures
                self@.dom().finite(),
                old(self)@.dom().finite(),
                split.0@.dom().finite(),
                split.1@.dom().finite(),
                split.0@.dom().subset_of(old(self)@.dom()),
                split.1@.dom().subset_of(old(self)@.dom()),
                split.0@.dom().disjoint(split.1@.dom()),
                forall|key| #[trigger] old(self)@.dom().contains(key) ==> split.0@.dom().contains(key) || split.1@.dom().contains(key),
                split.0.spec_orderedtablesteph_wf(),
                split.1.spec_orderedtablesteph_wf();
    }

    // 9. impls

    /// Find by key in a ParamBST of pairs via recursive BST descent.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- BST search by key
    fn bst_find_by_key<K: StT + Ord, V: StT + Ord>(
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
                    assert(tree@ =~= left@.union(right@).insert(root_pair@));
                    assert(tree@.len() == left@.len() + right@.len() + 1);
                    // Propagate key uniqueness and View generation to subtrees (subsets of tree@).
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
                            assert(k.cmp_spec(&root_pair.0) == Equal);
                            assert(k@ == root_pair.0@);
                            assert(tree@.contains(root_pair@));
                            assert(tree@.contains((k@, root_pair.1@)));
                            lemma_pair_in_set_map_contains(tree@, k@, root_pair.1@);
                        }
                        Some(v_clone)
                    },
                    Less => {
                        let result = bst_find_by_key(&left, k);
                        proof {
                            assert(k.cmp_spec(&root_pair.0) == Less);
                            assert(k@ != root_pair.0@);
                            if result is Some {
                                lemma_map_contains_pair_in_set(left@, k@);
                                let vv: V::V = choose|vv: V::V| left@.contains((k@, vv));
                                assert(tree@.contains((k@, vv)));
                                lemma_pair_in_set_map_contains(tree@, k@, vv);
                            } else {
                                if spec_pair_set_to_map(tree@).contains_key(k@) {
                                    lemma_map_contains_pair_in_set(tree@, k@);
                                    let vv: V::V = choose|vv: V::V| tree@.contains((k@, vv));
                                    assert(!left@.contains((k@, vv)));
                                    assert(root_pair@.0 != k@);
                                    assert(right@.contains((k@, vv)));
                                    // right@ is View-generated: (k@, vv) in right@ has a Pair preimage.
                                    let ghost p_wit: Pair<K, V> = choose|p: Pair<K, V>| p@ == (k@, vv);
                                    assert(right@.contains(p_wit@));
                                    // Expose trigger fires: p_wit.cmp_spec(&root_pair) == Greater.
                                    // But p_wit.0@ == k@ and k < root_pair.0, so contradiction.
                                    lemma_cmp_equal_congruent(p_wit.0, *k, root_pair.0);
                                    assert(p_wit.0.cmp_spec(&root_pair.0) == Less);
                                    assert(false);
                                }
                            }
                        }
                        result
                    },
                    Greater => {
                        let result = bst_find_by_key(&right, k);
                        proof {
                            assert(k.cmp_spec(&root_pair.0) == Greater);
                            assert(k@ != root_pair.0@);
                            if result is Some {
                                lemma_map_contains_pair_in_set(right@, k@);
                                let vv: V::V = choose|vv: V::V| right@.contains((k@, vv));
                                assert(tree@.contains((k@, vv)));
                                lemma_pair_in_set_map_contains(tree@, k@, vv);
                            } else {
                                if spec_pair_set_to_map(tree@).contains_key(k@) {
                                    lemma_map_contains_pair_in_set(tree@, k@);
                                    let vv: V::V = choose|vv: V::V| tree@.contains((k@, vv));
                                    assert(!right@.contains((k@, vv)));
                                    assert(root_pair@.0 != k@);
                                    assert(left@.contains((k@, vv)));
                                    let ghost p_wit: Pair<K, V> = choose|p: Pair<K, V>| p@ == (k@, vv);
                                    assert(left@.contains(p_wit@));
                                    lemma_cmp_equal_congruent(p_wit.0, *k, root_pair.0);
                                    assert(p_wit.0.cmp_spec(&root_pair.0) == Greater);
                                    assert(false);
                                }
                            }
                        }
                        result
                    },
                }
            }
        }
    }

    impl<K: StT + Ord, V: StT + Ord> OrderedTableStEphTrait<K, V> for OrderedTableStEph<K, V> {
        open spec fn spec_orderedtablesteph_wf(&self) -> bool {
            self.tree.spec_bstparasteph_wf()
            && spec_key_unique_pairs_set(self.tree@)
            && self.tree@.len() < usize::MAX as nat
            && obeys_feq_fulls::<K, V>()
            && obeys_feq_full::<Pair<K, V>>()
            && vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>()
            && view_ord_consistent::<Pair<K, V>>()
            && spec_pair_key_determines_order::<K, V>()
            && vstd::laws_cmp::obeys_cmp_spec::<K>()
            && view_ord_consistent::<K>()
            && spec_set_pair_view_generated::<K, V>(self.tree@)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size(&self) -> (count: usize)
            ensures count == self@.dom().len(), self@.dom().finite()
        {
            let r = self.tree.size();
            proof {
                lemma_pair_set_to_map_len(self.tree@);
                lemma_pair_set_to_map_dom_finite(self.tree@);
            }
            r
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (empty: Self)
            ensures empty@ == Map::<K::V, V::V>::empty()
        {
            let tree = ParamBST::<Pair<K, V>>::new();
            proof {
                lemma_set_to_map_empty::<K::V, V::V>();
            }
            OrderedTableStEph { tree }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(k: K, v: V) -> (tree: Self)
            ensures tree@ == Map::<K::V, V::V>::empty().insert(k@, v@), tree@.dom().finite(), tree.spec_orderedtablesteph_wf()
        {
            let bst = ParamBST::singleton(Pair(k, v));
            proof {
                // bst@ == Set::empty().insert((k@, v@)).
                let s = Set::<(K::V, V::V)>::empty().insert((k@, v@));
                assert(bst@ =~= s);
                // spec_pair_set_to_map(s) should be Map::empty().insert(k@, v@).
                lemma_set_to_map_empty::<K::V, V::V>();
                lemma_key_unique_empty::<K::V, V::V>();
                lemma_key_unique_insert(Set::<(K::V, V::V)>::empty(), k@, v@);
                lemma_set_to_map_insert(Set::empty(), k@, v@);
                lemma_pair_set_to_map_dom_finite(s);
                // Type axioms for wf: feq via broadcast, rest from requires.
                assert(obeys_feq_full_trigger::<K>());
                assert(obeys_feq_full_trigger::<V>());
                assert(obeys_feq_full_trigger::<Pair<K, V>>());
            }
            OrderedTableStEph { tree: bst }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- delegates to bst_find_by_key
        fn find(&self, k: &K) -> (found: Option<V>)
        {
            bst_find_by_key(&self.tree, k)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- delegates to find
        fn lookup(&self, k: &K) -> (value: Option<V>) {
            self.find(k)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn is_empty(&self) -> (is_empty: bool)
            ensures is_empty == self@.dom().is_empty()
        {
            let r = self.tree.is_empty();
            proof {
                lemma_pair_set_to_map_dom_finite(self.tree@);
                lemma_pair_set_to_map_len(self.tree@);
            }
            r
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- delegates to find
        fn find_iter(&self, k: &K) -> (found: Option<V>)
        {
            // Delegate to recursive find for now.
            self.find(k)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to insert
        fn insert_iter<F: Fn(&V, &V) -> V>(&mut self, k: K, v: V, combine: F)
        {
            // Delegate to recursive insert for now.
            self.insert(k, v, combine)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- find + BST delete/insert
        fn insert<F: Fn(&V, &V) -> V>(&mut self, k: K, v: V, combine: F)
        {
            // Find existing value for this key.
            let existing = self.find(&k);
            let ghost old_tree_view = self.tree@;
            let ghost old_map = self@;
            match existing {
                Some(old_v) => {
                    // Key exists: combine old and new values.
                    let combined = combine(&old_v, &v);
                    proof {
                        // Establish old_tree_view.contains((k@, old_v@)).
                        lemma_map_contains_pair_in_set(old_tree_view, k@);
                        let ghost v_chosen: V::V = choose|vv: V::V| old_tree_view.contains((k@, vv));
                        lemma_pair_in_set_map_contains(old_tree_view, k@, v_chosen);
                        assert(old_tree_view.contains((k@, old_v@)));
                    }
                    // Delete old pair, insert new pair.
                    let k_del = k.clone_plus();
                    let ov_del = old_v.clone_plus();
                    proof {
                        lemma_cloned_view_eq(k, k_del);
                        lemma_cloned_view_eq(old_v, ov_del);
                    }
                    self.tree.delete(&Pair(k_del, ov_del));
                    let ghost mid_tree_view = self.tree@;
                    proof {
                        // mid_tree_view =~= old_tree_view.remove((k@, old_v@)).
                        lemma_set_to_map_remove_pair(old_tree_view, k@, old_v@);
                        lemma_key_unique_remove(old_tree_view, (k@, old_v@));
                        // spec_pair_set_to_map(mid_tree_view) =~= old_map.remove(k@).
                        // k@ is NOT in the map after remove.
                        assert(!spec_pair_set_to_map(mid_tree_view).dom().contains(k@));
                    }
                    let k_clone = k.clone_plus();
                    proof { lemma_cloned_view_eq(k, k_clone); }
                    self.tree.insert(Pair(k_clone, combined));
                    proof {
                        // self.tree@ =~= mid_tree_view.insert((k@, combined@)).
                        lemma_set_to_map_insert(mid_tree_view, k@, combined@);
                        // self@ =~= old_map.remove(k@).insert(k@, combined@).
                        lemma_key_unique_insert(mid_tree_view, k@, combined@);
                        lemma_pair_set_to_map_dom_finite(self.tree@);
                        // Postcondition proofs.
                        let ghost new_map = spec_pair_set_to_map(self.tree@);
                        assert(new_map =~= old_map.remove(k@).insert(k@, combined@));
                        assert(new_map.contains_key(k@));
                        assert(new_map.dom() =~= old_map.dom().insert(k@)) by {
                            assert(old_map.dom().contains(k@));
                            assert(old_map.remove(k@).insert(k@, combined@).dom()
                                =~= old_map.dom().remove(k@).insert(k@));
                            assert(old_map.dom().remove(k@).insert(k@)
                                =~= old_map.dom());
                        };
                        assert forall|key: K::V| key != k@ && #[trigger] old_map.contains_key(key)
                            implies new_map[key] == old_map[key]
                        by {
                            assert(new_map[key] == old_map.remove(k@).insert(k@, combined@)[key]);
                        };
                        // self.tree@.len() < usize::MAX.
                        assert(self.tree@.len() <= old_tree_view.len());
                        assert(self.tree@.len() < usize::MAX as nat);
                    }
                },
                None => {
                    // Key does not exist: insert new pair.
                    self.tree.insert(Pair(k, v));
                    proof {
                        // tree@ = old_tree_view.insert((k@, v@)).
                        lemma_set_to_map_insert(old_tree_view, k@, v@);
                        lemma_pair_set_to_map_dom_finite(self.tree@);
                        lemma_key_unique_insert(old_tree_view, k@, v@);
                        // tree@.len() == old_tree_view.len() + 1 (fresh element).
                        lemma_pair_set_to_map_len(old_tree_view);
                        assert(self.tree@.len() < usize::MAX as nat);
                    }
                },
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to delete
        fn delete_iter(&mut self, k: &K) -> (updated: Option<V>)
        {
            self.delete(k)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- find + BST delete
        fn delete(&mut self, k: &K) -> (updated: Option<V>)
        {
            let existing = self.find(k);
            let ghost old_tree_view = self.tree@;
            let ghost old_map = self@;
            match existing {
                Some(v) => {
                    let v_clone = v.clone_plus();
                    let k_clone = k.clone_plus();
                    proof {
                        lemma_cloned_view_eq(*k, k_clone);
                        lemma_cloned_view_eq(v, v_clone);
                    }
                    self.tree.delete(&Pair(k_clone, v_clone));
                    proof {
                        // After delete: tree@ = old_tree_view.remove((k@, v@)).
                        lemma_set_to_map_remove_pair(old_tree_view, k@, v@);
                        lemma_pair_set_to_map_dom_finite(self.tree@);
                        lemma_key_unique_remove(old_tree_view, (k@, v@));
                    }
                    Some(v)
                },
                None => {
                    proof {
                        // Nothing to delete. self@ == old_map == old_map.remove(k@).
                        assert(self@ =~= old_map.remove(k@));
                        lemma_pair_set_to_map_dom_finite(self.tree@);
                    }
                    None
                },
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- in_order traversal + collect keys
        fn domain(&self) -> (domain: ArraySetStEph<K>)
        {
            let sorted = self.tree.in_order();
            let len = sorted.length();
            let mut domain = ArraySetStEph::empty();
            let mut i: usize = 0;
            proof {
                lemma_pair_set_to_map_dom_finite(self.tree@);
            }
            while i < len
                invariant
                    obeys_feq_clone::<K>(),
                    len as nat == sorted@.len(),
                    sorted@.len() == self.tree@.len(),
                    forall|v: <Pair<K, V> as View>::V| self.tree@.contains(v) <==> #[trigger] sorted@.contains(v),
                    0 <= i <= len,
                    domain.spec_arraysetsteph_wf(),
                    domain@.finite(),
                    forall|kv: K::V| domain@.contains(kv) ==>
                        #[trigger] self@.dom().contains(kv),
                    forall|j: int| 0 <= j < i ==>
                        domain@.contains(#[trigger] sorted@[j].0),
                    self.spec_orderedtablesteph_wf(),
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
                    assert(self.tree@.contains(elem));
                    lemma_pair_in_set_map_contains(self.tree@, elem.0, elem.1);
                }
                domain.insert(key_clone);
                i += 1;
            }
            proof {
                assert(domain@ =~= self@.dom()) by {
                    assert forall|kv: K::V| self@.dom().contains(kv)
                        implies #[trigger] domain@.contains(kv)
                    by {
                        lemma_map_contains_pair_in_set(self.tree@, kv);
                        let v: V::V = choose|v: V::V| self.tree@.contains((kv, v));
                        // (kv, v) is in tree@, so in sorted@.
                        assert(sorted@.contains((kv, v)));
                        // sorted@ is the in-order traversal, so there exists an index j.
                        let j = choose|j: int| 0 <= j < sorted@.len()
                            && (#[trigger] sorted@[j]) == (kv, v);
                        assert(domain@.contains(sorted@[j].0));
                    };
                };
            }
            domain
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- n BST inserts into treap
        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (tabulated: Self)
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
                // Empty set is trivially view-generated.
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
                    forall|k: &K| f.requires((k,)),
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
                    // Prove the new pair is NOT already in tree@ (freshness).
                    assert(!tree@.contains((seq_view[i as int], val@))) by {
                        if tree@.contains((seq_view[i as int], val@)) {
                            let j = choose|j: int| 0 <= j < i as int
                                && (seq_view[i as int], val@).0 == seq_view[j];
                            assert(seq_view[i as int] == seq_view[j]);
                            // no_duplicates: distinct indices have distinct values.
                            assert(false);
                        }
                    };
                }
                tree.insert(Pair(k_clone, val));
                proof {
                    lemma_view_gen_insert::<K, V>(old_tree, Pair(k_clone, val));
                    // tree@ =~= old_tree.insert((seq_view[i], val@)).
                    // The pair was fresh, so set insert increases len by 1.
                    assert(old_tree.finite());
                    assert(!old_tree.contains((seq_view[i as int], val@)));
                    assert(tree@ =~= old_tree.insert((seq_view[i as int], val@)));
                    assert(tree@.len() == i as nat + 1);
                    // i < len and len == keys@.len() < usize::MAX, so i+1 <= len < usize::MAX.
                    assert((i as nat + 1) <= len as nat);
                    assert(tree@.len() < usize::MAX as nat);
                    // Key seq_view[i] was not in old_tree map domain.
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
            let tabulated = OrderedTableStEph { tree };
            proof {
                lemma_pair_set_to_map_dom_finite(tree@);
                // Prove dom =~= keys@.
                assert(tabulated@.dom() =~= keys@) by {
                    assert forall|key: K::V| #[trigger] tabulated@.dom().contains(key)
                        implies keys@.contains(key)
                    by {
                        lemma_map_contains_pair_in_set(tree@, key);
                        let v: V::V = choose|v: V::V| tree@.contains((key, v));
                        let j = choose|j: int| 0 <= j < i as int && (key, v).0 == seq_view[j];
                        assert(seq_view.to_set().contains(seq_view[j]));
                    };
                    assert forall|key: K::V| keys@.contains(key)
                        implies #[trigger] tabulated@.dom().contains(key)
                    by {
                        assert(seq_view.to_set().contains(key));
                        let j = choose|j: int| 0 <= j < seq_view.len()
                            && (#[trigger] seq_view[j]) == key;
                        assert(tree@.contains((seq_view[j], results[j]@)));
                        lemma_pair_in_set_map_contains(tree@, key, results[j]@);
                    };
                };
                // Prove value witness.
                assert forall|key: K::V| #[trigger] tabulated@.contains_key(key)
                    implies (exists|key_arg: K, result: V|
                        key_arg@ == key && f.ensures((&key_arg,), result)
                        && tabulated@[key] == result@)
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
                // Prove spec_orderedtablesteph_wf — type axioms from requires + broadcast.
                assert(obeys_feq_full_trigger::<V>());
            }
            tabulated
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- in_order + n BST inserts
        fn map<F: Fn(&K, &V) -> V>(&self, f: F) -> (mapped: Self)
        {
            // Collect sorted, apply f, rebuild.
            let sorted = self.tree.in_order();
            let len = sorted.length();
            let mut new_tree = ParamBST::<Pair<K, V>>::new();
            let mut i: usize = 0;
            proof {
                lemma_sorted_keys_pairwise_distinct(self.tree@, sorted@);
                lemma_key_unique_empty::<K::V, V::V>();
                // Empty set is trivially view-generated.
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
                    self.spec_orderedtablesteph_wf(),
                    forall|k: &K, v: &V| f.requires((k, v)),
                    obeys_feq_clone::<Pair<K, V>>(),
                    new_tree.spec_bstparasteph_wf(),
                    vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                    view_ord_consistent::<Pair<K, V>>(),
                    sorted@.len() == self.tree@.len(),
                    forall|v: (K::V, V::V)| self.tree@.contains(v) <==> #[trigger] sorted@.contains(v),
                    forall|ii: int, jj: int|
                        0 <= ii < sorted@.len() && 0 <= jj < sorted@.len() && ii != jj
                        ==> (#[trigger] sorted@[ii]).0 != (#[trigger] sorted@[jj]).0,
                    new_tree@.len() == i as nat,
                    new_tree@.len() < usize::MAX as nat,
                    spec_key_unique_pairs_set(new_tree@),
                    // Every pair in new_tree has a key from sorted[0..i).
                    forall|p: (K::V, V::V)| new_tree@.contains(p) ==>
                        exists|j: int| 0 <= j < i as int && p.0 == (#[trigger] sorted@[j]).0,
                    // Completeness: every processed key has a mapped entry in new_tree.
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
                    // Prove key freshness: no pair in old_new_tree has key sorted@[i].0.
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
                    // Completeness maintenance.
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
            let mapped = OrderedTableStEph { tree: new_tree };
            proof {
                lemma_pair_set_to_map_dom_finite(new_tree@);
                lemma_pair_set_to_map_dom_finite(self.tree@);
                // Prove dom equality.
                assert(mapped@.dom() =~= self@.dom()) by {
                    assert forall|key: K::V| #[trigger] mapped@.dom().contains(key)
                        implies self@.dom().contains(key)
                    by {
                        lemma_map_contains_pair_in_set(new_tree@, key);
                        let v: V::V = choose|v: V::V| new_tree@.contains((key, v));
                        let j = choose|j: int| 0 <= j < i as int && (key, v).0 == (#[trigger] sorted@[j]).0;
                        assert(self.tree@.contains(sorted@[j]));
                        lemma_pair_in_set_map_contains(self.tree@, sorted@[j].0, sorted@[j].1);
                    };
                    assert forall|key: K::V| self@.dom().contains(key)
                        implies #[trigger] mapped@.dom().contains(key)
                    by {
                        lemma_map_contains_pair_in_set(self.tree@, key);
                        let v: V::V = choose|v: V::V| self.tree@.contains((key, v));
                        assert(sorted@.contains((key, v)));
                        let j = choose|j: int| 0 <= j < sorted@.len() && sorted@[j] == (key, v);
                        // By completeness invariant (j < sorted@.len() == i):
                        assert(spec_pair_set_to_map(new_tree@).dom().contains(sorted@[j].0));
                        assert(sorted@[j].0 == key);
                    };
                };
                // Type axioms flow from self.spec_orderedtablesteph_wf().
                assert(spec_pair_key_determines_order::<K, V>());
                assert(vstd::laws_cmp::obeys_cmp_spec::<K>());
                assert(view_ord_consistent::<K>());
                assert(obeys_feq_fulls::<K, V>());
            }
            mapped
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
            let filtered_tree = self.tree.filter(pair_pred, Ghost(pair_spec_pred));
            let filtered = OrderedTableStEph { tree: filtered_tree };
            proof {
                lemma_pair_set_to_map_dom_finite(filtered_tree@);
                lemma_pair_set_to_map_dom_finite(self.tree@);
                lemma_key_unique_subset(self.tree@, filtered_tree@);
                // filtered_tree@ ⊆ self.tree@.
                // Prove postconditions.
                assert(filtered@.dom().subset_of(self@.dom())) by {
                    assert forall|k: K::V| filtered@.dom().contains(k)
                        implies #[trigger] self@.dom().contains(k)
                    by {
                        lemma_map_contains_pair_in_set(filtered_tree@, k);
                        let v: V::V = choose|v: V::V| filtered_tree@.contains((k, v));
                        assert(self.tree@.contains((k, v)));
                        lemma_pair_in_set_map_contains(self.tree@, k, v);
                    };
                };
                assert forall|k: K::V| #[trigger] filtered@.contains_key(k)
                    implies filtered@[k] == self@[k]
                by {
                    lemma_map_contains_pair_in_set(filtered_tree@, k);
                    let v: V::V = choose|v: V::V| filtered_tree@.contains((k, v));
                    assert(self.tree@.contains((k, v)));
                    lemma_pair_in_set_map_contains(self.tree@, k, v);
                    lemma_pair_in_set_map_contains(filtered_tree@, k, v);
                };
                assert forall|k: K::V| self@.dom().contains(k) && spec_pred(k, self@[k])
                    implies #[trigger] filtered@.dom().contains(k)
                by {
                    lemma_map_contains_pair_in_set(self.tree@, k);
                    let v: V::V = choose|v: V::V| self.tree@.contains((k, v));
                    lemma_pair_in_set_map_contains(self.tree@, k, v);
                    // v == self@[k], so spec_pred(k, v) holds.
                    assert(pair_spec_pred((k, v)));
                    assert(filtered_tree@.contains((k, v)));
                    lemma_pair_in_set_map_contains(filtered_tree@, k, v);
                };
                // Prove filtered_tree@.len() < usize::MAX (subset of self.tree@).
                vstd::set_lib::lemma_len_subset(filtered_tree@, self.tree@);
                // Type axioms flow from self.spec_orderedtablesteph_wf().
                assert(spec_pair_key_determines_order::<K, V>());
                assert(vstd::laws_cmp::obeys_cmp_spec::<K>());
                assert(view_ord_consistent::<K>());
                assert(obeys_feq_fulls::<K, V>());
            }
            filtered
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- in_order traversal + fold
        fn reduce<R, F: Fn(R, &K, &V) -> R>(&self, init: R, f: F) -> (reduced: R)
            ensures self@.dom().finite()
        {
            let sorted = self.tree.in_order();
            let len = sorted.length();
            let mut reduced = init;
            let mut i: usize = 0;
            while i < len
                invariant
                    i <= len,
                    len as nat == sorted@.len(),
                    forall|r: R, k: &K, v: &V| f.requires((r, k, v)),
                decreases len - i,
            {
                let pair = sorted.nth(i);
                reduced = f(reduced, &pair.0, &pair.1);
                i = i + 1;
            }
            proof { lemma_pair_set_to_map_dom_finite(self.tree@); }
            reduced
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- find per element
        fn intersection<F: Fn(&V, &V) -> V>(&mut self, other: &Self, f: F)
        {
            // Collect both tables, iterate self's entries, keep those in other.
            let ghost old_tree = self.tree@;
            let ghost old_map = self@;
            let ghost other_map = other@;
            let sorted = self.tree.in_order();
            let len = sorted.length();
            let mut new_tree = ParamBST::<Pair<K, V>>::new();
            let ghost mut result_v1: Seq<V> = Seq::empty();
            let ghost mut result_v2: Seq<V> = Seq::empty();
            let ghost mut result_r: Seq<V> = Seq::empty();
            let mut i: usize = 0;
            proof {
                assert(obeys_feq_full_trigger::<V>());
                assert(obeys_feq_full_trigger::<K>());
                lemma_pair_set_to_map_dom_finite(old_tree);
                lemma_sorted_keys_pairwise_distinct(old_tree, sorted@);
                lemma_key_unique_empty::<K::V, V::V>();
                // Empty set is trivially view-generated.
                assert(spec_set_pair_view_generated::<K, V>(new_tree@)) by {
                    assert forall|elem: (K::V, V::V)| new_tree@.contains(elem)
                        implies exists|p: Pair<K, V>| (#[trigger] p@) == elem by {
                        assert(false);
                    };
                };
            }
            while i < len
                invariant
                    self.tree@ == old_tree,
                    old(self).tree@ == old_tree,
                    old(self).spec_orderedtablesteph_wf(),
                    other.spec_orderedtablesteph_wf(),
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<V>(),
                    obeys_feq_full::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
                    forall|v1: &V, v2: &V| f.requires((v1, v2)),
                    old_map == spec_pair_set_to_map(old_tree),
                    other_map == other@,
                    len as nat == sorted@.len(),
                    sorted@.len() == old_tree.len(),
                    0 <= i <= len,
                    new_tree.spec_bstparasteph_wf(),
                    vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                    view_ord_consistent::<Pair<K, V>>(),
                    new_tree@.len() <= i as nat,
                    new_tree@.len() < usize::MAX as nat,
                    spec_key_unique_pairs_set(new_tree@),
                    spec_key_unique_pairs_set(old_tree),
                    result_v1.len() == new_tree@.len() as int,
                    result_v2.len() == new_tree@.len() as int,
                    result_r.len() == new_tree@.len() as int,
                    sorted@.len() == old_tree.len(),
                    forall|v: <Pair<K, V> as View>::V| old_tree.contains(v) <==> #[trigger] sorted@.contains(v),
                    forall|ii: int, jj: int|
                        0 <= ii < sorted@.len() && 0 <= jj < sorted@.len() && ii != jj
                        ==> (#[trigger] sorted@[ii]).0 != (#[trigger] sorted@[jj]).0,
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        exists|j: int| 0 <= j < i as int && p.0 == (#[trigger] sorted@[j]).0,
                    // Entries in new_tree have keys in other's domain.
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        other_map.dom().contains(p.0),
                    // Entries in new_tree have keys from old_tree.
                    forall|v: <Pair<K, V> as View>::V| old_tree.contains(v) <==> #[trigger] sorted@.contains(v),
                    // Completeness: processed keys in other are in new_tree.
                    forall|j2: int| 0 <= j2 < i as int && other_map.dom().contains(sorted@[j2].0)
                        ==> #[trigger] spec_pair_set_to_map(new_tree@).dom().contains(sorted@[j2].0),
                    // Value tracking: each entry's value is f(old_val, other_val).
                    forall|kv: K::V| #[trigger] spec_pair_set_to_map(new_tree@).dom().contains(kv) ==>
                        (exists|v1: V, v2: V, r: V|
                            v1@ == old_map[kv] && v2@ == other_map[kv]
                            && f.ensures((&v1, &v2), r)
                            && spec_pair_set_to_map(new_tree@)[kv] == r@),
                    spec_set_pair_view_generated::<K, V>(new_tree@),
                decreases len - i,
            {
                let pair = sorted.nth(i);
                proof { reveal(obeys_view_eq); }
                let other_find = other.find(&pair.0);
                match other_find {
                    Some(other_v) => {
                        let combined = f(&pair.1, &other_v);
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
                            // pair values link.
                            assert(sorted@.contains(sorted@[i as int])) by { assert(sorted@[i as int] == sorted@[i as int]); };
                            assert(old_tree.contains(sorted@[i as int]));
                            lemma_pair_in_set_map_contains(old_tree, sorted@[i as int].0, sorted@[i as int].1);
                        }
                        new_tree.insert(Pair(key_clone, combined));
                        proof {
                            lemma_view_gen_insert::<K, V>(old_new_tree_view, Pair(key_clone, combined));
                            assert(new_tree@.len() <= i as nat + 1);
                            assert(new_tree@.len() < usize::MAX as nat);
                            lemma_key_unique_insert(old_new_tree_view, sorted@[i as int].0, combined@);
                            result_v1 = result_v1.push(pair.1);
                            result_v2 = result_v2.push(other_v);
                            result_r = result_r.push(combined);
                            // Completeness maintenance for new key.
                            lemma_pair_in_set_map_contains(new_tree@, sorted@[i as int].0, combined@);
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
                            assert(spec_pair_set_to_map(new_tree@)[sorted@[i as int].0] == combined@) by {
                                lemma_pair_in_set_map_contains(new_tree@, sorted@[i as int].0, combined@);
                            };
                            assert(pair.1@ == old_map[sorted@[i as int].0]);
                            assert(other_v@ == other_map[sorted@[i as int].0]);
                            assert(f.ensures((&pair.1, &other_v), combined));
                            // Value tracking for old keys.
                            assert forall|kv: K::V| #[trigger] spec_pair_set_to_map(new_tree@).dom().contains(kv)
                                implies (exists|v1: V, v2: V, r: V|
                                    v1@ == old_map[kv] && v2@ == other_map[kv]
                                    && f.ensures((&v1, &v2), r)
                                    && spec_pair_set_to_map(new_tree@)[kv] == r@)
                            by {
                                if kv == sorted@[i as int].0 {
                                    // New entry.
                                    assert(pair.1@ == old_map[kv]);
                                    assert(other_v@ == other_map[kv]);
                                    assert(f.ensures((&pair.1, &other_v), combined));
                                    assert(spec_pair_set_to_map(new_tree@)[kv] == combined@);
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
            self.tree = new_tree;
            proof {
                lemma_pair_set_to_map_dom_finite(new_tree@);
                lemma_pair_set_to_map_dom_finite(old_tree);
                // Domain proof: self@.dom() =~= old_map.dom().intersect(other_map.dom()).
                assert(self@.dom() =~= old_map.dom().intersect(other_map.dom())) by {
                    // Forward: key in self → key in old AND other.
                    assert forall|k: K::V| #[trigger] self@.dom().contains(k)
                        implies old_map.dom().contains(k) && other_map.dom().contains(k)
                    by {
                        lemma_map_contains_pair_in_set(new_tree@, k);
                        let v: V::V = choose|v: V::V| new_tree@.contains((k, v));
                        let j = choose|j: int| 0 <= j < i as int && (k, v).0 == (#[trigger] sorted@[j]).0;
                        assert(sorted@.contains(sorted@[j])) by { assert(sorted@[j] == sorted@[j]); };
                        assert(old_tree.contains(sorted@[j]));
                        lemma_pair_in_set_map_contains(old_tree, sorted@[j].0, sorted@[j].1);
                    };
                    // Backward: key in old AND other → key in self.
                    assert forall|k: K::V|
                        old_map.dom().contains(k) && other_map.dom().contains(k)
                        implies #[trigger] self@.dom().contains(k)
                    by {
                        lemma_map_contains_pair_in_set(old_tree, k);
                        let v: V::V = choose|v: V::V| old_tree.contains((k, v));
                        assert(sorted@.contains((k, v)));
                        let j = choose|j: int| 0 <= j < sorted@.len() && sorted@[j] == (k, v);
                        assert(spec_pair_set_to_map(new_tree@).dom().contains(sorted@[j].0));
                        assert(sorted@[j].0 == k);
                    };
                };
                // Value proof.
                assert forall|k: K::V| #[trigger] self@.contains_key(k) implies
                    (exists|v1: V, v2: V, r: V|
                        v1@ == old_map[k] && v2@ == other_map[k]
                        && f.ensures((&v1, &v2), r)
                        && self@[k] == r@)
                by {
                    // From value tracking invariant.
                    assert(spec_pair_set_to_map(new_tree@).dom().contains(k));
                };
                // WF proofs: new_tree size bounded by old_tree size.
                assert(new_tree@.len() < usize::MAX as nat);
                // Type axioms flow from old(self).spec_orderedtablesteph_wf().
                assert(spec_pair_key_determines_order::<K, V>());
                assert(vstd::laws_cmp::obeys_cmp_spec::<K>());
                assert(view_ord_consistent::<K>());
                assert(obeys_feq_fulls::<K, V>());
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- insert per element
        fn union<F: Fn(&V, &V) -> V>(&mut self, other: &Self, f: F)
        {
            let ghost old_tree = self.tree@;
            let ghost old_map = self@;
            let ghost other_map = other@;
            proof {
                assert(obeys_feq_full_trigger::<V>());
                assert(obeys_feq_full_trigger::<K>());
                assert(obeys_feq_full_trigger::<Pair<K, V>>());
                lemma_pair_set_to_map_len(old_tree);
                lemma_pair_set_to_map_len(other.tree@);
            }
            // Phase 1: iterate self entries, merge with other where overlapping.
            let self_sorted = self.tree.in_order();
            let self_len = self_sorted.length();
            let mut new_tree = ParamBST::<Pair<K, V>>::new();
            let mut i: usize = 0;
            proof {
                lemma_sorted_keys_pairwise_distinct(old_tree, self_sorted@);
                lemma_key_unique_empty::<K::V, V::V>();
                // Empty set is trivially view-generated.
                assert(spec_set_pair_view_generated::<K, V>(new_tree@)) by {
                    assert forall|elem: (K::V, V::V)| new_tree@.contains(elem)
                        implies exists|p: Pair<K, V>| (#[trigger] p@) == elem by {
                        assert(false);
                    };
                };
            }
            while i < self_len
                invariant
                    self.tree@ == old_tree,
                    old(self).spec_orderedtablesteph_wf(),
                    other.spec_orderedtablesteph_wf(),
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<V>(),
                    obeys_feq_full::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
                    forall|v1: &V, v2: &V| f.requires((v1, v2)),
                    self_len as nat == self_sorted@.len(),
                    self_sorted@.len() == old_tree.len(),
                    forall|v: <Pair<K, V> as View>::V| old_tree.contains(v) <==> #[trigger] self_sorted@.contains(v),
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
                    spec_key_unique_pairs_set(old_tree),
                    old_map == spec_pair_set_to_map(old_tree),
                    other_map == other@,
                    // Phase 1 completeness.
                    forall|j2: int| 0 <= j2 < i as int
                        ==> #[trigger] spec_pair_set_to_map(new_tree@).dom().contains(self_sorted@[j2].0),
                    // Phase 1 value tracking (unified per-pair).
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        old_map.dom().contains(p.0) &&
                        ((!other_map.dom().contains(p.0) && old_tree.contains(p))
                        || (other_map.dom().contains(p.0) && exists|v1: V, v2: V, r: V|
                            v1@ == old_map[p.0] && v2@ == other_map[p.0]
                            && f.ensures((&v1, &v2), r) && p.1 == r@)),
                    spec_set_pair_view_generated::<K, V>(new_tree@),
                decreases self_len - i,
            {
                let pair = self_sorted.nth(i);
                proof { reveal(obeys_view_eq); }
                let other_find = other.find(&pair.0);
                let ghost old_new_tree_view = new_tree@;
                proof {
                    // Freshness: self_sorted@[i].0 not in new_tree.
                    assert(!spec_pair_set_to_map(old_new_tree_view).dom().contains(self_sorted@[i as int].0)) by {
                        if spec_pair_set_to_map(old_new_tree_view).dom().contains(self_sorted@[i as int].0) {
                            lemma_map_contains_pair_in_set(old_new_tree_view, self_sorted@[i as int].0);
                            let vv: V::V = choose|vv: V::V| old_new_tree_view.contains((self_sorted@[i as int].0, vv));
                            let jj = choose|jj: int| 0 <= jj < i as int && (self_sorted@[i as int].0, vv).0 == (#[trigger] self_sorted@[jj]).0;
                            assert(false);
                        }
                    };
                    // Link sorted entry to old_tree.
                    assert(self_sorted@.contains(self_sorted@[i as int])) by { assert(self_sorted@[i as int] == self_sorted@[i as int]); };
                    assert(old_tree.contains(self_sorted@[i as int]));
                    lemma_pair_in_set_map_contains(old_tree, self_sorted@[i as int].0, self_sorted@[i as int].1);
                }
                match other_find {
                    Some(ov) => {
                        let combined = f(&pair.1, &ov);
                        let key_clone = pair.0.clone_plus();
                        proof { lemma_cloned_view_eq(pair.0, key_clone); }
                        new_tree.insert(Pair(key_clone, combined));
                        proof {
                            lemma_view_gen_insert::<K, V>(old_new_tree_view, Pair(key_clone, combined));
                            assert(new_tree@.len() == i as nat + 1);
                            assert(new_tree@.len() < usize::MAX as nat);
                            lemma_key_unique_insert(old_new_tree_view, self_sorted@[i as int].0, combined@);
                            // Completeness maintenance.
                            lemma_pair_in_set_map_contains(new_tree@, self_sorted@[i as int].0, combined@);
                            assert forall|j2: int| 0 <= j2 < i as int
                                implies #[trigger] spec_pair_set_to_map(new_tree@).dom().contains(self_sorted@[j2].0)
                            by {
                                lemma_map_contains_pair_in_set(old_new_tree_view, self_sorted@[j2].0);
                                let w: V::V = choose|w: V::V| old_new_tree_view.contains((self_sorted@[j2].0, w));
                                assert(new_tree@.contains((self_sorted@[j2].0, w)));
                                lemma_pair_in_set_map_contains(new_tree@, self_sorted@[j2].0, w);
                            };
                            // Value tracking maintenance: new pair is combined.
                            assert(old_map.dom().contains(self_sorted@[i as int].0)) by {
                                lemma_pair_in_set_map_contains(old_tree, self_sorted@[i as int].0, self_sorted@[i as int].1);
                            };
                            assert forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) implies
                                old_map.dom().contains(p.0) &&
                                ((!other_map.dom().contains(p.0) && old_tree.contains(p))
                                || (other_map.dom().contains(p.0) && exists|v1: V, v2: V, r: V|
                                    v1@ == old_map[p.0] && v2@ == other_map[p.0]
                                    && f.ensures((&v1, &v2), r) && p.1 == r@))
                            by {
                                if old_new_tree_view.contains(p) {
                                    // Existing pair — invariant held before insert.
                                } else {
                                    // New pair: p == (self_sorted@[i].0, combined@).
                                    assert(p.0 == self_sorted@[i as int].0);
                                    assert(p.1 == combined@);
                                    assert(other_map.dom().contains(p.0));
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
                            assert(new_tree@.len() == i as nat + 1);
                            assert(new_tree@.len() < usize::MAX as nat);
                            lemma_key_unique_insert(old_new_tree_view, self_sorted@[i as int].0, self_sorted@[i as int].1);
                            // Completeness maintenance.
                            lemma_pair_in_set_map_contains(new_tree@, self_sorted@[i as int].0, self_sorted@[i as int].1);
                            assert forall|j2: int| 0 <= j2 < i as int
                                implies #[trigger] spec_pair_set_to_map(new_tree@).dom().contains(self_sorted@[j2].0)
                            by {
                                lemma_map_contains_pair_in_set(old_new_tree_view, self_sorted@[j2].0);
                                let w: V::V = choose|w: V::V| old_new_tree_view.contains((self_sorted@[j2].0, w));
                                assert(new_tree@.contains((self_sorted@[j2].0, w)));
                                lemma_pair_in_set_map_contains(new_tree@, self_sorted@[j2].0, w);
                            };
                            // Value tracking maintenance: new pair is self-only.
                            assert(old_map.dom().contains(self_sorted@[i as int].0)) by {
                                lemma_pair_in_set_map_contains(old_tree, self_sorted@[i as int].0, self_sorted@[i as int].1);
                            };
                            assert forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) implies
                                old_map.dom().contains(p.0) &&
                                ((!other_map.dom().contains(p.0) && old_tree.contains(p))
                                || (other_map.dom().contains(p.0) && exists|v1: V, v2: V, r: V|
                                    v1@ == old_map[p.0] && v2@ == other_map[p.0]
                                    && f.ensures((&v1, &v2), r) && p.1 == r@))
                            by {
                                if old_new_tree_view.contains(p) {
                                    // Existing pair — invariant held before insert.
                                } else {
                                    // New pair: p == self_sorted@[i] (cloned).
                                    assert(p.0 == self_sorted@[i as int].0);
                                    assert(p.1 == self_sorted@[i as int].1);
                                    assert(!other_map.dom().contains(p.0));
                                    assert(old_tree.contains(p));
                                }
                            };
                        }
                    },
                }
                i += 1;
            }
            // Phase 2: iterate other entries, add those not in self.
            let other_sorted = other.tree.in_order();
            let other_len = other_sorted.length();
            let mut j: usize = 0;
            proof {
                lemma_sorted_keys_pairwise_distinct(other.tree@, other_sorted@);
                // Bridge: old keys preserved (Phase 1 completeness → per-key form).
                assert forall|kv: K::V| #[trigger] old_map.dom().contains(kv)
                    implies spec_pair_set_to_map(new_tree@).dom().contains(kv)
                by {
                    lemma_map_contains_pair_in_set(old_tree, kv);
                    let vv: V::V = choose|vv: V::V| old_tree.contains((kv, vv));
                    assert(self_sorted@.contains((kv, vv)));
                    let jx: int = choose|jx: int| 0 <= jx < self_sorted@.len() as int && self_sorted@[jx] == (kv, vv);
                    assert(spec_pair_set_to_map(new_tree@).dom().contains(self_sorted@[jx].0));
                };
                lemma_pair_set_to_map_len(old_tree);
                lemma_pair_set_to_map_len(other.tree@);
            }
            while j < other_len
                invariant
                    self.tree@ == old_tree,
                    old(self).spec_orderedtablesteph_wf(),
                    other.spec_orderedtablesteph_wf(),
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
                    obeys_feq_full::<V>(),
                    obeys_feq_full::<K>(),
                    other_map == other@,
                    old_map == spec_pair_set_to_map(old_tree),
                    other_len as nat == other_sorted@.len(),
                    other_sorted@.len() == other.tree@.len(),
                    self_sorted@.len() == old_tree.len(),
                    forall|v: <Pair<K, V> as View>::V| other.tree@.contains(v) <==> #[trigger] other_sorted@.contains(v),
                    forall|v: <Pair<K, V> as View>::V| old_tree.contains(v) <==> #[trigger] self_sorted@.contains(v),
                    forall|ii: int, jj: int|
                        0 <= ii < other_sorted@.len() && 0 <= jj < other_sorted@.len() && ii != jj
                        ==> (#[trigger] other_sorted@[ii]).0 != (#[trigger] other_sorted@[jj]).0,
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        old_map.dom().contains(p.0) ||
                        (exists|j2: int| 0 <= j2 < j as int && p.0 == (#[trigger] other_sorted@[j2]).0),
                    0 <= j <= other_len,
                    new_tree.spec_bstparasteph_wf(),
                    vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                    view_ord_consistent::<Pair<K, V>>(),
                    new_tree@.len() <= self_sorted@.len() + j as nat,
                    self_sorted@.len() + other_sorted@.len() < usize::MAX as nat,
                    spec_key_unique_pairs_set(new_tree@),
                    spec_key_unique_pairs_set(old_tree),
                    // Old keys preserved.
                    forall|kv: K::V| #[trigger] old_map.dom().contains(kv)
                        ==> spec_pair_set_to_map(new_tree@).dom().contains(kv),
                    // Other completeness.
                    forall|j2: int| 0 <= j2 < j as int && !old_map.dom().contains(other_sorted@[j2].0)
                        ==> #[trigger] spec_pair_set_to_map(new_tree@).dom().contains(other_sorted@[j2].0),
                    // Phase 2 value tracking (3-way).
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        (old_map.dom().contains(p.0) &&
                            ((!other_map.dom().contains(p.0) && old_tree.contains(p))
                            || (other_map.dom().contains(p.0) && exists|v1: V, v2: V, r: V|
                                v1@ == old_map[p.0] && v2@ == other_map[p.0]
                                && f.ensures((&v1, &v2), r) && p.1 == r@)))
                        || (!old_map.dom().contains(p.0) && other.tree@.contains(p)),
                    spec_set_pair_view_generated::<K, V>(new_tree@),
                decreases other_len - j,
            {
                let pair = other_sorted.nth(j);
                proof { reveal(obeys_view_eq); }
                let in_self = self.find(&pair.0);
                match in_self {
                    None => {
                        // find returned None → !old_map.dom().contains(other_sorted@[j].0).
                        let cloned = pair.clone_plus();
                        let ghost old_new_tree_view = new_tree@;
                        proof {
                            lemma_cloned_view_eq(*pair, cloned);
                            // Freshness: other_sorted@[j].0 not already in new_tree.
                            assert(!spec_pair_set_to_map(old_new_tree_view).dom().contains(other_sorted@[j as int].0)) by {
                                if spec_pair_set_to_map(old_new_tree_view).dom().contains(other_sorted@[j as int].0) {
                                    lemma_map_contains_pair_in_set(old_new_tree_view, other_sorted@[j as int].0);
                                    let vv: V::V = choose|vv: V::V| old_new_tree_view.contains((other_sorted@[j as int].0, vv));
                                    if old_map.dom().contains(other_sorted@[j as int].0) {
                                        assert(false);
                                    } else {
                                        let j2 = choose|j2: int| 0 <= j2 < j as int && (other_sorted@[j as int].0, vv).0 == (#[trigger] other_sorted@[j2]).0;
                                        assert(false);
                                    }
                                }
                            };
                        }
                        new_tree.insert(cloned);
                        proof {
                            lemma_view_gen_insert::<K, V>(old_new_tree_view, cloned);
                            assert(new_tree@.len() <= self_sorted@.len() + j as nat + 1);
                            lemma_key_unique_insert(old_new_tree_view, other_sorted@[j as int].0, other_sorted@[j as int].1);
                            // Old keys preserved maintenance.
                            assert forall|kv: K::V| #[trigger] old_map.dom().contains(kv)
                                implies spec_pair_set_to_map(new_tree@).dom().contains(kv)
                            by {
                                lemma_map_contains_pair_in_set(old_new_tree_view, kv);
                                let w: V::V = choose|w: V::V| old_new_tree_view.contains((kv, w));
                                assert(new_tree@.contains((kv, w)));
                                lemma_pair_in_set_map_contains(new_tree@, kv, w);
                            };
                            // Other completeness maintenance.
                            assert(other_sorted@.contains(other_sorted@[j as int])) by {
                                assert(other_sorted@[j as int] == other_sorted@[j as int]);
                            };
                            assert(other.tree@.contains(other_sorted@[j as int]));
                            lemma_pair_in_set_map_contains(new_tree@, other_sorted@[j as int].0, other_sorted@[j as int].1);
                            assert forall|j2: int| 0 <= j2 < j as int + 1
                                && !old_map.dom().contains(other_sorted@[j2].0)
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
                                (old_map.dom().contains(p.0) &&
                                    ((!other_map.dom().contains(p.0) && old_tree.contains(p))
                                    || (other_map.dom().contains(p.0) && exists|v1: V, v2: V, r: V|
                                        v1@ == old_map[p.0] && v2@ == other_map[p.0]
                                        && f.ensures((&v1, &v2), r) && p.1 == r@)))
                                || (!old_map.dom().contains(p.0) && other.tree@.contains(p))
                            by {
                                if old_new_tree_view.contains(p) {
                                } else {
                                    assert(p.0 == other_sorted@[j as int].0);
                                    assert(p.1 == other_sorted@[j as int].1);
                                    assert(!old_map.dom().contains(p.0));
                                    assert(other.tree@.contains(p));
                                }
                            };
                        }
                    },
                    Some(_) => {},
                }
                j += 1;
            }
            self.tree = new_tree;
            proof {
                lemma_pair_set_to_map_dom_finite(new_tree@);
                lemma_pair_set_to_map_dom_finite(old_tree);
                lemma_pair_set_to_map_dom_finite(other.tree@);
                // 1. Domain: self@.dom() =~= old_map.dom().union(other_map.dom()).
                assert(self@.dom() =~= old_map.dom().union(other_map.dom())) by {
                    assert forall|kv: K::V| self@.dom().contains(kv)
                        implies #[trigger] old_map.dom().union(other_map.dom()).contains(kv)
                    by {
                        lemma_map_contains_pair_in_set(self.tree@, kv);
                        let vv: V::V = choose|vv: V::V| self.tree@.contains((kv, vv));
                        if !old_map.dom().contains(kv) {
                            assert(other.tree@.contains((kv, vv)));
                            lemma_pair_in_set_map_contains(other.tree@, kv, vv);
                        }
                    };
                    assert forall|kv: K::V| #[trigger] old_map.dom().union(other_map.dom()).contains(kv)
                        implies self@.dom().contains(kv)
                    by {
                        if old_map.dom().contains(kv) {
                        } else {
                            assert(other_map.dom().contains(kv));
                            lemma_map_contains_pair_in_set(other.tree@, kv);
                            let vv: V::V = choose|vv: V::V| other.tree@.contains((kv, vv));
                            assert(other_sorted@.contains((kv, vv)));
                            let jx: int = choose|jx: int| 0 <= jx < other_sorted@.len() as int && other_sorted@[jx] == (kv, vv);
                            assert(!old_map.dom().contains(other_sorted@[jx].0));
                            assert(spec_pair_set_to_map(new_tree@).dom().contains(other_sorted@[jx].0));
                        }
                    };
                };
                // 2. Self-only values.
                assert forall|k: K::V| #[trigger] old_map.contains_key(k) && !other_map.contains_key(k)
                    implies self@[k] == old_map[k]
                by {
                    lemma_map_contains_pair_in_set(self.tree@, k);
                    let vv: V::V = choose|vv: V::V| self.tree@.contains((k, vv));
                    assert(old_tree.contains((k, vv)));
                    lemma_pair_in_set_map_contains(self.tree@, k, vv);
                    lemma_pair_in_set_map_contains(old_tree, k, vv);
                };
                // 3. Other-only values.
                assert forall|k: K::V| #[trigger] other_map.contains_key(k) && !old_map.contains_key(k)
                    implies self@[k] == other_map[k]
                by {
                    lemma_map_contains_pair_in_set(self.tree@, k);
                    let vv: V::V = choose|vv: V::V| self.tree@.contains((k, vv));
                    assert(other.tree@.contains((k, vv)));
                    lemma_pair_in_set_map_contains(self.tree@, k, vv);
                    lemma_pair_in_set_map_contains(other.tree@, k, vv);
                };
                // 4. Both values.
                assert forall|k: K::V| #[trigger] old_map.contains_key(k) && other_map.contains_key(k) implies
                    (exists|v1: V, v2: V, r: V|
                        v1@ == old_map[k] && v2@ == other_map[k]
                        && f.ensures((&v1, &v2), r)
                        && self@[k] == r@)
                by {
                    lemma_map_contains_pair_in_set(self.tree@, k);
                    let vv: V::V = choose|vv: V::V| self.tree@.contains((k, vv));
                    lemma_pair_in_set_map_contains(self.tree@, k, vv);
                };
                // 5. wf.
                assert(self.tree@.len() < usize::MAX as nat);
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- delete per element
        fn difference(&mut self, other: &Self)
        {
            let ghost old_tree = self.tree@;
            let ghost old_map = self@;
            let ghost other_map = other@;
            let sorted = self.tree.in_order();
            let len = sorted.length();
            let mut new_tree = ParamBST::<Pair<K, V>>::new();
            let mut i: usize = 0;
            proof {
                lemma_sorted_keys_pairwise_distinct(old_tree, sorted@);
                lemma_key_unique_empty::<K::V, V::V>();
            }
            while i < len
                invariant
                    self.tree@ == old_tree,
                    old(self).spec_orderedtablesteph_wf(),
                    other.spec_orderedtablesteph_wf(),
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
                    old_map == spec_pair_set_to_map(old_tree),
                    other_map == other@,
                    len as nat == sorted@.len(),
                    sorted@.len() == old_tree.len(),
                    forall|v: <Pair<K, V> as View>::V| old_tree.contains(v) <==> #[trigger] sorted@.contains(v),
                    forall|ii: int, jj: int|
                        0 <= ii < sorted@.len() && 0 <= jj < sorted@.len() && ii != jj
                        ==> (#[trigger] sorted@[ii]).0 != (#[trigger] sorted@[jj]).0,
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        exists|j: int| 0 <= j < i as int && p.0 == (#[trigger] sorted@[j]).0,
                    // All entries in new_tree are from old_tree (values preserved).
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==> old_tree.contains(p),
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
                    spec_key_unique_pairs_set(old_tree),
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
                            // Prove sorted@[i] is in old_tree for the subset invariant.
                            assert(sorted@.contains(sorted@[i as int])) by { assert(sorted@[i as int] == sorted@[i as int]); };
                            assert(old_tree.contains(sorted@[i as int]));
                            // Maintain completeness: new_tree now includes sorted@[i].
                            assert(new_tree@.contains(sorted@[i as int]));
                        }
                    },
                    Some(_) => {},
                }
                i += 1;
            }
            self.tree = new_tree;
            proof {
                lemma_pair_set_to_map_dom_finite(new_tree@);
                lemma_pair_set_to_map_dom_finite(old_tree);
                // Prove: self@.dom() =~= old_map.dom().difference(other_map.dom())
                assert(self@.dom() =~= old_map.dom().difference(other_map.dom())) by {
                    // Forward: k in self dom ==> k in old dom and k not in other dom.
                    assert forall|k: K::V| #[trigger] self@.dom().contains(k)
                        implies old_map.dom().contains(k) && !other_map.dom().contains(k)
                    by {
                        lemma_map_contains_pair_in_set(new_tree@, k);
                        let v: V::V = choose|v: V::V| new_tree@.contains((k, v));
                        assert(old_tree.contains((k, v)));
                        lemma_pair_in_set_map_contains(old_tree, k, v);
                        assert(!other_map.dom().contains(k));
                    };
                    // Backward: k in old dom and k not in other dom ==> k in self dom.
                    assert forall|k: K::V|
                        old_map.dom().contains(k) && !other_map.dom().contains(k)
                        implies #[trigger] self@.dom().contains(k)
                    by {
                        lemma_map_contains_pair_in_set(old_tree, k);
                        let v: V::V = choose|v: V::V| old_tree.contains((k, v));
                        assert(sorted@.contains((k, v)));
                        let j = choose|j: int| 0 <= j < sorted@.len() && sorted@[j] == (k, v);
                        assert(new_tree@.contains(sorted@[j]));
                        lemma_pair_in_set_map_contains(new_tree@, k, v);
                    };
                };
                // Prove: values preserved.
                assert forall|k: K::V| #[trigger] self@.contains_key(k)
                    implies self@[k] == old_map[k]
                by {
                    lemma_map_contains_pair_in_set(new_tree@, k);
                    let v: V::V = choose|v: V::V| new_tree@.contains((k, v));
                    assert(old_tree.contains((k, v)));
                    lemma_pair_in_set_map_contains(new_tree@, k, v);
                    lemma_pair_in_set_map_contains(old_tree, k, v);
                };
                // Type axioms flow from old(self).spec_orderedtablesteph_wf().
                assert(spec_pair_key_determines_order::<K, V>());
                assert(vstd::laws_cmp::obeys_cmp_spec::<K>());
                assert(view_ord_consistent::<K>());
                assert(obeys_feq_fulls::<K, V>());
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- filter by key set membership
        fn restrict(&mut self, keys: &ArraySetStEph<K>)
        {
            let ghost old_tree = self.tree@;
            let ghost old_map = self@;
            let ghost keys_set = keys@;
            let sorted = self.tree.in_order();
            let len = sorted.length();
            let mut new_tree = ParamBST::<Pair<K, V>>::new();
            let mut i: usize = 0;
            proof {
                lemma_sorted_keys_pairwise_distinct(old_tree, sorted@);
                lemma_key_unique_empty::<K::V, V::V>();
            }
            while i < len
                invariant
                    self.tree@ == old_tree,
                    old(self).spec_orderedtablesteph_wf(),
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
                    // Entries in new_tree have keys in keys_set.
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        keys_set.contains(p.0),
                    // Completeness: processed entries in keys_set are in new_tree.
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
            self.tree = new_tree;
            proof {
                lemma_pair_set_to_map_dom_finite(new_tree@);
                lemma_pair_set_to_map_dom_finite(old_tree);
                // Prove: self@.dom() =~= old_map.dom().intersect(keys_set)
                assert(self@.dom() =~= old_map.dom().intersect(keys_set)) by {
                    // Forward: k in self dom ==> k in old dom and k in keys_set.
                    assert forall|k: K::V| #[trigger] self@.dom().contains(k)
                        implies old_map.dom().contains(k) && keys_set.contains(k)
                    by {
                        lemma_map_contains_pair_in_set(new_tree@, k);
                        let v: V::V = choose|v: V::V| new_tree@.contains((k, v));
                        assert(old_tree.contains((k, v)));
                        lemma_pair_in_set_map_contains(old_tree, k, v);
                        assert(keys_set.contains(k));
                    };
                    // Backward: k in old dom and k in keys_set ==> k in self dom.
                    assert forall|k: K::V|
                        old_map.dom().contains(k) && keys_set.contains(k)
                        implies #[trigger] self@.dom().contains(k)
                    by {
                        lemma_map_contains_pair_in_set(old_tree, k);
                        let v: V::V = choose|v: V::V| old_tree.contains((k, v));
                        assert(sorted@.contains((k, v)));
                        let j = choose|j: int| 0 <= j < sorted@.len() && sorted@[j] == (k, v);
                        assert(new_tree@.contains(sorted@[j]));
                        lemma_pair_in_set_map_contains(new_tree@, k, v);
                    };
                };
                // Prove: values preserved.
                assert forall|k: K::V| #[trigger] self@.contains_key(k)
                    implies self@[k] == old_map[k]
                by {
                    lemma_map_contains_pair_in_set(new_tree@, k);
                    let v: V::V = choose|v: V::V| new_tree@.contains((k, v));
                    assert(old_tree.contains((k, v)));
                    lemma_pair_in_set_map_contains(new_tree@, k, v);
                    lemma_pair_in_set_map_contains(old_tree, k, v);
                };
                // Type axioms flow from old(self).spec_orderedtablesteph_wf().
                assert(spec_pair_key_determines_order::<K, V>());
                assert(vstd::laws_cmp::obeys_cmp_spec::<K>());
                assert(view_ord_consistent::<K>());
                assert(obeys_feq_fulls::<K, V>());
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- filter by key set exclusion
        fn subtract(&mut self, keys: &ArraySetStEph<K>)
        {
            let ghost old_tree = self.tree@;
            let ghost old_map = self@;
            let ghost keys_set = keys@;
            let sorted = self.tree.in_order();
            let len = sorted.length();
            let mut new_tree = ParamBST::<Pair<K, V>>::new();
            let mut i: usize = 0;
            proof {
                lemma_sorted_keys_pairwise_distinct(old_tree, sorted@);
                lemma_key_unique_empty::<K::V, V::V>();
            }
            while i < len
                invariant
                    self.tree@ == old_tree,
                    old(self).spec_orderedtablesteph_wf(),
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
                    // Entries in new_tree have keys NOT in keys_set.
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        !keys_set.contains(p.0),
                    // Completeness: processed entries not in keys_set are in new_tree.
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
            self.tree = new_tree;
            proof {
                lemma_pair_set_to_map_dom_finite(new_tree@);
                lemma_pair_set_to_map_dom_finite(old_tree);
                // Prove: self@.dom() =~= old_map.dom().difference(keys_set)
                assert(self@.dom() =~= old_map.dom().difference(keys_set)) by {
                    // Forward: k in self dom ==> k in old dom and k not in keys_set.
                    assert forall|k: K::V| #[trigger] self@.dom().contains(k)
                        implies old_map.dom().contains(k) && !keys_set.contains(k)
                    by {
                        lemma_map_contains_pair_in_set(new_tree@, k);
                        let v: V::V = choose|v: V::V| new_tree@.contains((k, v));
                        assert(old_tree.contains((k, v)));
                        lemma_pair_in_set_map_contains(old_tree, k, v);
                        assert(!keys_set.contains(k));
                    };
                    // Backward: k in old dom and k not in keys_set ==> k in self dom.
                    assert forall|k: K::V|
                        old_map.dom().contains(k) && !keys_set.contains(k)
                        implies #[trigger] self@.dom().contains(k)
                    by {
                        lemma_map_contains_pair_in_set(old_tree, k);
                        let v: V::V = choose|v: V::V| old_tree.contains((k, v));
                        assert(sorted@.contains((k, v)));
                        let j = choose|j: int| 0 <= j < sorted@.len() && sorted@[j] == (k, v);
                        assert(new_tree@.contains(sorted@[j]));
                        lemma_pair_in_set_map_contains(new_tree@, k, v);
                    };
                };
                // Prove: values preserved.
                assert forall|k: K::V| #[trigger] self@.contains_key(k)
                    implies self@[k] == old_map[k]
                by {
                    lemma_map_contains_pair_in_set(new_tree@, k);
                    let v: V::V = choose|v: V::V| new_tree@.contains((k, v));
                    assert(old_tree.contains((k, v)));
                    lemma_pair_in_set_map_contains(new_tree@, k, v);
                    lemma_pair_in_set_map_contains(old_tree, k, v);
                };
                // Type axioms flow from old(self).spec_orderedtablesteph_wf().
                assert(spec_pair_key_determines_order::<K, V>());
                assert(vstd::laws_cmp::obeys_cmp_spec::<K>());
                assert(view_ord_consistent::<K>());
                assert(obeys_feq_fulls::<K, V>());
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- in_order traversal + vec copy
        fn collect(&self) -> (collected: AVLTreeSeqStPerS<Pair<K, V>>)
            ensures
                self@.dom().finite(),
                collected.spec_avltreeseqstper_wf(),
                collected@.len() == self@.dom().len(),
        {
            let sorted = self.tree.in_order();
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
            let collected = AVLTreeSeqStPerS::from_vec(out);
            proof {
                lemma_pair_set_to_map_len(self.tree@);
                lemma_pair_set_to_map_dom_finite(self.tree@);
            }
            collected
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to first_key_iter
        fn first_key(&self) -> (first: Option<K>)
            where K: TotalOrderBridge
        {
            self.first_key_iter()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- BST min_key + key extraction
        fn first_key_iter(&self) -> (first: Option<K>)
            where K: TotalOrderBridge
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> first matches None,
                first matches Some(k) ==> self@.dom().contains(k@),
                first matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(v, t),
        {
            proof {
                lemma_pair_set_to_map_dom_finite(self.tree@);
                lemma_pair_set_to_map_len(self.tree@);
            }
            let min_pair = self.tree.min_key();
            match min_pair {
                None => None,
                Some(pair) => {
                    let key = pair.0.clone_plus();
                    // Expose BST type invariant: every view has a backing Pair value.
                    reveal_param_bst_backings(&self.tree);
                    proof {
                        lemma_reveal_view_injective::<K>();
                        lemma_cloned_view_eq(pair.0, key);
                        lemma_pair_in_set_map_contains(self.tree@, pair.0@, pair.1@);
                        assert(key == pair.0);
                        assert forall|t: K| #[trigger] self@.dom().contains(t@)
                            implies TotalOrder::le(key, t) by {
                            lemma_map_contains_pair_in_set(self.tree@, t@);
                            let vv: V::V = choose|vv: V::V| self.tree@.contains((t@, vv));
                            if pair.0@ == t@ {
                                assert(key@ == t@);
                                assert(key == t);
                                K::reflexive(key);
                            } else {
                                assert(pair@ != (t@, vv));
                                // From reveal_param_bst_backings: exists tp with tp@ == (t@, vv).
                                let tp: Pair<K, V> = choose|tp: Pair<K, V>| #[trigger] self.tree@.contains(tp@) && tp@ == (t@, vv);
                                // min_key: pair.cmp_spec(&tp) == Less (since pair@ != tp@).
                                assert(pair.cmp_spec(&tp) == Less);
                                // Keys differ: pair.0@ != tp.0@.
                                assert(pair.0@ != tp.0@);
                                // spec_pair_key_determines_order: pair.0.cmp_spec(&tp.0) == Less.
                                assert(pair.0.cmp_spec(&tp.0) == Less);
                                // tp.0@ == t@ → tp.0 == t (obeys_feq_eq).
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to last_key_iter
        fn last_key(&self) -> (last: Option<K>)
            where K: TotalOrderBridge
        {
            self.last_key_iter()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(lg n), Span O(lg n) -- BST max_key + key extraction
        fn last_key_iter(&self) -> (last: Option<K>)
            where K: TotalOrderBridge
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> last matches None,
                last matches Some(k) ==> self@.dom().contains(k@),
                last matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(t, v),
        {
            proof {
                lemma_pair_set_to_map_dom_finite(self.tree@);
                lemma_pair_set_to_map_len(self.tree@);
            }
            let max_pair = self.tree.max_key();
            match max_pair {
                None => None,
                Some(pair) => {
                    let key = pair.0.clone_plus();
                    // Expose BST type invariant for quantifier instantiation.
                    reveal_param_bst_backings(&self.tree);
                    proof {
                        lemma_reveal_view_injective::<K>();
                        lemma_cloned_view_eq(pair.0, key);
                        lemma_pair_in_set_map_contains(self.tree@, pair.0@, pair.1@);
                        assert(key == pair.0);
                        // max_key ensures: forall|tp| tree@.contains(tp@) ==> tp.cmp_spec(&pair) == Less || pair@ == tp@
                        // This means pair has the max key. Bridge to TotalOrder::le(t, key) for all t in domain.
                        assert forall|t: K| #[trigger] self@.dom().contains(t@)
                            implies TotalOrder::le(t, key) by {
                            lemma_map_contains_pair_in_set(self.tree@, t@);
                            let vv: V::V = choose|vv: V::V| self.tree@.contains((t@, vv));
                            if pair.0@ == t@ {
                                assert(key@ == t@);
                                assert(key == t);
                                K::reflexive(key);
                            } else {
                                assert(pair@ != (t@, vv));
                                // Backing Pair witness from reveal_param_bst_backings.
                                let tp: Pair<K, V> = choose|tp: Pair<K, V>| #[trigger] self.tree@.contains(tp@) && tp@ == (t@, vv);
                                // max_key: tp.cmp_spec(&pair) == Less (since pair@ != tp@).
                                assert(tp.cmp_spec(&pair) == Less);
                                // Keys differ: tp.0@ != pair.0@.
                                assert(tp.0@ != pair.0@);
                                // spec_pair_key_determines_order: tp.0.cmp_spec(&pair.0) == Less.
                                assert(tp.0.cmp_spec(&pair.0) == Less);
                                // tp.0@ == t@ → tp.0 == t.
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to previous_key_iter
        fn previous_key(&self, k: &K) -> (predecessor: Option<K>)
            where K: TotalOrderBridge
        {
            self.previous_key_iter(k)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- in_order traversal + linear scan for predecessor
        fn previous_key_iter(&self, k: &K) -> (predecessor: Option<K>)
            where K: TotalOrderBridge
            ensures
                self@.dom().finite(),
                predecessor matches Some(pk) ==> self@.dom().contains(pk@),
                predecessor matches Some(v) ==> TotalOrder::le(v, *k) && v@ != k@,
                predecessor matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(t, *k) && t@ != k@ ==> TotalOrder::le(t, v),
        {
            proof { lemma_reveal_view_injective::<K>(); lemma_pair_set_to_map_dom_finite(self.tree@); }
            let sorted = self.tree.in_order();
            let len = sorted.length();
            proof {
                assert forall|j: int| 0 <= j < sorted@.len()
                    implies self.tree@.contains(#[trigger] sorted@[j]) by {
                    assert(sorted@.contains(sorted@[j]));
                };
            }
            let mut found = false;
            let mut best: Option<K> = None;
            let mut i: usize = 0;
            while i < len
                invariant
                    self.spec_orderedtablesteph_wf(),
                    0 <= i <= len,
                    len as nat == sorted@.len(),
                    !found ==> best is None,
                    !found ==> forall|j: int| 0 <= j < i as int ==>
                        !(TotalOrder::le((#[trigger] sorted.spec_index(j)).0, *k) && sorted.spec_index(j).0@ != k@),
                    found ==> best is Some,
                    found ==> self@.dom().contains(best->Some_0@),
                    found ==> TotalOrder::le(best->Some_0, *k) && best->Some_0@ != k@,
                    found ==> forall|j: int| 0 <= j < i as int
                        && TotalOrder::le((#[trigger] sorted.spec_index(j)).0, *k) && sorted.spec_index(j).0@ != k@
                        ==> TotalOrder::le(sorted.spec_index(j).0, best->Some_0),
                    forall|j: int| 0 <= j < sorted@.len() ==>
                        self.tree@.contains(#[trigger] sorted@[j]),
                decreases len - i,
            {
                let elem = sorted.nth(i);
                let c = TotalOrder::cmp(&elem.0, k);
                match c {
                    core::cmp::Ordering::Less => {
                        if !found {
                            found = true;
                            let k_clone = elem.0.clone_plus();
                            proof {
                                lemma_reveal_view_injective::<K>();
                                lemma_cloned_view_eq(elem.0, k_clone);
                                assert(self.tree@.contains(sorted@[i as int]));
                                lemma_pair_in_set_map_contains(self.tree@, sorted@[i as int].0, sorted@[i as int].1);
                                K::reflexive(k_clone);
                            }
                            best = Some(k_clone);
                        } else {
                            let old_best = best.take().unwrap();
                            let c2 = TotalOrder::cmp(&elem.0, &old_best);
                            match c2 {
                                core::cmp::Ordering::Greater => {
                                    let k_clone = elem.0.clone_plus();
                                    proof {
                                        lemma_reveal_view_injective::<K>();
                                        lemma_cloned_view_eq(elem.0, k_clone);
                                        assert(self.tree@.contains(sorted@[i as int]));
                                        lemma_pair_in_set_map_contains(self.tree@, sorted@[i as int].0, sorted@[i as int].1);
                                        assert forall|j: int| 0 <= j < i + 1
                                            && TotalOrder::le((#[trigger] sorted.spec_index(j)).0, *k) && sorted.spec_index(j).0@ != k@
                                            implies TotalOrder::le(sorted.spec_index(j).0, k_clone) by {
                                            if j == i as int {
                                                K::reflexive(k_clone);
                                            } else {
                                                K::transitive(sorted.spec_index(j).0, old_best, k_clone);
                                            }
                                        };
                                    }
                                    best = Some(k_clone);
                                },
                                _ => {
                                    proof {
                                        K::total(elem.0, old_best);
                                    }
                                    best = Some(old_best);
                                },
                            }
                        }
                    },
                    core::cmp::Ordering::Equal => {},
                    core::cmp::Ordering::Greater => {
                        proof {
                            if TotalOrder::le(elem.0, *k) {
                                K::antisymmetric(elem.0, *k);
                            }
                        }
                    },
                }
                i = i + 1;
            }
            proof {
                if found {
                    assert forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(t, *k) && t@ != k@
                        implies TotalOrder::le(t, best->Some_0) by {
                        lemma_map_contains_pair_in_set(self.tree@, t@);
                        let v: V::V = choose|v: V::V| self.tree@.contains((t@, v));
                        assert(sorted@.contains((t@, v)));
                        let j = choose|j: int| 0 <= j < sorted@.len() && sorted@[j] == (t@, v);
                        assert(sorted.spec_index(j).0 == t);
                    };
                }
            }
            best
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to next_key_iter
        fn next_key(&self, k: &K) -> (successor: Option<K>)
            where K: TotalOrderBridge
        {
            self.next_key_iter(k)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- in_order traversal + linear scan for successor
        fn next_key_iter(&self, k: &K) -> (successor: Option<K>)
            where K: TotalOrderBridge
            ensures
                self@.dom().finite(),
                successor matches Some(nk) ==> self@.dom().contains(nk@),
                successor matches Some(v) ==> TotalOrder::le(*k, v) && v@ != k@,
                successor matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(*k, t) && t@ != k@ ==> TotalOrder::le(v, t),
        {
            proof { lemma_reveal_view_injective::<K>(); lemma_pair_set_to_map_dom_finite(self.tree@); }
            let sorted = self.tree.in_order();
            let len = sorted.length();
            proof {
                assert forall|j: int| 0 <= j < sorted@.len()
                    implies self.tree@.contains(#[trigger] sorted@[j]) by {
                    assert(sorted@.contains(sorted@[j]));
                };
            }
            let mut found = false;
            let mut best: Option<K> = None;
            let mut i: usize = 0;
            while i < len
                invariant
                    self.spec_orderedtablesteph_wf(),
                    0 <= i <= len,
                    len as nat == sorted@.len(),
                    !found ==> best is None,
                    !found ==> forall|j: int| 0 <= j < i as int ==>
                        !(TotalOrder::le(*k, (#[trigger] sorted.spec_index(j)).0) && sorted.spec_index(j).0@ != k@),
                    found ==> best is Some,
                    found ==> self@.dom().contains(best->Some_0@),
                    found ==> TotalOrder::le(*k, best->Some_0) && best->Some_0@ != k@,
                    found ==> forall|j: int| 0 <= j < i as int
                        && TotalOrder::le(*k, (#[trigger] sorted.spec_index(j)).0) && sorted.spec_index(j).0@ != k@
                        ==> TotalOrder::le(best->Some_0, sorted.spec_index(j).0),
                    forall|j: int| 0 <= j < sorted@.len() ==>
                        self.tree@.contains(#[trigger] sorted@[j]),
                decreases len - i,
            {
                let elem = sorted.nth(i);
                let c = TotalOrder::cmp(&elem.0, k);
                match c {
                    core::cmp::Ordering::Greater => {
                        if !found {
                            found = true;
                            let k_clone = elem.0.clone_plus();
                            proof {
                                lemma_reveal_view_injective::<K>();
                                lemma_cloned_view_eq(elem.0, k_clone);
                                assert(self.tree@.contains(sorted@[i as int]));
                                lemma_pair_in_set_map_contains(self.tree@, sorted@[i as int].0, sorted@[i as int].1);
                                K::reflexive(k_clone);
                            }
                            best = Some(k_clone);
                        } else {
                            let old_best = best.take().unwrap();
                            let c2 = TotalOrder::cmp(&elem.0, &old_best);
                            match c2 {
                                core::cmp::Ordering::Less => {
                                    let k_clone = elem.0.clone_plus();
                                    proof {
                                        lemma_reveal_view_injective::<K>();
                                        lemma_cloned_view_eq(elem.0, k_clone);
                                        assert(self.tree@.contains(sorted@[i as int]));
                                        lemma_pair_in_set_map_contains(self.tree@, sorted@[i as int].0, sorted@[i as int].1);
                                        assert forall|j: int| 0 <= j < i + 1
                                            && TotalOrder::le(*k, (#[trigger] sorted.spec_index(j)).0) && sorted.spec_index(j).0@ != k@
                                            implies TotalOrder::le(k_clone, sorted.spec_index(j).0) by {
                                            if j == i as int {
                                                K::reflexive(k_clone);
                                            } else {
                                                K::transitive(k_clone, old_best, sorted.spec_index(j).0);
                                            }
                                        };
                                    }
                                    best = Some(k_clone);
                                },
                                _ => {
                                    proof {
                                        K::total(old_best, elem.0);
                                    }
                                    best = Some(old_best);
                                },
                            }
                        }
                    },
                    core::cmp::Ordering::Equal => {},
                    core::cmp::Ordering::Less => {
                        proof {
                            if TotalOrder::le(*k, elem.0) {
                                K::antisymmetric(*k, elem.0);
                            }
                        }
                    },
                }
                i = i + 1;
            }
            proof {
                if found {
                    assert forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(*k, t) && t@ != k@
                        implies TotalOrder::le(best->Some_0, t) by {
                        lemma_map_contains_pair_in_set(self.tree@, t@);
                        let v: V::V = choose|v: V::V| self.tree@.contains((t@, v));
                        assert(sorted@.contains((t@, v)));
                        let j = choose|j: int| 0 <= j < sorted@.len() && sorted@[j] == (t@, v);
                        assert(sorted.spec_index(j).0 == t);
                    };
                }
            }
            best
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- delegates to split_key_iter
        fn split_key(&mut self, k: &K) -> (split: (Self, Option<V>, Self))
            where Self: Sized
        {
            self.split_key_iter(k)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- in_order + n BST inserts into two new trees
        fn split_key_iter(&mut self, k: &K) -> (split: (Self, Option<V>, Self))
            where Self: Sized
        {
            let ghost old_tree = self.tree@;
            let ghost old_map = self@;
            let found_val = self.find(k);
            let sorted = self.tree.in_order();
            let len = sorted.length();
            let mut left_tree = ParamBST::<Pair<K, V>>::new();
            let mut right_tree = ParamBST::<Pair<K, V>>::new();
            let mut i: usize = 0;
            proof {
                assert(obeys_feq_full_trigger::<Pair<K, V>>());
                reveal(obeys_view_eq);
                lemma_sorted_keys_pairwise_distinct(old_tree, sorted@);
                lemma_key_unique_empty::<K::V, V::V>();
            }
            while i < len
                invariant
                    self.tree@ == old_tree,
                    old(self).spec_orderedtablesteph_wf(),
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
                    view_ord_consistent::<K>(),
                    vstd::laws_cmp::obeys_cmp_spec::<K>(),
                    old_map == spec_pair_set_to_map(old_tree),
                    len as nat == sorted@.len(),
                    sorted@.len() == old_tree.len(),
                    forall|v: <Pair<K, V> as View>::V| old_tree.contains(v) <==> #[trigger] sorted@.contains(v),
                    0 <= i <= len,
                    left_tree.spec_bstparasteph_wf(),
                    right_tree.spec_bstparasteph_wf(),
                    vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                    view_ord_consistent::<Pair<K, V>>(),
                    left_tree@.len() <= i as nat,
                    left_tree@.len() < usize::MAX as nat,
                    right_tree@.len() <= i as nat,
                    right_tree@.len() < usize::MAX as nat,
                    spec_key_unique_pairs_set(left_tree@),
                    spec_key_unique_pairs_set(right_tree@),
                    spec_key_unique_pairs_set(old_tree),
                    forall|ii: int, jj: int|
                        0 <= ii < sorted@.len() && 0 <= jj < sorted@.len() && ii != jj
                        ==> (#[trigger] sorted@[ii]).0 != (#[trigger] sorted@[jj]).0,
                    forall|p: (K::V, V::V)| #[trigger] left_tree@.contains(p) ==>
                        exists|j: int| 0 <= j < i as int && p.0 == (#[trigger] sorted@[j]).0,
                    forall|p: (K::V, V::V)| #[trigger] right_tree@.contains(p) ==>
                        exists|j: int| 0 <= j < i as int && p.0 == (#[trigger] sorted@[j]).0,
                    // Left and right entries come from old_tree.
                    forall|p: (K::V, V::V)| #[trigger] left_tree@.contains(p) ==> old_tree.contains(p),
                    forall|p: (K::V, V::V)| #[trigger] right_tree@.contains(p) ==> old_tree.contains(p),
                    // Key k@ not in left or right.
                    forall|p: (K::V, V::V)| #[trigger] left_tree@.contains(p) ==> p.0 != k@,
                    forall|p: (K::V, V::V)| #[trigger] right_tree@.contains(p) ==> p.0 != k@,
                    // Left and right are disjoint.
                    left_tree@.disjoint(right_tree@),
                    // Completeness: processed non-k entries are in left or right.
                    forall|j: int| 0 <= j < i as int && sorted@[j].0 != k@
                        ==> (#[trigger] left_tree@.contains(sorted@[j]) || right_tree@.contains(sorted@[j])),
                decreases len - i,
            {
                let pair = sorted.nth(i);
                proof { reveal(obeys_view_eq); }
                let c = pair.0.cmp(k);
                proof {
                    reveal(vstd::laws_cmp::obeys_cmp_ord);
                }
                match c {
                    core::cmp::Ordering::Less => {
                        proof {
                            // pair.0@ != k@ from cmp Less + view_ord_consistent.
                            assert(pair.0@ != k@);
                        }
                        let cloned = pair.clone_plus();
                        let ghost old_left = left_tree@;
                        proof {
                            lemma_cloned_view_eq(*pair, cloned);
                            assert(!spec_pair_set_to_map(old_left).dom().contains(sorted@[i as int].0)) by {
                                if spec_pair_set_to_map(old_left).dom().contains(sorted@[i as int].0) {
                                    lemma_map_contains_pair_in_set(old_left, sorted@[i as int].0);
                                    let vv: V::V = choose|vv: V::V| old_left.contains((sorted@[i as int].0, vv));
                                    let jj = choose|jj: int| 0 <= jj < i as int && (sorted@[i as int].0, vv).0 == (#[trigger] sorted@[jj]).0;
                                    assert(false);
                                }
                            };
                            assert(old_left.len() < usize::MAX as nat);
                            // sorted@[i] not in right_tree (key uniqueness + index tracking).
                            assert(!right_tree@.contains(sorted@[i as int])) by {
                                if right_tree@.contains(sorted@[i as int]) {
                                    let rj = choose|rj: int| 0 <= rj < i as int && sorted@[i as int].0 == (#[trigger] sorted@[rj]).0;
                                    assert(false);
                                }
                            };
                        }
                        left_tree.insert(cloned);
                        proof {
                            assert(left_tree@.len() <= i as nat + 1);
                            lemma_key_unique_insert(old_left, sorted@[i as int].0, sorted@[i as int].1);
                            assert(sorted@.contains(sorted@[i as int])) by { assert(sorted@[i as int] == sorted@[i as int]); };
                            assert(old_tree.contains(sorted@[i as int]));
                            assert(left_tree@.contains(sorted@[i as int]));
                        }
                    },
                    core::cmp::Ordering::Greater => {
                        proof {
                            // pair.0@ != k@ from cmp Greater + view_ord_consistent.
                            assert(pair.0@ != k@);
                        }
                        let cloned = pair.clone_plus();
                        let ghost old_right = right_tree@;
                        proof {
                            lemma_cloned_view_eq(*pair, cloned);
                            assert(!spec_pair_set_to_map(old_right).dom().contains(sorted@[i as int].0)) by {
                                if spec_pair_set_to_map(old_right).dom().contains(sorted@[i as int].0) {
                                    lemma_map_contains_pair_in_set(old_right, sorted@[i as int].0);
                                    let vv: V::V = choose|vv: V::V| old_right.contains((sorted@[i as int].0, vv));
                                    let jj = choose|jj: int| 0 <= jj < i as int && (sorted@[i as int].0, vv).0 == (#[trigger] sorted@[jj]).0;
                                    assert(false);
                                }
                            };
                            assert(old_right.len() < usize::MAX as nat);
                            // sorted@[i] not in left_tree (key uniqueness + index tracking).
                            assert(!left_tree@.contains(sorted@[i as int])) by {
                                if left_tree@.contains(sorted@[i as int]) {
                                    let lj = choose|lj: int| 0 <= lj < i as int && sorted@[i as int].0 == (#[trigger] sorted@[lj]).0;
                                    assert(false);
                                }
                            };
                        }
                        right_tree.insert(cloned);
                        proof {
                            assert(right_tree@.len() <= i as nat + 1);
                            lemma_key_unique_insert(old_right, sorted@[i as int].0, sorted@[i as int].1);
                            assert(sorted@.contains(sorted@[i as int])) by { assert(sorted@[i as int] == sorted@[i as int]); };
                            assert(old_tree.contains(sorted@[i as int]));
                            assert(right_tree@.contains(sorted@[i as int]));
                        }
                    },
                    core::cmp::Ordering::Equal => {
                        proof {
                            // cmp Equal + view_ord_consistent => pair.0@ == k@.
                            assert(pair.0@ == k@);
                        }
                    },
                }
                i += 1;
            }
            let left_table = OrderedTableStEph { tree: left_tree };
            let right_table = OrderedTableStEph { tree: right_tree };
            *self = Self::empty();
            proof {
                lemma_pair_set_to_map_dom_finite(old_tree);
                lemma_pair_set_to_map_dom_finite(left_tree@);
                lemma_pair_set_to_map_dom_finite(right_tree@);
                // Prove: left and right subsets of old_map domain.
                assert(left_table@.dom().subset_of(old_map.dom())) by {
                    assert forall|key: K::V| left_table@.dom().contains(key)
                        implies #[trigger] old_map.dom().contains(key)
                    by {
                        lemma_map_contains_pair_in_set(left_tree@, key);
                        let v: V::V = choose|v: V::V| left_tree@.contains((key, v));
                        assert(old_tree.contains((key, v)));
                        lemma_pair_in_set_map_contains(old_tree, key, v);
                    };
                };
                assert(right_table@.dom().subset_of(old_map.dom())) by {
                    assert forall|key: K::V| right_table@.dom().contains(key)
                        implies #[trigger] old_map.dom().contains(key)
                    by {
                        lemma_map_contains_pair_in_set(right_tree@, key);
                        let v: V::V = choose|v: V::V| right_tree@.contains((key, v));
                        assert(old_tree.contains((key, v)));
                        lemma_pair_in_set_map_contains(old_tree, key, v);
                    };
                };
                // k@ not in left or right domains.
                assert(!left_table@.dom().contains(k@)) by {
                    if left_table@.dom().contains(k@) {
                        lemma_map_contains_pair_in_set(left_tree@, k@);
                        let v: V::V = choose|v: V::V| left_tree@.contains((k@, v));
                        assert(false);
                    }
                };
                assert(!right_table@.dom().contains(k@)) by {
                    if right_table@.dom().contains(k@) {
                        lemma_map_contains_pair_in_set(right_tree@, k@);
                        let v: V::V = choose|v: V::V| right_tree@.contains((k@, v));
                        assert(false);
                    }
                };
                // Left and right domains are disjoint (flows from set disjointness).
                assert(left_table@.dom().disjoint(right_table@.dom())) by {
                    assert forall|key: K::V|
                        !(left_table@.dom().contains(key) && #[trigger] right_table@.dom().contains(key))
                    by {
                        if left_table@.dom().contains(key) && right_table@.dom().contains(key) {
                            lemma_map_contains_pair_in_set(left_tree@, key);
                            lemma_map_contains_pair_in_set(right_tree@, key);
                            let lv: V::V = choose|v: V::V| left_tree@.contains((key, v));
                            let rv: V::V = choose|v: V::V| right_tree@.contains((key, v));
                            // left_tree.disjoint(right_tree) means no pair in both.
                            // But we have (key, lv) in left and (key, rv) in right.
                            // Both come from old_tree with key uniqueness, so lv == rv.
                            // Then (key, lv) is in both, contradicting disjointness.
                            assert(old_tree.contains((key, lv)));
                            assert(old_tree.contains((key, rv)));
                            // Key uniqueness: lv == rv.
                            assert(lv == rv);
                            // Now (key, lv) in both left and right contradicts disjointness.
                            assert(left_tree@.contains((key, lv)));
                            assert(right_tree@.contains((key, lv)));
                            assert(!left_tree@.disjoint(right_tree@));
                            assert(false);
                        }
                    };
                };
                // Completeness: all old_map keys are in left, right, or equal to k@.
                assert forall|key: K::V| #[trigger] old_map.dom().contains(key)
                    implies left_table@.dom().contains(key) || right_table@.dom().contains(key) || key == k@
                by {
                    lemma_map_contains_pair_in_set(old_tree, key);
                    let v: V::V = choose|v: V::V| old_tree.contains((key, v));
                    assert(sorted@.contains((key, v)));
                    let j = choose|j: int| 0 <= j < sorted@.len() && sorted@[j] == (key, v);
                    if key != k@ {
                        // By completeness invariant, sorted@[j] is in left or right.
                        assert(left_tree@.contains(sorted@[j]) || right_tree@.contains(sorted@[j]));
                        if left_tree@.contains(sorted@[j]) {
                            lemma_pair_in_set_map_contains(left_tree@, key, v);
                        } else {
                            lemma_pair_in_set_map_contains(right_tree@, key, v);
                        }
                    }
                };
                // Size bounds.
                vstd::set_lib::lemma_len_subset(left_tree@, old_tree);
                vstd::set_lib::lemma_len_subset(right_tree@, old_tree);
                // Prove wf.
                assert(spec_pair_key_determines_order::<K, V>());
                assert(vstd::laws_cmp::obeys_cmp_spec::<K>());
                assert(view_ord_consistent::<K>());
                assert(obeys_feq_fulls::<K, V>());
            }
            (left_table, found_val, right_table)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- delegates to union
        fn join_key(&mut self, other: Self)
        {
            self.union(&other, |v1, _v2| v1.clone());
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- delegates to get_key_range_iter
        fn get_key_range(&self, k1: &K, k2: &K) -> (range: Self)
        {
            self.get_key_range_iter(k1, k2)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- in_order + conditional BST inserts
        fn get_key_range_iter(&self, k1: &K, k2: &K) -> (range: Self)
        {
            let sorted = self.tree.in_order();
            let len = sorted.length();
            let mut new_tree = ParamBST::<Pair<K, V>>::new();
            let mut i: usize = 0;
            proof {
                lemma_sorted_keys_pairwise_distinct(self.tree@, sorted@);
                lemma_key_unique_empty::<K::V, V::V>();
            }
            while i < len
                invariant
                    self.spec_orderedtablesteph_wf(),
                    obeys_feq_full::<Pair<K, V>>(),
                    len as nat == sorted@.len(),
                    sorted@.len() == self.tree@.len(),
                    0 <= i <= len,
                    new_tree.spec_bstparasteph_wf(),
                    vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                    view_ord_consistent::<Pair<K, V>>(),
                    new_tree@.len() <= i as nat,
                    new_tree@.len() < usize::MAX as nat,
                    spec_key_unique_pairs_set(new_tree@),
                    spec_key_unique_pairs_set(self.tree@),
                    forall|v: <Pair<K, V> as View>::V| self.tree@.contains(v) <==> #[trigger] sorted@.contains(v),
                    forall|ii: int, jj: int|
                        0 <= ii < sorted@.len() && 0 <= jj < sorted@.len() && ii != jj
                        ==> (#[trigger] sorted@[ii]).0 != (#[trigger] sorted@[jj]).0,
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        exists|j: int| 0 <= j < i as int && p.0 == (#[trigger] sorted@[j]).0,
                    // All entries in new_tree are from self.tree (values preserved).
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==> self.tree@.contains(p),
                decreases len - i,
            {
                let pair = sorted.nth(i);
                let ge_k1 = match pair.0.cmp(k1) {
                    Less => false,
                    _ => true,
                };
                let le_k2 = match pair.0.cmp(k2) {
                    Greater => false,
                    _ => true,
                };
                if ge_k1 && le_k2 {
                    let cloned = pair.clone_plus();
                    let ghost old_new_tree = new_tree@;
                    proof {
                        lemma_cloned_view_eq(*pair, cloned);
                        // Freshness proof.
                        assert(!spec_pair_set_to_map(old_new_tree).dom().contains(sorted@[i as int].0)) by {
                            if spec_pair_set_to_map(old_new_tree).dom().contains(sorted@[i as int].0) {
                                lemma_map_contains_pair_in_set(old_new_tree, sorted@[i as int].0);
                                let vv: V::V = choose|vv: V::V| old_new_tree.contains((sorted@[i as int].0, vv));
                                let jj = choose|jj: int| 0 <= jj < i as int && (sorted@[i as int].0, vv).0 == (#[trigger] sorted@[jj]).0;
                                assert(false);
                            }
                        };
                    }
                    new_tree.insert(cloned);
                    proof {
                        assert(new_tree@.len() <= i as nat + 1);
                        lemma_key_unique_insert(old_new_tree, sorted@[i as int].0, sorted@[i as int].1);
                        // Maintain subset invariant.
                        assert(sorted@.contains(sorted@[i as int])) by { assert(sorted@[i as int] == sorted@[i as int]); };
                        assert(self.tree@.contains(sorted@[i as int]));
                    }
                }
                i += 1;
            }
            let range = OrderedTableStEph { tree: new_tree };
            proof {
                lemma_pair_set_to_map_dom_finite(new_tree@);
                lemma_pair_set_to_map_dom_finite(self.tree@);
                // Prove: range@.dom().subset_of(self@.dom())
                assert(range@.dom().subset_of(self@.dom())) by {
                    assert forall|k: K::V| range@.dom().contains(k)
                        implies #[trigger] self@.dom().contains(k)
                    by {
                        lemma_map_contains_pair_in_set(new_tree@, k);
                        let v: V::V = choose|v: V::V| new_tree@.contains((k, v));
                        assert(self.tree@.contains((k, v)));
                        lemma_pair_in_set_map_contains(self.tree@, k, v);
                    };
                };
                // Prove: values preserved.
                assert forall|key: K::V| #[trigger] range@.dom().contains(key)
                    implies range@[key] == self@[key]
                by {
                    lemma_map_contains_pair_in_set(new_tree@, key);
                    let v: V::V = choose|v: V::V| new_tree@.contains((key, v));
                    assert(self.tree@.contains((key, v)));
                    lemma_pair_in_set_map_contains(new_tree@, key, v);
                    lemma_pair_in_set_map_contains(self.tree@, key, v);
                };
                // Prove wf: size bound.
                vstd::set_lib::lemma_len_subset(new_tree@, self.tree@);
                // Type axioms flow from self.spec_orderedtablesteph_wf().
                assert(spec_pair_key_determines_order::<K, V>());
                assert(vstd::laws_cmp::obeys_cmp_spec::<K>());
                assert(view_ord_consistent::<K>());
                assert(obeys_feq_fulls::<K, V>());
            }
            range
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to rank_key_iter
        fn rank_key(&self, k: &K) -> (rank: usize)
            where K: TotalOrderBridge
        {
            self.rank_key_iter(k)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- in_order traversal + count elements <= k
        #[verifier::loop_isolation(false)]
        fn rank_key_iter(&self, k: &K) -> (rank: usize)
            where K: TotalOrderBridge
        {
            proof {
                lemma_reveal_view_injective::<K>();
                assert(obeys_feq_full_trigger::<K>());
                lemma_pair_set_to_map_dom_finite(self.tree@);
            }
            let sorted = self.tree.in_order();
            let len = sorted.length();
            let mut count: usize = 0;
            let mut i: usize = 0;
            let ghost filter_pred = |x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, *k) && t@ != k@;
            let ghost mut counted_keys: Set<K::V> = Set::empty();
            proof {
                lemma_sorted_keys_pairwise_distinct(self.tree@, sorted@);
            }
            while i < len
                invariant
                    self.spec_orderedtablesteph_wf(),
                    obeys_feq_full::<K>(),
                    obeys_view_eq::<K>(),
                    len as nat == sorted@.len(),
                    sorted@.len() == self.tree@.len(),
                    forall|v: <Pair<K, V> as View>::V| self.tree@.contains(v) <==> #[trigger] sorted@.contains(v),
                    forall|ii: int, jj: int|
                        0 <= ii < sorted@.len() && 0 <= jj < sorted@.len() && ii != jj
                        ==> (#[trigger] sorted@[ii]).0 != (#[trigger] sorted@[jj]).0,
                    0 <= i <= len,
                    0 <= count <= i,
                    counted_keys.finite(),
                    count as nat == counted_keys.len(),
                    forall|x: K::V| #[trigger] counted_keys.contains(x) ==>
                        (exists|j: int| #![trigger sorted@[j]] 0 <= j < i as int
                            && sorted@[j].0 == x && filter_pred(x)),
                    forall|j: int| #![trigger sorted@[j]] 0 <= j < i as int && filter_pred(sorted@[j].0) ==>
                        counted_keys.contains(sorted@[j].0),
                    forall|x: K::V| counted_keys.contains(x) ==> #[trigger] self@.dom().contains(x),
                decreases len - i,
            {
                let pair = sorted.nth(i);
                let c = TotalOrder::cmp(&pair.0, k);
                proof { reveal(obeys_view_eq); }
                match c {
                    core::cmp::Ordering::Less => {
                        proof {
                            assert(count < len) by { };
                            // pair.0 < k: witness for filter_pred.
                            assert(TotalOrder::le(pair.0, *k) && pair.0 != *k);
                            assert(pair.0@ != k@);
                            assert(filter_pred(pair.0@)) by {
                                assert(pair.0@ == pair.0@ && TotalOrder::le(pair.0, *k) && pair.0@ != k@);
                            };
                            // pair.0@ not already counted (pairwise distinct keys).
                            assert(!counted_keys.contains(pair.0@)) by {
                                if counted_keys.contains(pair.0@) {
                                    let jj = choose|jj: int| 0 <= jj < i as int
                                        && (#[trigger] sorted@[jj]).0 == pair.0@ && filter_pred(pair.0@);
                                    assert(sorted@[jj as int].0 == sorted@[i as int].0);
                                }
                            };
                            counted_keys = counted_keys.insert(pair.0@);
                            // In self@.dom().
                            assert(sorted@.contains(sorted@[i as int])) by {
                                assert(sorted@[i as int] == sorted@[i as int]);
                            };
                            assert(self.tree@.contains(sorted@[i as int]));
                            lemma_pair_in_set_map_contains(self.tree@, sorted@[i as int].0, sorted@[i as int].1);
                        }
                        count = count + 1;
                    },
                    core::cmp::Ordering::Equal => {
                        proof {
                            // pair.0 == k: filter_pred(pair.0@) is false.
                            assert(pair.0 == *k);
                            assert(!filter_pred(pair.0@)) by {
                                if filter_pred(pair.0@) {
                                    let t: K = choose|t: K| #![trigger t@] t@ == pair.0@ && TotalOrder::le(t, *k) && t@ != k@;
                                    assert(t@ == pair.0@ && pair.0@ == k@);
                                    assert(t@ != k@);
                                }
                            };
                        }
                    },
                    core::cmp::Ordering::Greater => {
                        proof {
                            // k < pair.0: filter_pred(pair.0@) is false.
                            assert(TotalOrder::le(*k, pair.0) && pair.0 != *k);
                            assert(pair.0@ != k@);
                            assert(!filter_pred(pair.0@)) by {
                                if filter_pred(pair.0@) {
                                    let t: K = choose|t: K| #![trigger t@] t@ == pair.0@ && TotalOrder::le(t, *k) && t@ != k@;
                                    // t@ == pair.0@, so by obeys_view_eq t == pair.0.
                                    assert(t@ == pair.0@);
                                    assert(t == pair.0);
                                    // t.le(k) && k.le(pair.0) with t == pair.0 gives pair.0.le(k) && k.le(pair.0).
                                    TotalOrder::antisymmetric(pair.0, *k);
                                }
                            };
                        }
                    },
                }
                i = i + 1;
            }
            proof {
                // counted_keys =~= self@.dom().filter(filter_pred).
                assert forall|x: K::V| counted_keys.contains(x)
                    implies #[trigger] self@.dom().filter(filter_pred).contains(x) by {
                };
                assert forall|x: K::V| #[trigger] self@.dom().filter(filter_pred).contains(x)
                    implies counted_keys.contains(x) by {
                    // x is in self@.dom() and filter_pred(x) holds.
                    lemma_map_contains_pair_in_set(self.tree@, x);
                    let vv: V::V = choose|vv: V::V| self.tree@.contains((x, vv));
                    assert(sorted@.contains((x, vv)));
                    let j = choose|j: int| 0 <= j < sorted@.len() && sorted@[j] == (x, vv);
                    assert(sorted@[j].0 == x && filter_pred(sorted@[j].0));
                };
                assert(counted_keys =~= self@.dom().filter(filter_pred));
                self@.dom().lemma_len_filter(filter_pred);
                lemma_pair_set_to_map_len(self.tree@);
            }
            count
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- in_order traversal + index into sorted sequence
        #[verifier::loop_isolation(false)]
        fn select_key(&self, i: usize) -> (selected: Option<K>)
            where K: TotalOrderBridge
        {
            proof {
                assert(obeys_feq_full_trigger::<K>());
                assert(obeys_feq_full_trigger::<Pair<K, V>>());
                lemma_pair_set_to_map_dom_finite(self.tree@);
            }
            if i >= self.size() {
                None
            } else {
                let sorted = self.tree.in_order();
                let len = sorted.length();
                proof {
                    assert forall|jj: int| 0 <= jj < sorted@.len()
                        implies self.tree@.contains(#[trigger] sorted@[jj]) by {
                        assert(sorted@.contains(sorted@[jj]));
                    };
                }
                let mut j: usize = 0;
                let mut result_key: Option<K> = None;
                while j < len
                    invariant
                        j <= len,
                        len as nat == sorted@.len(),
                        self.spec_orderedtablesteph_wf(),
                        obeys_view_eq::<K>(),
                        obeys_feq_full::<K>(),
                        obeys_feq_full::<Pair<K, V>>(),
                        self@.dom().finite(),
                        i < self@.dom().len(),
                        forall|jj: int| 0 <= jj < sorted@.len() ==>
                            self.tree@.contains(#[trigger] sorted@[jj]),
                        result_key matches Some(rk) ==>
                            self@.dom().contains(rk@) &&
                            self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@]
                                t@ == x && TotalOrder::le(t, rk) && t@ != rk@).len() == i as int,
                    decreases len - j,
                {
                    let candidate = sorted.nth(j);
                    let candidate_key = candidate.0.clone_plus();
                    proof { lemma_cloned_view_eq(candidate.0, candidate_key); }
                    let rank_val = self.rank_key(&candidate_key);
                    if rank_val == i && result_key.is_none() {
                        proof {
                            lemma_pair_in_set_map_contains(self.tree@, sorted@[j as int].0, sorted@[j as int].1);
                        }
                        result_key = Some(candidate_key);
                    }
                    j = j + 1;
                }
                result_key
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- delegates to split_rank_key_iter
        fn split_rank_key(&mut self, i: usize) -> (split: (Self, Self))
            where Self: Sized
        {
            self.split_rank_key_iter(i)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- in_order + n BST inserts into two new trees
        fn split_rank_key_iter(&mut self, i: usize) -> (split: (Self, Self))
            where Self: Sized
        {
            let ghost old_tree = self.tree@;
            let ghost old_map = self@;
            let sorted = self.tree.in_order();
            let size = sorted.length();
            let split_at: usize = if i >= size { size } else { i };
            proof {
                assert(obeys_feq_full_trigger::<Pair<K, V>>());
                lemma_sorted_keys_pairwise_distinct(old_tree, sorted@);
                lemma_key_unique_empty::<K::V, V::V>();
            }
            let mut left_tree = ParamBST::<Pair<K, V>>::new();
            let mut right_tree = ParamBST::<Pair<K, V>>::new();
            let mut j: usize = 0;
            while j < size
                invariant
                    old(self).spec_orderedtablesteph_wf(),
                    obeys_feq_full::<Pair<K, V>>(),
                    size as nat == sorted@.len(),
                    sorted@.len() == old_tree.len(),
                    forall|v: <Pair<K, V> as View>::V| old_tree.contains(v) <==> #[trigger] sorted@.contains(v),
                    split_at <= size,
                    0 <= j <= size,
                    left_tree.spec_bstparasteph_wf(),
                    right_tree.spec_bstparasteph_wf(),
                    vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                    view_ord_consistent::<Pair<K, V>>(),
                    left_tree@.len() <= j as nat,
                    right_tree@.len() <= j as nat,
                    spec_key_unique_pairs_set(left_tree@),
                    spec_key_unique_pairs_set(right_tree@),
                    spec_key_unique_pairs_set(old_tree),
                    forall|ii: int, jj: int|
                        0 <= ii < sorted@.len() && 0 <= jj < sorted@.len() && ii != jj
                        ==> (#[trigger] sorted@[ii]).0 != (#[trigger] sorted@[jj]).0,
                    forall|p: (K::V, V::V)| #[trigger] left_tree@.contains(p) ==>
                        exists|jj: int| 0 <= jj < j as int && p.0 == (#[trigger] sorted@[jj]).0,
                    forall|p: (K::V, V::V)| #[trigger] right_tree@.contains(p) ==>
                        exists|jj: int| 0 <= jj < j as int && p.0 == (#[trigger] sorted@[jj]).0,
                    // Subset: values preserved from old_tree.
                    forall|p: (K::V, V::V)| #[trigger] left_tree@.contains(p) ==> old_tree.contains(p),
                    forall|p: (K::V, V::V)| #[trigger] right_tree@.contains(p) ==> old_tree.contains(p),
                    // Disjointness.
                    left_tree@.disjoint(right_tree@),
                    // Completeness: every processed entry is in left or right.
                    forall|j2: int| 0 <= j2 < j as int
                        ==> (#[trigger] left_tree@.contains(sorted@[j2]) || right_tree@.contains(sorted@[j2])),
                decreases size - j,
            {
                let elem = sorted.nth(j);
                let cloned = elem.clone_plus();
                proof { lemma_cloned_view_eq(*elem, cloned); }
                if j < split_at {
                    let ghost old_left = left_tree@;
                    proof {
                        // Freshness for left_tree.
                        assert(!spec_pair_set_to_map(old_left).dom().contains(sorted@[j as int].0)) by {
                            if spec_pair_set_to_map(old_left).dom().contains(sorted@[j as int].0) {
                                lemma_map_contains_pair_in_set(old_left, sorted@[j as int].0);
                                let vv: V::V = choose|vv: V::V| old_left.contains((sorted@[j as int].0, vv));
                                let jj = choose|jj: int| 0 <= jj < j as int && (sorted@[j as int].0, vv).0 == (#[trigger] sorted@[jj]).0;
                                assert(false);
                            }
                        };
                    }
                    proof {
                        // left_tree@.len() <= j < size <= usize::MAX.
                        assert(old_left.len() < usize::MAX as nat);
                    }
                    left_tree.insert(cloned);
                    proof {
                        assert(left_tree@.len() <= j as nat + 1);
                        lemma_key_unique_insert(old_left, sorted@[j as int].0, sorted@[j as int].1);
                        // Subset: sorted@[j] is in old_tree.
                        assert(sorted@.contains(sorted@[j as int])) by { assert(sorted@[j as int] == sorted@[j as int]); };
                        assert(old_tree.contains(sorted@[j as int]));
                        // Disjointness: sorted@[j] not in right_tree.
                        assert(!right_tree@.contains(sorted@[j as int])) by {
                            if right_tree@.contains(sorted@[j as int]) {
                                let rj = choose|rj: int| 0 <= rj < j as int && sorted@[j as int].0 == (#[trigger] sorted@[rj]).0;
                                assert(false);
                            }
                        };
                    }
                } else {
                    let ghost old_right = right_tree@;
                    proof {
                        // Freshness for right_tree.
                        assert(!spec_pair_set_to_map(old_right).dom().contains(sorted@[j as int].0)) by {
                            if spec_pair_set_to_map(old_right).dom().contains(sorted@[j as int].0) {
                                lemma_map_contains_pair_in_set(old_right, sorted@[j as int].0);
                                let vv: V::V = choose|vv: V::V| old_right.contains((sorted@[j as int].0, vv));
                                let jj = choose|jj: int| 0 <= jj < j as int && (sorted@[j as int].0, vv).0 == (#[trigger] sorted@[jj]).0;
                                assert(false);
                            }
                        };
                        // right_tree@.len() <= j < size <= usize::MAX.
                        assert(old_right.len() < usize::MAX as nat);
                    }
                    right_tree.insert(cloned);
                    proof {
                        assert(right_tree@.len() <= j as nat + 1);
                        lemma_key_unique_insert(old_right, sorted@[j as int].0, sorted@[j as int].1);
                        // Subset: sorted@[j] is in old_tree.
                        assert(sorted@.contains(sorted@[j as int])) by { assert(sorted@[j as int] == sorted@[j as int]); };
                        assert(old_tree.contains(sorted@[j as int]));
                        // Disjointness: sorted@[j] not in left_tree.
                        assert(!left_tree@.contains(sorted@[j as int])) by {
                            if left_tree@.contains(sorted@[j as int]) {
                                let lj = choose|lj: int| 0 <= lj < j as int && sorted@[j as int].0 == (#[trigger] sorted@[lj]).0;
                                assert(false);
                            }
                        };
                    }
                }
                j += 1;
            }
            let left_table = OrderedTableStEph { tree: left_tree };
            let right_table = OrderedTableStEph { tree: right_tree };
            *self = Self::empty();
            proof {
                lemma_pair_set_to_map_dom_finite(old_tree);
                lemma_pair_set_to_map_dom_finite(left_tree@);
                lemma_pair_set_to_map_dom_finite(right_tree@);
                // Subset proofs.
                assert(left_table@.dom().subset_of(old_map.dom())) by {
                    assert forall|key: K::V| left_table@.dom().contains(key)
                        implies #[trigger] old_map.dom().contains(key)
                    by {
                        lemma_map_contains_pair_in_set(left_tree@, key);
                        let v: V::V = choose|v: V::V| left_tree@.contains((key, v));
                        assert(old_tree.contains((key, v)));
                        lemma_pair_in_set_map_contains(old_tree, key, v);
                    };
                };
                assert(right_table@.dom().subset_of(old_map.dom())) by {
                    assert forall|key: K::V| right_table@.dom().contains(key)
                        implies #[trigger] old_map.dom().contains(key)
                    by {
                        lemma_map_contains_pair_in_set(right_tree@, key);
                        let v: V::V = choose|v: V::V| right_tree@.contains((key, v));
                        assert(old_tree.contains((key, v)));
                        lemma_pair_in_set_map_contains(old_tree, key, v);
                    };
                };
                // Disjointness.
                assert(left_table@.dom().disjoint(right_table@.dom())) by {
                    assert forall|key: K::V|
                        !(left_table@.dom().contains(key) && #[trigger] right_table@.dom().contains(key))
                    by {
                        if left_table@.dom().contains(key) && right_table@.dom().contains(key) {
                            lemma_map_contains_pair_in_set(left_tree@, key);
                            lemma_map_contains_pair_in_set(right_tree@, key);
                            let lv: V::V = choose|v: V::V| left_tree@.contains((key, v));
                            let rv: V::V = choose|v: V::V| right_tree@.contains((key, v));
                            assert(old_tree.contains((key, lv)));
                            assert(old_tree.contains((key, rv)));
                            assert(lv == rv);
                            assert(left_tree@.contains((key, lv)));
                            assert(right_tree@.contains((key, lv)));
                            assert(!left_tree@.disjoint(right_tree@));
                            assert(false);
                        }
                    };
                };
                // Completeness: every key in old domain is in left or right.
                assert forall|key: K::V| #[trigger] old_map.dom().contains(key)
                    implies left_table@.dom().contains(key) || right_table@.dom().contains(key)
                by {
                    lemma_map_contains_pair_in_set(old_tree, key);
                    let v: V::V = choose|v: V::V| old_tree.contains((key, v));
                    assert(sorted@.contains((key, v)));
                    let j2 = choose|j2: int| 0 <= j2 < sorted@.len() && sorted@[j2] == (key, v);
                    assert(left_tree@.contains(sorted@[j2]) || right_tree@.contains(sorted@[j2]));
                    if left_tree@.contains(sorted@[j2]) {
                        lemma_pair_in_set_map_contains(left_tree@, key, v);
                    } else {
                        assert(right_tree@.contains(sorted@[j2]));
                        lemma_pair_in_set_map_contains(right_tree@, key, v);
                    }
                };
                // WF proofs.
                vstd::set_lib::lemma_len_subset(left_tree@, old_tree);
                vstd::set_lib::lemma_len_subset(right_tree@, old_tree);
                assert(spec_pair_key_determines_order::<K, V>());
                assert(vstd::laws_cmp::obeys_cmp_spec::<K>());
                assert(view_ord_consistent::<K>());
                assert(obeys_feq_fulls::<K, V>());
            }
            (left_table, right_table)
        }
    }

    // 10. iterators

    impl<K: StT + Ord, V: StT + Ord> OrderedTableStEph<K, V> {
        /// Returns an iterator over the table entries via in-order traversal.
        pub fn iter(&self) -> (it: OrderedTableStEphIter<K, V>)
            requires
                self.spec_orderedtablesteph_wf(),
            ensures
                it@.0 == 0,
                it@.1.len() == self.tree@.len(),
                iter_invariant(&it),
        {
            let sorted = self.tree.in_order();
            OrderedTableStEphIter { inner: sorted.seq.into_iter() }
        }
    }

    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct OrderedTableStEphIter<K: StT + Ord, V: StT + Ord> {
        pub inner: IntoIter<Pair<K, V>>,
    }

    impl<K: StT + Ord, V: StT + Ord> View for OrderedTableStEphIter<K, V> {
        type V = (int, Seq<Pair<K, V>>);
        open spec fn view(&self) -> (int, Seq<Pair<K, V>>) { self.inner@ }
    }

    pub open spec fn iter_invariant<K: StT + Ord, V: StT + Ord>(it: &OrderedTableStEphIter<K, V>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<K: StT + Ord, V: StT + Ord> std::iter::Iterator for OrderedTableStEphIter<K, V> {
        type Item = Pair<K, V>;

        fn next(&mut self) -> (next: Option<Pair<K, V>>)
            ensures ({
                let (old_index, old_seq) = old(self)@;
                match next {
                    None => {
                        &&& self@ == old(self)@
                        &&& old_index >= old_seq.len()
                    },
                    Some(element) => {
                        let (new_index, new_seq) = self@;
                        &&& 0 <= old_index < old_seq.len()
                        &&& new_seq == old_seq
                        &&& new_index == old_index + 1
                        &&& element == old_seq[old_index]
                    },
                }
            })
        {
            self.inner.next()
        }
    }

    /// Ghost iterator for ForLoopGhostIterator support.
    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct OrderedTableStEphGhostIterator<K: StT + Ord, V: StT + Ord> {
        pub pos: int,
        pub elements: Seq<Pair<K, V>>,
    }

    impl<K: StT + Ord, V: StT + Ord> View for OrderedTableStEphGhostIterator<K, V> {
        type V = Seq<Pair<K, V>>;
        open spec fn view(&self) -> Seq<Pair<K, V>> { self.elements.take(self.pos) }
    }

    impl<K: StT + Ord, V: StT + Ord> vstd::pervasive::ForLoopGhostIteratorNew for OrderedTableStEphIter<K, V> {
        type GhostIter = OrderedTableStEphGhostIterator<K, V>;
        open spec fn ghost_iter(&self) -> OrderedTableStEphGhostIterator<K, V> {
            OrderedTableStEphGhostIterator { pos: self@.0, elements: self@.1 }
        }
    }

    impl<K: StT + Ord, V: StT + Ord> vstd::pervasive::ForLoopGhostIterator for OrderedTableStEphGhostIterator<K, V> {
        type ExecIter = OrderedTableStEphIter<K, V>;
        type Item = Pair<K, V>;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &OrderedTableStEphIter<K, V>) -> bool {
            &&& self.pos == exec_iter@.0
            &&& self.elements == exec_iter@.1
        }

        open spec fn ghost_invariant(&self, init: Option<&Self>) -> bool {
            init matches Some(init) ==> {
                &&& init.pos == 0
                &&& init.elements == self.elements
                &&& 0 <= self.pos <= self.elements.len()
            }
        }

        open spec fn ghost_ensures(&self) -> bool {
            self.pos == self.elements.len()
        }

        open spec fn ghost_decrease(&self) -> Option<int> {
            Some(self.elements.len() - self.pos)
        }

        open spec fn ghost_peek_next(&self) -> Option<Pair<K, V>> {
            if 0 <= self.pos < self.elements.len() { Some(self.elements[self.pos]) } else { None }
        }

        open spec fn ghost_advance(&self, _exec_iter: &OrderedTableStEphIter<K, V>) -> OrderedTableStEphGhostIterator<K, V> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, K: StT + Ord, V: StT + Ord> std::iter::IntoIterator for &'a OrderedTableStEph<K, V> {
        type Item = Pair<K, V>;
        type IntoIter = OrderedTableStEphIter<K, V>;
        fn into_iter(self) -> (it: Self::IntoIter)
            requires
                self.spec_orderedtablesteph_wf(),
            ensures
                it@.0 == 0,
                it@.1.len() == self.tree@.len(),
                iter_invariant(&it),
        {
            self.iter()
        }
    }

    // 11. derive impls in verus!

    impl<K: StT + Ord, V: StT + Ord> Clone for OrderedTableStEph<K, V> {
        fn clone(&self) -> (cloned: Self) {
            OrderedTableStEph {
                tree: self.tree.clone(),
            }
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- n BST inserts from sorted entries
    #[verifier::loop_isolation(false)]
    pub fn from_sorted_entries<K: StT + Ord, V: StT + Ord>(
        entries: AVLTreeSeqStPerS<Pair<K, V>>,
    ) -> (result: OrderedTableStEph<K, V>)
        requires
            entries.spec_avltreeseqstper_wf(),
            obeys_feq_clone::<Pair<K, V>>(),
            obeys_feq_full::<Pair<K, V>>(),
            vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
            view_ord_consistent::<Pair<K, V>>(),
            spec_pair_key_determines_order::<K, V>(),
            vstd::laws_cmp::obeys_cmp_spec::<K>(),
            view_ord_consistent::<K>(),
            obeys_feq_fulls::<K, V>(),
            entries@.len() < usize::MAX as nat,
            // Entries must have unique keys.
            forall|ii: int, jj: int| 0 <= ii < jj < entries@.len()
                ==> (#[trigger] entries@[ii]).0 != (#[trigger] entries@[jj]).0,
        ensures
            result@.dom().finite(),
            result.spec_orderedtablesteph_wf(),
    {
        proof {
            assert(obeys_feq_full_trigger::<K>());
            assert(obeys_feq_full_trigger::<V>());
            assert(obeys_feq_full_trigger::<Pair<K, V>>());
            lemma_key_unique_empty::<K::V, V::V>();
        }
        let len = entries.length();
        let mut tree = ParamBST::<Pair<K, V>>::new();
        let mut i: usize = 0;
        while i < len
            invariant
                i <= len,
                len as nat == entries@.len(),
                entries@.len() < usize::MAX as nat,
                entries.spec_avltreeseqstper_wf(),
                tree.spec_bstparasteph_wf(),
                vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                view_ord_consistent::<Pair<K, V>>(),
                tree@.len() <= i as nat,
                tree@.len() < usize::MAX as nat,
                spec_key_unique_pairs_set(tree@),
                // Provenance: every element in the tree came from entries[0..i].
                forall|kv: K::V, vv: V::V| #[trigger] tree@.contains((kv, vv)) ==>
                    exists|j: int| #![trigger entries@[j]] 0 <= j < i as int && entries@[j] == (kv, vv),
                spec_set_pair_view_generated::<K, V>(tree@),
                // Entries have unique keys (from requires).
                forall|ii: int, jj: int| 0 <= ii < jj < entries@.len()
                    ==> (#[trigger] entries@[ii]).0 != (#[trigger] entries@[jj]).0,
            decreases len - i,
        {
            let ghost old_tree = tree@;
            let elem = entries.nth(i);
            let cloned = elem.clone_plus();
            proof { lemma_cloned_view_eq(*elem, cloned); }
            tree.insert(cloned);
            proof {
                assert(tree@.len() <= i as nat + 1);
                assert(i as nat + 1 <= len as nat);
                assert(tree@.len() < usize::MAX as nat);
                // Prove provenance for the new tree.
                assert forall|kv: K::V, vv: V::V| #[trigger] tree@.contains((kv, vv))
                    implies exists|j: int| #![trigger entries@[j]] 0 <= j < i as int + 1 && entries@[j] == (kv, vv) by {
                    if old_tree.contains((kv, vv)) {
                        let j = choose|j: int| #![trigger entries@[j]] 0 <= j < i as int && entries@[j] == (kv, vv);
                        assert(entries@[j] == (kv, vv) && j < i as int + 1);
                    } else {
                        // Must be the newly inserted element.
                        assert((kv, vv) == cloned@);
                        assert(entries@[i as int] == cloned@);
                    }
                };
                // Prove key uniqueness is maintained.
                assert(spec_key_unique_pairs_set(tree@)) by {
            
                    assert forall|k: K::V, v1: V::V, v2: V::V|
                        tree@.contains((k, v1)) && tree@.contains((k, v2)) implies v1 == v2 by {
                        if old_tree.contains((k, v1)) && old_tree.contains((k, v2)) {
                            // Both in old tree: follows from old invariant.
                        } else if !old_tree.contains((k, v1)) && !old_tree.contains((k, v2)) {
                            // Both are the new element.
                            assert((k, v1) == cloned@ && (k, v2) == cloned@);
                        } else {
                            // One old, one new: contradiction via unique keys.
                            if old_tree.contains((k, v1)) {
                                // (k, v2) == cloned@, so k == cloned@.0 == entries@[i].0.
                                let j1 = choose|j: int| #![trigger entries@[j]]
                                    0 <= j < i as int && entries@[j] == (k, v1);
                                // entries@[j1].0 == k == entries@[i].0, but j1 < i.
                                assert(entries@[j1].0 == entries@[i as int].0);
                                assert(j1 < i as int);
                                assert(false); // contradicts unique keys
                            } else {
                                // (k, v1) == cloned@
                                let j2 = choose|j: int| #![trigger entries@[j]]
                                    0 <= j < i as int && entries@[j] == (k, v2);
                                assert(entries@[j2].0 == entries@[i as int].0);
                                assert(j2 < i as int);
                                assert(false);
                            }
                        }
                    };
                };
            }
            i = i + 1;
        }
        let result = OrderedTableStEph { tree };
        proof { lemma_pair_set_to_map_dom_finite(tree@); }
        result
    }

    } // verus!

    // 13. derive impls outside verus!

    use std::fmt;

    impl<K: StT + Ord, V: StT + Ord> PartialEq for OrderedTableStEph<K, V> {
        fn eq(&self, other: &Self) -> bool {
            self.size() == other.size()
        }
    }

    impl<K: StT + Ord, V: StT + Ord> fmt::Debug for OrderedTableStEph<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedTableStEph(size: {})", self.size())
        }
    }

    impl<K: StT + Ord, V: StT + Ord> fmt::Display for OrderedTableStEph<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedTableStEph(size: {})", self.size())
        }
    }

    impl<K: StT + Ord, V: StT + Ord> fmt::Debug for OrderedTableStEphIter<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("OrderedTableStEphIter").finish()
        }
    }

    impl<K: StT + Ord, V: StT + Ord> fmt::Display for OrderedTableStEphIter<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedTableStEphIter")
        }
    }

    // 12. macros

    /// Macro for creating ephemeral ordered tables from sorted key-value pairs.
    #[macro_export]
    macro_rules! OrderedTableStEphLit {
        () => {
            $crate::Chap43::OrderedTableStEph::OrderedTableStEph::OrderedTableStEph::empty()
        };
        ($($key:expr => $val:expr),+ $(,)?) => {{
            let pairs = vec![$($crate::Types::Types::Pair($key, $val)),+];
            let seq = $crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::AVLTreeSeqStPerS::from_vec(pairs);
            $crate::Chap43::OrderedTableStEph::OrderedTableStEph::from_sorted_entries(seq)
        }};
    }
}
