# veracity-analyze-alg-analysis — new veracity program

## Overview

A read-only analysis tool that audits algorithmic analysis annotations across
a Verus/Rust codebase. Emits errors in emacs compile format (`file:line: error: message`)
so you can jump to problems from a compilation buffer.

## What it checks

### Error: missing Code review annotation

Every exec fn in a trait (not spec fn, not proof fn, not boilerplate) must have a
`/// - Alg Analysis: Code review (Claude Opus 4.6):` line. If missing, emit:

```
src/Chap18/ArraySeqStEph.rs:135: error: fn `length` missing Code review annotation
```

**Boilerplate exclusions** (never need annotations):
- `fn clone` (Clone impl)
- `fn eq` (PartialEq impl)
- `fn fmt` (Debug/Display impl)
- `fn next` (Iterator impl)
- `fn default` (Default impl)
- `fn drop` (Drop impl)
- `fn view` (View impl)
- `fn inv` (RwLockPredicate impl)
- `fn partial_cmp` / `fn cmp` (Ord impls)

**File exclusions**:
- Skip files matching `Example*.rs` or `Problem*.rs`
- Skip files in `src/standards/`, `src/experiments/`, `src/vstdplus/`

### Error: St file with parallel cost spec

If a file is an St variant (`*StEph.rs` or `*StPer.rs`) and has an APAS annotation
with Span different from Work (i.e., APAS says parallel span), but the Code review
says Span = Work, that's EXPECTED — not an error. However, if the APAS annotation
claims parallel span and the Code review also claims parallel span on an St file,
that IS an error (St files are sequential by definition):

```
src/Chap26/MergeSortStPer.rs:42: error: St file claims parallel span — Code review says Span O(lg n) but St implementations are sequential
```

### Error: Mt function with DIFFERS

If an Mt file (`*MtEph.rs` or `*MtPer.rs`) has a Code review line that says
`— DIFFERS`, emit an error. Mt implementations should match APAS parallel costs.
A DIFFERS on an Mt file is a real blocker:

```
src/Chap41/AVLTreeSetMtEph.rs:181: error: Mt fn `intersection` DIFFERS from APAS — sequential split-join
```

### Info: St function with DIFFERS (expected)

If an St file has DIFFERS, emit info (not error). This is expected behavior:

```
src/Chap26/MergeSortStPer.rs:42: info: St fn `merge_sort` sequential as expected — DIFFERS in span only
```

### Warning: APAS annotation without Code review

If a function has `/// - Alg Analysis: APAS (...)` but no corresponding
`Code review (Claude Opus 4.6):` line, that's a warning:

```
src/Chap18/ArraySeqStEph.rs:135: warning: fn `length` has APAS annotation but no Code review
```

### Summary

At the end, print a summary:

```
=============================================================
Summary
=============================================================

  Total exec fns scanned:    5276
  Boilerplate excluded:       733
  Files excluded:              42

  Code review annotations:    567
  Missing Code review:       4709  ← errors

  Mt DIFFERS (blockers):       75  ← errors
  St DIFFERS (expected):      123  ← info
  APAS without Code review:     0  ← warnings

  Errors: 4784
  Warnings: 0
  Info: 123
```

## Implementation notes

### Parsing

Use `syn` with features `["full", "parsing", "extra-traits"]`. It's 65K lines,
MIT/Apache-2.0, 4 dependencies, 2.6s build.

`///` doc comments are preserved by syn as `#[doc = "..."]` attributes on the
following item. Read these to find `Alg Analysis:` annotations.

To find exec fns: parse with `syn::parse_file`, walk items, look for `fn` items
inside `impl` blocks and `trait` blocks. Skip items with `spec` or `proof` in
their declaration (syn won't parse Verus keywords directly — detect via doc
attrs or raw text scanning).

**Important**: syn cannot parse `verus! { }` macro blocks. You'll need to either:
1. Extract the contents of `verus! { }` and parse that, or
2. Fall back to line-based scanning for fn declarations inside verus blocks

Option 2 is more robust for this tool since we only need fn names and their
preceding doc comments, not full AST analysis.

### Detecting spec/proof fns

Verus marks fns with mode keywords before `fn`:
- `spec fn foo` — spec function
- `proof fn foo` — proof function
- `fn foo` — exec function (default mode)
- `pub open spec fn` — also spec
- `pub proof fn` — also proof

Scan the text for `fn ` preceded by `spec ` or `proof ` on the same line.

### Detecting file variant type

Parse filename:
- `*StEph.rs` or `*StPer.rs` → St (sequential)
- `*MtEph.rs` or `*MtPer.rs` → Mt (multi-threaded)
- Everything else → Other (base modules, algorithms)

### Output format

Emacs compile format, one line per diagnostic:

```
file:line: error: message
file:line: warning: message
file:line: info: message
```

Lines must be absolute paths or relative to cwd for emacs `compile-goto-error`.

### CLI

```
veracity-analyze-alg-analysis [OPTIONS]

Options:
  -c, --codebase DIR    Root of the project (default: .)
  -e, --exclude DIR     Exclude directories (repeatable)
  --errors-only         Only show errors, not info/warnings
  --summary-only        Only show the summary table
  --mt-only             Only show Mt DIFFERS errors
  --missing-only        Only show missing Code review errors
```

## Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace on
Rust source. All edits must be token-aware or AST-aware. Parse ensures/requires
blocks with brace/comma/semicolon awareness. A string-hacking detector will flag
and kill tools that corrupt source syntax.

**However**: this tool is READ ONLY. It does not modify source files. The no-string-hacking
constraint applies to future write tools. For this read-only analyzer, line-based
scanning of fn declarations and doc comments is acceptable and preferred over
full AST parsing, since Verus syntax extends Rust in ways syn cannot parse.

## First round: read only

This prompt is for the first round. Build the analyzer. It reads files, scans for
exec fns, checks annotations, emits diagnostics. No file modifications.

Test against `~/projects/APAS-VERUS` to verify output is correct.

## Build

Build in `~/projects/veracity/` alongside the other veracity tools. Follow the
existing veracity project structure for binary targets, CLI argument parsing, etc.
