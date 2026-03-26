# R80 Agent 4 — BSTSplayMtEph build_balanced sorted precondition (Chap37, 1 hole)

## Objective

Prove the 1 remaining assume in `build_balanced` in BSTSplayMtEph.rs by adding a sorted
precondition and proving BST ordering is preserved through recursive construction.

## Baseline

- 4908 verified, 0 errors, 0 warnings
- BSTSplayMtEph.rs: 1 hole (assume `spec_is_bst_link(Some(node))` in build_balanced)

## The hole

Agent 3 R79 narrowed `build_balanced` from `external_body` to 1 assume:
```rust
assume(spec_is_bst_link(Some(node)));
```

This assume says the constructed node is a valid BST. It's true because `build_balanced`
splits a sorted slice at its midpoint — left elements < pivot < right elements — and
recurses. But the function currently lacks a sorted precondition.

## Strategy

1. **Add sorted precondition** to `build_balanced`:
   ```rust
   requires
       forall|i: int, j: int| 0 <= i < j < values@.len() ==>
           #[trigger] values@[i]@ <= #[trigger] values@[j]@
   ```

2. **Prove BST ordering** from sorted + split:
   - Left slice `values[..mid]` has all elements < `values[mid]` (sorted)
   - Right slice `values[mid+1..]` has all elements > `values[mid]` (sorted)
   - Recursive calls produce BST-valid subtrees (by induction)
   - Node with left subtree, pivot = `values[mid]`, right subtree is BST-valid

3. **Check callers** — who calls `build_balanced`? They must now prove the sorted
   precondition. Since `build_balanced` is called on sorted slices from `in_order()`
   results, the precondition should be satisfiable.

## Key resources

- `src/Chap37/BSTSplayMtEph.rs` — Read `build_balanced` and its callers
- Agent 3 R79 report: `plans/agent3-round79-report.md`

## Validation

Run `scripts/validate.sh`, then `scripts/rtt.sh`, then `scripts/ptt.sh`. Push to `agent4/ready`.

## Report

Write `plans/agent4-round80-report.md` with holes before/after (table with Chap column).
