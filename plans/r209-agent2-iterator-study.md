# r209 agent 2 — study APAS-VERUS's iterators and map them to the prophetic model

Date: 2026-09-21. One background subagent, review only. Runs concurrently with
r209 agent 1, which is editing Chap05, 17, 49–51, 61–66, `src/vstdplus/`, and
`src/standards/using_hashmap_standard.rs`; read those on disk, do not edit
anything, and say which version you read.

## Question

Which iterator definitions and iterator consumers in the crate are still on
the pre-05.21 model, what does each become under the 09.13 prophetic model,
how much of that is mechanical, and in what order should the chapters be
migrated?

## Inputs

- `src/standards/prophetic_iterators_standard.rs`, `iterators_standard.rs`,
  `wrapping_iterators_standard.rs`, `iterator_ptt_standard.rs` (rewritten in
  r207; the target shapes), and `docs/StandardsUpgrade.md` §2–3 (the two
  styles, the five spec fns, the constructor triple, the `decreases` rule, and
  behaviour 2: a trait method whose `ensures` names `IteratorSpec::remaining`
  on a same-crate iterator type breaks that type's `next`).
- `src/experiments/prophetic_adaptor_*.rs` (eleven probes, results in headers).
- `plans/verus-0.2026.05.21-iterator-migration.md` §4 (old → new API mapping),
  §5 (file inventory), §10 (the 71-iterator classification: 68 delegated, 3
  custom, and the cost table). Re-derive; it predates r207 and r208.
- `docs/Chap02to06Validation.md` §5–6 and the current `src/Chap05`,
  `src/Chap06`: the migration already done, the template for the rest.
- `plans/r204-veracity-iterator-upgrade-apply-report.md`: the veracity rewriter
  that once applied 1,256 mechanical edits on a fixture (the tool is not on
  this machine; the report lists its rewrite classes D1–D10, T1–T10).
- `~/projects/verus/source/vstd/std_specs/iter.rs`, `slice.rs`, `vec.rs`,
  `hash.rs`, `range.rs`: which std iterators implement `IteratorSpecImpl`.
- `~/projects/verus/examples/guide/iterators.rs` and
  `source/rust_verify_test/tests/iterators.rs`: custom iterator examples.

## Procedure

1. Inventory, by `grep` over `src/`, `tests/`, `rust_verify_test/tests/`
   (exclude `experiments/`): every `impl Iterator for`, every
   `ForLoopGhostIterator`, `ForLoopGhostIteratorNew`, `iter_invariant`,
   `it@.0`, `it@.1`, `.pos`, `.elements` on an iterator, every
   `for x in it: expr` and every manual `loop { match it.next() }`, and every
   `IteratorSpecImpl` already present. One row per file: Chap, file, iterator
   structs defined, backing field, loops consumed, old-model constructs count,
   already migrated (yes/no).
2. Classify each iterator definition: delegated (a `Vec`, slice, `HashSet`,
   or `HashMap` underneath: return the std iterator), chained (re-exposes
   another APAS collection's iterator), custom (a lazy traversal with no std
   iterator underneath: hand-written `IteratorSpecImpl`). Read the body of
   `iter()` and `next()` for each; the classification is evidence-based, not
   name-based. Record the `iter()`/`next()` work and space before and after,
   which must be equal (the r206 §4 rule).
3. For each old construct give the new construct (the §4 mapping of the May
   plan, checked against the r207 standards) and the exact edit shape: a
   deletion, a one-token substitution, a clause substitution, or a rewrite
   that needs a proof. Count sites per shape per file.
4. For the custom iterators (expected: `Chap37/AVLTreeSeq.rs`,
   `AVLTreeSeqStEph.rs`, `AVLTreeSeqStPer.rs`), sketch the `IteratorSpecImpl`
   in a code block: the five spec fns, the type invariant, the constructor
   triple, and how behaviour 2 constrains where `iter()` may live (inherent
   impl or trait with `ensures` over the inner iterator). Do not write the
   files.
5. Consumers only (chapters that loop over another chapter's collection):
   list the loop-invariant rewrites needed, with one worked example per shape.
6. Order: group chapters by dependency (`Cargo.toml` `[features]`), and give a
   migration order in which every chapter's dependencies are migrated before
   it. Mark which chapters block which.
7. Tool decision: total mechanical sites versus proof-needing sites; whether a
   CST transformer built on `~/projects/CSTs` (read its `README.md`) pays for
   itself, and which rewrite classes it would implement.

## Rules

Read `plans/r207-round-plan.md`; its common rules bind you. You edit nothing
under `src/`, `tests/`, `rust_verify_test/`, `scripts/`, or `plans/`. You do
not run `validate.sh`, `validate-standard.sh`, `rtt.sh`, `ptt.sh`, or
`profile.sh`; another agent holds Verus. No subagents. No commits. Write in
the ComputAItionalThinking vocabulary. Every table has a `#` column first and,
when it names files, a `Chap` column second; cells at most 40 characters.

## Deliverable: `docs/IteratorMigrationStudy.md`

1. Summary: counts of iterator definitions by class, consumer files, old-model
   sites by edit shape, chapters already migrated.
2. The inventory table (procedure 1).
3. The classification table with the cost columns (procedure 2).
4. The construct mapping with edit shapes and counts (procedure 3).
5. The three custom iterator sketches (procedure 4).
6. Consumer loop rewrites (procedure 5).
7. Migration order with the dependency graph (procedure 6).
8. Tool recommendation (procedure 7).
9. What you could not determine and why.
