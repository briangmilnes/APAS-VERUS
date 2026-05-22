# Plan — `veracity-iterator-upgrade --detect`

Status: SCOPING. Author: 2026-05-22.

A read-only veracity tool that scans APAS-VERUS for the obsolete
`ForLoopGhostIterator` iterator model and emits a structured report of
(a) what must be **deleted** and (b) which invariant lines can be
**transformed line by line** to the new prophetic-iterator model (verus
0.2026.05.21, PR #2163).

Companion documents:
- Standard (target shape): `src/standards/prophetic_iterators_standard.rs`
- User reference: `docs/PropheticIterators.md`
- Migration schedule: `plans/verus-0.2026.05.21-iterator-migration.md`

This plan covers `--detect` only. A future `--apply` mode (mechanical
rewrite) is out of scope here.

## 0. Veracity work-area constraint — READ FIRST

Veracity development for this tool MUST happen entirely inside the
veracity repo's own test fixture:

```
~/projects/veracity/tests/fixtures/APAS-VERUS/
```

Veracity MUST NOT read from, write to, or in any other way touch the
live APAS-VERUS tree at `~/projects/APAS-VERUS/`. The live tree is
where the human and the APAS-VERUS agents are working; veracity
edits there would race with in-flight proof work and destroy hours of
Z3 budget.

Concretely:

- The calibration corpus, the pilot at `Chap18/ArraySeqStEph.rs`, the
  3 custom files, and every other file referenced in this plan are
  copies inside the veracity fixture — not the live paths. If the
  fixture is stale, refresh it by copying from `~/projects/APAS-VERUS/`
  at a known commit, then commit the snapshot into the veracity repo.
- The `--root` flag is REQUIRED and has no default. The tool refuses
  to run without an explicit `--root`. This prevents an accidental
  `cd ~/projects/APAS-VERUS && veracity-iterator-upgrade --detect`
  from touching live source.
- The tool refuses to run if `--root` resolves (after `realpath`) to a
  path matching `*/APAS-VERUS` that is NOT under
  `*/veracity/tests/fixtures/`. Override with
  `--i-know-what-im-doing-not-a-fixture` (verbose on purpose).
- Output paths (`--out-dir`) MUST be under the same `--root` (or
  under `/tmp`); the tool refuses to write outside that subtree.
- All test runs, calibration, and regression checks happen inside the
  fixture. The veracity test harness compares the tool's output
  against a checked-in golden file under
  `~/projects/veracity/tests/fixtures/APAS-VERUS/analyses/`.

The user (or a separate orchestrator, not the veracity agent) is the
only party allowed to run `--detect` against the live APAS-VERUS tree,
and only after the tool has been calibrated green against the fixture.

## Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace
on Rust source. All detection must be token-aware or AST-aware. Parse
`ensures`/`requires` blocks, `impl` headers, struct definitions, and loop
invariants with brace/comma/semicolon awareness. A string-hacking detector
will flag and kill tools that corrupt source syntax. Use `syn` (preferred)
or a tokenizer with bracket tracking.

## 1. Goal

Given a checkout root, produce — for every collection file — a precise
inventory of:

1. **Deletions** — items that must be removed because the old iterator
   protocol no longer exists in verus. These are syntactic — the items
   are defined in the file and their callers (the for-loop desugaring,
   ghost invariants) no longer exist.
2. **Transforms** — invariant and ensures lines whose old form
   (`it@.0`, `it@.1`, `iter_invariant(&it)`) maps to a new form
   (`IteratorSpec::remaining(&it)`, `it.seq()`, `it.index()`) by a
   purely local rewrite. The detect pass shows the new form alongside
   the old so a human (or `--apply`) can confirm.
3. **Unresolved** — patterns the tool sees but cannot classify
   automatically (manual `loop` with prophetic-incompatible
   `decreases`, custom AVLTreeSeq iterators, etc.). Reported with file
   and line, never silently dropped.

The tool MUST NOT modify any source file in `--detect` mode.

## 2. Inputs and invocation

```
veracity-iterator-upgrade --detect [--root PATH] [--ignore GLOB ...]
                                   [--format md|json|compile|all]
                                   [--out-dir PATH]
```

- `--root PATH` — default `.` (current dir, expected to be APAS-VERUS).
- `--ignore GLOB` — default ignore list: `src/standards/**`,
  `src/experiments/**`, `rust_verify_test/**`, `target/**`,
  `analyses/**`, `logs/**`, `docs/**`.
- `--format` — default `all`. Three outputs:
  - `md` — human-readable Markdown report.
  - `json` — machine input to a future `--apply`.
  - `compile` — GNU-style `file:line:col: tag: message` lines, one per
    finding, suitable for `M-x compile` / `next-error` stepping.
- `--out-dir PATH` — default `analyses/`. Writes:
  - `analyses/iterator-upgrade-detect.md`
  - `analyses/iterator-upgrade-detect.json`
  - `analyses/iterator-upgrade-detect.compile` (the `M-x compile` log)

The 71 collection files to scan are enumerated in
`docs/PropheticIterators.md` (per-file table); the tool may take that
file as a hint but must not depend on it — its own AST scan is
authoritative.

## 3. Classification: delegated vs custom

For each file the tool determines the iterator style by inspecting the
`iter()` / `into_iter()` return type and the `*Iter` struct (if any):

- **delegated** — file's `iter()` returns either (a) a `std::slice::Iter`
  / `std::vec::IntoIter` / `hash_set::Iter` / `hash_map::Iter` directly,
  or (b) a wrapper struct whose only field is one of those, OR (c) a
  wrapper struct whose only field is another APAS `*Iter` (chained).
  After migration: bare std iterator, wrapper deleted.
- **custom** — file's `iter()` returns a hand-rolled iterator type with
  no std iterator field (genuine lazy traversal). After migration:
  type kept, six `IteratorSpecImpl` spec fns added, old
  `ForLoopGhostIterator` block deleted.

The 3 custom files are pinned by name: `Chap37/AVLTreeSeq.rs`,
`Chap37/AVLTreeSeqStEph.rs`, `Chap37/AVLTreeSeqStPer.rs`. Everything
else is delegated. The tool flags any file it classifies differently
than this pinned list as **Unresolved (U-CLASS)** — it does not
silently override the pin.

## 4. Deletion classes

Each class is a syntactically recognisable item. The tool reports the
class, the item's identifier (struct name, impl head), and a contiguous
line range.

| # | Class | What | Example identifier |
|---|-------|------|--------------------|
| 1 | D1 | `pub struct XxxGhostIterator` definition | `ArraySeqStEphGhostIterator` |
| 2 | D2 | `impl View for XxxGhostIterator` with `(int, Seq<T>)` view | `View for ArraySeqStEphGhostIterator` |
| 3 | D3 | `impl ForLoopGhostIteratorNew for XxxIter` block | the 5-line block w/ `type GhostIter` |
| 4 | D4 | `impl ForLoopGhostIterator for XxxGhostIterator` block | the 6-method block |
| 5 | D5 | Outside-`verus!` Debug/Display for `XxxGhostIterator` | `Debug for ArraySeqStEphGhostIterator` |
| 6 | D6 | (delegated only) `pub struct XxxIter` wrapper | `ArraySeqStEphIter<'a, T>` |
| 7 | D7 | (delegated only) `impl View for XxxIter` w/ `(int, Seq<T>)` | `View for ArraySeqStEphIter` |
| 8 | D8 | (delegated only) `impl Iterator for XxxIter` with `ensures` | `Iterator for ArraySeqStEphIter` |
| 9 | D9 | (delegated only) outside-`verus!` Debug/Display for `XxxIter` | `Debug for ArraySeqStEphIter` |
| 10 | D10 | `pub open spec fn iter_invariant(it: &XxxIter) -> bool` | `iter_invariant<'a, T>` |

Classes D1–D5 apply to **both** styles. Classes D6–D10 are
delegated-only: in delegated files the wrapper struct is replaced by
the bare std iterator, so the wrapper, its View, its
`Iterator::next` impl, its outside-`verus!` Debug/Display, and its
`iter_invariant` predicate all disappear.

In **custom** files the `XxxIter` struct survives (D6–D9 NOT
emitted), but its current `View` (D7-equivalent) and `iter_invariant`
predicate (D10-equivalent) are replaced by a
`#[verifier::type_invariant]` + six `IteratorSpecImpl` spec fns. The
tool reports the custom-file replacements as `U-CUSTOM` (out of
mechanical scope — see §6).

### Detection rules (AST)

- D1 — `ItemStruct` whose name ends in `GhostIterator`.
- D2 — `ItemImpl` whose trait is `View` and self-type ends in
  `GhostIterator`.
- D3 — `ItemImpl` whose trait path ends in
  `ForLoopGhostIteratorNew`.
- D4 — `ItemImpl` whose trait path ends in `ForLoopGhostIterator`
  (without the `New` suffix).
- D5 — `ItemImpl` whose trait is `Debug` or `Display` and self-type
  ends in `GhostIterator`, located outside the `verus!{ ... }` macro.
- D6 — `ItemStruct` whose name ends in `Iter` (not `GhostIterator`)
  with exactly one field whose type matches one of: `std::slice::Iter`,
  `std::vec::IntoIter`, `std::collections::hash_set::Iter`,
  `std::collections::hash_map::Iter`, or another APAS `*Iter` type.
- D7 — `ItemImpl` of `View` for a D6 struct.
- D8 — `ItemImpl` of `std::iter::Iterator` (or `Iterator`) for a D6
  struct, where `fn next` has an `ensures` clause. The tool reports
  the impl head + the `next` body's `ensures` block separately.
- D9 — D5 pattern but matching `XxxIter` instead of
  `XxxGhostIterator`.
- D10 — `ItemFn` whose name is `iter_invariant` and whose parameter
  type is a reference to a D6 struct.

Each match records `{class, ident, file, line_start, line_end}`.

## 5. Transform classes (line-by-line in invariants)

These are local rewrites on `ensures`/`invariant`/`decreases` clauses
that the detect pass can name and pair (old → new). The tool emits the
**old line** verbatim and the **proposed new line**; it does not
rewrite the file.

| # | Class | Old form (one line) | New form |
|---|-------|---------------------|----------|
| 1 | T1 | `it@.0 == k,` | `IteratorSpec::remaining(&it).len() + k == it.seq().len(),` |
| 2 | T2 | `it@.1 == self.seq@,` | `it.seq() == self.seq@,` |
| 3 | T3 | `it@.1 == <expr>,` (generic RHS) | `it.seq() == <expr>,` |
| 4 | T4 | `iter_invariant(&it),` | *(remove — subsumed by `IteratorSpec` ensures at construction)* |
| 5 | T5 | `it@.0 < it@.1.len(),` | `IteratorSpec::decrease(&it).unwrap() > 0,` |
| 6 | T6 | `decreases self.seq@.len() - it@.0,` | `decreases IteratorSpec::decrease(&it).unwrap(),` |
| 7 | T7 | `it@.0 == self.seq@.len(),` (post-loop terminal) | `it.index() == self.seq@.len(),` |
| 8 | T8 | `iter_invariant(&it)` inside an `ensures` chain (on a constructor) | the prophetic triple: `IteratorSpec::remaining(&it) == self.seq@.as_ref(), IteratorSpec::decrease(&it) is Some, IteratorSpec::initial_value_relation(&it, &it),` |

Transform T8 is multi-line on the new side; the tool emits it as a
single transform with a multi-line `new` block.

### Detection rules (AST + token)

Walk the syntax tree, and for each `ensures` / `requires` / `invariant`
clause (and the `Expr` inside a `decreases` attribute), tokenize and
match against the patterns above using a small pattern matcher (not a
text regex). The matcher must:

- recognise `it@` as a method receiver (View::view call) followed by
  `.0` / `.1` field access on the tuple result;
- bind the metavariable `<expr>` to a balanced expression up to the
  next top-level comma or semicolon;
- recognise `iter_invariant` as a call by path resolution, not just by
  name (do not match `foo::iter_invariant` if `foo` is not the current
  file's module).

Each match records `{class, file, line, col_start, col_end, old_text,
new_text}`. The `new_text` is a **string the tool generates** from a
template — it is shown to the user for review but is not applied.

### Out of scope for T-classes

- Loops whose invariant references the *index* `i` rather than the
  iterator state (`for (i, item) in ...enumerate()` loops). Those
  invariants name `i`, not `it@`, and are unchanged by the migration.
- Conjunction reordering, hoisting, or simplification.
- `requires` on functions outside iterator constructors.

## 6. Unresolved classes

Reported but not auto-classified. The detect pass writes these to the
report so a human can decide.

| # | Code | Meaning |
|---|------|---------|
| 1 | U-CLASS | File's actual style disagrees with the pinned classification (see §3) |
| 2 | U-CUSTOM | Custom-style file (3 AVLTreeSeq) — needs hand-ported `IteratorSpecImpl`, not mechanical |
| 3 | U-LOOP | Manual `loop` with `decreases` that references something other than `IteratorSpec::decrease` — needs human review (the prophetic `seq()` is barred from `decreases`) |
| 4 | U-POST | Post-loop assertion referencing `it@` after the loop exit — `it` is not in scope post-loop in the new model; the fact must come from `when_used_as_spec` |
| 5 | U-CHAIN | `D6` wrapper whose field is another APAS `*Iter` (chained) — deletion is correct, but the new `IntoIterator::IntoIter` type depends on the inner collection's migration; flag for ordering |
| 6 | U-MULTI | Multi-iterator loop (zip-like) — the prophetic model handles these via separate `IteratorSpec::remaining` invariants; flag for human |
| 7 | U-OTHER | Anything else the tool's matcher fails on inside an `ensures`/`invariant` that mentions `it` |

## 7. Output format

### Per-file Markdown report

For each file the tool emits a section:

```
## src/Chap18/ArraySeqStEph.rs (delegated)

Deletions (6):
| # | Class | Item | Lines |
|---|-------|------|-------|
| 1 | D1 | ArraySeqStEphGhostIterator           | 955–961 |
| 2 | D2 | View for ArraySeqStEphGhostIterator  | 963–966 |
| 3 | D3 | ForLoopGhostIteratorNew for …Iter    | 998–1003 |
| 4 | D4 | ForLoopGhostIterator for …GhostIter  | 1005–1038 |
| 5 | D5 | Debug for …GhostIterator             | 1133–1137 |
| 6 | D5 | Display for …GhostIterator           | 1139–1143 |

Transforms (4):
| # | Class | Line | Old | New |
|---|-------|------|-----|-----|
| 1 | T1 | 934 | it@.0 == 0,                       | IteratorSpec::remaining(&it).len() == it.seq().len(), |
| 2 | T2 | 935 | it@.1 == self.seq@,               | it.seq() == self.seq@,                                |
| 3 | T4 | 936 | iter_invariant(&it),              | <remove>                                              |
| 4 | T8 | 1045–1047 | (constructor ensures triple) | (prophetic triple, 3 lines)                          |

Unresolved (0):
```

Table cells capped at 40 characters; longer text goes to a footnote
section below the table.

### Aggregate summary

At the top of the report, the per-chapter table:

| # | Chap | File | Style | D | T | U |
|---|------|------|-------|---|---|---|
| 1 | 18 | ArraySeqStEph.rs | delegated | 6 | 4 | 0 |
| 2 | 18 | ArraySeqStPer.rs | delegated | 6 | 4 | 0 |
| … |
| 71 | — | hash_map_with_view_plus.rs | delegated | 6 | 4 | 0 |

Plus a grand-total row: `total D=…, T=…, U=…`.

### JSON

Mirrors the Markdown but machine-readable. Top level:

```json
{
  "tool": "veracity-iterator-upgrade",
  "mode": "detect",
  "verus_version": "release/rolling/0.2026.05.21.b02cf68",
  "root": "/home/milnes/projects/APAS-VERUS",
  "files": [
    {
      "path": "src/Chap18/ArraySeqStEph.rs",
      "style": "delegated",
      "deletions": [
        {"class": "D1", "ident": "ArraySeqStEphGhostIterator",
         "lines": [955, 961]}
      ],
      "transforms": [
        {"class": "T2", "line": 935, "col_start": 17, "col_end": 39,
         "old": "it@.1 == self.seq@,",
         "new": "it.seq() == self.seq@,"}
      ],
      "unresolved": []
    }
  ],
  "summary": {"files": 71, "deletions": 426, "transforms": 284,
              "unresolved": 0}
}
```

### Compile-compatible log (`M-x compile`)

`analyses/iterator-upgrade-detect.compile` is a plain-text file where
every line is a single finding in GNU error format:

```
<path>:<line>:<col>: <tag>: <message>
```

- `<path>` — path relative to `--root` (so `compile-mode` resolves it
  when run from the project root).
- `<line>:<col>` — 1-based line and column of the item or clause.
  Deletion items use the first column of the item's first line.
- `<tag>` — `delete`, `transform`, `unresolved`, or `summary`.
  Maps to Emacs `compilation-error-regexp-alist` severities:
  - `delete` → `warning` (yellow). The item must go.
  - `transform` → `info` (cyan). A safe local rewrite is available.
  - `unresolved` → `error` (red). Human review required.
  - `summary` → printed once at the top, no severity.
- `<message>` — one line. For transforms: `T<n>: <old>  →  <new>`
  (escaping newlines as `\n` if multi-line). For deletions:
  `D<n>: <ident> [<line_start>-<line_end>]`. For unresolved:
  `U-<code>: <reason>`.

Example excerpt:

```
src/Chap18/ArraySeqStEph.rs:1:1: summary: delegated, D=9 T=6 U=0
src/Chap18/ArraySeqStEph.rs:946:5: warning: D6: ArraySeqStEphIter [946-948]
src/Chap18/ArraySeqStEph.rs:950:5: warning: D7: View for ArraySeqStEphIter [950-953]
src/Chap18/ArraySeqStEph.rs:955:5: warning: D1: ArraySeqStEphGhostIterator [955-961]
src/Chap18/ArraySeqStEph.rs:934:17: info: T1: it@.0 == 0,  →  IteratorSpec::remaining(&it).len() == it.seq().len(),
src/Chap18/ArraySeqStEph.rs:935:17: info: T2: it@.1 == self.seq@,  →  it.seq() == self.seq@,
src/Chap18/ArraySeqStEph.rs:936:17: info: T4: iter_invariant(&it),  →  <remove>
src/Chap37/AVLTreeSeqStEph.rs:1:1: error: U-CUSTOM: hand-port IteratorSpecImpl required
```

The first column MUST be the absolute first column of the offending
token (Emacs `next-error` jumps there). Multi-line transforms (T8) are
emitted as a single line with `\n` escapes — the user sees the
expansion in the message and the buffer jumps to the start.

Default Emacs `compile-mode` parses GNU format out of the box, so:

```
M-x compile RET cat analyses/iterator-upgrade-detect.compile RET
```

then `C-x \`` to step through findings.

Header lines (above the first finding) are prefixed with `# ` so they
are ignored by `compilation-error-regexp-alist` but visible to the
reader:

```
# veracity-iterator-upgrade --detect
# root: /home/milnes/projects/APAS-VERUS
# verus: release/rolling/0.2026.05.21.b02cf68
# generated: 2026-05-22T13:14:15Z
# totals: files=71 D=426 T=284 U=3
```

## 8. Exit codes

- `0` — scan completed; report written. (Findings ≥ 0 is success.)
- `2` — one or more files failed to parse. The report still lists
  what was scanned; the failures are listed at the top.
- `3` — invocation error (bad flag, missing root).

`--detect` never returns nonzero solely because findings exist; this
keeps CI usage non-blocking.

## 9. Implementation outline

1. **Parser layer** — `syn::parse_file` per `.rs` file under `--root`.
   Skip files whose module is inside the ignore list (string-prefix
   match on the absolute path, no regex).
2. **verus! macro handling** — `syn` does not parse inside the
   `verus!` macro by default. Use the same tokenisation strategy
   veracity already uses for `veracity-review-verus-style`: parse the
   token stream, locate `verus!` macro invocations, splice their
   contents into a synthesised `mod` and re-parse with `syn` plus the
   verus syntax extensions if available, OR walk the token stream
   directly with bracket tracking for the items the tool needs (struct,
   impl, fn). Whichever path is taken, the matcher MUST be
   token-aware (no string regex). Either choice is acceptable; pick the
   one already used by other veracity tools to keep dependencies
   shared.
3. **Pinned classification** — hard-coded list of the 3 custom files
   from §3. Everything else delegated.
4. **Deletion matchers** — visitor pattern over the AST: for each
   `ItemStruct`/`ItemImpl`/`ItemFn`, run the §4 detection rules and
   collect matches.
5. **Transform matchers** — visit `ensures`/`invariant`/`requires`/`decreases`
   nodes. For each clause, run a small AST-level pattern matcher
   (don't reflow tokens through a string buffer) against the T1–T8
   patterns. Bind metavariables (`<expr>`) by walking subtrees.
6. **Unresolved bin** — any `ensures`/`invariant`/`decreases` clause
   that mentions an identifier classified as the loop iterator but
   matches no T-pattern is bucketed as `U-OTHER` with the verbatim
   line.
7. **Report writer** — two output files (Markdown, JSON). Write once,
   atomically.

The tool is **single-pass and read-only**. No staged writes.

## 10. Calibration on the pilot

`Chap18/ArraySeqStEph.rs` is the documented pilot. Expected `--detect`
output (read from the current file, verified by hand):

| # | Class | Lines | Notes |
|---|-------|-------|-------|
| 1 | D1 | 955–961 | `ArraySeqStEphGhostIterator` |
| 2 | D2 | 963–966 | `View for …GhostIterator` |
| 3 | D3 | 998–1003 | `ForLoopGhostIteratorNew for …Iter` |
| 4 | D4 | 1005–1038 | `ForLoopGhostIterator for …GhostIter` |
| 5 | D5 | 1133–1141 | Debug + Display outside-`verus!` |
| 6 | D6 | 946–948 | `ArraySeqStEphIter<'a, T>` wrapper |
| 7 | D7 | 950–953 | `View for ArraySeqStEphIter` |
| 8 | D8 | 972–996 | `Iterator for ArraySeqStEphIter` (incl. `ensures`) |
| 9 | D10 | 968–970 | `iter_invariant<'a, T>` |
| 10 | T1+T2+T4 | 934–936 | constructor `ensures` block |
| 11 | T8 | 1045–1047 | `IntoIterator for &Self` ensures |
| 12 | T8 | 1058–1060 | `IntoIterator for Self` ensures |

Calibration: run `--detect` against the pilot and confirm every row
above appears, with no extra rows. If extra or missing, fix the
matcher before scanning the other 70 files.

## 11. Non-goals

- **Apply / rewrite.** `--detect` MUST NOT modify files. The eventual
  `--apply` mode is a separate plan, separate review, separate diff.
- **Custom iterator porting.** The 3 AVLTreeSeq files are reported as
  `U-CUSTOM` and require hand-written `IteratorSpecImpl` blocks. The
  tool does not synthesize those — it only flags them.
- **Loop-body semantic rewrites.** Invariants that change *meaning*
  (e.g., from index-based to remaining-based) are out of scope.
- **Verus version detection.** The tool assumes verus 0.2026.05.21
  semantics. Earlier versions are out of scope.

## 12. Risks and open questions

1. **verus! macro parsing.** Whether to splice into a synthesized
   `mod` (one-time setup cost, accurate AST) or to walk the token tree
   directly (less setup, more bespoke matching). Pick whatever other
   veracity-* tools already do; keep the dependency surface uniform.
2. **Chained-iterator ordering.** A file whose `*Iter` wrapper holds
   another APAS `*Iter` (15 files, U-CHAIN) cannot have D6 applied
   until the backing file has been migrated. `--detect` does not
   enforce ordering — it reports `U-CHAIN` so the migration round can
   schedule.
3. **`iter_invariant` predicate reuse.** Some files reference
   `iter_invariant(&it)` from *another* module's `iter_invariant`
   (cross-file). Path resolution in the matcher must be careful here;
   misclassification would either delete a still-needed predicate or
   miss a deletable one. Calibration set should include at least one
   cross-file caller (the chained wrappers in Chap05/06).
4. **T8 multi-line `new` text.** Markdown table rendering with
   newlines in cells is awkward. Either (a) emit T8 transforms outside
   the per-file table as a separate "constructor ensures rewrites"
   subsection, or (b) accept multi-line cells. Decide before
   implementation.

## 13. Acceptance criteria

- Running `veracity-iterator-upgrade --detect` on the APAS-VERUS root
  produces Markdown, JSON, and compile-format reports under `analyses/`.
- All 71 collection files appear in the per-chapter summary table.
- Pilot calibration (§10) matches by hand-inspection.
- The 3 custom files appear as `U-CUSTOM` (no D6–D10 against them).
- Tool exits `0` with findings, `2` only on parse failure, `3` only on
  bad invocation.
- Zero source files modified by the run (verified with `git status`).
- `M-x compile RET cat analyses/iterator-upgrade-detect.compile RET`
  followed by `C-x \`` steps through every finding, jumping the buffer
  to the right line and column.

## 14. See also

- `src/standards/prophetic_iterators_standard.rs` — the target shape.
- `docs/PropheticIterators.md` — variant complexity table and the
  71-iterator inventory.
- `plans/verus-0.2026.05.21-iterator-migration.md` — schedule and
  decisions.
- `~/projects/verus/source/vstd/std_specs/iter.rs` — `IteratorSpec` /
  `IteratorSpecImpl` definitions.
- `~/projects/verus/examples/guide/iterators.rs` — the canonical
  `VecIterator` example.
