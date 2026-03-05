<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Chapter 39 — Treaps: Review Against Prose

**Date:** 2026-03-05 (updated)
**Reviewer:** Claude-Opus-4.6
**Previous reviews:** 2026-02-19, 2026-02-27, 2026-02-28, 2026-03-05

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

From `veracity-review-proof-holes -d src/Chap39/` (2026-03-05):

```
Modules:
   1 clean (no holes)
   3 holed (contains holes)
   4 total

Proof Functions:
   7 clean
   0 holed
   7 total

Holes Found: 41 total
   41 × external_body

Info (5):
  BSTParaTreapMtEph.rs:83  — verus_rwlock_external_body (expected, unfixable)
  BSTTreapMtEph.rs:301     — accept() for recursive type structural equality
  BSTTreapMtEph.rs:314     — accept() for recursive type structural equality
  BSTTreapMtEph.rs:326     — verus_rwlock_external_body (expected, unfixable)
  BSTTreapMtEph.rs:1065    — accept() for recursive type structural equality
```

**BSTTreapStEph is fully clean** — the is_lt transitivity blocker was resolved via `IsLtTransitive` trait bound. No assume() or external_body in StEph. The 41 external_body holes are in MtEph (13), ParaTreapMtEph (18), and SetTreapMtEph (10) — all at threading/RwLock boundaries.

## Verus Style Review Summary

From `veracity-review-verus-style` (2026-03-05): **66 passed, 6 warnings** across 4 files.

| # | File | Warnings | Details |
|---|------|:--------:|---------|
| 1 | BSTTreapStEph | 1 | [22] free spec fn `spec_contains_link` (must stay free: `reveal_with_fuel` limitation) |
| 2 | BSTTreapMtEph | 5 | [22] 5 free spec fns should be in trait (deferred to NodeTrait design) |
| 3 | BSTParaTreapMtEph | 0 | All checks pass |
| 4 | BSTSetTreapMtEph | 0 | All checks pass |

The [22] warnings on StEph/MtEph are documented: `spec_contains_link` must remain free due to `reveal_with_fuel` limitation; the 4 other MtEph free spec fns will move to a `NodeTrait` when designed.

## Verusification Table

| # | Module | Inside V! | Spec Coverage | Proof Holes | Proof Fns | Spec Fns | Status |
|---|--------|:---------:|:-------------:|:-----------:|:---------:|:--------:|--------|
| 1 | BSTTreapStEph | 35/35 | 35/35 (100%) | 0 | 8 | 7+9 trait | Fully verified: BST + containment + wf |
| 2 | BSTTreapMtEph | 34/34 | 22/34 (65%) | 13 ext_body | 7 | 5 free | Structurally verified + BST proofs |
| 3 | BSTParaTreapMtEph | 18/33 (55%) | 18/33 (55%) | 18 ext_body | 0 | 0 | Threading boundary external_body |
| 4 | BSTSetTreapMtEph | 22/22 (100%) | 12/22 (55%) | 10 ext_body | 0 | 0 | Shim delegating to ParaTreap |
| | **Totals** | **109/124** | **87/124 (70%)** | **41 ext_body** | **15** | **21** | |

### Spec Strength Classification

**BSTTreapStEph** (35 specified functions — all functions have specs):

| # | Function | Strength | Notes |
|---|----------|:--------:|-------|
| 1 | new | Strong | Ensures size==0, wf, bst. |
| 2 | size | Strong | Ensures sz == spec_size(). |
| 3 | is_empty | Strong | Ensures empty == (size==0). |
| 4 | height | Strong | Ensures h == spec_height(). |
| 5 | insert | Strong | Ensures wf, bounded size, containment (both dirs), BST preserved. |
| 6 | delete | Strong | Ensures wf, bounded size, containment preserved, BST preserved. |
| 7 | find | Partial | Forward containment only. Reverse blocked by PartialEq spec bridge. |
| 8 | contains | Partial | Existential: `found ==> exists v, spec_contains(v)`. Blocked by PartialEq. |
| 9 | minimum | Strong | Ensures match with spec_min_link. |
| 10 | maximum | Strong | Ensures match with spec_max_link. |
| 11 | in_order | Partial | Length only. Full structural blocked by Clone axiom gap. |
| 12 | pre_order | Partial | Length only. Same Clone blocker. |
| 13 | new_node | Strong | Ensures wf, size==1. |
| 14 | size_link | Strong | Ensures sz == spec_size_link. |
| 15 | update_size | Strong | Ensures correct size + key/children unchanged. |
| 16 | rotate_left | Strong | Ensures wf + size + BST + containment biconditional. No assume. |
| 17 | rotate_right | Strong | Ensures wf + size + BST + containment biconditional. |
| 18 | clone_link | Partial | Ensures size preserved. Missing: structural eq. |
| 19 | height_link | Strong | Ensures h == spec_height_link. |
| 20 | insert_link | Strong | Ensures wf + size bounds + containment (both dirs) + BST. |
| 21 | delete_link | Strong | Ensures wf + size bounds + containment + BST. |
| 22 | find_link | Partial | Forward containment only. |
| 23 | min_link | Strong | Ensures match with spec_min_link. |
| 24 | max_link | Strong | Ensures match with spec_max_link. |
| 25 | in_order_vec | Partial | Length preservation only. |
| 26 | pre_order_vec | Partial | Length preservation only. |
| 27-34 | 8 proof lemmas | Strong | All clean, no holes. |
| 35 | default | Strong | Ensures size==0, wf, bst. |

**Summary:** 27 strong, 8 partial (find/contains: PartialEq bridge; in_order/pre_order: Clone gap; clone_link: structural eq), 0 weak, 0 none.

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

All critical and high-severity items from previous reviews have been resolved. Remaining items:

| # | Sev | Chap | File | Function(s) | Issue | Status |
|---|-----|------|------|-------------|-------|--------|
| 1 | ~~Critical~~ | 39 | BSTTreapStEph.rs | rotate_left | ~~assume() for is_lt transitivity~~ | DONE — resolved via `IsLtTransitive` trait bound. |
| 2 | ~~Critical~~ | 39 | BSTTreapStEph.rs | rotate_right | ~~Missing BST preservation~~ | DONE — BST + containment biconditional in ensures. |
| 3 | ~~Critical~~ | 39 | BSTTreapStEph.rs | insert_link | ~~Missing BST/containment~~ | DONE — full containment (both dirs) + BST in ensures. |
| 4 | ~~Critical~~ | 39 | BSTTreapStEph.rs | insert (trait) | ~~Missing BST/containment~~ | DONE — propagated from insert_link. |
| 5 | Medium | 39 | BSTTreapMtEph.rs | insert_link | MtEph has BST proofs but trait fns lack specs due to Arc<RwLock<>>. | Open |
| 6 | ~~High~~ | 39 | BSTTreapStEph.rs | find_link | ~~No spec~~ | DONE — partial (forward containment). Reverse blocked by PartialEq bridge. |
| 7 | Medium | 39 | BSTTreapStEph.rs | contains | Weak existential spec. | Blocked by PartialEq spec bridge. |
| 8 | ~~High~~ | 39 | BSTTreapStEph.rs | min_link, max_link | ~~No spec~~ | DONE — match with spec_min_link/spec_max_link. |
| 9 | ~~High~~ | 39 | BSTTreapStEph.rs | height | ~~No ensures~~ | DONE — ensures h == spec_height(). |
| 10 | ~~Medium~~ | 39 | BSTTreapStEph.rs | in_order_vec | ~~No spec~~ | DONE — length preservation. Full structural blocked by Clone gap. |
| 11 | ~~Medium~~ | 39 | BSTTreapStEph.rs | pre_order_vec | ~~No spec~~ | DONE — length preservation. Same Clone blocker. |
| 12 | Medium | 39 | BSTTreapMtEph.rs | trait fns | No requires/ensures on trait methods. | Blocked by Arc<RwLock<>> hiding spec fn access. |
| 13 | Medium | 39 | BSTParaTreapMtEph.rs | All fns | Outside verus! with external_body. | Large effort — threading boundary. |
| 14 | Medium | 39 | BSTSetTreapMtEph.rs | All fns | external_body delegations. | Blocked on #13. |
| 15 | ~~Low~~ | 39 | BSTSetTreapMtEph.rs | — | ~~No TOC header~~ | DONE — TOC present. |
| 16 | Medium | 39 | Both St/Mt | free spec fns | [22] style warnings: 6 free spec fns should be in traits. | `spec_contains_link` must stay free (reveal_with_fuel). Others deferred to NodeTrait design. |
| 17 | HIGH | 39 | Both St/Mt | `#![auto]` triggers | 24 `#![auto]` annotations in insert_link/delete_link. | DONE — replaced with explicit `#[trigger]` (2026-03-05). |

### Remaining Blockers

| # | Blocker | Affects | Description |
|---|---------|---------|-------------|
| 1 | Generic Clone axiom gap | in_order/pre_order full specs | `T: Clone` has no spec guaranteeing `clone(x) == x`. Blocks structural `=~=`. |
| 2 | PartialEq spec bridge | find/contains bidirectional | For generic `T: PartialEq`, exec `==` does not bridge to spec `==` without `obeys_eq_spec()`. |

### Remaining Work

| # | Priority | Task | Effort |
|---|----------|------|--------|
| 1 | Medium | Design and implement NodeTrait (multi-struct spec style) for Link/Node | Large |
| 2 | Medium | Strengthen MtEph trait specs (blocked by Arc<RwLock<>> design) | Large |
| 3 | Medium | ParaTreap verusification — move algorithmic core inside verus! | Large |
| 4 | Low | Resolve PartialEq bridge for find/contains bidirectional specs | Blocked by Verus |
| 5 | Low | Resolve Clone axiom gap for full structural traversal specs | Blocked by Verus |
