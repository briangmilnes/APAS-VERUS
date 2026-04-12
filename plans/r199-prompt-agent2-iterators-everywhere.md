# R199 Prompt — Agent 2: Iterators everywhere — inside verus!, new coverage, RTTs, PTTs (big round). AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent2`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER add `assume`, `admit`, `accept`, or `external_body` to
   algorithmic logic.** Iterator bridges may carry `assume(iter_invariant(self))`
   in `next()` per project policy (Verus doesn't allow `requires` on
   external trait impls); that's allowed. No other new holes.
6. **NEVER modify existing verified code to make new iterators work.**
   If adding an iterator to a module requires changing the module's
   existing trait/impl, STOP and report. The iterator is an addition,
   not a refactor.
7. **NEVER touch `Example*.rs` or `Problem*.rs` files.**

## Read all standards first.

**Especially these:**
- `src/standards/iterators_standard.rs` — the 10-component iterator
  standard. Every new iterator must implement this.
- `src/standards/wrapping_iterators_standard.rs` — when an iterator
  delegates to a backing collection's iterator.
- `docs/APAS-VERUSIterators.rs` — the 6 iteration test patterns.

**Also read:**
- `docs/VerusOptimizationsApplied.md` — project-wide SMT/proof patterns.
- Several existing iterator-bearing files end-to-end as references:
  - `src/Chap18/ArraySeqStEph.rs` (the canonical example)
  - `src/Chap41/AVLTreeSetMtEph.rs` (tree iterator pattern)
  - `src/Chap52/EdgeSetGraphMtEph.rs` (new R197 reference)

## Context

Agent3's R198 iterator audit documented three gap categories:

1. **One file has IntoIterator *outside* `verus!` due to an old Verus
   bug**: `src/Chap18/ArraySeqStPer.rs` (lines 1031–1032 carry a
   comment about "ill-typed AIR on proj%%core!iter.traits.collect.IntoIterator./Item").
   Verus has been upgraded since — needs re-test. If the bug is fixed,
   move IntoIterator inside `verus!` and recover consume patterns.
2. **~30 View-bearing files have no iterator infrastructure at all.**
   Many are collections where iteration is a natural operation (BST
   variants, AVLTreeSet variants, Tables, OrderedSet/Table MtEph/MtPer,
   AdjTableGraph/EdgeSetGraph variants). Priority queues (Chap45) are
   a legitimate skip — they're pop-iterated, not walk-iterated.
3. **Missing PTTs** for some of the iterators we already have (agent3
   closed 1 gap; agent's audit was per-existing-iterator-file, not
   per-newly-covered-type).

## Goal

Close all three gap categories in one round. Strict ordering to
minimize PTT run time: **all code changes first, then RTTs, then
PTTs, then one PTT run at the end.**

## Plan (4 strict phases — do not reorder)

### Phase 1: Source-code changes (no testing yet)

**1a. Re-test the Verus ill-typed AIR bug on `src/Chap18/ArraySeqStPer.rs`.**

- Move the `impl IntoIterator for &'a ArraySeqStPerS<T>` and `impl
  IntoIterator for ArraySeqStPerS<T>` blocks from outside `verus!`
  (currently at lines ~1031–1055) to inside `verus!` (before the
  `} // verus!` at line 1058).
- Remove the "moved outside verus! — Verus hits ill-typed AIR"
  comment.
- Run **only** `scripts/validate.sh isolate Chap18` to check.
- If it verifies clean → keep the move. Note in report that the
  bug is fixed.
- If it still fails with "ill-typed AIR" or similar → revert the
  move, keep the comment (update to mention the Verus commit it
  still fails against), and move on.

**1b. Audit and produce a coverage table.**

Before writing new iterators, build
`plans/r199-iterator-coverage-audit.md` with every collection-shape
type in `src/Chap*/`. Columns:

| # | Chap | File | Type | Has iterator? | Priority |
|---|------|------|------|---------------|----------|

Use this grep as a starting point:

```bash
grep -lrE "impl[[:space:]<][^>]*>[[:space:]]+View[[:space:]]+for" src/Chap*/ \
  | sort -u > /tmp/view_files.txt
grep -lrE "IntoIterator|ForLoopGhostIterator" src/Chap*/ \
  | sort -u > /tmp/iter_files.txt
comm -23 /tmp/view_files.txt /tmp/iter_files.txt
```

Priority guide:
- **high**: canonical collection semantics — Sets, Maps, Tables, Sequences,
  BST variants, AVLTreeSet variants.
- **medium**: domain-specific collections where iteration has a clear
  meaning — AdjTableGraph, EdgeSetGraph.
- **skip**: priority queues (Chap45 — by definition pop-iterated),
  "SpecsAndLemmas" files (no data), standalone helper types.

**1c. Write iterators for high-priority gaps.**

For each **high**-priority file, add the 10 iterator-standard
components (see `src/standards/iterators_standard.rs`):

1. Custom iterator struct (`<Type>Iter<'a, …>`).
2. `View` impl for the iterator struct.
3. `ForLoopGhostIteratorNew` impl.
4. `ForLoopGhostIterator` impl.
5. `Iterator for <Type>Iter` (Rust trait — outside verus! if needed,
   but prefer inside per iterators_standard).
6. `iter_invariant` spec fn.
7. `IntoIterator for &<Type>` (borrow iterator).
8. `IntoIterator for <Type>` (consume iterator) — only if a
   consumable iteration makes sense (skip for tree-shaped data
   where consuming iteration isn't the natural pattern; document
   the skip).
9. `.iter()` method on the type.
10. `.into_iter()` method on the type (if consume iterator exists).

**Wrap delegate iterators** where the backing collection already has
one (see `src/standards/wrapping_iterators_standard.rs`). Most
BST/AVLTreeSet iterators can delegate to the sequence view's Vec
iterator.

**If a specific file's iterator can't be written cleanly** (trait
bounds too restrictive, View shape incompatible, etc.), STOP on
that file, note the blocker, and move on to the next. Do **not** add
`external_body` to force it.

**Target**: close at least 15 of the ~30 high-priority gaps in this
round. If more close cleanly, great. If fewer, document what
blocked each unclosed one.

**1d. Move any remaining outside-verus! iterator impls inside.**

For any other file where iterator impls live outside `verus!` (not
just 1a's ArraySeqStPer), try moving them inside and validating.
Same rule: if the move breaks verification, revert and move on.

### Phase 2: Validate source-code changes

After all of Phase 1:

```bash
scripts/validate.sh
```

Must be clean (zero errors, zero new holes). If not, fix the
offending iterator (or revert the change for that specific file) and
re-run. Do NOT proceed to Phase 3 until validate is clean.

### Phase 3: Write RTTs

For every new iterator written in Phase 1, add a Rust runtime test
in `tests/ChapNN/Test<Type>.rs` (append to existing if present, or
create new and register in `Cargo.toml`).

Each iterator gets at minimum these RTT functions:
- `test_<type>_iter_empty` — iterate over empty, expect 0 elements.
- `test_<type>_iter_single` — iterate over singleton, expect 1 element.
- `test_<type>_iter_basic` — iterate over 3–10 elements, check
  ordering matches spec.
- `test_<type>_into_iter` — consume iteration (if implemented),
  check elements.

Run RTTs:

```bash
scripts/rtt.sh
```

All tests must pass. Count added tests, note in report.

### Phase 4: Write PTTs (all at once, then run PTT ONCE)

For every new iterator written in Phase 1, add a PTT file
`rust_verify_test/tests/ChapNN/Prove<Type>.rs` covering the 6
iteration patterns from `docs/APAS-VERUSIterators.rs`:

1. loop-borrow-iter
2. loop-borrow-into
3. for-borrow-iter
4. for-borrow-into
5. loop-consume (only if IntoIterator-for-Self is implemented)
6. for-consume (only if IntoIterator-for-Self is implemented)

**Use existing `rust_verify_test/tests/Chap18/ProveArraySeqStEph.rs`
as the exact template.**

Register each new PTT file in `rust_verify_test/Cargo.toml`.

**Write ALL PTT files first. Run PTT only once at the very end.**
PTT compile is ~4 minutes; per-file runs would blow the round budget.

```bash
scripts/ptt.sh
```

All 225+ tests must pass. If any new PTT fails:
- If the backing iterator proves fine but a specific pattern (e.g.,
  consume) doesn't, comment out just that pattern with a
  `// SKIPPED: <reason>` line referencing the specific blocker. Do
  NOT comment out the whole PTT file.
- If multiple patterns fail on the same file, re-run validate on
  that source file in isolation — the iterator may have a latent
  spec issue that's only caught by PTT.
- If you can't unblock within a reasonable effort, note in report
  which files hit which blockers.

## Out of scope

- **Do not** add iterators to priority queues (Chap45 BinaryHeapPQ,
  LeftistHeapPQ, BalancedTreePQ, SortedListPQ, UnsortedListPQ,
  HeapsortExample). Pop-iteration is the semantic model.
- **Do not** add iterators to `*SpecsAndLemmas` files.
- **Do not** touch benchmarks — agent2's R197 bench work stands.
- **Do not** modify any existing iterator's 10 components.
- **Do not** modify `src/standards/iterators_standard.rs` or
  `docs/APAS-VERUSIterators.rs`.
- **Do not** touch Chap52 Mt iterators — Agent3 just built
  EdgeSetGraphMtEph and the rest of Chap52 is stable.

## Commit strategy

Commit in phases:

1. After Phase 1 complete + validate clean:
   `git commit -m "R199 Agent 2: iterators — N new, M moved inside verus!"`
2. After Phase 3 RTTs complete + pass:
   `git commit -m "R199 Agent 2: +K RTTs for new iterators"`
3. After Phase 4 PTTs complete + pass:
   `git commit -m "R199 Agent 2: +P PTTs for new iterators"`

This way if any phase fails, we can back out just that phase.

## Report

Write `plans/agent2-round199-report.md` with:

- Phase 1a result: Verus ill-typed AIR bug — fixed or still broken?
- Phase 1b coverage table summary (# files audited, # high-priority
  targets, # skipped).
- Phase 1c: per-file outcome (iterator added, blocked with reason).
- Phase 1d: which other files had iterators moved inside verus!.
- Phase 2: validate wall time, verified count delta.
- Phase 3: RTT count delta, new tests per file.
- Phase 4: PTT count delta, new PTTs per file, any pattern skips
  with justifications.
- **Bugs found** (separate prominent section if any — e.g., PTT
  reveals an iterator spec bug).

## RCP

See commit strategy above. Final:

```bash
git push
```
