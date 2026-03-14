# Agent 1 — Round 13 Prompt

## Mission

Prove 40 of the 61 Mt holes in Chap43. Strengthen RwLock invariants, replace
assumes with asserts, prove external_body wrappers. This is mechanical work —
do it fast.

## Your Files

- `OrderedSetMtEph.rs` — 36 assume, 3 external_body (39 total)
- `OrderedTableMtPer.rs` — 14 assume, 8 external_body (22 total)
- `OrderedTableMtEph.rs` — 15 external_body (bonus)
- `AugOrderedTableMtEph.rs` — 5 external_body (bonus)

## Step 1: Strengthen the RwLock Invariant

Read the `*Inv` struct (OrderedSetMtEphInv, OrderedTableMtPerInv). If `inv`
is weak, add:
- `ghost_field@ == v@`
- `v@.finite()`
- Any other properties the assumes need

Update constructors and mutators to establish/preserve the invariant.

## Step 2: Replace Assumes with Asserts

After `acquire_read`/`acquire_write`, Verus knows `inv(pred, locked_val)`.
Change every `assume(...)` to `assert(...)`. If Verus can't prove it, add
an intermediate assertion bridging inv to the property.

## Step 3: Prove External_body Wrappers

For each external_body function:
1. Remove `#[verifier::external_body]`
2. Write the body: typically acquire lock, call inner method, release
3. The inner StEph/StPer methods have verified ensures — chain them

## References

- `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs` — READ FIRST
- `src/Chap41/AVLTreeSetMtEph.rs` — Agent 4 did this exact pattern
- `src/Chap42/TableMtEph.rs` — Agent 3's broadcast proof for feq

## DO NOT TOUCH

- Chap43 St/StPer files — Agent 2
- Chap41 — Agents 3 and 4
- Chap42, Chap47 — Agent 4

## Rules

- Run `scripts/validate.sh` after every change.
- NO accept(). Skip Example files.
- Push to `agent1/ready`. Write `plans/agent1-round13-report.md`.
- **Prove or move on.** Don't spend more than 10 minutes on any single hole.

## Target: 61 → ≤ 21 (-40)
