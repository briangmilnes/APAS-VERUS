# veracity-analyze-alg-analysis — update after R125 annotation standardization

## Context

Get a clean tests/fixtures/APAS-AI

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

## Definitive annotation format

Every exec fn in a trait should have annotations in `///` doc comments
immediately above the `fn` declaration. There are exactly these formats:

### Cost spec function (has textbook reference)

Two lines. The APAS line gives the textbook's cost. The Code review line
gives the implementation's actual cost.

```rust
/// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(|a|), Span O(lg |a|)
/// - Alg Analysis: Code review (Claude Opus 4.6): Work O(|a|), Span O(lg |a|) — matches APAS
```

The APAS reference is `(ChNN <type> NN.NN)` where type is one of:
`CS` (Cost Spec), `Alg` (Algorithm), `Thm` (Theorem), `Def` (Definition),
`Ex` (Exercise), `DT` (Data Type).

The Code review ends with one of:
- `— matches APAS` (implementation matches textbook)
- `— DIFFERS: reason` (implementation differs, with brief explanation)
- `— St sequential (APAS parallel)` (St file, expected sequential span)

A function may have multiple APAS lines (appears in multiple cost specs):
```rust
/// - Alg Analysis: APAS (Ch20 CS 20.2): Work O(1), Span O(1)
/// - Alg Analysis: APAS (Ch22 CS 22.2): Work O(1), Span O(1)
/// - Alg Analysis: Code review (Claude Opus 4.6): Work O(1), Span O(1) — matches APAS
```

### Utility function (not in textbook)

One line:
```rust
/// - Alg Analysis: APAS: N/A — implementation utility, not in prose.
```

### Semantics-only chapter (no cost spec in textbook)

One line:
```rust
/// - Alg Analysis: APAS: no cost spec (semantics-only chapter).
```

### No cost stated in textbook

One line:
```rust
/// - Alg Analysis: APAS: (no cost stated — Chapter NN is purely definitional)
```

### OLD format (error — should not exist after R125)

Any line starting with `/// - APAS:` is old format and should be flagged:
```rust
/// - APAS: Work Θ(|v|), Span Θ(1)          // OLD — error
/// - APAS: N/A — reason                     // OLD — error (already fixed by sed)
/// - APAS: no cost spec                     // OLD — error (already fixed by sed)
```

### Boilerplate functions (no annotation needed)

These fn names never need annotations: `clone`, `eq`, `fmt`, `next`,
`default`, `drop`, `view`, `inv`, `cmp`, `partial_cmp`, `hash`,
`into_iter`, `iter`.

### File exclusions

Skip: `Example*.rs`, `Problem*.rs`, files in `src/standards/`,
`src/experiments/`, `src/vstdplus/`.

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
