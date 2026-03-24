# R72 Agent 4 — PTT Iterator Coverage (Chap43) + OrderedSetStEph fix

## Objective

Create iterator PTTs for Chap43 modules. Each PTT proves the iterator's
requires chain works end-to-end: constructor ensures wf, wf satisfies iter()
requires, iter ensures feed loop invariants.

Also fix the out-of-standard unsafe `&*ptr` in OrderedSetStEph.rs.

## Style Guide

Read `plans/ptt-iterator-style-guide.md` before writing any code.

## Assigned modules (PTTs)

1. `src/Chap43/OrderedSetStEph.rs`
2. `src/Chap43/OrderedSetStPer.rs`
3. `src/Chap43/OrderedTableMtEph.rs`
4. `src/Chap43/OrderedTableStEph.rs`

Note: `ProveOrderedTableStPer.rs` already exists — do NOT recreate it.

## Fix: OrderedSetStEph.rs unsafe `&*ptr`

The `Iterator::next()` impl in `OrderedSetStEph.rs` uses an unsafe raw
pointer dereference (`unsafe { &*ptr }`) to return `&'a T` from an owned
`Vec<T>`. This is out of standard. The standard pattern (see
`src/standards/iterators_standard.rs`) delegates to an inner iterator.

Fix the iterator to follow the standard wrapping pattern: store a
`std::slice::Iter<'a, T>` or `std::vec::IntoIter<T>` as the inner iterator
and delegate `next()` to it. The `next()` body should be
`self.inner.next()` inside an `external_body`. Read the iterator standard
and the OrderedTableStEph.rs iterator for the correct pattern.

Run `scripts/validate.sh` after this fix to confirm zero regressions.

## Instructions (PTTs)

For each module:

1. **Check if it has iterator infrastructure** (section 10: iterator struct,
   `impl Iterator`, `iter()`, `IntoIterator`, `iter_invariant`,
   `ForLoopGhostIterator*`). If it does NOT have iterators, skip it and
   report "no iterator infrastructure."

2. **Read the constructor** (`singleton()`, etc.) and `iter()` trait
   signatures. Note:
   - What does the constructor `ensures`? Must include wf.
   - What does `iter()` `requires`? Must be satisfied by constructor ensures.
   - What type axiom `requires` does the constructor have (e.g.,
     `obeys_feq_clone`, `obeys_view_eq`)? These go in the test function's
     `requires`.

3. **Create the PTT file** at `rust_verify_test/tests/Chap43/Prove<Module>.rs`
   with 4 patterns: loop-borrow-iter, loop-borrow-into, for-borrow-iter,
   for-borrow-into. If the module also has a consuming `IntoIterator for T`
   (not just `for &T`), add loop-consume and for-consume patterns too.

4. **Run `scripts/ptt.sh`** after each file to verify it compiles and proves.

## Canonical examples

- Simple (no type axiom requires): `rust_verify_test/tests/Chap18/ProveArraySeqStEph.rs`
- With type axioms: `rust_verify_test/tests/Chap43/ProveOrderedTableStPer.rs`

## Rules

- Read `CLAUDE.md` on startup.
- Fix the OrderedSetStEph unsafe FIRST, validate, then create PTTs.
- Do NOT run validate from PTT creation — only `scripts/ptt.sh` for PTTs.
- If a pattern doesn't prove, debug it. Do not skip patterns.
- Report what you created, any modules skipped, and the OrderedSetStEph fix.
