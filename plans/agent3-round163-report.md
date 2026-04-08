# Agent 3 Round 163 Report — Compress Table Proof Functions

## Summary

Extracted shared proof infrastructure from TableStEph, TableStPer, and TableMtEph
into a new `src/Chap42/TableCommon.rs` module. All three files now `pub use` from
TableCommon, maintaining API compatibility for downstream consumers (Chap43, Chap52).

## Approach

All three Table files had identical generic spec functions and proof lemmas operating
on `Seq<(KV, VV)>` / `Map<KV, VV>`. These are pure math about sequence-to-map
conversion — not algorithm-specific. Extracting them to a shared module eliminates
triple duplication without violating the standalone rule (TableCommon is a shared
utility, not St/Mt/Per importing from each other).

## Lemmas Created in TableCommon

| # | Chap | File | Function | Kind | Lines | Callers |
|---|------|------|----------|------|-------|---------|
| 1 | 42 | TableCommon.rs | spec_entries_to_map | spec | 10 | StEph, StPer, MtEph, Chap52 |
| 2 | 42 | TableCommon.rs | spec_keys_no_dups | spec | 4 | StEph, StPer, MtEph, Chap43, Chap52 |
| 3 | 42 | TableCommon.rs | lemma_entries_to_map_finite | proof | 7 | StEph, StPer, MtEph |
| 4 | 42 | TableCommon.rs | lemma_entries_to_map_key_in_seq | proof | 12 | StEph, StPer, MtEph |
| 5 | 42 | TableCommon.rs | lemma_entries_to_map_contains_key | proof | 11 | StEph, StPer, MtEph |
| 6 | 42 | TableCommon.rs | lemma_entries_to_map_len | proof | 16 | StEph, StPer, MtEph |
| 7 | 42 | TableCommon.rs | lemma_entries_to_map_no_key | proof | 6 | StEph, StPer, MtEph |
| 8 | 42 | TableCommon.rs | lemma_entries_to_map_get | proof | 15 | StEph, StPer, MtEph |
| 9 | 42 | TableCommon.rs | lemma_entries_to_map_dom_subset | proof | 11 | StEph, StPer |
| 10 | 42 | TableCommon.rs | lemma_entries_to_map_dom_same_keys | proof | 12 | StEph, StPer, MtEph |
| 11 | 42 | TableCommon.rs | lemma_entries_to_map_subseq_value | proof | 105 | MtEph |
| 12 | 42 | TableCommon.rs | lemma_entries_to_map_skip_prefix | proof | 38 | MtEph |
| 13 | 42 | TableCommon.rs | lemma_entries_to_map_ignore_suffix | proof | 18 | MtEph |
| 14 | 42 | TableCommon.rs | lemma_entries_to_map_agree_on_key | proof | 28 | MtEph |

## Lines Removed per File

| # | Chap | File | Lines Before | Lines After | Delta |
|---|------|------|-------------|------------|-------|
| 1 | 42 | TableStEph.rs | 2944 | 2731 | −213 |
| 2 | 42 | TableStPer.rs | 3255 | 3029 | −226 |
| 3 | 42 | TableMtEph.rs | 2677 | 2282 | −395 |
| 4 | 42 | TableCommon.rs | 0 | 429 | +429 |
| | | | | **Net** | **−405** |

## Verification

- Full validation: **5726 verified, 0 errors**
- RTT: **3776 pass**
- No new warnings or trigger notes

## Technique

Used compact MtEph versions of lemmas as base for TableCommon (fewer verbose assert
steps than StEph/StPer versions). The compact versions verify cleanly because the
broadcast groups provide the same SMT context. The more verbose StEph/StPer versions
had extra assert steps that were helpful locally but not structurally necessary.
