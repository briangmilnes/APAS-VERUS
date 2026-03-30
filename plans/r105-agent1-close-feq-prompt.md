# R105 Agent 1 — Close obeys_feq_view_injective, STEP 20

## Objective

`obeys_feq_view_injective` is the #1 most expensive quantifier across the
codebase: 888K instantiations. It's an `open spec fn` in `src/vstdplus/feq.rs`
whose 2-variable forall leaks into every module that imports feq via broadcast.

Close it. Add `reveal(obeys_feq_view_injective::<T>)` only where the quantifier
body is actually needed.

## The change

In `src/vstdplus/feq.rs` line 63:

```rust
// Before:
pub open spec fn obeys_feq_view_injective<T: Eq + View + Sized>() -> bool {
    forall|x: T, y: T| #[trigger] x.view() == #[trigger] y.view() ==> x == y
}

// After:
pub closed spec fn obeys_feq_view_injective<T: Eq + View + Sized>() -> bool {
    forall|x: T, y: T| #[trigger] x.view() == #[trigger] y.view() ==> x == y
}
```

## What breaks

A test run found 78 errors across 40 files. Error counts per file:

- StarPartitionMtEph.rs: 65 errors (heaviest — uses view injectivity extensively)
- OrderedTableStEph.rs: 11
- StructChainedHashTable.rs: 10
- OrderedTableStPer.rs: 9
- VecChainedHashTableStEph.rs: 5
- QuadProbFlatHashTableStEph.rs: 5
- LinkedListChainedHashTableStEph.rs: 5
- SetMtEph.rs: 5
- LinProbFlatHashTableStEph.rs: 3
- DoubleHashFlatHashTableStEph.rs: 3
- SetStEph.rs: 3
- SCCStEph.rs: 2
- 27 files with 1 error each (WeightedDirGraph×12, graph chapters, feq.rs, etc.)

## Fix pattern

For each error, add `reveal(obeys_feq_view_injective::<T>)` in a proof block
at the point where the quantifier body is needed. The type parameter must match
the concrete type being used.

```rust
// In exec code:
proof { reveal(obeys_feq_view_injective::<MyType>); }

// In proof fn:
reveal(obeys_feq_view_injective::<MyType>);
```

Most 1-error files need a single reveal. StarPartitionMtEph needs many because
it reasons about view injectivity in multiple proof blocks.

## Strategy

Work in tiers:
1. Make the one-line change in feq.rs
2. Fix Chap05 (SetStEph, SetMtEph) — small, validates fast with isolate
3. Fix Chap06 (12 WeightedDirGraph files — same template, fix one copy the rest)
4. Fix Chap43 (OrderedTableStEph, OrderedTableStPer)
5. Fix Chap47 (7 hash table files)
6. Fix Chap55, 61, 62, 63, 65, 66 (graph chapters, 1-2 errors each)
7. Fix vstdplus/feq.rs and hash_map_with_view_plus.rs
8. Full validate

Use `scripts/validate.sh isolate ChapNN` for iterative work on each tier.

## Profile after

After full validate passes, run:
```bash
scripts/profile.sh isolate Chap52
scripts/profile.sh isolate Chap06
```

Report before/after for `obeys_feq_view_injective` and total instantiations.

Before: Chap52 total 5,554,249. Chap06 total 1,973,673.

## Isolation

Use isolate per chapter during development:
```bash
scripts/validate.sh isolate Chap05
scripts/validate.sh isolate Chap06
# etc.
```

Full validate at the end.

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT add `assume` or `accept`.
- Do NOT change the spec fn body — only change `open` to `closed`.
- Do NOT change any other spec fn.
- The vstd/seq.rs errors in the test run are NOT in vstd — they're source
  locations of ensures clauses that our code fails to satisfy. Fix our code.
- If StarPartitionMtEph is too hard (65 errors), fix everything else first
  and leave StarPartition for last. Report what you couldn't fix.

## STEP 20

## Report

Write `plans/agent1-r105-close-feq-report.md`.
