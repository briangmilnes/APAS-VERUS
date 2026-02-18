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
//! The multiset (permutation) property is not proven here; it's orthogonal to the
//! f64 question and already proven in MergeSortStPer for integer sorts.

use vstd::prelude::*;
use vstd::float::FloatBitsProperties;

verus! {

broadcast use {
    vstd::std_specs::vec::group_vec_axioms,
    vstd::seq::group_seq_axioms,
};

// Spec-level ordering on f64 via bit representation.
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

// Sorted: pairwise ordered.
pub open spec fn spec_f64_sorted(s: Seq<f64>) -> bool {
    forall|a: int, b: int| 0 <= a < b < s.len() ==> #[trigger] f64_le_spec(s[a], s[b])
}

// Sorted via vstd's sorted_by with our f64 ordering.
pub open spec fn spec_f64_sorted_vstd(s: Seq<f64>) -> bool {
    vstd::relations::sorted_by(s, |a: f64, b: f64| f64_le_spec(a, b))
}

// The bit ordering is a total preorder on all f64 (reflexive, transitive, strongly
// connected — all provable from u64 arithmetic). Antisymmetry requires assuming that
// bit-identical f64 values are equal, which Verus can't prove since f64 is uninterpreted.
//
// NaN is not a problem for the *bit* ordering: every NaN has a definite u64 bit pattern,
// so to_bits comparisons are well-defined. NaN IS a problem for the *exec* comparison
// (IEEE 754 says NaN <= x is always false), which is why f64_le requires all_well_behaved.
//
// Note: this axiom is informational — the sort doesn't use total_ordering. It verifies
// spec_f64_sorted (pairwise f64_le_spec) directly via loop invariants.
proof fn axiom_f64_le_total_ordering()
    ensures
        vstd::relations::total_ordering::<f64>(|a: f64, b: f64| f64_le_spec(a, b)),
{
    let leq = |a: f64, b: f64| f64_le_spec(a, b);
    assert forall|a: f64| #[trigger] leq(a, a) by {}
    assert forall|a: f64, b: f64, c: f64|
        #![trigger leq(a, b), leq(b, c)]
        leq(a, b) && leq(b, c) ==> leq(a, c)
    by {}
    // Strongly connected: for all u64 x, y, x <= y || y <= x. True but Verus needs help.
    assume(vstd::relations::strongly_connected::<f64>(leq));
    // Antisymmetric: to_bits(a) == to_bits(b) implies a == b. True for IEEE 754 but
    // unprovable since Verus treats f64 as uninterpreted.
    assume(vstd::relations::antisymmetric::<f64>(leq));
}

// Transitivity lemma for direct use in proofs.
proof fn lemma_f64_le_transitive(a: f64, b: f64, c: f64)
    requires
        f64_le_spec(a, b),
        f64_le_spec(b, c),
    ensures
        f64_le_spec(a, c),
{}

// Exec f64 comparison that bridges to the spec ordering.
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
fn insertion_sort_f64(v: &mut Vec<f64>)
    requires
        all_well_behaved(old(v)@),
    ensures
        v@.len() == old(v)@.len(),
        all_well_behaved(v@),
        spec_f64_sorted(v@),
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
            spec_f64_sorted(v@.take(i as int)),
        decreases n - i,
    {
        // Insert v[i] into the sorted prefix v[0..i].
        proof {
            assert forall|a: int, b: int| 0 <= a < b < i as int
                implies #[trigger] f64_le_spec(v@[a], v@[b])
            by {
                assert(v@.take(i as int)[a] == v@[a]);
                assert(v@.take(i as int)[b] == v@[b]);
                assert(f64_le_spec(v@.take(i as int)[a], v@.take(i as int)[b]));
            }
        }
        let mut j: usize = i;
        while j > 0 && !f64_le(v[j - 1], v[j])
            invariant
                0 <= j <= i,
                i < n,
                n == v@.len(),
                all_well_behaved(v@),
                // v[0..j] is sorted.
                forall|a: int, b: int| 0 <= a < b < j as int
                    ==> #[trigger] f64_le_spec(v@[a], v@[b]),
                // v[j..=i] is sorted.
                forall|a: int, b: int| j as int <= a < b <= i as int
                    ==> #[trigger] f64_le_spec(v@[a], v@[b]),
                // Everything in 0..j is <= everything in (j+1)..=i.
                forall|a: int, b: int|
                    #![trigger f64_le_spec(v@[a], v@[b])]
                    0 <= a < j as int && (j + 1) as int <= b && b <= i as int
                    ==> f64_le_spec(v@[a], v@[b]),
            decreases j,
        {
            let tmp = v[j];
            v.set(j, v[j - 1]);
            v.set(j - 1, tmp);
            j -= 1;
        }

        // After inner loop: either j == 0 or v[j-1] <= v[j].
        // In both cases, v[0..=i] is sorted.
        proof {
            // When j > 0, we know f64_le(v[j-1], v[j]) was true.
            // When j == 0, v[0..=i] = v[j..=i] which is sorted by invariant.
            assert forall|a: int, b: int| 0 <= a < b <= i as int
                implies #[trigger] f64_le_spec(v@[a], v@[b])
            by {
                if a < j as int && b == j as int {
                    // a is in the sorted prefix, b == j.
                    // We know v[j-1] <= v[j] (loop exit), and v[a] <= v[j-1] (prefix sorted).
                    if j > 0 {
                        assert(f64_le_spec(v@[a], v@[j as int - 1]));
                        assert(f64_le_spec(v@[j as int - 1], v@[j as int]));
                        lemma_f64_le_transitive(v@[a], v@[j as int - 1], v@[j as int]);
                    }
                }
            }
            // Connect to the take-based sorted spec.
            assert(v@.take((i + 1) as int) =~= v@.take(i as int + 1));
            assert forall|a: int, b: int|
                0 <= a < b < v@.take(i as int + 1).len()
                implies #[trigger] f64_le_spec(v@.take(i as int + 1)[a], v@.take(i as int + 1)[b])
            by {
                assert(v@.take(i as int + 1)[a] == v@[a]);
                assert(v@.take(i as int + 1)[b] == v@[b]);
            }
        }
        i += 1;
    }
    proof {
        assert(v@.take(n as int) =~= v@);
    }
}

// Small test: verify the postcondition holds.
fn test_sort() {
    let mut v: Vec<f64> = Vec::new();
    v.push(3.0);
    v.push(1.0);
    v.push(2.0);
    proof { assume(all_well_behaved(v@)); }
    insertion_sort_f64(&mut v);
    assert(v@.len() == 3);
    assert(spec_f64_sorted(v@));
}

} // verus!
