# Agent 2 R103 Report: DocumentIndex String Specs

## Objective

Test new Verus string specs on Chap44 DocumentIndex. Prove the 4 external_body holes.

## Results

| # | Chap | File | Function | Before | After | Technique |
|---|------|------|----------|--------|-------|-----------|
| 1 | 44 | DocumentIndex.rs | to_seq | external_body | proved | Vec+from_vec loop |
| 2 | 44 | DocumentIndex.rs | get_all_words | external_body | proved | Vec+from_vec loop |
| 3 | 44 | DocumentIndex.rs | complex_query | external_body | proved | Strengthened find ensures + set lemmas |
| 4 | 44 | DocumentIndex.rs | make_index | external_body | external_body | Blocked: no String specs |

**Holes: 4 -> 1** (-3 holes)

## What was done

### to_seq and get_all_words (2 holes removed)

Both functions were simple accumulation loops: iterate a source collection, clone each element,
append to an output ArraySeqStPerS via singleton+append. The external_body existed because
the loop invariants hadn't been written.

**Fix:** Replaced the singleton+append pattern with Vec::push+ArraySeqStPerS::from_vec.
This avoids `obeys_feq_clone` requirements (from_vec has no requires). Minimal while-loop
invariants suffice since `spec_arrayseqstper_wf()` is always true.

### complex_query (1 hole removed)

complex_query chains `find -> and -> and_not -> or`. The external_body existed because
`find` didn't ensure `spec_avltreesetstper_wf()` on its result, and `or` requires
`docs_a@.len() + docs_b@.len() < usize::MAX`.

**Fix (3 parts):**

1. **Strengthened `spec_documentindex_wf`** to require all stored DocumentSets are
   well-formed with bounded size (`ds@.len() <= usize::MAX / 2`). This is a reasonable
   constraint: each word maps to fewer than 2^63 documents.

2. **Changed `find` implementation** from `Table::find` (returns clone, no wf ensures) to
   `Table::find_ref` + `ClonePreservesWf::clone_wf` (returns reference to stored value,
   then wf-preserving clone). Added `ensures found.spec_avltreesetstper_wf(),
   found@.len() <= usize::MAX / 2` to both DocumentIndexTrait::find and
   QueryBuilderTrait::find.

3. **Proved `or` length bound** using vstd lemmas `lemma_len_intersect` (|A intersect B| <= |A|)
   and `lemma_len_difference` (|A \ B| <= |A|). Since find results are <= usize::MAX/2,
   intersection and difference results are also <= usize::MAX/2, so their sum < usize::MAX.

### make_index (blocked)

make_index uses `String::to_lowercase()`, `char::is_alphabetic()`, `String::push(char)`,
and `Vec::sort_unstable_by()`. None of these have Verus specs in vstd/string.rs.

The new Verus string specs (PR #2238) added:
- `str::chars()` with `Chars` iterator and ForLoopGhostIterator
- `String::new()`, `String::clone()`, `String::eq()`, `String::as_str()`
- `str` basics: `len`, `is_ascii`, `char_boundary`, `split_at`

Missing for make_index/tokens:
- `String::to_lowercase()` — no spec
- `char::is_alphabetic()` — no spec
- `String::push(char)` — no spec
- `String::is_empty()` for String — no spec
- `Vec::sort_unstable_by()` — no spec

## Verification

- Full validation: 5420 verified, 0 errors
- RTT: 3083 passed
- PTT: 157 passed

## Steps used

4 edit iterations, 5 validate runs (including 1 OOM retry on full validate).
