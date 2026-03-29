# Agent 2 — R102 Report: StarPartitionMtEph Proof

## Objective

Remove `#[verifier::external_body]` from `parallel_star_partition` in
`src/Chap62/StarPartitionMtEph.rs` and prove the function body verifies.

## Result

**Success.** The external_body is removed. Chap62 has **0 holes** across all 4 files.
Isolated validation: **1253 verified, 0 errors, 0 warnings**.

## Holes Before/After

| # | Chap | File | Before | After |
|---|------|------|--------|-------|
| 1 | 62 | StarPartitionMtEph.rs | 1 | 0 |
| 2 | 62 | StarPartitionStEph.rs | 0 | 0 |
| 3 | 62 | StarContractionMtEph.rs | 0 | 0 |
| 4 | 62 | StarContractionStEph.rs | 0 | 0 |

## Techniques Used

### 1. View-injective bridge (`obeys_feq_view_injective`)

The core problem: `SetStEph::to_seq()` ensures value-level `no_duplicates`
(`seq@[i] != seq@[j]`), but the proof needs view-level (`seq@[i]@ != seq@[j]@`).

Solution: `obeys_feq_view_injective::<V>()` from `obeys_feq_full::<V>()` gives
`x@ == y@ ==> x == y`. Combined with value-level `no_duplicates`, view-equal
elements lead to value-equal elements, contradicting distinctness.

Added `broadcast use group_feq_axioms` and explicit
`assert(obeys_feq_view_injective::<V>())` at each contradiction point (Loops 1, 2, 6).

### 2. Proof-after-mutation pattern

The original proof sketched invariants BEFORE `push`/`set` using ghost `new_entry`
values. But `clone_view()` produces a different value-level object (same view).
The `assert forall` about `th_edges@.push(ghost_entry)` didn't match the actual
`th_edges@` after pushing `clone_view()`.

Fix: move `push`/`set` BEFORE the proof block, then prove invariants about the
actual post-mutation state using `pre_th`/`pre_p` ghost snapshots.

### 3. Loop invariant carry-through

Verus loop bodies only have access to their declared invariants. Facts from before
a loop are lost unless carried as invariants. Added carry-through invariants to
Loop 4 (which bridges Loops 3→5) for: `valid_key_type_Edge`, `vertices_vec@.no_duplicates()`,
vertex_to_index domain, coin_flips coverage, th_edges properties, graph vertex membership.

### 4. Trigger standardization

The th_edges invariant trigger differed between loops (`contains_key` vs map-index).
Standardized all instances to use `#[trigger] coin_flips@.contains_key(vertices_vec@[...])`.
Added explicit trigger-firing assertions at merge points and loop body ends.

### 5. `feq()` for exec-level equality

Generic `V: PartialEq` doesn't have `ensures r == (self@ == other@)` on `PartialEq::eq`.
Replaced `*vertex == center` with `feq(vertex, &center)` which provides the view-equality
ensures via `obeys_feq_full`.

### 6. `implies` vs `==>`

Verus `assert forall` only assumes the antecedent of the outermost `implies`, not
inner `==>`. Restructured nested implications to use `(cond1 && cond2) implies conclusion`
so both conditions are assumed in the `by` block.

## Verification

- Isolated: `scripts/validate.sh isolate Chap64` — 1253 verified, 0 errors
- Full validation not yet run (user requested commit first)
