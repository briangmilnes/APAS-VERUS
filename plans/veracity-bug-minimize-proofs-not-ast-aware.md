# Veracity Bug: minimize-proofs proof block detector is not AST-aware

## The bug

The proof block detector in `veracity-minimize-proofs` operates on lines, not on the
Rust/Verus AST. It pattern-matches indented lines and treats them as proof blocks. This
causes four symptoms, all from the same root cause.

Discovered during R170 minimize-proofs merge into APAS-VERUS.
Base commit (pre-merge): `a940468df`. Final commit (post-merge): `c4a3c3d7a`.

## The fix

The detector must be token/AST-aware. It must distinguish:

- `proof { }` blocks — testable, may be UNNEEDED
- `assert(...)` statements — testable, may be UNNEEDED
- `ensures`/`requires`/`invariant` clauses — contract, never removable
- `#[cfg(...)]` attributes — structural, never removable
- `let ghost` bindings — not proof blocks, must not be displaced by marker insertion

## Symptom 1: Ensures clauses marked as UNNEEDED proof block

Ensures clause lines look like indented expressions, so the detector treats them as
proof blocks. The function body verifies without the ensures (weaker postcondition is
always easier), but callers depend on the ensures.

### Files fixed during merge

| # | File | Line | Ensures removed | Detected by |
|---|------|------|-----------------|-------------|
| 1 | Chap18/ArraySeqMtPer.rs | 1849 | `it@.1 == self.seq@` | PTT: arrayseqmtper_for_consume |
| 2 | Chap50/OptBinSearchTreeMtPer.rs | 420 | `ensures equal == (self@ == other@)` | Manual audit |
| 3 | Chap50/MatrixChainStPer.rs | 422 | `ensures equal == (self@ == other@)` | Manual audit |

### Unmerged instances in agent worktrees

| # | Agent | File | Line | Ensures removed |
|---|-------|------|------|-----------------|
| 1 | 3 | Chap38/BSTParaStEph.rs | 565 | forall quantifier |
| 2 | 3 | Chap38/BSTParaStEph.rs | 733-736 | finite, union, disjoint |
| 3 | 3 | Chap38/BSTParaMtEph.rs | 586 | union =~= remove |
| 4 | 3 | Chap38/BSTParaMtEph.rs | 715 | len == 0 ==> reduced@ == base@ |
| 5 | 6 | Chap19/ArraySeqMtEphSlice.rs | 711,717 | element equality after append |

## Symptom 2: NEEDED marker insertion displaces exec code

When inserting `// Veracity: NEEDED proof block` markers, the tool places them at the
"first line" of the detected proof block. But `let ghost` bindings preceding a `proof {}`
block are not part of the proof block. The marker lands between the ghost binding and
the proof block, shifting exec-visible code. Verus still verifies (ghost is erased), but
runtime behavior changes.

### Files reverted (30 RTT failures)

| # | File | RTT failures | Reverted in |
|---|------|-------------|-------------|
| 1 | Chap37/BSTRBMtEph.rs | test_rb_balancing | `27a6127d9` |
| 2 | Chap53/PQMinStPer.rs | 6 tests | `c22c64b44` |
| 3 | Chap57/DijkstraStEphF64.rs | 12 tests | `c22c64b44` |
| 4 | Chap57/DijkstraStEphU64.rs | (paired) | `c22c64b44` |
| 5 | Chap59/JohnsonMtEphF64.rs | 7 tests | `c22c64b44` |
| 6 | Chap59/JohnsonStEphF64.rs | 5 tests | `c22c64b44` |
| 7 | Chap59/JohnsonMtEphI64.rs | (paired) | `c22c64b44` |
| 8 | Chap59/JohnsonStEphI64.rs | (paired) | `c22c64b44` |

### Example: BSTRBMtEph rotate_left

```rust
// BEFORE (correct):
let ghost old_x_right = x.right;
proof { reveal_with_fuel(spec_is_bst_link, 2); }

// AFTER (marker displaced the ghost binding):
// Veracity: NEEDED proof block
let ghost old_x_right = x.right;
proof { reveal_with_fuel(spec_is_bst_link, 2); }
```

## Symptom 3: #[cfg(verus_keep_ghost)] treated as proof block

The `#[cfg(verus_keep_ghost)]` attribute on `PartialEqSpecImpl` impls is a compile-time
gate. Commenting it out exposes the impl to cargo, which fails with
`cannot find trait PartialEqSpecImpl in this scope`.

| # | File | Line | Fixed in |
|---|------|------|----------|
| 1 | Chap18/ArraySeqStPer.rs | 1024 | `a940468df` |
| 2 | Chap37/AVLTreeSeq.rs | 1339 | `27a6127d9` |

## Symptom 4: Removed asserts cause conjunction flakiness

Some asserts that are individually UNNEEDED are Z3 speed hints that stabilize later
conjunction proofs. The tool tests each assert in isolation and correctly finds the
function verifies without it. But removing the assert destabilizes a conjunction proof
40 lines later in the same function.

| # | File | Assert removed | Error |
|---|------|---------------|-------|
| 1 | Chap36/QuickSortStEph.rs:503 | `assert(T::le(elem, pivot)); assert(elem != pivot);` | lemma_partition_sort_concat precondition conjunction |

Chap36 was fully reverted in `64c3ac385`.

This symptom is harder to fix with AST awareness alone. Possible mitigations:
- After all individual removals, re-verify the full function and restore if it fails
- Flag functions with known conjunction flakiness as no-minimize

## Severity

| Symptom | Detection | Severity |
|---------|-----------|----------|
| 1. Ensures gutting | PTT or manual audit | High — silent spec weakening |
| 2. Marker shift | RTT | Critical — silent runtime corruption |
| 3. cfg gate removal | RTT compile error | Medium — caught at compile |
| 4. Hint removal | Validate conjunction fail | Medium — caught at verify |
