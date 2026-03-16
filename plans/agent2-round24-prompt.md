# Agent 2 — Round 24: Chap40 BST Ordering Invariant

## Mission

Define `spec_ordered_link` (BST ordering invariant), add it to wf predicates, prove
preservation through rotations and insert, then prove find/contains/delete across all
3 Chap40 files. You proved `insert` in BSTSizeStEph in R23 — now finish the job.

## Current State (13 holes)

| # | Chap | File | Holes | Functions |
|---|------|------|:-----:|-----------|
| 1 | 40 | BSTSizeStEph.rs | 3 | find, contains, delete |
| 2 | 40 | BSTKeyValueStEph.rs | 5 | insert, delete, find, contains, get |
| 3 | 40 | BSTReducedStEph.rs | 5 | insert, delete, find, contains, get |

## What You Already Have (from R23)

In BSTSizeStEph.rs you already proved:
- `make_node` content ensures
- `rotate_left`/`rotate_right` content preservation (pre-move ghost capture technique)
- `insert_link` content ensures with cmp_spec eq bridge
- `find_link` soundness ensures

What's missing for find/contains: the **completeness** direction — if an element IS in
the tree, find WILL return it. This requires the BST ordering invariant to prove the
search goes to the correct subtree.

## Step 1: Define `spec_ordered_link`

```rust
open spec fn spec_ordered_link<K: View, V: View>(link: &Link<K, V>) -> bool
    decreases *link,
{
    match link {
        None => true,
        Some(node) => {
            &&& spec_ordered_link(&node.left)
            &&& spec_ordered_link(&node.right)
            &&& forall|k: K| spec_content_link(&node.left).contains(k)
                ==> k.cmp_spec(&node.key) == Less
            &&& forall|k: K| spec_content_link(&node.right).contains(k)
                ==> k.cmp_spec(&node.key) == Greater
        }
    }
}
```

The exact formulation depends on how `spec_content_link` works in each file — it may
return `Set<T>`, `Set<(K,V)>`, or `Map<K,V>`. Adapt accordingly.

## Step 2: Add to wf predicates

```rust
open spec fn spec_bstsizesteph_wf(self) -> bool {
    &&& /* existing size constraints */
    &&& spec_ordered_link(&self.root)
}
```

Same for BSTKeyValueStEph and BSTReducedStEph.

## Step 3: Prove ordering preservation

- **Rotations**: Rotation preserves BST ordering if it preserves content (already proved)
  and doesn't change the relative ordering. You may need explicit proof steps.
- **Insert**: BST insert into the correct subtree based on comparison preserves ordering.
  Your R23 `match value.cmp(&node.key)` restructuring already navigates correctly.

## Step 4: Prove find completeness

With `spec_ordered_link` in the invariant:
```rust
fn find_link(link: &Link<K, V>, target: &K) -> (found: Option<&V>)
    requires spec_ordered_link(link),
    ensures
        found is Some <==> spec_content_link(link).contains(*target),
```

The `<==` direction (completeness) now provable: if target is in the tree, the BST
ordering guarantees the search visits the correct subtree, so it will be found.

## Step 5: Prove delete

Delete typically uses find + remove + rebalance. With ordering invariant and content
tracking through the chain, the ensures `self@ == old(self)@.remove(key)` should follow.

## Step 6: Apply to BSTKeyValueStEph and BSTReducedStEph

The same pattern applies to all 3 files. BSTKeyValueStEph uses `Map<K,V>` with
`TotalOrder::cmp` (which already gives structural equality). BSTReducedStEph uses
plain operators — apply the `cmp_spec` bridge from R23.

## Reference

- Your R23 report: `plans/agent2-round23-report.md` — rotation content preservation,
  cmp_spec eq bridge, pre-move ghost capture technique
- BSTPlainStEph.rs or BSTAVLStEph.rs — clean BST files that may already have ordering
  invariant patterns you can reference
- `src/standards/total_order_standard.rs` — TotalOrder pattern

## Important

- You MAY add `spec_ordered_link` and strengthen wf predicates — that's the goal.
- You MAY add requires to functions (e.g., `requires spec_ordered_link(link)`).
- Do NOT weaken any existing ensures.
- Do NOT add `assume`, `accept`, or `external_body`.
- `scripts/validate.sh` after each file — 0 errors.

## Deliverables

- `spec_ordered_link` defined and added to wf in all 3 files
- Proven find/contains/delete where possible
- `plans/agent2-round24-report.md`
- 0 errors on validate.
- Commit + push to `agent2/ready`.
