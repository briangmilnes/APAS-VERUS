# Agent 2 — R103 Report: DocumentIndex tokens

## Objective

Prove `tokens()` by moving it inside `verus!` using vstd string specs and
vstdplus/strings.rs wraps.

## What Was Done

Moved `tokens()` from outside `verus!` (unverified) into the `verus!` block
with a fully verified implementation:

- Uses vstd `str::chars()` with `Chars::next()` in a decreasing loop
- Per-char lowercase via `char_to_ascii_lowercase` (ASCII-only, matching
  the project's ASCII word tokenization semantics)
- Per-char alphabetic check via `char_is_ascii_alphabetic`
- Word building via `string_push`, emit check via `string_is_empty`
- Collects into `Vec<Word>`, converts via `ArraySeqStPerS::from_vec`
- Ensures `words.spec_arrayseqstper_wf()` (proven automatically through
  `from_vec` postcondition)

## Holes Before/After

| # | Chap | File | Before | After | Delta |
|---|------|------|--------|-------|-------|
| 1 | 44 | DocumentIndex.rs | 1 | 1 | 0 |

The hole count stays at 1 because `tokens` was outside `verus!` (invisible
to veracity). The remaining hole is `make_index` (`external_body`), blocked
by `sort_unstable_by` having no Verus spec. However:

- `tokens` is now **verified** (was completely unverified outside verus!)
- The `fn_missing_wf_requires` warning on `QueryBuilder::new` is resolved

## What Blocks make_index

`make_index` calls `sort_unstable_by` on `Vec<(Word, DocumentId)>`. Rust's
`sort_unstable_by` has no Verus `assume_specification`. Options:
1. Write a verified merge sort (Chap36 has one) — significant effort
2. Add an `assume_specification` for `sort_unstable_by` — needs vstd upstream
3. Leave as external_body — current state, honest about the gap

## Verification

- `scripts/validate.sh isolate Chap44`: 2185 verified, 0 errors
- `scripts/rtt.sh`: 3083 passed, 0 skipped

## Commit

`aed9b245c` on `agent2/ready`
