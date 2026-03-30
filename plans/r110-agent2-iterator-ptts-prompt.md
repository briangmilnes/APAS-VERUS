# R110 Agent 2 — Write missing iterator PTTs for all collections. AFK.

## Objective

22 collections have `iter()` but no iterator PTT patterns. Write PTTs for all of
them following the 6-pattern standard in `src/standards/iterator_ptt_standard.rs`.

## Resource constraint

Agent 1 is running `scripts/validate.sh isolate Chap65` intermittently. Verus+Z3
uses 8GB+. Do NOT run `scripts/validate.sh`, `scripts/ptt.sh`, or `scripts/rtt.sh`.

Instead:
1. Write all PTT files.
2. Run `cargo check -p rust_verify_test` (compile check only, no Z3, fast).
3. Commit when cargo check passes.

We run `ptt.sh` on main after merge.

## The 22 missing collections

### No PTT file — create new (15)

| # | Chap | Module | PTT to create |
|---|------|--------|---------------|
| 1 | 05 | SetStEph | ProveSetStEph.rs |
| 2 | 37 | AVLTreeSeqMtPer | ProveAVLTreeSeqMtPer.rs |
| 3 | 37 | AVLTreeSeqStEph | ProveAVLTreeSeqStEph.rs |
| 4 | 37 | AVLTreeSeqStPer | ProveAVLTreeSeqStPer.rs |
| 5 | 37 | BSTSetAVLMtEph | ProveBSTSetAVLMtEph.rs |
| 6 | 37 | BSTSetBBAlphaMtEph | ProveBSTSetBBAlphaMtEph.rs |
| 7 | 37 | BSTSetPlainMtEph | ProveBSTSetPlainMtEph.rs |
| 8 | 37 | BSTSetRBMtEph | ProveBSTSetRBMtEph.rs |
| 9 | 37 | BSTSetSplayMtEph | ProveBSTSetSplayMtEph.rs |
| 10 | 43 | AugOrderedTableMtEph | ProveAugOrderedTableMtEph.rs |
| 11 | 43 | AugOrderedTableStEph | ProveAugOrderedTableStEph.rs |
| 12 | 43 | AugOrderedTableStPer | ProveAugOrderedTableStPer.rs |
| 13 | 43 | OrderedSetStPer | ProveOrderedSetStPer.rs |
| 14 | 43 | OrderedTableMtEph | ProveOrderedTableMtEph.rs |
| 15 | 41 | AVLTreeSetMtPer | ProveAVLTreeSetMtPer.rs |

### PTT exists — add iterator patterns (7)

| # | Chap | Module | Existing PTT | Add patterns |
|---|------|--------|-------------|-------------|
| 16 | 18 | ArraySeq | ProveArraySeq.rs | 4-6 patterns |
| 17 | 18 | ArraySeqMtEph | ProveArraySeqMtEph.rs | 4-6 patterns |
| 18 | 18 | ArraySeqMtPer | ProveArraySeqMtPer.rs | 4-6 patterns |
| 19 | 18 | ArraySeqStPer | ProveArraySeqStPer.rs | 4-6 patterns |
| 20 | 18 | LinkedListStEph | ProveLinkedListStEph.rs | 4-6 patterns |
| 21 | 18 | LinkedListStPer | ProveLinkedListStPer.rs | 4-6 patterns |
| 22 | 41 | AVLTreeSetMtEph | ProveAVLTreeSetMtEph.rs | check if 6 patterns are the right 6 |

## How to write each PTT

### Read first for every module

Before writing a PTT, read the source file to find:
- The struct name (e.g., `SetStEphS<T>`)
- The iterator struct name (e.g., `SetStEphBorrowIter<'a, T>`)
- The ghost iterator struct name (e.g., `SetStEphGhostIterator<'a, T>`)
- The `iter_invariant` spec fn name
- The constructor (`new`, `empty`, `singleton`, `from_vec` — whatever builds an instance)
- Whether `IntoIterator for Self` exists (determines if consume patterns apply)
- The module path for `use` statements

### Template

Follow the exact patterns in `rust_verify_test/tests/standards/Proveiterators_standard.rs`.
Read that file. Every PTT follows that structure.

The 4 borrow patterns (loop-borrow-iter, loop-borrow-into, for-borrow-iter, for-borrow-into)
are always required. The 2 consume patterns (loop-consume, for-consume) are only required
if the module implements `IntoIterator for Self`.

### Test naming

```
#[test] setsteph_loop_borrow_iter
#[test] setsteph_loop_borrow_into
#[test] setsteph_for_borrow_iter
#[test] setsteph_for_borrow_into
#[test] setsteph_loop_consume       // only if IntoIterator for Self exists
#[test] setsteph_for_consume        // only if IntoIterator for Self exists
```

Use lowercase module name, no underscores between words (match existing convention).

### Mt variants

Mt collections need lock setup. Read how existing Mt PTTs work:
- `rust_verify_test/tests/Chap05/ProveSetMtEph.rs` — Mt set iterator patterns
- `rust_verify_test/tests/Chap06/ProveDirGraphMtEph.rs` — Mt graph iterator patterns
- `rust_verify_test/tests/Chap19/ProveArraySeqMtEph.rs` — Mt array seq patterns

Mt iterators iterate over a locked snapshot. The constructor typically requires
building the inner type then wrapping it. Copy the pattern from existing Mt PTTs.

### BSTSet* modules (Chap37)

All 5 BSTSet*MtEph modules (AVL, BBAlpha, Plain, RB, Splay) share the same
iterator interface via the BSTSet trait. Read one (e.g., BSTSetAVLMtEph.rs) to
understand the constructor and iterator types, then replicate for all 5.

## File placement

- New files: `rust_verify_test/tests/ChapNN/ProveModuleName.rs`
- Make sure the ChapNN directory exists in `rust_verify_test/tests/`.
- Existing files: add patterns at the end of the file.

## Cargo.toml

New PTT files may need to be registered. Check `rust_verify_test/Cargo.toml` to
see if test files are auto-discovered or need explicit `[[test]]` entries.

## Work order

1. Read `rust_verify_test/tests/standards/Proveiterators_standard.rs` (template)
2. Read 2-3 existing collection PTTs to see the real patterns
3. Start with Chap18 (add patterns to existing PTTs — safest, most templates nearby)
4. Do Chap05 SetStEph
5. Do Chap37 (bulk — 8 files, similar patterns)
6. Do Chap43 (5 files)
7. Do Chap41 (1-2 files)
8. `cargo check -p rust_verify_test` — fix any compile errors
9. Commit

## Rules

- Do NOT run `scripts/validate.sh`, `scripts/ptt.sh`, or `scripts/rtt.sh`.
- Only `cargo check -p rust_verify_test` for compilation.
- Read each source module before writing its PTT. Get the types right.
- Follow the exact pattern from Proveiterators_standard.rs. Don't improvise.
- If a module's iterator doesn't follow the standard pattern (e.g., hand-rolled
  iterator without `ForLoopGhostIterator`), skip the for-loop patterns and note
  it in your report.
- No subagents.

## Report

Write `plans/agent2-r110-iterator-ptts-report.md`. Include a table of all 22
modules with status (done/skipped) and which patterns were written.
