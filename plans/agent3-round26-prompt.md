# Agent 3 — Round 26: Chap47 Flat Hash Table Lookup + Insert Proofs

## R25 Feedback

Good round. Three chained hash lookups proved with real backward/forward scan proofs.
The ghost alias pattern and the two new proof lemmas are useful infrastructure. The wf
specs for flat hash tables are the foundation for this round's work.

The hole count went up (+3) because each lookup needed 2 eq/clone bridge assumes.
That's the correct tradeoff — external_body on algorithmic logic is worse than approved
eq/clone bridge assumes.

## Mission

1. Prove lookup on 3 flat hash table files using your new wf specs (3 holes)
2. Prove insert on flat hash tables if time permits (3 more holes)
3. Fix Clone derive warnings on any non-Copy `derive(Clone)` in your files

## Current State (Chap47: 37 holes)

| # | Chap | File | Holes | Types |
|---|------|------|:-----:|-------|
| 1 | 47 | LinProbFlatHashTableStEph.rs | 4 | 4 external_body (insert, lookup, delete, resize) |
| 2 | 47 | QuadProbFlatHashTableStEph.rs | 4 | 4 external_body (insert, lookup, delete, resize) |
| 3 | 47 | DoubleHashFlatHashTableStEph.rs | 4 | 4 external_body (insert, lookup, delete, resize) |
| 4 | 47 | VecChainedHashTableStEph.rs | 9 | 8 assume + 1 external_body (resize) |
| 5 | 47 | LinkedListChainedHashTableStEph.rs | 9 | 8 assume + 1 external_body (resize) |
| 6 | 47 | StructChainedHashTable.rs | 5 | 4 assume + 1 external_body (resize) |
| 7 | 47 | ParaHashTableStEph.rs | 2 | 2 external_body (call_hash_fn, compute_second_hash — genuine FFI) |

## Part 1: Flat Hash Table Lookup (Priority 1)

You defined wf specs in R25 encoding the probing invariants:
- `spec_linprobflathashsteph_wf`: linear probe chain integrity
- `spec_quadprobflathashsteph_wf`: quadratic probe chain integrity
- `spec_doublehashflathashsteph_wf`: double hash probe chain integrity

Now prove lookup using these wf specs.

### Lookup Pattern (all three strategies)

Lookup does:
1. Hash the key → starting slot `h`
2. Probe from `h` according to strategy (linear, quadratic, double-hash)
3. At each probed slot:
   - If Empty: key not found → return None
   - If Occupied with matching key: found → return Some(value)
   - If Occupied with different key: continue probing
4. If all slots probed without finding: return None

### Proof Strategy

The wf invariant guarantees: if key `k` is in the table at slot `i`, then all slots on
the probe path from `hash(k)` to `i` are occupied. This means:

**Correctness of "not found"**: If we hit an Empty slot during probing, the key cannot
be at any later slot (because the wf invariant says the probe chain has no gaps).
Therefore Empty ⟹ key not in table.

**Correctness of "found"**: If we find a matching key at slot `i`, the wf invariant
(no duplicate keys) guarantees this is THE entry for this key.

### Per-Strategy Proof Differences

**Linear probing**: Simplest. Probe sequence is `h, h+1, h+2, ...` mod capacity. The
loop index directly maps to the probe offset. The wf invariant says all slots from `h`
to `i` are occupied for any key at slot `i`.

**Quadratic probing**: Probe sequence is `h, h+1, h+4, h+9, ...` (h + j² mod capacity).
The loop tracks attempt number `j`. The wf invariant uses `exists n: nat` with
`(h + n*n) % capacity == i`.

**Double hashing**: Probe sequence is `h, h+s, h+2s, ...` where `s = hash2(key)`.
The loop tracks step count. The wf invariant uses `exists n: nat` with
`(h + n*s) % capacity == i`.

### Eq Bridge Pattern

Each lookup needs to compare keys. Use the standard eq bridge assume pattern (same as
your chained hash lookups):
```rust
let matches = slot_key == target_key;
proof { assume(matches == (slot_key@ == target_key@)); }
```

This is the only allowed assume. 1 per lookup function.

### Ghost Tracking

Track the probe attempt number as a ghost variable in the loop:
```rust
let ghost mut attempt: nat = 0;
// loop invariant: current_slot == (h + probe_offset(attempt)) % capacity
```

## Part 2: Flat Hash Table Insert (Priority 2, if time permits)

Insert follows the same probing pattern but writes instead of reads:
1. Probe from `hash(key)` according to strategy
2. At each slot:
   - If Empty: insert here → return
   - If Occupied with matching key: update value → return
   - If Occupied with different key: continue probing
3. If table full: resize and retry

### Insert Proof

Proving insert requires showing the wf invariant is maintained:
- New entry: probe chain integrity still holds (the new entry fills a gap, doesn't
  create one)
- Update existing: no structural change, wf trivially maintained
- No duplicate keys: either updating existing or adding at new position

This is harder than lookup. Attempt it only if lookup goes smoothly.

## Part 3: Clone Derive Warnings (Priority 3)

Check your Chap47 files for `#[derive(Clone)]` on non-Copy types. If any exist, replace
with manual Clone impl per `src/standards/partial_eq_eq_clone_standard.rs`.

## What to Skip

- **resize** on any hash table: rehashing proof is complex, defer
- **delete** on flat hash tables: tombstone/shift-back semantics add complexity, defer
- **ParaHashTableStEph call_hash_fn/compute_second_hash**: genuine FFI, stays external_body

## Important

- You MAY use the eq/clone bridge assume inside lookup/insert bodies — this is the
  approved pattern.
- Do NOT add any other `assume`, `accept`, or `external_body`.
- Do NOT add `requires true`. Omit requires if no precondition needed.
- Do NOT weaken any existing ensures.
- `scripts/validate.sh` after each file — 0 errors.

## Deliverables

- Lookup proved in LinProbFlatHashTableStEph, QuadProbFlatHashTableStEph,
  DoubleHashFlatHashTableStEph (3 holes)
- Insert proved if time permits (up to 3 more holes)
- Clone derive warnings fixed if present
- `plans/agent3-round26-report.md`
- 0 errors on validate.
- Commit + push to `agent3/ready`.
