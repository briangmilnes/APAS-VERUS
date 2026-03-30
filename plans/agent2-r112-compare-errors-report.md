# Agent 2 R112 Report — Fix 5 compare-par-mut errors

## Summary

Fixed all 5 `veracity-compare-par-mut` errors. Each was an MtEph function with
`ensures wf` where the StEph variant had real postconditions.

## Fixes

| # | Chap | File | Function | StEph ensures added to MtEph |
|---|------|------|----------|------------------------------|
| 1 | 43 | AugOrderedTableMtEph.rs | `singleton` | `tree@.dom().finite()` |
| 2 | 50 | MatrixChainMtEph.rs | `set_dimension` | `self@.dimensions =~= old(self)@.dimensions.update(index, dim)` |
| 3 | 50 | MatrixChainMtEph.rs | `update_dimension` | `self@.dimensions =~= old(self)@.dimensions.update(index, MatrixDim{rows,cols})` |
| 4 | 50 | OptBinSearchTreeMtEph.rs | `set_key_prob` | `self@.keys =~= old(self)@.keys.update(index, key_prob)` |
| 5 | 50 | OptBinSearchTreeMtEph.rs | `update_prob` | `self@.keys.len() == old(self)@.keys.len()` |

## Techniques

- **Chap43 singleton**: `spec_augorderedtablemteph_wf` already implies `dom().finite()`,
  so adding the ensures required no body changes.
- **Chap50 MatrixChain set/update_dimension**: Bodies already reconstruct Self with
  `ghost_dimensions: Ghost(new_ghost)` where `new_ghost = dims@` after `.set()`. The
  ensures follows directly from the reconstruction pattern.
- **Chap50 OBST set_key_prob**: Added `let ghost kp = key_prob` to capture the parameter
  before move, then `self.ghost_keys = Ghost(old_keys.update(index, kp))` after mutations
  to keep ghost state consistent with lock contents.
- **Chap50 OBST update_prob**: No body changes needed. Ghost_keys is unchanged, so
  `self@.keys.len() == old(self)@.keys.len()` holds automatically.

## Validation

```
verification results:: 5388 verified, 0 errors
RTT: 3197 tests run: 3197 passed, 0 skipped
veracity-compare-par-mut: Phase 2 summary: 57 groups compared, 0 errors
veracity-compare-par-mut: Phase 4 summary: 0 errors
```
