# new-mut-ref Validation Report

Date: 2026-04-14  
Verus tag: `release/rolling/0.2026.04.10.fc697a7`  
Command: `scripts/validate.sh` with `-V new-mut-ref` added to the default Verus invocation  
Log: `logs/validate.20260414-172857.log`

## Result: BLOCKED — Verus Internal Panics

The run crashed after 49s with 3 Verus Internal Errors before Z3 ran at all
(peak Z3 RSS = 0MB). Normal full validation takes ~7 min and verifies 5765 functions.
No verification was performed.

## Panics

All three are the same error: `var_local_id failed` in `rustc_mir_build/src/builder/mod.rs:275`.

| # | Chap | File | Function | Line | Span |
|---|------|------|----------|------|------|
| 1 | 37 | AVLTreeSeq.rs | `delete_value` | 1077 | 1077–1169 |
| 2 | 42 | TableStPer.rs | `insert` | 1627 | 1627–1852 |
| 3 | 42 | TableStPer.rs | `insert_wf` | 1855 | 1855–2171 |

Full panic text from log:

```
thread 'rustc' (2621139) panicked at rustc_mir_build/src/builder/mod.rs:275:13:
Verus Internal Error: var_local_id failed: (30958, 157)
... self.fn_span = src/Chap37/AVLTreeSeq.rs:1077:9: 1169:10

Verus Internal Error: var_local_id failed: (20307, 709)
... self.def_id = DefId(…::Chap42::TableStPer::TableStPer::{impl#8}::insert)
... self.fn_span = src/Chap42/TableStPer.rs:1627:9: 1852:10

Verus Internal Error: var_local_id failed: (20333, 763)
... self.def_id = DefId(…::Chap42::TableStPer::TableStPer::{impl#8}::insert_wf)
... self.fn_span = src/Chap42/TableStPer.rs:1855:9: 2171:10
```

## Common Pattern in Affected Functions

Two of the three affected functions (`insert`, `insert_wf`) share this pattern:

```rust
fn insert<F: Fn(&V, &V) -> V>(&self, key: K, value: V, combine: F) -> (updated: Self) {
    let ghost mut src: Seq<int> = Seq::empty();   // <-- ghost mut inside &self fn
    let mut i: usize = 0;
    while i < self.entries.length()
        ...
```

The third (`delete_value`) uses `&mut self` with `let ghost seq = self.spec_avltreeseq_seq()`.

The `var_local_id failed` error is a Verus compiler assertion failure triggered by
`new-mut-ref` changing how mutable variable IDs are assigned. The likely trigger is
`let ghost mut` inside a function that takes `&self` (immutable receiver).

## Diagnosis

This is a **Verus upstream compiler bug**, not a proof obligation in APAS-VERUS.
The code is semantically correct and was verified clean without `new-mut-ref`.
The `new-mut-ref` mode changes the MIR lowering path for mutable references and
apparently triggers an ID-mapping assertion for `ghost mut` locals in certain functions.

## Action Taken

1. Added `-V new-mut-ref` to `scripts/validate.sh` default invocation.
2. Ran full validation — crashed with 3 internal panics, 0 verified.
3. Reverted `scripts/validate.sh` to env-var-only `new-mut-ref` (original state).

`new-mut-ref` remains available via: `VERUS_EXTRA_ARGS="-V new-mut-ref" scripts/validate.sh`

## Recommendation

File a Verus upstream bug with the three panic traces. The reproducing pattern is
`let ghost mut <var>` inside a function body, under `-V new-mut-ref`.

Until the upstream fix lands, `new-mut-ref` cannot be the default for APAS-VERUS.

## Stats

| Metric | Value |
|--------|-------|
| Elapsed | 49s |
| Peak rust_verify RSS | 4246 MB |
| Peak Z3 RSS | 0 MB (Z3 never ran) |
| Verified | 0 (crashed before verification) |
| Warnings | 442 |
| Internal panics | 3 |
