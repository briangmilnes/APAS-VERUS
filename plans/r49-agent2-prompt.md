# R49 Agent 2 Prompt — Chap38 BST Parallel (7 holes)

## REQUIRED READING (before writing ANY code)

1. `src/standards/arc_usage_standard.rs`
2. `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs`
3. `src/standards/partial_eq_eq_clone_standard.rs`
4. `src/standards/using_closures_standard.rs`
5. `src/Chap38/BSTParaMtEph.rs` (full file)
6. `src/Chap38/BSTParaStEph.rs` (full file)
7. `src/vstdplus/arc_rwlock.rs`

## Target Files

| # | Chap | File | Holes | Type |
|---|------|------|-------|------|
| 1 | 38 | BSTParaMtEph.rs | 6 | 1 assume + 5 external_body |
| 2 | 38 | BSTParaStEph.rs | 1 | 1 assume (clone_elem) |

## Hole Inventory

### BSTParaMtEph.rs (6 holes)

| # | Line | Function | Type | Role |
|---|------|----------|------|------|
| 1 | 451 | `expose_internal` | external_body | ROOT CAUSE — unblocks 4 below |
| 2 | 353 | `find` | assume | Loop invariant too weak |
| 3 | 502 | `split_inner` | external_body | Downstream of expose_internal |
| 4 | 601 | `intersect_inner` | external_body | Downstream of expose_internal |
| 5 | 634 | `difference_inner` | external_body | Downstream of expose_internal |
| 6 | 667 | `filter_inner` | external_body | Downstream of expose_internal |

### BSTParaStEph.rs (1 hole)

| # | Line | Function | Type | Notes |
|---|------|----------|------|-------|
| 1 | 152 | `clone_elem` | assume | Clone bridge pattern |

## Strategy

### Priority 1: expose_internal (root cause)

This is the key function. It acquires a read lock on `Arc<RwLock<Option<Box<NodeInner<T>>>>>`,
reads the tree node, clones left/key/right, releases the lock, and returns an `Exposed<T>`.

The challenge: connecting what the RwLock contains to the ghost set in `ghost_locked_root`.
Currently `BSTParaMtEphInv` only checks `size >= 1` for non-empty nodes — it doesn't
relate the physical tree to the abstract set.

**Approach options:**

1. **Strengthen `BSTParaMtEphInv`**: Make the predicate parameterized with a ghost set and
   assert the tree contents match. This would require updating `new_param_bst` and all
   construction sites. Read `toplevel_coarse_rwlocks_for_mt_modules.rs` for the standard
   pattern — it shows how to use `pub ghost` fields in the RwLockPredicate struct to carry
   construction-time context.

2. **Ghost field approach**: The `ghost_locked_root` field is set at construction and never
   changes. If the RwLock predicate can be made to assert that the locked value "corresponds"
   to some set, and that set is the ghost_locked_root, then acquire_read gives you the
   connection you need.

3. **Read the existing Mt files that work** (e.g., `src/Chap41/AVLTreeSetMtEph.rs`) to see
   how they handle the same pattern. Those files have RWLOCK_GHOST structural assumes — study
   their approach.

Once expose_internal is proved, the 4 downstream functions (split_inner, intersect_inner,
difference_inner, filter_inner) call expose_internal and then do recursive work. They should
become provable once expose_internal has proper ensures.

### Priority 2: find (assume in loop)

The `find` function searches the tree with `invariant true`. Strengthen the loop invariant
to track the current node's relationship to the tree's contents. The assume at line 353
bridges the loop result to the ensures — replace it with a real proof from the loop invariant.

### Priority 3: split_inner, intersect_inner, difference_inner, filter_inner

These all call expose_internal then recursively process subtrees. Once expose_internal
provides the decomposition ensures, prove these by structural induction on the exposed node.
Key set properties needed: union/insert/remove commutativity, disjointness propagation.
Use `vstd::set::group_set_axioms` and `vstd::set_lib::group_set_properties`.

### Priority 4: clone_elem (BSTParaStEph.rs)

Standard clone bridge pattern. Low priority — this is likely irreducible without Verus
changes to Clone verification. But check if there's a way to prove it for the specific
type used.

## Validation

After changes:
```bash
scripts/validate.sh
scripts/rtt.sh
scripts/ptt.sh
```
Run sequentially, not in parallel.

## Report

Write `plans/agent2-round49-report.md` with holes before/after per file, techniques used,
remaining holes with blockers. Include Chap column in all tables.
