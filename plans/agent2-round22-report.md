# Agent 2 — Round 22 Report

## Mission

Prove holes across Chap05, Chap38, Chap39, Chap40 (4 chapters, 9 files).

## Results

| # | Chap | File | Holes Before | Holes After | Proven | Technique |
|---|------|------|:------------:|:-----------:|:------:|-----------|
| 1 | 05 | MappingStEph.rs | 1 | 0 | 1 | `lemma_map_size` with injective projection |
| 2 | 38 | BSTParaStEph.rs | 5 | 5 | 0 | Blocked: Clone bridge, size bounds |
| 3 | 38 | BSTParaMtEph.rs | 9 | 9 | 0 | Blocked: View external_body (root) |
| 4 | 39 | BSTTreapStEph.rs | 2 | 2 | 0 | Blocked: structural-vs-runtime equality |
| 5 | 39 | BSTTreapMtEph.rs | 6 | 6 | 0 | Blocked: Clone bridge, Mt wrapper gaps |
| 6 | 39 | BSTParaTreapMtEph.rs | 10 | 10 | 0 | Blocked: View external_body (root) |
| 7 | 40 | BSTKeyValueStEph.rs | 5 | 5 | 0 | Blocked: weak _link specs, equality gap |
| 8 | 40 | BSTReducedStEph.rs | 5 | 5 | 0 | Blocked: weak _link specs, ReduceOp uninterp |
| 9 | 40 | BSTSizeStEph.rs | 4 | 4 | 0 | Blocked: weak _link specs, equality gap |
| | | **Total** | **47** | **46** | **1** | |

## Chapters Closed

- **Chap05**: 0 holes (was 1). Chapter fully clean.

## Verification

- validate: 3958 verified, 0 errors
- RTT: 2600 passed
- PTT: 147 passed

## Proven Hole: Chap05 MappingStEph::size

Removed `external_body` from `size()` in `src/Chap05/MappingStEph.rs`.

**Proof strategy**: The relation is functional (each domain element maps to at most one
range element), so projecting pairs to first coordinates is injective. Applied
`vstd::set_lib::lemma_map_size` with `proj = |p: (X::V, Y::V)| p.0` to establish
`self.mapping@.len() == self@.dom().len()`.

## Blocker Analysis

All remaining holes share one or more of five root causes:

### 1. Structural-vs-Runtime Equality Gap (Chap38/39/40 StEph)

`spec_contains_link` and `spec_content_link` use spec-level structural `==` on type T.
Runtime `find`/`insert` use `PartialEq::eq` and `PartialOrd::lt`. Neither direction is
provable for generic T:
- `PartialEq::eq(a, b) ⇏ a == b` (structural) — runtime eq doesn't imply spec eq
- `a == b (structural) ⇏ PartialEq::eq(a, b)` — spec eq doesn't imply runtime eq

Attempted: removed `external_body` from `BSTTreapStEph::insert_link`. Verus proved 6/7
ensures but failed on `spec_contains_link(&inserted, value)` in the duplicate-key case
(where runtime `==` is true but spec structural `==` is unestablished).

**Fix**: Add a trait bound or type class guaranteeing `PartialEq::eq(a, b) <==> (a == b)`.

### 2. Clone Bridge (Chap38/39 all files)

`Clone::clone` has no verified `ensures` in Verus. After `let k = node.key.clone()`,
Verus can't prove `k@ == node.key@`. The `assume(k@ == node.key@)` workaround is the
standard Clone bridge pattern.

### 3. Size Bounds (Chap38 BSTParaStEph)

Three `assume(left@.len() + right@.len() < usize::MAX)` in insert/delete/union.
`join_mid` computes `1 + ls + rs` as `usize`; overflow requires a size bound. Current
`spec_bstparasteph_wf` is just `self@.finite()` — no usize bound.

**Fix**: Add `self@.len() < usize::MAX as nat` to `spec_bstparasteph_wf`.

### 4. Mt View External_Body (Chap38/39 MtEph)

`BSTParaMtEph::view()` and `BSTParaTreapMtEph::view()` are `external_body` returning
dummy `Set::empty()`. All downstream accepts (new, singleton, expose, size, find) are
consequences. Need proper View connecting RwLock inner state to ghost tracking.

### 5. Weak _link Helper Specs (Chap40 all files)

`insert_link`, `find_link`, `delete_link` have size-only ensures — no content
(`spec_content_link`) ensures. The top-level trait functions need content ensures
(`self@ == old(self)@.insert(value)` etc.), creating the gap that `external_body` bridges.

**Fix**: Strengthen all _link function ensures to track content changes. Requires solving
blocker #1 first.
