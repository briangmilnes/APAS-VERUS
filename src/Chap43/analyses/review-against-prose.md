<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 43: Ordering and Augmentation — Review Against Prose

**Date:** 2026-02-13
**Reviewer:** Claude-Opus-4.6
**Prose Source:** `prompts/Chap43.txt`, `prompts/Chap43part2.txt`

## Phase 1: ADT and Interface Completeness

The prose defines three ADTs:

1. **ADT 43.1 — Ordered Sets**: Extends set ADT (41.1) with: `first`, `last`, `previous`, `next`, `split`, `join`, `getRange`, `rank`, `select`, `splitRank`.
2. **Ordered Tables**: "completely analogous" operations on tables, extending ADT 42.1 with key-based ordering.
3. **ADT 43.3 — Reducer-Augmented Ordered Table**: Extends ordered tables with `reduceVal` and supports efficient range reductions.

| # | ADT Operation | OrderedSetStEph | OrderedSetStPer | OrderedSetMtEph | OrderedTableStEph | OrderedTableStPer | OrderedTableMtEph | OrderedTableMtPer | AugOrderedTableStEph | AugOrderedTableStPer | AugOrderedTableMtEph |
|---|---|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| 1 | first / first_key | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | - | ✅ | ✅ | ✅ |
| 2 | last / last_key | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | - | ✅ | ✅ | ✅ |
| 3 | previous / previous_key | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | - | ✅ | ✅ | ✅ |
| 4 | next / next_key | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | - | ✅ | ✅ | ✅ |
| 5 | split / split_key | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | - | ✅ | ✅ | ✅ |
| 6 | join / join_key | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | - | ✅ | ✅ | ✅ |
| 7 | getRange / get_key_range | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | - | ✅ | ✅ | ✅ |
| 8 | rank / rank_key | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | - | ✅ | ✅ | ✅ |
| 9 | select / select_key | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | - | ✅ | ✅ | ✅ |
| 10 | splitRank / split_rank_key | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | - | ✅ | ✅ | ✅ |
| 11 | reduceVal | - | - | - | - | - | - | - | ✅ | ✅ | ✅ |
| 12 | reduce_range | - | - | - | - | - | - | - | ✅ | ✅ | ✅ |
| 13 | reduce_range_parallel | - | - | - | - | - | - | - | - | - | ✅ |

**Notes:**
- `OrderedTableMtPer` is a minimal implementation that only covers base table operations (`size`, `empty`, `singleton`, `find`, `insert`, `delete`, `domain`, `map`, `filter`). It does not implement the ADT 43.1 ordering operations (first_key, last_key, previous_key, next_key, split_key, join_key, get_key_range, rank_key, select_key, split_rank_key).
- `AugOrderedTableMtEph` adds `reduce_range_parallel` beyond what the prose specifies — this is an enhancement.
- Example 43.1 is implemented and demonstrates all ordered set operations from the textbook.

**Missing from prose:** The prose mentions Exercise 43.1 ("Describe how to implement previous and next using the other ordered set functions") but no separate exercise file exists. The implementations do implement `previous`/`next` directly.

## Phase 2: Implementation Strategy

| # | File | Backing Store | Strategy |
|---|---|---|---|
| 1 | `OrderedSetStEph.rs` | `AVLTreeSetStEph` (Chap41) | Delegate base ops; ordering ops via `to_seq` + linear scan |
| 2 | `OrderedSetStPer.rs` | `AVLTreeSetStPer` (Chap41) | Delegate base ops; ordering ops via `to_seq` + linear scan |
| 3 | `OrderedSetMtEph.rs` | `ParamTreap` (Chap39) | Base ops delegate; ordering ops use tree traversal/split |
| 4 | `OrderedTableStEph.rs` | `TableStEph` (Chap42) | Delegate base ops; ordering ops via `collect` + linear scan |
| 5 | `OrderedTableStPer.rs` | `TableStPer` (Chap42) | Delegate base ops; ordering ops via `collect` + linear scan |
| 6 | `OrderedTableMtEph.rs` | `TableMtEph` (Chap42) | Delegate base ops; ordering ops via `collect` + linear scan |
| 7 | `OrderedTableMtPer.rs` | `ParamTreap<Pair<K,V>>` (Chap39) | Direct treap operations; binary search for find |
| 8 | `AugOrderedTableStEph.rs` | `OrderedTableStEph` + cached reduction | Delegate + maintain cached_reduction |
| 9 | `AugOrderedTableStPer.rs` | `OrderedTableStPer` + cached reduction | Delegate + maintain cached_reduction |
| 10 | `AugOrderedTableMtEph.rs` | `OrderedTableMtEph` + cached reduction | Delegate + maintain cached_reduction; ParaPair! for parallel range reduction |
| 11 | `Example43_1.rs` | Uses `OrderedSetStPer` | Textbook example demonstration |

**Key design decisions:**
- StEph/StPer implementations linearize ordering operations through `to_seq()`/`collect()` followed by linear scanning. This makes operations O(n) rather than the O(lg n) the prose specifies. The prose assumes BST-based implementations where these operations can be done in O(lg n) by tree traversal.
- MtEph set implementation correctly uses tree traversal (`expose()`) for first/last and `split()` for previous/next/rank, achieving O(lg n).
- Augmented tables maintain a `cached_reduction` field, enabling O(1) `reduce_val`. However, `reduce_range` calls `get_key_range` then `reduce_val`, and `get_key_range` itself is O(n) in the St implementations, so the claimed O(lg n) for `reduce_range` is only achieved in the Mt implementation.

## Phase 3: Cost Analysis

The prose states (Cost Specification 43.2): "The work and span for all the operations in ADT 43.1 is O(lg n)."

| # | Operation | APAS Cost | Claude St Cost | Claude Mt Cost | Match? |
|---|---|---|---|---|---|
| 1 | first/last | O(lg n) | O(lg n) claimed, actually O(n) due to to_seq | O(lg n) via tree traversal | St: ❌ Mt: ✅ |
| 2 | previous/next | O(lg n) | O(lg n) claimed, actually O(n) due to to_seq + linear scan | O(lg n) via split + tree traversal | St: ❌ Mt: ✅ |
| 3 | split | O(lg n) | O(lg n) claimed, actually O(n) due to to_seq + filtering | O(lg n) via tree split | St: ❌ Mt: ✅ |
| 4 | join | O(lg n) | O(lg(m+n)) claimed, delegates to union which is O(m+n) | O(lg(m+n)) via join_pair | St: ❌ Mt: ✅ |
| 5 | getRange | O(lg n) | O(lg n) claimed, actually O(n) due to to_seq + filtering | O(lg n) via split | St: ❌ Mt: ✅ |
| 6 | rank | O(lg n) | O(lg n) claimed, actually O(n) due to to_seq + linear count | O(lg n) via split | St: ❌ Mt: ✅ |
| 7 | select | O(lg n) | O(lg n) claimed, actually O(n) due to to_seq | O(lg n) via in_order + index | St: ❌ Mt: ⚠️ |
| 8 | splitRank | O(lg n) | O(lg n) claimed, actually O(n) due to to_seq + splitting | O(lg n) via in_order + split | St: ❌ Mt: ⚠️ |
| 9 | reduceVal | O(1) | O(1) | O(1) | ✅ |
| 10 | reduce_range | O(lg n) | O(lg n) claimed, actually O(n) due to get_key_range | O(lg n) | St: ❌ Mt: ✅ |

**Major finding:** The StEph/StPer implementations claim O(lg n) costs in their doc comments but actually run in O(n) because they linearize via `to_seq()`/`collect()` before operating. The Mt implementations correctly achieve O(lg n) for most operations through tree-native operations.

For MtEph `select` and `split_rank`: these call `in_order()` which is O(n), making the actual cost O(n) rather than O(lg n). An augmented BST with subtree sizes would be needed for true O(lg n).

## Phase 4: Example Verification

| # | Example | Prose Expected | Implementation | Status |
|---|---|---|---|---|
| 1 | first A → 'artie' | min element | `first()` on sorted set | ✅ |
| 2 | next(A, 'quinn') → 'rachel' | min {k > 'quinn'} | `next()` scans | ✅ |
| 3 | next(A, 'mike') → 'rachel' | min {k > 'mike'} | `next()` scans | ✅ |
| 4 | getRange A ('burt', 'mike') → {'burt', 'finn', 'mike'} | {k: k1 ≤ k ≤ k2} | `get_range()` filters | ✅ |
| 5 | rank(A, 'rachel') → 4 | \|{k < 'rachel'}\| | `rank()` counts | ✅ |
| 6 | rank(A, 'quinn') → 4 | \|{k < 'quinn'}\| | `rank()` counts | ✅ |
| 7 | select(A, 5) → 'sam' | k at rank 5 | `select()` indexes | ✅ |
| 8 | splitRank(A, 3) → ({artie,burt,finn}, {mike,rachel,sam,tina}) | split at rank 3 | `split_rank()` partitions | ✅ |

`Example43_1.rs` also demonstrates: `last`, `previous`, `split`, and `join` operations with both string and integer examples.

**TRAMLAW/QADSAN examples (from Chap43part2.txt):**
- The augmented table implementations correctly support `reduceVal(getRange(T, t1, t2))` pattern for range queries.
- AugOrderedTableMtEph adds `reduce_range_parallel` for parallel range reduction.

## Phase 5: Verus Verification Status

**None of the Chapter 43 source files contain `verus!` blocks.** All code is plain Rust with no formal verification.

| # | File | Inside verus! | Specs (requires/ensures) | Status |
|---|---|---|---|---|
| 1 | `OrderedSetStEph.rs` | No | None | Unverified |
| 2 | `OrderedSetStPer.rs` | No | None | Unverified |
| 3 | `OrderedSetMtEph.rs` | No | None | Unverified |
| 4 | `OrderedTableStEph.rs` | No | None | Unverified |
| 5 | `OrderedTableStPer.rs` | No | None | Unverified |
| 6 | `OrderedTableMtEph.rs` | No | None | Unverified |
| 7 | `OrderedTableMtPer.rs` | No | None | Unverified |
| 8 | `AugOrderedTableStEph.rs` | No | None | Unverified |
| 9 | `AugOrderedTableStPer.rs` | No | None | Unverified |
| 10 | `AugOrderedTableMtEph.rs` | No | None | Unverified |
| 11 | `Example43_1.rs` | No | None | Unverified |

## Phase 6: TOC Headers and File Structure

No source files contain TOC section headers (the standard `// Table of Contents` block). Since none of the files use `verus!` blocks, the TOC standard doesn't strictly apply — these are plain Rust modules.

| # | File | Has TOC | Has Module Header | Has Copyright |
|---|---|---|---|---|
| 1 | `OrderedSetStEph.rs` | No | Yes | Yes |
| 2 | `OrderedSetStPer.rs` | No | Yes | Yes |
| 3 | `OrderedSetMtEph.rs` | No | Yes | Yes |
| 4 | `OrderedTableStEph.rs` | No | Yes | Yes |
| 5 | `OrderedTableStPer.rs` | No | Yes | Yes |
| 6 | `OrderedTableMtEph.rs` | No | Yes | Yes |
| 7 | `OrderedTableMtPer.rs` | No | Yes | Yes |
| 8 | `AugOrderedTableStEph.rs` | No | Yes | Yes |
| 9 | `AugOrderedTableStPer.rs` | No | Yes | Yes |
| 10 | `AugOrderedTableMtEph.rs` | No | Yes | Yes |
| 11 | `Example43_1.rs` | No | Yes | Yes |

## Phase 7: Test Coverage

**Runtime tests (RTT):** 12 test files found in `tests/Chap43/`:

| # | Test File | Tests |
|---|---|---|
| 1 | `TestOrderedSetStEph.rs` | Ordered set operations (StEph) |
| 2 | `TestOrderedSetStPer.rs` | Ordered set operations (StPer) |
| 3 | `TestOrderedSetMtEph.rs` | Ordered set operations (MtEph) |
| 4 | `TestOrderedTableStEph.rs` | Ordered table operations (StEph) |
| 5 | `TestOrderedTableStPer.rs` | Ordered table operations (StPer) |
| 6 | `TestOrderedTableMtEph.rs` | Ordered table operations (MtEph) |
| 7 | `TestOrderedTableMtPer.rs` | Ordered table operations (MtPer) |
| 8 | `TestAugOrderedTableStEph.rs` | Augmented table operations (StEph) |
| 9 | `TestAugOrderedTableStPer.rs` | Augmented table operations (StPer) |
| 10 | `TestAugOrderedTableMtEph.rs` | Augmented table operations (MtEph) |
| 11 | `TestExample43_1.rs` | Example 43.1 execution |

**Proof time tests (PTT):** None found in `rust_verify_test/tests/` matching Chap43. Expected, since no Verus code exists.

## Phase 8: Cost Annotations

Cost annotations use two formats:
1. **Trait declarations:** `/// claude-4-sonet: Work Θ(...), Span Θ(...)` — present on most trait methods.
2. **Implementations:** `/// Claude Work: O(...), Span: O(...)` — present on most impl methods.

**APAS cost annotations:** Only `Example43_1.rs` has explicit APAS cost comments. The source files reference APAS costs indirectly through the `claude-4-sonet` annotations which attempt to match the textbook but are inaccurate for the St implementations (see Phase 3).

**Missing APAS cost comment pairs:**
- All 11 files lack explicit `/// APAS: Work Θ(...), Span Θ(...)` paired comments.
- The Claude-attributed cost annotations are present but often overstate performance (claiming O(lg n) where actual is O(n)).

**Recommendation:** Add paired APAS/Claude cost comments to ordering operation implementations. The APAS costs should reference Cost Specification 43.2 ("O(lg n) for all ADT 43.1 operations"). Claude costs should reflect the actual implementation complexity.

## Proof Holes Summary

```
veracity-review-proof-holes -d src/Chap43/

Modules:
   11 clean (no holes)
   0 holed (contains holes)
   11 total

Holes Found: 0 total

🎉 No proof holes found! All proofs are complete.
```

There are no proof holes because there is no Verus code — all 11 files are plain Rust without `verus!` blocks.

## Spec Strength Summary

| Classification | Count |
|---|---|
| strong | 0 |
| partial | 0 |
| weak | 0 |
| none | 265 |

All 265 extracted function entries have `spec_strength: none` because no file contains `verus!` blocks or `requires`/`ensures` specifications.

## Overall Assessment

**Chapter 43 implements all three prose ADTs (Ordered Sets, Ordered Tables, Reducer-Augmented Ordered Tables) across 10 implementation files plus 1 example file, with 12 runtime test files. No Verus verification exists.**

### Strengths

1. **Interface completeness:** All 10 operations from ADT 43.1 are implemented across set and table variants (except OrderedTableMtPer which is minimal).
2. **Augmented tables:** The reducer-augmented table (ADT 43.3) is correctly implemented with O(1) `reduce_val` via cached reduction, matching the prose's key innovation.
3. **Example coverage:** Example 43.1 is faithfully reproduced and tests all operations from the textbook.
4. **Variant coverage:** 3 ordered set variants (StEph, StPer, MtEph), 4 ordered table variants (StEph, StPer, MtEph, MtPer), and 3 augmented table variants (StEph, StPer, MtEph).
5. **Mt tree operations:** OrderedSetMtEph correctly uses tree traversal for O(lg n) first/last and split-based previous/next/rank.

### Weaknesses

1. **No Verus verification:** Zero `verus!` blocks, zero specs, zero formal proofs across all 11 files.
2. **Incorrect cost claims:** StEph/StPer implementations claim O(lg n) for ordering operations but actually run in O(n) because they linearize via `to_seq()`/`collect()`. The doc comments are misleading.
3. **No APAS cost pairs:** No files have the standard `/// APAS: ... / /// Claude: ...` cost comment pairs.
4. **OrderedTableMtPer incomplete:** Missing all ADT 43.1 ordering operations (first_key through split_rank_key).
5. **No TOC headers:** None of the files follow the standard TOC format (though this is less critical for non-Verus files).
6. **No PTTs:** No proof-time tests exist (expected given no Verus code).
7. **Augmented table reduce not O(1) on mutating ops:** After `insert`, the cached reduction update is correct for appending but incorrect when the insert replaces an existing key (the old value's contribution is not subtracted). After `delete`, a full O(n) recalculation is performed, which the prose avoids via augmented BST node annotations.

### Recommendations

1. **Verus-ify the StPer implementations first** — they have the simplest ownership model.
2. **Fix cost annotations** — the St implementations should honestly state O(n) for ordering operations, or be rewritten to use BST-native traversals.
3. **Complete OrderedTableMtPer** — add the 10 missing ADT 43.1 ordering operations.
4. **Add APAS/Claude paired cost comments** to all functions per the project standard.
5. **Consider augmented BST approach** — the current cached-reduction strategy works but requires O(n) recalculation on any mutation. A proper augmented BST stores partial reductions in each node, enabling O(lg n) updates.
