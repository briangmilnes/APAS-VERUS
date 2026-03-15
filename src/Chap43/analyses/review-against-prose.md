# Chapter 43: Ordering and Augmentation -- Review Against Prose

Reviewer: Claude-Opus-4.6 (Agent 3)
Date: 2026-03-15

## Phase 1: Inventory

| # | Chap | File | Type | Lines | Holes | Status |
|---|------|------|------|-------|-------|--------|
| 1 | 43 | OrderedTableStEph.rs | StEph | 916 | 12 external_body | Active |
| 2 | 43 | OrderedTableStPer.rs | StPer | 837 | 10 external_body, 1 accept | Active |
| 3 | 43 | OrderedTableMtEph.rs | MtEph | 863 | 7 external_body | Active |
| 4 | 43 | OrderedTableMtPer.rs | MtPer | 526 | 2 assume, 6 external_body, 3 accept | Active |
| 5 | 43 | AugOrderedTableStEph.rs | StEph | 825 | 10 external_body | Active |
| 6 | 43 | AugOrderedTableStPer.rs | StPer | 876 | 2 assume, 6 external_body | Active |
| 7 | 43 | AugOrderedTableMtEph.rs | MtEph | 714 | 8 external_body (1 fn_missing_requires) | Active |
| 8 | 43 | OrderedSetStEph.rs | StEph | 779 | 1 assume, 11 external_body | Active |
| 9 | 43 | OrderedSetStPer.rs | StPer | 800 | 9 external_body | Active |
| 10 | 43 | OrderedSetMtEph.rs | MtEph | 507 | 7 assume, 8 external_body | Active |

Total: 10 files, 7641 lines, 99 holes (12 assume + 87 external_body).
Skipped: Example43_1.rs (per project rules).

## Phase 2: Prose Inventory

### APAS Textbook Coverage (Chapter 43)

| # | Prose Item | Type | Files Implementing |
|---|-----------|------|--------------------|
| 1 | ADT 43.1 Ordered Sets | Interface | OrderedSet{StEph,StPer,MtEph} |
| 2 | ADT 43.1 Ordered Tables (keys) | Interface | OrderedTable{StEph,StPer,MtEph,MtPer} |
| 3 | Cost Spec 43.2 (tree-based O(log n)) | Cost | All files |
| 4 | Augmented tables (ADT 43.3) | Interface | AugOrderedTable{StEph,StPer,MtEph} |
| 5 | first/last/previous/next | Operations | All 10 files |
| 6 | split/join | Operations | All 10 files |
| 7 | getRange | Operations | All 10 files |
| 8 | rank/select/splitRank | Operations | All 10 files |
| 9 | reduce_val/reduce_range | Operations | AugOrderedTable files only |
| 10 | reduce_range_parallel | Extension | AugOrderedTableMtEph only |

### Operations Not in APAS

| # | Operation | Files | Notes |
|---|-----------|-------|-------|
| 1 | collect (sorted entries) | All table files | Helper; returns sorted entry sequence |
| 2 | reduce_range_parallel | AugOrderedTableMtEph | Parallel extension using ParaPair! |

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

Cost annotations have been added to OrderedTableStEph.rs trait in the dual format:
```
/// - APAS: Work ..., Span ...
/// - Claude-Opus-4.6: Work ..., Span ... -- explanation
```

The other files retain existing annotations (either `/// claude-4-sonet:` or `/// ADT 43.1` format).
The key findings from cost analysis apply uniformly across all variants:

**Architectural deviation**: All OrderedTable variants use linear-scan Table (TableStEph/StPer/MtEph) as the backing store, NOT balanced BSTs. This means:

| # | Operation | APAS Cost | Actual Cost | Deviation |
|---|-----------|-----------|-------------|-----------|
| 1 | find/lookup | O(log n) | O(n) | Linear scan in entries |
| 2 | insert | O(log n) | O(n) | Linear scan for dup check |
| 3 | delete | O(log n) | O(n) | Linear scan to locate |
| 4 | first_key | O(log n) | O(n log n) | collect() + index |
| 5 | last_key | O(log n) | O(n log n) | collect() + index |
| 6 | previous_key | O(log n) | O(n log n) | collect() + scan |
| 7 | next_key | O(log n) | O(n log n) | collect() + scan |
| 8 | split_key | O(log n) | O(n log n) | collect() + partition + rebuild |
| 9 | join_key | O(m log(n/m+1)) | O(n + m) | Delegates to union (linear) |
| 10 | get_key_range | O(log n + m) | O(n log n) | collect() + filter + rebuild |
| 11 | rank_key | O(log n) | O(n log n) | collect() + count |
| 12 | select_key | O(log n) | O(n log n) | collect() + index |
| 13 | split_rank_key | O(log n) | O(n log n) | collect() + partition + rebuild |
| 14 | tabulate | O(n log n) | O(n^2) | Sequential insert loop |
| 15 | map | O(n) | O(n log n) | collect() + rebuild |
| 16 | filter | O(n) | O(n log n) | collect() + filter + rebuild |

**OrderedSet variants** (wrapping AVLTreeSetStEph/StPer) are closer to APAS costs because AVL trees provide O(log n) operations. However, ordering operations (first, last, previous, next, split, rank, select, splitRank) still use collect+scan patterns with external_body.

### 3b. Implementation Fidelity

| # | Chap | File | Fidelity | Notes |
|---|------|------|----------|-------|
| 1 | 43 | OrderedTableStEph.rs | Partial | Wraps TableStEph (linear), not balanced BST |
| 2 | 43 | OrderedTableStPer.rs | Partial | Wraps TableStPer (linear), not balanced BST |
| 3 | 43 | OrderedTableMtEph.rs | Partial | Wraps TableMtEph (linear), not balanced BST |
| 4 | 43 | OrderedTableMtPer.rs | Partial | Coarse RwLock over StPer; persistent semantics correct |
| 5 | 43 | AugOrderedTableStEph.rs | Partial | calculate_reduction is external_body; correct delegation |
| 6 | 43 | AugOrderedTableStPer.rs | Partial | Same pattern; calculate_reduction has assume(reducer.requires) |
| 7 | 43 | AugOrderedTableMtEph.rs | Partial | reduce_range_parallel uses ParaPair!; correct structure |
| 8 | 43 | OrderedSetStEph.rs | Good | Wraps AVLTreeSetStEph (balanced BST); base ops verified |
| 9 | 43 | OrderedSetStPer.rs | Good | Wraps AVLTreeSetStPer; get_range/split_rank VERIFIED |
| 10 | 43 | OrderedSetMtEph.rs | Partial | Coarse RwLock over StEph; assumes for wf bridging |

The fundamental implementation fidelity issue: APAS Chapter 43 specifies that ordered sets and tables should be implemented using balanced BSTs to achieve O(log n) for ordering operations. The ordered TABLE implementations instead wrap linear-scan Table modules, making all ordering operations O(n log n) at best.

The ordered SET implementations wrap AVL tree sets, which is closer to the textbook. However, the ordering operations (first, last, previous, next, etc.) still use collect+scan patterns instead of direct tree traversal.

### 3c. Spec Fidelity

| # | Chap | File | Spec Strength | Notes |
|---|------|------|---------------|-------|
| 1 | 43 | OrderedTableStEph.rs | Strong | Full specs on base ops; ordering ops have proper TotalOrder ensures |
| 2 | 43 | OrderedTableStPer.rs | Strong | Full specs; persistent semantics correctly return new values |
| 3 | 43 | OrderedTableMtEph.rs | Weakened | insert, map, filter, split_key etc. only ensure dom().finite() |
| 4 | 43 | OrderedTableMtPer.rs | Weakened | Most ops only ensure dom().finite(); find has no spec |
| 5 | 43 | AugOrderedTableStEph.rs | Strong | Full specs matching APAS; ordering ops have TotalOrder ensures |
| 6 | 43 | AugOrderedTableStPer.rs | Strong | Full specs; persistent semantics correct |
| 7 | 43 | AugOrderedTableMtEph.rs | Weakened | Most ops only ensure dom().finite(); find ensures contains_key only |
| 8 | 43 | OrderedSetStEph.rs | Strong | Full specs with set equality ensures on base ops |
| 9 | 43 | OrderedSetStPer.rs | Strong | Full specs; verified get_range and split_rank |
| 10 | 43 | OrderedSetMtEph.rs | Mixed | Base ops (insert/delete/intersection/union/diff) have full specs; ordering ops weaker |

Key spec fidelity observations:
- St variants have strong specs matching the APAS interface precisely.
- Mt variants have systematically weakened specs (often just `dom().finite()`), which is a common pattern in the codebase for Mt modules that use coarse locking.
- OrderedTableMtPer.find has NO ensures clause at all.
- The ordered set operations (first, last, previous, next) have correct TotalOrder-based specs across all variants that declare them.

## Phase 4: Parallelism Review

| # | Chap | File | Parallel? | Notes |
|---|------|------|-----------|-------|
| 1 | 43 | OrderedTableStEph.rs | No | Single-threaded; no parallelism expected |
| 2 | 43 | OrderedTableStPer.rs | No | Single-threaded; no parallelism expected |
| 3 | 43 | OrderedTableMtEph.rs | No | Mt module but no actual threading; sequential impl |
| 4 | 43 | OrderedTableMtPer.rs | No | Coarse RwLock; all ops acquire lock, delegate to St, release |
| 5 | 43 | AugOrderedTableStEph.rs | No | Single-threaded |
| 6 | 43 | AugOrderedTableStPer.rs | No | Single-threaded |
| 7 | 43 | AugOrderedTableMtEph.rs | Yes | reduce_range_parallel uses ParaPair! for fork-join |
| 8 | 43 | OrderedSetStEph.rs | No | Single-threaded |
| 9 | 43 | OrderedSetStPer.rs | No | Single-threaded |
| 10 | 43 | OrderedSetMtEph.rs | No | Coarse RwLock; sequential delegation |

Only AugOrderedTableMtEph.rs has actual parallelism (reduce_range_parallel). The Mt modules use coarse locking for thread safety but do not exploit parallelism internally. This is consistent with the project's standard Mt module pattern.

## Phase 5: Runtime Test Review

| # | Chap | File | RTT File | Coverage |
|---|------|------|----------|----------|
| 1 | 43 | OrderedTableStEph.rs | TestOrderedTableStEph.rs | Basic ops (empty, insert, lookup, delete, first_key) |
| 2 | 43 | OrderedTableStPer.rs | TestOrderedTableStPer.rs | Basic ops |
| 3 | 43 | OrderedTableMtEph.rs | TestOrderedTableMtEph.rs | Basic ops |
| 4 | 43 | OrderedTableMtPer.rs | TestOrderedTableMtPer.rs | Basic ops |
| 5 | 43 | AugOrderedTableStEph.rs | TestAugOrderedTableStEph.rs | Basic ops + reduce |
| 6 | 43 | AugOrderedTableStPer.rs | TestAugOrderedTableStPer.rs | Basic ops + reduce |
| 7 | 43 | AugOrderedTableMtEph.rs | TestAugOrderedTableMtEph.rs | Basic ops + reduce |
| 8 | 43 | OrderedSetStEph.rs | TestOrderedSetStEph.rs | Basic ops (empty, singleton, insert, find, delete) |
| 9 | 43 | OrderedSetStPer.rs | TestOrderedSetStPer.rs | Basic ops |
| 10 | 43 | OrderedSetMtEph.rs | TestOrderedSetMtEph.rs | Basic ops |

All 10 implementation files have corresponding RTT files. Coverage is basic (constructors, insert, find, delete, a few ordering ops). Advanced operations (get_key_range, split_rank, reduce_range_parallel) have limited or no RTT coverage.

## Phase 6: PTT Review

| # | Chap | File | PTT File | Patterns |
|---|------|------|----------|----------|
| 1 | 43 | OrderedTableStPer.rs | ProveOrderedTableStPer.rs | 4 iterator patterns (loop-borrow-iter, loop-borrow-into, for-borrow-iter, for-borrow-into) |

Only 1 PTT file exists for the entire chapter, covering iterator verification for OrderedTableStPer. No PTT files for the ordering operations, augmented tables, or set modules.

Recommended PTT additions: None critical. The existing PTT covers the most important callability concern (iterators). Ordering operations are mostly external_body so PTTs would not test proof obligations.

## Phase 7: Gap Analysis

### Proof Gaps

| # | Chap | File | Gap Type | Count | Description |
|---|------|------|----------|-------|-------------|
| 1 | 43 | OrderedTableStEph.rs | external_body | 12 | All ordering ops + map/filter/collect |
| 2 | 43 | OrderedTableStPer.rs | external_body | 10 | All ordering ops + collect |
| 3 | 43 | OrderedTableMtEph.rs | external_body | 7 | 4 ordering ops (first/last/previous/next) + filter + rank + select |
| 4 | 43 | OrderedTableMtPer.rs | external_body | 6 | 4 ordering ops + rank + select |
| 5 | 43 | OrderedTableMtPer.rs | assume | 2 | size (ghost view mismatch), from_st_table (wf) |
| 6 | 43 | AugOrderedTableStEph.rs | external_body | 10 | calculate_reduction + all ordering ops + map + clone |
| 7 | 43 | AugOrderedTableStPer.rs | external_body | 6 | first/last/previous/next + rank + select |
| 8 | 43 | AugOrderedTableStPer.rs | assume | 2 | reducer.requires in calculate_reduction + join_key |
| 9 | 43 | AugOrderedTableMtEph.rs | external_body | 8 | calculate_reduction + 4 ordering + rank + select + reduce_range_parallel |
| 10 | 43 | OrderedSetStEph.rs | external_body | 11 | All ordering ops + from_seq + to_seq(iter next) |
| 11 | 43 | OrderedSetStEph.rs | assume | 1 | to_seq clone/view bridging |
| 12 | 43 | OrderedSetStPer.rs | external_body | 9 | first/last/previous/next + split + rank + select + split_rank + iter next |
| 13 | 43 | OrderedSetMtEph.rs | external_body | 8 | first/last/previous/next + filter + to_seq + rank + select |
| 14 | 43 | OrderedSetMtEph.rs | assume | 7 | size, find, split(2x wf), get_range(wf), split_rank(2x wf) |

### Architectural Gap

The most significant gap is the backing store choice. APAS Cost Spec 43.2 requires O(log n) for all ADT 43.1 operations. The current implementations use:
- OrderedTable: linear-scan Table (O(n) for find/insert/delete, O(n log n) for ordering ops)
- OrderedSet: AVL tree (O(log n) for base ops, but ordering ops still use collect+scan)

To close this gap, the OrderedTable implementations would need to be reimplemented on top of AVL trees (like the OrderedSet variants) or a dedicated balanced BST that supports ordered operations natively.

### Verified Highlights

Notable verified functions (NOT external_body):
- OrderedTableMtEph: split_key, join_key, get_key_range, split_rank_key (4 ordering ops verified)
- OrderedSetStPer: get_range, split_rank (verified with full loop invariants and subset proofs)
- All base table ops (size, empty, singleton, find, insert, delete, domain, tabulate, reduce, intersection, union, difference, restrict, subtract) in StEph/StPer variants

## Phase 8: TOC Review

| # | Chap | File | TOC Present? | Sections Correct? |
|---|------|------|-------------|-------------------|
| 1 | 43 | OrderedTableStEph.rs | Yes | Mostly; sections 3/6/7 omitted correctly; 12 and 13 present |
| 2 | 43 | OrderedTableStPer.rs | Yes | Correct ordering |
| 3 | 43 | OrderedTableMtEph.rs | Yes | Correct ordering |
| 4 | 43 | OrderedTableMtPer.rs | Yes | Correct; includes section 11 (coarse locking variant) |
| 5 | 43 | AugOrderedTableStEph.rs | Yes | Includes section 7 for calculate_reduction; correct |
| 6 | 43 | AugOrderedTableStPer.rs | Yes | Includes section 7 for calculate_reduction; correct |
| 7 | 43 | AugOrderedTableMtEph.rs | Yes | Includes section 7 for calculate_reduction; correct |
| 8 | 43 | OrderedSetStEph.rs | Yes | Correct ordering |
| 9 | 43 | OrderedSetStPer.rs | Yes | Correct ordering |
| 10 | 43 | OrderedSetMtEph.rs | Yes | Minor: section 12 appears after 13 (should be before) |

One TOC ordering issue: OrderedSetMtEph.rs has section 13 (derive impls outside verus!) before section 12 (macros). Per standard, macros (12) should come before outside-verus impls (13).

## Summary

Chapter 43 implements the APAS Ordered Sets and Ordered Tables interface (ADT 43.1) plus Augmented Tables across 10 files with 4 variants each (StEph, StPer, MtEph, MtPer or subset thereof). The implementation is functionally complete -- all ADT 43.1 operations are present. However:

1. **99 proof holes** remain (12 assume + 87 external_body).
2. **Systematic cost deviation**: OrderedTable implementations use linear-scan backing stores instead of balanced BSTs, resulting in O(n) base ops and O(n log n) ordering ops vs. the O(log n) specified by APAS Cost Spec 43.2.
3. **Mt spec weakening**: Multi-threaded variants have systematically weaker ensures clauses (often just `dom().finite()`).
4. **Strong spec coverage** on single-threaded variants: StEph and StPer trait specs faithfully capture the APAS interface semantics including TotalOrder constraints.
5. **Full RTT coverage**: All 10 files have runtime tests.
6. **Limited PTT coverage**: 1 PTT file covering iterator patterns for OrderedTableStPer.
