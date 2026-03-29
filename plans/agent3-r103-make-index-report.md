# Agent 3 — R103 Report: Prove DocumentIndex make_index

## Objective

Remove the last Chap44 proof hole: `make_index` was `external_body` because it
called `sort_unstable_by` which has no Verus spec.

## Approach: Table-based insert (no sort)

Rewrote `make_index` to iterate docs, iterate words per doc, and insert each
word into the table using `TableStPer::insert` with a replace-clone combine
closure. When a word already exists, the existing document set is unioned with a
singleton containing the current doc_id via `find_ref` + `union`, then the
result replaces the old entry.

Key proof technique: **ghost subset tracking**. A ghost variable `gds: Set<Seq<char>>`
tracks the set of all doc_id views from processed documents. The invariant
`stored_value@ ⊆ gds` ensures each document set's size is bounded by the number
of documents processed. This maintains the `<= usize::MAX/2` bound required by
`spec_documentindex_wf` without needing membership checks or exact size tracking
per set.

## Changes

| # | Chap | File | Change |
|---|------|------|--------|
| 1 | 44 | DocumentIndex.rs | Rewrote `make_index`: removed `external_body`, `sort_unstable_by`, `Vec`, `AVLTreeSeqStPerS::from_vec`, `AVLTreeSetStPer::from_seq`. New impl uses `find_ref` + `union` + `insert` with ghost subset tracking. |
| 2 | 44 | DocumentIndex.rs | Added requires to `make_index` trait: `docs.spec_len() <= usize::MAX/2`, `obeys_view_eq`, `obeys_feq_full`, `obeys_cmp_spec`, `view_ord_consistent`. |

## Holes Before/After

| # | Chap | File | Before | After |
|---|------|------|--------|-------|
| 1 | 44 | DocumentIndex.rs | 1 | 0 |

## Verification

- **Verified**: 5424 (was ~5420 before)
- **RTT**: 3083 passed
- **PTT**: 157 passed
- **Chapters clean**: 45 of 46 (Chap65 only remaining)
- **Total holes**: 4 (all in Chap65)

## Techniques Used

1. **Ghost subset tracking** — `gds: Set<Seq<char>>` tracks processed doc_ids;
   `lemma_len_subset` bounds stored set sizes via subset relation.
2. **View-based wf transfer** — After `insert` with clone-combine, the new stored
   value has the same view as `new_set`, so `spec_avltreesetstper_wf` transfers
   because it depends only on `self@.finite()` and `self@.len()`.
3. **Axiom propagation** — `obeys_view_eq`, `obeys_cmp_spec`, `view_ord_consistent`
   added to requires since no broadcast proofs exist for `String` in vstd.
4. **Pair_feq_trigger** — Triggers broadcast axiom for `obeys_feq_full::<Pair<Word, DocumentSet>>()`.

## Requires Added to make_index

The original `make_index` had no requires (just `ensures di.spec_documentindex_wf()`).
Five preconditions were added:

- `docs.spec_len() <= usize::MAX/2` — bounds document set sizes.
- `obeys_view_eq::<Word>()` — needed for table `find_ref`/`insert`.
- `obeys_feq_full::<Pair<Word, DocumentSet>>()` — needed for table `insert`.
- `obeys_cmp_spec::<DocumentId>()` — needed for set `union`.
- `view_ord_consistent::<DocumentId>()` — needed for set `union`.

These are standard axiom conditions already required by `find`, `query_and`,
`query_or`, and `query_and_not` in the same trait.
