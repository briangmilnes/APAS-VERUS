# Agent 8 — Round 171 Report

## Objective

Prove `lemma_combined_cycle` in `src/Chap26/ETSPMtEph.rs`, removing its
`#[verifier::external_body]`.

## Hole delta

| # | Chap | File | Holes Before | Holes After | Delta |
|---|------|------|-------------|-------------|-------|
| 1 | 26 | ETSPMtEph.rs | 1 | 0 | −1 |

## Chapters closed

None (Chap26 had other holes from before; this removes the one assigned).

## Verification

`scripts/validate.sh isolate Chap26` → **1176 verified, 0 errors**.

## Technique

The `lemma_combined_cycle` proof in MtEph was marked `external_body` with the
comment "Z3 matching loop on spec_edges_form_cycle modular indexing trigger."
ETSPStEph.rs already has a full proof of the same lemma. The fix required:

1. **Add `spec_next_edge_from` (closed spec)** between `spec_edges_valid` and
   `spec_edges_form_cycle`. A closed spec prevents Z3 from chaining `tour[i]`
   triggers through `tour[(i+1)%n]`, which was the matching loop root cause.

2. **Update `spec_edges_form_cycle`** to reference `spec_next_edge_from` instead
   of the inline modular index expression.

3. **Add `lemma_next_edge_from_eq`** — a point-reveal helper that discloses
   `spec_next_edge_from` at a single index without re-opening the matching loop.

4. **Fix base case proofs** (n==2, n==3) to call `reveal(spec_next_edge_from)`
   since the closed spec is now opaque to Z3. The n==3 case also got the
   conjunction flakiness fix (assert each conjunct, then the bundle) matching the
   StEph pattern.

5. **Copy proof body** from ETSPStEph.rs `lemma_combined_cycle` (92 lines) into
   MtEph verbatim — the two files share identical types (`Point`, `Edge`, `Seq`)
   and identical lemma signatures, so no adaptation was needed.

6. **Remove `#[verifier::external_body]`** and the `// BYPASSED:` comment.

## Remaining holes

No holes were assigned beyond `lemma_combined_cycle`. The f64 sort/split
`external_body` in ETSPMtEph.rs (around line 519) was left untouched per prompt.
