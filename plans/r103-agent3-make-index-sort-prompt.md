# R103 Agent 3 — Prove DocumentIndex make_index (table-based, no sort), STEP 20

## Objective

The last Chap44 hole: `make_index` is external_body because it calls
`sort_unstable_by` which has no Verus spec. Rewrite to avoid sorting entirely.

## Approach: Table-based insert (textbook Algorithm 44.2)

The current code sorts `(word, doc_id)` pairs then groups by word. The textbook
approach is simpler: iterate docs, iterate words per doc, insert each word into
the table. If the word exists, union the document sets. No sort needed.

```rust
fn make_index(docs: &DocumentCollection) -> Self {
    let mut table = TableStPer::empty();
    for i in 0..docs.length() {
        let doc = docs.nth(i);
        let doc_id = doc.0.clone();
        let words = tokens(&doc.1);
        for j in 0..words.length() {
            let word = words.nth(j);
            let singleton = AVLTreeSetStPer::singleton(doc_id.clone());
            table = table.insert(word, singleton, |old_set, new_set| old_set.union(&new_set));
        }
    }
    DocumentIndex { word_to_docs: table }
}
```

Use our ArraySeqStPerS types throughout — no Vec, no conversion.

`tokens` is already verified (agent2 R103). `TableStPer::insert` with combine
is verified. `AVLTreeSetStPer::union` and `singleton` are verified.

## What needs to verify

- Outer loop: iterate docs (ArraySeqStPerS), maintain table wf
- Inner loop: iterate words (ArraySeqStPerS from tokens), maintain table wf
- Combine closure: `|old, new| old.union(&new)` — needs requires/ensures
  that both args are wf and result is wf
- Final: `DocumentIndex { word_to_docs: table }` with `spec_documentindex_wf`

## Read first

- `src/Chap44/DocumentIndex.rs` — current make_index (line 98)
- `src/Chap42/TableStPer.rs` — insert with combine
- `src/Chap41/AVLTreeSetStPer.rs` — singleton, union

## Isolation

```bash
scripts/validate.sh isolate Chap44
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT use Vec. Use ArraySeqStPerS throughout.
- Do NOT use sort_unstable_by or any Rust stdlib sort.
- Do NOT add assume or accept.
- `tokens` returns `ArraySeqStPerS<Word>` — use it directly.
- The combine closure needs explicit requires/ensures for Verus.
  Read `src/standards/using_closures_standard.rs`.

## STEP 20

## Report

Write `plans/agent3-r103-make-index-report.md`.
