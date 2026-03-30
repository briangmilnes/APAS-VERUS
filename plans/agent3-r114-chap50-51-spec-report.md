# R114 Agent 3 — Chap50 + Chap51 Spec Strengthening Report

## Summary

Strengthened MtEph/MtPer trait specs in Chap50 to match StEph/StPer counterparts.
Chap51 Mt traits already matched their St counterparts — no changes needed.

## Changes Made

### Chap50 — 4 trait functions strengthened

| # | Chap | File | Function | Spec Added |
|---|------|------|----------|------------|
| 1 | 50 | MatrixChainMtEph.rs | `dimensions` | `requires wf, ensures dims@ =~= self@.dimensions` |
| 2 | 50 | MatrixChainMtPer.rs | `dimensions` | `ensures dims@ =~= self@.dimensions` |
| 3 | 50 | OptBinSearchTreeMtEph.rs | `keys` | `requires wf, ensures keys@.len() == self@.keys.len()` |
| 4 | 50 | OptBinSearchTreeMtPer.rs | `keys` | `ensures keys@ =~= self@.keys` |

### Chap50 — Remaining [12] warnings (not actionable)

| # | Chap | File | Function | Reason |
|---|------|------|----------|--------|
| 1 | 50 | MatrixChainMtEph.rs | `memo_size` | View type has no `memo` field |
| 2 | 50 | MatrixChainMtPer.rs | `memo_size` | View type has no `memo` field |
| 3 | 50 | OptBinSearchTreeMtEph.rs | `memo_size` | View type has no `memo` field |
| 4 | 50 | OptBinSearchTreeMtPer.rs | `memo_size` | View type has no `memo` field |
| 5 | 50 | OptBinSearchTreeMtPer.rs | `optimal_cost` | StPer also has no spec |
| 6 | 50 | OptBinSearchTreeStEph.rs | `optimal_cost` | St-side gap (not Mt-vs-St) |
| 7 | 50 | OptBinSearchTreeStPer.rs | `optimal_cost` | St-side gap (not Mt-vs-St) |

### Chap51 — No changes needed

All 8 Chap51 files (TopDownDP{St,Mt}{Eph,Per}, BottomUpDP{St,Mt}{Eph,Per}) already
have matching specs between Mt and St for all implemented functions. The only
differences are missing functions in Mt versions (memo_size, is_memoized,
get_memoized, insert_memo, clear_memo, initialize_base_cases, compute_cell_value)
which are not in scope.

## Notes

- OBSTMtEph `keys` ensures only `keys@.len() == self@.keys.len()` (not content
  equality) because the lock invariant `OptBSTMtEphKeysInv` only tracks length,
  not content. The StEph counterpart provides `keys@ =~= self@.keys` because it
  directly owns the Vec.
- MtPer `dimensions`/`keys` return `&Arc<T>` so the ensures is trivially provable
  from the view definition. No wf requires needed.
- MtEph `dimensions`/`keys` go through RwLock acquire_read, so they need
  `requires wf()` and proof assertions in the impl body.

## Verification

- Full crate: 5388 verified, 0 errors
- RTT: 3529 passed, 0 skipped
- [12] warnings: 11 → 7 (−4)
