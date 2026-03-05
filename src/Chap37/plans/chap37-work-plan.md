# Chapter 37 Work Plan — Binary Search Trees

Generated 2026-03-05. Based on full chapter review: verusification state,
proof-holes log, style log, and textbook alignment.

## Status Summary

- 19 files, 11,287 lines
- 42 holes (13 assume, 29 external_body) — 38 in AVLTreeSeq* family
- 9 proof fns, all clean
- Analysis logs up to date
- No trigger warnings

## Spec Strength

| Strength | Files |
|----------|-------|
| Strong | BSTPlainStEph, BSTAVLStEph |
| Partial | BSTBBAlphaStEph, BSTRBStEph, BSTPlainMtEph, BSTAVLMtEph, AVLTreeSeq |
| Weak | BSTSplayStEph, BSTRBMtEph, BSTSplayMtEph, AVLTreeSeqStPer, AVLTreeSeqMtPer |
| None | BSTSetPlainMtEph, BSTSetAVLMtEph, BSTSetBBAlphaMtEph, BSTSetRBMtEph, BSTSetSplayMtEph |

## Threading

Textbook explicitly says ADT is "designed to support parallelism":
- Individual ops (find, insert, delete) are sequential O(lg n).
- Bulk ops (union, intersection, difference, filter, reduce) are parallel.
- MtEph files correctly wrap sequential ops behind RwLock.
- BSTSet files use ParaPair! for union/intersection/difference (correct).
- BSTSet split/join are O(n) (wrong, should be O(lg n)).
- BSTSet filter/reduce are sequential (wrong, should be parallel).

## Proposed Work

| # | Sev | File(s) | Work | Notes |
|---|-----|---------|------|-------|
| 1 | high | BSTBBAlphaStEph.rs | Prove weight_balanced through insert/delete | Spec exists; needs balance proof |
| 2 | high | BSTSet*MtEph.rs (5) | O(lg n) split via BST traversal | Current O(n) via in_order+filter |
| 3 | high | BSTSet*MtEph.rs (5) | O(lg n) join_pair/join_m | Current O(n) via BTreeSet rebuild |
| 4 | med | BSTPlainMtEph.rs | Add requires/ensures to trait | Verified ops, specless trait |
| 5 | med | BSTAVLMtEph.rs | Add requires/ensures to trait | Same as BSTPlainMtEph |
| 6 | med | BSTBBAlphaMtEph.rs | Add requires/ensures to trait | Same pattern |
| 7 | med | BSTRBStEph.rs | Extend BalBinTree with color | Enables RB color invariant |
| 8 | med | BSTRBMtEph.rs | Migrate std::sync to vstd::rwlock | Matches Plain/AVL/BB[a] pattern |
| 9 | med | BSTSplayMtEph.rs | Migrate std::sync to vstd::rwlock | Same |
| 10 | med | BSTSplayStEph.rs | Prove BST ordering on splay/insert | 2 holes; no BST preservation |
| 11 | med | BSTSet*MtEph.rs (5) | Parallel filter/reduce | Textbook says O(lg n) span |
| 12 | low | BSTPlainMtEph.rs | Remove duplicate TOC/section comments | Style cleanup |
| 13 | low | BSTRBMtEph.rs | Eliminate size overflow assume | Add wf precondition |
| 14 | low | BSTSplayMtEph.rs | Eliminate size overflow assume | Add wf precondition |
| 15 | low | BSTSplayStEph.rs | Eliminate Node clone external_body | Prove clone correctness |
| 16 | low | AVLTreeSeq*.rs (4) | Lift external_body on 29 fns | Large workstream, 38 holes |
| 17 | low | AVLTreeSeq*.rs (4) | Fix style (free spec fns to traits, TOC order) | 20+ style warnings |

## Recommended Order (simplest first)

12 -> 4,5,6 -> 13,14 -> 8,9 -> 10 -> 1 -> 15 -> 7 -> 2,3,11 -> 16,17

Items 2/3/11 require O(lg n) split/join on BalBinTree (significant algorithmic task).
Item 7 affects Chap23 BalBinTree and cascades to all BST files.
Items 16/17 (AVLTreeSeq) are a self-contained workstream.
