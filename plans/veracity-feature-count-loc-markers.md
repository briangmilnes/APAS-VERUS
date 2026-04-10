# Feature: veracity-count-loc should report Veracity marker statistics

## What

Add a `--markers` or `--veracity-stats` flag to `veracity-count-loc` that counts
and categorizes `// Veracity:` comment lines in the codebase.

## Why

After running `veracity-minimize-proofs`, the codebase accumulates thousands of
`// Veracity:` comment lines (10,529 in APAS-VERUS after R170). These inflate the
total line count and make it hard to distinguish actual code growth from marker
overhead. The current LOC report doesn't distinguish Veracity comments from other
comments.

## Output format

When `--markers` is passed, add a section to the report:

```
Veracity Markers
----------------
  NEEDED assert:        4,812
  NEEDED proof block:   3,456
  NEEDED (speed hint):    892
  NEEDED (cpu hint):      318
  NEEDED (mem hint):      200
  UNNEEDED assert:        612
  UNNEEDED proof block:   303
  Total markers:       10,593
  % of total lines:      5.3%

Per chapter:
  Chap06:    739 markers (21 unneeded)
  Chap18:    648 markers (18 unneeded)
  Chap37:    876 markers (24 unneeded)
  Chap42:    996 markers (160 unneeded)
  Chap55:    952 markers (271 unneeded)
  ...
```

## Also adjust the main LOC report

When Veracity markers are present, show an adjusted breakdown:

```
    Spec/   Proof/    Exec/    Rust/  Veracity  File
  31,172/  40,314/  66,874/  11,801/   10,529  TOTAL
  31,172/  30,636/  66,874/  11,801/        0  TOTAL (excluding Veracity markers)
```

The "excluding markers" line subtracts NEEDED markers from proof lines (since they
get counted as proof comments) and UNNEEDED markers from whichever category the
commented-out code was in.

## Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace on
Rust source. All edits must be token-aware or AST-aware. Parse ensures/requires
blocks with brace/comma/semicolon awareness. A string-hacking detector will flag
and kill tools that corrupt source syntax.

(Note: for this feature, regex on `// Veracity:` comment lines is fine — these are
comment-only lines with a fixed prefix. The constraint applies to any code
modification, not to reading comment lines.)
