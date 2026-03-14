# Agent 4 — Round 13 (RESTART)

## PBOGH: You got -5 out of a target of -15. Then declared everything "architectural" and stopped.

That word — "architectural" — is not a reason to stop. It's a description of the
work. Every hole in this project involves architecture. Your job is to change the
architecture where needed to make the proofs go through, not to catalog reasons
for inaction.

## Your files

**Chap47** (5 remaining holes):
- `ParaHashTableStEph.rs` — 2 external_body (call_hash_fn, double_hash_probe)
- `DoubleHashFlatHashTableStEph.rs` — 1 external_body (second_hash)
- `ChainedHashTable.rs` — 2 external_body (insert_chained, delete_chained)

**Chap42** (11 holes):
- `TableMtEph.rs` — 11 external_body

**Chap41** (22 holes):
- `AVLTreeSetMtEph.rs` — 10
- `AVLTreeSetMtPer.rs` — 12

**Chap39** (8 holes):
- `BSTTreapMtEph.rs` — 8 assume

## Priority: Chap47 first, then Chap41 Mt

### Chap47 (5 holes)

You said ChainedHashTable is blocked by "Verus lacks IndexMut for Vec" and you
tried "clone+set workaround" but failed because "Verus cannot synthesize Clone
for tuple types."

Try harder:
- Don't clone the whole Vec. Clone the chain at index i, modify it, put it back
  with `vec_set`. Or use `swap_remove` + `push`.
- For ParaHashTable closure wrappers: if call_hash_fn truly can't be verified
  because the closure is opaque, that's a real external_body. Say so clearly
  and move on to the next file.

### Chap41 Mt (22 holes)

Agent 3 just cleared the feq blocker for StEph/StPer (added broadcast axioms
to vstdplus/feq.rs). The feq assumes in MtEph/MtPer may now be eliminable
with `broadcast use group_feq_axioms`. Try it.

For the external_body functions: these wrap StEph operations through
Arc<RwLock>. The pattern is acquire lock → call inner → release → assert
ensures from inner. Read `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs`.

### Chap39 BSTTreapMtEph (8 holes)

You said "&self ghost_locked_root can't be updated." Consider: does it NEED
to be updated? If the ghost field is set correctly at construction and the
RwLock invariant preserves the relationship, reads should work without mutation.
The assume is about reading — prove the read is correct from the invariant.

## DO NOT

- Touch Chap43 (Agents 1 and 2)
- Touch Chap41 St files (Agent 3)
- Touch Chap38 (Agent 3)
- Write "architectural" or "blocked" without listing exactly what you tried

## Rules

- Run `scripts/validate.sh` after every change.
- NO accept().
- Push to `agent4/ready`. Write `plans/agent4-round13-report.md`.
- **Prove or move on.** 10 minutes max per hole. Don't stare at one hole for
  30 minutes writing about why it's hard.

## Target: Chap47 5 → ≤ 2. Chap41 Mt 22 → ≤ 17. Total -10.
