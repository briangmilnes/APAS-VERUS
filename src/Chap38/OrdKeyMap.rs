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
                // (k, v1) and (k, v2) are in s1 union s2.
                // By key separation, both must be in the same set.
                if s1.contains((k, v1)) && s2.contains((k, v2)) {
                    assert(false); // key separation
                }
                if s2.contains((k, v1)) && s1.contains((k, v2)) {
                    assert(false); // key separation
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
            ensures empty@ == Map::<K::V, V::V>::empty();

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
                !parts.2@.contains_key(k@);

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
                    assert(tree@ =~= left@.union(right@).insert(root_pair@));
                    assert(tree@.len() == left@.len() + right@.len() + 1);
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
                            assert(k.cmp_spec(&root_pair.0) == Equal);
                            assert(k@ == root_pair.0@);
                            assert(tree@.contains(root_pair@));
                            assert(tree@.contains((k@, root_pair.1@)));
                            lemma_pair_in_set_map_contains(tree@, k@, root_pair.1@);
                        }
                        Some(v_clone)
                    },
                    Less => {
                        let result = ordkeymap_find(&left, k);
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
                                    let ghost p_wit: Pair<K, V> = choose|p: Pair<K, V>| p@ == (k@, vv);
                                    assert(right@.contains(p_wit@));
                                    lemma_cmp_equal_congruent(p_wit.0, *k, root_pair.0);
                                    assert(p_wit.0.cmp_spec(&root_pair.0) == Less);
                                    assert(false);
                                }
                            }
                        }
                        result
                    },
                    Greater => {
                        let result = ordkeymap_find(&right, k);
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
                    assert(tree@ =~= left@.union(right@).insert(root_pair@));
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
                            assert(k@ == root_pair.0@);
                            assert(tree@.contains(root_pair@));
                            lemma_pair_in_set_map_contains(tree@, root_pair.0@, root_pair.1@);
                            assert forall|p: Pair<K, V>| (#[trigger] left@.contains(p@))
                                implies p.0.cmp_spec(k) == Less by {
                                assert(p.cmp_spec(&root_pair) == Less);
                                assert(p.0@ != root_pair.0@) by {
                                    if p.0@ == root_pair.0@ {
                                        assert(tree@.contains(p@));
                                        assert(tree@.contains(root_pair@));
                                    }
                                };
                                assert(p.0.cmp_spec(&root_pair.0) == Less);
                                assert(root_pair.0 == *k);
                                lemma_cmp_equal_congruent(root_pair.0, *k, p.0);
                            };
                            assert forall|p: Pair<K, V>| (#[trigger] right@.contains(p@))
                                implies p.0.cmp_spec(k) == Greater by {
                                assert(p.cmp_spec(&root_pair) == Greater);
                                assert(p.0@ != root_pair.0@) by {
                                    if p.0@ == root_pair.0@ {
                                        assert(tree@.contains(p@));
                                        assert(tree@.contains(root_pair@));
                                    }
                                };
                                assert(p.0.cmp_spec(&root_pair.0) == Greater);
                                assert(root_pair.0 == *k);
                            };
                            assert(!spec_pair_set_to_map(left@).dom().contains(k@)) by {
                                if spec_pair_set_to_map(left@).dom().contains(k@) {
                                    lemma_map_contains_pair_in_set(left@, k@);
                                    let lv: V::V = choose|lv: V::V| left@.contains((k@, lv));
                                    let lp: Pair<K, V> = choose|lp: Pair<K, V>| lp@ == (k@, lv);
                                    assert(left@.contains(lp@));
                                    assert(lp.0.cmp_spec(k) == Less);
                                    assert(lp.0@ == k@);
                                    assert(lp.0 == *k);
                                    reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                                }
                            };
                            assert(!spec_pair_set_to_map(right@).dom().contains(k@)) by {
                                if spec_pair_set_to_map(right@).dom().contains(k@) {
                                    lemma_map_contains_pair_in_set(right@, k@);
                                    let rv: V::V = choose|rv: V::V| right@.contains((k@, rv));
                                    let rp: Pair<K, V> = choose|rp: Pair<K, V>| rp@ == (k@, rv);
                                    assert(right@.contains(rp@));
                                    assert(rp.0.cmp_spec(k) == Greater);
                                    assert(rp.0@ == k@);
                                    assert(rp.0 == *k);
                                    reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                                }
                            };
                        }
                        (left, Some(v), right)
                    },
                    Less => {
                        let (ll, found, lr) = ordkeymap_split(&left, k);
                        proof {
                            assert forall|t: Pair<K, V>| (#[trigger] lr@.contains(t@))
                                implies t.cmp_spec(&root_pair) == Less by {
                                assert(left@.contains(t@));
                            };
                            assert forall|t: Pair<K, V>| (#[trigger] right@.contains(t@))
                                implies t.cmp_spec(&root_pair) == Greater by {};
                            assert(!lr@.contains(root_pair@));
                            assert(!right@.contains(root_pair@));
                            assert(lr@.disjoint(right@)) by {
                                assert forall|v: <Pair<K,V> as View>::V|
                                    !(lr@.contains(v) && #[trigger] right@.contains(v)) by {
                                    if lr@.contains(v) && right@.contains(v) {
                                        assert(left@.contains(v));
                                    }
                                };
                            };
                            assert(lr@.len() + right@.len() < usize::MAX as nat) by {
                                vstd::set_lib::lemma_len_subset(lr@, left@);
                            };
                        }
                        let new_right = ParamBST::join_m(lr, root_pair, right);
                        proof {
                            assert(ll@.subset_of(tree@)) by {
                                assert forall|v: <Pair<K,V> as View>::V| ll@.contains(v)
                                    implies #[trigger] tree@.contains(v) by {
                                    assert(left@.contains(v));
                                };
                            };
                            assert(new_right@ =~= lr@.union(right@).insert(root_pair@));
                            assert(new_right@.subset_of(tree@)) by {
                                assert forall|v: <Pair<K,V> as View>::V| new_right@.contains(v)
                                    implies #[trigger] tree@.contains(v) by {
                                    if lr@.contains(v) { assert(left@.contains(v)); }
                                    else if right@.contains(v) {}
                                    else { assert(v == root_pair@); }
                                };
                            };
                            assert(ll@.disjoint(new_right@)) by {
                                assert forall|v: <Pair<K,V> as View>::V|
                                    !(ll@.contains(v) && #[trigger] new_right@.contains(v)) by {
                                    if ll@.contains(v) && new_right@.contains(v) {
                                        if lr@.contains(v) {
                                            assert(ll@.disjoint(lr@));
                                        } else if right@.contains(v) {
                                            assert(left@.contains(v));
                                        } else {
                                            assert(v == root_pair@);
                                            assert(left@.contains(v));
                                            assert(!left@.contains(root_pair@));
                                        }
                                    }
                                };
                            };
                            assert forall|kv: <K as View>::V, vv: <V as View>::V|
                                #[trigger] tree@.contains((kv, vv))
                                implies ll@.contains((kv, vv)) || new_right@.contains((kv, vv)) || kv == k@ by {
                                if left@.contains((kv, vv)) {
                                    if !ll@.contains((kv, vv)) && kv != k@ {
                                        assert(lr@.contains((kv, vv)));
                                    }
                                } else if (kv, vv) == root_pair@ {
                                } else {
                                    assert(right@.contains((kv, vv)));
                                }
                            };
                            assert forall|p: Pair<K, V>| (#[trigger] new_right@.contains(p@))
                                implies p.0.cmp_spec(k) == Greater by {
                                if lr@.contains(p@) {}
                                else if right@.contains(p@) {
                                    assert(p.cmp_spec(&root_pair) == Greater);
                                    assert(p.0@ != root_pair.0@) by {
                                        if p.0@ == root_pair.0@ {
                                            assert(tree@.contains(p@));
                                            assert(tree@.contains(root_pair@));
                                        }
                                    };
                                    assert(p.0.cmp_spec(&root_pair.0) == Greater);
                                    assert(k.cmp_spec(&root_pair.0) == Less);
                                    lemma_cmp_antisymmetry(*k, root_pair.0);
                                    reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                                } else {
                                    assert(p@ == root_pair@);
                                    assert(p.0@ == root_pair.0@);
                                    assert(p.0 == root_pair.0);
                                    lemma_cmp_antisymmetry(*k, root_pair.0);
                                }
                            };
                            assert(!spec_pair_set_to_map(new_right@).dom().contains(k@)) by {
                                if spec_pair_set_to_map(new_right@).dom().contains(k@) {
                                    lemma_map_contains_pair_in_set(new_right@, k@);
                                    let nv: V::V = choose|nv: V::V| new_right@.contains((k@, nv));
                                    let np: Pair<K, V> = choose|np: Pair<K, V>| np@ == (k@, nv);
                                    assert(new_right@.contains(np@));
                                    assert(np.0.cmp_spec(k) == Greater);
                                    assert(np.0 == *k);
                                    reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                                }
                            };
                            lemma_key_unique_subset(tree@, new_right@);
                            lemma_view_gen_subset::<K, V>(new_right@, tree@);
                            vstd::set_lib::lemma_set_disjoint_lens(lr@, right@);
                            assert(new_right@.len() == lr@.len() + right@.len() + 1) by {
                                let lr_r = lr@.union(right@);
                                vstd::set_lib::lemma_set_disjoint_lens(lr@, right@);
                                assert(lr_r.len() == lr@.len() + right@.len());
                                assert(!lr_r.contains(root_pair@)) by {
                                    if lr_r.contains(root_pair@) {
                                        if lr@.contains(root_pair@) { assert(left@.contains(root_pair@)); }
                                    }
                                };
                                assert(lr_r.insert(root_pair@).len() == lr_r.len() + 1) by {
                                    assert(lr_r.insert(root_pair@) =~= lr_r.union(Set::empty().insert(root_pair@)));
                                    let singleton = Set::empty().insert(root_pair@);
                                    assert(lr_r.disjoint(singleton));
                                    vstd::set_lib::lemma_set_disjoint_lens(lr_r, singleton);
                                };
                            };
                            if found is Some {
                                let fv = found->Some_0;
                                lemma_map_contains_pair_in_set(left@, k@);
                                let lv: V::V = choose|lv: V::V| left@.contains((k@, lv));
                                assert(tree@.contains((k@, lv)));
                                lemma_pair_in_set_map_contains(tree@, k@, lv);
                                lemma_pair_in_set_map_contains(left@, k@, lv);
                            }
                        }
                        (ll, found, new_right)
                    },
                    Greater => {
                        let (rl, found, rr) = ordkeymap_split(&right, k);
                        proof {
                            assert forall|t: Pair<K, V>| (#[trigger] left@.contains(t@))
                                implies t.cmp_spec(&root_pair) == Less by {};
                            assert forall|t: Pair<K, V>| (#[trigger] rl@.contains(t@))
                                implies t.cmp_spec(&root_pair) == Greater by {
                                assert(right@.contains(t@));
                            };
                            assert(!left@.contains(root_pair@));
                            assert(!rl@.contains(root_pair@));
                            assert(left@.disjoint(rl@)) by {
                                assert forall|v: <Pair<K,V> as View>::V|
                                    !(left@.contains(v) && #[trigger] rl@.contains(v)) by {
                                    if left@.contains(v) && rl@.contains(v) {
                                        assert(right@.contains(v));
                                    }
                                };
                            };
                            assert(left@.len() + rl@.len() < usize::MAX as nat) by {
                                vstd::set_lib::lemma_len_subset(rl@, right@);
                            };
                        }
                        let new_left = ParamBST::join_m(left, root_pair, rl);
                        proof {
                            assert(new_left@ =~= left@.union(rl@).insert(root_pair@));
                            assert(new_left@.subset_of(tree@)) by {
                                assert forall|v: <Pair<K,V> as View>::V| new_left@.contains(v)
                                    implies #[trigger] tree@.contains(v) by {
                                    if left@.contains(v) {}
                                    else if rl@.contains(v) { assert(right@.contains(v)); }
                                    else { assert(v == root_pair@); }
                                };
                            };
                            assert(rr@.subset_of(tree@)) by {
                                assert forall|v: <Pair<K,V> as View>::V| rr@.contains(v)
                                    implies #[trigger] tree@.contains(v) by {
                                    assert(right@.contains(v));
                                };
                            };
                            assert(new_left@.disjoint(rr@)) by {
                                assert forall|v: <Pair<K,V> as View>::V|
                                    !(new_left@.contains(v) && #[trigger] rr@.contains(v)) by {
                                    if new_left@.contains(v) && rr@.contains(v) {
                                        if left@.contains(v) {
                                            assert(right@.contains(v));
                                        } else if rl@.contains(v) {
                                            assert(rl@.disjoint(rr@));
                                        } else {
                                            assert(v == root_pair@);
                                            assert(right@.contains(v));
                                            assert(!right@.contains(root_pair@));
                                        }
                                    }
                                };
                            };
                            reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                            assert(root_pair.0.cmp_spec(k) == Less);
                            assert forall|p: Pair<K, V>| (#[trigger] new_left@.contains(p@))
                                implies p.0.cmp_spec(k) == Less by {
                                if left@.contains(p@) {
                                    assert(p.cmp_spec(&root_pair) == Less);
                                    assert(p.0@ != root_pair.0@) by {
                                        if p.0@ == root_pair.0@ {
                                            assert(tree@.contains(p@));
                                            assert(tree@.contains(root_pair@));
                                        }
                                    };
                                    assert(p.0.cmp_spec(&root_pair.0) == Less);
                                } else if rl@.contains(p@) {
                                } else {
                                    assert(p@ == root_pair@);
                                    assert(p.0 == root_pair.0);
                                }
                            };
                            assert(!spec_pair_set_to_map(new_left@).dom().contains(k@)) by {
                                if spec_pair_set_to_map(new_left@).dom().contains(k@) {
                                    lemma_map_contains_pair_in_set(new_left@, k@);
                                    let nv: V::V = choose|nv: V::V| new_left@.contains((k@, nv));
                                    let np: Pair<K, V> = choose|np: Pair<K, V>| np@ == (k@, nv);
                                    assert(new_left@.contains(np@));
                                    assert(np.0.cmp_spec(k) == Less);
                                    assert(np.0 == *k);
                                    reveal(vstd::laws_cmp::obeys_partial_cmp_spec_properties);
                                }
                            };
                            assert forall|kv: <K as View>::V, vv: <V as View>::V|
                                #[trigger] tree@.contains((kv, vv))
                                implies new_left@.contains((kv, vv)) || rr@.contains((kv, vv)) || kv == k@ by {
                                if right@.contains((kv, vv)) {
                                    if !rl@.contains((kv, vv)) && kv != k@ {
                                        assert(rr@.contains((kv, vv)));
                                    }
                                } else if (kv, vv) == root_pair@ {
                                } else {
                                    assert(left@.contains((kv, vv)));
                                }
                            };
                            lemma_key_unique_subset(tree@, new_left@);
                            lemma_view_gen_subset::<K, V>(new_left@, tree@);
                            vstd::set_lib::lemma_set_disjoint_lens(left@, rl@);
                            assert(new_left@.len() == left@.len() + rl@.len() + 1) by {
                                let l_rl = left@.union(rl@);
                                vstd::set_lib::lemma_set_disjoint_lens(left@, rl@);
                                assert(l_rl.len() == left@.len() + rl@.len());
                                assert(!l_rl.contains(root_pair@)) by {
                                    if l_rl.contains(root_pair@) {
                                        if rl@.contains(root_pair@) { assert(right@.contains(root_pair@)); }
                                    }
                                };
                                assert(l_rl.insert(root_pair@).len() == l_rl.len() + 1) by {
                                    assert(l_rl.insert(root_pair@) =~= l_rl.union(Set::empty().insert(root_pair@)));
                                    let singleton = Set::empty().insert(root_pair@);
                                    assert(l_rl.disjoint(singleton));
                                    vstd::set_lib::lemma_set_disjoint_lens(l_rl, singleton);
                                };
                            };
                            if found is Some {
                                let fv = found->Some_0;
                                lemma_map_contains_pair_in_set(right@, k@);
                                let rv: V::V = choose|rv: V::V| right@.contains((k@, rv));
                                assert(tree@.contains((k@, rv)));
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
                    assert(tree@ =~= left@.union(right@).insert(root_pair@));
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
                        proof { assert(k.cmp_spec(&root_pair.0) == Less); }
                        let left_result = ordkeymap_next(&left, k);
                        match left_result {
                            Some(lk) => {
                                proof {
                                    lemma_map_contains_pair_in_set(left@, lk@);
                                    let vv: V::V = choose|vv: V::V| left@.contains((lk@, vv));
                                    assert(tree@.contains((lk@, vv)));
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
                                            assert(lp.cmp_spec(&root_pair) == Less);
                                            assert(lp.0@ != root_pair.0@) by {
                                                if lp.0@ == root_pair.0@ {
                                                    assert(tree@.contains(lp@));
                                                    assert(tree@.contains(root_pair@));
                                                }
                                            };
                                            assert(lp.0.cmp_spec(&root_pair.0) == Less);
                                            assert(lp.0 == lk);
                                            assert(root_pair.0 == t);
                                            K::cmp_spec_less_implies_le(lk, t);
                                        } else {
                                            assert(right@.contains((t@, tv)));
                                            let lp: Pair<K, V> = choose|lp: Pair<K, V>| #[trigger] left@.contains(lp@) && lp@ == (lk@, vv);
                                            let tp: Pair<K, V> = choose|tp: Pair<K, V>| #[trigger] right@.contains(tp@) && tp@ == (t@, tv);
                                            assert(lp.cmp_spec(&root_pair) == Less);
                                            assert(tp.cmp_spec(&root_pair) == Greater);
                                            assert(lp.0@ != root_pair.0@) by {
                                                if lp.0@ == root_pair.0@ {
                                                    assert(tree@.contains(lp@));
                                                    assert(tree@.contains(root_pair@));
                                                }
                                            };
                                            assert(tp.0@ != root_pair.0@) by {
                                                if tp.0@ == root_pair.0@ {
                                                    assert(tree@.contains(tp@));
                                                    assert(tree@.contains(root_pair@));
                                                }
                                            };
                                            assert(lp.0.cmp_spec(&root_pair.0) == Less);
                                            assert(tp.0.cmp_spec(&root_pair.0) == Greater);
                                            assert(lp.0 == lk);
                                            assert(tp.0 == t);
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
                                    assert(key == root_pair.0);
                                    assert(tree@.contains(root_pair@));
                                    lemma_pair_in_set_map_contains(tree@, root_pair.0@, root_pair.1@);
                                    K::cmp_spec_less_implies_le(*k, root_pair.0);
                                    assert(k@ != root_pair.0@);
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
                                            assert(right@.contains((t@, tv)));
                                            let tp: Pair<K, V> = choose|tp: Pair<K, V>| #[trigger] right@.contains(tp@) && tp@ == (t@, tv);
                                            assert(tp.cmp_spec(&root_pair) == Greater);
                                            assert(tp.0@ != root_pair.0@) by {
                                                if tp.0@ == root_pair.0@ {
                                                    assert(tree@.contains(tp@));
                                                    assert(tree@.contains(root_pair@));
                                                }
                                            };
                                            assert(tp.0.cmp_spec(&root_pair.0) == Greater);
                                            assert(tp.0 == t);
                                            K::cmp_spec_greater_implies_le(t, root_pair.0);
                                        }
                                    };
                                }
                                Some(key)
                            },
                        }
                    },
                    Equal => {
                        proof { assert(k.cmp_spec(&root_pair.0) == Equal); assert(k@ == root_pair.0@); }
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
                                            assert(tp.cmp_spec(&root_pair) == Less);
                                            assert(tp.0@ != root_pair.0@) by {
                                                if tp.0@ == root_pair.0@ {
                                                    assert(tree@.contains(tp@));
                                                    assert(tree@.contains(root_pair@));
                                                }
                                            };
                                            assert(tp.0.cmp_spec(&root_pair.0) == Less);
                                            assert(tp.0 == t);
                                            K::cmp_spec_less_implies_le(t, root_pair.0);
                                            assert(TotalOrder::le(t, root_pair.0));
                                            assert(root_pair.0 == *k);
                                            K::antisymmetric(t, *k);
                                        } else if (t@, tv) == root_pair@ {
                                        } else {
                                            assert(right@.contains((t@, tv)));
                                        }
                                    };
                                }
                                None
                            },
                            Some(min_pair) => {
                                let key = min_pair.0.clone_plus();
                                proof {
                                    lemma_cloned_view_eq(min_pair.0, key);
                                    assert(key == min_pair.0);
                                    assert(right@.contains(min_pair@));
                                    assert(tree@.contains(min_pair@));
                                    lemma_pair_in_set_map_contains(tree@, min_pair.0@, min_pair.1@);
                                    assert(min_pair.0@ != root_pair.0@) by {
                                        if min_pair.0@ == root_pair.0@ {
                                            assert(tree@.contains(min_pair@));
                                            assert(tree@.contains(root_pair@));
                                        }
                                    };
                                    assert(min_pair.cmp_spec(&root_pair) == Greater);
                                    assert(min_pair.0.cmp_spec(&root_pair.0) == Greater);
                                    K::cmp_spec_greater_implies_le(min_pair.0, root_pair.0);
                                    assert(root_pair.0 == *k);
                                    assert(key@ != k@);
                                    assert forall|t: K| #![trigger t@]
                                        spec_pair_set_to_map(tree@).dom().contains(t@)
                                        && TotalOrder::le(*k, t) && t@ != k@
                                        implies TotalOrder::le(key, t) by {
                                        lemma_map_contains_pair_in_set(tree@, t@);
                                        let tv: V::V = choose|tv: V::V| tree@.contains((t@, tv));
                                        if left@.contains((t@, tv)) {
                                            let tp: Pair<K, V> = choose|tp: Pair<K, V>| #[trigger] left@.contains(tp@) && tp@ == (t@, tv);
                                            assert(tp.cmp_spec(&root_pair) == Less);
                                            assert(tp.0@ != root_pair.0@) by {
                                                if tp.0@ == root_pair.0@ {
                                                    assert(tree@.contains(tp@));
                                                    assert(tree@.contains(root_pair@));
                                                }
                                            };
                                            assert(tp.0.cmp_spec(&root_pair.0) == Less);
                                            assert(tp.0 == t);
                                            K::cmp_spec_less_implies_le(t, root_pair.0);
                                            assert(root_pair.0 == *k);
                                            K::antisymmetric(t, *k);
                                        } else if (t@, tv) == root_pair@ {
                                        } else {
                                            assert(right@.contains((t@, tv)));
                                            let tp: Pair<K, V> = choose|tp: Pair<K, V>| #[trigger] right@.contains(tp@) && tp@ == (t@, tv);
                                            if min_pair@ == tp@ {
                                                K::reflexive(key);
                                            } else {
                                                assert(min_pair.cmp_spec(&tp) == Less);
                                                assert(min_pair.0@ != tp.0@) by {
                                                    if min_pair.0@ == tp.0@ {
                                                        assert(tree@.contains(min_pair@));
                                                        assert(tree@.contains(tp@));
                                                    }
                                                };
                                                assert(min_pair.0.cmp_spec(&tp.0) == Less);
                                                assert(tp.0 == t);
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
                        proof { assert(k.cmp_spec(&root_pair.0) == Greater); }
                        let result = ordkeymap_next(&right, k);
                        proof {
                            if result is Some {
                                let rk = result->Some_0;
                                lemma_map_contains_pair_in_set(right@, rk@);
                                let rv: V::V = choose|rv: V::V| right@.contains((rk@, rv));
                                assert(tree@.contains((rk@, rv)));
                                lemma_pair_in_set_map_contains(tree@, rk@, rv);
                                assert forall|t: K| #![trigger t@]
                                    spec_pair_set_to_map(tree@).dom().contains(t@)
                                    && TotalOrder::le(*k, t) && t@ != k@
                                    implies TotalOrder::le(rk, t) by {
                                    lemma_map_contains_pair_in_set(tree@, t@);
                                    let tv: V::V = choose|tv: V::V| tree@.contains((t@, tv));
                                    if left@.contains((t@, tv)) {
                                        let tp: Pair<K, V> = choose|tp: Pair<K, V>| #[trigger] left@.contains(tp@) && tp@ == (t@, tv);
                                        assert(tp.cmp_spec(&root_pair) == Less);
                                        assert(tp.0@ != root_pair.0@) by {
                                            if tp.0@ == root_pair.0@ {
                                                assert(tree@.contains(tp@));
                                                assert(tree@.contains(root_pair@));
                                            }
                                        };
                                        assert(tp.0.cmp_spec(&root_pair.0) == Less);
                                        assert(tp.0 == t);
                                        K::cmp_spec_less_implies_le(t, root_pair.0);
                                        K::cmp_spec_greater_implies_le(*k, root_pair.0);
                                        K::transitive(t, root_pair.0, *k);
                                        K::antisymmetric(t, *k);
                                    } else if (t@, tv) == root_pair@ {
                                        assert(root_pair.0 == t);
                                        K::cmp_spec_greater_implies_le(*k, root_pair.0);
                                        K::antisymmetric(t, *k);
                                    } else {
                                        assert(right@.contains((t@, tv)));
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
                                        assert(tp.cmp_spec(&root_pair) == Less);
                                        assert(tp.0@ != root_pair.0@) by {
                                            if tp.0@ == root_pair.0@ {
                                                assert(tree@.contains(tp@));
                                                assert(tree@.contains(root_pair@));
                                            }
                                        };
                                        assert(tp.0.cmp_spec(&root_pair.0) == Less);
                                        assert(tp.0 == t);
                                        K::cmp_spec_less_implies_le(t, root_pair.0);
                                        K::cmp_spec_greater_implies_le(*k, root_pair.0);
                                        K::transitive(t, root_pair.0, *k);
                                        if TotalOrder::le(*k, t) && t@ != k@ {
                                            K::antisymmetric(t, *k);
                                        }
                                    } else if (t@, tv) == root_pair@ {
                                        assert(root_pair.0 == t);
                                        K::cmp_spec_greater_implies_le(*k, root_pair.0);
                                        if TotalOrder::le(*k, t) && t@ != k@ {
                                            K::antisymmetric(t, *k);
                                        }
                                    } else {
                                        assert(right@.contains((t@, tv)));
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
                    assert(tree@ =~= left@.union(right@).insert(root_pair@));
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
                                    assert(tree@.contains((rk@, rv)));
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
                                            assert(rp.cmp_spec(&root_pair) == Greater);
                                            assert(rp.0@ != root_pair.0@) by {
                                                if rp.0@ == root_pair.0@ {
                                                    assert(tree@.contains(rp@));
                                                    assert(tree@.contains(root_pair@));
                                                }
                                            };
                                            assert(rp.0.cmp_spec(&root_pair.0) == Greater);
                                            assert(rp.0 == rk);
                                            assert(root_pair.0 == t);
                                            K::cmp_spec_greater_implies_le(rk, root_pair.0);
                                        } else {
                                            assert(left@.contains((t@, tv)));
                                            let tp: Pair<K, V> = choose|tp: Pair<K, V>| #[trigger] left@.contains(tp@) && tp@ == (t@, tv);
                                            let rp: Pair<K, V> = choose|rp: Pair<K, V>| #[trigger] right@.contains(rp@) && rp@ == (rk@, rv);
                                            assert(tp.cmp_spec(&root_pair) == Less);
                                            assert(rp.cmp_spec(&root_pair) == Greater);
                                            assert(tp.0@ != root_pair.0@) by {
                                                if tp.0@ == root_pair.0@ {
                                                    assert(tree@.contains(tp@));
                                                    assert(tree@.contains(root_pair@));
                                                }
                                            };
                                            assert(rp.0@ != root_pair.0@) by {
                                                if rp.0@ == root_pair.0@ {
                                                    assert(tree@.contains(rp@));
                                                    assert(tree@.contains(root_pair@));
                                                }
                                            };
                                            assert(tp.0.cmp_spec(&root_pair.0) == Less);
                                            assert(rp.0.cmp_spec(&root_pair.0) == Greater);
                                            assert(tp.0 == t);
                                            assert(rp.0 == rk);
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
                                    assert(key == root_pair.0);
                                    assert(tree@.contains(root_pair@));
                                    lemma_pair_in_set_map_contains(tree@, root_pair.0@, root_pair.1@);
                                    K::cmp_spec_greater_implies_le(*k, root_pair.0);
                                    assert(k@ != root_pair.0@);
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
                                            assert(left@.contains((t@, tv)));
                                            let tp: Pair<K, V> = choose|tp: Pair<K, V>| #[trigger] left@.contains(tp@) && tp@ == (t@, tv);
                                            assert(tp.cmp_spec(&root_pair) == Less);
                                            assert(tp.0@ != root_pair.0@) by {
                                                if tp.0@ == root_pair.0@ {
                                                    assert(tree@.contains(tp@));
                                                    assert(tree@.contains(root_pair@));
                                                }
                                            };
                                            assert(tp.0.cmp_spec(&root_pair.0) == Less);
                                            assert(tp.0 == t);
                                            K::cmp_spec_less_implies_le(t, root_pair.0);
                                        }
                                    };
                                }
                                Some(key)
                            },
                        }
                    },
                    Equal => {
                        proof { assert(k@ == root_pair.0@); }
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
                                            assert(tp.cmp_spec(&root_pair) == Greater);
                                            assert(tp.0@ != root_pair.0@) by {
                                                if tp.0@ == root_pair.0@ {
                                                    assert(tree@.contains(tp@));
                                                    assert(tree@.contains(root_pair@));
                                                }
                                            };
                                            assert(tp.0.cmp_spec(&root_pair.0) == Greater);
                                            assert(tp.0 == t);
                                            K::cmp_spec_greater_implies_le(t, root_pair.0);
                                            assert(root_pair.0 == *k);
                                            K::antisymmetric(*k, t);
                                        } else if (t@, tv) == root_pair@ {
                                        } else {
                                            assert(left@.contains((t@, tv)));
                                        }
                                    };
                                }
                                None
                            },
                            Some(max_pair) => {
                                let key = max_pair.0.clone_plus();
                                proof {
                                    lemma_cloned_view_eq(max_pair.0, key);
                                    assert(key == max_pair.0);
                                    assert(left@.contains(max_pair@));
                                    assert(tree@.contains(max_pair@));
                                    lemma_pair_in_set_map_contains(tree@, max_pair.0@, max_pair.1@);
                                    assert(max_pair.0@ != root_pair.0@) by {
                                        if max_pair.0@ == root_pair.0@ {
                                            assert(tree@.contains(max_pair@));
                                            assert(tree@.contains(root_pair@));
                                        }
                                    };
                                    assert(max_pair.cmp_spec(&root_pair) == Less);
                                    assert(max_pair.0.cmp_spec(&root_pair.0) == Less);
                                    K::cmp_spec_less_implies_le(max_pair.0, root_pair.0);
                                    assert(root_pair.0 == *k);
                                    assert(key@ != k@);
                                    assert forall|t: K| #![trigger t@]
                                        spec_pair_set_to_map(tree@).dom().contains(t@)
                                        && TotalOrder::le(t, *k) && t@ != k@
                                        implies TotalOrder::le(t, key) by {
                                        lemma_map_contains_pair_in_set(tree@, t@);
                                        let tv: V::V = choose|tv: V::V| tree@.contains((t@, tv));
                                        if right@.contains((t@, tv)) {
                                            let tp: Pair<K, V> = choose|tp: Pair<K, V>| #[trigger] right@.contains(tp@) && tp@ == (t@, tv);
                                            assert(tp.cmp_spec(&root_pair) == Greater);
                                            assert(tp.0@ != root_pair.0@) by {
                                                if tp.0@ == root_pair.0@ {
                                                    assert(tree@.contains(tp@));
                                                    assert(tree@.contains(root_pair@));
                                                }
                                            };
                                            assert(tp.0.cmp_spec(&root_pair.0) == Greater);
                                            assert(tp.0 == t);
                                            K::cmp_spec_greater_implies_le(t, root_pair.0);
                                            assert(root_pair.0 == *k);
                                            K::antisymmetric(*k, t);
                                        } else if (t@, tv) == root_pair@ {
                                        } else {
                                            assert(left@.contains((t@, tv)));
                                            let tp: Pair<K, V> = choose|tp: Pair<K, V>| #[trigger] left@.contains(tp@) && tp@ == (t@, tv);
                                            if max_pair@ == tp@ {
                                                K::reflexive(key);
                                            } else {
                                                assert(tp.cmp_spec(&max_pair) == Less);
                                                assert(tp.0@ != max_pair.0@) by {
                                                    if tp.0@ == max_pair.0@ {
                                                        assert(tree@.contains(tp@));
                                                        assert(tree@.contains(max_pair@));
                                                    }
                                                };
                                                assert(tp.0.cmp_spec(&max_pair.0) == Less);
                                                assert(tp.0 == t);
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
                                assert(tree@.contains((lk@, lv)));
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
                                        assert(root_pair.0 == t);
                                        K::cmp_spec_less_implies_le(*k, root_pair.0);
                                        K::antisymmetric(t, *k);
                                    } else {
                                        assert(right@.contains((t@, tv)));
                                        let tp: Pair<K, V> = choose|tp: Pair<K, V>| #[trigger] right@.contains(tp@) && tp@ == (t@, tv);
                                        assert(tp.cmp_spec(&root_pair) == Greater);
                                        assert(tp.0@ != root_pair.0@) by {
                                            if tp.0@ == root_pair.0@ {
                                                assert(tree@.contains(tp@));
                                                assert(tree@.contains(root_pair@));
                                            }
                                        };
                                        assert(tp.0.cmp_spec(&root_pair.0) == Greater);
                                        assert(tp.0 == t);
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
                                        assert(root_pair.0 == t);
                                        K::cmp_spec_less_implies_le(*k, root_pair.0);
                                        if TotalOrder::le(t, *k) && t@ != k@ {
                                            K::antisymmetric(t, *k);
                                        }
                                    } else {
                                        assert(right@.contains((t@, tv)));
                                        let tp: Pair<K, V> = choose|tp: Pair<K, V>| #[trigger] right@.contains(tp@) && tp@ == (t@, tv);
                                        assert(tp.cmp_spec(&root_pair) == Greater);
                                        assert(tp.0@ != root_pair.0@) by {
                                            if tp.0@ == root_pair.0@ {
                                                assert(tree@.contains(tp@));
                                                assert(tree@.contains(root_pair@));
                                            }
                                        };
                                        assert(tp.0.cmp_spec(&root_pair.0) == Greater);
                                        assert(tp.0 == t);
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
                    assert(tree@ =~= Set::empty());
                    assert(spec_pair_set_to_map(tree@).dom() =~= Set::empty());
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
                    assert(tree@ =~= left@.union(right@).insert(root_pair@));
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
                                    assert(tree_dom.contains(x) && rank_pred(x));
                                    lemma_map_contains_pair_in_set(tree@, x);
                                    let xv: V::V = choose|xv: V::V| tree@.contains((x, xv));
                                    let t: K = choose|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, *k) && t@ != k@;
                                    if left@.contains((x, xv)) {
                                        lemma_pair_in_set_map_contains(left@, x, xv);
                                    } else if (x, xv) == root_pair@ {
                                        assert(t@ == root_pair.0@);
                                        assert(t == root_pair.0);
                                        K::cmp_spec_less_implies_le(*k, root_pair.0);
                                        K::antisymmetric(t, *k);
                                    } else {
                                        assert(right@.contains((x, xv)));
                                        let xp: Pair<K, V> = choose|xp: Pair<K, V>| #[trigger] right@.contains(xp@) && xp@ == (x, xv);
                                        assert(xp.cmp_spec(&root_pair) == Greater);
                                        assert(xp.0@ != root_pair.0@) by {
                                            if xp.0@ == root_pair.0@ {
                                                assert(tree@.contains(xp@));
                                                assert(tree@.contains(root_pair@));
                                            }
                                        };
                                        assert(xp.0.cmp_spec(&root_pair.0) == Greater);
                                        K::cmp_spec_greater_implies_le(xp.0, root_pair.0);
                                        K::cmp_spec_less_implies_le(*k, root_pair.0);
                                        assert(xp.0 == t);
                                        K::transitive(*k, root_pair.0, t);
                                        K::antisymmetric(t, *k);
                                    }
                                };
                                assert forall|x: K::V| #[trigger] left_dom.filter(rank_pred).contains(x)
                                    implies tree_dom.filter(rank_pred).contains(x) by {
                                    assert(left_dom.contains(x));
                                    lemma_map_contains_pair_in_set(left@, x);
                                    let xv: V::V = choose|xv: V::V| left@.contains((x, xv));
                                    assert(tree@.contains((x, xv)));
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
                            assert(k@ == root_pair.0@);
                            assert(*k == root_pair.0);
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
                                        assert(t@ == k@);
                                        assert(t@ != k@);
                                    } else {
                                        assert(right@.contains((x, xv)));
                                        let xp: Pair<K, V> = choose|xp: Pair<K, V>| #[trigger] right@.contains(xp@) && xp@ == (x, xv);
                                        assert(xp.cmp_spec(&root_pair) == Greater);
                                        assert(xp.0@ != root_pair.0@) by {
                                            if xp.0@ == root_pair.0@ {
                                                assert(tree@.contains(xp@));
                                                assert(tree@.contains(root_pair@));
                                            }
                                        };
                                        assert(xp.0.cmp_spec(&root_pair.0) == Greater);
                                        K::cmp_spec_greater_implies_le(xp.0, root_pair.0);
                                        assert(xp.0 == t);
                                        assert(root_pair.0 == *k);
                                        K::antisymmetric(t, *k);
                                    }
                                };
                                assert forall|x: K::V| #[trigger] left_dom.contains(x)
                                    implies tree_dom.filter(rank_pred).contains(x) by {
                                    lemma_map_contains_pair_in_set(left@, x);
                                    let xv: V::V = choose|xv: V::V| left@.contains((x, xv));
                                    assert(tree@.contains((x, xv)));
                                    lemma_pair_in_set_map_contains(tree@, x, xv);
                                    let xp: Pair<K, V> = choose|xp: Pair<K, V>| #[trigger] left@.contains(xp@) && xp@ == (x, xv);
                                    assert(xp.cmp_spec(&root_pair) == Less);
                                    assert(xp.0@ != root_pair.0@) by {
                                        if xp.0@ == root_pair.0@ {
                                            assert(tree@.contains(xp@));
                                            assert(tree@.contains(root_pair@));
                                        }
                                    };
                                    assert(xp.0.cmp_spec(&root_pair.0) == Less);
                                    K::cmp_spec_less_implies_le(xp.0, root_pair.0);
                                    assert(xp.0@ != root_pair.0@);
                                    assert(xp.0@ != k@);
                                    assert(root_pair.0 == *k);
                                    assert(rank_pred(x));
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
                            assert(k@ != root_pair.0@);
                            let tree_dom = spec_pair_set_to_map(tree@).dom();
                            let left_dom = spec_pair_set_to_map(left@).dom();
                            let right_dom = spec_pair_set_to_map(right@).dom();
                            lemma_pair_set_to_map_dom_finite(left@);
                            lemma_pair_set_to_map_dom_finite(right@);
                            lemma_pair_set_to_map_len(left@);
                            lemma_pair_set_to_map_len(right@);
                            let root_key_set = Set::empty().insert(root_pair.0@);
                            assert(tree_dom.filter(rank_pred) =~= left_dom.union(root_key_set).union(right_dom.filter(rank_pred))) by {
                                assert forall|x: K::V| #[trigger] tree_dom.filter(rank_pred).contains(x)
                                    implies left_dom.union(root_key_set).union(right_dom.filter(rank_pred)).contains(x) by {
                                    lemma_map_contains_pair_in_set(tree@, x);
                                    let xv: V::V = choose|xv: V::V| tree@.contains((x, xv));
                                    if left@.contains((x, xv)) {
                                        lemma_pair_in_set_map_contains(left@, x, xv);
                                    } else if (x, xv) == root_pair@ {
                                        assert(x == root_pair.0@);
                                    } else {
                                        assert(right@.contains((x, xv)));
                                        lemma_pair_in_set_map_contains(right@, x, xv);
                                    }
                                };
                                assert forall|x: K::V| #[trigger] left_dom.union(root_key_set).union(right_dom.filter(rank_pred)).contains(x)
                                    implies tree_dom.filter(rank_pred).contains(x) by {
                                    if left_dom.contains(x) {
                                        lemma_map_contains_pair_in_set(left@, x);
                                        let xv: V::V = choose|xv: V::V| left@.contains((x, xv));
                                        assert(tree@.contains((x, xv)));
                                        lemma_pair_in_set_map_contains(tree@, x, xv);
                                        let xp: Pair<K, V> = choose|xp: Pair<K, V>| #[trigger] left@.contains(xp@) && xp@ == (x, xv);
                                        assert(xp.cmp_spec(&root_pair) == Less);
                                        assert(xp.0@ != root_pair.0@) by {
                                            if xp.0@ == root_pair.0@ {
                                                assert(tree@.contains(xp@));
                                                assert(tree@.contains(root_pair@));
                                            }
                                        };
                                        assert(xp.0.cmp_spec(&root_pair.0) == Less);
                                        K::cmp_spec_less_implies_le(xp.0, root_pair.0);
                                        K::cmp_spec_greater_implies_le(*k, root_pair.0);
                                        K::transitive(xp.0, root_pair.0, *k);
                                        assert(xp.0@ != root_pair.0@);
                                        assert(root_pair.0@ != k@);
                                        assert(xp.0@ != k@);
                                        assert(rank_pred(x));
                                    } else if root_key_set.contains(x) {
                                        assert(x == root_pair.0@);
                                        assert(tree@.contains(root_pair@));
                                        lemma_pair_in_set_map_contains(tree@, root_pair.0@, root_pair.1@);
                                        K::cmp_spec_greater_implies_le(*k, root_pair.0);
                                        assert(root_pair.0@ != k@);
                                        assert(rank_pred(root_pair.0@));
                                    } else {
                                        lemma_map_contains_pair_in_set(right@, x);
                                        let xv: V::V = choose|xv: V::V| right@.contains((x, xv));
                                        assert(tree@.contains((x, xv)));
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
                            assert(left_dom.disjoint(right_dom.filter(rank_pred))) by {
                                assert forall|x: K::V| !(left_dom.contains(x) && #[trigger] right_dom.filter(rank_pred).contains(x)) by {
                                    if left_dom.contains(x) && right_dom.contains(x) {
                                        lemma_map_contains_pair_in_set(left@, x);
                                        lemma_map_contains_pair_in_set(right@, x);
                                        let lv: V::V = choose|lv: V::V| left@.contains((x, lv));
                                        let rv: V::V = choose|rv: V::V| right@.contains((x, rv));
                                        assert(tree@.contains((x, lv)));
                                        assert(tree@.contains((x, rv)));
                                        assert(lv == rv);
                                        assert(left@.contains((x, lv)));
                                        assert(right@.contains((x, lv)));
                                    }
                                };
                            };
                            assert(root_key_set.disjoint(right_dom.filter(rank_pred))) by {
                                assert forall|x: K::V| !(root_key_set.contains(x) && #[trigger] right_dom.filter(rank_pred).contains(x)) by {
                                    if root_key_set.contains(x) && right_dom.contains(x) {
                                        assert(x == root_pair.0@);
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
                    assert(tree@ =~= left@.union(right@).insert(root_pair@));
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
                                assert(tree@.contains((sel_key@, sv)));
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
                                            assert(sp.cmp_spec(&root_pair) == Less);
                                            assert(sp.0@ != root_pair.0@) by {
                                                if sp.0@ == root_pair.0@ {
                                                    assert(tree@.contains(sp@));
                                                    assert(tree@.contains(root_pair@));
                                                }
                                            };
                                            assert(sp.0.cmp_spec(&root_pair.0) == Less);
                                            K::cmp_spec_less_implies_le(sp.0, root_pair.0);
                                            assert(sp.0 == sel_key);
                                            assert(t@ == root_pair.0@);
                                            assert(t == root_pair.0);
                                            K::antisymmetric(t, sel_key);
                                        } else {
                                            assert(right@.contains((x, xv)));
                                            let sp: Pair<K, V> = choose|sp: Pair<K, V>| #[trigger] left@.contains(sp@) && sp@ == (sel_key@, sv);
                                            let xp: Pair<K, V> = choose|xp: Pair<K, V>| #[trigger] right@.contains(xp@) && xp@ == (x, xv);
                                            assert(sp.cmp_spec(&root_pair) == Less);
                                            assert(xp.cmp_spec(&root_pair) == Greater);
                                            assert(sp.0@ != root_pair.0@) by {
                                                if sp.0@ == root_pair.0@ {
                                                    assert(tree@.contains(sp@));
                                                    assert(tree@.contains(root_pair@));
                                                }
                                            };
                                            assert(xp.0@ != root_pair.0@) by {
                                                if xp.0@ == root_pair.0@ {
                                                    assert(tree@.contains(xp@));
                                                    assert(tree@.contains(root_pair@));
                                                }
                                            };
                                            assert(sp.0.cmp_spec(&root_pair.0) == Less);
                                            assert(xp.0.cmp_spec(&root_pair.0) == Greater);
                                            K::cmp_spec_less_implies_le(sp.0, root_pair.0);
                                            K::cmp_spec_greater_implies_le(xp.0, root_pair.0);
                                            assert(sp.0 == sel_key);
                                            assert(xp.0 == t);
                                            K::transitive(sel_key, root_pair.0, t);
                                            K::antisymmetric(t, sel_key);
                                        }
                                    };
                                    assert forall|x: K::V| #[trigger] left_dom.filter(rank_pred_sel).contains(x)
                                        implies tree_dom.filter(rank_pred_sel).contains(x) by {
                                        lemma_map_contains_pair_in_set(left@, x);
                                        let xv: V::V = choose|xv: V::V| left@.contains((x, xv));
                                        assert(tree@.contains((x, xv)));
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
                        assert(key == root_pair.0);
                        assert(tree@.contains(root_pair@));
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
                                    assert(t@ == key@);
                                    assert(t@ != key@);
                                } else {
                                    assert(right@.contains((x, xv)));
                                    let xp: Pair<K, V> = choose|xp: Pair<K, V>| #[trigger] right@.contains(xp@) && xp@ == (x, xv);
                                    assert(xp.cmp_spec(&root_pair) == Greater);
                                    assert(xp.0@ != root_pair.0@) by {
                                        if xp.0@ == root_pair.0@ {
                                            assert(tree@.contains(xp@));
                                            assert(tree@.contains(root_pair@));
                                        }
                                    };
                                    assert(xp.0.cmp_spec(&root_pair.0) == Greater);
                                    K::cmp_spec_greater_implies_le(xp.0, root_pair.0);
                                    assert(xp.0 == t);
                                    assert(root_pair.0 == key);
                                    K::antisymmetric(t, key);
                                }
                            };
                            assert forall|x: K::V| #[trigger] left_dom.contains(x)
                                implies tree_dom.filter(rank_pred_root).contains(x) by {
                                lemma_map_contains_pair_in_set(left@, x);
                                let xv: V::V = choose|xv: V::V| left@.contains((x, xv));
                                assert(tree@.contains((x, xv)));
                                lemma_pair_in_set_map_contains(tree@, x, xv);
                                let xp: Pair<K, V> = choose|xp: Pair<K, V>| #[trigger] left@.contains(xp@) && xp@ == (x, xv);
                                assert(xp.cmp_spec(&root_pair) == Less);
                                assert(xp.0@ != root_pair.0@) by {
                                    if xp.0@ == root_pair.0@ {
                                        assert(tree@.contains(xp@));
                                        assert(tree@.contains(root_pair@));
                                    }
                                };
                                assert(xp.0.cmp_spec(&root_pair.0) == Less);
                                K::cmp_spec_less_implies_le(xp.0, root_pair.0);
                                assert(xp.0@ != root_pair.0@);
                                assert(root_pair.0 == key);
                                assert(xp.0@ != key@);
                                assert(rank_pred_root(x));
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
                                assert(tree@.contains((sel_key@, sv)));
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
                                        let t: K = choose|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, sel_key) && t@ != sel_key@;
                                        if left@.contains((x, xv)) {
                                            lemma_pair_in_set_map_contains(left@, x, xv);
                                        } else if (x, xv) == root_pair@ {
                                        } else {
                                            assert(right@.contains((x, xv)));
                                            lemma_pair_in_set_map_contains(right@, x, xv);
                                        }
                                    };
                                    assert forall|x: K::V| #[trigger] left_dom.union(root_key_set).union(right_dom.filter(rank_pred_sel)).contains(x)
                                        implies tree_dom.filter(rank_pred_sel).contains(x) by {
                                        if left_dom.contains(x) {
                                            lemma_map_contains_pair_in_set(left@, x);
                                            let xv: V::V = choose|xv: V::V| left@.contains((x, xv));
                                            assert(tree@.contains((x, xv)));
                                            lemma_pair_in_set_map_contains(tree@, x, xv);
                                            let xp: Pair<K, V> = choose|xp: Pair<K, V>| #[trigger] left@.contains(xp@) && xp@ == (x, xv);
                                            let sp: Pair<K, V> = choose|sp: Pair<K, V>| #[trigger] right@.contains(sp@) && sp@ == (sel_key@, sv);
                                            assert(xp.cmp_spec(&root_pair) == Less);
                                            assert(sp.cmp_spec(&root_pair) == Greater);
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
                                            assert(xp.0.cmp_spec(&root_pair.0) == Less);
                                            assert(sp.0.cmp_spec(&root_pair.0) == Greater);
                                            K::cmp_spec_less_implies_le(xp.0, root_pair.0);
                                            K::cmp_spec_greater_implies_le(sp.0, root_pair.0);
                                            assert(sp.0 == sel_key);
                                            K::transitive(xp.0, root_pair.0, sel_key);
                                            assert(xp.0@ != sel_key@);
                                            assert(rank_pred_sel(x));
                                        } else if root_key_set.contains(x) {
                                            assert(x == root_pair.0@);
                                            assert(tree@.contains(root_pair@));
                                            lemma_pair_in_set_map_contains(tree@, root_pair.0@, root_pair.1@);
                                            let sp: Pair<K, V> = choose|sp: Pair<K, V>| #[trigger] right@.contains(sp@) && sp@ == (sel_key@, sv);
                                            assert(sp.cmp_spec(&root_pair) == Greater);
                                            assert(sp.0@ != root_pair.0@) by {
                                                if sp.0@ == root_pair.0@ {
                                                    assert(tree@.contains(sp@));
                                                    assert(tree@.contains(root_pair@));
                                                }
                                            };
                                            assert(sp.0.cmp_spec(&root_pair.0) == Greater);
                                            K::cmp_spec_greater_implies_le(sp.0, root_pair.0);
                                            assert(sp.0 == sel_key);
                                            assert(root_pair.0@ != sel_key@);
                                            assert(rank_pred_sel(root_pair.0@));
                                        } else {
                                            lemma_map_contains_pair_in_set(right@, x);
                                            let xv: V::V = choose|xv: V::V| right@.contains((x, xv));
                                            assert(tree@.contains((x, xv)));
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
                                            assert(lv == rv);
                                            assert(left@.contains((x, lv)));
                                            assert(right@.contains((x, lv)));
                                        }
                                    };
                                };
                                assert(root_key_set.disjoint(right_dom.filter(rank_pred_sel))) by {
                                    assert forall|x: K::V| !(root_key_set.contains(x) && #[trigger] right_dom.filter(rank_pred_sel).contains(x)) by {
                                        if root_key_set.contains(x) && right_dom.contains(x) {
                                            assert(x == root_pair.0@);
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
            ensures empty@ == Map::<K::V, V::V>::empty()
        {
            let inner = ParamBST::<Pair<K, V>>::new();
            proof {
                lemma_set_to_map_empty::<K::V, V::V>();
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
            }
            (left, found, right)
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
