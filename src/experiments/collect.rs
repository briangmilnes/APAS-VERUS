//  Experiment: collect (group-by-key)
//
//  Two spec families: _rec (recursive), _deduplicating (map_values/remove_duplicates).
//  Two exec versions of each function, one per spec family.

pub mod collect {

    use vstd::prelude::*;
    use vstd::std_specs::cmp::PartialEqSpec;
    #[cfg(verus_keep_ghost)]
    use crate::vstdplus::feq::feq::*;

    verus! {

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
        vstd::seq_lib::group_seq_properties,
    };

    pub open spec fn obeys_spec_eq<T: PartialEq>() -> bool {
        forall|x: T, y: T| x.eq_spec(&y) <==> x == y
    }

    pub fn vec_mem<T: Eq + PartialEq>(v: &Vec<T>, needle: &T) -> (found: bool)
        requires
            obeys_spec_eq::<T>(),
            T::obeys_eq_spec(),
        ensures
            found == v@.contains(*needle),
    {
        let len = v.len();
        let mut j: usize = 0;
        #[verifier::loop_isolation(false)]
        while j < len
            invariant
                j <= len,
                len == v@.len(),
                forall|m: int| 0 <= m < j ==> v@[m] != *needle,
            decreases len - j,
        {
            if v[j] == *needle {
                return true;
            }
            j += 1;
        }
        false
    }

    pub open spec fn spec_collect_keys_rec<K, V>(pairs: Seq<(K, V)>) -> Seq<K>
        decreases pairs.len()
    {
        if pairs.len() == 0 {
            Seq::empty()
        } else {
            let rest = spec_collect_keys_rec(pairs.drop_last());
            let k = pairs.last().0;
            if rest.contains(k) { rest } else { rest.push(k) }
        }
    }

    pub open spec fn spec_collect_keys_deduplicating<K, V>(pairs: Seq<(K, V)>) -> Seq<K> {
        pairs.map_values(|p: (K, V)| p.0).remove_duplicates(Seq::empty())
    }

    pub open spec fn spec_values_for_key_rec<K, V>(pairs: Seq<(K, V)>, k: K) -> Seq<V>
        decreases pairs.len()
    {
        if pairs.len() == 0 {
            Seq::empty()
        } else {
            let rest = spec_values_for_key_rec(pairs.drop_last(), k);
            if pairs.last().0 == k { rest.push(pairs.last().1) } else { rest }
        }
    }

    // No set/seq equivalent for values_for_key: needs filter or recursion.

    pub open spec fn spec_collect_values_rec<K, V>(pairs: Seq<(K, V)>, keys: Seq<K>) -> Seq<(K, Seq<V>)>
        decreases keys.len()
    {
        if keys.len() == 0 {
            Seq::empty()
        } else {
            spec_collect_values_rec(pairs, keys.drop_last()).push(
                (keys.last(), spec_values_for_key_rec(pairs, keys.last()))
            )
        }
    }

    pub open spec fn spec_collect_rec<K, V>(pairs: Seq<(K, V)>) -> Seq<(K, Seq<V>)> {
        spec_collect_values_rec(pairs, spec_collect_keys_rec(pairs))
    }

    pub open spec fn spec_collect_deduplicating<K, V>(pairs: Seq<(K, V)>) -> Seq<(K, Seq<V>)> {
        spec_collect_values_rec(pairs, spec_collect_keys_deduplicating(pairs))
    }

    pub fn collect_keys_rec<K: Clone + Eq + PartialEq, V>(
        pairs: &Vec<(K, V)>,
    ) -> (keys: Vec<K>)
        requires
            obeys_feq_clone::<K>(),
            obeys_spec_eq::<K>(),
            K::obeys_eq_spec(),
        ensures
            keys@ =~= spec_collect_keys_rec(pairs@),
    {
        let plen = pairs.len();
        let mut keys: Vec<K> = Vec::new();
        let ghost s = pairs@;
        let mut i: usize = 0;
        #[verifier::loop_isolation(false)]
        while i < plen
            invariant
                i <= plen,
                plen == s.len(),
                s == pairs@,
                keys@ =~= spec_collect_keys_rec(s.take(i as int)),
            decreases plen - i,
        {
            proof {
                assert(s.take(i as int + 1) =~= s.take(i as int).push(s[i as int]));
                let ghost t = s.take(i as int + 1);
                assert(t.len() > 0);
                assert(t.drop_last() =~= s.take(i as int));
                assert(t.last() == s[i as int]);
                reveal(spec_collect_keys_rec);
            }
            let k = pairs[i].0.clone();
            proof {
                axiom_cloned_implies_eq_owned::<K>(pairs@[i as int].0, k);
            }
            let found = vec_mem(&keys, &k);
            if !found {
                keys.push(k);
            }
            i += 1;
        }
        proof {
            assert(s.take(plen as int) =~= s);
        }
        keys
    }

    pub fn collect_keys_deduplicating<K: Clone + Eq + PartialEq, V>(
        pairs: &Vec<(K, V)>,
    ) -> (keys: Vec<K>)
        requires
            obeys_feq_clone::<K>(),
            obeys_spec_eq::<K>(),
            K::obeys_eq_spec(),
        ensures
            keys@ =~= spec_collect_keys_deduplicating(pairs@),
    {
        let plen = pairs.len();
        let mut keys: Vec<K> = Vec::new();
        let ghost all_keys = pairs@.map_values(|p: (K, V)| p.0);
        let mut i: usize = 0;
        #[verifier::loop_isolation(false)]
        while i < plen
            invariant
                i <= plen,
                plen == pairs@.len(),
                all_keys == pairs@.map_values(|p: (K, V)| p.0),
                all_keys.remove_duplicates(Seq::empty())
                    =~= all_keys.skip(i as int).remove_duplicates(keys@),
            decreases plen - i,
        {
            let k = pairs[i].0.clone();
            proof {
                axiom_cloned_implies_eq_owned::<K>(pairs@[i as int].0, k);
                assert(all_keys[i as int] == pairs@[i as int].0);
                assert(k == all_keys[i as int]);
                assert(all_keys.skip(i as int)[0] == all_keys[i as int]);
                assert(all_keys.skip(i as int).skip(1) =~= all_keys.skip(i as int + 1));
                reveal(Seq::remove_duplicates);
            }
            let found = vec_mem(&keys, &k);
            if !found {
                proof {
                    assert(keys@.push(k) =~= keys@ + seq![all_keys[i as int]]);
                }
                keys.push(k);
            }
            i += 1;
        }
        proof {
            assert(all_keys.skip(plen as int) =~= Seq::empty());
            reveal(Seq::remove_duplicates);
        }
        keys
    }

    pub fn collect_values_rec<K: Clone + Eq + PartialEq, V: Clone + Eq + PartialEq>(
        pairs: &Vec<(K, V)>,
        keys: &Vec<K>,
    ) -> (groups: Vec<(K, Vec<V>)>)
        requires
            obeys_feq_clone::<K>(),
            obeys_feq_clone::<V>(),
            obeys_spec_eq::<K>(),
            K::obeys_eq_spec(),
        ensures
            groups@.len() == keys@.len(),
            forall|i: int| #![trigger groups@[i]] 0 <= i < groups@.len() ==>
                groups@[i].0 == keys@[i] &&
                groups@[i].1@ =~= spec_values_for_key_rec(pairs@, keys@[i]),
    {
        let klen = keys.len();
        let plen = pairs.len();
        let mut groups: Vec<(K, Vec<V>)> = Vec::new();
        let mut ki: usize = 0;
        #[verifier::loop_isolation(false)]
        while ki < klen
            invariant
                ki <= klen,
                klen == keys@.len(),
                plen == pairs@.len(),
                groups@.len() == ki as nat,
                forall|i: int| #![trigger groups@[i]] 0 <= i < ki ==>
                    groups@[i].0 == keys@[i] &&
                    groups@[i].1@ =~= spec_values_for_key_rec(pairs@, keys@[i]),
            decreases klen - ki,
        {
            let k = keys[ki].clone();
            proof { axiom_cloned_implies_eq_owned::<K>(keys@[ki as int], k); }
            let mut vals: Vec<V> = Vec::new();
            let mut pi: usize = 0;
            #[verifier::loop_isolation(false)]
            while pi < plen
                invariant
                    pi <= plen,
                    plen == pairs@.len(),
                    ki < klen,
                    klen == keys@.len(),
                    k == keys@[ki as int],
                    vals@ =~= spec_values_for_key_rec(pairs@.take(pi as int), k),
                decreases plen - pi,
            {
                proof {
                    assert(pairs@.take(pi as int + 1) =~= pairs@.take(pi as int).push(pairs@[pi as int]));
                    let ghost t = pairs@.take(pi as int + 1);
                    assert(t.len() > 0);
                    assert(t.drop_last() =~= pairs@.take(pi as int));
                    assert(t.last() == pairs@[pi as int]);
                    reveal(spec_values_for_key_rec);
                }
                let pk = pairs[pi].0.clone();
                proof { axiom_cloned_implies_eq_owned::<K>(pairs@[pi as int].0, pk); }
                if pk == k {
                    let v = pairs[pi].1.clone();
                    proof { axiom_cloned_implies_eq_owned::<V>(pairs@[pi as int].1, v); }
                    vals.push(v);
                }
                pi += 1;
            }
            proof {
                assert(pairs@.take(plen as int) =~= pairs@);
            }
            groups.push((k, vals));
            ki += 1;
        }
        groups
    }

    pub fn collect_values_deduplicating<K: Clone + Eq + PartialEq, V: Clone + Eq + PartialEq>(
        pairs: &Vec<(K, V)>,
        keys: &Vec<K>,
    ) -> (groups: Vec<(K, Vec<V>)>)
        requires
            obeys_feq_clone::<K>(),
            obeys_feq_clone::<V>(),
            obeys_spec_eq::<K>(),
            K::obeys_eq_spec(),
        ensures
            groups@.len() == keys@.len(),
            forall|i: int| #![trigger groups@[i]] 0 <= i < groups@.len() ==>
                groups@[i].0 == keys@[i] &&
                groups@[i].1@ =~= spec_values_for_key_rec(pairs@, keys@[i]),
    {
        let klen = keys.len();
        let plen = pairs.len();
        let mut groups: Vec<(K, Vec<V>)> = Vec::new();
        let mut ki: usize = 0;
        #[verifier::loop_isolation(false)]
        while ki < klen
            invariant
                ki <= klen,
                klen == keys@.len(),
                plen == pairs@.len(),
                groups@.len() == ki as nat,
                forall|i: int| #![trigger groups@[i]] 0 <= i < ki ==>
                    groups@[i].0 == keys@[i] &&
                    groups@[i].1@ =~= spec_values_for_key_rec(pairs@, keys@[i]),
            decreases klen - ki,
        {
            let k = keys[ki].clone();
            proof { axiom_cloned_implies_eq_owned::<K>(keys@[ki as int], k); }
            let mut vals: Vec<V> = Vec::new();
            let mut pi: usize = 0;
            #[verifier::loop_isolation(false)]
            while pi < plen
                invariant
                    pi <= plen,
                    plen == pairs@.len(),
                    ki < klen,
                    klen == keys@.len(),
                    k == keys@[ki as int],
                    vals@ =~= spec_values_for_key_rec(pairs@.take(pi as int), k),
                decreases plen - pi,
            {
                proof {
                    assert(pairs@.take(pi as int + 1) =~= pairs@.take(pi as int).push(pairs@[pi as int]));
                    let ghost t = pairs@.take(pi as int + 1);
                    assert(t.len() > 0);
                    assert(t.drop_last() =~= pairs@.take(pi as int));
                    assert(t.last() == pairs@[pi as int]);
                    reveal(spec_values_for_key_rec);
                }
                let pk = pairs[pi].0.clone();
                proof { axiom_cloned_implies_eq_owned::<K>(pairs@[pi as int].0, pk); }
                if pk == k {
                    let v = pairs[pi].1.clone();
                    proof { axiom_cloned_implies_eq_owned::<V>(pairs@[pi as int].1, v); }
                    vals.push(v);
                }
                pi += 1;
            }
            proof {
                assert(pairs@.take(plen as int) =~= pairs@);
            }
            groups.push((k, vals));
            ki += 1;
        }
        groups
    }

    pub fn collect_rec<K: Clone + Eq + PartialEq, V: Clone + Eq + PartialEq>(
        pairs: &Vec<(K, V)>,
    ) -> (groups: Vec<(K, Vec<V>)>)
        requires
            obeys_feq_clone::<K>(),
            obeys_feq_clone::<V>(),
            obeys_spec_eq::<K>(),
            K::obeys_eq_spec(),
        ensures
            groups@.len() == spec_collect_keys_rec(pairs@).len(),
            forall|i: int| #![trigger groups@[i]] 0 <= i < groups@.len() ==>
                groups@[i].0 == spec_collect_keys_rec(pairs@)[i] &&
                groups@[i].1@ =~= spec_values_for_key_rec(pairs@, spec_collect_keys_rec(pairs@)[i]),
    {
        let keys = collect_keys_rec(pairs);
        collect_values_rec(pairs, &keys)
    }

    pub fn collect_deduplicating<K: Clone + Eq + PartialEq, V: Clone + Eq + PartialEq>(
        pairs: &Vec<(K, V)>,
    ) -> (groups: Vec<(K, Vec<V>)>)
        requires
            obeys_feq_clone::<K>(),
            obeys_feq_clone::<V>(),
            obeys_spec_eq::<K>(),
            K::obeys_eq_spec(),
        ensures
            groups@.len() == spec_collect_keys_deduplicating(pairs@).len(),
            forall|i: int| #![trigger groups@[i]] 0 <= i < groups@.len() ==>
                groups@[i].0 == spec_collect_keys_deduplicating(pairs@)[i] &&
                groups@[i].1@ =~= spec_values_for_key_rec(pairs@, spec_collect_keys_deduplicating(pairs@)[i]),
    {
        let keys = collect_keys_deduplicating(pairs);
        collect_values_deduplicating(pairs, &keys)
    }

    } // verus!
}
