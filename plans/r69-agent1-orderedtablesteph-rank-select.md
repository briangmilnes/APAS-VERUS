# R69 Agent 1: OrderedTableStEph rank_key + select_key Proofs

## Goal

Prove `rank_key_iter` and `select_key` in `src/Chap43/OrderedTableStEph.rs`, removing
the 2 `external_body` marks. Target: 10 → 8 holes.

## Context

Agent 3 (R68) proved the identical `rank_key_iter` and `select_key` admits in
OrderedTableStPer.rs. Read that file for the working proof technique.

## Technique (from Agent 3 R68 report)

### rank_key_iter

Ghost set `counted_keys: Set<K::V>` tracks which keys have been counted as < k.
- On `Less` branch: insert key into counted_keys
- On `Equal` branch: prove `!filter_pred` via view equality contradiction
- On `Greater` branch: prove `!filter_pred` via `TotalOrder::antisymmetric` + contradiction
- Post-loop: set extensionality connects `counted_keys` to `dom().filter(pred)`

### select_key

Loop invariant on `result_key matches Some(rk)` with `rank_key` ensures. Postconditions
are conditional on `Some`, so vacuously true if no match found.

## Steps

1. **Read** `src/Chap43/OrderedTableStPer.rs` — find rank_key_iter and select_key
   implementations (the proved versions)
2. **Read** `src/Chap43/OrderedTableStEph.rs` — find the external_body marks at
   lines ~3408 and ~3435
3. **Adapt** Agent 3's proof from StPer to StEph:
   - StEph uses `&mut self` (ephemeral), StPer returns new values (persistent)
   - The ghost proof logic should be nearly identical
   - Key imports may differ slightly
4. **Remove** both `#[verifier::external_body]` annotations
5. **Validate**, **rtt**, **ptt**

## Also Fix

- `fn_missing_wf_ensures` on `from_sorted_entries` (line ~3752): add
  `ensures result.spec_orderedtablesteph_wf()` to the signature

## Constraints

- Only modify `src/Chap43/OrderedTableStEph.rs`.
- Do NOT add new `assume`, `accept`, or `external_body`.
- Do NOT weaken ensures.
- Run validate, rtt, ptt sequentially.
