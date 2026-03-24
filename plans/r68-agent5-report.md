# Agent 5 Round 68 Report

## Task 1: Strengthen `collect_in_order` Trait Ensures (Chap38)

**File**: `src/Chap38/BSTParaStEph.rs`

The trait's `collect_in_order` only ensured `out@.len() == old(out)@.len() + self@.len()`.
The impl already proved stronger postconditions including element preservation, membership,
and completeness. Lifted the impl's ensures to the trait declaration.

**Change**: Added three ensures clauses to `BSTParaStEphTrait::collect_in_order`:
1. Element preservation: prior elements unchanged
2. Membership: new elements are in `self@`
3. Completeness: every value in `self@` appears in the output

`in_order` trait already had strong bidirectional membership ensures — no change needed.

## Task 2: Repair Chap53 StPer Files After AVLTreeSetStPer Rewire

**Files**: `src/Chap53/PQMinStPer.rs`, `src/Chap53/GraphSearchStPer.rs`

R67 rewired AVLTreeSetStPer from `AVLTreeSeqStPerS<T>` backing to `ParamBST<T>`. The old
`.elements` field (a sequence type with `.length()`, `.nth()`, `.spec_seq()`) became
`.tree` (a set type with `.size()`, `.to_seq()`). Both Chap53 StPer files were broken and
commented out.

### Changes Applied

| # | Change Category | Description |
|---|-----------------|-------------|
| 1 | Field access | `.elements.length()` → `.size()` |
| 2 | Indexed access | `.elements.nth(i)` → `.to_seq().nth(i)` |
| 3 | Spec references | `.elements.spec_seq().len()` → `@.len()` |
| 4 | New requires | Added `obeys_cmp_spec::<T>()` and `view_ord_consistent::<T>()` |
| 5 | Proof hints | Added set non-emptiness proof for `nth(0)` preconditions |
| 6 | Import gate | `#[cfg(verus_keep_ghost)]` on `view_ord_consistent` import |

For PQMinStPer, cmp specs needed for three compound types: `V`, `Pair<Pair<P,V>,V>`,
`Pair<V,P>`.

### Infrastructure

- Uncommented `PQMinStPer` and `GraphSearchStPer` in `src/lib.rs`
- Uncommented `TestPQMinStPer` and `TestGraphSearchStPer` in `Cargo.toml`

## Verification Results

| # | Step | Result |
|---|------|--------|
| 1 | validate | 4393 verified, 0 errors |
| 2 | rtt | 2528 passed, 0 skipped |
| 3 | ptt | 145 passed, 0 skipped |

## Hole Counts

| # | Chap | Status | Holes |
|---|------|--------|-------|
| 1 | 38 | Clean | 0 |
| 2 | 53 | Clean | 0 |

Both chapters remain at 0 actionable holes.

## Techniques Used

- Lifted impl ensures to trait declaration (pattern: impl proves more than trait declares)
- `to_seq()` bridge for indexed access on set-backed containers
- Set non-emptiness proof via contradiction for `nth` preconditions
- `obeys_cmp_spec`/`view_ord_consistent` propagation through requires and loop invariants
- `#[cfg(verus_keep_ghost)]` gate for spec-only imports
