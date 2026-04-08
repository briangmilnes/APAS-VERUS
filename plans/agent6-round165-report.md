# Agent 6 — Round 165 Report

## Task
Extract shared/duplicated spec functions and proof lemmas from Chap18, Chap19, and Chap43
into dedicated shared modules, following the `src/Chap42/TableSpecsAndLemmas.rs` pattern.

## Files Created

| # | Chap | File | Lines |
|---|------|------|-------|
| 1 | 18 | `ArraySeqSpecsAndLemmas.rs` | 112 |
| 2 | 19 | `ArraySeqSpecsAndLemmas.rs` | 112 |
| 3 | 43 | `OrderedSpecsAndLemmas.rs` | 308 |

All three registered as first entries in their chapter blocks in `src/lib.rs`.

## Files Modified

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 18 | `ArraySeq.rs` | `pub use ArraySeqSpecsAndLemmas::*`; removed spec_iterate, spec_inject |
| 2 | 18 | `ArraySeqStEph.rs` | removed spec_iterate, spec_inject |
| 3 | 18 | `ArraySeqStPer.rs` | removed spec_iterate, spec_inject |
| 4 | 18 | `ArraySeqMtEph.rs` | removed Sections 6a and 7a (spec_iterate, spec_inject, spec_ninject, both lemmas) |
| 5 | 18 | `ArraySeqMtEphSlice.rs` | removed Section 7 inject/ninject content |
| 6 | 18 | `ArraySeqMtPer.rs` | removed spec_iterate, spec_inject |
| 7 | 18 | `LinkedListStEph.rs` | removed spec_iterate |
| 8 | 18 | `LinkedListStPer.rs` | removed spec_iterate |
| 9 | 19 | `ArraySeqStEph.rs` | removed spec_iterate, spec_inject |
| 10 | 19 | `ArraySeqStPer.rs` | removed spec_iterate, spec_inject |
| 11 | 19 | `ArraySeqMtEph.rs` | removed Sections 6 and 7 inject/ninject content |
| 12 | 19 | `ArraySeqMtEphSlice.rs` | removed inject/ninject spec and proof fns |
| 13 | 43 | `OrderedTableStEph.rs` | `pub use OrderedSpecsAndLemmas::*`; removed Sections 6 and 7 (15 fns) |
| 14 | 43 | `OrderedTableStPer.rs` | specific OrdKeyMap import; `pub use OrderedSpecsAndLemmas::*`; removed Sections 6 and 7 |

## Key Technical Decision — Chap43 OrdKeyMap Conflict

`Chap41/OrdKeyMap.rs` already defines the same four spec fns as `OrderedSpecsAndLemmas.rs`
(`spec_pair_set_to_map`, `spec_set_pair_view_generated`, `spec_key_unique_pairs_set`,
`spec_pair_key_determines_order`) plus `lemma_pair_set_to_map_dom_finite`.

`OrderedTableStPer.rs` originally used `use OrdKeyMap::*` (glob). Adding
`pub use OrderedSpecsAndLemmas::*` caused 33 E0659 ambiguity errors.

Fix: changed StPer's OrdKeyMap glob to a specific import
(`use OrdKeyMap::{OrdKeyMap, OrdKeyMapTrait}`) — same pattern StEph already used.
Then added `pub use OrderedSpecsAndLemmas::*` without conflict.

OrdKeyMap's lemmas are `proof fn` (not `pub`), so downstream importers of StPer
now correctly receive the public `pub(crate) proof fn` versions from OrderedSpecsAndLemmas.

## Verification

```
verification results:: 5715 verified, 0 errors
```

Full codebase clean. Isolate Chap43: 2775 verified, 0 errors.

## Net Lines Removed

−944 lines (−912 deletions net across all modified files).
