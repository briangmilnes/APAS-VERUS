# R144 Agent 3 — Fix conjunction flakiness in StarPartition closure. AFK. DOT.

## Problem

`src/Chap62/StarPartitionMtEph.rs:1715` — closure ensures has conjunction
flakiness. Both sub-assertions prove individually (Verus shows ✔ on each)
but the conjunction fails. This is the Z3 conjunction flakiness pattern.

## The fix

The closure at line ~1708 (`f_pm`) has this ensures:

```rust
ensures
    forall|j: int| 0 <= j < nv as int ==>
        #[trigger] r@.contains_key(va2@[j]@) &&
        r@[va2@[j]@]@ == pa2@[j]@,
    forall|v_view: V::V| #[trigger] r@.contains_key(v_view) ==>
        exists|j: int| 0 <= j < nv as int && #[trigger] va2@[j]@ == v_view,
```

Split the first forall's conjunction into two separate ensures clauses:

```rust
ensures
    forall|j: int| 0 <= j < nv as int ==>
        #[trigger] r@.contains_key(va2@[j]@),
    forall|j: int| 0 <= j < nv as int ==>
        r@.contains_key(va2@[j]@) ==>
        r@[va2@[j]@]@ == pa2@[j]@,
    forall|v_view: V::V| #[trigger] r@.contains_key(v_view) ==>
        exists|j: int| 0 <= j < nv as int && #[trigger] va2@[j]@ == v_view,
```

## Also check build_partition_map_mt

The function `build_partition_map_mt` that the closure calls may have the
same conjunction in its ensures. If so, split it identically. Search for
`build_partition_map_mt` and check its ensures clause.

## Also check callers

Any code after line ~1724 that uses the closure's result may have been
relying on the conjunction as a single fact. After splitting, callers get
both facts separately. This is usually fine but check that no proof breaks.

## Also fix the two assert-forall warnings

Lines 1217 and 1670 have `==>` instead of `implies` in `assert forall`:
```
warning: using ==> in `assert forall` does not currently assume the antecedent
```
Change `==>` to `implies` in those two assert-forall statements.

## Validation

Run `scripts/validate.sh isolate Chap62`. Then `scripts/rtt.sh`.

## When done

RCP.
