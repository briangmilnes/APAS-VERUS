# Cost Analysis Matching: APAS vs Codebase

## Goal

Build a comprehensive table matching APAS textbook cost specifications against what
the APAS-VERUS codebase actually declares. Count coverage, flag differences, identify
gaps.

## Inputs

### 1. APAS Ground Truth (TOML files)

9 files in `analyses/apas-cost-reference-ch*.toml` covering chapters 1–66. Format:

```toml
[[cost_spec]]
ref = "Ch38 CS 38.11"
chapter = 38
description = "Parametric BST cost specification"
operations = [
    { name = "find",   work = "O(lg |t|)", span = "O(lg |t|)" },
    { name = "union",  work = "O(m * lg(n/m))", span = "O(lg n)" },
]
```

### 2. Codebase Cost Comments

Source files have cost comments in two formats:

**Format A** (structured, in trait doc comments):
```rust
/// - APAS: Work O(lg |t|), Span O(lg |t|)
/// - Claude-Opus-4.6: Work O(lg |t|), Span O(lg |t|) -- agrees with APAS.
fn find(&self, key: &T) -> ...
```

**Format B** (ad-hoc, scattered):
```rust
// Work: O(m log(n/m)), Span: O(log n x log m)
// Work Theta(n), Span Theta(n)
/// O(1) slice: shares backing storage
```

## Task

### Phase 1: Parse TOML

Read all 9 `analyses/apas-cost-reference-ch*.toml` files. Build a table of:
- Chapter, reference (e.g., "CS 38.11"), operation name, APAS work, APAS span

Count total operations across all chapters.

### Phase 2: Scan Source Files

For every `.rs` file in `src/Chap*/` (excluding `analyses/`, `Example*.rs`):
1. Find all cost comments (patterns: `Work`, `Span`, `O(`, `Theta(`, `Omega(`)
2. Extract: file, function name (nearest `fn` declaration), declared work, declared span
3. Note whether the comment cites APAS or is AI-generated ("Claude-Opus")
4. Note whether there's an APAS line AND a Claude line (dual annotation)

### Phase 3: Match

For each APAS TOML operation, find the corresponding source function(s). Matching rules:
- TOML `name` maps to Rust `fn` name (may differ: TOML `joinMid` = Rust `join_mid`,
  TOML `find` = Rust `find`, etc.)
- One TOML operation may map to multiple files (StEph, StPer, MtEph, MtPer variants)
- A source file may have functions not in the TOML (helpers, internal functions)

### Phase 4: Report

Produce three output files in `analyses/`:

#### 4a. `cost-matching-report.md` — Full table

```
| # | Chap | APAS Ref | Op | APAS Work | APAS Span | File | Fn | Source Work | Source Span | Status |
```

Status values:
- `match` — source agrees with APAS
- `DIFF` — source declares different cost (flag for human review)
- `missing` — APAS specifies cost but source file has no cost comment
- `extra` — source has cost comment but APAS TOML has no entry
- `no-file` — APAS specifies an operation but no corresponding source file exists

#### 4b. `cost-matching-summary.md` — Counts

```
| # | Chap | APAS Ops | Matched | Diff | Missing | Extra | Coverage % |
```

Plus totals row. Coverage = (Matched + Diff) / APAS Ops.

#### 4c. `cost-differences.md` — Differences only

Every row where Status = `DIFF`, with:
- What APAS says
- What source says
- Which is correct (your analysis — read the algorithm, judge the cost)
- Recommended fix (update comment, update TOML, or flag as intentional difference)

### Phase 5: Normalize

When comparing costs, normalize notation:
- `lg` = `log` = `log2` (APAS uses `lg` for log base 2)
- `O(lg n)` = `O(log n)` = `O(log2 n)`
- `|t|` = `n` = `|a|` (context-dependent)
- `Theta(f)` implies both `O(f)` and `Omega(f)` — compatible with `O(f)`
- `m * lg(n/m)` — standard APAS form for set operations where m <= n

## Constraints

- Do NOT modify any source files. This is read-only analysis.
- Do NOT modify the TOML files.
- Write output to `analyses/cost-matching-report.md`, `analyses/cost-matching-summary.md`,
  `analyses/cost-differences.md`.
- Every table must have a `#` index column and a `Chap` column.
- Flag unclear matches for human review rather than guessing.

## Approach

1. Read all 9 TOML files, parse into structured data
2. Glob `src/Chap*/*.rs`, skip `Example*.rs` and `analyses/`
3. For each source file, extract cost comments with associated function names
4. Match TOML operations to source functions by chapter + operation name
5. Compare work/span after normalization
6. Write the three output files
7. Report summary counts
