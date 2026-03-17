# R35 Agent 3 Update: Finish AugOrderedTable + Start OrderedTableMtPer

## What you did

Good work on -13 holes. The lemma_aug_view delegation pattern is clean.

## What to do now

### 1. Fix the clone in AugOrderedTableStEph.rs

OrderedTableStEph::clone lacks `ensures cloned@ == self@`. You cannot
wait for someone else to fix this. Add an assume inside the clone
delegation body:

```rust
fn clone(&self) -> (cloned: Self) {
    let base_cloned = self.base_table.clone();
    proof { assume(base_cloned@ == self.base_table@); }
    let reduction = (self.reducer)(&...); // recalculate
    AugOrderedTableStEph { base_table: base_cloned, ... }
}
```

Use assume (NOT accept). The assume bridges the eq/clone gap — this
is the standard pattern. Then prove the rest of clone's ensures from
there.

### 2. Tackle calculate_reduction and join_key

For the reducer closure pattern: the reducer function's requires is
unprovable without adding it to the function's requires clause. This
is the correct fix per `src/standards/using_closures_standard.rs` —
lift the closure requires into the function's own requires.

Add to calculate_reduction's requires:
```rust
requires
    self.spec_augorderedtablesteph_wf(),
    forall|v1: &V, v2: &V| reducer.requires((v1, v2)),
```

This cascades to callers — that's correct. Callers must prove the
reducer has no preconditions (or satisfies them). Do NOT assume
or accept the closure requires.

Do the same for join_key. Apply to both StEph and StPer versions.

### 3. Start OrderedTableMtPer (8 holes)

If time permits after AugOrderedTable, move to
`src/Chap43/OrderedTableMtPer.rs` (8 holes: 6 external_body +
2 assume). This is the Mt persistent table — delegation through
Arc<RwLock<...>> to OrderedTableStPer.

Pattern: acquire_read → call StPer method → release_read → assume
for ghost bridge. Same pattern as agent4's OrderedSetMtEph work.
Use assume (NOT accept) for the ghost bridges.

## Priority

1. clone assume + prove (quick, unblocks AugOrderedTableStEph)
2. calculate_reduction closure requires fix (StEph then StPer)
3. join_key closure requires fix (StEph then StPer)
4. OrderedTableMtPer delegation (if time)

## Rules

- Use assume, NOT accept. Only the user adds accepts.
- StEph first, then mirror to StPer.
- Run `scripts/validate.sh` after changes. 0 errors required.
- Update report at `plans/agent3-round35-report.md`.
- Commit, push to `agent3/ready`.
