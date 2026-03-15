# Agent 3 — Round 17 Report: Spec Audit Chap37 + Chap38 + Chap39

## Mission

Audit every trait function's `requires`/`ensures` against APAS textbook prose in
Chap37, Chap38, and Chap39. Strengthen weak/missing specs. Add `#[verifier::external_body]`
where the corrected spec breaks an existing proof body.

## Holes Before/After Per File

| # | Chap | File | Before | After | Delta | Notes |
|---|------|------|--------|-------|-------|-------|
| 1 | 37 | BSTSplayStEph.rs | 1 | 7 | +6 | 6 new external_body with strong specs |
| 2 | 37 | AVLTreeSeqStEph.rs | 0 | 1 | +1 | set gets external_body |
| 3 | 37 | AVLTreeSeqStPer.rs | 0 | 4 | +4 | set/subseq/values/to_arrayseq |
| 4 | 38 | BSTParaStEph.rs | 5 | 6 | +1 | reduce gets external_body |
| 5 | 39 | BSTTreapStEph.rs | 0 | 2 | +2 | insert_link/find get external_body |
| 6 | 37 | Chap37 total | 4 | 15 | +11 | |
| 7 | 38 | Chap38 total | 14 | 15 | +1 | |
| 8 | 39 | Chap39 total | 16 | 18 | +2 | |
| 9 | — | **Project total** | **103** | **117** | **+14** | All from spec strengthening |

Hole count increased because previously-weak specs that verified (not counted as holes)
are now strong specs with `external_body` (counted as holes). Each new hole is a better
state: correct spec, clear proof target, stronger caller guarantees.

## Verification Counts

- Verified: 4137 (was 4150; -13 from external_body conversions)
- RTT: 2600 passed
- PTT: 147 passed
- Errors: 0
- Warnings: 0

## Spec Changes Summary

### BSTSplayStEph.rs (Chap37) — 7 functions fixed

| # | Function | Change |
|---|----------|--------|
| 1 | insert | Added `contains(value)`, preserves contains |
| 2 | find | Bidirectional `<==>` containment |
| 3 | contains | `found == contains(target)` — verifies from find |
| 4 | minimum | Added `forall\|x\| contains(x) ==> le(min, x)` |
| 5 | maximum | Added `forall\|x\| contains(x) ==> le(x, max)` |
| 6 | in_order | Length matches spec_in_order |
| 7 | pre_order | Length matches spec_pre_order |

Added spec fns: `spec_in_order_link`, `spec_pre_order_link`, `spec_in_order`, `spec_pre_order`.

### AVLTreeSeqStEph.rs (Chap37) — 4 functions fixed

| # | Function | Change |
|---|----------|--------|
| 1 | set | `wf, seq =~= old(seq).update(i, item@)` |
| 2 | update | Same as set — verifies from set |
| 3 | singleton | `seq[0] == item@` — verifies from insert_at_link |
| 4 | insert_value | Added `wf` — verifies from push_back |

### AVLTreeSeqStPer.rs (Chap37) — 4 functions fixed

| # | Function | Change |
|---|----------|--------|
| 1 | set | `wf, seq =~= seq.update(i, item@)` |
| 2 | subseq_copy | Added `wf` |
| 3 | values_in_order | `map_values =~= spec_seq()` |
| 4 | to_arrayseq | `len == spec_seq().len()` |

### BSTParaStEph.rs (Chap38) — 1 function fixed

| # | Function | Change |
|---|----------|--------|
| 1 | reduce | `empty ==> result@ == base@` |

filter and in_order left unchanged: closure limitation and set-to-seq gap respectively.

### BSTTreapStEph.rs (Chap39) — 4 functions fixed

| # | Function | Change |
|---|----------|--------|
| 1 | insert | Added `contains(value)` |
| 2 | insert_link | Added `spec_contains_link(&inserted, value)` |
| 3 | find | Bidirectional `<==>` containment + requires bst |
| 4 | contains | `found == contains(target)` — verifies from find |

## Techniques Used

- **Bidirectional containment**: `found.is_some() <==> spec_contains(target)` pattern
  used in both BSTSplay and BSTTreap. The `contains` method derives from `find` without
  external_body in both cases.
- **BST ordering requires**: Find completeness needs BST invariant; added `self.spec_bst()`
  to requires in BSTTreap.
- **Recursive spec functions**: Added `spec_in_order_link` and `spec_pre_order_link` for
  BSTSplay traversal specs.
- **External_body for fuel limits**: Treap insert_link can't prove containment through
  rotations due to recursive fuel limits.

## Remaining Holes (What Blocks Them)

- **BSTSplayStEph**: 6 external_body on algorithmic trait functions. Splay rotations are
  complex; proving containment preservation and ordering through double rotations requires
  careful structural induction.
- **AVLTreeSeqStEph**: 1 external_body (set). Proving update-at-index through AVL rotation
  rebalancing requires relating tree index positions to sequence indices.
- **AVLTreeSeqStPer**: 4 external_body. Arc path-copying complexity blocks proofs.
- **BSTParaStEph**: 1 new + 5 existing. Existing holes are assumes for size overflow bounds.
- **BSTTreapStEph**: 2 external_body (insert_link, find). Treap rotation fuel limits and
  BST ordering proofs through random priority rebalancing.

## Commit

`a3f4888b` — pushed to `agent3/ready`.
