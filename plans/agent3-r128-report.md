# R128 Agent 3 Report — Parallelize Chap42 TableMtEph operations

## Summary

Parallelized `map` and `tabulate` in `src/Chap42/TableMtEph.rs` using recursive
divide-and-conquer with `join()` from HFScheduler. Added `clone_pred2` helper to
`src/vstdplus/clone_plus.rs`. Updated all 12 DIFFERS annotations.

## Functions Table

| # | Chap | Function     | Parallelized? | Old Span              | New Span                    | Reason if not                                    |
|---|------|--------------|--------------|-----------------------|-----------------------------|--------------------------------------------------|
| 1 | 42   | domain       | No           | O(n)                  | O(n)                        | Returns ArraySetStEph; no efficient parallel merge |
| 2 | 42   | tabulate     | Yes          | O(\|s\|·W(f))         | O(lg \|s\| + S(f))          | D&C via tabulate_table_dc + join                 |
| 3 | 42   | map          | Yes          | O(n·W(f))             | O(lg n + max W(f))          | D&C via map_table_dc + join                      |
| 4 | 42   | filter       | No           | O(n + Σ W(f(k,v)))    | O(n + Σ W(f(k,v)))          | D&C no-dup proof needs source ordering; deferred |
| 5 | 42   | intersection | No           | O(n·m)                | O(n·m)                      | Nested linear scans on unsorted array            |
| 6 | 42   | union        | No           | O(n·m)                | O(n·m)                      | Nested linear scans on unsorted array            |
| 7 | 42   | difference   | No           | O(n·m)                | O(n·m)                      | Nested linear scans on unsorted array            |
| 8 | 42   | find         | No           | O(n)                  | O(n)                        | Single-element linear scan                       |
| 9 | 42   | delete       | No           | O(n)                  | O(n)                        | Single-element linear scan + copy                |
| 10| 42   | insert       | No           | O(n)                  | O(n)                        | Single-element linear scan + copy                |
| 11| 42   | restrict     | No           | O(n·m)                | O(n·m)                      | Nested linear scans on array                     |
| 12| 42   | subtract     | No           | O(n·m)                | O(n·m)                      | Nested linear scans on array                     |

## D&C Helper Functions Added

- `map_table_dc<K, V, F>`: Recursive D&C map for table entries. Splits entries at
  midpoint, clones keys and applies `f` to values in each half via `join()`, appends
  results. Returns `ArraySeqMtEphS<Pair<K, V>>` with strong ensures: same length,
  same keys at each position, `f.ensures` for values.

- `tabulate_table_dc<K, V, F>`: Recursive D&C tabulate from key sequence. Splits
  `ArraySeqStEphS<K>` at midpoint, builds entries for each half via `join()`, appends.
  Returns `ArraySeqMtEphS<Pair<K, V>>` with ensures matching the sequential version.

## Other Changes

- Added `Clone` bound to trait closure parameters for `map`, `filter`, `tabulate`
  (required for `clone_fn` in D&C arms).
- Added `clone_pred2` to `src/vstdplus/clone_plus.rs` — binary predicate clone with
  mixed types (for future filter D&C).
- Updated all 12 Code review annotations with accurate span analysis.

## Verification

- 2246 verified (isolate Chap42), 0 Chap42 errors.
- 1 pre-existing rlimit in Chap37/AVLTreeSeqStEph.rs (not my change).
- 3534 RTTs pass.

## What Blocks Filter D&C

The filter D&C helper was written and tested but removed because the `spec_fn` ghost
parameter (`Ghost<spec_fn(K::V, V::V) -> bool>`) contains `PhantomData` with `K::V`
and `V::V` types that don't implement `Send`, preventing the join closure from being
`Send + 'static`. Additionally, the no-duplicate-keys proof requires source-index
ordering that the existential ensures don't capture. Two fixes are needed:
1. Restructure ensures to avoid capturing `spec_fn` in closures (use `f.ensures` directly)
2. Add source-ordering ghost return or no-dup preservation ensures
