# Agent 3 — Round 23: Chap47 Ghost Hash Infrastructure

## Mission

Add the ghost hash function infrastructure to Chap47 and use it to prove hash table
insert and delete operations. You wrote the proof lemmas in R22
(`lemma_table_to_map_update_insert`, `lemma_table_to_map_update_remove`) — now build
the infrastructure those lemmas need.

## The Plan

### Step 1: Add `ghost spec_hash` to HashTable

In `src/Chap47/ParaHashTableStEph.rs`, add a ghost field:

```rust
pub struct HashTable<Key, Value, Entry, Metrics, H> {
    pub table: Vec<Entry>,
    pub current_size: usize,
    pub metrics: Metrics,
    pub hash_fn: H,
    pub ghost spec_hash: FnSpec(Key) -> nat,  // NEW
}
```

Note: `FnSpec(Key) -> nat` — takes key, returns a natural number (the bucket index before
mod). The `% table.len()` is done at use sites.

### Step 2: Strengthen `spec_hashtable_wf`

Add the hash bucket invariant:

```rust
open spec fn spec_hashtable_wf(self) -> bool {
    &&& self.table@.len() > 0
    &&& self.current_size <= self.table@.len()
    // NEW: every key lives in its correct bucket
    &&& forall|k: Key, j: int|
        0 <= j < self.table@.len() as int
        && j != (self.spec_hash)(k) as int % self.table@.len() as int
        ==> !self.table@[j].spec_entry_to_map().dom().contains(k)
}
```

This says: if a key is in the table, it's in bucket `spec_hash(key) % size`. No key
appears in a wrong bucket.

### Step 3: Strengthen `call_hash_fn` ensures

```rust
pub fn call_hash_fn<Key, H: Fn(&Key, usize) -> usize>(
    hash_fn: &H, key: &Key, table_size: usize,
    Ghost(spec_hash): Ghost<FnSpec(Key) -> nat>,  // NEW parameter
) -> (index: usize)
    ensures
        index < table_size,
        index == spec_hash(*key) as usize % table_size,  // NEW
```

This connects the exec hash to the ghost hash. `call_hash_fn` stays `external_body` —
it's genuinely an FFI boundary — but now has a useful spec.

### Step 4: Update `createTable` to accept `spec_hash`

```rust
fn createTable(size: usize, hash_fn: H, Ghost(spec_hash): Ghost<FnSpec(Key) -> nat>)
    -> (table: HashTable<Key, Value, Entry, Metrics, H>)
    ensures
        table@ == Map::empty(),
        table.spec_hashtable_wf(),
        table.spec_hash == spec_hash,
```

### Step 5: Update all strategy files

Each strategy file (VecChained, LinkedListChained, StructChained, LinProb, QuadProb,
DoubleHash) has `insert`, `lookup`, `delete`, `resize`. Update their signatures to
thread `spec_hash` through where needed.

### Step 6: Prove insert and delete

Use `lemma_table_to_map_update_insert` (from R22) in each strategy file's `insert`:
```rust
fn insert(table: &mut HashTable<...>, key: Key, value: Value)
    // remove external_body
{
    let index = call_hash_fn(&table.hash_fn, &key, table.table.len(), Ghost(table.spec_hash));
    // ... existing body ...
    proof {
        lemma_table_to_map_update_insert(old(table).table@, index as int, key, value, table.spec_hash);
    }
}
```

### Step 7: Update test files

The RTT tests in `tests/Chap47/` create hash tables. Update them to pass a ghost
`spec_hash` parameter. Since tests run outside verus!, the ghost parameter is erased —
this should be minimal change.

## Files to Modify

| # | Chap | File | Changes |
|---|------|------|---------|
| 1 | 47 | ParaHashTableStEph.rs | Add ghost field, strengthen wf, update call_hash_fn |
| 2 | 47 | ChainedHashTable.rs | Thread spec_hash through insert/delete_chained |
| 3 | 47 | FlatHashTable.rs | Thread spec_hash through insert/delete_with_probe |
| 4 | 47 | VecChainedHashTableStEph.rs | Prove insert + delete |
| 5 | 47 | LinkedListChainedHashTableStEph.rs | Prove insert + delete |
| 6 | 47 | StructChainedHashTable.rs | Prove insert + delete |
| 7 | 47 | LinProbFlatHashTableStEph.rs | Prove insert + delete |
| 8 | 47 | QuadProbFlatHashTableStEph.rs | Prove insert + delete |
| 9 | 47 | DoubleHashFlatHashTableStEph.rs | Prove insert + delete |
| 10 | 47 | tests/Chap47/*.rs | Update createTable calls |

## Target: Close 14 of 24 strategy-file holes (insert + delete across 7 files)

Lookup and resize remain deferred. The 2 hash wrapper external_body holes
(`call_hash_fn`, `compute_second_hash`) are genuine FFI — they stay.

## Important

- This is a cascading API change. Go step by step. Validate after EACH step.
- You MAY modify function signatures (adding Ghost parameters) — that's the point.
- You MAY strengthen ensures and add requires.
- Do NOT add `assume` or `accept` in algorithmic code.
- The `call_hash_fn` external_body STAYS — it's a genuine FFI boundary.
- `scripts/validate.sh` after each step — 0 errors.
- Run `scripts/rtt.sh` after updating test files to ensure tests still pass.

## Deliverables

- Ghost hash infrastructure in ParaHashTableStEph.rs
- Proven insert + delete across strategy files
- Updated tests
- `plans/agent3-round23-report.md`
- 0 errors on validate, RTT pass.
- Commit + push to `agent3/ready`.
