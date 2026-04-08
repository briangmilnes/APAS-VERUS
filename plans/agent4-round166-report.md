# R166 Agent 4 Report: Lift proof patterns Chap42+40

## Summary

Extracted 3 reusable proof lemmas into `TableSpecsAndLemmas.rs` and applied them
across all 3 Table implementation files (StEph, StPer, MtEph), replacing repeated
no-dups, dom-forward, and value-preservation proof blocks.

Chap40 BST files use entirely different proof patterns (rotation, ordering, content)
with no overlap with Table's entries-to-map patterns. No extraction was possible there.

## New lemmas in TableSpecsAndLemmas.rs

| # | Lemma | Purpose |
|---|-------|---------|
| 1 | `lemma_subseq_no_dups` | Monotone subsequence of no-dup seq has no dups |
| 2 | `lemma_subseq_dom_forward` | Subsequence dom ⊆ original dom |
| 3 | `lemma_subseq_value_agrees` | Subsequence map values agree with original map |

## Functions compressed (by lemma calls replacing inline proofs)

| # | Chap | File | Function | Patterns replaced |
|---|------|------|----------|-------------------|
| 1 | 42 | TableStEph.rs | filter | no_dups, dom_forward, value_agrees |
| 2 | 42 | TableStEph.rs | difference | no_dups, value_agrees |
| 3 | 42 | TableStEph.rs | intersection | no_dups |
| 4 | 42 | TableStEph.rs | delete | no_dups, value_agrees |
| 5 | 42 | TableStEph.rs | restrict | no_dups, value_agrees |
| 6 | 42 | TableStEph.rs | subtract | no_dups, value_agrees |
| 7 | 42 | TableMtEph.rs | filter | no_dups, dom_forward, value_agrees |
| 8 | 42 | TableMtEph.rs | intersection | no_dups |
| 9 | 42 | TableMtEph.rs | difference | no_dups, value_agrees |
| 10 | 42 | TableMtEph.rs | restrict | no_dups, value_agrees |
| 11 | 42 | TableMtEph.rs | subtract | no_dups, value_agrees |
| 12 | 42 | TableStPer.rs | filter | no_dups, dom_forward, value_agrees |
| 13 | 42 | TableStPer.rs | intersection | no_dups |
| 14 | 42 | TableStPer.rs | difference | no_dups, value_agrees |
| 15 | 42 | TableStPer.rs | restrict | no_dups, value_agrees |
| 16 | 42 | TableStPer.rs | subtract | no_dups, value_agrees |

## Line counts

| File | Before | After | Delta |
|------|--------|-------|-------|
| TableSpecsAndLemmas.rs | 429 | 540 | +111 |
| TableStEph.rs | 2733 | 2592 | −141 |
| TableStPer.rs | 3031 | 2873 | −158 |
| TableMtEph.rs | 2284 | 2203 | −81 |
| **Total** | **8477** | **8208** | **−269** |

## Chap40 assessment

Chap40 BST{Size,Reduced,KeyValue}StEph files (~5K lines) share BST-specific proof
lemmas (rotate, ordered_assemble, cmp_antisymmetry, cmp_transitivity) that are
duplicated across variants. These could be extracted into a shared `BSTSpecsAndLemmas.rs`,
but they require different type signatures (Size uses `Link<T>`, Reduced uses
`Link<K,V,R>`, KeyValue uses `Link<K,V>`) so the extraction is non-trivial and
would require either separate generic lemmas per arity or a unifying trait.

## Verification

- 5587 verified, 0 errors
- 3776 RTT passed
- 221 PTT passed
