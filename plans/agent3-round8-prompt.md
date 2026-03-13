# Agent 3 — Round 8: Chap47 Hash Tables + Quick Wins

## Mission

Prove hash table implementations in Chap47 (39 external_body holes). Close Chap50 (1 hole)
and Chap52 (1 hole) as quick wins.

**Your success metric is holes eliminated and chapters closed.** Chap50 and Chap52 are
one hole each — there is no excuse for not closing them. For Chap47, every external_body
is an algorithmic stub that needs a real proof body. These are hash table operations:
insert, find, delete, resize. The algorithms are straightforward. Prove them.

## Your Files (ONLY touch these)

Chap47 (9 files, 39 holes — all external_body):
1. `src/Chap47/VecChainedHashTableStEph.rs` — 5 ext_body
2. `src/Chap47/LinkedListChainedHashTableStEph.rs` — 5 ext_body
3. `src/Chap47/StructChainedHashTable.rs` — 5 ext_body
4. `src/Chap47/LinProbFlatHashTableStEph.rs` — 6 ext_body
5. `src/Chap47/QuadProbFlatHashTableStEph.rs` — 6 ext_body
6. `src/Chap47/DoubleHashFlatHashTableStEph.rs` — 7 ext_body
7. `src/Chap47/ChainedHashTable.rs` — 2 ext_body
8. `src/Chap47/FlatHashTable.rs` — 2 ext_body
9. `src/Chap47/ParaHashTableStEph.rs` — 1 ext_body

Quick wins:
10. `src/Chap50/MatrixChainMtEph.rs` — 1 assume
11. `src/Chap52/EdgeSetGraphMtPer.rs` — 1 external_body

## CRITICAL WARNING: Do Not Restructure Chap47

**DO NOT restructure the Rust layout of the hash table files.** The struct definitions,
trait hierarchy, module organization, and type relationships in Chap47 took hours of
careful design work. Your job is to fill in proof bodies for the external_body stubs,
not to reorganize the code. Keep the existing structure exactly as-is. Change only what
is inside the `external_body` function bodies — replace them with proven implementations.

If you think the structure needs changing to make a proof work, stop and explain why in
your report. Do not restructure first and explain later.

## Strategy

### Hash Table Approach
The hash table files follow two patterns:

**Chained hash tables** (Vec, LinkedList, Struct variants):
- Array of buckets, each bucket is a list of key-value pairs.
- Insert: hash key → find bucket → append to list.
- Find: hash key → find bucket → search list.
- Delete: hash key → find bucket → remove from list.
- The proof obligation is: view (abstract map) matches the concrete bucket contents.

**Flat/open-addressing hash tables** (Linear, Quadratic, Double-hash probing):
- Single array, collision resolution by probing.
- Insert: hash key → probe until empty slot → insert.
- Find: hash key → probe until found or empty.
- Delete: hash key → mark as tombstone.
- Harder proofs: need to show probe sequence terminates, no infinite loops.

### Start with the easiest
1. **VecChainedHashTableStEph.rs** (5 ext_body) — simplest chained variant.
2. **LinkedListChainedHashTableStEph.rs** (5 ext_body) — similar pattern.
3. **StructChainedHashTable.rs** (5 ext_body) — similar pattern.
4. **ChainedHashTable.rs** (2 ext_body) — generic wrapper, prove after concrete variants.
5. Then flat hash tables (harder: probing termination proofs).
6. **ParaHashTableStEph.rs** (1 ext_body) — load factor computation.

### Quick Wins First
Start with Chap50 and Chap52 to build momentum:
- **Chap50 MatrixChainMtEph.rs**: 1 assume — likely an overflow/bounds check. Read the
  StEph variant for context.
- **Chap52 EdgeSetGraphMtPer.rs**: 1 ext_body — probably an out_neighbors thread wrapper.
  Read the StEph variant and use the arc_rwlock pattern.

## Standards to Read First

1. `src/standards/mod_standard.rs` — module layout
2. `src/standards/spec_wf_standard.rs` — wf predicate conventions
3. `src/standards/partial_eq_eq_clone_standard.rs` — eq/clone workaround

## Validation

```bash
scripts/validate.sh          # must show 0 errors
scripts/holes.sh src/Chap47/ # track Chap47 progress
scripts/holes.sh src/Chap50/ # verify Chap50 closed
scripts/holes.sh src/Chap52/ # verify Chap52 closed
```

## Target

**Chap50**: Closed (0 holes). No excuses.
**Chap52**: Closed (0 holes). No excuses.
**Chap47**: 39 → ≤ 20. At least all 3 chained hash table variants proven.

## When Done

Push to `agent3/ready`. Write `plans/agent3-round8-report.md` with:
- Holes before/after per file (table)
- Chapters closed
- Verification counts
- Techniques used
- Remaining holes with what blocks them
- Commit hash
