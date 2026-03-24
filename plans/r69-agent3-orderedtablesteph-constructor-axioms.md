# R69 Agent 3: OrderedTableStEph Constructor Axiom Elimination

## Goal

Eliminate constructor axiom assumes in `src/Chap43/OrderedTableStEph.rs`. Target: the 6
tabulate axiom assumes → 0 by lifting to requires.

## Current Axiom Holes (6)

All in `tabulate`:
- Line ~1326: `assume(obeys_cmp_spec::<Pair<K, V>>())`
- Line ~1327: `assume(view_ord_consistent::<Pair<K, V>>())`
- Line ~1453: `assume(spec_pair_key_determines_order::<K, V>())`
- Line ~1454: `assume(obeys_cmp_spec::<K>())`
- Line ~1455: `assume(view_ord_consistent::<K>())`
- Line ~1456: `assume(obeys_feq_fulls::<K, V>())`

`empty()` and `singleton()` in StEph have 0 holes — they already get axioms from
somewhere. Check how they do it and apply the same pattern to `tabulate`.

## Strategy

Same as Agent 2's approach for StPer: lift axiom predicates to the trait `requires`
for `tabulate`. The 6 assumes become redundant.

```rust
fn tabulate<F: Fn(&K) -> V>(f: F, keys: &ArraySetStEph<K>) -> (tabulated: Self)
    requires
        ...,  // existing requires
        vstd::laws_cmp::obeys_cmp_spec::<Pair<K, V>>(),
        view_ord_consistent::<Pair<K, V>>(),
        spec_pair_key_determines_order::<K, V>(),
        vstd::laws_cmp::obeys_cmp_spec::<K>(),
        view_ord_consistent::<K>(),
        obeys_feq_fulls::<K, V>(),
    ensures ...;
```

### Caller Impact

Check who calls `tabulate()`:
- RTT tests (concrete types — axioms hold trivially)
- Possibly AugOrderedTable or other Chap43 modules
- If callers have wf on another table of the same type, the axioms are in scope

### Also Fix

- `from_sorted_entries` (line ~3752): add `ensures result.spec_orderedtablesteph_wf()`

## Steps

1. **Read** OrderedTableStEph.rs — check how `empty()` and `singleton()` avoid axiom
   assumes (they're clean, 0 holes). Use the same pattern for `tabulate`.
2. **Add** axiom predicates to `tabulate` requires in the trait
3. **Delete** 6 axiom assumes from `tabulate` impl body
4. **Fix** callers if needed
5. **Add** `ensures wf` to `from_sorted_entries`
6. **Validate**, **rtt**, **ptt**

## Constraints

- Modify only `src/Chap43/OrderedTableStEph.rs` and possibly callers in Chap43.
- Do NOT modify files outside Chap43.
- Do NOT modify OrderedTableStPer.rs (Agent 2 owns that).
- Do NOT add new `assume`, `accept`, or `external_body`.
- Do NOT weaken ensures.
- Run validate, rtt, ptt sequentially.
