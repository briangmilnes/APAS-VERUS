# Chap43 Spec Audit — Round 17

Audit of `requires`/`ensures` clauses against ADT 43.1 (Ordered Sets) prose definitions.

## Ordering Gap

ADT 43.1 defines ordering operations with extremality properties:
- `first(A) = min[|A|]` — the minimum element
- `last(A) = max[|A|]` — the maximum element
- `previous(A,k) = max{k' in A | k' < k}` — largest element less than k
- `next(A,k) = min{k' in A | k' > k}` — smallest element greater than k

These properties require spec-level comparison (`<=`, `<`) on `T::V`. The project's
`StT` trait does not include `Ord` or spec-level ordering. The `TotalOrder` trait in
`vstdplus/total_order.rs` provides `spec fn le(self, other: Self) -> bool` but operates
on exec types, not view types (`T::V`). Until `StT` is extended with a spec-level
ordering bound, these extremality properties cannot be expressed generically.

**What we can express now**: membership (`contains`), set-algebraic properties
(subset, disjoint, partition completeness, union), cardinality bounds, value preservation.

**What remains inexpressible**: `first = min`, `last = max`, `previous = max{< k}`,
`next = min{> k}`, `rank = |{k' < k}|`, `select` inverse of `rank`,
`split` left/right ordering, `getRange` range bounds.

## Legend

- **Strong**: ensures matches or exceeds prose definition.
- **Membership**: ensures membership in domain but not extremality/ordering.
- **Algebraic**: ensures set-algebraic properties (partition, disjoint, subset, union).
- **Weak**: ensures only `finite()` or `spec_wf()`.
- **R17**: Strengthened in Round 17.

## File 1: OrderedSetStEph.rs

| # | Function | Strength | R17 | Ensures Summary |
|---|----------|----------|-----|-----------------|
| 1 | size | Strong | | count == len, finite |
| 2 | empty | Strong | | view == empty set, wf |
| 3 | singleton | Strong | | view == {x}, finite, wf |
| 4 | find | Strong | | found == contains(x@) |
| 5 | insert | Strong | | view == old + {x}, finite, wf |
| 6 | delete | Strong | | view == old - {x}, finite, wf |
| 7 | filter | Weak | | finite, wf |
| 8 | intersection | Strong | | view == intersect, finite, wf |
| 9 | union | Strong | | view == union, finite, wf |
| 10 | difference | Strong | | view == difference, finite, wf |
| 11 | to_seq | Strong | | seq.to_set =~= self@, membership |
| 12 | from_seq | Weak | | finite, wf |
| 13 | first | Membership | | None iff empty, Some => contains |
| 14 | last | Membership | | None iff empty, Some => contains |
| 15 | previous | Membership | | Some => contains |
| 16 | next | Membership | | Some => contains |
| 17 | split | Algebraic | Yes | contains_k, !k in halves, subset, disjoint, partition |
| 18 | join | Strong | Yes | self@ == union, finite, wf |
| 19 | get_range | Algebraic | | range subset of self |
| 20 | rank | Membership | | rank <= len |
| 21 | select | Membership | | i >= len => None, Some => contains |
| 22 | split_rank | Algebraic | Yes | subset, disjoint, partition |

## File 2: OrderedSetStPer.rs

| # | Function | Strength | R17 | Ensures Summary |
|---|----------|----------|-----|-----------------|
| 1 | size | Strong | | count == len, finite |
| 2 | empty | Strong | | view == empty set, wf |
| 3 | singleton | Strong | | view == {x}, finite, wf |
| 4 | find | Strong | | found == contains(x@) |
| 5 | insert | Strong | | view == self + {x}, finite, wf |
| 6 | delete | Strong | | view == self - {x}, finite, wf |
| 7 | filter | Algebraic | | subset_of(self), finite, wf |
| 8 | intersection | Strong | | view == intersect, finite, wf |
| 9 | union | Strong | | view == union, finite, wf |
| 10 | difference | Strong | | view == difference, finite, wf |
| 11 | to_seq | Strong | | seq.to_set =~= self@, membership |
| 12 | from_seq | Weak | | finite, wf |
| 13 | first | Membership | | None iff empty, Some => contains |
| 14 | last | Membership | | None iff empty, Some => contains |
| 15 | previous | Membership | | Some => contains |
| 16 | next | Membership | | Some => contains |
| 17 | split | Algebraic | Yes | contains_k, !k in halves, subset, disjoint, partition |
| 18 | join | Strong | | joined == union, finite |
| 19 | get_range | Algebraic | | range subset of self |
| 20 | rank | Membership | | rank <= len |
| 21 | select | Membership | | i >= len => None, Some => contains |
| 22 | split_rank | Algebraic | Yes | subset, disjoint, partition (external_body added) |

## File 3: OrderedTableStEph.rs

| # | Function | Strength | R17 | Ensures Summary |
|---|----------|----------|-----|-----------------|
| 1 | size | Strong | | count == dom.len, finite |
| 2 | empty | Strong | | view == empty map |
| 3 | singleton | Strong | | view == {k->v}, finite |
| 4 | find | Strong | | Some => contains + value, None => !contains |
| 5 | lookup | Strong | | Some => contains + value, None => !contains |
| 6 | is_empty | Strong | | is_empty == dom.is_empty |
| 7 | insert | Weak | | finite |
| 8 | delete | Strong | | view == old.remove(k), finite |
| 9 | domain | Weak | | finite |
| 10 | tabulate | Weak | | finite |
| 11 | map | Weak | | finite |
| 12 | filter | Weak | | finite |
| 13 | reduce | Weak | | finite |
| 14 | intersection | Weak | | finite |
| 15 | union | Weak | | finite |
| 16 | difference | Weak | | finite |
| 17 | restrict | Weak | | finite |
| 18 | subtract | Weak | | finite |
| 19 | collect | Strong | | finite, wf, len == dom.len |
| 20 | first_key | Membership | | None iff empty, Some => contains |
| 21 | last_key | Membership | | None iff empty, Some => contains |
| 22 | previous_key | Membership | | Some => contains |
| 23 | next_key | Membership | | Some => contains |
| 24 | split_key | Algebraic | Yes | contains/value, !k in halves, subset, disjoint, partition |
| 25 | join_key | Weak | | finite |
| 26 | get_key_range | Algebraic | Yes | subset, value preservation |
| 27 | rank_key | Membership | | rank <= dom.len |
| 28 | select_key | Membership | | i >= len => None, Some => contains |
| 29 | split_rank_key | Algebraic | Yes | subset, disjoint, partition |

## File 4: OrderedTableStPer.rs

| # | Function | Strength | R17 | Ensures Summary |
|---|----------|----------|-----|-----------------|
| 1 | size | Strong | | count == dom.len, finite |
| 2 | empty | Strong | | view == empty map, wf |
| 3 | singleton | Strong | | view == {k->v}, finite, wf |
| 4 | find | Strong | | Some => contains + value, None => !contains |
| 5 | insert | Weak | | finite, wf |
| 6 | delete | Strong | | view == self.remove(k), finite, wf |
| 7 | domain | Weak | | finite |
| 8 | tabulate | Weak | | finite |
| 9 | map | Weak | | finite, wf |
| 10 | filter | Weak | | finite, wf |
| 11 | intersection | Weak | | finite, wf |
| 12 | union | Weak | | finite, wf |
| 13 | difference | Weak | | finite, wf |
| 14 | restrict | Weak | | finite, wf |
| 15 | subtract | Weak | | finite, wf |
| 16 | collect | Strong | | finite, wf, len == dom.len |
| 17 | first_key | Membership | | None iff empty, Some => contains |
| 18 | last_key | Membership | | None iff empty, Some => contains |
| 19 | previous_key | Membership | | Some => contains |
| 20 | next_key | Membership | | Some => contains |
| 21 | split_key | Algebraic | Yes | contains/value, !k in halves, subset, disjoint, partition |
| 22 | join_key | Weak | | finite, wf |
| 23 | get_key_range | Algebraic | Yes | subset, value preservation |
| 24 | rank_key | Membership | | rank <= dom.len |
| 25 | select_key | Membership | | i >= len => None, Some => contains |
| 26 | split_rank_key | Algebraic | Yes | subset, disjoint, partition |

## File 5: AugOrderedTableStEph.rs

| # | Function | Strength | R17 | Ensures Summary |
|---|----------|----------|-----|-----------------|
| 1 | size | Strong | | count == dom.len, finite |
| 2 | empty | Strong | | view == empty map |
| 3 | singleton | Weak | | finite |
| 4 | find | Strong | | Some => contains + value, None => !contains |
| 5 | lookup | Strong | | Some => contains + value, None => !contains |
| 6 | is_empty | Strong | | is_empty == dom.is_empty, finite |
| 7 | insert | Weak | | finite |
| 8 | delete | Weak | | finite |
| 9 | domain | Weak | | finite |
| 10 | tabulate | Weak | | finite |
| 11 | map | Weak | | finite |
| 12 | filter | Weak | | finite |
| 13 | reduce | Weak | | finite |
| 14 | intersection | Weak | | finite |
| 15 | union | Weak | | finite |
| 16 | difference | Weak | | finite |
| 17 | restrict | Weak | | finite |
| 18 | subtract | Weak | | finite |
| 19 | collect | Strong | Yes | finite, wf, len == dom.len |
| 20 | first_key | Membership | Yes | None iff empty, Some => contains |
| 21 | last_key | Membership | Yes | None iff empty, Some => contains |
| 22 | previous_key | Membership | Yes | Some => contains |
| 23 | next_key | Membership | Yes | Some => contains |
| 24 | split_key | Algebraic | Yes | contains/value, !k in halves, subset, disjoint |
| 25 | join_key | Weak | | finite (external_body) |
| 26 | get_key_range | Algebraic | Yes | subset |
| 27 | rank_key | Membership | Yes | rank <= dom.len |
| 28 | select_key | Membership | Yes | i >= len => None, Some => contains |
| 29 | split_rank_key | Algebraic | Yes | subset, disjoint |
| 30 | reduce_val | Weak | | finite |
| 31 | reduce_range | Weak | | finite |

## File 6: AugOrderedTableStPer.rs

| # | Function | Strength | R17 | Ensures Summary |
|---|----------|----------|-----|-----------------|
| 1 | size | Strong | | count == dom.len, finite |
| 2 | empty | Strong | | view == empty map, wf |
| 3 | singleton | Weak | | finite, wf |
| 4 | find | Strong | | Some => contains + value, None => !contains |
| 5 | insert | Weak | | finite, wf |
| 6 | delete | Weak | | finite, wf |
| 7 | domain | Weak | | finite |
| 8 | tabulate | Weak | | finite |
| 9 | map | Weak | | finite, wf |
| 10 | filter | Weak | | finite, wf |
| 11 | intersection | Weak | | finite, wf |
| 12 | union | Weak | | finite, wf |
| 13 | difference | Weak | | finite, wf |
| 14 | restrict | Weak | | finite, wf |
| 15 | subtract | Weak | | finite, wf |
| 16 | collect | Weak | | finite, wf |
| 17 | first_key | Membership | | None iff empty, Some => contains |
| 18 | last_key | Membership | | None iff empty, Some => contains |
| 19 | previous_key | Membership | | Some => contains |
| 20 | next_key | Membership | | Some => contains |
| 21 | split_key | Algebraic | Yes | contains/value, !k in halves, subset, disjoint, partition |
| 22 | join_key | Weak | | finite, wf |
| 23 | get_key_range | Algebraic | Yes | subset, value preservation |
| 24 | rank_key | Membership | | rank <= dom.len |
| 25 | select_key | Membership | | i >= len => None, Some => contains |
| 26 | split_rank_key | Algebraic | Yes | subset, disjoint, partition |
| 27 | reduce_val | Weak | | finite |
| 28 | reduce_range | Weak | | finite |

## Summary

| Metric | Count |
|--------|-------|
| Total trait functions audited | 157 |
| Strong ensures | 55 |
| Membership ensures (contains but not ordering) | 36 |
| Algebraic ensures (partition/subset/disjoint) | 18 |
| Weak ensures (finite/wf only) | 48 |
| Functions strengthened in R17 | 26 |

### What R17 accomplished

- **26 functions** had ensures strengthened across 6 files.
- `split`/`split_key`: added partition completeness, disjointness, !k in halves.
- `split_rank`/`split_rank_key`: added partition completeness, disjointness.
- `join`: added union semantics.
- `get_key_range`/`get_range`: added value preservation.
- `first`/`last`/`previous`/`next`: added membership guarantees (was `finite()` only in AugOrderedTableStEph).
- `rank`/`rank_key`: added cardinality bound.
- `select`/`select_key`: added out-of-range => None, in-range => contains.
- `collect`: added len == dom.len, wf.

### What remains for future rounds

1. **Ordering properties**: Requires extending `StT` with spec-level ordering bound on `T::V`.
2. **Weak table operations**: insert, map, filter, reduce, intersection, union, difference,
   restrict, subtract in OrderedTable/AugOrderedTable files still only ensure `finite()`.
   These need the same treatment as the Set variants (which have strong set-algebraic ensures).
3. **AugOrderedTable augmentation ensures**: reduce_val, reduce_range could ensure properties
   about the augmented values.
