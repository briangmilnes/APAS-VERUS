# Agent 3 — Round 20: Prove Chap47 Hash Table Operations (33 holes)

## Mission

Prove hash table operations across 8 Chap47 files. All dependencies are clean.
33 holes: 26 external_body + 7 fn_missing_spec.

## Required Reading

- `src/Chap47/ParaHashTableStEph.rs` — the shared `HashTable` struct and
  `HashTableOps` trait. This defines the View and the ensures for all operations.
- `src/Chap47/ChainedHashTable.rs` — the ONE clean file. Read how it proves things.
- `src/standards/view_standard.rs` — View patterns.

## Architecture

Chap47 has a generic `HashTable<Key, Value, Entry, Metrics, H>` struct with a
`HashTableOps` trait defining `insert`, `lookup`, `delete`, `resize`. Five probe
strategies implement this trait:

| # | File | Strategy | Holes |
|---|------|----------|-------|
| 1 | LinProbFlatHashTableStEph.rs | Linear probing | 4 ext_body |
| 2 | QuadProbFlatHashTableStEph.rs | Quadratic probing | 4 ext_body |
| 3 | DoubleHashFlatHashTableStEph.rs | Double hashing | 4 ext_body |
| 4 | LinkedListChainedHashTableStEph.rs | Linked list chains | 4 ext_body |
| 5 | VecChainedHashTableStEph.rs | Vec chains | 4 ext_body |
| 6 | StructChainedHashTable.rs | Struct-based chains | 4 ext_body |
| 7 | FlatHashTable.rs | Shared flat probe logic | 1 fn_missing_ensures |
| 8 | ParaHashTableStEph.rs | Shared infra | 2 ext_body + 3 fn_missing |

Plus StructChainedHashTable has 3 helper fns missing requires/ensures.

## Strategy

### Phase 1: Fix fn_missing_spec (7 holes)

These are functions WITHOUT requires/ensures. Add them first — they're spec holes,
not proof holes.

**ParaHashTableStEph.rs** (3 missing):
- `createTable` (line 183): missing requires. Add `requires initial_size > 0` or
  whatever the constructor needs.
- `metrics` (line 254): missing requires. Probably needs `requires table.spec_wf()`.
- `loadAndSize` (line 262): missing requires. Same pattern.

**FlatHashTable.rs** (1 missing):
- `lookup_with_probe` (line 84): missing ensures. Add the lookup ensures from the
  trait — `found matches Some(v) ==> self@.dom().contains(key@) && self@[key@] == v@`.

**StructChainedHashTable.rs** (3 missing):
- `chain_insert` (line 103): missing requires. These are internal helpers — add
  appropriate requires/ensures for chain operations.
- `chain_lookup` (line 136): missing requires.
- `chain_delete` (line 157): missing requires.

### Phase 2: Prove hash table operations (26 external_body)

All 5 probe strategies implement the same 4 operations. Pick the simplest strategy
first, prove it, then adapt for the others.

**Recommended order:**
1. **VecChainedHashTableStEph.rs** — Vec-based chains are simplest (Vec has good
   vstd support). Prove insert/lookup/delete/resize here first.
2. **LinkedListChainedHashTableStEph.rs** — similar to Vec chains.
3. **StructChainedHashTable.rs** — struct-based chains.
4. **LinProbFlatHashTableStEph.rs** — linear probing (flat table).
5. **QuadProbFlatHashTableStEph.rs** — adapt from linear.
6. **DoubleHashFlatHashTableStEph.rs** — adapt from linear.

### Proof Pattern for Hash Table Operations

The core invariant for all hash tables:
- `self@` (the View) is a `Map<Key, Value>`.
- `insert(key, value)` ensures `self@ == old(self)@.insert(key@, value@)`.
- `lookup(key)` ensures `found matches Some(v) ==> self@[key@] == v@`.
- `delete(key)` ensures `self@ == old(self)@.remove(key@)`.

The proof challenge is connecting the physical storage (array of entries, chains)
to the logical map view. You need:
1. A well-formedness invariant relating physical entries to the map.
2. Proof that each operation preserves the invariant.
3. Proof that the operation's ensures follows from the invariant.

### ParaHashTableStEph.rs Infrastructure (2 external_body)

- `call_hash_fn` (line 85): Wraps hash function call. May need external_body
  if hash functions are opaque. Check if there's a spec for the hash.
- `compute_second_hash` (line 113): Uses `std::hash::Hash`. Likely needs
  external_body (Rust std hash isn't verified). Leave if so.

## Procedure

1. Read `ParaHashTableStEph.rs` — understand HashTable struct, View, trait.
2. Read `ChainedHashTable.rs` — the clean file, see what patterns work.
3. Fix fn_missing_spec (Phase 1) — add requires/ensures.
4. `scripts/validate.sh`
5. Prove VecChainedHashTableStEph.rs operations (Phase 2, start simple).
6. `scripts/validate.sh` — iterate.
7. Move to next strategy files.
8. Final `scripts/validate.sh` — 0 errors.

## Important

- Start with fn_missing_spec — those are quick wins.
- Prove ONE strategy fully before moving to the next.
- Hash function calls (`call_hash_fn`, `compute_second_hash`) may genuinely need
  external_body — std::hash isn't verified in Verus. That's OK.
- `resize` may be the hardest operation — save it for last within each strategy.
- Do NOT modify files outside Chap47.
- Do NOT add `assume` or `accept`.

## Deliverables

- fn_missing_spec holes fixed (7).
- As many external_body holes proved as possible.
- `plans/agent3-round20-report.md`
- 0 errors on validate.
- Commit + push to `agent3/ready`.
