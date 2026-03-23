# Agent 3 — Round 61

You are Agent 3 working in `~/projects/APAS-VERUS-agent3`.

## Baseline

- Main: 4496 verified, 0 errors, 12 holes, 2610 RTT, 147 PTT.
- Your worktree: rebase onto main before starting (`git fetch origin && git rebase origin/main`).

## Targets

### Target 1: Chap43 OrderedSetStPer.rs `select` — 1 hole (line 1067)

Same BST rank hole you identified in R59. The blocker: BST ordering
invariant → sorted inorder traversal bridge.

Your R59 report said: need `lemma_bst_wf_implies_inorder_sorted` deriving
`spec_seq_sorted(spec_inorder_values(root))` from the BST invariant.

Approach:
1. Read `src/Chap41/AVLTreeSetStEph.rs` — BST wf predicate.
2. Read `src/Chap37/AVLTreeSeqStPer.rs` — the StPer variant may have
   different helper lemmas available.
3. Write the recursive proof locally in OrderedSetStPer.rs (standalone).
4. Leaf: trivial. Interior: left sorted (IH) + right sorted (IH) +
   max(left) < key < min(right) from BST → concatenation is sorted.
5. Use to close the assume.

Agent 1 is working on the StEph version independently. You own StPer.

### Target 2: Chap43 OrderedSetStEph.rs `select` — 1 hole (line 1146)

If you finish StPer's select, attempt StEph too. The proof structure
should be nearly identical but using StEph-specific helper functions.

### Target 3: Audit eq_clone_workaround warnings

151 `assume_eq_clone_workaround` warnings remain. Spot-check 5-10:
confirm they're all inside `PartialEq::eq` or `Clone::clone` bodies per
`src/standards/partial_eq_eq_clone_standard.rs`. Report any that are NOT
in eq/clone bodies — those would be bugs.

## Validation

Run `scripts/validate.sh`, `scripts/rtt.sh`, `scripts/ptt.sh` sequentially.
Write report to `plans/agent3-round61-report.md`. Push to `agent3/ready`.
