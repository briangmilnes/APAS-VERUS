# Veracity Style Review: Add Summary Section and Log to analyses/

## Problem

The style review output is 1MB+ of per-file detail. Reading it costs thousands of tokens to extract the signal. There is no summary. The output logs to `src/analyses/veracity-review-verus-style.log` but there is no separate summary file.

## Requirements

### 1. Summary section at end of output

After all per-file output, append a summary section with three tables:

**Table 1: Warnings by rule**

```
Style Review Summary
=====================

Warnings by rule:
| # | Rule     | Count | Description                                          |
|---|----------|-------|------------------------------------------------------|
| 1 | [22:proof] | 558 | Free proof fn should be trait method                |
| 2 | [22:spec]  | 532 | Free spec fn should be trait signature with impl body |
| 3 | [22:exec]  | 438 | Free exec fn should be trait method                 |
| 4 | [18]       | 718 | Import ordering                                     |
| 5 | [14]       | 342 | Missing Debug/Display impl                          |
| ...                                                                          |

Total: 3180 warnings, 246 files checked.
```

Sort by count descending.

**Table 2: Warnings by chapter**

```
Warnings by chapter:
| # | Chapter | Warnings | Top rules                    |
|---|---------|----------|------------------------------|
| 1 | Chap06  | 257      | [22:exec] 85, [14] 62, ...   |
| 2 | Chap43  | 232      | [22:spec] 71, [18] 54, ...   |
| 3 | Chap50  | 225      | [22:proof] 68, [17] 42, ...  |
| ...                                                    |
```

Sort by warning count descending. "Top rules" shows the top 3 rule codes and their counts for that chapter.

**Table 3: [23b] bound mismatch patterns (if any [23b] warnings exist)**

```
[23b] bound mismatch patterns:
| # | Gap (fn has, trait lacks) | Count | Example files                        |
|---|--------------------------|-------|--------------------------------------|
| 1 | Send+Sync+'static        | 12    | Chap18/ArraySeqMtEphSlice, Chap19/.. |
| 2 | DeepView                 | 2     | Chap18/ArraySeq                      |
| ...                                                                         |
```

Group by the gap (bounds the free fn has that the trait lacks). Sort by count descending.

### 2. Log the summary to a separate file

Write the summary section (tables only, not the per-file detail) to:

```
src/analyses/veracity-review-verus-style-summary.log
```

The full per-file output continues to log to:

```
src/analyses/veracity-review-verus-style.log
```

Both files are overwritten on each run.

### 3. Console output

Print both the per-file detail AND the summary to stdout (as today, via tee or equivalent). The summary appears last so it is visible without scrolling.

## Constraint: No String Hacking

Do NOT use regex, line-by-line string replacement, or naive find-and-replace on Rust source. All edits must be token-aware or AST-aware. Parse type parameter bounds with brace/comma/semicolon awareness. A string-hacking detector will flag and kill tools that corrupt source syntax.
