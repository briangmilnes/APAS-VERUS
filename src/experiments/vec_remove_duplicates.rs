//  Experiment: vec_remove_duplicates
//
//  Executable remove_duplicates on Vec, proven against Seq::remove_duplicates.

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

pub fn vec_mem_from<T: Eq + PartialEq>(v: &Vec<T>, from: usize, needle: &T) -> (found: bool)
    requires
        obeys_spec_eq::<T>(),
        T::obeys_eq_spec(),
        from <= v@.len(),
    ensures
        found == v@.skip(from as int).contains(*needle),
{
    let len = v.len();
    let mut j: usize = from;
    #[verifier::loop_isolation(false)]
    while j < len
        invariant
            from <= j <= len,
            len == v@.len(),
            forall|m: int| from as int <= m < j ==> v@[m] != *needle,
        decreases len - j,
    {
        if v[j] == *needle {
            proof {
                let wit = (j - from) as int;
                assert(v@.skip(from as int)[wit] == v@[j as int]);
                assert(v@.skip(from as int).contains(*needle));
            }
            return true;
        }
        j += 1;
    }
    proof {
        let s = v@.skip(from as int);
        assert forall|k: int| 0 <= k < s.len() implies s[k] != *needle by {
            assert(s[k] == v@[from as int + k]);
        }
    }
    false
}

pub fn vec_remove_duplicates<T: Clone + Eq + PartialEq>(v: &Vec<T>) -> (deduplicated: Vec<T>)
    requires
        obeys_feq_clone::<T>(),
        obeys_spec_eq::<T>(),
        T::obeys_eq_spec(),
    ensures
        deduplicated@ =~= v@.remove_duplicates(Seq::empty()),
{
    let len = v.len();
    let mut deduplicated: Vec<T> = Vec::new();
    let mut i: usize = 0;
    #[verifier::loop_isolation(false)]
    while i < len
        invariant
            i <= len,
            len == v@.len(),
            v@.remove_duplicates(Seq::empty()) =~= v@.skip(i as int).remove_duplicates(deduplicated@),
        decreases len - i,
    {
        let found = vec_mem_from(&deduplicated, 0, &v[i]);
        proof {
            assert(deduplicated@.skip(0) =~= deduplicated@);
            assert(found == deduplicated@.contains(v@[i as int]));
            assert(v@.skip(i as int)[0] == v@[i as int]);
            assert(v@.skip(i as int).skip(1) =~= v@.skip(i as int + 1));
            reveal(Seq::remove_duplicates);
        }
        if !found {
            let elem = v[i].clone();
            proof {
                axiom_cloned_implies_eq_owned::<T>(v@[i as int], elem);
                assert(deduplicated@.push(elem) =~= deduplicated@ + seq![v@[i as int]]);
            }
            deduplicated.push(elem);
        }
        i += 1;
    }
    proof {
        assert(v@.skip(len as int) =~= Seq::empty());
        reveal(Seq::remove_duplicates);
    }
    deduplicated
}

} // verus!
