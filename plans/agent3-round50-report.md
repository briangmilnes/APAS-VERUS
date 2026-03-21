<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Agent 3 — Round 50 Report

## Summary

Refactored `src/Chap39/BSTParaTreapMtEph.rs` from `Arc<RwLock<...>>` to plain `RwLock<...>`,
implementing a comprehensive `RwLockPredicate` with real BST ordering invariants,
`#[verifier::type_invariant]`, and full proof scaffolding for `join_pair_inner`,
`union_inner`, `intersect_inner`, `difference_inner`, and `filter_inner`.

**Verification: 4472 verified, 0 errors, 0 trigger warnings.**

## Holes Before/After

| # | Chap | File | Before (holes) | After (holes) | Delta |
|---|:----:|---|:---:|:---:|:---:|
| 1 | 39 | BSTParaTreapMtEph.rs | ~30 (pre-refactor) | 2 | -28 |
| 2 | 39 | BSTSetTreapMtEph.rs | 0 | 0 | 0 |
| 3 | 39 | BSTTreapStEph.rs | 0 | 0 | 0 |
| 4 | 39 | BSTTreapMtEph.rs | unchanged | unchanged | 0 |

## Chapters Closed

- **Chap39/BSTParaTreapMtEph.rs** — Main refactoring complete; 0 verification errors.

## Verification Counts

| Phase | Count |
|---|---|
| Verified | 4472 |
| Errors | 0 |
| Trigger warnings | 0 |

## Key Techniques

### 1. `if` case-split inside `assert forall` for conjunction hypothesis splitting

The root cause of persistent `assertion failed` errors: inside `assert forall|s, o| H(s,o) ==> C(s,o) by { ... }`,
Z3 does not split the conjunction `H(s,o)` into unit ground facts. E-matching on the one-variable
ordering foralls requires unit ground facts. The fix:

```rust
assert forall|s: T, o: T| #![trigger lrv.contains(s@), rrv.contains(o@)]
    lrv.contains(s@) && rrv.contains(o@) ==> s.cmp_spec(&o) == Less by {
    reveal(ParamTreap::spec_ghost_locked_root);
    if lrv.contains(s@) && rrv.contains(o@) {
        // Both are unit facts here — E-matching fires on one-var ordering foralls.
        assert(s.cmp_spec(&ak) == Less);
        assert(o.cmp_spec(&ak) == Greater);
        lemma_cmp_antisymmetry(o, ak);
        lemma_cmp_transitivity(s, ak, o);
    }
};
```

The `if` arm forces case-split; inside the branch, each conjunct is a unit ground fact.

### 2. `reveal(ParamTreap::spec_ghost_locked_root)` for closed-spec transparency

`spec_ghost_locked_root` is `closed` (private). Inside `assert forall` bodies, Z3 can't
see the set definition without an explicit `reveal`. Adding `reveal(...)` at the top of
each `by {}` block eliminated all "function is uninterpreted" errors.

### 3. Ghost view capture before moves

Before `ParaPair!` moves exec variables into closures, capture their ghost views:
```rust
let ghost lrv = lr@;
let ghost rlv = rl@;
```
These ghost variables remain available in the ambient proof context after the moves.

### 4. Ordering foralls materialized before closures

One-variable ordering foralls (establishing all elements in a subtree are `< key` or `> key`)
must be asserted before the `move` closures consume the exec variables:
```rust
assert forall|t: T| #[trigger] lrv.contains(t@) implies t.cmp_spec(&k) == Less by { ... };
```
This puts the quantified fact into the SMT context so it's available for later cross-ordering
proofs.

## Remaining Holes (Actionable)

| # | Chap | File | Line | Type | Description | Blocked by |
|---|:----:|---|:---:|---|---|---|
| 1 | 39 | BSTParaTreapMtEph.rs | 127 | assume | `clone_elem` clone bridge | Clone workaround pattern |
| 2 | 39 | BSTParaTreapMtEph.rs | 1696 | assume | `filter_parallel` pred send limit | `Pred<T>` not `Send` |

## Warnings (Non-critical)

| # | Chap | File | Issue | Reason |
|---|:----:|---|---|---|
| 1 | 39 | BSTParaTreapMtEph.rs | `fn_missing_requires` on `param_treap_assert_finite` | No real preconditions; needs `// veracity: no_requires` |
| 2 | 39 | BSTParaTreapMtEph.rs | `fn_missing_requires` on `tree_priority_internal` | No real preconditions; needs `// veracity: no_requires` |
| 3 | 39 | BSTParaTreapMtEph.rs | `fn_missing_ensures` on `reduce_inner` | Can't spec without `op.ensures`; generic `F` |
| 4 | 39 | BSTParaTreapMtEph.rs | `fn_missing_ensures` on `reduce_parallel` | Same as above |
| 5-8 | 39 | BSTParaTreapMtEph.rs | Clone workaround assumes | Acceptable pattern for BST recursive clone |
