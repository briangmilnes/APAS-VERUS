<style>
body { max-width: 100% !important; width: 100% !important; margin: 0 !important; padding: 1em !important; }
.markdown-body { max-width: 100% !important; width: 100% !important; }
table { width: 100% !important; table-layout: fixed; }
</style>

# Agent 1 — Round 52 Report

## Summary

All 3 holes in Chap47 are structurally blocked. No holes were closed this round.
Verification remains at 4472 verified, 0 errors.

## Holes Before / After

| # | Chap | File | Holes Before | Holes After | Delta |
|---|:----:|---|---:|---:|---:|
| 1 | 47 | ParaHashTableStEph.rs | 2 | 2 | 0 |
| 2 | 47 | QuadProbFlatHashTableStEph.rs | 1 | 1 | 0 |
| — | — | **Total** | **3** | **3** | **0** |

## Verification Count

- 4472 verified, 0 errors (unchanged)

## Holes Closed

None.

## Hole Analysis

### Hole 1 — `clone_elem` `assume(c == *x)` (ParaHashTableStEph.rs:123)

**What it does:** Proves that `x.clone()` returns a value structurally equal to `*x`.

**Why it's blocked:** `ClonePreservesView` (vstdplus/clone_view.rs, added by Agent4 in R50)
provides `ensures result@ == self@` (view equality). The current ensures is `c == *x`
(structural equality), which is stronger. Changing to view equality would require updating
all 20+ callers across Chap47 files and reworking proofs that depend on structural equality
for hash computations (where `call_hash_fn` takes the exec `Key` value, not the view).

**What would close it:** A generic `assume_specification` for `T::clone` in vstd (upstream
change) or removing the structural equality requirement from `clone_elem` + updating all
callers to use view equality only.

### Hole 2 — `call_hash_fn` `external_body` (ParaHashTableStEph.rs:501)

**What it does:** Connects the exec `H: Fn(&Key, usize) -> usize` hash function to the
ghost `spec_fn(Key) -> nat` specification.

**Why it's blocked:** This is a structural trust boundary. In Verus, generic `Fn` closures
carry no spec by default. Connecting exec behavior to a ghost spec requires one of:
- A custom trait with a verified spec (would require redesigning the entire hash table type
  hierarchy), or
- `external_body` (current approach), or
- `assume` inside the body.

Adding `requires hash_fn.ensures((key, table_size), result) ==> result matches spec` would
cascade through `linear_probe`, `quadratic_probe`, `double_hash_probe` and all concrete
insert/lookup/delete implementations (all generic). The wf predicates don't include
hash-function consistency, so the cascade has no place to terminate.

**What would close it:** A non-generic hash function design where the concrete hash function
type carries a Verus spec, or a verified `HashMap`-style type in vstd that encapsulates the
spec-hash relationship.

### Hole 3 — `assume(false)` in QuadProb insert (QuadProbFlatHashTableStEph.rs:383)

**What it does:** Asserts the loop-exhausted branch (all m probe positions tried, none
empty) is unreachable.

**Why it's blocked:** `LinProbFlatHashTableStEph.rs` closes its equivalent hole via
`lemma_probe_mod_identity` — linear probing `(h + d) % m` visits ALL m distinct slots,
so "all probe positions non-empty" + "some slot is empty" (from `spec_has_insert_capacity`)
contradicts. Quadratic probing `(h + d²) % m` visits only ~(m+1)/2 distinct slots even
for prime m — there are ~m/2 slots the probe never reaches. So the same argument fails.

**What would close it:** One of:
- A proof that for prime m, the quadratic probe sequence covers all m slots (false — it
  covers (m+1)/2 at most), OR
- A key-specific capacity requires threading through the trait and resize function (complex;
  the resize loop would need a non-trivial invariant about free probe slots per key), OR
- Restricting QuadProb to half-full tables and proving probe coverage in that regime.

## Blockers Summary

| # | Chap | File | Line | Hole | Blocker |
|---|:----:|---|-----:|------|---------|
| 1 | 47 | ParaHashTableStEph.rs | 123 | `assume(c == *x)` | No generic clone spec in vstd |
| 2 | 47 | ParaHashTableStEph.rs | 501 | `external_body` | Generic Fn spec boundary |
| 3 | 47 | QuadProbFlatHashTableStEph.rs | 383 | `assume(false)` | Quad probe visits only m/2 slots |

## Notes on `fn_missing_wf_requires` Warnings

The veracity report shows 6 `fn_missing_wf_requires` and 2 `fn_missing_wf_ensures` warnings
on functions in `ParaHashTableStEph.rs`. These are WARNINGS, not HOLES. They say "requires
should include `table.spec_hashtable_wf()`." The functions already require
`Self::spec_impl_wf(table)`, which for concrete implementations implies `spec_hashtable_wf`.
Veracity doesn't know about this implication, hence the warning. Fixing these by adding
explicit `spec_hashtable_wf` alongside `spec_impl_wf` would be correct but requires user
approval (it would change the trait interface without restructuring the wf predicates).
