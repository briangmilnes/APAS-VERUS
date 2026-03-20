# R49 Agent 3 Prompt — Chap39 Treap Parallel (7 holes)

## REQUIRED READING (before writing ANY code)

1. `src/standards/arc_usage_standard.rs`
2. `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs`
3. `src/standards/partial_eq_eq_clone_standard.rs`
4. `src/standards/using_closures_standard.rs`
5. `src/Chap39/BSTParaTreapMtEph.rs` (full file)
6. `src/Chap39/BSTTreapStEph.rs` (read for the sequential pattern)
7. `src/vstdplus/arc_rwlock.rs`

## Target Files

| # | Chap | File | Holes | Type |
|---|------|------|-------|------|
| 1 | 39 | BSTParaTreapMtEph.rs | 7 | 1 assume + 5 external_body + 1 assume(false) |

## Hole Inventory

| # | Line | Function | Type | Role |
|---|------|----------|------|------|
| 1 | 158 | `expose_internal` | external_body | ROOT CAUSE — unblocks 4 below |
| 2 | 759 | `find` | assume | Loop invariant too weak |
| 3 | 275 | `split_inner` | external_body | Downstream of expose_internal |
| 4 | 355 | `intersect_inner` | external_body | Downstream of expose_internal |
| 5 | 374 | `difference_inner` | external_body | Downstream of expose_internal |
| 6 | 445 | `filter_parallel` | external_body | Downstream of expose_internal |
| 7 | 445+ | `reduce_inner` | external_body | Downstream of expose_internal |

## Strategy

### Priority 1: expose_internal (root cause)

Same pattern as Chap38/BSTParaMtEph.rs. The function acquires a read lock on
`Arc<RwLock<Option<NodeInner<T>>>>`, reads the tree node, clones left/key/priority/right,
releases the lock, and returns `Option<(ParamTreap<T>, T, i64, ParamTreap<T>)>`.

The core challenge: relating the RwLock contents to `ghost_locked_root`. Study the
RwLockPredicate and determine if it can be strengthened to carry the abstract set relationship.

**Key difference from Chap38**: The treap has a `priority: i64` field in each node.
The expose_internal returns the priority along with left/key/right. The ensures need to
capture that the key, left, and right decompose the tree's abstract set, same as Chap38.

**Approach:**

1. Read `toplevel_coarse_rwlocks_for_mt_modules.rs` — it shows how to use `pub ghost` fields
   in the RwLockPredicate struct to link physical contents to abstract state.
2. Study how `src/Chap41/AVLTreeSetMtEph.rs` handles the RwLock-to-ghost-set connection.
   That file uses RWLOCK_GHOST structural assumes — understand why and whether you can do better.
3. Strengthen the RwLockPredicate to carry a ghost set that matches the tree contents.
4. Use the strengthened predicate in acquire_read to establish the ensures.

### Priority 2: find (assume in loop)

The `find` implementation walks the tree holding RwLock read handles. The assume at ~line 759
bridges the loop result to `self@.contains(key@)`. Strengthen the loop invariant to track
the current node's position in the tree. The key insight is that each step descends into
a subtree that is a subset of the original set.

### Priority 3: Downstream functions

Once expose_internal is proved:
- `split_inner`: Recursively splits via expose, then joins. Set algebra proof.
- `intersect_inner`: Uses split + parallel recursion. Set intersection proof.
- `difference_inner`: Uses split + parallel recursion. Set difference proof.
- `filter_parallel`: Uses expose + parallel filter on subtrees. Subset proof.
- `reduce_inner`: Parallel reduce via ParaPair on subtrees. Closure specs needed.

For reduce_inner and filter_parallel, read `using_closures_standard.rs` — you'll need
named closure variables with explicit requires/ensures for the join() calls.

### Also address: fn_missing_requires warnings

| # | Line | Function | Issue |
|---|------|----------|-------|
| 1 | 204 | `tree_priority` | Missing requires + ensures |
| 2 | 217 | `tree_size` | Missing requires + ensures |
| 3 | 305 | `join_pair_inner` | Missing requires |

Add real requires/ensures (not `requires true`). These functions read from the RwLock,
so they need at minimum the tree's well-formedness. Check what callers assume about
the return values.

## Validation

After changes:
```bash
scripts/validate.sh
scripts/rtt.sh
scripts/ptt.sh
```
Run sequentially, not in parallel.

## Report

Write `plans/agent3-round49-report.md` with holes before/after per file, techniques used,
remaining holes with blockers. Include Chap column in all tables.
