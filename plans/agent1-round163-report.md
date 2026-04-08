# R163 Agent 1 Report: Compress BSTSplay proof functions

## Summary

Extracted two proof lemmas (`lemma_zig_child_ordering`, `lemma_zag_child_ordering`)
that capture the repeated BST ordering argument after splay rotations. Replaced 20
inline proof blocks across both files with lemma calls + trigger assertions.

## Lemmas created

| # | Chap | File | Lemma | Requires (summary) | Ensures (summary) | Lines |
|---|------|------|-------|--------------------|--------------------|-------|
| 1 | 37 | BSTSplayStEph.rs | `lemma_zig_child_ordering` | le(lo,hi), lo≠hi, subtree decomposition | ∀x∈child: le(lo,x) ∧ x≠lo | 18 |
| 2 | 37 | BSTSplayStEph.rs | `lemma_zag_child_ordering` | le(lo,hi), lo≠hi, subtree decomposition | ∀x∈child: le(x,hi) ∧ x≠hi | 18 |
| 3 | 37 | BSTSplayMtEph.rs | `lemma_zig_child_ordering` | le(lo,hi), lo≠hi, subtree decomposition | ∀x∈child: le(lo,x) ∧ x≠lo | 18 |
| 4 | 37 | BSTSplayMtEph.rs | `lemma_zag_child_ordering` | le(lo,hi), lo≠hi, subtree decomposition | ∀x∈child: le(x,hi) ∧ x≠hi | 18 |

The zig lemma handles right-rotation BST ordering (elements > pivot);
the zag lemma is the mirror for left-rotation (elements < pivot).
Each file gets its own copy because StEph uses `spec_contains_link(&Link<T>)`
while MtEph uses `link_contains(Link<T>)`.

## Replacement sites

Each lemma call replaces a 10-20 line `assert forall ... by { reveal; case-split; transitive; antisymmetric }` block.

| # | Chap | File | Case | Lemma(s) used | Lines removed | Lines added |
|---|------|------|------|---------------|---------------|-------------|
| 1 | 37 | BSTSplayStEph.rs | Zig at Equal | zig | 11 | 3 |
| 2 | 37 | BSTSplayStEph.rs | Zig-zig (chained) | zig×2 | 31 | 4 |
| 3 | 37 | BSTSplayStEph.rs | Zig-zig fallback | zig | 11 | 3 |
| 4 | 37 | BSTSplayStEph.rs | Zig-zag | zag+zig | 24 | 2 |
| 5 | 37 | BSTSplayStEph.rs | Zig-zag fallback | zig | 10 | 3 |
| 6 | 37 | BSTSplayStEph.rs | Zag at Equal | zag | 11 | 1 |
| 7 | 37 | BSTSplayStEph.rs | Zag-zag (chained) | zag×2 | 31 | 2 |
| 8 | 37 | BSTSplayStEph.rs | Zag-zag fallback | zag | 11 | 1 |
| 9 | 37 | BSTSplayStEph.rs | Zag-zig | zig+zag | 24 | 2 |
| 10 | 37 | BSTSplayStEph.rs | Zag-zig fallback | zag | 10 | 1 |
| 11-20 | 37 | BSTSplayMtEph.rs | (same 10 cases) | (same) | ~similar | ~similar |

## Function proof lines before/after

| # | Chap | File | Function | Before | After | Delta |
|---|------|------|----------|--------|-------|-------|
| 1 | 37 | BSTSplayStEph.rs | splay | 694 | 590 | −104 |
| 2 | 37 | BSTSplayMtEph.rs | splay | 718 | 624 | −94 |

## File line counts

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 37 | BSTSplayStEph.rs | 1803 | 1699 | −104 |
| 2 | 37 | BSTSplayMtEph.rs | 2258 | 2164 | −94 |

## Total proof lines removed: −198

## Technique

The key insight is that all splay rotation cases (zig, zig-zig, zig-zag, and their
mirrors) share the same BST ordering argument structure: after a rotation, elements
in the rotated subtree decompose into key `hi` plus two sub-trees, all of which are
`> lo` either directly or via transitivity through `hi`.

The chained approach (zig-zig, zag-zag) is especially effective: first prove
`left.right > left_key`, then use that fact as `sub_hi` to prove `ll.right > ll_key`,
replacing a 20-line deep case analysis with 2 one-line lemma calls.

A Z3 conjunction flakiness workaround was needed: the lemma requires `le(lo, hi) &&
lo != hi`, but Z3 can prove each conjunct individually while failing the conjunction.
Fix: assert `spec_contains_link(orig_root_left, left_key)` to trigger the forall that
gives `left_key != root_key` as a separate fact before the lemma call.

## Patterns found but not extracted

**Element preservation proofs** (~28 lines per rotation case, ~8 cases per file)
differ enough between cases (each reasons about different sub-tree compositions)
that a single lemma couldn't cover them without complex parameterization. These
remain inline.

**Double-rotation inner BST ordering** (e.g., ll.right > ll_key in zig-zig) was
captured via the chained lemma approach rather than a dedicated lemma, which was
more effective.

## Verification

- `scripts/validate.sh isolate Chap37`: 1962 verified, 0 errors
- `scripts/validate.sh` (full): 5747 verified, 0 errors
