# Agent 2 R98 Report: Add delete_wf to Table Modules

## Summary

Added `delete_wf` to all four Table modules, mirroring the `insert_wf` pattern
from R96. `delete_wf` is a variant of `delete` that additionally ensures
stored-value well-formedness is preserved for all remaining keys after deletion.

## Changes

| # | Chap | File | Change | Verified |
|---|------|------|--------|----------|
| 1 | 42 | TableStEph.rs | Trait decl + full proof impl for `delete_wf` | Yes |
| 2 | 42 | TableStPer.rs | Trait decl + full proof impl for `delete_wf` | Yes |
| 3 | 43 | OrderedTableStPer.rs | Trait decl + `external_body` impl delegating to `delete` | Yes |
| 4 | 43 | OrderedTableMtPer.rs | Trait decl + `external_body` impl delegating to `delete_wf` | Yes |

## Design

### TableStEph and TableStPer (fully proved)

```rust
fn delete_wf(&mut self, key: &K)  // TableStEph (mutable)
fn delete_wf(&self, key: &K) -> (updated: Self)  // TableStPer (persistent)
    where K: ClonePreservesView, V: ClonePreservesWf
    requires
        old(self).spec_table*_wf(),
        obeys_view_eq::<K>(),
        forall|k: K::V| old(self)@.contains_key(k) ==>
            old(self).spec_stored_value(k).spec_wf(),
    ensures
        self@ =~= old(self)@.remove(key@),
        self.spec_table*_wf(),
        forall|k: K::V| self@.contains_key(k) ==>
            self.spec_stored_value(k).spec_wf();
```

The proof follows the same loop structure as `delete`, but uses `clone_view()`
and `clone_wf()` instead of `clone_plus()`. For each non-matching entry, we
prove `pair.1.spec_wf()` by connecting it to the stored-value-wf precondition
through the uniqueness of key indices (no_dups). The loop invariant tracks
`kept@[j].1.spec_wf()` for all kept entries. After the loop, the stored-value-wf
quantifier follows from the kept-wf invariant plus the sv_idx == idx uniqueness
argument.

### OrderedTableStPer and OrderedTableMtPer (external_body)

Simpler ensures (no `spec_stored_value` — these modules don't expose it):

```rust
fn delete_wf(&self, k: &K) -> (table: Self)
    ensures
        table@ == self@.remove(k@),
        forall|k2: K::V| k2 != k@ && self@.contains_key(k2) ==> table@[k2] == self@[k2],
        table.spec_orderedtable*_wf();
```

Both use `external_body` wrapping the underlying `delete`/`delete_wf`.

## Verification

```
scripts/validate.sh isolate Chap43
verification results:: 2576 verified, 0 errors
```

## Techniques

- Mirrored the insert_wf pattern exactly: same bounds, same proof structure.
- clone_view/clone_wf instead of clone_plus for wf preservation.
- Uniqueness argument (no_dups => chosen == idx) to connect spec_stored_value to
  concrete entry positions.
- No new assumes, accepts, or external_body on algorithmic logic.
