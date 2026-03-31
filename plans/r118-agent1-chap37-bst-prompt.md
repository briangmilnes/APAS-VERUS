# R118 Agent 1 — Strengthen Chap37 BST MtEph specs. AFK. DOT.

## Problem

`veracity-compare-par-mut` reports 41 warnings on Chap37. Six MtEph files
have weaker specs than their StEph counterparts. The warnings cluster into
three patterns: missing wf/structural requires, missing ensures clauses,
and missing spec functions.

## Files and warning counts

| # | Chap | File | Warnings | Pattern |
|---|------|------|----------|---------|
| 1 | 37 | BSTAVLMtEph.rs | 9 | Missing tree_is_avl/tree_is_bst requires/ensures |
| 2 | 37 | BSTRBMtEph.rs | 12 | Missing tree_is_bst requires, insert ensures |
| 3 | 37 | BSTSplayMtEph.rs | 9 | Missing 5 spec fns, weak ensures |
| 4 | 37 | BSTPlainMtEph.rs | 1 | Missing `delete` |
| 5 | 37 | BSTBBAlphaMtEph.rs | 1 | Missing `delete` |
| 6 | 37 | AVLTreeSeqStEph.rs | 3 | Missing `values_in_order`, weak requires/ensures vs StPer |
| 7 | 37 | AVLTreeSeqMtPer.rs | 6 | Missing `to_arrayseq`, weak set/iter/values_in_order specs |

## Warnings by file

### BSTAVLMtEph.rs (9 warnings)

- `new`: missing `tree_is_avl::<T>(tree.spec_root())` ensures
- `insert`: missing `spec_root().spec_height() <= usize::MAX - 1` requires
- `insert`: missing `tree_is_avl::<T>(inserted.spec_root())` ensures
- `contains`: missing `self.spec_root().tree_is_bst()` requires
- `find`: missing `self.spec_root().tree_is_bst()` requires

Note: R116 already strengthened `spec_bstavlmteph_wf` to include `tree_is_avl`.
These remaining warnings are about individual function requires/ensures that
should reference wf or tree_is_bst/tree_is_avl directly.

### BSTRBMtEph.rs (12 warnings)

- `new`: missing `tree.spec_root().tree_is_bst()` ensures
- `insert`: missing `tree_is_bst` requires, missing 3 ensures (tree_contains,
  tree_is_bst, forall containment preservation)
- `contains`: missing `tree_is_bst` requires
- `find`: missing `tree_is_bst` requires
- `size`: missing `spec_size() <= usize::MAX` requires
- `height`: missing `spec_height() <= usize::MAX` requires

### BSTSplayMtEph.rs (9 warnings)

- Missing 5 spec fns: `spec_size`, `spec_height`, `spec_contains`,
  `spec_in_order`, `spec_pre_order`. These are tree-structural specs that
  may not be expressible on the MtEph type (behind RwLock). Assess whether
  the ghost view supports them.
- `height`: missing `spec_height() < usize::MAX` requires
- `insert`: missing `self.spec_contains(value)` ensures
- `in_order`: missing `seq.spec_len() == self.spec_in_order().len()` ensures
- `pre_order`: missing `seq.spec_len() == self.spec_pre_order().len()` ensures

### BSTPlainMtEph.rs (1 warning) + BSTBBAlphaMtEph.rs (1 warning)

Both missing `delete`. Check if delete exists as a free function or needs
implementing. If the underlying StEph delete logic is complex (splay/rebalance),
document what's needed but don't implement unless straightforward.

### AVLTreeSeqStEph.rs (3 warnings)

- Missing `values_in_order` from StPer
- `set`: missing `obeys_feq_clone::<T>()` requires
- `from_vec`: missing `tree.spec_seq() =~= values@.map_values(|t: T| t@)` ensures

### AVLTreeSeqMtPer.rs (6 warnings)

- Missing `to_arrayseq` from StPer
- `values_in_order`: missing requires (StPer has them)
- `set`: missing `outcome.unwrap().spec_seq() =~= ...update(index)` ensures
- `iter`: missing `avltreeseq_iter_invariant(&it)` ensures

## Strategy

1. Start with BSTRBMtEph (12 warnings, most mechanical — adding wf requires
   and tree_is_bst ensures).
2. Then BSTAVLMtEph (9 warnings, similar pattern, R116 groundwork helps).
3. Then BSTSplayMtEph (9 warnings, assess spec fn feasibility first).
4. Then AVLTreeSeq files (9 warnings combined).
5. BSTPlainMtEph + BSTBBAlphaMtEph last (2 warnings, may need delete impl).

## Read first

For each file pair, read the StEph variant first (for reference specs),
then the MtEph variant.

## Validate

Use `scripts/validate.sh isolate Chap37`.
Run `scripts/rtt.sh Chap37` after.

## Rules

- Do NOT weaken any ensures.
- Do NOT add assume or accept in algorithmic code.
- Mt standalone: do NOT import from StEph. Copy spec fn bodies.
- If a spec fn can't be defined on the MtEph type (RwLock hides tree
  structure), document why and skip. Don't force it.
- No subagents.

## STEP 30

## Report

Write `plans/agent1-r118-chap37-bst-report.md`. Include before/after
warning count per file.
