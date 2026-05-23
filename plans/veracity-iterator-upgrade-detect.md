# Plan — `veracity-iterator-upgrade --detect`

Status: **PHASE 1 PARTIAL** — core matchers shipped 2026-05-23 (veracity
commit `6bb41ef`); enhanced report layout (§7 wide-MD, manifest check,
legend, `Iter` column, aggregate U-by-class, U-OTHER clustering,
chain-ordering appendix, `tool_sha`) is Phase 1.5 work, tracked in §13a.
Author: 2026-05-22 (revised 2026-05-23).

A read-only veracity tool that scans APAS-VERUS for the obsolete
`ForLoopGhostIterator` iterator model and emits a structured report of
(a) what must be **deleted** and (b) which invariant lines can be
**transformed line by line** to the new prophetic-iterator model (verus
0.2026.05.21, PR #2163).

**Phase 1 results (veracity fixture, 2026-05-23):** 50 iterator files
scanned, D=500 T=206 U=156. Goldens committed to
`~/projects/veracity/analyses/iterator-upgrade-detect.{md,json,compile}`
(tracked in veracity) and also produced under
`tests/fixtures/APAS-VERUS/analyses/` per §0. The fixture is stale vs.
the 71-file inventory in `docs/PropheticIterators.md` — Phase 1.5 (§13a)
adds the manifest-check gate that will surface the 21-file gap loudly.

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
  against a checked-in golden at
  `~/projects/veracity/tests/fixtures/APAS-VERUS/analyses/iterator-upgrade-detect.golden.compile`
  (the compile-format output is the most diffable). Regression test is
  `diff -u <golden> <produced>` — exit 0 means the matchers haven't
  drifted. JSON and Markdown goldens are nice-to-have but not the
  primary gate.

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
veracity-iterator-upgrade --detect  --root PATH
                                   [--ignore GLOB ...]
                                   [--format md|json|compile|all]
                                   [--out-dir PATH]
```

- `--root PATH` — **REQUIRED, no default.** See §0; the tool refuses to
  run without an explicit `--root` so it cannot accidentally scan the
  live tree.
- `--ignore GLOB` — default ignore list: `src/standards/**`,
  `src/experiments/**`, `rust_verify_test/**`, `target/**`,
  `analyses/**`, `logs/**`, `docs/**`, `**/*.golden.compile`,
  `**/*.golden.md`, `**/*.golden.json`.
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
  struct, where `fn next` has an `ensures` clause. **Single finding**
  spanning the whole `impl` block. No internal split between impl head
  and `ensures` body in `--detect` (that split, if it ever matters,
  belongs to `--apply`).
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

- recognise the iterator binder by **literal name `it` only**. APAS
  convention names every iterator `it` (see the Chap18 pilot and every
  other constructor); anything else (`iter`, `i`, custom names) drops
  to `U-OTHER`. This is intentional — narrow matchers, fail closed.
- recognise `it@` as a method receiver (View::view call) followed by
  `.0` / `.1` field access on the tuple result;
- bind the metavariable `<expr>` to a balanced expression up to the
  next top-level comma or semicolon. **Balancing tracks `(`, `[`, `{`,
  AND `<…>` when the `<` is unambiguously a generic argument list
  (immediately after a path expression).** `Foo<A, B>` must not split
  on its inner comma.
- recognise `iter_invariant` as a call **defined in the same file**
  (`ItemFn` already collected by the D10 matcher). A call whose
  callee is `iter_invariant` but resolves to a different file (or
  whose path is qualified, e.g. `other::iter_invariant`) drops to
  `U-OTHER` with reason `cross-file iter_invariant — verify referent`.
  `verus_syn` does no path resolution; cross-file deduction is
  unreliable and we refuse to guess.

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

The Markdown report MUST be wide-rendered. Inline the same `<style>`
block used at the top of `docs/PropheticIterators.md` as the first
non-empty content in the file:

```html
<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>
```

This forces GitHub/most markdown renderers to use the full viewport
width — non-negotiable. Cells may still wrap, but the table itself
must not be column-constrained.

**Path format in Markdown:** drop the `src/` prefix in every path
column. Show `Chap05/MappingStEph.rs`, not
`src/Chap05/MappingStEph.rs`. The JSON output keeps full paths; the
compile-format log keeps full paths (Emacs needs them). The Markdown is
for human reading and the `src/` prefix is dead weight.

The previous "40-character cap" rule is dropped for the wide layout;
truncate only when a single cell exceeds ~80 chars, and footnote the
overflow.

### Required top-matter (in order)

The Markdown report MUST open with these sections, in this order,
before per-file findings:

1. **Manifest check.** Compare the scanned-file set against
   `docs/PropheticIterators.md`'s 71-file inventory. Emit a one-line
   header: `Scanned N of 71 (… missing, … extra)`. If non-zero, follow
   with a small table of missing files (chap, file) and extra files
   (chap, file, reason). A run that does not match the manifest is
   announced loudly — silent partial runs caused the May 2026 false
   sense of completeness on the first veracity report.
2. **Legend.** A table with one row per U-code explaining what it
   means and what action it implies. Required keys: U-OTHER, U-CHAIN,
   U-CUSTOM, U-CLASS, U-LOOP, U-POST, U-MULTI. Format:

   ```
   | # | Code | Means | Action |
   |---|------|-------|--------|
   | 1 | U-OTHER  | `it`-bearing clause matched no T1–T8 template | Extend matcher or hand-fix |
   | 2 | U-CHAIN  | Chained-wrapper iterator; type depends on backing | Order migration: backing first |
   | 3 | U-CUSTOM | File is pinned-custom; needs hand-written IteratorSpecImpl | Manual port, not mechanical |
   | 4 | U-CLASS  | File matchers see as custom but plan pinned as delegated | Reconcile pin list vs matcher D6 rule |
   | … |
   ```

3. **Per-file summary table** — adds an `Iter` column showing the
   line of the file's `*Iter` struct definition (or `—` if delegated
   to a bare std iter type and there is no `*Iter`). Existing columns
   stay. Short paths.

   ```
   | # | Chap | File | Iter | Style | D | T | U |
   |---|------|------|------|-------|---|---|---|
   | 1 | 05 | Chap05/MappingStEph.rs | 505 | delegated | 12 | 3 | 5 |
   | … |
   ```

4. **Aggregate unresolved-by-class table.** A roll-up across all
   files. Required.

   ```
   | # | Code | Count | Files affected |
   |---|------|-------|----------------|
   | 1 | U-OTHER  | 111 | 38 |
   | 2 | U-CHAIN  |  19 | 19 |
   | 3 | U-CUSTOM |  18 |  3 |
   | 4 | U-CLASS  |   8 |  8 |
   | … |
   ```

5. **U-OTHER pattern clustering.** Required. The 111 U-OTHER findings
   in the first report reduced to six syntactic shapes via two trivial
   substitutions (`it@.0 → it.index()`, `it@.1 → it.seq()`). The tool
   MUST produce this clustering, not leave it to the reader.
   Clustering rule: normalize each U-OTHER message by stripping
   identifiers and literals down to `<ident>`/`<lit>`, then group by
   the resulting skeleton. Show the top patterns with counts and a
   suggested new form:

   ```
   | # | Skeleton | Count | Suggested new form |
   |---|----------|-------|--------------------|
   | 1 | `it@.1.no_duplicates()`            | 16 | `it.seq().no_duplicates()` |
   | 2 | `it@.1.map(...).to_set() == <e>`  | 12 | `it.seq().map(...).to_set() == <e>` |
   | 3 | `it@.0 <= <ident>.len()`           | 30 | `it.index() <= <ident>.len()` |
   | 4 | `it@.1.len() == <expr>`            | 12 | `it.seq().len() == <expr>` |
   | 5 | `<ident> == it@.1` (ghost alias)   |  4 | `<ident> == it.seq()` |
   | 6 | `forall \|i\| ... it@.1[i] ...`    | 30 | substitute throughout |
   ```

   Show at least the top 10 skeletons or all skeletons with count ≥ 2,
   whichever is larger. A cluster with count ≥ 5 is a strong signal
   the matcher should be extended to cover it (file a follow-up T-class
   in the next plan revision).

6. **Chain-ordering appendix.** Required (was nice-to-have; not
   anymore). See §7.5 — the appendix MUST be in every report that has
   any U-CHAIN findings.

### Per-file Markdown report

For each file the tool emits a section:

```
## Chap18/ArraySeqStEph.rs (delegated) — Iter@946

Deletions (6):
| # | Class | Item | Lines |
|---|-------|------|-------|
| 1 | D1 | ArraySeqStEphGhostIterator           | 955–961 |
| 2 | D2 | View for ArraySeqStEphGhostIterator  | 963–966 |
| 3 | D3 | ForLoopGhostIteratorNew for …Iter    | 998–1003 |
| 4 | D4 | ForLoopGhostIterator for …GhostIter  | 1005–1038 |
| 5 | D5 | Debug for …GhostIterator             | 1133–1137 |
| 6 | D5 | Display for …GhostIterator           | 1139–1143 |

Transforms (3):
| # | Class | Line | Old | New |
|---|-------|------|-----|-----|
| 1 | T1 | 934 | it@.0 == 0,                       | IteratorSpec::remaining(&it).len() == it.seq().len(), |
| 2 | T2 | 935 | it@.1 == self.seq@,               | it.seq() == self.seq@,                                |
| 3 | T4 | 936 | iter_invariant(&it),              | <remove>                                              |

Constructor `ensures` rewrites (T8) (2):

```
src/Chap18/ArraySeqStEph.rs:1045 — IntoIterator for &Self
  old:
    it@.0 == 0,
    it@.1 == self.seq@,
    iter_invariant(&it),
  new:
    IteratorSpec::remaining(&it) == self.seq@.as_ref(),
    IteratorSpec::decrease(&it) is Some,
    IteratorSpec::initial_value_relation(&it, &it),
```

(One block per T8 finding, kept out of the table so the markdown stays
single-line clean.)

Unresolved (0):
```

Table cells capped at 40 characters; longer text goes to a footnote
section below the table. Multi-line `new` text (T8) is emitted in the
dedicated subsection above, not in the table. In the compile-format
log T8 stays on a single line with `\n` escapes — Emacs `next-error`
jumps to the start of the old span and the user reads the expansion in
the message.

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
  "tool_sha": "<git SHA of veracity binary at build time>",
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
# tool_sha: <git SHA of veracity binary at build time>
# generated: 2026-05-22T13:14:15Z
# totals: files=71 D=426 T=284 U=3
```

### 7.5 U-CHAIN ordering appendix (REQUIRED)

When the matcher walks a chained wrapper (D6 whose only field is
another APAS `*Iter`), record the edge `wrapper → backing-collection`.
At report-write time, run a topological sort over the resulting graph
and emit a chain-ordering appendix at the bottom of the Markdown
report:

```
## Chain ordering (15 chained wrappers)

| Layer | File | Backing |
|------:|:-----|:--------|
| 1 | (independent) | std::slice / std::vec / hash_set / hash_map |
| 2 | Chap05/SetStEph.rs | HashSetWithViewPlus |
| 3 | Chap05/RelationStEph.rs | Chap05/SetStEph.rs |
| 4 | Chap05/MappingStEph.rs | Chap05/RelationStEph.rs |
| 4 | Chap06/DirGraphStEph.rs | Chap05/SetStEph.rs |
| … |
```

Files at the same layer can be migrated in parallel; a layer-`k+1`
file must wait for its backing layer-`k` file. The JSON `summary` block carries the same
edge list for orchestrator scripts.

If the chain graph has a cycle (shouldn't happen, but matchers can
misclassify), the appendix prints the cycle as a single `error:` line
in the compile log: `U-CHAIN: cycle detected: A → B → A`.

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
   `verus!` macro. Use the parsing stack veracity already depends on:
   `ra_ap_syntax` to locate the `verus!` macro span, then
   `verus_syn::parse_file` on the body, then a `verus_syn::visit::Visit`
   walk for the matchers. Reference exemplar: `src/bin/full_generic_feq.rs`
   in the veracity repo. **Do not add new parsing infrastructure** — the
   string-hacking ban requires AST-only matching, and the existing crates
   already provide it.
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
   that mentions the literal identifier `it` but matches no T-pattern
   is bucketed as `U-OTHER` with the verbatim line. Non-`it` binders
   are not iterator-binders by our convention and are ignored.
7. **Matcher unit tests** — one minimal `.rs` fixture per T-class
   (T1–T8) and per D-class (D1–D10) under
   `tests/fixtures/iterator-upgrade-detect/{T1,T2,...,D1,...}.rs`,
   each ~15 lines, each with a checked-in `*.golden.compile` output.
   `cargo test` invokes the binary on each fixture and `diff -u` against
   the golden. Adds maybe a day; saves debugging cycles every time the
   matchers drift.
8. **Report writer** — three output files (Markdown, JSON,
   compile-format). Write once, atomically.

The tool is **single-pass and read-only**. No staged writes.

## 10. Calibration on the pilot

`Chap18/ArraySeqStEph.rs` is the documented pilot. **Observed
`--detect` output (Phase 1, 2026-05-23)** — line numbers reflect the
fixture's current source (they are offset by ~1 from the per-hand
numbers in this plan because the matcher anchors at the AST item's
first attribute, not the `pub` keyword):

| # | Class | Lines | Notes |
|---|-------|-------|-------|
|  1 | D6  | 944–947   | `ArraySeqStEphIter<'a, T>` wrapper |
|  2 | D7  | 949–952   | `View for ArraySeqStEphIter` |
|  3 | D1  | 954–960   | `ArraySeqStEphGhostIterator` |
|  4 | D2  | 962–965   | `View for …GhostIterator` |
|  5 | D10 | 967–969   | `iter_invariant<'a, T>` |
|  6 | D8  | 971–995   | `Iterator for ArraySeqStEphIter` (single finding) |
|  7 | D3  | 997–1002  | `ForLoopGhostIteratorNew for …Iter` |
|  8 | D4  | 1004–1037 | `ForLoopGhostIterator for …GhostIter` |
|  9 | D9  | 1120–1124 | `Debug for ArraySeqStEphIter` (outside `verus!`) |
| 10 | D9  | 1126–1130 | `Display for ArraySeqStEphIter` (outside `verus!`) |
| 11 | D5  | 1132–1136 | `Debug for …GhostIterator` (outside `verus!`) |
| 12 | D5  | 1138–1142 | `Display for …GhostIterator` (outside `verus!`) |
| 13 | T8  | 935       | constructor #1 `iter()` ensures triple |
| 14 | T8  | 1046      | constructor #2 `IntoIterator for &Self` ensures triple |
| 15 | T1  | 1057      | `it@.0 == 0` (constructor #3, no `iter_invariant`) |
| 16 | T2  | 1058      | `it@.1 == self.seq@` (constructor #3) |

**One row per item.** D5 emits two findings (Debug and Display), as
does D9. D8 emits a single finding spanning the whole impl. The third
constructor (`IntoIterator for Self`) has only `it@.0 == 0, it@.1 ==
self.seq@,` (no `iter_invariant(&it)`) so T8 does NOT fire there — T1
and T2 fire individually, which produces an identical end state under
`--apply`. The original plan §10 row 12 implicitly conflated these two
behaviours; the matcher's stricter interpretation per §5 ("T8 fires
when `iter_invariant(&it)` is present") is correct.

**Pilot scorecard:** 12 D, 4 T (2 T8 + T1 + T2), 0 U. Matches the §4/§5
spec exactly.

Calibration: run `--detect` against the pilot and confirm every row
above appears, with no extra rows. The shipped binary passes.

## 11. Non-goals

- **Apply / rewrite.** `--detect` MUST NOT modify files. The eventual
  `--apply` mode is a separate plan, separate review, separate diff.
  An intermediate `--dry-run-apply` (emits unified diffs per file,
  still no source mutation) is the planned Phase 2 — see §14.
- **Custom iterator porting.** The 3 AVLTreeSeq files are reported as
  `U-CUSTOM` and require hand-written `IteratorSpecImpl` blocks. The
  tool does not synthesize those — it only flags them.
- **Loop-body semantic rewrites.** Invariants that change *meaning*
  (e.g., from index-based to remaining-based) are out of scope.
- **Verus version detection.** The tool assumes verus 0.2026.05.21
  semantics. Earlier versions are out of scope.
- **Verifying the proposed `new` lines.** `--detect` shows the
  template-generated `new` text; it does NOT prove it verifies under
  Verus. That's `--apply` + `scripts/validate.sh` territory in Phase 2.
- *(removed: U-CHAIN ordering appendix is now §7.5 required content,
  not a non-goal.)*

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
3. **`iter_invariant` predicate reuse — RESOLVED.** Cross-file
   `iter_invariant` calls are reported as `U-OTHER` with reason
   `cross-file iter_invariant — verify referent`. `verus_syn` does no
   path resolution; we refuse to guess. Calibration set must include
   at least one cross-file caller (the chained wrappers in Chap05/06)
   to confirm they reach `U-OTHER`, not silent miss.
4. **T8 multi-line `new` text — RESOLVED.** Markdown: dedicated
   "Constructor `ensures` rewrites" subsection per file (option (a),
   see §7). Compile-format: single line with `\n` escapes.

## 13. Acceptance criteria

Each item is annotated with Phase 1 status — ✅ passing, ⚠ partial,
❌ not yet.

- ✅ Running `veracity-iterator-upgrade --detect` on the APAS-VERUS
  root produces Markdown, JSON, and compile-format reports under
  `analyses/`.
- ⚠ All 71 collection files appear in the per-chapter summary table.
  Phase 1 sweep saw **50 of 71** — the veracity fixture is stale
  relative to the live tree. Phase 1.5 adds the manifest-check gate
  (§13a #1) that will surface this as a loud header row instead of a
  silent partial.
- ✅ Pilot calibration (§10) matches.
- ✅ The 3 custom files (`Chap37/AVLTreeSeq{,StEph,StPer}.rs`) reach
  `U-CUSTOM` for D6–D10 (the wrapper, View, Iterator impl, Debug/Display
  outside `verus!`, and `iter_invariant` predicate).
- ✅ Tool exits `0` with findings, `2` only on parse failure, `3` only
  on bad invocation.
- ✅ Zero source files modified by the run (`git status` clean in the
  fixture after a sweep).
- ✅ `M-x compile RET cat analyses/iterator-upgrade-detect.compile RET`
  followed by `` C-x ` `` steps through every finding (GNU error format
  verified by hand).

**Markdown report acceptance — all six required (Phase 1.5):**

- ❌ The report opens with the wide-MD `<style>` block from §7.
- ❌ The report opens with a **manifest check** line: `Scanned N of 71`.
  A run with `N < 71` lists the missing files explicitly.
- ❌ The report includes a **legend table** for every U-code that
  appears in the report.
- ❌ The per-file summary table includes the **`Iter` column** (line of
  the `*Iter` struct, or `—` for bare-std-iter files), and uses
  **short paths** (no `src/` prefix). Current report uses long paths.
- ❌ The report includes an **aggregate "Unresolved by class"** table
  (rows summed across all files).
- ❌ The report includes a **U-OTHER pattern clustering** table with at
  least the top 10 skeletons (or all skeletons with count ≥ 2,
  whichever is larger) and a suggested new form per row.
- ❌ The report includes the **chain-ordering appendix** (§7.5) whenever
  any U-CHAIN finding exists.

JSON header acceptance:

- ❌ `tool_sha` field carrying the veracity binary's git SHA at build
  time. Phase 1 emits `tool`, `mode`, `root`, `generated`, but not
  `tool_sha`.

## 13a. Phase 1.5 — close the report-layout gaps

Eleven gaps between the Phase 1 ship and the §7 / §13 spec. Each
subsection below names the gap, the precise output it must produce,
the data the matcher already has (or needs), and the acceptance
check.

Phase 1.5 lands as a single PR (or a string of small ones) before
Phase 2 starts. Gating principle: Phase 2 (`--dry-run-apply`) is
worth more once the report layout is information-dense enough that a
reviewer can scope the migration without leaving the Markdown.

Order in which to ship them: §13a.1 (manifest) first — without it,
every other downstream consumer treats a 50-file run as a 71-file run.
Then §13a.2–§13a.6 are independent layout fixes; ship them in a single
commit. §13a.7 (`tool_sha`) is a build-system change, separate commit.
§13a.8–§13a.9 (chain ordering, U-OTHER clustering) are real algorithm
work, separate commit each. §13a.10 (unit tests) gates everything.
§13a.11 (fixture refresh) is APAS-VERUS-side, not veracity-side.

### 13a.1 Manifest check

**Spec.** At startup, read
`{root}/docs/PropheticIterators.md` and extract the 71-file
inventory (every row whose first cell is a numeric index and whose
second cell starts with `src/`). After the scan, compare the
file-set the matcher reached against the inventory; emit two diffs:
**missing** (in inventory, not scanned) and **extra** (scanned, not
in inventory).

**Markdown output, top of report, between the totals line and the
per-file summary:**

```
## Manifest check

Scanned 50 of 71 inventory files. 21 missing, 0 extra.

### Missing (21)

| # | Chap | File |
|--:|------|------|
| 1 | 06 | DirGraphMtPer.rs |
| 2 | 07 | RegExpMatcherStEph.rs |
| … |
```

**JSON output.** Top-level `manifest` block:

```json
"manifest": {
  "inventory_path": "docs/PropheticIterators.md",
  "inventory_count": 71,
  "scanned_count": 50,
  "missing": ["src/Chap06/DirGraphMtPer.rs", ...],
  "extra":   []
}
```

**Compile-format output.** One line at the top:

```
docs/PropheticIterators.md:1:1: warning: manifest: scanned 50 of 71 — 21 missing, 0 extra
```

**Fallback when the inventory file is absent** (older fixture, or run
against a non-APAS-VERUS root): emit `Scanned N of ?` and skip the
missing/extra tables. Do not error.

**Acceptance.** On the current Phase 1 sweep, the line reads
`Scanned 50 of 71 — 21 missing, 0 extra`. After the fixture refresh
(§13a.11), it reads `Scanned 71 of 71`.

### 13a.2 Short paths in Markdown

**Spec.** In every Markdown table cell that contains a file path,
strip the leading `src/` segment. So `src/Chap05/MappingStEph.rs` →
`Chap05/MappingStEph.rs`. Per-file section headings (`### …`) also
use the short form.

**Out of scope.** JSON keeps absolute paths (machine readers don't
need pretty). Compile-format keeps absolute paths (Emacs
`compilation-error-regexp-alist` needs them to resolve via
`compilation-search-path`).

**Acceptance.** `grep -c "src/Chap" iterator-upgrade-detect.md` is 0
in Markdown but unchanged in JSON / compile.

### 13a.3 `Iter` column in per-file summary

**Spec.** The per-file summary table grows a column showing the line
number of the file's `*Iter` wrapper struct (the D6 finding's
`line_start`). For files that have no `*Iter` struct (bare std-iter
delegation) write `—`.

```
| # | Chap | File | Iter | Style | D | T | U |
|---|------|------|-----:|-------|--:|--:|--:|
| 1 | 05 | Chap05/MappingStEph.rs | 505 | delegated | 12 | 3 | 5 |
| 2 | 18 | Chap18/ArraySeqStEph.rs | 944 | delegated | 12 | 4 | 0 |
```

**Data.** Already tracked — the D6 finding contains the line. Just
read the first D6 finding per file at write time. If no D6 exists,
emit `—`.

**Acceptance.** Pilot row shows `944` in the Iter column.

### 13a.4 Legend table

**Spec.** Below the manifest check, emit a legend table for every
U-code that appears at least once in the report:

```
## Legend

| # | Code | Means | Action |
|---|------|-------|--------|
| 1 | U-OTHER  | `it`-bearing clause matched no T1–T8 template | Extend matcher or hand-fix |
| 2 | U-CHAIN  | Chained-wrapper iterator; backing must migrate first | Schedule per chain appendix |
| 3 | U-CUSTOM | File is pinned-custom; needs hand-written IteratorSpecImpl | Manual port, not mechanical |
| 4 | U-CLASS  | Matcher saw custom but pin says delegated (or vice versa) | Reconcile pin list vs D6 rule |
```

Filter rows to codes that have count ≥ 1 in this run. Row text is
fixed (canonical).

**Acceptance.** Every U-code mentioned downstream in the report
appears here exactly once with non-empty Action.

### 13a.5 Aggregate "Unresolved by class" table

**Spec.** Below the legend, a roll-up of `u.code` across every file:

```
## Unresolved by class

| # | Code | Count | Files affected |
|---|------|------:|---------------:|
| 1 | U-OTHER  | 111 | 38 |
| 2 | U-CHAIN  |  19 | 19 |
| 3 | U-CUSTOM |  18 |  3 |
| 4 | U-CLASS  |   8 |  8 |
```

`Count` is the total number of findings; `Files affected` is the
number of distinct files contributing ≥ 1 finding of that code.

**Acceptance.** Sum of `Count` column equals the grand-total `U` in
the manifest line.

### 13a.6 Wide-MD `<style>` block

**Spec.** The very first content of the Markdown report (before
`# Iterator-Upgrade Detect Report`) is the verbatim style block:

```html
<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
.container, .container-lg, .container-xl, main, article { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>
```

GitHub's default Markdown renderer constrains content to a centered
~900px column; this block overrides that for renderers that respect
inline `<style>` (GitHub does for repo-page MD via the gh-md-toc
CSS, plus most local previewers).

**Acceptance.** `head -10 iterator-upgrade-detect.md | grep -c "max-width"` ≥ 4.

### 13a.7 JSON `tool_sha`

**Spec.** Add a `tool_sha` field to the JSON header block, carrying
the veracity binary's git commit SHA at build time:

```json
{
  "tool": "veracity-iterator-upgrade",
  "tool_sha": "6bb41ef9...",
  "mode": "detect",
  ...
}
```

**Build wiring.** A `build.rs` in the veracity crate runs
`git rev-parse HEAD` and writes the result to
`env::set_var("GIT_HASH", …)` via `cargo:rustc-env=GIT_HASH=...`.
The binary reads it with `env!("GIT_HASH")`. Fall back to the literal
string `"unknown"` if the build is not in a git checkout.

**Why.** Two reports from different binaries should be diffable; the
SHA lets a reviewer prove they're comparing apples to apples.

**Acceptance.** `jq .tool_sha iterator-upgrade-detect.json` returns a
40-char hex string for any build inside a git checkout.

### 13a.8 Chain-ordering appendix (§7.5)

**Spec.** During the D6 visit, when the wrapper's sole non-PhantomData
field is another APAS `*Iter`, record an edge `(this_file,
backing_file)`. (Resolve `backing_file` by matching the
type's last-segment ident against the file's own use-list — if the
ident appears in a `use crate::ChapNN::...::*Iter` line, the file is
`ChapNN/…`. If unresolved, record the edge with `backing_file =
<unresolved:ident>` and emit a compile-log `warning:` line for that
edge.)

At write time, do a topological sort over the edge set. Emit an
appendix at the bottom of the Markdown report:

```
## Chain ordering (19 chained wrappers)

| Layer | File | Backing |
|------:|:-----|:--------|
| 1 | (independent) | std::slice / std::vec / hash_set / hash_map |
| 2 | Chap05/SetStEph.rs | HashSetWithViewPlus |
| 3 | Chap05/RelationStEph.rs | Chap05/SetStEph.rs |
| 4 | Chap05/MappingStEph.rs | Chap05/RelationStEph.rs |
| 4 | Chap06/DirGraphStEph.rs | Chap05/SetStEph.rs |
| … |
```

Files at the same layer can migrate in parallel; a layer-`k+1` file
must wait for its layer-`k` backing. JSON `summary.chain_edges`
carries `[{wrapper, backing, layer}, ...]` for orchestrator scripts.

**Cycle handling.** If the chain graph has a cycle (shouldn't happen
but matchers can misclassify), emit `U-CHAIN: cycle detected: A → B → A`
to the compile log as an `error:` line, AND mark every node on the
cycle with layer `?` in the appendix.

**Acceptance.** The chain appendix lists every U-CHAIN file; sum of
appendix rows equals the U-CHAIN count from §13a.5.

### 13a.9 U-OTHER pattern clustering

**Spec.** Normalize each U-OTHER message to a skeleton, group by
skeleton, sort by count, emit a top-N table. The clustering rule:

1. Take the U-OTHER `message` field (the rendered clause text after
   the `unrecognized 'it'-bearing clause:` prefix).
2. Tokenize via `verus_syn` (same expression that was emitted).
3. Walk the AST. Replace every `Expr::Path` whose segments are a
   single ident NOT in {`it`, `self`, `Self`, `old`, `IteratorSpec`,
   `Some`, `None`, `true`, `false`} with the literal placeholder
   `<ident>`. Replace every `Expr::Lit` (integer, string, float, bool,
   char) with `<lit>`. Re-render to canonical-spacing source via
   `render_expr` (already implemented).
4. Group findings by the resulting skeleton string. Sort by count
   descending.

Emit the table below the aggregate "Unresolved by class" table:

```
## U-OTHER patterns (top N)

| # | Skeleton | Count | Suggested new form |
|--:|----------|------:|--------------------|
| 1 | `it@.1.no_duplicates()`              | 16 | `it.seq().no_duplicates()` |
| 2 | `it@.1.map(<ident>).to_set() == <ident>` | 12 | `it.seq().map(<ident>).to_set() == <ident>` |
| 3 | `it@.0 <= <ident>.len()`             | 30 | `it.index() <= <ident>.len()` |
| 4 | `it@.1.len() == <expr>`              | 12 | `it.seq().len() == <expr>` |
| 5 | `<ident> == it@.1`                   |  4 | `<ident> == it.seq()` |
| 6 | `forall \|i\| ... it@.1[i] ...`      | 30 | substitute throughout |
```

Show at least the top 10 skeletons, OR all skeletons with count ≥ 2,
whichever is larger.

**Suggested new form.** A static rewrite based on the two trivial
substitutions: `it@.0 → it.index()`, `it@.1 → it.seq()`. Apply both
substitutions to the skeleton text; emit the result. If the skeleton
mentions neither, leave the suggested cell empty.

**Promotion rule.** Any cluster with count ≥ 5 is a strong signal
that the matcher should grow a new T-class to cover it. Such clusters
are flagged with `→ T(new)` in a footnote column or comment.

**Acceptance.** Sum of `Count` column equals the total U-OTHER count
from §13a.5. Top row's count is ≥ 5 (otherwise the clustering didn't
buy anything and we'd ship as-is).

### 13a.10 Matcher unit-test fixtures

**Spec.** Per-T-class and per-D-class fixtures, ~15 lines each, with
checked-in goldens. Layout:

```
~/projects/veracity/tests/fixtures/iterator-upgrade-detect/
├── D1_ghost_iterator_struct.rs
├── D1_ghost_iterator_struct.golden.compile
├── D2_view_for_ghost.rs
├── D2_view_for_ghost.golden.compile
├── ...
├── D10_iter_invariant_predicate.rs
├── D10_iter_invariant_predicate.golden.compile
├── T1_index_eq_lit.rs
├── T1_index_eq_lit.golden.compile
├── T2_view_eq_self_seq.rs
├── T2_view_eq_self_seq.golden.compile
├── ...
├── T8_constructor_triple.rs
├── T8_constructor_triple.golden.compile
└── README.md            # what each fixture is for
```

Each `.rs` is a minimal verus! block containing exactly the pattern
that should fire one matcher. Each `.golden.compile` is the expected
output of `veracity-iterator-upgrade --detect --root <fixture-dir>
--out-dir /tmp/x` filtered to this file's findings.

**Test wiring.** A `cargo test` integration test
(`tests/iterator_upgrade_matchers.rs`) iterates the fixture directory,
runs the binary on each, and `diff -u` against the golden. Any drift
in matcher output fails the test.

**Acceptance.** `cargo test --test iterator_upgrade_matchers` is
green; deleting any one matcher and re-running fails ≥ 1 test.

### 13a.12 Unique transforms — single unified table (REQUIRED)

The per-file Transforms tables and the §13a.9 U-OTHER clustering table
together repeat the same handful of shapes hundreds of times. On the
2026-05-23 sweep, `it@.1 == wa_seq` alone fires 70+ times across the
Chap06 weighted-graph files. A reader paging through per-file tables
sees the same `old | new` row over and over with no sense of how many
distinct rewrites are actually needed.

**Spec.** Add a single global table near the top of the Markdown
report (immediately after the §13a.5 "Unresolved by class" table) that
unifies *every* `it`-bearing rewrite the tool sees, deduped by
skeleton. This replaces the §13a.9 U-OTHER-only clustering with a
broader view: matched (T1–Tn) and unmatched (U-OTHER) shapes appear in
the same table.

**Source set.** Every finding that is either:

- a `transforms[*]` entry (any T-class), OR
- an `unresolved[*]` entry with `code == "U-OTHER"`.

The §13a.9 clustering rule (ident/literal normalization through
`verus_syn` re-parse + `render_expr`) applies unchanged. Group by
skeleton.

**Strict `it` filter.** A finding whose normalized skeleton contains
no token `it` is dropped from this table. APAS convention names every
iterator literal `it`; a clause without `it` is not iterator-bearing
and must not appear here. (This already matches the §5 / §6 matcher
rules; the table-write step just enforces it as a belt-and-braces
check against future matcher drift.)

**Output:**

```
## Unique transforms

50 distinct it-bearing rewrites across the codebase
(total findings: 743; T-matched: 395; U-OTHER: 348).

| # | Status | Old skeleton | New skeleton | Count | Files |
|--:|--------|--------------|--------------|------:|------:|
| 1 | T3        | `it@.1 == <ident>`               | `it.seq() == <ident>,`                              | 72 | 14 |
| 2 | T6        | `<ident>.len() - it@.0`          | `IteratorSpec::decrease(&it).unwrap(),`             | 68 | 14 |
| 3 | U-OTHER   | `it@.0 <= <ident>.<ident>()`     | `it.index() <= <ident>.<ident>()`                   | 112 | 21 |
| 4 | T4        | `iter_invariant(&it)`            | `<remove>`                                          | 38 | 30 |
| 5 | U-OTHER   | `it@.1.<ident>()`                | `it.seq().<ident>()`                                | 19 |  9 |
| … |
```

Columns:

- **Status.** `T1`–`Tn` for matched shapes, `U-OTHER` for unmatched
  (with the §13a.9 promotion tag `→ T(new)` when count ≥ 5).
- **Old skeleton / New skeleton.** Normalized forms. New is the
  template-generated prophetic rewrite for T-classes, or the static
  `it@.0 → it.index()` / `it@.1 → it.seq()` substitution for
  U-OTHER (per §13a.9).
- **Count.** Total findings sharing this skeleton.
- **Files.** Number of distinct files contributing ≥ 1 finding.

Sort by Count descending. Show the top 50 rows, or all rows with
Count ≥ 2, whichever is larger.

**Per-file Transforms tables stay.** The unified table is a global
view; per-file detail keeps line numbers and exact `old` strings for
`M-x compile` stepping. The redundancy is the point — the unified
table tells the reader "there are N distinct shapes total" so they
know how much hand-work is actually pending.

**JSON.** Top-level `unique_rewrites` array, parallel structure:
`[{status, old_skeleton, new_skeleton, count, files}, ...]`. The
existing per-file `transforms` / `unresolved` arrays are unchanged.

**Acceptance.**

- Sum of `Count` across the table = (total T-class findings) +
  (total U-OTHER findings). Other U-codes (`U-CHAIN`, `U-CUSTOM`,
  `U-CLASS`) are not in this table — those aren't transforms, they
  are deletions or pin-list reconciliations.
- Every skeleton row in the table literally contains the token `it`.
- The table appears once, at the top of the report, between the
  "Unresolved by class" table (§13a.5) and the per-file summary
  (§13a.3).
- §13a.9 is subsumed: the existing standalone "U-OTHER patterns"
  table becomes redundant. Keep it only if it shows information not
  in the unified table — otherwise delete the standalone table to
  reduce duplication.

### 13a.11 Fixture refresh

**Out of scope for the veracity tool itself** — this is a
fixture-management task on the APAS-VERUS side. The current veracity
fixture
(`~/projects/veracity/tests/fixtures/APAS-VERUS/`) was last refreshed
some time before 2026-04. The live tree has progressed since;
21 files in the manifest are missing from the fixture.

**Procedure.**

```
# In veracity:
rm -rf tests/fixtures/APAS-VERUS
cp -r ~/projects/APAS-VERUS tests/fixtures/APAS-VERUS
# Strip the inner .git so the fixture is just a snapshot:
rm -rf tests/fixtures/APAS-VERUS/.git
# Commit the snapshot.
```

Per the memory `feedback_fixture_management.md`: rm and re-clone, never
git clean. Per memory `feedback_fixture_no_stash.md`: the fixture is
read-only — no in-place edits.

**Acceptance.** After refresh, the §13a.1 manifest line reads
`Scanned 71 of 71 — 0 missing, 0 extra`.

## 14. Phase 2 — `--dry-run-apply` and `--apply`

Phase 2 has its own plan at `plans/veracity-iterator-upgrade-apply.md`.
It adds two modes: `--dry-run-apply` (emits unified diffs per file, no
mutation) and `--apply` (rewrites files in place inside the veracity
fixture). The (a)-vs-(b) open question this section used to carry was
resolved by shipping both as separate modes.

Scope: 1198 of 1236 findings are mechanical (500 D-deletes + 480
textually-trivial T-rewrites + 218 templated T-rewrites). 38 U-class
findings (U-CUSTOM, U-CHAIN, U-CLASS) are out of scope and stay for
human review.

## 15. See also

- `src/standards/prophetic_iterators_standard.rs` — the target shape.
- `docs/PropheticIterators.md` — variant complexity table and the
  71-iterator inventory.
- `plans/verus-0.2026.05.21-iterator-migration.md` — schedule and
  decisions.
- `plans/veracity-iterator-upgrade-detect-review.md` — veracity-side
  review of this plan and the Phase 2 recommendation.
- `~/projects/verus/source/vstd/std_specs/iter.rs` — `IteratorSpec` /
  `IteratorSpecImpl` definitions.
- `~/projects/verus/examples/guide/iterators.rs` — the canonical
  `VecIterator` example.

**Veracity-side artifacts (Phase 1, 2026-05-23):**
- `~/projects/veracity/src/bin/iterator_upgrade.rs` — binary source.
- `~/projects/veracity/analyses/iterator-upgrade-detect.{md,json,compile}` — Phase 1 goldens (tracked in veracity).
- `~/projects/veracity/tests/fixtures/APAS-VERUS/analyses/iterator-upgrade-detect.{md,json,compile}` — same artifacts under the fixture root (per §0).
- `~/projects/veracity/Cargo.toml` — `[[bin]]` entry `veracity-iterator-upgrade`.
