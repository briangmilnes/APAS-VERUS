# R201 Prompt — Agent 2: Iterator expansion — BST/Map/Table/Graph coverage. AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent2`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER add `assume`, `admit`, `accept`, or `external_body`** —
   except the standard `assume(iter_invariant(self))` inside hand-rolled
   iterator `next()` bodies (documented project policy).
6. **NEVER modify existing verified code to make new iterators work.**
   Iterators are additive. If adding one requires changing a trait,
   spec fn, or existing proof, STOP and report.
7. **NEVER touch `Example*.rs` or `Problem*.rs` files.**

## Read all standards first.

Especially:
- `src/standards/iterators_standard.rs` (10-component pattern).
- `src/standards/wrapping_iterators_standard.rs` (delegate-to-backing pattern).
- `docs/APAS-VERUSIterators.rs` (6 iteration test patterns).
- `docs/VerusOptimizationsApplied.md` (Verus SMT patterns).

## Context

R199 (agent2) added iterators to 19 high-priority files. R201
continues into the next tier of ~15–18 candidates: the BST
non-Splay variants, Chap41 OrdKeyMap, Chap42 TableMtEph, Chap43
Ordered* variants, and Chap52 AdjTableGraph / EdgeSetGraph variants.

R199 delivered RTTs on all 19 but only 4 PTT files (~21%). R201
uses a **strict PTT 1:1 rule**: every new iterator in Phase 1 must
get a PTT in Phase 4, unless specifically documented as blocked.

## Goal

Add full 10-component iterators to ~15–18 more collection types.
Three-phase execution, **PTTs last in one batch** (PTT compile is
~4 min; per-file PTT runs would blow the round budget).

## Plan (4 strict phases — do not reorder)

### Phase 1: Audit and select targets

Build `plans/r201-iterator-expansion-inventory.md` with one row per
target. The **expected target list** (verify each exists and lacks
iterator infrastructure):

| # | Chap | File | Notes |
|---|---|---|---|
| 1 | 37 | BSTAVLStEph | AVL binary search tree, sequential ephemeral |
| 2 | 37 | BSTAVLMtEph | AVL binary search tree, multi-threaded ephemeral |
| 3 | 37 | BSTBBAlphaStEph | BB[alpha] weight-balanced BST |
| 4 | 37 | BSTBBAlphaMtEph | BB[alpha] MtEph variant |
| 5 | 37 | BSTPlainStEph | Unbalanced BST |
| 6 | 37 | BSTPlainMtEph | Unbalanced BST MtEph |
| 7 | 37 | BSTRBStEph | Red-black BST |
| 8 | 37 | BSTRBMtEph | Red-black BST MtEph |
| 9 | 41 | OrdKeyMap | ordered key-value map |
| 10 | 42 | TableMtEph | multi-threaded Table |
| 11 | 43 | OrderedSetMtEph | multi-threaded ordered set |
| 12 | 43 | OrderedTableMtPer | multi-threaded persistent ordered table |
| 13 | 43 | AugOrderedTableStEph | augmented ordered table (reduction monoid) |
| 14 | 43 | AugOrderedTableStPer | augmented ordered table, persistent |
| 15 | 52 | AdjTableGraphStEph | adjacency-table graph, sequential ephemeral |
| 16 | 52 | AdjTableGraphStPer | adjacency-table graph, sequential persistent |
| 17 | 52 | EdgeSetGraphStEph | edge-set graph, sequential ephemeral |
| 18 | 52 | EdgeSetGraphStPer | edge-set graph, sequential persistent |

For each row:
- `grep -l "impl IntoIterator\|ForLoopGhostIterator" <file>` — confirm
  the file does NOT already have iterator infrastructure. If it does,
  mark "already covered" and skip.
- Read the file's trait to identify the natural element type for
  iteration (element of the set, or key of the map, or `Pair<K,V>` for
  tables).

Close any rows where a backing collection already has an iterator
(e.g., AdjTableGraph internally wraps an OrderedTable which may now
have one). For those, prefer the "wrapping iterator" pattern per
`src/standards/wrapping_iterators_standard.rs`.

If any entry turns out to be substantively blocked (e.g., `TableMtEph`
has a View shape incompatible with clean iteration), STOP on that
entry, note the blocker, and move on. Do NOT add an iterator that
requires new `external_body` or assumes beyond `iter_invariant`.

### Phase 2: Implement iterators

For each selected target, add the 10-component iterator set from
`src/standards/iterators_standard.rs`:

1. Custom iterator struct (`<Type>Iter<'a, …>`).
2. `View` impl for the iterator struct.
3. `ForLoopGhostIteratorNew` impl.
4. `ForLoopGhostIterator` impl.
5. `Iterator for <Type>Iter` (inside `verus!` where possible).
6. `iter_invariant` spec fn (namespace it: `iter_invariant_<type>`
   — avoid glob-import E0659 collisions that bit R199's ArraySetStEph).
7. `IntoIterator for &<Type>` (borrow iterator).
8. `IntoIterator for <Type>` (consume iterator) — only if a consumable
   iteration makes sense for this type. For tree-shaped / lock-wrapped
   types, consume may not be meaningful; skip with a documented reason.
9. `.iter()` method.
10. `.into_iter()` method (if (8) exists).

**For BST variants (rows 1–8)**: the canonical pattern is
snapshot-based — call `in_order()` or equivalent to produce a Vec,
then iterate via `slice::Iter`. See `BSTSplayStEph`'s iterator
(landed in R199) as the reference.

**For Mt variants (BST*MtEph, TableMtEph, OrderedSetMtEph,
OrderedTableMtPer)**: iterator must hold a read lock or a snapshot.
A snapshot pattern is simpler and avoids Send/Sync proof complexity
— acquire-read, copy out the keys/entries, release-read, iterate
the snapshot. See `AVLTreeSetMtPer`'s iterator (landed in R199).

**For graph variants (rows 15–18)**: iteration semantics =
iterate over edges (or vertices — pick one, document it).

**Naming rule** (prevent E0659 like ArraySetStEph hit): name the
`iter_invariant` spec fn `iter_invariant_<modulename>`, with the
module name in lowercase without internal underscores.

### Phase 3: Validate + RTTs

After all Phase 2 iterators written:

```bash
scripts/validate.sh     # full — iterator specs may cross chapters
scripts/rtt.sh
```

Both must be clean before Phase 4.

For each new iterator, add an RTT test file or append to existing
`tests/ChapNN/Test<File>.rs`:
- `test_<type>_iter_empty` — iterate over empty, count 0.
- `test_<type>_iter_basic` — iterate over small populated instance,
  check collected elements.
- `test_<type>_into_iter` — consume iteration (if implemented).

Run `scripts/rtt.sh` again. All must pass.

### Phase 4: PTTs (1:1 with Phase 2)

**Strict rule this round**: every iterator added in Phase 2 must
get a PTT file in this phase. Exceptions must be documented with
specific blockers.

For each iterator, create
`rust_verify_test/tests/ChapNN/Prove<File>.rs`:

- Model on `rust_verify_test/tests/Chap18/ProveArraySeqStEph.rs`.
- Cover the 6 patterns (skip consume patterns for types without
  `IntoIterator for Self`, marked with `// SKIPPED: <reason>` in the
  file).
- Register in `rust_verify_test/Cargo.toml`.

**Write all PTT files first, then run `scripts/ptt.sh` exactly ONCE.**
This is the hard budget rule — PTT compile is ~4 min, per-file runs
blow the round.

Expected PTT delta: **+~18 files × 4–6 patterns = +70–100 tests**.

## Out of scope

- Priority queues (Chap45 — legitimate skip per R199 rationale).
- Backfilling PTTs for R199's iterators (agent1's R201 scope).
- AIR-bug verification on Chap18/ArraySeqStPer (agent1's R201 scope).
- BSTTreapStEph delete test (agent1's R201 scope).
- Any chapter not on the target list above.

## Rules

- **Hard PTT 1:1**: no iterator without a PTT file. If blocked,
  document why. R199 added 4 PTTs for 19 iterators and the user
  explicitly flagged the gap.
- **Namespace iter_invariant**: always `iter_invariant_<module>` to
  avoid the glob-import E0659 that bit ArraySetStEph.
- **Never modify existing Vec<_> / OrderedTable / HashMap internals**.
  Snapshot iterators should read, not mutate.
- **If Z3 destabilizes an existing proof after adding an iterator
  spec fn** (the `BSTParaTreapMtEph::join_with_priority` pattern that
  R199 hit): bridge with an explicit assertion. Do NOT add holes,
  do NOT remove the new iterator.

## Commit strategy

Commit in phases so partial work can be reverted if a phase breaks:

1. Phase 2 iterators landed + validate clean:
   `R201 Agent 2: iterators for N more collection types (validated)`
2. Phase 3 RTTs:
   `R201 Agent 2: +M RTTs for new iterators`
3. Phase 4 PTTs:
   `R201 Agent 2: +K PTTs for new iterators (1:1 coverage)`

## Report

Write `plans/agent2-round201-report.md` with:

- Phase 1 inventory outcome (# targeted, # already covered, # blocked).
- Phase 2: iterators added (per-file table with element type, bounds,
  any Z3 bridge assertions used).
- Phase 3: validate + rtt numbers.
- Phase 4: PTT count delta vs 237 baseline, per-file pattern coverage,
  any documented skips.
- Any Verus instability encountered and how resolved.

## RCP

See commit strategy above. Final:

```
git push
```
