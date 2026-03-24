# R66 Agent 1 Continuation: Fix empty/singleton by removing type axioms from wf

## Problem

`spec_avltreesetsteph_wf()` currently includes `obeys_cmp_spec::<T>()` and
`view_ord_consistent::<T>()`. These are type-level axioms (properties of `T`'s `Ord`
impl), not instance properties. Constructors `empty()` and `singleton()` can't prove
them for generic `T` because there's no witness — you can't compare nothing to nothing
or one thing to itself and derive a total order axiom.

**This is what causes the 2 verification errors.**

## Root Cause

BSTParaStEph itself does NOT include these axioms in `spec_bstparasteph_wf()`. It puts
them as explicit `requires` on individual methods that need comparisons (find, insert,
delete, split, min_key, join_pair, union, intersect, difference, filter). Its `new()` and
`singleton()` verify cleanly because wf doesn't demand the axioms.

## Fix: Mirror BSTParaStEph's Design

### Step 1: Remove axioms from wf

Change `spec_avltreesetsteph_wf` from:
```rust
open spec fn spec_avltreesetsteph_wf(&self) -> bool {
    self.tree.spec_bstparasteph_wf()
    && self@.len() < usize::MAX as nat
    && vstd::laws_cmp::obeys_cmp_spec::<T>()
    && view_ord_consistent::<T>()
}
```
to:
```rust
open spec fn spec_avltreesetsteph_wf(&self) -> bool {
    self.tree.spec_bstparasteph_wf()
    && self@.len() < usize::MAX as nat
}
```

### Step 2: Add axioms to trait method requires

Add `vstd::laws_cmp::obeys_cmp_spec::<T>(), view_ord_consistent::<T>()` to the
`requires` clause of every method that (directly or transitively) calls a BST method
needing them. Both the trait declaration and the impl must match.

**Methods that need axioms added (21 total):**

In `AVLTreeSetStEphTrait`:
1. `find` — calls `tree.find` (BST requires both)
2. `insert` — calls `tree.insert` (BST requires both)
3. `delete` — calls `tree.delete` (BST requires both)
4. `filter` — calls `tree.filter` (BST requires both)
5. `intersection` — calls `tree.intersect` (BST requires both)
6. `union` — calls `tree.union` (BST requires both)
7. `difference` — calls `tree.difference` (BST requires both)
8. `find_iter` — delegates to `find`
9. `insert_iter` — delegates to `insert`
10. `delete_iter` — delegates to `delete`
11. `filter_iter` — delegates to `filter`
12. `intersection_iter` — delegates to `intersection`
13. `union_iter` — delegates to `union`
14. `difference_iter` — delegates to `difference`
15. `from_seq` — calls `insert` in loop

In `AVLTreeSetStEphTotalOrderTrait`:
16. `insert_sorted` — delegates to `insert`
17. `delete_sorted` — delegates to `delete`
18. `filter_sorted` — delegates to `filter`
19. `intersection_sorted` — delegates to `intersection`
20. `difference_sorted` — delegates to `difference`
21. `union_sorted` — delegates to `union`

**Methods that do NOT need axioms:**
- `size` — calls `tree.size` (no axioms needed)
- `to_seq` — calls `tree.in_order` (no axioms needed)
- `empty` — calls `ParamBST::new` (no axioms needed)
- `singleton` — calls `ParamBST::singleton` (no axioms needed)
- `spec_elements_sorted` — spec fn, no calls
- `spec_values_seq` — spec fn, no calls

### Step 3: Update from_seq loop invariant

`from_seq` calls `insert` in a loop. After adding axioms to `insert`'s requires, the
loop invariant must include `obeys_cmp_spec::<T>() && view_ord_consistent::<T>()` so the
invariant → insert requires chain holds. OR, since they're type-level constants (not
affected by loop iterations), they can be established once before the loop by
broadcast/assertion and Verus will carry them through.

The simplest approach: add the axioms to `from_seq`'s own requires. Then they're in scope
for the entire function body including the loop.

### Step 4: Update PartialEq::eq

`PartialEq::eq` currently does:
```rust
proof {
    assume(self.spec_avltreesetsteph_wf());
    assume(other.spec_avltreesetsteph_wf());
}
let equal = self.size() == other.size() && self.difference(other).size() == 0;
```

After the fix, `self.difference(other)` requires the axioms but wf no longer provides
them. Add two more assumes to the proof block:
```rust
proof {
    assume(self.spec_avltreesetsteph_wf());
    assume(other.spec_avltreesetsteph_wf());
    assume(vstd::laws_cmp::obeys_cmp_spec::<T>());
    assume(view_ord_consistent::<T>());
}
```

These fall under the existing `assume_eq_clone_workaround` pattern — PartialEq::eq is
already an assume-heavy boundary. Tag the new assumes the same way.

### Step 5: Check callers — AVLTreeSetMtEph (Chap41)

AVLTreeSetMtEph wraps StEph behind RwLock. Its methods acquire read/write handles and
call StEph methods. After this fix, the StEph methods it calls (insert, delete, find,
filter, etc.) will require axioms. Since MtEph already uses `structural_false_positive
RWLOCK_GHOST` assumes for postconditions, the axioms won't be automatically in scope.

**Check**: Do the MtEph methods that call axiom-requiring StEph methods verify? If not,
add `assume(obeys_cmp_spec::<T>()); assume(view_ord_consistent::<T>());` inside the
existing RWLOCK_GHOST proof blocks. These are structural false positives — the type axioms
are always true for concrete types, and MtEph methods can't prove them for generic T any
more than StEph constructors can. Tag them as `structural_false_positive TYPE_AXIOM`.

### Step 6: Verify

Run `scripts/validate.sh`. The target is 0 errors. The verified count should go UP from
4338 (the empty/singleton functions now verify instead of erroring).

Then run `scripts/rtt.sh` and `scripts/ptt.sh`.

## Constraints

- Do NOT add new `external_body`, `admit()`, or `assume()` in algorithmic code.
- The only new `assume()` allowed is inside `PartialEq::eq` (eq_clone_workaround pattern)
  and inside MtEph RWLOCK_GHOST proof blocks (structural_false_positive pattern).
- Do NOT modify BSTParaStEph.rs (Chap38).
- Do NOT uncomment Chap43 modules — that's a separate task.
- Do NOT change the `_iter`/`_sorted` delegation pattern — they should still delegate to
  their default counterparts.
- Run `scripts/holes.sh src/Chap41/` after to confirm hole count stays at 0.

## Definition of Done

1. `spec_avltreesetsteph_wf` has no `obeys_cmp_spec` or `view_ord_consistent`
2. `empty()` and `singleton()` verify with 0 errors
3. All 21 axiom-requiring methods have explicit requires for both axioms
4. `scripts/validate.sh` shows 0 errors
5. `scripts/rtt.sh` passes (same count as before or higher)
6. `scripts/ptt.sh` passes (147)
7. `scripts/holes.sh src/Chap41/` shows 0 holes
