# Agent 1 — Round 60

You are Agent 1 working in `~/projects/APAS-VERUS-agent1`.

## Baseline

- Main: 4496 verified, 0 errors, 18 holes, 2610 RTT, 147 PTT.
- Your worktree: rebase onto main before starting (`git fetch origin && git rebase origin/main`).

## Targets

### Target 1: Chap47 ParaHashTableStEph.rs — 2 holes + 8 warnings

**Hole 1 (line 120):** `assume(c == *x)` — Clone bridge in `clone_entry`.
This is the same eq/clone workaround pattern used everywhere. Convert to
`accept()` if the clone is inside a `Clone::clone` body, or use
`clone_fn2`/`clone_plus` from vstdplus if it's in algorithmic code.
Read `src/standards/partial_eq_eq_clone_standard.rs` first.

**Hole 2 (line 342):** `external_body` — identify what function this is on,
read the body, determine if it's algorithmic logic (prove it) or structural
boundary (report it).

**Warnings (8):** Add real `requires self.spec_hashtable_wf()` and
`ensures result.spec_hashtable_wf()` to `createTable`, `insert`, `lookup`,
`delete`, `metrics`, `loadAndSize`, `resize`. These are the wf propagation
pattern — read the function bodies to confirm the right predicates. Do NOT
add `requires true` or tautologies.

### Target 2: Chap43 OrderedSetStEph.rs + OrderedSetStPer.rs — 2 holes

These are the `select` rank holes that Agent 3 left open. The blocker is:
need `lemma_bst_wf_implies_inorder_sorted` — a recursive proof that the BST
ordering invariant in `spec_avltreeseqsteph_wf` implies
`spec_seq_sorted(spec_inorder_values(root))`.

This is a real proof challenge. The approach:
1. Read `src/Chap41/AVLTreeSetStEph.rs` — find the BST wf predicate and
   `spec_inorder_values`.
2. Write a recursive proof lemma `lemma_bst_wf_implies_inorder_sorted` in
   Chap41 (or locally in Chap43 if standalone rules require it).
3. The induction follows the tree structure: leaf case trivial, interior
   case combines left-sorted + right-sorted + root bounds.
4. Use the lemma in OrderedSetStEph `select` and OrderedSetStPer `select`
   to close the assume.

If you can't prove the lemma after genuine effort, report what you tried
and where you got stuck. Do not add assume/accept.

## Validation

Run `scripts/validate.sh`, `scripts/rtt.sh`, `scripts/ptt.sh` sequentially.
Fix trigger warnings in your files. Write report to
`plans/agent1-round60-report.md`. Push to `agent1/ready`.
