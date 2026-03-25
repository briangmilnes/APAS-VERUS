# R74 Agent 3 — Prove Chap37 rwlock wf assumes + Chap41 type-axiom assumes (17 holes)

## Objective

Prove or eliminate 17 `assume` holes across 3 files:

### BSTSetPlainMtEph.rs — 6 rwlock:predicate assumes

All in `union`, `intersection`, `difference` — assuming wf on results returned from
ParaPair thread boundaries:
- Line ~271: `assume(left_union.spec_bstsetplainmteph_wf())`
- Line ~272: `assume(right_union.spec_bstsetplainmteph_wf())`
- Line ~303: `assume(left_inter.spec_bstsetplainmteph_wf())`
- Line ~304: `assume(right_inter.spec_bstsetplainmteph_wf())`
- Line ~338: `assume(left_diff.spec_bstsetplainmteph_wf())`
- Line ~339: `assume(right_diff.spec_bstsetplainmteph_wf())`

### BSTSetBBAlphaMtEph.rs — 6 rwlock:predicate assumes

Same pattern as BSTSetPlainMtEph but with `spec_bstsetbbalphamteph_wf()`.

### AVLTreeSetMtPer.rs — 5 assumes

- Line ~246: `assume(obeys_cmp_spec::<T>())` in `from_seq`
- Line ~247: `assume(view_ord_consistent::<T>())` in `from_seq`
- Line ~248: `assume(st@.len() + 1 < usize::MAX as nat)` in `from_seq`
- Line ~366: `assume(obeys_cmp_spec::<T>())` in `find`
- Line ~367: `assume(view_ord_consistent::<T>())` in `find`

## Strategy

### rwlock wf assumes (12 holes)

These assumes say "the result of union/intersection/difference is well-formed." The
underlying BSTSet*StEph functions should already ensure wf on their results. The Mt
wrapper needs to prove that wf propagates through the RwLock read → call St function →
return result chain.

Approach:
1. Read `src/Chap37/BSTSetPlainMtEph.rs` — find union/intersection/difference.
2. Read the corresponding StEph trait (`BSTSetPlainStEphTrait`) — check if union/intersection/
   difference have `ensures result.spec_bstsetplainsteph_wf()`.
3. If StEph ensures wf, the Mt wrapper can use that ensures to prove its own wf.
4. If StEph does NOT ensure wf, you'll need to add that ensures to StEph first (which
   should already verify since StEph is clean).
5. Reference: `src/Chap41/AVLTreeSetMtEph.rs` (0 holes) for the pattern.

### type-axiom assumes (5 holes)

Same pattern as Chap43 — generic type axioms. Check if `T: StTInMtT + Ord + TotalOrder`
implies `obeys_cmp_spec::<T>()` and `view_ord_consistent::<T>()` via broadcast lemmas.
The capacity bound `st@.len() + 1 < usize::MAX` may need a real requires clause.

## Assigned files

| # | File | Holes |
|---|------|-------|
| 1 | src/Chap37/BSTSetPlainMtEph.rs | 6 assume (rwlock wf) |
| 2 | src/Chap37/BSTSetBBAlphaMtEph.rs | 6 assume (rwlock wf) |
| 3 | src/Chap41/AVLTreeSetMtPer.rs | 5 assume (type-axiom + capacity) |

## Validation

```bash
scripts/validate.sh    # must pass: 4735+ verified, 0 errors
scripts/rtt.sh         # must pass: 2619+ tests
```

Fix all warnings in your assigned files before committing.

## Required reading (before writing any code)

1. `CLAUDE.md` — project rules.
2. `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs` — RwLock Mt wrapper pattern.
3. `src/standards/partial_eq_eq_clone_standard.rs` — eq/clone assume pattern (ONLY in eq/clone bodies).
4. `src/Chap37/BSTSetPlainStEph.rs` — the St counterpart for BSTSetPlainMtEph. Read this
   FIRST to understand what ensures the St functions provide. Your Mt wrapper must use
   those ensures to prove its own postconditions.
5. `src/Chap37/BSTSetBBAlphaStEph.rs` — the St counterpart for BSTSetBBAlphaMtEph.
6. `src/Chap41/AVLTreeSetStPer.rs` — the St counterpart for AVLTreeSetMtPer.
7. `src/Chap41/AVLTreeSetMtEph.rs` — working Mt example (0 holes) for reference.

## Rules

- Do NOT weaken ensures. Do NOT add `accept()`.
- Do NOT sequentialize parallel code.
- Commit to your branch, push to `origin/agent3/ready`.
- Write report to `plans/agent3-round74-report.md`.
