# R35 Agent 4: OrderedSetMtEph + OrderedTableMtEph (Mt Delegation)

## Goal

Prove or accept holes in OrderedSetMtEph.rs (8 external_body +
7 assume) and OrderedTableMtEph.rs (7 external_body). These are
Mt modules with RwLock patterns.

## Background: Mt Delegation Pattern

Mt modules wrap St counterparts behind `Arc<RwLock<...>>`. Each
operation: acquire_read → call St method → release_read → bridge
result to ghost view.

The external_body holes are RwLock delegation wrappers. The assume
holes are RwLock ghost state bridges (same pattern as BSTTreapMtEph
R34 — these are structurally unprovable and should be converted to
accept() with the user's pre-approval).

**User has pre-approved assume→accept conversions for RwLock ghost
boundary holes.** Agent4 R34 established the pattern and the user
accepted it. Apply the same treatment here.

## OrderedSetMtEph.rs targets (15 holes)

### external_body (8 — delegation wrappers)

| # | Line | Function | Notes |
|---|------|----------|-------|
| 1 | 294 | filter | Closure-based filtering |
| 2 | 345 | to_seq | Convert to sequence |
| 3 | 378 | first | Ordering operation |
| 4 | 389 | last | Ordering operation |
| 5 | 400 | previous | Ordering operation |
| 6 | 411 | next | Ordering operation |
| 7 | 459 | rank | Ordering operation |
| 8 | 470 | select | Ordering operation |

### assume (7 — RwLock ghost bridges)

| # | Line | Function | Notes |
|---|------|----------|-------|
| 1 | 241 | size | `count == self@.len()` |
| 2 | 269 | find | `found == self@.contains(x@)` |
| 3 | 430 | split (left) | Left result set matches |
| 4 | 431 | split (right) | Right result set matches |
| 5 | 455 | get_range | Range result matches |
| 6 | 489 | split_rank (left) | Left result matches |
| 7 | 490 | split_rank (right) | Right result matches |

## OrderedTableMtEph.rs targets (7 holes)

### external_body (7 — delegation wrappers)

| # | Line | Function | Notes |
|---|------|----------|-------|
| 1 | 396 | filter | Closure-based filtering |
| 2 | 505 | first_key | Ordering operation |
| 3 | 525 | last_key | Ordering operation |
| 4 | 545 | previous_key | Ordering operation |
| 5 | 566 | next_key | Ordering operation |
| 6 | 659 | rank_key | Ordering operation |
| 7 | 677 | select_key | Ordering operation |

## Approach

### For external_body delegation wrappers:
1. Remove external_body
2. Write the delegation: acquire_read → call St method → release_read
3. Add ghost state bridge (accept) to connect result to `self@`
4. The ordering operations (first, last, etc.) follow the same pattern
   as existing proved operations (insert, delete, find)

### For assume → accept:
1. Verify each assume matches the RWLOCK_GHOST pattern
   (acquire → call → release → assume bridges ghost state)
2. Convert `assume(...)` to `accept(...)` with comment
3. These are the same structural gap as BSTTreapMtEph R34

### For filter:
Filter takes a closure — this may be harder. Read
`src/standards/using_closures_standard.rs` first. If the closure
pattern blocks you, skip filter and report what's needed.

## Also fix

- OrderedSetMtEph.rs line 68: `fn_missing_wf_ensures` on `from_st`
- OrderedTableMtEph.rs line 887: `fn_missing_wf_ensures` on
  `from_sorted_entries`

## Priority

1. Convert 7 assumes to accepts (quick, pattern established)
2. Prove ordering delegations in OrderedTableMtEph (first_key through
   select_key — 5 pure ordering operations)
3. Prove ordering delegations in OrderedSetMtEph (first through
   select — 5 pure ordering operations)
4. to_seq, filter last (may be harder)

## Rules

- Read existing proved Mt delegation methods in the same files FIRST
  (insert, delete, find, size) — copy their pattern
- Read `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs`
- Run `scripts/validate.sh` after changes. 0 errors required.
- Write report to `plans/agent4-round35-report.md`
- Commit, push to `agent4/ready`
