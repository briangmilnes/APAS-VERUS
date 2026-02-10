//  Experiment: collect (group-by-key)
//
//  HYPOTHESIS: A standalone collect_keys on Vec<(K,V)> can be proven against
//  a recursive spec, using clone equality axioms and Seq::contains.
//  This avoids the ArraySeqS View indirection and tests the core grouping logic.
//
//  Working directly on Vec<(K,V)> where K: Eq + PartialEq + Clone.

#[allow(unused_imports)]
pub mod collect {

    use vstd::prelude::*;

    verus! {

    broadcast use {
        vstd::std_specs::vec::group_vec_axioms,
        vstd::seq::group_seq_axioms,
        vstd::seq_lib::group_seq_properties,
    }

    // Clone equality axiom: cloning a value produces a spec-equal copy.
    pub open spec fn obeys_clone_eq<T>() -> bool {
        forall|x: T, y: T| #[trigger] cloned(x, y) ==> x == y
    }

    // Recursive spec: distinct keys from pairs, in first-occurrence order.
    // spec_collect_keys([(a,1),(b,2),(a,3)]) = [a, b].
    pub open spec fn spec_collect_keys<K, V>(pairs: Seq<(K, V)>) -> Seq<K>
        decreases pairs.len()
    {
        if pairs.len() == 0 {
            Seq::empty()
        } else {
            let rest = spec_collect_keys(pairs.drop_last());
            let k = pairs.last().0;
            if rest.contains(k) { rest } else { rest.push(k) }
        }
    }

    // Exec: extract distinct keys from a Vec of pairs.
    pub fn collect_keys<K: Clone + Eq + PartialEq, V>(
        pairs: &Vec<(K, V)>,
    ) -> (keys: Vec<K>)
        requires
            obeys_clone_eq::<K>(),
        ensures
            keys@ =~= spec_collect_keys(pairs@),
    {
        let plen = pairs.len();
        let mut keys: Vec<K> = Vec::new();
        let ghost s = pairs@;
        let mut i: usize = 0;
        while i < plen
            invariant
                i <= plen,
                plen == s.len(),
                s == pairs@,
                obeys_clone_eq::<K>(),
                keys@ =~= spec_collect_keys(s.take(i as int)),
            decreases plen - i,
        {
            proof {
                assert(s.take(i as int + 1) =~= s.take(i as int).push(s[i as int]));
                let ghost t = s.take(i as int + 1);
                assert(t.len() > 0);
                assert(t.drop_last() =~= s.take(i as int));
                assert(t.last() == s[i as int]);
                reveal(spec_collect_keys);
            }
            let k = pairs[i].0.clone();
            proof {
                assert(cloned(pairs@[i as int].0, k));
            }
            let klen = keys.len();
            let mut found = false;
            let mut j: usize = 0;
            while j < klen
                invariant_except_break
                    j <= klen,
                    klen == keys@.len(),
                    !found,
                    forall|m: int| 0 <= m < j ==> keys@[m] != k,
                decreases klen - j,
            {
                if keys[j] == k {
                    found = true;
                    break;
                }
                j += 1;
            }
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

    } // verus!
} // mod collect
