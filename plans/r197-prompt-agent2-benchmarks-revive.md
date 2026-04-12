# R197 Prompt — Agent 2: Revive benchmarks; coverage analysis; add new benches. AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent2`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER add `assume`, `admit`, `accept`, or `external_body`.**
6. **NEVER modify `src/` to make benchmarks compile.** If a bench
   doesn't compile because the API changed, fix the bench, not the
   src. If the src is genuinely broken, STOP and report.
7. **NEVER touch `Example*.rs` or `Problem*.rs` files.**

## Read all standards first.

## Context

`benches/` currently contains exactly one benchmark file:
`benches/Chap03/BenchInsertionSortStEph.rs`. There is one `[[bench]]`
entry in `Cargo.toml`. This is criterion-based but has been ignored
for many rounds while proof work consumed all attention.

Meanwhile `src/` has grown to ~70 algorithm files across 44 chapters,
including parallel (Mt) variants. None of them have benchmarks.

## Goal

Three-part deliverable:

1. **Revive** the existing `BenchInsertionSortStEph` so it compiles
   and runs end-to-end against current `src/` (the API may have shifted
   since the bench was last touched).
2. **Inventory** all algorithm files under `src/Chap*/` and produce
   a coverage table: which have benches, which don't, which warrant one.
3. **Add new benchmarks** for the **top 10 highest-priority gaps**
   identified in the inventory. Hard rule: **each individual
   benchmark must run in under 1 second wall time**, not per-iteration —
   per benchmark group as a whole. Use small input sizes and a small
   `sample_size`.

## Plan

### Step 1: Get the existing bench running

```bash
cargo bench --bench BenchInsertionSortStEph 2>&1 | tail -40
```

If it fails to compile: fix the bench imports / API calls to match
the current `InsertionSortStEph` trait. The fix should be in the bench
file only, not in `src/`.

If it runs: capture wall time and a representative throughput number
in the report. Note current input sizes (32, 64, 128).

If runtime exceeds 1s for the whole bench group, drop the largest
input until it fits.

### Step 2: Bench coverage inventory

Build `plans/r197-bench-coverage-inventory.md` with one row per
candidate algorithm file:

| # | Chap | File | Currently benched? | Asymptotic class | Bench priority |
|---|------|------|--------------------|------------------|----------------|
| 1 | 03   | InsertionSortStEph.rs | YES | O(n²) sort | (existing) |
| 2 | 03   | QuickSortStEph.rs     | NO  | O(n lg n) sort | high |
| 3 | 65   | UnionFindPCStEph.rs   | NO  | ~O(α(n)) per op | high |
| 4 | 65   | KruskalStEph.rs       | NO  | O(m lg m) MST | high |

Definitions:
- "Currently benched" — yes if `benches/ChapNN/Bench<File>.rs` exists.
- "Asymptotic class" — from APAS textbook prose (Algorithm header
  comments often state cost). Skip if unclear.
- "Bench priority" — high / medium / low / skip based on:
  - **high**: Algorithm is referenced from later chapters (e.g.,
    UnionFind used by Kruskal), or has interesting Mt vs St
    comparison potential.
  - **medium**: Standalone algorithm with clear cost spec.
  - **low**: Simple wrapper, mostly delegation.
  - **skip**: Demo/utility, trivial cost.

Skip `Example*.rs`, `Problem*.rs`, and any file whose chapter is
commented out in `lib.rs`.

### Step 3: Add benchmarks for top 10 high-priority gaps

For each chosen file, create `benches/ChapNN/Bench<FileBaseName>.rs`
modeled on the existing `BenchInsertionSortStEph.rs`. Patterns:

```rust
fn bench_my_op(c: &mut Criterion) {
    let mut group = c.benchmark_group("MyOp");
    group.sample_size(20);                          // small
    group.warm_up_time(Duration::from_millis(200)); // short
    group.measurement_time(Duration::from_millis(500));

    for &n in &[/* small sizes that keep total <1s */] {
        group.bench_with_input(BenchmarkId::new("variant", n), &n, |b, &n| {
            b.iter_batched(
                || setup(n),
                |input| { use_input(input); },
                BatchSize::SmallInput,
            );
        });
    }
    group.finish();
}
```

Hard budget per `[[bench]]` entry: **< 1 second wall time** when run
as `cargo bench --bench BenchX`. Tune input sizes to fit.

For Mt benchmarks (`*MtEph.rs`), include both:
- A single-thread baseline run.
- A parallel run with `HFScheduler` or `join` — whichever the API uses.
This makes the Mt-vs-St comparison legible in the lecture.

Register every new bench in `Cargo.toml`:

```toml
[[bench]]
name = "BenchQuickSortStEph"
path = "benches/Chap03/BenchQuickSortStEph.rs"
harness = false
```

### Step 4: Run all benches, capture timings

```bash
cargo bench 2>&1 | tee logs/bench-all-$(date +%Y%m%d-%H%M%S).log
```

Confirm every bench finishes in < 1s. Capture a summary table:

| # | Chap | Bench | n | Median time | Throughput |
|---|------|-------|---|-------------|------------|

### Step 5: Out-of-scope notes

Capture in `plans/r197-bench-coverage-gaps-remaining.md` the
medium/low priority benches that this round did NOT add, so future
rounds have a punch list.

## Validation

```bash
cargo bench 2>&1 | tail -50
scripts/validate.sh    # confirm we didn't break anything
scripts/rtt.sh         # confirm tests still pass
```

The validate and rtt are sanity checks — adding benchmarks should not
touch verified code. If either regresses, STOP.

## Rules

- **Never increase a bench's input size to "make the numbers more
  interesting"** if it pushes runtime over 1s. The 1s budget is hard.
- **Never call `external_body` functions in a bench expecting them to
  do real work** — they may be no-ops. Read the implementation first.
- **Never benchmark a `*StPer.rs` (persistent) variant** by mutating it —
  persistent structures return new instances; benchmark return-value
  reuse correctly.
- **Mt benches must use the standard threading patterns** —
  `HFScheduler` for fork-join. Do not invent new threading idioms.

## Report

Write `plans/agent2-round197-report.md` with:

- Whether `BenchInsertionSortStEph` compiled clean or required fixes
  (and what fixes).
- Bench coverage inventory summary (chap-level: # files with /
  without bench, by priority).
- New benches added (per file, input sizes, median time).
- Total bench wall time (all benches together).
- Any algorithm files that **could not** be benchmarked productively
  in < 1s (note why — too slow even at small n, requires complex
  setup, etc.).
- Any RTT / validate regressions (should be zero).

## RCP

`git add -A && git commit -m "R197 Agent 2: revive benchmarks; coverage; +N new benches"`,
then `git push`.
