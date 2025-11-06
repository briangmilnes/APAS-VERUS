//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 3 insertion sort over mutable slices - Generic version using TotalOrdered trait.
pub mod InsertionSortStEph {

    use vstd::multiset::*;
    use vstd::prelude::*;
    use vstd::relations::*;
    use crate::vstdadditions::TotalOrdered::TotalOrdered::*;

    verus! {

broadcast use vstd::seq_lib::group_to_multiset_ensures;

pub trait InsertionSortStTrait<T: TotalOrdered> {
    fn insertion_sort(a: &mut [T]) -> &[T];
}

pub open spec fn sorted_prefix<T: TotalOrdered>(v: &[T], i: int) -> bool {
    forall|k: int, l: int| 0 <= k < l < i ==> T::le(#[trigger] v[k], #[trigger] v[l])
}

pub open spec fn is_sorted<T: TotalOrdered>(v: &[T]) -> bool {
    forall|i: int, j: int| 0 <= i < j < v.len() ==> T::le(#[trigger] v[i], #[trigger] v[j])
}

#[verifier::loop_isolation(false)]
pub fn insertion_sort<T: TotalOrdered + Copy>(a: &mut [T]) -> (r: &[T])
    ensures
        r.len() == old(a).len(),
        r@.to_multiset() == old(a)@.to_multiset(),
{
    if a.len() <= 1 {
        a
    } else {
        let mut up = 1;
        while up < a.len()
            invariant
                a.len() == old(a).len(),
                0 < up <= a.len(),
                a@.to_multiset() == old(a)@.to_multiset(),
            decreases a.len() - up,
        {
            let mut down = up;
            while down >= 1
                invariant
                    a.len() == old(a).len(),
                    down < a.len(),
                    0 <= down <= up,
                    a@.to_multiset() == old(a)@.to_multiset(),
                decreases down,
            {
                if down == 0 {
                    break;
                }
                match a[down - 1].compare(&a[down]) {
                    Cmp::Greater => {
                        let swapme = a[down];
                        a[down] = a[down - 1];
                        a[down - 1] = swapme;
                        down -= 1;
                    }
                    Cmp::Less | Cmp::Equal => {
                        break;
                    }
                }
            }
            up += 1;
        }
        a
    }
}

} // verus!
}
