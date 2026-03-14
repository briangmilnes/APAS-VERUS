# Agent 4 — Round 13 Report

## Summary

Removed 5 external_body holes from Chap47 hash table probe functions.
Two Chap47 files now fully clean. All other assigned chapters (Chap42, Chap41 Mt,
Chap39) have exclusively architectural holes that cannot be removed without
structural changes (IndexMut limitation, view bridge, parallel ops, closure specs).

**Holes: 272 → 267 (−5)**
**Verified: 3986 → 4004 (+18)**
**RTT: 2600 pass | PTT: 147 pass**

## Hole Changes by File

| # | Chap | File                          | Before | After | Delta | Status     |
|---|------|-------------------------------|--------|-------|-------|------------|
| 1 | 47   | ParaHashTableStEph.rs         | 4      | 2     | −2    | 2 remain   |
| 2 | 47   | LinProbFlatHashTableStEph.rs  | 1      | 0     | −1    | CLEAN      |
| 3 | 47   | QuadProbFlatHashTableStEph.rs | 1      | 0     | −1    | CLEAN      |
| 4 | 47   | DoubleHashFlatHashTableStEph  | 2      | 1     | −1    | 1 remain   |
| 5 | 47   | ChainedHashTable.rs           | 2      | 2     |  0    | blocked    |
| 6 | 42   | TableMtEph.rs                 | 11     | 11    |  0    | blocked    |
| 7 | 41   | AVLTreeSetMtEph.rs            | 10     | 10    |  0    | blocked    |
| 8 | 41   | AVLTreeSetMtPer.rs            | 12     | 12    |  0    | blocked    |
| 9 | 39   | BSTTreapMtEph.rs              | 8      | 8     |  0    | blocked    |

## Chapters Closed

None (Chap47 went from 10 → 5 but not yet 0).

## Techniques Used

**Probe function verification (5 holes removed):**
Replace raw closure call `(hash_fn)(key, table_size)` with `call_hash_fn(hash_fn, key, table_size)`
which has external_body with tight `ensures index < table_size`. Then use wrapping arithmetic
(`wrapping_add`, `wrapping_mul`) for overflow safety, and `% table_size` which Verus/Z3
automatically proves yields `result < table_size` when `table_size > 0`.

## Remaining Holes — What Blocks Them

### Chap47 (5 remaining)

| Hole | File | Blocker |
|------|------|---------|
| `call_hash_fn` external_body | ParaHashTableStEph | Opaque Fn closure; can't verify |
| `double_hash_probe` ext_body | ParaHashTableStEph | Uses DefaultHasher (std lib) |
| `second_hash` external_body | DoubleHashFlat... | Uses DefaultHasher (std lib) |
| `insert_chained` ext_body | ChainedHashTable | Verus lacks IndexMut for Vec |
| `delete_chained` ext_body | ChainedHashTable | Verus lacks IndexMut for Vec |

ChainedHashTable: Attempted clone+set workaround for IndexMut. Failed because
Verus cannot synthesize Clone for `(Key, Value)` tuple types.

### Chap42 — TableMtEph.rs (11 remaining)

All 11 external_body functions use parallel `join()`, closure specs not in trait
signatures, or require sorted-entries invariant not currently tracked. Architectural.

### Chap41 — AVLTreeSetMtEph/MtPer (22 remaining)

- View bridge assumes: ghost_set_view can't connect to locked inner through &self
- Parallel external_body: nested functions, ParaPair, sort/dedup outside verus!
- Unsafe Send/Sync: required for Ghost field (veracity counts as holes)

### Chap39 — BSTTreapMtEph.rs (8 remaining)

All 8 assumes bridge &self ghost view to locked data. Methods use &self with
RwLock interior mutability; ghost_locked_root can't be updated through &self.
Would require &mut self or GhostCell/AtomicGhost.

## Verification

```
verification results:: 4004 verified, 0 errors
RTT: 2600 tests run: 2600 passed
PTT: 147 tests run: 147 passed
Total holes: 267
```

## Commit

Will be committed on branch `agent4/ready`.
