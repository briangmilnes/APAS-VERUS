# Agent 3 — Round 19: Tier 2 Spec Audit — Chap05 + Chap18 + Chap19

## Mission

Audit every exec fn's `requires`/`ensures` in Chap05 (Sets, Relations, Mappings, Kleene),
Chap18 (Sequences), and Chap19 (Stacks, Queues, Deques) against APAS textbook prose.
These are foundation ADTs that many other chapters depend on. Their specs must be correct.

## Procedure

For each chapter:

1. **Read** `prompts/ChapNN.txt` — the APAS textbook prose defining the ADT.
2. **Read** the StEph trait file(s) — the source-of-truth spec signatures.
3. **Read** `src/ChapNN/analyses/veracity-review-module-fn-impls.md` — function inventory.
4. **Compare** each trait fn's ensures against the prose definition.
5. **Classify** each fn spec as:
   - **strong**: ensures faithfully encodes the prose
   - **partial**: ensures present but missing key properties
   - **weak**: ensures only structural (wf, len, finite, true)
   - **missing**: no requires/ensures at all
6. **Write** `src/ChapNN/analyses/spec-audit.md` — per-function classification table.
7. **Fix** weak/missing/partial specs by writing correct requires/ensures in the trait.
8. **Add `external_body`** to impl fns that can't prove the strengthened spec.
9. `scripts/validate.sh` — 0 errors.

## Required Reading

- `src/standards/total_order_standard.rs` — if ordering specs needed.
- `src/standards/using_closures_standard.rs` — if filter/map/tabulate specs needed.
- `src/standards/view_standard.rs` — for View patterns.

## Files to Audit

### Chap05 (5 files)

| # | File | ADT | Key Operations |
|---|------|-----|----------------|
| 1 | SetStEph.rs | ADT 5.1 Sets | find, insert, delete, filter, union, intersection, difference |
| 2 | SetMtEph.rs | Mt wrapper | Same ops, delegating |
| 3 | MappingStEph.rs | ADT 5.2 Mappings | apply, update, domain, range |
| 4 | RelationStEph.rs | ADT 5.3 Relations | related, add, remove, domain, range |
| 5 | KleeneStPer.rs | Kleene closure | closure operations |

### Chap18 (7 files)

| # | File | ADT | Key Operations |
|---|------|-----|----------------|
| 1 | ArraySeqStEph.rs | ADT 18.1 Sequences | nth, update, subseq, append, tabulate, filter, map |
| 2 | ArraySeqStPer.rs | Same, persistent | Same ops |
| 3 | ArraySeqMtEph.rs | Mt wrapper | Delegating |
| 4 | ArraySeqMtPer.rs | Mt wrapper | Delegating |
| 5-7 | Deque/Ring variants | Variants | Same ADT |

### Chap19 (4 files)

| # | File | ADT | Key Operations |
|---|------|-----|----------------|
| 1 | StackStEph.rs | ADT 19.1 Stacks | push, pop, top |
| 2 | QueueStEph.rs | ADT 19.2 Queues | enqueue, dequeue, front |
| 3-4 | Mt/StPer variants | Wrappers | Same ops |

## What "Strong" Means for Foundation ADTs

These chapters define basic ADTs. Strong specs look like:

```rust
// Set::insert
ensures self@ == old(self)@.insert(v@),

// Seq::nth
ensures i < self@.len() ==> result@ == self@[i as int],

// Stack::push
ensures self@ == old(self)@.push(v@),

// Queue::enqueue
ensures self@ =~= old(self)@.push(v@),

// Mapping::apply
ensures self@.dom().contains(k@) ==> result@ == self@[k@],
```

## Important

- The prose is the source of truth. Not the current code.
- Do NOT weaken ensures. Add `external_body` if proof breaks.
- Focus on trait signatures (impls inherit).
- StEph and StPer are both authoritative — check both.
- Mt/MtPer should mirror St specs.
- Skip Example files.

## Deliverables

- `src/Chap05/analyses/spec-audit.md`
- `src/Chap18/analyses/spec-audit.md`
- `src/Chap19/analyses/spec-audit.md`
- Strengthened ensures where needed.
- `plans/agent3-round19-report.md`
- 0 errors on validate.
- Commit + push to `agent3/ready`.
