# R118 Agent 3 — Strengthen Chap50 MatrixChain/OptBinSearchTree specs. AFK. DOT.

## Problem

`veracity-compare-par-mut` reports 44 warnings on Chap50. Two DP modules:
MatrixChain (30 warnings) and OptBinSearchTree (14 warnings). The dominant
pattern is Mt variants missing memo-related ensures from their St counterparts.

## Files

| # | Chap | File | Warnings |
|---|------|------|----------|
| 1 | 50 | MatrixChainStPer.rs | 1 (no wf) |
| 2 | 50 | MatrixChainStEph.rs | 1 (no wf) |
| 3 | 50 | MatrixChainMtPer.rs | 10 |
| 4 | 50 | MatrixChainMtEph.rs | 18 |
| 5 | 50 | OptBinSearchTreeMtPer.rs | 5 |
| 6 | 50 | OptBinSearchTreeMtEph.rs | 9 |

## Warnings by category

### Missing wf on St variants (2 warnings)

- `MatrixChainStPer.rs`: no spec_*_wf
- `MatrixChainStEph.rs`: no spec_*_wf

Check the struct. MatrixChain has `dimensions` and `memo` fields. A meaningful
wf might be `dimensions@.len() >= 2` (need at least 2 dimensions for a chain)
and `spec_memo_correct(dimensions, memo)` or similar.

### Wf naming (1 warning)

- `OptBinSearchTreeMtEph.rs`: wf name `spec_obstmteph_wf` doesn't follow
  pattern `spec_optbinsearchtree*_wf`. Rename to `spec_optbinsearchtreemteph_wf`.

### Missing memo ensures on constructors (MtPer + MtEph)

Both Mt variants are missing `memo =~= Map::empty()` ensures on constructors:

**MatrixChainMtPer** (3 warnings): `new`, `from_dimensions`, `from_dim_pairs`
**MatrixChainMtEph** (6 warnings): `new`, `from_dimensions`, `from_dim_pairs`,
  `set_dimension`, `update_dimension`, `clear_memo`

**OptBinSearchTreeMtPer** (3 warnings): `new`, `from_keys_probs`, `from_key_probs`
**OptBinSearchTreeMtEph** (5 warnings): `new`, `from_keys_probs`, `from_key_probs`,
  `set_key_prob`, `update_prob`, `clear_memo`

These are all the same pattern: add `ensures obj@.memo =~= Map::empty()`.

### Missing memo ensures on memo_size (4 warnings)

- `MatrixChainMtPer.memo_size`: StPer has ensures, MtPer doesn't
- `MatrixChainMtEph.memo_size`: StEph has ensures, MtEph doesn't
- `OptBinSearchTreeMtPer.memo_size`: StPer has ensures, MtPer doesn't
- `OptBinSearchTreeMtEph.memo_size`: StEph has ensures, MtEph doesn't

Check what the St ensures is — likely `count == self@.memo.dom().len()`.

### Weak matrix_chain_rec specs (MtPer + MtEph)

**MatrixChainMtPer** (5 warnings):
- Missing `spec_memo_correct(old(self)@.dimensions, old(self)@.memo)` requires
- Missing 4 ensures: dimensions preserved, memo contains (i,j), memo[(i,j)] == cost,
  spec_memo_correct post

**MatrixChainMtEph** (5 warnings): Same pattern.

These are the core DP recursion specs. The Mt variants need the memoization
correctness invariant carried through. This is the hardest part — check if
the RwLock pattern can express `spec_memo_correct` through the lock boundary.

### OptBinSearchTree keys ensures (2 warnings)

- `OptBinSearchTreeMtEph.keys`: missing `keys@ =~= self@.keys` ensures
  (both StEph and MtPer have it)

### from_dimensions dimensions ensures (1 warning)

- `MatrixChainMtEph.from_dimensions`: MtPer has `mc@.dimensions =~= dimensions@`
  but MtEph doesn't.

## Strategy

1. Read all 6 files (St variants for reference).
2. Rename `spec_obstmteph_wf` → `spec_optbinsearchtreemteph_wf` (1 warning).
3. Add St wf predicates if meaningful (2 warnings).
4. Add memo-empty ensures on constructors — most mechanical (9 warnings).
5. Add memo_size ensures (4 warnings).
6. Add keys ensures to OptBinSearchTree (2 warnings).
7. Strengthen matrix_chain_rec specs (10 warnings) — assess RwLock feasibility.
8. Validate: `scripts/validate.sh isolate Chap50`.
9. RTT: `scripts/rtt.sh Chap50`.

## Rules

- Do NOT weaken any ensures.
- Do NOT add assume or accept.
- Mt standalone: do NOT import from St counterparts.
- Renaming wf: update all references (trait, impl, requires, ensures, callers).
- No subagents.

## STEP 30

## Report

Write `plans/agent3-r118-chap50-dp-report.md`. Include before/after warning
count per file.
