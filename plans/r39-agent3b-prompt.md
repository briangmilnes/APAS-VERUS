# R39b Agent 3: AVLTreeSetStEph Len Bounds + DijkstraStEphU64 Proofs

## Baseline
- Main at agent3/ready merge point, Verus release/0.2026.03.17.a96bad0, Rust 1.94.0

## MANDATORY RULES — READ BEFORE WRITING ANY CODE

**DO NOT USE `accept()` ANYWHERE. NOT ONCE. NOT EVER. NOT FOR ANY REASON.**
**DO NOT weaken ensures.** Do not delete postconditions to make proofs easier.
**DO NOT add `requires true` or `// veracity: no_requires`.**
**DO NOT convert assume() to accept().**

Read CLAUDE.md before starting.

## Assignment

Two files, four holes, one warning. All algorithmic proof work.

### File 1: `src/Chap41/AVLTreeSetStEph.rs` — 2 holes

**Hole 1** (line 1085): `assume(new_vec@.len() < usize::MAX)` in `insert`
**Hole 2** (line 1352): `assume(new_vec@.len() < usize::MAX)` in `insert_sorted`

Both holes follow the same pattern. The code:
1. Calls `lemma_wf_implies_len_bound::<T>(&self.elements.root)` (line 1083/1350)
2. Asserts `self.elements@.len() < usize::MAX` (line 1084/1351)
3. Builds `new_vec` by manipulating elements (insert into sorted position or append)
4. Assumes `new_vec@.len() < usize::MAX` (THE HOLE)
5. Calls `AVLTreeSeqStEphS::from_vec(new_vec)` (line 1087/1354)

**The proof**: `new_vec` has at most `self.elements@.len() + 1` elements (we add one
element during insert). The comment at line 1077-1082 says the tree wf bound gives
`total_size < usize::MAX` but doesn't leave room for +1. However, read the ACTUAL
bound from `lemma_wf_implies_len_bound` — it may give `len < usize::MAX` which means
`len + 1 <= usize::MAX`, which means `new_vec@.len() <= self.elements@.len() + 1 <=
usize::MAX`. Check whether `from_vec` requires `< usize::MAX` or `<= usize::MAX`.

**Strategy**:
1. Read `lemma_wf_implies_len_bound` to understand the exact bound it provides
2. Read `AVLTreeSeqStEphS::from_vec` requires to see what it needs
3. Track `new_vec@.len()` through the construction: how many elements go in?
4. If the bound is off-by-one (tree needs `< usize::MAX` but we need `<= usize::MAX`),
   you may need to strengthen the wf or add `self.elements@.len() + 1 < usize::MAX`
   to insert's requires. Check what the current `insert` requires clause says.

### File 2: `src/Chap57/DijkstraStEphU64.rs` — 2 holes + 1 warning

**Hole 3** (line 212): `assume(BinaryHeapPQ::<PQEntry>::spec_is_exec_heap(pq.spec_seq()))`

This assume says the PQ maintains its heap invariant after `delete_min`. The proof should
follow from BinaryHeapPQ's postconditions. Read:
- `src/Chap45/BinaryHeapPQ.rs` — check `delete_min` ensures clause
- Does `delete_min` ensure `spec_is_exec_heap` on the returned PQ?
- If yes, the assume is provable from the ensures. If the ensures is too weak, this
  hole may require strengthening BinaryHeapPQ's ensures first (report if so).

**Hole 4** (line 253): `proof { assume(remaining_budget > 0); }`

This is a ghost budget tracking proof. The loop invariant (line 206) says:
`pq@.len() + remaining_budget <= m as int + 1`

And the inner loop invariant (line 241) says:
`pq@.len() + remaining_budget <= m as int`

After `pq.insert(...)`, `remaining_budget` decreases by 1. The assume says the budget
hasn't been exhausted. The proof needs: total PQ inserts across the entire algorithm
is bounded by |E| (each edge relaxation inserts at most once). This is a counting
argument — each edge is visited at most once in the neighbor iteration.

**Strategy**: The invariant already tracks `pq@.len() + remaining_budget <= m`. After
an insert, `pq@.len()` increases by 1 and `remaining_budget` decreases by 1, so the
invariant is maintained. But we need `remaining_budget > 0` BEFORE the insert. From the
inner invariant: `pq@.len() + remaining_budget <= m` and `pq@.len() >= 0`, so
`remaining_budget <= m`. But we need `remaining_budget > 0`, which means we need to
show we haven't exhausted the budget. The insert only happens if `new_dist < u_dist`
(line 250), so not every neighbor triggers an insert. The key: each insert consumes
one unit of budget, and total budget starts at m (= |E|). Each insert corresponds to
one edge relaxation. The total number of edge relaxations across all Dijkstra iterations
is at most |E|. So remaining_budget > 0 when we reach the insert.

Look for the initial budget value (where `remaining_budget` is first assigned) and trace
the invariant. The proof may need an additional assertion connecting the iteration
count to the edge count.

**Warning** (line 104): `fn_missing_requires` on `pq_entry_new`. This is a constructor
function — it may genuinely have no preconditions. If so, either the function needs no
requires (leave the warning) or you can add a real precondition if one applies (e.g.,
`vertex < some_bound`). Read the callers to determine if there's a natural precondition.

### Priority

1. Start with the AVLTreeSetStEph holes — they're the same pattern, fix one fixes both
2. Then DijkstraStEphU64 hole at line 212 (PQ heap invariant)
3. Then DijkstraStEphU64 hole at line 253 (budget tracking)
4. Fix the warning if time permits

## Validation

Run `scripts/validate.sh` after changes. Must be 0 errors.
Run `scripts/rtt.sh` — runtime tests must pass.
Write your report to `plans/agent3-r39b-report.md`.
