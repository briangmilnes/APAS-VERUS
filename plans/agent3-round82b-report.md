# Agent 3 Round 82b Report — AdjTableGraph: OrderedTable → Table

## Objective

Switch `AdjTableGraphStEph.rs` and `AdjTableGraphStPer.rs` from `OrderedTable` (Chap43)
to `Table` (Chap42). The `OrderedTable<V, AVLTreeSet<V>>` requires `V: Ord` on the value
type, but `AVLTreeSet` does not (and should not) implement `Ord`. `Table<K, V>` only
requires `K: Ord` (keys), not `V: Ord` (values), making it the correct backing store.

## Changes Made

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 52 | AdjTableGraphStEph.rs | `OrderedTableStEph` → `TableStEph` import, struct, from_table |
| 2 | 52 | AdjTableGraphStPer.rs | `OrderedTableStPer` → `TableStPer` import, struct, from_table |
| 3 | 52 | AdjTableGraphStPer.rs | Removed `view_ord_consistent` import (no longer needed) |
| 4 | 52 | AdjTableGraphStPer.rs | Removed Pair/cmp ordering requires from `empty()` |
| 5 | 52 | AdjTableGraphStPer.rs | Added `combine` closures to all `insert()` calls (Table insert requires combine, OrderedTable StPer did not) |
| 6 | 52 | AdjTableGraphStPer.rs | Added `#[cfg(verus_keep_ghost)]` to `spec_sum_adj_sizes` import |
| 7 | - | lib.rs | Uncommented both `pub mod AdjTableGraphStEph` and `pub mod AdjTableGraphStPer` |

## Verification Results

| Metric | Before | After | Delta |
|--------|--------|-------|-------|
| Verified | 4714 | 4723 | +9 |
| Errors | 8 | 34 | +26 |
| PTT | 157 pass | 157 pass | 0 |
| RTT | pre-existing failures | same | 0 |

The 8 pre-existing errors are from `EdgeSetGraphStPer.rs` (untouched). The 26 new errors
are all from AdjTableGraph methods calling Table operations that require
`spec_tablesteph_wf()` / `spec_tablestper_wf()`.

## Blockers

Two independent blockers prevent full verification:

### 1. Verus Compiler Bug: sst_to_air.rs Crash on Type Projection

Including `self.adj.spec_tablesteph_wf()` in the graph wf (or any open spec fn that
references the table's internal entries type) crashes Verus:

```
panicked at vir/src/sst_to_air.rs:174:25:
abstract datatype should be boxed Datatype(Path(Path(Some("vstd"), ["set" :: "Set"])),
  [Projection { trait_typ_args: [TypParam("V")], trait_path: Path(Some("vstd"),
  ["view" :: "View"]), name: "V" }], [])
```

The crash occurs because `spec_tablesteph_wf()` internally accesses `self.entries@` which
has type `Seq<(V::V, Set<V::V>)>`. The `Set<<V as View>::V>` type projection inside a
tuple inside a Seq causes a boxing assertion failure in Verus's SST-to-AIR translation.

Tested mitigations (all failed):
- Direct inclusion: `self.adj.spec_tablesteph_wf()` in graph wf → crash
- `spec_keys_no_dups` directly → crash (same type projection)
- `#[verifier::opaque]` wrapper → crash (same type during reveal)

Without table wf in graph wf, methods can't satisfy Table operation preconditions.

### 2. Table API Gaps: Missing wf Postconditions

Even if the Verus bug were fixed, the Table API doesn't propagate wf:

- `TableStEph::empty()` ensures `empty@ == Map::empty()` but NOT `spec_tablesteph_wf()`
- `TableStEph::insert()` ensures key/value facts but NOT `spec_tablesteph_wf()`
- `TableStEph::delete()` ensures `self@ == old(self)@.remove(key@)` but NOT wf

So graph methods that mutate the table can't prove the result is still wf, even if
they could prove the input was wf.

## What Verifies

- `empty()` — graph closure vacuously true on empty map
- `from_table()` — graph closure guaranteed by requires
- All spec functions — ghost-only, no exec calls
- View impl — trivial

## What Doesn't Verify (26 Errors)

All exec methods calling Table operations: `num_vertices`, `num_edges`, `vertices`,
`has_edge`, `out_neighbors`, `out_degree`, `insert_vertex`, `delete_vertex`,
`insert_edge`, `delete_edge`. Each fails with "precondition not satisfied" on
`spec_tablesteph_wf()` / `spec_tablestper_wf()`.

## Recommendations

1. **Fix Verus bug**: Report the `sst_to_air.rs:174` crash with `Set<<V as View>::V>`
   type projections in boxed datatype contexts.
2. **Improve Table API**: Add `ensures self.spec_tablesteph_wf()` to `empty()`,
   `insert()`, `delete()`, and other mutating methods. Currently only `TableStPer::empty()`
   includes wf in ensures.
3. **After fixes**: Re-include `self.adj.spec_table{steph,stper}_wf()` in graph wf and
   verify all methods.
