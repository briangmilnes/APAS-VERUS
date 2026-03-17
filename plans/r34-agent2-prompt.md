# R34 Agent 2: Chap47 Hash Table Proofs

## Goal

Prove external_body functions in Chap47 hash table implementations.
14 real holes across 7 files.

## Targets (by file)

### Open-addressing tables (insert + delete + resize)

1. **LinProbFlatHashTableStEph.rs** (3 external_body):
   - `insert` (line 94)
   - `delete` (line 230)
   - `resize` (line 260)

2. **DoubleHashFlatHashTableStEph.rs** (3 external_body + 1 assume):
   - `insert` (line 91)
   - `delete` (line 269)
   - `resize` (line 299)
   - lookup assume (line 129) — `forall|j| 0 <= j < m ==> ...`

3. **QuadProbFlatHashTableStEph.rs** (3 external_body):
   - `insert` (line 76)
   - `delete` (line 264)
   - `resize` (line 294)

### Chained tables (resize only)

4. **LinkedListChainedHashTableStEph.rs** (1 external_body):
   - `resize` (line 434)

5. **StructChainedHashTable.rs** (1 external_body):
   - `resize` (line 399)

6. **VecChainedHashTableStEph.rs** (1 external_body):
   - `resize` (line ~similar)

### Parallel hash table

7. **ParaHashTableStEph.rs** (1 real + 1 FP):
   - `split` (line 463) — real hole
   - `compute_second_hash` (line 493) — OPAQUE_EXTERNAL FP, skip

## Pattern

All three open-addressing tables follow the same structure: `FlatHashTable<T>`
backing store with linear/quadratic/double-hash probing. insert/delete
operate on the probe sequence. resize creates a new table and reinserts.

The chained tables all have the same resize pattern: allocate new buckets,
rehash all entries.

## Priority

Start with the 3 chained table resizes (mechanical, same pattern).
Then the open-addressing insert/delete. Resize last (depends on insert).

## Rules

- Run `scripts/validate.sh` after changes. 0 errors required.
- Write report to `plans/agent2-round34-report.md`.
- Commit, push to `agent2/ready`.
