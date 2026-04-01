# R135 Agent 4 — Audit: BSTParaMtEph join_mid Rebalancing

## Summary

The three DIFFERS annotations on `join_mid`, `join_pair`, and `join_m` are **correct
observations, not bugs**. The parametric BST in Chapter 38 intentionally defines only the
*interface* — rebalancing is supplied by concrete implementations (Chapter 39: Treaps).
The O(1) `join_mid` is the simplest valid implementation of the parametric interface.
No code changes needed.

## Detailed Findings

### 1. join_mid (MtEph line 505, StEph line 536) — O(1), no rebalancing

Both StEph and MtEph have identical implementations: wrap `Node(left, key, right)` into a
new `ParamBST` with `new_param_bst(Some(Box::new(NodeInner { key, size, left, right })), ...)`.
No rotations, no priority checks, no height updates.

**This is correct for Chapter 38.** The textbook says (Ch38 p.266):

> "joinMid is the inverse of expose. [...] However, it might have to rebalance the tree to
> maintain the invariants of the balancing scheme. [...] The particular rotations required
> will depend on the balancing scheme."

And (Ch38 p.266):

> "In the next chapter we will discuss how to implement this interface with a particular
> balancing scheme called Treaps."

Chapter 38 defines the parametric *framework*. The `join_mid` function is a *hook* that
concrete implementations override. Without a balancing scheme, it's just a node constructor.

**Confirmation:** Chapter 39's `BSTParaTreapMtEph.rs` line 2046 implements `join_mid` by
calling `join_with_priority(left, key, priority, right)`, which performs actual rotations
based on treap priorities.

**The APAS cost spec CS 38.11** (`join: O(lg(|t1|+|t2|))`) describes costs *when a balanced
implementation is plugged in*. The parametric impl itself doesn't meet these bounds — that's
the expected tradeoff of the unspecialized base.

### 2. join_pair (MtEph line 343/645) — delegates to union_inner

| # | Chap | File | Function | Behavior |
|---|------|------|----------|----------|
| 1 | 38 | BSTParaStEph.rs | join_pair (line 883) | Algorithm 38.4: find min of right, split, recursive join. O(n). Requires `self < other` ordering. |
| 2 | 38 | BSTParaMtEph.rs | join_pair (line 645) | Delegates to `union_inner`. O(m lg(n/m)). No ordering requirement. |
| 3 | 38 | BSTParaMtEph.rs | join_pair_inner (line 652) | Algorithm 38.4 variant: exposes right, recursive merge. O(n). Requires `self < other` ordering. |

**The MtEph `join_pair` trait signature lacks the ordering precondition** that StEph has.
StEph's trait requires `forall|s, o| self@.contains(s@) && other@.contains(o@) ==> s < o`,
while MtEph's only requires `self@.len() + other@.len() <= usize::MAX`. Without the ordering
guarantee, `join_pair` cannot use Algorithm 38.4 — so it falls back to `union_inner`, which
works for arbitrary (not necessarily ordered) trees.

MtEph compensates by providing `join_pair_inner` (line 352/652), which *does* have the
ordering precondition and implements Algorithm 38.4. It's used by `delete` (line 600) where
the ordering is known from `split`.

**Is the missing ordering precondition on MtEph's `join_pair` a bug?**
It's a design choice. The MtEph trait makes `join_pair` usable in more contexts (callers
don't need to prove ordering), at the cost of worse asymptotics. The APAS `joinPair`
(Algorithm 38.4) does assume ordering (`T1 < T2`). Adding the ordering precondition to
MtEph's `join_pair` would allow using `join_pair_inner` directly, matching APAS. But this
would break any caller that relies on the weaker contract.

### 3. join_m (MtEph line 781, StEph line 576) — O(1), delegates to join_mid

Both files: `Self::join_mid(Exposed::Node(left, key, right))`. This is exactly APAS
Algorithm 38.2: `joinM(L, k, R) = joinMid(Node(L, k, R))`. The O(1) cost is a direct
consequence of `join_mid` being O(1). When a balanced `join_mid` is provided (Treaps),
`join_m` automatically becomes O(lg n).

### Are the three DIFFERS related?

Yes. They share a single root cause: **Chapter 38's parametric BST is an unbalanced
framework without a plugged-in balancing scheme.**

| # | Chap | File | Function | Root Cause |
|---|------|------|----------|------------|
| 1 | 38 | BSTParaMtEph.rs | join_mid (~505) | No balancing scheme: O(1) wrapper |
| 2 | 38 | BSTParaMtEph.rs | join_pair (~645) | Consequence of #1 + missing ordering precondition |
| 3 | 38 | BSTParaMtEph.rs | join_m (~781) | Direct consequence of #1 |

### Callers of join_mid

| # | Chap | File | Caller | How it uses join_mid |
|---|------|------|--------|---------------------|
| 1 | 38 | BSTParaMtEph.rs | join_m (line 783) | Wraps into Exposed::Node |
| 2 | 38 | BSTParaMtEph.rs | join_pair_inner (line 718) | Rebuilds after recursive merge |
| 3 | 38 | BSTParaMtEph.rs | split_inner (lines 1022, 1072) | Rebuilds subtree after split |
| 4 | 38 | BSTParaMtEph.rs | union_inner (line 1306) | Joins left/right union results with pivot |
| 5 | 38 | BSTParaMtEph.rs | intersect_inner | Same pattern as union |
| 6 | 38 | BSTParaMtEph.rs | difference_inner | Same pattern as union |
| 7 | 38 | BSTParaMtEph.rs | filter_parallel | Joins filtered subtrees |
| 8 | 38 | BSTParaMtEph.rs | insert (line 577) | Joins split halves with new key |
| 9 | 38 | BSTParaMtEph.rs | Clone for ParamBST (line 888) | Reconstructs after expose |

All callers use `join_mid` correctly — they maintain BST ordering invariants in the
preconditions. The trees produced are valid BSTs (correct ordering), just not balanced.
When the Chapter 39 Treap instantiation overrides `join_mid`, all these callers automatically
get balanced trees.

## Recommendation

**No code changes.** The DIFFERS annotations are accurate and serve as documentation that
the parametric base doesn't meet CS 38.11 cost bounds. The real balanced implementations
live in Chapter 39 (Treaps). The annotations correctly highlight where performance differs
from the textbook specification.

One minor design observation: MtEph's `join_pair` trait could be strengthened with the
ordering precondition (matching StEph and APAS Algorithm 38.4), which would allow using
the O(n) `join_pair_inner` instead of O(m lg(n/m)) `union_inner`. This would require
auditing callers to confirm they can provide the ordering proof. This is a potential future
improvement, not a current bug.
