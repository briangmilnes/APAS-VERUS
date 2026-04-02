# R139 Agent 1 — Annotate ~39 OK Mt DIFFERS as documented choices. AFK. DOT.

## Setup

Read ALL files in `src/standards/` before starting.

Report file: `plans/r139-agent1-annotate-ok-differs-report.md`

## Problem

`analyses/veracity-analyze-alg-analysis.log` reports 76 Mt DIFFERS. Of these, ~39
are documented choices where the implementation intentionally differs from APAS.
These need a `— OK:` tag after the DIFFERS reason so veracity can distinguish real
targets from accepted differences.

## The categories

### Category 1: Vec-backed (slice version matches APAS)

These files are Vec-backed (`ArraySeqMtEph` in Chap18 and Chap19). The slice-backed
`ArraySeqMtEphSlice` (Chap19) is the one that matches APAS for reduce, map, filter.
The Vec versions are correct sequential implementations — they differ because Vec
operations are O(n) where slice operations are O(1).

Change the Code review annotation from:
```
/// - Alg Analysis: Code review (Claude Opus 4.6): ... — DIFFERS: sequential clone loop
```
To:
```
/// - Alg Analysis: Code review (Claude Opus 4.6): ... — DIFFERS: Vec-backed sequential; see ArraySeqMtEphSlice for O(lg n) span — OK: documented choice
```

Files:
- `src/Chap18/ArraySeqMtEph.rs` — subseq, append, filter, update, inject, ninject,
  scan, map, tabulate, flatten (10 DIFFERS)
- `src/Chap18/ArraySeqMtPer.rs` — subseq, append, filter, update, inject, reduce,
  scan, map, tabulate, flatten (10 DIFFERS)
- `src/Chap19/ArraySeqMtEph.rs` — subseq, append, update, inject, ninject, scan,
  tabulate, flatten (8 DIFFERS)

### Category 2: PRAM gap (APAS assumes PRAM, not fork-join)

APAS CS 41.3 assumes PRAM (parallel random access machine) with O(1) span for
bitwise operations on machine words. Fork-join cannot achieve O(1) span on arrays
of words. This is a model gap, not an implementation bug.

Change the Code review annotation to end with:
```
— DIFFERS: PRAM model assumes O(1) parallel word ops; fork-join model is O(lg u) — OK: PRAM gap
```

Files:
- `src/Chap41/ArraySetEnumMtEph.rs` — size, to_seq, singleton, from_seq, filter,
  intersection, difference, union, delete, insert (10 DIFFERS)

### Category 3: Parametric BST (by design)

`BSTParaMtEph` is parametric — it delegates to the specific BST implementation's
join. The parametric join_mid is O(1) wrapper. This is the correct parametric design.

Change annotations to end with:
```
— DIFFERS: parametric impl delegates to concrete BST join — OK: parametric design
```

Files:
- `src/Chap38/BSTParaMtEph.rs` — join_mid, join_pair, join_m (3 DIFFERS)

### Category 4: Unordered table (array-backed)

`TableMtEph` is an array-backed unordered table. All operations are O(n) by the
nature of unordered arrays. APAS's O(n) work applies to hash-based or tree-based
tables. The array implementation is correct for its data structure.

Change annotations to end with:
```
— DIFFERS: array-backed unordered table, linear scan by design — OK: array representation
```

Files:
- `src/Chap42/TableMtEph.rs` — domain, filter, intersection, union, difference,
  delete, insert, restrict, subtract (9 DIFFERS)

### Category 5: spec_fn not Send (blocked on Verus)

Verus currently does not allow spec_fn closures to be Send. These operations need
filter with a spec_fn predicate, which can't be passed to join().

Change annotations to end with:
```
— DIFFERS: spec_fn not Send in Verus, blocks parallel filter — OK: Verus limitation
```

Files:
- `src/Chap38/BSTParaMtEph.rs` — filter (1 DIFFERS)
- `src/Chap41/AVLTreeSetMtEph.rs` — filter (1 DIFFERS)
- `src/Chap41/AVLTreeSetMtPer.rs` — filter (1 DIFFERS)

## Do NOT change these (real targets)

Leave these DIFFERS annotations unchanged — they are real algorithmic targets:
- `src/Chap19/ArraySeqMtEphSlice.rs` — any future scan/tabulate/inject/flatten
- `src/Chap35/OrderStatSelectMt*.rs` — sequential filter in select
- `src/Chap36/QuickSortMtEph.rs` — sequential partition
- `src/Chap41/AVLTreeSet*.rs` — to_seq, from_seq (non-filter)
- `src/Chap50/OptBinSearchTreeMt*.rs` — parallel DP
- `src/Chap52/*` — graph operations
- `src/Chap62/StarPartitionMtEph.rs` — loops 1,5
- `src/Chap66/BoruvkaMtEph.rs` — sequential outer loop

## Validation

Run `scripts/validate.sh`. Then `scripts/rtt.sh`.

After validation, run `scripts/all-holes-by-chap.sh` and
`~/projects/veracity/target/release/veracity-analyze-alg-analysis -c .` to
regenerate analysis logs.

## Rules

- Only change `Code review` annotation lines. Do NOT change APAS lines.
- Append `— OK: <category>` to the DIFFERS reason. Do not change the reason itself.
- Do NOT change any non-annotation code.
- Do NOT add assumes, accepts, or external_body.

## When done

RCP.
