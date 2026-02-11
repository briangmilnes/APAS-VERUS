//  Experiment: collect2 (group-by-key, single loop)
//
//  HYPOTHESIS: we can do this in a single function dual loop and prove it's
//  specification and ordering more easily.
//
//  RESULT: 


pub mod collect2 {

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

    pub fn vec_find_key<K: Eq + PartialEq, V>(
        collected: &Vec<(K, Vec<V>)>,
        needle: &K,
    ) -> (found: Option<usize>)
        requires
            obeys_spec_eq::<K>(),
            K::obeys_eq_spec(),
        ensures
            match found {
                Some(idx) => idx < collected@.len() && collected@[idx as int].0 == *needle,
                None => forall|m: int| #![trigger collected@[m]] 0 <= m < collected@.len() ==> collected@[m].0 != *needle,
            },
    {
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
            obeys_spec_eq::<K>(),
            K::obeys_eq_spec(),
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
            decreases plen - i,
        {
            let k = pairs[i].0.clone();
            let v = pairs[i].1.clone();
            match vec_find_key(&collected, &k) {
                Some(idx) => {
                    let mut entry = collected.remove(idx);
                    entry.1.push(v);
                    collected.insert(idx, entry);
                }
                None => {
                    let mut vals: Vec<V> = Vec::new();
                    vals.push(v);
                    collected.push((k, vals));
                }
            }
            i += 1;
        }
        collected
    }

    } // verus!
}
