# HFScheduler Migration Plan

**Date**: 2026-03-06
**Goal**: Migrate all Mt modules using raw `std::thread::spawn` / `std::thread::scope`
to use HFScheduler `join()` or `ParaPair!` per the project standard.

---

## Audit Summary

| Category | Count | Action |
|---|---|---|
| Raw `std::thread::spawn/scope` (need migration) | 16 files | Migrate to HFScheduler |
| `vstd::thread::*` TSM demos (intentional) | 3 files | Keep as-is |
| Unused `use std::thread` import (dead code) | 4 files | Remove import |
| Already compliant (HFScheduler/ParaPair!) | ~28 files | No action |

---

## Category A: Unused `use std::thread` Imports (Quick Cleanup)

Remove the dead import from these 4 files. No functional change.

| # | Chap | File | Notes |
|---|---|---|---|
| 1 | 43 | OrderedTableMtEph.rs | No thread usage at all |
| 2 | 43 | AugOrderedTableMtEph.rs | Already uses ParaPair! |
| 3 | 52 | AdjSeqGraphMtEph.rs | No thread usage at all |
| 4 | 59 | JohnsonMtEphI64.rs | Already uses ParaPair! |

**Effort**: Trivial. 4 one-line edits.

---

## Category B: TSM Demos (Keep As-Is)

These 3 Chap11 files intentionally use `vstd::thread::*` with Tokenized State Machines.
They exist to demonstrate the TSM parallelism pattern. Chap11 already has
`FibonacciMtPerAllThreads.rs` (ParaPair!) and `FibonacciHFScheduler.rs` (Chap02) as
HFScheduler exemplars.

| # | Chap | File | Pattern |
|---|---|---|---|
| 1 | 11 | FibonacciMtEph2Threads.rs | TSM + vstd::thread, 2-thread split |
| 2 | 11 | FibonacciMtEphRecomputes.rs | TSM + vstd::thread, recursive |
| 3 | 11 | FibonacciMtPerTSM.rs | TSM + vstd::thread, recursive |

**Action**: None. These are intentional pattern demonstrations.

---

## Category C: Binary Fork-Join Migrations (Easy)

Files that spawn exactly 2 threads and join both. Direct replacement:
`thread::spawn(f1)` + `thread::spawn(f2)` + `.join()` + `.join()`
becomes `let (a, b) = join(f1, f2);` or `let Pair(a, b) = ParaPair!(f1, f2);`.

All of these are currently `external_body`. Migration preserves `external_body` status
but switches to the standard threading mechanism.

| # | Chap | File | Spawns | Notes |
|---|---|---|---|---|
| 1 | 49 | MinEditDistMtEph.rs | 2 | Recursive DP, 2-way split |
| 2 | 49 | MinEditDistMtPer.rs | 2 | Recursive DP, 2-way split |
| 3 | 49 | SubsetSumMtEph.rs | 2 | Recursive DP, 2-way split |
| 4 | 49 | SubsetSumMtPer.rs | 2 | Recursive DP, 2-way split |
| 5 | 50 | OptBinSearchTreeMtEph.rs | 2 | Parallel min reduction |
| 6 | 50 | OptBinSearchTreeMtPer.rs | 2 | Parallel min reduction |
| 7 | 50 | MatrixChainMtEph.rs | 2 | Parallel min reduction |
| 8 | 50 | MatrixChainMtPer.rs | 2 | Parallel min reduction |
| 9 | 51 | TopDownDPMtEph.rs | 2 | Recursive DP, 2-way split |
| 10 | 51 | TopDownDPMtPer.rs | 2 | Recursive DP, 2-way split |

**Pattern**: In each file, replace:
```rust
let handle1 = thread::spawn(move || { ... });
let handle2 = thread::spawn(move || { ... });
let r1 = handle1.join().unwrap();
let r2 = handle2.join().unwrap();
```
with:
```rust
let f1 = move || { ... };
let f2 = move || { ... };
let (r1, r2) = join(f1, f2);
```

**Imports**: Add `use crate::Chap02::HFSchedulerMtEph::HFSchedulerMtEph::join;`,
remove `use std::thread;`.

**Effort**: ~10 minutes per file. Mechanical transformation.

---

## Category D: Thread-Scope Migrations (Medium)

Files using `std::thread::scope` which allows borrowing into spawned threads.
HFScheduler `join()` requires `Send + 'static`, so captured data must be
owned or `Arc`-wrapped. May need `clone()` or `Arc` wrapping.

| # | Chap | File | Scope Usage | Notes |
|---|---|---|---|---|
| 1 | 26 | ETSPMtEph.rs | 1 scope call | `find_best_swap_parallel` — recursive binary split |
| 2 | 37 | AVLTreeSeqMtPer.rs | 1 scope call | `par_to_vec` — parallel tree-to-vec |

**ETSPMtEph.rs**: Already uses HFScheduler `join` for other operations. The `find_best_swap_parallel`
uses `thread::scope` for a recursive binary split over a tour slice. Convert to `join()` with
owned data (clone the slices or pass indices).

**AVLTreeSeqMtPer.rs**: Uses `thread::scope` with `Arc<Mutex<Option<T>>>` array.
Heavier refactor — may need to restructure as recursive binary `join()` instead of
flat parallel-for.

**Effort**: ~30-60 minutes per file. Requires reasoning about ownership.

---

## Category E: Multi-Spawn / Loop-Spawn Migrations (Hard)

Files that spawn threads inside loops or use many spawn points throughout the impl.

| # | Chap | File | Spawns | Notes |
|---|---|---|---|---|
| 1 | 42 | TableMtEph.rs | 8 | Every table op spawns; biggest migration |
| 2 | 51 | BottomUpDPMtEph.rs | 1 (in loop) | Diagonal pebbling: spawn per diagonal cell |
| 3 | 51 | BottomUpDPMtPer.rs | 1 (in loop) | Same pattern as Eph |
| 4 | 64 | SpanTreeMtEph.rs | 3 | BFS-like traversal, Arc shared state |

**TableMtEph.rs**: 8 spawn sites across `tabulate`, `inject`, `reduce`, `filter`, `find`,
`combine`, `extract`, `restrict`. Each does a binary split of entries, spawns one half,
runs the other in the current thread, then joins. This is the canonical pattern for
`join(f1, f2)`. The migration is mechanical but touches many functions.

**BottomUpDP{Mt}{Eph,Per}**: Spawns threads in a loop for diagonal pebbling. Each diagonal
has independent cells that can run in parallel. This doesn't fit binary `join()` directly.
Options: (a) nest binary `join()` calls in a recursive tree, (b) use sequential fallback
for now and parallelize later with a proper parallel-for.

**SpanTreeMtEph.rs**: Uses `Arc` shared state (`spanning_edges`, `valid`) with multiple
`thread::spawn` calls. The structure is: spawn a thread for one child, run the other in
current thread, join, then spawn another thread. Needs restructuring into nested `join()`.

**Effort**: TableMtEph ~2 hours (mechanical but many sites). BottomUpDP ~1 hour each
(design decision needed). SpanTreeMtEph ~1 hour (restructure required).

---

## Execution Order

1. **Phase 1 — Quick cleanup**: Remove 4 unused `use std::thread` imports (Category A).
2. **Phase 2 — Easy migrations**: Convert 10 binary fork-join files (Category C).
   Group by chapter: Chap49 (4 files), Chap50 (4 files), Chap51/TopDown (2 files).
3. **Phase 3 — Medium migrations**: Convert 2 thread-scope files (Category D).
4. **Phase 4 — Hard migrations**: Convert 4 multi-spawn files (Category E).
   TableMtEph first (most impactful), then BottomUpDP, then SpanTree.

After each phase: `scripts/validate.sh`, `scripts/rtt.sh`, commit.

---

## Compliant Files (Reference)

These 28 files already use HFScheduler `join()` or `ParaPair!`:

**Direct HFScheduler join()** (14):
Chap02/FibonacciHFScheduler, Chap05/SetMtEph, Chap18/ArraySeqMtEph,
Chap18/ArraySeqMtPer, Chap19/ArraySeqMtEph, Chap26/ETSPMtEph (partially),
Chap26/MergeSortMtPer, Chap26/ScanDCMtPer, Chap27/ReduceContractMtEph,
Chap35/OrderStatSelectMtEph, Chap35/OrderStatSelectMtPer,
Chap52/EdgeSetGraphMtPer, Chap54/BFSMtEph, Chap54/BFSMtPer.

**ParaPair! macro** (14+):
Chap06/{Dir,LabDir,LabUnDir,UnDir}GraphMtEph, Chap11/FibonacciMtPerAllThreads,
Chap36/QuickSort{MtEph,MtEphSlice}, Chap37/{AVLTreeSeqMtPer(partial),BSTRBMtEph,
BSTSetAVL/Plain/RB/SplayMtEph, BSTSplayMtEph}, Chap38/BSTParaMtEph,
Chap39/BSTParaTreapMtEph, Chap41/AVLTreeSetMtEph, Chap43/AugOrderedTableMtEph,
Chap59/JohnsonMtEphI64, Chap61/{EdgeContraction,VertexMatching}MtEph,
Chap62/StarContractionMtEph, Chap63/ConnectivityMtEph, Chap66/BoruvkaMtEph.
