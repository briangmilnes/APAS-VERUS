# R48 Agent 3: Chap47 Hash Tables (4 holes)

## Assignment

Continue Chap47 hash table work from R46. You proved LinProb's assume(false)
via proof-by-contradiction using lemma_probe_mod_identity. Now tackle the
remaining 4 holes.

## Baseline

38 holes total. 4419 verified. Your chapter: Chap47 (4 holes).

## REQUIRED READING

1. `src/standards/using_closures_standard.rs`
2. `src/standards/partial_eq_eq_clone_standard.rs`
3. Your own R46 report: `plans/agent3-round46-report.md`

## Current Holes

Run `scripts/holes.sh src/Chap47/` to verify.

| # | Chap | File | Line | Function | Type | Notes |
|---|------|------|------|----------|------|-------|
| 1 | 47 | DoubleHashFlatHashTableStEph.rs | 366 | insert | assume(false) | Table-full diverge |
| 2 | 47 | QuadProbFlatHashTableStEph.rs | 383 | insert | assume(false) | Table-full diverge |
| 3 | 47 | ParaHashTableStEph.rs | 110 | clone_elem | assume | Clone bridge |
| 4 | 47 | ParaHashTableStEph.rs | 488 | call_hash_fn | external_body | Closure calling |

## Strategy

### #1 Double hashing assume(false)

You proved LinProb by showing linear probing visits all m slots (mod identity).
Double hashing uses `h(k) + i * h2(k)` for probe sequence. This visits all m
slots IF gcd(h2(k), m) == 1. Typical implementations ensure this by making m
prime or m a power of 2 with h2 always odd. Check what this implementation does.

If the implementation ensures gcd(h2(k), m) == 1, then the same proof-by-
contradiction approach works: m probes visit m distinct slots, so an empty must
be found. You already have the counting infrastructure from R46.

### #2 Quadratic probing assume(false)

Quadratic probing uses `h(k) + c1*i + c2*i²`. This does NOT visit all m slots
in general. It visits m/2 slots if m is prime and c1=c2=1/2, or all m if m is
a power of 2 with specific c1,c2. Check the implementation.

If the implementation ensures at least m/2 probes, and the load factor < 0.5,
the proof still works. If the load factor can exceed 0.5, this may truly need
the assume.

### #3 clone_elem

This is the standard Clone bridge: `assume(c == *x)` inside clone(). Per
`partial_eq_eq_clone_standard.rs`, this is the accepted pattern — it lives
inside the clone body. This is likely irreducible unless Verus adds Clone
spec support. Leave it.

### #4 call_hash_fn

This wraps a closure call `f(key)` in external_body because Verus cannot
verify opaque closure calls. Check if the closure has spec ensures — if so,
you might be able to use vstd's closure calling support or a helper. If the
closure is truly opaque, this stays external_body with tight ensures.

## What NOT to do
- Do NOT add `assume()` or `accept()` without user approval.
- Do NOT weaken ensures clauses.
- Do NOT revert your R46 work.

## Validation

Run `scripts/validate.sh` after each file change. Show full output.
Run `scripts/rtt.sh` after all changes.
Run `scripts/holes.sh src/Chap47/`.
Write your report to `plans/agent3-round48-report.md`.
