# R37 Agent 4: Chap47 Hash Tables + Chap57 Dijkstra

## Goal

Prove hash table insert/delete operations across 3 open-addressing files,
plus structural holes in ParaHashTable and StructChainedHashTable. Then
tackle 2 Dijkstra assumes.

## Context

Chap47 has 3 open-addressing hash table implementations (LinProb,
DoubleHash, QuadProb). In each, `lookup` is already fully verified with
loop invariants and modular arithmetic proofs. `insert` and `delete` use
the same probe sequence but are external_body. The lookup proofs are your
template.

## Tier 1: Open-Addressing Insert/Delete (6 holes)

### LinProbFlatHashTableStEph.rs (2 external_body)

| # | Line | Function | Notes |
|---|------|----------|-------|
| 1 | 94 | insert | Linear probe: (h + attempt) % m |
| 2 | 230 | delete | Linear probe: same sequence, tombstone |

**Pattern**: `lookup` (lines ~125-226) is fully proved. It uses the same
probe sequence `(h + attempt) % m`. Study the loop invariants in lookup
and adapt them for insert (which also writes to the table) and delete
(which tombstones).

Key differences from lookup:
- insert: must prove slot is writable (Empty or Tombstone), then
  `table.table.set(idx, Occupied(entry))` preserves wf
- delete: must prove slot transitions from Occupied to Tombstone
  preserves wf

### DoubleHashFlatHashTableStEph.rs (3 holes)

| # | Line | Function | Notes |
|---|------|----------|-------|
| 1 | 91 | insert | Double probe: (h + attempt × step) % m |
| 2 | 129 | lookup | Has 1 assume bridging second_hash step to wf |
| 3 | 269 | delete | Double probe: same sequence, tombstone |

The lookup assume (line 129) bridges the fixed `step` value from
`second_hash` to the well-formedness invariant. Try to prove this from
the second_hash specification.

### QuadProbFlatHashTableStEph.rs (2 external_body)

| # | Line | Function | Notes |
|---|------|----------|-------|
| 1 | 76 | insert | Quadratic probe: (h + attempt²) % m |
| 2 | 264 | delete | Quadratic probe, tombstone |

Same approach: lookup is proved, adapt its invariants for insert/delete.

### Approach for all probe-based functions

1. Read the proved `lookup` in the same file — understand loop invariants.
2. Remove `external_body` from insert/delete.
3. Add loop invariants matching lookup's pattern:
   - Probe index stays in bounds: `0 <= idx < m`
   - Probe sequence is correct: `idx == (h + f(attempt)) % m`
   - Table wf preserved through mutations
   - Entry appears in table view iff occupied
4. After the loop, connect the final state to the ensures.
5. For insert: the ensures says `self@.contains_key(k) && self@[k] == v@`.
6. For delete: the ensures says `!self@.contains_key(k)`.

## Tier 2: Structural Holes (3 holes)

### StructChainedHashTable.rs (1 external_body)

| # | Line | Function | Notes |
|---|------|----------|-------|
| 1 | 399 | resize | Collects all pairs, rebuilds in new table |

**resize**: Iterates all chains, collects pairs, creates new table with
doubled capacity, reinserts all. The proof needs: all entries preserved,
new capacity is correct, wf maintained. May need intermediate lemma about
collect preserving the multiset.

### ParaHashTableStEph.rs (2 external_body)

| # | Line | Function | Notes |
|---|------|----------|-------|
| 1 | 463 | call_hash_fn | Calls user-provided Fn hash function |
| 2 | 493 | compute_second_hash | Computes SipHash for double hashing |

These are likely structural — `call_hash_fn` wraps an opaque Fn closure,
and `compute_second_hash` uses `std::hash::Hasher`. If Verus can't reason
about these external types, leave as external_body and report.

## Tier 3: Chap57 Dijkstra (2 assumes)

### DijkstraStEphU64.rs

| # | Line | Function | Notes |
|---|------|----------|-------|
| 1 | 202 | assume | PQ maintains heap property after delete_min |
| 2 | 243 | assume | Remaining budget > 0 (total inserts ≤ |E|) |

**Heap property assume (202)**: `assume(BinaryHeapPQ::spec_is_exec_heap(pq.spec_seq()))`.
Check if BinaryHeapPQ's `delete_min` has an ensures about heap property.
If so, assert it instead of assuming.

**Budget assume (243)**: `assume(remaining_budget > 0)`. This bounds the
number of PQ inserts. Each edge is processed at most once, so total inserts
≤ |E|. Prove from the loop invariant tracking processed edges.

**Expected: -5 to -9 holes total.**

## Rules

- assume() only. NEVER accept().
- Do NOT modify CLAUDE.md.
- Do NOT modify ~/projects/veracity/.
- Do NOT touch any Chap43 files (assigned to Agents 1-3).
- Read the proved `lookup` functions FIRST — they are your proof templates.
- Run `scripts/validate.sh` after changes. 0 errors required.
- Write report to `plans/agent4-round37-report.md`.
- Commit, push to `agent4/ready`.
