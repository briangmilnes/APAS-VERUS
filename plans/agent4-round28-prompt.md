# Agent 4 — R28: Chap41 fn_missing_requires + Chap45 holes

## State

Main at latest commit. 4114 verified, 0 errors. You are Agent 4.

## Assignment

Work in Chap41 and Chap45 only.

### Task 1: Chap41 — Fix 3 fn_missing_requires_ensures

| File | Line | Function | What to add |
|------|------|----------|-------------|
| AVLTreeSetMtEph.rs | 312 | parallel_filter | Needs `requires self.spec_avltreesetmteph_wf()` + closure specs. Needs `ensures` matching the sequential filter contract. |
| AVLTreeSetMtEph.rs | 371 | parallel_intersect | Needs `requires self.spec_avltreesetmteph_wf() && other.spec_avltreesetmteph_wf()`. Needs `ensures` for intersection semantics. |
| AVLTreeSetMtPer.rs | 231 | parallel_sort | Needs `requires self.spec_avltreesetmtper_wf()`. Needs `ensures` that result is sorted + same multiset. |

For each: read the function body AND read the StEph counterpart to see what specs it has.
Copy the spec pattern from StEph into MtEph/MtPer. **Do NOT import from StEph** — duplicate
the spec. Mt standalone rule.

### Task 2: Chap41 — Prove AVLTreeSetStEph insert assume (line 958)

```rust
proof { assume(new_vec@.len() < usize::MAX); }
```

This assume says the new vector length fits in usize after an insert. The fix:
- Add `requires self@.len() < usize::MAX - 1` to the insert trait method.
- This propagates to callers — find all callers of insert and add the bound to their
  requires too. The callers are in:
  - AVLTreeSetStEph.rs itself (from_iter, etc.)
  - Chap41 test files
  - Possibly Chap43/Chap52/Chap53 (but you should NOT touch those — only Chap41).

  If callers outside Chap41 break, leave them broken and document in your report.
  The orchestrator will fix cross-chapter cascades.

### Task 3: Chap45 — Fix 2 fn_missing_requires

| File | Line | Function | What to add |
|------|------|----------|-------------|
| BinaryHeapPQ.rs | 284 | parent | `requires i > 0` (parent of index i is (i-1)/2, needs i > 0) |
| LeftistHeapPQ.rs | 84 | total_order_le | Read the function — likely needs the arguments to be valid/ordered |

### Task 4: Chap45 — Prove BinaryHeapPQ extract_all sortedness (line 1031)

```rust
proof { assume(Self::spec_sorted(result.seq@)); }
```

This assume says extract_all returns a sorted sequence. The proof:
- extract_all repeatedly calls extract_min, which returns the minimum element.
- Each extracted element is >= the previous one (heap property maintained).
- Build the sortedness proof by induction on the extraction loop.
- You may need a loop invariant that tracks: all previously extracted elements are
  <= everything remaining in the heap.

Skip Example files (Example41_3.rs, Example45_2.rs).

## Rules

- Do NOT touch files outside Chap41 and Chap45.
- Do NOT add `requires true`.
- Do NOT add `assume` or `accept` to fix proofs.
- Run `scripts/validate.sh` after changes. 0 errors required.

## Deliverable

- `scripts/validate.sh` passes with 0 errors.
- Write report to `plans/agent4-round28-report.md`.
- `git add -A && git commit` with descriptive message.
- `git push origin agent4/ready`.
