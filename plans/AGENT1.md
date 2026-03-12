# Agent 1 Report — PBOGH Round 4

## Changes

| # | Chap | File | Change | Result |
|---|------|------|--------|--------|
| 1 | 57 | DijkstraStEphI64.rs | Added `requires true` to `pq_entry_new` | Error cleared, Chap57 fully clean |
| 2 | 55 | SCCStEph.rs | Added `requires true` to `check_wf_adj_list_eph` | fn_missing_requires cleared |
| 3 | 55 | SCCStPer.rs | Added `requires true` to `check_wf_adj_list_per` | fn_missing_requires cleared |

## Errors Cleared

| # | Chap | File | Error Type | Resolution |
|---|------|------|-----------|------------|
| 1 | 57 | DijkstraStEphI64.rs | fn_missing_requires | Added `requires true` — no precondition needed |
| 2 | 55 | SCCStEph.rs | fn_missing_requires | Added `requires true` to `check_wf_adj_list_eph` |
| 3 | 55 | SCCStPer.rs | fn_missing_requires | Added `requires true` to `check_wf_adj_list_per` |

## Still Open (awaiting hole checker update for fn_missing_ensures)

| # | Chap | File | Error | Note |
|---|------|------|-------|------|
| 1 | 55 | SCCStEph.rs | fn_missing_ensures on `compute_finish_order` | `ensures true` is bogus; real ensures needs proof work |
| 2 | 55 | SCCStPer.rs | fn_missing_ensures on `compute_finish_order` | Same |

## Chapters Status

- **Chap57**: 3/3 modules clean, 0 holes, 0 errors. Fully clean.
- **Chap55**: 6/8 modules clean, 0 holes, 2 fn_missing_ensures errors remaining.

## Blockers

| # | Chap | File | Issue | Why Blocked |
|---|------|------|-------|-------------|
| 1 | 18 | ArraySeq.rs | `iter_mut` fn_missing_requires_ensures | `#[verifier::external]`; `IterMut` has no vstd spec. Awaiting Bryan Parno's prophetic iterator. |
| 2 | 52 | EdgeSetGraphMtPer.rs | `out_neighbors` external_body | `AVLTreeSetMtPer::filter` lacks `Ghost(spec_pred)` companion (Chap41 upstream API change needed). |
| 3 | 18 | All 6 files | trivial_spec_wf `{ true }` | Vec-backed types — `true` is correct, no structural invariant. |
| 4 | 19 | All 3 files | trivial_spec_wf `{ true }` | Same as Chap18. |
| 5 | 12 | Exercise12_5.rs | trivial_spec_wf `{ true }` | Lock-free stack — no provable wf beyond `true`. |

## Summary

- Verified: 3670, 0 errors
- Errors fixed: 3 (across 3 files)
- Chap57 now fully clean (was 1 error)
- Remaining: 10 trivial_spec_wf (unfixable), 2 fn_missing_ensures (awaiting checker update), 1 external_body, 1 external
