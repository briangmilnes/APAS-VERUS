# Agent 4 — R89 Chap62 Report (StarPartition + StarContraction)

## Objective

Remove `external_body` from 2 functions in Chap62:
1. `route_edges_parallel` in StarContractionMtEph.rs
2. `parallel_star_partition` in StarPartitionMtEph.rs

## Results

| # | Chap | File | Function | Holes Before | Holes After | Status |
|---|------|------|----------|-------------|-------------|--------|
| 1 | 62 | StarContractionMtEph.rs | `route_edges_parallel` | 1 | 0 | Proved |
| 2 | 62 | StarPartitionMtEph.rs | `parallel_star_partition` | 1 | 1 | Blocked |

## 1. route_edges_parallel — PROVED (0 effort)

Removed `#[verifier::external_body]`. The function verified immediately with
zero errors. The proof body (base cases, recursive structure, edge closure
postcondition) was already complete and correct.

The function performs divide-and-conquer edge routing through a partition map
using sequential recursive calls (not ParaPair). Arc cloning for shared data.

## 2. parallel_star_partition — BLOCKED (structural proof issue)

### Problem: value-level vs view-level `no_duplicates`

The proof body has a fundamental design flaw that causes 5 cascading verification
errors across loops 2-6.

**Root cause:** `SetStEph::to_seq()` ensures `seq@.no_duplicates()` at the
VALUE level (`Seq<V>.no_duplicates()` means `i != j => seq[i] != seq[j]`).
But the proof repeatedly tries to derive contradictions from VIEW-level equality
(`vertices_vec@[i]@ == vertices_vec@[j]@`), which is strictly weaker than value
equality. The pattern appears 6+ times:

```rust
if jjv == jv2 {        // jjv = vertices_vec@[jj]@, jv2 = vertices_vec@[j]@
    // VIEW equality: seq[jj]@ == seq[j]@
    assert(vertices_vec@.no_duplicates());
    assert(false);      // ← FAILS: no_duplicates is VALUE-level, not VIEW-level
}
```

Value-level `no_duplicates` says `seq[i] != seq[j]` for `i != j`, but this does
NOT imply `seq[i]@ != seq[j]@` in general. Two distinct V values could map to
the same view.

### What would fix it

The fix requires either:
1. **A lemma connecting view equality to value equality** for types satisfying
   `StT + Hash + Eq`. For hash-based sets, elements with equal views hash
   equally and are considered duplicates, so the set can't contain two distinct
   values with the same view. This needs a bridge lemma:
   `forall i, j: 0 <= i < j < seq.len() ==> seq[i]@ != seq[j]@`
   derived from `to_seq`'s membership postcondition + set uniqueness.

2. **Rewrite all 6 loops** to use view-mapped sequences and view-level
   `no_duplicates` instead of value-level. This is ~200 lines of invariant
   rewriting.

Both approaches are significant proof work (estimated 10+ iterations) beyond
the STEP 20 budget for this round.

## Verification

- **Isolate Chap64**: 1244 verified, 0 errors
- **Full validation**: 5296 verified, 2 errors (pre-existing UnionFindStEph)
- **Net gain**: +12 verifications (from route_edges_parallel proof)
- **StarContractionMtEph**: 0 holes (was 1)
- **StarPartitionMtEph**: 1 hole (unchanged — external_body remains)

## Files Modified

1. `src/Chap62/StarContractionMtEph.rs` — removed `#[verifier::external_body]`
   from `route_edges_parallel`
