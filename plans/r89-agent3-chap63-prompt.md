# R89 Agent 3 — Fix ConnectivityMtEph verify errors (Chap63), STEP 20

## Objective

Fix 3 verification errors in `src/Chap63/ConnectivityMtEph.rs`. The file compiles
but has loop invariant and precondition failures.

## Error Locations

1. **Line 132**: precondition not satisfied — `HashMapWithViewPlus::new()` requires
   `obeys_key_model::<V>()` or similar. Likely needs the precondition added to the
   enclosing function's requires or proved from existing bounds.

2. **Line 138**: invariant `forall|k1: V, k2: V| k1@ == k2@ ==> k1 == k2` — this
   is the feq/key model invariant. Needs to be provable from the type bounds on V.
   Check if `obeys_feq_clone::<V>()` or `valid_key_type::<V>()` is in scope.

3. **Line 139**: invariant `result@.contains_key(k) ==> partition_map@.contains_key(k)`
   — the loop body inserts into `result` from `partition_map`, but the invariant isn't
   maintained. Need to prove that keys inserted into result come from partition_map.

## Read first

- `src/Chap63/ConnectivityMtEph.rs` — your file
- `src/Chap62/StarContractionMtEph.rs` — star_contract_mt that ConnectivityMtEph calls
- `src/vstdplus/hash_map_with_view_plus.rs` — HashMapWithViewPlus API and requires

## lib.rs — do NOT modify

The file is already uncommented.

## Isolation

```bash
scripts/validate.sh isolate Chap63
```

If Chap63 doesn't have an isolate feature, use:
```bash
scripts/validate.sh isolate Chap64
```

## No Subagents

Do NOT use the Agent tool to spawn subagents.

## Important

- Do NOT modify files outside Chap63.
- Do NOT add assume or accept.
- Use external_body only as last resort.

## STEP 20

## Report

Write `plans/agent3-r89-chap63-report.md`.
