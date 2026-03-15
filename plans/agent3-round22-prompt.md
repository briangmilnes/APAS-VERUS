# Agent 3 — Round 22: Chap47 Hash Table Spec Strengthening + Proofs

## Mission

Strengthen EntryTrait specs and prove hash table insert/delete operations. The 26
`external_body` holes are NOT all equally blocked. Insert and delete can be proved TODAY
by strengthening entry-level specs without any ghost hash function.

## Key Insight

The previous round concluded that all 26 holes are blocked by the opaque hash function.
That analysis was wrong for insert and delete. Here's why:

**Insert does NOT need to know WHERE the hash puts the key.** The proof is:
1. `call_hash_fn` gives `index < table_size` (already ensured).
2. `table[index].insert(key, value)` modifies that one entry.
3. If EntryTrait::insert ensures `== old.insert(key, value)`, then
   `spec_table_to_map` after the update equals `old(table)@.insert(key, value)`.

**Delete is similar** — you don't need to know which bucket, just that the bucket's
delete behaves correctly.

**Lookup DOES need the ghost hash** for full correctness (same key → same bucket).
Defer lookup proofs to a future round.

## Step 1: Strengthen EntryTrait::insert ensures

Current (too weak):
```rust
fn insert(&mut self, key: Key, value: Value)
    ensures self.spec_entry_to_map().dom().contains(key);
```

Strengthen to:
```rust
fn insert(&mut self, key: Key, value: Value)
    ensures self.spec_entry_to_map() == old(self).spec_entry_to_map().insert(key, value);
```

Prove this for all 4 EntryTrait implementations:
- `Vec<(Key, Value)>` in VecChainedHashTableStEph.rs
- `LinkedListStEphS<(Key, Value)>` in LinkedListChainedHashTableStEph.rs
- `ChainList<Key, Value>` in StructChainedHashTable.rs
- `FlatEntry<Key, Value>` in FlatHashTable.rs

## Step 2: Strengthen EntryTrait::delete ensures

Current:
```rust
fn delete(&mut self, key: &Key) -> (deleted: bool)
    ensures !deleted ==> self.spec_entry_to_map() == old(self).spec_entry_to_map();
```

Strengthen to:
```rust
fn delete(&mut self, key: &Key) -> (deleted: bool)
    ensures self.spec_entry_to_map() == old(self).spec_entry_to_map().remove(*key);
```

Prove for all 4 implementations.

## Step 3: Prove table-level insert

With the strengthened entry specs, prove `insert` in each strategy file:
```rust
fn insert(table: &mut HashTable<...>, key: Key, value: Value)
    ensures table@ == old(table)@.insert(key, value);
```

Use `lemma_table_to_map_update_contains` (already exists from R20) as a starting point.
You'll need to extend it to prove full map equality, not just `dom.contains`.

## Step 4: Prove table-level delete

Similar approach — strengthen ensures to `table@ == old(table)@.remove(*key)`.

## Files (9 total)

| # | Chap | File | Holes | Role |
|---|------|------|-------|------|
| 1 | 47 | ParaHashTableStEph.rs | 2 | Infrastructure (call_hash_fn, compute_second_hash) |
| 2 | 47 | ChainedHashTable.rs | 0 | Base chained algorithm |
| 3 | 47 | FlatHashTable.rs | 0 | Base flat algorithm |
| 4 | 47 | VecChainedHashTableStEph.rs | 4 | Vec-backed chained strategy |
| 5 | 47 | LinkedListChainedHashTableStEph.rs | 4 | LinkedList-backed chained strategy |
| 6 | 47 | StructChainedHashTable.rs | 4 | Struct-backed chained strategy |
| 7 | 47 | LinProbFlatHashTableStEph.rs | 4 | Linear probing flat strategy |
| 8 | 47 | QuadProbFlatHashTableStEph.rs | 4 | Quadratic probing flat strategy |
| 9 | 47 | DoubleHashFlatHashTableStEph.rs | 4 | Double hashing flat strategy |

## What to Skip

- **Lookup**: Needs ghost hash function. Defer to future round.
- **Resize**: Needs to prove map preservation across rehash. Defer.
- **call_hash_fn external_body**: Stays — it's a genuine FFI boundary.
- **compute_second_hash external_body**: Stays — same reason.

## Target: Prove insert + delete across all 7 strategy files (14 of 26 holes)

## Important

- Read `src/Chap47/ParaHashTableStEph.rs` thoroughly — it defines EntryTrait, HashTable,
  spec_table_to_map, and the existing proof lemmas.
- Read the R20 report: `plans/agent3-round20-report.md` for context on what was already tried.
- You MAY strengthen ensures (that's the whole point of this round).
- Do NOT weaken existing ensures.
- Do NOT add `assume`, `accept`, or `external_body`.
- `scripts/validate.sh` after each file — 0 errors.

## Deliverables

- Strengthened EntryTrait ensures (insert + delete) across all 4 implementations.
- Proven table-level insert + delete across strategy files.
- `plans/agent3-round22-report.md`
- 0 errors on validate.
- Commit + push to `agent3/ready`.
