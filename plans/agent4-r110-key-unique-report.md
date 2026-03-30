# Agent 4 R110 Report: Remove opaque from spec_key_unique_pairs_set

## Objective

Remove `#[verifier::opaque]` from `spec_key_unique_pairs_set` in both
`OrderedTableStEph.rs` and `OrderedTableStPer.rs`. These were 2 of the 7
remaining holes in the entire project.

## The Problem

The flat quantifier form:
```rust
forall|k: KV, v1: VV, v2: VV|
    s.contains((k, v1)) && s.contains((k, v2)) ==> v1 == v2
```

has symmetric triggers `s.contains((k, v1))` and `s.contains((k, v2))`. Z3 uses
these as a multi-trigger: every pair of `contains` terms with the same first
component fires the quantifier. Each instantiation can produce equalities that
merge terms, generating new trigger matches — a classic symmetric matching loop
that caused 91K instantiations in R105.

## The Fix: Nested Quantifiers

Restructured the quantifier to nest the second universal inside the first:

```rust
forall|k: KV, v: VV| #[trigger] s.contains((k, v)) ==>
    forall|v2: VV| s.contains((k, v2)) ==> v == v2
```

**Why this works:**
- The outer trigger is a single `s.contains((k, v))` — no symmetry.
- Each outer instantiation produces an inner quantifier scoped to a specific `k`.
- The inner quantifier fires against existing `contains` terms only — it doesn't
  produce new `contains` terms that feed back to the outer quantifier.
- Total instantiations: bounded by O(n²) where n = number of `contains` terms in
  the e-graph. No self-feeding loop.

**Semantically equivalent:** The nested form says exactly the same thing as the
flat form — if a pair (k, v) is in the set, then every other pair (k, v2) in the
set must have v == v2.

## Changes

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 43 | OrderedTableStEph.rs | Removed `#[verifier::opaque]`, restructured quantifier, removed 12 `reveal` calls |
| 2 | 43 | OrderedTableStPer.rs | Same: removed opaque, restructured quantifier, removed 9 `reveal` calls |

## Why All Proofs Still Pass

Every proof lemma that previously used `reveal(spec_key_unique_pairs_set)` falls
into one of two patterns:

1. **Lemmas that assert the flat form explicitly** (e.g., `lemma_key_unique_insert`,
   `lemma_key_unique_disjoint_union`, the `make_index` loop invariant). These
   assert `forall|k, v1, v2| ... ==> v1 == v2` as a local step. Z3 can derive
   the nested definition from this flat assertion.

2. **Lemmas that rely on Z3 to derive key-uniqueness consequences** (e.g.,
   `lemma_key_unique_remove`, `lemma_key_unique_subset`, `lemma_key_unique_empty`,
   `lemma_sorted_keys_pairwise_distinct`). The nested form is directly available
   (no longer hidden behind opaque), so Z3 can use it without reveal.

## Verification Results

- **Full validation:** 5433 verified, 0 errors
- **RTT:** 3083 passed
- **PTT:** 157 passed
- **Chap43 holes:** 0 (was 2)

## Holes Before/After

| # | Chap | File | Before | After |
|---|------|------|--------|-------|
| 1 | 43 | OrderedTableStEph.rs | 1 (opaque) | 0 |
| 2 | 43 | OrderedTableStPer.rs | 1 (opaque) | 0 |

**Project total: 7 → 5 holes remaining.**
