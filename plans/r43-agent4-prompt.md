# R43 Agent 4: Chap38 BSTParaMtEph + Chap39 BSTParaTreapMtEph (55 holes)

## Baseline

- Main at latest commit after R42 merges, 4362 verified, 0 errors, 139 holes
- 34 clean chapters

## MANDATORY RULES — READ BEFORE WRITING ANY CODE

**DO NOT USE `accept()` ANYWHERE. NOT ONCE. NOT EVER. NOT FOR ANY REASON.**
**DO NOT weaken ensures.** Do not delete postconditions to make proofs easier.
**DO NOT add `requires true` or `// veracity: no_requires`.**
**DO NOT convert assume() to accept().**
**DO NOT move code outside `verus!{}` to dodge verification.** If you can't
prove a body, leave the `external_body` and report what you tried.
**DO NOT add assume() or accept() outside the four permitted patterns.**
**DO NOT remove or silence the RWLOCK_GHOST structural_false_positive notes** —
they are already classified as info, not errors.

Read CLAUDE.md and `src/standards/mod_standard.rs` before starting.
Read `src/standards/using_closures_standard.rs` before touching any closure code.
Read `src/standards/arc_usage_standard.rs` before touching any Arc code.

## Your Assignment

You are working on the two Parallel BST implementation files in Chap38 and Chap39.
These have 55 combined holes. Realistically, target 10-20 holes closed in one round.

| # | Chap | File | Holes | Hole Types |
|---|------|------|-------|------------|
| 1 | 38 | BSTParaMtEph.rs | 27 actionable | 4 assume + 23 external_body |
| 2 | 39 | BSTParaTreapMtEph.rs | 27 actionable | 3 assume + 24 external_body |

The veracity counts from the analysis logs:
- `src/Chap38/analyses/veracity-review-verus-proof-holes.log`: 27 holes, 2 RWLOCK_GHOST structural info
- `src/Chap39/analyses/veracity-review-verus-proof-holes.log`: 27 holes, 1 RWLOCK_GHOST + 1 OPAQUE_EXTERNAL structural info

## Step 0: Read the Clean Counterparts First

**Do this before writing a single line.** The single-threaded counterpart for Chap38 is
already clean and proven:

- `src/Chap38/BSTParaStEph.rs` — **Read this entire file.** It is the proof template.
  The StEph carries a `ghost_locked_root: Ghost<Set<...>>` field plus a `#[verifier::type_invariant]`
  that ties the ghost state to the RwLock predicate. The MtEph (Chap38/BSTParaMtEph.rs) does NOT
  have this design — it uses `#[verifier::external_body]` on the `View` impl instead, so
  `self@` is opaque to the verifier. This is the root cause of all the algorithmic assumes.

For Chap39, the analogous clean file is:
- `src/Chap39/BSTTreapStEph.rs` — Read the relevant sections for structural context.
  Note that `BSTParaTreapMtEph.rs` uses the same opaque-view design as BSTParaMtEph.

Understanding WHY the StEph proves and the MtEph does not is the key to planning your work.

## The Core Problem: Opaque View vs. Ghost-Tracked View

In `BSTParaStEph.rs`, the struct has:
```rust
pub struct ParamBST<T: StT + Ord> {
    pub(crate) locked_root: RwLock<Option<Box<NodeInner<T>>>, BSTParaStEphInv<T>>,
    pub(crate) ghost_locked_root: Ghost<Set<<T as View>::V>>,
}
```

The `BSTParaStEphInv<T>` predicate carries `pub ghost contents: Set<T::V>`, so
`pred().contents` pins down exactly what is in the locked set. The `#[verifier::type_invariant]`
connects `ghost_locked_root@` to `locked_root.pred().contents`, giving Verus full visibility
into `self@` after `use_type_invariant(self)`.

In `BSTParaMtEph.rs`, the struct has:
```rust
pub struct ParamBST<T: MtKey> {
    pub root: Arc<RwLock<Option<Box<NodeInner<T>>>, BSTParaMtEphInv>>,
}
```

`BSTParaMtEphInv` is a unit struct (no ghost data). The `View` impl has `#[verifier::external_body]`
and always returns `Set::empty()` — so `self@` is meaningless to the verifier. This means:
- `new()` cannot prove `empty@ == Set::empty()` without assume.
- `size()` cannot prove `count == self@.len()` without assume.
- etc.

## Strategy for Chap38 BSTParaMtEph.rs

### Option A: Port the Ghost-Tracking Pattern (Preferred, Hard)

Adopt the StEph design for the MtEph. Add `pub ghost ghost_locked_root: Ghost<Set<T::V>>`
to `ParamBST<T>`, add ghost contents to `BSTParaMtEphInv`, add a `#[verifier::type_invariant]`,
update `new_param_bst_arc` to accept and thread ghost state, and propagate through all
constructors. This closes the four algorithmic assumes (new, singleton, join_mid, size)
and opens the path to proving the remaining external_body functions.

This is ~100-200 lines of structural change plus proof work for each function. It is the
right long-term fix. If you attempt this, proceed incrementally:
1. Add ghost field and type invariant. Validate — should still compile.
2. Update new_param_bst_arc, new(), singleton(). Prove these three. Validate.
3. Port expose() (use_type_invariant + RwLock read). Prove.
4. Port join_mid() (ghost arithmetic). Prove.
5. Port size(), is_empty() (trivial after type_invariant). Prove.
6. Port the inner helper functions (split_inner, join_m, join_pair_inner, etc.)
   using the StEph proofs as templates. Validate after each batch.

The helpers (split_inner, union_inner, intersect_inner, difference_inner, filter_inner,
reduce_inner, collect_in_order, filter_parallel, reduce_parallel) contain `ParaPair!`
fork-join parallelism. These must remain inside `verus!{}`. Use the using_closures_standard
for the closure specs. The MtKey bound already includes Send + Sync + 'static.

### Option B: Minimally Prove the Four Algorithmic Assumes (Fallback, Medium)

If the full structural migration is too risky in one round, focus on closing the four
algorithmic assumes using partial ghost tracking:

1. **new() assume** (line 203): `assume(empty@ == Set::empty() && empty.spec_bstparamteph_wf())`
   The ghost is that new_param_bst_arc(None) corresponds to the empty set. Currently
   unprovable because View is external_body. If you add ghost tracking, this becomes
   `new_param_bst_arc(None, Ghost(Set::empty()))` and follows directly.

2. **singleton() assume** (line 217): `assume(tree@ == Set::empty().insert(key@) && tree@.finite())`
   Similar: provable once new() and join_mid() connect ghost state to the rwlock predicate.

3. **join_mid() assume** (line 256): `assume(exposed is Leaf ==> joined@ == Set::empty())`
   The Leaf arm calls `Self::new()`, so if new() is proven, this follows.

4. **size() assume** (line 272): `assume(count == self@.len() && self@.finite())`
   Provable once the type_invariant links the node.size field to the ghost content count.
   In the StEph version, `spec_size_wf` makes this immediate.

### Option C: External_body Inner Helpers via assume_specification (Exploratory)

The inner helpers (split_inner, union_inner, etc.) are already inside `verus!{}` with
`#[verifier::external_body]`. If you give them full specs, callers (the trait impl methods)
can use those specs without proving the bodies yet. The trait impl methods (split, join_pair,
union, intersect, difference) become simple delegation wrappers that Verus can verify once
the inner helper specs exist.

**This does not close holes in the inner helpers themselves** but it closes holes in the
trait impl delegation wrappers. Assess whether the trait impl methods are separately counted
as holes by veracity.

Example: `split_inner` already has specs in the file:
```rust
#[verifier::external_body]
fn split_inner<T: MtKey + 'static>(tree: &ParamBST<T>, key: &T) -> (parts: (ParamBST<T>, B, ParamBST<T>))
    ensures
        parts.1 == tree@.contains(key@),
        parts.0@.finite(),
        parts.2@.finite()
```

The trait impl `split()` (not external_body) just delegates: `{ split_inner(self, key) }`.
Look at the current file — `split()` is NOT marked external_body, it delegates to split_inner.
So if split_inner has specs, split() should be provable. Check whether `split` is currently
being counted as a hole.

## Priority Order for Chap38

1. **Confirm the current hole counts** by running `scripts/holes.sh src/Chap38/` at the start.
2. **Attempt Option A** (ghost tracking migration). If it proves clean, you close 10-15 holes.
3. If Option A is too large, **attempt Option B** (close the 4 algorithmic assumes with partial
   ghost tracking). This is a smaller structural change.
4. Then look at the external_body inner helpers. The ones that use ParaPair! (union_inner,
   intersect_inner, difference_inner, filter_inner, filter_parallel, reduce_inner,
   reduce_parallel) require closure specs per using_closures_standard. The sequential helpers
   (split_inner, join_m, join_pair_inner, min_key, collect_in_order) are simpler.
5. The two RWLOCK_GHOST structural_false_positives (expose assume line 233, size assume line 272)
   are already classified as info — do not attempt to remove them as holes until you understand
   whether veracity counts them.

## Specific Holes in BSTParaMtEph.rs

The veracity log shows these actionable holes:

| # | Line | Type | Function | Notes |
|---|------|------|----------|-------|
| 1 | 80 | external_body | ParamBST::view | Root cause of all other holes |
| 2 | 203 | assume [algorithmic] | new() | Needs ghost tracking to prove |
| 3 | 217 | assume [algorithmic] | singleton() | Needs ghost tracking to prove |
| 4 | 256 | assume [algorithmic] | join_mid() | Needs ghost tracking to prove |
| 5 | 344 | assume [rwlock:reader] | find() | Loop finds key; needs set membership proof |
| 6 | 355 | external_body | join_pair() | Delegates to join_pair_inner |
| 7 | 360 | external_body | union() | Delegates to union_inner |
| 8 | 365 | external_body | intersect() | Delegates to intersect_inner |
| 9 | 370 | external_body | difference() | Delegates to difference_inner |
| 10 | 375 | external_body | filter() | Delegates to filter_parallel |
| 11 | 385 | external_body | reduce() | Delegates to reduce_parallel |
| 12 | 390 | external_body | in_order() | Collects via collect_in_order |
| 13 | 442 | external_body | new_leaf() | Constructor helper |
| 14 | 447 | external_body | expose_internal() | Reader: acquire lock, match, release |
| 15 | 458 | external_body | join_mid() (free fn) | Matches on Exposed |
| 16 | 471 | external_body | split_inner() | Recursive split; has specs |
| 17 | 496 | external_body | join_m() | Wraps join_mid |
| 18 | 501 | external_body | min_key() | Recursive minimum; no specs yet |
| 19 | 512 | external_body | join_pair_inner() | Uses min_key + split + join_m |
| 20 | 524 | external_body | union_inner() | ParaPair! fork-join |
| 21 | 538 | external_body | intersect_inner() | ParaPair! fork-join |
| 22 | 555 | external_body | difference_inner() | ParaPair! fork-join |
| 23 | 574 | external_body | filter_inner() | ParaPair! fork-join |
| 24 | 597 | external_body | filter_parallel() | Wraps filter_inner |
| 25 | 606 | external_body | reduce_inner() | ParaPair! fork-join |
| 26 | 630 | external_body | reduce_parallel() | Wraps reduce_inner |
| 27 | 640 | external_body | collect_in_order() | Sequential DFS accumulation |

The two RWLOCK_GHOST structural_false_positives (expose line 233, size line 272) are NOT
counted in the 27 actionable holes — leave them as is.

The 3 assume_eq_clone_workaround warnings (Clone impls for Exposed, NodeInner, ParamBST)
are not counted as holes by veracity — leave them as is per the standard.

## Strategy for Chap39 BSTParaTreapMtEph.rs

Start Chap39 only after making meaningful progress in Chap38. The design is the same:
opaque view, no ghost field, same structural limitation.

**Key difference**: BSTParaTreapMtEph is more complex than BSTParaMtEph because:
- It has Treap priorities (`i64 priority` field in NodeInner).
- `priority_for(key)` uses a hasher and is `#[verifier::external_body]` (OPAQUE_EXTERNAL,
  classified as structural info, not an actionable hole).
- The join operation is `join_with_priority` instead of `join_mid` — it rotates to maintain
  the heap property. Verifying `join_with_priority` requires a priority heap spec.
- `expose_with_priority` (line 522) has an algorithmic assume that covers all the key
  structural properties including the BST ordering AND set membership spec.

The Chap39 holes:

| # | Line | Type | Function | Notes |
|---|------|------|----------|-------|
| 1 | 67 | external_body | ParamTreap::view | Root cause |
| 2 | 142 | external_body | tree_priority() | Reads priority from lock |
| 3 | 152 | external_body | tree_size() | Reads size from lock |
| 4 | 162 | external_body | make_node() | Constructs NodeInner |
| 5 | 172 | external_body | join_with_priority() | Heap-rotation join |
| 6 | 197 | external_body | split_inner() | Recursive split |
| 7 | 220 | external_body | join_pair_inner() | Uses split + join_with_priority |
| 8 | 236 | external_body | union_inner() | ParaPair! fork-join |
| 9 | 252 | external_body | intersect_inner() | ParaPair! fork-join |
| 10 | 272 | external_body | difference_inner() | ParaPair! fork-join |
| 11 | 292 | external_body | filter_inner() | ParaPair! fork-join |
| 12 | 313 | external_body | filter_parallel() | Wraps filter_inner |
| 13 | 322 | external_body | reduce_inner() | ParaPair! fork-join |
| 14 | 346 | external_body | reduce_parallel() | Wraps reduce_inner |
| 15 | 356 | external_body | collect_in_order() | Sequential DFS |
| 16 | 507 | assume [algorithmic] | new() | Empty set spec |
| 17 | 537 | assume [algorithmic] | expose_with_priority() | Full structural spec |
| 18 | 555 | external_body | join_mid() impl | Delegates to join_with_priority |
| 19 | 663 | assume [algorithmic] | find() | Loop membership spec |
| 20 | 673 | external_body | split() | Delegates to split_inner |
| 21 | 678 | external_body | join_pair() | Delegates to join_pair_inner |
| 22 | 683 | external_body | union() | Delegates to union_inner |
| 23 | 688 | external_body | intersect() | Delegates to intersect_inner |
| 24 | 693 | external_body | difference() | Delegates to difference_inner |
| 25 | 698 | external_body | filter() | Delegates to filter_parallel |
| 26 | 707 | external_body | reduce() | Delegates to reduce_parallel |
| 27 | 717 | external_body | in_order() | Collects via collect_in_order |

The RWLOCK_GHOST structural info (size line 576) and OPAQUE_EXTERNAL (priority_for line 131)
are not actionable holes — leave them.

For Chap39, apply the same ghost-tracking migration if you succeeded in Chap38. The Treap
version needs additional spec work: a `spec_treap_priority_wf` predicate that captures the
max-heap property on priorities. Without a spec for `priority_for`, the heap property cannot
be stated in terms of the abstract priority values — use an existential or opaque spec for
the priority assigned to a key.

## Proof Technique Notes

### Ghost Tracking in MtEph vs. StEph

The StEph uses a single `RwLock<..., BSTParaStEphInv<T>>` with ghost contents in the
predicate struct. The MtEph uses `Arc<RwLock<..., BSTParaMtEphInv>>` with a unit-struct
predicate. To port the ghost-tracking pattern:

1. Change `BSTParaMtEphInv` to carry `pub ghost contents: Set<T::V>`.
   This is not a unit struct anymore — it needs type parameter `T`.
2. Add `ghost_locked_root: Ghost<Set<T::V>>` to `ParamBST<T>`.
3. Add `#[verifier::type_invariant]` spec fn linking them.
4. Update `new_arc_rwlock` call site to pass the predicate with contents.
   Check `src/vstdplus/arc_rwlock.rs` for how `new_arc_rwlock` works.

Note: `clone_arc_rwlock` in `vstdplus/arc_rwlock.rs` may need a spec adjustment
to carry the ghost contents through Arc clone. Read that file before modifying.

### ParaPair! Closure Specs

When proving functions that use `crate::ParaPair!(move || f1(), move || f2())`, each
closure must have explicit `ensures` per `src/standards/using_closures_standard.rs`.
The pattern is:
```rust
let ghost view1 = left@;
let ghost view2 = right@;
let f1 = move || -> (r: ParamBST<T>)
    ensures r@ =~= spec_op(view1, ...)
{ ... };
let f2 = move || -> (r: ParamBST<T>)
    ensures r@ =~= spec_op(view2, ...)
{ ... };
let Pair(res1, res2) = crate::ParaPair!(f1, f2);
```

Do NOT use inline closures in ParaPair! arms. Bind to named variables with explicit ensures.

### RwLock Acquire/Release Pattern

After `let handle = rwlock.acquire_read()`, use `handle.borrow()` to access the data.
The `handle.release_read()` must come before the proof block that uses the ghost state,
because after release the lock predicate reverts to the invariant. In the StEph, the
proof block comes after `handle.release_read()` and uses `use_type_invariant(self)` to
recover the ghost state. In the MtEph (once you add ghost tracking), follow the same
pattern.

### The find() Loop

`find()` uses a fuel-bounded `while remaining > 0` loop instead of recursion (because
recursion would require a `decreases` on `self@.len()`, which requires the type_invariant).
Once you add ghost tracking, you can switch find to recursive like StEph. Until then, the
loop approach with an `assume` at the end is the current state — do not change it without
first proving the ghost tracking.

### Structural false positives

The `expose()` body (line 233) has:
```rust
proof { assume(self@.len() == 0 ==> exposed is Leaf); }
```
This is classified by veracity as `RWLOCK_GHOST` structural info. It IS an algorithmic
assume in disguise — once you add ghost tracking and prove expose, this goes away. But
do not attempt to remove it in isolation.

## Execution Plan

1. Run `scripts/holes.sh src/Chap38/` and `scripts/holes.sh src/Chap39/` to confirm
   current hole counts match the baseline above.

2. Read `src/Chap38/BSTParaStEph.rs` in full. Note especially:
   - The `BSTParaStEphInv<T>` predicate struct with ghost contents.
   - The `new_param_bst()` constructor signature.
   - The `#[verifier::type_invariant]` block.
   - The `expose()` proof with `use_type_invariant`.
   - The `join_mid()` proof with ghost arithmetic.
   - The `split()` proof structure.
   - The `union()`, `intersect()`, `difference()` proofs.

3. Attempt the ghost-tracking migration in `src/Chap38/BSTParaMtEph.rs` (Option A).
   Start with structural changes, validate, then prove each function in order.

4. If Option A is not completing in reasonable time, fall back to Option B (prove only
   the 4 algorithmic assumes with minimal structural change).

5. Validate after each meaningful batch. Must show 0 errors.

6. When Chap38 work is stable and committed, move to Chap39 and apply what you learned.

7. Run `scripts/rtt.sh` before your final commit.

8. Run `scripts/holes.sh src/Chap38/ src/Chap39/` for your before/after counts.

## What NOT to Attempt

- Do NOT try to prove `priority_for()` in Chap39 — it uses DefaultHasher, which is not
  in Verus's verified API. The OPAQUE_EXTERNAL classification is correct.
- Do NOT try to close the RWLOCK_GHOST structural_false_positive notes — they are info.
- Do NOT try to prove `filter_parallel` or `reduce_parallel` in the Mt files without first
  establishing ghost tracking AND having full closure spec support — these are stretch goals.
- Do NOT add any new assume() or accept() patterns beyond what already exists, except
  inside Clone bodies per the standard.

## Validation

```bash
scripts/holes.sh src/Chap38/            # baseline hole count
scripts/holes.sh src/Chap39/            # baseline hole count
# ... work ...
scripts/validate.sh                     # must show 0 errors
scripts/rtt.sh                          # runtime tests must pass
scripts/holes.sh src/Chap38/ src/Chap39/  # final hole count
```

Do not run validate and rtt concurrently — they compete for CPU and memory. Always sequential.

## Report

Write your report to `plans/agent4-r43-report.md`.

Include:
- Holes before/after per file (table with Chap column).
- Which approach you took (Option A, B, or C) and why.
- Key proof techniques that worked.
- Remaining holes with a precise explanation of what blocks each one.
- Any structural insight about the MtEph vs. StEph ghost-tracking gap that future rounds
  should act on.

## Expected Outcome

Conservative: 5-10 holes closed (Option B + some external_body delegations).
Optimistic: 15-20 holes closed (Option A succeeds for BSTParaMtEph).
Best case: 20+ holes if Chap39 also gets ghost tracking.

This is the hardest assignment in R43. The structural work is real — not just proof massage.
Do the proof work. If you hit a wall, document exactly where and what you tried.
