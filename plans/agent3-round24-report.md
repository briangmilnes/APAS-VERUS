# Agent 3 — Round 24 Report

## Mission

Prove hash table insert and delete operations across Chap47 chained hash table strategy
files. Use ghost hash infrastructure from R23 and proof lemmas from R22. Apply standard
PartialEq eq bridge `assume` pattern in entry-level scan loops. Target: close insert +
delete external_body holes across chained hash table files.

## Summary

R24 proved **insert and delete** on all 3 chained hash table strategy files, removing
**6 external_body holes** from algorithmic code. The proofs use a "filter out all key
entries" approach for delete (avoids needing uniqueness invariants), ghost `prefix_map`
tracking through scan loops, and explicit Map insert/remove commutativity assertions for
the SMT solver. Two new helper lemmas were added to ParaHashTableStEph.rs. The flat hash
tables (LinProb, QuadProb, DoubleHash) are architecturally blocked by `spec_hashtable_wf`
requiring keys at their hash slot, incompatible with open addressing probing.

## Changes

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 47 | ParaHashTableStEph.rs | Added `lemma_seq_pairs_no_key_not_in_map` and `lemma_seq_pairs_has_key_in_map` proof fns |
| 2 | 47 | VecChainedHashTableStEph.rs | Proved insert (R24 partial), proved delete with filter-all-keys loop + eq/clone bridges |
| 3 | 47 | LinkedListChainedHashTableStEph.rs | Proved insert (R24 partial), proved delete mirroring VecChained approach |
| 4 | 47 | StructChainedHashTable.rs | Strengthened chain_insert/chain_delete ensures, added eq bridges, proved table-level insert and delete |

## Holes Before/After

| # | Chap | File | Before (ext) | After (ext) | After (assume) | Delta (ext) |
|---|------|------|-------------|-------------|----------------|-------------|
| 1 | 47 | ParaHashTableStEph.rs | 2 | 2 | 0 | 0 |
| 2 | 47 | ChainedHashTable.rs | 0 | 0 | 0 | 0 |
| 3 | 47 | FlatHashTable.rs | 0 | 0 | 0 | 0 |
| 4 | 47 | VecChainedHashTableStEph.rs | 4 | 2 | 6 | -2 |
| 5 | 47 | LinkedListChainedHashTableStEph.rs | 4 | 2 | 6 | -2 |
| 6 | 47 | StructChainedHashTable.rs | 4 | 2 | 2 | -2 |
| 7 | 47 | LinProbFlatHashTableStEph.rs | 4 | 4 | 0 | 0 |
| 8 | 47 | QuadProbFlatHashTableStEph.rs | 4 | 4 | 0 | 0 |
| 9 | 47 | DoubleHashFlatHashTableStEph.rs | 4 | 4 | 0 | 0 |
| | | **Total** | **26 ext** | **20 ext** | **14 assume** | **-6 ext** |

All 14 assumes are approved-pattern eq/clone bridges per CLAUDE.md standards:
- 4 eq bridges (PartialEq spec bridge in insert/delete scan loops)
- 6 clone bridges in VecChained/LinkedList delete (Key + Value clone in filter loops)
- 4 clone bridges in VecChained/LinkedList clone helper functions (pre-existing from R24 partial)

## Techniques

1. **Filter-out-key approach for delete**: Build new Vec/chain excluding all key-matching
   entries. Resulting map equals `original_map.remove(key)` regardless of duplicates.
   Avoids needing a uniqueness invariant on the entry sequence.

2. **Ghost prefix_map**: Track `spec_seq_pairs_to_map(original.subrange(0, i))` through
   each loop iteration. The loop invariant connects new_bucket's map to
   `prefix_map.remove(key)`.

3. **Map commutativity assertions**: SMT solver needs explicit hints:
   - `m.insert(a,v).remove(b) =~= m.remove(b).insert(a,v)` when `a != b`
   - `m.insert(k,v).remove(k) =~= m.remove(k)`
   - `m.insert(a,v1).insert(b,v2) =~= m.insert(b,v2).insert(a,v1)` when `a != b`

4. **Identity update trick**: `lemma_table_to_map_update_contains(table, idx, table[idx], key)`
   with `assert(table.update(idx, table[idx]) =~= table)` proves entry containment implies
   table containment.

5. **StructChained stronger ensures**: Strengthened `chain_insert` and `chain_delete` from
   weak ensures (just `dom().contains(key)` / partial spec) to full Map equality ensures
   (`spec_chain_to_map(result) == spec_chain_to_map(chain).insert(key, value)` etc.).

## Verification

- `scripts/validate.sh`: 4026 verified, 0 errors
- `scripts/rtt.sh`: 2613 tests passed

## Remaining Holes (34 total: 20 external_body, 14 assume)

**external_body (20)**:
- `call_hash_fn`, `compute_second_hash` in ParaHashTableStEph.rs (2) — inherently external
- `lookup`, `resize` in VecChained, LinkedList, StructChained (6) — provable future targets
- `insert`, `lookup`, `delete`, `resize` in LinProb, QuadProb, DoubleHash (12) — blocked

**assume (14)**: All approved eq/clone bridge patterns in proven insert/delete functions.

## Architectural Blocker: Flat Hash Tables

The 3 flat hash table files (LinProb, QuadProb, DoubleHash) with 12 external_body holes
are **architecturally blocked**. `spec_hashtable_wf` requires keys to reside at their
hash slot: `j != (spec_hash)(k) % size ==> !table[j].contains(k)`. Open addressing with
probing displaces keys to non-hash slots on collision, violating this invariant after any
collision-causing insert. Fixing this requires redesigning `spec_hashtable_wf` to
accommodate probing semantics (e.g., a "reachable from hash slot via probing" predicate).
