# Agent 1 — Round 61

You are Agent 1 working in `~/projects/APAS-VERUS-agent1`.

## Baseline

- Main: 4496 verified, 0 errors, 18 holes, 2610 RTT, 147 PTT.
- Warnings: 159 (8 real fn_missing_wf in Chap47 ParaHashTable).
- Your worktree: rebase onto main before starting (`git fetch origin && git rebase origin/main`).

## Targets

### Target 1: Chap47 ParaHashTableStEph.rs — 2 holes + 8 wf warnings

**Hole 1 (line 120):** `assume(c == *x)` — Clone bridge in `clone_entry`.
Read the function. If this is inside a `Clone::clone` body, it's the
standard eq/clone workaround — convert to `accept()` only with user
approval (do NOT convert on your own; report it as a candidate for accept).
If it's in algorithmic code, use `clone_plus` or `clone_fn2` from
`crate::vstdplus::clone_plus`. Read
`src/standards/partial_eq_eq_clone_standard.rs` first.

**Hole 2 (line 342):** `external_body` — identify what function this is,
read the body, determine if it's algorithmic logic (prove it) or structural
boundary (report it).

**Warnings (8):** Add real `requires table.spec_hashtable_wf()` and
`ensures result.spec_hashtable_wf()` to the trait signatures for:
`createTable` (ensures), `insert` (requires), `lookup` (requires),
`delete` (requires), `metrics` (requires), `loadAndSize` (requires),
`resize` (requires + ensures).

Read each function body first — confirm that (a) the function uses
`self`/`table` in ways that assume wf, and (b) the output maintains wf.
Then add the spec to both trait and impl. Verify that callers can satisfy
the new requires — check RTT and any call sites.

### Target 2: Chap43 OrderedSetStEph.rs + OrderedSetStPer.rs — 2 holes

The `select` rank holes from R59. Blocker: need
`lemma_bst_wf_implies_inorder_sorted`.

Approach:
1. Read `src/Chap41/AVLTreeSetStEph.rs` — find the BST wf predicate,
   `spec_inorder_values`, and any existing sortedness specs.
2. The BST invariant says: for every interior node, all left descendants
   < key < all right descendants. This implies inorder traversal is sorted.
3. Write a recursive proof lemma. The induction follows the tree structure:
   - Leaf: trivially sorted (single element or empty).
   - Interior: left inorder is sorted (IH), right inorder is sorted (IH),
     max(left) < key < min(right) from BST invariant, so
     left ++ [key] ++ right is sorted.
4. Place the lemma in Chap43 (standalone rule) or Chap41 if it's general
   enough to share.
5. Use the lemma in `select` to close the assume.

This is the hardest target. If you can't prove it after genuine effort,
report what you tried.

## Validation

Run `scripts/validate.sh`, `scripts/rtt.sh`, `scripts/ptt.sh` sequentially.
Write report to `plans/agent1-round61-report.md`. Push to `agent1/ready`.
