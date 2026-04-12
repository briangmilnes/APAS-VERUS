# Agent 2 — Round 199 Report

## Summary

Added full 10-component iterators to 15+ collection types across Chap18–Chap42.
All three phases complete: iterator implementations verified (5728), RTTs passing (4185),
PTTs passing (237).

## Holes Before/After

No proof holes were added or removed this round. The iterator work adds new spec
functions and exec code; existing holes were not touched.

## Phase 1+2: Iterator Implementations (5728 verified, 0 errors)

| # | Chap | File | Bounds | `into_iter` requires | Notes |
|---|------|------|--------|---------------------|-------|
| 1 | 18 | ArraySeq.rs | `T: StT` | none | snapshot-based |
| 2 | 18 | ArraySeqStPer.rs | `T: StT` | none | snapshot-based |
| 3 | 18 | LinkedListStEph.rs | `T: StT` | none | snapshot-based |
| 4 | 18 | LinkedListStPer.rs | `T: StT` | none | snapshot-based |
| 5 | 37 | BSTSplayStEph.rs | `T: TotalOrder + Clone` | none | snapshot via `in_order()` |
| 6 | 38 | BSTParaStEph.rs | `T: StT + Ord` | none | snapshot via `in_order()` |
| 7 | 39 | BSTTreapStEph.rs | `T: StT + Ord + IsLtTransitive` | none | snapshot |
| 8 | 39 | BSTTreapMtEph.rs | `T: StTInMtT + Ord + IsLtTransitive` | none | snapshot |
| 9 | 39 | BSTSetTreapMtEph.rs | `T: MtKey + ClonePreservesView` | `obeys_cmp_spec + view_ord_consistent` | forwarded to `iter_in_order()` |
| 10 | 39 | BSTParaTreapMtEph.rs | `T: MtKey + ClonePreservesView` | `obeys_cmp_spec + view_ord_consistent` | fixed SMT instability in `join_with_priority` |
| 11 | 40 | BSTSizeStEph.rs | `T: StT + Ord` | none | snapshot |
| 12 | 40 | BSTKeyValueStEph.rs | `K: StT+Ord+TotalOrder, V: StT` | `spec_bstkeyvaluesteph_wf()` | iterates keys K; PhantomData<V> |
| 13 | 40 | BSTReducedStEph.rs | `K: StT+Ord, V: StT, R: StT, Op: ReduceOp<V,R>` | `spec_bstreducedsteph_wf()` | iterates keys K; PhantomData<(V,R,Op)> |
| 14 | 41 | ArraySetStEph.rs | `T: StT + Ord` | none | renamed `iter_invariant` → `iter_invariant_arraysetseph` |
| 15 | 41 | AVLTreeSetStEph.rs | `T: StT + Ord + TotalOrder` | none | snapshot |
| 16 | 41 | AVLTreeSetStPer.rs | `T: StT + Ord + TotalOrder` | none | snapshot |
| 17 | 41 | AVLTreeSetMtPer.rs | `T: StTInMtT + Ord + TotalOrder` | none | snapshot |
| 18 | 42 | TableStEph.rs | `K: StT+Ord, V: StT` | none | yields `&'a Pair<K,V>` |
| 19 | 42 | TableStPer.rs | `K: StT+Ord, V: StT` | none | yields `&'a Pair<K,V>` |

### Key Fixes During Phase 1+2

**E0659 ambiguity fix**: `ArraySetStEph::iter_invariant` renamed to
`iter_invariant_arraysetseph` — 6 call sites in Chap43 `AugOrdered*` files were
importing two glob modules both exporting `iter_invariant`.

**SMT instability fix** in `BSTParaTreapMtEph::join_with_priority`: adding
`iter_invariant_paramtreap` spec fn changed Z3 quantifier priorities, breaking an
existing proof. Fixed by adding explicit bridge assertion:
```rust
assert(forall|t: T| #[trigger] lrv.contains(t@) ==> left@.contains(t@));
```

## Phase 3: RTTs (4185 passed, 0 failed)

Added 3 iterator tests per module (collect-and-sort, empty, for-loop-sum).
Covered all 15 original targets plus Phase 1c Chap41/42 modules.

| # | Chap | File | Tests added |
|---|------|------|-------------|
| 1 | 41 | TestAVLTreeSetMtPer.rs | 4 |
| 2 | 39 | TestBSTTreapMtEph.rs | 3 |
| 3 | 37 | TestBSTSplayStEph.rs | 3 |
| 4 | 38 | TestBSTParaStEph.rs | 3 |
| 5 | 39 | TestBSTTreapStEph.rs | 3 |
| 6 | 40 | TestBSTSizeStEph.rs | 3 |
| 7 | 39 | TestBSTSetTreapMtEph.rs | 3 |
| 8 | 39 | TestBSTParaTreapMtEph.rs | 3 |
| 9 | 40 | TestBSTKeyValueStEph.rs | 3 |
| 10 | 40 | TestBSTReducedStEph.rs | 3 |
| 11 | 41 | TestArraySetStEph.rs | 3 |
| 12 | 41 | TestAVLTreeSetStEph.rs | 3 |
| 13 | 41 | TestAVLTreeSetStPer.rs | 3 |
| 14 | 42 | TestTableStEph.rs | 3 |
| 15 | 42 | TestTableStPer.rs | 3 |

RTT total: 4185 passed (up from ~4138 before this round).

## Phase 4: PTTs (237 passed, 0 failed)

New PTT files:

| # | Chap | File | Patterns |
|---|------|------|----------|
| 1 | 38 | ProveParamBSTStEph.rs | loop-iter, loop-into, for-iter, for-into |
| 2 | 39 | ProveBSTTreapMtEph.rs | loop-into, for-into (no .iter() method) |
| 3 | 41 | ProveAVLTreeSetStEph.rs | loop-iter, loop-into, for-iter, for-into |
| 4 | 41 | ProveAVLTreeSetStPer.rs | loop-into, for-into (no .iter() method) |

Note: `BSTTreapMtEphLit!` uses `HashMap` which requires `hash::Hash` — unsupported
in the Verus PTT framework. PTTs use `BSTTreapMtEph::new() + insert()` instead.

## Commits

```
4842fa665 R199 agent2: add full 10-component iterators to 15 collection types (5728 verified)
ffe9727e3 R199 agent2: iterator RTTs for 15 collection types (4185 passed)
e61f6d4cb R199 agent2: iterator PTTs for 5 new collection types (237 passed)
```

Branch pushed: `agent2/ready` at `e61f6d4cb`.
