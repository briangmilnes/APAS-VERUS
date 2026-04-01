# Architecture: Coarse Lock + TSM + Parallel Inside

## Status: Design (2026-04-01)

This document describes the target Mt module architecture for APAS-VERUS.
It combines three layers to achieve thread safety, zero assumes, and parallel
computation simultaneously.

## 1. The Problem

APAS Mt modules need:
1. **Thread safety** — multiple threads call operations concurrently
2. **Zero assumes** — proof doesn't trust unverified ghost state
3. **Parallel computation** — map, reduce, filter, union use fork-join internally

Current architecture gets (1) via coarse RwLock but sacrifices (3). All computation
inside the lock is sequential on St types. The Mt types have parallel operations
(map_dc, reduce_dc, filter_dc, ParaPair union/intersect/difference) but they're
trapped behind their own lock boundaries.

## 2. Mt Trait Architecture: Locked and Unlocked

Each Mt module exposes two trait layers:

**Locked trait** — the public API. Takes `&self` / `&mut self`. Acquires the lock,
delegates to the unlocked trait on the owned interior, releases the lock. Returns
`Result` for capacity-bounded operations. This is what external callers use.

```rust
pub trait FooMtLockedTrait: Sized {
    fn insert(&mut self, x: T) -> Result<(), ()>
        requires old(self).wf(),
        ensures ...;

    fn size(&self) -> (n: usize)
        requires self.wf(),
        ensures ...;

    fn parallel_map(&mut self, f: &F) -> Result<(), ()>
        requires self.wf(), ...
        ensures ...;
}
```

**Unlocked trait** — the full implementation on owned data (not behind a lock).
Contains all operations: sequential (insert, find, size, nth) and parallel
(map, reduce, filter, union). Called by the locked trait after `acquire_write`,
or called directly when the data is already owned (e.g., inside another
module's lock).

```rust
pub trait FooMtUnlockedTrait: Sized {
    fn insert(self, x: T) -> Self
        requires self.wf(), ...
        ensures ...;

    fn find(&self, x: &T) -> Option<&T>
        requires self.wf(), ...
        ensures ...;

    fn size(&self) -> usize
        requires self.wf(), ...
        ensures ...;

    fn map(self, f: &F) -> Self          // parallel internally where applicable
        requires self.wf(), ...
        ensures ...;

    fn reduce(&self, f: &F, id: T) -> T  // parallel internally where applicable
        requires self.wf(), ...
        ensures ...;
}
```

The locked trait acquires the lock, calls the unlocked trait, releases:

```rust
fn map(&mut self, f: &F) -> Result<(), ()> {
    let (interior, write_handle) = self.lock.acquire_write();
    // capacity check...
    let new_data = interior.data.map(f);  // unlocked trait — parallel internally
    // step TSM...
    write_handle.release_write(new_interior);
    Ok(())
}

fn size(&self) -> usize {
    let read_handle = self.lock.acquire_read();
    let n = read_handle.borrow().data.size();  // unlocked trait — sequential
    read_handle.release_read();
    n
}
```

When M1 stores M2 inside its lock, M1's locked trait calls M2's **unlocked** trait
directly on the owned M2 data — bypassing M2's lock since M1 already has exclusive
access. This works for all operations, sequential or parallel.

## 3. Mt Inside Mt: Composable Parallelism

If module M1 stores module M2 (an Mt type) inside M1's locked interior, then
after M1's `acquire_write`, you own M2. You call M2's unlocked parallel operations
directly — not through M2's lock, but through M2's unlocked trait on owned data.
M1's lock already gives exclusive access to M2.

This is composition: M1 provides the lock boundary. M2 provides the parallel
algorithms via its unlocked trait. No nested locks. No unsafe.

## 4. Slice-Backed Sequences: O(1) Split

`ArraySeqMtEphSliceS` (Chap19) stores `Arc<Vec<T>>` with offset+length window.
`slice()` and `subseq_copy()` are O(1) — Arc::clone + adjust window. Both halves
share the same backing storage. They're owned, `Send + 'static`, and move directly
into `join()` closures.

This makes splitting data for fork-join free.

## 5. Pre-Allocated Output: O(1) Rejoin

For operations producing new sequences (map, filter, tabulate), pre-allocate the
output Vec before forking:

1. Allocate output Vec of size n: O(n) work, O(1) span
2. Split input via O(1) slice
3. `join(write_left_half, write_right_half)` — both arms write to disjoint
   regions of the same pre-allocated output
4. No rejoin needed — output is already contiguous

For merging adjacent slices from the same backing `Arc<Vec<T>>`:
- Condition: `left.data == right.data && left.start + left.len == right.start`
- Merge: `{ data: left.data, start: left.start, len: left.len + right.len }` — O(1)

This eliminates the Vec concat bottleneck that limits current D&C operations.

The Rust/Verus challenge: parallel writes to disjoint regions of the same Vec
requires either PCell-per-element with PointsTo tokens, or Verus's new-mut-ref
support for mutable references to disjoint sub-places. The FIFO example in
Verus demonstrates disjoint concurrent writes via PCell + PointsTo.

## 6. The Three Layers

### 6.1. Layer 1 — Thread Safety

One coarse `RwLock` on the outer module. `&mut self` for writes, `&self` for
reads. The lock serializes concurrent access to the whole module.

### 6.2. Layer 2 — Zero Assumes

TSM token inside the lock alongside the data. `RwLockPredicate` ties
`token.value() == data.abstract_state()`. After acquire, the predicate proves
the relationship. No ghost field outside the lock, no assume bridge.

### 6.3. Layer 3 — Parallel Computation

After `acquire_write`, you own the interior including its Mt data structures.
Call their unlocked parallel operations directly. `join()` arms take owned slices
or subtrees. Pre-allocate output for O(1) rejoin. Step TSM token. Release.

## 7. Interior Structure

```
FooMtEph {
    lock: RwLock<FooInterior, FooInv>,
    inst: Tracked<FooSM::Instance>,
}

FooInterior {
    sequences: ArraySeqMtEphSliceS<T>,     // O(1) slice, parallel reduce/map
    sets: AVLTreeSetMtEph<T>,              // parallel union/intersect/difference
    tables: OrderedTableMtEph<K, V>,       // parallel tabulate/map
    token: Tracked<FooSM::state>,          // TSM token
}
```

## 8. Operation Lifecycle

### 8.1. Write (parallel)

```
acquire_write → own (interior, write_handle)
  ↓
exec-time capacity check → Err if full
  ↓
call Mt data structure's unlocked parallel operation on owned data
  (internally: O(1) slice split, join, pre-allocated output)
  ↓
step TSM token (proof block)
  ↓
release_write(updated interior)
```

### 8.2. Read

```
acquire_read → borrow interior
  ↓
predicate proves data wf
  ↓
compute from real data
  ↓
release_read → return value
```

## 9. View

Since writes take `&mut self`, PCell Approach B gives View + zero assumes.
Ghost field outside the lock updated atomically with release_write. `&mut self`
guarantees sole ownership during write — no other thread observes the gap.

## 10. Cost Analysis vs APAS

| Operation  | APAS Span              | Current Span    | New Span               | Notes                          |
|------------|------------------------|-----------------|------------------------|--------------------------------|
| length     | O(1)                   | O(1)            | O(1)                   | Match                          |
| nth        | O(1)                   | O(1)            | O(1)                   | Match                          |
| subseq     | O(1)                   | O(j) copy       | O(1) slice             | Fixed by slice-backed          |
| append     | O(1)                   | O(n)            | O(1) adjacent merge    | Fixed if slices are adjacent   |
| filter     | O(lg n + max S(f))     | O(lg n) D&C     | O(lg n + max S(f))     | Match with pre-alloc output    |
| update     | O(1)                   | O(n) clone      | O(1) slice + write     | Fixed by slice-backed          |
| inject     | O(lg degree)           | O(n+m)          | O(lg degree)           | Needs parallel inject impl     |
| reduce     | O(lg n * max S(f))     | O(lg n) D&C     | O(lg n * max S(f))     | Match                          |
| scan       | O(lg n)                | O(n)            | O(lg n)                | Needs Blelloch impl            |
| map        | O(1 + max S(f))        | O(lg n) D&C     | O(lg n + max S(f))     | lg n from D&C depth, not PRAM  |
| tabulate   | O(1 + max S(f))        | O(n)            | O(lg n + max S(f))     | lg n from D&C depth            |
| flatten    | O(lg n)                | O(Σ a_i)        | O(lg n)                | With adjacent slice merge      |
| union      | O(lg n)                | O(lg² n)        | O(lg² n)               | Fork-join vs PRAM gap          |
| intersect  | O(lg n)                | O(lg² n)        | O(lg² n)               | Fork-join vs PRAM gap          |
| difference | O(lg n)                | O(lg² n)        | O(lg² n)               | Fork-join vs PRAM gap          |

### 10.1. Remaining gaps

**O(lg n) vs O(1) for map/tabulate**: APAS assumes PRAM O(1) fork. Fork-join D&C
has O(lg n) recursion depth. This is the fundamental PRAM-vs-fork-join gap —
unavoidable without true PRAM.

**O(lg² n) vs O(lg n) for set operations**: Fork-join ParaPair does O(lg n) work
per recursive level sequentially (the split). PRAM does the split concurrently.
Standard fork-join gap.

**scan**: Needs Blelloch parallel prefix (up-sweep/down-sweep). Not implemented
but architecturally possible with O(1) slice split.

**inject**: Needs parallel inject implementation. Deterministic ordering
constraint makes this harder than map/filter.

## 11. Disjoint Parallel Writes

The pre-allocated output pattern requires two `join()` arms to write to disjoint
regions of the same Vec. Options:

1. **PCell per element**: Each output slot is a `PCell<T>`. Each join arm gets
   `PointsTo` tokens for its slice region. The FIFO Verus example demonstrates
   this pattern for a ring buffer with `storage_map` sharding in a TSM.

2. **Split mutable slices**: Verus's `new-mut-ref` work enables mutable references
   to disjoint sub-places. Once stable, `&mut output[0..mid]` and
   `&mut output[mid..n]` could coexist in separate join arms.

3. **Two separate Vecs + adjacent merge**: Each arm writes to its own Vec. If the
   output is slice-backed and both Vecs are placed adjacent in a single backing
   allocation, the merge is O(1) window adjustment. This avoids the disjoint-write
   problem but requires allocation coordination.

Option 3 is simplest with current Verus. Options 1-2 give true O(1) span for
the output construction.

## 12. What's Needed

### 12.1. Experiments

1. **coarse_lock_parallel_tsm.rs**: Rewrite with `ArraySeqMtEphSliceS` as one
   inner Mt type and an AVLTreeSet/OrderedTable as another. Demonstrate parallel
   reduce (O(1) split, O(lg n) span) and parallel union (ParaPair) both inside
   a single `acquire_write`.

2. **disjoint_write_pcell.rs**: Experiment with PCell-per-slot output array.
   Two join arms write to disjoint PointsTo regions. Proves O(1) rejoin for
   map/tabulate.

### 12.2. Migration

1. Verify experiments
2. Migrate one real module (graph module from Chap52+ — has both sequences and sets)
3. Update `toplevel_coarse_rwlocks_for_mt_modules.rs` standard
4. Systematize across all Mt modules

### 12.3. Verus Dependencies

- `make-ghost-send-sync` branch: fixes `Ghost<T>` Send/Sync, eliminates need for
  `unsafe impl Send/Sync` on types containing Ghost fields
- `new-mut-ref`: enables disjoint mutable borrows, potentially simplifying the
  parallel write pattern
- Clone on closures: still unrecognized by Verus; `clone_fn` workaround remains
  necessary
