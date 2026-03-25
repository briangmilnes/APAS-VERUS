# Agent 4 — Round 75 Report

## Summary

Proved 6 holes across 2 Chap37 files: 5 external_body removed from BSTRBMtEph (rotate_left,
rotate_right, flip_colors, fix_up, insert_link) and 1 external_body removed from
BSTSplayMtEph (splay). Added explicit `#[trigger]` annotations to 32 quantifiers across
3 Chap37 files to eliminate all auto-trigger notes.

## Holes Before/After

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 37 | BSTRBMtEph.rs | 8 | 3 | -5 |
| 2 | 37 | BSTSplayMtEph.rs | 6 | 5 | -1 |
| **Total** | | | **14** | **8** | **-6** |

## Techniques Used

**Take/modify/put-back pattern**: Worked around Verus's lack of `Option::as_mut()` support.
`link.take()` extracts owned `Node<T>`, modify fields, then `*link = Some(node)` puts it
back. Used in rotate_left, rotate_right, flip_colors, fix_up, insert_link.

**Redundant update elimination**: rotate_left and rotate_right had `as_mut()` calls solely
for redundant `update()` calls after color-only changes. Since color doesn't affect size,
these were unnecessary. Removing them eliminated the `as_mut()` dependency.

**Ghost captures for color-only changes**: In flip_colors, Z3 couldn't connect modified
children (color flipped) back to original BST properties. Solution: capture ghost originals
(`orig_left`, `orig_right`, `node_key`) before modification, increase `reveal_with_fuel`
to 3, and add explicit `link_contains` equivalence assertions bridging modified ↔ original.

**Ghost intermediates for chained transforms**: In fix_up, three conditional transforms
(rotate_left → rotate_right → flip_colors) each produce an intermediate link state. Ghost
snapshots (`after_rl`, `after_rr`, `after_fl`) chain containment equivalences to prove the
overall postcondition.

**Moved variables in proof blocks**: Verus allows referencing exec variables after move in
`proof { }` blocks (proof mode operates on mathematical values, no ownership). This enabled
the splay proof (~900 lines) to verify by removing `#[verifier::external_body]`.

**Explicit trigger annotations**: Added `#[trigger]` to 32 quantifiers across BSTRBMtEph.rs
(10), BSTSplayMtEph.rs (20), and BSTAVLMtEph.rs (8) to eliminate all auto-trigger notes.

## What Was Proved

**BSTRBMtEph.rs** (-5 holes):
- `rotate_left`: Full proof with take/put-back + BST ordering assertions
- `rotate_right`: Symmetric to rotate_left
- `flip_colors`: Color-only modification with ghost captures proving BST invariant preserved
- `fix_up`: Three conditional transforms with ghost intermediate chaining
- `insert_link`: Removed external_body; existing proof code verified as-is once dependencies
  (rotate/flip/fix_up) were proved

**BSTSplayMtEph.rs** (-1 hole):
- `splay`: ~900 lines of proof code verified on first attempt after removing external_body

## Remaining Holes

**BSTRBMtEph.rs** (3 remaining):
- `filter_parallel` (external_body): Thread-spawn boundary for parallel filter
- `reduce_parallel` (external_body): Thread-spawn boundary for parallel reduce
- `height` assume: `link_height(*data) < usize::MAX` — needs Red-Black balance lemma (height ≤ 2·log₂(n+1))

**BSTSplayMtEph.rs** (5 remaining):
- `clone` (external_body): Recursive `Node<T>` Clone triggers Verus cycle error. Cannot remove external_body.
- `build_balanced` (external_body): Blocked by clone (uses `values[mid].clone()`)
- `filter_parallel` (external_body): Thread-spawn boundary + blocked by clone
- `reduce_parallel` (external_body): Thread-spawn boundary + blocked by clone
- `height` assume: Same as BSTRBMtEph — needs balance lemma

## Warnings

- 3 `fn_missing_requires` in BSTRBMtEph.rs: `is_red`, `size_link`, `update` genuinely
  have no precondition. Left for user to annotate `// veracity: no_requires`.
- 2 `fn_missing_requires` in BSTSplayMtEph.rs: `size_link`, `update` — same situation.

## Verification

- 4748 verified, 0 errors
- 2619 RTT passed
- 157 PTT passed
- 0 trigger notes in Chap37 files
