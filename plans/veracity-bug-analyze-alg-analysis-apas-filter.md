# Veracity Bug: analyze_alg_analysis false-positive "APAS without Code review"

## Bug

`veracity-analyze-alg-analysis` reports 692 false-positive warnings:
"has APAS cost spec but no Code review". All 692 functions DO have Code review
lines. The parser silently skips them.

## Root cause

File: `~/projects/veracity/src/bin/analyze_alg_analysis.rs`

Around line 248-251:

```rust
let is_code_review = trimmed.contains("Code review")
    || trimmed.contains("Claude-Opus")
    || trimmed.contains("Claude Opus");
if is_code_review && !trimmed.contains("APAS") {
    // ... parse as Code review line
}
```

The `!trimmed.contains("APAS")` guard intends to prevent APAS-only annotation lines
from being misidentified as Code review lines. But it checks the ENTIRE line, not
just the annotation prefix. Any Code review line whose suffix text mentions "APAS"
is silently rejected.

## Affected annotation patterns

All of these are valid Code review lines that the parser skips:

```
/// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — matches APAS
/// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — DIFFERS: St sequential, APAS parallel
/// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(lg n) — agrees with APAS
/// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — DIFFERS: APAS says O(1) but Vec-backed
```

The word "APAS" appears in the suffix after the colon — in the comparison text
("matches APAS", "DIFFERS: ... APAS parallel"), not in the annotation type prefix.

## Fix

Change the `!trimmed.contains("APAS")` check to only examine the prefix portion
of the line (before the first colon after "Alg Analysis:"). The Code review
identification should look at the annotation type field, not the freetext suffix.

Suggested approach: split on the first `:` after "Alg Analysis:" to get the type
field, then check for "APAS" only in that field. Or: check that the line matches
the Code review pattern first (contains "Code review"), and only then check that
it's not ALSO an APAS line (i.e., doesn't start with "Alg Analysis: APAS").

Example fix:

```rust
let is_code_review = trimmed.contains("Code review")
    || trimmed.contains("Claude-Opus")
    || trimmed.contains("Claude Opus");
let is_apas_line = trimmed.contains("Alg Analysis: APAS");  // prefix check, not full-line
if is_code_review && !is_apas_line {
    // ... parse as Code review line
}
```

## Impact

- 692 false-positive warnings ("has APAS cost spec but no Code review")
- Every Code review annotation that says "matches APAS" or "APAS parallel"
  is invisible to the tool
- The "APAS without Code review" warning count is unreliable until fixed
- 1119 Code review lines across 179 files are affected

## Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace on
Rust source. All edits must be token-aware or AST-aware. Parse ensures/requires
blocks with brace/comma/semicolon awareness. A string-hacking detector will flag
and kill tools that corrupt source syntax.

## Discovered by

R132 Agent 4 (2026-04-01). Agent correctly identified the bug but incorrectly
applied a workaround (replacing "APAS" with "textbook" in 1119 source lines).
The workaround was rejected — the fix belongs in veracity, not in APAS-VERUS
source annotations.
