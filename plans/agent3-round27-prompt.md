# Agent 3 — Round 27: Wire Per-Module WF into Flat Hash Tables + Insert Proofs

## R26 Feedback

Good round — 3 lookup proofs completed with real probe-chain reasoning. The centralized
`FlatEntry::clone` with ensures is useful infrastructure.

However, you have a spec architecture problem: each flat hash table module has its own wf
predicate defined (`spec_linprobflathashsteph_wf`, `spec_quadprobflathashsteph_wf`,
`spec_doublehashflathashsteph_wf`) but **none of them are used anywhere**. Your lookup
proofs use `spec_hashtable_wf` (from ParaHashTableStEph), which encodes chained hashing
semantics: "each key k is at slot hash(k) % m." That works for lookup via the attempt-0
trick, but breaks for insert because insert places keys at non-hash slots during collision
handling.

The fix is NOT "a design change" or "a new trait." The fix is: replace `spec_hashtable_wf`
with the per-module wf in each file's trait impl functions. Each file's wf already encodes
the correct probe-chain invariant. Wire it in.

## Mission

1. Replace `spec_hashtable_wf` with per-module wf in all flat hash table trait impls
2. Prove insert on all 3 flat hash tables using the per-module wf
3. Attempt delete if time permits

## Part 1: Wire Per-Module WF (Priority 1)

### Current State

Each file defines a per-module wf (dead code) and uses `spec_hashtable_wf` (wrong):

| # | Chap | File | Per-Module WF (defined, unused) | Generic WF (used, wrong for open addressing) |
|---|------|------|---------------------------------|----------------------------------------------|
| 1 | 47 | LinProbFlatHashTableStEph.rs | `spec_linprobflathashsteph_wf` (line 37) | `spec_hashtable_wf` (line 113) |
| 2 | 47 | QuadProbFlatHashTableStEph.rs | `spec_quadprobflathashsteph_wf` (line 39) | `spec_hashtable_wf` (line 114) |
| 3 | 47 | DoubleHashFlatHashTableStEph.rs | `spec_doublehashflathashsteph_wf` (line 39) | `spec_hashtable_wf` (line 131) |

### What to Do

In each file:
1. Read the per-module wf definition (lines 37-60). Understand what it asserts:
   - No duplicate keys
   - Probe chain integrity (all slots between hash(k) and slot(k) are occupied)
   - The specific probing function (linear: `h+i`, quadratic: `h+i^2`, double: `h+i*s`)
2. In the trait impl, replace every `spec_hashtable_wf(table)` with the per-module wf
3. Also add the per-module wf to the trait's `spec_*_wf` function (the struct-level wf)
   if it's not already there

### Impact on Lookup

Your existing lookup proofs may need updating once `spec_hashtable_wf` is replaced with
the per-module wf. The per-module wf is STRONGER than `spec_hashtable_wf` (it knows about
probe chains, not just hash-slot placement), so lookup should be easier to prove, not
harder. But the proof steps may differ — you may need to use the probe-chain invariant
instead of the attempt-0 trick.

Review each lookup proof after the wf swap and fix any verification failures.

## Part 2: Insert Proofs (Priority 2)

With the per-module wf wired in, insert becomes provable. The proof obligation is:

**Insert maintains the per-module wf invariant:**
1. Hash the key → slot h
2. Probe until finding an Empty slot or a matching key
3. Write the entry
4. Prove: the probe chain invariant still holds for all keys in the table

### Insert Proof Strategy

For linear probing (simplest — do this first):
- **New key at empty slot**: All existing probe chains are unaffected (we didn't move
  anything). The new key's probe chain is: from hash(k) to the empty slot, all intermediate
  slots were occupied (we probed past them). So the invariant holds for the new key.
- **Update existing key**: No structural change. WF trivially maintained.
- **No duplicate keys**: If we found a matching key, we update (not insert). If we found
  empty, the key wasn't in the table (by the wf invariant — no gaps in probe chains).

For quadratic and double hashing: same argument but with the respective probing functions.

### Eq Bridge

Same pattern as lookup — 1 assume per insert for key comparison:
```rust
let matches = slot_key == target_key;
proof { assume(matches == (slot_key@ == target_key@)); }
```

## Part 3: Delete (Priority 3, if time permits)

Delete in open-addressing hash tables requires either:
- **Tombstoning**: Mark deleted slots as Deleted (not Empty). Lookup probes past Deleted
  slots. Insert can reuse Deleted slots.
- **Shift-back**: After deletion, shift subsequent entries back to fill the gap.

Read the existing delete bodies to see which strategy they use. The proof is harder than
insert because deletion can break probe chains for other keys.

Only attempt if insert goes smoothly.

## What to Skip

- **resize**: Rehashing proof is complex, defer
- **ParaHashTableStEph**: Its 2 holes are genuine FFI (`call_hash_fn`, `compute_second_hash`)
- **Chained hash tables**: Different design, different round

## Important

- You MAY replace `spec_hashtable_wf` with per-module wf in requires/ensures.
- You MAY use the eq/clone bridge assume inside insert bodies.
- Do NOT add any other `assume`, `accept`, or `external_body`.
- Do NOT add `requires true`.
- Do NOT weaken any existing ensures.
- Do NOT invent a shared `spec_flat_hashtable_wf` — each module's wf is specific to its
  probing strategy and must remain separate.
- `scripts/validate.sh` after each file — 0 errors.

## Deliverables

- Per-module wf wired into all 3 flat hash table files (replacing `spec_hashtable_wf`)
- Lookup proofs still passing with new wf
- Insert proved in LinProb, QuadProb, DoubleHash (3 holes)
- `plans/agent3-round27-report.md`
- 0 errors on validate.
- Commit + push to `agent3/ready`.
