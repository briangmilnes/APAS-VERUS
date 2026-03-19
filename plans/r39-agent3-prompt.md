# R39 Agent 3: Apply feq Technique to Chap47 Flat Hash Tables

## Baseline
- Main at `e6e3c688`, Verus release/0.2026.03.17.a96bad0, Rust 1.94.0
- 4337 verified, 175 holes, 29 clean chapters

## MANDATORY RULES — READ BEFORE WRITING ANY CODE

**DO NOT USE `accept()` ANYWHERE. NOT ONCE. NOT EVER. NOT FOR ANY REASON.**
**DO NOT weaken ensures.** Do not delete postconditions to make proofs easier.
**DO NOT add `requires true` or `// veracity: no_requires`.**
**DO NOT convert assume() to accept().**

Read CLAUDE.md and `src/standards/partial_eq_eq_clone_standard.rs` before starting.

## Context

In R38, Agent 4 proved 22 eq/clone bridge assumes in Chap47's chained hash tables
using the **feq technique**:
- `feq()` with broadcast trigger assertions proves eq bridges without assumes
- `clone_elem` helpers consolidate clone bridges
- 22 holes closed in LinkedListChainedHashTableStEph.rs + VecChainedHashTableStEph.rs

In the same round, Agent 1 attempted the same holes in the **flat** hash tables and
concluded they were "impossible without type system changes." Agent 1 did NOT try the
feq technique that Agent 4 used successfully.

## Assignment

Apply Agent 4's feq technique to the flat hash table eq/clone bridges in Chap47.

### Required Reading

Before writing any code, read these files to understand the proven technique:
1. `src/Chap47/LinkedListChainedHashTableStEph.rs` — see how Agent 4 proved eq bridges
2. `src/Chap47/VecChainedHashTableStEph.rs` — same technique, second file
3. `src/vstdplus/feq/feq.rs` — understand feq, obeys_feq_full, feq axioms
4. `plans/agent4-r38-report.md` — Agent 4's technique description

### Target Files and Holes

1. **`src/Chap47/LinProbFlatHashTableStEph.rs`** — 6 holes
   - Line 132: eq bridge in insert (`assume(eq == spec_flat_has_key(...))`)
   - Line 348: assume(false) table full
   - Line 391: eq bridge in lookup
   - Line 497: eq bridge in delete
   - Lines 715-716: clone bridges in resize

2. **`src/Chap47/QuadProbFlatHashTableStEph.rs`** — 6 holes
   - Line 110: eq bridge in insert
   - Line 366: assume(false) table full
   - Line 404: eq bridge in lookup
   - Line 563: eq bridge in delete
   - Lines 835-836: clone bridges in resize

3. **`src/Chap47/DoubleHashFlatHashTableStEph.rs`** — 9 holes
   - Line 96: wf bridge in insert (forall quantifier about step)
   - Line 149: eq bridge in insert
   - Line 362: assume(false) table full
   - Line 375: wf bridge in lookup
   - Line 425: eq bridge in lookup
   - Line 521: wf bridge in delete
   - Line 573: eq bridge in delete
   - Lines 807-808: clone bridges in resize

4. **`src/Chap47/StructChainedHashTable.rs`** — 4 holes
   - Line 124: eq bridge in insert
   - Line 163: eq bridge in lookup
   - Line 167: clone bridge in lookup
   - Line 205: eq bridge in delete

### The feq Technique (from Agent 4)

The eq bridge pattern is:
```rust
// BEFORE (assume):
let eq = k == key;
proof { assume(eq == spec_flat_has_key(table.table@[slot], key)); }

// AFTER (feq proof):
let eq = k == key;
proof {
    assert(obeys_feq_full_trigger::<Key>());
    // feq ensures: eq == (k@ == key@)
    // Then connect k@ == key@ to spec_flat_has_key via spec definition
    assert(eq == spec_flat_has_key(table.table@[slot as int], key));
}
```

The clone bridge pattern is:
```rust
// BEFORE (assume):
let k = key.clone();
proof { assume(k == pairs@[j].0); }

// AFTER (clone_plus proof):
let k = key.clone_plus();
proof { lemma_cloned_view_eq(*key_ref, k); }
```

### The Flat Table Difference

Flat hash tables use `spec_flat_has_key(entry, key)` which checks if a slot contains
the key. The eq bridge needs to connect `PartialEq::eq` result to this spec function.
Read the spec definition of `spec_flat_has_key` to understand the chain:
- `eq == (k@ == key@)` (from feq/PartialEq ensures)
- `(k@ == key@) == spec_flat_has_key(entry, key)` (from spec definition, when entry is Occupied(k, _))

### The assume(false) Table-Full Holes

These 3 holes (one per flat table) assert that with load factor < 1, the probing loop
always finds an empty slot. The proof needs pigeonhole: if num_elements < capacity,
there must be at least one Empty slot. Note: Deleted slots complicate this — a table
can have all Deleted slots and 0 Occupied, with num_elements=0 but no Empty slot.
Check if the resize/insert logic maintains an invariant about minimum Empty slots.
If this is genuinely hard, leave these 3 assumes and focus on the eq/clone bridges.

### The DoubleHash wf Bridges

Lines 96, 375, 521 in DoubleHashFlatHashTableStEph.rs are different — they assume a
forall about `spec_flat_has_key` connecting the runtime step to the wf existential
witness. These may need `compute_second_hash` to not be `external_body`. If they're
blocked by that, leave them and report why.

### Strategy

1. Start with LinProbFlatHashTableStEph.rs (simplest probing scheme)
2. Prove the eq bridges using feq (3 per file)
3. Prove the clone bridges using clone_plus (2 per file)
4. Try the assume(false) table-full proof
5. Replicate to QuadProb, DoubleHash, StructChained

### Expected Results

Best case: -22 to -25 holes (all eq/clone bridges across 4 files).
Conservative: -12 to -15 holes (eq/clone bridges in LinProb + QuadProb + Struct).
DoubleHash wf bridges may remain.

## Validation

Run `scripts/validate.sh` after each file. Must be 0 errors.
Write your report to `plans/agent3-r39-report.md`.
