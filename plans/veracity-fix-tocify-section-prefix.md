# Veracity Tocify: Add "Section" Prefix to All Section Headers

## Context

You are working in `~/projects/veracity-agent1`. Do not touch `~/projects/veracity`.

## Problem

Section headers in APAS-VERUS source files use bare numbers:

```
//  4a. type definitions — struct Leaf
```

In emacs these are hard to find because searching for "4" matches everywhere.
The fix is to prefix every section number with `Section `:

```
//  Section 4a. type definitions — struct Leaf
```

This applies to BOTH the Table of Contents block at the top of each file AND
the inline section headers throughout the file.

## Before/After

### TOC block

Before:
```
//  Table of Contents
//  1. module
//  2. imports
//  3. broadcast use
//  4a. type definitions — struct Leaf
//  5a. view impls — struct Leaf
```

After:
```
//  Table of Contents
//  Section 1. module
//  Section 2. imports
//  Section 3. broadcast use
//  Section 4a. type definitions — struct Leaf
//  Section 5a. view impls — struct Leaf
```

### Inline section headers

Before:
```
//		4a. type definitions — struct Leaf
```

After:
```
//		Section 4a. type definitions — struct Leaf
```

## Rules

- The word `Section` goes before the number, after the comment prefix and whitespace.
- Preserve existing whitespace/tab patterns (tabs after `//`).
- Apply to all section numbers 1-14 (with or without letter suffixes a/b/c/...).
- The "Table of Contents" header line itself does not change.
- Do NOT change anything else in the file — only section header lines.

## Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace on
Rust source. All edits must be token-aware or AST-aware. Parse section header
patterns with proper awareness. A string-hacking detector will flag and kill
tools that corrupt source syntax.
