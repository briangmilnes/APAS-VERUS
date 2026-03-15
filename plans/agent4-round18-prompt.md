# Agent 4 — Round 18: Add View Impl to Chap47 Hash Tables

## Mission

Add `View` impls to Chap47 hash tables so that insert/lookup/delete/resize can have
functional (not just structural) ensures. You declared "no View impl" as a structural
blocker in R17. The building blocks already exist — `spec_chain_to_map` in
StructChainedHashTable converts chains to `Map<Key, Value>`. You are wiring them up.

## Architecture

There are two hash table families:

### Chained Hash Tables (linked-list chains)

| # | File | Role |
|---|------|------|
| 1 | StructChainedHashTable.rs | **Has `spec_chain_to_map` already** |
| 2 | ChainedHashTable.rs | Trait for chained hash tables |
| 3 | LinkedListChainedHashTableStEph.rs | Impl using linked list chains |
| 4 | VecChainedHashTableStEph.rs | Impl using Vec chains |

The `StructChainedHashTable.rs` already has:
```rust
pub open spec fn spec_chain_to_map<Key, Value>(
    chain: Option<Box<Node<Key, Value>>>,
) -> Map<Key, Value>
    decreases chain,
{
    match chain {
        None => Map::empty(),
        Some(node) => spec_chain_to_map(node.next).insert(node.key, node.value),
    }
}
```

For the whole table, fold over all chains:
```rust
pub open spec fn spec_table_to_map<Key, Value>(
    table: Seq<ChainList<Key, Value>>,
) -> Map<Key, Value>
    decreases table.len(),
{
    if table.len() == 0 {
        Map::empty()
    } else {
        spec_table_to_map(table.drop_last()).union_prefer_right(
            spec_chain_to_map(table.last().head)
        )
    }
}
```

Then add View:
```rust
impl<Key, Value> View for HashTable<Key, Value> {
    type V = Map<Key, Value>;
    open spec fn view(&self) -> Map<Key, Value> {
        spec_table_to_map(self.table@)
    }
}
```

### Flat Hash Tables (open addressing)

| # | File | Role |
|---|------|------|
| 5 | FlatHashTable.rs | Trait + base type with `table: Vec<Entry<Key, Value>>` |
| 6 | LinProbFlatHashTableStEph.rs | Linear probing impl |
| 7 | QuadProbFlatHashTableStEph.rs | Quadratic probing impl |
| 8 | DoubleHashFlatHashTableStEph.rs | Double hashing impl |

Flat hash tables use an array of `Entry<Key, Value>` where Entry is:
```rust
enum Entry<Key, Value> {
    Empty,
    Deleted,
    Occupied(Key, Value),
}
```

Write `spec_flat_table_to_map`:
```rust
pub open spec fn spec_flat_table_to_map<Key, Value>(
    table: Seq<Entry<Key, Value>>,
) -> Map<Key, Value>
    decreases table.len(),
{
    if table.len() == 0 {
        Map::empty()
    } else {
        let rest = spec_flat_table_to_map(table.drop_last());
        match table.last() {
            Entry::Empty => rest,
            Entry::Deleted => rest,
            Entry::Occupied(k, v) => rest.insert(k, v),
        }
    }
}
```

Then:
```rust
impl<Key, Value> View for FlatHashTable<Key, Value> {
    type V = Map<Key, Value>;
    open spec fn view(&self) -> Map<Key, Value> {
        spec_flat_table_to_map(self.table@)
    }
}
```

### ParaHashTable (generic wrapper)

| # | File | Role |
|---|------|------|
| 9 | ParaHashTableStEph.rs | Generic hash table wrapper used by all variants |

ParaHashTableStEph defines the `HashTable` struct that both chained and flat variants use.
The View impl may go here or in the variant-specific files depending on which struct
actually holds the data.

## Procedure

1. **Read** `src/Chap47/StructChainedHashTable.rs` — understand spec_chain_to_map.
2. **Read** `src/Chap47/ParaHashTableStEph.rs` — understand the HashTable struct.
3. **Read** `src/Chap47/FlatHashTable.rs` — understand Entry enum and flat table structure.
4. **Read** `src/Chap47/ChainedHashTable.rs` — understand the chained table trait.

5. **Add spec_table_to_map** to StructChainedHashTable.rs (fold over chains).
6. **Add View impl** on the appropriate struct.
7. **Add spec_flat_table_to_map** to FlatHashTable.rs.
8. **Add View impl** on the flat hash table struct.

9. **Strengthen ensures** on insert/lookup/delete/resize:
   ```rust
   fn insert(&mut self, key: Key, value: Value)
       ensures
           self@ == old(self)@.insert(key, value),
   ;

   fn lookup(&self, key: &Key) -> (found: Option<&Value>)
       ensures
           self@.dom().contains(*key) ==> found.is_some()
               && *found.unwrap() == self@[*key],
           !self@.dom().contains(*key) ==> found.is_none(),
   ;

   fn delete(&mut self, key: &Key) -> (deleted: bool)
       ensures
           deleted == old(self)@.dom().contains(*key),
           self@ == old(self)@.remove(*key),
   ;

   fn resize(&mut self)
       ensures
           self@ == old(self)@,
   ;
   ```

10. Add `external_body` where proof breaks.
11. `scripts/validate.sh` — 0 errors.

## Important

- `spec_chain_to_map` already exists. You are building on it, not starting from scratch.
- The View impl is the core deliverable. Once View exists, the functional ensures follow.
- Add `external_body` freely — strong spec + external_body > weak spec.
- Do NOT punt with "structural blocker" or "needs View." You are adding the View.
- Do NOT modify files in other chapters (other agents' scope).
- Also fix `second_hash` in DoubleHashFlatHashTableStEph.rs — the one function with
  missing ensures entirely. At minimum add `ensures true` or a hash-related postcondition.

## Deliverables

- View impls on hash table types.
- spec_table_to_map / spec_flat_table_to_map spec functions.
- Functional ensures on insert/lookup/delete/resize.
- Fixed second_hash ensures.
- `plans/agent4-round18-report.md`
- 0 errors on validate.
- Commit + push to `agent4/ready`.
