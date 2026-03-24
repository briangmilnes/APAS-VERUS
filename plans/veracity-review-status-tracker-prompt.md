# Veracity Tool: Review Status Tracker

## Goal

Build a veracity tool `veracity-review-status` that tracks human review status of
verified source files. Adds/updates `REVIEWED:` annotations, counts review coverage,
and flags files modified after their last review date.

## Motivation

APAS-VERUS has 240+ modules verified by AI agents. Verification (Verus) proves
correctness of specs, but a human must review:
- Are the specs faithful to the textbook?
- Are the assumes/accepts justified?
- Is the code readable and well-structured?
- Do the cost annotations match APAS?

Without tracking, there's no way to know which files have been human-reviewed and
which are running on pure AI output.

## Annotation Format

Every `.rs` file in `src/Chap*/` gets a `REVIEWED:` line in the module header,
immediately after the copyright line:

```rust
//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! REVIEWED: NO

//! Brief module description.
```

After human review:

```rust
//  Copyright (C) 2025 Acar, Blelloch and Milnes from 'Algorithms Parallel and Sequential'.

//! REVIEWED: Brian Milnes <briangmilnes@gmail.com> 2026-03-24

//! Brief module description.
```

Rules:
- Exactly one `REVIEWED:` line per file
- Placed as the first `//!` line after the copyright comment
- `NO` means never reviewed
- Reviewer name + email + ISO date means reviewed on that date
- Multiple reviewers: separate `REVIEWED:` lines (one per reviewer, each with own date)
- Date is ISO 8601: `YYYY-MM-DD`

## Output Formats

### Default: Emacs compile buffer format

The default output for `report`, `stale`, and `unreviewed` is emacs compile buffer
format — one diagnostic per file, navigable with `M-x compile` / `next-error` in emacs.

Format: `<file>:<line>: <level>: <message>`

Severity levels and their meanings:

| Level | Meaning | When |
|-------|---------|------|
| `error` | No review line | File has no `REVIEWED:` annotation at all |
| `error` | Not reviewed | File has `REVIEWED: NO` (or a `REVIEWED:` line with no reviewer email) |
| `error` | Bad review line format | `REVIEWED:` line exists but doesn't parse (missing date, bad date format, etc.) |
| `warning` | File updated since review | File was modified (git) after the review date |
| `info` | Reviewed | File has a valid, current review |

`<line>` is the line number where the `REVIEWED:` annotation appears (or line 1 if
no annotation exists).

Example output:

```
src/Chap18/ArraySeqMtEph.rs:1: error: no review line
src/Chap39/BSTTreapStEph.rs:4: error: not reviewed
src/Chap43/OrderedTableStPer.rs:4: error: bad review line format: '//! REVIEWED: Brian Milnes 2026-03-24'
src/Chap18/ArraySeqStEph.rs:4: warning: file updated since review (reviewed 2026-03-01, modified 2026-03-22)
src/Chap18/ArraySeqStPer.rs:4: info: reviewed (Brian Milnes <briangmilnes@gmail.com> 2026-03-20)
```

Sorting: errors first, then warnings, then info. Within each level, sort by chapter
number then filename.

Summary line at the end (not in compile format — just a plain text line):

```
Review status: 45 reviewed, 12 stale, 5 bad format, 187 unreviewed, 5 missing, 244 total
```

### Markdown: `-m` / `--markdown`

With `-m` or `--markdown`, produce the table format described below instead of the
emacs compile format. This is for reports and documentation.

```
| # | Chap | File | Reviewed | Reviewer | Date | Stale? | Days Since |
|---|------|------|----------|----------|------|--------|------------|
| 1 | 18   | ArraySeqStEph.rs | YES | Brian Milnes | 2026-03-01 | STALE | 23 |
| 2 | 18   | ArraySeqStPer.rs | YES | Brian Milnes | 2026-03-20 | OK    | 4  |
| 3 | 18   | ArraySeqMtEph.rs | NO  | -            | -          | -     | -  |
```

Summary:
```
| # | Metric | Count | % |
|---|--------|-------|---|
| 1 | Reviewed (current) | 45 | 18% |
| 2 | Reviewed (stale)   | 12 | 5%  |
| 3 | Not reviewed       | 187 | 77% |
| 4 | Total files        | 244 | 100% |
```

Per-chapter breakdown:
```
| # | Chap | Files | Reviewed | Stale | Unreviewed | Coverage % |
```

## Commands

### 1. `veracity-review-status report`

Scan all source files, produce the report in the active output format (default: emacs
compile, or markdown with `-m`).

**Stale detection**: Compare the `REVIEWED:` date against `git log -1 --format=%ai -- <file>`.
If the file was modified after the review date, mark `STALE` / emit error.

### 2. `veracity-review-status init`

Add `//! REVIEWED: NO` to every `.rs` file in scope that doesn't already have a
`REVIEWED:` line. Idempotent — skips files that already have the annotation. This is
the bulk operation for bootstrapping review tracking across the whole codebase.

Placement: insert as the first `//!` line after the copyright block. The copyright block
is the contiguous run of `//` lines at the top of the file (before the first `//!` or
blank line).

### 2a. `veracity-review-status add <file>`

Add `//! REVIEWED: NO` to a single file that is missing a `REVIEWED:` line. Idempotent —
does nothing if the file already has one. Same placement rules as `init`. Use this to
fix individual `error: no review line` diagnostics without running `init` across
everything.

### 3. `veracity-review-status mark <file> <reviewer> [date]`

Set a file's review status:

```bash
veracity-review-status mark src/Chap18/ArraySeqStEph.rs "Brian Milnes <briangmilnes@gmail.com>"
# Uses today's date

veracity-review-status mark src/Chap18/ArraySeqStEph.rs "Brian Milnes <briangmilnes@gmail.com>" 2026-03-24
# Explicit date
```

If the file has `REVIEWED: NO`, replace it. If it has an existing review by the same
person, update the date. If it has a review by a different person, add a second
`REVIEWED:` line (preserving the existing one).

### 4. `veracity-review-status stale`

List only stale files (reviewed but modified since review). Default format: emacs
compile (one `error:` line per stale file). With `-m`: table format.

### 5. `veracity-review-status unreviewed`

List only unreviewed files. Default format: emacs compile (one `error:` line per
unreviewed file). With `-m`: table format.

## Scope

Files to track:
- All `src/Chap*/*.rs` files
- Exclude `Example*.rs` (textbook demos, not algorithmic implementations)
- Exclude `analyses/` subdirectories
- Exclude `src/standards/*.rs` (reference patterns, not implementations)
- Exclude `src/experiments/*.rs`
- Include `src/Types/Types.rs`, `src/Concurrency/Concurrency.rs`
- Include `src/vstdplus/*.rs`

## Git Integration

Stale detection uses git. The tool must:
1. Run `git log -1 --format=%aI -- <file>` to get last modification date
2. Parse the ISO date from the `REVIEWED:` annotation
3. Compare: if `git_date > review_date`, the file is stale

Edge cases:
- File has only analysis/whitespace changes since review — still counts as stale
  (conservative; human can re-mark if the changes are trivial)
- File is untracked or not in git — skip stale check, report as unreviewed
- Multiple reviewers — file is stale if modified after ALL review dates

## Implementation Notes

### Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace on Rust
source. The `init` and `mark` commands modify source files — they must use token-aware
insertion that respects the file structure. Specifically:
- Find the copyright block (contiguous `//` lines at file start)
- Find the first `//!` line after it
- Insert or replace the `REVIEWED:` line at the correct position
- Preserve all other content exactly

### Output

- Default format is emacs compile buffer (navigable with `M-x compile` / `next-error`)
- With `-m` / `--markdown`: table format for reports and documentation
- `report` writes to stdout and optionally to `analyses/veracity-review-status.log`
- All tables follow APAS-VERUS conventions: `#` index column, `Chap` column for
  file-referencing tables, max 40 chars per cell
- Exit code 0 if no errors/warnings, 1 if any errors found, 2 on tool failure

### CLI Interface

```bash
veracity-review-status report [path] [-m]       # default: src/
veracity-review-status report src/Chap43/ [-m]  # single chapter
veracity-review-status init [path]              # add REVIEWED: NO to all files missing it
veracity-review-status add <file>               # add REVIEWED: NO to one file
veracity-review-status mark <file> <reviewer> [date]
veracity-review-status stale [path] [-m]
veracity-review-status unreviewed [path] [-m]
```
