# Agent 1 — Round 16

## Project State

136 holes, 4097 verified, 38 clean chapters, 8 holed.

## Your Assignment: Chap42/TableMtEph + Chap41/AVLTreeSetMtPer

### Chap42/TableMtEph.rs (6 external_body)

This file is an Mt (multi-threaded) wrapper around TableStEph. The inner TableStEph
is fully proved (0 holes). Your job is to remove `external_body` from the Mt wrapper
functions and write real bodies.

**Pattern** (read `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs` first):
1. Remove `#[verifier::external_body]`
2. Acquire the RwLock: `let locked = self.rw_lock.acquire_read()` (or `acquire_write`)
3. Call the inner StEph method on the locked value
4. Bridge ghost state: the lock invariant (`inv`) guarantees the relationship between
   the ghost field (`self.ghost_locked_*`) and the locked value
5. Release lock, return result

For iteration/collection functions that need to build results from locked data,
use the **collect+while loop** pattern:
- Call `collect()` on the locked inner type to get an AVLTreeSeqStPer sequence
- Iterate with `while i < size { let elem = seq.nth(i); ... i += 1; }` (NOT `for`)
- Add loop invariants: `i <= size`, `size as nat == seq.spec_seq().len()`, wf, decreases

Run `scripts/holes.sh src/Chap42/TableMtEph.rs` to see the specific functions.

### Chap41/AVLTreeSetMtPer.rs (10 holes: 3 assume + 7 external_body)

Another Mt wrapper, this time persistent (MtPer = Arc+RwLock around StPer inner type).

- **7 external_body**: Same lock-call-bridge pattern as TableMtEph above
- **3 assume**:
  - `assume(r == self@.len())` — size: the inner StPer's `size()` ensures `result == self@.len()`.
    After acquiring the lock, bridge through the invariant to connect inner result to outer view.
  - `assume(obeys_feq_full::<T>())` — feq axiom bridge. Use:
    `broadcast use crate::vstdplus::feq::group_feq_axioms;` then
    `assert(obeys_feq_full_trigger::<T>());` to trigger the broadcast axiom.
  - `assume(!self@.contains(x@))` — delete postcondition: bridge from inner StPer's ensures.

Run `scripts/holes.sh src/Chap41/AVLTreeSetMtPer.rs` to see the specific functions and lines.

## DO NOT TOUCH

- Chap43 (Agent 2)
- Chap39 or Chap38 (Agent 3)
- Chap37, Chap47, Chap45, Chap41/AVLTreeSetMtEph, Chap41/AVLTreeSetStEph (Agent 4)
- Any Example files

## Critical Rules

- Run `scripts/validate.sh` after every change. Show full output.
- **NO accept().** NO assume→accept conversion.
- **DO NOT weaken ensures** to make proofs easier. If a trait declares a postcondition,
  you must prove it, not delete it. Leave `external_body` in place if you cannot prove
  the full ensures.
- Read `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs` before starting.
- Read `src/standards/using_closures_standard.rs` before writing closure code.
- Push to `agent1/ready`. Write `plans/agent1-round16-report.md`.

## Target: -8 (stretch -14). Close Chap42.
