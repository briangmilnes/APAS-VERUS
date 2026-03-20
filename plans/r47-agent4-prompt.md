# R47 Agent 4: Chap38 BSTParaMtEph (9 holes)

## Assignment

Continue proving BSTParaMtEph.rs. You've closed 25 holes across R43-R45.
The remaining 9 are the hardest — RwLock boundary, BST ordering gaps,
Arc::clone limitations. Agent 2 is taking Chap39 this round.

## Baseline

47 holes total. 4413 verified. Your chapter: Chap38 (9 holes).

## Current State

Run `scripts/holes.sh src/Chap38/` to see current holes with root-cause annotations.

From your R45 report:

| # | Chap | File | Function | Type | Blocker |
|---|------|------|----------|------|---------|
| 1 | 38 | BSTParaMtEph.rs | expose_internal | external_body | ROOT CAUSE: RwLock boundary |
| 2 | 38 | BSTParaMtEph.rs | split_inner | external_body | BST ordering not in ghost |
| 3 | 38 | BSTParaMtEph.rs | min_key | external_body | Downstream of expose_internal |
| 4 | 38 | BSTParaMtEph.rs | join_pair_inner | external_body | Downstream of min_key |
| 5 | 38 | BSTParaMtEph.rs | intersect_inner | external_body | Downstream of split_inner |
| 6 | 38 | BSTParaMtEph.rs | difference_inner | external_body | Downstream of split_inner |
| 7 | 38 | BSTParaMtEph.rs | filter_inner | external_body | Arc::clone missing spec |
| 8 | 38 | BSTParaMtEph.rs | reduce_inner | external_body | Arc::clone missing spec |
| 9 | 38 | BSTParaMtEph.rs | find | assume | BST search correctness |
| 10 | 38 | BSTParaStEph.rs | clone_elem | assume | Clone workaround (irreducible) |

Run holes.sh yourself to verify — line numbers may have shifted.

Note: #10 (clone_elem in BSTParaStEph.rs) is the standard Clone workaround,
likely irreducible.

## Strategy

### 1. Arc::clone helper

`filter_inner` and `reduce_inner` are blocked by Arc::clone. Factor the
Arc::clone into a tiny `external_body` helper with tight ensures:

```rust
#[verifier::external_body]
fn clone_param_bst(tree: &ParamBST<T>) -> (out: ParamBST<T>)
    ensures out@ == tree@
{ tree.clone() }
```

If the rest of `filter_inner`/`reduce_inner` can then be proved, you trade
2 hard holes for 1 small external_body helper. Net -1 at minimum, and
`filter_parallel`/`reduce_parallel` were already proved in R45 (they
delegate to these).

### 2. min_key cascade

`min_key` calls `expose_internal`. If expose_internal's ensures are strong
enough, min_key may prove. Then `join_pair_inner` (which calls min_key)
may cascade.

### 3. BST ordering in ghost field

`split_inner`, `intersect_inner`, `difference_inner` need BST ordering.
The ghost field carries `Set<T::V>` — membership only. Options:
- Add a ghost ordering predicate to the struct
- Prove ordering from the BST structure via expose_internal's ensures
- Add a spec function that the BST invariant implies sorted order

This is the hardest problem. If you can't solve it, focus on #1 and #2.

### 4. find assume

The `find` assume needs BST search correctness. Same ordering gap as #3.
If ordering lands in the ghost field, this may close too.

## What NOT to do
- Do NOT add `#[cfg(not(verus_keep_ghost))]` to anything. Forbidden on fn/impl/type.
- Do NOT add `assume()` or `accept()` without user approval.
- Do NOT weaken ensures clauses.
- Do NOT sequentialize parallel implementations.

## Validation

Run `scripts/validate.sh` after each file change. Show full output.
Run `scripts/rtt.sh` after all changes.
Run `scripts/holes.sh src/Chap38/`.
Write your report to `plans/agent4-round47-report.md`.
