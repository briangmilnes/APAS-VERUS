# Veracity Bugs: minimize-proofs R170

Three bugs discovered during R170 minimize-proofs merge into APAS-VERUS main.

Base commit (pre-merge): `a940468df`
Final commit (post-merge): `c22c64b44`

---

## Bug 1: Ensures clauses marked as UNNEEDED proof block

### Description

`veracity-minimize-proofs` incorrectly identifies `ensures` clauses as "proof blocks"
and comments them out with `// Veracity: UNNEEDED proof block`. The function body
verifies without the ensures (weaker postcondition is always easier to prove), but
callers depend on the ensures to prove their obligations.

### Examples found and fixed

| # | File | Line | Ensures removed | Detected by |
|---|------|------|-----------------|-------------|
| 1 | Chap18/ArraySeqMtPer.rs | 1849 | `it@.1 == self.seq@` (into_iter) | PTT fail: arrayseqmtper_for_consume, arrayseqmtper_loop_consume |
| 2 | Chap50/OptBinSearchTreeMtPer.rs | 420 | `ensures equal == (self@ == other@)` (PartialEq::eq) | Manual audit |
| 3 | Chap50/MatrixChainStPer.rs | 422 | `ensures equal == (self@ == other@)` (PartialEq::eq) | Manual audit |

### Examples found in agent worktrees (not yet merged)

| # | Agent | File | Line | Ensures removed |
|---|-------|------|------|-----------------|
| 1 | 3 | Chap38/BSTParaStEph.rs | 565 | forall quantifier in ensures |
| 2 | 3 | Chap38/BSTParaStEph.rs | 733-736 | 4 ensures clauses (finite, union, disjoint) |
| 3 | 3 | Chap38/BSTParaMtEph.rs | 586 | ensures: union =~= remove |
| 4 | 3 | Chap38/BSTParaMtEph.rs | 715 | ensures: len == 0 ==> reduced@ == base@ |
| 5 | 6 | Chap19/ArraySeqMtEphSlice.rs | 711,717 | ensures: element equality after append |

### Root Cause

The proof block detection heuristic matches any indented line between braces. An
ensures clause line looks like an indented expression, so it gets detected as a
"proof block". The detector does not distinguish between:
- Lines inside `proof { }` blocks (removable)
- Lines in `ensures` clauses between `ensures` and `{` (never removable)
- Lines in `invariant` blocks (removable as loop invariants)

### Fix

The proof block detector must track parser state: inside `ensures`/`requires` blocks
(between the keyword and the opening `{`), lines are contract, not proof. Only lines
inside `proof { }` blocks or `assert(...)` statements are testable.

---

## Bug 2: NEEDED marker insertion shifts exec code, corrupting runtime behavior

### Description

When `veracity-minimize-proofs` inserts `// Veracity: NEEDED proof block` comment
markers, it places them at the beginning of the proof block it just tested. But if
exec code (including `let ghost` bindings) is interleaved with proof blocks, the
marker can land between an exec statement and the proof block that follows it,
effectively reordering the exec code.

Ghost bindings don't affect Verus verification (ghost code is erased), but the
reordering can shift exec code relative to `let mut` bindings, `if let` arms, or
other exec-visible statements, corrupting runtime behavior.

### Files reverted due to this bug

| # | File | RTT failures | Commit before | Commit after (reverted) |
|---|------|-------------|---------------|------------------------|
| 1 | Chap37/BSTRBMtEph.rs | test_rb_balancing (height <= 6) | `a940468df` | reverted in `27a6127d9` |
| 2 | Chap53/PQMinStPer.rs | 6 tests (test_pq_min_*) | `27a6127d9` | reverted in `c22c64b44` |
| 3 | Chap57/DijkstraStEphF64.rs | 12 tests (all Dijkstra F64) | `27a6127d9` | reverted in `c22c64b44` |
| 4 | Chap57/DijkstraStEphU64.rs | (paired with F64) | `27a6127d9` | reverted in `c22c64b44` |
| 5 | Chap59/JohnsonMtEphF64.rs | 7 tests (test_mt_*) | `27a6127d9` | reverted in `c22c64b44` |
| 6 | Chap59/JohnsonStEphF64.rs | 5 tests (all Johnson St F64) | `27a6127d9` | reverted in `c22c64b44` |
| 7 | Chap59/JohnsonMtEphI64.rs | (paired with F64) | `27a6127d9` | reverted in `c22c64b44` |
| 8 | Chap59/JohnsonStEphI64.rs | (paired with F64) | `27a6127d9` | reverted in `c22c64b44` |

Total: 30 RTT failures across 8 files, 4 chapters.

### Detailed example: BSTRBMtEph rotate_left

```rust
// BEFORE (correct, commit a940468df):
let ghost old_x_right = x.right;
proof {
    reveal_with_fuel(spec_is_bst_link, 2);
    reveal_with_fuel(link_contains, 2);
}

// AFTER (corrupted by marker insertion):
// Veracity: NEEDED proof block
let ghost old_x_right = x.right;
proof {
    reveal_with_fuel(spec_is_bst_link, 2);
    reveal_with_fuel(link_contains, 2);
}
```

The marker displaced the `let ghost` binding. Similar shifts occurred in
`rotate_right` and at closing braces of proof blocks, moving `}` markers
relative to exec statements.

### Root Cause

The marker insertion algorithm places `// Veracity: NEEDED proof block` at the
first line of the detected proof block. But "first line" is determined by the
same heuristic that detects proof blocks — which doesn't understand that a
`let ghost` binding preceding a `proof {}` block is not part of the proof block,
even though it's logically associated with it.

### Impact

- Verus verification passes (ghost code is erased, exec shifts are invisible to Z3)
- Only RTTs catch the corruption, and only if the specific runtime path is tested
- Silent corruption if RTTs don't cover the affected code path

### Fix

Marker insertion must be line-preserving: insert the marker comment on its own new
line before the proof block, never displacing existing lines. Alternatively, use
end-of-previous-line comments instead of beginning-of-block comments.

---

## Bug 3: #[cfg(verus_keep_ghost)] treated as proof block

### Description

The `#[cfg(verus_keep_ghost)]` attribute on `PartialEqSpecImpl` impls is a compile-time
gate that hides the impl from the non-Verus compiler (cargo test). Verus defines the
`PartialEqSpecImpl` trait; cargo does not. The cfg gate is structural, not a proof block.

Commenting it out with `// Veracity: UNNEEDED proof block` exposes the impl to cargo,
which fails with `cannot find trait PartialEqSpecImpl in this scope`.

### Files affected and fixed

| # | File | Line | Commit fixed |
|---|------|------|-------------|
| 1 | Chap18/ArraySeqStPer.rs | 1024 | `a940468df` |
| 2 | Chap37/AVLTreeSeq.rs | 1339 | `27a6127d9` |

### Root Cause

Same as Bug 1: the proof block detector matches any line that looks like an
indented statement. `#[cfg(verus_keep_ghost)]` is indented and precedes an impl
block, so it looks like a proof block boundary.

### Fix

The detector should never mark `#[cfg(...)]` attributes as proof blocks. Attributes
are syntactic metadata, not proof content.

---

## Bug 4: Removed asserts cause conjunction flakiness

### Description

Not a veracity bug per se, but a consequence of the minimize approach: some asserts
that are individually UNNEEDED (the proof passes without them) are Z3 speed hints
that stabilize later conjunction proofs. Removing them causes "all sub-assertions
succeeded but conjunction failed" errors.

### Files reverted

| # | File | Error | Commit reverted |
|---|------|-------|----------------|
| 1 | Chap36/QuickSortStEph.rs (full chapter) | lemma_partition_sort_concat precondition | `64c3ac385` |

The specific assert removed was at line 503:
```rust
proof { assert(T::le(elem, pivot)); assert(elem != pivot); }
```
This was marked UNNEEDED because the function verifies without it. But it's a
Z3 hint that stabilizes the conjunction in `lemma_partition_sort_concat`'s
precondition 40 lines later.

### Root Cause

The 0.00 threshold correctly identifies that removing this assert doesn't increase
CPU or memory for the current function. But the assert's value is as a trigger hint
for a *later* proof obligation in the same function. The minimize tool tests each
assert in isolation; it doesn't detect cross-assert dependencies within a function.

### Mitigation

This is harder to fix automatically. Possible approaches:
- After all individual removals, re-verify the full function and restore if it fails
- Track "conjunction stability" as a separate metric
- Flag functions with known conjunction flakiness (Chap35, Chap36) as no-minimize

---

## Summary

| Bug | Type | Files affected | Detection | Severity |
|-----|------|---------------|-----------|----------|
| 1 | Ensures gutting | 5+ files | PTT, manual audit | High — silent spec weakening |
| 2 | Marker shift | 8 files | RTT (30 failures) | Critical — runtime corruption |
| 3 | cfg gate removal | 2 files | RTT (compile error) | Medium — caught at compile |
| 4 | Hint removal | 1 chapter | Validate (conjunction flakiness) | Medium — caught at verify |
