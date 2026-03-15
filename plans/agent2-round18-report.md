# Agent 2 — Round 18 Report

## Mission

Add `T: TotalOrder` bound to Chap43 ordered set/table traits and write extremality
ensures for first, last, previous, next per ADT 43.1.

## Results

- **4120 verified, 0 errors**
- **2600 RTTs pass**
- **13 files modified** (10 Chap43 + 3 Chap52 callers)
- **24 functions** now have TotalOrder-based ordering ensures
- Chap43 holes: 38 → 60 (+22, all external_body with strong specs)

## What Changed

### TotalOrder Bound Added

Every Chap43 ordered set/table type now requires `TotalOrder` on the key type:

| # | File | Bound Change |
|---|------|-------------|
| 1 | OrderedSetStEph.rs | `T: StT + Ord` → `T: StT + Ord + TotalOrder` |
| 2 | OrderedSetStPer.rs | same |
| 3 | OrderedSetMtEph.rs | `T: MtKey` → `T: MtKey + TotalOrder` |
| 4 | OrderedTableStEph.rs | `K: StT + Ord` → `K: StT + Ord + TotalOrder` |
| 5 | OrderedTableStPer.rs | same |
| 6 | OrderedTableMtEph.rs | `K: MtKey` → `K: MtKey + TotalOrder` |
| 7 | OrderedTableMtPer.rs | `K: MtKey + 'static` → `K: MtKey + TotalOrder + 'static` |
| 8 | AugOrderedTableStEph.rs | `K: StT + Ord` → `K: StT + Ord + TotalOrder` |
| 9 | AugOrderedTableStPer.rs | same |
| 10 | AugOrderedTableMtEph.rs | `K: MtKey` → `K: MtKey + TotalOrder` |
| 11 | AdjTableGraphStEph.rs | `V: StT + Ord` → `V: StT + Ord + TotalOrder` |
| 12 | AdjTableGraphStPer.rs | same |
| 13 | AdjTableGraphMtPer.rs | `V: StTInMtT + Ord` → `V: StTInMtT + Ord + TotalOrder` |

### Extremality Ensures Added (ADT 43.1)

All St files (StEph, StPer) for sets and tables got ordering ensures on 4 functions each.
Pattern uses T-level quantification with `self@.contains(x@)` filter:

**first (min):**
```
first matches Some(v) ==>
    forall|x: T| self@.contains(x@) ==> #[trigger] TotalOrder::le(v, x)
```

**last (max):**
```
last matches Some(v) ==>
    forall|x: T| self@.contains(x@) ==> #[trigger] TotalOrder::le(x, v)
```

**previous (max strictly less than k):**
```
predecessor matches Some(v) ==> TotalOrder::le(v, *k) && v@ != k@,
predecessor matches Some(v) ==>
    forall|x: T| self@.contains(x@) && TotalOrder::le(x, *k) && x@ != k@
        ==> #[trigger] TotalOrder::le(x, v),
predecessor matches None ==>
    forall|x: T| self@.contains(x@) ==> #[trigger] TotalOrder::le(*k, x)
```

**next (min strictly greater than k):**
```
successor matches Some(v) ==> TotalOrder::le(*k, v) && v@ != k@,
successor matches Some(v) ==>
    forall|x: T| self@.contains(x@) && TotalOrder::le(*k, x) && x@ != k@
        ==> #[trigger] TotalOrder::le(v, x),
successor matches None ==>
    forall|x: T| self@.contains(x@) ==> #[trigger] TotalOrder::le(x, *k)
```

### Functions With New Ordering Ensures

| # | Chap | File | Function | New external_body? |
|---|------|------|----------|-------------------|
| 1 | 43 | OrderedSetStEph.rs | first | yes (was verified) |
| 2 | 43 | OrderedSetStEph.rs | last | yes (was verified) |
| 3 | 43 | OrderedSetStEph.rs | previous | already external_body |
| 4 | 43 | OrderedSetStEph.rs | next | already external_body |
| 5 | 43 | OrderedSetStPer.rs | first | yes (was verified) |
| 6 | 43 | OrderedSetStPer.rs | last | yes (was verified) |
| 7 | 43 | OrderedSetStPer.rs | previous | yes (was verified) |
| 8 | 43 | OrderedSetStPer.rs | next | yes (was verified) |
| 9 | 43 | OrderedTableStEph.rs | first_key | yes (was verified) |
| 10 | 43 | OrderedTableStEph.rs | last_key | yes (was verified) |
| 11 | 43 | OrderedTableStEph.rs | previous_key | yes (was verified) |
| 12 | 43 | OrderedTableStEph.rs | next_key | yes (was verified) |
| 13 | 43 | OrderedTableStPer.rs | first_key | yes (was verified) |
| 14 | 43 | OrderedTableStPer.rs | last_key | yes (was verified) |
| 15 | 43 | OrderedTableStPer.rs | previous_key | yes (was verified) |
| 16 | 43 | OrderedTableStPer.rs | next_key | yes (was verified) |
| 17 | 43 | AugOrderedTableStEph.rs | first_key | yes (was verified) |
| 18 | 43 | AugOrderedTableStEph.rs | last_key | yes (was verified) |
| 19 | 43 | AugOrderedTableStEph.rs | previous_key | yes (was verified) |
| 20 | 43 | AugOrderedTableStEph.rs | next_key | yes (was verified) |
| 21 | 43 | AugOrderedTableStPer.rs | first_key | yes (was verified) |
| 22 | 43 | AugOrderedTableStPer.rs | last_key | yes (was verified) |
| 23 | 43 | AugOrderedTableStPer.rs | previous_key | yes (was verified) |
| 24 | 43 | AugOrderedTableStPer.rs | next_key | yes (was verified) |

### Hole Count Before/After

| # | File | Before | After | Delta |
|---|------|--------|-------|-------|
| 1 | AugOrderedTableMtEph.rs | 2 | 2 | 0 |
| 2 | AugOrderedTableStEph.rs | 3 | 7 | +4 |
| 3 | AugOrderedTableStPer.rs | 2 | 6 | +4 |
| 4 | OrderedSetMtEph.rs | 9 | 9 | 0 |
| 5 | OrderedSetStEph.rs | 9 | 11 | +2 |
| 6 | OrderedSetStPer.rs | 3 | 7 | +4 |
| 7 | OrderedTableMtPer.rs | 2 | 2 | 0 |
| 8 | OrderedTableStEph.rs | 4 | 8 | +4 |
| 9 | OrderedTableStPer.rs | 4 | 8 | +4 |
| - | **Total** | **38** | **60** | **+22** |

Each +1 hole is a function that traded a verified body with weak ensures for
external_body with strong ordering ensures. This is the correct trade-off per CLAUDE.md:
"An external_body with a strong spec is a placeholder for a real proof."

## Design Decision: T-Quantifier Pattern

The ordered set view is `Set<T::V>` (not `Set<T>`), so `self@.contains(x)` gives `x: T::V`.
But `TotalOrder::le` operates on `T`, not `T::V`. Two approaches:

1. Add `T::V: TotalOrder` bound → requires `Seq<char>: TotalOrder` for String keys
2. Quantify over `T` values with `self@.contains(x@)` filter → only needs `T: TotalOrder`

Chose approach 2 (same as LeftistHeapPQ pattern). No where clauses needed, no impact on
String keys, and `TotalOrder::le(v, x)` operates directly on `T` values.

## What Was NOT Done

- **rank/select**: Not given ordering ensures. Expressing `rank = |{k' < k}|` requires
  `Set::filter` on `T::V` values with `TotalOrder::le`, which needs `T::V: TotalOrder`.
  Left with existing specs (`rank <= len`, `contains(v@)`).
- **Mt files**: Not given ordering ensures (kept `finite()` only). The Mt specs could be
  strengthened to match St specs, but this requires connecting the lock snapshot to the
  ordering postconditions.

## Techniques

- `replace_all` for systematic bound propagation across all struct/trait/impl/iterator types
- `Ord::cmp` disambiguation where `TotalOrder::cmp` created ambiguity (4 files)
- External_body with strong specs as proof placeholders

## Commit

```
e7f91ea4 (before this round)
```
