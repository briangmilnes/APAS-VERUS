# R132 Agent 3 — Add RTTs for parallel D&C functions. AFK. DOT.

## Setup

Read ALL files in `src/standards/` before starting.

Report file: `plans/r132-agent3-parallel-rtts-report.md`

## Problem

R127-R128 parallelized map, reduce, filter with D&C join() in three files:
- `src/Chap18/ArraySeqMtEph.rs` — map_dc, reduce_dc, filter_dc
- `src/Chap18/ArraySeqMtPer.rs` — map_inner, reduce_inner, filter_inner
- `src/Chap19/ArraySeqMtEph.rs` — map_dc, reduce_dc, filter_dc

These parallel paths have no dedicated RTTs. The existing RTTs test the trait methods
(which now delegate to D&C), but there are no tests that specifically exercise:
- Large inputs (where D&C actually splits)
- Edge cases (empty, singleton, power-of-2 sizes, odd sizes)
- Result equivalence with sequential versions

## What to do

Add RTTs in the existing test files:
- `tests/Chap18/TestArraySeqMtEph.rs`
- `tests/Chap18/TestArraySeqMtPer.rs`
- `tests/Chap19/TestArraySeqMtEph.rs`

For each of map, reduce, filter in each file, add tests that:

1. **Empty**: `map/reduce/filter` on empty sequence
2. **Singleton**: single element
3. **Small**: 3-5 elements (D&C base case boundary)
4. **Medium**: 100 elements (forces multiple D&C splits)
5. **Large**: 10000 elements (exercises deep recursion)
6. **Correctness**: verify result matches expected (e.g., map doubles, reduce sums,
   filter keeps evens)

Use the trait methods (which delegate to D&C internally). Example:

```rust
#[test]
fn test_map_dc_medium() {
    let seq = ArraySeqMtEphS::tabulate(&|i: usize| i as u64, 100);
    let doubled = ArraySeqMtEphS::map(&seq, &|x: &u64| *x * 2);
    assert_eq!(doubled.length(), 100);
    for i in 0..100 {
        assert_eq!(*doubled.nth(i), (i as u64) * 2);
    }
}
```

## Validation

Run `scripts/rtt.sh`. All existing + new tests must pass.

## Rules

- Tests only — do NOT modify source files.
- Use existing test file patterns (look at how current tests are structured).
- Name tests clearly: `test_<operation>_<variant>_<size>`.
