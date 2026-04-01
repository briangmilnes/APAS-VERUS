# R131 Agent 2 Report: Chap62 HashMap to OrderedTable Assessment

## Summary

**Decision: Keep all three HashMapWithViewPlus maps.** OrderedTable does not offer span
improvements and would regress work bounds. One pre-existing verification bug was fixed.

## Map-by-Map Analysis

### 1. vertex_to_index: HashMapWithViewPlus<V, usize>

| | HashMap (current) | OrderedTable |
|---|---|---|
| Build | O(n) seq loop | O(n lg n) seq insert loop |
| Lookup | O(1) expected | O(lg n) |
| Build span | O(n) | O(n lg n) |
| Parallel build? | No (sequential) | tabulate exists but O(n lg n) span |

**Keep HashMap.** Loop 1 builds vertex_to_index sequentially. OrderedTableMtEph's
`tabulate` takes `ArraySetStEph<K>` and `Fn(&K) -> V`, but the function needs to know
each vertex's index in vertices_vec (which is hash-ordered, not sorted). This is
circular without the map. Sequential insert loop would be O(n lg n) — strictly worse
than O(n). No span improvement since both are sequential.

### 2. coin_flips: HashMapWithViewPlus<V, bool>

| | HashMap (current) | OrderedTable |
|---|---|---|
| Build | O(n) work, O(lg n) span (D&C) | O(n lg n) work, O(lg n) span (D&C + union merge) |
| Lookup | O(1) | O(lg n) |
| Merge (D&C) | O(n) iterator merge | O(m log(n/m+1)) union |

**Keep HashMap.** Already built in parallel via `hash_coin_flips_mt` with O(lg n) span.
Switching to OrderedTable would require D&C with `union` merge — same span but worse
work. Lookup degrades from O(1) to O(lg n), inflating total algorithm work from O(n+m)
to O((n+m) lg n).

### 3. partition_map: HashMapWithViewPlus<V, V>

| | HashMap (current) | OrderedTable |
|---|---|---|
| Build | O(n lg n) work, O(lg n) span (D&C) | O(n lg n) work, O(lg n) span (D&C + union) |
| Lookup | O(1) | O(lg n) |

**Keep HashMap.** Already built in parallel via `build_partition_map_mt`. Same span
with OrderedTable, but lookup degrades O(1) to O(lg n).

## Spec Gap: OrderedTableMtEph find

A critical barrier independent of performance: **OrderedTableMtEph's `find` ensures is
too weak.** It only guarantees `self@.contains_key(k@)` — it does NOT ensure `v@ == self@[k@]`.

- `HashMapWithViewPlus::get` ensures: `Some(v) => self@.contains_key(k@) && *v == self@[k@]`
- `OrderedTableStEph::find` ensures: `Some(v) => self@.contains_key(k@) && v@ == self@[k@]` (strong)
- `OrderedTableMtEph::find` ensures: `Some(v) => self@.contains_key(k@)` (weak — no value)

The RwLock boundary in MtEph drops the value correspondence. StarPartitionMtEph's proofs
depend on extracting actual values from lookups (coin flip booleans, vertex indices), so
MtEph's weak find breaks all three maps' proofs.

**Workaround**: Use `OrderedTableStEph` through `arc_deref` (immutable sharing via Arc).
The StEph find has the strong spec. But the algorithmic trade-offs still don't justify
the switch (see above).

## R130 Correction

My R130 report incorrectly stated "usize and bool do not implement View in Verus vstd."
This is wrong. vstd provides identity View impls for all primitives (bool, usize, u8..u128,
i8..i128, isize, char). Both `OrderedTableMtEph<V, usize>` and `OrderedTableMtEph<V, bool>`
are valid types. The barrier is spec strength and algorithmic cost, not type compatibility.

## Bug Fix

Fixed a pre-existing verification error in Loop 5 (`StarPartitionMtEph.rs:1027`).

**Problem**: The invariant `vertex_to_index@.contains_key(p_vec@[j2]@)` failed on loop
entry. Z3 could not connect `build_p_vec_mt`'s ensures (`p_vec@[j]@ == vertices_vec@[j]@`)
with Loop 1's proven facts about `vertex_to_index`.

**Fix**: Added proof bridge (11 lines) between Loop 4 output and Loop 5 entry that
explicitly chains:
1. `p_vec@[j2]@ == vertices_vec@[j2]@` (from `build_p_vec_mt`)
2. `vertex_to_index@.contains_key(vertices_vec@[j2]@)` (from Loop 1)
3. `vertex_to_index@[vertices_vec@[j2]@] as usize == j2` (from Loop 1)

## Verification

```
scripts/validate.sh isolate Chap62: 1339 verified, 0 errors
```

Pre-existing RTT compilation errors (experiments, not Chap62): `test1_named_fn_clone`,
`test2_named_fn_mt_bounds`, `test5_named_fn_clone_fn2` — unrelated to this task.

## Holes

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 62 | StarPartitionMtEph.rs | No new holes. Bug fix only (bridge proof). |

## Files Changed

| # | Chap | File | Lines | Description |
|---|------|------|-------|-------------|
| 1 | 62 | StarPartitionMtEph.rs | 997-1009 | Added proof bridge for Loop 5 invariant init |
