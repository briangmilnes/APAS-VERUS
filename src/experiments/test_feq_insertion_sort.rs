// Copyright (c) 2025 Brian G. Milnes
//! Insertion sort experiments
//!
//! Two implementations:
//! 1. while-loop form (which mostly fails for us in Verus)
//! 2. loop-loop form (our workaround pattern)

pub mod test_feq_insertion_sort {
    use vstd::prelude::*;
    use vstd::std_specs::cmp::OrdSpec;
    use vstd::laws_cmp::obeys_cmp_spec;
    use core::cmp::Ordering;

    verus! {

// Veracity: added broadcast group
broadcast use {
    crate::vstdplus::feq::feq::group_feq_axioms,
    vstd::seq::group_seq_axioms,
    vstd::seq_lib::group_seq_properties,
    vstd::seq_lib::group_to_multiset_ensures,
};

    pub open spec fn sorted_spec<T: Ord + Sized>(s: Seq<T>) -> bool {
        forall|i: int, j: int| 0 <= i < j < s.len() ==> s[i].cmp_spec(&s[j]) != Ordering::Greater
    }

    pub fn is_sorted<T: Ord + Clone + Sized>(v: &Vec<T>) -> (sorted: bool)
        requires obeys_cmp_spec::<T>()
        ensures sorted <==> sorted_spec(v@)
    {
        if v.len() <= 1 {
            return true;
        }
        let mut i: usize = 0;
        #[verifier::loop_isolation(false)]
        loop
            invariant
                0 <= i < v.len(),
                obeys_cmp_spec::<T>(),
                forall|a: int, b: int| 0 <= a < b <= i as int ==> v@[a].cmp_spec(&v@[b]) != Ordering::Greater,
            decreases v.len() - i,
        {
            if i + 1 >= v.len() {
                return true;
            }
            let cmp_result = v[i].cmp(&v[i + 1]);
            proof {
                assume(cmp_result == v@[i as int].cmp_spec(&v@[(i + 1) as int]));
            }
            if matches!(cmp_result, Ordering::Greater) {
                return false;
            }
            proof {
                assume(forall|a: int, b: int| 0 <= a < b <= (i + 1) as int ==> v@[a].cmp_spec(&v@[b]) != Ordering::Greater);
            }
            i = i + 1;
        }
    }

    // While-loop form - natural but struggles in Verus
    pub fn insertion_sort_while<T: Ord + Copy + Sized>(a: &mut Vec<T>)
        requires
            obeys_cmp_spec::<T>(),
            old(a).len() < usize::MAX,
        ensures
            a.len() == old(a).len(),
            sorted_spec(a@),
    {
        if a.len() <= 1 {
            return;
        }
        let mut up: usize = 1;
        while up < a.len()
            invariant
                a.len() == old(a).len(),
                0 < up <= a.len(),
                obeys_cmp_spec::<T>(),
                forall|i: int, j: int| 0 <= i < j < up as int ==> a@[i].cmp_spec(&a@[j]) != Ordering::Greater,
            decreases a.len() - up,
        {
            let mut down: usize = up;
            while down >= 1
                invariant
                    a.len() == old(a).len(),
                    0 <= down <= up,
                    up < a.len(),
                    obeys_cmp_spec::<T>(),
                decreases down,
            {
                if down == 0 {
                    break;
                }
                let cmp_result = a[down - 1].cmp(&a[down]);
                proof {
                    assume(cmp_result == a@[(down - 1) as int].cmp_spec(&a@[down as int]));
                }
                match cmp_result {
                    Ordering::Greater => {
                        let swapme = a[down];
                        a.set(down, a[down - 1]);
                        a.set(down - 1, swapme);
                        down = down - 1;
                    }
                    Ordering::Less | Ordering::Equal => {
                        break;
                    }
                }
            }
            up = up + 1;
            proof {
                assume(forall|i: int, j: int| 0 <= i < j < up as int ==> a@[i].cmp_spec(&a@[j]) != Ordering::Greater);
            }
        }
        proof {
            assume(sorted_spec(a@));
        }
    }

    // Loop-loop form - our workaround pattern
    pub fn insertion_sort_loop<T: Ord + Copy + Sized>(a: &mut Vec<T>)
        requires
            obeys_cmp_spec::<T>(),
            old(a).len() < usize::MAX,
        ensures
            a.len() == old(a).len(),
            sorted_spec(a@),
    {
        if a.len() <= 1 {
            return;
        }
        let ghost old_len = a.len();
        let mut up: usize = 1;

        #[verifier::loop_isolation(false)]
        loop
            invariant
                a.len() == old_len,
                0 < up <= a.len(),
                obeys_cmp_spec::<T>(),
                forall|i: int, j: int| 0 <= i < j < up as int ==> a@[i].cmp_spec(&a@[j]) != Ordering::Greater,
            decreases a.len() - up,
        {
            if up >= a.len() {
                return;
            }

            let mut down: usize = up;

            #[verifier::loop_isolation(false)]
            loop
                invariant
                    a.len() == old_len,
                    0 <= down <= up,
                    up < a.len(),
                    obeys_cmp_spec::<T>(),
                decreases down,
            {
                if down == 0 {
                    break;
                }

                let cmp_result = a[down - 1].cmp(&a[down]);
                proof {
                    assume(cmp_result == a@[(down - 1) as int].cmp_spec(&a@[down as int]));
                }

                match cmp_result {
                    Ordering::Greater => {
                        let swapme = a[down];
                        a.set(down, a[down - 1]);
                        a.set(down - 1, swapme);
                        down = down - 1;
                    }
                    Ordering::Less | Ordering::Equal => {
                        break;
                    }
                }
            }

            proof {
                assume(forall|i: int, j: int| 0 <= i < j < (up + 1) as int ==> a@[i].cmp_spec(&a@[j]) != Ordering::Greater);
            }

            up = up + 1;
        }
    }

    pub fn test_insertion_sort_while<T: Ord + Copy + Sized>(v: Vec<T>) -> (sorted_v: Vec<T>)
        requires
            obeys_cmp_spec::<T>(),
            v.len() < usize::MAX,
        ensures
            sorted_v.len() == v.len(),
            sorted_spec(sorted_v@),
    {
        let mut result = v;
        insertion_sort_while(&mut result);
        result
    }

    pub fn test_insertion_sort_loop<T: Ord + Copy + Sized>(v: Vec<T>) -> (sorted_v: Vec<T>)
        requires
            obeys_cmp_spec::<T>(),
            v.len() < usize::MAX,
        ensures
            sorted_v.len() == v.len(),
            sorted_spec(sorted_v@),
    {
        let mut result = v;
        insertion_sort_loop(&mut result);
        result
    }

    } // verus!
}
