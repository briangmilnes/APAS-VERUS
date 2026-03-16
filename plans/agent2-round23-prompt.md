# Agent 2 — Round 23: Chap40 Augmented BST _link Spec Strengthening

## Mission

Strengthen helper function specs in Chap40's 3 augmented BST files and prove the 14
`external_body` holes. All 3 files have clean external dependencies — the blockers are
internal (weak _link helper ensures).

## Current State (14 holes)

| # | Chap | File | Holes | Functions |
|---|------|------|:-----:|-----------|
| 1 | 40 | BSTKeyValueStEph.rs | 5 | insert, delete, find, contains, get |
| 2 | 40 | BSTReducedStEph.rs | 5 | insert, delete, find, contains, get |
| 3 | 40 | BSTSizeStEph.rs | 4 | insert, delete, find, contains |

All holes are `external_body` on top-level trait functions that delegate to `_link`
helpers with weak ensures.

## Root Cause

The `_link` helpers (e.g., `insert_link`, `find_link`, `delete_link`) have size-only
ensures — they track tree size but not content (`spec_content_link`). The top-level
functions need content ensures (`self@ == old(self)@.insert(key, value)` etc.), which
can't be derived from size-only helper specs.

## Approach

### Step 1: Read and understand the pattern

All 3 files share the same treap-based BST structure (from Chap39). Read one file
thoroughly (suggest BSTKeyValueStEph.rs) to understand:
- The type structure (Link, Node, Tree)
- `spec_content_link` — what the abstract content spec looks like
- The current `_link` helper ensures (what's there, what's missing)
- The top-level trait ensures (what needs to be proved)

### Step 2: Strengthen `insert_link` ensures

Add content ensures:
```rust
fn insert_link(link: &mut Link<K, V>, key: K, value: V, priority: u64)
    ensures
        // existing ensures (size, etc.) stay
        spec_content_link(&*link) == spec_content_link(&*old(link)).insert((key@, value@)),
        // OR whatever the content model is — it may be a Map<K::V, V::V>
```

Read the View impl to understand what `self@` returns — it might be a Map, Set, or Seq.
The ensures must match.

### Step 3: Strengthen `find_link` and `delete_link`

Similar pattern:
- `find_link`: `ensures found is Some ==> spec_content_link(link).contains(key)`
- `delete_link`: `ensures spec_content_link(&*link) == spec_content_link(&*old(link)).remove(key)`

### Step 4: Prove top-level operations

With strengthened _link helpers, remove `external_body` from each top-level function.
The bodies already delegate to the helpers — they should verify with the stronger specs.

### Step 5: Handle the PartialEq bridge

Agent 2's R22 report identified the structural-vs-runtime equality gap. The `_link`
helpers use runtime `==` (via `PartialOrd::lt`) for BST navigation, but specs use
structural `==`. You have two options:

1. **Add `requires` that bounds equality**: If the type has a `PartialEqSpecImpl` bound
   or the existing code uses `assume` in eq body (standard pattern), you can prove through
   the bridge.
2. **Leave the equality gap as external_body**: If the bridge is truly missing, prove
   everything EXCEPT the equality-dependent steps and report what remains.

Do not add new `assume` calls. Use existing eq/clone bridge patterns if they're already
in the file.

## Important

- You MAY add requires and strengthen ensures on `_link` helper functions.
- Do NOT weaken any existing ensures.
- Do NOT add `assume`, `accept`, or `external_body`.
- `scripts/validate.sh` after each file — 0 errors.

## Deliverables

- Strengthened _link helper specs in all 3 Chap40 files.
- Proven external_body holes where possible.
- `plans/agent2-round23-report.md` with clear accounting of what proved and what didn't.
- 0 errors on validate.
- Commit + push to `agent2/ready`.
