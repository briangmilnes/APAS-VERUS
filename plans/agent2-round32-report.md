# Agent 2 — Round 32 Report

## Summary

R32 targeted Chap39 BSTTreap: proving external_body holes in StEph and investigating
reader-predicate assumes in MtEph.

### Results

| # | Chap | File | Task | Result |
|---|------|------|------|--------|
| 1 | 39 | BSTTreapStEph.rs | Remove external_body from `find` | Done — biconditional proof |
| 2 | 39 | BSTTreapStEph.rs | Remove external_body from `insert_link` | Done — full proof with rotations |
| 3 | 39 | BSTTreapMtEph.rs | Prove 6 reader-predicate assumes | Architectural: cannot eliminate (see below) |

### Holes Before/After

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 39 | BSTTreapStEph.rs | 2 | 0 | -2 |
| 2 | 39 | BSTTreapMtEph.rs | 6 | 6 | 0 |
| 3 | — | total_order.rs | 3 | 6 | +3 (new String accepts for new methods) |

BSTTreapStEph.rs is now **fully clean** (0 holes, 16 clean proof functions).

### Verification

- 4147 verified, 0 errors
- 2613 RTTs pass
- 147 PTTs pass

### Techniques Used

1. **is_lt_antisymmetric** — New method on IsLtTransitive trait. Given `!a.is_lt(&b) && !b.is_lt(&a)`, concludes `a == b` (structural equality). Bridges the gap between exec comparison results and spec-level structural equality for generic types.

2. **is_lt_irreflexive** — New method on IsLtTransitive trait. Proves `!a.is_lt(&a)`. Essential for BST contradiction proofs: if a key is in a subtree, transitivity gives `a.is_lt(&a)`, which contradicts irreflexivity.

3. **Order-based branching** — Restructured find_link to use `*target < node.key` / `node.key < *target` / else (instead of `*target == node.key` first). The else branch uses is_lt_antisymmetric for structural equality. Avoids needing obeys_eq_spec or view injectivity.

4. **BST contradiction proofs** — For the reverse direction of find_link's biconditional (in-tree → found), each branch eliminates wrong subtrees via is_lt_transitive + is_lt_irreflexive. Explicit step-by-step assertions guide Verus through the reasoning.

5. **reveal_with_fuel** — Controls recursive unfolding of spec_contains_link. Fuel 2 for find_link, fuel 3 for insert_link (deeper structural changes from rotations).

### Changes Made

**vstdplus/total_order.rs** — Extended IsLtTransitive trait:
- Added `is_lt_irreflexive(a)` ensures `!a.is_lt(&a)`
- Added `is_lt_antisymmetric(a, b)` requires `!a.is_lt(&b) && !b.is_lt(&a)` ensures `a == b`
- Implemented for all 12 integer types (empty bodies) and String (accept)

**Chap39/BSTTreapStEph.rs** — 2 external_body holes removed:
- `find_link`: Removed external_body, biconditional proof with BST contradiction reasoning
- `find`: Removed external_body (delegates to proven find_link)
- `insert_link`: Removed external_body, full proof with rotation containment assertions
- Removed view injectivity from find/find_link/contains requires (no longer needed)

**Chap39/BSTTreapMtEph.rs** — Strengthened infrastructure, 0 assumes eliminated:
- Added `spec_set_of_link`: Maps Link<T> → Set<T::V> recursively
- Added `lemma_contains_implies_in_set`: spec_contains_link ⟹ spec_set_of_link.contains
- Added `lemma_set_of_link_finite`: spec_set_of_link is always finite
- Strengthened `find_link` to biconditional postcondition (was forward-only)
- Added `requires T::obeys_partial_cmp_spec()` to trait find/contains

### Architectural Analysis: MtEph Reader Assumes

The 6 reader-predicate assumes in BSTTreapMtEph.rs are **lock-boundary gaps** inherent to
the coarse RwLock pattern. They cannot be eliminated with current vstd infrastructure:

1. **RwLock has no spec-level contents accessor.** The locked value is opaque outside
   acquire/release. View must use an external ghost field (`ghost_locked_root`).

2. **The invariant can't bridge external and internal state.** `RwLockPredicate::inv` only
   sees the locked value, not `self.ghost_locked_root@`. There is no mechanism to enforce
   `spec_set_of_link(&locked_link) == ghost_locked_root@`.

3. **Writers update ghost independently.** `insert` sets `ghost_locked_root = old_set.insert(value@)`
   without proving it matches `spec_set_of_link(&released_link)`. The correspondence is
   maintained by construction, not by proof.

4. **Universal pattern.** Every Mt module in the codebase (Chap37-43, ~100+ assumes) has
   the same gap. The toplevel_coarse_rwlocks standard explicitly calls these "reader accepts."

5. **Failed experiments.** `src/experiments/struct_rwlock_with_fn_specs.rs` attempted to
   bridge this gap and failed. `rwlock_tsm.rs` uses tokenized state machines but is not
   integrated into Mt modules.

**What would be needed:** A vstd extension providing either (a) a spec-level contents accessor
for RwLock, or (b) tracked ghost tokens that synchronize external ghost state with the locked
value across acquire/release boundaries.
