# Agent 1 — Round 22: Prove Chap37 BST Holes (Root Blocker)

## Mission

Prove the 14 `external_body` holes across Chap37. This chapter is the root blocker —
Chap41, 42, 43, 45, 52, 53, 55, 57, 65 all depend on it.

## Current State

Chap37 has 19 files. The StEph files are already clean:

| # | Chap | File | Status | Holes |
|---|------|------|--------|-------|
| 1 | 37 | BSTPlainStEph.rs | clean | 0 |
| 2 | 37 | BSTAVLStEph.rs | clean | 0 |
| 3 | 37 | BSTRBStEph.rs | clean | 0 |
| 4 | 37 | BSTBBAlphaStEph.rs | clean | 0 |
| 5 | 37 | BSTSplayStEph.rs | holed | 6 ext_body (BSTSet ops) |
| 6 | 37 | AVLTreeSeq.rs | holed | 1 ext_body (iterator next) |
| 7 | 37 | AVLTreeSeqStEph.rs | holed | 1 ext_body (set) |
| 8 | 37 | AVLTreeSeqStPer.rs | holed | 1 ext_body |
| 9 | 37 | AVLTreeSeqMtPer.rs | holed | 2 ext_body (build_balanced, subseq_copy) |
| 10 | 37 | BSTSplayMtEph.rs | holed | 1 ext_body |
| 11 | 37 | BSTRBMtEph.rs | holed | 2 ext_body |
| 12 | 37 | BSTSetPlainMtEph.rs | info only | 0 real holes |
| 13 | 37 | BSTSetAVLMtEph.rs | info only | 0 real holes |
| 14 | 37 | BSTSetBBAlphaMtEph.rs | info only | 0 real holes |
| 15 | 37 | BSTSetRBMtEph.rs | info only | 0 real holes |
| 16 | 37 | BSTSetSplayMtEph.rs | info only | 0 real holes |
| 17 | 37 | BSTPlainMtEph.rs | info only | 0 real holes |
| 18 | 37 | BSTAVLMtEph.rs | info only | 0 real holes |
| 19 | 37 | BSTBBAlphaMtEph.rs | info only | 0 real holes |

Note: Many files show as "holed" in the global count due to `fn_missing_requires` warnings.
These exec fns should have `requires spec_wf(self)` (or the module's wf predicate) added.
Fix these as you go — they're real gaps, not false positives. A BST function that operates
on a tree needs to require the tree is well-formed.

## Priority Order

1. **AVLTreeSeq.rs** — iterator `next` (1 ext_body). High value: unblocks Chap41/45.
2. **AVLTreeSeqStEph.rs** — `set` (1 ext_body). Core sequence operation.
3. **AVLTreeSeqStPer.rs** — (1 ext_body). Should parallel StEph.
4. **AVLTreeSeqMtPer.rs** — `build_balanced_from_slice`, `subseq_copy` (2 ext_body).
5. **BSTSplayStEph.rs** — 6 ext_body on BSTSet operations (union, intersect, etc.).
6. **BSTSplayMtEph.rs** — 1 ext_body.
7. **BSTRBMtEph.rs** — 2 ext_body.

## Approach

- Read the prose: `prompts/Chap37.txt`
- Read the fn-impls: `src/Chap37/analyses/veracity-review-module-fn-impls.md`
- Read the review: `src/Chap37/analyses/review-against-prose.md`
- For each `external_body`, read the function, understand its spec, write the proof body.
- The BST multi-struct pattern uses `decreases *self` for recursive specs. Read
  `src/standards/multi_struct_standard.rs` if unfamiliar.
- The AVLTreeSeq iterator uses the standard iterator pattern. Read
  `src/standards/iterators_standard.rs`.

## Important

- You MAY add `requires spec_wf(self)` to functions flagged `fn_missing_requires`.
- Do NOT weaken existing ensures or remove existing requires.
- Do NOT add `assume`, `accept`, or `external_body`.
- `scripts/validate.sh` after each file — 0 errors.
- The `fn_missing_requires` warnings are informational. A function with no precondition
  is fine if it genuinely works for all inputs.

## Deliverables

- Proven `external_body` holes in Chap37 source files.
- `plans/agent1-round22-report.md`
- 0 errors on validate.
- Commit + push to `agent1/ready`.
