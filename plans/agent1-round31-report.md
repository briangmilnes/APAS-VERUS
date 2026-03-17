# Agent 1 — Round 31 Report

## Summary

R31 targeted fn_missing_requires fixes in Chap03, Chap23, Chap37, Chap55, and Chap66.

**Result:** 2 fn_missing_requires errors fixed in Chap37/BSTSplayStEph.rs. 17 remaining
fn_missing_requires are genuinely precondition-free functions needing `// veracity: no_requires`
annotations (user only). Non-Chap37 files already had `requires true` (warnings, not errors) —
chapters already clean.

## Verification

- **4116 verified, 0 errors** (scripts/validate.sh)
- Holes: 189 total (unchanged — fn_missing_requires are not counted in hole metric)

## Changes Made

### Chap37/BSTSplayStEph.rs (5 → 3 fn_missing_requires)

| # | Chap | File | Function | Fix | Rationale |
|---|------|------|----------|-----|-----------|
| 1 | 37 | BSTSplayStEph.rs | `in_order_collect` | Added `requires spec_is_bst_link(link)` | BST traversal; callers have BST invariant; recursive children inherit from parent BST |
| 2 | 37 | BSTSplayStEph.rs | `pre_order_collect` | Added `requires spec_is_bst_link(link)` | Same pattern as in_order_collect |

### Unfixable fn_missing_requires (17 functions across 10 files)

These functions are genuinely precondition-free. Adding `requires true` is forbidden per
CLAUDE.md. Adding `// veracity: no_requires` is user-only. No real, non-vacuous precondition
exists.

| # | Chap | File | Function | Why No Precondition |
|---|------|------|----------|---------------------|
| 1 | 37 | BSTSplayStEph.rs | `new_node` | Constructor: takes key, creates leaf node |
| 2 | 37 | BSTSplayStEph.rs | `size_link` | Field reader: returns `node.size`. Adding BST req cascades to `update` → `splay` |
| 3 | 37 | BSTSplayStEph.rs | `update` | Field writer: recomputes size. 40+ callers in rotation code |
| 4 | 37 | BSTSetSplayMtEph.rs | `rebuild_from_vec` | Construction: builds from arbitrary Vec via insert loop |
| 5 | 37 | BSTSetSplayMtEph.rs | `from_sorted_iter` | Construction: builds from arbitrary iterator |
| 6 | 37 | BSTSetPlainMtEph.rs | `rebuild_from_vec` | Construction: same pattern |
| 7 | 37 | BSTSetPlainMtEph.rs | `from_sorted_iter` | Construction: same pattern |
| 8 | 37 | BSTSetBBAlphaMtEph.rs | `rebuild_from_vec` | Construction: same pattern |
| 9 | 37 | BSTSetBBAlphaMtEph.rs | `from_sorted_iter` | Construction: same pattern |
| 10 | 37 | BSTSetAVLMtEph.rs | `rebuild_from_vec` | Construction: same pattern |
| 11 | 37 | BSTSetAVLMtEph.rs | `from_sorted_iter` | Construction: same pattern |
| 12 | 37 | BSTSetRBMtEph.rs | `from_sorted_iter` | Construction: same pattern |
| 13 | 37 | AVLTreeSeqStEph.rs | `clone_link` | Deep clone: works on any link, wf conditional in ensures |
| 14 | 37 | AVLTreeSeqStEph.rs | `push_left_iter` | Iterator scaffolding: pushes nodes during init |
| 15 | 37 | BSTSplayMtEph.rs | `build_balanced` | Recursive construction: any slice → balanced tree |
| 16 | 37 | BSTRBMtEph.rs | `build_balanced` | Same pattern |
| 17 | 37 | AVLTreeSeqStPer.rs | `push_left_iter_stper` | Iterator scaffolding: persistent variant |

### Non-Chap37 Files (no changes needed)

These files already have `requires true` (warning, not error) and are already clean:

| # | Chap | File | Function | Status |
|---|------|------|----------|--------|
| 1 | 3 | InsertionSortStEph.rs | `insertion_sort` | requires_true warning; no real precondition (any slice) |
| 2 | 23 | BalBinTreeStEph.rs | `clone_tree` | requires_true warning; no real precondition (any tree) |
| 3 | 55 | SCCStEph.rs | `check_wf_adj_list_eph` | requires_true warning; validator must accept any input |
| 4 | 55 | SCCStPer.rs | `check_wf_adj_list_per` | requires_true warning; validator must accept any input |
| 5 | 66 | BoruvkaStEph.rs | `coin_flip` | requires_true warning; pure bit manipulation |

## Analysis

### Why fn_missing_requires Persists

Veracity flags all free exec functions without `requires` clauses. The 17 unfixable functions
fall into 4 categories:

1. **Constructors** (rebuild_from_vec, from_sorted_iter, new_node): Take arbitrary input,
   build valid structures via `new()` + `insert()` loops. No precondition on input values.

2. **Field accessors** (size_link, update): Read/write cached fields. Adding BST invariant
   cascades through 40+ callers in rotation code, requiring intermediate BST preservation
   proofs at every rotation step.

3. **Clone helpers** (clone_link): Work on any link; ensures are conditional on wf.
   Adding wf requires would cascade to `Clone::clone` (which can't have requires).

4. **Iterator scaffolding** (push_left_iter, push_left_iter_stper): Internal helpers
   for iterator initialization. No precondition on tree structure.

### Recommended Resolution

Add `// veracity: no_requires` annotations to these 17 functions. This is a user-only action
per CLAUDE.md rules.

## Techniques

- Added BST invariant (`spec_is_bst_link`) as requires for traversal functions where the
  recursive decomposition of the invariant provides child invariants to recursive calls.
- Analyzed cascading effects of adding requires to shared helpers (size_link → update → splay)
  and correctly avoided changes that would break existing proofs.
- Attempted `ensures true` removal (11 functions) — discovered this creates fn_missing_ensures
  errors, reverted all.
