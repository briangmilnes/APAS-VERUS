<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Agent2 Round 54 Report

## Validation Result

```
verification results:: 4477 verified, 0 errors
Elapsed: 131s
```

Zero errors. Zero trigger warnings. Clean.

## Holes Before / After

| # | Chap | File | Holes Before | Holes After | Delta |
|---|:----:|---|:---:|:---:|:---:|
| 1 | 41 | AVLTreeSetStEph.rs | 1 | 1 | — |
| 2 | 43 | AugOrderedTableMtEph.rs | 1 | 1 | — |
| 3 | 43 | AugOrderedTableStPer.rs | 1 | 1 | — |
| 4 | 43 | OrderedSetStEph.rs | 1 | 1 | — |
| 5 | 43 | OrderedSetStPer.rs | 1 | 1 | — |
| 6 | 43 | OrderedTableMtPer.rs | 1 | 1 | — |
| 7 | 43 | OrderedTableStPer.rs | ❌ baseline | ⚠ clean | **fixed** |

Actionable holes: **6 before, 6 after** (count unchanged).

## What Was Fixed

### 1. `OrderedTableStPer.rs` — baseline verification error (Chap43)

The `intersection` method (line 1346) failed its precondition: `other.find(&pair.0)`
required `other.spec_orderedtablestper_wf()`, but that predicate includes `finite()` on
the mapped view — not provable from loop invariants without re-reading the lock.

**Fix**: introduced `spec_orderedtablestper_find_wf` — a weaker predicate containing
only the structural properties `find` actually needs (AVL seq wf + no-dup keys). Changed
`find`'s `requires` in the trait to use the weaker predicate. Existing loop invariants
satisfy `spec_orderedtablestper_find_wf` without additional work.

Result: `OrderedTableStPer.rs` went from ❌ (verification error) to ⚠ (warnings only,
all `assume_eq_clone_workaround`).

### 2. `OrderedSetStPer.rs` — `fn_missing_requires` warning (Chap43)

`from_sorted_elements` had no `requires` clause. Added:

```rust
requires elements@.len() < usize::MAX,
```

This is a real constraint (the AVL tree needs room to insert), consistent with the
ephemeral counterpart. Warning gone.

## What Was Attempted But Not Closed

| # | Chap | File | Hole | Reason Not Closed |
|---|:----:|---|---|---|
| 1 | 41 | AVLTreeSetStEph.rs:722 | union capacity | Needs Chap53 requires cascade |
| 2 | 43 | OrderedTableMtPer.rs:316 | `assume(len < usize::MAX)` | `AVLTreeSeqStPer::wf` lacks len bound |
| 3 | 43 | OrderedSetStEph.rs:1134 | filter cardinality in `select` | Needs sortedness proof infra |
| 4 | 43 | OrderedSetStPer.rs:1031 | filter cardinality in `select` | Same as above |
| 5 | 43 | AugOrderedTableStPer.rs:124 | closure clone preserves `requires` | No Verus mechanism |
| 6 | 43 | AugOrderedTableMtEph.rs:672 | `reduce_range_parallel` ext_body | Threading boundary |

## Techniques Used

- **Weaker predicate split**: factored `spec_orderedtablestper_find_wf` from the full
  `spec_orderedtablestper_wf` to satisfy the precondition that loop invariants can prove.
- **Real requires on constructor**: added a capacity bound to `from_sorted_elements`.

## Remaining Holes — What Blocks Them

**AVLTreeSetStEph.rs:722 union capacity** — adding `requires self@.len() + other@.len() < usize::MAX`
to the `union` trait causes 10 errors in Chap53 files (not in scope this round).

**OrderedTableMtPer.rs:316 len capacity** — `AVLTreeSeqStPer::spec_avltreeseqstper_wf`
does not include a `len < usize::MAX` bound (unlike the ephemeral `StEph` counterpart).
The lock boundary loses the bound. The real fix is to add the bound to the StPer wf
predicate, which propagates through all callers.

**Select filter cardinality (StEph/StPer)** — `select` returns the `i`-th element by
scanning linearly and assuming there are exactly `i` elements less than it. Proving this
formally requires a sortedness invariant on the backing sequence, which `spec_wf` currently
does not capture. Building this proof infrastructure would require new lemmas connecting
`AVLTreeSeq::spec_avltreeseqsteph_wf` to `spec_seq_sorted`.

**Closure clone preserves `requires` (AugOrderedTableStPer.rs:124)** — Verus has no
general axiom that `Clone::clone` preserves a closure's `requires` property for arbitrary
`F: Fn + Clone`. The `assume` is the only current mechanism. No workaround found without
adding a custom trait or axiom (would require user approval).
