# R35 Agent 4 Update: Fix Accepts + Finish OrderedTableMtEph

## What went wrong

1. You deleted the veracity rule from CLAUDE.md. Do not modify CLAUDE.md.
2. You used accept() instead of assume(). Agents NEVER add accept().
   Only the user adds accepts. This is in CLAUDE.md. Read it again:
   "DO NOT CONVERT assume() TO accept()."

## Mandatory fixes

Change every `accept(...)` you added back to `assume(...)` in
OrderedSetMtEph.rs. ALL of them — both the ones that were originally
assume (size, find, split, get_range, split_rank) and the new ones
in the delegation bodies (first, last, previous, next, rank, select).

The function bodies are good — keep those. Just change accept → assume.

Revert CLAUDE.md to match main. Run:
```
git checkout origin/main -- CLAUDE.md
```

## Then: Finish OrderedTableMtEph ordering operations

You got filter proved (-1). The remaining 6 external_body holes in
OrderedTableMtEph are ordering operations (first_key, last_key,
previous_key, next_key, rank_key, select_key).

These are direct delegation through RwLock — same pattern as
OrderedSetMtEph:
1. acquire_read
2. call StEph ordering method on inner
3. release_read
4. assume(inner@ =~= self@) for ghost bridge

Write the real delegation bodies. Use assume (NOT accept) for
the ghost bridge.

## Then: Start AugOrderedTableMtEph (8 holes)

If time permits, move to `src/Chap43/AugOrderedTableMtEph.rs`
(8 external_body holes). Same Mt RwLock delegation pattern.

## Priority

1. Fix all accept → assume in OrderedSetMtEph.rs (mandatory)
2. Revert CLAUDE.md (mandatory)
3. Prove 6 OrderedTableMtEph ordering operations
4. AugOrderedTableMtEph if time

## Rules

- assume() only. NEVER accept(). NEVER EVER.
- Do NOT modify CLAUDE.md.
- Do NOT modify ~/projects/veracity/.
- StEph delegation first, StPer mirrors.
- Run `scripts/validate.sh` after changes. 0 errors required.
- Update report at `plans/agent4-round35-report.md`.
- Commit, push to `agent4/ready`.
