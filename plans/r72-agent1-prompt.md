# R72 Agent 1 — PTT Iterator Coverage (Chap37 Seq modules)

## Objective

Create iterator PTTs for Chap37 sequence modules. Each PTT proves the
iterator's requires chain works end-to-end: constructor ensures wf, wf
satisfies iter() requires, iter ensures feed loop invariants.

## Style Guide

Read `plans/ptt-iterator-style-guide.md` before writing any code.

## Assigned modules

1. `src/Chap37/AVLTreeSeq.rs`
2. `src/Chap37/AVLTreeSeqStEph.rs`
3. `src/Chap37/AVLTreeSeqStPer.rs`

## Instructions

For each module:

1. **Check if it has iterator infrastructure** (section 10: iterator struct,
   `impl Iterator`, `iter()`, `IntoIterator`, `iter_invariant`,
   `ForLoopGhostIterator*`). If it does NOT have iterators, skip it and
   report "no iterator infrastructure."

2. **Read the constructor** (`new()`, `singleton()`, etc.) and `iter()`
   trait signatures. Note:
   - What does the constructor `ensures`? Must include wf.
   - What does `iter()` `requires`? Must be satisfied by constructor ensures.
   - What type axiom `requires` does the constructor have (e.g.,
     `obeys_feq_clone`, `obeys_view_eq`)? These go in the test function's
     `requires`.

3. **Create the PTT file** at `rust_verify_test/tests/Chap37/Prove<Module>.rs`
   with 4 patterns: loop-borrow-iter, loop-borrow-into, for-borrow-iter,
   for-borrow-into. If the module also has a consuming `IntoIterator for T`
   (not just `for &T`), add loop-consume and for-consume patterns too.

4. **Run `scripts/ptt.sh`** after each file to verify it compiles and proves.

## Canonical examples

- Simple (no type axiom requires): `rust_verify_test/tests/Chap18/ProveArraySeqStEph.rs`
- With type axioms: `rust_verify_test/tests/Chap43/ProveOrderedTableStPer.rs`

## Rules

- Read `CLAUDE.md` on startup.
- Do NOT modify source files — only create new PTT files.
- Do NOT run `scripts/validate.sh` — only `scripts/ptt.sh`.
- If a pattern doesn't prove, debug it. Do not skip patterns.
- Report what you created and any modules skipped (no iterators).
