# R105 Agent 2 — Opaque spec_key_unique_pairs_set in OrderedTable, STEP 20

## Objective

`spec_key_unique_pairs_set` is a 3-variable `forall` that fires 91K times in
OrderedTableStPer and drives `lemma_sorted_keys_pairwise_distinct` (167K) and
`no_duplicates` (226K) as collateral. Combined ~580K instantiations, #2 actionable
target after feq.

The spec fn and all its callers are in the same module, so `closed` doesn't help.
Use `#[verifier::opaque]` instead — it hides the body from Z3 even within the
same module. Add `reveal(spec_key_unique_pairs_set)` only where the body is needed.

## The change

In BOTH `src/Chap43/OrderedTableStPer.rs` and `src/Chap43/OrderedTableStEph.rs`:

```rust
// Before:
pub open spec fn spec_key_unique_pairs_set<KV, VV>(s: Set<(KV, VV)>) -> bool {
    forall|k: KV, v1: VV, v2: VV|
        s.contains((k, v1)) && s.contains((k, v2)) ==> v1 == v2
}

// After:
#[verifier::opaque]
pub open spec fn spec_key_unique_pairs_set<KV, VV>(s: Set<(KV, VV)>) -> bool {
    forall|k: KV, v1: VV, v2: VV|
        s.contains((k, v1)) && s.contains((k, v2)) ==> v1 == v2
}
```

Note: `opaque` goes on `open spec fn`, not `closed`. It stays `open` (visible to
other modules if they `reveal`) but Z3 doesn't see the body by default.

## Where reveals are needed

Functions that actually reason about `s.contains((k, v1)) && s.contains((k, v2)) ==> v1 == v2`:

- `lemma_sorted_keys_pairwise_distinct` — needs the forall to prove no_duplicates
- `lemma_pair_set_to_map_*` family — needs key uniqueness for map construction
- `lemma_key_unique_insert` — proves uniqueness preserved after insert
- `lemma_key_unique_remove` — proves uniqueness preserved after remove
- `lemma_key_unique_subset` — proves uniqueness preserved for subsets
- `lemma_key_unique_disjoint_union` — proves uniqueness for union of disjoint sets
- Any function that asserts `v1 == v2` from two `contains` facts

Functions that just carry `spec_key_unique_pairs_set(s)` in requires/ensures
(passing it through without unfolding) should NOT need a reveal.

## Method

1. Add `#[verifier::opaque]` to the spec fn in OrderedTableStPer.rs
2. Run `scripts/validate.sh isolate Chap43` to see what breaks
3. For each error, add `reveal(spec_key_unique_pairs_set::<KV, VV>)` in the
   proof block where the body is needed
4. Iterate until Chap43 is clean
5. Do the same for OrderedTableStEph.rs
6. Run `scripts/validate.sh isolate Chap52` to check downstream
7. Run full `scripts/validate.sh`

## Profile after

After full validate passes, run:
```bash
scripts/profile.sh isolate Chap43
scripts/profile.sh isolate Chap52
```

Report before/after for:
- `spec_key_unique_pairs_set`: before 91,557
- `lemma_sorted_keys_pairwise_distinct`: before 166,734
- `no_duplicates`: before 226,465 (may drop as collateral)
- Chap43 total: before 5,254,378
- Chap52 total: before 5,554,249

## Scope

ONLY modify files in `src/Chap43/`. Do NOT modify:
- vstdplus/ (agent1 is working on feq there)
- Any other chapter
- If downstream chapters break (Chap44, 52, etc.), report which functions fail

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT add `assume` or `accept`.
- Do NOT change the spec fn body.
- Do NOT change `open` to `closed` — use `#[verifier::opaque]` on the `open` fn.
- `reveal` syntax: `reveal(spec_key_unique_pairs_set::<KV, VV>);`
- If a function needs the body in its ensures (not just carries the predicate),
  it may need `reveal` in the ensures clause via `ensures ... by { reveal(...) }`.
  But try without first — most ensures just state `spec_key_unique_pairs_set(result)`
  without needing the body.

## STEP 20

## Report

Write `plans/agent2-r105-opaque-key-unique-report.md`.
