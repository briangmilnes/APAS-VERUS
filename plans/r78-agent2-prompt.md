# R78 Agent 2 — BSTSplayMtEph clone + cascade + height (Chap37, 5 holes)

## Objective

Prove or narrow 5 holes in BSTSplayMtEph.rs. Root cause is recursive `clone` on Node<T>.

## Baseline

- 4898 verified, 0 errors, 0 warnings
- BSTSplayMtEph.rs: 5 holes

## Holes

| # | Chap | File | Line | Function | Type | Blocked by |
|---|------|------|------|----------|------|------------|
| 1 | 37 | BSTSplayMtEph.rs | 1803 | clone | external_body | ROOT |
| 2 | 37 | BSTSplayMtEph.rs | 1455 | build_balanced | external_body | clone |
| 3 | 37 | BSTSplayMtEph.rs | 1481 | filter_parallel | external_body | clone |
| 4 | 37 | BSTSplayMtEph.rs | 1514 | reduce_parallel | external_body | clone |
| 5 | 37 | BSTSplayMtEph.rs | 1731 | height (Mt) | assume | — |

## Strategy

### clone (#1 — ROOT)

Verus has trouble with recursive Clone on `Node<T>` with `Box<Node<T>>` children. This
has been attempted in R75 and R76 without success.

New approaches to try:
- **`strictly_cloned` broadcasts**: Agent 4 R76 added `axiom_strictly_cloned_implies_eq`
  to `vstdplus/feq.rs`. Check if these help prove `clone()@ == self@` for recursive nodes.
- **Manual recursive clone**: Write a `fn clone_link(link: &Link<T>) -> Link<T>` with
  `decreases` that manually traverses and clones each node. This avoids Verus's derive
  Clone limitation. See `src/Chap37/AVLTreeSeqStEph.rs:clone_link` for the pattern.
- **If clone stays external_body**: narrow it to just the part Verus can't handle.

### build_balanced, filter_parallel, reduce_parallel (#2-4)

All blocked by clone. They clone nodes during tree construction or parallel traversal.
If clone is proved, these may cascade. If not, check if they can be restructured to
avoid cloning (e.g., move semantics, or narrowing external_body to just the clone call).

### height (#5)

`assume(link_height(*data) < usize::MAX)`. Agent 4 R77 proved the RB variant using
`lemma_height_le_size`. The Splay variant is harder because Splay trees use a `cached_size`
field that may diverge from recursive size after splay rotations.

Check if `lemma_height_le_size` exists for Splay or can be adapted. The lock predicate
should bound size ≤ usize::MAX, and height ≤ size.

## Key resources

- `src/Chap37/BSTSplayMtEph.rs` — Read fully
- `src/Chap37/AVLTreeSeqStEph.rs` — `clone_link` pattern (manual recursive clone)
- `src/Chap37/BSTRBMtEph.rs` — Agent 4's height fix pattern
- `src/vstdplus/feq.rs` — `strictly_cloned` broadcasts

## Validation

Run `scripts/validate.sh`, then `scripts/rtt.sh`, then `scripts/ptt.sh`. Push to `agent2/ready`.

## Report

Write `plans/agent2-round78-report.md` with holes before/after (table with Chap column).
