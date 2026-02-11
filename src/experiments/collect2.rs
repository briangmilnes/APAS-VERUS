//  Experiment: collect2 (group-by-key, single loop)

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

    pub fn vec_find_key<K: Eq + PartialEq, V>(
        collected: &Vec<(K, Vec<V>)>,
        needle: &K,
    ) -> (result: Option<usize>)
        requires
            obeys_spec_eq::<K>(),
            K::obeys_eq_spec(),
        ensures
            match result {
                Some(idx) => idx < collected@.len() && collected@[idx as int].0 == *needle,
                None => forall|m: int| 0 <= m < collected@.len() ==> collected@[m].0 != *needle,
            },
    {
        let len = collected.len();
        let mut j: usize = 0;
        #[verifier::loop_isolation(false)]
        while j < len
            invariant
                j <= len,
                len == collected@.len(),
                forall|m: int| 0 <= m < j ==> collected@[m].0 != *needle,
            decreases len - j,
        {
            if collected[j].0 == *needle {
                return Some(j);
            }
            j += 1;
        }
        None
    }

    // Verus limitation: "index for &mut not supported" prevents collected[idx].1.push(v).
    // Workaround: remove the entry at idx, mutate it, insert it back at the same index.
    pub fn collect2<K: Clone + Eq + PartialEq, V: Clone + Eq + PartialEq>(
        pairs: &Vec<(K, V)>,
    ) -> (collected: Vec<(K, Vec<V>)>)
        requires
            obeys_feq_clone::<K>(),
            obeys_feq_clone::<V>(),
            obeys_spec_eq::<K>(),
            obeys_spec_eq::<V>(),
            K::obeys_eq_spec(),
            V::obeys_eq_spec(),
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
                    if !vec_mem(&entry.1, &v) {
                        entry.1.push(v);
                    }
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
