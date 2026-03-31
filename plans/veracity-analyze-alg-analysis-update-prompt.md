# veracity-analyze-alg-analysis — update after R125 annotation standardization

## Context

After R125, all alg analysis annotations will be in one of two standard formats:

```rust
/// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(|a|), Span O(lg |a|)
/// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a|), Span O(lg |a|) — matches APAS
```

Or for utilities:
```rust
/// - Alg Analysis: APAS: N/A — implementation utility, not in prose.
/// - Alg Analysis: APAS: no cost spec (semantics-only chapter).
```

The old `/// - APAS: Work Θ(...)` format has been eliminated.

## Changes needed to veracity-analyze-alg-analysis

### 1. St parallel span should NOT be an error

Currently: 112 errors for St files where the APAS annotation has parallel span.

These are correct — the APAS textbook gives parallel cost specs. The St file
implements it sequentially. The Code review line says DIFFERS. This is the
expected state.

**New behavior**: If an St file has:
- An APAS line with Work != Span (parallel)
- A Code review line with `— DIFFERS` or `— St sequential`

Then emit **info**, not error. The system is correctly documented.

Only emit **error** if:
- An St file's Code review line claims parallel span (agent got it wrong)
- An St file has APAS parallel annotation but NO Code review line (incomplete)

### 2. Recognize the standard annotation format

The tool should parse these two-line pairs:

```
/// - Alg Analysis: APAS (ChNN <type> NN.NN): Work O(...), Span O(...)
/// - Alg Analysis: Code review (Claude Opus 4.6): Work O(...), Span O(...) — matches APAS
```

And also:
```
/// - Alg Analysis: APAS: N/A — reason
/// - Alg Analysis: APAS: no cost spec — reason
/// - Alg Analysis: APAS: (no cost stated — reason)
```

### 3. Flag remaining old format as errors

If any `/// - APAS:` lines remain (old format), emit error:
```
src/Chap05/SetStEph.rs:123: error: old format annotation `/// - APAS: Work Θ(...)` — reformat to `/// - Alg Analysis: APAS (ChNN ref): Work O(...), Span O(...)`
```

### 4. Summary categories

```
  Alg analysis annotations:  NNNN  (both APAS + Code review lines)
  Missing alg analysis:      NNNN  <- errors (exec fns with no annotation)
  Old format remaining:      NNNN  <- errors (/// - APAS: lines not yet reformatted)

  Mt DIFFERS (blockers):       NN  <- errors
  St DIFFERS (expected):       NN  <- info (correctly documented)
  St parallel Code review:      N  <- errors (agent claimed parallel on St)
  APAS without Code review:    NN  <- warnings

  Errors: NNNN
  Warnings: NNN
  Info: NNN
```
