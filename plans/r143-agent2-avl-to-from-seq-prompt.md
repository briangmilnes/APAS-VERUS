# R142 Agent 2 — Parallelize AVL to_seq and from_seq (Chap41). AFK.

## Setup

Read ALL files in `src/standards/` before starting.
Read `src/Chap41/AVLTreeSetMtEph.rs` — to_seq and from_seq implementations.
Read `src/Chap41/AVLTreeSetMtPer.rs` — same two functions.
Read `src/Chap38/BSTParaMtEph.rs` — the underlying ParamBST with expose/split.
Read `prompts/Chap41.txt` — APAS cost specs for ordered sets.

Report file: `plans/r142-agent2-avl-to-from-seq-report.md`

## Problem

4 DIFFERS in Chap41:

```
AVLTreeSetMtEph.rs: to_seq  — sequential in-order traversal
AVLTreeSetMtEph.rs: from_seq — sequential loop of inserts
AVLTreeSetMtPer.rs: to_seq  — sequential in-order traversal
AVLTreeSetMtPer.rs: from_seq — sequential loop of inserts
```

APAS says to_seq is O(n) work, O(n) span (in-order traversal is inherently
sequential for the output ordering). But from_seq on a SORTED input can be
O(n) work, O(lg n) span via parallel build.

## to_seq

Read what APAS says about to_seq's cost. If APAS gives O(n) span (sequential),
then our implementation already matches and the DIFFERS annotation is wrong.
Update the annotation from DIFFERS to matches APAS.

If APAS gives O(lg n) span, then we need D&C: split the BST at root, recurse
on left and right in parallel via join(), concatenate results. This gives
O(n) work, O(lg n) span with a balanced tree.

Check the APAS cost spec before implementing.

## from_seq

APAS from_seq on a sorted sequence: build a balanced BST by splitting the
sequence at the midpoint, making the middle element the root, and recursing
on both halves in parallel.

```
fn from_seq(s: &ArraySeqMtEphSliceS<T>) -> Self {
    if s.is_empty() { return empty(); }
    if s.is_singleton() { return singleton(s.nth(0)); }
    let mid = s.length() / 2;
    let left_seq = s.slice(0, mid);
    let right_seq = s.slice(mid + 1, s.length() - mid - 1);
    let root = s.nth(mid);
    let (left_tree, right_tree) = join(
        || from_seq(&left_seq),
        || from_seq(&right_seq),
    );
    // combine left_tree, root, right_tree into a BST
}
```

This requires the input to be sorted. Check from_seq's requires — if it
requires sorted input, this approach works. If not, sort first (O(n lg n)).

The BST combine step uses ParamBST's join_mid or direct node construction.

## Both files

Make the same fix in both MtEph and MtPer. They are standalone — duplicate
the implementation per the standalone rule.

## Validation

Run `scripts/validate.sh isolate Chap41`. Then `scripts/rtt.sh`.

## Rules

- Do NOT add assumes, accepts, or external_body.
- Named closures with explicit ensures for join() calls.
- Use clone_fn/clone_fn2 for closure cloning.
- Update alg analysis annotations from DIFFERS to matches APAS.

## When done

RCP.
