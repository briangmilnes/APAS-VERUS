//  Experiment: collect2 (group-by-key, single loop)
//
//  HYPOTHESIS: we can do this in a single function dual loop and prove it's
//  specification and ordering more easily.
//
//  RESULT: 


pub mod collect2 {

    use vstd::prelude::*;
    use vstd::laws_eq::obeys_concrete_eq;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;

    verus! {

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
        vstd::seq_lib::group_seq_properties,
        crate::vstdplus::feq::feq::group_feq_axioms
        // Veracity: added broadcast groups
        vstd::seq_lib::group_to_multiset_ensures,
    };

    pub open spec fn deep_view<K, V>(s: Seq<(K, Vec<V>)>) -> Seq<(K, Seq<V>)> {
        s.map_values(|e: (K, Vec<V>)| (e.0, e.1@))
    }

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

    pub open spec fn spec_collect2<K, V>(pairs: Seq<(K, V)>) -> Seq<(K, Seq<V>)>
        decreases pairs.len()
    {
        if pairs.len() == 0 {
            Seq::empty()
        } else {
            let rest = spec_collect2(pairs.drop_last());
            let k = pairs.last().0;
            let v = pairs.last().1;
            match spec_find_key_index(rest, k) {
                Some(i) => rest.update(i, (k, rest[i].1.push(v))),
                None => rest.push((k, seq![v])),
            }
        }
    }

    // Bridge: deep_view preserves .0 at every index
    proof fn lemma_deep_view_key<K, V>(s: Seq<(K, Vec<V>)>, i: int)
        requires
            0 <= i < s.len(),
        ensures
            deep_view(s)[i].0 == s[i].0,
            deep_view(s).len() == s.len(),
    {
    }

    // When vec_find_key returns Some(idx), spec_find_key_index on deep_view agrees
    proof fn lemma_find_key_some<K, V>(s: Seq<(K, Vec<V>)>, k: K, idx: usize)
        requires
            idx < s.len(),
            s[idx as int].0 == k,
            forall|m: int| #![trigger s[m]] 0 <= m < idx as int ==> s[m].0 != k,
        ensures
            spec_find_key_index(deep_view(s), k) == Some(idx as int),
        decreases s.len(),
    {
        reveal(spec_find_key_index);
        lemma_deep_view_key::<K, V>(s, 0);
        if idx == 0 {
        } else {
            assert(deep_view(s).skip(1) =~= deep_view(s.skip(1)));
            lemma_find_key_some::<K, V>(s.skip(1), k, (idx - 1) as usize);
        }
    }

    // When vec_find_key returns None, spec_find_key_index on deep_view is None
    proof fn lemma_find_key_none<K, V>(s: Seq<(K, Vec<V>)>, k: K)
        requires
            forall|m: int| #![trigger s[m]] 0 <= m < s.len() ==> s[m].0 != k,
        ensures
            spec_find_key_index(deep_view(s), k) == None::<int>,
        decreases s.len(),
    {
        reveal(spec_find_key_index);
        if s.len() > 0 {
            lemma_deep_view_key::<K, V>(s, 0);
            assert(deep_view(s).skip(1) =~= deep_view(s.skip(1)));
            lemma_find_key_none::<K, V>(s.skip(1), k);
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

    // The Verus limitation of "index for &mut not supported", as of Version: 0.2026.02.05.80fb5a4,
    // prevents collected[idx].1.push(v). So we remove the entry at idx, mutate it, insert it back at the same index.
    pub fn collect2<K: Clone + Eq + PartialEq, V: Clone + Eq + PartialEq>(
        pairs: &Vec<(K, V)>,
    ) -> (collected: Vec<(K, Vec<V>)>)
        requires
            obeys_feq_clone::<K>(),
            obeys_feq_clone::<V>(),
            obeys_concrete_eq::<K>(),
        ensures
            deep_view(collected@) =~= spec_collect2(pairs@),

    {
        let plen = pairs.len();
        let mut collected: Vec<(K, Vec<V>)> = Vec::new();
        let mut i: usize = 0;
        #[verifier::loop_isolation(false)]
        while i < plen
            invariant
                i <= plen,
                plen == pairs@.len(),
                deep_view(collected@) =~= spec_collect2(pairs@.take(i as int)),
            decreases plen - i,
        {
            proof {
                assert(pairs@.take(i as int + 1) =~= pairs@.take(i as int).push(pairs@[i as int]));
                let ghost t = pairs@.take(i as int + 1);
                assert(t.drop_last() =~= pairs@.take(i as int));
                assert(t.last() == pairs@[i as int]);
                reveal(spec_collect2);
            }
            let ghost old_collected = collected@;
            let k = pairs[i].0.clone();
            let v = pairs[i].1.clone();
            proof {
                axiom_cloned_implies_eq_owned::<K>(pairs@[i as int].0, k);
                axiom_cloned_implies_eq_owned::<V>(pairs@[i as int].1, v);
            }
            match vec_find_key(&collected, &k) {
                Some(idx) => {
                    proof {
                        lemma_find_key_some(collected@, k, idx);
                    }
                    let mut entry = collected.remove(idx);
                    entry.1.push(v);
                    collected.insert(idx, entry);
                }
                None => {
                    proof {
                        lemma_find_key_none(collected@, k);
                    }
                    let mut vals: Vec<V> = Vec::new();
                    vals.push(v);
                    collected.push((k, vals));
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
