# R144 Agent 2 Report: Fix join_pair to require disjoint T1 < T2 (Chap38)

## Summary

Fixed the `join_pair` function in `src/Chap38/BSTParaMtEph.rs` to match APAS Algorithm 38.4
(joinPair), which assumes T1 < T2 (all keys in left tree less than all keys in right tree).

## Change

**File:** `src/Chap38/BSTParaMtEph.rs`

### Trait declaration (line ~350)

Added requires to match `join_pair_inner`:
- `self@.finite(), other@.finite()`
- `self@.disjoint(other@)`
- `forall|s: T, o: T| ... self@.contains(s@) && other@.contains(o@) ==> s.cmp_spec(&o) == Less`

Changed ensures from `joined@ == self@.union(other@)` to `joined@ =~= self@.union(other@)`
to match `join_pair_inner`.

### Impl body (line ~660)

Replaced `union_inner(self, &other)` (O(m lg(n/m)) general union) with
`self.join_pair_inner(&other)` (O(lg n) disjoint join). Removed the now-unnecessary
`proof { use_type_invariant(self); use_type_invariant(&other); }` block since
`join_pair_inner` handles its own invariants.

### Alg Analysis annotations

Updated from DIFFERS to "matches APAS; delegates to join_pair_inner".

## Callers

All callers already prove the disjoint ordering precondition:

| # | Chap | File | Caller | Status |
|---|------|------|--------|--------|
| 1 | 38 | BSTParaStEph.rs | `delete` (line 661) | Already proves disjointness after split |
| 2 | 38 | BSTParaStEph.rs | `join_pair` impl (line 987) | StEph trait already has disjoint requires |
| 3 | 38 | BSTParaStEph.rs | `intersect` (line 1248) | Already proves disjointness |
| 4 | 38 | BSTParaStEph.rs | `difference` (line 1350) | Already proves disjointness |
| 5 | 38 | BSTParaStEph.rs | `filter` (line 1706) | Already proves disjointness |
| 6 | 39 | BSTSetTreapMtEph.rs | `join_pair` (line 380) | Chap39 trait already has disjoint requires |

Note: MtEph callers (delete, intersect, difference, filter) all call `join_pair_inner`
directly, not `join_pair`, so they were unaffected.

## DIFFERS resolved

```
BSTParaMtEph.rs: join_pair — was DIFFERS (general union, no disjoint precondition)
                           — now matches APAS (disjoint T1 < T2 precondition, O(lg n))
```

## Validation

- **Verified:** 5684 verified, 0 errors
- **RTT:** 3690 passed, 0 skipped
- **PTT:** 221 passed, 0 skipped
- **Chap38 holes:** 0 proof holes
