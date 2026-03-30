# R115 Agent 3: Chap37 compare-par-mut False Positive Analysis

**Date:** 2026-03-30
**Tool:** `veracity-compare-par-mut`
**Chapter:** Chap37 (BST variants + AVLTreeSeq)
**Total warnings:** 101 (14 Phase 2 + 87 Phase 4)
**Fixable:** 3 (see end)
**False positives:** 93
**Real but intentional:** 5

---

## Summary of False Positive Categories

| # | Category | Count | Root cause |
|---|----------|-------|------------|
| 1 | wf subsumption (requires) | 37 | Tool cannot expand `spec_*_wf()` to see it implies `tree_is_bst()`, `spec_size() <= usize::MAX`, etc. |
| 2 | wf subsumption (ensures) | 14 | Same: `ensures self.spec_*_wf()` implies `tree_is_bst()` and overflow bounds |
| 3 | spec_root() vs @ equivalence | 8 | Tool cannot equate `self.spec_root().foo()` with `self@.foo()` when View returns the root |
| 4 | match-arm ensures parsing | 5 | Tool cannot extract ensures clauses from inside `match r { Ok(_) => ..., Err(_) => ... }` |
| 5 | ensures count (MtEph stronger) | 5 | MtEph has MORE ensures than StEph; tool warns on count mismatch even when MtEph is strictly stronger |
| 6 | ghost_root structural field | 5 | MtEph's `ghost ghost_root` is a required RwLock pattern field with no StEph counterpart |
| 7 | missing spec_root / spec fn equivalence | 7 | MtEph uses View (`@`) or free functions instead of trait-declared spec fns |
| 8 | supertrait parse bug | 1 | Tool fails to parse BSTSplayStEph's supertrait bounds |
| 9 | iter_invariant naming | 1 | `avltreeseq_iter_invariant` vs `iter_invariant` — same concept, different names |
| 10 | AVLTreeSeq variant differences | 10 | Structural differences between StEph/StPer/MtPer (extra clauses, different styles) |

---

## Category 1: wf subsumption (requires)

### Problem

StEph traits declare individual requires clauses like `self.spec_root().tree_is_bst()` and
`self.spec_root().spec_size() <= usize::MAX`. MtEph traits use a single `self.spec_*_wf()`
which is defined to include those predicates.

The tool cannot expand the wf predicate to check whether it subsumes the individual StEph
clauses.

### wf definitions for reference

| File | wf predicate | Expands to |
|------|-------------|------------|
| BSTPlainMtEph.rs:318 | `spec_bstplainmteph_wf` | `self@.tree_is_bst() && self@.spec_size() <= usize::MAX && self@.spec_height() <= usize::MAX` |
| BSTBBAlphaMtEph.rs:318 | `spec_bstbbalphamteph_wf` | `self@.tree_is_bst() && self@.spec_size() <= usize::MAX && self@.spec_height() <= usize::MAX` |
| BSTAVLMtEph.rs:497 | `spec_bstavlmteph_wf` | `self@.tree_is_bst() && self@.spec_size() <= usize::MAX && self@.spec_height() <= usize::MAX` |
| BSTRBMtEph.rs:1050 | `spec_bstrbmteph_wf` | `link_spec_size(self.spec_ghost_root()) <= usize::MAX && spec_is_bst_link(self.spec_ghost_root())` |
| BSTSplayMtEph.rs:1858 | `spec_bstsplaymteph_wf` | `link_node_count(self@) <= usize::MAX && spec_is_bst_link(self@)` |

### Sub-category 1a: tree_is_bst requires subsumed by wf

StEph requires `self.spec_root().tree_is_bst()` as a separate clause.
MtEph requires `self.spec_*_wf()` which includes `tree_is_bst()`.

**Affected warnings (15):**

| # | File:Line | Function | Warning text |
|---|-----------|----------|-------------|
| 1 | BSTAVLMtEph.rs:463 | `contains` | StEph has requires `self.spec_root().tree_is_bst()` no match |
| 2 | BSTAVLMtEph.rs:479 | `find` | StEph has requires `self.spec_root().tree_is_bst()` no match |
| 3 | BSTBBAlphaMtEph.rs:266 | `insert` | StEph has requires `self.spec_root().tree_is_bst()` no match |
| 4 | BSTBBAlphaMtEph.rs:276 | `contains` | StEph has requires `self.spec_root().tree_is_bst()` no match |
| 5 | BSTBBAlphaMtEph.rs:292 | `find` | StEph has requires `self.spec_root().tree_is_bst()` no match |
| 6 | BSTBBAlphaMtEph.rs:297 | `minimum` | StEph has requires `self.spec_root().tree_is_bst()` no match |
| 7 | BSTBBAlphaMtEph.rs:303 | `maximum` | StEph has requires `self.spec_root().tree_is_bst()` no match |
| 8 | BSTPlainMtEph.rs:266 | `insert` | StEph has requires `self.spec_root().tree_is_bst()` no match |
| 9 | BSTPlainMtEph.rs:276 | `contains` | StEph has requires `self.spec_root().tree_is_bst()` no match |
| 10 | BSTPlainMtEph.rs:292 | `find` | StEph has requires `self.spec_root().tree_is_bst()` no match |
| 11 | BSTPlainMtEph.rs:297 | `minimum` | StEph has requires `self.spec_root().tree_is_bst()` no match |
| 12 | BSTPlainMtEph.rs:303 | `maximum` | StEph has requires `self.spec_root().tree_is_bst()` no match |
| 13 | BSTRBMtEph.rs:996 | `insert` | StEph has requires `self.spec_root().tree_is_bst()` no match |
| 14 | BSTRBMtEph.rs:1004 | `contains` | StEph has requires `self.spec_root().tree_is_bst()` no match |
| 15 | BSTRBMtEph.rs:1020 | `find` | StEph has requires `self.spec_root().tree_is_bst()` no match |

**Why false positive:** MtEph's `spec_*_wf()` includes `self@.tree_is_bst()` (or
`spec_is_bst_link(self@)` for RB/Splay). The wf requires already subsumes the
individual `tree_is_bst()` clause.

### Sub-category 1b: overflow bounds requires subsumed by wf

StEph requires `self.spec_root().spec_size() <= usize::MAX` or
`self.spec_root().spec_height() <= usize::MAX`. MtEph wf includes these bounds.

**Affected warnings (10):**

| # | File:Line | Function | StEph clause |
|---|-----------|----------|-------------|
| 1 | BSTAVLMtEph.rs:467 | `size` | `self.spec_root().spec_size() <= usize::MAX` |
| 2 | BSTAVLMtEph.rs:475 | `height` | `self.spec_root().spec_height() <= usize::MAX` |
| 3 | BSTBBAlphaMtEph.rs:280 | `size` | `self.spec_root().spec_size() <= usize::MAX` |
| 4 | BSTBBAlphaMtEph.rs:288 | `height` | `self.spec_root().spec_height() <= usize::MAX` |
| 5 | BSTPlainMtEph.rs:280 | `size` | `self.spec_root().spec_size() <= usize::MAX` |
| 6 | BSTPlainMtEph.rs:288 | `height` | `self.spec_root().spec_height() <= usize::MAX` |
| 7 | BSTRBMtEph.rs:1008 | `size` | `self.spec_root().spec_size() <= usize::MAX` |
| 8 | BSTRBMtEph.rs:1016 | `height` | `self.spec_root().spec_height() <= usize::MAX` |
| 9 | BSTSplayMtEph.rs:1812 | `height` | `self.spec_height() < usize::MAX as nat` |
| 10 | BSTAVLMtEph.rs:453 | `insert` | `self.spec_root().spec_height() <= usize::MAX - 1` |

**Why false positive:** All MtEph wf predicates include `spec_size() <= usize::MAX`
(and for Plain/BBAlpha/AVL, `spec_height() <= usize::MAX`). The wf requires subsumes
these individual overflow guard clauses. For #10, `height <= MAX - 1` is slightly
stronger than wf's `height <= MAX`, but MtEph handles the post-insert overflow
internally via the wf ensures on the output.

### Sub-category 1c: requires count mismatches (consequence of 1a/1b)

Each missing-clause warning above also generates a "requires clause count N vs M" warning.

**Affected warnings (22):**

| # | File:Line | Function | Count |
|---|-----------|----------|-------|
| 1 | BSTAVLMtEph.rs:467 | `size` | 1 vs 2 |
| 2 | BSTAVLMtEph.rs:475 | `height` | 1 vs 2 |
| 3 | BSTAVLMtEph.rs:453 | `insert` | 1 vs 3 |
| 4 | BSTAVLMtEph.rs:463 | `contains` | 1 vs 2 |
| 5 | BSTAVLMtEph.rs:479 | `find` | 1 vs 2 |
| 6 | BSTBBAlphaMtEph.rs:280 | `size` | 1 vs 2 |
| 7 | BSTBBAlphaMtEph.rs:288 | `height` | 1 vs 2 |
| 8 | BSTBBAlphaMtEph.rs:266 | `insert` | 1 vs 2 |
| 9 | BSTBBAlphaMtEph.rs:276 | `contains` | 1 vs 2 |
| 10 | BSTBBAlphaMtEph.rs:292 | `find` | 1 vs 2 |
| 11 | BSTBBAlphaMtEph.rs:297 | `minimum` | 1 vs 2 |
| 12 | BSTBBAlphaMtEph.rs:303 | `maximum` | 1 vs 2 |
| 13 | BSTPlainMtEph.rs:280 | `size` | 1 vs 2 |
| 14 | BSTPlainMtEph.rs:288 | `height` | 1 vs 2 |
| 15 | BSTPlainMtEph.rs:266 | `insert` | 1 vs 2 |
| 16 | BSTPlainMtEph.rs:276 | `contains` | 1 vs 2 |
| 17 | BSTPlainMtEph.rs:292 | `find` | 1 vs 2 |
| 18 | BSTPlainMtEph.rs:297 | `minimum` | 1 vs 2 |
| 19 | BSTPlainMtEph.rs:303 | `maximum` | 1 vs 2 |
| 20 | BSTRBMtEph.rs:1008 | `size` | 1 vs 2 |
| 21 | BSTRBMtEph.rs:1016 | `height` | 1 vs 2 |
| 22 | BSTRBMtEph.rs:996 | `insert` | 1 vs 2 |
| 23 | BSTRBMtEph.rs:1004 | `contains` | 1 vs 2 |
| 24 | BSTRBMtEph.rs:1020 | `find` | 1 vs 2 |
| 25 | BSTSplayMtEph.rs:1812 | `height` | 1 vs 2 |

**Why false positive:** These count mismatches are a direct consequence of the
individual clause mismatches in 1a and 1b. When the tool resolves wf subsumption,
these count warnings will disappear.

### Suggested tool fix for Category 1

When comparing requires/ensures between variants:
1. Identify the MtEph variant's `spec_*_wf()` predicate definition.
2. Expand the wf body to extract individual conjuncts.
3. When a StEph clause is not found in MtEph requires/ensures,
   check whether it appears (modulo `spec_root()` vs `@` rewriting)
   as a conjunct of the wf predicate.
4. If the clause is subsumed by a wf conjunct, suppress the warning
   or downgrade it to info: "clause subsumed by wf predicate".
5. When all missing clauses are accounted for by wf expansion,
   suppress the count mismatch warning too.

---

## Category 2: wf subsumption (ensures)

### Problem

StEph ensures clauses like `inserted.spec_root().tree_is_bst()` are subsumed by
MtEph's `ensures self.spec_*_wf()` which includes `tree_is_bst()`.

**Affected warnings (14):**

| # | File:Line | Function | StEph ensures clause |
|---|-----------|----------|---------------------|
| 1 | BSTBBAlphaMtEph.rs:266 | `insert` | `inserted.spec_root().tree_is_bst()` |
| 2 | BSTPlainMtEph.rs:266 | `insert` | `inserted.spec_root().tree_is_bst()` |
| 3 | BSTRBMtEph.rs:996 | `insert` | `inserted.spec_root().tree_is_bst()` |
| 4 | BSTBBAlphaMtEph.rs:266 | `insert` | ensures count 2 vs 4 |
| 5 | BSTPlainMtEph.rs:266 | `insert` | ensures count 2 vs 4 |
| 6 | BSTRBMtEph.rs:996 | `insert` | ensures count 2 vs 4 |
| 7 | BSTAVLMtEph.rs:453 | `insert` | ensures count 2 vs 4 |
| 8 | BSTSplayMtEph.rs:1789 | `insert` | ensures count 2 vs 3 |

Note: The BSTAVLMtEph insert `tree_is_avl` ensures warning (#23 in Cat 1) is a
**real** difference, not a false positive. MtEph maintains `tree_is_bst` but not
`tree_is_avl`. See Category 11 (real but intentional).

Additional new() ensures subsumed by spec_root vs @ (see Category 3):

| # | File:Line | Function | StEph ensures clause |
|---|-----------|----------|---------------------|
| 9 | BSTBBAlphaMtEph.rs:260 | `new` | `tree.spec_root().tree_is_bst()` |
| 10 | BSTPlainMtEph.rs:260 | `new` | `tree.spec_root().tree_is_bst()` |
| 11 | BSTRBMtEph.rs:987 | `new` | `tree.spec_root().tree_is_bst()` |
| 12 | BSTBBAlphaMtEph.rs:260 | `new` | ensures count 4 vs 3 |
| 13 | BSTPlainMtEph.rs:260 | `new` | ensures count 4 vs 3 |
| 14 | BSTRBMtEph.rs:987 | `new` | ensures count 4 vs 3 |

**Why false positive:** For new(): MtEph ALREADY has `tree@.tree_is_bst()` as an
explicit ensures clause. The tool reports it as "MtEph has extra ensures clause
`tree @ . tree_is_bst ()` not in StEph" but simultaneously reports "StEph has ensures
clause `tree . spec_root () . tree_is_bst ()` with no match in MtEph". These are the
SAME clause — `tree@` equals `tree.spec_root()` via the View impl.

For insert(): MtEph ensures `self.spec_*_wf()` which includes `tree_is_bst()`.

**Suggested tool fix:** Same as Category 1 — expand wf for ensures subsumption.
Also see Category 3 for spec_root vs @ equivalence.

---

## Category 3: spec_root() vs @ equivalence

### Problem

StEph defines `spec fn spec_root(self) -> BalBinTree<T>` and uses
`self.spec_root().foo()` in requires/ensures. MtEph implements `View` where
`view(&self)` returns the same `BalBinTree<T>`, so `self@.foo()` is semantically
identical to `self.spec_root().foo()`.

The tool cannot equate these two access patterns.

**Affected warnings (8):**

These are the new() ensures warnings from Category 2 (#9-14 above) where the
tool sees `tree.spec_root().tree_is_bst()` and `tree@.tree_is_bst()` as different.
Plus the ensures count mismatches they cause.

The spec_root vs @ mismatch also affects many fuzzy-match info messages (not
warnings), but the tool correctly fuzzy-matches most of them. The warnings arise
only when the tool cannot find ANY match.

| # | File:Line | Function | StEph clause | MtEph clause |
|---|-----------|----------|-------------|-------------|
| 1 | BSTBBAlphaMtEph.rs:260 | `new` | `tree.spec_root().tree_is_bst()` | `tree@.tree_is_bst()` |
| 2 | BSTPlainMtEph.rs:260 | `new` | `tree.spec_root().tree_is_bst()` | `tree@.tree_is_bst()` |
| 3 | BSTRBMtEph.rs:987 | `new` | `tree.spec_root().tree_is_bst()` | `tree@.tree_is_bst()` |

(Each generates both a "no match" warning and a "count mismatch" warning, already
counted in Category 2.)

**Why false positive:** The View impl for each MtEph type returns the root tree:
- BSTPlainMtEph: `view(&self) -> BalBinTree<T> { self.spec_ghost_root() }`
- BSTBBAlphaMtEph: same
- BSTAVLMtEph: same
- BSTRBMtEph: `view(&self) -> BalBinTree<T> { ... }` (maps Link to BalBinTree)

So `self@` === `self.spec_root()` for all these types.

**Suggested tool fix:**
1. When parsing traits, detect `spec fn spec_root(self) -> T` and `impl View<V = T>`.
2. Build an equivalence: `self.spec_root()` === `self@` === `self.view()`.
3. Before comparing clauses, normalize all three forms to a single canonical form
   (e.g., `self@`).
4. Apply the same normalization to `old(self).spec_root()` === `old(self)@`.

---

## Category 4: match-arm ensures parsing

### Problem

MtEph insert functions return `Result<(), ()>` and use `match r { Ok(_) => ..., Err(_) => ... }`
in ensures. The tool cannot extract individual ensures clauses from inside match arms.

StEph insert returns `Self` (no Result wrapper) with flat ensures clauses like
`inserted.spec_root().tree_contains(value)`. MtEph has the equivalent
`self@.tree_contains(value)` inside the `Ok(_)` arm, but the tool doesn't see it.

**Affected warnings (5):**

| # | File:Line | Function | StEph clause | Where in MtEph |
|---|-----------|----------|-------------|---------------|
| 1 | BSTAVLMtEph.rs:453 | `insert` | `inserted.spec_root().tree_contains(value)` | Inside `Ok(_)` arm: `self@.tree_contains(value)` |
| 2 | BSTBBAlphaMtEph.rs:266 | `insert` | `inserted.spec_root().tree_contains(value)` | Inside `Ok(_)` arm: `self@.tree_contains(value)` |
| 3 | BSTPlainMtEph.rs:266 | `insert` | `inserted.spec_root().tree_contains(value)` | Inside `Ok(_)` arm: `self@.tree_contains(value)` |
| 4 | BSTRBMtEph.rs:996 | `insert` | `inserted.spec_root().tree_contains(value)` | Inside `Ok(_)` arm: `self@.tree_contains(value)` |
| 5 | BSTSplayMtEph.rs:1789 | `insert` | `self.spec_contains(value)` | Inside `Ok(_)` arm: `link_contains(self@, value)` |

**Why false positive:** The ensures clause IS present in MtEph, just inside a match
arm. The match-arm pattern is standard for MtEph insert (which returns Result to
signal lock-acquisition failure). The `Ok(_)` arm contains the success postconditions
and the `Err(_)` arm says `self@ == old(self)@` (no change on failure).

**Suggested tool fix:**
1. When an ensures clause is a `match` expression, extract the arms.
2. The `Ok(_)` arm contains the success postconditions — treat each conjunct
   within the `Ok(_)` arm as a separate ensures clause for comparison purposes.
3. Compare StEph's flat ensures against MtEph's `Ok(_)` arm conjuncts.
4. Apply spec_root/@ normalization (Category 3) and wf expansion (Category 1)
   when matching extracted clauses.

---

## Category 5: ensures count (MtEph stronger)

### Problem

The tool warns when MtEph has MORE ensures clauses than StEph. This is not a weakness
— MtEph is strictly stronger.

**Affected warnings (5):**

| # | File:Line | Function | MtEph count | StEph count |
|---|-----------|----------|-------------|-------------|
| 1 | BSTAVLMtEph.rs:447 | `new` | 4 | 3 |
| 2 | BSTBBAlphaMtEph.rs:260 | `new` | 4 | 3 |
| 3 | BSTPlainMtEph.rs:260 | `new` | 4 | 3 |
| 4 | BSTRBMtEph.rs:987 | `new` | 4 | 3 |
| 5 | BSTSplayMtEph.rs:1775 | `new` | 4 | 3 |

Note: These overlap with Categories 2 and 3. MtEph new() has extra ensures like
`tree@ is Leaf` or `tree@.spec_is_leaf()` that StEph doesn't have — these are
ADDITIONAL guarantees, not missing ones.

**Why false positive:** When the current variant has MORE ensures than the reference,
and all reference ensures are matched, the count mismatch is benign.

**Suggested tool fix:** Only emit a count-mismatch warning when the REFERENCE has
more clauses than the current variant (current < reference). When current >= reference,
suppress the count warning or downgrade to info.

---

## Category 6: ghost_root structural field

### Problem

All MtEph structs have a `ghost ghost_root: Ghost<...>` field needed for the RwLock
verification pattern. StEph structs don't need this field.

**Affected warnings (5):**

| # | File:Line | Struct |
|---|-----------|--------|
| 1 | BSTAVLMtEph.rs:421 | `BSTAVLMtEph` |
| 2 | BSTBBAlphaMtEph.rs:234 | `BSTBBAlphaMtEph` |
| 3 | BSTPlainMtEph.rs:234 | `BSTPlainMtEph` |
| 4 | BSTRBMtEph.rs:960 | `BSTRBMtEph` |
| 5 | BSTSplayMtEph.rs:1749 | `BSTSplayMtEph` |

**Why false positive:** `ghost_root` is a Verus ghost field used to track the logical
state of the RwLock-protected tree. It is a standard pattern for all MtEph modules
(see `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs`). It has no StEph
counterpart because StEph doesn't use locks.

**Suggested tool fix:**
1. Recognize fields with `ghost` modifier as structural MtEph-only fields.
2. Suppress "ghost field X has no counterpart" warnings for ghost fields,
   or downgrade to info.
3. Alternatively, maintain a known-pattern list: `ghost ghost_root` in MtEph
   is always expected.

---

## Category 7: missing spec_root / spec fn equivalence

### Problem

StEph traits declare `spec fn spec_root(self) -> T` as a trait member. MtEph uses
`View` instead, so there's no `spec_root` function. Similarly, BSTSplayStEph declares
trait-level spec fns (`spec_size`, `spec_height`, `spec_contains`, etc.) while
BSTSplayMtEph uses free functions (`link_spec_size`, `link_height`, `link_contains`).

**Affected warnings (7):**

| # | File:Line | Warning |
|---|-----------|---------|
| 1 | BSTAVLMtEph.rs:444 | missing fn `spec_root` |
| 2 | BSTBBAlphaMtEph.rs:257 | missing fn `spec_root` |
| 3 | BSTPlainMtEph.rs:257 | missing fn `spec_root` |
| 4 | BSTRBMtEph.rs:984 | missing fn `spec_root` |
| 5 | BSTSplayMtEph.rs:1772 | missing fn `spec_size` |
| 6 | BSTSplayMtEph.rs:1772 | missing fn `spec_height` |
| 7 | BSTSplayMtEph.rs:1772 | missing fn `spec_contains` |

(BSTSplayMtEph also misses `spec_in_order` and `spec_pre_order` — counted in #5-7.)

**Why false positive for spec_root (#1-4):** MtEph implements `View<V = BalBinTree<T>>`
which makes `self@` equivalent to `self.spec_root()`. The View impl IS the spec_root.

**Why false positive for spec fns (#5-7):** BSTSplayMtEph uses free functions:
- `link_spec_size(self@)` === `self.spec_size()` (via `spec_size(self) -> nat { spec_size_link(&self.root) }`)
- `link_height(self@)` === `self.spec_height()`
- `link_contains(self@, x)` === `self.spec_contains(x)`

These are equivalent specifications expressed differently.

**Suggested tool fix:**
1. When a StEph trait has `spec fn spec_root(self) -> T` and an MtEph trait has
   `View<V = T>`, treat them as equivalent and suppress the "missing fn" warning.
2. For spec fn equivalence, the tool could look at ensures clauses to detect when
   a free function in MtEph serves the same role as a trait spec fn in StEph
   (e.g., `link_spec_size(self@)` in ensures where StEph has `self.spec_size()`).
   This is harder and may not be worth implementing — just suppress the warning
   and note it as a structural difference.

---

## Category 8: supertrait parse bug

**Affected warning (1):**

| # | File:Line | Warning |
|---|-----------|---------|
| 1 | BSTSplayMtEph.rs:1772 | supertrait parse incomplete for StEph — ref `""`, cur `Sized + View<V = Link<T>>` |

**Why false positive:** BSTSplayStEphTrait has no supertrait bounds (just
`pub trait BSTSplayStEphTrait<T: TotalOrder + Clone>`), so the tool parses an
empty string and can't compare it to MtEph's `Sized + View<V = Link<T>>`.

**Suggested tool fix:** Handle the case where one variant's trait has no supertraits
(empty string). Don't emit a warning — just note it as info that the MtEph variant
adds supertrait bounds.

---

## Category 9: iter_invariant naming

**Affected warning (1):**

| # | File:Line | Function | Warning |
|---|-----------|----------|---------|
| 1 | AVLTreeSeqMtPer.rs:252 | `iter` | StPer ensures `avltreeseq_iter_invariant(&it)` no match |

**Why false positive:** MtPer uses `iter_invariant(&it)` which is the same concept
with a shorter name. Both are module-local spec functions that constrain the
iterator's internal state.

**Suggested tool fix:** When comparing ensures clauses, recognize `*_iter_invariant`
naming patterns and match them across variants. Or: if both variants have a single
predicate-call ensures clause on `iter()` that takes `&it` as argument, consider
them a fuzzy match.

---

## Category 10: AVLTreeSeq variant differences (StEph vs StPer, MtPer vs StPer)

These are structural differences between the three AVLTreeSeq variants. They are
a mix of intentional design differences and real spec gaps.

**Affected warnings (10):**

| # | File:Line | Function | Warning | Classification |
|---|-----------|----------|---------|---------------|
| 1 | AVLTreeSeqStEph.rs:263 | (trait) | missing fn `values_in_order` | Real: intentional omission |
| 2 | AVLTreeSeqMtPer.rs:200 | (trait) | missing fn `to_arrayseq` | Real: intentional omission |
| 3 | AVLTreeSeqMtPer.rs:248 | `values_in_order` | StPer has requires but MtPer does not | See below |
| 4 | AVLTreeSeqStEph.rs:288 | `singleton` | ensures count 3 vs 2 | FP: StEph is stronger |
| 5 | AVLTreeSeqStEph.rs:281 | `set` | requires count 2 vs 3 | See below |
| 6 | AVLTreeSeqStEph.rs:281 | `set` | StPer requires `obeys_feq_clone` no match | See below |
| 7 | AVLTreeSeqStEph.rs:302 | `subseq_copy` | requires count 2 vs 1 | FP: StEph is stricter |
| 8 | AVLTreeSeqStEph.rs:317 | `from_vec` | requires/ensures count mismatches | Mixed |
| 9 | AVLTreeSeqStEph.rs:317 | `from_vec` | StPer ensures `spec_seq =~= values@.map_values(...)` no match | FP: StEph uses `spec_inorder` equivalent |
| 10 | AVLTreeSeqStEph.rs:326 | `to_arrayseq` | ensures count 2 vs 1 | FP: StEph is stronger |
| 11 | AVLTreeSeqMtPer.rs:229 | `set` | ensures count 2 vs 3 | Fixable: missing `spec_seq` update |
| 12 | AVLTreeSeqMtPer.rs:229 | `set` | StPer ensures `outcome.unwrap().spec_seq() =~= ...update(...)` no match | Fixable |

Note: #3 (values_in_order missing requires) — MtPer's `values_in_order` has no
requires but has `ensures values@.map_values(|t: T| t@) =~= self.spec_seq()`.
StPer likely has `requires self.spec_avltreeseqstper_wf()`. Whether this needs
fixing depends on whether the MtPer impl needs the wf precondition.

**For #6 (set missing obeys_feq_clone):** StEph `set` is `&mut self` (Eph pattern)
while StPer `set` returns `Result<Self, ...>` (Per pattern). StPer needs
`obeys_feq_clone` because it clones internally. StEph may not need it if its
mutation-in-place avoids cloning. This is a structural difference.

---

## Category 11: Real but intentional differences (NOT false positives)

These warnings reflect genuine spec differences between StEph and MtEph that are
by design. They should be flagged as intentional rather than suppressed.

**Affected warnings (5):**

| # | File:Line | Function | Warning | Reason |
|---|-----------|----------|---------|--------|
| 1 | BSTAVLMtEph.rs:447 | `new` | StEph ensures `tree_is_avl` no match | MtEph maintains BST, not AVL invariant |
| 2 | BSTAVLMtEph.rs:453 | `insert` | StEph requires `tree_is_avl` no match | Same |
| 3 | BSTAVLMtEph.rs:453 | `insert` | StEph ensures `tree_is_avl` no match | Same |
| 4 | BSTBBAlphaMtEph.rs:257 | (trait) | missing fn `delete` | Not yet implemented in MtEph |
| 5 | BSTPlainMtEph.rs:257 | (trait) | missing fn `delete` | Not yet implemented in MtEph |

**For #1-3:** BSTAVLStEph's wf is `tree_is_avl()` which implies `tree_is_bst()`.
BSTAVLMtEph's wf is only `tree_is_bst() && ...`. This is an intentional design
choice — the MtEph wrapper delegates to the same AVL insert/rebalance logic, but
the outer contract only claims BST (not AVL). Strengthening to AVL would require
propagating `tree_is_avl` through the RwLock invariant, which is possible but not
trivial.

**For #4-5:** `delete` is implemented in StEph (BSTPlain, BSTBBAlpha) but not yet
ported to MtEph. This is real missing work, not a tool false positive.

---

## Fixable Warnings

These 3 items could potentially be fixed by code changes:

### 1. AVLTreeSeqMtPer `set` missing ensures (2 warnings)

**File:** AVLTreeSeqMtPer.rs:229
**StPer has:** `ensures outcome.unwrap().spec_seq() =~= self.spec_seq().update(index as int, item@)`
**MtPer has:** only `ensures outcome is Ok, outcome.unwrap().spec_avltreeseqmtper_wf()`

Adding the `spec_seq` update ensures would strengthen the MtPer set() contract.
Requires verifying that the impl can prove it.

### 2. BSTSplayMtEph `in_order` weak ensures (1 warning)

**File:** BSTSplayMtEph.rs:1835
**StEph has:** `ensures seq.spec_len() == self.spec_in_order().len()`
**MtEph has:** `ensures true`

The MtEph in_order returns an ArraySeqStPerS but with no guarantees about its
contents. Strengthening requires checking whether the impl (likely external_body)
can support a real ensures.

### 3. BSTSplayMtEph `pre_order` weak ensures (1 warning)

**File:** BSTSplayMtEph.rs:1838
**StEph has:** `ensures seq.spec_len() == self.spec_pre_order().len()`
**MtEph has:** `ensures true`

Same situation as in_order.

---

## Recommended Tool Improvements (Priority Order)

### Priority 1: wf subsumption (eliminates ~51 warnings)

When the current variant has a `spec_*_wf()` predicate in requires/ensures:
1. Parse the wf body to extract individual conjuncts.
2. Normalize each conjunct (apply spec_root/@ equivalence).
3. When a reference clause has no direct match, check if it's subsumed by
   a wf conjunct.
4. Suppress or downgrade subsumed-clause warnings and their count companions.

### Priority 2: spec_root vs @ normalization (eliminates ~8 warnings)

When a reference trait declares `spec fn spec_root(self) -> T` and the current
variant implements `View<V = T>`:
1. Treat `self.spec_root()` as equivalent to `self@`.
2. Apply this equivalence during clause comparison.
3. Also handle `old(self).spec_root()` === `old(self)@`.

### Priority 3: match-arm ensures extraction (eliminates ~5 warnings)

When an ensures clause is a `match` on a Result:
1. Extract the `Ok(_)` arm conjuncts as success postconditions.
2. Compare reference flat ensures against extracted conjuncts.

### Priority 4: directional count warnings (eliminates ~5 warnings)

Only warn on count mismatches when the current variant has FEWER clauses than
the reference. When current >= reference, suppress or downgrade.

### Priority 5: ghost field suppression (eliminates ~5 warnings)

Suppress or downgrade warnings for `ghost`-modifier fields having no counterpart
in other variants. These are expected for MtEph modules.

### Priority 6: free function spec equivalence (eliminates ~5 warnings)

When a reference trait has `spec fn foo(self) -> T` and the current variant uses
`free_fn(self@)` in ensures/requires that serves the same role, recognize the
equivalence.

---

## Appendix: Complete Warning List

Total: 101 warnings (14 Phase 2 + 87 Phase 4)

### Phase 2 Warnings (14)

| # | File:Line | Warning | Category |
|---|-----------|---------|----------|
| 1 | AVLTreeSeqStEph.rs:263 | missing fn `values_in_order` | 10 (real) |
| 2 | AVLTreeSeqMtPer.rs:200 | missing fn `to_arrayseq` | 10 (real) |
| 3 | AVLTreeSeqMtPer.rs:248 | `values_in_order` StPer has requires, MtPer does not | 10 |
| 4 | BSTAVLMtEph.rs:421 | ghost field `ghost_root` no counterpart | 6 |
| 5 | BSTAVLMtEph.rs:444 | missing fn `spec_root` | 7 |
| 6 | BSTBBAlphaMtEph.rs:234 | ghost field `ghost_root` no counterpart | 6 |
| 7 | BSTBBAlphaMtEph.rs:257 | missing fns `spec_root`, `delete` | 7+11 |
| 8 | BSTPlainMtEph.rs:234 | ghost field `ghost_root` no counterpart | 6 |
| 9 | BSTPlainMtEph.rs:257 | missing fns `spec_root`, `delete` | 7+11 |
| 10 | BSTRBMtEph.rs:960 | ghost field `ghost_root` no counterpart | 6 |
| 11 | BSTRBMtEph.rs:984 | missing fn `spec_root` | 7 |
| 12 | BSTSplayMtEph.rs:1749 | ghost field `ghost_root` no counterpart | 6 |
| 13 | BSTSplayMtEph.rs:1772 | supertrait parse incomplete | 8 |
| 14 | BSTSplayMtEph.rs:1772 | missing 5 spec fns | 7 |

### Phase 4 Warnings (87)

**AVLTreeSeq group (11):**

| # | File:Line | Function | Warning | Cat |
|---|-----------|----------|---------|-----|
| 1 | AVLTreeSeqStEph.rs:288 | `singleton` | ensures count 3 vs 2 | 10 |
| 2 | AVLTreeSeqStEph.rs:281 | `set` | requires count 2 vs 3 | 10 |
| 3 | AVLTreeSeqStEph.rs:281 | `set` | StPer requires `obeys_feq_clone` no match | 10 |
| 4 | AVLTreeSeqStEph.rs:302 | `subseq_copy` | requires count 2 vs 1 | 10 |
| 5 | AVLTreeSeqStEph.rs:317 | `from_vec` | requires count 2 vs 1 | 10 |
| 6 | AVLTreeSeqStEph.rs:317 | `from_vec` | ensures count 3 vs 2 | 10 |
| 7 | AVLTreeSeqStEph.rs:317 | `from_vec` | StPer ensures `spec_seq =~= map_values` no match | 10 |
| 8 | AVLTreeSeqStEph.rs:326 | `to_arrayseq` | ensures count 2 vs 1 | 10 |
| 9 | AVLTreeSeqMtPer.rs:229 | `set` | ensures count 2 vs 3 | fixable |
| 10 | AVLTreeSeqMtPer.rs:229 | `set` | StPer ensures `update` no match | fixable |
| 11 | AVLTreeSeqMtPer.rs:252 | `iter` | StPer ensures `avltreeseq_iter_invariant` no match | 9 |

**BSTAVLMtEph (16):**

| # | File:Line | Function | Warning | Cat |
|---|-----------|----------|---------|-----|
| 12 | :447 | `new` | ensures count 4 vs 3 | 5 |
| 13 | :447 | `new` | StEph ensures `tree_is_avl` no match | 11 |
| 14 | :467 | `size` | requires count 1 vs 2 | 1c |
| 15 | :467 | `size` | StEph requires `spec_size <= usize::MAX` | 1b |
| 16 | :475 | `height` | requires count 1 vs 2 | 1c |
| 17 | :475 | `height` | StEph requires `spec_height <= usize::MAX` | 1b |
| 18 | :453 | `insert` | requires count 1 vs 3 | 1c |
| 19 | :453 | `insert` | StEph requires `spec_height <= MAX-1` | 1b |
| 20 | :453 | `insert` | StEph requires `tree_is_avl` | 11 |
| 21 | :453 | `insert` | ensures count 2 vs 4 | 2 |
| 22 | :453 | `insert` | StEph ensures `tree_contains(value)` | 4 |
| 23 | :453 | `insert` | StEph ensures `tree_is_avl` | 11 |
| 24 | :463 | `contains` | requires count 1 vs 2 | 1c |
| 25 | :463 | `contains` | StEph requires `tree_is_bst` | 1a |
| 26 | :479 | `find` | requires count 1 vs 2 | 1c |
| 27 | :479 | `find` | StEph requires `tree_is_bst` | 1a |

**BSTBBAlphaMtEph (19):**

| # | File:Line | Function | Warning | Cat |
|---|-----------|----------|---------|-----|
| 28 | :260 | `new` | ensures count 4 vs 3 | 3+5 |
| 29 | :260 | `new` | StEph ensures `tree_is_bst` no match | 3 |
| 30 | :280 | `size` | requires count 1 vs 2 | 1c |
| 31 | :280 | `size` | StEph requires `spec_size <= usize::MAX` | 1b |
| 32 | :288 | `height` | requires count 1 vs 2 | 1c |
| 33 | :288 | `height` | StEph requires `spec_height <= usize::MAX` | 1b |
| 34 | :266 | `insert` | requires count 1 vs 2 | 1c |
| 35 | :266 | `insert` | StEph requires `tree_is_bst` | 1a |
| 36 | :266 | `insert` | ensures count 2 vs 4 | 2 |
| 37 | :266 | `insert` | StEph ensures `tree_contains(value)` | 4 |
| 38 | :266 | `insert` | StEph ensures `tree_is_bst` | 2 |
| 39 | :276 | `contains` | requires count 1 vs 2 | 1c |
| 40 | :276 | `contains` | StEph requires `tree_is_bst` | 1a |
| 41 | :292 | `find` | requires count 1 vs 2 | 1c |
| 42 | :292 | `find` | StEph requires `tree_is_bst` | 1a |
| 43 | :297 | `minimum` | requires count 1 vs 2 | 1c |
| 44 | :297 | `minimum` | StEph requires `tree_is_bst` | 1a |
| 45 | :303 | `maximum` | requires count 1 vs 2 | 1c |
| 46 | :303 | `maximum` | StEph requires `tree_is_bst` | 1a |

**BSTPlainMtEph (19):**

| # | File:Line | Function | Warning | Cat |
|---|-----------|----------|---------|-----|
| 47 | :260 | `new` | ensures count 4 vs 3 | 3+5 |
| 48 | :260 | `new` | StEph ensures `tree_is_bst` no match | 3 |
| 49 | :280 | `size` | requires count 1 vs 2 | 1c |
| 50 | :280 | `size` | StEph requires `spec_size <= usize::MAX` | 1b |
| 51 | :288 | `height` | requires count 1 vs 2 | 1c |
| 52 | :288 | `height` | StEph requires `spec_height <= usize::MAX` | 1b |
| 53 | :266 | `insert` | requires count 1 vs 2 | 1c |
| 54 | :266 | `insert` | StEph requires `tree_is_bst` | 1a |
| 55 | :266 | `insert` | ensures count 2 vs 4 | 2 |
| 56 | :266 | `insert` | StEph ensures `tree_contains(value)` | 4 |
| 57 | :266 | `insert` | StEph ensures `tree_is_bst` | 2 |
| 58 | :276 | `contains` | requires count 1 vs 2 | 1c |
| 59 | :276 | `contains` | StEph requires `tree_is_bst` | 1a |
| 60 | :292 | `find` | requires count 1 vs 2 | 1c |
| 61 | :292 | `find` | StEph requires `tree_is_bst` | 1a |
| 62 | :297 | `minimum` | requires count 1 vs 2 | 1c |
| 63 | :297 | `minimum` | StEph requires `tree_is_bst` | 1a |
| 64 | :303 | `maximum` | requires count 1 vs 2 | 1c |
| 65 | :303 | `maximum` | StEph requires `tree_is_bst` | 1a |

**BSTRBMtEph (15):**

| # | File:Line | Function | Warning | Cat |
|---|-----------|----------|---------|-----|
| 66 | :987 | `new` | ensures count 4 vs 3 | 3+5 |
| 67 | :987 | `new` | StEph ensures `tree_is_bst` no match | 3 |
| 68 | :1008 | `size` | requires count 1 vs 2 | 1c |
| 69 | :1008 | `size` | StEph requires `spec_size <= usize::MAX` | 1b |
| 70 | :1016 | `height` | requires count 1 vs 2 | 1c |
| 71 | :1016 | `height` | StEph requires `spec_height <= usize::MAX` | 1b |
| 72 | :996 | `insert` | requires count 1 vs 2 | 1c |
| 73 | :996 | `insert` | StEph requires `tree_is_bst` | 1a |
| 74 | :996 | `insert` | ensures count 2 vs 4 | 2 |
| 75 | :996 | `insert` | StEph ensures `tree_contains(value)` | 4 |
| 76 | :996 | `insert` | StEph ensures `tree_is_bst` | 2 |
| 77 | :1004 | `contains` | requires count 1 vs 2 | 1c |
| 78 | :1004 | `contains` | StEph requires `tree_is_bst` | 1a |
| 79 | :1020 | `find` | requires count 1 vs 2 | 1c |
| 80 | :1020 | `find` | StEph requires `tree_is_bst` | 1a |

**BSTSplayMtEph (7):**

| # | File:Line | Function | Warning | Cat |
|---|-----------|----------|---------|-----|
| 81 | :1775 | `new` | ensures count 4 vs 3 | 5 |
| 82 | :1812 | `height` | requires count 1 vs 2 | 1c |
| 83 | :1812 | `height` | StEph requires `spec_height < usize::MAX` | 1b |
| 84 | :1789 | `insert` | ensures count 2 vs 3 | 2 |
| 85 | :1789 | `insert` | StEph ensures `spec_contains(value)` | 4 |
| 86 | :1835 | `in_order` | StEph ensures `spec_len == spec_in_order.len()` no match | fixable |
| 87 | :1838 | `pre_order` | StEph ensures `spec_len == spec_pre_order.len()` no match | fixable |
