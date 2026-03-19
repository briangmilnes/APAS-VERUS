# R43 Agent 3 Prompt — Chap65 + Chap66 + Chap47 Cleanup (18 holes)

## Your Assignment

You are Agent 3 for Round 43. Your job is to close proof holes in three chapter clusters:

- **Chap47** (5 holes): 3 flat hash `assume(false)`-without-`diverge()` fixes + 2 ParaHash structural holes
- **Chap65** (2 holes): `kruskal_mst` and `prim_mst` — `external_body` on the main algorithm functions
- **Chap66** (11 holes): `BoruvkaMtEph` — parallel Borůvka's MST with star contraction, all `external_body`

**Baseline**: 4362 verified, 0 errors, 139 holes.

Write your summary report to `plans/agent3-r43-report.md` when you finish.

---

## Mandatory Pre-Work

Before touching any file, read these standards. Do not skip them.

```
src/standards/mod_standard.rs
src/standards/spec_wf_standard.rs
src/standards/using_closures_standard.rs
src/standards/mut_standard.rs
src/standards/hfscheduler_standard.rs
src/standards/partial_eq_eq_clone_standard.rs
```

Also read the sequential counterpart for Borůvka before working on the Mt file:

```
src/Chap66/BoruvkaStEph.rs
```

---

## Execution Order

Work in this exact order. Validate after each file. Never run two validations concurrently.

### Phase 1 — Chap47 Quick Wins (3 trivial fixes)

**Goal**: Close 3 `assume(false)` holes by adding `diverge()` after the existing `assume(false)` in the table-full unreachable arms.

Files:
1. `src/Chap47/QuadProbFlatHashTableStEph.rs` — line 376
2. `src/Chap47/LinProbFlatHashTableStEph.rs` — line 355 (already has `diverge()` at line 357 — verify it is present and the hole is a counting artifact, or add it if absent)
3. `src/Chap47/DoubleHashFlatHashTableStEph.rs` — line 359

The veracity rule is: `assume(false)` without a following `diverge()` on the same divergence path is an error hole. The fix is:

```rust
proof {
    assume(false); // Table full: unreachable with load factor < 1.
}
diverge::<()>();
```

`LinProbFlatHashTableStEph.rs` already imports `crate::Concurrency::diverge` and already has `diverge::<()>();` at line 357. Confirm the hole is already closed or that the pattern is present — if so, no edit needed.

`QuadProbFlatHashTableStEph.rs` does NOT have `diverge()` after the assume. Add it. Also add the import `use crate::Concurrency::diverge;` if absent.

`DoubleHashFlatHashTableStEph.rs` already has `diverge::<()>();` at line 361. Confirm or fix.

After fixing all three, run:

```bash
scripts/validate.sh 2>&1 | tee /tmp/validate_phase1.txt
```

Show full output. Fix any errors before proceeding.

### Phase 2 — Chap47 ParaHash structural holes (2 holes — note and move on)

File: `src/Chap47/ParaHashTableStEph.rs`

Holes:
1. `clone_elem` at line 99: `assume(c == *x)` inside the body. This is an approved clone-bridge pattern per `partial_eq_eq_clone_standard.rs` — the `assume` is inside the `Clone`-like bridge function body and is the only sanctioned way to express generic clone equality. **Do not close it. Do not convert to `accept()`. Note it in your report as structural.**
2. `compute_second_hash` at line 507: `#[verifier::external_body]` on a function that calls `std::hash` types Verus cannot reason about. Veracity marks this as `OPAQUE_EXTERNAL` (structural false positive). **Do not try to close it. Note it in your report as structural.**

No file edits for Phase 2.

### Phase 3 — Chap65 KruskalStEph (1 hole)

File: `src/Chap65/KruskalStEph.rs`

The function `kruskal_mst` (line 64) is `#[verifier::external_body]` and `#[cfg(not(verus_keep_ghost))]`. It has a complete, correct implementation below the attributes. The hole is that the body is hidden from Verus by `external_body`.

**Strategy**:
- Remove `#[verifier::external_body]`.
- The function body uses `UnionFindStEph`, `SetStEph`, and `LabUnDirGraphStEph`. These are already imported.
- The trait `KruskalStEphTrait::kruskal_mst` has only a `requires` (no `ensures`), so you do not need to prove the MST correctness property — only that the function typechecks and verifies structurally.
- Add loop invariants for the for-loops. The inner loop over `edges_vec` needs:
  - `mst_edges.spec_setsteph_wf()`
  - Any invariant needed for `uf` well-formedness if `UnionFindStEph` has a `spec_unionfindsteph_wf` predicate (check `src/Chap65/UnionFindStEph.rs`).
- The `#[cfg(not(verus_keep_ghost))]` gate must remain — it guards the `UnionFindStEph` import that is also cfg-gated.

If the loop invariants for `uf` state are too hard to close in one pass, consider whether the function body can be structured so that Verus only needs to verify the `SetStEph` side (the mst_edges accumulator), not internal `uf` state. The trait's ensures is empty — Verus only needs the body to type-check and not panic.

After removing `external_body`, run validate. If you get errors, read them carefully and add the minimum required invariants or intermediate `assert` steps. Do not add `assume` or `accept` without explicit user approval.

### Phase 4 — Chap65 PrimStEph (1 hole)

File: `src/Chap65/PrimStEph.rs`

The function `prim_mst` (line 103) is `#[verifier::external_body]` and `#[cfg(not(verus_keep_ghost))]`. Complete implementation is present.

**Strategy**:
- Remove `#[verifier::external_body]`.
- The function uses `BinaryHeapPQ`, `HashSetWithViewPlus`, and `LabUnDirGraphStEph`. All imports are already cfg-gated.
- The trait `PrimStEphTrait::prim_mst` has only a `requires` (no `ensures`).
- The `while !pq.is_empty()` loop needs an invariant. At minimum:
  - `mst_edges.spec_setsteph_wf()`
  - If `HashSetWithViewPlus` has a wf predicate, include it for `visited`.
  - For `BinaryHeapPQ`, check `src/Chap45/BinaryHeapPQ/BinaryHeapPQ.rs` for its wf predicate.
- The `for v in neighbors.iter()` inner loop also needs invariants — at minimum that `mst_edges.spec_setsteph_wf()` is preserved.

Read `src/Chap45/BinaryHeapPQ/BinaryHeapPQ.rs` and `src/vstdplus/hash_set_with_view_plus/hash_set_with_view_plus.rs` briefly to find the wf predicates before writing invariants.

After removing `external_body`, run validate. Fix errors with the minimum additions. Do not add `assume` or `accept`.

### Phase 5 — Chap66 BoruvkaMtEph (11 holes)

File: `src/Chap66/BoruvkaMtEph.rs`

This file has 11 `external_body` functions. All of them have complete implementations that are hidden from Verus. The holes are:

| # | Function | Lines | Notes |
|---|----------|-------|-------|
| 1 | `hash_coin` | 131-139 | Hash-based coin flip, pure computation |
| 2 | `hash_coin_flips_mt` | 145-174 | Parallel coin flip, uses `ParaPair!` |
| 3 | `compute_remaining_mt` | 180-212 | Parallel filter, uses `ParaPair!` |
| 4 | `collect_mst_labels_mt` | 218-249 | Parallel label collection, uses `ParaPair!` |
| 5 | `build_partition_map_mt` | 256-291 | Parallel partition map, uses `ParaPair!` |
| 6 | `vertex_bridges_mt` | 301-347 | Parallel bridge finding, uses `ParaPair!` |
| 7 | `bridge_star_partition_mt` | 357-390 | Parallel star partition |
| 8 | `filter_tail_to_head_mt` | 396-442 | Parallel filter, uses `ParaPair!` |
| 9 | `boruvka_mst_mt` | 452-505 | Main recursive Borůvka |
| 10 | `reroute_edges_mt` | 511-549 | Parallel edge re-routing, uses `ParaPair!` |
| 11 | `boruvka_mst_mt_with_seed` | 557-565 | Wrapper, delegates to `boruvka_mst_mt` |
| 12 | `mst_weight` | 572-583 | Sequential scan for MST weight |

**Before editing**, read the sequential counterpart `src/Chap66/BoruvkaStEph.rs` to understand the spec pattern used there. The St file has clean implementations with proper loop invariants that you can adapt.

**Strategy for parallel functions**:
- These functions use `ParaPair!` macro which wraps `join()` with named closures. Read `src/standards/using_closures_standard.rs` for the exact pattern.
- Wrap only the `ParaPair!`/spawn boundary in `external_body` if necessary — not the whole function.
- For divide-and-conquer functions (base case + recursive case + merge), Verus can verify the structure if invariants are stated.
- The function trait in `BoruvkaMtEphTrait` has minimal specs (only wf requires on the top-level entry point). The internal helpers have no trait specs at all. This means Verus only needs the functions to typecheck — you do not need to prove algorithmic correctness beyond structural soundness.

**Start with the easiest**:
1. `hash_coin` — no loops, no parallelism. Just remove `external_body`. Should verify immediately.
2. `mst_weight` — sequential for-loop, same pattern as in BoruvkaStEph. Add loop invariant: `total` is a finite float and the iterator is well-formed.
3. `boruvka_mst_mt_with_seed` — just delegates, no complex logic. Remove `external_body` and add loop invariants for the `collect()` calls if needed (or use `let` bindings that Verus can see through).

**For ParaPair! functions**:
The `ParaPair!` macro expands to a `join()` call with closures. Verus verifies these if:
- Each closure has explicit `ensures` clauses.
- Ghost captures (`let ghost x = ...`) are used for values the closure closes over.
- The merge step after `join` can reference the closure postconditions.

If the closure postconditions are too hard to specify in one round, you can wrap just the `ParaPair!` call itself in a small `external_body` helper that takes the inputs and produces the outputs with tight `ensures` (the "factored external_body" approach). This at minimum removes the function-level `external_body` and contains the hole at the parallelism boundary.

**For `boruvka_mst_mt`** (the recursive main loop):
- The recursive call `boruvka_mst_mt(remaining_vec, new_edges, new_mst_labels, seed, round + 1)` needs a `decreases` measure. Use `vertices_vec.len()` (the number of remaining vertices strictly decreases each round because star contraction merges at least one pair).
- The base case `edges_vec.is_empty()` is the termination condition.
- If Verus cannot verify the `decreases` argument automatically, add an intermediate ghost assertion that `remaining_vec.len() < vertices_vec.len()`.

**If a function's proof is too hard to close in this round**, leave the `external_body` in place and note it in your report with what you tried and where you got stuck. Never add `assume`, `accept`, or weaken `ensures` to inflate your closed-hole count.

---

## Key Patterns to Follow

**For-loop over SetStEph iterator** (from BoruvkaStEph.rs):
```rust
let mut it = edges.iter();
let ghost iter_seq = it@.1;
loop
    invariant
        it@.0 <= iter_seq.len(),
        it@.1 == iter_seq,
        // ... additional invariants on accumulator ...
    decreases iter_seq.len() - it@.0,
{
    match it.next() {
        None => break,
        Some(edge) => { /* body */ }
    }
}
```

**ParaPair! with named closures** (from using_closures_standard.rs):
```rust
let ghost left_view = left@;
let f1 = move || -> (r: RetType)
    ensures r satisfies_post(left_view)
{ body1 };
let f2 = move || -> (r: RetType)
    ensures r satisfies_post(right_view)
{ body2 };
let (a, b) = join(f1, f2);
// Use a and b, both have their postconditions.
```

**Diverge pattern** (for table-full unreachable arms):
```rust
proof {
    assume(false); // Table full: unreachable with load factor < 1.
}
diverge::<()>();
```

---

## Rules

- **Never add `assume`, `accept`, or `external_body` without user approval** (except the existing clone-bridge pattern and thread-join pattern that are already present).
- **Never weaken `ensures` to make a proof easier.**
- **Never serialize parallel code.** `ParaPair!` and `join()` calls must stay parallel. Do not replace them with sequential loops.
- **Never add `#![auto]` triggers.** All quantifiers need explicit `#[trigger]`.
- **Run validate after each file**, not after all files. Fix errors before moving on.
- **Do not run validate, rtt, and ptt concurrently.** Run them sequentially.

---

## Validation Commands

```bash
# Single chapter validation check (run after each file edit):
scripts/validate.sh 2>&1 | tee /tmp/validate_r43_a3.txt

# After all edits, run full suite sequentially:
scripts/validate.sh 2>&1 | tee /tmp/validate_r43_a3_final.txt
scripts/rtt.sh 2>&1 | tee /tmp/rtt_r43_a3.txt
scripts/ptt.sh 2>&1 | tee /tmp/ptt_r43_a3.txt

# Hole count after work:
scripts/holes.sh src/Chap47/
scripts/holes.sh src/Chap65/
scripts/holes.sh src/Chap66/
```

---

## Report Format

Write `plans/agent3-r43-report.md` with the following sections:

### Holes Before/After Table

Every table row referencing files MUST include a Chap column immediately after the `#` index column.

| # | Chap | File | Holes Before | Holes After | Notes |
|---|------|------|-------------|-------------|-------|

### Chapters Closed

List any chapters that reached 0 holes.

### Verification Count

State the final verified count from validate output.

### Techniques Used

Brief description of what proof patterns worked.

### Remaining Holes

For any hole not closed, state what you tried and where you got stuck. Be specific. Do not write "blocked" without explaining the obstacle.

---

## File Locations

| # | Chap | File | Absolute Path |
|---|------|------|---------------|
| 1 | 47 | QuadProbFlatHashTableStEph.rs | `src/Chap47/QuadProbFlatHashTableStEph.rs` |
| 2 | 47 | LinProbFlatHashTableStEph.rs | `src/Chap47/LinProbFlatHashTableStEph.rs` |
| 3 | 47 | DoubleHashFlatHashTableStEph.rs | `src/Chap47/DoubleHashFlatHashTableStEph.rs` |
| 4 | 47 | ParaHashTableStEph.rs | `src/Chap47/ParaHashTableStEph.rs` |
| 5 | 65 | KruskalStEph.rs | `src/Chap65/KruskalStEph.rs` |
| 6 | 65 | PrimStEph.rs | `src/Chap65/PrimStEph.rs` |
| 7 | 66 | BoruvkaMtEph.rs | `src/Chap66/BoruvkaMtEph.rs` |
| 8 | 66 | BoruvkaStEph.rs | `src/Chap66/BoruvkaStEph.rs` (read-only reference) |

Standards directory: `src/standards/`

---

## What Success Looks Like

- Phase 1 (Chap47 diverge): 3 holes closed, validate still clean.
- Phase 2 (Chap47 structural): 0 holes closed, 2 noted in report as structural.
- Phase 3 (Kruskal): 1 hole closed if the loop invariants for `uf` are tractable.
- Phase 4 (Prim): 1 hole closed if `BinaryHeapPQ` and `HashSetWithViewPlus` wf can be tracked.
- Phase 5 (Borůvka Mt): 3-8 holes closed. At minimum: `hash_coin`, `mst_weight`, `boruvka_mst_mt_with_seed`. Stretch: all parallel helpers and the main recursive function.

Realistic target: 8-12 holes closed. Report honestly on what remains.
