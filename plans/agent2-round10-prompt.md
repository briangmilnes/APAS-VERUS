# Agent 2 — Round 10 Prompt

## Mission

Continue Chap47 hash table proofs (11 remaining). Start Chap42 table proofs (18 holes).

## Your Files (no other agent touches these)

**Chap47** (11 remaining external_body):
- `ParaHashTableStEph.rs` — 5: compute_load_factor, call_hash_fn, linear_probe,
  quadratic_probe, double_hash_probe
- `LinProbFlatHashTableStEph.rs` — 1: probe
- `QuadProbFlatHashTableStEph.rs` — 1: probe
- `DoubleHashFlatHashTableStEph.rs` — 2: probe, second_hash
- `ChainedHashTable.rs` — 1: insert_chained (no IndexMut)
- `StructChainedHashTable.rs` — 0 (clean)
- `VecChainedHashTableStEph.rs` — 0 (clean)
- `LinkedListChainedHashTableStEph.rs` — 0 (clean)
- `FlatHashTable.rs` — 1: find_slot requires

**Chap42** (18 holes across 4 files):
- `TableStEph.rs`
- `TableStPer.rs`
- `TableMtEph.rs`
- `Example42_1.rs` — SKIP (Example files are demo code)

## Priority Order

1. **Chap47** — you have deep context. Try the hard ones.
2. **Chap42** — new territory. Read the files first, identify what's provable.

## Specific Guidance

### Chap47 Remaining 11

You said these are all Verus limitations. Challenge that:

| # | Function | Blocker You Identified | Workaround to Try |
|---|----------|----------------------|-------------------|
| 1 | compute_load_factor | usize→f64 cast | Write a spec fn returning bool (load > threshold) using integer arithmetic instead of float. |
| 2 | call_hash_fn | Fn closure call | Inline the hash function. Match on Fn trait, call directly. |
| 3 | linear_probe | Fn closure + wrapping_add | Replace Fn call with direct hash. Use checked arithmetic + mod. |
| 4 | quadratic_probe | Fn closure + wrapping ops | Same as linear_probe. |
| 5 | double_hash_probe | Fn closure + Hash + wrapping | Hardest. May need to accept external_body here. |
| 6-8 | probe (3 flat variants) | Fn closure call | These call the probing functions above. If probes are proved, these follow. |
| 9 | second_hash | DefaultHasher | Truly external — DefaultHasher is opaque. Acceptable external_body. |
| 10 | insert_chained | No IndexMut | Use swap pattern: take from vec[i], modify, put back. |
| 11 | delete_chained | No IndexMut | Same swap pattern. |

Wait — the Round 9 report says insert_chained and delete_chained are in ChainedHashTable.rs.
But your report also said you proved insert/delete in several chained variants. Check if
the ChainedHashTable versions can use the same pattern.

### Chap42

Chap42 depends on Chap41::ArraySetStEph (3 holes — Agent 4 is fixing those). Some proofs
may be blocked until ArraySetStEph is clean. But:

1. Read all 4 files first. Understand the Table abstraction.
2. Identify holes that DON'T depend on ArraySetStEph.
3. Prove those. For blocked ones, document what's needed from ArraySetStEph.

The Table types are ordered key-value stores. They likely use AVLTreeSeq or sorted arrays
internally. Check the backing data structure.

## Rules

- Run `scripts/validate.sh` after every change.
- NO accept(). NO assume→accept conversions.
- Read standards before modifying trait signatures. Update ALL callers when adding requires.
- Push to `agent2/ready`.
- Write `plans/agent2-round10-report.md`.

## Targets

- Chap47: ≤ 8 holes
- Chap42: ≤ 12 holes
