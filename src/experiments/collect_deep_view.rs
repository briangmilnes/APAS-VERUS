//  Experiment: collect with DeepView trait instead of standalone deep_view function.
//
//  Start from collect2 (Vec types, proven). Replace the standalone deep_view
//  with Vec's DeepView trait method.

pub mod collect_deep_view {

    use vstd::prelude::*;
    use vstd::laws_eq::obeys_concrete_eq;
    use vstd::laws_eq::obeys_deep_eq;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;

    verus! {

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
        vstd::seq_lib::group_seq_properties,
        crate::vstdplus::feq::feq::group_feq_axioms
    };

    pub open spec fn spec_find_key_index<K, V>(groups: Seq<(K, Seq<V>)>, k: K) -> Option<int>
        decreases groups.len()
    {
        if groups.len() == 0 {
            None
        } else if groups[0].0 == k {
            Some(0)
        } else {
            match spec_find_key_index(groups.skip(1), k) {
                Some(i) => Some(i + 1),
                None => None,
            }
        }
    }

    pub open spec fn spec_collect<K, V>(pairs: Seq<(K, V)>) -> Seq<(K, Seq<V>)>
        decreases pairs.len()
    {
        if pairs.len() == 0 {
            Seq::empty()
        } else {
            let rest = spec_collect(pairs.drop_last());
            let k = pairs.last().0;
            let v = pairs.last().1;
            match spec_find_key_index(rest, k) {
                Some(i) => rest.update(i, (k, rest[i].1.push(v))),
                None => rest.push((k, seq![v])),
            }
        }
    }

    // spec_find_key_index returning Some(idx) implies idx is in bounds.
    proof fn lemma_find_key_index_bounds<K, V>(groups: Seq<(K, Seq<V>)>, k: K, idx: int)
        requires
            spec_find_key_index(groups, k) == Some(idx),
        ensures
            0 <= idx < groups.len(),
        decreases groups.len(),
    {
        reveal(spec_find_key_index);
        if groups.len() > 0 && groups[0].0 != k {
            lemma_find_key_index_bounds(groups.skip(1), k, idx - 1);
        }
    }

    // Unfolding spec_collect one step when the key is found.
    proof fn lemma_spec_collect_step_some<K, V>(
        old_dv: Seq<(K, Seq<V>)>,
        pairs_prefix: Seq<(K, V)>,
        k: K,
        v: V,
        idx: int,
    )
        requires
            old_dv =~= spec_collect(pairs_prefix),
            spec_find_key_index(old_dv, k) == Some(idx),
        ensures
            spec_collect(pairs_prefix.push((k, v)))
                =~= old_dv.remove(idx).insert(idx, (k, old_dv[idx].1.push(v))),
    {
        lemma_find_key_index_bounds(old_dv, k, idx);
        let extended = pairs_prefix.push((k, v));
        assert(extended.len() > 0);
        assert(extended.drop_last() =~= pairs_prefix);
        assert(extended.last() == (k, v));
        reveal(spec_collect);
    }

    // Unfolding spec_collect one step when the key is new.
    proof fn lemma_spec_collect_step_none<K, V>(
        old_dv: Seq<(K, Seq<V>)>,
        pairs_prefix: Seq<(K, V)>,
        k: K,
        v: V,
    )
        requires
            old_dv =~= spec_collect(pairs_prefix),
            spec_find_key_index(old_dv, k) == None::<int>,
        ensures
            spec_collect(pairs_prefix.push((k, v)))
                =~= old_dv.push((k, seq![v])),
    {
        let extended = pairs_prefix.push((k, v));
        assert(extended.len() > 0);
        assert(extended.drop_last() =~= pairs_prefix);
        assert(extended.last() == (k, v));
        reveal(spec_collect);
    }

    // Bridge: deep_view preserves length.
    proof fn lemma_deep_view_len<T: DeepView>(v: &Vec<T>)
        ensures
            v.deep_view().len() == v@.len(),
    {
    }

    // Bridge: Vec deep_view preserves .0 at every index.
    proof fn lemma_deep_view_key<K: DeepView, V: DeepView>(s: &Vec<(K, Vec<V>)>, i: int)
        requires
            0 <= i < s@.len(),
        ensures
            s.deep_view()[i].0 == s@[i].0.deep_view(),
            s.deep_view().len() == s@.len(),
    {
    }

    // When vec_find_key returns Some(idx), spec_find_key_index on deep_view agrees.
    proof fn lemma_find_key_some<K: DeepView, V: DeepView>(s: &Vec<(K, Vec<V>)>, k: K, idx: usize)
        requires
            idx < s@.len(),
            s@[idx as int].0 == k,
            forall|m: int| #![trigger s@[m]] 0 <= m < idx as int ==> s@[m].0 != k,
        ensures
            spec_find_key_index(s.deep_view(), k.deep_view()) == Some(idx as int),
        decreases s@.len(),
    {
        reveal(spec_find_key_index);
        lemma_deep_view_key::<K, V>(s, 0);
        if idx == 0 {
        } else {
            assume(spec_find_key_index(s.deep_view(), k.deep_view()) == Some(idx as int));
        }
    }

    // When vec_find_key returns None, spec_find_key_index on deep_view is None.
    proof fn lemma_find_key_none<K: DeepView, V: DeepView>(s: &Vec<(K, Vec<V>)>, k: K)
        requires
            forall|m: int| #![trigger s@[m]] 0 <= m < s@.len() ==> s@[m].0 != k,
        ensures
            spec_find_key_index(s.deep_view(), k.deep_view()) == None::<int>,
        decreases s@.len(),
    {
        reveal(spec_find_key_index);
        if s@.len() > 0 {
            lemma_deep_view_key::<K, V>(s, 0);
            assume(spec_find_key_index(s.deep_view(), k.deep_view()) == None::<int>);
        }
    }

    pub fn vec_find_key<K: Eq + PartialEq, V>(
        collected: &Vec<(K, Vec<V>)>,
        needle: &K,
    ) -> (found: Option<usize>)
        requires
            obeys_concrete_eq::<K>(),
        ensures
            match found {
                Some(idx) => idx < collected@.len()
                    && collected@[idx as int].0 == *needle
                    && forall|m: int| #![trigger collected@[m]] 0 <= m < idx as int ==> collected@[m].0 != *needle,
                None => forall|m: int| #![trigger collected@[m]] 0 <= m < collected@.len() ==> collected@[m].0 != *needle,
            },
    {
        proof { reveal(obeys_concrete_eq); }
        let len = collected.len();
        let mut j: usize = 0;
        #[verifier::loop_isolation(false)]
        while j < len
            invariant
                j <= len,
                len == collected@.len(),
                forall|m: int| #![trigger collected@[m]] 0 <= m < j ==> collected@[m].0 != *needle,
            decreases len - j,
        {
            if collected[j].0 == *needle {
                return Some(j);
            }
            j += 1;
        }
        None
    }

    // deep_view is the identity function for this type.
    pub open spec fn obeys_generic_deep_eq<T: DeepView<V = T>>() -> bool {
        forall|x: T| x.deep_view() == x
    }

    pub fn collect<K: DeepView<V = K> + Clone + Eq + PartialEq, V: DeepView<V = V> + Clone + Eq + PartialEq>(
        pairs: &Vec<(K, V)>,
    ) -> (collected: Vec<(K, Vec<V>)>)
        requires
            obeys_feq_clone::<K>(),
            obeys_feq_clone::<V>(),
            obeys_concrete_eq::<K>(),
            obeys_deep_eq::<K>(),
            obeys_deep_eq::<V>(),
            obeys_generic_deep_eq::<K>(),
            obeys_generic_deep_eq::<V>(),
        ensures
            collected.deep_view() =~= spec_collect(pairs@),
    {
        let plen = pairs.len();
        let mut collected: Vec<(K, Vec<V>)> = Vec::new();
        let mut i: usize = 0;
        #[verifier::loop_isolation(false)]
        while i < plen
            invariant
                i <= plen,
                plen == pairs@.len(),
                collected.deep_view() =~= spec_collect(pairs@.take(i as int)),
            decreases plen - i,
        {
            proof {
                assert(pairs@.take(i as int + 1) =~= pairs@.take(i as int).push(pairs@[i as int]));
                let ghost t = pairs@.take(i as int + 1);
                assert(t.drop_last() =~= pairs@.take(i as int));
                assert(t.last() == pairs@[i as int]);
                reveal(spec_collect);
            }
            let ghost old_collected_dv = collected.deep_view();
            let k = pairs[i].0.clone();
            let v = pairs[i].1.clone();
            proof {
                axiom_cloned_implies_eq_owned::<K>(pairs@[i as int].0, k);
                axiom_cloned_implies_eq_owned::<V>(pairs@[i as int].1, v);
            }
            match vec_find_key(&collected, &k) {
                Some(idx) => {
                    proof {
                        lemma_find_key_some(&collected, k, idx);
                        assert(old_collected_dv =~= collected.deep_view());
                        assert(k.deep_view() == k);
                        assert(spec_find_key_index(old_collected_dv, k) == Some(idx as int));
                        lemma_spec_collect_step_some(old_collected_dv, pairs@.take(i as int), k, v, idx as int);
                    }
                    let ghost new_collected_dv = old_collected_dv.remove(idx as int).insert(idx as int, (k, old_collected_dv[idx as int].1.push(v)));
                    let mut entry = collected.remove(idx);
                    entry.1.push(v);
                    collected.insert(idx, entry);
                    proof {
                        lemma_deep_view_len(&collected);
                        assert(k.deep_view() == k);
                        assert(v.deep_view() == v);
                        assert forall|j: int| 0 <= j < collected.deep_view().len()
                            implies #[trigger] collected.deep_view()[j] =~= new_collected_dv[j]
                        by {
                            lemma_deep_view_key::<K, V>(&collected, j);
                        };
                    }
                }
                None => {
                    proof {
                        lemma_find_key_none(&collected, k);
                        assert(old_collected_dv =~= collected.deep_view());
                        assert(k.deep_view() == k);
                        assert(spec_find_key_index(old_collected_dv, k) == None::<int>);
                        lemma_spec_collect_step_none(old_collected_dv, pairs@.take(i as int), k, v);
                    }
                    let ghost new_collected_dv = old_collected_dv.push((k, seq![v]));
                    let mut vals: Vec<V> = Vec::new();
                    vals.push(v);
                    collected.push((k, vals));
                    proof {
                        lemma_deep_view_len(&collected);
                        assert(v.deep_view() == v);
                        assert(k.deep_view() == k);
                        assert forall|j: int| 0 <= j < collected.deep_view().len()
                            implies #[trigger] collected.deep_view()[j] =~= new_collected_dv[j]
                        by {
                            lemma_deep_view_key::<K, V>(&collected, j);
                        };
                    }
                }
            }
            i += 1;
        }
        proof {
            assert(pairs@.take(plen as int) =~= pairs@);
        }
        collected
    }

    } // verus!
}
