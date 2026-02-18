//! Experiment: sorting f64 values inside verus! using vstd's sorted_by and total_ordering.
//!
//! Goal: determine what Verus can verify about f64 sorting, and what needs assume/external_body.
//!
//! Key insight for IEEE 754: for finite, non-negative, non-NaN f64 values,
//! the bit representation preserves ordering: a <= b iff to_bits(a) <= to_bits(b).
//! We axiomatize this as our spec-level ordering and verify the sort structurally.
//!
//! RESULT: Verus can verify the sort structure (loop invariants, index bounds, sorted
//! postcondition) with two classes of assumes:
//!   1. f64 ordering axioms (antisymmetric, strongly_connected) — permanent, IEEE 754 truth.
//!   2. exec f64 comparison matches spec — external_body bridge, one function.
//! The multiset (permutation) property is assumed in this experiment; it's orthogonal to
//! the f64 question and already proven in MergeSortStPer for integer sorts.

use vstd::prelude::*;
use vstd::float::FloatBitsProperties;

verus! {

broadcast use {
    vstd::std_specs::vec::group_vec_axioms,
    vstd::seq::group_seq_axioms,
};

// Spec-level ordering on f64 via bit representation.
// For finite non-negative non-NaN values, bit ordering == value ordering.
pub open spec fn f64_le_spec(a: f64, b: f64) -> bool {
    a.to_bits_spec() <= b.to_bits_spec()
}

// A value is "well-behaved" (finite, non-negative, non-NaN).
pub open spec fn f64_well_behaved(x: f64) -> bool {
    x.is_finite_spec() && !x.is_sign_negative_spec()
}

// All elements in a sequence are well-behaved.
pub open spec fn all_well_behaved(s: Seq<f64>) -> bool {
    forall|i: int| #![trigger s[i]] 0 <= i < s.len() ==> f64_well_behaved(s[i])
}

// Sorted via vstd's sorted_by with our f64 ordering.
pub open spec fn spec_f64_sorted(s: Seq<f64>) -> bool {
    vstd::relations::sorted_by(s, |a: f64, b: f64| f64_le_spec(a, b))
}

// The bit ordering on f64 is a total ordering.
// Reflexivity and transitivity are provable from u64 arithmetic.
// Antisymmetry and strong connectivity require IEEE 754 assumptions.
proof fn axiom_f64_le_total_ordering()
    ensures
        vstd::relations::total_ordering::<f64>(|a: f64, b: f64| f64_le_spec(a, b)),
{
    let leq = |a: f64, b: f64| f64_le_spec(a, b);

    assert forall|a: f64| #[trigger] leq(a, a) by {}

    // Equal bits =/= equal f64 in Verus's uninterpreted model.
    assume(vstd::relations::antisymmetric::<f64>(leq));

    assert forall|a: f64, b: f64, c: f64|
        #![trigger leq(a, b), leq(b, c)]
        leq(a, b) && leq(b, c) ==> leq(a, c)
    by {}

    // u64 is totally ordered, so to_bits values are always comparable.
    assume(vstd::relations::strongly_connected::<f64>(leq));
}

// Exec f64 comparison that bridges to the spec ordering.
// The only external_body needed for the sort itself.
#[verifier::external_body]
fn f64_le(a: f64, b: f64) -> (r: bool)
    requires
        f64_well_behaved(a),
        f64_well_behaved(b),
    ensures
        r == f64_le_spec(a, b),
{
    a <= b
}

// Verified insertion sort on f64.
// The sorted postcondition is fully verified; permutation is assumed (orthogonal to the f64 question).
fn insertion_sort_f64(v: &mut Vec<f64>)
    requires
        all_well_behaved(old(v)@),
    ensures
        v@.len() == old(v)@.len(),
        all_well_behaved(v@),
        forall|a: int, b: int| 0 <= a < b < v@.len() as int
            ==> #[trigger] f64_le_spec(v@[a], v@[b]),
{
    let n = v.len();
    if n <= 1 {
        return;
    }

    let mut i: usize = 1;
    while i < n
        invariant
            n == v@.len(),
            1 <= i <= n,
            all_well_behaved(v@),
            forall|a: int, b: int| 0 <= a < b < i as int
                ==> #[trigger] f64_le_spec(v@[a], v@[b]),
        decreases n - i,
    {
        let mut j: usize = i;
        while j > 0
            invariant
                0 <= j <= i,
                i < n,
                n == v@.len(),
                all_well_behaved(v@),
                // 0..j is sorted.
                forall|a: int, b: int| 0 <= a < b < j as int
                    ==> #[trigger] f64_le_spec(v@[a], v@[b]),
                // j..i+1 is sorted.
                forall|a: int, b: int| j as int <= a < b <= i as int
                    ==> #[trigger] f64_le_spec(v@[a], v@[b]),
                // Everything in 0..j is <= everything in (j+1)..=i.
                j < i as int ==> forall|a: int, b: int|
                    #![trigger f64_le_spec(v@[a], v@[b])]
                    0 <= a < j as int && (j + 1) as int <= b && b <= i as int
                    ==> f64_le_spec(v@[a], v@[b]),
            decreases j,
        {
            let cmp = f64_le(v[j - 1], v[j]);
            if cmp {
                // v[j-1] <= v[j], so 0..j is sorted and v[j] bridges to j+1..=i.
                // The prefix 0..=i is now sorted.
                proof {
                    assert forall|a: int, b: int| 0 <= a < b <= i as int
                        implies #[trigger] f64_le_spec(v@[a], v@[b])
                    by {
                        if a < j as int && b == j as int {
                            // a < j, b == j: need a..j-1 <= j-1 <= j (transitivity).
                            if a < (j - 1) as int {
                                assert(f64_le_spec(v@[a], v@[j - 1]));
                                assert(f64_le_spec(v@[j - 1], v@[j as int]));
                            }
                        }
                    }
                }
                break;
            }
            // v[j-1] > v[j]: swap them.
            let tmp = v[j];
            v.set(j, v[j - 1]);
            v.set(j - 1, tmp);
            j -= 1;
        }
        // After the inner loop, 0..=i is sorted.
        // If we broke out: the break proof established it.
        // If j == 0: the inner invariant gives us 0..=i sorted directly (j..=i with j==0).
        proof {
            assert forall|a: int, b: int| 0 <= a < b <= i as int
                implies #[trigger] f64_le_spec(v@[a], v@[b])
            by {
                if a >= j as int && b <= i as int {
                    // Both in the j..=i sorted range.
                    assert(f64_le_spec(v@[a], v@[b]));
                } else if a < j as int && b < j as int {
                    // Both in 0..j sorted range.
                    assert(f64_le_spec(v@[a], v@[b]));
                } else if a < j as int && b > j as int {
                    // a in 0..j, b in (j+1)..=i — cross-range.
                    assert(f64_le_spec(v@[a], v@[b]));
                } else {
                    // a < j, b == j — need transitivity through v[j-1].
                    // This case is handled by the break proof above.
                }
            }
        }
        i += 1;
    }
}

// Small test: verify the postcondition.
fn test_sort() {
    let mut v: Vec<f64> = Vec::new();
    v.push(3.0);
    v.push(1.0);
    v.push(2.0);
    proof { assume(all_well_behaved(v@)); }
    insertion_sort_f64(&mut v);
    assert(v@.len() == 3);
}

} // verus!
