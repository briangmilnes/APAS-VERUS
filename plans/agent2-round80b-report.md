# Agent 2 Round 80b Report

## Objective
Prove the 1 remaining assume in `build_balanced` in BSTSplayMtEph.rs (Chap37).

## Result: 1 hole closed

### Holes Before/After

| # | Chap | File | Holes Before | Holes After | Delta |
|---|------|------|-------------|-------------|-------|
| 1 | 37 | BSTSplayMtEph.rs | 1 | 0 | -1 |

### Technique

The `build_balanced` function constructs a balanced BST from a sorted slice by
splitting at the midpoint and recursing. The assume `assume(spec_is_bst_link(Some(node)))`
claimed the result was a valid BST but didn't prove it.

**Proof strategy:**
1. Added a strictly-sorted precondition to `build_balanced` and `from_sorted_slice`
   (trait + impl): `forall|i, j| i < j ==> le(values[i], values[j]) && values[i] != values[j]`
2. Added a containment postcondition: `link_contains(link, x) ==> exists|i| values[i] == x`
3. Proved sub-slice sorted preservation for recursive calls
4. Proved BST ordering from sorted + containment:
   - Left elements (from `values[0..mid]`) are strictly less than pivot `values[mid]`
   - Right elements (from `values[mid+1..]`) are strictly greater than pivot
5. Used the feq broadcast pattern (`obeys_feq_clone::<T>()`) to establish that
   `clone(values[mid])` equals `values@[mid]`, connecting the exec key to the spec value
6. Proved containment ensures by mapping subtree elements back to slice positions

**Key challenge:** Verus's generic `Clone::clone` has no postcondition. The `cloned(a, b)`
trigger term + `axiom_cloned_implies_eq_owned` broadcast axiom + `obeys_feq_clone::<T>()`
precondition resolved this without any assumes in `build_balanced`.

**Imports added:** `obeys_feq_clone` from `vstdplus::feq`, broadcast use `group_feq_axioms`.

### Verification

- 4908 verified, 0 errors, 0 warnings
- 3076 RTT passed
- 157 PTT passed
- 45 clean chapters, 1 holed, 7 holes global
