//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 3 insertion sort over mutable slices.

pub mod InsertionSortStEph {

    use vstd::prelude::*;
    use vstd::multiset::*;

    verus! {

        pub open spec fn is_sorted(v: &[u64]) -> bool {
            forall|i: int, j: int| 0 <= i < j < v.len() ==> #[trigger] v[i] <= #[trigger] v[j]
        }

        pub open spec fn sorted_prefix(v: &[u64], i: int) -> bool {
            forall|k: int, l: int| 0 <= k < l < i ==> #[trigger] v[k] <= #[trigger] v[l]
        }

        pub open spec fn slice_down_excl_up_incl_gt_key(a: &[u64], down: int, up: int, key: u64) -> bool {
            forall|k: int| down < k <= up ==> #[trigger] a[k] > key
        }

        proof fn lemma_sorted_prefix_full_implies_sorted(v: &[u64])
            requires sorted_prefix(v, v.len() as int)
            ensures is_sorted(v)
        {
            assert forall|i: int, j: int| 0 <= i < j < v.len() implies #[trigger] v[i] <= #[trigger] v[j] by {
                assert(sorted_prefix(v, v.len() as int));
            }
        }

        proof fn lemma_insert_key_extends_sorted_prefix(a: &[u64], down: int, up: int, key: u64)
            requires
                0 <= down <= up,
                sorted_prefix(a, down),
                slice_down_excl_up_incl_gt_key(a, down, up, key),
                down == 0 || a[(down - 1) as int] <= key,
            ensures
                sorted_prefix(a, (up + 1) as int),
        {
            assume(false);
        }

       pub trait InsertionSortStTrait {
         fn insertion_sort(a: &mut [u64]) -> &[u64];
       }

       pub fn insertion_sort(a: &mut [u64]) -> (r: &[u64])
           ensures r.len() == old(a).len(),
                   r@.to_multiset() == old(a)@.to_multiset(),
        {
            if a.len() <= 1 {
                a
            } else {
                assert(a@.to_multiset() == old(a)@.to_multiset());
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
                    while down > 0   
                      invariant 
                        a.len() == old(a).len(),
                        a.len() == alen,
                        down < alen,
                        0 <= down <= up,
                        a@.to_multiset() == old(a)@.to_multiset(),
                        sorted_prefix(a, down as int),
                      decreases down
                    {
                        if a[down - 1] > a[down] {
                            let swapme = a[down];
                            a[down] = a[down - 1];
                            a[down - 1] = swapme;
                            down -= 1;
                        } else {
                            break;
                        }
                    }
                    up += 1;
                }
                a
                }
            }
    }
}
