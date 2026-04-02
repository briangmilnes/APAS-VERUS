# R137 Agent 3 — Experiment: AtomicBool spinlock + PCell for Mt module. AFK.

## Setup

Read ALL files in `src/standards/` before starting.
Read `docs/architecture-coarse-lock-parallel-mt.md` — especially section 2 (two concurrency styles).
Read `src/experiments/verus_basic_lock_pcell.rs` — this is the pattern you're building on.
Read `src/experiments/bst_plain_mt_tsm.rs` — the RwLock + TSM version for comparison.

Report file: `plans/r137-agent3-atomic-spinlock-experiment-report.md`

## Problem

Our RwLock + TSM architecture has 2 accepts per file for Caller-Observable State
(the ghost↔lock bridge). The PCell + TSM + Atomics path eliminates them but
we thought it required complex storage_map protocols (FIFO pattern).

It doesn't. `verus_basic_lock_pcell.rs` shows a simple AtomicBool spinlock +
PCell pattern in ~40 lines. The PointsTo flows through the atomic invariant —
when unlocked, PointsTo sits in the atomic's ghost state. When acquired, the
thread takes it out. Simple CAS to acquire, store to release.

## Task

Write `src/experiments/atomic_spinlock_pcell_mt.rs` — the same 10-operation
Vec-backed set from `bst_plain_mt_tsm.rs`, but using AtomicBool + PCell instead
of RwLock + TSM.

## Architecture

```rust
struct_with_invariants!{
    struct SetMtAtomic {
        pub atomic: AtomicBool<_, Option<cell::PointsTo<SetInner>>, _>,
        pub cell: PCell<SetInner>,
    }

    spec fn wf(self) -> bool {
        invariant on atomic with (cell) is (v: bool, g: Option<cell::PointsTo<SetInner>>) {
            match g {
                None => v == true,  // locked — PointsTo taken by a thread
                Some(points_to) => {
                    points_to.id() == cell.id()
                    && points_to.is_init()
                    && v == false     // unlocked — PointsTo stored here
                    // Can add data invariants here:
                    // && points_to.value().elements@.len() <= usize::MAX
                }
            }
        }
    }
}
```

The PCell holds `SetInner` (same inner type as bst_plain_mt_tsm). The AtomicBool's
ghost state holds `Option<PointsTo<SetInner>>`.

## Operations to implement

Same 10 as bst_plain_mt_tsm:
1. `new_empty()` → construct
2. `mt_insert(&self, val: u64)` → acquire, take from PCell, mutate, put back, release
3. `mt_delete(&self, val: &u64)` → same pattern
4. `mt_contains(&self, val: &u64)` → acquire, borrow from PCell, compute, release
5. `mt_size(&self)` → acquire, borrow, compute, release
6. `mt_is_empty(&self)` → same
7. `mt_find(&self, val: &u64)` → same
8. `mt_minimum(&self)` → same
9. `mt_maximum(&self)` → same

## Write pattern (insert/delete)

```rust
fn mt_insert(&self, val: u64)
    requires self.wf(),
{
    // Acquire: CAS false→true, take PointsTo from atomic ghost state
    let tracked_perm = self.acquire();
    let tracked mut perm = tracked_perm.get();

    // Take data from PCell — PointsTo proves what we got. ZERO ASSUMES.
    let mut inner = self.cell.take(Tracked(&mut perm));

    // Mutate
    inner.insert(val);

    // Put back — PointsTo updated
    self.cell.put(Tracked(&mut perm), inner);

    // Release: store false, return PointsTo to atomic ghost state
    self.release(Tracked(perm));
}
```

## Read pattern (contains/size)

```rust
fn mt_contains(&self, val: &u64) -> (found: bool)
    requires self.wf(),
{
    let tracked_perm = self.acquire();
    let inner = self.cell.borrow(Tracked(tracked_perm.borrow()));
    let found = inner.contains(val);
    self.release(tracked_perm);
    found
}
```

## What this proves

- Zero assumes, zero accepts — PointsTo IS the proof
- No TSM needed — the atomic invariant handles everything
- No RwLock — AtomicBool spinlock is simpler and lighter
- Same operations, same inner type, same functional specs as the TSM version
- Compare line count and complexity with bst_plain_mt_tsm.rs

## View question

Try both:
- **Without View**: specs on return values only. Same as bst_plain_mt_tsm.
- **With View**: add a `ghost_count: Ghost<nat>` field. After release, update it.
  Since all operations take `&self` (not `&mut self`), this means multiple threads
  can call insert concurrently. The ghost_count may be stale. Document whether
  View specs can reference `self@` meaningfully with `&self` writes.

  If `&self` writes break View, try changing writes to `&mut self` and see if
  the ensures can then prove `self@ == new_count`.

## Do NOT register in lib.rs

Experiments stay commented out. Temporarily uncomment to validate, comment back.

## Validation

Validate with experiment temporarily uncommented. Then `scripts/rtt.sh`.

## Rules

- Zero assumes, zero accepts, zero external_body in experiment code.
- Copy acquire/release from `verus_basic_lock_pcell.rs` — don't reinvent.
- Copy SetInner from `bst_plain_mt_tsm.rs` — same inner type.
- Include a `#[test]` exercising all operations.

## When done

RCP.
