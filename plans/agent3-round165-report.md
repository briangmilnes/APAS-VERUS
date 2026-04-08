# Agent 3 — Round 165 Report

## Goal

Extract duplicated BST proof lemmas from Chap37's BST files into
`src/Chap37/BSTSpecsAndLemmas.rs`, following the pattern of
`src/Chap42/TableSpecsAndLemmas.rs`.

## Findings: Actual Duplication

After reading all 19 Chap37 files, the confirmed identical lemmas across
multiple files were:

| # | Lemma | Files duplicated in |
|---|-------|---------------------|
| 1 | `lemma_bst_deep` | BSTAVLStEph.rs, BSTAVLMtEph.rs, BSTRBStEph.rs |
| 2 | `lemma_max_plus_one` | BSTAVLStEph.rs, BSTAVLMtEph.rs |

Note: The prompt estimated ~13 duplicated functions. After reading the code,
the count is 2 unique lemmas (duplicated 3× and 2× respectively). Other shared
code (`BSTSpecFns` trait + impls, `lemma_node_contains`, `lemma_bst_left`,
`lemma_bst_right`) was already centralized in `BSTPlainStEph.rs` and imported
by other files. Splay zig/zag lemmas exist in both BSTSplayStEph and
BSTSplayMtEph but use different types/spec fns and must remain standalone per
the chapter standalone rule.

## Changes Made

| # | Chap | File | Action |
|---|------|------|--------|
| 1 | 37 | BSTSpecsAndLemmas.rs | Created (93 lines): sections 1, 2, 7 |
| 2 | 37 | BSTAVLStEph.rs | Removed 2 lemmas (57 lines), added import |
| 3 | 37 | BSTAVLMtEph.rs | Removed 2 lemmas (62 lines), added import |
| 4 | 37 | BSTRBStEph.rs | Removed 1 lemma (49 lines), added import |
| 5 | — | lib.rs | Added `BSTSpecsAndLemmas` as first Chap37 entry |

## BSTSpecsAndLemmas.rs Contents

- `pub proof fn lemma_bst_deep<T: TotalOrder>(tree: BalBinTree<T>)` — decomposes
  `tree_is_bst` two levels deep, exposing ordering quantifiers for children and
  grandchildren. Used by AVL and RB rotation proofs.
- `pub proof fn lemma_max_plus_one(a: nat, b: nat)` — arithmetic lemma:
  `max(a+1, b) <= max(a, b) + 1`. Used by AVL height bookkeeping.

## Line Counts

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 37 | BSTSpecsAndLemmas.rs | 0 | 93 | +93 |
| 2 | 37 | BSTAVLStEph.rs | 1071 | ~1009 | −62 |
| 3 | 37 | BSTAVLMtEph.rs | 1077 | ~1010 | −67 |
| 4 | 37 | BSTRBStEph.rs | 717 | ~670 | −47 |
| — | — | Chap37 total | 20711 | 20628 | −83 |

Net: −83 lines across the chapter (176 lines removed from 3 files, 93 added in new file).

## Validation

```
verification results:: 5731 verified, 0 errors
RTT: 3776 tests run: 3776 passed, 0 skipped
PTT: 221 tests run: 221 passed, 0 skipped
```

All clean.
