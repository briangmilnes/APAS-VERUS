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

## Commands

### 1. `veracity-review-status report`

Scan all source files, produce a report:

```
| # | Chap | File | Reviewed | Reviewer | Date | Stale? | Days Since |
|---|------|------|----------|----------|------|--------|------------|
| 1 | 18   | ArraySeqStEph.rs | YES | Brian Milnes | 2026-03-01 | STALE | 23 |
| 2 | 18   | ArraySeqStPer.rs | YES | Brian Milnes | 2026-03-20 | OK    | 4  |
| 3 | 18   | ArraySeqMtEph.rs | NO  | -            | -          | -     | -  |
```

**Stale detection**: Compare the `REVIEWED:` date against `git log -1 --format=%ai -- <file>`.
If the file was modified after the review date, mark `STALE`. This means the review is
out of date and the file needs re-review.

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

### 2. `veracity-review-status init`

Add `//! REVIEWED: NO` to every `.rs` file in `src/Chap*/` that doesn't already have a
`REVIEWED:` line. Idempotent — skips files that already have the annotation.

Placement: insert as the first `//!` line after the copyright block. The copyright block
is the contiguous run of `//` lines at the top of the file (before the first `//!` or
blank line).

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

List only stale files (reviewed but modified since review). This is the "what needs
re-review" query.

```bash
veracity-review-status stale
# Output: file paths, one per line, with review date and last-modified date
```

### 5. `veracity-review-status unreviewed`

List only unreviewed files. This is the "what's never been reviewed" query.

```bash
veracity-review-status unreviewed
# Output: file paths, one per line
```

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

- `report` writes to stdout and optionally to `analyses/veracity-review-status.log`
- All tables follow APAS-VERUS conventions: `#` index column, `Chap` column for
  file-referencing tables, max 40 chars per cell
- Exit code 0 on success, 1 on parse errors

### CLI Interface

```bash
veracity-review-status report [path]          # default: src/
veracity-review-status report src/Chap43/     # single chapter
veracity-review-status init [path]            # add REVIEWED: NO to all files
veracity-review-status mark <file> <reviewer> [date]
veracity-review-status stale [path]
veracity-review-status unreviewed [path]
```
