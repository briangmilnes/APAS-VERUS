# R138 Agent 4 Report: Merge TotalOrderBridge into TotalOrder

## Summary

Merged the `TotalOrderBridge` trait into `TotalOrder` by adding two bridge proof
methods (`cmp_spec_less_implies_le`, `cmp_spec_greater_implies_le`) directly to the
`TotalOrder` trait. The separate `TotalOrderBridge` trait was removed (commented out).

## What Changed

### `src/vstdplus/total_order.rs`
- Added `cmp_spec_less_implies_le` and `cmp_spec_greater_implies_le` to the `TotalOrder`
  trait with `where Self: Ord` method-level bounds (avoids ambiguity between
  `PartialOrd::le` and `TotalOrder::le` that a supertrait would cause).
- Added empty proof bodies for all 12 numeric type impls (u8..isize) — Z3 proves these.
- Added assume-based proof bodies for String impl (opaque `cmp_spec`).
- Commented out the `TotalOrderBridge` trait and all 14 impls.

### Chap43 (7 files)
- Removed `use ...TotalOrderBridge` import from all 7 files.
- Replaced all 118 `TotalOrderBridge` bound references with `TotalOrder`.

| # | Chap | File | Replacements |
|---|------|------|-------------|
| 1 | 43 | OrderedTableStEph.rs | 22 |
| 2 | 43 | OrderedTableStPer.rs | 22 |
| 3 | 43 | OrderedTableMtEph.rs | 12 |
| 4 | 43 | OrderedTableMtPer.rs | 12 |
| 5 | 43 | AugOrderedTableStEph.rs | 12 |
| 6 | 43 | AugOrderedTableStPer.rs | 12 |
| 7 | 43 | AugOrderedTableMtEph.rs | 14 |
| 8 | 43 | OrderedSetStEph.rs | import only |
| 9 | 43 | OrderedSetStPer.rs | import only |
| 10 | 43 | OrderedSetMtEph.rs | import only |

### Chap52
- `AdjTableGraphMtPer.rs` — replaced 2 fully-qualified `TotalOrderBridge` references.

### Chap57
- `DijkstraStEphU64.rs` — added bridge lemma impls to PQEntry's TotalOrder impl
  (assume-based, Ord::cmp is external_body so cmp_spec is opaque).
- `DijkstraStEphF64.rs` — added bridge lemma impls to PQEntry's TotalOrder impl
  (external_body, consistent with existing pattern).

## Design Decision: `where Self: Ord` vs Supertrait

Adding `Ord` as a supertrait of `TotalOrder` caused 199 ambiguity errors across the
codebase (`T::le(...)` resolves to both `PartialOrd::le` and `TotalOrder::le`). Using
method-level `where Self: Ord` bounds on just the two bridge lemmas avoids this ripple
while still requiring `Ord` only when the bridge lemmas are called.

## Validation

- Full verify: 5584 verified, 0 errors
- RTT: 3616 passed, 0 skipped
- Zero references to `TotalOrderBridge` remain in source code (only in comments).
