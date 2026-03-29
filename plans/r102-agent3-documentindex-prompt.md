# R102 Agent 3 — Move DocumentIndex into verus!, add requires/ensures, STEP 20

## Objective

Most of `src/Chap44/DocumentIndex.rs` is outside `verus!` for no good reason.
Move the trait, impl, and QueryBuilder code inside `verus!`. Add requires/ensures.

## What to move inside verus!

### 1. DocumentIndexTrait (line 53-93)

Move the entire trait inside `verus!`. Add requires/ensures:

- `find`: requires `self.word_to_docs.spec_tablestper_wf()`, ensures match on found/not-found
- `query_and/or/and_not`: requires both args wf, ensures result wf
- `size`: requires wf, ensures count == docs@.len()
- `empty`: ensures result wf
- `make_index`: keep `external_body` (uses String::to_lowercase, chars, sort_unstable_by)

### 2. DocumentIndexTrait impl (line 95-191)

Move inside `verus!`. Most functions are one-line delegations:
- `find` → `self.word_to_docs.find(word)` — should verify trivially
- `query_and` → `docs_a.intersection(docs_b)` — trivial
- `query_or` → `docs_a.union(docs_b)` — trivial
- `query_and_not` → `docs_a.difference(docs_b)` — trivial
- `size` → `docs.size()` — trivial
- `empty` → `TableStPer::empty()` — trivial
- `word_count` → `self.word_to_docs.size()` — trivial
- `make_index` → `external_body` (String ops)
- `to_seq` → loop, needs invariant but straightforward
- `get_all_words` → loop, same

### 3. QueryBuilderTrait + impl (line 276-331)

Move inside `verus!`. All functions are one-line delegations to DocumentIndexTrait.
Should verify trivially once DocumentIndexTrait has ensures.

### 4. tokens() (line 196-218)

Keep outside `verus!` OR wrap in `external_body`. Uses `String::to_lowercase()`,
`chars()`, `char::is_alphabetic()` — Rust stdlib without Verus specs.

### 5. create_finder (line 223-225)

Keep outside `verus!` — returns `impl Fn` with lifetime capture.

## What stays outside verus!

- `tokens()` — String/char processing
- `create_finder()` — impl Fn return
- `Display`, `Debug` impls — standard pattern
- `DocumentCollectionLit!` macro — macros are always outside

## Add a wf predicate

```rust
pub open spec fn spec_documentindex_wf(&self) -> bool {
    self.word_to_docs.spec_tablestper_wf()
}
```

Add to the struct impl inside verus!.

## Read first

- `src/Chap44/DocumentIndex.rs` — your file
- `src/standards/mod_standard.rs` — module structure
- `src/standards/table_of_contents_standard.rs` — TOC ordering

## Isolation

```bash
scripts/validate.sh isolate Chap44
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT add assume or accept.
- `external_body` on make_index and tokens is fine — String ops aren't verifiable.
- The goal: every function that CAN verify DOES verify. Only String-processing
  stays external_body.
- Add loop invariants to to_seq and get_all_words if you move them inside verus!.
  Or use external_body on those too if the loop invariant is too complex.
- Update the TOC when restructuring.

## STEP 20

## Report

Write `plans/agent3-r102-documentindex-report.md`.
