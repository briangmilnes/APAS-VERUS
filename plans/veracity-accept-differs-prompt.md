# veracity-analyze-alg-analysis: Add accepted difference mechanism

## Problem

The alg analysis tool reports Mt DIFFERS as errors. Many are documented choices
(Vec-backed when slice exists, PRAM model gap, parametric BST design, array-backed
unordered table). These clutter the error output and make real targets hard to find.

We need a way to mark a DIFFERS as accepted so veracity downgrades it from error
to info.

## Annotation Convention

Currently a difference is annotated as:
```rust
/// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — DIFFERS: sequential loop
```

To accept the difference, change `DIFFERS` to `ACCEPTED DIFFERENCE`:
```rust
/// - Alg Analysis: Code review (Claude Opus 4.6): Work O(n), Span O(n) — ACCEPTED DIFFERENCE: Vec-backed sequential; see ArraySeqMtEphSlice for O(lg n) span
```

Same line, same position. The reason text after the colon explains why the
difference is acceptable.

## What veracity should do

1. Parse `— ACCEPTED DIFFERENCE:` on Code review lines (currently only parses
   `— DIFFERS:`).
2. If present, classify as info (not error). Print with `[accepted]` tag.
3. Add a summary line: `Mt ACCEPTED DIFFERENCE: N` alongside the existing
   `Mt DIFFERS` count.
4. Add `--include-accepted` flag to show accepted differences in detail
   (default: hide from error list).

## Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace on
Rust source. All edits must be token-aware or AST-aware. Parse ensures/requires
blocks with brace/comma/semicolon awareness. A string-hacking detector will flag
and kill tools that corrupt source syntax.

## Testing

After implementing, run on APAS-VERUS. Before changing any annotations, the
output should be identical to current (74 Mt DIFFERS errors, 0 accepted).
After changing one annotation from `DIFFERS` to `ACCEPTED DIFFERENCE`, it
should show 73 errors + 1 accepted.
