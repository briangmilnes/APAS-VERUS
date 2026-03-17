# R35 Agent 3 Update 2: OrderedTableMtPer

## What you did

Excellent work. -17 holes, AugOrderedTableStEph fully clean,
AugOrderedTableStPer down to 1 structural (closure clone bridge).
The reducer-in-wf pattern and lemma_reducer_clone_total centralization
were exactly right.

## What to do now

### OrderedTableMtPer.rs (8 holes)

This is the Mt persistent ordered table — delegation through
`Arc<RwLock<...>>` to OrderedTableStPer. Same RwLock delegation
pattern as the Mt files you've seen.

Read the file first. Look at any already-proved methods (insert,
delete, find, size) to see the existing delegation pattern in this
file. Then prove the remaining holes.

Expected holes (verify by reading the file):
- 6 external_body: ordering operations (first_key, last_key,
  previous_key, next_key, rank_key, select_key)
- 2 assume: RwLock ghost bridges

#### Pattern for external_body ordering operations:
1. `use_type_invariant(self)` or equivalent
2. `acquire_read` on the RwLock
3. Call the StPer ordering method on the inner value
4. `release_read`
5. `assume(inner@ =~= self@)` for the ghost bridge

One assume per delegation. The inner StPer method's ensures transfer
through the view equality.

#### Pattern for existing assumes:
These are RwLock ghost boundary assumes. Leave them as assume (NOT
accept). They are the same structural gap as every other Mt module.

### If time: OrderedSetMtPer

If OrderedTableMtPer goes quickly, there may be an OrderedSetMtPer.rs
file with similar holes. Check if it exists and has holes. Same pattern.

## Rules

- assume() only. NEVER accept().
- Do NOT modify CLAUDE.md.
- Do NOT modify ~/projects/veracity/.
- Read existing proved Mt delegation methods in the file FIRST.
- Run `scripts/validate.sh` after changes. 0 errors required.
- Update report at `plans/agent3-round35-report.md`.
- Commit, push to `agent3/ready`.
