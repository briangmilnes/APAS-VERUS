# R159 Agent 2 — Minimize QuadProbFlatHashTable::insert Proof. AFK.

## CRITICAL SAFETY RULES

1. **NEVER modify `~/projects/verus/`.** Not a single file.
2. **NEVER run `rm -rf` on any directory.**
3. **NEVER run PTTs.** Skip `scripts/ptt.sh` entirely.
4. **NEVER delete `target/` or any subdirectory.**

## Setup

Read ALL files in `src/standards/` before starting.
Read `src/Chap47/QuadProbFlatHashTableStEph.rs` — your file.

Report file: `plans/r159-agent2-minimize-quadprob-report.md`

## Problem

`fn insert` at line 572 has 228 lines and 128 asserts. Most are AI-generated
intermediate assertions that Z3 doesn't need. The minimizer's brute-force
approach (remove one assert, validate, repeat) takes 26 minutes at 12s per
validation. You will do it smarter.

## You are a senior proof engineer.

Do NOT test assertions one by one. Think about what the proof NEEDS.

The function has three code paths:
1. **Overwrite** (key found, replace value) — must prove table_to_map updated,
   no-dup preserved, probe chain preserved, wf preserved.
2. **Insert new** (empty slot found) — must prove table_to_map extended,
   no-dup preserved, probe chain extended, wf preserved, num_elements updated.
3. **Probe continues** (occupied, different key) — must prove loop invariant
   maintained, slot advances correctly.

## The approach

### Step 1: Understand what the proof must establish

Read the function's `ensures` clause and loop `invariant`. These are the
proof obligations. Everything in the body exists to satisfy them. Write
down the 5-8 key obligations.

### Step 2: Strip ALL proof blocks and asserts

Comment out EVERY `proof { ... }` block and every standalone `assert`
in the function body. Keep:
- The exec code (variable bindings, match arms, assignments, return)
- The loop invariant (Verus needs this)
- The ensures clause

### Step 3: Validate and read errors

```bash
scripts/validate.sh isolate Chap47
```

Verus will report which ensures/invariants it can't prove. Read them.
They tell you exactly what Z3 needs help with.

### Step 4: Add back ONLY what's needed

For each failing obligation:
1. Think: what is the MINIMUM assertion that proves this?
2. Often it's ONE assert or ONE lemma call, not a 20-line proof block.
3. Z3 is good at: simple equalities, arithmetic, single-step unfolding.
4. Z3 is bad at: multi-step chains, choosing witnesses for existentials,
   connecting two distant facts.

Common patterns where Z3 needs help:
- **Existential witnesses**: `assert(table.table@[spec_tri_probe(h, n, m)] == ...)`
  — Z3 can't guess which `n` satisfies `exists|n|`.
- **Trigger activation**: `assert(spec_flat_has_key(table.table@[i], k))` —
  Z3 needs a ground term matching a quantifier's trigger pattern.
- **Lemma calls**: `lemma_table_to_map_update_insert(...)` — Z3 can't
  discover and apply library lemmas on its own.

Patterns where Z3 does NOT need help:
- `assert(x == x)` — tautologies
- `assert(table.table@[j] == old_table_seq[j])` when j != slot and only
  slot was modified — Z3 sees the `set` operation and infers unchanged slots.
- `assert(spec_flat_has_key(...) ==> k == key)` when this follows directly
  from the `Occupied(key, value)` constructor — Z3 unfolds one step.
- Intermediate equalities that just restate what the line above computed.

### Step 5: Iterate

Validate after adding back the minimum. New failures may appear because
some assertions were stepping-stones for others. Add the minimum for each
new failure. Usually 3-5 rounds to converge.

### Step 6: Apply to the other two hash table inserts

`LinProbFlatHashTableStEph::insert` (119 asserts) and
`DoubleHashFlatHashTableStEph::insert` (130 asserts) have the same proof
structure — same three code paths, same obligations. Apply the same
minimal proof pattern. The only difference is the probe function
(linear vs double hash vs quadratic).

## What to keep (almost certainly needed)

- `lemma_table_to_map_update_insert(...)` — library lemma, Z3 can't derive
- `lemma_reveal_view_injective::<Key>()` — reveals view injectivity
- `lemma_small_mod(...)` — arithmetic lemma for initial probe
- The `assert forall ... by { ... }` blocks for no-dup and probe-chain
  preservation — but check if the BODIES of these blocks can be simplified.
  The `assert forall` header is needed (it's the proof obligation).
  The `by` body may have redundant case analysis.

## What to remove (almost certainly unneeded)

- `assert(x == x)` style tautologies
- `assert(table.table@[j] == old_table_seq[j])` for unchanged slots
- Intermediate equalities restating what set() just did
- Redundant case-split assertions inside `by` blocks
- `assert(obeys_feq_full_trigger::<Key>())` — the broadcast handles this

## Validation

`scripts/validate.sh isolate Chap47` — 12 seconds per run.
Full `scripts/validate.sh` before pushing. Then `scripts/rtt.sh`.

## Target

Reduce `insert` from 228 lines / 128 asserts to under 80 lines / 20 asserts
across all three hash table variants. Report line count and assert count
before/after for each.

## Rules

- Do NOT weaken any ensures or loop invariant.
- Do NOT add assumes, accepts, or external_body.
- All existing RTTs must pass.
- If you can't get below 20 asserts, that's fine — report what's irreducible.

## When done

RCP.
