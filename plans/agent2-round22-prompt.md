# Agent 2 — Round 22: Prove Chap38 + Chap39 + Chap40 + Chap05 size

## Mission

Prove holes across 4 chapters with clean dependencies. All targets are independent of the
Chap37 root blocker.

## Targets

| # | Chap | File | Hole Type | Count |
|---|------|------|-----------|-------|
| 1 | 05 | MappingStEph.rs | external_body on `size` | 1 |
| 2 | 38 | BSTParaStEph.rs | 4 assume() + 1 external_body (clone) | 5 |
| 3 | 38 | BSTParaMtEph.rs | 8 external_body + 1 assume_specification | 9 |
| 4 | 39 | BSTTreapStEph.rs | 2 external_body | 2 |
| 5 | 39 | BSTTreapMtEph.rs | 6 assume() | 6 |
| 6 | 39 | BSTParaTreapMtEph.rs | 10 external_body | 10 |
| 7 | 40 | BSTKeyValueStEph.rs | external_body | ? |
| 8 | 40 | BSTReducedStEph.rs | external_body | ? |
| 9 | 40 | BSTSizeStEph.rs | external_body | ? |

## Priority 1: Chap05 MappingStEph::size (trivial)

```rust
#[verifier::external_body]
fn size(&self) -> (size: N) { self.mapping.size() }
```

This just delegates to the underlying mapping's `size()`. Remove `external_body`, the body
is already correct. The proof should be trivial — `self.mapping.size()` already has ensures
on its return value.

## Priority 2: Chap38 BSTParaStEph assumes

The 4 `assume()` calls in BSTParaStEph.rs are:
- `assume(k@ == node.key@)` in `expose` — clone doesn't preserve view (Clone bridge)
- `assume(left@.len() + right@.len() < usize::MAX as nat)` in `insert` — size bound
- Same size bound assume in `delete`
- Same size bound assume in `union`

The size bound assumes may need `spec_wf` to establish that the tree size is bounded.
The clone assume is the standard Clone bridge workaround.

## Priority 3: Chap39 Treap holes

BSTTreapStEph.rs has 2 external_body. BSTTreapMtEph.rs has 6 assumes (likely Clone/eq
bridge workarounds). BSTParaTreapMtEph.rs has 10 external_body (Mt wrapper operations).

## Priority 4: Chap40 augmented BSTs

BSTKeyValueStEph, BSTReducedStEph, BSTSizeStEph — augmented BST variants. Read the files
to understand hole types.

## Approach

- Read each file before attempting proofs.
- Read the prose: `prompts/Chap38.txt`, `prompts/Chap39.txt`, `prompts/Chap40.txt`
- Read the fn-impls: `src/ChapNN/analyses/veracity-review-module-fn-impls.md`
- Start with Chap05 (guaranteed quick win), then Chap38 StEph, then work down the list.
- For Clone bridge assumes: these are the standard workaround pattern. Leave them if they're
  inside `Clone::clone` bodies. Flag if they're in algorithmic code.
- You MAY add `requires spec_wf(self)` to functions flagged `fn_missing_requires`.
- Do NOT weaken existing ensures or remove existing requires.

## Important

- Do NOT add `assume`, `accept`, or `external_body`.
- `scripts/validate.sh` after each chapter — 0 errors.

## Deliverables

- Proven holes in source files.
- `plans/agent2-round22-report.md`
- 0 errors on validate.
- Commit + push to `agent2/ready`.
