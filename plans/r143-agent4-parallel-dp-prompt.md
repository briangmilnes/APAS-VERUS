# R143 Agent 4 — Parallel DP for OptBinSearchTree (Chap50). AFK.

## Setup

Read ALL files in `src/standards/` before starting.
Read `src/Chap50/OptBinSearchTreeMtEph.rs` — the sequential DP implementation.
Read `src/Chap50/OptBinSearchTreeMtPer.rs` — same algorithm, persistent version.
Read `prompts/Chap50.txt` — APAS description of optimal BST.

Report file: `plans/r143-agent4-parallel-dp-report.md`

## Problem

2 DIFFERS:
```
OptBinSearchTreeMtEph.rs: obst_rec — sequential DP table fill, APAS Span O(n lg n) assumes parallel
OptBinSearchTreeMtPer.rs: obst_rec — same
```

APAS says Work O(n²) (or O(n³) for the naive version), Span O(n lg n).
Our implementation fills the DP table sequentially: Span = Work.

## The parallel DP pattern

The DP table for optimal BST has entry `cost[i][j]` = optimal cost for keys
i..j. Dependencies: `cost[i][j]` depends on `cost[i][k-1]` and `cost[k][j]`
for all k in i..j.

**Diagonal wavefront**: entries on the same diagonal (j - i = constant) are
independent. Fill diagonal d=0, then d=1, then d=2, ..., up to d=n-1.
Each diagonal has O(n) independent entries. Within each entry, finding the
optimal k requires O(d) work.

- Diagonals: n
- Entries per diagonal: O(n)
- Work per entry: O(d) where d = diagonal number
- Total work: O(Σ n*d) = O(n³) naive, O(n²) with Knuth's optimization
- Span: O(n) diagonals × O(lg n) per diagonal (parallel reduce over k) = O(n lg n)

## Investigation

1. Read the current `obst_rec` implementation. Understand the DP recurrence.

2. Determine if Knuth's optimization (monotonicity of optimal root) is used.
   If so, the inner loop per entry is O(1) amortized, and the parallel version
   needs careful handling.

3. If the naive O(n³) version: parallelize each diagonal. For diagonal d,
   use `tabulate(|i| compute_entry(i, i+d), n-d)` to fill all entries in
   parallel. The inner minimize-over-k step uses `reduce` with a min function.

4. If Knuth's optimized version: the inner loop boundaries depend on
   neighboring entries, which creates dependencies within a diagonal.
   Document this constraint.

## Implementation sketch (naive version)

```
for d in 0..n {
    // All entries on diagonal d are independent
    let entries = tabulate(|i| {
        // Find optimal k for cost[i][i+d]
        let best_k = reduce over k in i..i+d+1 {
            cost[i][k-1] + cost[k+1][i+d] + weight[i][i+d]
        };
        best_k
    }, n - d);
    // Write entries back to table
}
```

The outer loop over diagonals is sequential (O(n) rounds). Each round's
tabulate is parallel (O(n) entries, each O(d) work → O(lg n) span with
reduce). Total span: O(n lg n).

## Both files

Make the same fix in both MtEph and MtPer per the standalone rule.

## Validation

Run `scripts/validate.sh isolate Chap50`. Then `scripts/rtt.sh`.

## Rules

- Do NOT add assumes, accepts, or external_body.
- The DP table access pattern must respect dependencies (diagonal ordering).
- If Knuth's optimization prevents parallelization within a diagonal,
  document this clearly. The outer diagonal loop can still be sequential
  with parallel entry computation.

## When done

RCP.
