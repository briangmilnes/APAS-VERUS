# Veracity Tool: Algorithm Analysis Annotator

## Purpose

New veracity tool: `veracity-annotate-alg-analysis`. Adds standardized algorithm
cost annotations (`Work`, `Span`) to exec functions in APAS-VERUS source files.

## What it does

For every exec `fn` inside a trait declaration in `src/Chap*/*.rs`, insert two
`///` doc comment lines:

```rust
/// - Alg Analysis: APAS: Work O(lg n), Span O(lg n)
/// - Alg Analysis: Claude-Opus-4.6 (1M): NONE
fn find(&self, key: &T) -> ...
```

The tool handles the APAS line only. The Claude line is always `NONE` (agents
fill it in later).

### APAS line logic

1. Read `analyses/apas-cost-reference-all.toml` — 80 cost specs with chapter,
   operation name, work, span.
2. For each exec `fn` in a trait in a `src/ChapNN/` file:
   - Match by chapter number + function name against the TOML.
   - If matched: `/// - Alg Analysis: APAS: Work <work>, Span <span>`
   - If not matched: `/// - Alg Analysis: APAS: NONE`
3. For each exec `fn` at module level (not in a trait):
   - Same matching logic. Most will be NONE.

### Claude line

Always: `/// - Alg Analysis: Claude-Opus-4.6 (1M): NONE`

This is a placeholder. Agents fill in actual analysis later.

### Existing annotations

Many files already have `/// - APAS:` and `/// - Claude-Opus-4.6:` lines.
The tool must:
- Detect existing annotations (patterns: `/// - APAS:`, `/// - Claude-Opus`,
  `/// - Alg Analysis:`)
- Replace them with the new format
- Preserve any existing Claude analysis (if the line has real work/span, keep
  it; don't overwrite with NONE)
- Normalize `/// - APAS: N/A` → `/// - Alg Analysis: APAS: NONE`
- Normalize `/// - APAS: N/A — Verus-specific scaffolding.` → `/// - Alg Analysis: APAS: NONE`

### Placement

The two lines go in the `///` doc comment block of the trait declaration,
not the impl. If the trait fn has existing doc comments, add the Alg Analysis
lines at the end of the doc block, before the `fn` keyword. If no doc comment
exists, add a minimal one:

```rust
/// - Alg Analysis: APAS: NONE
/// - Alg Analysis: Claude-Opus-4.6 (1M): NONE
fn foo(&self) -> ...
```

### What gets annotated

- Every exec `fn` in a trait declaration: YES
- Spec fns (`spec fn`), proof fns (`proof fn`): NO
- Free exec functions at module level that do algorithmic work: YES
- Constructors (`new`, `empty`, `singleton`): YES (most are O(1))
- `clone`, `eq`, iterator `next`: YES

### What gets skipped

- `Example*.rs` files
- Chap65 (commented out of lib.rs)
- `src/standards/`, `src/experiments/`, `src/vstdplus/`, `src/Types/`,
  `src/Concurrency/`
- Spec-only files with no exec functions

## TOML format

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

Matching: TOML `name` maps to Rust `fn` name. Common transforms:
- `joinMid` → `join_mid` (camelCase to snake_case)
- Names may be exact matches or need case normalization

One TOML operation can match functions in multiple files (StEph, StPer, MtEph,
MtPer variants all get the same APAS line).

## CLI interface

```bash
veracity-annotate-alg-analysis \
  -c ~/projects/APAS-VERUS \
  --toml analyses/apas-cost-reference-all.toml \
  [--chapter ChapNN]        # optional: limit to one chapter
  [--dry-run]               # show what would change without modifying files
```

Output: emacs compile format (file:line: level: message) showing each annotation
added or updated.

## Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace on
Rust source. All edits must be token-aware or AST-aware. Parse doc comments,
trait blocks, and fn declarations with brace/bracket awareness. A string-hacking
detector will flag and kill tools that corrupt source syntax.
