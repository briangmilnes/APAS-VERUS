# Agent 4 — Round 14

## Good restart: -4 with clone_entry and iter(). Keep going.

You found the clone_entry workaround for ChainedHashTable — creative. Now apply that
same "find a workaround, don't declare blocked" energy to the remaining files.

## Your files

### Chap42 — TableMtEph.rs (11 external_body)

Agent 4's R13 report said 3 of these (intersection, union, difference) are sequential
code that just need sorted-entries invariant. The other 8 use join()/parallel closures.

**Start with the 3 sequential ones.** These are NOT parallel. They iterate entries
and build results. Use the collect+while loop pattern:
- Replace for-loop iteration with `while i < entries.length() { let e = entries.nth(i); ... }`
- Track sorted-entries invariant in loop invariant if needed
- Or: if `spec_tablemteph_wf` doesn't include sorted-entries, add it

Then try the simpler parallel ones:
- `domain`: acquires read lock, calls inner StEph domain(). Should be straightforward.
- `delete`: acquires write lock, removes entry. Try acquire_write + inner.delete + release.
- `map`: if the body is `for each entry: apply f`, convert to while loop.

### Chap39 — BSTTreapMtEph.rs (8 assume)

All 8 are view bridge: ghost_locked_root vs locked data after acquire_read/write.
Your R13 report said "unit struct RwLock invariant can't carry instance-specific ghost data."

**Try this**: The RwLock invariant doesn't need to be a unit struct. Change the
`BSTTreapMtEphInv` struct to carry a ghost field relating the locked Link<T> to the
ghost_locked_root view. After `acquire_read`, the inv gives you the relationship.
This is exactly what Agent 1 did for OrderedSetMtEph — read Agent 1's R13 report
at `plans/agent1-round13-report.md` for the pattern.

### Chap45 — BinaryHeapPQ.rs (1 assume)

`assume(Self::spec_sorted(result.seq@))` in extract_all_sorted. This needs the
heap property invariant to prove that repeated extract_min produces sorted output.
The heap property should be in `spec_binaryheappq_wf`. If it is, the proof follows
from: each extract_min returns the minimum, so the sequence is non-decreasing.

### Chap45 — BalancedTreePQ.rs (1 external)

`#[verifier::external]` on an impl block. Check if it can be moved inside verus!
with appropriate specs. If the impl uses features Verus can't handle, leave it.

## Read before writing

- `src/standards/using_closures_standard.rs` — for any closure work
- `plans/agent1-round13-report.md` — for the ghost field / type_invariant pattern

## DO NOT

- Touch Chap43 (Agents 1 and 2)
- Touch Chap41 St files (Agent 3)
- Touch Chap38 (Agent 3)

## Rules

- Run `scripts/validate.sh` after every change.
- NO accept().
- Push to `agent4/ready`. Write `plans/agent4-round14-report.md`.
- 10 minutes max per hole. Prove or move on.

## Target: TableMtEph 11 → ≤ 7. BSTTreapMtEph 8 → ≤ 5. Chap45 -1. Total -8.
