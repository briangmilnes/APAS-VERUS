# Agent 3 — Round 25 Report

## Goal

Chap47 hash table proof work in three parts:
1. Define per-strategy wf specs for 3 flat hash table files
2. Prove lookup (remove external_body) in 3 chained hash table files
3. Replace `requires true` with real specs across 4 files

## Results

- Verification: **4083 verified, 0 errors**
- RTT: **2613 passed, 0 skipped**
- Chap47 holes: **34 → 37** (net +3: traded 3 external_body for 6 eq/clone assumes)

## Per-File Changes

| # | Chap | File | Holes Before | Holes After | Delta | Notes |
|---|------|------|-------------|-------------|-------|-------|
| 1 | 47 | FlatHashTable.rs | 0 | 0 | 0 | Added `spec_flat_has_key` helper |
| 2 | 47 | LinProbFlatHashTableStEph.rs | 4 | 4 | 0 | Added `spec_linprobflathashsteph_wf` |
| 3 | 47 | QuadProbFlatHashTableStEph.rs | 4 | 4 | 0 | Added `spec_quadprobflathashsteph_wf` |
| 4 | 47 | DoubleHashFlatHashTableStEph.rs | 4 | 4 | 0 | Added `spec_doublehashflathashsteph_wf` |
| 5 | 47 | VecChainedHashTableStEph.rs | 8 | 9 | +1 | Proved lookup (-1 ext_body, +2 assumes) |
| 6 | 47 | LinkedListChainedHashTableStEph.rs | 8 | 9 | +1 | Proved lookup (-1 ext_body, +2 assumes) |
| 7 | 47 | StructChainedHashTable.rs | 4 | 5 | +1 | Proved lookup (-1 ext_body, +2 assumes) |
| 8 | 47 | ParaHashTableStEph.rs | 2 | 2 | 0 | Real requires, 2 new proof lemmas, trigger fix |

## Part 1: Per-Strategy WF Specs (3 files)

Created open spec fns encoding open-addressing invariants for each probing strategy:

- **`spec_linprobflathashsteph_wf`**: No duplicate keys + linear probe chain integrity (all slots from hash(k) to slot i are non-Empty).
- **`spec_quadprobflathashsteph_wf`**: No duplicate keys + quadratic probe chain integrity (exists attempt n with (h+n^2)%m == i and no Empty gaps).
- **`spec_doublehashflathashsteph_wf`**: No duplicate keys + double hash probe chain integrity (exists step s>=1 and attempt n with (h+n*s)%m == i and no Empty gaps).

All specs use `spec_flat_has_key` (new helper in FlatHashTable.rs) to avoid `EntryTrait` trait bounds that prevent unconstrained type params in wf specs. Explicit `#![trigger]` annotations on existential quantifiers.

## Part 2: Prove Lookup (3 files, -3 external_body)

Removed `external_body` from lookup in all three chained hash table implementations:

- **VecChainedHashTableStEph**: Backward scan matching `spec_seq_pairs_to_map` last-wins semantics. Ghost alias `bv = table.table@[index]@` avoids exec-to-spec bridging issues.
- **LinkedListChainedHashTableStEph**: Same backward scan pattern using `table.table@[index].seq@`.
- **StructChainedHashTable**: Strengthened `chain_lookup` ensures to full correctness spec (`spec_chain_to_map`). Forward scan matches head-wins semantics.

Two new proof lemmas in ParaHashTableStEph.rs:
- `lemma_seq_pairs_last_key_gives_value`: If pairs[i].0 == key and no later index has the same key, the map value equals pairs[i].1.
- `lemma_table_to_map_unique_entry_value`: If key appears in exactly one entry's map, spec_table_to_map gives that entry's value.

Each lookup uses only approved eq/clone bridge assumes.

## Part 3: Replace `requires true` (7 instances across 4 files)

| # | Chap | File | Function | Change |
|---|------|------|----------|--------|
| 1 | 47 | ParaHashTableStEph.rs | `metrics` | `requires true` → `requires spec_hashtable_wf(table)` |
| 2 | 47 | ParaHashTableStEph.rs | `loadAndSize` | `requires true` → `requires spec_hashtable_wf(table)` |
| 3 | 47 | VecChainedHashTableStEph.rs | `clone_vec_pairs` | Removed `requires true` (no precondition needed) |
| 4 | 47 | LinkedListChainedHashTableStEph.rs | `clone_linked_list_entry` | Removed `requires true` (no precondition needed) |
| 5 | 47 | StructChainedHashTable.rs | `chain_insert` | Removed `requires true` (no precondition needed) |
| 6 | 47 | StructChainedHashTable.rs | `chain_lookup` | Removed `requires true` (no precondition needed) |
| 7 | 47 | StructChainedHashTable.rs | `chain_delete` | Removed `requires true` (no precondition needed) |

## Additional Fix

- Fixed pre-existing auto-trigger warning in `ParaHashTableStEph.rs:322` with explicit `#[trigger]`.

## Techniques Used

- Ghost alias pattern (`let ghost bv = table.table@[index]@`) to avoid exec-to-spec reference bridging issues in Verus.
- `spec_flat_has_key` helper to bypass `EntryTrait` trait bounds on spec functions.
- Explicit `#![trigger]` on existential quantifiers in wf specs.
- Backward scan for last-wins semantics in Vec/LinkedList chained tables.
- Forward scan for head-wins semantics in StructChained table.

## Remaining Holes (37)

- 12 × external_body on flat hash table insert/lookup/delete/resize (4 per probing strategy)
- 2 × external_body on call_hash_fn/compute_second_hash (hash function wrappers)
- 3 × external_body on resize (one per chained table)
- 20 × assume (all eq/clone bridge assumes in scan loops)
