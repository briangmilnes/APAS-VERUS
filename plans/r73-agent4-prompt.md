# R73 Agent 4 — Fix Chap64, Chap65, Chap66

## Objective

Fix 8 broken files across 3 graph algorithm chapters so they compile,
verify, and pass RTT. These were hidden behind `all_chapters` and never
cargo-compiled. They are now commented out in lib.rs and need to be
uncommented once fixed.

## Assigned files

### Chap64 — Spanning Trees & TSP (709 lines total)

| # | File | Lines | Description |
|---|------|-------|-------------|
| 1 | src/Chap64/SpanTreeStEph.rs | 185 | Spanning tree (sequential) |
| 2 | src/Chap64/SpanTreeMtEph.rs | 192 | Spanning tree (parallel) |
| 3 | src/Chap64/TSPApproxStEph.rs | 332 | TSP approximation |

### Chap65 — Minimum Spanning Trees (1049 lines total)

| # | File | Lines | Description |
|---|------|-------|-------------|
| 4 | src/Chap65/UnionFindStEph.rs | 364 | Union-Find data structure |
| 5 | src/Chap65/KruskalStEph.rs | 329 | Kruskal's algorithm |
| 6 | src/Chap65/PrimStEph.rs | 356 | Prim's algorithm |

### Chap66 — Boruvka's Algorithm (1537 lines total)

| # | File | Lines | Description |
|---|------|-------|-------------|
| 7 | src/Chap66/BoruvkaStEph.rs | 538 | Boruvka's MST (sequential) |
| 8 | src/Chap66/BoruvkaMtEph.rs | 999 | Boruvka's MST (parallel) |

## Approach

These are graph algorithm files that were written but never compiled under
cargo or verified under Verus. They likely have import errors, type
mismatches, and missing specs.

### Read these first:

1. `CLAUDE.md` — project rules
2. `src/standards/mod_standard.rs` — module structure
3. `src/standards/toplevel_coarse_rwlocks_for_mt_modules.rs` — for Mt files
4. The graph infrastructure they depend on:
   - `src/Chap52/AdjTableGraphStEph.rs` or `AdjSeqGraphStEph.rs` — graph types
   - `src/Chap53/GraphSearchStEph.rs` — graph search (BFS/DFS)
   - `src/Chap53/PQMinStEph.rs` — priority queue (for Prim)

### For each file:

1. Read the file. Identify compilation errors (stale imports, type mismatches,
   missing traits).
2. Fix imports to match current module structure.
3. Ensure specs are present and match textbook semantics.
4. Uncomment the chapter/module in src/lib.rs.
5. Run `scripts/validate.sh` to verify.

### Dependency order

Fix in this order — later files may depend on earlier ones:
1. UnionFindStEph (standalone data structure)
2. SpanTreeStEph, SpanTreeMtEph (may depend on graph types)
3. KruskalStEph (depends on UnionFind + graph)
4. PrimStEph (depends on PQMin + graph)
5. TSPApproxStEph (depends on spanning tree)
6. BoruvkaStEph, BoruvkaMtEph (depends on UnionFind + graph)

### Uncommenting in lib.rs

The chapters are currently commented out:
```rust
// pub mod Chap64 — broken: never cargo-compiled
// pub mod Chap65 — broken: never cargo-compiled
// pub mod Chap66 — broken: never cargo-compiled
```

Uncomment each chapter only after ALL its files compile and verify. Use the
standard module block format:
```rust
#[cfg(not(any(feature = "experiments_only", feature = "dev_only", feature = "wf")))]
pub mod Chap64 {
    pub mod SpanTreeStEph;
    ...
}
```

## Validation

1. `scripts/validate.sh` — must show 0 errors
2. `scripts/rtt.sh` — check if RTT files exist in tests/Chap64-66/
3. Fix all warnings (triggers, missing requires, etc.)

## Rules

- Read `CLAUDE.md` on startup.
- Do NOT weaken specs. If the textbook specifies a postcondition, prove it.
- Do NOT add assume or accept on algorithmic logic.
- Do NOT sequentialize Mt files. The Mt versions must remain parallel.
- Uncomment modules in lib.rs ONLY after they compile and verify.
- If a file is deeply broken (not just stale imports but structurally wrong),
  report what's wrong and what it needs. Don't spend hours on one file.
- Report: files fixed, verification count, any remaining issues.

## Also: rank_key_iter and select_key proofs

If you finish Chap64-66 with time remaining, read
`plans/r73-rank-key-iter-prompt.md` and `plans/r73-select-key-prompt.md`.
These are the last 2 real algorithmic external_body holes in the project,
both in `src/Chap43/OrderedTableStEph.rs`. Prove them if you can.
