# Agent 2 — Round 9: Finish Chap47 Hash Tables + Close Chap52

## Mission

Finish the hash table proofs in Chap47 (20 ext_body remaining). Close Chap52 (1 ext_body).

**Chap52 is 1 hole. Close it. No excuses.** Last round's agent deferred it with
"parallel filter spec" — that is not acceptable. Do the work.

## Your Files (ONLY touch these)

Chap47 (9 files, 20 ext_body remaining):
1. `src/Chap47/ChainedHashTable.rs` — 2 ext_body (lines 64, 101)
2. `src/Chap47/DoubleHashFlatHashTableStEph.rs` — 4 ext_body (lines 40, 150, 187, 196)
3. `src/Chap47/LinProbFlatHashTableStEph.rs` — 3 ext_body (lines 116, 153, 161)
4. `src/Chap47/LinkedListChainedHashTableStEph.rs` — 1 ext_body (line 176)
5. `src/Chap47/ParaHashTableStEph.rs` — 5 ext_body (lines 55, 63, 72, 82, 92)
6. `src/Chap47/QuadProbFlatHashTableStEph.rs` — 3 ext_body (lines 118, 155, 163)
7. `src/Chap47/StructChainedHashTable.rs` — 1 ext_body (line 256)
8. `src/Chap47/VecChainedHashTableStEph.rs` — 1 ext_body (line 180)

Chap52:
9. `src/Chap52/EdgeSetGraphMtPer.rs` — 1 ext_body

**DO NOT touch files in any other chapter.**

## CRITICAL: Do Not Restructure Chap47

**DO NOT restructure the Rust layout of the hash table files.** The struct definitions,
trait hierarchy, module organization, and type relationships took hours of careful
design. Prove the stubs in place. If you think the structure needs changing, stop
and explain why in your report.

## What Was Already Proved (Round 8)

A previous agent proved 19 of the original 39 Chap47 holes:
- insert/lookup/delete across all 6 concrete hash table implementations
- 3 hash_index methods
- 2 FlatHashTable default methods

The 20 remaining are the harder ones. Read the files to understand what's left:
likely iterators, resize/rehash, probe termination proofs, parallel operations.

## Chap47 Strategy

**Flat hash tables** (LinProb, QuadProb, DoubleHash):
- The remaining ext_body are likely iterator, resize, or probe functions.
- Probe termination: for a non-full table, the probe sequence must terminate.
  The load factor invariant (`num_entries < capacity`) guarantees this.
- Resize: build new table, rehash all entries. Prove view preservation.

**ParaHashTableStEph** (5 ext_body):
- These are parallel hash table operations. Use fork-join pattern.
- Read `src/standards/arc_rwlock_for_hfscheduler_standard.rs` for the threading pattern.

**Chained hash tables** (1 each in LinkedList, Struct, Vec + 2 in ChainedHashTable):
- Mostly done — remaining stubs are likely iterators or edge cases.

## Chap52 Strategy

1 ext_body in EdgeSetGraphMtPer.rs. Read the StEph variant for context.
This is likely a thread-boundary wrapper (out_neighbors or similar).
Use the Arc<RwLock> standard pattern.

## Standards to Read First

1. `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs` — for ParaHashTable
2. `src/standards/arc_rwlock_for_hfscheduler_standard.rs` — Arc<RwLock> bridges
3. `src/standards/collection_iterators_standard.rs` — if iterator stubs remain

## Validation

```bash
scripts/validate.sh          # must show 0 errors
scripts/holes.sh src/Chap47/ # track Chap47
scripts/holes.sh src/Chap52/ # verify Chap52 closed
```

## Target

**Chap47**: 20 → ≤ 10. Prove at least the flat hash table stubs.
**Chap52**: Closed (0 holes). This is non-negotiable.

## When Done

Push to `agent2/ready`. Write `plans/agent2-round9-report.md`.
