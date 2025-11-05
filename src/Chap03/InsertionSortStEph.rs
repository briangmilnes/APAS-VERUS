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

        pub open spec fn slice_j_excl_i_incl_gt_key(a: &[u64], j: int, i: int, key: u64) -> bool {
            forall|k: int| j < k <= i ==> #[trigger] a[k] > key
        }

        proof fn lemma_sorted_prefix_full_implies_sorted(v: &[u64])
            requires sorted_prefix(v, v.len() as int)
            ensures is_sorted(v)
        {
            assert forall|i: int, j: int| 0 <= i < j < v.len() implies #[trigger] v[i] <= #[trigger] v[j] by {
                assert(sorted_prefix(v, v.len() as int));
            }
        }

        proof fn lemma_insert_key_extends_sorted_prefix(a: &[u64], j: int, i: int, key: u64)
            requires
                0 <= j <= i,
                sorted_prefix(a, j),
                slice_j_excl_i_incl_gt_key(a, j, i, key),
                j == 0 || a[(j - 1) as int] <= key,
            ensures
                sorted_prefix(a, (i + 1) as int),
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
                let l = a.len();
                let mut i: usize = 1;
                while i < l
                    invariant 
                        a.len() == old(a).len(),
                        a.len() == l,
                        0 < i <= l,
                        a@.to_multiset() == old(a)@.to_multiset(),
                        sorted_prefix(a, i as int),
                    decreases a.len() - i
                {
                    let mut j = i;
                    while j > 0   
                      invariant 
                        a.len() == old(a).len(),
                        a.len() == l,
                        j < l,
                        0 <= j <= i,
                        a@.to_multiset() == old(a)@.to_multiset(),
                        sorted_prefix(a, j as int),
                      decreases j
                    {
                        if a[j - 1] > a[j] {
                            let tmp = a[j];
                            a[j] = a[j - 1];
                            a[j - 1] = tmp;
                            j -= 1;
                        } else {
                            break;
                        }
                    }
                    i += 1;
                }
                a
                }
            }
    }
}
