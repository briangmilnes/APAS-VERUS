# R80b Agent 2 — BSTSplayMtEph build_balanced sorted precondition (Chap37, 1 hole)

## Objective

Prove the 1 remaining assume in `build_balanced` in BSTSplayMtEph.rs by adding a sorted
precondition and proving BST ordering is preserved through recursive construction.

## Baseline

- 4908 verified, 0 errors, 0 warnings
- BSTSplayMtEph.rs: 1 hole (`assume(spec_is_bst_link(Some(node)))` in build_balanced)

## The hole

Agent 3 R79 narrowed `build_balanced` from `external_body` to 1 assume. The function
builds a balanced BST from a sorted slice by splitting at the midpoint and recursing.
The assume says the result is a valid BST — which is true because sorted input + midpoint
split guarantees left < pivot < right.

## Strategy

1. **Read `build_balanced` in `src/Chap37/BSTSplayMtEph.rs`** — understand the current
   body and where the assume is.

2. **Add a sorted precondition**:
   ```rust
   requires
       forall|i: int, j: int| 0 <= i < j < values@.len() ==>
           TotalOrder::le(#[trigger] values@[i], #[trigger] values@[j])
   ```

3. **Prove BST ordering from sorted + split**:
   - Left slice `values[..mid]`: all elements ≤ pivot (from sorted)
   - Right slice `values[mid+1..]`: all elements ≥ pivot (from sorted)
   - Recursive calls produce valid BSTs (by induction — decreases values.len())
   - Combined node is BST-valid

4. **Check callers** — who calls `build_balanced`? They must now prove sorted. Since
   `build_balanced` is called on results from `in_order()` (which returns sorted data)
   or on sorted slices, the precondition should be satisfiable at all call sites.
   If a caller can't prove sorted, add the sorted ensures to the function that
   produces the input.

5. **Remove the assume** once the proof goes through.

## STEP 15

At most 15 edit/verify iterations. Then stop and report.

## Important

- Do NOT comment out or delete any existing proof lemmas or code.
- Do NOT add new assumes or accepts.
- If the sorted proof is too hard, leave the assume and report what you tried.

## Validation

Run `scripts/validate.sh`, then `scripts/rtt.sh`, then `scripts/ptt.sh`. Push to `agent2/ready`.

## Report

Write `plans/agent2-round80b-report.md` with holes before/after (table with Chap column).
