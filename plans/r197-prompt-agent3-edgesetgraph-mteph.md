# R197 Prompt — Agent 3: Implement `EdgeSetGraphMtEph` (+ tests + bench). AFK.

## Agent worktree

`/home/milnes/projects/APAS-VERUS-agent3`

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER modify `~/projects/veracity/`.** Not a single file.
3. **NEVER run `rm -rf` on any directory.**
4. **NEVER run `git clean`.**
5. **NEVER add `assume`, `admit`, `accept`, or `external_body` except
   exactly where the reference files (`EdgeSetGraphMtPer.rs`,
   `EdgeSetGraphStEph.rs`) already use them — and only for the same
   reasons (thread-spawn boundaries, eq/clone bridges). No new
   algorithmic holes.**
6. **NEVER touch `Example*.rs` or `Problem*.rs`.**
7. **NEVER modify `src/Chap52/EdgeSetGraphStEph.rs`, `src/Chap52/EdgeSetGraphMtPer.rs`,
   or any other existing Chap52 file.** They are references only.

## Read all standards first.

Pay extra attention to:
- `standards/multi_struct_standard.rs`
- `standards/toplevel_coarse_rwlocks_for_mt_modules.rs`
- `standards/mt_type_bounds_standard.rs`
- `standards/partial_eq_eq_clone_standard.rs`
- `standards/using_closures_standard.rs`
- `standards/iterators_standard.rs`

Also read:
- `docs/VerusOptimizationsApplied.md` — project-wide Verus optimization
  patterns (opaque vs closed, conjunction flakiness, broadcast cross-fire,
  eq/clone bridge).

## Context

Chap52 has three EdgeSet graph variants — `StEph`, `StPer`, `MtPer` —
but `EdgeSetGraphMtEph` is missing. It's a consistency gap, not an
algorithmic one: `AdjMatrixGraphMtEph` and `AdjSeqGraphMtEph` exist;
the MtEph row of the Chap52 representation matrix is incomplete.

A placeholder `tests/Chap52/TestEdgeSetGraphMtEph.rs` exists (5 lines)
noting "Source implementation src/Chap52/EdgeSetGraphMtEph.rs does not
exist yet."

## Goal

Create `src/Chap52/EdgeSetGraphMtEph.rs` — a mutable, thread-safe
edge-set-representation of a graph. Deliverables:

1. The source file, verifying clean (0 errors, 0 new `external_body` /
   `assume` / `accept` beyond what the references use).
2. `tests/Chap52/TestEdgeSetGraphMtEph.rs` — full runtime test suite
   (replace the 5-line placeholder).
3. `benches/Chap52/BenchEdgeSetGraphMtEph.rs` — criterion benchmarks
   with hard <1 second wall-time per group.
4. Register all three in `Cargo.toml` and `src/lib.rs` as appropriate.

## Plan

### Step 1: Read the references thoroughly

Read these three files end-to-end before writing anything:

- `src/Chap52/EdgeSetGraphStEph.rs` — sequential mutable edge set;
  gives you the trait shape, spec fns, helper lemmas.
- `src/Chap52/EdgeSetGraphMtPer.rs` — multi-threaded persistent
  edge set; gives you the Arc/RwLock + `MtEph`/`MtT` bounds pattern.
- `src/Chap52/AdjSeqGraphMtEph.rs` **or** `src/Chap52/AdjMatrixGraphMtEph.rs`
  — the **MtEph** sibling patterns in the same chapter. Pick whichever is
  closer in shape (MtEph uses per-instance RwLock; the mut variant
  stores mutable inner state behind a lock).

The MtEph signature is: **mutable, thread-safe**. Writes go through a
coarse RwLock (per `standards/toplevel_coarse_rwlocks_for_mt_modules.rs`);
reads can run concurrently. The view returns the post-lock state.

### Step 2: Skeleton

Follow the Table-of-Contents order (per `standards/table_of_contents_standard.rs`):

```
//  Table of Contents
//  1. module
//  2. imports
//  3. broadcast use
//  4. type definitions
//  5. view impls
//  6. spec fns
//  7. proof fns/broadcast groups
//  8. traits
//  9. impls
//  10. iterators   (if applicable)
//  11. top level coarse locking
//  12. derive impls in verus!
//  13. macros      (if applicable)
//  14. derive impls outside verus!
```

The RwLock invariant struct is named `EdgeSetGraphMtEphInv` (per
`standards/rwlock-predicate-naming.mdc`; `XInv` for module `X`). Its
`inv` must carry a real invariant — never just `true`.

### Step 3: Trait surface

The trait must include all public operations from `EdgeSetGraphStEph`'s
trait, adjusted for `&self` (concurrent reads) and `&self` with
interior mutability via the lock (for writes). See
`standards/mut_standard.rs` for the Mt wrapping pattern.

At minimum (match the StEph trait):
- `new` / constructor
- `has_edge`
- `insert_edge`
- `delete_edge`
- `out_neighbors`
- `num_edges` / `size`
- `contains_vertex`
- Plus any helper methods the StEph trait exposes

### Step 4: Proof strategy

- Reuse the spec functions from `EdgeSetGraphStEph` where possible —
  they describe the abstract state. The Mt variant's View returns the
  same shape.
- Coarse locking means every mutation: acquire write lock, mutate
  inner state, release. The proof reduces to StEph-correctness + lock
  preservation.
- If you hit a matching loop or rlimit, apply the patterns in
  `docs/VerusOptimizationsApplied.md`:
  - `#[verifier::opaque]` on bundled wf predicates.
  - Decompose wf into per-clause lemmas.
  - Wrap broadcast-group-heavy call sites in a nested module.
- Cap `#[verifier::rlimit(N)]` at 200. If a function needs more,
  **STOP and report** — do not push past 200.

### Step 5: Tests (`tests/Chap52/TestEdgeSetGraphMtEph.rs`)

Replace the 5-line placeholder. Cover:
- Empty construction → `num_edges() == 0`.
- Insert edge → `has_edge` returns true after insert.
- Delete edge → `has_edge` returns false after delete.
- Insert duplicate → idempotent (same count).
- `out_neighbors(v)` returns the right set.
- **Concurrent stress test** with `std::thread::spawn` + join:
  N threads each insert M disjoint edges; after join, `num_edges() == N*M`.
  Use a timeout wrapper (required for Mt tests per CLAUDE.md).
- Each test < 200 ms.

Model on `tests/Chap52/TestAdjMatrixGraphMtPer.rs` or
`tests/Chap52/TestEdgeSetGraphMtPer.rs` for concurrent-test patterns.

Register in `Cargo.toml`:

```toml
[[test]]
name = "TestEdgeSetGraphMtEph"
path = "tests/Chap52/TestEdgeSetGraphMtEph.rs"
```

(If already present, confirm; do not duplicate.)

### Step 6: Benchmark (`benches/Chap52/BenchEdgeSetGraphMtEph.rs`)

Model on `benches/Chap03/BenchInsertionSortStEph.rs` + R197 agent2's
new bench files. Bench groups:
- `EdgeSetMtEphInsert` — scale insert over n = [32, 64, 128] edges.
- `EdgeSetMtEphQuery` — `has_edge` on pre-populated graph.
- Optional: concurrent insert via `HFScheduler` (only if the single-group
  wall time still fits under 1s).

Settings (match agent2's pattern):

```rust
group.sample_size(10);
group.warm_up_time(Duration::from_millis(100));
group.measurement_time(Duration::from_millis(300));
```

Hard budget: **< 1 s per bench group**.

Register in `Cargo.toml`:

```toml
[[bench]]
name = "BenchEdgeSetGraphMtEph"
path = "benches/Chap52/BenchEdgeSetGraphMtEph.rs"
harness = false
```

### Step 7: `src/lib.rs`

Add `pub mod EdgeSetGraphMtEph;` under `pub mod Chap52 { ... }`,
bottom-up order — after `EdgeSetGraphMtPer` and before any downstream
consumers.

### Step 8: Validation

Run each step separately (CLAUDE.md rule — no pipelining):

```bash
scripts/validate.sh isolate Chap52     # fastest feedback
scripts/validate.sh                    # full, after isolate is clean
scripts/rtt.sh                         # includes new test
scripts/ptt.sh                         # should be unchanged
cargo bench --bench BenchEdgeSetGraphMtEph    # new bench only
```

All must be clean. Zero new holes in `analyses/chapter-cleanliness-status.log`.
Zero new rlimit overrides above 200.

## Stretch goal (only if primary finishes cleanly with time to spare)

`src/Chap52/AdjTableGraphMtEph.rs` — the **other** missing Chap52 MtEph
file. Same pattern, same deliverables (src + test + bench), modeled on
`AdjTableGraphStEph.rs` + `AdjTableGraphMtPer.rs`.

If you reach for the stretch, label the commit as
`R197 Agent 3: EdgeSetGraphMtEph + AdjTableGraphMtEph + tests + benches`.

## Out of scope

- **Do not** implement `TestAdjMatrixGraphMtEph.rs` or
  `TestAdjSeqGraphMtEph.rs` — those are test-coverage gaps for
  existing src files. Agent1's R197 scope covers RTT coverage.
- **Do not** modify any existing Chap52 src file.
- **Do not** touch chapters other than Chap52.

## Rules

- **Never make the Mt variant a single-thread wrapper** — it must
  actually be thread-safe, backed by a real RwLock. See the reference
  MtEph files in other chapters for the pattern.
- **Never skip the concurrent stress test** — Mt files without one
  are indistinguishable from misconfigured StEph files.
- **Never use raw `std::thread::spawn`** for parallelism inside the
  module. If parallel ops are needed internally (unlikely for this
  file), go through HFScheduler.
- **If you cannot make the Verus proof close** after reading
  `docs/VerusOptimizationsApplied.md` and trying the patterns there,
  **STOP and report** what you tried and where Z3 got stuck. Do
  **not** add `external_body` to algorithmic logic to "close" the proof.

## Report

Write `plans/agent3-round197-report.md` with:

- File added: `src/Chap52/EdgeSetGraphMtEph.rs` — LOC, hole count,
  rlimit overrides used.
- Test: number of `#[test]` functions, wall time, concurrent test pass.
- Bench: group names, input sizes, median timings, total wall time.
- Any Verus optimization patterns applied (opaque, decomposed wf,
  nested wrapper module, etc.) and what problem they solved.
- If stretch goal taken: same data for `AdjTableGraphMtEph`.
- Before/after `analyses/chapter-cleanliness-status.log` — should show
  "+1 file" but still 0 new holes.
- Any new `assume` / `accept` / `external_body` and justification.

## RCP

```
git add -A
git commit -m "R197 Agent 3: EdgeSetGraphMtEph + tests + bench"
git push
```

If the stretch goal completes, amend the commit message accordingly.
