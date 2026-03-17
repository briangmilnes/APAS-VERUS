# R36 Agent 1: Chap41 AVLTreeSetMtPer + AVLTreeSetMtEph

## Goal

Prove Mt RwLock delegation wrappers in AVLTreeSetMtPer.rs (8 ext_body)
and AVLTreeSetMtEph.rs (5 ext_body). These wrap AVLTreeSetStPer/StEph
behind Arc<RwLock<...>>.

## Background

You proved OrderedSetStEph ordering operations in R35. Now apply the
Mt delegation pattern to the underlying AVLTreeSet Mt modules.

Read `src/Chap43/OrderedSetMtEph.rs` for the RwLock delegation pattern
(agent4's R35 work). The pattern:

1. `proof { use_type_invariant(self); }` (or equivalent invariant access)
2. `let rlock = self.inner.acquire_read();`
3. `let result = rlock.method();`
4. `self.inner.release_read(rlock);`
5. `proof { assume(inner@ =~= self@); }` — one assume per delegation

Also read `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs`.

## AVLTreeSetMtPer.rs targets (8 external_body)

Read the file first. Identify all external_body functions. These should
be delegation wrappers around AVLTreeSetStPer methods.

The 8 functions will be operations like insert, delete, find, size,
first, last, etc. For each:
- Remove external_body
- Write the delegation body (acquire_read or acquire_write)
- Add one assume for the ghost bridge
- The StPer method's ensures transfer through view equality

## AVLTreeSetMtEph.rs targets (5 external_body + structural)

Same pattern but wrapping AVLTreeSetStEph. The file also has:
- 2 × unsafe impl Send/Sync — structural FPs, leave as-is
- 1 × rwlock:reader assume — structural, leave as assume

## Priority

1. AVLTreeSetMtPer.rs (8 ext_body) — start here, likely mechanical
2. AVLTreeSetMtEph.rs (5 ext_body) — same pattern
3. Fix any fn_missing_requires/ensures warnings in these files

## Rules

- assume() only. NEVER accept().
- Do NOT modify CLAUDE.md.
- Do NOT modify ~/projects/veracity/.
- Read existing proved methods in each file FIRST — copy their pattern.
- One assume per delegation for the ghost bridge.
- Run `scripts/validate.sh` after changes. 0 errors required.
- Write report to `plans/agent1-round36-report.md`.
- Commit, push to `agent1/ready`.
