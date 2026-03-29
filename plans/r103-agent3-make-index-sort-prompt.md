# R103 Agent 3 — Prove DocumentIndex make_index (replace sort_unstable_by), STEP 20

## Objective

The last Chap44 hole: `make_index` is external_body because it calls
`sort_unstable_by` which has no Verus spec. Replace it with a verified
sort or an assume_specification bridge.

## Option A: assume_specification for sort_unstable_by (quick)

Write in `vstdplus/strings.rs` or a new `vstdplus/vec_sort.rs`:

```rust
#[verifier::external_body]
pub fn vec_sort_by<T, F: FnMut(&T, &T) -> core::cmp::Ordering>(v: &mut Vec<T>, compare: F)
    ensures v@.to_multiset() =~= old(v)@.to_multiset()
{
    v.sort_unstable_by(compare);
}
```

The ensures says: output is a permutation of input. We could add sorted
ensures too if we have a spec for the comparator, but permutation alone
may be enough for make_index (it groups by word, and grouping only needs
equal elements adjacent, which sorted gives).

## Option B: Use Chap36 QuickSort (verified, more work)

Convert `Vec<(Word, DocumentId)>` to `ArraySeqStEphS`, sort with
`QuickSortStEph::quick_sort_first`, convert back. Needs `TotalOrder`
impl for `(Word, DocumentId)` — lexicographic on (String, String).

This is more work but eliminates the external_body entirely.

## Option C: Restructure make_index to avoid sort

The sort groups words. Alternative: use a Table directly — insert each
(word, doc_id) pair into the table. Table::insert with combine handles
duplicates. No sort needed. This is actually closer to the textbook's
Algorithm 44.2 which uses a table-based approach.

## Recommendation

Try Option C first — it's the cleanest and textbook-aligned. make_index
becomes: iterate docs, iterate words per doc, for each word insert into
table with combine that unions the document set. No sort. No external_body.

If Option C is too complex, fall back to Option A (assume_specification).

## Read first

- `src/Chap44/DocumentIndex.rs` — make_index (line 98)
- `src/Chap42/TableStPer.rs` — insert with combine

## Isolation

```bash
scripts/validate.sh isolate Chap44
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT add assume or accept in algorithmic code.
- An assume_specification on sort_unstable_by (Option A) is acceptable —
  same category as external_body bridges on std library functions.
- Option C (no sort) is preferred if it works.

## STEP 20

## Report

Write `plans/agent3-r103-make-index-report.md`.
