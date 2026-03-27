# R89 Agent 1 Report: Strengthen Table find ensures with value wf

## Objective

Enable callers of `Table::find` to obtain well-formedness guarantees on the returned
value. The specific motivating case: `AdjTableGraphStEph` stores
`Table<V, AVLTreeSetStEph<V>>` and needs returned neighbor sets to be wf to call
`find`, `size`, etc. on them.

## Approach: `find_ref` + `spec_stored_value`

The core problem: `find` returns an owned clone via `clone_plus`, and `cloned(a, b)`
does not transfer exec-level properties like wf (for types with manual Clone, `cloned`
only guarantees view equality `b@ == a@`, not structural equality `b == a`).

Solution: add `find_ref` which returns `Option<&V>`. A reference IS the stored value —
`*v == spec_stored_value(key@)` — so ALL exec-level properties (wf, internal invariants)
transfer directly through SMT equality. No clone, no bridge needed.

### New infrastructure in Table

| # | Chap | File | Addition | Purpose |
|---|------|------|----------|---------|
| 1 | 42 | TableStEph.rs | `spec fn spec_stored_value` | Returns concrete V at key (via `choose` over entries) |
| 2 | 42 | TableStEph.rs | `fn find_ref` | Returns `Option<&V>` with `*v == spec_stored_value(key@)` |
| 3 | 42 | TableStPer.rs | `spec fn spec_stored_value` | Same for persistent table |
| 4 | 42 | TableStPer.rs | `fn find_ref` | Same for persistent table |

Both `find_ref` implementations verify cleanly. The proof shows uniqueness of the
matching index (from `spec_keys_no_dups`), then resolves `choose` to that index,
establishing `*v == spec_stored_value(key@)`.

Existing `find` signature and ensures are UNCHANGED — fully backwards compatible.

### AdjTableGraphStEph improvements

Strengthened `spec_adjtablegraphsteph_wf` to include:
- Table internal invariants (keys unique, feq properties)
- All stored neighbor sets are wf: `spec_stored_value(k).spec_avltreesetsteph_wf()`
- Type-level predicates: `obeys_view_eq`, `obeys_cmp_spec`, `view_ord_consistent`

**Key discovery**: wf conjuncts must be ordered with simple boolean predicates FIRST,
quantifiers LAST. When `forall` leads the conjunction, Z3 cannot efficiently extract
the simpler conjuncts needed by called functions. Reordering fixed all extraction failures.

## Holes: before/after

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 42 | TableStEph.rs | 0 | 0 | 0 |
| 2 | 42 | TableStPer.rs | 0 | 0 | 0 |
| 3 | 52 | AdjTableGraphStEph.rs | 12 | 9 | -3 |

### Functions proven (external_body removed)

| # | Chap | File | Function | Technique |
|---|------|------|----------|-----------|
| 1 | 52 | AdjTableGraphStEph.rs | `num_vertices` | table wf in graph wf |
| 2 | 52 | AdjTableGraphStEph.rs | `has_edge` | `find_ref` + stored value wf |
| 3 | 52 | AdjTableGraphStEph.rs | `out_degree` | `find_ref` + stored value wf |

### Remaining 9 holes in AdjTableGraphStEph.rs

| # | Function | Blocker |
|---|----------|---------|
| 1 | `empty` | TableStEph::empty ensures don't include wf; feq proof needed |
| 2 | `from_table` | Need to prove all stored values are wf from caller's input |
| 3 | `num_edges` | Iterating domain + summing sizes; loop invariant complexity |
| 4 | `vertices` | Building set from domain; clone wf propagation |
| 5 | `out_neighbors` | Returns owned clone; clone doesn't preserve wf |
| 6 | `insert_vertex` | Table insert + closure requires + wf maintenance |
| 7 | `delete_vertex` | Iterating + nested set ops + wf maintenance |
| 8 | `insert_edge` | Table find + insert + set insert + wf maintenance |
| 9 | `delete_edge` | Table find + insert + set delete + wf maintenance |

Root causes:
- `empty` (5 downstream): TableStEph::empty doesn't ensure `spec_tablesteph_wf()`
- `out_neighbors` (clone gap): `clone` on AVLTreeSetStEph only ensures `cloned@ == self@`, not wf
- Mutation functions: maintaining the strengthened wf invariant through table mutations

## Verification counts

- Chap42 isolate: 2154 verified (baseline 2152, +2 from find_ref implementations)
- Chap52 isolate: 2758 verified (baseline 2751, +7)
- Chap43 isolate: 2569 verified, 0 errors (backwards compatible)
- Full crate: 5269 verified, 2 errors (both pre-existing in Chap63/ConnectivityMtEph.rs)

## Next steps to unblock more holes

1. **Add `ensures wf` to TableStEph::empty/singleton** — unblocks `empty`, `from_table`,
   and 4 downstream functions in AdjTableGraph.
2. **Add Clone wf preservation for AVLTreeSetStEph** — either strengthen Clone ensures
   to include wf, or add a `clone_wf()` method. Unblocks `out_neighbors`.
3. **Prove mutation functions** — once `empty` ensures wf, mutation functions become
   provable by maintaining the wf invariant through table insert/delete operations.
4. **Extend to AdjTableGraphStPer and AdjTableGraphMtPer** — same pattern applies.
