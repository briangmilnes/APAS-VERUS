<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Agent 3 Round 58 Report

## Summary

Fixed 3 of 8 `fn_missing_requires`/`fn_missing_ensures` warnings. The remaining 5 have
no real precondition and cannot be fixed without adding tautological `requires true`
(forbidden by CLAUDE.md).

## Holes Before / After

| # | Chap | File | Before | After | Delta |
|---|:----:|---|:------:|:-----:|:-----:|
| 1 | 39 | BSTParaTreapMtEph.rs | 4 | 2 | -2 |
| 2 | 39 | BSTTreapMtEph.rs | 1 | 0 | -1 |
| 3 | 57 | DijkstraStEphU64.rs | 1 | 1 | 0 |
| 4 | 59 | JohnsonStEphI64.rs | 2 | 2 | 0 |

**Total fn_missing closed: 3**

## Changes Made

| # | Chap | File | Function | Change |
|---|:----:|---|---|---|
| 1 | 39 | BSTTreapMtEph.rs | `size_link` | Added `requires Lnk::spec_link_size_wf(link)` |
| 2 | 39 | BSTTreapMtEph.rs | `update` | Added `spec_link_size_wf` for children (cascaded) |
| 3 | 39 | BSTParaTreapMtEph.rs | `reduce_inner` | Named return; added `ensures tree@.len() == 0 ==> result == identity` |
| 4 | 39 | BSTParaTreapMtEph.rs | `reduce_parallel` | Named return; added `ensures tree@.len() == 0 ==> result == base` |

### Rationale for each fix

**`size_link`**: The ensures `sz as nat == Lnk::spec_size_link(link)` requires the
cached size field to match the recursive spec. This holds only when
`spec_link_size_wf(link)` is satisfied — i.e., the node's `size` field correctly
stores `1 + left_size + right_size`. Adding this as a precondition matches the same
pattern used by `rotate_left`, `rotate_right`, `insert_link`, `delete_link`, and
`height_link` in the same file.

**`update` (cascaded)**: `update` calls `size_link(&node.left)` and
`size_link(&node.right)`. With the new precondition on `size_link`, `update` must
guarantee `spec_link_size_wf` for both children. All callers (in `rotate_*`,
`insert_link`, `delete_link`) already establish this via `assert(Lnk::spec_link_size_wf(...))`.

**`reduce_inner` / `reduce_parallel`**: These recursive helpers had `requires` but no
`ensures`. The only non-tautological postcondition provable without introducing a
`Ghost(spec_op)` parameter is: when the tree is empty, the reduce returns the identity/base
element directly (leaf case returns `identity` unchanged). Verified from the `Exposed::Leaf`
branch returning the identity parameter.

## Warnings Not Fixed (no real precondition)

| # | Chap | File | Function | Reason |
|---|:----:|---|---|---|
| 1 | 39 | BSTParaTreapMtEph.rs | `param_treap_assert_finite` | Type invariant export; no user-visible precondition |
| 2 | 39 | BSTParaTreapMtEph.rs | `tree_priority_internal` | Reads root priority from any valid treap; no constraint |
| 3 | 57 | DijkstraStEphU64.rs | `pq_entry_new` | Simple struct constructor; any i64/usize valid |
| 4 | 59 | JohnsonStEphI64.rs | `adjust_distance` | Pure i128 arithmetic; no overflow possible |
| 5 | 59 | JohnsonStEphI64.rs | `reweight_edge` | Pure i128 arithmetic with clamping; no overflow possible |

These 5 require `// veracity: no_requires` annotations (user-only per CLAUDE.md).

## Verification Results

Validation shows `4484 verified, 1 errors`. The 1 error is pre-existing (confirmed by
running on `HEAD` before applying any R58 changes): `src/Chap43/OrderedSetStPer.rs:910`
— `invariant not satisfied before loop` on `size as nat == self@.len()`. This is in a
DO NOT TOUCH file and was already present before this round.

## Techniques Used

- Pattern matching against similar functions in the same file to identify the correct
  `spec_link_size_wf` precondition for `size_link`.
- Empty-tree identity lemma for reduce: `Exposed::Leaf` branch returns identity unchanged,
  and `expose_internal` ensures `tree@.len() == 0 ==> exposed is Leaf`.
- Cascaded `update` requires to satisfy the propagated `size_link` precondition.
