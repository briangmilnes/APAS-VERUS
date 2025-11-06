//! Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 3 insertion sort over mutable slices - Generic version.
//!
//! NOTE: This generic version does not currently verify with Verus.
//! The issue is that spec_le() method calls on generic T: SpecOrd don't work
//! in forall quantifiers. Error: "types are not compatible with this operator"
//! This appears to be a limitation in how Verus handles generic comparison operators
//! in spec contexts. The u64-specific version (InsertionSortStEph.rs) verifies fully.

pub mod InsertionSortStEphGeneric {

    use vstd::prelude::*;
    use vstd::multiset::*;

    verus! {

        broadcast use vstd::seq_lib::group_to_multiset_ensures;

        pub trait InsertionSortStTrait<T> {
            fn insertion_sort(a: &mut [T]) -> &[T];
        }

        pub open spec fn is_sorted<T: SpecOrd>(v: &[T]) -> bool {
            forall|i: int, j: int| 0 <= i < j < v.len() ==> (#[trigger] v[i]).spec_le(#[trigger] v[j])
        }

        pub open spec fn sorted_prefix<T: SpecOrd>(v: &[T], i: int) -> bool {
            forall|k: int, l: int| 0 <= k < l < i ==> v[k].spec_le(v[l])
        }

       pub fn insertion_sort<T: Copy + Ord + SpecOrd>(a: &mut [T]) -> (r: &[T])
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
                            forall|i: int, j: int| 0 <= i < j <= up && i < a.len() && j < a.len() && j != down ==> a[i].spec_le(a[j]),
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

