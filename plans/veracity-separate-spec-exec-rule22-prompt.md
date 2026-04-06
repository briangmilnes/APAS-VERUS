# veracity-review-verus-style: Split rule [22] into spec vs exec

## Problem

Rule [22] currently reports both free spec functions and free exec functions
as the same warning:

```
warning: [22] free spec fn spec_wf should be an abstract signature in a trait
warning: [22] free fn insert_node should be a trait method
```

These are very different problems. Free exec functions that take the type as
first arg are a clear malpattern — they should be trait methods. Free spec
functions are sometimes correct (helper predicates, standalone specs) and
sometimes should be trait abstract signatures.

We need to see them separately to prioritize.

## Fix

Split rule [22] into two rules:

- **[22a]** `free spec fn ... should be an abstract signature in a trait` — spec functions
- **[22b]** `free exec fn ... should be a trait method` — exec functions (fn, not spec fn, not proof fn)

Or use sub-labels:
```
warning: [22:spec] free spec fn spec_wf should be an abstract signature in a trait
warning: [22:exec] free fn insert_node should be a trait method
```

The summary line should count them separately too:
```
[22:spec] 400
[22:exec] 137
```

## Also split proof functions

Free proof functions (lemmas) are a third category. Some belong in traits,
some are genuinely standalone. Add:

- **[22c]** or **[22:proof]** for `free proof fn ...`

## Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace on
Rust source. All edits must be token-aware or AST-aware. Parse ensures/requires
blocks with brace/comma/semicolon awareness. A string-hacking detector will flag
and kill tools that corrupt source syntax.
