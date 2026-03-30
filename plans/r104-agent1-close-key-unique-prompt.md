# R104 Agent 1 — Close spec_key_unique_pairs_set in OrderedTableStPer, STEP 20

## Objective

`spec_key_unique_pairs_set` is an `open spec fn` with a 3-variable `forall` that
leaks into Z3 for every function downstream of OrderedTableStPer. Profiling shows
it at 137K instantiations (#8 most expensive quantifier across the codebase).
It also drives `lemma_sorted_keys_pairwise_distinct` (250K) and `no_duplicates`
(250K) as collateral.

Close it. Add `reveal` only where the quantifier body is actually needed.

## The change

In `src/Chap43/OrderedTableStPer.rs`:

```rust
// Before:
pub open spec fn spec_key_unique_pairs_set<KV, VV>(s: Set<(KV, VV)>) -> bool {
    forall|k: KV, v1: VV, v2: VV|
        s.contains((k, v1)) && s.contains((k, v2)) ==> v1 == v2
}

// After:
pub closed spec fn spec_key_unique_pairs_set<KV, VV>(s: Set<(KV, VV)>) -> bool {
    forall|k: KV, v1: VV, v2: VV|
        s.contains((k, v1)) && s.contains((k, v2)) ==> v1 == v2
}
```

Then add `reveal(spec_key_unique_pairs_set::<KV, VV>)` in proof blocks of
functions that need the quantifier body. Most callers just need the fact
`spec_key_unique_pairs_set(s)` to be true — they don't need the forall unfolded.

## Where reveals are likely needed

Search for functions that:
1. Mention `spec_key_unique_pairs_set` in their body (not just requires/ensures)
2. Assert something about `s.contains((k, v1))` and `s.contains((k, v2))`
3. Prove `v1 == v2` from key uniqueness

These are probably in:
- `lemma_sorted_keys_pairwise_distinct`
- `lemma_pair_set_to_map_*` family
- `insert` proof
- `delete` proof
- `difference`, `subtract`, `restrict` proofs
- Possibly callers in other Chap43 files (OrderedTableStEph, OrderedSetStPer)

## Method

1. Make the one-line change (`open` → `closed`)
2. Run `scripts/validate.sh isolate Chap43` to see what breaks
3. For each error, read the function, add `reveal(spec_key_unique_pairs_set::<KV, VV>)`
   in a `proof { }` block at the top of the function body (or inside the relevant
   proof block if it's a proof fn)
4. Iterate until Chap43 is clean
5. Run `scripts/validate.sh isolate Chap52` to check downstream (graph representations)
6. Fix any downstream breaks the same way
7. Run full `scripts/validate.sh` to confirm everything passes

## Profile before/after

After the fix verifies, run:
```bash
scripts/profile.sh isolate Chap43
scripts/profile.sh isolate Chap52
```

Report the before/after instantiation counts for:
- `spec_key_unique_pairs_set`
- `lemma_sorted_keys_pairwise_distinct`
- `no_duplicates`
- Total per-module

Before counts (from existing profiles):
- Chap43: 5,254,378 total
- Chap52: 5,554,249 total
- `spec_key_unique_pairs_set_174`: 137,361
- `lemma_sorted_keys_pairwise_distinct_246`: 250,101
- `no_duplicates_104`: 250,641

## Isolation

Start with:
```bash
scripts/validate.sh isolate Chap43
```

Then expand to Chap52, then full.

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT add `assume` or `accept`.
- Do NOT change the spec fn body — only change `open` to `closed`.
- Do NOT change any other spec fn.
- `reveal` syntax: `reveal(spec_key_unique_pairs_set::<KV, VV>);` inside a
  `proof { }` block. The type params must match the function's generics.
- If a function breaks and you can't figure out where to reveal, leave it
  and report which function failed.

## STEP 20

## Report

Write `plans/agent1-r104-close-key-unique-report.md`.
