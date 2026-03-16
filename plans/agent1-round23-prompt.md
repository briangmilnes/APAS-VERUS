# Agent 1 — Round 23: Chap37 BSTSplay Spec Infrastructure

## Mission

Build spec infrastructure for BSTSplayStEph.rs and prove its 6 `external_body` holes +
1 trivial_spec_wf. The splay tree is the last major hole source in Chap37 (7 of 10
remaining holes). The Mt wrapper (BSTSplayMtEph.rs, 1 hole) should follow once StEph
is solid.

## Current State: BSTSplayStEph.rs (7 holes)

| # | Chap | Hole | Function | Issue |
|---|------|------|----------|-------|
| 1 | 37 | trivial_spec_wf | `spec_bstsplaysteph_wf` | Body is `{ true }` — needs `spec_is_bst_link` |
| 2 | 37 | external_body | `insert` | Delegates to `insert_link` which has weak ensures |
| 3 | 37 | external_body | `find` | Delegates to `find_link` which has weak ensures |
| 4 | 37 | external_body | `minimum` | Delegates to `min_link` |
| 5 | 37 | external_body | `maximum` | Delegates to `max_link` |
| 6 | 37 | external_body | `in_order` | Tree traversal to sequence |
| 7 | 37 | external_body | `pre_order` | Tree traversal to sequence |

## The TODO Already in the Code

Line 519: `// TODO: Strengthen to spec_is_bst_link(&self.root) when splay/bst_insert proves BST preservation.`

That's your job. Do it.

## Approach

### Step 1: Strengthen `spec_bstsplaysteph_wf`

Change from `{ true }` to a real BST invariant:
```rust
open spec fn spec_bstsplaysteph_wf(self) -> bool {
    spec_is_bst_link(&self.root)
}
```

You'll need `spec_is_bst_link` that verifies BST ordering. It may already exist in the
file or you may need to write it. It should verify that for every node, all left subtree
values are less and all right subtree values are greater.

### Step 2: Strengthen helper ensures

The _link helper functions (`insert_link`, `find_link`, `min_link`, `max_link`, `splay`,
`bst_insert`) currently have weak ensures (likely `ensures true` or size-only). Add
content ensures:

- `insert_link`: preserves existing elements, adds new element, preserves BST property
- `find_link`: returns the element if present
- `splay`: preserves elements (multiset equality), preserves BST property
- `min_link`/`max_link`: returns the minimum/maximum element
- `in_order_link`/`pre_order_link`: produces correctly ordered sequence

### Step 3: Prove top-level operations

With strengthened helpers, the top-level `insert`, `find`, `minimum`, `maximum`,
`in_order`, `pre_order` should be provable by delegation.

### Step 4: BSTSplayMtEph.rs (1 hole)

If time permits, prove the Mt wrapper's remaining hole.

## Also: Remaining AVLTreeSeq holes (3)

If you finish BSTSplay, attack the remaining Chap37 holes:
- AVLTreeSeq.rs: iterator `next` (1 ext_body)
- AVLTreeSeqMtPer.rs: `build_balanced_from_slice`, `subseq_copy` (2 ext_body)

## Reference

- Read the prose: `prompts/Chap37.txt` for splay tree operations
- Read existing clean BST StEph files (BSTPlainStEph.rs, BSTAVLStEph.rs) for how
  they structure BST invariants and content ensures
- Read `src/standards/multi_struct_standard.rs` for the recursive spec pattern

## Important

- You MAY strengthen `spec_bstsplaysteph_wf`, add requires to functions, and strengthen
  ensures on helper functions. That is the explicit goal.
- Do NOT weaken any existing ensures.
- Do NOT add `assume`, `accept`, or `external_body`.
- `scripts/validate.sh` after changes — 0 errors.

## Deliverables

- Strengthened spec infrastructure in BSTSplayStEph.rs
- Proven external_body holes
- `plans/agent1-round23-report.md`
- 0 errors on validate.
- Commit + push to `agent1/ready`.
