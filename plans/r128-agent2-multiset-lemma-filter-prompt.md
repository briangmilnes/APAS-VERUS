# R128 Agent 2 — Multiset filter distribution lemma + parallelize filter. AFK.

## Task 1: Write the multiset filter distribution lemma

Write a proof lemma in `src/vstdplus/seq_multiset.rs` (or the appropriate vstdplus file):

```rust
pub proof fn lemma_multiset_filter_distributes_over_add<A>(
    m1: Multiset<A>, m2: Multiset<A>, f: spec_fn(A) -> bool,
)
    ensures m1.add(m2).filter(f) =~= m1.filter(f).add(m2.filter(f)),
```

**Proof sketch**: By multiset extensionality (`=~=`). For any `v`:
- LHS: `m1.add(m2).filter(f).count(v)` = `if f(v) { m1.count(v) + m2.count(v) } else { 0 }`
  (by `axiom_filter_count` and `axiom_multiset_add`)
- RHS: `m1.filter(f).add(m2.filter(f)).count(v)` = `m1.filter(f).count(v) + m2.filter(f).count(v)`
  = `if f(v) { m1.count(v) } else { 0 } + if f(v) { m2.count(v) } else { 0 }`
  = LHS

Use `assert forall|v: A| m1.add(m2).filter(f).count(v) == m1.filter(f).add(m2.filter(f)).count(v) by { ... }` 
then extensionality gives `=~=`.

Also write the corollary for sequences:

```rust
pub proof fn lemma_seq_concat_to_multiset_filter<A>(
    a: Seq<A>, b: Seq<A>, f: spec_fn(A) -> bool,
)
    ensures
        (a + b).to_multiset().filter(f)
            =~= a.to_multiset().filter(f).add(b.to_multiset().filter(f)),
```

This follows from `lemma_multiset_commutative` (already in vstd: `(a + b).to_multiset() =~= a.to_multiset().add(b.to_multiset())`) + the distribution lemma above.

Check if `src/vstdplus/seq_multiset.rs` exists; if not, create it and register in
`src/vstdplus/mod.rs`.

## Task 2: Parallelize filter in `src/Chap18/ArraySeqMtEph.rs`

Using the lemma from Task 1, rewrite the `filter` trait method body to use
divide-and-conquer with `join()`. Follow the same pattern as the existing `map_dc`
and `reduce_dc` from R127.

Pattern:
- Base: len 0 → empty, len 1 → test and keep or drop
- Recursive: split at mid, join(filter_left, filter_right), append results
- Post-join proof: use `lemma_seq_concat_to_multiset_filter` to prove the multiset
  ensures of the concatenated result

The trait signature for filter needs `F: Clone + Send + Sync + 'static` bounds
(same as map/reduce got in R127). Add them to both trait declaration and impl.

## Task 3: Parallelize filter in `src/Chap18/ArraySeqMtPer.rs`

Same pattern as Task 2 but for the MtPer file. Read how R127 agent 3 structured
the file (it may have `_inner` helpers you can reuse or replace).

## Task 4: Parallelize filter in `src/Chap19/ArraySeqMtEph.rs`

Same pattern as Task 2 but for Chap19. Read how R127 agent 4 structured the file.

## Read these standards FIRST

1. `src/standards/using_closures_standard.rs`
2. `src/standards/hfscheduler_standard.rs`

## Validation

After each file change, validate with isolate:
- `scripts/validate.sh isolate Chap18` (after Tasks 2-3)
- `scripts/validate.sh isolate Chap19` (after Task 4)
Then run `scripts/rtt.sh` once at the end.

## Rules

- Named closures with explicit `ensures` for every `join()` call.
- Do NOT add `assume`, `accept`, or `external_body`.
- Do NOT weaken `ensures` clauses — the multiset postcondition MUST be preserved.
- Use `clone_pred` from `crate::vstdplus::clone_plus::clone_plus::*` for cloning predicates.

## When done

Commit with `git add -A && git commit` and push.

## Report

Write `plans/agent2-r128-report.md` with:
- Whether the lemma proof succeeded
- Table of filter parallelizations (# | Chap | File | Old Span | New Span | Status)
- Verification counts
