# Agent 3 — Round 61

You are Agent 3 working in `~/projects/APAS-VERUS-agent3`.

## Baseline

- Main: 4496 verified, 0 errors, 18 holes, 2610 RTT, 147 PTT.
- Warnings: 159 (8 real fn_missing_wf in Chap47, all others are
  eq_clone_workaround structural warnings).
- Your worktree: rebase onto main before starting (`git fetch origin && git rebase origin/main`).

## Context

Most remaining holes are in Chap53 (Agent 2's territory) or structural
(Chap26 float, Chap45 Example). The proof frontier has narrowed.

## Targets

### Target 1: Chap43 OrderedSetStEph.rs `select` — 1 hole

This is the `select` rank hole at line 1146:
```
assume(self@.filter(|x| exists|t| le(t, result) && t@ == x && t@ != result@).len() == i)
```

The blocker is: the BST ordering invariant implies sorted inorder
traversal, but no lemma bridges the two. Agent 1 is also assigned this
(Target 2) — you may both attempt it independently (standalone rule means
each file gets its own proof).

Your approach for `OrderedSetStEph.rs` specifically:
1. Read `src/Chap41/AVLTreeSetStEph.rs` for the BST wf predicate and
   `spec_inorder_values`.
2. Read `src/Chap43/OrderedSetStEph.rs` `select` to understand exactly
   what the assume asserts.
3. The key insight: `select(i)` returns the i-th element in sorted order.
   The filter counts elements ≤ result, which equals i by the BST
   rank property. You need to connect the AVL tree's structural BST
   invariant to this counting property.
4. Write whatever helper lemmas you need locally in OrderedSetStEph.rs.

### Target 2: Chap43 OrderedSetStPer.rs `select` — 1 hole

Same hole, different file (StPer variant). Line 1067. Same approach.

### Target 3: Audit `assume_eq_clone_workaround` warnings

The 151 remaining warnings are all `assume_eq_clone_workaround`. Run:
```bash
scripts/holes.sh src/ 2>&1 | grep assume_eq_clone_workaround | wc -l
```

These should all be inside `PartialEq::eq` or `Clone::clone` bodies. Spot
check 5-10 of them to confirm they follow the standard pattern from
`src/standards/partial_eq_eq_clone_standard.rs`. If any are NOT in eq/clone
bodies (i.e., they're in algorithmic code), report them — those would be
real bugs.

## Validation

Run `scripts/validate.sh`, `scripts/rtt.sh`, `scripts/ptt.sh` sequentially.
Write report to `plans/agent3-round61-report.md`. Push to `agent3/ready`.
