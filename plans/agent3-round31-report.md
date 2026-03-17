# Agent 3 Round 31 Report

## Assignment

1. Chap39/BSTTreapMtEph.rs: Fix 5 `requires_true` warnings + prove 6 assumes
2. Chap41: Fix 6 `fn_missing_requires` in AVLTreeSetMtEph.rs (4) and AVLTreeSetMtPer.rs (2)

## Results

### Verification

- validate: 4116 verified, 0 errors
- RTT: 2613 passed
- PTT: 147 passed

### Chap39/BSTTreapMtEph.rs Changes

| # | Chap | File | Function | Change | Result |
|---|------|------|----------|--------|--------|
| 1 | 39 | BSTTreapMtEph.rs | find_link | `requires true` → `requires Lnk::spec_bst_link(link)` | requires_true warning eliminated |
| 2 | 39 | BSTTreapMtEph.rs | min_link | `requires true` → `requires Lnk::spec_bst_link(link)` | requires_true warning eliminated |
| 3 | 39 | BSTTreapMtEph.rs | max_link | `requires true` → `requires Lnk::spec_bst_link(link)` | requires_true warning eliminated |
| 4 | 39 | BSTTreapMtEph.rs | clone_link | `requires true` preserved | No change (genuinely no precondition) |
| 5 | 39 | BSTTreapMtEph.rs | size_link | `requires true` preserved | No change (trivial field accessor) |
| 6 | 39 | BSTTreapMtEph.rs | size (trait impl) | Split assume: prove `self@.finite()` via `use_type_invariant` | Assume narrowed from `len + finite` to `len` only |

**Net requires_true reduction: 5 → 2 (3 eliminated with real BST preconditions)**

### Chap39 Assume Analysis

All 6 assumes (find, size, minimum, maximum, in_order, pre_order) bridge the same
architectural gap: `ghost_locked_root@` (manually maintained ghost set) vs actual tree
contents (locked behind RwLock).

**Why they're not provable:**

1. The `ghost_locked_root: Ghost<Set<V>>` is updated in `insert`/`delete` to shadow
   tree mutations, but this sync is unverified.
2. The RwLock predicate (`BSTTreapMtEphInv`) is a unit struct fixed at creation time.
   It enforces structural well-formedness but can't track the evolving ghost set.
3. At `acquire_read` time, readers know the tree is well-formed (BST, size-wf) but have
   no proof connecting tree contents to `ghost_locked_root@`.

**What would be needed:**

- A ghost token protocol where mutations produce certificates and readers consume them
- Or a richer RwLock variant that can track evolving ghost state
- Both are beyond vstd's current API

**What I tried:**
- Adding `spec_link_to_set` function to convert tree to set (useful but doesn't close gap)
- Enriching BSTTreapMtEphInv with ghost fields (fails: predicate is immutable after creation)
- Deriving View from tree (impossible: can't access locked value in spec)
- Using type_invariant (can't reference locked value)

**Partial success:** The `size()` assume was narrowed by proving `self@.finite()` via
`use_type_invariant`, reducing the assume from `result == self@.len() && self@.finite()`
to just `result == self@.len()`.

### Chap41 Analysis

The 6 fn_missing items are on **nested functions inside `external_body` methods**:

| # | Chap | File | Nested Function | Parent Method | Issue |
|---|------|------|-----------------|---------------|-------|
| 1 | 41 | AVLTreeSetMtEph.rs | parallel_filter | filter | fn_missing_requires + fn_missing_ensures |
| 2 | 41 | AVLTreeSetMtEph.rs | parallel_intersect | intersection | fn_missing_requires + fn_missing_ensures |
| 3 | 41 | AVLTreeSetMtPer.rs | parallel_sort | from_seq | fn_missing_requires + fn_missing_ensures |

**Why they're not fixable:**
- These are implementation-detail functions inside `#[verifier::external_body]` methods
- They use `ParaPair!` for parallelism, which can't be verified by Verus
- Adding Verus specs would not be verified (parent is external_body)
- Lifting them to module-level `external_body` would ADD holes (net regression)

These are veracity false positives on parallel helper code inside unverified blocks.

### Holes Before/After

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 39 | BSTTreapMtEph.rs | 6 assumes, 5 req_true warn | 6 assumes, 2 req_true warn | -3 warnings |
| 2 | 41 | AVLTreeSetMtEph.rs | 9 holes, 2 fn_missing | 9 holes, 2 fn_missing | 0 |
| 3 | 41 | AVLTreeSetMtPer.rs | 9 holes, 1 fn_missing | 9 holes, 1 fn_missing | 0 |

### Techniques Used

1. Real BST preconditions on search/min/max helpers (eliminates vacuous requires true)
2. Type invariant usage to partially prove postconditions (narrows assumes)
3. Architectural analysis of ghost set ↔ RwLock gap

### Remaining Work

**Chap39 assumes (6):** Require ghost token protocol or richer concurrency primitive.
This is a project-wide pattern affecting ALL Mt modules with RwLock + ghost set.

**Chap41 fn_missing (3 functions):** Nested parallel helpers inside external_body.
Would need full parallel algorithm verification to resolve.

**clone_link/size_link requires_true (2):** These functions genuinely have no
precondition. Need user `// veracity: no_requires` annotation.
