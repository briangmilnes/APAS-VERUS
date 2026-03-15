# Chap27 Spec Audit — Contraction (Reduce & Scan)

## Summary

All exec functions have **strong** specifications.

## Per-Function Classification

| # | File | Function | requires | ensures | Classification |
|---|------|----------|----------|---------|----------------|
| 1 | ReduceContractStEph.rs | reduce_contract | spec_monoid(f, id), closure contracts | reduced == fold_left(id, spec_f) | **strong** |
| 2 | ScanContractStEph.rs | scan_contract | spec_monoid(f, id), closure contracts | forall i: scanned[i] == fold_left(input[0..i]) | **strong** |
| 3 | ScanContractStEph.rs | expand_scan | contracted/scanned preconditions | forall k: prefixes[k] == fold_left(input[0..k]) | **strong** |

## Notes

- Reduce postcondition directly asserts result equals mathematical fold under monoid.
- Scan postcondition ensures each output is the exclusive prefix fold.
- Expand ensures interleaved expansion produces complete prefix scan.
- Faithfully encodes APAS Algorithms 27.2 and 27.3.
