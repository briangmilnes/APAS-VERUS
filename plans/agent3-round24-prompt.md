# Agent 3 — Round 24: Chap47 Prove Insert/Delete with Local Eq Bridges

## Mission

Prove hash table insert and delete operations across the 7 strategy files. You built
the ghost hash infrastructure in R23 and the proof lemmas in R22. Now use them.

## What You Have

From R22:
- `lemma_table_to_map_update_insert` — single-entry update → table map insert
- `lemma_table_to_map_update_remove` — single-entry update → table map remove
- `lemma_table_to_map_not_contains` — key absent from all entries → absent from table

From R23:
- `spec_hash: Ghost<spec_fn(Key) -> nat>` field on HashTable
- Hash bucket invariant in `spec_hashtable_wf`
- `call_hash_fn` ensures `index == spec_hash(*key) % table_size`

## The Remaining Blocker: PartialEq in Entry Impls

The R23 report says generic `Key: PartialEq` blocks connecting exec `==` to spec `==`.
But you don't need to solve the generic case. Each concrete entry implementation has
a specific scan loop — add the standard eq bridge `assume` there.

### The Pattern

In each entry implementation (Vec, LinkedList, StructChained, FlatEntry), the scan
loop does something like:

```rust
if self[i].0 == key { /* found */ }
```

Add the standard eq bridge assume INSIDE the loop body:

```rust
let eq_result = self[i].0 == key;
proof { assume(eq_result == (self[i].0@ == key@)); }  // standard PartialEq bridge
if eq_result { /* found */ }
```

This is the same pattern used in `PartialEq::eq` bodies throughout the codebase
(see `src/standards/partial_eq_eq_clone_standard.rs`). It's an approved workaround
for Verus's lack of PartialEq spec bridge on generic types.

**IMPORTANT**: This assume is ONLY acceptable inside the scan/comparison logic of
the entry implementations, following the same principle as the PartialEq::eq body
pattern. Do NOT use assume anywhere else.

## Step-by-Step

### Step 1: Prove Vec entry insert (VecChainedHashTableStEph.rs)

The Vec entry `insert` pushes `(key, value)` to the vec. With the eq bridge, prove:
```rust
ensures self.spec_entry_to_map() == old(self).spec_entry_to_map().insert(key@, value@)
```

### Step 2: Prove Vec entry delete

Similar — scan, find matching key, remove. With eq bridge:
```rust
ensures self.spec_entry_to_map() == old(self).spec_entry_to_map().remove(key@)
```

### Step 3: Prove table-level insert (VecChainedHashTableStEph.rs)

Remove `external_body` from `insert`. The body:
1. Calls `call_hash_fn` → gets `index` with `index == spec_hash(key) % size`
2. Calls `table[index].insert(key, value)` → entry map updated
3. Call `lemma_table_to_map_update_insert` → table map updated

```rust
fn insert(table: &mut HashTable<...>, key: Key, value: Value) {
    let index = call_hash_fn(&table.hash_fn, &key, table.table.len(), Ghost(table.spec_hash));
    table.table[index].insert(key, value);
    proof {
        lemma_table_to_map_update_insert(
            old(table).table@, index as int, key@, value@, table.spec_hash
        );
    }
}
```

### Step 4: Prove table-level delete

Same pattern with `lemma_table_to_map_update_remove`.

### Step 5: Repeat for all 7 strategy files

Apply the same pattern to:
- LinkedListChainedHashTableStEph.rs
- StructChainedHashTable.rs
- LinProbFlatHashTableStEph.rs
- QuadProbFlatHashTableStEph.rs
- DoubleHashFlatHashTableStEph.rs

For flat hash tables, the entry model is different (single slot, not list) but the
table-level proof pattern is the same.

### Step 6: Maintain wf through insert/delete

The hash bucket invariant says keys only appear in their correct bucket. After insert,
the new key is in `spec_hash(key) % size` — correct by construction. After delete,
a key is removed from its correct bucket — invariant trivially maintained.

## What to Skip

- **Lookup**: Still needs the full PartialEq bridge at the table level (scanning the
  correct bucket is not enough — you need to prove the key IS in that bucket). Defer.
- **Resize**: Needs to prove map preservation across full rehash. Defer.
- **call_hash_fn / compute_second_hash**: Genuine FFI, stays external_body.

## Target: 14 holes closed (insert + delete across 7 strategy files)

## Important

- The eq bridge `assume` is ONLY acceptable in entry-level scan loops, following the
  standard PartialEq workaround pattern. Get user approval if you need assumes elsewhere.
- You MAY strengthen ensures on entry-level insert/delete.
- Do NOT add `assume` or `accept` in table-level algorithmic code.
- `scripts/validate.sh` after each file — 0 errors.
- `scripts/rtt.sh` to verify tests still pass.

## Deliverables

- Proven insert + delete across 7 strategy files (14 holes)
- `plans/agent3-round24-report.md`
- 0 errors on validate, RTT pass.
- Commit + push to `agent3/ready`.
