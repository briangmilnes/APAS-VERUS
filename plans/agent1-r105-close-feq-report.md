# R105 Agent 1 Report: Close `obeys_feq_view_injective`

## Objective

Close `obeys_feq_view_injective` from `open` to `closed` spec fn to eliminate
888K quantifier instantiations leaking through broadcast into every module.

## Result

| Metric | Before | After | Delta |
|--------|--------|-------|-------|
| Chap52 total instantiations | 5,554,249 | 3,579,659 | -1,974,590 (-36%) |
| `obeys_feq_view_injective` instantiations | 888,000 | 0 | -888,000 |
| Verified count | 5,427 (before fix) | 5,427 | 0 |
| RTT | 3,083 | 3,083 | 0 |
| PTT | 157 | 157 | 0 |
| Errors | 0 | 0 | 0 |

## Changes

### Core change (1 file)

**`src/vstdplus/feq.rs`**: Changed `obeys_feq_view_injective` from `open` to `closed`.
Added `lemma_reveal_view_injective<T>()` proof fn that requires
`obeys_feq_view_injective::<T>()` and ensures the forall body, enabling callers
to reveal the quantifier only where needed.

### `hash_map_with_view_plus.rs`

Replaced the literal quantifier `forall|k1: Key, k2: Key| k1@ == k2@ ==> k1 == k2`
in `HashMapWithViewPlusTrait::new()` requires with `obeys_feq_view_injective::<Key>()`.

### Literal quantifier replacement (8 files)

Replaced all instances of `forall|k1: V, k2: V| k1@ == k2@ ==> k1 == k2` (and
`T` variant) in requires/invariants with `obeys_feq_view_injective::<V>()`:

| # | Chap | File | Instances |
|---|------|------|-----------|
| 1 | 17 | MathSeq.rs | 3 |
| 2 | 63 | ConnectivityMtEph.rs | 6 |
| 3 | 63 | ConnectivityStEph.rs | 4 |
| 4 | 65 | KruskalStEph.rs | 1 |
| 5 | 65 | UnionFindStEph.rs | 1 |
| 6 | 66 | BoruvkaMtEph.rs | 22 |
| 7 | 66 | BoruvkaStEph.rs | 9 |
| 8 | exp | simple_hash_set_iter.rs | 8 |

### `lemma_reveal_view_injective` calls (21 files)

Added `lemma_reveal_view_injective::<T>()` in proof blocks where the quantifier
body is actually needed:

| # | Chap | File | Calls | Type param |
|---|------|------|-------|------------|
| 1 | 05 | SetStEph.rs | 2 | `SetStEph<T>`, `T` |
| 2 | 05 | SetMtEph.rs | 2 | `SetMtEph<T>`, `T` |
| 3 | 06 | WeightedDirGraphStEph*.rs (12 files) | 1 each | `LabEdge<V, TYPE>` |
| 4 | 17 | MathSeq.rs | 1 | `T` |
| 5 | 41 | AVLTreeSetStPer.rs | 1 | `T` |
| 6 | 43 | OrderedTableStEph.rs | 6 | `K` |
| 7 | 43 | OrderedTableStPer.rs | 5 | `K` |
| 8 | 47 | DoubleHashFlatHashTableStEph.rs | 3 | `Key` |
| 9 | 47 | LinProbFlatHashTableStEph.rs | 3 | `Key` |
| 10 | 47 | QuadProbFlatHashTableStEph.rs | 3 | `Key` |
| 11 | 47 | LinkedListChainedHashTableStEph.rs | 3 | `Key` |
| 12 | 47 | VecChainedHashTableStEph.rs | 3 | `Key` |
| 13 | 47 | StructChainedHashTable.rs | 3 | `Key` |
| 14 | 62 | StarPartitionMtEph.rs | 3 | `V` |

### Broadcast triggers (3 files)

Added `assert(obeys_feq_full_trigger::<T>())` where the broadcast axiom needed
explicit triggering (functions that didn't have `obeys_feq_full` in scope):

| # | Chap | File | Type |
|---|------|------|------|
| 1 | 55 | SCCStEph.rs | `usize` |
| 2 | 55 | TopoSortStEph.rs | `usize` |
| 3 | 66 | BoruvkaStEph.rs | `usize` |

### PTT fixes (1 file)

**`rust_verify_test/tests/Chap18/ProveArraySeq.rs`**: Added `use feq::*`,
`broadcast use group_feq_axioms`, and `assert(obeys_feq_full_trigger::<u64>())`
to 6 test functions that call ArraySeq methods requiring `obeys_feq_full::<u64>()`.

## Technique

The `reveal()` builtin in Verus cannot take turbofish generic params from outer
scope (E0401). Workaround: `lemma_reveal_view_injective<T>()` proof fn that
takes T from its own signature, calls `reveal(obeys_feq_view_injective)` (Verus
infers T), and re-exports the forall body as an ensures clause. Callers invoke
`lemma_reveal_view_injective::<ConcreteType>()` with turbofish.

Inside loops without `loop_isolation(false)`, the broadcast trigger
`assert(obeys_feq_full_trigger::<Key>())` must precede the lemma call to
establish `obeys_feq_full::<Key>()` (which includes `obeys_feq_view_injective`)
in the loop body's isolated scope.

## Files changed

38 files, +202 -85 lines.
