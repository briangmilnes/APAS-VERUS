# Agent 1 — Round 61

You are Agent 1 working in `~/projects/APAS-VERUS-agent1`.

## Baseline

- Main: 4496 verified, 0 errors, 12 holes, 2610 RTT, 147 PTT.
- Your worktree: rebase onto main before starting (`git fetch origin && git rebase origin/main`).

## Targets

### Target 1: Chap47 ParaHashTableStEph.rs — 2 holes + 8 wf warnings

**Hole 1 (line 120):** `assume(c == *x)` — Clone bridge. Read the function.
If inside a `Clone::clone` body, it's the standard eq/clone workaround —
report it as a candidate for accept (do NOT convert yourself). If in
algorithmic code, use `clone_plus` or `clone_fn2` from
`crate::vstdplus::clone_plus`. Read
`src/standards/partial_eq_eq_clone_standard.rs` first.

**Hole 2 (line 342):** `external_body` — identify the function, read the
body, determine if algorithmic (prove it) or structural (report it).

**8 wf warnings:** Add real `requires table.spec_hashtable_wf()` and
`ensures result.spec_hashtable_wf()` to trait signatures for: `createTable`
(ensures), `insert` (requires), `lookup` (requires), `delete` (requires),
`metrics` (requires), `loadAndSize` (requires), `resize`
(requires + ensures). Read each function body first. Verify callers can
satisfy new requires.

### Target 2: Chap43 OrderedSetStEph.rs `select` — 1 hole (line 1146)

The BST rank hole. Need `lemma_bst_wf_implies_inorder_sorted` bridging
the AVL BST ordering invariant to sorted inorder traversal.

1. Read `src/Chap41/AVLTreeSetStEph.rs` — BST wf, `spec_inorder_values`.
2. Write recursive proof: leaf trivial, interior combines IH on left/right
   with BST bound (max(left) < key < min(right)).
3. Place lemma locally in OrderedSetStEph.rs (standalone rule).
4. Use to close the `select` assume.

## Validation

Run `scripts/validate.sh`, `scripts/rtt.sh`, `scripts/ptt.sh` sequentially.
Write report to `plans/agent1-round61-report.md`. Push to `agent1/ready`.
