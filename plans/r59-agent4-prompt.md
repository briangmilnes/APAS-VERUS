# R59 Agent 4 — Stabilize Flaky Z3 Proofs

## Assignment

Fix 3 flaky proofs that cause nondeterministic `4484 verified, 1 errors` results.
These are NOT holes — they are proofs that pass sometimes and fail sometimes due to
Z3 resource limits. The fix is adding intermediate assertions so Z3 doesn't have to
search as hard. No specs are weakened.

**Zero hole-count impact. High stability impact.**

## Targets

### Target 1: StarContractionStEph.rs — `star_contract` (line 129)

**Flaky postcondition (line 142):**
```rust
ensures
    graph@.A.is_empty() ==>
        exists|s: &SetStEph<V>| s@ == graph@.V && s.spec_setsteph_wf()
            && base.ensures((s,), result),
```

The function body is just:
```rust
star_contract_fuel(graph, base, expand, graph.sizeV())
```

The callee `star_contract_fuel` (line 67) ensures:
```rust
(graph@.A.is_empty() || fuel == 0) ==>
    exists|s: &SetStEph<V>| s@ == graph@.V && s.spec_setsteph_wf()
        && base.ensures((s,), result),
```

Z3 flakes because it must see that `graph@.A.is_empty()` implies the callee's
antecedent `(graph@.A.is_empty() || fuel == 0)`, and then propagate the existential.

**Fix:** Bind the result and add a proof block:
```rust
{
    let result = star_contract_fuel(graph, base, expand, graph.sizeV());
    proof {
        if graph@.A.is_empty() {
            // Callee guarantees the existential when antecedent holds
            assert(graph@.A.is_empty() || graph.sizeV() == 0);
            // The existential from star_contract_fuel now flows
        }
    }
    result
}
```

If that's not enough, try the explicit witness pattern:
```rust
proof {
    if graph@.A.is_empty() {
        assert(exists|s: &SetStEph<V>| s@ == graph@.V
            && s.spec_setsteph_wf() && base.ensures((s,), result));
    }
}
```

### Target 2: StarContractionMtEph.rs — `star_contract_mt` (same pattern)

Check `star_contract_mt` (the wrapper around `star_contract_mt_fuel`). It has the
same existential ensures at line ~143. Apply the same fix.

`star_contract_mt_fuel` (line 86) has the same structure as the StEph version.

### Target 3: OrderedSetStPer.rs — `get_range` (line 898)

**Flaky loop invariant (line 910):**
```rust
size as nat == self@.len(),
```

The loop in `get_range` iterates over `elements` (an AVL tree sequence), collecting
elements in range [k1, k2]. The invariant `size as nat == self@.len()` should hold
trivially because `self` is `&self` (immutable). But after `result = result.insert(v)`
at ~line 942, Z3 sometimes loses track of the fact that `self@` didn't change.

**Fix:** Add an explicit reassertion after the insert:
```rust
result = result.insert(v);
proof {
    // self is immutable — reassert for Z3
    assert(size as nat == self@.len());
}
```

Also add an assertion before the loop to anchor the invariant:
```rust
assert(size as nat == self@.len());
while i < size
```

Check if `OrderedSetStEph.rs` has the same `get_range` function with the same
pattern — if so, apply the same fix there too for consistency (even if it hasn't
flaked yet).

## Validation Protocol

Run `scripts/validate.sh` **at least 3 times** after all fixes to confirm stability.
If any run still shows 1 error, read the error location and add more assertions.
The goal is consistent `4484+ verified, 0 errors` across multiple runs.

## Do NOT

- Add `assume`, `accept`, or `external_body`
- Weaken any `ensures` clause
- Change any spec function
- Modify any function signatures

## Report

Write `plans/agent4-round59-report.md` with:
- For each target: what assertion was added, which line
- Validation results (at least 3 runs showing 0 errors)
- Verification count
