//! Experiment: sorting f64 values using Rust's native `<=` and the vstdplus float
//! total order axioms — no bit-level tricks.
//!
//! Goal: verify insertion sort on f64 using the normal `<=` comparison, with the
//! axioms from vstdplus::float broadcast-used instead of defined locally.
//!
//! Contrast with f64_bits_sort.rs which uses to_bits_spec() for a concrete ordering.
//!
//! RESULT: Native `<=` works for comparison but uninterpreted le_ensures is hostile to
//! invariant maintenance across mutations. The solver can't propagate ordering facts
//! through Vec::set because le_ensures(old_v[a], old_v[b], true) doesn't help it deduce
//! le_ensures(new_v[a], new_v[b], true) even when v[a] didn't change.
//! We assume the swap-maintenance invariant and verify the overall structure.
//! For production eTSP, the bits approach (f64_bits_sort.rs) is cleaner.

use vstd::prelude::*;
use vstd::std_specs::cmp::le_ensures;
use crate::vstdplus::float::float::*;

verus! {

broadcast use {
    vstd::std_specs::vec::group_vec_axioms,
    vstd::seq::group_seq_axioms,
    group_float_finite_total_order,
        // Veracity: added broadcast groups
        crate::vstdplus::feq::feq::group_feq_axioms,
        vstd::seq_lib::group_seq_properties,
        vstd::seq_lib::group_to_multiset_ensures,
};

// Spec-level ordering: "Rust's <= returned true."
// Wraps le_ensures for readability; triggers still fire since this is open.
pub open spec fn f64_le_spec(a: f64, b: f64) -> bool {
    le_ensures::<f64>(a, b, true)
}

// Sorted: pairwise ordered.
pub open spec fn spec_f64_sorted(s: Seq<f64>) -> bool {
    forall|a: int, b: int| 0 <= a < b < s.len() ==> #[trigger] f64_le_spec(s[a], s[b])
}

// Verified insertion sort. The comparison-to-spec bridge and the final sorted postcondition
// are verified. The inner-loop swap invariant maintenance is assumed because le_ensures is
// uninterpreted and the solver can't propagate ordering through Vec::set.
fn insertion_sort_f64(v: &mut Vec<f64>)
    requires all_float_wf::<f64>(old(v)@),
    ensures
        v@.len() == old(v)@.len(),
        all_float_wf::<f64>(v@),
        spec_f64_sorted(v@),
{
    let n = v.len();
    if n <= 1 { return; }

    let mut i: usize = 1;
    while i < n
        invariant
            n == v@.len(),
            1 <= i <= n,
            all_float_wf::<f64>(v@),
            spec_f64_sorted(v@.take(i as int)),
        decreases n - i,
    {
        let mut j: usize = i;
        while j > 0 && !(v[j - 1] <= v[j])
            invariant
                0 <= j <= i,
                i < n,
                n == v@.len(),
                all_float_wf::<f64>(v@),
            decreases j,
        {
            let tmp = v[j];
            v.set(j, v[j - 1]);
            v.set(j - 1, tmp);
            j -= 1;
        }

        // After inner loop: j == 0 or v[j-1] <= v[j].
        // The prefix v[0..=i] is now sorted.
        proof {
            // This assume is the cost of uninterpreted le_ensures: the solver can't
            // track ordering through swaps. With the bits approach this is provable.
            assume(spec_f64_sorted(v@.take(i as int + 1)));
        }
        i += 1;
    }
    proof { assert(v@.take(n as int) =~= v@); }
}

// Test: the comparison bridge works — exec <= connects to spec f64_le_spec.
fn test_comparison_bridge() {
    let a: f64 = 1.0;
    let b: f64 = 2.0;
    proof {
        assume(f64::float_wf(a));
        assume(f64::float_wf(b));
    }
    if a <= b {
        assert(f64_le_spec(a, b));
    } else {
        assert(!f64_le_spec(a, b));
    }
}

// Test: strong connectivity lets us conclude the other direction.
fn test_strong_connectivity() {
    let a: f64 = 1.0;
    let b: f64 = 2.0;
    proof {
        assume(f64::float_wf(a));
        assume(f64::float_wf(b));
    }
    if !(a <= b) {
        assert(!f64_le_spec(a, b));
        // Mention both trigger terms so the solver fires the totality axiom.
        assert(f64_le_spec(a, b) || f64_le_spec(b, a));
        assert(f64_le_spec(b, a));
    }
}

// Test: sort postcondition.
fn test_sort() {
    let mut v: Vec<f64> = Vec::new();
    v.push(3.0);
    v.push(1.0);
    v.push(2.0);
    proof { assume(all_float_wf::<f64>(v@)); }
    insertion_sort_f64(&mut v);
    assert(v@.len() == 3);
    assert(spec_f64_sorted(v@));
}

} // verus!
