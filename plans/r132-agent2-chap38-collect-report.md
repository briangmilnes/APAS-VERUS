# R132 Agent 2 Report — Strengthen BSTParaMtEph/StEph collect_in_order and in_order ensures

## Summary

Strengthened the `collect_in_order` and `in_order` ensures in both `BSTParaStEph.rs` and
`BSTParaMtEph.rs` (Chap38) to include view-level no-duplicates guarantees.

## Changes

### collect_in_order — both files + MtEph inner helper

Added a new ensures clause:

```rust
forall|i: int, j: int| #![trigger out@[i], out@[j]]
    old(out)@.len() <= i < j < out@.len() ==> out@[i]@ != out@[j]@
```

This says: the newly-appended portion of the output vector has no duplicate views.

**Proof**: 5-case analysis on which region each index falls in (left subtree, key, right subtree).
The BST expose ensures give `left@.disjoint(right@)`, `!left@.contains(key@)`,
`!right@.contains(key@)`, so elements from different regions have distinct views.
Elements within the same region have distinct views by induction.

### in_order — both files

Added a new ensures clause:

```rust
seq@.no_duplicates()
```

**Proof**: Lift the view-level no-dups from `collect_in_order` through `ArraySeqStPerS::from_vec`,
which preserves `result@[i] == out@[i]@`.

### Dropped: seq@.to_set() =~= self@

Initially added `seq@.to_set() =~= self@` to `in_order` ensures. Removed because it
destabilized two downstream proofs in `Chap43/OrderedTableStEph.rs` (lines 2739, 2827 —
loop invariant `self@.dom().contains(min_key@)` failed). The `to_set` ensures introduces
`Set::new(|a| sorted@.contains(a))` which added Z3 trigger load that pushed those proofs
past their rlimit. The property is trivially derivable from the existing containment
biconditional `forall|v| self@.contains(v) <==> seq@.contains(v)` — callers don't need
it stated explicitly.

## Files Modified

| # | Chap | File | What |
|---|------|------|------|
| 1 | 38 | BSTParaStEph.rs | Trait + impl: strengthened ensures |
| 2 | 38 | BSTParaMtEph.rs | Trait + impl + collect_in_order_inner: strengthened ensures |

## Validation

| Step | Result |
|------|--------|
| validate (isolate Chap38) | 1072 verified, 0 errors |
| validate (full) | 5472 verified, 0 errors |
| RTT | 3529 passed, 0 skipped |
| PTT | 221 passed, 0 skipped |

## Impact for Chap41

The strengthened `collect_in_order` ensures now directly provide the no-duplicate-views
property that Chap41's `to_seq` previously needed manual `assume` statements for. The
`seq@.to_set() =~= self@` property can be derived in one `assert` from the containment
biconditional, so Chap41 callers can establish it locally without an assume.
