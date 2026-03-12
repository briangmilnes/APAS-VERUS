# Agent 3 Report — Round 4

## Assignment

Chap26/50/53/66 (DP + Graph Algorithms). Priority: Chap26 → Chap66 → Chap50 → Chap53.

## Results

| # | File | Before | After | Delta | Technique |
|---|------|--------|-------|-------|-----------|
| 1 | OBSTMtPer.rs | 12 | 3 | -9 | View + verify constructors/readers + accepts |
| 2 | OBSTMtEph.rs | 15 | 6 | -9 | Ghost field + View + verify constructors + accepts |
| 3 | MatrixChainMtEph.rs | 15 | 7 | -8 | Ghost field + View + verify constructors + accepts |
| 4 | MatrixChainMtPer.rs | 4 | 3 | -1 | arc_deref + lock read for memo_size |
| — | All 4 memo_size | +4 | 0 | -4 | arc_deref + lock read (bonus from Phase 4) |

Chap50 total: 48 → 21 holes (-27).

## Overall Numbers

| Metric | Round 3 | Round 4 | Delta |
|--------|---------|---------|-------|
| Verified | 3670 | 3700 | +30 |
| Total holes | 456 | 429 | -27 |
| external_body | 385 | 358 | -27 |

## Techniques Applied

1. **Ghost field pattern for MtEph**: Added `ghost_keys: Ghost<Seq<T>>` to structs with
   `Arc<RwLock<Vec<T>>>`. View projects through ghost field. Constructors set ghost from
   initial value. Clone copies ghost. Readers use `arc_deref` + lock read + reader accept.
   Writers kept as external_body (complex borrow interactions with ghost update).

2. **arc_deref + lock read**: New pattern proven to work through `Arc<RwLock<T>>`:
   `arc_deref(&self.field)` → `&RwLock<T>` → `acquire_read()` → `borrow().len()` →
   `release_read()`. Used for `num_keys`, `num_matrices`, and all 4 `memo_size` methods.

3. **Arc clone for clear_memo**: `self.memo.clone()` produces an owned Arc clone.
   Lock write through the clone avoids borrow conflict with `&mut self` ghost fields.
   Works because Arc clone releases the borrow on self immediately.

4. **Standard accept patterns**: KeyProb::clone, OBSTMtPerS/OBSTMtEphS/MatrixChainMtEphS
   clone, PartialEq::eq — all using lock-boundary accept pattern from standards.

## Permanent Holes (not reducible)

| # | Chap | Holes | Reason |
|---|------|-------|--------|
| 1 | 26 | 4 | f64 sort/swap (ETSPStEph, ETSPMtEph) |
| 2 | 50 | 21 | parallel_min_reduction (fork-join), obst_rec/matrix_chain_rec (recursion + locks + f64/usize arithmetic), optimal_cost (delegates), lock-boundary writers |
| 3 | 53 | 11 | external_body graph search (complex generics + lock ops) |
| 4 | 66 | 3 | StdRng randomization + raw HashMap |

## Files Modified

- `src/Chap50/OptBinSearchTreeMtPer.rs`
- `src/Chap50/OptBinSearchTreeMtEph.rs`
- `src/Chap50/MatrixChainMtEph.rs`
- `src/Chap50/MatrixChainMtPer.rs`

## Verification

- `scripts/validate.sh`: 3700 verified, 0 errors
- `scripts/rtt.sh`: 2600 tests passed
- `scripts/holes.sh src/Chap50/`: 21 holes (down from 48)
- No trigger warnings in validate output
