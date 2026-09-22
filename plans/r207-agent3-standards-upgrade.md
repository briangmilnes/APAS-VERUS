# r207 agent 3 — bring `src/standards/` and their PTTs to Verus 0.2026.09.13

Read `plans/r207-round-plan.md` first and obey its common rules. Start only
after `docs/NameResolutionUpgrade.md` reports that no `E0425`, `E0432`, or
`E0407` remains.

## Goal

Every file in `src/standards/` and every PTT in
`rust_verify_test/tests/standards/` verifies on 09.13 and shows the current
vstd shape. Iteration is the priority.

## Iteration facts at 09.13

- `IteratorSpecImpl` declares exactly: `obeys_prophetic_iter_laws`,
  `remaining` (prophetic), `will_return_none` (prophetic), `decrease`, `peek`.
  `initial_value_relation` does not exist (`#2739`, Aug 1).
- Optional: `ExactSizeIteratorSpecImpl::exact_len`,
  `DoubleEndedIteratorSpecImpl::peek_back`.
- For-loop state: `it.index()`, `it.seq()` (prophetic), `it.history()`,
  `it.wf()`. `seq()` may not appear in `decreases`.
- Reference: `~/projects/verus/examples/guide/iterators.rs`,
  `~/projects/verus/source/vstd/std_specs/iter.rs`,
  `~/projects/verus/source/docs/guide/src/iterator-specs*.md`,
  `~/projects/verus/source/rust_verify_test/tests/iterators.rs`.
- The 05.21 pilot: `src/standards/prophetic_iterators_standard.rs`,
  `rust_verify_test/tests/standards/Proveprophetic_iterators_standard.rs`,
  `docs/PropheticIterators.md`, and the three
  `src/experiments/prophetic_iter_*.rs` files. All declare six spec fns.

## Files and expected work

| # | File | Expected change |
|---|------|-----------------|
| 1 | `prophetic_iterators_standard.rs` | remove `initial_value_relation`; check the constructor `ensures` triple against the guide; verify |
| 2 | `Proveprophetic_iterators_standard.rs` (PTT) | same; all seven loop forms verify |
| 3 | `iterators_standard.rs` | old 10-component model. Rewrite to the delegated style (return `std::slice::Iter`); keep the same example collection and the same `iter()` cost |
| 4 | `wrapping_iterators_standard.rs` | old model. Decide with evidence whether wrapping is still needed at 09.13 (an adaptor over a std iterator implementing `IteratorSpecImpl` by delegation); rewrite or mark as superseded by the prophetic standard |
| 5 | `iterator_ptt_standard.rs` | the six PTT patterns must use `index()`/`seq()` |
| 6 | `view_standard.rs`, `deep_view_standard.rs`, `mod_standard.rs`, `table_of_contents_standard.rs` | each embeds an old-model iterator (2–3 errors each in May); replace with the delegated style |
| 7 | `finite_sets_standard.rs` | its premise (finiteness carried by wf) is false at 09.13. Rewrite: finiteness is by type; `finite()` is deprecated and must not be written; `Set::new` returns `Option`; show `filter` over an enclosing set and the `lemma_set_new_some` route |
| 8 | `spec_wf_standard.rs`, `using_hashmap_standard.rs`, and any other standard that writes `.finite()` | remove the conjunct; keep the rest |
| 9 | all remaining standards | verify unchanged; list any that fail and why |
| 10 | `docs/PropheticIterators.md`, `docs/APAS-VERUSIterators.rs`, `.cursor/rules/apas-verus/collection-iterators.mdc`, the CLAUDE.md "Collection Iterator Standard" section | update the text to the 09.13 shape; do not edit CLAUDE.md sections outside that one |

## Procedure

1. `scripts/validate.sh isolate` does not cover standards; use
   `scripts/validate-path.sh` on each standards file, or `scripts/validate.sh dev_only`
   if it includes standards (read the script). Read every log.
2. Edit one file, validate it, read the log, then the next. Record each
   `N verified, M errors` line.
3. Run `scripts/ptt.sh` once at the end; read the log.
4. Do not touch `src/ChapNN/`. If a standard depends on a chapter file that
   does not compile yet, say so in the doc and leave the standard at its best
   state.

## Deliverable: `docs/StandardsUpgrade.md`

1. Table: Chap (write `std`), file, status before, status after (`N verified,
   M errors`), what changed.
2. The 09.13 iterator standard in prose: the two styles (delegated, custom),
   the five spec fns, the constructor postconditions, loop-invariant idioms,
   the `decreases` rule.
3. The finite-sets rule as rewritten.
4. Open questions for the orchestrator.
