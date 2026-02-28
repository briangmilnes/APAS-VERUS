<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 39 — Treaps: Review Against Prose

**Date:** 2026-02-28 (updated)
**Reviewer:** Claude-Opus-4.6
**Previous reviews:** 2026-02-19, 2026-02-27, 2026-02-28

## Phase 1: Inventory (tool-generated)

See `veracity-review-module-fn-impls.md` for full function inventory.

| # | Module | V! fns | -V! fns | Specs (Unk) | Holes | NoSpec |
|---|--------|:------:|:-------:|:-----------:|:-----:|:------:|
| 1 | BSTParaTreapMtEph | 1 | 32 | 0 | 1 | 32 |
| 2 | BSTSetTreapMtEph | 0 | 22 | 0 | 0 | 22 |
| 3 | BSTTreapMtEph | 28 | 0 | 9 | 1 | 18 |
| 4 | BSTTreapStEph | 33 | 0 | 22 (1 holed) | 1 | 10 |

**Changes since Feb 27 review:**
- BSTTreapStEph: `rotate_left` now has BST preservation spec (`spec_bst_link(&Some(x)) ==> spec_bst_link(&Some(rotated))`). However, the proof uses one `assume()` at line 470 for is_lt transitivity through the rotated subtree. Module status changed from 0 holes to 1 hole.
- Feb 28: `find_link` gained partial spec (forward containment). `in_order_vec` and `pre_order_vec` gained length-preservation specs. Spec coverage 20→23/33.
- BSTTreapMtEph, ParaTreap, SetTreap: unchanged.

## Phase 2: Prose Inventory

Source: `prompts/Chap39.txt`

### Definitions

| # | Item | Description |
|---|------|-------------|
| 1 | Definition 39.1 (Treap) | BST over keys K with priority function p : K -> Z, satisfying BST property on keys and max-heap property on priorities. |
| 2 | Treap Type | `type T = TLeaf \| TNode of (T x K x Z x T)` — recursive type: (left, key, priority, right). |
| 3 | Exposed Type | `type E = Leaf \| Node of (T x K x T)` — exposed view hiding priority. |

### Algorithms (Data Structure 39.3)

| # | Item | Description |
|---|------|-------------|
| 1 | `priority(T)` | Returns priority of root or -inf for leaf. O(1). |
| 2 | `join(T1, (k, p), T2)` | Core join maintaining BST + heap property. |
| 3 | `expose(T)` | Strips priority from node. O(1). |
| 4 | `joinMid(E)` | Recomputes priority from key, delegates to join. |
| 5 | Algorithm 39.2 (qsTree) | Analytical tool for height proof. Not an ADT op. |

### Cost Specifications

| # | Operation | Work | Span |
|---|-----------|------|------|
| 1 | priority | O(1) | O(1) |
| 2 | join | O(log n) w.h.p. | O(log n) w.h.p. |
| 3 | expose | O(1) | O(1) |
| 4 | joinMid | O(log n) w.h.p. | O(log n) w.h.p. |
| 5 | split | O(log n) w.h.p. | O(log n) w.h.p. |

### Theorems

| # | Item | Description |
|---|------|-------------|
| 1 | Height bound | O(lg n) w.h.p. via quicksort recursion tree isomorphism. |
| 2 | Exercise 39.1 | Unique priorities => unique tree structure. |

## Phase 3: Algorithmic Analysis

### 3a. Cost Annotations

All cost annotations present in dual-line `/// - APAS:` / `/// - Claude-Opus-4.6:` format across all four source files.

### 3b. Implementation Fidelity

| # | Prose Item | File | Function | Fidelity | Notes |
|---|------------|------|----------|----------|-------|
| 1 | `join(T1, (k,p), T2)` | BSTParaTreapMtEph | `join_with_priority` | Faithful | Three-way case matches prose. |
| 2 | `expose(T)` | BSTParaTreapMtEph | `expose` | Faithful | Strips priority. |
| 3 | `joinMid(E)` | BSTParaTreapMtEph | `join_mid` | Faithful | Delegates to join. |
| 4 | `priority(T)` | BSTParaTreapMtEph | `tree_priority` | Faithful | i64::MIN for empty. |
| 5 | `split` | BSTParaTreapMtEph | `split_inner` | Faithful | Recursive split. |
| 6 | Treap type | BSTParaTreapMtEph | `ParamTreap`+`NodeInner` | Faithful | Matches TNode. |
| 7 | Exposed type | BSTParaTreapMtEph | `Exposed<T>` | Faithful | Matches E. |
| 8 | qsTree | — | — | Not impl | Analytical only; appropriate. |
| 9 | StEph insert | BSTTreapStEph | `insert_link` | Alternative | Rotation-based (Aragon & Seidel). Same O(log n) cost. |
| 10 | MtEph insert | BSTTreapMtEph | `insert_link` | Alternative | Rotation-based, Arc<RwLock<>>. |
| 11 | ParaTreap insert | BSTParaTreapMtEph | `insert` | Faithful | split+join. |

### 3c. Spec Fidelity

**BSTTreapStEph.rs** — Partial specification coverage:

| # | Function | Spec Status | Assessment |
|---|----------|-------------|------------|
| 1 | new | `ensures spec_size()==0, spec_wf()` | Strong for constructor. |
| 2 | size | `ensures sz as nat == spec_size()` | Strong. |
| 3 | is_empty | `ensures empty == (spec_size()==0)` | Strong. |
| 4 | height | `requires spec_size() < MAX, spec_wf()` | Partial — no ensures. |
| 5 | insert | `requires old.spec_size()+1 <= MAX, old.spec_wf()` / `ensures spec_wf(), bounded size` | Partial — does not ensure `spec_contains_link`. |
| 6 | find (via find_link) | `ensures found.is_some() ==> spec_contains_link(link, *found.unwrap())` | Partial — forward direction only. Full bidirectional spec blocked by PartialEq spec bridge for generic T. |
| 7 | contains | No spec | None. Needs find_link bidirectional spec. |
| 8 | minimum | No spec | None. |
| 9 | maximum | No spec | None. |
| 10 | in_order (via in_order_vec) | `ensures ordered@.len() == spec_in_order_link(link).len()` | Partial — length only. Full structural `=~=` spec blocked by generic Clone axiom gap. |
| 11 | pre_order (via pre_order_vec) | `ensures ordered@.len() == spec_pre_order_link(link).len()` | Partial — length only. Same Clone blocker as in_order. |

Spec functions defined: `spec_size_link`, `spec_contains_link`, `spec_bst_link`, `spec_size_wf_link`, `spec_in_order_link`, `spec_pre_order_link`, `spec_height_link` (7 total).

Proof functions (8 total, all clean):
- `lemma_height_le_size`, `lemma_size_wf_child_bounded`, `lemma_wf_decompose`, `lemma_wf_assemble_node` (structural wf).
- `lemma_contains_left`, `lemma_contains_right` (containment propagation through children).
- `lemma_bst_decompose` (BST property decomposes to children + quantified bounds).
- `lemma_contains_root` (root key is contained in its own subtree).

Trait-level spec fns: `spec_size`, `spec_wf`, `spec_bst` (3 total).

**BSTTreapMtEph.rs** — Structural verification only:

All functions inside `verus!`. Has 3 proof fns (`lemma_height_le_size`, `lemma_size_wf_child_bounded`, `lemma_wf_assemble_node`) supporting `RwLock` invariant `TreapLinkWf`. The trait has no `requires`/`ensures` because the `Arc<RwLock<>>` wrapping prevents direct spec fn access on the struct. The RwLock invariant (`spec_size_wf_link` + `spec_size_link < MAX`) enforces well-formedness internally.

**BSTParaTreapMtEph.rs** — No specifications. All outside `verus!` except `new_treap_lock`.

**BSTSetTreapMtEph.rs** — No specifications. All outside `verus!`. Delegates all operations to `ParamTreap`, achieving correct asymptotic costs.

### 3d. Cost Fidelity Table

| # | Function | File | APAS Work | Impl Work | Match? |
|---|----------|------|-----------|-----------|--------|
| 1 | join_with_priority | ParaTreap | O(log n) | O(log n) | Yes |
| 2 | expose | ParaTreap | O(1) | O(1) | Yes |
| 3 | join_mid | ParaTreap | O(log n) | O(log n) | Yes |
| 4 | tree_priority | ParaTreap | O(1) | O(1) | Yes |
| 5 | split_inner | ParaTreap | O(log n) | O(log n) | Yes |
| 6 | insert (ParaTreap) | ParaTreap | O(lg n) | O(lg n) | Yes |
| 7 | delete (ParaTreap) | ParaTreap | O(lg n) | O(lg n) | Yes |
| 8 | union | ParaTreap | O(m lg(n/m)) | O(m lg(n/m)) | Yes |
| 9 | intersect | ParaTreap | O(m lg(n/m)) | O(m lg(n/m)) | Yes |
| 10 | difference | ParaTreap | O(m lg(n/m)) | O(m lg(n/m)) | Yes |
| 11 | filter | ParaTreap | O(n) | O(n) | Yes |
| 12 | reduce | ParaTreap | O(n) | O(n) | Yes |
| 13 | insert_link (StEph) | StEph | O(log n) | O(log n) | Yes |
| 14 | union | SetTreap | O(m lg(n/m)) | O(m lg(n/m)) | Yes |
| 15 | intersection | SetTreap | O(m lg(n/m)) | O(m lg(n/m)) | Yes |
| 16 | difference | SetTreap | O(m lg(n/m)) | O(m lg(n/m)) | Yes |
| 17 | split | SetTreap | O(log n) | O(log n) | Yes |
| 18 | join_pair | SetTreap | O(log n) | O(log n) | Yes |
| 19 | join_m | SetTreap | O(log n) | O(log n) | Yes |
| 20 | filter | SetTreap | O(n) | O(n) | Yes |
| 21 | reduce | SetTreap | O(n) | O(n) | Yes |

All cost fidelity matches.

### 3e. Notable Design Decisions

1. **priority_for uses hashing, not randomness.** BSTParaTreapMtEph derives priorities via `Debug` formatting + `Hash`. Deterministic but depends on hash distribution. StEph/MtEph take priority as a parameter.

2. **Two insertion strategies coexist.** StEph/MtEph use rotation-based (Aragon & Seidel 1989). ParaTreapMtEph uses split+join.

3. **BSTSetTreapMtEph is a thin shim over ParamTreap.** All operations delegate directly, achieving correct asymptotic costs.

4. **MtKey and Pred type aliases.** Keep signatures clean.

## Phase 4: Parallelism Review

### 4a. Mt Module Classification

| # | Module | Threading Model | Parallel? |
|---|--------|-----------------|-----------|
| 1 | BSTTreapMtEph | Arc<RwLock<>> at root; sequential under lock. | No — thread-safe only. |
| 2 | BSTSetTreapMtEph | Thin shim over ParamTreap. | Yes — inherits ParamTreap parallelism. |
| 3 | BSTParaTreapMtEph | Arc<RwLock<>> per node. ParaPair! fork-join. | Yes — genuine parallelism. |

### 4b. Parallel Operations in BSTParaTreapMtEph

| # | Function | ParaPair!? | APAS Span | Achieved Span |
|---|----------|:----------:|-----------|---------------|
| 1 | union_inner | Yes | O(lg n) | O(lg n) |
| 2 | intersect_inner | Yes | O(lg n) | O(lg n) |
| 3 | difference_inner | Yes | O(lg n) | O(lg n) |
| 4 | filter_inner | Yes | O(lg t) | O(lg t) |
| 5 | reduce_inner | Yes | O(lg t) | O(lg t) |
| 6 | join_with_priority | No | O(log n) | O(log n) |
| 7 | split_inner | No | O(log n) | O(log n) |

### 4c. Parallelism Gap Summary

No parallelism gaps. BSTSetTreapMtEph delegates to ParamTreap for all aggregate operations. BSTTreapMtEph is thread-safe but sequential by design (single RwLock at root).

## Phase 5: RTT Review

### 5a. Coverage Matrix

| # | Source Module | RTT File | Tests | Status |
|---|-------------|----------|:-----:|--------|
| 1 | BSTTreapStEph.rs | TestBSTTreapStEph.rs | 30 | Present |
| 2 | BSTTreapMtEph.rs | TestBSTTreapMtEph.rs | 30 | Present |
| 3 | BSTSetTreapMtEph.rs | TestBSTSetTreapMtEph.rs | 31 | Present |
| 4 | BSTParaTreapMtEph.rs | TestBSTParaTreapMtEph.rs | 26 | Present |

All source modules have RTT coverage.

### 5b. Missing Tests

| # | Priority | Recommendation |
|---|----------|---------------|
| 1 | Low | Structural property tests: verify BST ordering + heap property after ops. |
| 2 | Low | BSTParaTreapMtEph concurrent stress test. |

## Phase 6: PTT Review

BSTTreapStEph.rs and BSTTreapMtEph.rs are inside `verus!` but have no iterators or verified loops requiring PTT coverage. BSTParaTreapMtEph.rs and BSTSetTreapMtEph.rs are outside `verus!`.

**No PTTs needed** at this time.

### Unified Test Inventory

| # | Source Module | RTT File | PTT File | Status |
|---|-------------|----------|----------|--------|
| 1 | BSTTreapStEph.rs | TestBSTTreapStEph.rs | — | RTT only |
| 2 | BSTTreapMtEph.rs | TestBSTTreapMtEph.rs | — | RTT only |
| 3 | BSTSetTreapMtEph.rs | TestBSTSetTreapMtEph.rs | — | RTT only |
| 4 | BSTParaTreapMtEph.rs | TestBSTParaTreapMtEph.rs | — | RTT only |

## Phase 7: Gap Analysis

### Prose Items With No Implementation

| # | Item | Assessment |
|---|------|-----------|
| 1 | Algorithm 39.2 (qsTree) | Analytical tool. Omission appropriate. |
| 2 | Exercise 39.1 (uniqueness) | Would need formal proof. Future work. |
| 3 | Height bound theorem | Probabilistic; not expressible as Verus spec. Runtime test provides sanity check. |

### Code With No Prose Counterpart

| # | Item | File(s) | Notes |
|---|------|---------|-------|
| 1 | rotate_left/right | StEph, MtEph | Standard BST rotation. |
| 2 | Rotation-based insert | StEph, MtEph | Alternative to split+join. |
| 3 | height/height_link | StEph, MtEph | Utility. |
| 4 | Traversals (in_order, pre_order) | All | Standard BST. |
| 5 | BSTSetTreapMtEph (module) | SetTreap | Set API wrapper. |
| 6 | find, contains, min, max | All | Standard BST. |
| 7 | priority_for (hash) | ParaTreap | Deterministic priority. |
| 8 | size augmentation | All | Prose defers to Ch40. |
| 9 | delete | ParaTreap, SetTreap | Not in Ch39 prose. |
| 10 | join_pair, union, intersect, diff | ParaTreap, SetTreap | From Ch38 interface. |
| 11 | filter, reduce | ParaTreap, SetTreap | Aggregate ops. |

## Phase 8: TOC Review and In/Out Table

### TOC Status

| # | File | TOC Present? | Section Order |
|---|------|:------------:|---------------|
| 1 | BSTTreapStEph.rs | Yes | 1,4,6,8,9,11,12,13 — correct |
| 2 | BSTTreapMtEph.rs | Yes | 1,4,6,7,8,9,11,12,13 — correct |
| 3 | BSTParaTreapMtEph.rs | Yes | 1,4,6,8,9,12,13 — correct |
| 4 | BSTSetTreapMtEph.rs | No (has numbered section comments but no TOC header) | 4,6,8,9,12,13 — correct |

### In/Out Table

| # | File | Clone | PartialEq | Default | Drop | Iterator | Debug | Macro |
|---|------|:-----:|:---------:|:-------:|:----:|:--------:|:-----:|:-----:|
| 1 | BSTTreapStEph | ✅ in | - | ✅ in | - | - | ✅ out | ✅ out |
| 2 | BSTTreapMtEph | ✅ in | - | ✅ in | - | - | ✅ out | ✅ out |
| 3 | BSTSetTreapMtEph | ✅ out | - | - | - | - | ✅ out | ✅ out |
| 4 | BSTParaTreapMtEph | ✅ out | - | - | - | - | - | ✅ out |

## Proof Holes Summary

From `veracity-review-proof-holes -d src/Chap39/`:

```
Modules:
   3 clean (no holes)
   1 holed (contains holes)
   4 total

Proof Functions:
   11 clean
   0 holed
   11 total

Holes Found: 1 total
   1 × assume()

Location:
  BSTTreapStEph.rs:470 — assume(forall |k: T| spec_contains_link(&y.left, k) ==> k.is_lt(&yk))
    In: rotate_left (line 388)
    Purpose: BST rotation preserves ordering — all keys in new y.left < yk.
    Blocker: Verus cannot prove is_lt transitivity through partial_cmp_spec for generic T.

Info (5):
  BSTParaTreapMtEph.rs:76  — verus_rwlock_external_body (expected, unfixable)
  BSTTreapMtEph.rs:199     — accept() for recursive type structural equality
  BSTTreapMtEph.rs:212     — accept() for recursive type structural equality
  BSTTreapMtEph.rs:224     — verus_rwlock_external_body (expected, unfixable)
  BSTTreapMtEph.rs:554     — accept() for recursive type structural equality
```

The accept() calls bridge structural equality for recursive types behind RwLock. The external_body calls are inherent to Verus RwLock::new.

## Verus Style Review Summary

From `veracity-review-verus-style`: **56 passed, 23 warnings** across 4 files.

| # | File | Warnings | Details |
|---|------|:--------:|---------|
| 1 | BSTTreapStEph | 6 | [12] 6 trait fns missing requires/ensures (find, contains, min, max, in_order, pre_order) |
| 2 | BSTTreapMtEph | 11 | [12] 11 trait fns missing requires/ensures |
| 3 | BSTParaTreapMtEph | 4 | [13] trait impl outside verus!, [15] 3x Clone outside verus! |
| 4 | BSTSetTreapMtEph | 2 | [13] trait impl outside verus!, [15] Clone outside verus! |

The [13]/[15] warnings on ParaTreap and SetTreap are expected: those modules are not yet verusified. The [12] warnings on MtEph and StEph identify the spec gaps documented in Phase 3c.

## Verusification Table

| # | Module | Inside V! | Spec Coverage | Proof Holes | Proof Fns | Spec Fns | Status |
|---|--------|:---------:|:-------------:|:-----------:|:---------:|:--------:|--------|
| 1 | BSTTreapStEph | 33/33 | 23/33 (70%) | 1 assume() | 8 | 7+3 trait | BST proof infra + find/traversal specs |
| 2 | BSTTreapMtEph | 28/28 | 9/28 (32%) | 0 | 3 | 7 | Structurally verified |
| 3 | BSTParaTreapMtEph | 1/33 (3%) | 0/33 (0%) | 0 | 0 | 0 | Unverified |
| 4 | BSTSetTreapMtEph | 0/22 (0%) | 0/22 (0%) | 0 | 0 | 0 | Unverified |
| | **Totals** | **62/116** | **32/116 (28%)** | **1** | **11** | **17** | |

### Spec Strength Classification

**BSTTreapStEph** (23 specified functions):

| # | Function | Strength | Notes |
|---|----------|:--------:|-------|
| 1 | new | Strong | Ensures size==0, wf. |
| 2 | size | Strong | Ensures sz == spec_size(). |
| 3 | is_empty | Strong | Ensures empty == (size==0). |
| 4 | height | Partial | Requires only; no ensures on result value. |
| 5 | insert | Partial | Ensures wf + bounded size. Missing: spec_contains_link. |
| 6 | clone_link | Partial | Ensures size preserved. Missing: structural eq. |
| 7 | size_link | Strong | Ensures sz == spec_size_link. |
| 8 | height_link | Strong | Ensures h == spec_height_link. |
| 9 | update_size | Strong | Ensures correct size field + key/children unchanged. |
| 10 | rotate_left | Partial | Ensures wf + size preserved + BST preserved. **Has 1 assume().** |
| 11 | rotate_right | Partial | Ensures wf + size preserved. Missing: BST property. |
| 12 | insert_link | Partial | Ensures wf + bounded size. Missing: spec_contains_link. |
| 13 | lemma_height_le_size | Strong | height <= size. |
| 14 | lemma_size_wf_child_bounded | Strong | Children bounded under MAX. |
| 15 | lemma_wf_decompose | Strong | Decomposes node wf. |
| 16 | lemma_wf_assemble_node | Strong | Assembles node wf. |
| 17 | lemma_contains_left | Strong | Containment propagation: child left to parent. |
| 18 | lemma_contains_right | Strong | Containment propagation: child right to parent. |
| 19 | lemma_bst_decompose | Strong | BST decomposes to children + quantified bounds. |
| 20 | lemma_contains_root | Strong | Root key is contained in its own subtree. |
| 21 | find_link | Partial | Forward containment: `found.is_some() ==> spec_contains_link(link, *found.unwrap())`. Reverse direction blocked by PartialEq spec bridge. |
| 22 | in_order_vec | Partial | Length preservation: `ordered@.len() == spec_in_order_link(link).len()`. Full structural `=~=` blocked by generic Clone axiom gap. |
| 23 | pre_order_vec | Partial | Length preservation: `ordered@.len() == spec_pre_order_link(link).len()`. Same Clone blocker. |

**Summary:** 12 strong, 8 partial (rotate_left has 1 assume; find_link, in_order_vec, pre_order_vec have partial specs), 0 weak, 10 none.

**BSTTreapMtEph** (9 specified functions):

| # | Function | Strength | Notes |
|---|----------|:--------:|-------|
| 1 | lemma_height_le_size | Strong | height <= size. |
| 2 | lemma_size_wf_child_bounded | Strong | Children bounded. |
| 3 | lemma_wf_assemble_node | Strong | Assembles wf. |
| 4 | size_link | Strong | sz == spec_size_link. |
| 5 | update | Strong | Correct size + key/children preserved. |
| 6 | rotate_left | Partial | wf + size preserved. Missing: BST. |
| 7 | rotate_right | Partial | wf + size preserved. Missing: BST. |
| 8 | insert_link | Partial | wf + bounded size. Missing: contains. |
| 9 | height_link | Strong | h == spec_height_link. |

**Summary:** 6 strong, 3 partial, 0 weak, 18 none.

## Proposed Fixes Table

| # | Sev | Chap | File | Function(s) | Issue | Fix | Deps |
|---|-----|------|------|-------------|-------|-----|------|
| 1 | Critical | 39 | BSTTreapStEph.rs | rotate_left:470 | `assume()` for is_lt transitivity in BST rotation proof. | Prove via lemma establishing transitivity of `is_lt` for `StT + Ord`, or add a `lemma_is_lt_transitive` using `partial_cmp_spec` + `Ordering` semantics. Alternative: strengthen `spec_bst_link` to use `<=` on priorities and prove containment chains. | — |
| 2 | Critical | 39 | BSTTreapStEph.rs | rotate_right | BST property not preserved in ensures. | Add `spec_bst_link(&Some(x)) ==> spec_bst_link(&Some(rotated))` to ensures. Mirror rotate_left proof using the 4 BST lemmas. Same is_lt transitivity blocker as #1. | #1 |
| 3 | Critical | 39 | BSTTreapStEph.rs | insert_link | ensures does not preserve `spec_bst_link` or guarantee `spec_contains_link(key)`. | Add `requires spec_bst_link` + `ensures spec_bst_link, spec_contains_link(&inserted, value)`. Prove via case analysis on cmp + rotation BST specs. | #1, #2 |
| 4 | Critical | 39 | BSTTreapStEph.rs | insert (trait) | ensures missing BST + containment postconditions. | Propagate insert_link spec to trait: `requires spec_bst()` + `ensures spec_bst(), spec_contains_link`. | #3 |
| 5 | Critical | 39 | BSTTreapMtEph.rs | insert_link | Same gap as StEph: no BST/containment postcondition. | Mirror StEph fix. Needs BST lemma infrastructure added to MtEph first. | #3 |
| 6 | ~~High~~ DONE | 39 | BSTTreapStEph.rs | find_link | ~~No spec.~~ Partial spec added. | `ensures found.is_some() ==> spec_contains_link(link, *found.unwrap())`. Forward direction only; reverse blocked by PartialEq spec bridge for generic T. | — |
| 7 | High | 39 | BSTTreapStEph.rs | contains | No spec. Delegates to find. | Add `ensures c == spec_contains_link(link, target)`. | #6 |
| 8 | High | 39 | BSTTreapStEph.rs | min_link, max_link | No spec. | Add ensures relating result to spec ordering. Needs BST invariant. | #3 |
| 9 | High | 39 | BSTTreapStEph.rs | height (trait) | requires only, no ensures. | Add `ensures h as nat == spec_height_link(&self.root)`. Trivial: propagate from height_link. | — |
| 10 | ~~Medium~~ DONE | 39 | BSTTreapStEph.rs | in_order_vec | ~~No spec.~~ Length spec added. | `ensures ordered@.len() == spec_in_order_link(link).len()`. Full structural `=~=` blocked by generic Clone axiom gap. Restructured to use `Vec::append`. | — |
| 11 | ~~Medium~~ DONE | 39 | BSTTreapStEph.rs | pre_order_vec | ~~No spec.~~ Length spec added. | `ensures ordered@.len() == spec_pre_order_link(link).len()`. Same Clone blocker. Restructured to use `Vec::append`. | — |
| 12 | Medium | 39 | BSTTreapMtEph.rs | 11 trait fns | No requires/ensures. Style warning [12]. | Blocked: Arc<RwLock<>> hides spec fn access. May need internal invariant strengthening. | #5 |
| 13 | Medium | 39 | BSTParaTreapMtEph.rs | All 32 fns | Outside verus!. No specs. | Move algorithmic core inside verus!. Large effort. | — |
| 14 | Medium | 39 | BSTSetTreapMtEph.rs | All 22 fns | Outside verus!. No specs. | Blocked on ParaTreap verusification (#13). | #13 |
| 15 | Low | 39 | BSTSetTreapMtEph.rs | — | No TOC header. | Add `// Table of Contents` block. | — |
| 16 | Low | 39 | BSTSetTreapMtEph.rs | — | Section comment `// 6. helper functions` is non-standard. | Rename or remove. | — |
| 17 | Low | 39 | BSTParaTreapMtEph.rs | Clone impls | [15] Clone outside verus!. | Move inside when verusified. | #13 |

### Priority Summary

| Severity | Count | Actionable Now? |
|----------|:-----:|:---------------:|
| Critical | 5 | #1 is the key blocker — is_lt transitivity. Once solved, #2-4 follow. #5 (MtEph mirror) follows #3. |
| High | 3 (1 done) | #6 DONE (partial). #9 (height ensures) is trivial and independent. #7-8 need BST invariant from #3. |
| Medium | 3 (2 done) | #10-11 DONE (partial). #12-14 are larger efforts. |
| Low | 3 | Minor style/testing. |

### Recommended Execution Order

1. **Solve is_lt transitivity** (#1): The `assume()` at line 470 bridges a gap where Verus cannot prove `k.is_lt(&xk) && xk.is_lt(&yk) ==> k.is_lt(&yk)` for generic `T: StT + Ord`. Options: (a) write a `lemma_is_lt_transitive` exploiting `partial_cmp_spec` and `Ordering` semantics; (b) add a `PartialOrdTransitive` bound or use vstd's `TotalOrderSpec`; (c) restructure the proof to avoid chained comparisons.
2. **StEph rotate_right BST spec** (#2): Mirror rotate_left, applying the same transitivity approach.
3. **StEph insert_link BST+contains** (#3, #4): Add `spec_bst_link` + `spec_contains_link` ensures.
4. **StEph height ensures** (#9): Propagate height_link's existing spec to trait. Trivial.
5. ~~**StEph find/contains specs** (#6, #7)~~: #6 DONE (partial: forward containment). #7 (contains) still needs find_link bidirectional spec.
6. **StEph min/max specs** (#8): After BST invariant.
7. ~~**StEph in_order/pre_order specs** (#10, #11)~~: DONE (partial: length preservation). Full structural specs blocked by generic Clone axiom gap.
8. **MtEph mirrors** (#5, #12): After StEph specs are stable, port BST lemmas + specs.
9. **ParaTreap verusification** (#13): Large work item. Independent track.

### Blockers Discovered During #6, #10, #11

| # | Blocker | Affects | Description |
|---|---------|---------|-------------|
| 1 | Generic Clone axiom gap | #10, #11 full specs | `T: Clone` has no spec guaranteeing `clone(x) == x`. `cloned(a, b)` in vstd is trivially true for unspecified Clone. Blocks `ordered@ =~= spec_in_order_link(link)`. |
| 2 | PartialEq spec bridge | #6 reverse direction | For generic `T: PartialEq`, exec `==` does not bridge to spec `==` without `obeys_eq_spec()` being true. Blocks `spec_contains_link(link, *target)`. |
| 3 | is_lt transitivity | #1 (pre-existing) | Verus cannot prove `k.is_lt(&xk) && xk.is_lt(&yk) ==> k.is_lt(&yk)` for generic T. |
