# Eliminating RwLock Boundary Assumes: Experiment Results

Date: 2026-03-31

## Problem

APAS-VERUS has 19 proof holes, all from `assume()` calls at RwLock
boundaries in Mt (multi-threaded) modules. Every Mt operation follows
this pattern:

```rust
let (mut inner, write_handle) = self.lock.acquire_write();
proof { assume(self.ghost_view@ == inner@); }  // THE GAP
inner.insert(val);
self.ghost_view = Ghost(inner@);
write_handle.release_write(inner);
```

The assume bridges the gap between a ghost field on the struct
(`ghost_view`) and the actual value inside the lock (`inner`).
Verus's RwLock API provides `RwLockPredicate::inv(v)` after acquire,
which proves constraints on `v`, but nothing connects `v` to the
ghost field. There are 215 such assumes across 80 Mt files.

## Experiments

### 1. PCell + PointsTo (single-threaded, zero assumes)

File: `src/experiments/rwlock_no_ghost_field.rs`

**Approach**: Use `PCell<T>` with `Tracked<PointsTo<T>>`. The PointsTo
token IS the proof of what the cell contains. View reads directly from
the token's `mem_contents()`. No ghost field.

**Result**: SUCCEEDS. Zero assumes. All specs verified.

**Limitation**: PCell requires `&mut PointsTo` for writes, which means
single-owner access. However, PCell itself is `Send + Sync` — the
concurrency control is at the token level, not the cell level.

### 2. Generic LockedWrapper (centralized assume)

File: `src/experiments/locked_wrapper_generic.rs`

**Approach**: Generic `LockedWrapper<T: View>` struct holding
`Arc<RwLock<T>> + Ghost<T::V>`. View reads from ghost field.
Mt types become `type SetMtEph = LockedWrapper<SetStEph>`.
Every operation follows the same protocol: acquire, assume ghost == inner,
delegate to StEph method, update ghost, release.

**Result**: SUCCEEDS. Reduces 215 assumes to 1 per acquire (always the
same `ghost_view@ == inner@`). But still an assume — not zero.

**Value**: Standardizes the pattern. A veracity tool could verify every
Mt file follows the protocol. But does not eliminate the trust point.

### 3. State Machine + RwLock (concurrent, zero assumes)

File: `src/experiments/state_machine_set_mt.rs` (in progress)

**Approach**: Based on the Verus SOSP 2024 tutorial "counting to 2" pattern:

1. Define a `tokenized_state_machine!` that tracks abstract state
2. Store the state machine token INSIDE the RwLock alongside the data
3. `RwLockPredicate` ties the physical value to the token value
4. After acquire, the predicate PROVES physical == abstract
5. State machine transitions prove operations are valid
6. Zero assumes — the token IS the proof

**Architecture**:

```
tokenized_state_machine!(SetSM {
    fields {
        #[sharding(variable)]
        pub contents: Seq<u64>,
    }
    transition! { tr_insert(val: u64) { update contents = pre.contents.push(val); } }
});

struct SetLockInterior {
    steph: SetStEph,
    ghost_contents: Tracked<SetSM::contents>,
}

impl RwLockPredicate<SetLockInterior> for SetMtInv {
    open spec fn inv(self, interior: SetLockInterior) -> bool {
        // THE CONNECTION: concrete view == abstract token value.
        interior.steph@ == interior.ghost_contents@@.value
        && interior.ghost_contents@@.instance == self.instance
    }
}
```

After `acquire_write()`, the predicate guarantees
`interior.steph@ == token.value`. Operations step the token
(`instance.tr_insert()`) and mutate the steph (`inner.insert()`).
On release, the predicate re-verifies consistency.

**Status**: In progress. Compiles but not yet fully verified.

**Reference**: `~/projects/VerusCodebases/event-sites/2024-sosp-tutorial/exercises/advanced-rust-topics/solutions/solution_counting_to_2.rs`

## Key Insight

The ghost field pattern fails because nothing connects a field on the
outer struct to a value inside the lock. The state machine pattern
succeeds because the proof token lives WITH the data inside the lock,
and the predicate ties them together. After acquire, you have both
the data and the proof — no gap to bridge.

## View Question

The state machine approach eliminates assumes but raises a question:
how does the Mt type express `self@`? The abstract state is inside
the lock, inaccessible in spec mode.

Options:
- **No View**: Mt operations express specs only on return values.
  Callers can't write `requires mt@.contains(x)`.
- **Ghost snapshot**: Keep a ghost field for View, updated on every
  write. This reintroduces the ghost↔lock gap — but only for View,
  not for operation correctness. The operation proofs are zero-assume;
  only View requires the assume.
- **Ghost snapshot from token**: If the state machine instance can
  be queried in spec mode for its current value, View could read
  from the instance. This depends on Verus's ghost state model.

## Impact Assessment

| Approach | Assumes | Concurrent | View | LOC change |
|----------|---------|------------|------|------------|
| Current (ghost field) | 215 | Yes | Yes | 0 |
| LockedWrapper | 215 (1 per acquire) | Yes | Yes | ~-5K refactor |
| PCell + PointsTo | 0 | No* | Yes | N/A |
| State machine | 0 | Yes | TBD | ~80 files restructured |

\* PCell is Send+Sync but requires token ownership for access.

If the state machine experiment succeeds with View support, the path
is: define one state machine per ADT (Set, Table, BST, etc.), then
each Mt file becomes a thin wrapper around StEph + token protocol.
The 80 Mt files shrink dramatically and all 19 holes vanish.

## Next Steps

1. Complete `state_machine_set_mt.rs` — get it verifying with insert + size
2. Add View support (ghost snapshot or token query)
3. Test with a more complex type (BST with tree structure, not just Seq)
4. If successful, write a migration plan for the 80 Mt files
