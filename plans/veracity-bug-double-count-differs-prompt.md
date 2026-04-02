# veracity-analyze-alg-analysis: Bug — double-counting DIFFERS in Chap62/Chap66

## Problem

`veracity-analyze-alg-analysis` reports 23 Mt DIFFERS but only 17 are unique.
Six lines are counted twice:

```
src/Chap62/StarPartitionMtEph.rs:55   — appears 2x
src/Chap62/StarPartitionMtEph.rs:845  — appears 2x
src/Chap66/BoruvkaMtEph.rs:136       — appears 2x
src/Chap66/BoruvkaMtEph.rs:155       — appears 2x
src/Chap66/BoruvkaMtEph.rs:765       — appears 2x
src/Chap66/BoruvkaMtEph.rs:996       — appears 2x
```

## Likely cause

These functions have `/// - Alg Analysis:` annotations on both the trait
declaration and the impl body. The tool scans all `/// - Alg Analysis:`
lines and counts each one, so a function with annotations in both locations
gets counted twice.

## Fix

Deduplicate by (file, line_number) before counting. If the same file:line
appears multiple times in the output, emit it only once.

## Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace on
Rust source. All edits must be token-aware or AST-aware. Parse ensures/requires
blocks with brace/comma/semicolon awareness. A string-hacking detector will flag
and kill tools that corrupt source syntax.
