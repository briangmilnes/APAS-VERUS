# False Positive Hole Review Plan

Manual review of ~55 holes that agents declared structurally unprovable.
Goal: annotate confirmed false positives so the hole detector stops counting them.

## Category 1: Std Trait Methods (7 holes)

Can't add requires/ensures to std trait impls (Iterator::next, Ord::cmp, PartialOrd).

| # | Chap | Path | Function | Line |
|---|------|------|----------|------|
| 1 | 37 | src/Chap37/AVLTreeSeq.rs | `next` (Iterator) | 1118 |
| 2 | 41 | src/Chap41/AVLTreeSetMtPer.rs | `cmp` (Ord) | 581 |
| 3 | 43 | src/Chap43/OrderedSetStEph.rs | `next` (Iterator) | 623 |
| 4 | 43 | src/Chap43/OrderedSetStPer.rs | `next` (Iterator) | 647 |
| 5 | 43 | src/Chap43/OrderedTableMtEph.rs | `next` (Iterator) | 780 |
| 6 | 57 | src/Chap57/DijkstraStEphI64.rs | `cmp` (Ord) | 102 |
| 7 | 57 | src/Chap57/DijkstraStEphI64.rs | `partial_cmp` (PartialOrd) | 119 |

## Category 2: Thread-Spawn / 'static Closure Boundaries (33 holes)

external_body wraps thread spawn boundary, not algorithmic logic.

| # | Chap | Path | Function | Line |
|---|------|------|----------|------|
| 1 | 37 | src/Chap37/AVLTreeSeqMtPer.rs | `build_balanced_from_slice` | 510 |
| 2 | 37 | src/Chap37/AVLTreeSeqMtPer.rs | `subseq_copy` | 631 |
| 3 | 38 | src/Chap38/BSTParaMtEph.rs | `view` (View impl) | 82 |
| 4 | 38 | src/Chap38/BSTParaMtEph.rs | `split_inner` | 475 |
| 5 | 38 | src/Chap38/BSTParaMtEph.rs | `join_pair` | 363 |
| 6 | 38 | src/Chap38/BSTParaMtEph.rs | `union` | 368 |
| 7 | 38 | src/Chap38/BSTParaMtEph.rs | `intersect` | 373 |
| 8 | 38 | src/Chap38/BSTParaMtEph.rs | `difference` | 378 |
| 9 | 38 | src/Chap38/BSTParaMtEph.rs | `filter` | 383 |
| 10 | 38 | src/Chap38/BSTParaMtEph.rs | `reduce` | 393 |
| 11 | 38 | src/Chap38/BSTParaMtEph.rs | `in_order` | 398 |
| 12 | 39 | src/Chap39/BSTParaTreapMtEph.rs | `view` (View impl) | 69 |
| 13 | 39 | src/Chap39/BSTParaTreapMtEph.rs | `join_mid` | 548 |
| 14 | 39 | src/Chap39/BSTParaTreapMtEph.rs | `split` | 666 |
| 15 | 39 | src/Chap39/BSTParaTreapMtEph.rs | `join_pair` | 671 |
| 16 | 39 | src/Chap39/BSTParaTreapMtEph.rs | `union` | 676 |
| 17 | 39 | src/Chap39/BSTParaTreapMtEph.rs | `intersect` | 681 |
| 18 | 39 | src/Chap39/BSTParaTreapMtEph.rs | `difference` | 686 |
| 19 | 39 | src/Chap39/BSTParaTreapMtEph.rs | `filter` | 691 |
| 20 | 39 | src/Chap39/BSTParaTreapMtEph.rs | `reduce` | 700 |
| 21 | 39 | src/Chap39/BSTParaTreapMtEph.rs | `in_order` | 710 |
| 22 | 41 | src/Chap41/AVLTreeSetMtEph.rs | `to_seq` | 251 |
| 23 | 41 | src/Chap41/AVLTreeSetMtEph.rs | `filter` | 295 |
| 24 | 41 | src/Chap41/AVLTreeSetMtEph.rs | `intersection` | 348 |
| 25 | 41 | src/Chap41/AVLTreeSetMtEph.rs | `difference` | 413 |
| 26 | 41 | src/Chap41/AVLTreeSetMtEph.rs | `union` | 424 |
| 27 | 41 | src/Chap41/AVLTreeSetMtPer.rs | `from_seq` | 226 |
| 28 | 41 | src/Chap41/AVLTreeSetMtPer.rs | `filter` | 278 |
| 29 | 41 | src/Chap41/AVLTreeSetMtPer.rs | `intersection` | 341 |
| 30 | 41 | src/Chap41/AVLTreeSetMtPer.rs | `difference` | 399 |
| 31 | 41 | src/Chap41/AVLTreeSetMtPer.rs | `union` | 410 |
| 32 | 41 | src/Chap41/AVLTreeSetMtPer.rs | `delete` | 500 |
| 33 | 41 | src/Chap41/AVLTreeSetMtPer.rs | `insert` | 510 |

## Category 3: Eq/Clone Workaround (1 hole)

Assume inside expose for clone+cmp bridge. Approved pattern.

| # | Chap | Path | Function | Line |
|---|------|------|----------|------|
| 1 | 38 | src/Chap38/BSTParaStEph.rs | `expose` (assume) | 475 |

## Category 4: RwLock Ghost Tracking (8 holes)

Assumes bridging ghost state through RwLock. Predicate is immutable,
can't track mutable state changes.

| # | Chap | Path | Function | Line |
|---|------|------|----------|------|
| 1 | 39 | src/Chap39/BSTTreapMtEph.rs | `find` | 976 |
| 2 | 39 | src/Chap39/BSTTreapMtEph.rs | `size` | 992 |
| 3 | 39 | src/Chap39/BSTTreapMtEph.rs | `minimum` | 1017 |
| 4 | 39 | src/Chap39/BSTTreapMtEph.rs | `maximum` | 1027 |
| 5 | 39 | src/Chap39/BSTTreapMtEph.rs | `in_order` | 1037 |
| 6 | 39 | src/Chap39/BSTTreapMtEph.rs | `pre_order` | 1049 |
| 7 | 41 | src/Chap41/AVLTreeSetMtEph.rs | `size` | 241 |
| 8 | 41 | src/Chap41/AVLTreeSetMtEph.rs | `find` | 457 |

## Category 5: Unsafe Send/Sync Markers (2 holes)

Ghost field needs manual marker. Not proof obligations.

| # | Chap | Path | Struct | Line |
|---|------|------|--------|------|
| 1 | 41 | src/Chap41/AVLTreeSetMtEph.rs | `unsafe impl Send` | 622 |
| 2 | 41 | src/Chap41/AVLTreeSetMtEph.rs | `unsafe impl Sync` | 623 |

## Category 6: Opaque External Dependencies (2 holes)

Hash function dispatch through opaque Fn closures and std::hash.

| # | Chap | Path | Function | Line |
|---|------|------|----------|------|
| 1 | 47 | src/Chap47/ParaHashTableStEph.rs | `call_hash_fn` | 464 |
| 2 | 47 | src/Chap47/ParaHashTableStEph.rs | `compute_second_hash` | 494 |

## REMOVED — Not False Positives

### String literal functions (3 holes) — FIXABLE

Replace `Result<..., String>` with `Result<..., BellmanFordError>` enum.
No string allocation = no external_body needed.

| # | Chap | Path | Function | Line | Fix |
|---|------|------|----------|------|-----|
| 1 | 58 | src/Chap58/BellmanFordStEphI64.rs | `neg_cycle_error_string` | 85 | Enum variant |
| 2 | 58 | src/Chap58/BellmanFordStEphI64.rs | `algorithm_error_string` | 90 | Enum variant |
| 3 | 59 | src/Chap59/JohnsonStEphI64.rs | `neg_cycle_error_string` | 69 | Enum variant |

## Summary

- Confirmed false positives: 53 (categories 1-6)
- Fixable (not false positives): 3 (string→enum)
- Total current actionable: 196
- After suppressing false positives: ~143 real proof targets
