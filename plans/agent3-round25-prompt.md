# Agent 3 — Round 25: Chap47 Flat Hash Table Wf Specs + Chained Lookup

## Mission

Two goals: (1) Define per-strategy well-formedness specs for the 3 flat hash table files,
redesigning to accommodate open-addressing probing semantics. (2) Prove lookup on chained
hash tables.

User feedback: "It needs one per: spec_linprobflathashsteph_wf, spec_quadprobflathashsteph_wf,
spec_doublehashflathashsteph_wf."

## Current State (34 holes in Chap47)

| # | Chap | File | Holes | Types |
|---|------|------|:-----:|-------|
| 1 | 47 | LinProbFlatHashTableStEph.rs | 4 | 4 external_body (insert, lookup, delete, resize) |
| 2 | 47 | QuadProbFlatHashTableStEph.rs | 4 | 4 external_body (insert, lookup, delete, resize) |
| 3 | 47 | DoubleHashFlatHashTableStEph.rs | 4 | 4 external_body (insert, lookup, delete, resize) |
| 4 | 47 | VecChainedHashTableStEph.rs | 8 | 6 assume (eq/clone bridges), 2 external_body (lookup, resize) |
| 5 | 47 | LinkedListChainedHashTableStEph.rs | 8 | 6 assume (eq/clone bridges), 2 external_body (lookup, resize) |
| 6 | 47 | StructChainedHashTable.rs | 4 | 2 assume (eq bridges), 2 external_body (lookup, resize) |
| 7 | 47 | ParaHashTableStEph.rs | 2 | 2 external_body (call_hash_fn, compute_second_hash — genuine FFI) |

## Part 1: Flat Hash Table Well-Formedness Specs (Priority 1)

Currently, NONE of the 3 flat hash table files have ANY wf spec. Your R24 report noted
the architectural blocker: `spec_hashtable_wf` (from ParaHashTableStEph) requires keys
to be in their hash slot, but open-addressing probing displaces keys.

### Strategy: Define Probing-Aware Wf Specs

Each flat hash table strategy needs its own wf predicate that accounts for probing:

**LinProbFlatHashTableStEph.rs** — `spec_linprobflathashsteph_wf`:
- Table is an array of `Option<(Key, Value)>` slots
- For every occupied slot `i` containing key `k`:
  - `k` hashes to some slot `h = spec_hash(k) % table_size`
  - Slot `i` is reachable from `h` by linear probing: `i = (h + offset) % table_size`
    for some `offset >= 0`
  - All slots between `h` and `i` (wrapping) are occupied (no gaps in the probe chain)
- No duplicate keys in the table

**QuadProbFlatHashTableStEph.rs** — `spec_quadprobflathashsteph_wf`:
- Same structure, but probing is quadratic: slot `(h + c1*j + c2*j*j) % table_size`
- The wf invariant says each key is at a valid quadratic probe position from its hash
- Probe chain integrity: all earlier probe positions for the same hash are occupied

**DoubleHashFlatHashTableStEph.rs** — `spec_doublehashflathashsteph_wf`:
- Probing uses a second hash: slot `(h + j * h2(k)) % table_size`
- wf: each key is at a valid double-hash probe position
- Probe chain integrity with the second hash step size

### What to Implement

For each file:
1. Read the file thoroughly — understand the probing scheme
2. Define `spec_<strategy>_wf` with the probing-aware invariant described above
3. Add the wf spec to `new`'s ensures and mutation functions' requires/ensures
4. **Do not yet attempt to prove insert/lookup/delete** — just get the wf specs right
   and ensure they validate

Getting the wf spec right is the foundation. Proofs come after.

### Common Elements

All three share:
- `spec_hash: Ghost<spec_fn(Key) -> nat>` from R23
- Load factor constraint: `occupied_count < capacity * max_load_factor`
- Table size > 0
- No duplicate keys

## Part 2: Chained Hash Table Lookup (Priority 2)

3 files have `lookup` as `external_body`: VecChained, LinkedList, StructChained.

Lookup does:
1. Hash the key → bucket index
2. Scan the bucket for matching key
3. Return the value if found

This is structurally identical to the scan you proved for insert/delete. The difference:
lookup doesn't modify the table, it reads. The eq bridge assume pattern applies.

### Lookup Proof Pattern

```rust
fn lookup(&self, key: &Key) -> Option<&Value>
    requires self.spec_hashtable_wf(),
    ensures
        result is Some ==> self@.contains_key(key@) && result.unwrap()@ == self@[key@],
        result is None ==> !self@.contains_key(key@),
{
    let index = call_hash_fn(&self.hash_fn, key, self.table.len(), Ghost(self.spec_hash));
    // scan bucket at index for matching key
    // eq bridge assume in the scan loop
    // wf guarantees key is in this bucket if it exists
}
```

## Part 3: Replace `requires true` in Chap47 (Priority 3)

7 instances of `requires true` across 4 Chap47 files:
- LinkedListChainedHashTableStEph.rs (1)
- VecChainedHashTableStEph.rs (1)
- ParaHashTableStEph.rs (2)
- StructChainedHashTable.rs (3)

Replace with `spec_hashtable_wf()` or appropriate predicates.

## What to Skip

- **Flat hash table insert/lookup/delete/resize**: Just define the wf specs this round.
  The proofs are architecturally hard and need the wf foundation first.
- **Chained hash table resize**: Needs proving map preservation across rehash. Defer.
- **call_hash_fn / compute_second_hash**: Genuine FFI, stays external_body forever.

## Important

- You MAY define new spec functions (`spec_*_wf` predicates).
- You MAY add requires/ensures and strengthen existing specs.
- Do NOT weaken any existing ensures.
- Do NOT add `assume`, `accept`, or `external_body`.
- The eq bridge `assume` in scan loops is the ONLY allowed assume pattern in Chap47.
- Do NOT add `requires true`. If no precondition needed, omit requires entirely.
- `scripts/validate.sh` after changes — 0 errors.

## Deliverables

- `spec_linprobflathashsteph_wf` defined in LinProbFlatHashTableStEph.rs
- `spec_quadprobflathashsteph_wf` defined in QuadProbFlatHashTableStEph.rs
- `spec_doublehashflathashsteph_wf` defined in DoubleHashFlatHashTableStEph.rs
- Lookup proved in VecChained, LinkedList, StructChained (3 holes)
- `requires true` replaced with real specs (7 instances)
- `plans/agent3-round25-report.md`
- 0 errors on validate.
- Commit + push to `agent3/ready`.
