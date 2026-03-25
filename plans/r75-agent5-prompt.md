# R75 Agent 5 — Prove Chap37 BSTSetAVLMtEph + tree/table iterators (25 holes)

## Objective

Prove or eliminate 25 holes across 5 files — primarily the BSTSetAVLMtEph iterator
restructuring (17 holes) plus 4 tree/table iterator files (8 holes).

## Files and holes

| # | Chap | File | Holes | Root causes |
|---|------|------|-------|-------------|
| 1 | 37 | BSTSetAVLMtEph.rs | 17 | 4 root, 11 downstream + 2 external |
| 2 | 37 | AVLTreeSeqStEph.rs | 2 | 1 root, 1 downstream |
| 3 | 37 | AVLTreeSeqStPer.rs | 2 | 1 root, 1 downstream |
| 4 | 43 | OrderedTableMtEph.rs | 2 | 1 root, 1 downstream |
| 5 | 43 | AugOrderedTableMtEph.rs | 2 | 1 root, 1 downstream |

### BSTSetAVLMtEph.rs (17 holes — 15 external_body + 2 external)

Root causes (4 + 2 external):
- `rebuild_from_vec()` — line ~152 — external_body root cause
- `from_sorted_iter()` — line ~164 — external_body root cause
- `iter_in_order()` — line ~383 — external_body root cause
- `iter()` — line ~388 — external_body root cause
- `IntoIterator for &BSTSetAVLMtEph` — line ~471 — external root cause
- `IntoIterator for BSTSetAVLMtEph` — line ~480 — external root cause

Downstream (11 — all blocked by root causes above):
- `values_vec()` — blocked by iter
- `copy_set()` — blocked by from_sorted_iter
- `delete()` — blocked by iter
- `union()` — blocked by join_m
- `intersection()` — blocked by join_m
- `difference()` — blocked by join_m
- `split()` — blocked by from_sorted_iter
- `join_pair()` — blocked by from_sorted_iter
- `join_m()` — blocked by from_sorted_iter
- `filter()` — blocked by from_sorted_iter
- `reduce()` — blocked by iter

**Core problem**: BSTSetAVLMtEph wraps the AVL tree in a BTreeSet-like interface. Many
functions use std iterator adapters (`filter_map`, `fold`, `collect`, `BTreeSet::iter()`)
that Verus cannot compile inside `verus!`. The `external_body` markers exist because the
function bodies contain these unverifiable std iterator calls.

**Strategy**: Replace std iterator chains with explicit loops or verified iterator patterns.

1. **iter() / iter_in_order()**: These likely return `std::collections::btree_set::Iter`.
   Replace with the project's iterator standard — implement the 10-component iterator
   pattern from `src/standards/iterators_standard.rs`. The underlying AVL tree
   (BSTAVLMtEph) already has a verified iterator — wrap or delegate to it.

2. **from_sorted_iter()**: Likely uses `Iterator::collect()` or similar. Replace with an
   explicit loop that builds the set from sorted input.

3. **rebuild_from_vec()**: Likely iterates a Vec to rebuild. Replace with explicit loop.

4. **IntoIterator impls** (external): These are `#[verifier::external]` because they
   implement std traits. Check if they can be brought inside `verus!` by following the
   iterator standard pattern. If the underlying iterator is verified, the IntoIterator
   impls just need to return it.

5. **Downstream cascade**: Once the 4-6 root causes are fixed, the 11 downstream functions
   should be provable since they just compose the root-cause functions.

### AVLTreeSeqStEph.rs (2 holes — both external_body)

- `iter<'a>()` — line ~934 — external_body root cause (iterator body)
- `into_iter()` — line ~1193 — external_body downstream

**Strategy**: The AVL tree sequence iterator. Check if the iterator infrastructure (ghost
struct, invariants) is in place but just the body is unverified. The iterator standard
(`src/standards/iterators_standard.rs`) shows the pattern. Read `src/Chap18/ArraySeqStEph.rs`
as the reference implementation for collection iterators.

### AVLTreeSeqStPer.rs (2 holes — both external_body)

Same pattern as StEph. The StPer wraps StEph, so fixing StEph's iterator may fix StPer too.

### OrderedTableMtEph.rs (2 holes — both external_body)

- `iter<'a>()` — line ~671 — external_body root cause
- `into_iter()` — line ~807 — external_body downstream

**Strategy**: The Mt iterator wraps the St iterator through RwLock. Check how other Mt
iterator files handle this (e.g., `src/Chap18/ArraySeqMtEph.rs` for the pattern).

### AugOrderedTableMtEph.rs (2 holes — both external_body)

Same pattern as OrderedTableMtEph. Fix follows the same approach.

## Key resources

- `src/standards/iterators_standard.rs` — Iterator standard (10 components)
- `src/standards/wrapping_iterators_standard.rs` — Wrapping iterator pattern
- `src/Chap18/ArraySeqStEph.rs` — Reference iterator implementation
- `src/Chap18/ArraySeqMtEph.rs` — Reference Mt iterator
- `src/Chap37/BSTAVLMtEph.rs` — AVL tree (underlying type for BSTSetAVL)
- `src/Chap43/OrderedTableStEph.rs` — Sequential ordered table (now clean, 0 holes)
- `src/Chap43/AugOrderedTableStEph.rs` — Sequential aug ordered table

## Approach

1. Read the iterator standard and wrapping iterator standard first.
2. Read ArraySeqStEph.rs and ArraySeqMtEph.rs iterator sections as reference.
3. Start with AVLTreeSeqStEph.rs `iter()` — smallest file, clearest pattern.
4. Then OrderedTableMtEph.rs `iter()` — Mt wrapping pattern.
5. Then BSTSetAVLMtEph.rs — the big target. Fix root causes in order:
   a. `iter()` / `iter_in_order()` first
   b. `from_sorted_iter()` and `rebuild_from_vec()` next
   c. Downstream functions should cascade
6. `IntoIterator` external impls — bring inside verus! if possible.

## Important

This work pioneers the pattern for BSTSetRBMtEph (16 holes) and BSTSetSplayMtEph (13 holes)
which have the exact same root causes. Whatever approach works here will be replicated in
those files in R76. Document your approach clearly in your report.

## Validation

Run `scripts/validate.sh` after each file change. Run `scripts/rtt.sh` and `scripts/ptt.sh`
before committing. Push to `agent5/ready`.

## Report

Write `plans/agent5-round75-report.md` with holes before/after per file (table with Chap column).
Document the iterator restructuring approach in detail so it can be replicated.
