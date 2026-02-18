//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.
//! Chapter 3 insertion sort over mutable slices - Generic version using TotalOrder trait.
pub mod InsertionSortStEph {

    use vstd::multiset::*;
    use vstd::prelude::*;
    use vstd::relations::*;
    use crate::vstdplus::total_order::total_order::*;
    use core::cmp::Ordering;

    verus! {

broadcast use {
    vstd::seq_lib::group_to_multiset_ensures,
    vstd::seq::group_seq_axioms,
};

pub open spec fn sorted_prefix<T: TotalOrder>(v: &[T], i: int) -> bool {
    forall|k: int, l: int| 0 <= k < l < i ==> T::le(#[trigger] v[k], #[trigger] v[l])
}

/// Prefix elements [0, mid) are <= suffix elements (mid, hi].
pub open spec fn cross_sorted<T: TotalOrder>(v: Seq<T>, mid: int, hi: int) -> bool {
    forall|i: int, j: int| 0 <= i < mid < j <= hi ==> T::le(#[trigger] v[i], #[trigger] v[j])
}

pub open spec fn is_sorted<T: TotalOrder>(v: &[T]) -> bool {
    forall|i: int, j: int| 0 <= i < j < v.len() ==> T::le(#[trigger] v[i], #[trigger] v[j])
}

/// - APAS: Work Θ(n²), Span Θ(n²) — sequential insertion sort.
/// - Claude-Opus-4.6: Work Θ(n²), Span Θ(n²) — agrees with APAS. Iterative in-place variant, same cost as the recursive prose version.
#[cfg_attr(verus_keep_ghost, verifier::loop_isolation(false))]
pub fn insertion_sort<T: TotalOrder + Copy>(a: &mut [T]) -> (sorted: &[T])
        ensures
          sorted.len() == old(a).len(),
          sorted@.to_multiset() == old(a)@.to_multiset(),
          is_sorted(sorted),
{
    if a.len() <= 1 {
        a
    } else {
        let mut up: usize = 1;
        while up < a.len()
            invariant
                a.len() == old(a).len(),
                0 < up <= a.len(),
                a@.to_multiset() == old(a)@.to_multiset(),
                forall|k: int, l: int| 0 <= k < l < up as int
                    ==> T::le(#[trigger] a@[k], #[trigger] a@[l]),
            decreases a.len() - up,
        {
            let mut down: usize = up;
            let mut swapped = true;
            while down > 0 && swapped
                invariant
                    a.len() == old(a).len(),
                    0 <= down <= up < a.len(),
                    a@.to_multiset() == old(a)@.to_multiset(),
                    forall|k: int, l: int| 0 <= k < l < down as int
                        ==> T::le(#[trigger] a@[k], #[trigger] a@[l]),
                    forall|k: int, l: int| down as int <= k < l <= up as int
                        ==> T::le(#[trigger] a@[k], #[trigger] a@[l]),
                    cross_sorted::<T>(a@, down as int, up as int),
                    !swapped ==> (down > 0 && T::le(a@[down as int - 1], a@[down as int])),
                decreases down, if swapped { 1int } else { 0int },
            {
                match a[down - 1].cmp(&a[down]) {
                    Ordering::Greater => {
                        let tmp = a[down];
                        a[down] = a[down - 1];
                        a[down - 1] = tmp;
                        down -= 1;
                    }
                    _ => {
                        proof { T::reflexive(a@[down as int - 1]); }
                        swapped = false;
                    }
                }
            }
            proof {
                assert forall|k: int, l: int| 0 <= k < l <= up as int
                    implies T::le(#[trigger] a@[k], #[trigger] a@[l]) by {
                    if down as int == 0 {
                        // Segment invariant with down==0 covers [0, up].
                    } else {
                        // !swapped: T::le(a[down-1], a[down]) bridges prefix and segment.
                        if k < down as int {
                            if l == down as int {
                                if k < down as int - 1 {
                                    T::transitive(a@[k], a@[down as int - 1], a@[down as int]);
                                }
                            } else if l > down as int {
                                if k < down as int - 1 {
                                    T::transitive(a@[k], a@[down as int - 1], a@[down as int]);
                                }
                                T::transitive(a@[k], a@[down as int], a@[l]);
                            }
                        }
                    }
                };
            }
            up += 1;
        }
        a
    }
}

} // verus!
}
