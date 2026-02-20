# Planned Work (All Chapters)

Proof holes run per chapter. Priority: P1 (critical) > P2 (struct/Clone) > P3 (external_body) > P4 (dummy RwLockPred) > P5 (informational).

## Chapter summary

| # | Chapter | Clean | Holed | Holes | Errors |
|---|---------|:-----:|:-----:|-------|--------|
| 1 | Chap02 | 1 | 1 | 8 | 2 |
| 2 | Chap03 | 1 | 0 | 0 | 0 |
| 3 | Chap05 | 5 | 0 | 0 | 3 |
| 4 | Chap06 | 20 | 0 | 0 | 0 |
| 5 | Chap11 | 2 | 3 | 6 | 0 |
| 6 | Chap12 | 1 | 2 | 18 | 0 |
| 7 | Chap17 | 1 | 0 | 0 | 2 |
| 8 | Chap18 | 6 | 1 | 2 | 9 |
| 9 | Chap19 | 4 | 0 | 0 | 3 |
| 10 | Chap21 | 12 | 0 | 0 | 0 |
| 11 | Chap23 | 2 | 0 | 0 | 9 |
| 12 | Chap26 | 6 | 2 | 8 | 2 |
| 13 | Chap27 | 4 | 0 | 0 | 0 |
| 14 | Chap28 | 11 | 0 | 0 | 0 |
| 15 | Chap35 | 2 | 2 | 2 | 0 |
| 16 | Chap36 | 3 | 0 | 0 | 1 |
| 17 | Chap37 | 12 | 7 | 90 | 19 |
| 18 | Chap38 | 2 | 0 | 0 | 3 |
| 19 | Chap39 | 1 | 3 | 26 | 7 |
| 20 | Chap40 | 0 | 3 | 30 | 0 |
| 21 | Chap41 | 1 | 6 | 139 | 7 |
| 22 | Chap42 | 1 | 3 | 30 | 0 |
| 23 | Chap43 | 1 | 10 | 140 | 0 |
| 24 | Chap44 | 2 | 0 | 0 | 4 |
| 25 | Chap45 | 7 | 0 | 0 | 14 |
| 26 | Chap47 | 9 | 0 | 0 | 10 |
| 27 | Chap49 | 8 | 0 | 0 | 24 |
| 28 | Chap50 | 3 | 6 | 22 | 40 |
| 29 | Chap51 | 8 | 0 | 0 | 16 |
| 30 | Chap52 | 12 | 2 | 2 | 0 |
| 31 | Chap53 | 0 | 5 | 20 | 0 |
| 32 | Chap54 | 4 | 0 | 0 | 0 |
| 33 | Chap55 | 8 | 0 | 0 | 0 |
| 34 | Chap56 | 8 | 4 | 7 | 0 |
| 35 | Chap57 | 2 | 1 | 1 | 2 |
| 36 | Chap58 | 2 | 0 | 0 | 0 |
| 37 | Chap59 | 4 | 0 | 0 | 0 |
| 38 | Chap61 | 4 | 0 | 0 | 0 |
| 39 | Chap62 | 4 | 0 | 0 | 0 |
| 40 | Chap63 | 2 | 0 | 0 | 0 |
| 41 | Chap64 | 3 | 0 | 0 | 4 |
| 42 | Chap65 | 3 | 0 | 0 | 3 |
| 43 | Chap66 | 2 | 0 | 0 | 4 |

## Chapters fully clean (no holes, no errors)

Chap03, Chap06, Chap21, Chap27, Chap28, Chap54, Chap55, Chap58, Chap59, Chap61, Chap62, Chap63

## Planned work table (by severity)

| # | Sev | Chapter | File | Issue | Blocker |
|---|-----|---------|------|-------|---------|
| 1 | P5 | Chap02 | HFSchedulerMtEph | assume(false)+diverge (thread join) | Acceptable |
| 2 | P2 | Chap02 | HFSchedulerMtEph | PoolState, TaskState out; 6 ext_body (threading) | Mutex/Condvar |
| 3 | P5 | Chap11 | FibonacciMt* | assume(false)+diverge (thread join) | Acceptable |
| 4 | P3 | Chap12 | Exercise12_1 | SpinLock 6 ext_body | — |
| 5 | P3 | Chap12 | Exercise12_5 | ConcurrentStackMt 7 ext_body, 4 unsafe | — |
| 6 | P1 | Chap26 | ETSPMtEph, ETSPStEph | 2 assume() each (mod identity) | Z3 rlimit |
| 7 | P3 | Chap26 | ETSPMtEph, ETSPStEph | sort_and_split, find_best_swap ext_body | Vec/f64 |
| 8 | P3 | Chap18 | ArraySeq | 2 external (impl ArraySeqS, IntoIterator &mut) | Verus &mut |
| 9 | P3 | Chap35 | OrderStatSelect* | 2 ext_body | — |
| 10 | P3 | Chap37 | 7 modules | 90 holes (37 assume, 51 ext_body) | Graph types |
| 11 | P3 | Chap39 | 3 modules | 26 holes | — |
| 12 | P3 | Chap40 | 3 modules | 30 holes | — |
| 13 | P3 | Chap41 | 6 modules | 139 holes | — |
| 14 | P3 | Chap42 | 3 modules | 30 ext_body | — |
| 15 | P3 | Chap43 | 10 modules | 140 ext_body | — |
| 16 | P2 | Chap50 | MatrixChain*, OptBinSearchTree* | struct out, Ex* | HashMap |
| 17 | P3 | Chap50 | Probability | 15 ext_body | f64 axioms |
| 18 | P4 | Chap50 | MatrixChainMt*, OptBinSearchTreeMt* | dummy RwLockPred | — |
| 19 | P2 | Chap51 | TopDownDP* | TopDownDP*S out, Clone derived | HashMap |
| 20 | P4 | Chap51 | BottomUpDPMt*, TopDownDPMt* | dummy RwLockPred | — |
| 21 | P3 | Chap52 | AdjTableGraphMtPer, EdgeSetGraphMtPer | 2 ext_body | — |
| 22 | P3 | Chap53 | GraphSearch*, PQMin* | 20 ext_body | — |
| 23 | P3 | Chap56 | PathWeightUtils*, Example56_* | 7 ext_body | f64 |
| 24 | P3 | Chap57 | DijkstraStEphI64 | 1 ext_body | — |
| 25 | P2 | Chap57 | DijkstraStEphF64 | PQEntry out | F64Dist |
| 26 | P4 | Chap64 | SpanTreeMtEph | dummy RwLockPred | — |
| 27 | P2 | Chap65 | UnionFindStEph, PrimStEph | struct out | HashMap, ordered_float |
| 28 | P2 | Chap66 | Boruvka* | LabeledEdge out | ordered_float |

## Hole type legend

| Type | Description |
|------|-------------|
| assume(false)+diverge | Thread join — acceptable per assume-false-diverge rule |
| assume() | Non-thread assume — needs proof |
| external_body | Unverified body — needs verusification or justified |
| external_type_specification | External type spec — bridge to unverifiable types |
| struct/enum outside verus! | Type should move into verus! |
| dummy RwLockPredicate | Placeholder predicate for RwLock — low priority |

## Blockers

| Blocker | Affected |
|---------|----------|
| HashMap | MatrixChain*S, OptBinSearchTree*, TopDownDP*, UnionFindStEph |
| ordered_float | Example56_*, PrimStEph, Boruvka* |
| rand | BoruvkaMtEph |
| F64Dist | DijkstraStEphF64 PQEntry |
| Mutex/Condvar | Chap02 HFScheduler PoolState, TaskState |
| Vec/f64 | Chap26 ETSP sort_and_split, find_best_swap |
