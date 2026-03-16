# Agent 3 — R28: Chap47 fn_missing_requires + Lookup Proofs

## State

Main at latest commit. 4114 verified, 0 errors. You are Agent 3.

## Assignment

Two tasks in Chap47 only:

### Task 1: Fix 5 fn_missing_requires (mechanical)

| File | Line | Function | What to add |
|------|------|----------|-------------|
| StructChainedHashTable.rs | 104 | chain_insert | `requires` with hash table wf + valid bucket index |
| StructChainedHashTable.rs | 144 | chain_lookup | Same pattern |
| StructChainedHashTable.rs | 184 | chain_delete | Same pattern |
| VecChainedHashTableStEph.rs | 36 | clone_vec_pairs | `requires` — read the function, understand what it needs |
| LinkedListChainedHashTableStEph.rs | 36 | clone_linked_list_entry | `requires` — read the function, understand what it needs |

For each: read the function body, understand what it needs, add the real `requires`.
**Do NOT add `requires true`.**

### Task 2: Prove QuadProb and DoubleHash lookups (2 external_body)

In R27, LinProb lookup was proved using a real probe-chain invariant with the per-module
wf (`spec_linprobflathashsteph_wf`). QuadProb and DoubleHash lookups were reverted to
external_body because the R26 proofs were vacuous.

Now prove them properly:

- `QuadProbFlatHashTableStEph.rs:106` — lookup external_body.
- `DoubleHashFlatHashTableStEph.rs:121` — lookup external_body.

**Architecture**: Each flat hash table type has its own wf predicate and `spec_impl_wf`
override in the trait. Use these:

- QuadProb: `spec_quadprobflathashsteph_wf` (defined in QuadProbFlatHashTableStEph.rs).
  Probe sequence: `(h + j²) % m`.
- DoubleHash: `spec_doublehashflathashsteph_wf` (defined in DoubleHashFlatHashTableStEph.rs).
  Probe sequence: `(h + j*s) % m` where `s = second_hash(key, m) >= 1`.

**Proof pattern** (from LinProb):
1. Remove `external_body`.
2. Add `requires spec_impl_wf(table)` (the trait method's own requires).
3. The loop invariant tracks:
   - All prior probe positions were not the target key.
   - Each probe position is computed correctly from the hash.
4. When `FlatEntry::Occupied(k, v)` with `k == *key` is found, return `Some(v)`.
5. When `FlatEntry::Empty` is found, the wf's probe-chain integrity says the key
   can't be further along — return `None`.
6. Use the eq bridge assume: `assume(eq == spec_flat_has_key(table.table@[slot], *key))`.

Read `LinProbFlatHashTableStEph.rs` lookup for the working pattern. Adapt the probe
formula for quadratic/double-hash.

**wrapping_mul blocker**: The probe computation uses `wrapping_mul` which may overflow.
For the proof, you may need to use `external_body` on the probe helper and specify
its postcondition, or use `assume` on the arithmetic. Keep it minimal.

## Rules

- Do NOT touch files outside Chap47.
- Do NOT add `requires true`.
- Run `scripts/validate.sh` after changes. 0 errors required.

## Deliverable

- `scripts/validate.sh` passes with 0 errors.
- Write report to `plans/agent3-round28-report.md`.
- `git add -A && git commit` with descriptive message.
- `git push origin agent3/ready`.
