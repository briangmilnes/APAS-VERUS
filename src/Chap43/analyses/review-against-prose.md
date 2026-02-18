<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 43: Ordering and Augmentation — Review Against Prose

**Date:** 2026-02-17
**Reviewer:** Claude-Opus-4.6
**Prose Source:** `prompts/Chap43.txt`

## Phase 2: Implementation Strategy

The prose defines three ADT families. The implementations span 11 source files across 10 data structure variants plus one example file.

### ADT Family 1: Ordered Sets (ADT 43.1)

| # | File | Backing Store | Ordering Strategy | True Cost of Ordering Ops |
|---|---|---|---|---|
| 1 | `OrderedSetStEph.rs` | `AVLTreeSetStEph` (Chap41) | `to_seq()` → linear scan/filter | O(n) |
| 2 | `OrderedSetStPer.rs` | `AVLTreeSetStPer` (Chap41) | `to_seq()` → linear scan/filter | O(n) |
| 3 | `OrderedSetMtEph.rs` | `ParamTreap` (Chap39) | Tree traversal via `expose()` + `split()` | O(lg n) for most ops |

**StEph/StPer pattern:** Every ordering operation calls `to_seq()` which materializes the entire sorted sequence in O(n), then scans linearly. The operations `first`, `last`, `previous`, `next`, `get_range`, `rank`, `select`, `split`, `split_rank` all follow this pattern. `join` delegates to `union` which is O(m+n).

**MtEph pattern:** Uses tree-native operations. `first`/`last` traverse the left/right spine in O(lg n). `previous`/`next` split the tree then find min/max of one half in O(lg n). `rank` splits and measures left size in O(lg n). `get_range` uses two splits in O(lg n). However, `select` and `split_rank` call `in_order()` which materializes the sequence in O(n), making them O(n) despite the O(lg n) doc comment.

### ADT Family 2: Ordered Tables (ADT 43.1 adapted for key-value pairs)

| # | File | Backing Store | Ordering Strategy | True Cost of Ordering Ops |
|---|---|---|---|---|
| 4 | `OrderedTableStEph.rs` | `TableStEph` (Chap42) | `collect()` → linear scan/filter | O(n) |
| 5 | `OrderedTableStPer.rs` | `TableStPer` (Chap42) | `collect()` → linear scan/filter | O(n) |
| 6 | `OrderedTableMtEph.rs` | `TableMtEph` (Chap42) | `collect()` → linear scan/filter | O(n) |
| 7 | `OrderedTableMtPer.rs` | `ParamTreap<Pair<K,V>>` (Chap39) | No ordering ops implemented | N/A |

All table ordering implementations (StEph, StPer, MtEph) follow the same linearization pattern: call `collect()` to materialize sorted key-value pairs, then scan linearly. The MtEph table implementation does **not** use tree-native operations for ordering, unlike the MtEph set. This is a lost opportunity; the MtEph table delegates to `TableMtEph` which itself wraps a treap, but the ordering layer doesn't reach through to use the treap's `split`/`expose`.

OrderedTableMtPer is a **minimal stub** — it implements only the base table operations (`size`, `empty`, `singleton`, `find`, `insert`, `delete`, `domain`, `map`, `filter`) and none of the 10 ADT 43.1 ordering operations.

### ADT Family 3: Augmented Ordered Tables (ADT 43.3)

| # | File | Backing Store | Augmentation Strategy | reduce_val Cost | reduce_range Cost |
|---|---|---|---|---|---|
| 8 | `AugOrderedTableStEph.rs` | `OrderedTableStEph` + cached reduction | Flat cached `V` field | O(1) | O(n) (via get_key_range) |
| 9 | `AugOrderedTableStPer.rs` | `OrderedTableStPer` + cached reduction | Flat cached `V` field | O(1) | O(n) (via get_key_range) |
| 10 | `AugOrderedTableMtEph.rs` | `OrderedTableMtEph` + cached reduction | Flat cached `V` field; `ParaPair!` for parallel range | O(1) | O(n) (via get_key_range) |

All three augmented variants maintain a `cached_reduction` field that provides O(1) `reduce_val`. However, `reduce_range` calls `get_key_range` (O(n) in all implementations) followed by `reduce_val` (O(1)), so `reduce_range` is actually O(n) in all variants — not the O(lg n) the prose specifies.

The prose envisions augmentation stored per-node in a BST, enabling O(lg n) range reductions by combining subtree annotations during a single tree traversal. The current flat-cache approach cannot achieve this; it requires a full recalculation after every mutation that changes the set of values (delete, intersection, difference, filter, restrict, subtract).

**AugOrderedTableMtEph** adds `reduce_range_parallel` which uses `ParaPair!` to parallelize the reduction of two sub-ranges. This goes beyond the prose specification.

### Example File

| # | File | Purpose |
|---|---|---|
| 11 | `Example43_1.rs` | Demonstrates all Example 43.1 operations from the textbook using `OrderedSetStPer` |

## Phase 3: Cost Analysis

The prose states (Cost Specification 43.2): "The work and span for all the operations in ADT 43.1 is O(lg n)."

### Ordered Sets

| # | Operation | APAS | StEph/StPer Claimed | StEph/StPer Actual | MtEph Claimed | MtEph Actual | Match? |
|---|---|---|---|---|---|---|---|
| 1 | first | O(lg n) | O(lg n) | O(n) | O(lg n) | O(lg n) | St: ❌ Mt: ✅ |
| 2 | last | O(lg n) | O(lg n) | O(n) | O(lg n) | O(lg n) | St: ❌ Mt: ✅ |
| 3 | previous | O(lg n) | O(lg n) | O(n) | O(lg n) | O(lg n) | St: ❌ Mt: ✅ |
| 4 | next | O(lg n) | O(lg n) | O(n) | O(lg n) | O(lg n) | St: ❌ Mt: ✅ |
| 5 | split | O(lg n) | O(lg n) | O(n) | O(lg n) | O(lg n) | St: ❌ Mt: ✅ |
| 6 | join | O(lg n) | O(lg(m+n)) | O(m+n) via union | O(lg(m+n)) | O(lg(m+n)) | St: ❌ Mt: ✅ |
| 7 | getRange | O(lg n) | O(lg n) | O(n) | O(lg n) | O(lg n) | St: ❌ Mt: ✅ |
| 8 | rank | O(lg n) | O(lg n) | O(n) | O(lg n) | O(lg n) | St: ❌ Mt: ✅ |
| 9 | select | O(lg n) | O(lg n) | O(n) | O(lg n) | O(n) via `in_order()` | St: ❌ Mt: ❌ |
| 10 | splitRank | O(lg n) | O(lg n) | O(n) | O(lg n) | O(n) via `in_order()` | St: ❌ Mt: ❌ |

### Ordered Tables

| # | Operation | APAS | StEph/StPer/MtEph Claimed | StEph/StPer/MtEph Actual | Match? |
|---|---|---|---|---|---|
| 1 | first_key | O(lg n) | O(lg n) | O(n) via `collect()` | ❌ |
| 2 | last_key | O(lg n) | O(lg n) | O(n) via `collect()` | ❌ |
| 3 | previous_key | O(lg n) | O(lg n) | O(n) via `collect()` | ❌ |
| 4 | next_key | O(lg n) | O(lg n) | O(n) via `collect()` | ❌ |
| 5 | split_key | O(lg n) | O(lg n) | O(n) via `collect()` | ❌ |
| 6 | join_key | O(lg n) | O(lg(m+n)) | O(m+n) via union | ❌ |
| 7 | get_key_range | O(lg n) | O(lg n) | O(n) via `collect()` | ❌ |
| 8 | rank_key | O(lg n) | O(lg n) | O(n) via `collect()` | ❌ |
| 9 | select_key | O(lg n) | O(lg n) | O(n) via `collect()` | ❌ |
| 10 | split_rank_key | O(lg n) | O(lg n) | O(n) via `collect()` | ❌ |

### Augmented Tables

| # | Operation | APAS | All Variants Claimed | All Variants Actual | Match? |
|---|---|---|---|---|---|
| 1 | reduce_val | O(1) | O(1) | O(1) | ✅ |
| 2 | reduce_range | O(lg n) | O(lg n) | O(n) via get_key_range | ❌ |
| 3 | reduce_range_parallel | N/A (beyond prose) | O(lg n) | O(n) base + thread overhead | N/A |

**Summary:** The only implementations that achieve the prose's O(lg n) bound are OrderedSetMtEph's first/last/previous/next/split/join/getRange/rank — 8 of 10 operations. Every other implementation linearizes through `to_seq()`/`collect()`/`in_order()` and runs in O(n). **Doc comments in all StEph/StPer files claim O(lg n) which is incorrect.**

## Phase 4: Semantic Fidelity

### split signature mismatch

The prose defines `split(A, k) = ({k' < k}, k ∈? S, {k' > k})` — a 3-tuple returning left set, membership boolean/value, and right set.

| # | Implementation | Signature | Matches Prose? |
|---|---|---|---|
| 1 | OrderedSetStEph | `split(&mut self, k: &T) -> (Self, B, Self)` | ✅ |
| 2 | OrderedSetStPer | `split(&self, k: &T) -> (Self, B, Self)` | ✅ |
| 3 | OrderedSetMtEph | `split(&mut self, k: &T) -> (Self, B, Self)` | ✅ |
| 4 | OrderedTableStEph | `split_key(&mut self, k: &K) -> (Self, Self)` | ❌ drops found value, puts k≥ in right |
| 5 | OrderedTableStPer | `split_key(&self, k: &K) -> (Self, Option<V>, Self)` | ✅ |
| 6 | OrderedTableMtEph | `split_key(&mut self, k: &K) -> (Self, Self)` | ❌ drops found value, puts k≥ in right |
| 7 | AugOrderedTableStEph | `split_key(&mut self, k: &K) -> (Self, Self)` | ❌ drops found value |
| 8 | AugOrderedTableStPer | `split_key(&self, k: &K) -> (Self, Option<V>, Self)` | ✅ |
| 9 | AugOrderedTableMtEph | `split_key(&mut self, k: &K) -> (Self, Self)` | ❌ drops found value |

Four table implementations drop the found value from split, losing information the prose provides.

### join precondition not enforced

The prose requires `max(A1) < min(A2)` for join. All implementations delegate to `union`, which works correctly for overlapping sets but does not enforce or check the precondition. This is functionally safe but does not match the prose's precondition semantics.

### OrderedTableMtPer: map is actually filter

```rust
fn map<F: Pred<Pair<K, V>>>(&self, f: F) -> Self {
    OrderedTableMtPer { tree: self.tree.filter(f) }
}
```

The `map` method takes a predicate (returns bool) and calls `filter` internally. This is semantically a filter, not a map. The prose map transforms values; this removes entries. The test confirms this misuse:

```rust
let mapped = table.map(|pair: &Pair<i32, String>| pair.0 < 5);
assert_eq!(mapped.size(), 5); // keeps entries where key < 5
```

### Augmented table insert bug (documented)

When inserting a key that already exists, the cached reduction appends the new value's contribution without removing the old value's contribution. This produces incorrect results for non-idempotent reducers. Both StEph and MtEph test files document this:

```rust
// Note: Same bug as MtEph version - appends instead of replacing in cached reduction
// TODO: Fix AugOrderedTable reduction logic for key replacements
```

The StPer variant avoids this by always recalculating from scratch after insert, but that makes insert O(n).

## Phase 5: Verus Verification Status

**None of the Chapter 43 source files contain `verus!` blocks.** All code is plain Rust with no formal verification.

| # | File | Inside verus! | Specs (requires/ensures) | Proof Holes | Status |
|---|---|---|---|---|---|
| 1 | `OrderedSetStEph.rs` | No | None | 0 | Unverified |
| 2 | `OrderedSetStPer.rs` | No | None | 0 | Unverified |
| 3 | `OrderedSetMtEph.rs` | No | None | 0 | Unverified |
| 4 | `OrderedTableStEph.rs` | No | None | 0 | Unverified |
| 5 | `OrderedTableStPer.rs` | No | None | 0 | Unverified |
| 6 | `OrderedTableMtEph.rs` | No | None | 0 | Unverified |
| 7 | `OrderedTableMtPer.rs` | No | None | 0 | Unverified |
| 8 | `AugOrderedTableStEph.rs` | No | None | 0 | Unverified |
| 9 | `AugOrderedTableStPer.rs` | No | None | 0 | Unverified |
| 10 | `AugOrderedTableMtEph.rs` | No | None | 0 | Unverified |
| 11 | `Example43_1.rs` | No | None | 0 | Unverified |

## Phase 6: TOC Headers and File Structure

No source files contain TOC section headers (the standard `// Table of Contents` block). Since none of the files use `verus!` blocks, the TOC standard doesn't strictly apply.

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

### Runtime Tests (RTT): 11 test files

| # | Test File | # Tests | Covers | Key Scenarios |
|---|---|---|---|---|
| 1 | `TestOrderedSetStEph.rs` | 19 | All ADT 43.1 set ops + ephemeral semantics | empty, singleton, CRUD, ordering ops, filter, set ops, string ordering, macro |
| 2 | `TestOrderedSetStPer.rs` | 19 | All ADT 43.1 set ops + persistence | Same as StEph plus persistence verification, split at non-existing element |
| 3 | `TestOrderedSetMtEph.rs` | 20 | All ADT 43.1 set ops + threading | Same as StEph plus parallel ops, thread safety with Arc, large dataset |
| 4 | `TestOrderedTableStEph.rs` | 34 | All ADT 43.1 table ops + base table ops | CRUD, ordering, filter, map, reduce, restrict, subtract, domain, tabulate, clone |
| 5 | `TestOrderedTableStPer.rs` | 24 | All ADT 43.1 table ops + persistence | Same themes plus persistence verification, macro construction |
| 6 | `TestOrderedTableMtEph.rs` | 33 | All ADT 43.1 table ops + parallelism | Same themes plus thread safety, parallel filter/map/reduce, macro |
| 7 | `TestOrderedTableMtPer.rs` | 7 | Base table ops only (no ordering ops) | CRUD, filter, map (actually filter), domain, persistence |
| 8 | `TestAugOrderedTableStEph.rs` | 22 | ADT 43.3 + ordering + base ops | reduce_val, reduce_range, QADSAN scenario, split/join, map, filter, union, intersection, difference, restrict, subtract, tabulate, domain, collect, macro, Display/Debug |
| 9 | `TestAugOrderedTableStPer.rs` | 20 | ADT 43.3 + ordering + base ops | Same themes, TRAMLAW scenario, persistence |
| 10 | `TestAugOrderedTableMtEph.rs` | 26 | ADT 43.3 + ordering + parallelism | Same themes, parallel range reduction, concurrent reads/writes, thread safety |
| 11 | `TestExample43_1.rs` | 2 | Example 43.1 execution | Runs both string and integer examples |

**Total: 226 runtime tests across 11 files.**

### Proof Time Tests (PTT)

None. Expected since no Verus code exists.

### Test Quality Notes

1. **QADSAN/TRAMLAW scenarios** are well-tested in augmented table tests with realistic stock-price and sales-data examples.
2. **Thread safety** is tested via `Arc<Mutex<...>>` wrapping for write access and `Arc` sharing for concurrent reads.
3. **Edge cases** are well-covered: empty structures, singleton, delete nonexistent, split at boundaries, out-of-bounds select.
4. **The MtPer tests confirm the map-is-actually-filter bug** — the test asserts filtering behavior, not value transformation.
5. **The string concatenation tests document the insert-on-existing-key bug** with explicit TODO comments.

## Phase 8: Cost Annotations

### Annotation Format

Two formats are used:

1. **Trait declarations:** `/// claude-4-sonet: Work Θ(...), Span Θ(...)` — present on most trait methods
2. **Implementations:** `/// Claude Work: O(...), Span: O(...)` — present on most impl methods

### APAS Cost Annotations

Only `Example43_1.rs` has explicit APAS cost comments (`/// APAS: Work Θ(n log n), Span Θ(log n)`). No other file has paired `/// APAS: ...` / `/// Claude: ...` cost comments per the project standard.

### Cost Annotation Accuracy

| # | Issue | Severity |
|---|---|---|
| 1 | All StEph/StPer files claim O(lg n) for ordering ops that are actually O(n) | High |
| 2 | MtEph set claims O(lg n) for `select` and `split_rank` that are O(n) | Medium |
| 3 | All MtEph table ordering ops claim O(lg n) but are O(n) via `collect()` | High |
| 4 | AugOrderedTable `reduce_range` claims O(lg n) but is O(n) via `get_key_range` | High |
| 5 | MtEph set `split` and `join` trait annotations claim O(n) work (correct) but impl comments say O(lg n) | Low — trait is more honest |
| 6 | No files have APAS/Claude paired cost comments | Medium |

### Unused Imports

| # | File | Unused Import |
|---|---|---|
| 1 | `OrderedTableMtEph.rs` | `std::sync::Arc`, `std::thread` |
| 2 | `AugOrderedTableMtEph.rs` | `std::sync::Arc`, `std::thread` (uses `ParaPair!` instead) |

## Proof Holes

```
veracity-review-proof-holes -d src/Chap43/

Modules:
   11 clean (no holes)
   0 holed (contains holes)
   11 total

Holes Found: 0 total
```

There are no proof holes because there is no Verus code — all 11 files are plain Rust without `verus!` blocks.

## ADT Interface Completeness

| # | ADT Operation | OSet StEph | OSet StPer | OSet MtEph | OTbl StEph | OTbl StPer | OTbl MtEph | OTbl MtPer | Aug StEph | Aug StPer | Aug MtEph |
|---|---|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|:---:|
| 1 | first / first_key | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | - | ✅ | ✅ | ✅ |
| 2 | last / last_key | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | - | ✅ | ✅ | ✅ |
| 3 | previous / previous_key | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | - | ✅ | ✅ | ✅ |
| 4 | next / next_key | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | - | ✅ | ✅ | ✅ |
| 5 | split / split_key | ✅ | ✅ | ✅ | ⚠️ | ✅ | ⚠️ | - | ⚠️ | ✅ | ⚠️ |
| 6 | join / join_key | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | - | ✅ | ✅ | ✅ |
| 7 | getRange / get_key_range | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | - | ✅ | ✅ | ✅ |
| 8 | rank / rank_key | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | - | ✅ | ✅ | ✅ |
| 9 | select / select_key | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | - | ✅ | ✅ | ✅ |
| 10 | splitRank / split_rank_key | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | - | ✅ | ✅ | ✅ |
| 11 | reduceVal | - | - | - | - | - | - | - | ✅ | ✅ | ✅ |
| 12 | reduce_range | - | - | - | - | - | - | - | ✅ | ✅ | ✅ |
| 13 | reduce_range_parallel | - | - | - | - | - | - | - | - | - | ✅ |

**Legend:** ✅ = implemented and semantically matches prose. ⚠️ = implemented but split drops the found value (2-tuple instead of 3-tuple). `-` = not implemented.

## Example 43.1 Verification

| # | Prose Example | Expected Result | Implementation | Status |
|---|---|---|---|---|
| 1 | first A → 'artie' | min element | `first()` on sorted set | ✅ |
| 2 | next(A, 'quinn') → 'rachel' | min {k > 'quinn'} | `next()` with scan | ✅ |
| 3 | next(A, 'mike') → 'rachel' | min {k > 'mike'} | `next()` with scan | ✅ |
| 4 | getRange A ('burt','mike') → {'burt','finn','mike'} | {k: k1 ≤ k ≤ k2} | `get_range()` with filter | ✅ |
| 5 | rank(A, 'rachel') → 4 | \|{k < 'rachel'}\| | `rank()` with count | ✅ |
| 6 | rank(A, 'quinn') → 4 | \|{k < 'quinn'}\| | `rank()` with count | ✅ |
| 7 | select(A, 5) → 'sam' | k at rank 5 | `select()` with index | ✅ |
| 8 | splitRank(A, 3) → ({artie,burt,finn}, {mike,rachel,sam,tina}) | split at rank 3 | `split_rank()` with partition | ✅ |

`Example43_1.rs` also demonstrates `last`, `previous`, `split`, and `join` with both string and integer examples. The example file is a faithful reproduction of the textbook's Example 43.1.

## Review TODOs

| # | Priority | Category | Description |
|---|---|---|---|
| 1 | High | Cost | Fix doc comments in all St files: ordering ops are O(n), not O(lg n) |
| 2 | High | Cost | Fix doc comments in MtEph table: ordering ops are O(n) via `collect()` |
| 3 | High | Cost | Fix doc comments for `select`/`split_rank` in MtEph set: O(n) via `in_order()` |
| 4 | High | Semantic | Fix split_key in OrderedTableStEph, OrderedTableMtEph, AugOrderedTableStEph, AugOrderedTableMtEph to return `(Self, Option<V>, Self)` matching prose 3-tuple |
| 5 | High | Bug | Fix augmented table insert: when key already exists, cached_reduction is corrupted for non-idempotent reducers |
| 6 | High | API | Fix OrderedTableMtPer::map — currently implements filter semantics, not map |
| 7 | High | Completeness | Add all 10 ADT 43.1 ordering operations to OrderedTableMtPer |
| 8 | Medium | Cost | Add paired APAS/Claude cost comments to all functions per project standard |
| 9 | Medium | Verification | Verus-ify at least one variant per ADT family (suggest StPer variants first) |
| 10 | Medium | Architecture | Consider rewriting MtEph table ordering ops to use tree-native operations instead of `collect()` linearization |
| 11 | Medium | Architecture | Consider augmented BST approach (per-node annotations) for AugOrderedTable to achieve true O(lg n) reduce_range |
| 12 | Low | Style | No files have TOC headers (acceptable since no verus! blocks) |
| 13 | Low | Cleanup | Remove unused imports (`Arc`, `thread`) from OrderedTableMtEph.rs and AugOrderedTableMtEph.rs |
| 14 | Low | Testing | No PTTs exist (blocked on Verus-ification) |

## Overall Assessment

**Chapter 43 implements all three prose ADTs (Ordered Sets, Ordered Tables, Reducer-Augmented Ordered Tables) across 10 implementation files plus 1 example file, with 11 runtime test files and 226 tests. No Verus verification exists. All 11 files are proof-hole clean (trivially, since there is no Verus code).**

### Strengths

1. **Interface completeness:** All 10 operations from ADT 43.1 are implemented across 9 of 10 variants (MtPer is the exception). All 2 augmented operations from ADT 43.3 are implemented across 3 variants.
2. **Augmented tables:** The reducer-augmented table correctly provides O(1) `reduce_val` via cached reduction, matching the prose's key innovation.
3. **Example fidelity:** Example 43.1 is faithfully reproduced and tested with all operations from the textbook.
4. **Test depth:** 226 runtime tests including realistic QADSAN/TRAMLAW scenarios, thread safety, persistence verification, and edge cases.
5. **MtEph set tree operations:** OrderedSetMtEph correctly uses tree traversal for 8 of 10 ordering operations, achieving O(lg n).

### Weaknesses

1. **No Verus verification** across all 11 files.
2. **Systematically incorrect cost claims:** 7 of 10 implementation files claim O(lg n) for ordering operations that actually run in O(n).
3. **split_key semantic mismatch:** 4 table implementations drop the found value, diverging from the prose's 3-tuple return.
4. **OrderedTableMtPer is a stub** missing all ordering operations.
5. **OrderedTableMtPer::map is actually filter** — wrong API semantics.
6. **Augmented table insert bug** corrupts cached reduction on key replacement for non-idempotent reducers.
7. **No implementation achieves O(lg n) for all 10 ordering operations.** The closest is MtEph set (8/10), but `select` and `split_rank` still linearize.
8. **reduce_range is O(n) in all variants**, not the O(lg n) the prose specifies, because it depends on get_key_range which linearizes.
