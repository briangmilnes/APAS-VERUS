//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 3 insertion sort over mutable slices.

pub mod InsertionSortStEph {

    use vstd::prelude::*;
    use vstd::multiset::*;

    verus! {

        broadcast use vstd::seq_lib::group_to_multiset_ensures;

        pub trait InsertionSortStTrait {
            fn insertion_sort(a: &mut [u64]) -> &[u64];
        }

        pub open spec fn is_sorted(v: &[u64]) -> bool {
            forall|i: int, j: int| 0 <= i < j < v.len() ==> #[trigger] v[i] <= #[trigger] v[j]
        }

        pub open spec fn sorted_prefix(v: &[u64], i: int) -> bool {
            forall|k: int, l: int| 0 <= k < l < i ==> #[trigger] v[k] <= #[trigger] v[l]
        }

       pub fn insertion_sort(a: &mut [u64]) -> (r: &[u64])
           ensures r.len() == old(a).len(),
                   r@.to_multiset() == old(a)@.to_multiset(),
                   is_sorted(r),
        {
            if a.len() <= 1 {
                a
            } else {
                let alen = a.len();
                let mut up = 1;
                while up < alen
                    invariant 
                        a.len() == old(a).len(),
                        a.len() == alen,
                        0 < up <= alen,
                        a@.to_multiset() == old(a)@.to_multiset(),
                        sorted_prefix(a, up as int),
                    decreases a.len() - up
                {
                    let mut down = up;
                    while down >= 1 && a[down - 1] > a[down]
                        invariant
                            a.len() == old(a).len(),
                            a.len() == alen,
                            down < alen,
                            0 <= down <= up,
                            a@.to_multiset() == old(a)@.to_multiset(),
                            sorted_prefix(a, down as int),
                            forall|i: int, j: int| 0 <= i < j <= up && i < a.len() && j < a.len() && j != down ==> #[trigger] a[i] <= #[trigger] a[j],
                        decreases down
                    {
                        let swapme = a[down];
                        a[down] = a[down - 1];
                        a[down - 1] = swapme;
                        down -= 1;
                    }
                    up += 1;
                }
                a
                }
            }
    }
}
