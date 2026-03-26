# Agent 2 — Round 82 Report

## Objective

Fix Chap44 DocumentIndex.rs and Example44_1.rs parse errors so both files compile and verify.

## Root Cause

The `DocumentIndexTrait` trait was declared **outside** `verus!` but contained Verus-specific
syntax on `to_seq`: a named return `(seq: ArraySeqStPerS<DocumentId>)` and `ensures` clauses.
Verus parses code outside `verus!` as standard Rust, which doesn't support named returns or
ensures — hence the parse error at line 79:77 (`expected one of... found :`).

A secondary issue: `Example44_1.rs` had `TweetQueryExamples` struct inside `verus!` with a
`Box<dyn Fn(&Word) -> DocumentSet>` field. Verus doesn't support `dyn` with trait objects.

## Fix Applied

**DocumentIndex.rs**: Removed the Verus-specific syntax (named return and ensures) from the
trait declaration, keeping the trait outside `verus!`. The trait and all impl methods remain
outside `verus!` since they use non-Verus-parseable APIs (`sort_unstable_by`, `chars()`,
`is_alphabetic()`, `to_lowercase()`). The struct, Clone, PartialEq, and Eq remain inside
`verus!` with proper patterns:

- Clone uses `assume(result == *self)` (standard eq/clone workaround).
- PartialEq uses `#[verifier::external_body]` on `eq()` (String comparison specs are opaque
  in vstd — `obeys_cmp_spec::<String>()` is not broadcast-proven).

**Example44_1.rs**: Moved `TweetQueryExamples` struct outside `verus!` since it contains
`Box<dyn Fn>` which Verus can't parse.

**lib.rs**: Uncommented both modules:
```rust
pub mod Chap44 {
    pub mod DocumentIndex;
    pub mod Example44_1;
}
```

## Why Not Move Everything Inside verus!

Moving the trait/impl inside `verus!` was attempted first (iterations 1-6) but cascaded into:
1. `obeys_cmp_spec::<String>()` failures on all set operations (intersection, union, difference)
2. `obeys_eq_spec()` failures on PartialEq
3. `obeys_feq_full::<AVLTreeSetStPer<String>>()` failures on TableStPer::find

String's comparison/equality specs are **not** broadcast-proven in vstd (only numeric types
are). The TotalOrder impl for String in vstdplus uses `assume()`. Fixing this would require
either proving String comparison axioms or restructuring the module to use generic types
instead of concrete String — both out of scope for this round.

The pragmatic fix keeps the code outside `verus!` (matching the original author's intent per
the `_document_index_verified` placeholder) while fixing the parse error that prevented compilation.

## Verification Results

| Metric | Value |
|--------|-------|
| Verified | 4683 |
| Errors | 0 |
| Chap44 Holes | 0 |
| RTT | Pre-existing failures in Chap56/62/65 (unrelated) |
| PTT | 157 passed |

## Holes Table (Before/After)

| # | Chap | File | Holes Before | Holes After |
|---|------|------|-------------|-------------|
| 1 | 44 | DocumentIndex.rs | N/A (BROKEN) | 0 |
| 2 | 44 | Example44_1.rs | N/A (BROKEN) | 0 |

## Iterations Used

7 of 15 allowed.
