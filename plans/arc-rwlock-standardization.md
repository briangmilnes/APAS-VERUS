# Plan: Standardize Arc and RwLock Usage Across Modules

## Problem

`vstdplus/arc_rwlock.rs` provides two generic `external_body` functions —
`new_arc_rwlock` and `clone_arc_rwlock` — that preserve `pred()` through Arc
construction and cloning. These are the project's **only** sanctioned trust
boundary for Arc<RwLock>.

Only 4 production files use them. The remaining 16 files that combine Arc with
RwLock define their own local `external_body` constructors, duplicating the
trust surface unnecessarily.

## Current State

### Compliant files (use `new_arc_rwlock` / `clone_arc_rwlock`)

| # | Chap | File |
|---|------|------|
| 1 | 37 | BSTRBMtEph.rs |
| 2 | 37 | BSTSplayMtEph.rs |
| 3 | 39 | BSTTreapMtEph.rs |
| 4 | 41 | AVLTreeSetMtEph.rs |

### Non-compliant: local `external_body` Arc/RwLock constructors

These files define their own `external_body` functions that call `RwLock::new`
or `Arc::new(RwLock::new(...))` directly instead of using the standard bridge.

| # | Chap | File | Local external_body fn(s) |
|---|------|------|---------------------------|
| 1 | 38 | BSTParaStEph.rs | wraps `Arc::new(RwLock::new(...))` |
| 2 | 38 | BSTParaMtEph.rs | `new_bst_para_lock` |
| 3 | 39 | BSTParaTreapMtEph.rs | `new_treap_lock` |
| 4 | 49 | SubsetSumMtEph.rs | local Arc+RwLock constructor |
| 5 | 49 | SubsetSumMtPer.rs | local Arc+RwLock constructor |
| 6 | 49 | MinEditDistMtEph.rs | local Arc+RwLock constructor |
| 7 | 49 | MinEditDistMtPer.rs | local Arc+RwLock constructor |
| 8 | 50 | MatrixChainMtEph.rs | `new_mceph_dim_lock`, `new_mceph_memo_lock` |
| 9 | 50 | MatrixChainMtPer.rs | `new_mcper_memo_lock` |
| 10 | 50 | OptBinSearchTreeMtEph.rs | `new_obst_eph_keys_lock`, `new_obst_eph_memo_lock` |
| 11 | 50 | OptBinSearchTreeMtPer.rs | `new_obst_per_memo_lock` |
| 12 | 51 | BottomUpDPMtEph.rs | `new_bu_eph_lock` |
| 13 | 51 | BottomUpDPMtPer.rs | `new_bu_per_lock` |
| 14 | 51 | TopDownDPMtEph.rs | `new_td_eph_lock` |
| 15 | 51 | TopDownDPMtPer.rs | `new_td_per_lock` |
| 16 | 64 | SpanTreeMtEph.rs | `new_spanning_edges_lock`, `new_valid_lock` |

### Arc-only files (no RwLock — not in scope for this plan)

These files use `Arc` for persistent data structures without RwLock. They use
`external_body` to wrap `Arc::new(Node{...})` for tree node construction. This
is a separate pattern (persistent node sharing, not concurrency) and is out of
scope for this plan.

| # | Chap | File | Pattern |
|---|------|------|---------|
| 1 | 37 | AVLTreeSeqStPer.rs | Arc-wrapped persistent tree nodes |
| 2 | 37 | AVLTreeSeqMtPer.rs | Arc-wrapped persistent tree nodes |
| 3 | 41 | AVLTreeSetMtPer.rs | Arc-wrapped persistent tree nodes |

### RwLock-only files (no Arc — already compliant)

These files use `RwLock` directly without wrapping in Arc. They construct with
`RwLock::new(val, Ghost(pred))` inline — no `external_body` needed because
RwLock::new is a vstd verified function. These are already correct.

| # | Chap | File |
|---|------|------|
| 1 | 18 | ArraySeqMtEph.rs |
| 2 | 37 | BSTAVLMtEph.rs |
| 3 | 37 | BSTBBAlphaMtEph.rs |
| 4 | 37 | BSTPlainMtEph.rs |

### Other Arc uses (HFScheduler-based, no local locks)

These files import Arc but delegate lock management to HFSchedulerMtEph. They
pass `Arc<RwLock<...>>` to the scheduler but don't construct the Arc+RwLock
themselves (HFScheduler does). No fix needed.

| # | Chap | File |
|---|------|------|
| 1 | 11 | FibonacciMtEph2Threads.rs |
| 2 | 27 | ReduceContractMtEph.rs |
| 3 | 27 | ScanContractMtEph.rs |
| 4 | 42 | TableMtEph.rs |
| 5 | 43 | OrderedTableMtEph.rs |
| 6 | 43 | AugOrderedTableMtEph.rs |
| 7 | 61 | EdgeContractionMtEph.rs |
| 8 | 61 | VertexMatchingMtEph.rs |
| 9 | 62 | StarContractionMtEph.rs |
| 10 | 63 | ConnectivityMtEph.rs |
| 11 | 66 | BoruvkaMtEph.rs |

## Fix Plan

For each of the 16 non-compliant files:

### Step 1: Add import
```rust
use crate::vstdplus::arc_rwlock::arc_rwlock::{new_arc_rwlock, clone_arc_rwlock};
```

### Step 2: Delete local `external_body` constructor(s)
Remove the file-local `external_body` fn that wraps `RwLock::new(...)` or
`Arc::new(RwLock::new(...))`.

### Step 3: Replace call sites
Replace calls to the deleted local fn with `new_arc_rwlock(val, Ghost(pred))`
or `clone_arc_rwlock(&arc)` as appropriate.

**Subtlety for RwLock-only constructors**: Some files (Chap50, 51, 64) wrap
only `RwLock::new` in `external_body` without Arc. These need inspection:
- If the RwLock is later wrapped in Arc → use `new_arc_rwlock` directly.
- If the RwLock is used standalone (no Arc) → `RwLock::new` is a vstd
  verified function and needs no `external_body` at all. Remove the wrapper
  and call `RwLock::new` inline.

### Step 4: Validate
Run `scripts/validate.sh` after each file (or batch of related files within
the same chapter).

## Execution Order

Group by chapter to minimize context switching. Within each chapter, do
MtEph before MtPer (they often share patterns).

1. **Chap38**: BSTParaStEph.rs, BSTParaMtEph.rs
2. **Chap39**: BSTParaTreapMtEph.rs
3. **Chap49**: SubsetSumMtEph.rs, SubsetSumMtPer.rs, MinEditDistMtEph.rs, MinEditDistMtPer.rs
4. **Chap50**: MatrixChainMtEph.rs, MatrixChainMtPer.rs, OptBinSearchTreeMtEph.rs, OptBinSearchTreeMtPer.rs
5. **Chap51**: BottomUpDPMtEph.rs, BottomUpDPMtPer.rs, TopDownDPMtEph.rs, TopDownDPMtPer.rs
6. **Chap64**: SpanTreeMtEph.rs
7. Validate full codebase: `scripts/validate.sh`
8. Run RTT + PTT: `scripts/rtt.sh && scripts/ptt.sh`

## Risk

Low. The `ensures` on `new_arc_rwlock` match what every local `external_body`
constructor promises (`arc.pred() == pred`). The transformation is mechanical.
If any file's local constructor has *tighter* or *different* ensures than the
standard bridge, that file needs individual attention.

## Success Criteria

- Zero local `external_body` functions wrapping `RwLock::new` or
  `Arc::new(RwLock::new(...))` outside of `vstdplus/arc_rwlock.rs`.
- All 16 files import from `vstdplus::arc_rwlock::arc_rwlock`.
- `scripts/validate.sh` passes clean.
- `scripts/rtt.sh` and `scripts/ptt.sh` pass clean.
