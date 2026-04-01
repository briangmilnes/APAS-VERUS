# R131 Agent 2 — Chap62: replace HashMap with OrderedTable. AFK.

## Setup

Read ALL files in `src/standards/` before starting. Pay close attention to:
- `using_hashmap_standard.rs` — **especially the section on primitives implementing View**
- `using_closures_standard.rs`
- `hfscheduler_standard.rs`

Report file: `plans/r131-agent2-chap62-orderedtable-report.md`

## Correction from R130

Your R130 report stated: "usize and bool do not implement View in Verus vstd."
**This is wrong.** vstd provides identity View impls for ALL primitives:

```
vstd/view.rs:266: declare_identity_view!(bool);
vstd/view.rs:278: declare_identity_view!(usize);
```

Both `usize` and `bool` satisfy `StT` (Eq + Clone + Display + Debug + Sized + View).
Therefore `OrderedTableStEph<V, usize>` and `OrderedTableStEph<V, bool>` are valid.
The updated `using_hashmap_standard.rs` documents this explicitly.

## Task

Replace `HashMapWithViewPlus` with `OrderedTableMtEph` (or `OrderedTableStEph` /
`OrderedTableStPer` as appropriate) in `src/Chap62/StarPartitionMtEph.rs` for the
maps where it makes sense.

The three maps:
1. `vertex_to_index: HashMapWithViewPlus<V, usize>` — vertex → array index
2. `coin_flips: HashMapWithViewPlus<V, bool>` — vertex → coin flip result
3. `partition_map: HashMapWithViewPlus<V, V>` — vertex → center

### Work/Span trade-off

- HashMap: O(1) expected lookup, O(n) sequential build
- OrderedTable (BST-backed): O(lg n) lookup, O(n lg n) work for build, but
  potentially O(lg² n) span for parallel build via tabulate

APAS specifies O(n + m) work, O(lg n) span. The O(lg n) lookup increases work
to O((n+m) lg n). Whether this trade-off is worth it depends on the span gain.

**Decision rule**: Replace the map if parallel build reduces span below what
sequential HashMap build gives (O(n)). If not, document why HashMap is kept.

For `vertex_to_index`: if OrderedTable has a `tabulate` that can build in
O(lg² n) span from a sorted sequence of key-value pairs, the span improvement
justifies the work regression.

For `coin_flips`: already built in parallel (hash_coin_flips_mt from R128b).
Currently returns HashMapWithViewPlus. If OrderedTable has a parallel build
path, switch. If not, the hash-based parallel build already achieves O(lg n)
span — switching to OrderedTable only helps if it offers better integration.

For `partition_map`: built in Loop 6 which you already parallelized. If the
parallel build can produce an OrderedTable directly, switch.

### Check OrderedTable API

Read `src/Chap43/OrderedTableMtEph.rs` and `src/Chap43/OrderedTableStEph.rs`.
Check for:
- `tabulate` — parallel build from key sequence + function
- `from_seq` or similar — build from sorted key-value pairs
- `find` — lookup by key (O(lg n))
- The requires/ensures on each

If OrderedTable doesn't have a suitable parallel build path, document the gap
and keep HashMap for that map.

## Validation

Run `scripts/validate.sh isolate Chap62`. Then `scripts/rtt.sh`.

## Rules

- Do NOT add assumes, accepts, or external_body.
- Do NOT weaken ensures.
- Use trait aliases from Concurrency.rs (standard 23), not raw bounds.
- Named closures with ensures for all join() calls (standard 8).
- If a map can't be replaced, document why in the report.
