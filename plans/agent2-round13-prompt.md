# Agent 2 — Round 13 (RESTART)

## You produced ZERO holes in 40 minutes.

You were assigned 53 external_body functions in Chap43 StEph/StPer files and
proved none of them. Agent 1 proved 7 of these in Round 11 in less time.
What happened?

## Your ONE job

Prove external_body AVL tree wrappers in Chap43 StEph/StPer files. Start with
the simplest and work up.

## Files (53 external_body + 3 assume)

- `OrderedTableStEph.rs` — 14 external_body
- `OrderedSetStEph.rs` — 14 external_body, 1 assume
- `OrderedSetStPer.rs` — 12 external_body
- `OrderedTableStPer.rs` — 10 external_body
- `AugOrderedTableStEph.rs` — 3 external_body
- `AugOrderedTableStPer.rs` — 2 assume

## How to prove them

Each function wraps an inner AVLTreeSeq call. Example pattern:

```rust
// BEFORE (external_body):
#[verifier::external_body]
fn find(&self, key: &K) -> (r: Option<V>)
    requires self.spec_orderedtablesteph_wf(),
    ensures match r { Some(v) => self@.contains_key(key@), None => !self@.contains_key(key@) }
{ ... }

// AFTER (proved):
fn find(&self, key: &K) -> (r: Option<V>)
    requires self.spec_orderedtablesteph_wf(),
    ensures match r { Some(v) => self@.contains_key(key@), None => !self@.contains_key(key@) }
{
    let result = self.inner.find(key);  // inner has verified ensures
    // chain inner ensures to outer ensures with assertions if needed
    result
}
```

Agent 1 already proved `singleton`, `delete`, `iter` in these files in Round 11.
Read those proved functions for the exact pattern.

## Triage: do them in this order

1. **size, is_empty** — trivial delegation
2. **find, contains** — single inner call
3. **first, last** — single inner call
4. **singleton** — constructor, may already be proved in some files
5. **delete, insert** — one inner call + view mapping
6. **filter** — loop with inner calls
7. **get_range, split, split_rank** — two inner calls

Do NOT attempt from_sorted_elements, rank, select, previous, next until you've
proved at least 10 of the above.

## DO NOT

- Touch Chap43 Mt files (Agent 1)
- Touch Chap41 (Agent 3)
- Touch Chap42, Chap47 (Agent 4)
- Spend more than 10 minutes on any single hole

## Rules

- Run `scripts/validate.sh` after every change.
- NO accept().
- Push to `agent2/ready`. Write `plans/agent2-round13-report.md`.

## Target: -15 minimum. No excuses.
