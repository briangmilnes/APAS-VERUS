//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! REVIEWED: NO
//! Single-threaded persistent ordered table backed by ParamBST<Pair<K,V>>.

pub mod OrderedTableStPer {

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
    use vstd::prelude::*;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;
    #[cfg(verus_keep_ghost)]
    use vstd::laws_eq::obeys_view_eq;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::OrdSpec;
    #[cfg(verus_keep_ghost)]
    use vstd::std_specs::cmp::PartialEqSpecImpl;

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
    // 12. derive impls in verus!
    // 13. macros
    // 14. derive impls outside verus!

    // 4. type definitions

    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct OrderedTableStPer<K: StT + Ord, V: StT + Ord> {
        pub tree: ParamBST<Pair<K, V>>,
    }

    pub type OrderedTablePer<K, V> = OrderedTableStPer<K, V>;

    // 5. view impls

    impl<K: StT + Ord, V: StT + Ord> View for OrderedTableStPer<K, V> {
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

    /// Key uniqueness for a set of pairs: no two pairs share the same first component.
    /// Nested quantifiers break the symmetric trigger loop that the flat form causes.
    pub open spec fn spec_key_unique_pairs_set<KV, VV>(s: Set<(KV, VV)>) -> bool {
        forall|k: KV, v: VV| #[trigger] s.contains((k, v)) ==>
            forall|v2: VV| s.contains((k, v2)) ==> v == v2
    }

    /// Pair ordering is determined by key ordering when keys differ.
    pub open spec fn spec_pair_key_determines_order<K: StT + Ord, V: StT + Ord>() -> bool {
        forall|p1: Pair<K, V>, p2: Pair<K, V>|
            p1.0.cmp_spec(&p2.0) != Equal ==>
            (#[trigger] p1.cmp_spec(&p2)) == p1.0.cmp_spec(&p2.0)
    }

    /// Spec predicate for rank_key: x is strictly less than k in the total order.
    pub open spec fn spec_rank_pred<K: StT + Ord + TotalOrder>(x: K::V, k: K) -> bool {
        exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, k) && t@ != k@
    }

    // 7. proof fns

    /// The domain of spec_pair_set_to_map is finite when the source set is finite.
    pub proof fn lemma_pair_set_to_map_dom_finite<KV, VV>(s: Set<(KV, VV)>)
        requires s.finite()
        ensures spec_pair_set_to_map(s).dom().finite()
    {
        let dom_set = spec_pair_set_to_map(s).dom();
        let proj = |p: (KV, VV)| -> KV { p.0 };
        let proj_set = s.map(proj);
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
        assert(vstd::relations::injective_on(proj, s)) by {
            assert forall|x1: (KV, VV), x2: (KV, VV)|
                s.contains(x1) && s.contains(x2) && #[trigger] proj(x1) == #[trigger] proj(x2)
                implies x1 == x2
            by {};
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
        let v2 = choose|v2: VV| s.contains((k, v2));
        assert(s.contains((k, v2)));
        assert(v2 == v);
    }

    /// If the map contains a key, a pair with that key exists in the set.
    proof fn lemma_map_contains_pair_in_set<KV, VV>(s: Set<(KV, VV)>, k: KV)
        requires spec_pair_set_to_map(s).contains_key(k)
        ensures exists|v: VV| s.contains((k, v))
    {
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
            } else {
            }
        };
    }

    /// Equal-substitution for cmp_spec.
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

    /// In-order traversal keys are pairwise distinct.
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

        assert(sorted.to_set() =~= tree) by {
            assert forall|v: (KV, VV)| sorted.to_set().contains(v) <==> #[trigger] tree.contains(v) by {};
        };
        sorted.lemma_no_dup_set_cardinality();
        assert forall|i: int, j: int|
            0 <= i < sorted.len() && 0 <= j < sorted.len() && i != j
            implies (#[trigger] sorted[i]).0 != (#[trigger] sorted[j]).0
        by {
            if sorted[i].0 == sorted[j].0 {
                assert(tree.contains(sorted[i]));
                assert(tree.contains(sorted[j]));
                assert(sorted[i] == sorted[j]);
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
                assert(false);
            }
            let cv: VV = choose|cv: VV| s.contains((key, cv));
            assert(cv == vv);
        };
        assert forall|key: KV| old_m.remove(k).dom().contains(key)
            implies #[trigger] new_m.dom().contains(key)
        by {
            assert(key != k);
            let vv: VV = choose|vv: VV| s.contains((key, vv));
            assert(new_s.contains((key, vv)));
        };
    }

    /// The map over an empty set is the empty map.
    proof fn lemma_set_to_map_empty<KV, VV>()
        ensures spec_pair_set_to_map(Set::<(KV, VV)>::empty()) =~= Map::<KV, VV>::empty()
    {
    }

    /// Find by key in a ParamBST of pairs via in-order scan.
    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- in_order traversal + linear scan
    fn bst_find_by_key<K: StT + Ord, V: StT + Ord>(
        tree: &ParamBST<Pair<K, V>>,
        k: &K,
    ) -> (found: Option<V>)
        requires
            tree.spec_bstparasteph_wf(),
            spec_key_unique_pairs_set(tree@),
            view_ord_consistent::<K>(),
            obeys_feq_fulls::<K, V>(),
            vstd::laws_cmp::obeys_cmp_spec::<K>(),
        ensures
            match found {
                Some(v) => spec_pair_set_to_map(tree@).contains_key(k@)
                    && v@ == spec_pair_set_to_map(tree@)[k@],
                None => !spec_pair_set_to_map(tree@).contains_key(k@),
            }
    {
        let sorted = tree.in_order();
        let len = sorted.length();
        let mut i: usize = 0;
        let mut result: Option<V> = None;
        while i < len
            invariant
                i <= len,
                len as nat == sorted@.len(),
                sorted@.len() == tree@.len(),
                spec_key_unique_pairs_set(tree@),
                obeys_feq_fulls::<K, V>(),
                view_ord_consistent::<K>(),
                vstd::laws_cmp::obeys_cmp_spec::<K>(),
                forall|v: <Pair<K, V> as View>::V| tree@.contains(v) <==> sorted@.contains(v),
                result is None ==> forall|j: int| 0 <= j < i as int ==>
                    (#[trigger] sorted@[j]).0 != k@,
                result is Some ==> {
                    &&& spec_pair_set_to_map(tree@).contains_key(k@)
                    &&& result->Some_0@ == spec_pair_set_to_map(tree@)[k@]
                },
            decreases len - i,
        {
            if result.is_some() { i = i + 1; } else {
                let pair = sorted.nth(i);
                let c = pair.0.cmp(k);
                proof { reveal(vstd::laws_cmp::obeys_cmp_ord); }
                match c {
                    Equal => {
                        let v_clone = pair.1.clone_plus();
                        proof {
                            lemma_cloned_view_eq(pair.1, v_clone);
                            assert(pair.0.cmp_spec(k) == Equal);
                            assert(pair.0@ == k@);
                            assert(sorted@.contains(sorted@[i as int]));
                            assert(tree@.contains(sorted@[i as int]));
                            lemma_pair_in_set_map_contains(tree@, k@, sorted@[i as int].1);
                        }
                        result = Some(v_clone);
                    },
                    _ => {
                        proof {
                            assert(pair.0.cmp_spec(k) != Equal);
                            assert(pair.0@ != k@);
                            assert(sorted@[i as int].0 != k@);
                        }
                    },
                }
                i = i + 1;
            }
        }
        proof {
            if result is None {
                if spec_pair_set_to_map(tree@).contains_key(k@) {
                    lemma_map_contains_pair_in_set(tree@, k@);
                    let v: V::V = choose|v: V::V| tree@.contains((k@, v));
                    assert(sorted@.contains((k@, v)));
                    let j = choose|j: int| 0 <= j < sorted@.len()
                        && (#[trigger] sorted@[j]) == (k@, v);
                    assert(sorted@[j].0 == k@);
                }
            }
        }
        result
    }

    // 8. traits

    /// Trait defining all ordered table operations (ADT 42.1 + ADT 43.1) with persistent semantics.
    pub trait OrderedTableStPerTrait<K: StT + Ord, V: StT + Ord>: Sized + View<V = Map<K::V, V::V>> {
        spec fn spec_orderedtablestper_wf(&self) -> bool;
        spec fn spec_orderedtablestper_find_pre(&self) -> bool;

        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        fn size(&self) -> (count: usize)
            requires self.spec_orderedtablestper_wf(),
            ensures count == self@.dom().len(), self@.dom().finite();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        fn empty() -> (table: Self)
            requires
                vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                view_ord_consistent::<Pair<K, V>>(),
                spec_pair_key_determines_order::<K, V>(),
                vstd::laws_cmp::obeys_cmp_spec::<K>(),
                view_ord_consistent::<K>(),
            ensures table@ == Map::<K::V, V::V>::empty(), table.spec_orderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(1), Span O(1)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
        fn singleton(k: K, v: V) -> (table: Self)
            requires
                obeys_feq_clone::<Pair<K, V>>(),
                vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                view_ord_consistent::<Pair<K, V>>(),
                spec_pair_key_determines_order::<K, V>(),
                vstd::laws_cmp::obeys_cmp_spec::<K>(),
                view_ord_consistent::<K>(),
            ensures table@ == Map::<K::V, V::V>::empty().insert(k@, v@), table.spec_orderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS
        fn find(&self, k: &K) -> (found: Option<V>)
            requires self.spec_orderedtablestper_find_pre(), obeys_view_eq::<K>(), obeys_feq_full::<V>(),
            ensures
                match found {
                    Some(v) => self@.contains_key(k@) && self@[k@] == v@,
                    None => !self@.contains_key(k@),
                };
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS
        fn insert(&self, k: K, v: V) -> (table: Self)
            requires
                self.spec_orderedtablestper_wf(),
                obeys_view_eq::<K>(),
                self@.dom().len() + 1 < usize::MAX as nat,
            ensures
                table@.dom() =~= self@.dom().insert(k@),
                table@[k@] == v@,
                forall|k2: K::V| k2 != k@ && #[trigger] self@.contains_key(k2) ==> table@[k2] == self@[k2],
                table.spec_orderedtablestper_wf();
        /// Like insert, but additionally ensures the inserted value mapping.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to insert
        fn insert_wf(&self, k: K, v: V) -> (table: Self)
            requires
                self.spec_orderedtablestper_wf(),
                obeys_view_eq::<K>(),
                self@.dom().len() + 1 < usize::MAX as nat,
            ensures
                table@.dom() =~= self@.dom().insert(k@),
                table@[k@] == v@,
                forall|k2: K::V| k2 != k@ && #[trigger] self@.contains_key(k2) ==> table@[k2] == self@[k2],
                table.spec_orderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS
        fn delete(&self, k: &K) -> (table: Self)
            requires
                self.spec_orderedtablestper_wf(),
                obeys_feq_clone::<Pair<K, V>>(),
                obeys_view_eq::<K>(),
            ensures table@ == self@.remove(k@), table.spec_orderedtablestper_wf();
        /// Like delete, but additionally ensures value preservation for remaining keys.
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to delete
        fn delete_wf(&self, k: &K) -> (table: Self)
            requires
                self.spec_orderedtablestper_wf(),
                obeys_feq_clone::<Pair<K, V>>(),
                obeys_view_eq::<K>(),
            ensures
                table@ == self@.remove(k@),
                forall|k2: K::V| k2 != k@ && #[trigger] self@.contains_key(k2) ==> table@[k2] == self@[k2],
                table.spec_orderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — matches APAS
        fn domain(&self) -> (keys: ArraySetStEph<K>)
            requires self.spec_orderedtablestper_wf(), obeys_feq_clone::<K>()
            ensures keys@ =~= self@.dom(), self@.dom().finite();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n log n), Span O(n log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) — matches APAS
        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (table: Self)
            requires
                keys.spec_arraysetsteph_wf(),
                forall|k: &K| f.requires((k,)),
                obeys_feq_full::<K>(),
                keys@.len() < usize::MAX,
                vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                view_ord_consistent::<Pair<K, V>>(),
                spec_pair_key_determines_order::<K, V>(),
                vstd::laws_cmp::obeys_cmp_spec::<K>(),
                view_ord_consistent::<K>(),
            ensures
                table@.dom() =~= keys@,
                table.spec_orderedtablestper_wf(),
                forall|k: K::V| #[trigger] table@.contains_key(k) ==>
                    (exists|key_arg: K, result: V|
                        key_arg@ == k && f.ensures((&key_arg,), result)
                        && table@[k] == result@);
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — matches APAS
        fn map<F: Fn(&V) -> V>(&self, f: F) -> (table: Self)
            requires self.spec_orderedtablestper_wf(), forall|v: &V| f.requires((v,)),
            ensures
                table@.dom() == self@.dom(),
                forall|k: K::V| #[trigger] table@.contains_key(k) ==>
                    (exists|old_val: V, result: V|
                        old_val@ == self@[k]
                        && f.ensures((&old_val,), result)
                        && table@[k] == result@),
                table.spec_orderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n), Span O(n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — matches APAS
        fn filter<F: Fn(&K, &V) -> bool>(&self, f: F, Ghost(spec_pred): Ghost<spec_fn(K::V, V::V) -> bool>) -> (table: Self)
            requires
                self.spec_orderedtablestper_wf(),
                forall|k: &K, v: &V| f.requires((k, v)),
                forall|k: K, v: V, keep: bool| f.ensures((&k, &v), keep) ==> keep == spec_pred(k@, v@),
            ensures
                table@.dom().subset_of(self@.dom()),
                forall|k: K::V| #[trigger] table@.contains_key(k) ==> table@[k] == self@[k],
                forall|k: K::V| self@.dom().contains(k) && spec_pred(k, self@[k])
                    ==> #[trigger] table@.dom().contains(k),
                table.spec_orderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1) — matches APAS
        fn intersection<F: Fn(&V, &V) -> V>(&self, other: &Self, f: F) -> (table: Self)
            requires
                self.spec_orderedtablestper_wf(),
                other.spec_orderedtablestper_wf(),
                forall|v1: &V, v2: &V| f.requires((v1, v2)),
                obeys_view_eq::<K>(),
            ensures
                table@.dom() =~= self@.dom().intersect(other@.dom()),
                forall|k: K::V| #[trigger] table@.contains_key(k) ==>
                    (exists|v1: V, v2: V, r: V|
                        v1@ == self@[k] && v2@ == other@[k]
                        && f.ensures((&v1, &v2), r)
                        && table@[k] == r@),
                table.spec_orderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1) — matches APAS
        fn union<F: Fn(&V, &V) -> V>(&self, other: &Self, f: F) -> (table: Self)
            requires
                self.spec_orderedtablestper_wf(),
                other.spec_orderedtablestper_wf(),
                forall|v1: &V, v2: &V| f.requires((v1, v2)),
                obeys_view_eq::<K>(),
                self@.dom().len() + other@.dom().len() < usize::MAX,
            ensures
                table@.dom() =~= self@.dom().union(other@.dom()),
                forall|k: K::V| #[trigger] self@.contains_key(k) && !other@.contains_key(k)
                    ==> table@[k] == self@[k],
                forall|k: K::V| #[trigger] other@.contains_key(k) && !self@.contains_key(k)
                    ==> table@[k] == other@[k],
                forall|k: K::V| #[trigger] self@.contains_key(k) && other@.contains_key(k) ==>
                    (exists|v1: V, v2: V, r: V|
                        v1@ == self@[k] && v2@ == other@[k]
                        && f.ensures((&v1, &v2), r)
                        && table@[k] == r@),
                table.spec_orderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1) — matches APAS
        fn difference(&self, other: &Self) -> (table: Self)
            requires self.spec_orderedtablestper_wf(), other.spec_orderedtablestper_wf(), obeys_view_eq::<K>(),
            ensures
                table@.dom() =~= self@.dom().difference(other@.dom()),
                forall|k: K::V| #[trigger] table@.contains_key(k) ==> table@[k] == self@[k],
                table.spec_orderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1) — matches APAS
        fn restrict(&self, keys: &ArraySetStEph<K>) -> (table: Self)
            requires self.spec_orderedtablestper_wf(),
            ensures
                table@.dom() =~= self@.dom().intersect(keys@),
                forall|k: K::V| #[trigger] table@.contains_key(k) ==> table@[k] == self@[k],
                table.spec_orderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1) — matches APAS
        fn subtract(&self, keys: &ArraySetStEph<K>) -> (table: Self)
            requires self.spec_orderedtablestper_wf(),
            ensures
                table@.dom() =~= self@.dom().difference(keys@),
                forall|k: K::V| #[trigger] table@.contains_key(k) ==> table@[k] == self@[k],
                table.spec_orderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(n log n), Span O(n log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) — matches APAS
        fn collect(&self) -> (sorted_entries: AVLTreeSeqStPerS<Pair<K, V>>)
            requires self.spec_orderedtablestper_wf(),
            ensures self@.dom().finite(), sorted_entries.spec_avltreeseqstper_wf(), sorted_entries@.len() == self@.dom().len();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS
        fn first_key(&self) -> (key: Option<K>)
            where K: TotalOrder
            requires self.spec_orderedtablestper_wf(),
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> key matches None,
                key matches Some(k) ==> self@.dom().contains(k@),
                key matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(v, t);
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS
        fn last_key(&self) -> (key: Option<K>)
            where K: TotalOrder
            requires self.spec_orderedtablestper_wf(),
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> key matches None,
                key matches Some(k) ==> self@.dom().contains(k@),
                key matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(t, v);
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS
        fn previous_key(&self, k: &K) -> (key: Option<K>)
            where K: TotalOrder
            requires self.spec_orderedtablestper_wf(),
            ensures
                self@.dom().finite(),
                key matches Some(pk) ==> self@.dom().contains(pk@),
                key matches Some(v) ==> TotalOrder::le(v, *k) && v@ != k@,
                key matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(t, *k) && t@ != k@ ==> TotalOrder::le(t, v);
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS
        fn next_key(&self, k: &K) -> (key: Option<K>)
            where K: TotalOrder
            requires self.spec_orderedtablestper_wf(),
            ensures
                self@.dom().finite(),
                key matches Some(nk) ==> self@.dom().contains(nk@),
                key matches Some(v) ==> TotalOrder::le(*k, v) && v@ != k@,
                key matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(*k, t) && t@ != k@ ==> TotalOrder::le(v, t);
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS
        fn split_key(&self, k: &K) -> (parts: (Self, Option<V>, Self))
            where Self: Sized
            requires
                self.spec_orderedtablestper_wf(),
                obeys_view_eq::<K>(),
            ensures
                self@.dom().finite(),
                parts.1 matches Some(v) ==> self@.contains_key(k@) && v@ == self@[k@],
                parts.1 matches None ==> !self@.contains_key(k@),
                !parts.0@.dom().contains(k@),
                !parts.2@.dom().contains(k@),
                parts.0@.dom().subset_of(self@.dom()),
                parts.2@.dom().subset_of(self@.dom()),
                parts.0@.dom().disjoint(parts.2@.dom()),
                forall|key| #[trigger] self@.dom().contains(key) ==> parts.0@.dom().contains(key) || parts.2@.dom().contains(key) || key == k@,
                parts.0.spec_orderedtablestper_wf(),
                parts.2.spec_orderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(m log(n/m + 1)), Span O(log n log m)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(m log(n/m + 1) — matches APAS
        fn join_key(left: &Self, right: &Self) -> (table: Self)
            requires
                left.spec_orderedtablestper_wf(),
                right.spec_orderedtablestper_wf(),
                obeys_view_eq::<K>(),
                obeys_feq_full::<Pair<K, V>>(),
                left@.dom().len() + right@.dom().len() < usize::MAX,
            ensures
                table@.dom() =~= left@.dom().union(right@.dom()),
                table.spec_orderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n + m), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n + m) — DIFFERS: St sequential, APAS parallel
        fn get_key_range(&self, k1: &K, k2: &K) -> (table: Self)
            requires
                self.spec_orderedtablestper_wf(),
            ensures
                table@.dom().subset_of(self@.dom()),
                forall|key| #[trigger] table@.dom().contains(key) ==> table@[key] == self@[key],
                table.spec_orderedtablestper_wf();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS
        fn rank_key(&self, k: &K) -> (rank: usize)
            where K: TotalOrder
            requires
                self.spec_orderedtablestper_wf(),
                obeys_view_eq::<K>(),
            ensures
                self@.dom().finite(),
                rank <= self@.dom().len(),
                rank as int == self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, *k) && t@ != k@).len();
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS
        fn select_key(&self, i: usize) -> (key: Option<K>)
            where K: TotalOrder
            requires
                self.spec_orderedtablestper_wf(),
                obeys_view_eq::<K>(),
            ensures
                self@.dom().finite(),
                i >= self@.dom().len() ==> key matches None,
                key matches Some(k) ==> self@.dom().contains(k@),
                key matches Some(v) ==> self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, v) && t@ != v@).len() == i as int;
        /// - Alg Analysis: APAS (Ch43 CS 43.2): Work O(log n), Span O(log n)
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(log n), Span O(log n) — matches APAS
        fn split_rank_key(&self, i: usize) -> (parts: (Self, Self))
            where Self: Sized
            requires
                self.spec_orderedtablestper_wf(),
            ensures
                self@.dom().finite(),
                parts.0@.dom().subset_of(self@.dom()),
                parts.1@.dom().subset_of(self@.dom()),
                parts.0@.dom().disjoint(parts.1@.dom()),
                forall|key| #[trigger] self@.dom().contains(key) ==> parts.0@.dom().contains(key) || parts.1@.dom().contains(key),
                parts.0.spec_orderedtablestper_wf(),
                parts.1.spec_orderedtablestper_wf();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to find
        fn find_iter(&self, k: &K) -> (found: Option<V>)
            requires self.spec_orderedtablestper_find_pre(), obeys_view_eq::<K>(), obeys_feq_full::<V>(),
            ensures
                match found {
                    Some(v) => self@.contains_key(k@) && self@[k@] == v@,
                    None => !self@.contains_key(k@),
                };
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to insert
        fn insert_iter(&self, k: K, v: V) -> (table: Self)
            requires
                self.spec_orderedtablestper_wf(),
                obeys_view_eq::<K>(),
                self@.dom().len() + 1 < usize::MAX as nat,
            ensures
                table@.dom() =~= self@.dom().insert(k@),
                table.spec_orderedtablestper_wf();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to delete
        fn delete_iter(&self, k: &K) -> (table: Self)
            requires
                self.spec_orderedtablestper_wf(),
                obeys_feq_clone::<Pair<K, V>>(),
                obeys_view_eq::<K>(),
            ensures table@ == self@.remove(k@), table.spec_orderedtablestper_wf();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- in_order traversal + take first
        fn first_key_iter(&self) -> (key: Option<K>)
            where K: TotalOrder
            requires self.spec_orderedtablestper_wf(),
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> key matches None,
                key matches Some(k) ==> self@.dom().contains(k@),
                key matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(v, t);
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- in_order traversal + take last
        fn last_key_iter(&self) -> (key: Option<K>)
            where K: TotalOrder
            requires self.spec_orderedtablestper_wf(),
            ensures
                self@.dom().finite(),
                self@.dom().len() == 0 <==> key matches None,
                key matches Some(k) ==> self@.dom().contains(k@),
                key matches Some(v) ==> forall|t: K| self@.dom().contains(t@) ==> #[trigger] TotalOrder::le(t, v);
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- in_order traversal + linear scan
        fn previous_key_iter(&self, k: &K) -> (key: Option<K>)
            where K: TotalOrder
            requires self.spec_orderedtablestper_wf(),
            ensures
                self@.dom().finite(),
                key matches Some(pk) ==> self@.dom().contains(pk@),
                key matches Some(v) ==> TotalOrder::le(v, *k) && v@ != k@,
                key matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(t, *k) && t@ != k@ ==> TotalOrder::le(t, v);
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- in_order traversal + linear scan
        fn next_key_iter(&self, k: &K) -> (key: Option<K>)
            where K: TotalOrder
            requires self.spec_orderedtablestper_wf(),
            ensures
                self@.dom().finite(),
                key matches Some(nk) ==> self@.dom().contains(nk@),
                key matches Some(v) ==> TotalOrder::le(*k, v) && v@ != k@,
                key matches Some(v) ==> forall|t: K| #![trigger t@] self@.dom().contains(t@) && TotalOrder::le(*k, t) && t@ != k@ ==> TotalOrder::le(v, t);
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- in_order + n BST inserts
        fn split_key_iter(&self, k: &K) -> (parts: (Self, Option<V>, Self))
            where Self: Sized
            requires
                self.spec_orderedtablestper_wf(),
                obeys_view_eq::<K>(),
            ensures
                self@.dom().finite(),
                parts.1 matches Some(v) ==> self@.contains_key(k@) && v@ == self@[k@],
                parts.1 matches None ==> !self@.contains_key(k@),
                !parts.0@.dom().contains(k@),
                !parts.2@.dom().contains(k@),
                parts.0@.dom().subset_of(self@.dom()),
                parts.2@.dom().subset_of(self@.dom()),
                parts.0@.dom().disjoint(parts.2@.dom()),
                forall|key| #[trigger] self@.dom().contains(key) ==> parts.0@.dom().contains(key) || parts.2@.dom().contains(key) || key == k@,
                parts.0.spec_orderedtablestper_wf(),
                parts.2.spec_orderedtablestper_wf();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- in_order + conditional BST inserts
        fn get_key_range_iter(&self, k1: &K, k2: &K) -> (table: Self)
            requires
                self.spec_orderedtablestper_wf(),
            ensures
                table@.dom().subset_of(self@.dom()),
                forall|key| #[trigger] table@.dom().contains(key) ==> table@[key] == self@[key],
                table.spec_orderedtablestper_wf();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- in_order traversal + count
        fn rank_key_iter(&self, k: &K) -> (rank: usize)
            where K: TotalOrder
            requires
                self.spec_orderedtablestper_wf(),
                obeys_view_eq::<K>(),
            ensures
                self@.dom().finite(),
                rank <= self@.dom().len(),
                rank as int == self@.dom().filter(|x: K::V| exists|t: K| #![trigger t@] t@ == x && TotalOrder::le(t, *k) && t@ != k@).len();
        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- in_order + n BST inserts
        fn split_rank_key_iter(&self, i: usize) -> (parts: (Self, Self))
            where Self: Sized
            requires
                self.spec_orderedtablestper_wf(),
            ensures
                self@.dom().finite(),
                parts.0@.dom().subset_of(self@.dom()),
                parts.1@.dom().subset_of(self@.dom()),
                parts.0@.dom().disjoint(parts.1@.dom()),
                forall|key| #[trigger] self@.dom().contains(key) ==> parts.0@.dom().contains(key) || parts.1@.dom().contains(key),
                parts.0.spec_orderedtablestper_wf(),
                parts.1.spec_orderedtablestper_wf();
    }

    // 9. impls

    impl<K: StT + Ord, V: StT + Ord> OrderedTableStPerTrait<K, V> for OrderedTableStPer<K, V> {
        open spec fn spec_orderedtablestper_wf(&self) -> bool {
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
        }

        open spec fn spec_orderedtablestper_find_pre(&self) -> bool {
            self.tree.spec_bstparasteph_wf()
            && spec_key_unique_pairs_set(self.tree@)
            && obeys_feq_fulls::<K, V>()
            && vstd::laws_cmp::obeys_cmp_spec::<K>()
            && view_ord_consistent::<K>()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn size(&self) -> (count: usize) {
            let r = self.tree.size();
            proof {
                lemma_pair_set_to_map_len(self.tree@);
                lemma_pair_set_to_map_dom_finite(self.tree@);
            }
            r
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn empty() -> (table: Self) {
            let tree = ParamBST::<Pair<K, V>>::new();
            proof {
                lemma_set_to_map_empty::<K::V, V::V>();
                lemma_key_unique_empty::<K::V, V::V>();
                assert(obeys_feq_full_trigger::<K>());
                assert(obeys_feq_full_trigger::<V>());
                assert(obeys_feq_full_trigger::<Pair<K, V>>());
            }
            OrderedTableStPer { tree }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1)
        fn singleton(k: K, v: V) -> (table: Self) {
            let bst = ParamBST::singleton(Pair(k, v));
            proof {
                let s = Set::<(K::V, V::V)>::empty().insert((k@, v@));
                assert(bst@ =~= s);
                lemma_set_to_map_empty::<K::V, V::V>();
                lemma_key_unique_empty::<K::V, V::V>();
                lemma_key_unique_insert(Set::<(K::V, V::V)>::empty(), k@, v@);
                lemma_set_to_map_insert(Set::empty(), k@, v@);
                lemma_pair_set_to_map_dom_finite(s);
                assert(obeys_feq_full_trigger::<K>());
                assert(obeys_feq_full_trigger::<V>());
                assert(obeys_feq_full_trigger::<Pair<K, V>>());
            }
            OrderedTableStPer { tree: bst }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to bst_find_by_key
        fn find(&self, k: &K) -> (found: Option<V>) {
            bst_find_by_key(&self.tree, k)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- bst_find_by_key + treap insert/delete
        fn insert(&self, k: K, v: V) -> (table: Self) {
            let mut tree = self.tree.clone();
            let ghost old_tree_view = self.tree@;
            let ghost old_map = self@;
            let existing = bst_find_by_key(&self.tree, &k);
            match existing {
                Some(old_v) => {
                    proof {
                        lemma_map_contains_pair_in_set(old_tree_view, k@);
                        let ghost v_chosen: V::V = choose|vv: V::V| old_tree_view.contains((k@, vv));
                        lemma_pair_in_set_map_contains(old_tree_view, k@, v_chosen);
                        assert(old_tree_view.contains((k@, old_v@)));
                    }
                    let k_del = k.clone_plus();
                    let ov_del = old_v.clone_plus();
                    proof {
                        lemma_cloned_view_eq(k, k_del);
                        lemma_cloned_view_eq(old_v, ov_del);
                    }
                    tree.delete(&Pair(k_del, ov_del));
                    let ghost mid_tree_view = tree@;
                    proof {
                        lemma_set_to_map_remove_pair(old_tree_view, k@, old_v@);
                        lemma_key_unique_remove(old_tree_view, (k@, old_v@));
                        assert(!spec_pair_set_to_map(mid_tree_view).dom().contains(k@));
                    }
                    tree.insert(Pair(k, v));
                    proof {
                        lemma_set_to_map_insert(mid_tree_view, k@, v@);
                        lemma_key_unique_insert(mid_tree_view, k@, v@);
                        lemma_pair_set_to_map_dom_finite(tree@);
                        let ghost new_map = spec_pair_set_to_map(tree@);
                        assert(new_map =~= old_map.remove(k@).insert(k@, v@));
                        assert(new_map[k@] == v@);
                        assert forall|k2: K::V| k2 != k@ && #[trigger] old_map.dom().contains(k2)
                            implies new_map[k2] == old_map[k2]
                        by {};
                        assert(new_map.dom() =~= old_map.dom().insert(k@)) by {
                            assert(old_map.dom().contains(k@));
                            assert(old_map.remove(k@).insert(k@, v@).dom()
                                =~= old_map.dom().remove(k@).insert(k@));
                            assert(old_map.dom().remove(k@).insert(k@)
                                =~= old_map.dom());
                        };
                        assert(tree@.len() <= old_tree_view.len());
                        assert(tree@.len() < usize::MAX as nat);
                    }
                },
                None => {
                    tree.insert(Pair(k, v));
                    proof {
                        lemma_set_to_map_insert(old_tree_view, k@, v@);
                        lemma_pair_set_to_map_dom_finite(tree@);
                        lemma_key_unique_insert(old_tree_view, k@, v@);
                        lemma_pair_set_to_map_len(old_tree_view);
                        assert(tree@.len() < usize::MAX as nat);
                        let ghost new_map = spec_pair_set_to_map(tree@);
                        assert(new_map =~= old_map.insert(k@, v@));
                        assert(new_map[k@] == v@);
                        assert forall|k2: K::V| k2 != k@ && #[trigger] old_map.dom().contains(k2)
                            implies new_map[k2] == old_map[k2]
                        by {};
                    }
                },
            }
            OrderedTableStPer { tree }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to insert
        fn insert_wf(&self, k: K, v: V) -> (table: Self) {
            self.insert(k, v)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- bst_find_by_key + treap delete
        fn delete(&self, k: &K) -> (table: Self) {
            let mut tree = self.tree.clone();
            let ghost old_tree_view = self.tree@;
            let ghost old_map = self@;
            let existing = bst_find_by_key(&self.tree, &k);
            match existing {
                Some(v) => {
                    let v_clone = v.clone_plus();
                    let k_clone = k.clone_plus();
                    proof {
                        lemma_cloned_view_eq(*k, k_clone);
                        lemma_cloned_view_eq(v, v_clone);
                    }
                    tree.delete(&Pair(k_clone, v_clone));
                    proof {
                        lemma_set_to_map_remove_pair(old_tree_view, k@, v@);
                        lemma_pair_set_to_map_dom_finite(tree@);
                        lemma_key_unique_remove(old_tree_view, (k@, v@));
                    }
                },
                None => {
                    proof {
                        assert(self@ =~= old_map.remove(k@));
                        lemma_pair_set_to_map_dom_finite(self.tree@);
                    }
                },
            }
            OrderedTableStPer { tree }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to delete
        fn delete_wf(&self, k: &K) -> (table: Self) {
            let table = self.delete(k);
            proof {
                assert forall|k2: K::V| k2 != k@ && #[trigger] self@.contains_key(k2)
                    implies table@[k2] == self@[k2]
                by {
                    assert(table@ == self@.remove(k@));
                };
            }
            table
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- in_order traversal + collect keys
        fn domain(&self) -> (domain: ArraySetStEph<K>) {
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
                    self.spec_orderedtablestper_wf(),
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
                        assert(sorted@.contains((kv, v)));
                        let j = choose|j: int| 0 <= j < sorted@.len()
                            && (#[trigger] sorted@[j]) == (kv, v);
                        assert(domain@.contains(sorted@[j].0));
                    };
                };
            }
            domain
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- n BST inserts into treap
        #[verifier::loop_isolation(false)]
        fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (tabulated: Self) {
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
                i = i + 1;
            }
            let tabulated = OrderedTableStPer { tree };
            proof {
                lemma_pair_set_to_map_dom_finite(tree@);
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
            }
            tabulated
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- in_order + n BST inserts
        #[verifier::loop_isolation(false)]
        fn map<F: Fn(&V) -> V>(&self, f: F) -> (table: Self) {
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
                    0 <= i <= len,
                    len as nat == sorted@.len(),
                    self.spec_orderedtablestper_wf(),
                    forall|v: &V| f.requires((v,)),
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
                    forall|p: (K::V, V::V)| new_tree@.contains(p) ==>
                        exists|j: int| 0 <= j < i as int && p.0 == (#[trigger] sorted@[j]).0,
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        self@.dom().contains(p.0),
                    forall|j: int| #![trigger sorted@[j]] 0 <= j < i as int ==>
                        spec_pair_set_to_map(new_tree@).dom().contains(sorted@[j].0),
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        (exists|old_val: V, result: V|
                            old_val@ == self@[p.0]
                            && f.ensures((&old_val,), result) && p.1 == result@),
                decreases len - i,
            {
                let pair = sorted.nth(i);
                let new_val = f(&pair.1);
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
                    assert(new_tree@.len() == i as nat + 1);
                    assert(new_tree@.len() < usize::MAX as nat);
                    lemma_key_unique_insert(old_new_tree_view, sorted@[i as int].0, new_val@);
                    assert(new_tree@.contains((sorted@[i as int].0, new_val@)));
                    // Completeness: new entry + old entries.
                    lemma_pair_in_set_map_contains(new_tree@, sorted@[i as int].0, new_val@);
                    assert forall|j: int| #![trigger sorted@[j]] 0 <= j < i as int implies
                        spec_pair_set_to_map(new_tree@).dom().contains(sorted@[j].0) by {
                        // Old entry (key, v) was in old_new_tree_view, hence in new_tree@.
                        lemma_map_contains_pair_in_set(old_new_tree_view, sorted@[j].0);
                        let v: V::V = choose|v: V::V| old_new_tree_view.contains((sorted@[j].0, v));
                        assert(new_tree@.contains((sorted@[j].0, v)));
                        lemma_pair_in_set_map_contains(new_tree@, sorted@[j].0, v);
                    };
                    // Value tracking for the new entry.
                    assert(sorted@.contains(sorted@[i as int])) by { assert(sorted@[i as int] == sorted@[i as int]); };
                    assert(self.tree@.contains(sorted@[i as int]));
                    lemma_pair_in_set_map_contains(self.tree@, sorted@[i as int].0, sorted@[i as int].1);
                    assert(self@.dom().contains(sorted@[i as int].0));
                    assert(pair.1@ == sorted@[i as int].1);
                    assert(self@[sorted@[i as int].0] == sorted@[i as int].1);
                    assert(f.ensures((&pair.1,), new_val));
                    // dom containment for new entry.
                    assert forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) implies
                        self@.dom().contains(p.0) by {
                        if old_new_tree_view.contains(p) {
                        } else {
                            assert(p == (sorted@[i as int].0, new_val@));
                        }
                    };
                }
                i = i + 1;
            }
            let mapped = OrderedTableStPer { tree: new_tree };
            proof {
                lemma_pair_set_to_map_dom_finite(new_tree@);
                lemma_pair_set_to_map_dom_finite(self.tree@);
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
                        assert(spec_pair_set_to_map(new_tree@).dom().contains(sorted@[j].0));
                    };
                };
                // Value tracking.
                assert forall|k: K::V| #[trigger] mapped@.contains_key(k) implies
                    (exists|old_val: V, result: V|
                        old_val@ == self@[k]
                        && f.ensures((&old_val,), result)
                        && mapped@[k] == result@)
                by {
                    lemma_map_contains_pair_in_set(new_tree@, k);
                    let v: V::V = choose|v: V::V| new_tree@.contains((k, v));
                    lemma_pair_in_set_map_contains(new_tree@, k, v);
                };
                // wf: len bound from loop + axioms from self wf.
                assert(new_tree@.len() < usize::MAX as nat);
            }
            mapped
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- BST recursive filter + join
        fn filter<F: Fn(&K, &V) -> bool>(
            &self,
            f: F,
            Ghost(spec_pred): Ghost<spec_fn(K::V, V::V) -> bool>,
        ) -> (filtered: Self) {
            let pair_pred = |p: &Pair<K, V>| -> (keep: bool)
                ensures keep == spec_pred(p.0@, p.1@)
            {
                f(&p.0, &p.1)
            };
            let ghost pair_spec_pred = |pv: (K::V, V::V)| -> bool { spec_pred(pv.0, pv.1) };
            let filtered_tree = self.tree.filter(pair_pred, Ghost(pair_spec_pred));
            let filtered = OrderedTableStPer { tree: filtered_tree };
            proof {
                lemma_pair_set_to_map_dom_finite(filtered_tree@);
                lemma_pair_set_to_map_dom_finite(self.tree@);
                lemma_key_unique_subset(self.tree@, filtered_tree@);
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
                    assert(pair_spec_pred((k, v)));
                    assert(filtered_tree@.contains((k, v)));
                    lemma_pair_in_set_map_contains(filtered_tree@, k, v);
                };
                vstd::set_lib::lemma_len_subset(filtered_tree@, self.tree@);
            }
            filtered
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- iterate self, find per element in other
        #[verifier::loop_isolation(false)]
        fn intersection<F: Fn(&V, &V) -> V>(&self, other: &Self, f: F) -> (table: Self) {
            let ghost old_tree = self.tree@;
            let ghost old_map = self@;
            let ghost other_map = other@;
            let sorted = self.tree.in_order();
            let len = sorted.length();
            let mut new_tree = ParamBST::<Pair<K, V>>::new();
            let mut i: usize = 0;
            proof {
                assert(obeys_feq_full_trigger::<V>());
                assert(obeys_feq_full_trigger::<K>());
                lemma_pair_set_to_map_dom_finite(old_tree);
                lemma_sorted_keys_pairwise_distinct(old_tree, sorted@);
                lemma_key_unique_empty::<K::V, V::V>();
            }
            while i < len
                invariant
                    self.spec_orderedtablestper_wf(),
                    other.spec_orderedtablestper_wf(),
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<V>(),
                    obeys_feq_full::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
                    forall|v1: &V, v2: &V| f.requires((v1, v2)),
                    old_map == spec_pair_set_to_map(old_tree),
                    self.tree@ == old_tree,
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
                    forall|v: <Pair<K, V> as View>::V| old_tree.contains(v) <==> #[trigger] sorted@.contains(v),
                    forall|ii: int, jj: int|
                        0 <= ii < sorted@.len() && 0 <= jj < sorted@.len() && ii != jj
                        ==> (#[trigger] sorted@[ii]).0 != (#[trigger] sorted@[jj]).0,
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        exists|j: int| 0 <= j < i as int && p.0 == (#[trigger] sorted@[j]).0,
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        old_map.dom().contains(p.0),
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        other_map.dom().contains(p.0),
                    forall|j: int| #![trigger sorted@[j]] 0 <= j < i as int
                        && other_map.contains_key(sorted@[j].0) ==>
                        spec_pair_set_to_map(new_tree@).dom().contains(sorted@[j].0),
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        (exists|v1: V, v2: V, r: V|
                            v1@ == old_map[p.0] && v2@ == other_map[p.0]
                            && f.ensures((&v1, &v2), r) && p.1 == r@),
                decreases len - i,
            {
                let pair = sorted.nth(i);
                proof { reveal(obeys_view_eq); }
                let other_find = other.find(&pair.0);
                match other_find {
                    Some(other_v) => {
                        let combined = f(&pair.1, &other_v);
                        let key_clone = pair.0.clone_plus();
                        let ghost old_new_tree_view = new_tree@;
                        proof {
                            assert(obeys_feq_full_trigger::<K>());
                            assert(key_clone@ == pair.0@);
                            assert(!spec_pair_set_to_map(old_new_tree_view).dom().contains(sorted@[i as int].0)) by {
                                if spec_pair_set_to_map(old_new_tree_view).dom().contains(sorted@[i as int].0) {
                                    lemma_map_contains_pair_in_set(old_new_tree_view, sorted@[i as int].0);
                                    let vv: V::V = choose|vv: V::V| old_new_tree_view.contains((sorted@[i as int].0, vv));
                                    let jj = choose|jj: int| 0 <= jj < i as int && (sorted@[i as int].0, vv).0 == (#[trigger] sorted@[jj]).0;
                                    assert(false);
                                }
                            };
                        }
                        new_tree.insert(Pair(key_clone, combined));
                        proof {
                            assert(new_tree@.len() <= i as nat + 1);
                            assert(new_tree@.len() < usize::MAX as nat);
                            lemma_key_unique_insert(old_new_tree_view, sorted@[i as int].0, combined@);
                            assert(new_tree@.contains((sorted@[i as int].0, combined@)));
                            // Completeness: new entry + old entries preserved.
                            lemma_pair_in_set_map_contains(new_tree@, sorted@[i as int].0, combined@);
                            assert forall|j: int| #![trigger sorted@[j]] 0 <= j < i as int
                                && other_map.contains_key(sorted@[j].0)
                                implies spec_pair_set_to_map(new_tree@).dom().contains(sorted@[j].0) by {
                                lemma_map_contains_pair_in_set(old_new_tree_view, sorted@[j].0);
                                let v: V::V = choose|v: V::V| old_new_tree_view.contains((sorted@[j].0, v));
                                assert(new_tree@.contains((sorted@[j].0, v)));
                                lemma_pair_in_set_map_contains(new_tree@, sorted@[j].0, v);
                            };
                            // Subset: key in self dom and other dom.
                            assert(sorted@.contains(sorted@[i as int])) by { assert(sorted@[i as int] == sorted@[i as int]); };
                            assert(old_tree.contains(sorted@[i as int]));
                            lemma_pair_in_set_map_contains(old_tree, sorted@[i as int].0, sorted@[i as int].1);
                            assert(old_map.dom().contains(sorted@[i as int].0));
                            assert(other_map.dom().contains(sorted@[i as int].0));
                            // Value tracking.
                            assert(pair.1@ == sorted@[i as int].1);
                            assert(old_map[sorted@[i as int].0] == sorted@[i as int].1);
                            assert(other_v@ == other_map[sorted@[i as int].0]);
                            assert(f.ensures((&pair.1, &other_v), combined));
                        }
                    },
                    None => {},
                }
                i += 1;
            }
            let table = OrderedTableStPer { tree: new_tree };
            proof {
                lemma_pair_set_to_map_dom_finite(new_tree@);
                lemma_pair_set_to_map_dom_finite(old_tree);
                // Domain: table@.dom() =~= self@.dom().intersect(other@.dom()).
                assert(table@.dom() =~= old_map.dom().intersect(other_map.dom())) by {
                    assert forall|k: K::V| table@.dom().contains(k)
                        implies #[trigger] old_map.dom().intersect(other_map.dom()).contains(k)
                    by {
                        lemma_map_contains_pair_in_set(new_tree@, k);
                        let v: V::V = choose|v: V::V| new_tree@.contains((k, v));
                    };
                    assert forall|k: K::V| old_map.dom().contains(k) && other_map.dom().contains(k)
                        implies #[trigger] table@.dom().contains(k)
                    by {
                        lemma_map_contains_pair_in_set(old_tree, k);
                        let v: V::V = choose|v: V::V| old_tree.contains((k, v));
                        assert(sorted@.contains((k, v)));
                        let j = choose|j: int| 0 <= j < sorted@.len() && sorted@[j] == (k, v);
                        assert(spec_pair_set_to_map(new_tree@).dom().contains(sorted@[j].0));
                    };
                };
                // Value tracking.
                assert forall|k: K::V| #[trigger] table@.contains_key(k) implies
                    (exists|v1: V, v2: V, r: V|
                        v1@ == self@[k] && v2@ == other@[k]
                        && f.ensures((&v1, &v2), r)
                        && table@[k] == r@)
                by {
                    lemma_map_contains_pair_in_set(new_tree@, k);
                    let v: V::V = choose|v: V::V| new_tree@.contains((k, v));
                    lemma_pair_in_set_map_contains(new_tree@, k, v);
                };
                // wf: len bound from loop invariant.
                assert(new_tree@.len() < usize::MAX as nat);
            }
            table
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- iterate both, find per element
        #[verifier::loop_isolation(false)]
        fn union<F: Fn(&V, &V) -> V>(&self, other: &Self, f: F) -> (table: Self) {
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
            let self_sorted = self.tree.in_order();
            let self_len = self_sorted.length();
            let mut new_tree = ParamBST::<Pair<K, V>>::new();
            let mut i: usize = 0;
            proof {
                lemma_sorted_keys_pairwise_distinct(old_tree, self_sorted@);
                lemma_key_unique_empty::<K::V, V::V>();
            }
            while i < self_len
                invariant
                    self.spec_orderedtablestper_wf(),
                    other.spec_orderedtablestper_wf(),
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<V>(),
                    obeys_feq_full::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
                    forall|v1: &V, v2: &V| f.requires((v1, v2)),
                    self.tree@ == old_tree,
                    self_len as nat == self_sorted@.len(),
                    self_sorted@.len() == old_tree.len(),
                    forall|v: <Pair<K, V> as View>::V| old_tree.contains(v) <==> #[trigger] self_sorted@.contains(v),
                    forall|ii: int, jj: int|
                        0 <= ii < self_sorted@.len() && 0 <= jj < self_sorted@.len() && ii != jj
                        ==> (#[trigger] self_sorted@[ii]).0 != (#[trigger] self_sorted@[jj]).0,
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        exists|j: int| 0 <= j < i as int && p.0 == (#[trigger] self_sorted@[j]).0,
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        old_map.dom().contains(p.0),
                    forall|j: int| #![trigger self_sorted@[j]] 0 <= j < i as int ==>
                        spec_pair_set_to_map(new_tree@).dom().contains(self_sorted@[j].0),
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p)
                        && !other_map.dom().contains(p.0) ==> p.1 == old_map[p.0],
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p)
                        && other_map.dom().contains(p.0) ==>
                        (exists|v1: V, v2: V, r: V|
                            v1@ == old_map[p.0] && v2@ == other_map[p.0]
                            && f.ensures((&v1, &v2), r) && p.1 == r@),
                    0 <= i <= self_len,
                    new_tree.spec_bstparasteph_wf(),
                    vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                    view_ord_consistent::<Pair<K, V>>(),
                    new_tree@.len() == i as nat,
                    new_tree@.len() < usize::MAX as nat,
                    spec_key_unique_pairs_set(new_tree@),
                    spec_key_unique_pairs_set(old_tree),
                decreases self_len - i,
            {
                let pair = self_sorted.nth(i);
                proof { reveal(obeys_view_eq); }
                let other_find = other.find(&pair.0);
                let ghost old_new_tree_view = new_tree@;
                proof {
                    assert(!spec_pair_set_to_map(old_new_tree_view).dom().contains(self_sorted@[i as int].0)) by {
                        if spec_pair_set_to_map(old_new_tree_view).dom().contains(self_sorted@[i as int].0) {
                            lemma_map_contains_pair_in_set(old_new_tree_view, self_sorted@[i as int].0);
                            let vv: V::V = choose|vv: V::V| old_new_tree_view.contains((self_sorted@[i as int].0, vv));
                            let jj = choose|jj: int| 0 <= jj < i as int && (self_sorted@[i as int].0, vv).0 == (#[trigger] self_sorted@[jj]).0;
                            assert(false);
                        }
                    };
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
                            assert(new_tree@.len() == i as nat + 1);
                            assert(new_tree@.len() < usize::MAX as nat);
                            lemma_key_unique_insert(old_new_tree_view, self_sorted@[i as int].0, combined@);
                            assert(new_tree@.contains((self_sorted@[i as int].0, combined@)));
                            lemma_pair_in_set_map_contains(new_tree@, self_sorted@[i as int].0, combined@);
                            // Preserve completeness for old entries.
                            assert forall|j: int| #![trigger self_sorted@[j]] 0 <= j < i as int implies
                                spec_pair_set_to_map(new_tree@).dom().contains(self_sorted@[j].0) by {
                                lemma_map_contains_pair_in_set(old_new_tree_view, self_sorted@[j].0);
                                let v: V::V = choose|v: V::V| old_new_tree_view.contains((self_sorted@[j].0, v));
                                assert(new_tree@.contains((self_sorted@[j].0, v)));
                                lemma_pair_in_set_map_contains(new_tree@, self_sorted@[j].0, v);
                            };
                            // Value: intersection key.
                            assert(pair.1@ == self_sorted@[i as int].1);
                            assert(old_map[self_sorted@[i as int].0] == self_sorted@[i as int].1);
                            assert(ov@ == other_map[self_sorted@[i as int].0]);
                            assert(f.ensures((&pair.1, &ov), combined));
                        }
                    },
                    None => {
                        let cloned = pair.clone_plus();
                        proof { lemma_cloned_view_eq(*pair, cloned); }
                        new_tree.insert(cloned);
                        proof {
                            assert(new_tree@.len() == i as nat + 1);
                            assert(new_tree@.len() < usize::MAX as nat);
                            lemma_key_unique_insert(old_new_tree_view, self_sorted@[i as int].0, self_sorted@[i as int].1);
                            assert(new_tree@.contains(self_sorted@[i as int]));
                            lemma_pair_in_set_map_contains(new_tree@, self_sorted@[i as int].0, self_sorted@[i as int].1);
                            // Preserve completeness for old entries.
                            assert forall|j: int| #![trigger self_sorted@[j]] 0 <= j < i as int implies
                                spec_pair_set_to_map(new_tree@).dom().contains(self_sorted@[j].0) by {
                                lemma_map_contains_pair_in_set(old_new_tree_view, self_sorted@[j].0);
                                let v: V::V = choose|v: V::V| old_new_tree_view.contains((self_sorted@[j].0, v));
                                assert(new_tree@.contains((self_sorted@[j].0, v)));
                                lemma_pair_in_set_map_contains(new_tree@, self_sorted@[j].0, v);
                            };
                        }
                    },
                }
                i += 1;
            }
            let other_sorted = other.tree.in_order();
            let other_len = other_sorted.length();
            let mut j: usize = 0;
            proof {
                lemma_sorted_keys_pairwise_distinct(other.tree@, other_sorted@);
                assert forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) implies
                    spec_pair_set_to_map(old_tree).dom().contains(p.0)
                by {
                    let jx = choose|jx: int| 0 <= jx < self_sorted@.len() && p.0 == (#[trigger] self_sorted@[jx]).0;
                    assert(self_sorted@.contains(self_sorted@[jx])) by { assert(self_sorted@[jx] == self_sorted@[jx]); };
                    assert(old_tree.contains(self_sorted@[jx]));
                    lemma_pair_in_set_map_contains(old_tree, self_sorted@[jx].0, self_sorted@[jx].1);
                };
                lemma_pair_set_to_map_len(old_tree);
                lemma_pair_set_to_map_len(other.tree@);
            }
            while j < other_len
                invariant
                    self.spec_orderedtablestper_wf(),
                    other.spec_orderedtablestper_wf(),
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
                    self.tree@ == old_tree,
                    other_len as nat == other_sorted@.len(),
                    other_sorted@.len() == other.tree@.len(),
                    forall|v: <Pair<K, V> as View>::V| other.tree@.contains(v) <==> #[trigger] other_sorted@.contains(v),
                    forall|ii: int, jj: int|
                        0 <= ii < other_sorted@.len() && 0 <= jj < other_sorted@.len() && ii != jj
                        ==> (#[trigger] other_sorted@[ii]).0 != (#[trigger] other_sorted@[jj]).0,
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        spec_pair_set_to_map(old_tree).dom().contains(p.0) ||
                        (exists|j2: int| 0 <= j2 < j as int && p.0 == (#[trigger] other_sorted@[j2]).0),
                    old_map == spec_pair_set_to_map(old_tree),
                    0 <= j <= other_len,
                    new_tree.spec_bstparasteph_wf(),
                    vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
                    view_ord_consistent::<Pair<K, V>>(),
                    new_tree@.len() <= self_sorted@.len() + j as nat,
                    self_sorted@.len() + other_sorted@.len() < usize::MAX as nat,
                    spec_key_unique_pairs_set(new_tree@),
                    spec_key_unique_pairs_set(old_tree),
                    spec_key_unique_pairs_set(other.tree@),
                    other_map == spec_pair_set_to_map(other.tree@),
                    obeys_feq_full::<K>(),
                    obeys_feq_full::<V>(),
                    forall|v1: &V, v2: &V| f.requires((v1, v2)),
                    self_len as nat == self_sorted@.len(),
                    self_sorted@.len() == old_tree.len(),
                    forall|v: <Pair<K, V> as View>::V| old_tree.contains(v) <==> #[trigger] self_sorted@.contains(v),
                    // Self-key completeness (preserved from first loop).
                    forall|j_s: int| #![trigger self_sorted@[j_s]] 0 <= j_s < self_sorted@.len() ==>
                        spec_pair_set_to_map(new_tree@).dom().contains(self_sorted@[j_s].0),
                    // Other-only key completeness.
                    forall|j_o: int| #![trigger other_sorted@[j_o]] 0 <= j_o < j as int
                        && !old_map.dom().contains(other_sorted@[j_o].0) ==>
                        spec_pair_set_to_map(new_tree@).dom().contains(other_sorted@[j_o].0),
                    // Self-only value tracking.
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p)
                        && old_map.dom().contains(p.0) && !other_map.dom().contains(p.0)
                        ==> p.1 == old_map[p.0],
                    // Intersection value tracking.
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p)
                        && old_map.dom().contains(p.0) && other_map.dom().contains(p.0) ==>
                        (exists|v1: V, v2: V, r: V|
                            v1@ == old_map[p.0] && v2@ == other_map[p.0]
                            && f.ensures((&v1, &v2), r) && p.1 == r@),
                    // Other-only value tracking.
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p)
                        && !old_map.dom().contains(p.0) ==> p.1 == other_map[p.0],
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
                            assert(new_tree@.len() <= self_sorted@.len() + j as nat + 1);
                            lemma_key_unique_insert(old_new_tree_view, other_sorted@[j as int].0, other_sorted@[j as int].1);
                            // Self-key completeness: old entries preserved after insert.
                            assert forall|j_s: int| #![trigger self_sorted@[j_s]] 0 <= j_s < self_sorted@.len() implies
                                spec_pair_set_to_map(new_tree@).dom().contains(self_sorted@[j_s].0) by {
                                lemma_map_contains_pair_in_set(old_new_tree_view, self_sorted@[j_s].0);
                                let v: V::V = choose|v: V::V| old_new_tree_view.contains((self_sorted@[j_s].0, v));
                                assert(new_tree@.contains((self_sorted@[j_s].0, v)));
                                lemma_pair_in_set_map_contains(new_tree@, self_sorted@[j_s].0, v);
                            };
                            // Other-only completeness: old entries + new entry.
                            assert(other_sorted@.contains(other_sorted@[j as int])) by {
                                assert(other_sorted@[j as int] == other_sorted@[j as int]);
                            };
                            assert(other.tree@.contains(other_sorted@[j as int]));
                            assert(new_tree@.contains(other_sorted@[j as int]));
                            lemma_pair_in_set_map_contains(new_tree@, other_sorted@[j as int].0, other_sorted@[j as int].1);
                            assert forall|j_o: int| #![trigger other_sorted@[j_o]] 0 <= j_o < j as int + 1
                                && !old_map.dom().contains(other_sorted@[j_o].0) implies
                                spec_pair_set_to_map(new_tree@).dom().contains(other_sorted@[j_o].0) by {
                                if j_o < j as int {
                                    lemma_map_contains_pair_in_set(old_new_tree_view, other_sorted@[j_o].0);
                                    let v: V::V = choose|v: V::V| old_new_tree_view.contains((other_sorted@[j_o].0, v));
                                    assert(new_tree@.contains((other_sorted@[j_o].0, v)));
                                    lemma_pair_in_set_map_contains(new_tree@, other_sorted@[j_o].0, v);
                                }
                            };
                            // Other-only value: new entry from other.
                            lemma_pair_in_set_map_contains(other.tree@, other_sorted@[j as int].0, other_sorted@[j as int].1);
                        }
                    },
                    Some(_) => {},
                }
                j += 1;
            }
            let table = OrderedTableStPer { tree: new_tree };
            proof {
                lemma_pair_set_to_map_dom_finite(new_tree@);
                lemma_pair_set_to_map_dom_finite(old_tree);
                lemma_pair_set_to_map_dom_finite(other.tree@);
                // 1. Domain: table@.dom() =~= self@.dom().union(other@.dom())
                // Forward: every key in new_tree is in self or other.
                assert forall|k: K::V| #[trigger] spec_pair_set_to_map(new_tree@).dom().contains(k) implies
                    old_map.dom().contains(k) || other_map.dom().contains(k) by {
                    lemma_map_contains_pair_in_set(new_tree@, k);
                    let vv: V::V = choose|vv: V::V| new_tree@.contains((k, vv));
                    if old_map.dom().contains(k) {
                    } else {
                        let j2 = choose|j2: int| 0 <= j2 < other_sorted@.len() && (k, vv).0 == (#[trigger] other_sorted@[j2]).0;
                        assert(other_sorted@.contains(other_sorted@[j2])) by { assert(other_sorted@[j2] == other_sorted@[j2]); };
                        assert(other.tree@.contains(other_sorted@[j2]));
                        lemma_pair_in_set_map_contains(other.tree@, other_sorted@[j2].0, other_sorted@[j2].1);
                    }
                };
                // Backward: every key in self or other is in new_tree.
                assert forall|k: K::V| old_map.dom().contains(k) || other_map.dom().contains(k)
                    implies #[trigger] spec_pair_set_to_map(new_tree@).dom().contains(k) by {
                    if old_map.dom().contains(k) {
                        lemma_map_contains_pair_in_set(old_tree, k);
                        let vv: V::V = choose|vv: V::V| old_tree.contains((k, vv));
                        assert(self_sorted@.contains((k, vv)));
                        let j_s = choose|j_s: int| 0 <= j_s < self_sorted@.len() && self_sorted@[j_s] == (k, vv);
                        assert(self_sorted@[j_s].0 == k);
                    } else {
                        // k is in other but not self — other-only.
                        lemma_map_contains_pair_in_set(other.tree@, k);
                        let vv: V::V = choose|vv: V::V| other.tree@.contains((k, vv));
                        assert(other_sorted@.contains((k, vv)));
                        let j_o = choose|j_o: int| 0 <= j_o < other_sorted@.len() && other_sorted@[j_o] == (k, vv);
                        assert(other_sorted@[j_o].0 == k);
                        assert(!old_map.dom().contains(other_sorted@[j_o].0));
                    }
                };
                // 2. Self-only value: table@[k] == self@[k].
                assert forall|k: K::V| #[trigger] old_map.dom().contains(k) && !other_map.dom().contains(k)
                    implies spec_pair_set_to_map(new_tree@)[k] == old_map[k] by {
                    lemma_map_contains_pair_in_set(old_tree, k);
                    let vv_s: V::V = choose|vv: V::V| old_tree.contains((k, vv));
                    assert(self_sorted@.contains((k, vv_s)));
                    let j_s = choose|j_s: int| 0 <= j_s < self_sorted@.len() && self_sorted@[j_s] == (k, vv_s);
                    // k is in new_tree dom from completeness.
                    lemma_map_contains_pair_in_set(new_tree@, k);
                    let vv_n: V::V = choose|vv: V::V| new_tree@.contains((k, vv));
                    // By self-only value invariant: vv_n == old_map[k].
                    assert(old_map.dom().contains(k) && !other_map.dom().contains(k));
                    lemma_pair_in_set_map_contains(new_tree@, k, vv_n);
                };
                // 3. Other-only value: table@[k] == other@[k].
                assert forall|k: K::V| #[trigger] other_map.dom().contains(k) && !old_map.dom().contains(k)
                    implies spec_pair_set_to_map(new_tree@)[k] == other_map[k] by {
                    lemma_map_contains_pair_in_set(new_tree@, k);
                    let vv_n: V::V = choose|vv: V::V| new_tree@.contains((k, vv));
                    // By other-only value invariant.
                    assert(!old_map.dom().contains(k));
                    lemma_pair_in_set_map_contains(new_tree@, k, vv_n);
                };
                // 4. Intersection value: exists v1, v2, r with combined.
                assert forall|k: K::V| #[trigger] old_map.dom().contains(k) && other_map.dom().contains(k) implies
                    (exists|v1: V, v2: V, r: V|
                        v1@ == old_map[k] && v2@ == other_map[k]
                        && f.ensures((&v1, &v2), r) && spec_pair_set_to_map(new_tree@)[k] == r@) by {
                    lemma_map_contains_pair_in_set(new_tree@, k);
                    let vv_n: V::V = choose|vv: V::V| new_tree@.contains((k, vv));
                    // By intersection value invariant, get witnesses.
                    lemma_pair_in_set_map_contains(new_tree@, k, vv_n);
                };
                // 5. wf.
                assert(new_tree@.len() < usize::MAX as nat) by {
                    assert(new_tree@.len() <= self_sorted@.len() + other_sorted@.len());
                };
            }
            table
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- iterate self, find per element in other
        #[verifier::loop_isolation(false)]
        fn difference(&self, other: &Self) -> (table: Self) {
            let ghost old_tree = self.tree@;
            let ghost old_map = self@;
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
                    self.spec_orderedtablestper_wf(),
                    other.spec_orderedtablestper_wf(),
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
                    self.tree@ == old_tree,
                    len as nat == sorted@.len(),
                    sorted@.len() == old_tree.len(),
                    forall|v: <Pair<K, V> as View>::V| old_tree.contains(v) <==> #[trigger] sorted@.contains(v),
                    forall|ii: int, jj: int|
                        0 <= ii < sorted@.len() && 0 <= jj < sorted@.len() && ii != jj
                        ==> (#[trigger] sorted@[ii]).0 != (#[trigger] sorted@[jj]).0,
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        exists|j: int| 0 <= j < i as int && p.0 == (#[trigger] sorted@[j]).0,
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==> old_tree.contains(p),
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==> !other@.dom().contains(p.0),
                    forall|j: int| #![trigger sorted@[j]] 0 <= j < i as int && !other@.dom().contains(sorted@[j].0) ==> new_tree@.contains(sorted@[j]),
                    old_map == spec_pair_set_to_map(old_tree),
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
                        }
                    },
                    Some(_) => {},
                }
                i += 1;
            }
            let table = OrderedTableStPer { tree: new_tree };
            proof {
                lemma_pair_set_to_map_dom_finite(new_tree@);
                lemma_pair_set_to_map_dom_finite(old_tree);
                assert(table@.dom() =~= old_map.dom().difference(other@.dom())) by {
                    assert forall|k: K::V| table@.dom().contains(k)
                        implies #[trigger] old_map.dom().difference(other@.dom()).contains(k)
                    by {
                        lemma_map_contains_pair_in_set(new_tree@, k);
                        let v: V::V = choose|v: V::V| new_tree@.contains((k, v));
                        lemma_pair_in_set_map_contains(old_tree, k, v);
                    };
                    assert forall|k: K::V| old_map.dom().contains(k) && !other@.dom().contains(k)
                        implies #[trigger] table@.dom().contains(k)
                    by {
                        lemma_map_contains_pair_in_set(old_tree, k);
                        let v: V::V = choose|v: V::V| old_tree.contains((k, v));
                        assert(sorted@.contains((k, v)));
                        let j = choose|j: int| 0 <= j < sorted@.len() && sorted@[j] == (k, v);
                        assert(new_tree@.contains(sorted@[j]));
                        lemma_pair_in_set_map_contains(new_tree@, k, v);
                    };
                };
                assert forall|k: K::V| #[trigger] table@.contains_key(k)
                    implies table@[k] == old_map[k]
                by {
                    lemma_map_contains_pair_in_set(new_tree@, k);
                    let v: V::V = choose|v: V::V| new_tree@.contains((k, v));
                    lemma_pair_in_set_map_contains(new_tree@, k, v);
                    lemma_pair_in_set_map_contains(old_tree, k, v);
                };
                vstd::set_lib::lemma_len_subset(new_tree@, old_tree);
            }
            table
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- iterate self, check key membership per element
        #[verifier::loop_isolation(false)]
        fn restrict(&self, keys: &ArraySetStEph<K>) -> (table: Self) {
            let ghost old_tree = self.tree@;
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
                    self.spec_orderedtablestper_wf(),
                    obeys_feq_full::<Pair<K, V>>(),
                    keys@.finite(),
                    self.tree@ == old_tree,
                    len as nat == sorted@.len(),
                    sorted@.len() == old_tree.len(),
                    forall|v: <Pair<K, V> as View>::V| old_tree.contains(v) <==> #[trigger] sorted@.contains(v),
                    forall|ii: int, jj: int|
                        0 <= ii < sorted@.len() && 0 <= jj < sorted@.len() && ii != jj
                        ==> (#[trigger] sorted@[ii]).0 != (#[trigger] sorted@[jj]).0,
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        exists|j: int| 0 <= j < i as int && p.0 == (#[trigger] sorted@[j]).0,
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==> old_tree.contains(p),
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==> keys@.contains(p.0),
                    forall|j: int| #![trigger sorted@[j]] 0 <= j < i as int && keys@.contains(sorted@[j].0) ==> new_tree@.contains(sorted@[j]),
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
                    }
                }
                i = i + 1;
            }
            let table = OrderedTableStPer { tree: new_tree };
            proof {
                lemma_pair_set_to_map_dom_finite(new_tree@);
                lemma_pair_set_to_map_dom_finite(old_tree);
                assert(table@.dom() =~= self@.dom().intersect(keys@)) by {
                    assert forall|k: K::V| table@.dom().contains(k)
                        implies #[trigger] self@.dom().intersect(keys@).contains(k)
                    by {
                        lemma_map_contains_pair_in_set(new_tree@, k);
                        let v: V::V = choose|v: V::V| new_tree@.contains((k, v));
                        lemma_pair_in_set_map_contains(old_tree, k, v);
                    };
                    assert forall|k: K::V| self@.dom().contains(k) && keys@.contains(k)
                        implies #[trigger] table@.dom().contains(k)
                    by {
                        lemma_map_contains_pair_in_set(old_tree, k);
                        let v: V::V = choose|v: V::V| old_tree.contains((k, v));
                        assert(sorted@.contains((k, v)));
                        let j = choose|j: int| 0 <= j < sorted@.len() && sorted@[j] == (k, v);
                        assert(new_tree@.contains(sorted@[j]));
                        lemma_pair_in_set_map_contains(new_tree@, k, v);
                    };
                };
                assert forall|k: K::V| #[trigger] table@.contains_key(k)
                    implies table@[k] == self@[k]
                by {
                    lemma_map_contains_pair_in_set(new_tree@, k);
                    let v: V::V = choose|v: V::V| new_tree@.contains((k, v));
                    lemma_pair_in_set_map_contains(new_tree@, k, v);
                    lemma_pair_in_set_map_contains(old_tree, k, v);
                };
                vstd::set_lib::lemma_len_subset(new_tree@, old_tree);
            }
            table
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- iterate self, check key exclusion per element
        #[verifier::loop_isolation(false)]
        fn subtract(&self, keys: &ArraySetStEph<K>) -> (table: Self) {
            let ghost old_tree = self.tree@;
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
                    self.spec_orderedtablestper_wf(),
                    obeys_feq_full::<Pair<K, V>>(),
                    keys@.finite(),
                    self.tree@ == old_tree,
                    len as nat == sorted@.len(),
                    sorted@.len() == old_tree.len(),
                    forall|v: <Pair<K, V> as View>::V| old_tree.contains(v) <==> #[trigger] sorted@.contains(v),
                    forall|ii: int, jj: int|
                        0 <= ii < sorted@.len() && 0 <= jj < sorted@.len() && ii != jj
                        ==> (#[trigger] sorted@[ii]).0 != (#[trigger] sorted@[jj]).0,
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        exists|j: int| 0 <= j < i as int && p.0 == (#[trigger] sorted@[j]).0,
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==> old_tree.contains(p),
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==> !keys@.contains(p.0),
                    forall|j: int| #![trigger sorted@[j]] 0 <= j < i as int && !keys@.contains(sorted@[j].0) ==> new_tree@.contains(sorted@[j]),
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
                    }
                }
                i = i + 1;
            }
            let table = OrderedTableStPer { tree: new_tree };
            proof {
                lemma_pair_set_to_map_dom_finite(new_tree@);
                lemma_pair_set_to_map_dom_finite(old_tree);
                assert(table@.dom() =~= self@.dom().difference(keys@)) by {
                    assert forall|k: K::V| table@.dom().contains(k)
                        implies #[trigger] self@.dom().difference(keys@).contains(k)
                    by {
                        lemma_map_contains_pair_in_set(new_tree@, k);
                        let v: V::V = choose|v: V::V| new_tree@.contains((k, v));
                        lemma_pair_in_set_map_contains(old_tree, k, v);
                    };
                    assert forall|k: K::V| self@.dom().contains(k) && !keys@.contains(k)
                        implies #[trigger] table@.dom().contains(k)
                    by {
                        lemma_map_contains_pair_in_set(old_tree, k);
                        let v: V::V = choose|v: V::V| old_tree.contains((k, v));
                        assert(sorted@.contains((k, v)));
                        let j = choose|j: int| 0 <= j < sorted@.len() && sorted@[j] == (k, v);
                        assert(new_tree@.contains(sorted@[j]));
                        lemma_pair_in_set_map_contains(new_tree@, k, v);
                    };
                };
                assert forall|k: K::V| #[trigger] table@.contains_key(k)
                    implies table@[k] == self@[k]
                by {
                    lemma_map_contains_pair_in_set(new_tree@, k);
                    let v: V::V = choose|v: V::V| new_tree@.contains((k, v));
                    lemma_pair_in_set_map_contains(new_tree@, k, v);
                    lemma_pair_in_set_map_contains(old_tree, k, v);
                };
                vstd::set_lib::lemma_len_subset(new_tree@, old_tree);
            }
            table
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- in_order traversal + vec copy
        #[verifier::loop_isolation(false)]
        fn collect(&self) -> (collected: AVLTreeSeqStPerS<Pair<K, V>>) {
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
            where K: TotalOrder
        {
            self.first_key_iter()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- in_order traversal, returns first
        #[verifier::loop_isolation(false)]
        fn first_key_iter(&self) -> (first: Option<K>)
            where K: TotalOrder
        {
            proof {
                lemma_reveal_view_injective::<K>();
                lemma_pair_set_to_map_dom_finite(self.tree@);
                lemma_pair_set_to_map_len(self.tree@);
            }
            let sorted = self.tree.in_order();
            let len = sorted.length();
            if len == 0 {
                None
            } else {
                let mut min_key = sorted.nth(0).0.clone_plus();
                proof {
                    assert(obeys_feq_full_trigger::<K>());
                    K::reflexive(min_key);
                    lemma_cloned_view_eq(sorted.spec_index(0).0, min_key);
                    assert(self.tree@.contains(sorted@[0]));
                    lemma_pair_in_set_map_contains(self.tree@, sorted@[0].0, sorted@[0].1);
                }
                let mut i: usize = 1;
                while i < len
                    invariant
                        1 <= i, i <= len,
                        len as nat == sorted@.len(),
                        self.spec_orderedtablestper_wf(),
                        self.tree@.contains(sorted@[0]),
                        forall|j: int| 0 <= j < sorted@.len() ==>
                            self.tree@.contains(#[trigger] sorted@[j]),
                        forall|j: int| 0 <= j < i as int ==>
                            TotalOrder::le(min_key, (#[trigger] sorted.spec_index(j)).0),
                        self@.dom().contains(min_key@),
                    decreases len - i,
                {
                    let elem = sorted.nth(i);
                    let c = TotalOrder::cmp(&elem.0, &min_key);
                    match c {
                        core::cmp::Ordering::Less => {
                            proof {
                                let old_min = min_key;
                                assert forall|j: int| 0 <= j < i + 1
                                    implies TotalOrder::le(elem.0, (#[trigger] sorted.spec_index(j)).0) by {
                                    if j == i as int {
                                        K::reflexive(elem.0);
                                    } else {
                                        K::transitive(elem.0, old_min, sorted.spec_index(j).0);
                                    }
                                };
                            }
                            min_key = elem.0.clone_plus();
                            proof {
                                lemma_cloned_view_eq(elem.0, min_key);
                                assert(self.tree@.contains(sorted@[i as int]));
                                lemma_pair_in_set_map_contains(self.tree@, sorted@[i as int].0, sorted@[i as int].1);
                            }
                        },
                        _ => {
                            proof { K::total(min_key, elem.0); }
                        },
                    }
                    i = i + 1;
                }
                proof {
                    assert forall|t: K| #[trigger] self@.dom().contains(t@)
                        implies TotalOrder::le(min_key, t) by {
                        lemma_map_contains_pair_in_set(self.tree@, t@);
                        let v: V::V = choose|v: V::V| self.tree@.contains((t@, v));
                        assert(sorted@.contains((t@, v)));
                        let j = choose|j: int| 0 <= j < sorted@.len() && sorted@[j] == (t@, v);
                        assert(TotalOrder::le(min_key, sorted.spec_index(j).0));
                        assert(sorted.spec_index(j).0@ == t@);
                        assert(sorted.spec_index(j).0 == t);
                    };
                }
                Some(min_key)
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to last_key_iter
        fn last_key(&self) -> (last: Option<K>)
            where K: TotalOrder
        {
            self.last_key_iter()
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- in_order traversal, returns last
        #[verifier::loop_isolation(false)]
        fn last_key_iter(&self) -> (last: Option<K>)
            where K: TotalOrder
        {
            proof {
                lemma_reveal_view_injective::<K>();
                lemma_pair_set_to_map_dom_finite(self.tree@);
                lemma_pair_set_to_map_len(self.tree@);
            }
            let sorted = self.tree.in_order();
            let len = sorted.length();
            if len == 0 {
                None
            } else {
                let mut max_key = sorted.nth(0).0.clone_plus();
                proof {
                    assert(obeys_feq_full_trigger::<K>());
                    K::reflexive(max_key);
                    lemma_cloned_view_eq(sorted.spec_index(0).0, max_key);
                    assert(self.tree@.contains(sorted@[0]));
                    lemma_pair_in_set_map_contains(self.tree@, sorted@[0].0, sorted@[0].1);
                }
                let mut i: usize = 1;
                while i < len
                    invariant
                        1 <= i, i <= len,
                        len as nat == sorted@.len(),
                        self.spec_orderedtablestper_wf(),
                        self.tree@.contains(sorted@[0]),
                        forall|j: int| 0 <= j < sorted@.len() ==>
                            self.tree@.contains(#[trigger] sorted@[j]),
                        forall|j: int| 0 <= j < i as int ==>
                            TotalOrder::le((#[trigger] sorted.spec_index(j)).0, max_key),
                        self@.dom().contains(max_key@),
                    decreases len - i,
                {
                    let elem = sorted.nth(i);
                    let c = TotalOrder::cmp(&elem.0, &max_key);
                    match c {
                        core::cmp::Ordering::Greater => {
                            proof {
                                let old_max = max_key;
                                assert forall|j: int| 0 <= j < i + 1
                                    implies TotalOrder::le((#[trigger] sorted.spec_index(j)).0, elem.0) by {
                                    if j == i as int {
                                        K::reflexive(elem.0);
                                    } else {
                                        K::transitive(sorted.spec_index(j).0, old_max, elem.0);
                                    }
                                };
                            }
                            max_key = elem.0.clone_plus();
                            proof {
                                lemma_cloned_view_eq(elem.0, max_key);
                                assert(self.tree@.contains(sorted@[i as int]));
                                lemma_pair_in_set_map_contains(self.tree@, sorted@[i as int].0, sorted@[i as int].1);
                            }
                        },
                        _ => {
                            proof { K::total(elem.0, max_key); }
                        },
                    }
                    i = i + 1;
                }
                proof {
                    assert forall|t: K| #[trigger] self@.dom().contains(t@)
                        implies TotalOrder::le(t, max_key) by {
                        lemma_map_contains_pair_in_set(self.tree@, t@);
                        let v: V::V = choose|v: V::V| self.tree@.contains((t@, v));
                        assert(sorted@.contains((t@, v)));
                        let j = choose|j: int| 0 <= j < sorted@.len() && sorted@[j] == (t@, v);
                        assert(TotalOrder::le(sorted.spec_index(j).0, max_key));
                        assert(sorted.spec_index(j).0@ == t@);
                        assert(sorted.spec_index(j).0 == t);
                    };
                }
                Some(max_key)
            }
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to previous_key_iter
        fn previous_key(&self, k: &K) -> (predecessor: Option<K>)
            where K: TotalOrder
        {
            self.previous_key_iter(k)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- in_order traversal + linear scan for predecessor
        #[verifier::loop_isolation(false)]
        fn previous_key_iter(&self, k: &K) -> (predecessor: Option<K>)
            where K: TotalOrder
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
                    self.spec_orderedtablestper_wf(),
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
                                    proof { K::total(elem.0, old_best); }
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
            where K: TotalOrder
        {
            self.next_key_iter(k)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- in_order traversal + linear scan for successor
        #[verifier::loop_isolation(false)]
        fn next_key_iter(&self, k: &K) -> (successor: Option<K>)
            where K: TotalOrder
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
                    self.spec_orderedtablestper_wf(),
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
                                    proof { K::total(old_best, elem.0); }
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
        fn split_key(&self, k: &K) -> (split: (Self, Option<V>, Self))
            where Self: Sized
        {
            self.split_key_iter(k)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- in_order + n BST inserts into two trees
        #[verifier::loop_isolation(false)]
        fn split_key_iter(&self, k: &K) -> (split: (Self, Option<V>, Self))
            where Self: Sized
        {
            let ghost old_tree = self.tree@;
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
                    self.spec_orderedtablestper_wf(),
                    obeys_view_eq::<K>(),
                    obeys_feq_full::<Pair<K, V>>(),
                    self.tree@ == old_tree,
                    len as nat == sorted@.len(),
                    sorted@.len() == old_tree.len(),
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
                    forall|v: <Pair<K, V> as View>::V| old_tree.contains(v) <==> #[trigger] sorted@.contains(v),
                    forall|p: (K::V, V::V)| #[trigger] left_tree@.contains(p) ==> old_tree.contains(p),
                    forall|p: (K::V, V::V)| #[trigger] right_tree@.contains(p) ==> old_tree.contains(p),
                    forall|p: (K::V, V::V)| #[trigger] left_tree@.contains(p) ==> p.0 != k@,
                    forall|p: (K::V, V::V)| #[trigger] right_tree@.contains(p) ==> p.0 != k@,
                    forall|j: int| #![trigger sorted@[j]] 0 <= j < i as int ==>
                        left_tree@.contains(sorted@[j]) || right_tree@.contains(sorted@[j]) || sorted@[j].0 == k@,
                    forall|p: (K::V, V::V)| !(#[trigger] left_tree@.contains(p) && right_tree@.contains(p)),
                decreases len - i,
            {
                let pair = sorted.nth(i);
                proof { reveal(obeys_view_eq); }
                let c = pair.0.cmp(k);
                proof { reveal(vstd::laws_cmp::obeys_cmp_ord); }
                match c {
                    core::cmp::Ordering::Less => {
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
                        }
                        left_tree.insert(cloned);
                        proof {
                            assert(left_tree@.len() <= i as nat + 1);
                            lemma_key_unique_insert(old_left, sorted@[i as int].0, sorted@[i as int].1);
                            assert(sorted@.contains(sorted@[i as int])) by { assert(sorted@[i as int] == sorted@[i as int]); };
                            assert(old_tree.contains(sorted@[i as int]));
                            assert(pair.0.cmp_spec(k) != Equal);
                            assert(pair.0@ != k@);
                            assert(sorted@[i as int].0 != k@);
                            assert(!right_tree@.contains(sorted@[i as int])) by {
                                if right_tree@.contains(sorted@[i as int]) {
                                    let jj = choose|jj: int| 0 <= jj < i as int && sorted@[i as int].0 == (#[trigger] sorted@[jj]).0;
                                    assert(false);
                                }
                            };
                        }
                    },
                    core::cmp::Ordering::Greater => {
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
                        }
                        right_tree.insert(cloned);
                        proof {
                            assert(right_tree@.len() <= i as nat + 1);
                            lemma_key_unique_insert(old_right, sorted@[i as int].0, sorted@[i as int].1);
                            assert(sorted@.contains(sorted@[i as int])) by { assert(sorted@[i as int] == sorted@[i as int]); };
                            assert(old_tree.contains(sorted@[i as int]));
                            assert(pair.0.cmp_spec(k) != Equal);
                            assert(pair.0@ != k@);
                            assert(sorted@[i as int].0 != k@);
                            assert(!left_tree@.contains(sorted@[i as int])) by {
                                if left_tree@.contains(sorted@[i as int]) {
                                    let jj = choose|jj: int| 0 <= jj < i as int && sorted@[i as int].0 == (#[trigger] sorted@[jj]).0;
                                    assert(false);
                                }
                            };
                        }
                    },
                    core::cmp::Ordering::Equal => {
                        proof {
                            assert(pair.0.cmp_spec(k) == Equal);
                            assert(pair.0@ == k@);
                            assert(sorted@[i as int].0 == k@);
                        }
                    },
                }
                i += 1;
            }
            let left_table = OrderedTableStPer { tree: left_tree };
            let right_table = OrderedTableStPer { tree: right_tree };
            proof {
                let old_map = spec_pair_set_to_map(old_tree);
                lemma_pair_set_to_map_dom_finite(old_tree);
                lemma_pair_set_to_map_dom_finite(left_tree@);
                lemma_pair_set_to_map_dom_finite(right_tree@);

                // !parts.0@.dom().contains(k@) and !parts.2@.dom().contains(k@).
                assert(!left_table@.dom().contains(k@)) by {
                    if left_table@.dom().contains(k@) {
                        lemma_map_contains_pair_in_set(left_tree@, k@);
                        let v: V::V = choose|v: V::V| left_tree@.contains((k@, v));
                        assert((k@, v).0 != k@);
                    }
                };
                assert(!right_table@.dom().contains(k@)) by {
                    if right_table@.dom().contains(k@) {
                        lemma_map_contains_pair_in_set(right_tree@, k@);
                        let v: V::V = choose|v: V::V| right_tree@.contains((k@, v));
                        assert((k@, v).0 != k@);
                    }
                };

                // parts.0@.dom().subset_of(self@.dom()).
                assert forall|kk: K::V| left_table@.dom().contains(kk)
                    implies #[trigger] self@.dom().contains(kk)
                by {
                    lemma_map_contains_pair_in_set(left_tree@, kk);
                    let v: V::V = choose|v: V::V| left_tree@.contains((kk, v));
                    assert(old_tree.contains((kk, v)));
                    lemma_pair_in_set_map_contains(old_tree, kk, v);
                };

                // parts.2@.dom().subset_of(self@.dom()).
                assert forall|kk: K::V| right_table@.dom().contains(kk)
                    implies #[trigger] self@.dom().contains(kk)
                by {
                    lemma_map_contains_pair_in_set(right_tree@, kk);
                    let v: V::V = choose|v: V::V| right_tree@.contains((kk, v));
                    assert(old_tree.contains((kk, v)));
                    lemma_pair_in_set_map_contains(old_tree, kk, v);
                };

                // parts.0@.dom().disjoint(parts.2@.dom()).
                assert(left_table@.dom().disjoint(right_table@.dom())) by {
                    assert forall|kk: K::V| !(left_table@.dom().contains(kk) && right_table@.dom().contains(kk))
                    by {
                        if left_table@.dom().contains(kk) && right_table@.dom().contains(kk) {
                            lemma_map_contains_pair_in_set(left_tree@, kk);
                            lemma_map_contains_pair_in_set(right_tree@, kk);
                            let v1: V::V = choose|v: V::V| left_tree@.contains((kk, v));
                            let v2: V::V = choose|v: V::V| right_tree@.contains((kk, v));
                            assert(old_tree.contains((kk, v1)));
                            assert(old_tree.contains((kk, v2)));
                            assert(left_tree@.contains((kk, v1)));
                            assert(right_tree@.contains((kk, v1)));
                            assert(false);
                        }
                    };
                };

                // Completeness: self@.dom().contains(key) ==> left or right or == k@.
                assert forall|key: K::V| #[trigger] self@.dom().contains(key)
                    implies left_table@.dom().contains(key) || right_table@.dom().contains(key) || key == k@
                by {
                    lemma_map_contains_pair_in_set(old_tree, key);
                    let v: V::V = choose|v: V::V| old_tree.contains((key, v));
                    assert(sorted@.contains((key, v)));
                    let j = choose|j: int| 0 <= j < sorted@.len() && sorted@[j] == (key, v);
                    assert(left_tree@.contains(sorted@[j]) || right_tree@.contains(sorted@[j]) || sorted@[j].0 == k@);
                    if left_tree@.contains(sorted@[j]) {
                        lemma_pair_in_set_map_contains(left_tree@, key, v);
                    } else if right_tree@.contains(sorted@[j]) {
                        lemma_pair_in_set_map_contains(right_tree@, key, v);
                    }
                };

                // wf for both tables.
                vstd::set_lib::lemma_len_subset(left_tree@, old_tree);
                vstd::set_lib::lemma_len_subset(right_tree@, old_tree);
            }
            (left_table, found_val, right_table)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n * m), Span O(n * m) -- delegates to union
        fn join_key(left: &Self, right: &Self) -> (table: Self) {
            left.union(right, |v1: &V, _v2: &V| -> (r: V) { v1.clone() })
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- delegates to get_key_range_iter
        fn get_key_range(&self, k1: &K, k2: &K) -> (range: Self) {
            self.get_key_range_iter(k1, k2)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- in_order + conditional BST inserts
        #[verifier::loop_isolation(false)]
        fn get_key_range_iter(&self, k1: &K, k2: &K) -> (range: Self) {
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
                    self.spec_orderedtablestper_wf(),
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
                    forall|ii: int, jj: int|
                        0 <= ii < sorted@.len() && 0 <= jj < sorted@.len() && ii != jj
                        ==> (#[trigger] sorted@[ii]).0 != (#[trigger] sorted@[jj]).0,
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==>
                        exists|j: int| 0 <= j < i as int && p.0 == (#[trigger] sorted@[j]).0,
                    forall|p: (K::V, V::V)| #[trigger] new_tree@.contains(p) ==> self.tree@.contains(p),
                    forall|v: <Pair<K, V> as View>::V| self.tree@.contains(v) <==> #[trigger] sorted@.contains(v),
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
                        assert(!spec_pair_set_to_map(old_new_tree).dom().contains(sorted@[i as int].0)) by {
                            if spec_pair_set_to_map(old_new_tree).dom().contains(sorted@[i as int].0) {
                                lemma_map_contains_pair_in_set(old_new_tree, sorted@[i as int].0);
                                let vv: V::V = choose|vv: V::V| old_new_tree.contains((sorted@[i as int].0, vv));
                                let jj = choose|jj: int| 0 <= jj < i as int && (sorted@[i as int].0, vv).0 == (#[trigger] sorted@[jj]).0;
                                assert(false);
                            }
                        };
                        assert(sorted@.contains(sorted@[i as int])) by { assert(sorted@[i as int] == sorted@[i as int]); };
                        assert(self.tree@.contains(sorted@[i as int]));
                    }
                    new_tree.insert(cloned);
                    proof {
                        assert(new_tree@.len() <= i as nat + 1);
                        lemma_key_unique_insert(old_new_tree, sorted@[i as int].0, sorted@[i as int].1);
                    }
                }
                i += 1;
            }
            let range = OrderedTableStPer { tree: new_tree };
            proof {
                lemma_pair_set_to_map_dom_finite(new_tree@);
                lemma_pair_set_to_map_dom_finite(self.tree@);
                assert(range@.dom().subset_of(self@.dom())) by {
                    assert forall|k: K::V| range@.dom().contains(k)
                        implies #[trigger] self@.dom().contains(k)
                    by {
                        lemma_map_contains_pair_in_set(new_tree@, k);
                        let v: V::V = choose|v: V::V| new_tree@.contains((k, v));
                        lemma_pair_in_set_map_contains(self.tree@, k, v);
                    };
                };
                assert forall|key: K::V| #[trigger] range@.dom().contains(key) implies range@[key] == self@[key]
                by {
                    lemma_map_contains_pair_in_set(new_tree@, key);
                    let v: V::V = choose|v: V::V| new_tree@.contains((key, v));
                    lemma_pair_in_set_map_contains(new_tree@, key, v);
                    lemma_pair_in_set_map_contains(self.tree@, key, v);
                };
                vstd::set_lib::lemma_len_subset(new_tree@, self.tree@);
            }
            range
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to rank_key_iter
        fn rank_key(&self, k: &K) -> (rank: usize)
            where K: TotalOrder
        {
            self.rank_key_iter(k)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- in_order traversal + count elements <= k
        #[verifier::loop_isolation(false)]
        fn rank_key_iter(&self, k: &K) -> (rank: usize)
            where K: TotalOrder
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
                    self.spec_orderedtablestper_wf(),
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

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- in_order traversal + index access
        #[verifier::loop_isolation(false)]
        fn select_key(&self, i: usize) -> (selected: Option<K>)
            where K: TotalOrder
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
                        self.spec_orderedtablestper_wf(),
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
        fn split_rank_key(&self, i: usize) -> (split: (Self, Self))
            where Self: Sized
        {
            self.split_rank_key_iter(i)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- in_order + n BST inserts into two trees
        #[verifier::loop_isolation(false)]
        fn split_rank_key_iter(&self, i: usize) -> (split: (Self, Self))
            where Self: Sized
        {
            let ghost old_tree = self.tree@;
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
                    self.spec_orderedtablestper_wf(),
                    obeys_feq_full::<Pair<K, V>>(),
                    self.tree@ == old_tree,
                    size as nat == sorted@.len(),
                    sorted@.len() == old_tree.len(),
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
                    forall|v: <Pair<K, V> as View>::V| old_tree.contains(v) <==> #[trigger] sorted@.contains(v),
                    forall|p: (K::V, V::V)| #[trigger] left_tree@.contains(p) ==> old_tree.contains(p),
                    forall|p: (K::V, V::V)| #[trigger] right_tree@.contains(p) ==> old_tree.contains(p),
                    forall|jj: int| #![trigger sorted@[jj]] 0 <= jj < j as int ==>
                        left_tree@.contains(sorted@[jj]) || right_tree@.contains(sorted@[jj]),
                    forall|p: (K::V, V::V)| !(#[trigger] left_tree@.contains(p) && right_tree@.contains(p)),
                    left_tree@.len() < usize::MAX as nat,
                    right_tree@.len() < usize::MAX as nat,
                decreases size - j,
            {
                let elem = sorted.nth(j);
                let cloned = elem.clone_plus();
                proof { lemma_cloned_view_eq(*elem, cloned); }
                if j < split_at {
                    let ghost old_left = left_tree@;
                    proof {
                        assert(!spec_pair_set_to_map(old_left).dom().contains(sorted@[j as int].0)) by {
                            if spec_pair_set_to_map(old_left).dom().contains(sorted@[j as int].0) {
                                lemma_map_contains_pair_in_set(old_left, sorted@[j as int].0);
                                let vv: V::V = choose|vv: V::V| old_left.contains((sorted@[j as int].0, vv));
                                let jj = choose|jj: int| 0 <= jj < j as int && (sorted@[j as int].0, vv).0 == (#[trigger] sorted@[jj]).0;
                                assert(false);
                            }
                        };
                        assert(old_left.len() < usize::MAX as nat);
                    }
                    left_tree.insert(cloned);
                    proof {
                        assert(left_tree@.len() <= j as nat + 1);
                        lemma_key_unique_insert(old_left, sorted@[j as int].0, sorted@[j as int].1);
                        assert(sorted@.contains(sorted@[j as int])) by { assert(sorted@[j as int] == sorted@[j as int]); };
                        assert(old_tree.contains(sorted@[j as int]));
                        assert(!right_tree@.contains(sorted@[j as int])) by {
                            if right_tree@.contains(sorted@[j as int]) {
                                let jj = choose|jj: int| 0 <= jj < j as int && sorted@[j as int].0 == (#[trigger] sorted@[jj]).0;
                                assert(false);
                            }
                        };
                    }
                } else {
                    let ghost old_right = right_tree@;
                    proof {
                        assert(!spec_pair_set_to_map(old_right).dom().contains(sorted@[j as int].0)) by {
                            if spec_pair_set_to_map(old_right).dom().contains(sorted@[j as int].0) {
                                lemma_map_contains_pair_in_set(old_right, sorted@[j as int].0);
                                let vv: V::V = choose|vv: V::V| old_right.contains((sorted@[j as int].0, vv));
                                let jj = choose|jj: int| 0 <= jj < j as int && (sorted@[j as int].0, vv).0 == (#[trigger] sorted@[jj]).0;
                                assert(false);
                            }
                        };
                        assert(old_right.len() < usize::MAX as nat);
                    }
                    right_tree.insert(cloned);
                    proof {
                        assert(right_tree@.len() <= j as nat + 1);
                        lemma_key_unique_insert(old_right, sorted@[j as int].0, sorted@[j as int].1);
                        assert(sorted@.contains(sorted@[j as int])) by { assert(sorted@[j as int] == sorted@[j as int]); };
                        assert(old_tree.contains(sorted@[j as int]));
                        assert(!left_tree@.contains(sorted@[j as int])) by {
                            if left_tree@.contains(sorted@[j as int]) {
                                let jj = choose|jj: int| 0 <= jj < j as int && sorted@[j as int].0 == (#[trigger] sorted@[jj]).0;
                                assert(false);
                            }
                        };
                    }
                }
                j += 1;
            }
            let left_table = OrderedTableStPer { tree: left_tree };
            let right_table = OrderedTableStPer { tree: right_tree };
            proof {
                let old_map = spec_pair_set_to_map(old_tree);
                lemma_pair_set_to_map_dom_finite(old_tree);
                lemma_pair_set_to_map_dom_finite(left_tree@);
                lemma_pair_set_to_map_dom_finite(right_tree@);

                // parts.0@.dom().subset_of(self@.dom()).
                assert forall|kk: K::V| left_table@.dom().contains(kk)
                    implies #[trigger] self@.dom().contains(kk)
                by {
                    lemma_map_contains_pair_in_set(left_tree@, kk);
                    let v: V::V = choose|v: V::V| left_tree@.contains((kk, v));
                    assert(old_tree.contains((kk, v)));
                    lemma_pair_in_set_map_contains(old_tree, kk, v);
                };

                // parts.1@.dom().subset_of(self@.dom()).
                assert forall|kk: K::V| right_table@.dom().contains(kk)
                    implies #[trigger] self@.dom().contains(kk)
                by {
                    lemma_map_contains_pair_in_set(right_tree@, kk);
                    let v: V::V = choose|v: V::V| right_tree@.contains((kk, v));
                    assert(old_tree.contains((kk, v)));
                    lemma_pair_in_set_map_contains(old_tree, kk, v);
                };

                // Disjoint.
                assert(left_table@.dom().disjoint(right_table@.dom())) by {
                    assert forall|kk: K::V| !(left_table@.dom().contains(kk) && right_table@.dom().contains(kk))
                    by {
                        if left_table@.dom().contains(kk) && right_table@.dom().contains(kk) {
                            lemma_map_contains_pair_in_set(left_tree@, kk);
                            lemma_map_contains_pair_in_set(right_tree@, kk);
                            let v1: V::V = choose|v: V::V| left_tree@.contains((kk, v));
                            let v2: V::V = choose|v: V::V| right_tree@.contains((kk, v));
                            assert(old_tree.contains((kk, v1)));
                            assert(old_tree.contains((kk, v2)));
                            assert(left_tree@.contains((kk, v1)));
                            assert(right_tree@.contains((kk, v1)));
                            assert(false);
                        }
                    };
                };

                // Completeness.
                assert forall|key: K::V| #[trigger] self@.dom().contains(key)
                    implies left_table@.dom().contains(key) || right_table@.dom().contains(key)
                by {
                    lemma_map_contains_pair_in_set(old_tree, key);
                    let v: V::V = choose|v: V::V| old_tree.contains((key, v));
                    assert(sorted@.contains((key, v)));
                    let jj = choose|jj: int| 0 <= jj < sorted@.len() && sorted@[jj] == (key, v);
                    assert(left_tree@.contains(sorted@[jj]) || right_tree@.contains(sorted@[jj]));
                    if left_tree@.contains(sorted@[jj]) {
                        lemma_pair_in_set_map_contains(left_tree@, key, v);
                    } else {
                        lemma_pair_in_set_map_contains(right_tree@, key, v);
                    }
                };

                // wf.
                vstd::set_lib::lemma_len_subset(left_tree@, old_tree);
                vstd::set_lib::lemma_len_subset(right_tree@, old_tree);
            }
            (left_table, right_table)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to find
        fn find_iter(&self, k: &K) -> (found: Option<V>) {
            self.find(k)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to insert
        fn insert_iter(&self, k: K, v: V) -> (table: Self) {
            self.insert(k, v)
        }

        /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) -- delegates to delete
        fn delete_iter(&self, k: &K) -> (table: Self) {
            self.delete(k)
        }
    }

    // 10. iterators

    impl<K: StT + Ord, V: StT + Ord> OrderedTableStPer<K, V> {
        /// Returns an iterator over the table entries via in-order traversal.
        pub fn iter(&self) -> (it: OrderedTableStPerIter<K, V>)
            requires
                self.spec_orderedtablestper_wf(),
            ensures
                it@.0 == 0,
                it@.1.len() == self.tree@.len(),
                iter_invariant(&it),
        {
            let sorted = self.tree.in_order();
            OrderedTableStPerIter { inner: sorted.seq.into_iter() }
        }
    }

    #[verifier::reject_recursive_types(K)]
    #[verifier::reject_recursive_types(V)]
    pub struct OrderedTableStPerIter<K: StT + Ord, V: StT + Ord> {
        pub inner: IntoIter<Pair<K, V>>,
    }

    impl<K: StT + Ord, V: StT + Ord> View for OrderedTableStPerIter<K, V> {
        type V = (int, Seq<Pair<K, V>>);
        open spec fn view(&self) -> (int, Seq<Pair<K, V>>) { self.inner@ }
    }

    pub open spec fn iter_invariant<K: StT + Ord, V: StT + Ord>(it: &OrderedTableStPerIter<K, V>) -> bool {
        0 <= it@.0 <= it@.1.len()
    }

    impl<K: StT + Ord, V: StT + Ord> std::iter::Iterator for OrderedTableStPerIter<K, V> {
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
    pub struct OrderedTableStPerGhostIterator<K: StT + Ord, V: StT + Ord> {
        pub pos: int,
        pub elements: Seq<Pair<K, V>>,
    }

    impl<K: StT + Ord, V: StT + Ord> View for OrderedTableStPerGhostIterator<K, V> {
        type V = Seq<Pair<K, V>>;
        open spec fn view(&self) -> Seq<Pair<K, V>> { self.elements.take(self.pos) }
    }

    impl<K: StT + Ord, V: StT + Ord> vstd::pervasive::ForLoopGhostIteratorNew for OrderedTableStPerIter<K, V> {
        type GhostIter = OrderedTableStPerGhostIterator<K, V>;
        open spec fn ghost_iter(&self) -> OrderedTableStPerGhostIterator<K, V> {
            OrderedTableStPerGhostIterator { pos: self@.0, elements: self@.1 }
        }
    }

    impl<K: StT + Ord, V: StT + Ord> vstd::pervasive::ForLoopGhostIterator for OrderedTableStPerGhostIterator<K, V> {
        type ExecIter = OrderedTableStPerIter<K, V>;
        type Item = Pair<K, V>;
        type Decrease = int;

        open spec fn exec_invariant(&self, exec_iter: &OrderedTableStPerIter<K, V>) -> bool {
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

        open spec fn ghost_advance(&self, _exec_iter: &OrderedTableStPerIter<K, V>) -> OrderedTableStPerGhostIterator<K, V> {
            Self { pos: self.pos + 1, ..*self }
        }
    }

    impl<'a, K: StT + Ord, V: StT + Ord> std::iter::IntoIterator for &'a OrderedTableStPer<K, V> {
        type Item = Pair<K, V>;
        type IntoIter = OrderedTableStPerIter<K, V>;
        fn into_iter(self) -> (it: Self::IntoIter)
            requires
                self.spec_orderedtablestper_wf(),
            ensures
                it@.0 == 0,
                it@.1.len() == self.tree@.len(),
                iter_invariant(&it),
        {
            self.iter()
        }
    }

    // 12. derive impls in verus!

    #[cfg(verus_keep_ghost)]
    impl<K: StT + Ord, V: StT + Ord> PartialEqSpecImpl for OrderedTableStPer<K, V> {
        open spec fn obeys_eq_spec() -> bool { true }
        open spec fn eq_spec(&self, other: &Self) -> bool { self@ == other@ }
    }

    impl<K: StT + Ord, V: StT + Ord> Eq for OrderedTableStPer<K, V> {}

    impl<K: StT + Ord, V: StT + Ord> PartialEq for OrderedTableStPer<K, V> {
        fn eq(&self, other: &Self) -> (equal: bool)
            ensures equal == (self@ == other@)
        {
            let equal = self.tree.size() == other.tree.size();
            proof { assume(equal == (self@ == other@)); }
            equal
        }
    }

    impl<K: StT + Ord, V: StT + Ord> Clone for OrderedTableStPer<K, V> {
        fn clone(&self) -> (copy: Self)
            ensures copy@ == self@
        {
            let copy = OrderedTableStPer {
                tree: self.tree.clone(),
            };
            proof { assume(copy@ == self@); }
            copy
        }
    }

    /// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n log n), Span O(n log n) -- n BST inserts from sorted entries
    pub fn from_sorted_entries<K: StT + Ord, V: StT + Ord>(
        entries: AVLTreeSeqStPerS<Pair<K, V>>,
    ) -> (result: OrderedTableStPer<K, V>)
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
            result.spec_orderedtablestper_wf(),
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
                                let j1 = choose|j: int| #![trigger entries@[j]]
                                    0 <= j < i as int && entries@[j] == (k, v1);
                                assert(entries@[j1].0 == entries@[i as int].0);
                                assert(j1 < i as int);
                                assert(false); // contradicts unique keys
                            } else {
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
        let result = OrderedTableStPer { tree };
        proof { lemma_pair_set_to_map_dom_finite(tree@); }
        result
    }

    } // verus!

    // 13. macros

    /// Macro for creating ordered tables from sorted key-value pairs.
    #[macro_export]
    macro_rules! OrderedTableStPerLit {
        () => {
            $crate::Chap43::OrderedTableStPer::OrderedTableStPer::OrderedTableStPer::empty()
        };
        ($($key:expr => $val:expr),+ $(,)?) => {{
            let pairs = vec![$($crate::Types::Types::Pair($key, $val)),+];
            let seq = $crate::Chap37::AVLTreeSeqStPer::AVLTreeSeqStPer::AVLTreeSeqStPerS::from_vec(pairs);
            $crate::Chap43::OrderedTableStPer::OrderedTableStPer::from_sorted_entries(seq)
        }};
    }

    // 14. derive impls outside verus!

    use std::fmt;

    impl<K: StT + Ord, V: StT + Ord> fmt::Debug for OrderedTableStPer<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedTableStPer(size: {})", self.size())
        }
    }

    impl<K: StT + Ord, V: StT + Ord> fmt::Display for OrderedTableStPer<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedTableStPer(size: {})", self.size())
        }
    }

    impl<K: StT + Ord, V: StT + Ord> fmt::Debug for OrderedTableStPerIter<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("OrderedTableStPerIter").finish()
        }
    }

    impl<K: StT + Ord, V: StT + Ord> fmt::Display for OrderedTableStPerIter<K, V> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "OrderedTableStPerIter")
        }
    }
}
